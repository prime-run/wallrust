# Wallrust Templating Tutorial

## What is Wallrust Templating?

Wallrust can automatically generate configuration files for your favorite apps (like terminal emulators, editors, etc.) using colors extracted from your wallpaper. This is done using a feature called "templating." You write a template (a sort of blueprint), and Wallrust fills in the colors for you.

This guide will walk you through the basics, using a simple example for the [Kitty terminal emulator](https://sw.kovidgoyal.net/kitty/).

---

## 1. The Starting Point: A Simple `kitty.conf`

Here's a very basic Kitty config file, with some recognizable options:

```conf
font_family      family="RobotoMono Nerd Font"
bold_font        auto
tab_bar_edge     bottom
background       #1a1b26
foreground       #c0caf5
color0           #15161e
color1           #f7768e
color2           #9ece6a
color3           #e0af68
color4           #7aa2f7
color5           #bb9af7
color6           #7dcfff
color7           #a9b1d6
```

---

## 2. How to Add Colors Dynamically

Wallrust can fill in the color values for you, so you don't have to edit them by hand every time your wallpaper changes. Instead of hardcoding the colors, you'll use a template file that Wallrust understands.

For now, just know that you'll write a template that looks almost like your normal config, but with special placeholders for the colors. Wallrust will replace those placeholders with real colors it extracts from your wallpaper.

---

## 3. Where to Put Your Template File

1. **Create a template file** (for example, `kitty.conf.template`).
2. **Place it in:**
   ```
   ~/.config/wallrust/templates/
   ```
   (You may need to create these folders if they don't exist.)

---

## 4. Controlling Where the Output Goes

At the top of your template file, you can tell Wallrust where to write the final config file. For example, to write to your Kitty config:

```text
{# output: ~/.config/kitty/colors.conf #}
```

This line is a special instruction for Wallrust. It is **not** part of your Kitty config, and will not appear in the final file.

- If you don't add this line, Wallrust will put the generated file in its default output directory.

---

## 5. Enabling Backups (Optional)

If you want Wallrust to make a backup of your existing config file before overwriting it, add this line at the top of your template:

```text
{# backup: true #}
```

- The backup will be saved as `colors.conf.wr.bakup` in the same directory.
- If you don't add this, no backup is made.
- **Note:** The backup option is about saving your old config before it's replaced. It does not affect how the template is filled in.

---

## 6. Example: A Kitty Template for Wallrust

Here's what your template file might look like:

```text
{# output: ~/.config/kitty/colors.conf #}
{# backup: true #}
font_family      family="RobotoMono Nerd Font"
bold_font        auto
tab_bar_edge     bottom
background       #{{ primary[0] }}
foreground       #{{ text[0] }}
color0           #{{ primary[1] }}
color1           #{{ accents[0][0] }}
color2           #{{ accents[0][1] }}
color3           #{{ accents[0][2] }}
color4           #{{ accents[0][3] }}
color5           #{{ accents[0][4] }}
color6           #{{ accents[0][5] }}
color7           #{{ accents[0][6] }}
```

- The lines starting with `{# ... #}` are special Wallrust instructions.
- The `{{ ... }}` parts are placeholders for colors. Wallrust will fill these in for you.

---

## 7. How to Use Your Template

1. **Put your template file** in `~/.config/wallrust/templates/`.
2. **Run Wallrust** on your wallpaper:
   ```bash
   wallrust /path/to/your/wallpaper.jpg
   ```
3. Wallrust will:
   - Extract colors from your wallpaper
   - Fill in your template
   - Write the result to `~/.config/kitty/colors.conf` (or wherever you specified)
   - Make a backup if you asked for one

---

## 8. Key Points to Remember

- **Templates** are blueprints for your config files, with placeholders for colors.
- **Backup** is optional and only saves your old config before it's replaced.
- **Template and backup are separate:** The template controls what the new file looks like; the backup option just saves the old file.
- You can make templates for any app, not just Kitty!
- You don't need to know any programming to get started—just copy, edit, and use the placeholders.

---

## 9. Next Steps

- Try making templates for other apps (like Alacritty, Waybar, etc.)
- Explore more advanced features in the Wallrust documentation.
- Enjoy automatic, beautiful theming that matches your wallpaper!
