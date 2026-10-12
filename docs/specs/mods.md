# Mods: pipeline end to end

Status: draft (2026-10-13, m5-spec session, docs only). Built in M5 on the M0 foundations.
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
records = ["records/items.ron", "legacy/lanterns.plugin"] # native RON, or adapter formats by extension
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

```rust
// sf_mods
pub struct ModRepository { /* manifests by id, roots, issues */ }
pub struct Profile { pub name: String, pub enabled: Vec<ModId>, pub settings: SettingValues, pub limits: ScriptLimits }
pub struct LoadOrder { pub mods: Vec<ModManifest> }          // resolved
pub struct ModReport { pub issues: Vec<ModIssue> }           // + per-mod VFS and record summaries
pub trait ContentAdapter {                                   // implemented by content packs
    fn discover(&self, install: &InstallInfo) -> Vec<ModManifest>;      // synthesized mods
    fn mount(&self, m: &ModManifest, vfs: VfsBuilder) -> VfsBuilder;     // archives etc.
    fn apply_records(&self, m: &ModManifest, file: &VfsPath, db: &mut RecordDb, src: SourceId) -> Result<(), RecordFileError>;
}
pub fn load_profile(repo: &ModRepository, profile: &Profile, adapters: &[&dyn ContentAdapter]) -> LoadedProfile; // steps 1-4
// sf_vfs (bevy feature): VfsWatcher, SharedVfs (swappable)
// sf_records: KindRegistry, apply_record_file(), field-level Touch
```

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
