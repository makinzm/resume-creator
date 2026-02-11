# Keybindings Reference

Complete keybinding guide for Resume Creator.

## Quick Reference

| Context | Key | Action |
|---------|-----|--------|
| **Normal Mode** |
| | `↑` or `k` | Move up |
| | `↓` or `j` | Move down |
| | `Enter` | Select/edit |
| | `Tab` | Next section |
| | `Shift+Tab` | Previous section |
| | `a` | Add item |
| | `d` | Delete item |
| | `i` | Enter edit mode |
| | `s` | Save resume |
| | `e` | Export resume |
| | `l` | Toggle language |
| | `q` | Quit |
| **Edit Mode** |
| | `Backspace` | Delete character |
| | `Enter` | Save field & next |
| | `Esc` | Cancel & discard |
| | (text) | Type normally |
| **Confirmation** |
| | `y` | Confirm action |
| | `Esc` or `n` | Cancel |

## Detailed Guide

### Navigation (Normal Mode Only)

#### Movement
```
↑ or k     - Move selection up
↓ or j     - Move selection down
```

These work in:
- Menu lists (home screen)
- Item lists (experience, education, skills)
- Selection menus (language, export format)

#### Section Navigation
```
Tab        - Move to next resume section
Shift+Tab  - Move to previous resume section
```

Moves through sections in order:
1. Home → Personal Info
2. Personal → Summary
3. Summary → Experience
4. Experience → Education
5. Education → Skills
6. Skills → Home (wraps around)

### Selection and Editing

#### Selection
```
Enter      - Open/edit selected item
```

In the home menu, `Enter` opens the selected section.
In other sections, `Enter` starts editing that item.

#### Adding Items
```
a          - Add new item to current section
```

Works in:
- Experience (adds new job entry)
- Education (adds new degree entry)
- Skills (adds new skill entry)

Does nothing in Personal Info or Summary (single items).

#### Deleting Items
```
d          - Delete selected item
```

Works in:
- Experience (removes job entry)
- Education (removes degree entry)
- Skills (removes skill entry)

Does nothing in Personal Info or Summary.

### Editing Mode

When editing a field, you enter edit mode. Press `i` to toggle or use `Enter` while on an item.

#### Text Editing
```
(any text) - Type the text
Backspace  - Delete previous character
```

Edit mode allows you to:
- Enter/modify field values
- Navigate through multiple fields in a section
- See your input in real-time

#### Exiting Edit Mode
```
Esc        - Cancel editing (discard changes)
Enter      - Save current field and move to next field
```

When you press `Enter` in edit mode:
- The current field is saved
- You move to the next field in the section
- If it's the last field, editing mode exits

### Application Controls

#### Saving
```
s          - Save your entire resume
```

Saves your resume to `~/.config/resume-creator/` as a JSON file.
Shows a status message confirming the save.

#### Exporting
```
e          - Open export menu
```

Opens the export screen where you can:
1. Select format (Markdown or JSON)
2. Press `Enter` to export

Exports to your home directory with a timestamped filename.

#### Language Toggle
```
l          - Switch between English and Japanese
```

Toggles the UI language:
- English → 日本語
- 日本語 → English

Your resume content remains unchanged.

#### Quitting
```
q          - Quit the application
```

If you have unsaved changes:
- Shows a confirmation prompt
- Press `y` to quit without saving
- Press `Esc` to cancel and return to editing

If no unsaved changes:
- Application closes immediately

### Confirmation Dialogs

When prompted for confirmation (quit without save, delete item):

```
y          - Yes, confirm
Esc or n   - No, cancel
```

## Context-Sensitive Behavior

Not all keys work in all contexts. Resume Creator disables navigation shortcuts while editing to prevent accidental section changes.

### Available in Normal Mode
- `↑/k/↓/j` - Movement
- `Tab/Shift+Tab` - Section navigation
- `Enter` - Select
- `a/d` - Add/delete
- `i` - Toggle edit
- `s/e/l/q` - Application controls

### Available in Edit Mode
- `Backspace` - Delete
- `Enter` - Save & advance
- `Esc` - Cancel
- (Text input) - Type text

### Disabled in Edit Mode
- `↑/k/↓/j` - Movement blocked
- `Tab/Shift+Tab` - Section nav blocked
- `a/d` - Add/delete blocked
- `s/e/l/q` - App controls blocked

This prevents accidents while you're typing.

## Examples

### Add a Work Experience Entry

1. Navigate to Work Experience section
   - Press `Tab` until you reach Work Experience
2. Add a new job
   - Press `a`
3. Edit the company name
   - Press `i` (or wait, it auto-enters edit mode)
   - Type company name
   - Press `Enter`
4. Edit position
   - Type position name
   - Press `Enter`
5. Continue editing other fields
   - Dates, location, description
   - Press `Enter` after each field
6. Save when done
   - Press `s` to save entire resume

### Export Your Resume

1. Press `e` for export menu
2. Use `↑/↓` to select format
   - Markdown (.md)
   - JSON (.json)
3. Press `Enter` to export
4. Check your home directory for the file
   - File: `~/resume_YourName_YYYYMMDD_HHMMSS.md`

### Switch to Japanese

1. Press `l` to toggle language
2. The entire interface switches to Japanese
3. All menus and labels now display in Japanese
4. Press `l` again to switch back to English

## Tips

- **Vim Users**: You can use `hjkl` navigation (j=down, k=up, h/l usually don't do anything)
- **Mouse**: The terminal UI does not support mouse input; use keyboard only
- **Auto-Save**: There's no auto-save; always press `s` to save
- **Backspace Behavior**: Works as expected - deletes the character before the cursor
- **Field Navigation**: In edit mode, `Enter` moves to the next field, not to the next item

## Accessibility

If you have difficulty with certain key combinations:
- **Shift+Tab**: Some terminals may not support this. It's used for previous section only; you can wrap around using Tab
- **Alt/Ctrl Combinations**: Currently not used; all shortcuts use single keys
- **Case Sensitivity**: Commands are lowercase only (`q`, not `Q`)

## Troubleshooting

**Keys not working**
- Make sure you're not in edit mode (try pressing `Esc`)
- Some terminal emulators don't properly handle certain key combinations

**Shift+Tab not working**
- Some terminals don't support Shift+Tab. Use `Tab` repeatedly to cycle forward

**Can't exit edit mode**
- Press `Esc` to discard changes and exit
- Or press `Enter` repeatedly until you exit

---

See [README.md](../README.md) for the main documentation.
