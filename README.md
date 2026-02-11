# Resume Creator

A modern, terminal-based resume builder written in Rust. Create, edit, and export professional resumes directly from your terminal with support for English and Japanese languages.

## Features

- **TUI Interface**: Navigate and edit your resume entirely in the terminal
- **Multi-format Export**: Export resumes as Markdown or JSON
- **Bilingual Support**: Full support for English and Japanese (日本語)
- **Auto-save Detection**: Know when you have unsaved changes
- **Structured Data**: Organize your resume into sections:
  - Personal Information
  - Professional Summary
  - Work Experience
  - Education
  - Skills

## Requirements

- **Rust 1.70+** (for building from source)
- **Terminal with UTF-8 support** (especially for Japanese text)
- **Modern terminal emulator** recommended (e.g., iTerm2, GNOME Terminal, Windows Terminal)

## Installation

### From Source

1. Clone or download the resume-creator repository
2. Navigate to the project directory
3. Build the release binary:
   ```bash
   cargo build --release
   ```
4. Run the application:
   ```bash
   cargo run --release
   ```

   Or execute the compiled binary directly:
   ```bash
   ./target/release/resume-creator
   ```

### Quick Start

1. **Launch the app**: Run `cargo run --release` from the project directory
2. **Create your resume**: You'll start with a blank resume
3. **Add information**: Navigate to each section and press `Enter` to edit
4. **Save your work**: Press `s` to save your resume
5. **Export**: Press `e` to export your resume as Markdown or JSON

## Navigation Guide

### Main Menu (Home Screen)

The home screen displays your main menu options:
- **Edit Personal Info** - Add your name, email, phone, location
- **Add Experience** - Create work experience entries
- **Export Resume** - Export to Markdown or JSON
- **Exit** - Quit the application

### Section Navigation

Use these keys to move between sections:
- **Tab** - Move to next section
- **Shift+Tab** - Move to previous section

Resume sections flow in this order:
1. Personal Information
2. Professional Summary
3. Work Experience
4. Education
5. Skills

## Keybindings Reference

### Navigation
| Key | Action |
|-----|--------|
| `↑` or `k` | Move selection up |
| `↓` or `j` | Move selection down |
| `Tab` | Next section |
| `Shift+Tab` | Previous section |
| `Enter` | Select/edit current item |

### Editing
| Key | Action |
|-----|--------|
| `i` | Enter edit mode |
| `Esc` | Exit edit mode (discard changes) |
| `Enter` | Save field and move to next |
| `Backspace` | Delete character |
| (Type normally) | Enter text |

### Adding/Deleting
| Key | Action |
|-----|--------|
| `a` | Add new item to current section |
| `d` | Delete selected item |

### Other Commands
| Key | Action |
|-----|--------|
| `s` | Save your resume |
| `e` | Export your resume |
| `l` | Toggle language (English ↔ Japanese) |
| `q` | Quit application |
| `y` | Confirm quit (when prompted) |
| `Esc` | Cancel quit confirmation |

*Note: Most shortcuts are disabled while editing text. Use `Esc` to exit edit mode first.*

## Resume Sections

### Personal Information
Essential contact details for your resume:
- **Full Name** (required)
- **Email** (required)
- **Phone** (optional)
- **Location** (optional)
- **Website** (optional)

### Professional Summary
A brief overview of your professional background and goals. Keep it concise (2-3 sentences).

### Work Experience
Document your career history:
- **Position** - Your job title
- **Company** - Employer name
- **Start Date** - Month and year you started
- **End Date** - Month and year you left (leave blank if current)
- **Location** - Where you worked (optional)
- **Description** - Key responsibilities and achievements

### Education
Your academic credentials:
- **Degree** - e.g., "Bachelor of Science"
- **Field of Study** - e.g., "Computer Science"
- **Institution** - School/university name
- **Graduation Date** - Month and year
- **GPA** (optional)

### Skills
Organize your abilities:
- **Skill Name** - The skill itself
- **Proficiency Level** - Beginner, Intermediate, Advanced, or Expert
- **Category** - Technical, Language, Soft Skill, or Tool

## Saving Your Resume

Your resume is automatically saved to:
```
~/.config/resume-creator/
```

Resumes are stored with unique identifiers, allowing you to create multiple resume versions. Each resume is saved as a JSON file.

## Exporting Your Resume

When you export, your resume is saved to your home directory with a timestamped filename:
```
~/resume_YourName_YYYYMMDD_HHMMSS.md
~/resume_YourName_YYYYMMDD_HHMMSS.json
```

### Markdown Export
Perfect for sharing on GitHub, LinkedIn, or as a document.

### JSON Export
Useful for programmatic processing, web applications, or data interchange.

## Language Support

### English (Default)
The application defaults to English. All menus and instructions will be in English.

### Japanese (日本語)
Press `l` to switch to Japanese. The entire interface will switch to Japanese, including:
- Menu labels
- Section titles
- Field names
- Instructions

**Note**: Your actual resume content (names, descriptions, etc.) remains as you entered it.

## Terminal Requirements for Japanese

If using Japanese language mode, ensure your terminal:
1. **Supports UTF-8 encoding** - Most modern terminals do
2. **Has Japanese font support** - Install Japanese fonts on your system
3. **Uses a monospace font** - Required for proper character alignment

### Popular Terminal Options

| Terminal | Platform | UTF-8 | Japanese Support |
|----------|----------|-------|------------------|
| GNOME Terminal | Linux | ✓ | ✓ |
| Konsole | Linux (KDE) | ✓ | ✓ |
| xterm | Unix/Linux | ✓ (if configured) | ✓ |
| iTerm2 | macOS | ✓ | ✓ |
| Alacritty | Cross-platform | ✓ | ✓ |
| Windows Terminal | Windows | ✓ | ✓ (with fonts) |

## Tips and Tricks

1. **Vim Users**: The arrow keys and `hjkl` shortcuts are provided for navigation
2. **Multiple Resumes**: Each time you launch the app, you start fresh. Resumes are saved between sessions
3. **Backup**: Your resumes are stored in `~/.config/resume-creator/` - back this up regularly
4. **Export Formats**:
   - Use **Markdown** for human-readable documents, sharing, or converting to PDF
   - Use **JSON** for parsing, importing into other tools, or programmatic use

## File Locations

| Item | Location | Format |
|------|----------|--------|
| Saved Resumes | `~/.config/resume-creator/` | JSON |
| Exported Files | `~/` (home directory) | Markdown or JSON |

## Troubleshooting

**Q: Text isn't displaying correctly**
A: Ensure your terminal supports UTF-8 encoding and has proper font support, especially for Japanese.

**Q: Changes aren't being saved**
A: Make sure to press `s` to save. The app shows "Unsaved" status on the home screen.

**Q: Export file not created**
A: Check that you have write permissions to your home directory. The export file should appear there with a timestamped name.

**Q: Japanese characters look wrong**
A: Install Japanese fonts on your system and ensure your terminal is configured to use them.

## Development

This project is built with:
- **ratatui** - Terminal UI framework
- **crossterm** - Terminal control
- **serde** - Serialization
- **chrono** - Date/time handling
- **uuid** - Unique identifiers

## License

This project is provided as-is for educational and personal use.

---

**Happy resume building! 🚀**
