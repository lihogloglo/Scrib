# Brief: static scene B3b-2, chain materials and their bind groups

Coordinator `scene-coord`, 2026-11-11. One deliverable: each chain gets its `StandardMaterial` made exactly the way
the entity path makes it, and the batch key's `bind_group` is that material's real bind group, so batches split
the way the GS port checks them. Previous slices: B3a `b992292`, B3b-1 `1804108`.

## Why the order matters

The port rejects a range whose material is in a different bind group from its batch's
(`superfargoth/vendor/bevy_pbr/src/gpu_instance_batch/mod.rs:808-813`). The batch key is fixed at `plan_in`,
which runs when a cell leaves `LoadPhase::Slabs`. So the material has to exist, and its binding has to be known,
**before** planning. Create the material when the chain product resolves (end of `LoadPhase::Chains`), and make
the Slabs gate also wait for the material's binding.

## Where it stands (`superfargoth/crates/sf_scene_bevy/src/`)

- `chains.rs:17-19`: `ChainAssets::material: Option<Handle<StandardMaterial>>`, always `None`.
- `slabs.rs:72-90`: `extract_chain_slabs` writes `DrawSlabs { vertex_slab, index_slab, bind_group: 0 }` per wanted
  mesh (`TODO(B3b)` at :80).
- `residency.rs`: `chains_step` around :413-488 (Chains → Slabs), the Slabs gate :420-440, and the batch spawn
  `TODO(B3b-2)` at :269 and range material `TODO(B3b-2)` at :296.

## Read (line ranges only)

- `docs/briefs/scene-bevy-b3-b4.md` lines 587-616: step 6 (materials, dedupe per description) and step 8's
  `material` rule.
- `superfargoth/crates/sf_world_bevy/src/stream.rs:1545-1575`: `build_template`'s dedupe closure (copy its shape),
  and `:1225` for the `resource_scope` pattern.
- `sf_world_bevy/src/matclass.rs:142` `fold`; `spawn.rs:443` `ModelMaterials`, `:453` `standard_material`.
- `sf_scene/src/chains.rs:74` `ChainEntry::material_key`.
- `vendor/bevy_pbr/src/gpu_instance_batch/mod.rs`: :50 imports, :168, :806-813 (`RenderMaterialBindings::get` by
  `UntypedAssetId`, compare `.group`).

## Do

1. Make the chain's material from its product's `materials[0]` through `ModelMaterials` (+ `matclass::fold`
   unless matclass is off, matching `build_template`), deduplicated per `ChainEntry::material_key`, when the chain
   product resolves. Store it in `ChainAssets::material`.
2. `extract_chain_slabs` reports each wanted chain material's `MaterialBindingId.group`, and `DrawSlabs::bind_group`
   carries it. The Slabs gate waits until both the meshes and the material binding are known, under the existing
   `SLAB_WAIT_FRAMES` timeout.
3. The batch entity's `MeshMaterial3d` = the material of the chain whose mesh is the batch's `mesh`. A range's
   `material` = `None` when its chain's material is the batch's, `Some(id.untyped())` otherwise (step 8).

Items 1-3 are one deliverable: a material that never reaches the key or the batch is not done. Out of scope:
B4's buffer writes, promotion, counters.

## Tests (exact, headless, `residency_tests.rs`)

1. Two chains with the same material description share one `Handle<StandardMaterial>`; two with different
   descriptions get two.
2. With the bind group injected by hand into `ChainSlabTable` (as `stream_cell_in_with_slabs` already does for
   slabs): two chains in different groups land in different batches; two in the same group with different
   materials share a batch, and the non-batch one's range has `material: Some(..)`.
3. A chain whose binding never arrives: the cell plans after `SLAB_WAIT_FRAMES` without hanging.

## Rules

- **Hand off at ~80k tokens of context**: stop on code that compiles, run what tests you have, and report what's
  left.
- Every cargo command goes through `bash bevyrrowind/tools/cargo_turn.sh <args>` from the repo root. It always exits
  0, so judge by output. Check `tasklist | grep -iE "cargo|rustc"` first. Never `--workspace`, never `cargo fmt`.
  Filter cargo output.
- Other sessions have uncommitted files (menus: `mw_exterior_bevy`, root `Cargo.toml`, workspace-hack). Don't touch
  them. No commit, no game launch.
- Floor: `test -p sf_scene_bevy --all-targets` 13 passed (yours adds 3);
  `clippy -p sf_scene_bevy --all-targets -- -D warnings` clean.

## Report (≤ 40 lines)

Files with line ranges, the tests and counts, what was wrong in this brief, what B4 inherits, and the context you
had used when you stopped.
