//! Wallrust is a fast, CLI tool for extracting color palettes from images and generating theme/config files for ricing, theming, and automation workflows. Designed for power users, it leverages ImageMagick for robust color analysis and supports advanced customization, multiple output formats, and seamless integration with user-defined templates. Wallrust automates the process of keeping your desktop, terminal, and app configs in sync with your wallpaper, making it a cornerstone for ricing setups and theme switchers.
//!
//! ## Usage
//! ```text
//! Usage: wallrust [OPTIONS] [INPUT_IMAGE]
//!
//! Arguments:
//!   [INPUT_IMAGE]  
//!
//! Options:
//!   -f, --force             
//!   -o, --output-dir <DIR>  
//!   -v, --vibrant           Use vibrant color profile
//!   -p, --pastel            Use pastel color profile
//!   -m, --mono              Use monochrome color profile
//!   -c, --custom <CURVE>    Use custom color curve (provide curve string)
//!   -d, --dark              Force dark sort mode
//!   -l, --light             Force light sort mode
//!       --colors <COLORS>   Number of primary colors to extract [default: 4]
//!       --fuzz <FUZZ>       Color fuzziness percentage for k-means [default: 70]
//!       --detect-hyprland   Attempt to detect current Hyprland wallpaper via hyprctl
//!       --html              Generate HTML color palette preview
//!       --wallset           Generate thumbnails and dcol files compatible with wallbash scripts
//!       --no-templates      Skip custom template generation
//!   -h, --help              Print help
//!   -V, --version           Print version
//! ```
//!
//! ## Example usage
//! ```sh
//! # Basic: Extract a palette and generate CSS, JSON, dcol, and HTML preview
//! wallrust ~/Pictures/wallpaper.jpg --colors 6 --html
//!
//! # Generate and apply templates (from ~/.config/wallrust/templates/) to instantly rice your setup
//! wallrust ~/Pictures/wallpaper.jpg --output-dir ~/.config/kitty/
//!
//! # Use wallset mode for hash-based palette extraction (for theme switchers and caching)
//! wallrust ~/Pictures/wallpaper.jpg --wallset
//!
//! # Extract from the current Hyprland wallpaper, apply a pastel curve, and skip template generation
//! wallrust --detect-hyprland --pastel --no-templates
//! ```
//!
//! ## Advanced
//! Wallrust supports custom color curves, wallset mode (hash-based palette extraction for theme switching), palette caching, and automatic dark/light mode detection. It can generate and place files anywhere, with optional backup of previous configs. Integrates with Hyprland for automatic wallpaper detection and can be scripted for dynamic theme automation.
//!
//! ## Templating
//! Place [Tera](https://tera.netlify.app/) templates in `~/.config/wallrust/templates/` to generate any config file with palette variables (primary, text, accents, etc). Output paths and backup behavior can be controlled via template directives at the top of each template. This enables fully automated, wallpaper-driven config generation for any app.
//!
//! ## Available Tera Template Variables
//! The following variables are available in your templates:
//!
//! - `mode`: "dark" or "light" (auto-detected or forced)
//! - `wallpaper`: Path to the source image
//! - `primary`: Array of primary hex colors (e.g., ["AABBCC", ...])
//! - `text`: Array of text hex colors
//! - `accents`: 2D array of accent hex colors by primary index (e.g., `accents[0][0]`)
//! - `primary_rgba`: Array of RGBA strings for each primary color (e.g., "170,187,204,1.0")
//! - `text_rgba`: Array of RGBA strings for each text color
//! - `accents_rgba`: 2D array of RGBA strings for each accent color
//! - `is_dark`: Boolean, true if mode is dark
//!
//! Example usage in a Tera template:
//!
//! ```tera
//! background = #{{ primary[0] }}
//! foreground = #{{ text[0] }}
//! accent1 = #{{ accents[0][0] }}
//! ```
//! 

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

            /// Helper struct to ensure temporary MPC cache files are cleaned up after palette extraction.
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
