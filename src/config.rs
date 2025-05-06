use crate::error::WallbashError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

pub const DEFAULT_COLORS: usize = 4;
pub const DEFAULT_FUZZ: u8 = 70;
pub const CURVE_DEFAULT: &str = "32 50\n42 46\n49 40\n56 39\n64 38\n76 37\n90 33\n94 29\n100 20";
pub const CURVE_VIBRANT: &str = "18 99\n32 97\n48 95\n55 90\n70 80\n80 70\n88 60\n94 40\n99 24";
pub const CURVE_PASTEL: &str = "10 99\n17 66\n24 49\n39 41\n51 37\n58 34\n72 30\n84 26\n99 22";
pub const CURVE_MONO: &str = "10 0\n17 0\n24 0\n39 0\n51 0\n58 0\n72 0\n84 0\n99 0";
pub const CURVE_GRAYSCALE: &str = CURVE_MONO;
pub const PRY_DARK_BRI: u8 = 116;
pub const PRY_DARK_SAT: u8 = 110;
pub const PRY_DARK_HUE: u8 = 88;
pub const PRY_LIGHT_BRI: u8 = 100;
pub const PRY_LIGHT_SAT: u8 = 100;
pub const PRY_LIGHT_HUE: u8 = 114;
pub const TXT_DARK_BRI: u8 = 188;
pub const TXT_LIGHT_BRI: u8 = 16;
pub const ACCENT_COUNT: usize = 9;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorProfile {
    Default,
    Vibrant,
    Pastel,
    Mono,
    Custom(String),
}

impl Default for ColorProfile {
    fn default() -> Self {
        ColorProfile::Default
    }
}

impl ColorProfile {
    pub fn to_curve_string(&self) -> String {
        match self {
            ColorProfile::Default => CURVE_DEFAULT.to_string(),
            ColorProfile::Vibrant => CURVE_VIBRANT.to_string(),
            ColorProfile::Pastel => CURVE_PASTEL.to_string(),
            ColorProfile::Mono => CURVE_MONO.to_string(),
            ColorProfile::Custom(s) => s.clone(),
        }
    }
    pub fn from_cli(
        vibrant: bool,
        pastel: bool,
        mono: bool,
        custom: Option<String>,
    ) -> Result<Self, WallbashError> {
        let mut profile = ColorProfile::Default;
        let mut profile_count = 0;

        if vibrant {
            profile = ColorProfile::Vibrant;
            profile_count += 1;
        }
        if pastel {
            profile = ColorProfile::Pastel;
            profile_count += 1;
        }
        if mono {
            profile = ColorProfile::Mono;
            profile_count += 1;
        }
        if let Some(custom_curve) = custom {
            let cleaned_curve = custom_curve.replace("\\n", "\n");
            if cleaned_curve.split('\n').count() < ACCENT_COUNT {
                eprintln!(
                    "Warning: Custom curve has fewer than {} lines.",
                    ACCENT_COUNT
                );
            }
            profile = ColorProfile::Custom(cleaned_curve);
            profile_count += 1;
        }

        if profile_count > 1 {
            Err(WallbashError::InvalidInput(
                "Only one color profile (--vibrant, --pastel, --mono, --custom) can be specified."
                    .to_string(),
            ))
        } else {
            Ok(profile)
        }
    }
}

