# static-scene (orchestrator, 2026-11-08)
Scope: static-scene slices (superfargoth/docs/specs/static-scene.md): sf_scene, sf_diag views (S0), vendored
bevy_render/bevy_pbr counter patch; next sf_scene_bevy, bake step `scene.cells`. Not touching other sessions' crates.
Builds: this laptop now has Rust 1.98.1 + VS 2026 Build Tools. The repo's delirium-physics pin (91ee246) no longer
exists upstream; laptop builds patch it to a local clone via a scratchpad wrapper and restore Cargo.lock after.
Status: wave 1 (S0, S1a) compiled on the laptop before the owner stopped laptop builds (2026-11-08). From now on no cargo here: S1b, S2 and later slices are written uncompiled; owner verifies everything on 2026-11-10 (repo/bevyrrowind/docs/test-pass-2026-11-10.md).
