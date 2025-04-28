use crate::error::WallbashError;
use std::path::PathBuf;
use std::process::Command;

pub fn detect_hyprland_wallpaper() -> Result<PathBuf, WallbashError> {
    println!("Attempting to detect Hyprland wallpaper via hyprctl...");

    let output_hyprpaper = Command::new("hyprctl")
        .args(["hyprpaper", "listactive"])
        .output();

    if let Ok(ref out) = output_hyprpaper {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if let Some(line) = stdout.lines().find(|l| l.contains("Wallpaper ")) {
                if let Some(path_part) = line.split("Wallpaper ").nth(1) {
                    if let Some(path_str) = path_part.split(" on monitor").next() {
                        let path = PathBuf::from(path_str.trim());
                        if path.exists() {
                            println!("Detected hyprpaper wallpaper: {}", path.display());
                            return Ok(path);
                        } else {
                            eprintln!(
                                "Warning: hyprpaper reported path does not exist: {}",
                                path.display()
                            );
                        }
                    }
                }
            }
            eprintln!(
                "Warning: Could not parse hyprpaper listactive output: {}",
                stdout
            );
        } else {
            let stderr = String::from_utf8_lossy(&out.stderr);
            eprintln!(
                "Warning: `hyprctl hyprpaper listactive` failed: {}",
                stderr.trim()
            );
        }
    } else if let Err(e) = output_hyprpaper {
        if e.kind() == std::io::ErrorKind::NotFound {
            return Err(WallbashError::WallpaperDetection(
                "`hyprctl` command not found.".to_string(),
            ));
        }
        eprintln!(
            "Warning: Failed to execute `hyprctl hyprpaper listactive`: {}",
            e
        );
    }

    let output_getvar = Command::new("hyprctl")
        .args(["getvar", "wallpaper"])
        .output();

    if let Ok(ref out) = output_getvar {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if let Some(path_str) = stdout.splitn(2, ": ").nth(1) {
                let path = PathBuf::from(path_str.trim());
                if path.exists() {
                    println!("Detected wallpaper variable: {}", path.display());
                    return Ok(path);
                }
            }
        }
    }

    Err(WallbashError::WallpaperDetection(
        "Could not automatically detect wallpaper using hyprctl. Please provide the path manually."
            .to_string(),
    ))
}
