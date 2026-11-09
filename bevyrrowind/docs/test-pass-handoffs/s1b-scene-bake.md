# s1b-scene-bake: written without compiling (mw_bake part), 2026-11-08

Scope: static-scene slice S1b. `superfargoth/crates/sf_scene` (feature `bake`, `scene.cells`, `chain.rs`),
`bevyrrowind/crates/mw_bake` (`scene.rs` adapter), docs. Nothing committed. Builds stopped on the laptop by owner
decision partway: **`sf_scene` WAS compiled and tested** (41 tests pass with `--features bake`, 35 without; clippy
pedantic `--all-targets --no-deps -D warnings` clean both ways). **`mw_bake` was NOT compiled** (build killed).

## Files
- sf_scene: `Cargo.toml` (feature `bake` = optional sf_bake, sf_model (no default features), sf_terrain + `bake`,
  sf_world + `bake`; dev-dep sf_vfs), `src/lib.rs` (mods + re-exports), new `src/chain.rs`, new `src/bake.rs` +
  `src/bake/tests.rs`, `src/table.rs` (duplicate placements allowed: non-decreasing order, `find` = first instance,
  new `find_all`), `src/table/tests.rs` (+1 test, one assertion adjusted), `src/eligibility.rs` (`classify` takes
  `?Sized`).
- mw_bake: `Cargo.toml` (+ `sf_scene = { workspace = true, features = ["bake"] }`; dev-dep `sf_coords`), `src/lib.rs`
  (`pub mod scene` + doc line), `src/far.rs` (`script_tokens` now `pub(crate)`), new `src/scene.rs` +
  `src/scene/tests.rs` (4 tests), new `tests/scene_cells_real_install.rs` (ignored measurement).
- Docs: static-scene.md (§2 "Settled in S1b", S1b row), bake-adapter.md ("Scene cells"), CHANGELOG line.

## Owner commands (2026-11-10, repo root)
```
bash bevyrrowind/tools/cargo_turn.sh hakari generate && bash bevyrrowind/tools/cargo_turn.sh hakari manage-deps
bash bevyrrowind/tools/cargo_turn.sh test -p sf_scene
bash bevyrrowind/tools/cargo_turn.sh test -p sf_scene --features bake
bash bevyrrowind/tools/cargo_turn.sh clippy -p sf_scene --all-targets -- -D warnings
bash bevyrrowind/tools/cargo_turn.sh clippy -p sf_scene --features bake --all-targets -- -D warnings
bash bevyrrowind/tools/cargo_turn.sh test -p mw_bake --lib scene
bash bevyrrowind/tools/cargo_turn.sh clippy -p mw_bake --all-targets -- -D warnings
bash bevyrrowind/tools/cargo_turn.sh test --release -p mw_bake --test scene_cells_real_install -- --ignored --nocapture
```
Hakari: new workspace-internal edges only (sf_scene -> sf_bake/sf_model/sf_terrain/sf_world under `bake`;
mw_bake -> sf_scene; mw_bake dev -> sf_coords). No new external crates. sf_scene stays in final-excludes.
Measurement prints: load / models+exterior / scene.cells times; cells (empty, without land), instances, placements in
the scene, distinct chains, chain-id collisions, MiB of tables, instances per material class, step stats. It uses the
default bake cache (or `SF_SCENE_CACHE`), `MORROWIND_DIR` or the detected install.
Not done: `mw_boot_bevy::bake` does not call `mw_bake::scene::bake_scene(&mut baker, &db, &[world::exterior()])` yet
(one line after `bake_far`; other crate, left to the owner/orchestrator).

## Unsure (mw_bake, uncompiled; checked by reading source)
- `use super::*` in `scene/tests.rs` reaching `scene.rs`'s private `use` items (`extras`, tes3 `Activator`/`Script`).
- Closure param inference in `mixed_opaque_and_blended_model_splits` (`sub`, `cooked` closures).
- `Arc::new(bake).register(baker)` on a temporary (`register(self: &Arc<Self>)`).
- `baker.manifest().products.keys()` temporaries in one statement (`land_tiles`).
- Clippy pedantic on new code: `cast_possible_truncation` is not expected (no casts); `similar_names` maybe in tests.
- Test 2 expects the `named_rock` token from `"Named_Rock"->Disable` via `far::script_tokens` (quoted string, lowercased).

## Design notes / spec findings
- Scene placement id = index in the world-index cell (the 128-bit sf_world id cannot be a collision-free u32).
- Chain id = xor of the mesh key's halves: "global per space" without a directory step, but collisions are possible
  worldspace-wide (in-cell collision fails the cell); the runtime must check the mesh key per id. The measurement counts it.
- Land coverage = level-0 `terrain.tile` products in the session manifest; fill terrain inside covered tiles counts as
  land (it is drawn); cells over no tile get -inf. Block minima minus `MAX_QUANT_STEP` (conservative).
- Models with controllers come back `Cooked::Skipped`, so the reason is NotCooked, never Animated, for Morrowind.
- Only `sf_world` `PlacementClass::Static` placements have their `mesh.cook` read.
