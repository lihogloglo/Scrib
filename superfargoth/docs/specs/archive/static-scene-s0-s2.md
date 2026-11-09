# Static scene: as-built history, S0-S2

Moved out of [static-scene.md](../static-scene.md) on 2026-11-09 (doc budget, 25 KB). The spec's "Slices" table keeps
one short state per slice and points here. Append-only; grep it.

Verification state of every row below: built and tested on the laptop (ThinkPad X13, Rust 1.98.1), re-checked on
2026-11-09 at `main` 80c9a2c with **default features** (Delirium on) — `sf_scene` (`--features bake`),
`sf_world_bevy`, `mw_bake`, `sf_diag`, `sf_diag_bevy --features views` all compile and are clippy-pedantic clean
with `-D warnings`, 191 tests pass. Nothing here has been run in game or looked at by the owner; the owner's
remaining items are listed per slice.

## S0 as built (2026-11-08): per-view render counters

Counters for per-view instances, work items, batch sets, bind groups and pipeline states in the perf report:
`sf_diag::views` (Bevy-free sums), `sf_diag_bevy::RenderViewsPlugin` (feature `views`), a vendored
`bevy_render`/`bevy_pbr` counter patch (`PreprocessBindGroupCount`), and the report columns. Spec:
[diag.md](../diag.md) "Render views (static-scene S0)".

- Laptop 2026-11-08: `sf_diag` 46 tests, `sf_diag_bevy --features views` 11 tests; clippy pedantic clean on
  `sf_diag`, `sf_diag_bevy`, `mw_devtools_bevy` and `bevyrrowind`. That `bevyrrowind` check used
  `--no-default-features` (Delirium off), which exercises none of the Delirium code paths the shipped build has;
  **re-done 2026-11-09 with default features** (see above).
- Two fixes made after compiling: the vendored `prepare_preprocess_bind_groups` hit Bevy's 16-parameter system limit
  (the counter moved into a tuple parameter), and three `HashSet` type annotations.
- Hakari: `sf_diag_bevy` gained optional `bevy_pbr` / `bevy_core_pipeline`.
- **Open for the owner:** run `bevyrrowind/docs/perf.md` "Static scene baseline (S0)" ×3 and paste the summary table
  back there. The caveats on how to read those numbers (skipped views, no `--off views`, `cascade0` overloading,
  per-label sums) are listed with the recipe in `perf.md`; they are known issues to fix in code, not design intent.
- Handoff: `bevyrrowind/docs/test-pass-handoffs/s0-instrument.md`.

## S1a as built (2026-11-08): `sf_scene` core

New Bevy-free crate `sf_scene`: table format v1 (documented in `table.rs`), the eligibility trait + `Promotions`, the
slot allocator, the CPU cull/LOD twin, and `material.rs` with the 8 material classes and the orchestrator.

- Laptop: 28 tests pass (30 counting the orchestrator tests added the same day), clippy pedantic clean after 7 lint
  fixes made post-compile.
- `sf_scene` was added to hakari's final-excludes by hand; the hakari check was left to the owner.
- S1b and S1c then edited `chain.rs`, `bake.rs`, `table.rs` and `eligibility.rs`, so the S1a tree as such was never
  the tree that shipped — read the S1c row for the format that did.
- Handoff: `bevyrrowind/docs/test-pass-handoffs/s1a-scene-core.md`.

## S1b as built (2026-11-08): `scene.cells` bake + adapter hooks

`sf_scene::bake` (`CellsStep`, `SceneBake`, `SceneSource`, the pure `build_cell_table`, `land_min`),
`sf_scene::chain_layout`, and the Morrowind adapter `mw_bake::scene` (`MwSceneSource`, `scene_material`,
`bake_scene`) — the `scene.cells` bake step plus the adapter's eligibility hook and material class map.

- The S1b code **as written never compiled**: format v2 (S1c) superseded it the next day, and the first successful
  build of this area was the combined S1b+S1c tree of 2026-11-09 (see the S1c row).
