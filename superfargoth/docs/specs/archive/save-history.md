# Saves: history

Moved out of [save.md](../save.md) on 2026-10-25 (doc diet, ROADMAP 0.4), word for word: the
2026-10-22 research and crate survey, the industry comparison with its sources, the S1/S2 as-built
log with the budget measurements, and the full game-load note (save.md keeps a condensed copy).
save.md and the `sf_save*` code win where they differ. Headings are unchanged, so older references
(code comments such as `save.md "As built"`, budgets) resolve here.

## Research (2026-10-22)

- **Bethesda (Skyrim, Fallout 4).** Header (screenshot, name, location, plugin list), global tables
  (globals, weather, created objects, Papyrus VM state) and **change forms**: per changed form, a form
  id, a **change-flags bitmask** naming the changed parts, a per-form version and only the changed data,
  applied over the plugins' records (UESP; form version 74 in LE 1.9 / SE 1.5.97). Form ids are
  **load-order-indexed**: a plugin removed from the list loses its changes. SE compresses with LZ4.
  Failure modes: orphaned script instances bloat saves (ReSaver exists to clean them); "save relies on
  content no longer present".
- **Morrowind / OpenMW** (docs and release notes only). `.ess` = changed records + per-cell reference
  changes. OpenMW has one **save format version**: newer saves are refused, and since #6624 saves below a
  minimum are refused with the last release that converts them; 0.51 (2026) bumped it again. Lua state
  goes through `onSave`/`onLoad`.
- **Factorio.** Saves record mods and versions; the loader offers **sync mods with save**; mods migrate
  in `on_configuration_changed` and migration files.
- **Minecraft.** Every chunk, entity and `level.dat` carries a **DataVersion**; DataFixerUpper upgrades
  stepwise when old data loads (or all at once, "Optimize world"); one way; newer data refused.
- **Corruption.** Elden Ring: MD5 per section, `.sl2.bak`. Valheim: rolling `.old`; the usual failure
  is a 0-byte `.db` after a crash mid-save. `rename` is atomic for the name, not the data: write temp,
  `fsync`, rename, `fsync` the directory (LWN 323076).

**Takeaways**: deltas keyed by stable ids, never load order; per-part versions migrated stepwise; record
the mod list and keep data of missing content; atomic replace + checksums + a previous copy; watch size
per owner.

## Reuse (Bevy → crates → own)

1. **Bevy 0.20**: no save system. `bevy_world_serialization` (`DynamicWorld`, ex-`bevy_scene`) dumps
   reflected components to RON by type path, unversioned. Our saved state mostly lives in Bevy-free
   cores (`PlacementOverlay`, `WorldRpg`, `ContextSnapshot`), not components. Not used.
2. **Crates.** `moonshine-save` 0.8 (Bevy 0.20, MIT): reflected entities to RON; "does not support
   backwards compatibility, versioning, or validation". `bevy_save` 2.0.1: still Bevy 0.16 (2025-08).
   `savefile` 0.20: per-field versions, but a second derive beside serde on every type. `bincode`:
   unmaintained (RUSTSEC-2025-0141). `bitcode`, `rkyv`: formats tied to crate version or type layout.
   MessagePack/CBOR: field names cost size and `serde(default)` hides schema drift. `rusqlite` 0.40:
   incremental and transactional, but C, WAL sidecars, awkward to send to a guest. All rejected.
   **Used:** `postcard` 1 (stable documented format; already used by overlay, index pages, `sf_rpg`,
   script snapshots), `lz4_flex` 0.14, `blake3`, `tempfile` (all in the workspace), `serde-reflection`
   0.6 (MIT/Apache, tests only).
3. **Own:** container, version registry and migration runner, mod-list diff, dormancy, slots. No crate
   does owner-keyed versioned sections with dormant carry-forward. No Bevy project to adopt (Mudcrab has
   no saves yet); OpenMW is a behaviour reference only.

