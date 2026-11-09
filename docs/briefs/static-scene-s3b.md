# Brief: static scene from slice S3b on (handoff from the 2026-11-08/09 session)

Written 2026-11-09 by the static-scene coordinator session (it ran S0 to S3a and is closing on context length). Paste
everything below the line into a fresh session. It is a coordinator brief: the new session runs one slice at a time
through agents, reviews each, and merges to `main` only what compiles.

---

You are taking over the **static scene** work in Scrib, a Rust/Bevy 0.20 open-world RPG engine (repo
`github.com/lihogloglo/Scrib`; framework `superfargoth/` = game-agnostic `sf_*` crates, Morrowind pack `bevyrrowind/`
= `mw_*` crates; Bevy crates we patch are vendored in `superfargoth/vendor/`). Goal (owner, 2026-11-08): "a future
proof engine that can work flawlessly and fast", no quick fixes. The design is
`superfargoth/docs/specs/static-scene.md`: static, non-interactive objects stop being one Bevy entity per placement x
LOD x material part and become a retained GPU instance table per space, culled and LOD-picked on the GPU once per view
(main camera, sun cascades, lamp faces), drawn through Bevy's own material pipeline. It is the long-term answer to
hard problem H1 (render-thread bound towns, `bevyrrowind/docs/hard-problems.md`).

## Where it stands (all on `main` unless said)

| Slice | What | State |
|---|---|---|
| S0 | per-view render counters in the perf report (`sf_diag::views`, `sf_diag_bevy` feature `views`) | compiled + tested; owner's perf baseline pending |
| S1a/S1b/S1c | Bevy-free `sf_scene`: table format **v2**, eligibility + promotion, slot allocator, chain layout, 8 material classes, CPU twin of the cull and pixel-error LOD; `scene.cells` bake step (feature `bake`) + Morrowind adapter `mw_bake::scene` | compiled + tested; **not yet called by `bevyrrowind bake`** (one line in `mw_boot_bevy::bake` after `bake_far`); no table baked yet; Vvardenfell measurement test `mw_bake/tests/scene_cells_real_install.rs` needs the install |
| S2 | material classes on today's entity path (`sf_world_bevy::matclass`, `--off matclass`, default on), shared image samplers patch | compiled + tested; visual check **SS-matclass** pending |
| sun shadow early-out | vendored `pbr_functions.wesl` skips the sun cascade fetch where the sun can't light the fragment | shader-validated; visual check **SH-23** pending; no runtime switch |
| S3a | port of upstream **bevyengine/bevy#25978** (`GpuBatchedMesh3d`, persistent GPU instance batches drawn through the mesh-material pipeline) into vendored `bevy_pbr`/`bevy_render` | branch `static-scene/s3a`, merged to `main` by the closing session once its private-target check passes (check `git log origin/main`) |
| S3b | **next**: `sf_scene_bevy` runtime | not started |
| S4-S9 | classes + LOD + motion vectors; shadow views; horizon + Hi-Z occlusion; far field; promotion; retire old path | not started; **S9 waits for the owner's sign-off** |

Everything dated 2026-11-08/09 is **untested in game**: owner test pass on 2026-11-10 from
`bevyrrowind/docs/test-pass-2026-11-10.md` (static-scene rows; handoffs in `bevyrrowind/docs/test-pass-handoffs/`:
`static-scene.md`, `s0-instrument.md`, `s1a-scene-core.md`, `s1b-scene-bake.md`, `s2-material-classes.md`,
`s3a-gpu-batch.md`). Wait for its results before stacking more than S3b-S5 on untested ground.

## Read first (grep, don't read whole files)

1. `CONTRIBUTING.md` (whole); `bevyrrowind/CLAUDE.md` "Parallel sessions", "Hard rules"; `superfargoth/CLAUDE.md`
   "Hard rules", "Commands".
2. `superfargoth/docs/specs/static-scene.md`: all of it (~26 KB; it is the plan), especially §2 "Format v2 (S1c)", §3,
   §4, §6 "Port shape", §7, §8, "Slices", "Risks".
