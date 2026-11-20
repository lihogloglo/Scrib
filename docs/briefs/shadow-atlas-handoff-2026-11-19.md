# Handoff: Shadow Atlas & Single-Encoder Batched Point Shadows (CW-13)

- **Date:** 2026-11-19
- **Session file:** `.sessions/shadow-atlas.md`
- **Audit & architecture reference:** `docs/reviews/shadow-audit-2026-11-19.md` (especially §6–§7)
- **State at handoff:** **Checkpoint 1 (Slices A0 + A1) complete, benchmarked, and owner-verified (`SH-32`: "seems to be working (fps higher)").** Uncommitted in the working tree (note: other unrelated `sf_lod` / `mw_bake` / `sf_world` files are also dirty from another session — do not revert or mix them). **Slices A2 + A3 (Unified Cached 2D Shadow Atlas) not yet started.**

---

## 1. Owner-Approved Decisions (locked on 2026-11-19)

1. **Staged checkpoint:** Land & verify Slice A0 (mover audit) + Slice A1 (single-encoder batched point/spot shadow pass) before building Slices A2–A3 (2D Shadow Atlas). *(Checkpoint 1 is now done.)*
2. **Atlas VRAM budget (for Slices A2–A3):**
   - **High:** `4096²` Static (64 MiB) + `2048²` Live (16 MiB) in `Depth32Float` = **80 MiB total** (~45–60 simultaneously shadowed lamps across `512² / 256² / 128²` tiles).
   - **Medium:** `2048²` Static + `1024²` Live = 20 MiB.
   - **Ultra:** `8192²` Static + `4096×2048` Live = 288 MiB.
   - Test `Depth16Unorm` (half VRAM: 40 MiB High) as an A/B follow-up once visuals are confirmed.
3. **Spot lights:** Keep `SpotLight` on Bevy's stock directional-array path in Slice A3 (Morrowind content is almost exclusively `PointLight`), while keeping `ShadowAtlasTile` extensible for spot lights later.
4. **No Solari RT crutch:** Solari lacks transparent/skinned parity and requires RT hardware — all fixes must work on raster hardware.

---

## 2. What Was Done (Checkpoint 1: Slices A0 + A1)

### Root Cause Diagnosed
CW-4 (`sf_sky_bevy::lamp_cache`, static depth cache + mover overlay) already fixed `SH-21a` character flickering and dropped total views (`51 → 5`) and stutters (`29.3 → 12.8/min`), with depth copies costing only `0.042 ms`, but regressed render busy (`+0.94 ms`, `pt_night`). Code audit of `superfargoth/vendor/bevy_pbr/src/render/gpu_preprocess.rs:856-884` found the exact culprit: Bevy's `Core3dShadows` schedule runs 5 systems per drawn face (`allocate_uniforms`, `unpack_bins`, `early_gpu_preprocess`, `early_prepass_build_indirect_parameters`, `shared_shadow_pass`), opening **5 separate `wgpu::CommandEncoder`s per drawn face** (up to 45 encoders/frame when CW-4 drew 9 mover faces).

### Slice A0 — Mover Audit
Added a gated once-per-second `log::debug!` mover breakdown in `superfargoth/crates/sf_sky_bevy/src/lamp_cache.rs:769-814` and ran `balmora_walk_night` (`RUST_LOG=info,sf_sky_bevy::lamp_cache=debug`).

- **~65–80% (`movable=39..193`)**: Modular NPC body-part/clothing/armor meshes (~18–25 `Mesh3d` entities per `Character` with inherited `LampShadowMobility::Movable` × 4–8 NPCs near 6–8 lamps), counted as `(lamp, mesh)` pairs.
- **~15–25% (`rigid=13..57`)**: Newly streamed-in or LOD-switched non-`NoCpuCulling` props during the walk (`rigid=0` while standing still at start/end), intentionally drawn as movers during `settle_time_s` (`1 s`) before settling (covered by unit test `lamp_cache_tests::a_spawned_static_recaptures_its_face`).
- **~10–15% (`unbaked=10..39`)**: Newly settled statics waiting out `recapture_interval_s` (`2 s`), drawn as movers so shadows never pop out.
- **`animated=0`**: Zero animated-static movers.
- **Conclusion:** No mover-classification bug.