## Industry comparison

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Delta against content | Bethesda change forms with change flags; Morrowind `.ess` changed records; OpenMW per-cell ref changes | overlay + field-path record patches + per-system blobs | **match** |
| Identity | Bethesda: load-order-indexed form ids (break when a plugin is removed); OpenMW: content-file index + ref num | stable hashes and namespaced ids, order-free | **differ, better**: load order changes need no remap |
| Versioning | Minecraft DataVersion per chunk + DFU steps; OpenMW one global save version + minimum | per-section versions, stepwise typed migration, one-way | **match** (Minecraft granularity) |
| Too new / too old | OpenMW refuses both with a named release; Minecraft refuses newer | same | **match** |
| Mod list | Factorio sync mods; OpenMW/Bethesda warn and drop missing content | diff + sync offer + dormant carry-forward + clean | **match/better**: data kept until cleaned |
| Script state | Papyrus whole-VM state (bloat); OpenMW `onSave`/`onLoad`; Factorio `storage` | per-mod plain data + named timers | **match** (OpenMW/Factorio) |
| Atomic write | temp + fsync + rename (POSIX practice); Valheim `.old`; Elden Ring `.bak` | temp + fsync + rename + dir fsync + read-back + rotation + `.bak` | **match** |
| Integrity | Elden Ring MD5 per section | blake3 per section and TOC | **match** |
| Compression | Skyrim SE LZ4 | LZ4 per section, zstd possible later | **match** |
| Co-op | Enshrouded: world save on host, character with the player; Elden Ring separate co-op saves | host world save + guest character files with identical bytes | **match** (owner model) |

Sources (2026-10-22): UESP Skyrim save format pages; OpenMW #6624, 0.51 notes; Mojang "Programmers
play Minecraft's inner workings"; elden-ring-save-manager docs; Valheim host docs; LWN 323076;
RUSTSEC-2025-0141; crates.io and READMEs of the crates named in "Reuse".

**Other specs, at S2:** `sf_world` overlay v2 (blob versions, non-destructive flush); `PlayerKey` to
`sf_ids` and character extension blobs (rpg.md); physics.md's "`sf_save`, M10" becomes Phase 2.5.

## As built (S1, S2)

2026-10-23. `sf_save` (Bevy-free, 23 tests) and `sf_save_bevy` (19 tests; `script` feature for
`<mod>/script`). No serde derive was added to another crate: every saved type already had one.

**Container (S1).** As "File format", plus: the preamble's hash covers the other preamble bytes and the
TOC (a flipped kind or length byte is caught like a TOC byte); section offsets count from the TOC's end,
so the TOC sits after the preamble and `read_meta` reads the file's head only; sections must tile the
area contiguously in key order (`sf/meta` first, raw), so equal state gives equal bytes; an LZ4 section
whose declared raw length passes 255:1 is refused before allocating; LZ4 only from 128 B and only when
smaller. Decoding is strict (`sf_save::decode` refuses trailing bytes), so a shape change without a
version bump fails to load by name instead of loading garbage. `Registry` keys specs by owner (or any
owner) and **family** (the name before its first `/`: `character`, `script`); a spec refuses gaps in its
steps. Rotation groups: `Auto` + `GuestLeft` keep 3, `Quick` 2, `Exit` 1, `Manual` never; nothing
rotates unless the new file's directory sync succeeded. Manual overwrite (`SaveTarget::Path`) keeps
`.bak`. The read-back check compares the temp file's bytes before the rename, not after.

