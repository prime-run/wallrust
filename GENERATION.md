 # Wallrust Color Palette Generation

This document explains how Wallrust generates color palettes from wallpapers, how the palette arrays are structured, and how you can use them in Tera templates for custom theme generation.

## Overview

Wallrust extracts a set of **primary colors** from your wallpaper using k-means clustering. It then generates additional color variants for text and accents, providing a rich palette for use in themes and templates.

## Palette Structure

The generated palette is a set of arrays, each containing color values as 6-digit uppercase hex strings (e.g., `"AABBCC"`). These arrays are available as variables in Tera templates:

- `primary`: Main colors extracted from the image (length = number of colors, e.g., 4)
- `text`: Text colors derived from each primary color (length = number of colors)
- `accents`: Accent color variants for each primary color (2D array: `[primary][accent]`, usually 9 accents per primary)
- `primary_rgba`: RGBA string for each primary color (e.g., `"170,187,204,1.0"`)
- `text_rgba`: RGBA string for each text color
- `accents_rgba`: RGBA string for each accent color (2D array)
- `mode`: "dark" or "light" (auto-detected or forced)
- `wallpaper`: Path to the source image
- `is_dark`: Boolean, true if mode is dark

## Example: 4-Color Palette

Suppose you run Wallrust with `--colors 4`. The palette will look like:

```
primary = ["AABBCC", "112233", "445566", "778899"]
text =   ["223344", "334455", "556677", "8899AA"]
accents = [
  ["FF0000", "FF6666", "FF9999", "FFCCCC", "FF3333", "FF8888", "FFBBBB", "FF4444", "FFDDDD"],
  ["00FF00", "66FF66", "99FF99", "CCFFCC", "33FF33", "88FF88", "BBFFBB", "44FF44", "DDFFDD"],
  ["0000FF", "6666FF", "9999FF", "CCCCFF", "3333FF", "8888FF", "BBBBFF", "4444FF", "DDDDFF"],
  ["FFFF00", "FFFF66", "FFFF99", "FFFFCC", "FFFF33", "FFFF88", "FFFFBB", "FFFF44", "FFFFDD"]
]
primary_rgba = ["170,187,204,1.0", "17,34,51,1.0", "68,85,102,1.0", "119,136,153,1.0"]
text_rgba =    ["34,51,68,1.0", "51,68,85,1.0", "85,102,119,1.0", "136,153,170,1.0"]
accents_rgba = [
  ["255,0,0,1.0", "255,102,102,1.0", ...],
  ["0,255,0,1.0", "102,255,102,1.0", ...],
  ...
]
mode = "dark"
wallpaper = "/path/to/wallpaper.jpg"
is_dark = true
```

## Example: 2-Color Palette (JSON)

A palette exported as JSON (for 2 colors) might look like:

```
{
  "mode": "dark",
  "wallpaper": "/home/user/wall.jpg",
  "primary": ["AABBCC", "112233"],
  "text": ["223344", "334455"],
  "accents": [
    ["FF0000", "FF6666", ...],
    ["00FF00", "66FF66", ...]
  ],
  "primary_rgba": ["170,187,204,1.0", "17,34,51,1.0"],
  "text_rgba": ["34,51,68,1.0", "51,68,85,1.0"],
  "accents_rgba": [
    ["255,0,0,1.0", "255,102,102,1.0", ...],
    ["0,255,0,1.0", "102,255,102,1.0", ...]
  ],
  "is_dark": true
}
```

## How Variants Are Generated

- **Primary**: Extracted directly from the image (number = `--colors`)
- **Text**: For each primary, a contrasting color is generated for legibility
- **Accents**: For each primary, 9 accent variants are generated using color curve modulation (brightness, saturation, hue)
- **RGBA**: Each color is also available as an RGBA string for use in CSS or other formats

## Typical Array Lengths

- `primary`, `text`, `primary_rgba`, `text_rgba`: length = number of colors (e.g., 4)
- `accents`, `accents_rgba`: outer length = number of colors, inner length = 9 (number of accent variants)

## Using in Tera Templates

You can loop over these arrays in your templates:

```
{% for color in primary %}
primary{{ loop.index }} = #{{ color }}
{% endfor %}

{% for color in text %}
text{{ loop.index }} = #{{ color }}
{% endfor %}

{% for accent in accents[0] %}
accent0_{{ loop.index }} = #{{ accent }}
{% endfor %}
```

Or access specific values:

```
Main color: #{{ primary[0] }}
Text for main: #{{ text[0] }}
Accent 1 for main: #{{ accents[0][0] }}
```

You can also use RGBA variants for CSS:

```
--color-bg: rgba({{ primary_rgba[0] }});
--color-fg: rgba({{ text_rgba[0] }});
```

## Notes for Template Authors

- All color values are hex strings without the `#` (add it in your template if needed)
- Use `| default(...)` in Tera to provide fallbacks for missing values
- You can use Tera filters to convert hex to RGB, slice strings, etc.
- The number of colors and accent variants is configurable
- `mode` and `is_dark` can be used for conditional logic in your templates

For more examples, see the provided templates and the README-template.md.