### Slice A1 — Single-Encoder Batched Point/Spot Shadow Pass (`CW-13a`)
Replaced per-face `RootNonCameraView(Core3dShadows)` scheduling with a single-encoder batched pass gated behind opt-in live tunable `shadow.lamp.batched_pass` (default `false`).

#### Files Owned & Modified by This Session
| File | What changed |
|---|---|
| `superfargoth/vendor/bevy_pbr/src/render/gpu_preprocess.rs` | Factored `pub(crate)` helpers `record_allocate_uniforms_for_views`, `record_unpack_bins_for_views`, `record_early_gpu_preprocess_for_views` taking `&[Entity]` and `&mut RenderContext` (`run_build_indirect_parameters` was already `pub(crate)`). Stock `allocate_uniforms`, `unpack_bins`, `early_gpu_preprocess` delegate to them unchanged. |
| `superfargoth/vendor/bevy_pbr/src/render/light.rs` | Added `PointShadowBatchedPass(pub bool)` (`ExtractResource`, default `false`) and `BatchedShadowView` marker. Updated `skip_unchanged_shadow_views` to query `Or<(With<RootNonCameraView>, With<BatchedShadowView>)>` and live-swap `RootNonCameraView(Core3dShadows)` ↔ `BatchedShadowView` on all point/spot shadow views (stock and cached). Added `BatchedPointShadowParams` and `record_batched_point_shadows` recording GPU preprocess across all active `BatchedShadowView`s, then per-view `run_build_indirect_parameters` + `view_shadow_pass::<false>` inside **one single `RenderContext` command encoder** (note: indirect buffers are global per phase, so `run_build_indirect_parameters` must stay immediately before each view's `view_shadow_pass::<false>` inside the same encoder). Opens zero encoders when no batched view is active. |
| `superfargoth/vendor/bevy_pbr/src/lib.rs` | Registered `init_resource::<PointShadowBatchedPass>()`, `ExtractResourcePlugin::<PointShadowBatchedPass>::default()`, and `record_batched_point_shadows.in_set(RenderGraphSystems::Render).after(record_cached_point_shadow_copies).before(camera_driver)` (preserving the exact `record_cached_point_shadow_copies` chain asserted by `lamp_cache_render_tests`). |
| `superfargoth/vendor/README.md` | Documented `"own, CW-13a: batched point shadow pass"` row. |
| `superfargoth/crates/sf_sky_bevy/src/shadow_budget.rs` | Declared tunable `BATCHED_PASS` (`shadow.lamp.batched_pass`, default `false`) and added it to `TUNABLES`. |
| `superfargoth/crates/sf_sky_bevy/src/quality.rs` | Bound `BATCHED_PASS` live to `bevy_pbr::PointShadowBatchedPass`. |
| `superfargoth/crates/sf_sky_bevy/src/lib.rs` | Re-exported `BATCHED_PASS`. |
| `superfargoth/crates/sf_sky_bevy/src/lamp_cache.rs` | Added Slice A0 gated once-per-second `log::debug!` mover breakdown (`last_mover_log_s` on `LampCacheState`). |
| `superfargoth/crates/sf_sky_bevy/src/lamp_cache_render_tests.rs` | Added `batched_point_shadows_run_after_copies_and_before_camera_driver` and `batched_pass_tunable_binds_live`. |
| `bevyrrowind/crates/mw_environment_bevy/src/shadow_tunables.rs` | Registered env alias `BEVYRROWIND_LAMP_BATCHED_PASS` for `shadow.lamp.batched_pass`. |
| `docs/reviews/shadow-audit-2026-11-19.md` | Appended §6 (upstream Bevy GitHub API check) & §7 (Unified Cached 2D Shadow Atlas deep dive). |
| `docs/ledger/shadows.md`, `docs/ledger/perf.md` | Logged Slice A0 findings & Slice A1 `shadow_a1_batched` interleaved A/B results. |
| `bevyrrowind/docs/visual-checks/shadows.md`, `bevyrrowind/docs/visual-checks.md` | Added `SH-32` and recorded owner visual confirmation. |
| `bevyrrowind/benchmarks/runs/shadow_a1_batched/` | Saved 3-arm interleaved A/B reports & `summary.md`. |

### Verification Results (Checkpoint 1)
- **Unit tests:** `cargo_turn.sh test -p sf_sky_bevy --lib lamp_cache` → **19/19 passed**; `mw_environment_bevy` → **11/11 passed** *(note: 2 pre-existing unrelated failures in `sf_sky_bevy::sun_gate_tests` from an earlier `shadow.sun.gate` default flip on 2026-11-17)*.
- **Clippy:** `cargo_turn.sh clippy -p sf_sky_bevy -p mw_environment_bevy --all-targets -- -D warnings` → **0 warnings**.
- **Tunables census:** `node tools/tunables-census.mjs --check` → **0 new violations** in touched crates.
- **Interleaved A/B (`bevyrrowind/benchmarks/runs/shadow_a1_batched/summary.md`):** 3-arm Latin-square (`base`, `cache`, `batched`), 3 kept repeats/arm (18/18 quiet & focus-clean), `2560×1440` High + DLSS Quality, `RTX 4060 Laptop`, exe `target/profiling/bench-shadow-a1.exe`:
  - **`balmora_walk_night`:**
    - Median: `10.53 ms` (`base`) / `11.21 ms` (`cache`, `+0.67`) / **`9.83 ms` (`batched`, `−0.70 ms` vs base, `−1.38 ms` vs cache)**
    - p95 / p99: `13.63 / 15.35` (`base`) → `14.64 / 16.68` (`cache`) → **`11.64 / 13.47 ms` (`batched`, 1% low 65 → 74 fps)**
    - Thread Δ vs `base`: main `−0.81`, render `−0.80` (`−1.16` vs `cache`), graph `−0.52` (`−1.04` vs `cache`), GPU `−0.20` (`−0.67` vs `cache`)
    - `encode//sum`: `4.15` / `5.04` / **`3.18 ms`**; `encode/submit`: `2.32` / `2.64` / **`2.03 ms`**
    - `encode/queue/command_buffers`: `52.6` (max 258) / `70.6` (max 149) / **`39.0` (max 43)**
    - `encode/pass:shadow_point_light/count`: `2.93` (max 44) / `6.42` (max 22) / **`0.57` (max 1)**
    - `batched_point_shadows`: **`0.189 ms` GPU / `0.127 ms` CPU**
  - **`census`:**
    - Median: `6.98 ms` (`base`) / `6.85 ms` (`cache`) / **`6.61 ms` (`batched`, `−0.37 ms`, formal `faster` verdict vs `base`)**
    - p95 / worst: `9.98 / 57.6` → `8.97 / 24.9` → **`8.38 / 17.1 ms`**
    - Render busy `−0.52`, graph `−0.35`; `encode//sum` `2.69 → 2.16 ms`; command buffers `43.2 → 30.0 (max 31)`.
- **Visual check (`SH-32`):** Owner tested Balmora 22:00 with `shadow.lamp.cache.enabled=on` + `shadow.lamp.batched_pass=on` and reported: *"it seems to be working (fps higher)."*

---

## 3. Review Checklist for the Next Agent

Before building on top of Checkpoint 1 (or flipping defaults), please audit these items:

1. **Check `record_batched_point_shadows` edge cases (`superfargoth/vendor/bevy_pbr/src/render/light.rs:4115-4220`):**
   - Verify behavior when `shadow.lamp.batched_pass=on` with `shadow.lamp.cache.enabled=off` (stock point/spot lamps without CW-4 cache). It was designed to work for both stock and cached lamps — confirm with a quick run or test!
   - Verify `record_cached_point_shadow_copies` vs `record_batched_point_shadows` ordering: currently copies run in their own 1 encoder (`0.033 ms`) right before `record_batched_point_shadows`. In Slice A3, copies become in-pass depth-blits inside the Live Atlas `RenderPass`.
   - Verify `ApplyDeferred` ordering between `skip_unchanged_shadow_views` (which inserts/removes `BatchedShadowView` / `RootNonCameraView`) and `prepare_view_uniforms` / `prepare_preprocess_bind_groups` / `record_batched_point_shadows` (already separated by `PrepareResources` → `Render`, double-check no 1-frame lag on live toggle).
   - Add a headless unit test exercising `skip_unchanged_shadow_views` (or a small helper extracted from it) verifying `RootNonCameraView` ↔ `BatchedShadowView` toggling.
2. **Deciding defaults for CW-4 + A1 (optional pre-A2 step):**
   - `batched` cleared the range on `census` (`faster`) and beat `base` by `−0.70 ms` (and `cache` by `−1.38 ms`) on `balmora_walk_night` at `n=3` (0.03 ms shy of `base`'s 0.73 ms range on `balmora_walk_night`).
   - If you want to ship CW-4 + A1 **on by default** before finishing A2–A3: run a quick 6-repeat A/B (`--repeat 6`, `base` vs `batched` + optionally `base_batched` without `cache`) on `balmora_walk_night`, `balmora_walk` (day), and `census`, and check `SH-32` on Census + doors/pickups, then flip `shadow.lamp.cache.enabled` and `shadow.lamp.batched_pass` defaults to `true` (and confirm with owner).
3. **Catalog / tunables check:** Verify whether `shadow.lamp.batched_pass` needs to be added to `mw_devtools_bevy::perf::tunables::CATALOG` so `bevyrrowind tunables --md` lists it without warning.

---

## 4. What Is Left To Do (Slices A2 + A3 — Unified Cached 2D Shadow Atlas)

Now that the 5-encoders-per-face CPU cliff is gone, the remaining open problem is **`SH-27` / `SH-31` ("8-lamp cap, shadows only appear when close indoors and on Balmora streets")**. Everything below follows `docs/reviews/shadow-audit-2026-11-19.md` §7 and the owner-approved plan:

### Slice A2 — Bevy-Free 2D Shadow Atlas Allocator & Policy (`sf_sky::shadow_atlas`)
Keep `sf_sky` strictly Bevy-free (`CONTRIBUTING.md` compile-time rules).

- **New files:** `superfargoth/crates/sf_sky/src/shadow_atlas/{mod.rs,allocator.rs,policy.rs,tests.rs}`
- **`BuddyAtlas` (`allocator.rs`):**
  - 2D quadtree / buddy allocator over power-of-two atlas (`2048`, `4096`, `8192`), `min_tile` (`64` or `128`), `max_tile` (`512` or `1024`).
  - Returns `AtlasRect { x: u16, y: u16, size: u16 }`; O(log N) alloc + automatic sibling coalescing on `free(rect)`; zero external fragmentation.
  - Configurable spare reservation for handover crossfades (`handover_spares`, e.g. 2 lamps).
- **Tier & Pool Policy (`policy.rs`):**
  - Resolution tiers (`512² / 256² / 128²`) from distance/importance with **1.5× hysteresis** + `resize_cooldown_s = 2.0` (prevents resolution-resize recapture storms while walking).
  - **Empty-face omission:** faces with `statics_mask == 0` and no movers allocate zero tiles (`AtlasTileRef::Empty`), saving ~25–35% of atlas space on wall sconces/corners.
  - **Separate Live Atlas `BuddyAtlas`** (`2048²` on High) allocating temporary tiles only for active mover faces and returning them immediately when movers leave.
  - Optional `sqrt(luminance)` weighting in importance ranking so bright braziers outrank dim candles.
- **Tunables (`sf_tunables::def!`):**
  - `shadow.lamp.atlas.enabled` (`bool`, default `false`, env `BEVYRROWIND_LAMP_ATLAS`)
  - `shadow.lamp.atlas.static_size_px` (`u32`, default `4096`)
  - `shadow.lamp.atlas.live_size_px` (`u32`, default `2048`)
  - `shadow.lamp.atlas.guard_texels` (`f32`, default `2.0`)
  - `shadow.lamp.atlas.tier_512_m` (`f32`, default `10.0`), `shadow.lamp.atlas.tier_256_m` (`f32`, default `24.0`)
  - `shadow.lamp.atlas.resize_cooldown_s` (`f32`, default `2.0`)
  - `shadow.lamp.contact_max` (`usize`, default `8`) — **critical:** keep screen-space lamp contact shadows capped to the top 4–8 hero lamps so raising shadowed lamps from 8 → 48+ does NOT multiply contact raymarch GPU cost!
- **Tests (`tests.rs`):** buddy alloc/split/coalesce, zero fragmentation under churn, handover spare reservation, tier hysteresis/cooldown, empty-face omission, luminance ranking.

### Slice A3 — Vendored 2D Shadow Atlas Pass & Shader (`CW-13b`)
- **New files in `superfargoth/vendor/bevy_pbr/src/render/`:**
  - `point_shadow_atlas.rs`:
    - `PointShadowAtlasGpu`: `static_texture` (`Depth32Float`, `static_size²`), `live_texture` (`Depth32Float`, `live_size²`), `tiles_buffer: StorageBuffer<Vec<GpuShadowAtlasTile>>` (16 B/face: `uv_offset: Vec2`, `uv_scale: f32`, `flags_and_size: u32` packing `{atlas: 0=empty/1=static/2=live, inv_guard_tan_q16, tile_px}`).
    - `record_point_shadow_atlas` (inside the Slice A1 single encoder!):
      1. Batched GPU preprocess over all active capture + mover jobs.
      2. **ONE `RenderPass` on `static_texture` (`LoadOp::Load`):** per capture tile (0–4/frame), `set_viewport` + `set_scissor_rect` → clear quad (`DepthCompare::Always`, writes `0.0` reverse-Z far) → `shadow_phase.render(...)`.
      3. **ONE `RenderPass` on `live_texture` (`LoadOp::Load`):** per active mover tile, `set_viewport` + `set_scissor_rect` → depth-blit quad (`static_rect → live_rect`, `DepthCompare::Always`) → `shadow_phase.render(...)` (`DepthCompare::GreaterEqual`). All active mover faces share **1 `RenderPass`** and **1 `wgpu` bind-group slab walk** (maximizing `render.wgpu.bind_group_walk_once`).
    - Guard-band perspective projection per face: `tan_half = tile_px / (tile_px - 2.0 * guard_texels)` so 2×2 hardware PCF taps never bleed across tile boundaries.
  - `point_shadow_atlas_blit.wesl`: 3-vertex fullscreen triangle shader for sub-rect depth clear and `Static → Live` depth blit.
- **Shader & binding edits in `superfargoth/vendor/bevy_pbr/src/render/`:**
  - `mesh_view_bindings.rs` & `mesh_view_bindings.wesl`: add `MeshPipelineKey::POINT_SHADOW_ATLAS` / `MeshPipelineViewLayoutKey::POINT_SHADOW_ATLAS` (opt-in); bind `@group(0)` bindings `54..=56` (`point_shadow_atlas_static: texture_depth_2d`, `point_shadow_atlas_live: texture_depth_2d`, `point_shadow_atlas_tiles: array<types::ShadowAtlasTile>`); reuse existing comparison sampler `@binding(3)`.
  - `shadows.wesl` & `mesh_view_types.wesl`: under `@if(POINT_SHADOW_ATLAS)`, `fetch_point_shadow` selects major axis → `face_index`, indexes `point_shadow_atlas_tiles[light_id * 6u + face_index]`, early-outs `1.0` when `atlas == 0u`, rescales `normal_bias` by `tile_px` (prevents acne on `128²` far tiles while keeping `512²` near tiles sharp), applies `inv_guard_tan`, clamps UV to `[uv_min + half_texel, uv_max - half_texel]`, and samples static or live atlas with `textureSampleCompareLevel`.
  - Remove the 42-cube hard cap when atlas is enabled (`GpuClusteredLight` already sorts shadowed point lights first `0..point_shadow_count`).
- **Host wiring (`superfargoth/crates/sf_sky_bevy/src/shadow_atlas.rs` + `shadow_budget.rs`):**
  - Integrate `sf_sky::shadow_atlas` with `lamp_cache` / `PointShadowPlugin` behind `shadow.lamp.atlas.enabled`.
  - Cap `PointLight::contact_shadows_enabled` to `shadow.lamp.contact_max`.
- **Verification for Slice A3:**
  - Headless WESL + Naga shader compile test with `POINT_SHADOW_ATLAS` both on and off.
  - Unit tests for tile assignment, guard-band math, normal-bias scaling, live toggle on/off.
  - `node tools/tunables-census.mjs --check`, clippy `-D warnings`, `superfargoth/vendor/README.md` update.
  - Interleaved A/B (`base` 8-lamp cube vs `atlas8` 8-lamp parity vs `atlas48` 48-lamp atlas) on `balmora_walk_night`, `balmora_walk`, and `census`, logged to `docs/ledger/{shadows,perf}.md`.
  - Owner visual check (`SH-33`) in Census Office / crowded tavern (20+ lamps: shadows across whole room immediately, no close fade-in, no tile-seam bleeding, no `128²` acne) and Balmora 22:00.

### Slice A4 — Optional Follow-Ups (After A3 Sign-Off)
1. **`Depth16Unorm` A/B** for the Static/Live atlases (halves VRAM: 80 → 40 MiB High; check for shadow acne on large rooms).
2. **Idle / A3 NPC mover-redraw skip:** `balmora_walk_night` showed `skin/blocks_skipped = 146` vs `skin/blocks_changed = 15` (~90% of characters didn't change pose on a given frame). Skipping mover face redraws when every mover on that face had neither transform nor skin-block change since last draw could cut mover face draws further when NPCs stand idle near lamps.
3. **`SpotLight` atlas tiles** using the same `ShadowAtlasTile` buffer if Morrowind mods / future scenes add shadowed spotlights.

---

## 5. Practical Notes / Pitfalls for the Next Agent

- **Windows shell:** Bare `bash` is not on PowerShell's PATH; run cargo commands via `& "C:\Program Files\Git\bin\bash.exe" bevyrrowind/tools/cargo_turn.sh ...`.
- **Never `--all-features`:** Keep Bevy features unified (`--profile profiling -p bevyrrowind --features dlss` for benches). Private bench copy `target/profiling/bench-shadow-a1.exe` already exists alongside `target/profiling/dlss/`.
- **Preserve test contract:** `sf_sky_bevy/src/lamp_cache_render_tests.rs:169-180` asserts the exact squashed string `record_cached_point_shadow_copies.in_set(bevy_render::renderer::RenderGraphSystems::Render).before(bevy_core_pipeline::schedule::camera_driver)` in `vendor/bevy_pbr/src/lib.rs`.
- **Unrelated dirty files:** `git status` has ~30 modified files in `sf_lod*`, `mw_bake`, `sf_world*`, `mw_exterior_bevy`, `mw_world` from a separate session. Keep your scope strictly to the shadow/sky/pbr files listed above.
