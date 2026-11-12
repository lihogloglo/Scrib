# Content pipeline: VFS, records, mods, adapters, bake

## What it does

Every piece of content (base game, DLC, mod) is a mod. Mods mount file layers into one VFS and apply record files into one
record database. Expensive derived assets (cooked meshes, textures, terrain, nav, far field, scripts) are produced once by
`bevyrrowind bake` into a content-addressed store and read by the game. Framework crates are game-agnostic (`sf_*`);
Morrowind formats plug in as adapters (`mw_*`).

## How it works

**VFS.** `VfsSource` (`superfargoth/crates/sf_vfs/src/lib.rs:37`) is the only extension point (loose dir `DirSource`, archives,
generated `mem.rs`). `VfsBuilder::mount` (`:101`) in priority order, last mounted wins; `build` (`:113`) makes an immutable
lock-free index. `Vfs::read` `:195`, `resolve_preferring` `:210`. Changed layers rebuild (`rebuild.rs`) and publish through
`SharedVfs` (`shared.rs`); `watch.rs` drives hot reload (feature `watch`).

**Records.** `RecordDb` (`sf_records/src/store.rs`) keeps the resolved value of every record plus provenance per source
(`Op` `:10`, `Touch` `:18`). Kinds are arbitrary Rust types; serde kinds also load from RON/TOML record files
(`apply_record_file`, `file.rs`), patched by field path (`PatchOp`, `patch.rs:14`), so the conflict report separates real
conflicts from disjoint-field merges (`report.rs`).

**Mods.** `ContentAdapter` (`sf_mods/src/adapter.rs:19`) synthesizes manifests and record loaders for legacy formats.
`ModRepository::scan`, `Profile`, `resolve_load_order` (`order.rs:37`, keeps profile order where constraints allow), then
`load_profile` (`load.rs:179`) mounts the VFS, applies records and returns `LoadedProfile` with a `ModReport`. Manifest:
`ModManifest` (`manifest.rs:53`). Host-side game wiring: `mw_boot_bevy/src/mods.rs`, hot reload `hot_reload.rs`.

**Morrowind adapters.**
- `mw_vfs`: `BsaSource` (`mw_vfs/src/lib.rs:17`) and `mount_install` (`:108`): `Morrowind.bsa`, then INI archives, then
  loose `Data Files`; loose beats archives.
- `mw_records`: ESM/ESP onto `RecordDb` (`load` `lib.rs:186`, `apply_plugin` `:256`); later files replace records, cells
  merge references (keyed by source id), INFO keyed under topic.
- `mw_mods`: `MwAdapter` (`mw_mods/src/lib.rs:329`), the builtin `mw` mod (shipped graphs, Luau runtime) pinned first.
- `mw_nif`: `parse` (`lib.rs:159`) and `convert` (`:164`) NIF to engine-neutral `sf_model::Model` (Y up, meters, properties
  inherited, controllers to tracks). `mw_bake::to_model` (`importer.rs:207`) adapts it for the bake.

**Bake.** `sf_bake` is a build system with constructive traces over a blake3 store (spec: Build Systems a la Carte).
`Step` (`sf_bake/src/lib.rs:93`) has `plan` (known inputs) and `run` (reads recorded through `RunCx`); `ProductKey`
(step + subject) `:45`; `Baker` `:335` runs keys in parallel (rayon) and reuses a product when planned inputs and every
dynamic read still match. `Store` (`store.rs:56`): `objects/ab/cdef..`, `traces.log`, `profiles/` manifests; `FileHashMemo`
(`memo.rs`) caches file hashes by stat. `VfsInputs` (`mw_bake/src/inputs.rs`) gives file and record fingerprints.
`bevyrrowind bake` (`apps/bevyrrowind/src/main.rs:126,750`) runs `mw_boot_bevy/src/bake.rs:23` in this order: models
(`model.info`, `mesh.cook`, `mesh.rest`, `mesh.rig`) + `collision.cook` (`mw_bake/src/lib.rs:151`), exterior
(`terrain.*`, `world.*`), nav (`mw.nav.cell/links`, skip with `BEVYRROWIND_BAKE_SKIP_NAV`), far field (`lod.*`), actors
(skeleton/anim cook, graph compile), item bodies, scripts (MWScript to Luau), de-light and texture cook
(`texture.*`), static scene (`scene.cells`, `scene.chains`). Cache root defaults to `<local data dir>/bevyrrowind/bake`.
It prints a `BakeSummary` and saves the profile manifest.

## Bevy features used

None in the logic crates (Bevy-free by design). Bevy side: `sf_vfs/src/bevy.rs` (asset source for `baked://` and VFS paths),
`sf_model_bevy` reads cooked products. See [Render](../bevy/render-and-visibility.md) for what the products become.

## Switches and counters

`BEVYRROWIND_BAKE_SKIP_NAV=1`; `--cache <dir>`; `BakeReport`/`StepStats` (`lib.rs:258,276`) count hits, runs and time per step.
Re-run bake after changing animation slots or sources.

## Known limits

Cold bake time of the nav steps is above the "few minutes" gate (scaffold skip above). Game logs: see
[streaming ledger](../../ledger/streaming.md) for load-side hitches.

## Specs

`superfargoth/docs/specs/bake.md` (Data model, Inventory, What bakes when, Execution), `mods.md` (Data model, Pipeline, Hot
reload, Bake cache interaction), `bake-mesh-cook.md`; `bevyrrowind/docs/specs/bake-adapter.md`, `nif-runtime.md`.
