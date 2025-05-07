<div align="center">
  <a href="https://github.com/prime-run/wallrust">
    <img src="https://github.com/user-attachments/assets/f4fe0070-c08c-4305-baf9-f1a67034aae6" alt="wallrust logo" height="100">
  </a>
</div>
<h1 align="center">Wallrust</h1>

<div align="center">	
  <p align="center">  . </p>

A blazingly fast and feature-rich tool for image color palette extraction and theme generation, inspired by <a href="[https://github.com/prime-run/wallrust](https://github.com/prasanthrangan/hyprdots/wiki/Wallbash/427700a4d4fa268bc7208ab273d8ea1619da97e2)"> wallbash </a> 
</div>



<div align="center">
<p align="center">  
<img src="https://github.com/user-attachments/assets/f2b29c74-cc45-44e2-9792-e25af45abd68"
  alt="main-togo-screen-shot"
  width="633" height="353">
</p>
</div>

<p align="center">  . </p>

## Features ✨

* **Config File generation:** Blue-print your `example.conf` file and automatically populate them with generated colors and `cp` them all in place! 
* **Visual preview:** View extracted color palettets in a generated HTML file
* **Smart caching:** smart caching and cache invalidation for optimum performance!



<h2> Installation </h2>


<h3>  Archlinux  <img src="https://skillicons.dev/icons?i=arch,&theme=dark" height="20" style="vertical-align: middle;">  </h3>