impl std::fmt::Display for ColorProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorProfile::Default => write!(f, "default"),
            ColorProfile::Vibrant => write!(f, "vibrant"),
            ColorProfile::Pastel => write!(f, "pastel"),
            ColorProfile::Mono => write!(f, "mono"),
            ColorProfile::Custom(_) => write!(f, "custom"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SortMode {
    Auto,
    Dark,
    Light,
}

impl SortMode {
    pub fn from_cli(dark: bool, light: bool) -> Result<Self, WallbashError> {
        if dark && light {
            Err(WallbashError::InvalidInput(
                "Cannot specify both --dark and --light modes simultaneously".to_string(),
            ))
        } else if dark {
            Ok(SortMode::Dark)
        } else if light {
            Ok(SortMode::Light)
        } else {
            Ok(SortMode::Auto)
        }
    }
}

impl std::fmt::Display for SortMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortMode::Auto => write!(f, "auto"),
            SortMode::Dark => write!(f, "dark"),
            SortMode::Light => write!(f, "light"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Palette {
    pub mode: String,
    pub wallpaper: String,
    pub primary: Vec<String>,
    pub text: Vec<String>,
    pub accents: Vec<Vec<String>>,
    pub primary_rgba: Vec<String>,
    pub text_rgba: Vec<String>,
    pub accents_rgba: Vec<Vec<String>>,
    #[serde(default = "default_is_dark")]
    pub is_dark: bool,
}

fn default_is_dark() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub image_path: String,
    pub image_checksum: String,
    pub color_profile: ColorProfile,
    pub sort_mode: SortMode,
    pub palette: Palette,
    #[serde(default)]
    pub wallset: bool,
}

#[derive(Debug, Clone)]
pub struct AppPaths {
    pub template_dir: PathBuf,
    pub output_dir: PathBuf,
    pub thumbs_dir: PathBuf,
    pub dcols_dir: PathBuf,
    pub mpc_cache_file: PathBuf,
    pub wallbash_cache_file: PathBuf,
}

impl AppPaths {
    pub fn new(output_dir_override: Option<String>) -> Result<Self, WallbashError> {
        let home_dir = dirs::home_dir().ok_or(WallbashError::HomeDirNotFound)?;
        let config_dir = dirs::config_dir()
            .map(|p| p.join("wallrust"))
            .unwrap_or_else(|| home_dir.join(".config/wallrust"));
        let cache_dir = dirs::cache_dir()
            .map(|p| p.join("wallrust"))
            .unwrap_or_else(|| home_dir.join(".cache/wallrust"));
        let template_dir = config_dir.join("templates");
        
        let thumbs_dir = cache_dir.join("thumbs");
        let dcols_dir = cache_dir.join("dcols");

        let output_dir = match output_dir_override {
            Some(dir) => PathBuf::from(
                shellexpand::full(&dir)
                    .map_err(|e| {
                        WallbashError::PathExpansion(format!("Output directory expansion failed: {}", e))
                    })?
                    .into_owned(),
            ),
            None => {
                let wd = std::env::current_dir()?;
                println!(
                    "No output directory specified. Using current directory: {}",
                    wd.display()
                );
                wd
            }
        };

        fs::create_dir_all(&config_dir)?;
        fs::create_dir_all(&cache_dir)?;
        fs::create_dir_all(&template_dir)?;

        let mpc_cache_file = cache_dir.join("wallbash.mpc");
        let wallbash_cache_file = cache_dir.join("wallbash_cache.json");

        Ok(Self {
            template_dir,
            output_dir,
            thumbs_dir,
            dcols_dir,
            mpc_cache_file,
            wallbash_cache_file,
        })
    }
    
    pub fn ensure_thumbs_dir(&self) -> Result<(), WallbashError> {
        println!("Ensuring thumbnail directory exists: {}", self.thumbs_dir.display());
        if !self.thumbs_dir.exists() {
            println!("Creating thumbnail directory");
            fs::create_dir_all(&self.thumbs_dir)?;
        } else {
            println!("Thumbnail directory already exists");
        }
        Ok(())
    }
    
    pub fn ensure_dcols_dir(&self) -> Result<(), WallbashError> {
        println!("Ensuring dcols directory exists: {}", self.dcols_dir.display());
        if !self.dcols_dir.exists() {
            println!("Creating dcols directory");
            fs::create_dir_all(&self.dcols_dir)?;
        } else {
            println!("Dcols directory already exists");
        }
        Ok(())
    }
}
