# Brief C2a: the `--scene` switch turns the static scene on in the game

Coordinator: scene-coord, 2026-11-11. Spec: `superfargoth/docs/specs/scene-runtime-plan.md` §8 (`:705-729`, read only
that). One deliverable: **`bevyrrowind view ... --scene` runs the static scene instead of the precombine**, plus its
test. Headless only; never launch the game; don't commit (the coordinator does).

State: everything framework-side is built and headless-tested (`sf_scene_bevy::ScenePlugin`, `SceneHook`, B5a/B5b
promotion, O1 shift rewrite). `bevyrrowind bake` now bakes `scene.cells` + `scene.chains` (owner's cache has them).
The game never adds `ScenePlugin`. That is this slice.

## Coordinator rulings (don't revisit)

- **No `suppress_scene_placements`** (plan §8's first paragraph, `:707-712`): `CellScene::covered` already stops both
  the spawns and the model loads (`sf_world_bevy` test `scene_covered_placements_load_no_models_and_spawn_nothing`),
  and a spawner wrapper would also block B5a-2's promoted entity spawn. Skip the plan's last bullet (`:725`) too.
  Strike both from §8 in 2-3 lines saying why.
- Off by default. Interiors: out (`SceneConfig::worldspaces = Some(vec![exterior])`, the exterior worldspace id the
  app already has as `options.worldspace` when it is the exterior; if that is awkward, `None` is acceptable since only
  exterior cells have tables — say which you did).
- The GPU support gate (B5c) is a later slice: the owner's GPU supports GPU culling.

## What to change

1. `bevyrrowind/apps/bevyrrowind/src/main.rs`: `#[arg(long)] scene: bool` beside `lod_tint` (`:397`), threaded at both
   construction sites (`:479`, `:564`) into `ExteriorOptions` (`mw_exterior_bevy/src/exterior.rs:81`, new field `scene`).
2. `bevyrrowind/crates/mw_game_core/src/perf.rs`: `Part::Scene` with doc comment, `("scene", Part::Scene)` in
   `Part::ALL` (`:117`, count 54 → 55; fix any test that pins the count).
3. `bevyrrowind/crates/mw_exterior_bevy/src/exterior.rs`, in the block at `:265-292`:
   - `let scene = options.scene && !perf.off.has(Part::Scene);`, resolved **before** `add_plugins`.
   - `streaming.precombine` (`:279-280`) also false when `scene` (the two draw the same placements; never both).
     Put this rule plus the scene flag in one small pure function (e.g. `fn static_paths(scene_flag, off, lod_tint)
     -> (scene, precombine)`) so it is unit-testable; note `--off near` / `--off nif` / `--lod-tint` must turn the
     scene off too (the scene draws near statics; the NIF path and the tint view need every static as an entity).
   - `extensions.extend(sf_scene_bevy::baked_extensions());` next to `sf_lod_bevy`'s (`:212`), unconditionally (cheap,
     and the products may be read whenever the bake has them).
   - `app.add_plugins(sf_scene_bevy::ScenePlugin { config })` with `config.enabled = scene`, after
     `WorldStreamingPlugin` (it orders after `StreamingSystems::Drive` and inserts `SceneCells`). Check that the vendored
     `bevy_pbr` registers the GPU instance batch machinery under `DefaultPlugins` (grep `GpuInstanceBatch` in
     `superfargoth/vendor/bevy_pbr/src/lib.rs`); if a plugin must be added, add it only when `scene`, and say so.
   - Add `sf_scene_bevy` to `mw_exterior_bevy/Cargo.toml` (workspace dep). Then run
     `cargo hakari generate --diff` and `cargo hakari manage-deps --dry-run`; if either wants a change, apply it with
     `cargo hakari generate && cargo hakari manage-deps` and list the files it touched.
4. The F1 / `bevyrrowind/status` A/B reporting: if the app lists active parts or switches somewhere the owner reads
   (grep `Part::Precombine` across `bevyrrowind/` for the places that name it), add `scene` the same way. Nothing new.

## Test

A unit test of the pure function: `--scene` alone → scene on, precombine off; `--scene --off scene` → scene off,
precombine on; `--scene` with `--off near`, `--off nif` or `--lod-tint` → both off; neither flag → precombine on.
Plus whatever `Part::ALL` count test exists.

## Verify (one build at a time: `bash bevyrrowind/tools/cargo_turn.sh <args>`, output filtered to errors/summary)

- `test -p mw_game_core -p mw_exterior_bevy` and the app crate's tests (`-p bevyrrowind`).
- `clippy -p mw_game_core -p mw_exterior_bevy -p bevyrrowind --all-targets -- -D warnings`.
- `build --release -p bevyrrowind` (the owner's run binary; must succeed).

## Hand-off

Stop by ~80k tokens of context. Report the files changed, the pure function's name, the hakari result, test counts,
and anything that differed. Register in `.sessions/scene-c2a.md` (scope: the files above) and delete it when done.
Other sessions edit `exterior.rs`, `main.rs` and `perf.rs` too: touch only your lines, never revert others'.
