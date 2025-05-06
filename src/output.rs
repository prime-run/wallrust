use crate::config::{AppPaths, Palette, ACCENT_COUNT};
use crate::error::WallbashError;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};
use std::io::BufRead;
use shellexpand;

pub fn write_dcol(palette: &Palette, dcol_path: &Path) -> Result<(), WallbashError> {
    
    if let Some(parent) = dcol_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let mut writer = File::create(dcol_path)?;

    writeln!(writer, "dcol_mode=\"{}\"", palette.mode)?;
    writeln!(writer, "dcol_wallpaper=\"{}\"", palette.wallpaper)?;

    for i in 0..palette.primary.len() {
        writeln!(writer, "dcol_pry{}=\"{}\"", i + 1, palette.primary[i])?;
        writeln!(
            writer,
            "dcol_pry{}_rgba=\"{}\"",
            i + 1,
            palette.primary_rgba[i]
        )?;
        writeln!(writer, "dcol_txt{}=\"{}\"", i + 1, palette.text[i])?;
        writeln!(
            writer,
            "dcol_txt{}_rgba=\"{}\"",
            i + 1,
            palette.text_rgba[i]
        )?;

        for j in 0..ACCENT_COUNT {
            if let Some(accent_color) = palette.accents.get(i).and_then(|a| a.get(j)) {
                writeln!(writer, "dcol_{}xa{}=\"{}\"", i + 1, j + 1, accent_color)?;
                if let Some(accent_rgba) = palette.accents_rgba.get(i).and_then(|a| a.get(j)) {
                    writeln!(writer, "dcol_{}xa{}_rgba=\"{}\"", i + 1, j + 1, accent_rgba)?;
                }
            }
        }
        writeln!(writer)?;
    }

    println!("Generated {}", dcol_path.display());
    Ok(())
}

fn write_css(palette: &Palette, paths: &AppPaths) -> Result<(), WallbashError> {
    let css_path = paths.output_dir.join("wallrust.css");
    let mut writer = File::create(&css_path)?;

    writeln!(writer, "/* Wallbash Palette */")?;
    writeln!(writer, "/* Wallpaper: {} */", palette.wallpaper)?;
    writeln!(writer, "/* Mode: {} */", palette.mode)?;
    writeln!(writer, ":root {{")?;

    if let Some(pry1) = palette.primary.get(0) {
        writeln!(writer, "  --pry: #{};", pry1)?;
    }
    if let Some(txt1) = palette.text.get(0) {
        writeln!(writer, "  --txt: #{};", txt1)?;
    }
    for i in 0..palette.primary.len() {
        if let Some(pry) = palette.primary.get(i) {
            writeln!(writer, "  --pry{}: #{};", i + 1, pry)?;
        }
        if let Some(txt) = palette.text.get(i) {
            writeln!(writer, "  --txt{}: #{};", i + 1, txt)?;
        }
    }

    if let Some(accents1) = palette.accents.get(0) {
        for j in 0..ACCENT_COUNT {
            if let Some(acc) = accents1.get(j) {
                writeln!(writer, "  --xa{}: #{};", j + 1, acc)?;
            }
        }
    }

    
    
   

    writeln!(writer, "}}")?;

    println!("Generated {}", css_path.display());
    Ok(())
}

fn write_json(palette: &Palette, paths: &AppPaths) -> Result<(), WallbashError> {
    let json_path = paths.output_dir.join("wallrust.json");
    let json_string = serde_json::to_string_pretty(palette)?;
    let mut file = File::create(&json_path)?;
    file.write_all(json_string.as_bytes())?;

    println!("Generated {}", json_path.display());
    Ok(())
}

