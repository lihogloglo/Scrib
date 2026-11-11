# Brief B5c: the static scene turns itself off on GPUs without GPU culling

Coordinator: scene-coord, 2026-11-11. Spec: `superfargoth/docs/specs/scene-runtime-plan.md` §7's last paragraphs
(`:693-704`, read only those). One deliverable: **on an adapter where `GpuPreprocessingSupport::is_culling_supported()`
is false, the scene behaves exactly as `--off scene` (statics spawn as entities) and warns once at startup**, plus its
tests. Headless only; never launch the game; don't commit.

## Why

In a view without indirect drawing, both batching callers `warn_once` and skip instance batches, so scene statics go
**missing** (plan §7; `vendor/bevy_render/src/batching/gpu_preprocessing.rs`). The support answer lives in the render
world: `GpuPreprocessingSupport::is_culling_supported` (`gpu_preprocessing.rs:151-154`).

## Design (coordinator, don't revisit)

- Main-world `#[derive(Resource)] enum SceneSupport { Unknown, Supported, Unsupported }` in `sf_scene_bevy`.
  Inserted as `Unknown` in `ScenePlugin::finish` **only when there is a render app** (`lib.rs`, the `finish` body);
  with no render app there is no resource and everything behaves as today (all existing headless tests rely on this).
- Render world, `ExtractSchedule`: `extract_scene_support(mut main: ResMut<MainWorld>, support:
  Option<Res<GpuPreprocessingSupport>>)`, same shape as `extract_chain_slabs` (`slabs.rs:107-112`): while the main
  resource is `Unknown` and `support` exists, set `Supported` / `Unsupported`; on `Unsupported` and `SceneConfig::enabled`,
  one `log::warn!` ("GPU culling unsupported on this adapter: the static scene is off, statics draw as entities").
- The gate, in `SceneHook` (`residency.rs:741-` `impl CellScene for SceneHook`): `Unsupported` behaves as
  `!config.admits(..)` does today in **`acquire`** (returns `None`, `:742-745`), **`ready`** (returns `true`, `:754-757`)
  and **`covered`** (`:769-`, returns `None`); `Unknown` makes `ready` return `false` (the cell's fetch waits; the
  first extract answers within a frame, long before any table loads) and lets `acquire` proceed. Put the three-way
  check in one helper, e.g. `fn scene_on(world, worldspace) -> Option<bool>` (None = wait), so the three methods read alike.

## Tests (`residency_tests.rs`; fixture `fixture` ~`:245`, `stream_cell_in` ~`:735`; find how a covered-cell test
asserts that placements spawn as entities, e.g. grep `covered` in the file)

1. Rename/replace `promotion_is_safe_without_the_shader_fix` if it exists (grep; plan §7 says it must not assert a
   doubling): **`an_adapter_without_gpu_culling_turns_the_scene_off`**: insert `SceneSupport::Unsupported`, stream a
   cell in: `acquire` gives nothing, no batch is created, `covered` is `None` (so streaming spawns entities). Comment
   citing plan §7 and `is_culling_supported` as the gate read.
2. `SceneSupport::Unknown`: `ready` is false for an admitted worldspace; switching it to `Supported` lets the cell
   commit as usual.
3. No resource: unchanged (the existing suite covers it; just make sure it stays green).
The extract system itself needs a render app; don't fake one. Its logic can be a pure fn
`fn resolve(culling: bool) -> SceneSupport` if you want a unit test.

## Verify (`bash bevyrrowind/tools/cargo_turn.sh <args>`, real exit codes; output filtered)

- `test -p sf_scene_bevy` (28 before), `clippy -p sf_scene_bevy --all-targets --no-deps -- -D warnings`.
- Docs: plan §7 one "as built (B5c)" line; `static-scene.md` hardware-floor sentence if it says "B5 will" (grep
  `PreprocessingOnly` in `superfargoth/docs/specs/static-scene.md`).

## Hand-off

Stop by ~80k tokens. Report files, functions, test names and counts, anything that differed. Register in
`.sessions/scene-b5c.md`; delete it when done. Touch only `sf_scene_bevy` and the two spec lines.
