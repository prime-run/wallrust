use crate::config::{
    ColorProfile, Palette, SortMode, ACCENT_COUNT, CURVE_GRAYSCALE, PRY_DARK_BRI, PRY_DARK_HUE,
    PRY_DARK_SAT, PRY_LIGHT_BRI, PRY_LIGHT_HUE, PRY_LIGHT_SAT, TXT_DARK_BRI, TXT_LIGHT_BRI,
};
use crate::error::WallbashError;
use crate::imagemagick::{
    check_brightness_dark, color_from_hsb, get_average_saturation, get_hsb_hue, modulate_color,
};
use std::path::Path;

fn rgb_negative(hex_color: &str) -> Result<String, WallbashError> {
    let hex = hex_color.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(WallbashError::InvalidInput(format!(
            "Invalid hex color format for negation: '{}'",
            hex_color
        )));
    }
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok(format!("{:02X}{:02X}{:02X}", 255 - r, 255 - g, 255 - b))
}

pub fn rgba_convert(hex_color: &str) -> Result<String, WallbashError> {
    let hex = hex_color.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(WallbashError::InvalidInput(format!(
            "Invalid hex color format for rgba conversion: '{}'",
            hex_color
        )));
    }
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok(format!("rgba({},{},{},\\1)", r, g, b))
}

fn calculate_luma(hex_color: &str) -> Result<f64, WallbashError> {
    let hex = hex_color.trim_start_matches('#');
    if hex.len() != 6 {
        return Ok(0.0);
    }

    let r_srgb = u8::from_str_radix(&hex[0..2], 16)? as f64 / 255.0;
    let g_srgb = u8::from_str_radix(&hex[2..4], 16)? as f64 / 255.0;
    let b_srgb = u8::from_str_radix(&hex[4..6], 16)? as f64 / 255.0;

    let r = if r_srgb <= 0.03928 {
        r_srgb / 12.92
    } else {
        ((r_srgb + 0.055) / 1.055).powf(2.4)
    };
    let g = if g_srgb <= 0.03928 {
        g_srgb / 12.92
    } else {
        ((g_srgb + 0.055) / 1.055).powf(2.4)
    };
    let b = if b_srgb <= 0.03928 {
        b_srgb / 12.92
    } else {
        ((b_srgb + 0.055) / 1.055).powf(2.4)
    };

    Ok(0.2126 * r + 0.7152 * g + 0.0722 * b)
}

fn parse_curve(curve_str: &str) -> Result<Vec<(u8, u8)>, WallbashError> {
    let mut points = Vec::new();
    for line in curve_str.trim().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let bri = parts[0].parse::<u8>().map_err(|e| {
                WallbashError::InvalidInput(format!(
                    "Invalid brightness value in curve '{}': {}",
                    line, e
                ))
            })?;
            let sat = parts[1].parse::<u8>().map_err(|e| {
                WallbashError::InvalidInput(format!(
                    "Invalid saturation value in curve '{}': {}",
                    line, e
                ))
            })?;
            if bri > 100 || sat > 100 {
                return Err(WallbashError::InvalidInput(format!(
                    "Curve values must be between 0 and 100: '{}'",
                    line
                )));
            }
            points.push((bri, sat));
        } else if !line.trim().is_empty() {
            return Err(WallbashError::InvalidInput(format!(
                "Invalid line format in curve: '{}'",
                line
            )));
        }
    }
    //WARN: don't strictly enforce 9 points here, but rely on the loop logic later
    if points.len() != ACCENT_COUNT {
        eprintln!(
             "Warning: Parsed curve has {} points, but {} are expected for standard accent generation.",
             points.len(), ACCENT_COUNT
         );
    }
    Ok(points)
}

