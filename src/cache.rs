use crate::config::{CacheData, ColorProfile, Palette, SortMode};
use crate::error::WallbashError;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

fn calculate_checksum(file_path: &Path) -> Result<String, WallbashError> {
    let mut file = File::open(file_path).map_err(|e| {
        WallbashError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to open '{}': {}", file_path.display(), e),
        ))
    })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

pub fn read_cache(cache_file: &Path) -> Result<Option<CacheData>, WallbashError> {
    if !cache_file.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(cache_file)?;
    serde_json::from_str(&content)
        .map(Some)
        .map_err(WallbashError::JsonError)
}

pub fn write_cache(cache_file: &Path, data: &CacheData) -> Result<(), WallbashError> {
    let json_string = serde_json::to_string_pretty(data)?;
    let mut file = File::create(cache_file)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

pub fn needs_regeneration(
    cache_file: &Path,
    current_image_path: &Path,
    current_profile: &ColorProfile,
    current_sort_mode: SortMode,
) -> Result<Option<Palette>, WallbashError> {
    let image_path_str = current_image_path.display().to_string();

    match read_cache(cache_file)? {
        Some(cached_data) => {
            if cached_data.image_path != image_path_str {
                println!("Cache invalidated: Image path changed.");
                return Ok(None);
            }

            if &cached_data.color_profile != current_profile
                || cached_data.sort_mode != current_sort_mode
            {
                println!("Cache invalidated: Profile or sort mode changed.");
                return Ok(None);
            }

            let current_checksum = calculate_checksum(current_image_path)?;
            if cached_data.image_checksum != current_checksum {
                println!("Cache invalidated: Image content changed (checksum mismatch).");
                return Ok(None);
            }

            println!("Using cached palette for '{}'", image_path_str);
            Ok(Some(cached_data.palette))
        }
        None => {
            println!("No valid cache found.");
            Ok(None)
        }
    }
}

pub fn create_cache_data(
    image_path: &Path,
    profile: &ColorProfile,
    sort_mode: SortMode,
    palette: &Palette,
) -> Result<CacheData, WallbashError> {
    let checksum = calculate_checksum(image_path)?;
    Ok(CacheData {
        image_path: image_path.display().to_string(),
        image_checksum: checksum,
        color_profile: profile.clone(),
        sort_mode,
        palette: palette.clone(),
    })
}
