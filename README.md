# MHF File Editor (Rust)

A modern, user-friendly Rust GUI editor for Monster Hunter Frontier `mhfjmp.bin` and `mhfdat.bin` files.

---

## 📝 Project Overview

MHF File Editor is a graphical tool to view, edit, and save both `mhfjmp.bin` and `mhfdat.bin` files from Monster Hunter Frontier.
- **`mhfjmp.bin`**: Menu and area data (see below for details)
- **`mhfdat.bin`**: Weapon data (melee and ranged), names, descriptions, and more

It is designed for full binary compatibility with the original game formats and the Go-based tool (`mhfjmp-editor`).
All pointer logic, area/menu/string handling, weapon data, and Shift-JIS encoding are respected.

---

## ✨ Features

- **Modern UI**: Clean, tabbed interface (Menu Entries, Areas, Strings, Weapons)
- **Full Binary Compatibility**: Reads and writes `mhfjmp.bin` and `mhfdat.bin` files exactly as the game expects
- **Direct Editing**: Edit MenuEntry, Area, String, and Weapon data in-place
- **Weapon Editing** (`mhfdat.bin`):
  - View, filter, and edit all melee and ranged weapons
  - Edit weapon stats, names, and up to three descriptions per weapon
  - Live character counter for descriptions
  - Dummy weapon filter and advanced search
  - Responsive, user-friendly weapon list and detail view
- **Area/Entry Management** (`mhfjmp.bin`): Add, remove, and reorder Areas, AreaEntries, and Stage IDs
- **Pointer Logic**: All technical pointer fields are recalculated automatically on save
- **Shift-JIS Support**: All string fields (including MenuEntry Title/Description and weapon names/descriptions) are handled as Shift-JIS, supporting Japanese and special characters
- **Font Bundled**: Japanese text display is supported out-of-the-box (NotoSansCJKjp font included)
- **No CSV/Loose Export**: Strict binary workflow, no CSV import/export
- **Cross-Platform Build**: Windows 10/11 recommended, but Rust codebase is portable

---

## 🖥️ Requirements

- Rust (edition 2021 or later, recommended 2024)
- Windows 10/11 (recommended)
- [NotoSansCJKjp-Regular.otf](https://github.com/notofonts/noto-cjk/blob/main/Sans/OTF/Japanese/NotoSansCJKjp-Regular.otf) (already included in `assets/`)

---

## ⚡ Installation

1. **Clone this repository:**
   ```sh
   git clone <repo-url>
   cd MHJMP_Editor_Rust
   ```

2. **Build the project:**
   ```sh
   cargo build --release
   ```

3. **Check font:**
   - Ensure `assets/NotoSansCJKjp-Regular.otf` is present (should be by default).

---

## 🚀 Usage

1. **Run the editor:**
   ```sh
   cargo run --release
   ```
   or launch the built executable from `target/release/mhf-file-editor.exe`.

2. **Open a file:**
   - Click the **Open** button and select a `mhfjmp.bin` or `mhfdat.bin` file.

3. **Edit:**
   - For `mhfjmp.bin`: Use the **Menu Entry**, **Area Entry**, and **Strings** tabs to view and edit menu/area data.
   - For `mhfdat.bin`: Use the **Weapons** tab to view, filter, and edit melee and ranged weapons, including their stats, names, and descriptions.
     - Use the search and filter options to quickly find weapons.
     - Edit weapon names and up to three descriptions per weapon (with live character counter).
     - Toggle the dummy weapon filter to show/hide placeholder entries.

4. **Save:**
   - Click the **Save** button to write your changes back to the binary file.

5. **Notes:**
   - All string fields support Japanese and special characters (Shift-JIS).
   - All pointer fields are recalculated automatically.
   - The UI is fully resizable and supports dark mode.

---

## ⚠️ Disclaimer & Usage

**Important:**
- This tool is designed for use with decrypted Monster Hunter Frontier binary files (`mhfjmp.bin`, `mhfdat.bin`).
- You must decrypt your files before using this editor. The tool does not handle decryption.

**Usage:**
1. Decrypt your `.bin` files using an appropriate tool or method.
2. Open the decrypted file in MHF File Editor.
3. Edit menu, area, or weapon data as needed.
4. Save your changes.
   **Warning:** Changes are directly applied to the selected file. Always keep a backup of your original files.

**Legal & Warranty:**
- This software is provided as-is, without any warranty.
- Use at your own risk.
- The authors are not responsible for any data loss, corruption or other issues resulting from the use of this tool.
- This project is open-source and intended for educational and preservation purposes only.

---

## 🛠️ Technical Details

- **Binary Format:**  
  - Strictly matches the original Monster Hunter Frontier `mhfjmp.bin` and `mhfdat.bin` structures.
  - Handles all pointer fields, area/entry tables, weapon tables, and Shift-JIS string encoding.
- **Weapon Data (`mhfdat.bin`):**
  - Supports all melee and ranged weapon fields (stats, names, descriptions)
  - Robust error handling for file reading, pointer validation, and Shift-JIS decoding
  - Dummy weapon detection and filtering
- **Font:**  
  - Uses NotoSansCJKjp for full Japanese support, embedded in the binary.
- **No Console Window:**  
  - On Windows, the app launches without a console window for a clean GUI experience.
- **Performance:**  
  - Handles large files efficiently, with instant UI updates.

---

## 🧑‍💻 Development

- **Rust Edition:** 2024
- **Main dependencies:**  
  - `eframe`/`egui` for GUI
  - `encoding_rs` for Shift-JIS
  - `native-dialog` for file dialogs
  - `winit` for window management

- **Project structure:**
  - `src/app.rs` — Main UI logic
  - `src/app_mhfdat.rs` — UI logic for mhfdat weapon editing
  - `src/core/mhfdat.rs` — Binary read/write logic for mhfdat.bin
  - `src/model/mhfdat.rs` — Data structures for weapons
  - `src/binio.rs` — Binary read/write logic (legacy)
  - `src/model.rs` — Data structures (legacy)
  - `assets/` — Font and icons

---

## 🙏 Credits

- Original Go logic and binary format: [mhfjmp-editor](https://github.com/ezemania2/mhfjmp-editor)
- Japanese font: [NotoSansCJKjp](https://github.com/notofonts/noto-cjk)

---

## 📢 Issues & Contributions

- For bug reports or feature requests, please open an issue on this repository.
- Pull requests are welcome!

---

## 📄 License

Specify your license here (MIT, Apache-2.0, etc.)

---

N'hésite pas à adapter la section "Credits" et "License" selon tes besoins !
Ce README est prêt pour un projet open source professionnel. 