fn apply_templates(palette: &Palette, paths: &AppPaths) -> Result<(), WallbashError> {
    if !paths.template_dir.exists() {
        println!(
            "Template directory not found, skipping custom templates: {}",
            paths.template_dir.display()
        );
        return Ok(());
    }
    if !paths.template_dir.is_dir() {
        eprintln!(
            "Warning: Template path is not a directory: {}",
            paths.template_dir.display()
        );
        return Ok(());
    }

    let mut tera = match Tera::new(&format!("{}/*", paths.template_dir.display())) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "Warning: Failed to initialize Tera templates from '{}': {}",
                paths.template_dir.display(),
                e
            );
            return Ok(());
        }
    };

    tera.autoescape_on(vec![]);

    let context = Context::from_serialize(palette)?;

    for template_entry in fs::read_dir(&paths.template_dir)? {
        let template_path = match template_entry {
            Ok(entry) => entry.path(),
            Err(e) => {
                eprintln!("Warning: Failed to read entry in template directory: {}", e);
                continue;
            }
        };

        if template_path.is_file() {
            if let Some(template_name) = template_path.file_name().and_then(|n| n.to_str()) {
                let mut output_path_override: Option<String> = None;
                let mut backup_enabled: bool = false;
                if let Ok(file) = File::open(&template_path) {
                    let reader = std::io::BufReader::new(file);
                    for line in reader.lines().take(5) { // Only check the first 5 lines
                        if let Ok(l) = line {
                            let trimmed = l.trim();
                            if let Some(rest) = trimmed.strip_prefix("{# output:") {
                                if let Some(path) = rest.strip_suffix("#}") {
                                    output_path_override = Some(path.trim().to_string());
                                }
                            } else if let Some(rest) = trimmed.strip_prefix("#!output:") {
                                output_path_override = Some(rest.trim().to_string());
                            } else if let Some(rest) = trimmed.strip_prefix("{# backup:") {
                                if let Some(val) = rest.strip_suffix("#}") {
                                    backup_enabled = val.trim().eq_ignore_ascii_case("true");
                                }
                            } else if let Some(rest) = trimmed.strip_prefix("#!backup:") {
                                backup_enabled = rest.trim().eq_ignore_ascii_case("true");
                            }
                        }
                    }
                }
                match tera.render(template_name, &context) {
                    Ok(rendered_content) => {
                        let output_path = if let Some(path) = output_path_override {
                            
                            match shellexpand::full(&path) {
                                Ok(expanded) => Path::new(expanded.as_ref()).to_path_buf(),
                                Err(_) => {
                                    eprintln!("Warning: Failed to expand output path '{}', using default output dir.", path);
                                    paths.output_dir.join(template_name)
                                }
                            }
                        } else {
                            paths.output_dir.join(template_name)
                        };
                        if let Some(parent) = output_path.parent() {
                            if let Err(e) = fs::create_dir_all(parent) {
                                eprintln!("Warning: Failed to create output directory '{}': {}", parent.display(), e);
                            }
                        }
                        // Backup logic
                        if backup_enabled && output_path.exists() {
                            let backup_path = output_path.with_extension(
                                match output_path.extension().and_then(|e| e.to_str()) {
                                    Some(ext) => format!("{}.wr.bakup", ext),
                                    None => "wr.bakup".to_string(),
                                }
                            );
                            if let Err(e) = fs::copy(&output_path, &backup_path) {
                                eprintln!("Warning: Failed to backup '{}' to '{}': {}", output_path.display(), backup_path.display(), e);
                            } else {
                                println!("Backed up '{}' to '{}'", output_path.display(), backup_path.display());
                            }
                        }
                        match fs::write(&output_path, rendered_content) {
                            Ok(_) => println!("Generated from template: {}", output_path.display()),
                            Err(e) => eprintln!(
                                "Warning: Failed to write output file for template '{}': {}",
                                template_name, e
                            ),
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to render template '{}': {}",
                            template_name, e
                        );
                    }
                }
            }
        }
    }

    Ok(())
}


pub fn generate_outputs(palette: &Palette, paths: &AppPaths, skip_templates: bool) -> Result<(), WallbashError> {
    let dcol_path = paths.output_dir.join("wallrust.dcol");
    write_dcol(palette, &dcol_path)?;
    write_css(palette, paths)?;
    write_json(palette, paths)?;
    if !skip_templates {
        apply_templates(palette, paths)?;
    } else {
        println!("Skipping custom template generation (--no-templates set)");
    }
    Ok(())
}