3. `superfargoth/vendor/README.md` "The GPU-batched mesh port" (S3a's API and what is always on).
4. `bevyrrowind/docs/test-pass-handoffs/s3a-gpu-batch.md` (S3a's risks for S3b).
5. `superfargoth/docs/research/spud-code-2026-11-08.md` items 1-3 (the reference design).

## S3b, the next slice (one deliverable + its tests)

`sf_scene_bevy`: the runtime that draws scene-eligible statics of the **main view** from the baked tables, **opaque
class, LOD0 only**, through S3a's `GpuBatchedMesh3d`:
- per space a `StaticScene` (slot allocator from `sf_scene::alloc`, cell -> slot range); stream-in uploads a cell's
  table (`StaticCellTableView`, chain indices remapped with `write_remapped`) within the streaming budget, no entity
  spawns; stream-out releases after frames in flight;
- chain meshes built with `sf_scene::chain_layout` (the same function the bake used) into Bevy's mesh allocator;
  one batch entity per (space, material class) with `GpuBatchedMesh3d { mesh, max_capacity }` + `MeshMaterial3d`
  (never `Mesh3d`); a producer system in `GpuInstanceBatchSystems::Publish` writes the 64-byte `GpuMeshInstance`
  slots (frustum + sub-pixel cull on the CPU twin is acceptable for S3b; the GPU cull pass is S4);
- **behind a switch that is off by default** (planned `--scene`; when on, scene-eligible placements are not spawned
  as entities); the owner A/Bs it on their workstation;
- wire `bake_scene` into `bevyrrowind bake` (`mw_boot_bevy::bake`, after `bake_far`);
- tests: exact ones for upload/remap/slot bookkeeping (Bevy-free where possible), headless sanity that the plugin runs
  without a GPU; a visual-check entry for the owner (Balmora and Seyda Neen, `--scene` on vs off).
S3a's limits to design around: every slot up to `max_capacity` is copied every frame (size capacity to need);
the output buffer is replaced when capacity changes (rebind); no motion vectors yet (`previous_input_index =
u32::MAX`, statics are fine, S4 adds history); one AABB per batch.

## Working rules learned the hard way (owner, 2026-11-08/09)

- **Commit to a branch and push as you go; `main` only gets code that compiles.** Several sessions share the
  checkout `repo/`: never `git add -A` there. Work in your own worktree (`git worktree add -b static-scene/<slice>
  ../worktrees/<slice> origin/main`), or build commits in a temporary index. In shared files (root `Cargo.toml`,
  `Cargo.lock`, `.config/hakari.toml`, CHANGELOG, ROADMAP, DECISIONS, SYSTEMS, `visual-checks.md`, the test-pass
  ledger) commit only your lines; keep both sides on rebase.
- **The laptop** (ThinkPad X13, Ryzen 5 4650U, 15 GB) has Rust 1.98.1 + VS 2026 Build Tools since 2026-11-08 but no git
  identity: pass `GIT_AUTHOR_NAME=lihogloglo GIT_AUTHOR_EMAIL=7213266+lihogloglo@users.noreply.github.com` (and the
  `COMMITTER` pair) as env vars, never change git config. Compiling is allowed there when needed; **launching the
  game is never allowed on the laptop**, and agents never judge visuals anywhere. `cargo` needs
  `export PATH="$HOME/.cargo/bin:$PATH"` in Git Bash; always build through `bash bevyrrowind/tools/cargo_turn.sh ...
  -j 6`. A cold vendored-Bevy build takes ~20-25 min there.
- **Never share a target dir between checkouts that build vendored Bevy**: worktrees and `repo/` write vendored
  `bevy_render`/`bevy_pbr` under the same artifact names, so a build can silently link another checkout's patched
  Bevy (hit twice on 2026-11-09). Give each worktree its own `CARGO_TARGET_DIR`; Bevy-free crates are safe to share.
- Delirium is part of the game, a default feature (owner): never ship commands or checks that rely on
  `--no-default-features`. `delirium-physics` is pinned to `f4ff84d` (0.7.0) since 2026-11-09; the old pin vanished
  upstream. Open owner decision: pin a project-owned fork instead.
- A/B baseline tags (owner OK 2026-11-09): `ab/before-static-scene` = `3e219bf` (`main` just before any static-scene
  code) and `ab/after-s3a` (the S3a merge); build them for the parts without a runtime switch (the S3a port's
  shared-path changes, the sun early-out). The `backup/laptop-2026-11-09*` branches were deleted once `main` had all
  their content. The owner works on the laptop until 2026-11-10, then on the workstation.

## Open findings to schedule (not S3b)

- Bind groups rebuilt every frame per view in `prepare_preprocess_bind_groups` (2.3 ms in an old trace): static-scene
  does not remove it; a cache keyed by buffer ids + upstream PR bevyengine/bevy#26418. Measure with S0's
  `preprocess/bind_groups` first. Without it, the spec's "<= 1 ms for all statics" budget is optimistic.
- `prepare_lights` "recreates point shadow maps only on change" comment is stale (it doesn't churn): comment fix only.
- `static-scene.md` is over the 25 KB budget: move the S1a/S1b history to `specs/archive/`.
- S1c follow-ups: drop `ALPHA_MASKED`/`WIND` from instance flags (pure functions of the chain); define the
  `material_override` target (a per-cell override section, format v3); decode LOD0 only for eligible models.
