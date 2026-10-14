# MHF File Editor (Rust)

A GUI editor for Monster Hunter Frontier data files.

## Supported files

- mhfdat.bin: weapons (melee/ranged), armor, items, transmog/shop, weapon upgrades, names/descriptions,
  quests (HR, GR G1-G10, beginner), sigils, sharpness, bullet sets, seasonal events, and the tables read from their
  own header slots: monster crowns, Caravan routes and gems, skill effects, Hunting Road tables, horn melodies,
  extra crafting tables, arena sets.
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
- The tabs **Skill Effects, Caravan, Hunting Road, Arena**, the **Crowns** list of the Monsters tab, the **Horn Melodies**
  list of the Weapons tab, the quest sub-tabs and the tables hosted by the Shop tab (special / premium forging, combine
  shop, tower jewels, tower seals) read their tables from header slots, never from fixed addresses. Bytes the
  documentation does not name are shown as raw integers labelled with their offset (`unk +0x0A`) or kept as they are.
  A table that cannot be read when the file is opened is reported in red on its tab (the other tabs keep working); a
  build whose header word at 0xC is not 0xBC8 (the Vita file) does not load these tabs at all.
- **Adding entries.** Where the game reads the length of a table from the file, the tab has an **Add New** (or **Add
  item** / **Add skill**) button and a **Delete last** button (only the last row can go without renumbering the rows
  after it). When the number of rows changed, the save writes the table again at the end of the file (aligned to 0x10),
  repoints every pointer to it and updates its count; a table whose length did not change is rewritten where it is.
  The save is refused, and nothing is written, when a row would end its own table:

  | Tab | Tables that can grow | Refused on save |
  | --- | --- | --- |
  | Quests | HR quests (per star tier), GR quests G1 to G10 (one tab), beginner quests | quest id 0 in an HR or GR list (ends the list) |
  | Skill Effects | effect rows | skill id above 0x216, a type 1 effect id above 0x216, rows of one skill that are not contiguous (a new row goes after the last row of its skill) |
  | Caravan | voyage routes, ranking routes, gem skills, gem skill lists | empty route title, more than 12 ranking routes in a group, ranking route without title pointer, gem skill id -1 |
  | Hunting Road | points per monster, Felyne shop, Road skills | monster id outside 1 to 176 |
  | Monsters | crowns and monster descriptions (they share one monster count, so they grow together) | only one of the two changing |
  | Weapons | horn melodies | |
  | Arena | items of the item and ammo lists | quantity 65535 (-1, ends a list) |
  | Shop | special / premium forging, combine shop, tower jewel recipes, tower seals | equip type 255, combine item id 0 or above 16700, tower seal item id 0 |

  These keep their size and are refused if forced: the Caravan level index (15) and gem tiers (9), the arena stage ids
  (28 / 34), the arena equipment blocks (17 x 4), the tutorial sets and the tower upgrade costs; the number of lists
  in a pointer array never changes.
- Where the code lives: header slots and `important_nums` offsets in `src/model/mhfdat_pointers.rs`, row structs in
  `src/model/mhfdat.rs`, reading/writing in `src/core/mhfdat.rs`, loading in `src/app/load.rs`
  (`load_inplace_tables`), saving in `src/app/save.rs` (`table_plan`), one file per tab in `src/app/`.
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
- The Hunting Road monster points layout and the monster id list come from [BlazeMH/Blaze-Hunting-Road-Editor](https://github.com/BlazeMH/Blaze-Hunting-Road-Editor) (GPL-3.0)

## Issues & contributions

- Please open issues for bugs and feature requests. Pull requests are welcome.
