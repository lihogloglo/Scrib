# Saves: host world save + per-player character save (`sf_save`)

Status: **S1 + S2 built (Phase 2.5, 2026-10-23), game load correctness 2026-10-25**: see "As built" at the
end, which wins where it differs from the text above. S3 (mod diff, dormancy dialog, `clean`) and S4 (co-op)
not built. Crates: `sf_save` (Bevy-free: container, versions, migration runner, atomic files, slots, CLI; the
mod-list diff is S3), `sf_save_bevy` (thin plugin: requests, tick-boundary snapshot, background write,
load orchestration, autosave; the framework's section types).
Sources: [net.md](net.md) "Saves" and "Quest model", [rpg.md](rpg.md), [world-coordinates.md](world-coordinates.md),
[mods.md](mods.md), [streaming.md](streaming.md) "Runtime overlay", [scripting.md](scripting.md) "State,
timers and saves", [weather/state.md](weather/state.md) "State and forecast".
History (2026-10-22 research, crate survey, industry table, S1/S2 log and measurements):
[archive/save-history.md](archive/save-history.md).

<!-- agent-summary -->
> **Agent summary (2026-10-26).** Status: S1 + S2 built (Phase 2.5), game load correctness 2026-10-25; the save/load screen exists in `sf_ui_screens` (`saveload.rs`) but game wiring (ROADMAP 2.5 part B: load screen, save list, naming) is not done; SV-1 awaiting owner. Not built: S3 (mod diff, dormancy dialog, `clean`), S4 (co-op).
> Crates / key types: `sf_save` (Bevy-free: container, versions, migration, atomic files, slots), `sf_save_bevy` (plugin, tick-boundary snapshot, background write, autosave).
> Decisions: host world save plus per-player character save, host wins ("Owner decisions"); load is a full resident-cell reload, not a diff ("As built (game load correctness)"); LZ4 container with strict decode ("As built (S1, S2)").
> Read next: "What is saved" for sections; "Versioning and migration" for format changes; "As built" at the end (wins on conflicts).
<!-- /agent-summary -->

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

## Reuse (Bevy → crates → own)

Bevy 0.20 has no save system (`bevy_world_serialization` dumps reflected components, unversioned; our
saved state lives in Bevy-free cores, not components). No crate fits: `moonshine-save` has no versioning,
`bevy_save` is on Bevy 0.16, `savefile` needs a second derive, `bincode` is unmaintained, `bitcode`/`rkyv`
tie the format to crate version or layout, MessagePack/CBOR cost size and hide drift, `rusqlite` is C with
WAL sidecars. **Used:** `postcard` 1, `lz4_flex` 0.14, `blake3`, `tempfile`, `serde-reflection` 0.6 (tests).
**Own:** container, version registry and migration runner, mod-list diff, dormancy, slots. Survey and the
research behind it (Bethesda change forms, OpenMW, Factorio, Minecraft DataVersion, Elden Ring/Valheim
corruption handling): [archive](archive/save-history.md#reuse-bevy--crates--own).

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

Matches practice on every row; better on identity (stable ids, not Bethesda's load-order-indexed form ids)
and missing mods (dormant data kept until cleaned). Table: [archive](archive/save-history.md#industry-comparison).

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

2026-10-23: `sf_save` (Bevy-free, 23 tests), `sf_save_bevy` (19 tests; `script` feature for `<mod>/script`).
Full log and measurements: [archive](archive/save-history.md#as-built-s1-s2).

- **Container**: the preamble's hash also covers the TOC; section offsets count from the TOC's end
  (`read_meta` reads the head only); sections tile contiguously in key order (`sf/meta` first, raw); LZ4
  only from 128 B and when smaller, a declared ratio past 255:1 refused before allocating; decoding is
  strict (trailing bytes refused), so an unbumped shape change fails by name. Read-back runs on the temp
  file before the rename.
- **Registry** keys specs by owner (or any) and **family** (name before the first `/`: `character`,
  `script`); gaps in steps are refused. **Rotation**: `Auto` + `GuestLeft` keep 3, `Quick` 2, `Exit` 1,
  `Manual` never (overwrite via `SaveTarget::Path` keeps `.bak`); nothing rotates unless the directory
  sync succeeded.
- **Plugin**: one exclusive system in `Last` (a tick boundary) polls jobs, applies a finished load, then
  starts one load or save. Providers are `SaveSection`s (`value`, `family`, `raw_family` for the script
  runtime's postcard bytes); `get` clones on the main thread, encode/decode run in the job. Load: all
  `check`s before any apply (the overlay refuses blobs newer than `BlobVersions`), `on_load_reset` hooks,
  apply in `sections::order`; a missing section applies `Default`. A load cancels queued saves; at most one
  autosave waits behind a closed gate; `SaveConfig::background = false` runs inline.
- **Section types and versions live in `sf_save_bevy::sections`**, not the owner crates (owners stay
  save-agnostic; shape goldens fail here); the game wires resources with `read`/`write` closures.
  `sf/overlay` = `{ overlay bytes, blob_versions }` (the table is in the section; migrating old blobs waits
  for `sf_world` blob iteration, overlay v2; the flush is the game's `get` returning a copy). `sf/records`:
  `RecordOverlay` (created records under `save:`, field-path patches, `apply` reports skips, values as
  `SavedValue`, a frozen `sf_records::Value`) lives here until `sf_records` has a runtime overlay.
  `<mod>/script`: an unrun saved context stays dormant; a running context the save lacks keeps its state.
- Not built: `PlayerKey` stays in `sf_rpg`; `sf/thumbnail`, `sf/party` (S4).
- **Drift guard**: `serde-reflection` shape goldens `tests/shapes/<owner>.<family>.v<version>.json`;
  golden save `tests/golden/s2.sfsave` loads and re-saves to the same raw bytes; bless `SF_SAVE_BLESS=1`.
  Fuzz is in-test and deterministic (every truncation, every bit flip, 20k mutations), not `cargo fuzz`.
- **Budgets** (`cargo test -p sf_save_bevy --release budgets -- --nocapture`, synthetic 100-hour world):
  met (2.55 MiB, encode + write ~30 ms, load ~35 ms) except the main-thread snapshot, 3.5-5.6 ms warm and
  ~9 ms cold vs 5 ms, dominated by the overlay copy. If the owner's laptop confirms: copy-on-write overlay
  in `sf_world` (persistent maps or `Arc`'d per-cell buckets).

## As built (game load correctness, 2026-10-25)

- **Resident cells on load: full reload, not a diff** (Bethesda, OpenMW's `World::clear`; a diff would need
  every system's inverse). `sf_world::Scheduler::reload_resident` marks every live cell stale: it unloads
  through the normal stages (pending loads cancel) and streams back with the loaded overlay;
  `arrival_ready`, burst mode and `Streaming::is_idle` wait for it, and `Streaming::reload_resident` pins
  the loaded models until idle. `StartTravel::reload` calls it at the fade's cut. Deactivation hooks get
  `ActivateCx::discard`: physics skips its write-back.
- **Save flush:** `sf_physics_bevy::stream::persist_moved_items` writes resident moved items into the
  snapshot's overlay copy, so a knocked-over crate still resident saves moved.
- **Game:** menus and modals close in an `on_load_reset` hook; `CharacterBody` (`ActionState` +
  `Locomotion`) is saved and put back on arrival (`ActionBody::restore`). Test: `sf_world_bevy`
  `reload_resident_respawns_cells_from_the_loaded_overlay`.
