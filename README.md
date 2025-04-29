# Wallrust

A Rust implementation of wallpaper color palette extraction and theme generation, inspired by wallbash.

## Installation

```bash
git clone https://github.com/yourusername/wallrust.git
cd wallrust

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Process an image file
wallrust /path/to/wallpaper.[png , jpg , gif ]

# Auto-detect current Hyprland wallpaper
wallrust --detect-hyprland
```

### Command-Line Options

```bash
USAGE:
    wallrust [OPTIONS] [INPUT_IMAGE]

ARGS:
    <INPUT_IMAGE>    Path to the input wallpaper image

OPTIONS:
    -f, --force                Force regeneration (ignore cache)
    -o, --output-dir <DIR>     Set custom output directory
    -v, --vibrant              Use vibrant color profile
    -p, --pastel               Use pastel color profile
    -m, --mono                 Use monochrome profile
    -c, --custom <CURVE>       Use custom color curve
    -d, --dark                 Force dark sort mode
    -l, --light                Force light sort mode
    --html                     Generate HTML visualization of the color palette
    --colors <N>               Number of primary colors [default: 4]
    --fuzz <N>                 Color fuzziness percentage [default: 70]
    --detect-hyprland          Detect current Hyprland wallpaper
    -h, --help                 Print help
    -V, --version              Print version
```

### Examples

pallettes: `vibrant`, `pastel`, `mono`, `dark`, `light` , `custom` `colors <int>`

```bash

wallrust ~/Pictures/wallpaper.jpg --<pallette>

wallrust ~/Pictures/wallpaper.jpg --vibrant


# Extract 6 colors instead of default 4 <colors> Uint
wallrust ~/Pictures/wallpaper.jpg --colors 6

# Set custom output directory
wallrust ~/Pictures/wallpaper.jpg --output-dir ~/themes

# Force regeneration (ignore cache) (not recommended caching engine here is  smart! and reliable)
wallrust ~/Pictures/wallpaper.jpg --force

# ADVANCED Custom color curve (9 points of brightness and saturation)
wallrust ~/Pictures/wallpaper.jpg --custom "10 99\n17 66\n24 49\n39 41\n51 37\n58 34\n72 30\n84 26\n99 22"
```

## Output Files

Wallrust generates these files in the output directory:

- `wallbash.dcol`: Shell variables with color values
- `wallbash.css`: CSS color variables
- `wallbash.json`: Palette data in JSON format
- Custom template outputs (if templates exist)

### Output Directory

- Default: `~/.cache/wallbash/`
- Custom: Specified with `--output-dir`

## Custom Templates

Create template files in `~/.config/wallbash/templates/` using [Tera](https://github.com/Keats/tera) syntax. Available template variables:

```
mode        - "dark" or "light"
wallpaper   - Path to wallpaper
primary     - Array of primary hex colors
text        - Array of text hex colors
accents     - 2D array of accent hex colors by primary index
primary_rgba - RGBA versions of primary colors
text_rgba   - RGBA versions of text colors
accents_rgba - RGBA versions of accent colors
```

## Color Profiles

- **default**: Balanced colors with moderate saturation
- **vibrant**: High saturation, vivid colors
- **pastel**: Soft, muted colors with lower saturation
- **mono**: Grayscale palette

## Requirements

- ImageMagick
- Optional: hyprctl (for Hyprland wallpaper detection)
