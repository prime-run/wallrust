use crate::error::WallbashError;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::process::{Command, Output, Stdio};

lazy_static! {
    // example: `6400: (215,215,215) #D7D7D7 srgb(215,215,215)`
    // example: ` 1: ( 48, 50, 56) #303238 srgb(48,50,56)`
    static ref HISTOGRAM_RE: Regex = Regex::new(r"^\s*(\d+):\s*.*\s+#([0-9a-fA-F]{6})").unwrap();

    // example: `0.5` or `1.2345e-05`
    static ref FX_MEAN_RE: Regex = Regex::new(r"^[0-9.eE+-]+$").unwrap();

    // regex to parse HSB output from histogram:info: like hsb(H,S%,B%)
    // example: `1: ( 35, 80, 93) #EDBF23 hsb(47.7,76.2%,92.9%)` - We want the first number (Hue)
    static ref HSB_RE: Regex = Regex::new(r"hsb\((\d+\.?\d*),").unwrap();
}

pub fn run_magick(args: &[&str]) -> Result<Output, WallbashError> {
    let output_res = Command::new("magick")
        .args(args)
        .stdin(Stdio::null())
        .output();

    match output_res {
        Ok(output) => {
            // check stderr carefully - magick often puts stupid warnings here but still succeeds
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            // check if stderr contains "warning:" or "Warning:" case-insensitively
            // FIX: a more realiable method ?
            let is_warning_only = stderr_str.to_lowercase().contains("warning:");

            if !output.status.success() && !is_warning_only {
                // treat as error if non-zero exit AND stderr doesn't look like just warnings
                // hmmm?
                Err(WallbashError::MagickCommand {
                    cmd: format!("magick {}", args.join(" ")),
                    stderr: stderr_str.into_owned(),
                })
            } else {
                Ok(output)
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(WallbashError::MagickNotFound),
        Err(e) => Err(WallbashError::CommandRun {
            cmd: format!("magick {}", args.join(" ")),
            source: e,
        }),
    }
}

pub trait OutputExt {
    fn stdout_str(&self) -> Result<String, WallbashError>;
}

impl OutputExt for Output {
    fn stdout_str(&self) -> Result<String, WallbashError> {
        String::from_utf8(self.stdout.clone()).map_err(|e| {
            WallbashError::MagickParse(format!("Magick output is not valid UTF-8: {}", e))
        })
    }
}

pub fn ping_image(image_path: &Path) -> Result<(), WallbashError> {
    let path_str = image_path.to_str().ok_or_else(|| {
        WallbashError::InvalidInput(format!("Invalid path characters: {}", image_path.display()))
    })?;
    run_magick(&["-ping", path_str, "-format", "%t", "info:"])?;
    Ok(())
}

pub fn create_mpc_cache(image_path: &Path, mpc_path: &Path) -> Result<(), WallbashError> {
    let img_str = image_path.to_str().ok_or_else(|| {
        WallbashError::InvalidInput(format!("Invalid path characters: {}", image_path.display()))
    })?;
    let mpc_str = mpc_path.to_str().ok_or_else(|| {
        WallbashError::InvalidInput(format!("Invalid path characters: {}", mpc_path.display()))
    })?;
    run_magick(&[
        "-quiet",
        "-regard-warnings",
        &format!("{}[0]", img_str),
        "-alpha",
        "off",
        "+repage",
        mpc_str,
    ])?;
    Ok(())
}

pub fn extract_kmeans_colors(
    mpc_path: &Path,
    colors: usize,
    fuzz: u8,
) -> Result<Vec<(u64, String)>, WallbashError> {
    let mpc_arg = format!("mpc:{}", mpc_path.to_str().unwrap()); // Assumes valid path checked before
    let kmeans_output = run_magick(&[
        &mpc_arg,
        "-depth",
        "8",
        "-fuzz",
        &format!("{}%", fuzz),
        "+dither",
        "-kmeans",
        &colors.to_string(),
        "-depth",
        "8",
        "-format",
        "%c",
        "histogram:info:",
    ])?;

    let mut dcol_raw: Vec<(u64, String)> = Vec::new();
    for line in kmeans_output.stdout_str()?.lines() {
        if let Some(caps) = HISTOGRAM_RE.captures(line) {
            let count_str = caps.get(1).map_or("0", |m| m.as_str());
            let hex = caps.get(2).map_or("", |m| m.as_str());
            if !hex.is_empty() {
                let count = count_str.parse::<u64>().unwrap_or(0);
                dcol_raw.push((count, hex.to_uppercase()));
            }
        }
    }

    dcol_raw.sort_by(|a, b| b.0.cmp(&a.0));

    Ok(dcol_raw)
}

pub fn check_brightness_dark(target: &str) -> Result<bool, WallbashError> {
    let fx_output = run_magick(&[
        target,
        "-colorspace",
        "gray",
        "-format",
        "%[fx:mean]",
        "info:",
    ])?;

    let mean_str = fx_output.stdout_str()?.trim().to_string();
    if !FX_MEAN_RE.is_match(&mean_str) {
        if mean_str.contains("undefined") {
            eprintln!("Warning: fx:mean brightness calculation returned undefined for target '{}'. Assuming light.", target);
            return Ok(false);
        }
        return Err(WallbashError::MagickParse(format!(
            "Could not parse fx:mean output: '{}' for target '{}'",
            mean_str, target
        )));
    }

    let mean: f64 = mean_str.parse()?;

    Ok(mean < 0.5)
}

pub fn get_average_saturation(mpc_path: &Path) -> Result<f64, WallbashError> {
    let mpc_arg = format!("mpc:{}", mpc_path.to_str().unwrap());
    let fx_output = run_magick(&[
        &mpc_arg,
        "-colorspace",
        "HSL",
        "-channel",
        "g",
        "-separate",
        "+channel",
        "-format",
        "%[fx:mean]",
        "info:",
    ])?;

    let mean_str = fx_output.stdout_str()?.trim().to_string();
    if !FX_MEAN_RE.is_match(&mean_str) {
        if mean_str.contains("undefined") {
            eprintln!("Warning: fx:mean saturation calculation returned undefined. Assuming not grayscale.");
            return Ok(1.0);
        }
        return Err(WallbashError::MagickParse(format!(
            "Could not parse saturation fx:mean output: '{}'",
            mean_str
        )));
    }
    let mean: f64 = mean_str.parse()?;
    Ok(mean)
}

pub fn modulate_color(
    source_target: &str,
    bri: u8,
    sat: u8,
    hue: u8,
) -> Result<String, WallbashError> {
    let modulate_arg = format!("{},{},{}", bri, sat, hue);
    let mod_output = run_magick(&[
        source_target,
        "-depth",
        "8",
        "-normalize",
        "-modulate",
        &modulate_arg,
        "-depth",
        "8",
        "-format",
        "%c",
        "histogram:info:",
    ])?;

    let hex = HISTOGRAM_RE
        .captures(&mod_output.stdout_str()?)
        .and_then(|cap| cap.get(2))
        .map(|m| m.as_str().to_uppercase())
        .ok_or_else(|| {
            WallbashError::MagickParse(format!(
                "Could not parse modulated color from: {}",
                mod_output.stdout_str().unwrap_or_default()
            ))
        })?;
    Ok(hex)
}

pub fn get_hsb_hue(color_target: &str) -> Result<String, WallbashError> {
    let hue_output = run_magick(&[
        color_target,
        "-colorspace",
        "HSB",
        "-format",
        "%c",
        //NOTE:  eg output: `1: (237,191, 35) #EDBF23 hsb(47.7,76.2%,92.9%)`
        "histogram:info:",
    ])?;

    //FIX: for E0716: Bind the String to a variable first ***
    let output_string = hue_output.stdout_str()?;

    let hue_str = HSB_RE
        .captures(&output_string)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or_else(|| {
            WallbashError::MagickParse(format!("Could not parse HSB Hue from: {}", output_string))
        })?;
    Ok(hue_str.to_string())
}

pub fn color_from_hsb(hsb_string: &str) -> Result<String, WallbashError> {
    let hsb_target = format!("xc:{}", hsb_string);
    let acol_output = run_magick(&[
        &hsb_target,
        "-depth",
        "8",
        "-format",
        "%c",
        "histogram:info:",
    ])?;

    let hex = HISTOGRAM_RE
        .captures(&acol_output.stdout_str()?)
        .and_then(|cap| cap.get(2))
        .map(|m| m.as_str().to_uppercase())
        .ok_or_else(|| {
            WallbashError::MagickParse(format!(
                "Could not parse color from HSB ({}) output: {}",
                hsb_string,
                acol_output.stdout_str().unwrap_or_default()
            ))
        })?;
    Ok(hex)
}