pub fn generate_palette(
    wallpaper_path: &Path,
    mpc_path: &Path,
    mut initial_hex_colors: Vec<String>,
    num_colors: usize,
    profile: &ColorProfile,
    initial_sort_mode: SortMode,
) -> Result<Palette, WallbashError> {
    let mut palette = Palette::default();
    palette.wallpaper = wallpaper_path.display().to_string();

    let mut final_sort_mode = initial_sort_mode;
    if initial_sort_mode == SortMode::Auto {
        let mpc_target = format!("mpc:{}", mpc_path.to_str().unwrap());
        let is_dark = check_brightness_dark(&mpc_target)?;
        final_sort_mode = if is_dark {
            SortMode::Dark
        } else {
            SortMode::Light
        };
        println!("Auto-detected sort mode: {}", final_sort_mode);
    }
    palette.mode = final_sort_mode.to_string();
    palette.is_dark = final_sort_mode == SortMode::Dark;

    // NOTE: Sort primary colors hmmmmm
    // sort by perceived brightness (luma) before applying light/dark logic
    initial_hex_colors.sort_by_cached_key(|hex| {
        ordered_float::NotNan::new(calculate_luma(hex).unwrap_or(0.0)).unwrap_or_default()
    });
    if final_sort_mode == SortMode::Light {
        initial_hex_colors.reverse();
    }
    // now `initial_hex_colors` is sorted according to the final mode

    let mut current_curve_str = profile.to_curve_string();
    let saturation = get_average_saturation(mpc_path)?;
    if saturation < 0.12 {
        println!("Image detected as low saturation/grayscale, using mono curve.");
        current_curve_str = CURVE_GRAYSCALE.to_string();
    }

    let curve_points = parse_curve(&current_curve_str)?;

    palette.primary = vec![String::new(); num_colors];
    palette.text = vec![String::new(); num_colors];
    palette.accents = vec![Vec::with_capacity(ACCENT_COUNT); num_colors];
    palette.primary_rgba = vec![String::new(); num_colors];
    palette.text_rgba = vec![String::new(); num_colors];
    palette.accents_rgba = vec![Vec::with_capacity(ACCENT_COUNT); num_colors];

    for i in 0..num_colors {
        let current_hex = if let Some(hex) = initial_hex_colors.get(i).cloned() {
            hex
        } else if i > 0 && !palette.primary[i - 1].is_empty() {
            let prev_hex = &palette.primary[i - 1];
            println!(
                "Regenerating missing primary color {} from {}",
                i + 1,
                prev_hex
            );

            let prev_target = format!("xc:#{}", prev_hex);
            let is_prev_dark = check_brightness_dark(&prev_target)?;
            let (mod_bri, mod_sat, mod_hue) = if is_prev_dark {
                (PRY_DARK_BRI, PRY_DARK_SAT, PRY_DARK_HUE)
            } else {
                (PRY_LIGHT_BRI, PRY_LIGHT_SAT, PRY_LIGHT_HUE)
            };

            modulate_color(&prev_target, mod_bri, mod_sat, mod_hue)?
        } else {
            return Err(WallbashError::NotEnoughColors {
                required: num_colors,
                found: i,
            });
        };

        palette.primary[i] = current_hex.clone();
        palette.primary_rgba[i] = rgba_convert(&current_hex)?;

        let n_txt = rgb_negative(&current_hex)?;
        let pry_target = format!("xc:#{}", current_hex);
        let is_pry_dark = check_brightness_dark(&pry_target)?;
        let mod_bri_txt = if is_pry_dark {
            TXT_DARK_BRI
        } else {
            TXT_LIGHT_BRI
        };

        let tcol = modulate_color(&format!("xc:#{}", n_txt), mod_bri_txt, 10, 100)?;
        palette.text[i] = tcol.clone();
        palette.text_rgba[i] = rgba_convert(&tcol)?;

        let x_hue = get_hsb_hue(&pry_target)?;

        let mut sorted_curve = curve_points.clone();
        sorted_curve.sort_by(|a, b| a.0.cmp(&b.0));
        if final_sort_mode == SortMode::Light {
            sorted_curve.reverse();
        }

        for (x_bri, x_sat) in sorted_curve.iter().take(ACCENT_COUNT) {
            let hsb_arg = format!("hsb({},{}%,{}%)", x_hue, x_sat, x_bri);
            let acol = color_from_hsb(&hsb_arg)?;
            palette.accents[i].push(acol.clone());
            palette.accents_rgba[i].push(rgba_convert(&acol)?);
        }
        palette.accents[i].resize(ACCENT_COUNT, "000000".to_string());
        palette.accents_rgba[i].resize(ACCENT_COUNT, rgba_convert("000000")?);
    }

    Ok(palette)
}