- **Open for the owner:** the Vvardenfell bake measurement
  (`test --release -p mw_bake --test scene_cells_real_install -- --ignored --nocapture`, needs the install) against
  the `bake.md` gates, then the one `bake_scene` line in `mw_boot_bevy::bake` that actually wires the step into the
  game's bake. Until that line exists, no table is ever produced by `bevyrrowind bake`.
- Handoff: `bevyrrowind/docs/test-pass-handoffs/s1b-scene-bake.md`.

## S1c as built (2026-11-09): table format v2

Review fixes taken before any real bake, all in the spec's §2 "Format v2 (S1c)": cell-local **dense chain indices**
with `chain_index` / `write_remapped` for stream-in; a chain record carrying the mesh key; a material key
(`bake::material_key`) and class; `material_override` per instance; layers in the header; per-part spheres
(`bake::part_spheres`); scale-aware LOD in the CPU twin (§4 step 5); `scene.cells` step version 2.

- Laptop: `sf_scene` 41 tests, 50 with `--features bake`; `mw_bake` 48 lib tests (4 of them scene) and, over the whole
  crate, 68 passed / 12 ignored. Clippy pedantic `--all-targets --no-deps -D warnings` clean on `sf_scene` with and
  without `bake` and on `mw_bake`.
- The S1b Vvardenfell measurement now prints distinct material keys instead of id collisions.
- No new dev edges for hakari (`sf_scene`'s tests already used `sf_model::cook` under `bake`).
- **Open for the owner:** as S1b (the Vvardenfell measurement, then the `bake_scene` line).
- No handoff file: the state of 2026-11-09 is recorded in `bevyrrowind/docs/test-pass-2026-11-10.md`.

## S1c follow-ups (open, not scheduled)

- Drop `ALPHA_MASKED` and `WIND` from the instance flags: both are pure functions of the chain.
- Define the `material_override` target (a per-cell override section would be format v3).
- Decode LOD0 only for eligible models.

## S2 as built (2026-11-08): material classes for statics

`sf_world_bevy::matclass` folds every static template material into its material class — traits from alpha mode and
culling, wind off; unused UV channels, parallax and sub-unit depth bias canonicalised. Today's mapper already keys on
the class except **leaf transmission**, which stays split (one key for all masked materials would light non-leaves
with a second lobe and render them forward under `--rt`), as would normal maps, depth bias and other UV sets — none of
which statics use today. Vendored `bevy_render` `SharedImageSamplers` (one sampler per descriptor, because bindless
slabs count samplers). Gauges `statics/*` (materials, classes, keys, pipelines, bind groups, bindless). A/B switch
`--off matclass`, default on.

- Files: `sf_world_bevy/src/matclass.rs` (new) plus `lib.rs`, `spawn.rs`, `stream.rs`, `Cargo.toml` (+ `sf_scene`
  dep); `sf_diag/src/views.rs` (`StaticMaterials`, `StaticBindGroups`, `statics` gauge names);
  `superfargoth/vendor/bevy_render/src/texture/gpu_image.rs` (patch "own: shared image samplers") + the vendor README
  row; `mw_game_core/src/perf.rs` (`Part::MatClass`); `mw_exterior_bevy/src/exterior.rs`
  (`streaming.material_classes = !perf.off.has(Part::MatClass)`).
- As written on 2026-11-08 this slice **was never compiled**: the owner stopped builds mid-session, and the last edit
  (a `ChangedStaticMaterial` type alias fixing a `type_complexity` clippy error) was made blind. It was compiled,
  tested and linted on 2026-11-09 with default features (see above), so the "no build, no tests" state recorded in the
  original handoff is history.
- **Open for the owner:** the A/B perf run (`perf.md` "Static material classes (S2)", default vs `--off matclass`) and
  visual check **SS-matclass** (`visual-checks/rendering.md`, Balmora and Seyda Neen 14:00).
- Handoff: `bevyrrowind/docs/test-pass-handoffs/s2-material-classes.md` (written before the build; its "Build status at
  stop" section is superseded by the 2026-11-09 result above, its "APIs I was unsure of" list is still worth reading).
