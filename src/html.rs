use std::path::Path;
use std::fs;
use crate::config::Palette;

pub fn generate_html(palette: &Palette, output_path: &Path) -> anyhow::Result<()> {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Wallrust Color Palette</title>
    <style>
        :root {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            line-height: 1.6;
            color: #333;
        }}
        body {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }}
        h1, h2, h3 {{
            margin-top: 1.5em;
            margin-bottom: 0.5em;
        }}
        .container {{
            display: flex;
            flex-direction: column;
            gap: 20px;
        }}
        .info {{
            background-color:rgb(0, 0, 0);
            padding: 15px;
            border-radius: 5px;
        }}
        .palette-section {{
            margin-bottom: 30px;
        }}
        .palette {{
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
            margin-bottom: 20px;
        }}
        .color-block {{
            width: 100px;
            height: 100px;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(77, 77, 77, 0.1);
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            font-family: monospace;
            font-size: 0.8em;
            transition: transform 0.2s;
            word-break: break-all;
            padding: 5px;
            text-align: center;
            color: rgba(255, 255, 255, 0.7);
        }}
        .color-block span {{
            display: block;
            line-height: 1.2;
        }}
        .color-block:hover {{
            transform: scale(1.05);
        }}
        .primary-block {{
            width: 150px;
            height: 150px;
            font-size: 1em;
        }}
        .accent-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
            gap: 10px;
            margin-top: 10px;
        }}
        .accent-item {{
            width: 100%;
            height: 80px;
            border-radius: 5px;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            font-family: monospace;
            font-size: 0.7em;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
            word-break: break-all;
            padding: 5px;
            text-align: center;
            color: rgba(255, 255, 255, 0.7);
        }}
        .accent-item span {{
            display: block;
            line-height: 1.2;
        }}
        .color-text {{
        }}
        .light-text {{
            color: rgba(255, 255, 255, 0.9);
            text-shadow: 0 0 5px rgba(0,0,0,0.5);
        }}
    </style>
    <script>
        function getTextColor(hexcolor) {{
            const cleanHex = hexcolor.startsWith('#') ? hexcolor.slice(1) : hexcolor;
            const fullHex = cleanHex.length === 3 ? cleanHex.split('').map(char => char + char).join('') : cleanHex;

            const r = parseInt(fullHex.substr(0, 2), 16);
            const g = parseInt(fullHex.substr(2, 2), 16);
            const b = parseInt(fullHex.substr(4, 2), 16);
            const yiq = ((r * 299) + (g * 587) + (b * 114)) / 1000;
            return (yiq >= 128) ? 'rgba(0,0,0,0.7)' : 'rgba(255,255,255,0.9)';
        }}

        document.addEventListener('DOMContentLoaded', () => {{
            document.querySelectorAll('.color-block, .accent-item').forEach(block => {{
                const bgColor = block.style.backgroundColor;
                let hexColor = '#000000';
                if (bgColor.startsWith('rgb')) {{
                    const rgbaMatch = bgColor.match(/^rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*(\d*\.?\d+))?\)$/);
                    if (rgbaMatch) {{
                        const r = parseInt(rgbaMatch[1]);
                        const g = parseInt(rgbaMatch[2]);
                        const b = parseInt(rgbaMatch[3]);
                        hexColor = '#' + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1).toUpperCase();
                    }}
                }} else if (bgColor.startsWith('#')) {{
                    hexColor = bgColor;
                }}
                block.style.color = getTextColor(hexColor);
            }});
        }});
    </script>
</head>
<body>
    <div class="container">
        <h1>Wallrust Color Palette</h1>

        <div class="info">
            <p><strong>Wallpaper:</strong> {}</p>
            <p><strong>Mode:</strong> {}</p>
            <p>Color values are generated from the wallpaper using Wallrust.</p>
        </div>

        <div class="palette-section">
            <h2>Primary Colors</h2>
            <div class="palette" id="primary-palette">
                {}
            </div>

            <h2>Text Colors</h2>
            <div class="palette" id="text-palette">
                {}
            </div>
        </div>

        <div class="palette-section">
            <h2>Accent Colors</h2>
            <div class="accent-grid" id="accent-grid">
                {}
            </div>
        </div>
    </div>
</body>
</html>"#,
        palette.wallpaper,
        if palette.is_dark { "Dark" } else { "Light" },
        generate_color_blocks(&palette.primary, "primary-block"),
        generate_color_blocks(&palette.text, "color-block"),
        generate_accent_blocks(&palette.accents)
    );

    fs::write(output_path, html)?;
    Ok(())
}

fn generate_color_blocks(colors: &[String], class: &str) -> String {
    colors
        .iter()
        .map(|color| {
            format!(
                r#"<div class="{}" style="background-color: #{}">
                    <span class="color-text">#{}</span>
                    <span class="color-text">#{}</span>
                </div>"#,
                class, color, color, color
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_accent_blocks(accents: &[Vec<String>]) -> String {
    accents
        .iter()
        .flat_map(|primary_accents| {
            primary_accents.iter().map(|color| {
                format!(
                    r#"<div class="accent-item" style="background-color: #{}">
                        <span class="color-text">#{}</span>
                        <span class="color-text">#{}</span>
                    </div>"#,
                    color, color, color
                )
            })
        })
        .collect::<Vec<String>>()
        .join("\n")
} 