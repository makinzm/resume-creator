# Japanese Language Guide / 日本語ガイド

Complete guide for using Resume Creator in Japanese.

## Overview / 概要

Resume Creator supports full Japanese language interface. You can switch between English and Japanese at any time without losing your resume data.

## Switching to Japanese / 日本語への切り替え

### Quick Start
1. Launch Resume Creator
2. Press the `l` key (lowercase L)
3. The entire interface switches to Japanese (日本語)
4. Press `l` again to switch back to English

The language setting applies only to the UI - your resume content (names, descriptions, dates) remains exactly as you entered it.

## Terminal Requirements / ターミナル要件

To properly display Japanese characters, your terminal must meet these requirements:

### UTF-8 Encoding / UTF-8エンコーディング
- Your terminal must use **UTF-8 character encoding**
- This is the default on most modern terminals
- To verify: Check terminal settings or preferences

### Japanese Font Support / 日本語フォント対応
Your terminal needs a font that includes Japanese characters (kanji, hiragana, katakana).

#### Recommended Fonts / 推奨フォント

**Linux:**
- `Noto Sans CJK JP` (recommended)
- `DejaVu Sans Mono` (with CJK support)
- `Ubuntu Mono` (basic support)
- `Liberation Mono` (basic support)

**macOS:**
- `Menlo` (has built-in Japanese support)
- `Monaco`
- `Courier New`

**Windows:**
- `Courier New` (with Japanese support enabled)
- `Consolas`
- `MS Gothic` (for terminal emulators)

