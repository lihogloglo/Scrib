# s1a-scene-core (2026-11-08): handoff, written without compiling

**Update 2026-11-08 (orchestrator): compiled and tested on the laptop** (Rust 1.98.1 + VS 2026 Build Tools installed); see the static-scene.md Slices row. Fixes applied after compiling are noted there.

Scope: static-scene slice S1a, the Bevy-free `sf_scene` crate (table format, eligibility/promotion, slot allocator,
CPU cull/LOD twin) and its tests. This PC has no Rust toolchain: nothing was built, tested or linted. Nothing committed.
Never launches the game.

## Files
- New: `superfargoth/crates/sf_scene/` (`Cargo.toml`; `src/lib.rs`, `table.rs` + `table/tests.rs`, `eligibility.rs`,
  `alloc.rs`, `cull.rs` + `cull/tests.rs`; 28 tests). Deps: bitflags, glam, sf_coords, thiserror (all existing).
- Changed: root `Cargo.toml` (one `sf_scene` workspace-dep line), `Cargo.lock` (hand-added `sf_scene` package entry),
  `.config/hakari.toml` (`sf_scene` in `final-excludes.workspace-members`, by hand).
- Docs: static-scene.md "Slices" (S1 split into S1a / S1b), `docs/SYSTEMS.md` far-field row, CHANGELOG line.
- Not done: `superfargoth/crates/workspace-hack` regeneration (expected no diff: sf_scene is final-excluded and its
  deps are already in the hack with superset features). Owner runs hakari below.

## Verify (main workstation, from the repo root)
```
bash bevyrrowind/tools/cargo_turn.sh hakari generate && bash bevyrrowind/tools/cargo_turn.sh hakari manage-deps
bash bevyrrowind/tools/cargo_turn.sh hakari generate --diff && bash bevyrrowind/tools/cargo_turn.sh hakari manage-deps --dry-run
bash bevyrrowind/tools/cargo_turn.sh test -p sf_scene
bash bevyrrowind/tools/cargo_turn.sh clippy -p sf_scene --all-targets -- -D warnings
```
If Cargo.lock's hand entry is off, a plain build rewrites it (don't use `--locked` the first time).

## Unsure (likeliest snags)
- Precise capturing `impl ExactSizeIterator<..> + use<'a>` in `StaticCellTableView` (table.rs): needs Rust ≥ 1.82
  (toolchain is 1.98.1, fine); if the borrow checker still complains, collect into a Vec.
- glam 0.33: `Vec3::select` + `cmpge`, `Vec2::min_element`, `Vec3::abs_diff_eq` (inherent), `Vec4Swizzles::xyz`,
  `Affine3A { matrix3, translation }` field init.
- `pick_level([0.0], ..)` style calls: element type inferred from `impl IntoIterator<Item = f32>`.
- Clippy pedantic on 1.98: `similar_names`, `many_single_char_names`, `float_cmp` in tests; casts carry local allows.
- Float-exact asserts: `round_trip_is_exact` compares bytes; `affine_rows_round_trip` compares a glam Affine3A
  rebuilt from rows (pure copies, should be exact).
