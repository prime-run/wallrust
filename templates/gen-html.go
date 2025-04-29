package main

import (
	"bufio"
	"flag"
	"fmt"
	"html/template"
	"os"
	"regexp"
	"strings"
)

type Color struct {
	Name string
	Hex  string
}

type TemplateData struct {
	Wallpaper string
	Mode      string
	Primary   []Color
	Text      []Color
	Accents   []Color
}

const htmlTemplate = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Wallrust Color Palette</title>
    <style>
        :root {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            line-height: 1.6;
            color: #333; 
        }
        body {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        h1, h2, h3 {
            margin-top: 1.5em;
            margin-bottom: 0.5em;
        }
        .container {
            display: flex;
            flex-direction: column;
            gap: 20px;
        }
        .info {
            background-color: #f5f5f5;
            padding: 15px;
            border-radius: 5px;
        }
        .palette-section {
            margin-bottom: 30px;
        }
        .palette {
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
            margin-bottom: 20px;
        }
        .color-block {
            width: 100px;
            height: 100px;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
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
            color: rgba(0,0,0,0.7); 
        }
         .color-block span {
            display: block;
            line-height: 1.2;
         }
        .color-block:hover {
            transform: scale(1.05);
        }
        .primary-block {
            width: 150px;
            height: 150px;
            font-size: 1em;
        }
        .accent-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
            gap: 10px;
            margin-top: 10px;
        }
        .accent-item {
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
             color: rgba(0,0,0,0.7); 
        }
         .accent-item span {
            display: block;
            line-height: 1.2;
         }
        .color-text {
            text-shadow: 0 0 5px rgba(255,255,255,0.7);
        }
         .light-text {
            color: rgba(255,255,255,0.9);
            text-shadow: 0 0 5px rgba(0,0,0,0.5);
         }
    </style>
     <script>
        function getTextColor(hexcolor) {
            const cleanHex = hexcolor.startsWith('#') ? hexcolor.slice(1) : hexcolor;
            const fullHex = cleanHex.length === 3 ? cleanHex.split('').map(char => char + char).join('') : cleanHex;

            const r = parseInt(fullHex.substr(0, 2), 16);
            const g = parseInt(fullHex.substr(2, 2), 16);
            const b = parseInt(fullHex.substr(4, 2), 16);
            const yiq = ((r * 299) + (g * 587) + (b * 114)) / 1000;
            return (yiq >= 128) ? 'rgba(0,0,0,0.7)' : 'rgba(255,255,255,0.9)';
        }

        document.addEventListener('DOMContentLoaded', () => {
            document.querySelectorAll('.color-block, .accent-item').forEach(block => {
                const bgColor = block.style.backgroundColor;
                 let hexColor = '#000000'; 
                 if (bgColor.startsWith('rgb')) {
                     const rgbaMatch = bgColor.match(/^rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*(\d*\.?\d+))?\)$/);
                     if (rgbaMatch) {
                         const r = parseInt(rgbaMatch[1]);
                         const g = parseInt(rgbaMatch[2]);
                         const b = parseInt(rgbaMatch[3]);
                         hexColor = '#' + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1).toUpperCase();
                     }
                 } else if (bgColor.startsWith('#')) {
                     hexColor = bgColor;
                 }
                block.style.color = getTextColor(hexColor);
            });
        });
    </script>
