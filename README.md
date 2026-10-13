# MHF File Editor (Rust)

A GUI editor for Monster Hunter Frontier data files.

## Supported files

- mhfdat.bin: weapons (melee/ranged), armor, items, transmog/shop, weapon upgrades, names/descriptions,
  quests, sigils, sharpness, bullet sets, seasonal events, and (in-place editors) monster crowns, Caravan routes,
  skill effects, Hunting Road tables, extra crafting/weapon-parameter tables, arena sets.
- mhfjmp.bin: menu/area and strings.

## Build

Requirements: Rust (1.76+), Windows 10/11 recommended.

Steps:

1) Clone the repository
   ```sh
   git clone <https://github.com/ezemania2/mhf-file-editor>
   cd mhf-file-editor
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

### Opening files

The editor works on the **clear** file. If the file is still wrapped in ECD and/or JPK, those two wrappers (and
only those) are removed in memory; no archive format is ever guessed at (an earlier build handed every file to
rsfrontier's generic unpacker, whose NSO-archive heuristic panics on clear `mhfjmp.bin` files: issue #1).
A file that cannot be opened is refused with a message on the start screen, quoting the offsets, the sizes and the
SHA-256 of the file.

### Saving

- Saving writes into a copy of the file and replaces the original only when everything succeeded.
- Tables whose size can change (names, descriptions, shops, carve/part-break drops, ...) are rebuilt at the end of the
  file with absolute pointers and the header pointer is updated; the old copy is left where it is.
- The tabs **Crowns, Skill Effects, Caravan, Hunting Road, More Crafting, Weapon Params, Arena** and the quest
  sub-tabs **GR menus G1-G10 / Beginner quests** edit values *in place*: they rewrite the bytes of the table where it
  already is, no pointer moves and no row can be added or removed. Bytes the documentation does not name are shown as
  raw integers labelled with their offset (`unk +0x0A`). If a table would grow, or would not fit in the file, the save
  is refused with a message and nothing is written. A table that cannot be read when the file is opened is reported in
  red on its tab (the other tabs keep working); a build whose header word at 0xC is not 0xBC8 (the Vita file) does not
  load these tabs at all.
- Where the code lives: header slots and `important_nums` offsets in `src/model/mhfdat_pointers.rs`, row structs in
  `src/model/mhfdat.rs`, reading/writing in `src/core/mhfdat.rs`, loading in `src/app/load.rs`
  (`load_inplace_tables`), saving in `src/app/save.rs` (`table_patches`), one file per tab in `src/app/`.
- Pointers in the file are offsets from the start of the clear file; counts are read from `important_nums` (the block
  the first header pointer points to), never from fixed addresses.

Notes:
- **Always backup** your original files before editing.
- Files must be **decrypted** and **decompressed** before opening (Use RsFrontier for it).
- **Save**: Writes modified data to the binary file.
- **Compress** (separate button): Compresses the file with JPK Type 4.
- **Encrypt** (separate button): Encrypts the file with ECD.
- Compress and encrypt must be done **after** saving, in that order if both are needed.

## Credits

- Packing/encryption powered by RsFrontier (`rsfrontier-core`) by Pax: [Paxlord/rsfrontier](https://github.com/Paxlord/rsfrontier)
- Pattern sources by [Ezemania](https://github.com/ezemania2/mhf-patterns), [Wish](https://github.com/Mezeporta/010Templates) and [Variable](https://github.com/var-username/Monster-Hunter-Frontier-Patterns)
- Icons are taken from [Mael's](https://github.com/theMaelstro/MHFZZDatabase) repo

## Issues & contributions

- Please open issues for bugs and feature requests. Pull requests are welcome.
