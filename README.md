# Wallrust

A blazingly fast and feature-rich tool for image color palette extraction and theme generation, inspired by [wallbash](https://github.com/prasanthrangan/hyprdots/wiki/Wallbash/427700a4d4fa268bc7208ab273d8ea1619da97e2).

## Installation

### archlinux

`wallrust` is pushed to the arch [AUR](https://aur.archlinux.org/packages/wallrust).

use pre-built binary(recommended):

```bash
yay -Sy wallrust-bin

```

build from source:

```bash
yay -Sy wallrust
```

### cargo

```bash
cargo install wallrust
```

### download binaries from [releases](https://github.com/prime-run/wallrust/releases)

```bash
wget https://github.com/prime-run/wallrust/releases/download/v1.0.1/wallrust-1.0.1-linux-x86_64.tar.gz
tar -xvf wallrust-1.0.1-linux-x86_64.tar.gz
cp wallrust-1.0.1-linux-x86_64/wallrust ~/.local/bin/

```

make sure to add `~/.local/bin` to your `$PATH`

```bash
#bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc

#zsh
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
```

### Manual

Using makefile:

```bash
git clone https://github.com/yourusername/wallrust.git
cd wallrust
make

```

or use cargo for `debug` version:

```bash
git clone https://github.com/yourusername/wallrust.git
cd wallrust
cargo build
```

a simple [cargo-make](https://crates.io/crates/cargo-make/0.3.54) file is included.

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
    --wallset                  Generate thumbnails and dcol files compatible with wallbash scripts
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

# Generate thumbnails and hash-based dcol files (compatible with wallbash scripts)
wallrust ~/Pictures/wallpaper.jpg --wallset

# ADVANCED Custom color curve (9 points of brightness and saturation)
wallrust ~/Pictures/wallpaper.jpg --custom "10 99\n17 66\n24 49\n39 41\n51 37\n58 34\n72 30\n84 26\n99 22"
```

## Output Files

Wallrust generates these files in the output directory:

- `wallrust.dcol`: Shell variables with color values
- `wallrust.css`: CSS color variables
- `wallrust.json`: Palette data in JSON format
- Custom template outputs (if templates exist)

When using the `--wallset` flag, additional files are generated:
- Thumbnail in `~/.cache/wallrust/thumbs/{hash}.thmb`
- Hash-based dcol file in `~/.cache/wallrust/dcols/{hash}.dcol`

### Output Directory

- Default: `~/.cache/wallrust/`
- Custom: Specified with `--output-dir`
- Thumbnails and hash-based dcol files: `~/.cache/wallrust/thumbs/` and `~/.cache/wallrust/dcols/`

## Custom Templates

Create template files in `~/.config/wallrust/templates/` using [Tera](https://github.com/Keats/tera) syntax. Available template variables:

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

### Template Output Path and Backup Directives

You can control where a template's output is written and whether to back up the previous file by adding special directives at the top of your template:

- **Output Path**: Write the rendered file to a specific location (supports `~` and environment variables):
  ```
  {# output: ~/.config/kitty/colors.conf #}
  #!output: ~/.config/kitty/colors.conf
  ```
  If not specified, the output will be written to the default output directory.

- **Backup**: If set to `true`, and the output file exists, it will be backed up to `<output_path>.wr.bakup` before being overwritten:
  ```
  {# backup: true #}
  #!backup: true
  ```
  Default is `false` (no backup).

**Example at the top of a template:**
```
{# output: ~/.config/kitty/colors.conf #}
{# backup: true #}
```

## Color Profiles

- **default**: Balanced colors with moderate saturation
- **vibrant**: High saturation, vivid colors
- **pastel**: Soft, muted colors with lower saturation
- **mono**: Grayscale palette

## Wallset Mode

The `--wallset` flag enables a mode that works similarly to the wallbash scripts:

1. **Thumbnail Generation**: Creates a smaller thumbnail image optimized for color extraction
2. **Consistent File Naming**: Uses SHA256 hashing of the image path for consistent file names
3. **Directory Structure**: Organizes files in a way compatible with theme-switching scripts

This mode is ideal when:
- You need consistent color extraction between Wallrust and wallbash scripts
- You want optimized performance for large image files
- You're integrating with other tools that expect this specific format

Example with all options:
```bash
wallrust ~/Pictures/wallpaper.jpg --wallset --vibrant --dark
```

## Requirements

- ImageMagick
- Optional: hyprctl (for Hyprland wallpaper detection)