</head>
<body>
    <div class="container">
        <h1>Wallrust Color Palette</h1>

        <div class="info">
            <p><strong>Wallpaper:</strong> {{ .Wallpaper }}</p>
            <p><strong>Mode:</strong> {{ .Mode }}</p>
             <p>Color values are parsed from the provided CSS file's <code>:root</code> variables.</p>
        </div>

        <div class="palette-section">
            <h2>Primary Colors</h2>
            <div class="palette" id="primary-palette">
                {{ range .Primary }}
                <div class="color-block primary-block" style="background-color: {{ .Hex }}">
                    <span class="color-text">{{ .Name }}</span>
                    <span class="color-text">{{ .Hex }}</span>
                </div>
                {{ end }}
            </div>

            <h2>Text Colors</h2>
            <div class="palette" id="text-palette">
                {{ range .Text }}
                <div class="color-block" style="background-color: {{ .Hex }}">
                    <span class="color-text">{{ .Name }}</span>
                    <span class="color-text">{{ .Hex }}</span>
                </div>
                {{ end }}
            </div>
        </div>

        <div class="palette-section">
            <h2>Accent Colors</h2>
             <div class="accent-grid" id="accent-grid">
                {{ range .Accents }}
                <div class="accent-item" style="background-color: {{ .Hex }}">
                     <span class="color-text">{{ .Name }}</span>
                     <span class="color-text">{{ .Hex }}</span>
                </div>
                {{ end }}
             </div>
        </div>
    </div>
</body>
</html>`

var cssVarRegex = regexp.MustCompile(`--([a-zA-Z0-9_-]+)\s*:\s*([^;]+);`)

func main() {
	cssFile := flag.String("css", "", "Path to the CSS file containing the color palette")
	outputFile := flag.String("output", "palette.html", "Output HTML file name")
	flag.Parse()

	if *cssFile == "" {
		fmt.Println("Usage: go run your_script_name.go -css <path_to_css_file>")
		os.Exit(1)
	}

	colors, err := parseCSS(*cssFile)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error parsing CSS file: %v\n", err)
		os.Exit(1)
	}

	primary, text, accents := categorizeColors(colors)

	data := TemplateData{
		Wallpaper: "N/A (from CSS comments or metadata)",
		Mode:      "N/A (from CSS comments or metadata)",
		Primary:   primary,
		Text:      text,
		Accents:   accents,
	}

	tmpl, err := template.New("palette").Parse(htmlTemplate)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error parsing HTML template: %v\n", err)
		os.Exit(1)
	}

	outFile, err := os.Create(*outputFile)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error creating output file %s: %v\n", *outputFile, err)
		os.Exit(1)
	}
	defer outFile.Close()

	err = tmpl.Execute(outFile, data)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error executing template: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("Successfully generated %s\n", *outputFile)
}

func parseCSS(filePath string) (map[string]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, fmt.Errorf("could not open file: %w", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	colors := make(map[string]string)
	inRootBlock := false

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())

		if strings.Contains(line, ":root {") {
			inRootBlock = true
			continue
		}

		if inRootBlock && strings.Contains(line, "}") {
			inRootBlock = false
			break
		}

		if inRootBlock {
			matches := cssVarRegex.FindStringSubmatch(line)
			if len(matches) == 3 {
				varName := "--" + matches[1]
				hexValue := strings.TrimSpace(matches[2])

				if strings.HasPrefix(hexValue, "#") && (len(hexValue) == 7 || len(hexValue) == 4) {
					colors[varName] = hexValue
				} else {
					fmt.Fprintf(os.Stderr, "Warning: Skipping potential invalid color value '%s' for variable '%s'\n", hexValue, varName)
				}
			}
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error reading file: %w", err)
	}

	if !inRootBlock && len(colors) == 0 {
		return nil, fmt.Errorf(":root block not found or no color variables parsed")
	}

	return colors, nil
}

func categorizeColors(colors map[string]string) ([]Color, []Color, []Color) {
	var primary []Color
	var text []Color
	var accents []Color

	for name, hex := range colors {
		color := Color{Name: name, Hex: hex}
		if strings.HasPrefix(name, "--pry") || name == "--pry" {
			primary = append(primary, color)
		} else if strings.HasPrefix(name, "--txt") || name == "--txt" {
			text = append(text, color)
		} else if strings.HasPrefix(name, "--xa") {
			accents = append(accents, color)
		}
	}

	return primary, text, accents
}
