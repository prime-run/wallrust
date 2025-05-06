# Tera Templates for Wallrust

These example templates show how to use Wallrust's Tera templating system to generate custom theme files for various applications.

## How to Use These Templates

1. **Copy to Templates Directory**:
   ```bash
   # Create the templates directory if it doesn't exist
   mkdir -p ~/.config/wallrust/templates/
   
   # Copy the templates you want to use
   cp kitty-theme.conf ~/.config/wallrust/templates/
   cp polybar-colors.ini ~/.config/wallrust/templates/
   cp theme.css ~/.config/wallrust/templates/
   ```

2. **Run Wallrust**:
   ```bash
   # Run with default options
   wallrust ~/path/to/wallpaper.jpg
   
   # Or with specific options
   wallrust ~/path/to/wallpaper.jpg --vibrant --colors 6
   ```

3. **Use Generated Theme Files**:
   The processed template files will be created in your output directory (default: current directory or specified with `--output-dir`).

## Available Template Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `mode` | "dark" or "light" | `{{ mode }}` |
| `wallpaper` | Path to wallpaper | `{{ wallpaper }}` |
| `primary` | Array of primary hex colors | `{{ primary[0] }}` |
| `text` | Array of text hex colors | `{{ text[0] }}` |
| `accents` | 2D array of accent colors | `{{ accents[0][3] }}` |
| `primary_rgba` | RGBA versions of primary colors | `{{ primary_rgba[0] }}` |
| `text_rgba` | RGBA versions of text colors | `{{ text_rgba[0] }}` |
| `accents_rgba` | RGBA versions of accent colors | `{{ accents_rgba[0][0] }}` |

## Template Features and Examples

### Loops
```
{% for color in primary %}
color{{ loop.index }} = #{{ color }}
{% endfor %}
```

### Conditionals
```
{% if mode == "dark" %}
background = #000000
{% else %}
background = #FFFFFF
{% endif %}
```

### Filters
```
{{ wallpaper | split(pat="/") | last }}  <!-- Gets filename from path -->
{{ color | slice(start=0, end=2) | int(base=16) }}  <!-- Converts hex to decimal -->
```

### Default Values
```
{{ primary[1] | default(accents[0][0]) }}  <!-- Uses accent if primary[1] doesn't exist -->
```

## Creating Your Own Templates

1. Create a new file in `~/.config/wallrust/templates/` with any name
2. Use the examples here as a guide for Tera syntax
3. Design your template to match your application's configuration format
4. Run Wallrust to generate all themes automatically 