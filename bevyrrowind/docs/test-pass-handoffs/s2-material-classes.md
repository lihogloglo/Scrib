# S2 material classes: written without compiling (final state)

Agent session 2026-11-08, static-scene.md slice S2. Not committed. Owner decision mid-session: no more builds on
the laptop; the last edits below were made without compiling.

## Build status at stop
- `check -p sf_diag -p sf_world_bevy -p mw_game_core -p mw_exterior_bevy --tests`: **passed** (after fixes).
- `check -p bevyrrowind --no-default-features`: **passed** (default features not checked: Delirium redirect).
- `test` on the same crates: **killed** by the owner's stop (no result).
- `clippy --all-targets --no-deps -D warnings`: one error, `type_complexity` on the `changed` query of
  `track_static_materials`; **fixed without compiling** (type alias `ChangedStaticMaterial`). sf_world_bevy test
  targets and mw_exterior_bevy were not linted (blocked by that error).
- Edited after the last successful check: only that alias (matclass.rs ~235-243).

## Files changed
- `superfargoth/crates/sf_world_bevy/src/matclass.rs` (new): `traits_of`, `class_of`, `class_key`, `PipelineKey`,
  `fold` + `Split`/`Folded`, `StaticPart` marker, `StaticMaterialTracker` + `track_static_materials` (main world),
  `StaticMaterialIds` (manual `ExtractResource<RenderApp>`), `count_static_bind_groups` (render world); 4 tests.
- `sf_world_bevy/src/{lib.rs,spawn.rs,stream.rs}`, `Cargo.toml` (+ `sf_scene` dep): `StreamingConfig::material_classes`
  (default true), `build_template` folds, `StaticPart` in `spawn_template_ranges`, systems + `SharedImageSamplers` in
  `finish`.
- `superfargoth/crates/sf_diag/src/views.rs`: `StaticMaterials`, `StaticBindGroups`, `statics` gauge names; 1 test.
- `superfargoth/vendor/bevy_render/src/texture/gpu_image.rs`: patch "own: shared image samplers"; vendor README row.
- `bevyrrowind/crates/mw_game_core/src/perf.rs`: `Part::MatClass` ("matclass", ALL 53 -> 54).
- `bevyrrowind/crates/mw_exterior_bevy/src/exterior.rs`: `streaming.material_classes = !perf.off.has(Part::MatClass)`.
- Docs: static-scene.md S2 row, perf.md "Static material classes (S2)", visual-checks/rendering.md SS-matclass +
  index line (44 awaiting), CHANGELOG line.

## Owner's commands for 2026-11-10 (repo root, default features)
```sh
bash bevyrrowind/tools/cargo_turn.sh check -p sf_diag -p sf_world_bevy -p mw_game_core -p mw_exterior_bevy -p bevyrrowind --all-targets
bash bevyrrowind/tools/cargo_turn.sh test -p sf_diag -p sf_world_bevy -p mw_game_core -p mw_exterior_bevy
bash bevyrrowind/tools/cargo_turn.sh clippy -p sf_diag -p sf_world_bevy -p mw_game_core -p mw_exterior_bevy -p bevyrrowind --all-targets -- -D warnings
cargo hakari generate --diff && cargo hakari manage-deps --dry-run   # sf_world_bevy gained sf_scene (hakari final-exclude, should be a no-op)
cargo build --profile profiling -p bevyrrowind --features dlss
target/profiling/bevyrrowind.exe perf suite s2_matclass balmora seyda_neen red_mountain run balmora_walk flight \
    --repeat 3 --resolution 2560x1440 --env BEVYRROWIND_DLSS=quality --save benchmarks/runs/s2_matclass
target/profiling/bevyrrowind.exe perf suite s2_matclass_off balmora seyda_neen red_mountain run balmora_walk flight \
    --repeat 3 --resolution 2560x1440 --env BEVYRROWIND_DLSS=quality --off matclass --save benchmarks/runs/s2_matclass_off
```
Tests to see pass: `matclass::tests::{traits_follow_alpha_and_culling, one_key_per_class_for_representative_statics,
unused_variant_fields_fold_and_look_changes_split, tracker_counts_live_static_materials}`,
`views::tests::static_material_gauges`, `perf::tests::off_parses_aliases_dedups_and_labels`.
Visual check: SS-matclass (Balmora, Seyda Neen 14:00, default vs `--off matclass`).

## APIs I was unsure of (compiled, behaviour unverified)
- `RemovedComponents` in a bare `World` + `Schedule` (test `tracker_counts_live_static_materials`): despawns must be
  seen by the second `schedule.run`. If the test fails there, call `world.flush()` / use an `App` with `update()`.
- `bevy_render::extract_resource::extract_resource::<StaticMaterialIds, RenderApp, ()>` added to `ExtractSchedule`
  in `finish` (instead of `ExtractResourcePlugin`, which logs an error without a render app).
- `RenderMaterialBindings` holds every prepared material's `MaterialBindingId { group, .. }` keyed by
  `UntypedAssetId` (read in `material_bind_groups.rs` ~60-75, 324); `MaterialBindGroupAllocators` keyed by
  `TypeId::of::<StandardMaterial>()` (`add::<M>` ~2738).
- Counter system in `RenderSystems::Render` after `render_system` (as S0's `count_views`).
- `Option<SRes<SharedImageSamplers>>` as a `RenderAsset::Param` element (compiled; prepares unchanged when absent).
- `--rt` path: `switch_path` mutates materials; folded materials keep `opaque_render_method` (Auto) so unaffected.
- Whether StandardMaterial is bindless on the owner's device: read the `statics/bindless` gauge (needs binding-array
  features and >= 6 x 2048 sampler elements; DX12 lacks it).
