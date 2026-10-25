# Mods: pipeline end to end

Status: A4 built (2026-10-13, m5-a4: VFS watcher, snapshot swap, hot-reload routing; see "As built (A4)"). A1 built (2026-10-13, m5-a1: repository, profiles, `ContentAdapter`, `load_profile`, report; see "As built (A1)"). A2 built (2026-10-13, m5-a2: kind registry, native RON/TOML record files, generic field patches, field-level conflicts; see "As built (A2)"). A3 built (2026-10-13, m5-a3: Bevyrrowind's adapter, builtin mod, mods folder and profiles, `mods` CLI, profile fingerprint; see "As built (A3)"). Draft 2026-10-13 (m5-spec). Built in M5 on the M0 foundations.
Crates: `sf_mods` (manifests, repository, profiles, load order, report; built M0, extended),
`sf_vfs` (layers; built M0, gains a watcher and swap), `sf_records` (built M0, gains native
patch files and field-level conflicts), `sf_bake` (profile fingerprint), `sf_script`
([scripting.md](scripting.md)), `sf_ui_css` / `sf_ui` ([ui.md](ui.md)). Adapter side
(Bevyrrowind): legacy content as synthesized mods (`mw_records`, `mw_vfs`, `mw_install`).

## Goal

ARCHITECTURE §6: **everything is a mod**, freely combining asset overrides, record patches, Luau
scripts, UI styles and layouts, and legacy content. This spec joins the M0 pieces into one
pipeline that a player drives with a profile and a modder with a folder:

1. drop a folder with a `mod.toml` into the mods directory, enable it in a profile;
2. the game validates it, orders it, mounts its files, applies its records, starts its
   scripts, composes its styles, and re-bakes only what it changes;
3. every conflict and error is reported with the owning mod;
4. in dev builds, edits to its files apply live.

Non-goals (M5): a mod downloader/portal, a graphical mod manager (ui.md lists the screen; M5
ships the CLI report and the settings screen's mod-settings pages), archive mods (`.zip`, later
task), ESP field-level auto-merge (adapter task, may slip to M7).

## Canonical technique

**Manifest + dependency-aware load order + layered virtual file system + layered record
database with provenance**, the shape shared by Bethesda/MO2 (virtual `Data` folder, plugin
load order, conflict view), OpenMW (`data=` directories as VFS layers, `content=` list),
Factorio (`info.json` with versioned, optional and incompatible dependencies; per-mod Lua) and
Fabric/Minecraft (`fabric.mod.json`, `depends`/`breaks`). Our additions: **field-level record
patches** (what TES3Merge/Wrye Bash "bashed patches" do after the fact, built in), **CSS cascade
layers in load order** for UI (ui.md), and a **content-addressed bake** keyed by fingerprints
so changing mods re-derives only what they touch (bake.md).

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

## Data model

### Manifest additions (`mod.toml`)

```toml
id = "better_lanterns"
name = "Better Lanterns"
version = "1.2.0"
api = "^0.1"                                  # framework scripting/API version (scripting.md)

[dependencies]                                # required, ordered before this mod
my_game = "^0.5"                              # the content pack (gives access to its adapter API, e.g. @mw/*)

[optional]                                    # ordered before this mod when enabled (new)
hud_tweaks = "*"

[conflicts]                                   # refuses to load together (new; Factorio "!", Fabric "breaks")
old_lanterns = "<2.0"

load_after = ["some_texture_pack"]            # existing hints
load_before = []

[content]
assets  = ["assets"]                          # VFS roots, in order (existing)
records = ["records/items.ron", "legacy/lanterns.plugin"] # native RON, or adapter formats by extension; these and
                                              # the paths below are VFS paths inside the mod's asset roots
scripts = ["scripts/better_lanterns/main.luau"]          # entry points (scripting.md)

[ui]                                          # ui.md
styles = ["ui/better_lanterns/style.css"]
```

`[optional]` replaces the common use of `load_after` with a version check; `load_after` stays
for pure ordering. `deny_unknown_fields` stays on, so typos are errors.

### Mods directory and profiles

- `<user data>/mods/<id>/mod.toml` (loose folders). The adapter adds **synthesized mods** for
  legacy content it discovers (Bevyrrowind: the install's ESM/ESP files and their BSAs, plus
  one `morrowind-data` asset mod for the install's loose `Data Files`), and the content pack's
  own **builtin mod** (graphs, themes, translated-script runtime) mounted lowest.
- `<user data>/profiles/<name>.toml`: enabled mod ids in the player's preferred order, per-mod
  setting values (settings registry, ui.md), resource limits (scripting.md). The CLI and the
  future mod-manager screen write the same file (ARCHITECTURE §6.4).
- Repository: `ModRepository::scan(dirs, adapters) -> (Vec<ModManifest>, Vec<ModIssue>)`.

### One mod → one record source, one VFS group, one script context

| Pipeline stage | Per mod, in load order | Provenance kept |
|---|---|---|
| VFS | each `content.assets` root mounted as a `DirSource` labelled `<mod_id>:<root>` (adapter archives likewise) | `providers(path)` → mod ids |
| Records | one `SourceId` named by the mod id, `depends_on` = its dependencies' sources; record files applied in listed order | `Touch` history (+ fields, below) |
| Scripts | one context (scripting.md), entry points run in load order | context id = mod id |
| UI | stylesheets imported as `@layer mods.<id>` in load order; screens/slots/columns as records | layer name → mod id |
| Bake | inputs fingerprinted through the VFS and records, so products know which mod's bytes they used | traces |

### Native record files

RON (TOML accepted for simple cases) with explicit operations, per §6.1:

```ron
// records/items.ron
[
  define(kind: "item.light", id: "better_lanterns/lantern_blue", value: ( name: "Blue Lantern", radius: 6.0 )),
  patch(kind: "item.light", id: "lantern_01", set: { "radius": 5.0, "color": (0.9, 0.8, 0.6) }),
  patch(kind: "npc", id: "guard_01", append: { "inventory": [("lantern_01", 1)] }),
  patch(kind: "ui.column", id: "inventory.value", remove: ["width"]),
  delete(kind: "item.light", id: "lantern_ugly"),
]
```

- Kinds opt in through a **kind registry** in `sf_records`: name, Rust type (serde), and a
  fingerprint function (bake.md). Adapters register theirs (`mw_records` registers the TES3
  kinds under their tags).
- Patches apply on a generic value tree: the current record is serialized to a
  `ron::Value`-like tree, `set` (replace at a field path), `merge` (deep merge maps), `remove`
  (reset to default / drop map key), `append` (lists) are applied, and the tree deserializes
  back to `T`. One generic implementation for every kind; per-kind fast paths only if profiling
  asks.
- **Field-level touches**: `Touch` gains the field paths a patch wrote. A conflict is now
  "two sources not depending on each other wrote the same field (or one replaced the whole
  record after the other patched it)". Disjoint field patches merge silently and are reported
  as merges, not conflicts. This is TES3Merge/bashed-patch behaviour, built in.
- Adapter formats are applied by extension (`.esp`/`.esm` → `mw_records`), so a native mod can
  ship a legacy plugin.

## Pipeline (startup)

1. **Discover**: scan mods dirs, adapter discovery → manifests (+ issues: parse errors, bad ids,
   duplicate ids).
2. **Resolve**: profile → `resolve_load_order` (+ `[optional]`, `[conflicts]`, `api`
   compatibility against the framework's API version).
3. **Mount**: build the VFS (builtin, legacy archives, legacy loose, then mods in order).
4. **Records**: apply each mod's record files in order; adapters' formats via the kind registry.
5. **Fingerprint and bake**: compute the profile fingerprint; if it differs from the profile
   manifest's, run the (incremental) bake behind the loading screen (bake.md "Game start").
6. **UI**: build `UiRegistry` from records, compose stylesheets (ui.md).
7. **Scripts**: create contexts in load order, run entry points (scripting.md).
8. **Report**: `ModReport` (below) to the log, `sf_diag`, the CLI, and (later) the mod manager.

Steps 1–4 are Bevy-free and timed (Morrowind today: VFS 45 ms + records 371 ms for 3 files).
Gate: steps 1–4 + 6–7 ≤ 1 s for the vanilla profile + 20 small mods; cold start overall stays
in WISHLIST §0's < 10 s after the first bake.

## Validation and conflict reports

`ModReport` collects, per mod, typed issues with severity:

| Area | Errors (mod not loaded, or load refused) | Warnings / info |
|---|---|---|
| Manifest | parse error, invalid id, duplicate id, unknown field | missing description |
| Order | missing dependency, version mismatch, cycle, declared conflict present, `api` incompatible | optional dependency missing (info) |
| Files | a declared root/record file/entry point missing | empty asset root |
| VFS | — | per mod: files it adds, files it overrides and whose (MO2 conflict view); a legacy+modern pair shadowed by a later legacy file (`resolve_preferring` edge) |
| Records | record file parse error (file, line), unknown kind, patch of a missing record, field path that doesn't exist | conflicts (field-level, dependency-aware), merges, deletions of records others patch |
| Scripts | compile error (file, line), entry point error, `require` of a non-dependency | deprecated API use, slow handlers, suspended contexts (runtime) |
| UI | — | CSS rule errors attributed by layer (ui.md), screen replacement conflicts |
| Adapter | — | legacy translation diagnostics (scripting-adapter.md) |

Rendered as text (`<game> mods report [--profile P] [--mod ID]`) and as data (serde) for the
UI. The existing `ConflictReport::render` stays the record section.

## Hot reload (dev builds; player opt-in "mod author mode")

**Bevy first:** Bevy's `file_watcher` (notify, already in Bevyrrowind's tree via Bevy: `notify`
8.2.0 + `notify-debouncer-full` 0.7.0) watches directories and emits `AssetSourceEvent`s; asset
handles hot reload from those events. Our VFS is one asset source over many directories, so:

1. `sf_vfs` (bevy feature) gets a watcher that watches each `DirSource` root of the current
   VFS with Bevy's `FileWatcher` machinery, debounced.
2. On a change, the VFS rebuilds the index for that layer (create/delete can change winners;
   rebuilding the whole index is cheap: 20,955 files in 45 ms including BSA reads, the loose
   part is a fraction) and **swaps** the shared `Arc<Vfs>` (the reader holds an
   `RwLock<Arc<Vfs>>` or `arc-swap`; readers clone the `Arc` and never block).
3. It emits `ModifiedAsset`/`AddedAsset`/`RemovedAsset` for paths whose **winning** file
   changed (a change shadowed by a higher layer emits nothing).
4. Routing by kind:

| Changed file | Effect |
|---|---|
| Bevy assets loaded by path (textures, sounds, fonts, `.bsn`) | Bevy's own hot reload |
| `.css` | flair re-parses; composer re-imports (ui.md) |
| scripts | owning context reloads (scripting.md "Hot reload") |
| native record files | the source is re-applied: records rebuilt (the whole DB in dev: ~0.4 s for Morrowind), `RecordsChanged { kinds }` message; record consumers that opt in (UI registry, settings, weather presets) rebuild; placed world objects do not respawn (documented: reload the cell) |
| a bake input (model, terrain texture) | dev: re-run the affected products on a background task and swap them in (sf_bake traces find them); release: reported, applied at next start |

Legacy archives (BSA) and legacy plugins are not watched (restart), like OpenMW's `reloadlua`
not reloading packed scripts.

## Bake cache interaction

The bake (bake.md) already keys every product on input fingerprints and keeps one manifest per
profile. Mods add:

- **Profile fingerprint** = blake3(load-ordered (mod id, version, manifest hash), adapter
  versions, VFS layer stat keys, record source stat keys). Stored in the profile manifest. At
  startup a match means "no bake needed" (cold start reads only stats: bake.md "Cold start");
  a mismatch runs the incremental bake, whose traces rebuild only products whose inputs changed.
  A texture-only mod changes no product unless a step reads that texture (terrain layer arrays
  do; cooked meshes store texture paths, not bytes).
- **Mod-authored products** use the existing override paths ("authored beats derived").
- **Translated legacy scripts** are bake products (scripting-adapter.md), keyed on the script
  record's fingerprint and the translator version.
- **Native Luau scripts are not baked**: Luau compiles source in microseconds per file at load;
  a bytecode cache can be added if context start misses its gate.
- Profiles share objects; switching profiles back and forth costs nothing after the first bake.

## Interfaces

As built in A1 (2026-10-13); the draft's `LoadOrder` became `LoadedProfile::mods`:

```rust
// sf_mods
pub struct ModRepository { pub issues: Vec<ModIssue>, pub scan_time: Duration, /* manifests, pre-scanned roots, invalid set */ }
impl ModRepository { pub fn scan(mod_dirs: &[PathBuf], adapters: &[&dyn ContentAdapter]) -> Self; pub fn add(&mut self, m: ModManifest); }
pub struct Profile { pub name: String, pub enabled: Vec<ModId>, pub settings: SettingValues, pub limits: ScriptLimits }
pub struct ModReport { pub issues: Vec<ModIssue>, pub mods: Vec<ModSummary> }  // serde; render(only: Option<&ModId>)
pub trait ContentAdapter {                                   // implemented by content packs; every method defaulted
    fn name(&self) -> &str;
    fn discover(&self, issues: &mut Vec<ModIssue>) -> Vec<ModManifest>;              // synthesized mods
    fn mount(&self, m: &ModManifest) -> Result<Vec<Arc<dyn VfsSource>>, String>;     // archives etc., lowest first
    fn record_extensions(&self) -> &[&str];                                          // record files it applies
    fn register_kinds(&self, db: &mut RecordDb);
    fn apply_records(&self, cx: &mut RecordCx<'_>, file: &VfsPath) -> Result<(), RecordFileError>;
    fn finish_records(&self, db: &mut RecordDb);                                     // adapter-derived sources
}
pub fn load_profile(repo: &ModRepository, profile: &Profile, adapters: &[&dyn ContentAdapter], api: &Version) -> LoadedProfile; // steps 2-4
pub struct LoadedProfile { pub profile: Profile, pub mods: Vec<LoadedMod>, pub vfs: Arc<Vfs>, pub records: RecordDb, pub report: ModReport, pub timings: LoadTimings }
impl LoadedProfile { pub fn script_contexts(&self) -> Vec<ScriptPlan>; pub fn stylesheets(&self) -> Vec<(ModId, VfsPath)>; pub fn render_report(..) -> String; }
// sf_vfs (A4): SharedVfs (arc-swap), Vfs::watch_roots / rebuild(dirty) -> (Vfs, Vec<VfsChange>), VfsSource::watch_root / rescan,
//   watch::VfsWatcher::start(shared, debounce, on_event) (feature `watch`), bevy::shared_asset_source(shared, AssetEvents)
// sf_mods (A4): LoadedProfile::route(changes) -> HotReloadPlan, LoadedProfile::reload_records(vfs, adapters)
// sf_script_bevy (A4): Scripts::reload_changed(paths, &source) -> Vec<(context, Result)>
// sf_records (A2): RecordDb::register_serde / set_fingerprint / define_value / patch_value / delete_by_name / value_of / fingerprint,
//   apply_record_file(db, source, text, RecordFormat) -> Result<FileOutcome, FileError>, Value (serde tree), PatchOp, Touch { fields }
// sf_mods (A2): NativeRecords (ContentAdapter "native", .ron/.toml), RecordCx::error, IssueKind::{RecordConflict, RecordMerge}
// sf_mods (A3): ContentAdapter::stamp(&ModManifest) -> Vec<u8> / modern_alternates() -> &[(&str, &[&str])] (both defaulted),
//   LoadedProfile::fingerprint(adapters) -> [u8; 32], IssueKind::ShadowedModern { path, legacy, by } (Area::Vfs, warning)
// sf_vfs (A3): VfsSource::stat(path) -> Option<FileStat> (defaulted; DirSource keeps the listing's size + mtime), FileStat
// sf_bake (A3): Store::profile_fingerprint(profile) / save_profile_fingerprint(profile, &Digest)
```

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

## Verification

- `sf_mods` unit tests: repository scan, profile round trip, `[optional]`/`[conflicts]`/`api`
  checks, report contents for each issue type.
- `sf_records` tests: every patch op on a nested type; field-level conflict vs merge; patch
  of a missing record; RON error positions.
- `sf_vfs` tests: watcher-driven rebuild changes the winner; shadowed changes emit nothing;
  readers never block (stress test).
- Real install (Bevyrrowind): vanilla profile + a texture mod + a record-patch mod + a Lua mod:
  `mods report` lists exactly the expected overrides/merges; `which` shows the mod as winner;
  bake after adding the texture mod is a no-op except terrain layers if a land texture changed.
- Owner scenes MS-1..MS-3 (ROADMAP "M5").

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

## Open questions

1. Archive format for native mods (zip via the `zip` crate, or a seekable custom pack):
   after M5; loose folders cover modding and development.
2. Whether a record-file hot reload should also respawn affected loaded placements (cell
   reload) in dev; default no (documented).
3. Field-level ESP merging (§6.1 item 4: diff each ESP record against its master) is an adapter
   task in `mw_records`; schedule after the M5 core (ROADMAP M5 X1) or in M7.
