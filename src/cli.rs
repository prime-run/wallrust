use crate::config::{DEFAULT_COLORS, DEFAULT_FUZZ};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generates color palettes from wallpapers", long_about = None)]
pub struct Cli {
    pub input_image: Option<String>,

    #[arg(short, long)]
    pub force: bool,

    #[arg(short, long, value_name = "DIR")]
    pub output_dir: Option<String>,

    #[arg(short, long, help = "Use vibrant color profile")]
    pub vibrant: bool,

    #[arg(short, long, help = "Use pastel color profile")]
    pub pastel: bool,

    #[arg(short, long, help = "Use monochrome color profile")]
    pub mono: bool,

    #[arg(
        short,
        long,
        help = "Use custom color curve (provide curve string)",
        value_name = "CURVE"
    )]
    pub custom: Option<String>,

    // FIX: they need to make more sense! (I blame wallbash XD)
    #[arg(short, long, help = "Force dark sort mode")]
    pub dark: bool,

    #[arg(short, long, help = "Force light sort mode")]
    pub light: bool,

    //  K-Means
    #[arg(long, default_value_t = DEFAULT_COLORS, help = "Number of primary colors to extract")]
    pub colors: usize,

    #[arg(long, default_value_t = DEFAULT_FUZZ, help = "Color fuzziness percentage for k-means")]
    pub fuzz: u8,

    //NOTE:   detections
    #[arg(
        long,
        help = "Attempt to detect current Hyprland wallpaper via hyprctl"
    )]
    pub detect_hyprland: bool,

    #[arg(long, help = "Generate HTML color palette preview")]
    pub html: bool,
}