**Plugin (S2).** One exclusive system in `Last` (the frame's end: after every `FixedUpdate` tick of the
frame, a tick boundary) polls jobs, applies a finished load, then starts one load or save. Providers are
`SaveSection`s: `value` (one key), `family` (keys per instance) and `raw_family` (bytes already postcard,
the script runtime's); `get` clones on the main thread, encoding runs in the job; decoding runs in the
load job, then per-value `check`s (all before any apply: the overlay refuses blobs newer than
`BlobVersions`), `on_load_reset` hooks, apply in `sections::order`. A section the save lacks applies
`Default` (full replace). A load cancels saves queued before it; one autosave at most waits behind a
closed gate. `SaveConfig::background = false` runs jobs inline (tools, tests).

**Deviations from the text above.**
- Framework section types, versions and (future) frozen legacy types live in `sf_save_bevy::sections`,
  not in the owner crates: owners stay save-agnostic, and the shape goldens fail in this crate when an
  owner changes a saved type. The game wires its resources with closures (`read`/`write` helpers),
  because the state lives in heavy render crates (`sf_sky_bevy`, `sf_world_bevy`) or in the game.
- `sf/overlay` = `{ overlay: PlacementOverlay::to_bytes(), blob_versions }`: the blob version table is in
  the section, not in `sf_world`'s codec. Migrating older blobs needs `sf_world` to iterate blobs
  (overlay v2), and the non-destructive flush is the game's `get` closure (it returns a copy).
- `sf/records`: `sf_records` has no runtime overlay yet, so `RecordOverlay` (created records under
  `save:`, field-path patches, `apply` reporting what it skips) lives here, storing values as
  `SavedValue`, a frozen mirror of `sf_records::Value`.
- `<mod>/script`: a saved context the app doesn't run is kept dormant; a running context the save
  lacks keeps its state (fresh start of added mods is S3).
- `PlayerKey` stays in `sf_rpg`; `sf/thumbnail` (needs the renderer) and `sf/party` (S4) not built.
- Drift guard: `serde-reflection` traces each type by deserialization (every enum variant; nested enums
  listed explicitly) into `tests/shapes/<owner>.<family>.v<version>.json`; the overlay, whose codec
  needs real bytes, is traced from a sample. Golden save `tests/golden/s2.sfsave` (the played test
  level) must load to the same state and re-save to the same raw bytes. Bless: `SF_SAVE_BLESS=1`.
- Fuzz: deterministic in-test fuzz (truncation at every offset, every single-bit flip, 20k random
  mutations half of them re-signed past the TOC hash) instead of `cargo fuzz` + proptest.

**Budgets** (release, 4-core cloud runner, `cargo test -p sf_save_bevy --release budgets -- --nocapture`;
synthetic 100-hour world: 30k overlay changes, 2k journal entries, 1k heard lines, 300 stacks, 700
record changes, 1.5k globals, ~2 MiB script-like state):

| Measure | Budget | Measured |
|---|---|---|
| New game save | ≤ 256 KiB | 723 B (no thumbnail) |
| 100-hour save on disk | ≤ 8 MiB | 2.55 MiB (`sf` sections 1.48 MiB; the overlay is 1.44 MiB raw, ~50 B per change) |
| Snapshot, main thread, script state excluded | ≤ 5 ms | 3.5-5.6 ms warm (overlay copy 2.6-4.5 ms); a cold first copy of the overlay ~9 ms |
| Script-like state, 2 MiB | 25-33 ms/MiB (Lua) | 3.0-3.3 ms (a plain clone: not a Lua walk) |
| Encode + write, off-thread | ≤ 500 ms | 28-31 ms |
| Load before the streaming burst | ≤ 300 ms | 34-39 ms |

The snapshot is at the budget's edge and over it with a cold allocator: the overlay copy dominates.
If the owner's laptop confirms, the fix is a copy-on-write overlay in `sf_world` (persistent maps or
`Arc`'d per-cell buckets), so a snapshot is O(changed buckets).


## As built (game load correctness, 2026-10-25)

- **Resident cells on load: full reload, not a diff.** Bethesda's engines and OpenMW (`World::clear`, then the
  player's cell loads) tear the world down on load and stream it back from the saved state; diffing the live cells
  against the loaded overlay would need every system's inverse (respawn taken items, un-move bodies, undo scripts).
  `sf_world::Scheduler::reload_resident` marks every live cell stale: it unloads all the way through the normal
  stages (pending loads cancel, so no fetch made with the old overlay survives), then streams back with the loaded
  overlay; `arrival_ready`, burst mode and `Streaming::is_idle` wait for it. `Streaming::reload_resident` also pins
  the loaded models until idle (cells come back from memory). `StartTravel::reload` calls it at the fade's cut, so
  the teardown is never seen. Deactivation hooks get `ActivateCx::discard`: physics skips its write-back (the live
  pose would overwrite the loaded one).
- **Save flush:** `sf_physics_bevy::stream::persist_moved_items` writes resident moved items into the snapshot's
  overlay copy (the deactivation write, non-destructive), so a crate knocked over and still resident saves moved.
- **Game:** menus and modals close in an `on_load_reset` hook; the body (`CharacterBody`: `ActionState` +
  `Locomotion`) is saved and put back on arrival (`ActionBody::restore`). Test:
  `sf_world_bevy` `reload_resident_respawns_cells_from_the_loaded_overlay`.
