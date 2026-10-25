# Saves: host world save + per-player character save (`sf_save`)

Status: **S1 + S2 built (Phase 2.5, 2026-10-23)**: see "As built (S1, S2)" at the end, which wins
where it differs from the text above. S3 (mod diff, dormancy dialog, `clean`) and S4 (co-op) not built.
Crates: `sf_save` (Bevy-free: container, versions, migration runner, atomic files, slots, CLI; the
mod-list diff is S3), `sf_save_bevy` (thin plugin: requests, tick-boundary snapshot, background write,
load orchestration, autosave; the framework's section types).
Sources: [net.md](net.md) "Saves" and "Quest model", [rpg.md](rpg.md), [world-coordinates.md](world-coordinates.md),
[mods.md](mods.md), [streaming.md](streaming.md) "Runtime overlay", [scripting.md](scripting.md) "State,
timers and saves", [weather.md](weather.md) "State and forecast".

## Goal

Save and resume a game, single-player or co-op, on the same code path: the host writes one **world save**
holding the shared world and every character that has played in it; each guest keeps a **character
file** for every world they joined. Saves survive game updates (migrations), mod-list changes (missing
mods don't destroy data), crashes and power loss (never lose the last good save), and stay small and
quick enough to autosave without a visible hitch.

Non-goals: cloud sync (one plain file per save suits it), save editors beyond a dump, legacy save
import (open question 6), cross-version co-op, incremental or chained saves.

## Owner decisions this spec implements

| Decision | Here |
|---|---|
| Shared world, own characters; host world save + per-player character save; host copy wins in its world (net.md, 2026-10-19) | "Co-op" |
| Co-op world saves live apart from single-player saves (net.md) | "Slots and files" |
| Everything is a mod; stable ids, absolute positions (net rule 7) | "Mod-list changes" |
| Bevy, then crates, then our own (CLAUDE.md rule 8) | "Reuse" |

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

## Canonical technique

**A self-contained full snapshot per save, split into owner-keyed sections, each with its own schema
version, postcard-encoded, LZ4-compressed and blake3-hashed, written atomically; deltas against content
keyed by stable ids; stepwise migration on load; data of missing mods kept dormant.** Change forms in
spirit (only what differs from content), Minecraft in versioning, Factorio in mod handling.

## File format (container)

One file per save, built in memory off-thread and written in one go. Little-endian.

| Part | Content |
|---|---|
| Preamble (fixed 64 bytes) | magic `SFSV`, container version (u16), file kind (`World` / `Character`), TOC offset and length, blake3 of the TOC, total file length |
| TOC (postcard) | one entry per section: key `(owner, name)`, schema version (u32), codec (`Raw`, `Lz4`), offset, stored length, raw length, blake3 of the stored bytes, flags (`Optional`, `Dormant`) |
| Sections | in TOC order; `sf/meta` first and `Raw`, so a save list reads preamble + TOC + meta (a few KiB) |

- **Owner** is `sf` (framework), the adapter's name (`mw`), or a mod id. Names are owner-local
  (`sf/overlay`, `mw/globals`, `better_lanterns/script`). Owners map to mods, which drives dormancy.
- Truncation is caught by the total length; bit rot by the hashes; a wrong file by the magic.
- Limits on decode (untrusted input: shared saves, guest files): TOC ≤ 1 MiB, ≤ 4 096 sections, raw section
  ≤ 256 MiB, decompression into a buffer of the declared raw length only. Decoding never panics.
- The codec byte lets `Zstd` be added later without a container change.

### `sf/meta` (always present, never compressed)

World id (u128, random at new game), save id (u128), parent save id, sequence number in the world,
save reason (`Manual`, `Quick`, `Auto`, `Exit`, `GuestLeft`), wall-clock time (display only), game time,
play time, sim tick, framework version, content pack id + version, build id, profile name, **load order**
(mod id, version, manifest hash) and the **profile fingerprint** (mods.md), `GRID_CELL`
(world-coordinates.md), party list (player key, display name), and adapter display fields (character name,
level, location label) for the load screen.

## What is saved (section catalog)

Gameplay state only (net rule 3): no pose, camera, VFX, UI or streaming state. Everything is keyed by
stable ids (`PlacementId`, spawn ids, `RecordId`, `PlayerKey`, mod ids) and positions are `WorldPos` /
`Location` (net rule 7).

| Section | Owner crate | Content |
|---|---|---|
| `sf/meta` | `sf_save` | above |
| `sf/thumbnail` | `sf_save_bevy` | small screenshot (PNG via Bevy's `image`); `Optional` |
| `sf/clock` | `sf_sky` | `SkyClock` days and time scale, sim tick |
| `sf/weather` | `sf_weather`, `sf_wind` | `WeatherState { seed, overrides }`, `WindState`; later `sf_surface`'s coarse grid |
| `sf/records` | `sf_records` | the **runtime record overlay**: records created in play (spells, potions, enchantments: Bethesda's "created objects") under a `save:` id namespace, and runtime patches to existing records as (kind, id, field path, `Value`), the same ops mods use |
| `sf/overlay` | `sf_world` | `PlacementOverlay`: removed, moved and spawned placements, per-system state blobs, spawn counter |
| `sf/rpg` | `sf_rpg` | `WorldRpg`: quests, journal, known topics |
| `sf/party` | `sf_save_bevy` (`sf_net` later) | player keys and names, party leader, co-op settings (friendly fire) |
| `sf/character/<player key>` | per character | `CharacterSave` (below), one section per player who ever played here |
| `<adapter>/<name>` | adapter | adapter state (Bevyrrowind: MWScript globals and locals, its NPC rule state) |
| `<mod id>/script` | `sf_script` | that mod's `ContextSnapshot` (storage, entity-script states, timers, RNG) |

**Overlay blobs.** Per-placement state blobs (`physics` v1, actor health, AI progress, container
contents) are already keyed by owning system. The overlay codec gains a **blob version table**
(system → schema version) so each system migrates its own blobs (overlay format v2, `sf_world`).

**`CharacterSave`**: player key, display name, `Location` + facing, the gameplay body (`ActionState` +
`Locomotion`, action.md: "saved"), `CharacterRpg` (loadout), stats sheet (M7), appearance (character.md
assembly choices as record ids), and **extension blobs** keyed by owner (adapter or mod per-character
data, each with its own version). A character's state is one section so a guest file can carry the exact
same bytes.

### One path for unload and save

Live state of resident things (awake bodies, NPCs in R0, open containers) is written into the overlay by
the function each system already runs when a placement leaves R0 (physics.md "Deactivation"), into a
**copy** of the overlay and without removing anything, so streaming and saving can't disagree.

## Saving

1. **Request** (`SaveRequest { reason, slot }`, a command): from the UI, autosave timer, a guest leaving,
   or a script. Gate: never during a load, a streaming burst or a door crossing; the adapter's
   `can_save(reason)` adds content rules (Morrowind: no saving with enemies nearby). In co-op only the host
   saves.
2. **Snapshot** at a fixed-tick boundary (after `FixedUpdate` tick N, before N+1): each registered provider
   clones its plain state into a `SaveBuilder` (owner, name, version, value). Lua contexts snapshot on their
   own thread (scripting.md: 25-33 ms per MiB of script state, the one main-thread cost that scales).
3. **Encode** off-thread: postcard, LZ4, blake3, TOC; dormant sections copied through unchanged.
4. **Write** off-thread ("Corruption safety"); gameplay continues; a second request queues.
5. **Done** event with size per owner (UI, `sf_diag`).

Equal state gives equal bytes (ordered maps, no time outside `sf/meta`), so round trips compare bytes.

## Loading

Load is a **full replace**, never a merge into the running world:

1. Read the file and verify the preamble, TOC and every hash (off-thread).
2. Decode `sf/meta`; compare with the current profile ("Mod-list changes"); report or prompt.
3. Migrate each section to its current version and decode (off-thread).
4. On the main thread, behind a fade: despawn gameplay entities, reset resources, apply in order:
   records → overlay → clock and weather → rpg → party and characters → adapter → scripts (restore, then
   `core.on_load { reason = "restore" }`).
5. Streaming burst at each local player's location (streaming.md "Burst mode"), then fade in.

## Versioning and migration

- **Container version**: the preamble's; changes rarely; the reader keeps the previous version.
- **Section schema version**: a u32 per section key (and per overlay blob system, and per character
  extension blob). The owning crate holds the current version, **frozen copies of every older type**
  (`legacy::v3::WorldRpg`) and one step function per version (v3 → v4, typed). Owner crates don't depend
  on `sf_save`: they expose the version constant and the steps; `sf_save_bevy` registers them.
- **Load** runs the steps from the stored version up to the current one; the save is only ever written at
  the current version (Minecraft's one-way upgrade). A version newer than the build knows refuses the load,
  naming the section and owner (OpenMW does the same). Below an owner's minimum supported version: refused
  with the last build that can convert it (OpenMW's message); window in open question 1.
- **Drift guard**: a test records each saved type's serde shape with `serde-reflection` as a golden file;
  a shape change without a version bump and a step fails CI. Semantic changes with an unchanged shape
  (units, meanings) still need a bump; reviewers check.
- **Golden saves**: synthetic saves of every released section version stay in the repository and must
  load and migrate to their expected values.
- **Field-path patches survive struct changes**: `sf/records` patches address fields by path, so a
  patch to a field that no longer exists is dropped with a report rather than breaking the section.
- **Mods**: a mod's script state migrates in its own `core.on_load(info)` with `old_version`
  (scripting.md). Mods own no other binary sections today; if they gain some, the same version rules apply.

## Mod-list changes

The save stores the load order and profile fingerprint; the current profile is diffed against it:

| Difference | Default | Report |
|---|---|---|
| none (fingerprint equal) | load | none |
| order only | load: ids don't depend on order | info |
| mod added | load; its scripts start fresh (`reason = "start"`), its placements appear | info |
| mod version changed | load; its script `on_load` sees `old_version`; record and bake changes apply as usual | info |
| content pack or adapter version changed | load; section migrations run | info |
| mod missing or disabled | **ask**: load anyway (its data stays dormant), or cancel | warning, listing what it owns |

If a profile with exactly the save's mods exists, or every missing mod is installed, the dialog offers
**switch to the save's mods** (Factorio's sync).

**Dormancy.** Nothing a missing mod owns is destroyed on load:
- sections it owns (`<mod>/script`, its character extension blobs, its overlay blob systems) are carried
  forward unchanged with the `Dormant` flag;
- overlay entries whose base record no longer resolves are not instantiated (streaming skips them with a
  counter) and are kept;
- inventory stacks of unresolved items are unequipped, hidden from UI and rules, and kept;
- runtime record patches to missing records are kept; quests, topics and journal entries keep their ids
  (journal text is snapshotted already, rpg.md).

When the mod returns, its data reattaches by id. **Clean save** (UI action and `sf_save clean` CLI)
writes a new save without the dormant data of mods outside the current profile and reports what was
dropped and its size. Stable ids make this possible: Bethesda's load-order-indexed form ids can't keep
data for a plugin that is no longer in the list.

## Corruption safety

1. **Atomic replace.** Write `<file>.tmp` in the target directory, `sync_all`, rename over the target,
   then `fsync` the directory (POSIX). On Windows, `std::fs::rename` replaces atomically
   (`MoveFileExW` with `MOVEFILE_REPLACE_EXISTING`). A save is never written in place.
2. **Read-back check** of the new file (preamble, TOC, every hash; blake3 runs at GB/s) before the rotation
   deletes an older save.
3. **Keep the previous good copy**: autosaves rotate (3 by default), quicksaves rotate (2), and overwriting
   a manual slot keeps the old file as `<file>.bak` until the next verified write (Elden Ring's `.bak`,
   Valheim's `.old`).
4. **Integrity**: total length, magic and per-section blake3. A bad `Optional` section is skipped; a bad
   mod-owned section asks "load without <mod>'s state"; a bad framework or adapter section refuses the load
   and offers the newest earlier save of the same world (parent chain).
5. **Failures surface**: disk full or permission errors reach the UI; the temp file is removed; the old
   save is untouched.
6. **Bounded decoding** (limits above), fuzzed.
7. **Bloat is watched** (Skyrim's orphaned scripts): stored size per owner is in the TOC, reported by
   `sf_save info` and `sf_diag`, with a warning when one mod's script state passes its budget (scripting.md
   memory limits).

## Slots and files

- `<user data>/saves/single/<world id>/<n>-<reason>.sfsave`, `<user data>/saves/coop/<world id>/...`
  (net.md: co-op apart). A folder per world groups a playthrough (OpenMW's per-character folders).
- `<user data>/characters/<world id>/<player key>.sfchar`: guest character files, with one `.bak`.

## Size and time budgets

| Measure | Budget | Basis |
|---|---|---|
| New game save | ≤ 256 KiB (thumbnail included) | |
| 100-hour Morrowind-scale save | ≤ 8 MiB on disk | ~30k overlay changes at ~67 B before LZ4 (streaming.md page figure), 2k journal entries, ≤ 2 MiB script state; Morrowind `.ess` files are typically a few MB (unverified) |
| Snapshot, main thread | ≤ 5 ms, plus Lua at scripting.md's 25-33 ms/MiB | clones of plain data |
| Encode + write, off-thread | ≤ 500 ms for 8 MiB | LZ4 and blake3 at GB/s; the disk dominates |
| Load before the streaming burst | ≤ 300 ms for 8 MiB | read, verify, migrate, decode |

Full snapshots stay right while these hold. If saves pass ~50 MiB (procedural worlds, persistent terrain
deformation), revisit per-region files or SQLite (Minecraft's per-chunk regions).

## Co-op (net-ready)

Single-player is a host with one player: same file, same sections.

- **Host world save**: every section above, with one `sf/character/<key>` per player who ever joined: the
  last known state of every guest (net.md). Only the host writes it.
- **Guest character file** (`.sfchar`, kind `Character`): `sf/meta` (world id, player key, the world save
  id and sequence it came from, play time) and `sf/character` holding **the same bytes** (version, hash)
  as the host's section. The host sends them over the reliable channel after every host save and when the
  guest leaves; the guest checks the hash and writes atomically.
- **Join**: the handshake carries the guest's player key and, if any, its file's (world id, sequence,
  hash). Host has a section for that key → **host copy wins** (net.md). Else the guest's file for this
  world id is decoded, migrated and resolved under the same caps and dormancy rules, then accepted.
  Else the guest creates a character.
- **Consistency by construction**: a guest file is only written from a host save, so it is never newer
  than the host's latest save. When the host loads an older save, its older character copy wins with the
  world it belongs to (quest items and quest stages rewind together).
- Only host state is saved; unapplied requests and client predictions are dropped. "Forget player"
  deletes a character section. Guest files are untrusted input (bounded decoding; no anti-cheat).
- `PlayerKey` moves from `sf_rpg::ids` to the serde-only `sf_ids` (as `PlayerId` did).

Net rules: 1 (snapshot on a tick boundary), 3 (gameplay state only), 4 (no wall clock in state; meta time
is display), 5 (host writes, guests receive), 6 (characters keyed by player, no singleton), 7 (stable ids,
`WorldPos`), 8 (save/load requests and done events are data).

## Interfaces (shape)

`sf_save`: `SaveFile` (meta-only read, verify, sections), `SaveWriter`, `Registry` (versions, steps),
`diff_mods`, `write_atomic`, `SlotDir`, CLI `sf_save info|verify|dump|clean`. `sf_save_bevy`:
`SaveRequest`/`LoadRequest`/`SaveDone`/`LoadDone`, `SaveProviders` (snapshot, apply, order),
`SaveGate`. Owner crates: `SAVE_VERSION`, `legacy` modules, steps.

## Tests

All on synthetic data.
- **Container**: round trip; meta-only read; truncation at every offset and bit flips are errors, never
  panics; caps; dormant sections copied byte for byte.
- **Atomic write**: a fault-injecting file layer fails each step (write, sync, rename, dir sync,
  read-back); the old save survives, no temp file remains; rotation and `.bak`.
- **Migration**: golden saves per released version; `serde-reflection` shape goldens; too-new and
  too-old refusals.
- **Mods**: each diff case; a missing mod's script state, blobs, spawns and stacks survive saves and
  reattach on return; `clean` drops exactly them.
- **Canonical round trip** (headless Bevy, `sf_level`): pick up, move props, kill an NPC, advance a
  quest, run timers → save → load → save: identical sections apart from `sf/meta`.
- **Co-op**: guest file bytes equal the host section; all three join paths; older host save rewinds.
- **Fuzz**: `cargo fuzz` on decode, proptest variant in CI.
- **Budgets**: synthetic 100-hour world against the table; `sf_diag` counters `save/snapshot_ms`,
  `save/encode_ms`, `save/write_ms`, `load/total_ms`, `save/bytes/<owner>`.

## Slices

| Slice | Content | Done when |
|---|---|---|
| S1 | `sf_save` container, registry, atomic writes, slots, CLI `info`/`verify`/`dump` | container, atomic-write and fuzz tests green |
| S2 | sections for existing state (clock, weather, records overlay, placement overlay v2, rpg, scripts, one character); `sf_save_bevy` single-player save/load/autosave | canonical round trip in an `sf_level` level; budgets met |
| S3 | mod diff, dormancy, sync dialog data, `clean` | dormancy tests green; Bevyrrowind save survives removing and re-adding a mod |
| S4 (co-op N7) | party, character files, join paths | "save, quit, resume with a guest" (net.md N7) |

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

## Open questions (owner)

1. **Migration window.** Write migrations only from the first public release, and refuse older dev
   saves with a clear message until then? (Recommended: development moves fast; golden saves start at
   release.)
2. **Dormant data of removed mods**: keep it until the player runs "clean save" (default here), or drop it
   automatically after some number of saves?
3. **Save restrictions**: the framework blocks saving only during loads and transitions; should Bevyrrowind
   keep Morrowind's "no saving with enemies nearby" rule (`can_save`), and should co-op guests be able to
   ask the host for a save?
4. **Autosave**: default triggers and counts (here: 3 rotating autosaves, every 10 min of play, on rest,
   on a guest leaving; door crossings too, as Bethesda does?).
5. **Hosting a single-player world in co-op**: allow copying a single-player world into the co-op saves
   (its character becoming the host's), or co-op worlds only from a new game? (Related to net.md
   question 1.)
6. **Legacy saves**: should Bevyrrowind ever import Morrowind `.ess` saves (an adapter-side converter
   into these sections), or never?

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