#### Installing Fonts / フォントのインストール

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install fonts-noto-cjk-extra
```

**Linux (Fedora):**
```bash
sudo dnf install google-noto-sans-cjk-fonts
```

**macOS:**
```bash
brew install font-noto-sans-cjk
```

**Windows:**
- Download from [Google Fonts](https://fonts.google.com/noto)
- Install to `C:\Windows\Fonts\`

### Recommended Terminals / 推奨ターミナル

| Terminal | Platform | UTF-8 | Japanese | Notes |
|----------|----------|-------|----------|-------|
| **GNOME Terminal** | Linux | ✓ | ✓ | Excellent support |
| **Konsole** | Linux (KDE) | ✓ | ✓ | Good support |
| **iTerm2** | macOS | ✓ | ✓ | Best macOS option |
| **Alacritty** | Cross-platform | ✓ | ✓ | Modern, fast |
| **Windows Terminal** | Windows | ✓ | ✓ | Requires font setup |
| **Kitty** | Linux/macOS | ✓ | ✓ | Modern terminal |
| **Hyper** | Cross-platform | ✓ | ✓ | Electron-based |

### Configuration Examples / 設定例

#### GNOME Terminal / GNOMEターミナル
1. Open Preferences (メニュー → 設定)
2. Go to Profiles tab
3. Set Font to a CJK-compatible font (e.g., Noto Sans CJK)
4. Ensure text encoding is UTF-8

#### iTerm2 (macOS)
1. Preferences → Profiles
2. Go to Text tab
3. Select Font with CJK support (Menlo recommended)
4. Check encoding is UTF-8 (View → Encoding)

#### Windows Terminal
1. Settings → Defaults
2. Appearance section
3. Font: Set to a CJK-supporting font
4. Color scheme: Choose one you prefer

## Japanese UI Elements / 日本語UI要素

When you switch to Japanese, these elements change:

### Main Menu / メインメニュー
```
個人情報を編集 - Edit Personal Info
経歴を追加 - Add Experience
レジュメをエクスポート - Export Resume
終了 - Exit
```

### Resume Sections / レジュメセクション
```
個人情報 - Personal Information
プロフェッショナル概要 - Professional Summary
職務経歴 - Work Experience
教育 - Education
スキル - Skills
```

### Field Names / フィールド名
```
名前 - Full Name
メール - Email
電話 - Phone
所在地 - Location
会社 - Company
職位 - Position
学位 - Degree
機関 - Institution
スキル名 - Skill Name
習得度 - Proficiency Level
```

### Status Messages / ステータスメッセージ
```
保存されていない変更があります - Unsaved changes
レジュメが正常に保存されました - Resume saved successfully
```

## Using Japanese Text Input / 日本語テキスト入力

### IME (Input Method Editor) / IME（入力メソッドエディタ）

To input Japanese text while editing resume fields:

1. **Activate your system's IME**
   - Linux: Fcitx, IBus, or Uim (depending on distro)
   - macOS: Built-in Japanese input
   - Windows: Built-in IME or Google IME

2. **In Resume Creator**
   - Press `i` to enter edit mode
   - Your IME should activate automatically
   - Type in Japanese (hiragana/katakana)

3. **Converting Input**
   - Most IMEs support kanji conversion
   - Use Space or specific IME shortcut to convert
   - Press Enter to confirm and save field

### Example: Entering a Japanese Name / 日本語名を入力する例

1. Navigate to Personal Information (個人情報)
2. Press `i` to edit Full Name (名前)
3. Type in hiragana: `たなか たろう`
4. Press Space (or IME shortcut) to convert to kanji: `田中 太郎`
5. Press Enter to save

### Terminal IME Support / ターミナルIMEサポート

**Linux:**
- Fcitx (recommended for most terminals)
- IBus (GNOME, KDE)
- Uim (traditional option)

**macOS:**
- Use System Preferences → Keyboard → Input Sources
- Select Japanese input method
- Switch with Cmd+Space (or configured key)

**Windows:**
- Windows IME (default)
- Google IME (recommended)
- BAIDU Input Method (alternative)

## Resume Content in Japanese / レジュメコンテンツの日本語入力

You can enter resume data entirely in Japanese:

### Example / 例

**Personal Info (個人情報):**
- 名前: 田中太郎
- メール: tanaka@example.com
- 電話: 090-1234-5678
- 所在地: 東京都渋谷区

**Professional Summary (プロフェッショナル概要):**
```
ソフトウェアエンジニアとして10年の経験があります。
クラウドアーキテクチャとデータサイエンスを専門としています。
```

**Work Experience (職務経歴):**
- 会社: 株式会社テクノロジー
- 職位: シニアエンジニア
- 説明: Pythonを使用したML/AIプロジェクトを主導しました

## Exporting in Japanese / 日本語での書き出し

When you export a resume edited in Japanese:

### Markdown Export
- All Japanese text is preserved
- Markdown formatting remains clean
- File name includes timestamp but not Japanese characters
- Example: `resume_Taro_Tanaka_20240211_143022.md`

### JSON Export
- Complete Japanese text is preserved
- UTF-8 encoding ensures proper character storage
- Can be imported into other applications
- Example: `resume_Taro_Tanaka_20240211_143022.json`

### Tips for Exporting / エクスポートのコツ

1. **Save Before Export** - Always press `s` to save first
2. **Check Encoding** - Ensure your text editor uses UTF-8 when viewing
3. **Font Support** - When sharing, recipient's terminal must have Japanese font

## Troubleshooting / トラブルシューティング

### Problem: Japanese characters show as boxes / 日本語が四角形で表示される

**Solution:**
1. Install Japanese fonts (see "Installing Fonts" section above)
2. Configure terminal to use the installed font
3. Restart terminal and Resume Creator

### Problem: Cannot input Japanese text / 日本語が入力できない

**Solution:**
1. Verify your system IME is active
2. Check terminal emulator supports input methods
3. Try different terminal (Alacritty, iTerm2, GNOME Terminal)
4. Check if IME works in other terminal applications first

### Problem: Switching to Japanese shows incorrect characters / 日本語に切り替えるときに文字が間違っている

**Solution:**
1. Try pressing `l` multiple times to toggle languages
2. Restart the application
3. Ensure terminal encoding is UTF-8

### Problem: Export file shows garbled Japanese / エクスポート文字が文字化けしている

**Solution:**
1. Verify exported file is viewed/opened with UTF-8 encoding
2. In your text editor, set encoding to UTF-8 explicitly
3. If sharing, inform recipient to use UTF-8 encoding
4. Use JSON export instead (handles encoding better)

## Keyboard Layouts / キーボードレイアウト

Resume Creator uses only ASCII keys, so language/keyboard layout changes don't affect shortcuts:

- All shortcuts (`q`, `s`, `e`, `l`, `i`, `a`, `d`) work with standard key codes
- Switch your keyboard layout as needed without affecting shortcut functionality
- Input your resume in Japanese using your system IME, English using standard typing

### Pro Tip / プロのコツ

On Linux/macOS, you can use Ctrl+Space or Cmd+Space to toggle IME while Resume Creator is running, allowing rapid switching between Japanese input and command entry.

## Character Encoding / 文字エンコーディング

**Resume Creator uses UTF-8 throughout:**
- Resume data stored as UTF-8 JSON
- Exports maintain UTF-8 encoding
- Recommended: Always open exported files as UTF-8

If you encounter encoding issues:
1. Verify terminal uses UTF-8 (usually under Settings/Encoding)
2. Verify text editor opens files as UTF-8
3. For files received from others, explicitly set UTF-8 encoding

## System Locales / システムロケール

Japanese UI will work best if your system locale is set to Japanese, but it's not required:

**Linux:**
```bash
locale -a | grep ja_JP
# Should show: ja_JP.UTF-8
```

**macOS:**
```bash
locale -a | grep ja_JP
# Should show: ja_JP.UTF-8
```

**Windows:**
- Languages & Region → Language
- Check if Japanese is in your available languages

Not strictly necessary - Resume Creator detects terminal capabilities automatically.

## Support for Other CJK Languages / 他のCJK言語のサポート

Currently, Resume Creator officially supports:
- English (英語)
- Japanese (日本語)

Chinese (中文) and Korean (한국) are not currently supported but may be added in future versions.

---

**Happy resume building in Japanese! 頑張ってください！**

See [KEYBINDINGS.md](./KEYBINDINGS.md) and [README.md](../README.md) for more information.