`wallrust` is pushed to the arch [AUR](https://aur.archlinux.org/packages/wallrust).



```bash
# install pre-built binary (recommended):
yay -Sy wallrust-bin

```

or 

```bash
# build from source:
yay -Sy wallrust
```
<p align="center">.</p>


<h3> Cargo <img src="https://skillicons.dev/icons?i=rust,&theme=dark" height="20" style="vertical-align: middle;">  </h3>
wallrust can be installed on your machine or can be included in your project from [crates.io](https://crates.io/crates/wallrust)

```bash
cargo install wallrust
```
<p align="center">.</p>

<h3> download binaries from [releases](https://github.com/prime-run/wallrust/releases)  📥 </h3>

```bash
wget https://github.com/prime-run/wallrust/releases/download/v1.0.4/wallrust-1.0.4-linux-x86_64.tar.gz
tar -xvf wallrust-1.0.4-linux-x86_64.tar.gz
cp wallrust-1.0.4-linux-x86_64/wallrust ~/.local/bin/

```
make sure to add `~/.local/bin` to your `$PATH`

```bash
export PATH="$HOME/.local/bin:$PATH
```

<p align="center">.</p>

<h3> Clone and build <img src="https://skillicons.dev/icons?i=github,&theme=dark" height="20" style="vertical-align: middle;"> </h3>

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

and also simple [cargo-make](https://crates.io/crates/cargo-make/0.3.54) file is included.

<h2> Usage </h2>
```bash
wallrust <OPTIONS> <IMAGE>
```

<h3> Basic Usage </h3>

```bash

# Process an image file
wallrust /path/to/image

```
By default, 3 color palette files ( `.css` , `.dcol` , `.json`) will be generated in the same directory as image. You can specify the output directory by passing `-o /some/path/` or `--output-dir /some/path/`.

> [!TIP]
> By default, Wallrust generates 4 Primary colors and 9 accent colors (shades) for each one + 4 text colors. Also dark mode is set by default.


<h3> Simple Example </h3>

```bash
wallrust ./example.png --html --vibrant --output-dir /path/to/my-colors/ --colors 6
```
It will generate `6 primary` colors + `9 accents for each` of them and save them under `/path/to/my-colors/`. And `--html` flag will generate a visual html file of generated color under the same path.



<h3>  Command-Line Options </h3> 

| **Flag**              | **Action** |
|---------------------------|--------------------------------|
|   ` -f` , ` --force `           |Force regeneration (ignore cache) |
|   `-o`, `--output-dir<DIR>`    |Set custom output directory |
|   `-v`, `--vibrant`                    |Use vibrant color profile |
|   `-p`, `--pastel`              |Use pastel color profile |
|   `-m`, `--mono`                |Use monochrome profile |
|   `-c`, `--custom <CURVE>`      |Use custom color curve |
|   `-d`, `--dark`                |Force dark sort mode |
|   `-l`, `--light`               |Force light sort mode |
|   `--html`                    |Generate HTML visualization of the color palette |
|   `--colors <N>`              |Number of primary colors [default: 4] |
|   `--fuzz  <N>`                |Color fuzziness percentage [default: 70] |
|   `--detect-hyprland`         |Detect current Hyprland wallpaper |
|   `--wallset`                 |Generate thumbnails and dcol files compatible with wallbash scripts |
|   `--no-templates`            |Skip custom template generation |
|   `-h`, `--help`                |Print help |
|   `-V`, `--version`             |Print version |









<h3>  More Examples </h3> 

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

# Skip custom template generation
wallrust ~/Pictures/wallpaper.jpg --no-templates

# ADVANCED Custom color curve (9 points of brightness and saturation)
wallrust ~/Pictures/wallpaper.jpg --custom "10 99\n17 66\n24 49\n39 41\n51 37\n58 34\n72 30\n84 26\n99 22"
```

<h3>  Output Files </h3>

Wallrust generates these files in the output directory:

- `wallrust.dcol`: Shell variables with color values
- `wallrust.css`: CSS color variables
- `wallrust.json`: Palette data in JSON format
- Custom template outputs (if templates exist)

When using the `--wallset` flag, additional files are generated:
- Thumbnail in `~/.cache/wallrust/thumbs/{hash}.thmb`
- Hash-based dcol file in `~/.cache/wallrust/dcols/{hash}.dcol`

<h3>  Output Directory </h3> 

- Default: `~/.cache/wallrust/`
- Custom: Specified with `--output-dir`
- Thumbnails and hash-based dcol files: `~/.cache/wallrust/thumbs/` and `~/.cache/wallrust/dcols/`


<h3>  Color Profiles </h3> 

- **default**: Balanced colors with moderate saturation
- **vibrant**: High saturation, vivid colors
- **pastel**: Soft, muted colors with lower saturation
- **mono**: Grayscale palette

<h3>  Wallset Mode </h3> 

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




<h2>  Custom Templates (config file blue-print) <h2>

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


## Requirements

- [ImageMagick](https://github.com/ImageMagick/ImageMagick)
- Optional: [hyprctl](https://wiki.hyprland.org/Configuring/Using-hyprctl/) (for Hyprland wallpaper detection)






## Built With 🔧

[![rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](#)


<div align="center">
  
  <strong>Share</strong>

  <a href="https://x.com/intent/tweet?hashtags=opensource%2Creadme&text=Check%20this%20out:%20Wallrust!&url=https%3A%2F%2Fgithub.com%2Fprime-run%2Fwallrust">
    <img src="https://img.shields.io/badge/Share_on_X-%23000000.svg?logo=X&logoColor=white" alt="Share on X" />
  </a>
  
</div>


![250507_19h00m54s_screenshot](https://github.com/user-attachments/assets/383d2e63-dce2-4fc0-9f69-d509abd41c85)
![250507_19h00m54s_screenshot](https://github.com/user-attachments/assets/ae25d320-d145-43f0-963d-e249dd82f97b)
![output2](https://github.com/user-attachments/assets/293f488f-a983-4e8a-ae01-5b0c5375826a)
![wallrust](https://github.com/user-attachments/assets/f2b29c74-cc45-44e2-9792-e25af45abd68)
![wallrust_logo](https://github.com/user-attachments/assets/f4fe0070-c08c-4305-baf9-f1a67034aae6)
