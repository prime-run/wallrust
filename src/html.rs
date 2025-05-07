use crate::config::Palette;
use std::fs;
use std::path::Path;

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
            /* font-family intentionally omitted as per user instruction */
            line-height: 1.6;
            color: #e1e1e1;
            --primary-bg: #121212;
            --card-bg: #1e1e1e;
            --hover-bg: #2a2a2a;
            --border-color: #333;
            --accent-color: #6c5ce7;
            --text-primary: #e1e1e1;
            --text-secondary: #b0b0b0;
        }}
        body {{
            background-color: var(--primary-bg);
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }}
        h1, h2, h3 {{
            margin-top: 1.5em;
            margin-bottom: 0.5em;
            color: #fff;
        }}
        h1 {{
            font-size: 2.2rem;
            letter-spacing: -0.5px;
            background: linear-gradient(90deg, var(--accent-color), #a29bfe);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            text-align: center;
            margin-bottom: 1.5rem;
        }}
        h2 {{
            font-size: 1.6rem;
            position: relative;
            padding-bottom: 0.5rem;
        }}
        h2::after {{
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            width: 60px;
            height: 3px;
            background: linear-gradient(90deg, var(--accent-color), transparent);
            border-radius: 3px;
        }}
        .container {{
            display: flex;
            flex-direction: column;
            gap: 30px;
        }}
        .info {{
            background-color: var(--card-bg);
            padding: 20px;
            border-radius: 12px;
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
            border: 1px solid var(--border-color);
        }}
        .info p {{
            margin: 0.5em 0;
        }}
        .info strong {{
            color: #fff;
        }}
        .palette-section {{
            background-color: var(--card-bg);
            padding: 25px;
            border-radius: 12px;
            margin-bottom: 30px;
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
            border: 1px solid var(--border-color);
        }}
        .palette {{
            display: flex;
            flex-wrap: wrap;
            gap: 15px;
            margin-bottom: 30px;
        }}
        .color-block {{
            width: 110px;
            height: 110px;
            border-radius: 12px;
            box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            font-family: 'Fira Code', monospace;
            font-size: 0.8em;
            transition: all 0.3s ease;
            word-break: break-all;
            padding: 5px;
            text-align: center;
            position: relative;
            backdrop-filter: blur(5px);
        }}
        .color-block span {{
            display: block;
            line-height: 1.3;
            max-width: 100%;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }}
        .color-block:hover {{
            transform: translateY(-5px) scale(1.05);
            z-index: 1;
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
        }}
        .primary-block {{
            border-radius: 8px;
            width: 160px;
            height: 160px;
            font-size: 1em;
            padding: 15px;
            display: flex ;
            flex-direction: column;
            align-items: center;
        }}
        .accent-group {{
            margin-bottom: 2em;
        }}
        .accent-group-title {{
            font-size: 1.2em;
            font-weight: bold;
            margin-bottom: 0.5em;
            color: #fff;
        }}
        .accent-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
            gap: 15px;
            margin-top: 15px;
        }}
        .accent-item {{
            width: 100%;
            height: 90px;
            border-radius: 12px;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            font-family: 'Fira Code', monospace;
            font-size: 0.75em;
            box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
            word-break: break-all;
            padding: 5px;
            text-align: center;
            position: relative;
            transition: all 0.3s ease;
        }}
        .accent-item span {{
            display: block;
            line-height: 1.3;
            max-width: 100%;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }}
        .accent-item:hover {{
            transform: translateY(-5px) scale(1.05);
            z-index: 1;
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
        }}
        .color-text {{
            text-shadow: 0 0 5px rgba(255,255,255,0.7);
        }}
        .light-text {{
            color: rgba(255,255,255,0.9);
            text-shadow: 0 0 5px rgba(0,0,0,0.5);
        }}
        .color-name {{
            font-weight: bold;
            margin-bottom: 4px;
        }}
        .color-value {{
            font-size: 0.9em;
            opacity: 0.9;
            padding: 4px 8px;
            background: rgba(0,0,0,0.2);
            border-radius: 4px;
            margin-top: 5px;
        }}
        
        /* Dark mode header */
        header {{
            text-align: center;
            margin-bottom: 2rem;
            padding: 1rem;
            border-bottom: 1px solid var(--border-color);
        }}
        
        .section-title {{
            display: flex;
            align-items: center;
            gap: 10px;
            margin-bottom: 20px;
        }}
        
        .section-title::before {{
            content: "";
            width: 15px;
            height: 15px;
            background-color: var(--accent-color);
            border-radius: 50%;
        }}
        
        /* Responsive */
        @media (max-width: 768px) {{
            .palette {{
                justify-content: center;
            }}
            
            h1 {{
                font-size: 1.8rem;
            }}
        }}
        
        /* Add cool animation effects */
        .color-block, .accent-item {{
            animation: fadeIn 0.6s ease-out;
        }}
        
        @keyframes fadeIn {{
            from {{ opacity: 0; transform: translateY(20px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}
        
        /* Add subtle gradient overlays */
        .color-block::after, .accent-item::after {{
            content: "";
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            border-radius: inherit;
            background: linear-gradient(135deg, rgba(255,255,255,0.1) 0%, rgba(255,255,255,0) 50%, rgba(0,0,0,0.1) 100%);
            pointer-events: none;
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
            return (yiq >= 128) ? 'rgba(0,0,0,0.8)' : 'rgba(255,255,255,0.95)';
        }}

        document.addEventListener('DOMContentLoaded', () => {{
            // Add staggered animation delay to color blocks
            document.querySelectorAll('.color-block, .accent-item').forEach((block, index) => {{
                block.style.animationDelay = `${{index * 0.05}}s`;
                
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
                const textColor = getTextColor(hexColor);
                block.style.color = textColor;
            }});
        }});
    </script>
</head>
<body>
    <div class="container">
        <header>
            <h1>Wallrust theme engine </h1>
            <h5> written in rust </h5>
        </header>

        <div class="info">
            <p><strong>Image:</strong> {} </p>
            <p><strong>Mode:</strong> {} </p>
            <p><strong>Number of Primary Colors:</strong> {} </p>
            <p>Color values are generated from the Image using <a href="https://github.com/prime-run/wallrust">Wallrust.</a></p>
        </div>

        <div class="palette-section">
            <div class="section-title">
                <h2>Primary Colors</h2>
            </div>
            <div class="palette" id="primary-palette">
                {}
            </div>

            <div class="section-title">
                <h2>Text Colors</h2>
            </div>
            <div class="palette" id="text-palette">
                {}
            </div>
        </div>

        <div class="palette-section">
            <div class="section-title">
                <h2>Accent Colors</h2>
            </div>
            {}
        </div>
    </div>
</body>
</html>"#,
        palette.wallpaper,
        if palette.is_dark { "Dark" } else { "Light" },
        palette.primary.len(),
        generate_color_blocks(&palette.primary, "primary-block", "Primary"),
        generate_color_blocks(&palette.text, "color-block", "Text"),
        generate_grouped_accent_blocks(&palette.accents)
    );

    fs::write(output_path, html)?;
    Ok(())
}

fn generate_color_blocks(colors: &[String], class: &str, prefix: &str) -> String {
    colors
        .iter()
        .enumerate()
        .map(|(i, color)| {
            let title = format!("#{} - {} {}", color, prefix, i + 1);
            format!(
                r#"<div class="{}" style="background-color: #{}" title="{}">
                    <span class="color-name ">{} {}</span>
                    <span class="color-value ">#{}</span>
                </div>"#,
                class,
                color,
                title,
                prefix,
                i + 1,
                color
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate_grouped_accent_blocks(accents: &[Vec<String>]) -> String {
    accents
        .iter()
        .enumerate()
        .map(|(primary_idx, primary_accents)| {
            let group_title = format!(
                "<div class=\"accent-group\"><div class=\"accent-group-title\">Accents for Primary {}</div>",
                primary_idx + 1
            );
            let items = primary_accents
                .iter()
                .enumerate()
                .map(|(accent_idx, color)| {
                    let accent_name = format!("Accent {}-{}", primary_idx + 1, accent_idx + 1);
                    format!(
                        "<div class=\"accent-item\" style=\"background-color: #{}\" title=\"#{} - {}\"><span class=\"color-name\">{}</span><span class=\"color-value\">#{}</span></div>",
                        color, color, accent_name, accent_name, color
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");
            format!("{}<div class=\"accent-grid\">{}</div></div>", group_title, items)
        })
        .collect::<Vec<String>>()
        .join("\n")
}
