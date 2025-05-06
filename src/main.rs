mod cache;
mod cli;
mod config;
mod error;
mod html;
mod imagemagick;
mod output;
mod palette;
mod wallpaper;

use anyhow::{Context, Result};
use clap::Parser;
use config::AppPaths;
use error::WallbashError;
use std::fs;
use std::path::{PathBuf, Path};
use sha2::{Sha256, Digest};


fn calculate_hash(path: &Path) -> Result<String, WallbashError> {
    let path_str = path.to_str().ok_or_else(|| {
        WallbashError::InvalidInput(format!("Invalid path characters: {}", path.display()))
    })?;
    
    let mut hasher = Sha256::new();
    hasher.update(path_str.as_bytes());
    let result = hasher.finalize();
    
    Ok(format!("{:x}", result))
}


fn log_palette_preview(palette: &config::Palette, source: &str) {
    println!("----- {} Palette Preview -----", source);
    println!("Mode: {}", palette.mode);
    
    
    for i in 0..std::cmp::min(3, palette.primary.len()) {
        println!("Primary {}: {}", i+1, palette.primary[i]);
    }
    
    
    if let Some(accents) = palette.accents.first() {
        if let Some(accent) = accents.first() {
            println!("First Accent: {}", accent);
        }
    }
    println!("-------------------------------");
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let input_image_path = match cli.input_image {
        Some(path) => PathBuf::from(
            shellexpand::full(&path)
                .map_err(|e| {
                    WallbashError::PathExpansion(format!(
                        "Input image path expansion failed: {}",
                        e
                    ))
                })?
                .into_owned(),
        ),
        None if cli.detect_hyprland => {
            wallpaper::detect_hyprland_wallpaper().context("Failed wallpaper detection")?
        }
        None => {
            anyhow::bail!("No input image provided and --detect-hyprland not specified.");
        }
    };

    if !input_image_path.is_file() {
        anyhow::bail!(WallbashError::InvalidInput(format!(
            "Input image file not found or not a file: {}",
            input_image_path.display()
        )));
    }
    println!("Using wallpaper: {}", input_image_path.display());

    let app_paths =
        AppPaths::new(cli.output_dir).context("Failed to initialize application paths")?;
    let color_profile =
        config::ColorProfile::from_cli(cli.vibrant, cli.pastel, cli.mono, cli.custom)
            .context("Invalid color profile selection")?;
    let initial_sort_mode =
        config::SortMode::from_cli(cli.dark, cli.light).context("Invalid sort mode selection")?;

    let extraction_image_path;
    let file_hash;
    
    if cli.wallset {
        
        file_hash = calculate_hash(&input_image_path)?;
        
        
        app_paths.ensure_thumbs_dir()?;
        
        let thumbnail_path = app_paths.thumbs_dir.join(format!("{}.thmb", file_hash));
        println!("Thumbnail path: {}", thumbnail_path.display());
        
        if !thumbnail_path.exists() || cli.force {
            println!("Generating thumbnail for color extraction...");
            imagemagick::generate_thumbnail(&input_image_path, &thumbnail_path)
                .context("Failed to generate thumbnail")?;
        } else {
            println!("Using existing thumbnail: {}", thumbnail_path.display());
        }
        extraction_image_path = thumbnail_path;
    } else {
        
        extraction_image_path = input_image_path.clone();
        file_hash = String::new(); 
    }

    
    let should_force = if cli.force {
        println!("Force flag set, skipping cache check.");
        true
    } else if cli.wallset {
        println!("Checking if cache needs regeneration for wallset mode...");
        false
    } else {
        false
    };
    
    let cached_palette = if should_force {
        None
    } else {
        cache::needs_regeneration(
            &app_paths.wallbash_cache_file,
            &input_image_path,
            &color_profile,
            initial_sort_mode,
            cli.wallset
        )?
    };

    let final_palette = match cached_palette {
        Some(palette) => {
            if cli.wallset {
                println!("Using cached palette (from wallset mode)");
            } else {
                println!("Using cached palette (from regular mode)");
            }
            log_palette_preview(&palette, "Cached");
            palette
        },
        None => {
            println!(
                "Generating new palette (Profile: {}, Mode: {}, Colors: {}, Fuzz: {}, Wallset: {})...",
                color_profile, initial_sort_mode, cli.colors, cli.fuzz, cli.wallset
            );

            
            imagemagick::ping_image(&extraction_image_path).context("ImageMagick ping failed")?;

            struct CleanupGuard<'a>(&'a PathBuf);
            impl<'a> Drop for CleanupGuard<'a> {
                fn drop(&mut self) {
                    match fs::remove_file(self.0) {
                        Ok(_) => {}
                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                        Err(e) => eprintln!(
                            "Warning: Failed to remove temp MPC file {}: {}",
                            self.0.display(),
                            e
                        ),
                    }
                }
            }
            let mpc_path = app_paths.mpc_cache_file.clone();
            let _cleanup_guard = CleanupGuard(&mpc_path);

            
            imagemagick::create_mpc_cache(&extraction_image_path, &app_paths.mpc_cache_file)
                .context("Failed to create ImageMagick MPC cache")?;

            let mut base_colors_raw =
                imagemagick::extract_kmeans_colors(&app_paths.mpc_cache_file, cli.colors, cli.fuzz)
                    .context("Failed to extract k-means colors")?;

            if base_colors_raw.len() < cli.colors {
                println!(
                    "RETRYING K-Means: Found {} colors, need {}. Requesting {}.",
                    base_colors_raw.len(),
                    cli.colors,
                    cli.colors + 2
                );
                base_colors_raw = imagemagick::extract_kmeans_colors(
                    &app_paths.mpc_cache_file,
                    cli.colors + 2,
                    cli.fuzz,
                )
                .context("Failed to extract k-means colors on retry")?;

                if base_colors_raw.len() < cli.colors {
                    anyhow::bail!(WallbashError::NotEnoughColors {
                        required: cli.colors,
                        found: base_colors_raw.len()
                    });
                }
            }

            let base_hex_colors: Vec<String> = base_colors_raw
                .into_iter()
                .map(|(_, hex)| hex)
                .take(cli.colors)
                .collect();

            
            let generated_palette = palette::generate_palette(
                &input_image_path, 
                &app_paths.mpc_cache_file,
                base_hex_colors,
                cli.colors,
                &color_profile,
                initial_sort_mode,
            )
            .context("Failed to generate full palette")?;

            let cache_data = cache::create_cache_data(
                &input_image_path,
                &color_profile,
                initial_sort_mode,
                &generated_palette,
                cli.wallset
            )?;
            cache::write_cache(&app_paths.wallbash_cache_file, &cache_data)
                .context("Failed to write palette cache")?;

            
            log_palette_preview(&generated_palette, if cli.wallset { "Thumbnail" } else { "Original" });
            generated_palette
        }
    };

    
    output::generate_outputs(&final_palette, &app_paths, cli.no_templates)
        .context("Failed to generate output files")?;
    
    
    if cli.wallset && !file_hash.is_empty() {
        
        app_paths.ensure_dcols_dir()?;
        
        let dcol_path = app_paths.dcols_dir.join(format!("{}.dcol", file_hash));
        output::write_dcol(&final_palette, &dcol_path)
            .context("Failed to write dcol file to hashed path")?;
    }
        
    if cli.html {
        let html_path = app_paths.output_dir.join("palette.html");
        html::generate_html(&final_palette, &html_path)
            .context("Failed to generate HTML preview")?;
        println!("Generated HTML preview at: {}", html_path.display());
    }

    println!("Wallbash finished successfully.");
    Ok(())
}
