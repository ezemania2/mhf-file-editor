# MHF File Editor (Rust)

A GUI editor for Monster Hunter Frontier data files.

## Supported files

- mhfdat.bin: weapons (melee/ranged), armor, items, transmog/shop, weapon upgrades, names/descriptions.
- mhfjmp.bin: menu/area and strings.

## Build

Requirements: Rust (1.76+), Windows 10/11 recommended.

Steps:

1) Clone the repository
   ```sh
   git clone <https://github.com/ezemania2/mhf-file-editor>
   cd MHJMP_Editor_Rust/mhf-file-editor
   ```
2) Build a release
   ```sh
   cargo build --release
   ```

## Run

From sources:
```sh
cargo run --release
```
Or run the built executable from `target/release/mhf-file-editor.exe`.

## Usage (basics)

1) Open a supported file (typically `mhfdat.bin`).
2) Navigate with the left tabs (Weapons, Armor, Items, Shop).
3) Select an entry by its ID (IDs start at 0) to view and edit.
4) Save to write changes. Use “Save (Pack + Encrypt)” to JPK-compress and optionally ECD-encrypt the output.

Notes:
- Keep a backup of your original files.
- The editor works on decrypted data; “Save (Pack + Encrypt)” re-applies packing/encryption if needed.

## Credits

- Packing/encryption powered by RsFrontier (`rsfrontier-core`) by Pax: [Paxlord/rsfrontier](https://github.com/Paxlord/rsfrontier)
- Pattern sources by [Ezemania](https://github.com/ezemania2/mhf-patterns), [Wish](https://github.com/Mezeporta/010Templates) and [Variable](https://github.com/var-username/Monster-Hunter-Frontier-Patterns)

## Issues & contributions

- Please open issues for bugs and feature requests. Pull requests are welcome.

## License

Specify your license (e.g. MIT, Apache-2.0).