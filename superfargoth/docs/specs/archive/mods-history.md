# Mods: history

Moved out of [mods.md](../mods.md) on 2026-10-26 (doc diet S12a), word for word. Headings are unchanged. mods.md and the code win where they differ.

## What exists (M0, read 2026-10-13)

- `sf_mods`: `ModManifest` (`mod.toml`: id, name, version, `api` requirement, description,
  `dependencies` with version reqs, `load_after`/`load_before` hints, `content.{assets,
  records, scripts}` roots), `ModManifest::synthesized` for adapters, `resolve_load_order`
  (Kahn's algorithm, ties broken by the profile's order; errors: unknown, duplicate, missing
  dependency, version mismatch, cycle).
- `sf_vfs`: immutable layered index (`VfsBuilder::mount`, last wins), `DirSource`,
  `MemSource`, adapter sources (BSA in `mw_vfs`), `providers()` per path, `stats()`
  (files/winning per source), `resolve_preferring` (modern formats take a legacy file's slot),
  Bevy `AssetReader` over an `Arc<Vfs>`, `.meta` lookup.
- `sf_records`: sources in apply order with `depends_on`, define/patch/upsert/delete,
  per-record history (`Touch { source, op }`), dependency-aware conflict report.
- Bevyrrowind: ESM/ESP become synthesized mods (masters → dependencies, `KNOWN_ORDER` hints),
  one record source per file; the app mounts builtin graphs < BSAs < loose `Data Files`.

Missing for M5: a mods directory and profiles, manifest fields below, native record files,
script/UI hookup, a unified report, VFS hot reload, and the profile fingerprint for the bake.

### As built (A1, 2026-10-13)

Changes from the draft, each for a reason found while building:

- **Content paths are VFS paths inside the mod's own asset roots.** `content.records`,
  `content.scripts` and `ui.styles` name files as the mod's asset roots provide them (with
  `assets = ["assets"]`, `assets/records/x.ron` is listed as `records/x.ron`), so loose folders
  and future zip mods resolve the same way and the check is VFS case-insensitive. A record file
  is read from the **mod's own layers** (`ModFiles`), never from a later mod shipping the same
  path. Synthesized mods may fall back to the whole VFS (a legacy plugin sits in the shared data
  folder, not in its archive). Entry points must be under `scripts/<mod id>/` (`sf_script`'s
  namespace), else a Files error.
- **One broken mod never blocks the rest.** `load_profile` drops a mod that can't load (unknown,
  invalid, `api` incompatible, missing/mismatched dependency, enabled optional with the wrong
  version, declared conflict present, cycle member) with an error, re-resolves, and its dependents
  then fail with "requires X, which was not loaded". Factorio disables the same cases (its
  `info.json` docs: an enabled optional dependency outside the version requirement makes the mod
  incompatible); MO2 warns and loads anyway, which is how load orders silently break. The strict
  `resolve_load_order` stays (first error wins) for adapters that resolve their own lists; its
  `Cycle` now names only the mods on the cycle, not those downstream of it.
- **Declared conflicts refuse the declaring mod**, not the other one (it is the one that knows).
- **`ContentAdapter::mount` returns layers** instead of threading a `VfsBuilder`, so the pipeline
  knows which source indices belong to which mod (provenance, `ModFiles`, the override view).
  A mod's layers are its adapter layers, then its asset roots. Record files are routed by
  extension to the adapter that claims it (`record_extensions`); the native RON loader (A2) is
  one more `ContentAdapter`, so the framework format is not privileged. Until A2, a `.ron` file
  reports "no loader".
- **Script limits are profile-level only** (`[limits] memory_mib, call_timeout_ms,
  frame_soft_us`, unset = runtime default): a manifest can't raise its own limits (scripting.md
  OQ4: per-mod overrides belong to the mod manager later).
- **Asset roots are indexed once at scan** (`DirSource` per root, `Arc`-shared) and reused by
  every profile load; folder mods' declared files are checked against them at scan time, so a
  mod with a missing file is invalid before ordering (its dependents drop cleanly).
- **Profile file**: `enabled = [...]`, `[settings.<mod id>]` (free TOML tables; the settings
  registry types them in U6), `[limits]`. The name is the file stem. `deny_unknown_fields`.
