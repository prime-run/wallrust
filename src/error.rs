#[derive(Debug, thiserror::Error)]
pub enum WallbashError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ImageMagick command failed: {cmd}")]
    MagickCommand { cmd: String, stderr: String },

    #[error("Failed to parse ImageMagick output: {0}")]
    MagickParse(String),

    #[error("ImageMagick operation failed: {0}")]
    ImageMagickFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Hex color conversion error: {0}")]
    HexConversion(#[from] std::num::ParseIntError),

    #[error("Floating point conversion error: {0}")]
    FloatConversion(#[from] std::num::ParseFloatError),

    #[error("Required number of colors ({required}) not found, only found {found}")]
    NotEnoughColors { required: usize, found: usize },

    #[error("ImageMagick 'magick' command not found. Please ensure ImageMagick is installed and in your PATH.")]
    MagickNotFound,

    #[error("Failed to run command '{cmd}': {source}")]
    CommandRun { cmd: String, source: std::io::Error },

    #[error("Failed to get current wallpaper: {0}")]
    WallpaperDetection(String),

    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Template error: {0}")]
    TemplateError(#[from] tera::Error),

    #[error("Path expansion error: {0}")]
    PathExpansion(String),

    #[error("Could not determine home directory")]
    HomeDirNotFound,
}
