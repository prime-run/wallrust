<div align="center">
  <h1 align="center">Wallrust
    <p align="center">
    <a href="https://github.com/prime-run/wallrust">
    <img src="https://github.com/user-attachments/assets/f4fe0070-c08c-4305-baf9-f1a67034aae6" alt="wallrust logo" height="150">
  </a>
      </p>
    
  </h1>

</div>

Extract color palettes from images and **instantly rice _any_ setup or config file**. `wallrust` lets you blueprint your desired files, automatically populates them with colors and palettes, and saves them to your specified paths.

<h2>Why Wallrust?</h2>

- **Flexible Color Extraction:** Effortlessly generate beautiful palettes with defaults, or dive deep with advanced custom color curves, brightness/fuzz etc.
- **Ultimate Ricing tool :** Blueprint _any_ config file/script and orgnize them. `wallrust` automatically populates your blueprints with extracted image colors and places the generated files where you specify. And even backup previous ones. (yes! you can keep all your dotfiles in one place!)
- **Visual Preview:** Immediately view your generated color schemes on an auto-generated HTML preview page.
- **Performance:** <img src="https://skillicons.dev/icons?i=rust,&theme=dark" height="20" style="vertical-align: bottom;">

## Table of Contents

- [Installation](#installation)
  - [Arch Linux](#arch-linux)
  - [Cargo](#cargo)
  - [From Pre-built Binaries](#from-pre-built-binaries)
  - [From Source](#from-source)
- [Usage](#usage)
  - [HTML Visuals](#html-visuals)
  - [Wallset Flag](#wallset-flag)
  - [Command-Line Options](#command-line-options)
  - [Outputs](#outputs)
- [Advanced Usage](#advanced-usage)
  - [Fuzz](#fuzz)
  - [Custom Curve](#custom-curve)
- [Custom Templates (Config File Blueprint)](#custom-templates-config-file-blueprint)

<h2>Installation</h2>

### Arch Linux

`wallrust` is available on the Arch User Repository (AUR).

- **Pre-built binary (recommended):**
  ```bash
  yay -Sy wallrust-bin
  ```
- **Build from source:**
  ```bash
  yay -Sy wallrust
  ```

### Cargo

Install `wallrust` from [crates.io](https://crates.io/crates/wallrust):

```bash
cargo install wallrust
```

### From Pre-built Binaries

You can download pre-compiled binaries directly from the [releases page](https://github.com/prime-run/wallrust/releases).

For example, for version `1.0.4` on Linux x86_64:

```bash
wget https://github.com/prime-run/wallrust/releases/download/v1.0.4/wallrust-1.0.4-linux-x86_64.tar.gz
tar -xvf wallrust-1.0.4-linux-x86_64.tar.gz
sudo mv wallrust-1.0.4-linux-x86_64/wallrust /usr/local/bin/wallrust
# Ensure /usr/local/bin is in your $PATH
```

\*Note: Adjust the version and target architecture as needed.

### From Source

1.  Clone the repository:
    ```bash
    git clone https://github.com/prime-run/wallrust.git
    cd wallrust
    ```
2.  Build and install using:

    - **Makefile (for release build):**

      ```bash
      make install # Installs to ~/.cargo/bin by default
      ```

    - **Cargo (for debug build):**
      ```bash
      cargo build
      ```
    - **Cargo Make:**
      A `cargo-make.toml` file is also included for more complex build tasks if needed.
      ```bash
      cargo make --list # To see available tasks
      cargo make build # Example task
      ```

<h2>Usage</h2>

```bash
wallrust <OPTIONS> <IMAGE>
#
wallrust /path/to/image

```

By default, 3 color palette files ( `.css` , `.dcol` , `.json`) will be generated in the same directory as image. You can specify the output directory by passing `-o /some/path/` or `--output-dir /some/path/`.

> [!TIP]
> By default, Wallrust generates 4 Primary colors and 4 text colors to go with them. And also 9 accent colors (shades) for each primary. Also dark mode is set by default.

#### HTLM visuals

```bash
wallrust ./example.png --html --output-dir /path/to/my-dir/ --wallset --colors 4
```

here is an [example HTML](./examples/wallrust.html) file generated from [this image](https://github.com/user-attachments/assets/293f488f-a983-4e8a-ae01-5b0c5375826a) (with `--wallset` flag)

<div align="center">
<p align="center">  
<img src="https://github.com/user-attachments/assets/ae25d320-d145-43f0-963d-e249dd82f97b"
  alt="main-togo-screen-shot"
  width="480">
</p>
</div>

<h4>Wallset flag</h4>

Extracts color palettes that closely match the overall average colors of your wallpaper, making the generated palette more faithful to what you actually see on your desktop. Additionally, this mode outputs and caches a `.dcol` file compatible with theme-switching scripts.

\*Inspired by [wallbash](https://github.com/prasanthrangan/hyprdots/wiki/Wallbash/427700a4d4fa268bc7208ab273d8ea1619da97e2)

> [!NOTE]
> Technically, this mode analyzes a scaled-down version of your image and uses a hash-based filename for consistent results.

and more examples!

```bash
wallrust ~/Pictures/wallpaper.jpg --vibrant


# Extract 6 colors instead of default 4 <colors> Uint
wallrust ~/Pictures/wallpaper.jpg --colors 6

# Force regeneration (ignore cache)
# not recommended caching engine here is  smart! and reliable
wallrust ~/Pictures/wallpaper.jpg --force

# Generate thumbnails and hash-based dcol files
wallrust ~/Pictures/wallpaper.jpg --wallset

# Skip custom template (blueprint) generation
wallrust ~/Pictures/wallpaper.jpg --no-templates

# ADVANCED Custom color curve (9 points of brightness and saturation)
wallrust ~/Pictures/wallpaper.jpg --custom "10 99\n17 66\n24 49\n39 41\n51 37\n58 34\n72 30\n84 26\n99 22"
```

<h3>Command-Line Options</h3>

| **Flag**                   | **Action**                                                          |
| -------------------------- | ------------------------------------------------------------------- |
| `-f`, `--force`            | Force regeneration (ignore cache)                                   |
| `-o`, `--output-dir <DIR>` | Set custom output directory                                         |
| `--html`                   | Generate HTML visualization of the color palette                    |
| `--colors <N>`             | Number of primary colors [default: 4]                               |
| `--fuzz <N>`               | Color fuzziness percentage [default: 70]                            |
| `--detect-hyprland`        | Detect current Hyprland wallpaper                                   |
| `--wallset`                | Generate thumbnails and dcol files compatible with wallbash scripts |
| `--no-templates`           | Skip custom template generation                                     |
| `-v`, `--vibrant`          | Use vibrant color profile                                           |
| `-p`, `--pastel`           | Use pastel color profile                                            |
| `-m`, `--mono`             | Use monochrome profile                                              |
| `-c`, `--custom <CURVE>`   | Use custom color curve                                              |
| `-d`, `--dark`             | Force dark sort mode                                                |
| `-l`, `--light`            | Force light sort mode                                               |
| `-h`, `--help`             | Print help                                                          |

<h3>Outputs</h3>

By default Wallrust generates these files in the output directory:

- `wallrust.dcol`: Shell variables with color values
- `wallrust.css`: CSS color variables
- `wallrust.json`: Palette data in JSON format
- Custom template outputs (if path is not set in templates)

When using the `--wallset` flag, additional files are generated:

- Thumbnail in `~/.cache/wallrust/thumbs/{hash}.thmb`
- Hash-based dcol file in `~/.cache/wallrust/dcols/{hash}.dcol`

<h2>Advanced Usage</h2>

<h3>fuzz</h3>

The `--fuzz <N>` flag controls how much Wallrust groups similar colors when extracting your palette.

- **Higher values** (e.g., 90) group more similar colors together, resulting in a simpler palette with fewer, broader color groups.
- **Lower values** (e.g., 30) keep more subtle color differences, resulting in a more detailed palette with more unique colors.
- **Default:** `70` (good for most images)

**Examples:**

```bash
wallrust my.jpg --fuzz 90   # Fewer, broader color groups
wallrust my.jpg --fuzz 30   # More unique, detailed colors
```

[ImageMagick: Color Quantization and Fuzz](https://imagemagick.org/script/quantize.php)

<h3>custom curve</h3>

The `--custom` flag lets you pass your own `color curve`; A sequence of points, each written as `<brightness> <saturation>`, one per line. we smoothly interpolates between these points to create your palette.

- **Brightness:** 0 (darkest) to 100 (brightest)
- **Saturation:** 0 (gray) to 100 (most vivid)

```bash

#  built-in Mono Profile's custom curve:
wallrust my.jpg --custom "0 0\n100 0"


# below curve starts with a dark, slightly muted color (10 30), moves to a mid-brightness,
# even more  muted color (50 20), and ends with a very bright, very muted (almost gray) color (90 10).
# The result is a set of soft, pastel-like colors with low saturation throughout the brightness range
# (built-in pastel profile).

wallrust my.jpg --custom "10 30\n50 20\n90 10"


# Vivid to Muted Fade
# Starts vivid, fades to muted as brightness increases
wallrust my.jpg --custom "10 100\n50 60\n90 10"

```

---

<h2>Custom Templates (config file blueprint)</h2>

We use [Tera](https://github.com/Keats/tera) syntax in templates (blueprints). If you have no idea what that is, Don't worry you don't have to! just make sure you read [wallrust-template-toturial](./docs/WALLRUST_TEMPLATING_TUTORIAL.md) first.

Create template files in `~/.config/wallrust/templates/mytemplate.file` using [Tera](https://github.com/Keats/tera) syntax.

Available template variables:

| Variable       | Description                                    |
| -------------- | ---------------------------------------------- |
| `mode`         | `"dark"` or `"light"`                          |
| `wallpaper`    | Path to target wallpaper/image                 |
| `primary`      | Array of primary hex colors                    |
| `text`         | Array of text hex colors                       |
| `accents`      | 2D array of accent hex colors by primary index |
| `primary_rgba` | RGBA versions of primary colors                |
| `text_rgba`    | RGBA versions of text colors                   |
| `accents_rgba` | RGBA versions of accent colors                 |

<h3>Template Output Path and Backup Directives</h3>

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

For more information on templates, checkout [Templating](./docs/Templating.md).

<h2>Requirements</h2>

- [ImageMagick](https://github.com/ImageMagick/ImageMagick)
- Optional: [hyprctl](https://wiki.hyprland.org) (for Hyprland wallpaper detection)

<h2>Contributing</h2>

Contributions are welcome! Whether it's bug reports, feature suggestions, or code contributions, please feel free to open an issue or submit a PR.

Please ensure that any code contributions follow the existing style.

#### License

This project is licensed under MIT - see the [LICENSE](LICENSE) file for details.