- **Report**: `IssueKind` (one variant per row of the table above for steps 1-4, with
  `severity()` and `area()`), `ModSummary` per loaded mod: layers, files, winning, unique, and
  `overrides`/`overridden_by` per other mod (counts against the winner, the MO2 view). Text via
  `render`, data via serde; `render_report` appends `ConflictReport::render`.
- **Scripts hand-off**: `ScriptPlan` (id, mods, loaded dependencies incl. enabled optionals,
  entry points, `api`, version, profile limits) maps one to one onto `sf_script::ContextConfig`;
  `sf_mods` does not depend on `sf_script` (mlua stays off the content path's compile graph).

Not in A1 (later tasks): profile fingerprint (A3), builtin-mod-lowest rule and default profile
(A3), the `resolve_preferring` shadow warning (needs the adapter's alternates, A3), Scripts/UI
issue rows (L2, U2).

### As built (A2, 2026-10-13)

**Reuse check.** Bevy has no record database or patch format. Crates: `ron` 0.12 (already in the
tree) parses the files; `ron::Value` can't be the patch tree because it drops enum variant names
(a record with an enum doesn't round-trip); `serde-value` round-trips the data model but is
unmaintained since 2020 and can't hold "text not yet typed" nodes; JSON Patch / JSON Merge Patch
(RFC 6902 / 7386) give the op vocabulary (`set` = replace/add, `remove`, merge = 7386) but
their crates work on `serde_json::Value`, which has no enums, tuples or non-string keys. So
`sf_records::Value` is our own small serde data-model tree (~500 lines with its Serializer and
Deserializer), with one twist: values from a RON file stay **raw RON text until the record type
asks for them**, then `ron` parses them with the type's own hints. Enum variants, newtypes,
`Option`s and tuples read exactly as in a normal RON file, and errors carry file positions.

Changes and details found while building:

- **Kind registry**: `register::<T>(kind)` is unchanged (no serde bound; legacy and closure-only
  kinds). `register_serde::<T>(kind)` adds a serde codec and the default fingerprint (blake3 of
  the postcard bytes; bake.md "Native kinds use serde bytes"); `set_fingerprint::<T>(fn)` lets an
  adapter hash its own bytes. By-name API: `define_value`, `patch_value`, `delete_by_name`,
  `value_of` (inspection), `fingerprint`, `history_of`, `kind_names`, `is_serde_kind`.
- **File format** (RON): a list of `define(kind, id, value)`, `patch(kind, id, set: {path: v},
  merge: {path: map}, remove: [path], append: {path: [v]})`, `delete(kind, id)`; unknown op
  fields are errors. `implicit_some` is on by default (`radius: 5.0` fills an `Option<f32>`);
  `#![enable(...)]` at the top of a file adds more and applies to every value in it. **TOML**:
  `[[define]]`, `[[patch]]`, `[[delete]]` tables with the same fields, applied all defines, then
  patches, then deletes (TOML groups arrays of tables); values are read leniently (a string is a
  unit enum variant, `{ Variant = {...} }` a variant with data, an integer a float). Quote dotted
  paths in TOML (`"stats.luck" = 7`), else TOML makes them nested tables.
- **Field paths**: dot-separated struct fields, map keys (string, number, char, bool, unit-variant
  keys as written) and list indices; `Option` and enum payloads are transparent (`flicker.rate`
  reaches `flicker: Some(Fast(rate: ..))`). `set` may add a map key or a struct field the
  serializer skipped; `merge` deep-merges maps and structs (anything else replaces; merging into
  an enum keeps its variant); `remove` drops a map key or list element, or removes a struct
  field so serde's default applies (`#[serde(default)]`, `Option` → `None`; no default = error);
  `append` extends a list. Ops apply set, merge, remove, append; a patch is all or nothing.
  After deserializing, the record is re-serialized and every written path must exist, so a
  typo'd field is an error even for types without `deny_unknown_fields` (consequence: setting a
  `skip_serializing_if` field to its skipped value reads as missing; use `remove`).
- **Errors**: a syntax error rejects the file (`RecordFileError::at(line, col)`); otherwise each
  op applies independently and each failing op (unknown kind, kind without serde, missing
  record, bad path, wrong value type) is its own `RecordFile` error at the op's position, or
  inside the value when the error is there. `RecordCx::error` was added for this. One broken op
  never blocks the rest of the mod, matching "one broken mod never blocks the rest" (A1).
- **Field-level touches**: `Touch { source, op, fields }`; `fields` lists the paths a generic
  patch wrote (merge records its leaves), `None` for define, delete and closure patches (whole
  record). Patches from one source union their fields. A blind pair (later source doesn't depend
  on the earlier) is a **conflict** when their writes overlap (same path, a path and its parent,
  or a whole-record touch), else a **merge**. `KindSummary` gains `merges`; `RecordConflict` gains
  `fields` (contested paths, empty = whole record) and renders as the xEdit-style chain
  `a (define) -> b (patch radius) -> c (patch radius) [contested: radius]`.
- **Report**: `load_profile` turns each conflict into a `RecordConflict` warning and each merge
  into a `RecordMerge` info on the mod whose change won (the last toucher), with the full touch
  chain, so `mods report --mod X` shows X's record conflicts. `render_report` still appends the
  per-kind `ConflictReport` table.
- **Native loader**: `sf_mods::NativeRecords` (adapter name `native`, extensions `ron`/`toml`)
  reads through `cx.files` (the mod's own layers), strips a UTF-8 BOM, and applies into the
  mod's source. It registers no kinds; hosts pass it in `adapters` next to their content pack.

Not in A2: real-install check of a RON patch over a vanilla record (needs the TES3 record types
to derive serde and be registered with `register_serde`, A3); hot re-apply (A4); legacy plugin
field-level diffs (X1: they would produce `fields` lists, and the conflict logic already uses them).

### As built (A3, 2026-10-13)

The first content pack on the pipeline (Bevyrrowind's `mw_mods` crate; the framework side is
the hooks in "Interfaces"). Every app command now loads through a profile; the pre-mods path
(`mw_vfs::mount_install` + `mw_records::load`) remains only for tests and examples.

- **Synthesized mods.** One mod per content file found in the data folder (headers only: the
  first record's size is read from its 16-byte record header), id = file name lowercased with
  other characters mapped to `_`, masters as dependencies, the adapter's known ordering facts as
  `load_after`, `content.records = [<file>]`; its layer is the archive the game's config lists
  with the **same stem** (the base game's archive goes with its master). A disabled plugin takes
  its archive with it (the MO2 behaviour for paired archives); listed archives no plugin claims
  go to the loose-data mod. The loose data folder is one more synthesized mod, ordered after
  every plugin by `load_after` (hints to mods that aren't enabled are ignored), mounted by the
  adapter as a `DirSource` under its original label so stat keys and labels stay unchanged.
- **Builtin mod** (`mw`): layers the host passes in (the shipped animation graphs, the legacy
  script runtime at `scripts/mw/mw.luau` + the generated `scripts/mw/functions.luau`). **Pinned
  lowest** by putting it first in every profile it loads with (it has no dependencies, so the
  ordering keeps it first); no framework rule was needed. It can't be disabled from the CLI.
- **Default profile**: without `profiles/default.toml`, the adapter generates one (builtin, the
  game config's content files masters first, the loose-data mod), so a fresh install loads what
  it loaded before mods existed (62,025 live records, same layers). `mods enable|disable` write
  the file. User data: `--user-data`, else `BEVYRROWIND_USER_DATA`, else `<local data dir>/bevyrrowind`
  (`mods/`, `profiles/`).
- **Record sources are named by mod id** (`morrowind.esm`, was `Morrowind.esm`). The adapter
  keys land-texture slots by the source name, and placement ids lowercase the file name, so ids
  and every bake product stay the same for files whose names are already valid ids.
- **TES3 kinds are serde kinds** (the `tes3` crate's own `serde` feature; `register_serde` for
  every tag), so native RON patches reach vanilla records and their fingerprints are blake3 of
  postcard bytes. Cells serialize their references sorted, so CELL is fingerprintable now (the
  `Debug`-hash scaffold in the bake inputs is gone). The switch rebuilt nothing: no step reads
  TES3 records as direct `Input::Record`s yet.
- **Profile fingerprint** = blake3 over the load-ordered (id, version, manifest `Debug`), each
  synthesized mod's adapter `stamp` (adapter version; archives' size + mtime; builtin layer
  bytes), and (path, size, mtime) of every file in every directory layer. The stats come from
  the **directory listing** (`VfsSource::stat`, kept by `DirSource` at scan): stat-ing 7,218
  loose files one by one cost 375 ms on Windows, the listing costs nothing extra (4 ms to hash).
  Stored beside the manifest (`profiles/<profile>.fingerprint`) rather than in it, so manifests
  written before A3 still load. `bake` always runs the incremental bake (step code changes aren't
  in the fingerprint, so a match can't skip it in dev) and prints whether the fingerprint
  changed; `mods report` prints whether the last bake is current.
- **Shadow warning**: `modern_alternates` (Bevyrrowind: ktx2/png over dds/tga/bmp, glb/gltf over
  nif) drives `ShadowedModern` on the modern file's mod when a later mod's legacy file wins.
- **CLI**: `bevyrrowind [--profile P] [--user-data D] mods list | order | report [--mod ID]
  [--conflicts N] | enable ID [--after ID] | disable ID`.

Real install (owner's Steam install, release, 2026-10-13): the vanilla profile loads 62,025 live
records; load = scan 1 ms + mount 30 ms + records 245 ms + fingerprint 4 ms (289 ms total, the
old path took ~420 ms); a RON patch `patch(kind: "LIGH", id: "light_com_candle_01", set:
{"data.radius": 64})` in a folder mod applies (`record LIGH light_com_candle_01` shows radius 64)
and `mods report --mod dim_candles` shows `morrowind.esm (define) -> bloodmoon.esm (define) ->
dim_candles (patch data.radius)`; a loose texture mod wins in `which`; adding a texture-only mod
changes the fingerprint and re-bakes nothing. Release `bake` no-op: 3.0-3.2 s total, of which
loading content is 0.29 s; the rest is other steps' hit checks and reports (exterior source
0.52 s, models 0.33 s + their summary 0.37 s, interiors 0.25 s, scripts 0.18 s).

Open points: a field patch over a record from a mod it doesn't depend on reports as a
**conflict** (A2's rule treats the earlier define as a whole-record write); declaring the
dependency silences it. Whether a later field patch over an earlier define should count as a
merge instead is for X1. Every ESM-vs-ESM conflict is now a per-mod warning (3,165 for vanilla),
so the CLI prints counts and `mods report` the details. Mount problems (an unreadable archive)
are adapter-side warnings the host prints, as `ContentAdapter::mount` has no warning channel.

### As built (A4, 2026-10-13)

Hot reload, dev builds only (Bevyrrowind's `dev` feature turns on `sf_vfs/watch`; release has no
watcher and serves a fixed snapshot).

- **Watcher crate: `notify-debouncer-full` 0.7.0 over `notify` 8.2.0**, the pair Bevy 0.20's
  `file_watcher` uses (one copy in the tree; both the newest stable on crates.io, 2026-10-13;
  `notify` CC0-1.0 OR MIT OR Apache-2.0, the debouncer MIT OR Apache-2.0). **Not Bevy's `FileWatcher`
  itself** (the draft's step 1): it maps one directory to one asset source and forwards raw
  directory events, while our default source is one VFS over many layers where an edit under a
  higher layer must emit nothing. We use the same crate directly in a Bevy-free `sf_vfs::watch`
  and connect to Bevy only at the event channel. Windows: `ReadDirectoryChangesW`; atomic saves
  (temp file + rename) and buffer overflows ("rescan" events) are covered because a batch only
  marks layers dirty and the rebuild diffs rescanned listings. Debounce 150 ms.
- **Snapshot swap: `arc-swap` 1.9** (`SharedVfs`): readers `load()` an `Arc<Vfs>` and keep it as
  long as they want a consistent view; the watcher `store()`s the next one. Lock-free both ways
  (a `RwLock` would make readers wait out a writer). Consumers that still hold a startup
  `Arc<Vfs>` (Bevyrrowind's model/actor loaders) keep working: loose files are read from disk
  through any snapshot, so they see edits, only not added/removed files.
- **Rebuild = rescan the dirty directory layers, share the rest.** `VfsSource::watch_root` and
  `rescan` (defaulted; `DirSource` implements both, re-reading its listing, which also refreshes
  the A3 stats). `Vfs::rebuild(dirty)` keeps every layer position (mods' layer ranges stay
  valid) and diffs only paths of rescanned layers: a path changed when its winner's layer or
  indexed stat (size, mtime) changed. Adding/removing mods, archives and legacy plugins still
  need a restart.
- **Bevy:** `shared_asset_source(SharedVfs, AssetEvents)` reads the current snapshot and, when
  Bevy watches (`file_watcher` feature or `watch_for_changes_override`), registers Bevy's
  `async_channel` sender through `AssetSourceBuilder::with_watcher`; the watcher sends
  `Added/Modified/RemovedAsset` (or `...Meta` for `<path>.meta`) for each winning change. Paths
  are VFS paths (lowercase), which is how Bevyrrowind loads textures, so the asset server matches them.
- **Routing:** `LoadedProfile::route(changes) -> HotReloadPlan { scripts, records, styles, assets }`
  (record and style files by the mods' declared paths, scripts by `scripts/**.luau|lua`).
  `sf_script_bevy::Scripts::reload_changed(paths, source)` reloads every context that runs a
  changed file or can `require` it as a dependency library (snapshot → new state → restore:
  storage and timers kept). `LoadedProfile::reload_records(vfs, adapters)` re-runs step 4 into a
  new DB (whole DB: patches stack in load order) and replaces the report's record issues; the
  host publishes it (Bevyrrowind: `LiveRecords` + `RecordsChanged`; no consumer opts in yet, so
  placed objects keep their data until their cell reloads, as documented).
- **Not built (handoff):** dev re-bake of a changed bake input during play (owner decision 4,
  provisional): Bevyrrowind logs "bake input changed: run `bevyrrowind bake`" for `.nif`/`.kf`;
  the CSS route waits for U2's composer.

## Industry comparison

2026-10-13; "unverified" marks memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Virtual file system over mods | MO2 (USVFS hooks a virtual `Data` folder); OpenMW `data=` dirs, later wins (docs); Bethesda: loose files beat BSAs | `sf_vfs` layers in load order, provenance per path | **match** |
| Load order resolution | Vortex/LOOT: rules + masterlist sorting; Bethesda: plugin list, masters first; Factorio: dependency graph, `?` optional, `!` incompatible, `~` no order (unverified syntax); Fabric: `depends`/`recommends`/`breaks` | dependency graph + hints + profile order; `[optional]`, `[conflicts]` added | **match** |
| Conflict view | MO2 per-file winner/loser view; xEdit/TES5Edit per-record conflict view; TES3Merge/Wrye Bash merged patches | per-file overrides, dependency-aware record and field conflicts, automatic field merges | **match**, field-level merge built in instead of a separate tool |
| Record patching | Bethesda plugins override whole records; Skyrim SKSE/"Synthesis" patchers run code to merge (unverified); Paradox: whole-file or key overrides | explicit set/merge/remove/append patches | **match/better** |
| Scripted mods | Factorio per-mod Lua; OpenMW `.omwscripts` + Lua; Skyrim Papyrus `.pex` per form | `content.scripts` per mod, state per mod | **match** |
| Mod-provided UI styling | Skyrim SkyUI (swf replacement); WoW XML/Lua; OpenMW Lua UI | CSS layers in load order + slots (ui.md) | **match** |
| Hot reload | OpenMW `reloadlua` (loose scripts only); Bevy asset hot reload; UE Live Coding (code) | per-kind routing over one VFS watcher | **match** |
| Derived data per load order | Bethesda: precombines/LOD generated by tools (xLODGen/DynDOLOD) and shipped as files; MO2 output mods | bake per profile, content-addressed, incremental | **differ, by design** (bake.md) |
| Profiles | MO2 profiles (mod list + ini per profile); Vortex profiles | `profiles/<name>.toml` with settings and limits | **match** |
| Archives | BSA/BA2; Factorio zip; OpenMW BSA/BA2 | adapter archives now; native zip later | **gap (P2)**: zip mods after M5 |

