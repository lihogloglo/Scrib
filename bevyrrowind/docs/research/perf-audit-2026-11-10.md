# Performance audit (2026-11-10)

Read-only audit from the docs, the `s0_baseline` reports (`benchmarks/runs/s0_baseline/`, main `018c0f6`) and three
code audits (main thread, render thread and views, GPU passes). **No A/B was run** (agents don't launch the game);
the runs that would settle the guesses are in the last section. Every "estimate" and "guess" below is unmeasured.

**Verdict.** The 11 ms median target is met in all eight benchmarks (8.7-10.9 ms). What is left: pacing while
moving, and the fact that main thread, render thread and GPU sit within ~1.5 ms of each other, so no single fix
(the static scene included) reaches the 7 ms stretch alone.

## The machine

- Ryzen 7 7435HS (8 cores / 16 threads, Zen 3+), RTX 4060 Laptop 8 GB, 32 GB RAM (ASUS TUF A15 FA507NVR).
- Panel: **1920x1080 at 144 Hz**. The 1440p benchmarks run in a window larger than the screen. 144 Hz = 6.9 ms,
  the 7 ms stretch.
- GPU in the runs: ~90 W, steady 2580 MHz, a throttle reason flagged in 100 % of nvidia-smi samples
  (`clocks_event_reasons.active`; which reason was not checked. If it is a power cap, the laptop's performance mode
  matters).
- Fixed cost: DLSS Super Resolution to 1440p is 2.2 ms every frame, 20 % of the budget.
- Estimate: at 1440p about 8 ms is realistic and 7 ms very hard; at the panel's 1080p the GPU has room and 7 ms
  depends on the CPU lanes.

## Where we are (High, 1440p + DLSS Quality, ms)

| scenario | frame median | p95 | main busy | render busy | GPU load | bound by |
|---|---:|---:|---:|---:|---:|---|
| balmora (standing) | 9.6 | 11.0 | 6.8 | 7.4 | 95-97 % | GPU |
| balmora_walk | 10.8 | 13.6 | 8.0 | 8.9 | ~80 % | render thread |
| run | 10.2 | 16.1 | 7.6 | 8.8 | 87 % | render thread |
| flight | 10.9 | 14.0 | 7.8 | 8.6 | 85 % | render thread |

A frame costs about the slower CPU lane plus extract (~1.5 ms), or the GPU, whichever is larger.

- **Pacing is the failing gate**: 13-63 stutters per minute against a limit of 1, even standing in Balmora
  (16/min), which streaming cannot explain. Cause not found.
- **Main thread**: only ~2.8 of 8 ms is instrumented. Code shows 10-12 full scans per frame over all 31.8 k mesh
  entities in Bevy's `PostUpdate` visibility chain, none pruned (no `NoCpuCulling`; it made the walk 81 ms):
  `check_point_light_mesh_visibility` scans every caster once per shadowed lamp (`bevy_light` `lib.rs:517,594`),
  `check_visibility` once per camera (`bevy_camera` `visibility/mod.rs:748`), `check_dir_light_mesh_visibility`
  (`bevy_light` `lib.rs:342`), `reset_view_visibility` + `mark_newly_hidden_entities_invisible` (`visibility/mod.rs:733,908`),
  `check_visibility_ranges` (`visibility/range.rs:230`), and two `AssetChanged<MeshMaterial3d>` scans
  (`vendor/bevy_pbr/src/material.rs:743,937`) that run whenever any material changed, which is every frame
  (`mw_assets/src/animation.rs:44-118` steps NIF controllers through `Assets<StandardMaterial>`, ~3.6 writes per
  frame). The ms split is a guess until traced. Frozen characters cost no skinned bounds (ragdolls only) but are
  still visited by `write_bones` (`sf_anim_bevy/src/lib.rs:582`).
- **Render thread**: the standing view has 4 views and still costs 7.4 ms, so most of it is per-item work in the
  main view and 3 cascades (High is 3 cascades to 205 m, refit and redrawn every frame, no caching); the walk's 18
  lamp-face views add ~1.5 ms. One `queue.submit` per frame. `prepare_preprocess_bind_groups` rebuilds 189 bind
  groups per frame (`vendor/bevy_pbr/src/render/gpu_preprocess.rs:2560`). Suspicion, unconfirmed: cached lamp faces
  that lost `RootNonCameraView` (`sf_shadow_cache_bevy/src/render.rs:277`) never get `SkipRootViewRendering`
  (`light.rs:1445` filters on that marker), so they may still pay view uniforms and bind groups.
- **GPU**: forward, MSAA off, full depth prepass, 3 PCSS cascades with the temporal filter, 16-step contact shadows
  for the sun and the 8 budgeted lamps, GTAO medium. The sky env map refreshes every 8th frame but
  `sf_diag::pacing::derive_work` forward-fills it, so **"timed GPU" reads ~0.6 ms too high** (and the per-pass mean
  is over frames where the pass ran). Sun cascades are not timed usefully (`per_view_shadow_pass` 0.02 ms).
  Atmosphere LUTs (0.33 ms) and water FFT + velocity (0.22 ms) run every frame, water even with none in view.

## What the static scene brings (static-scene.md)

- **Main thread**: statics leave the ECS, so the visibility scans shrink to actors and movers. Probably the
  largest share of the untraced 5 ms.
- **Render thread**: S3b covers the main view only; cascades and lamp faces (most shadow items) wait for S5.
- **Pacing**: no entity spawns for statics (today one frame can carry 1,494 entity changes and 1,228 mesh removals).
- **GPU**: nothing until S3a3 (parked) and S4/S6; S3b's GPU numbers will read worse, as the spec says.
- **Risk**: precombine halved mesh entities and the frame did not move; code cannot tell the static share of phase
  items, so "no change" stays a possible S3b result.
- Estimate: CPU lanes 5-6 ms after S5; towns then GPU-bound at ~8.5 ms at 1440p. It buys headroom for mods and
  views and smoother movement, not the 7 ms stretch by itself.

## Improvements that don't depend on it

| # | what | standard technique | lane | effort | image change |
|---|---|---|---|---|---|
| 1 | Cached sun cascades / far cascades at half rate (`industry-audit.md:131`, missing) | CryEngine `r_ShadowsCache`, HDRP cached shadows | render + GPU | L | owner check |
| 2 | Per-lamp caster lists: reuse the lamp cache's static lists instead of a full scan per lamp | per-light caster culling, cached caster lists | main | S-M | no |
| 3 | NIF material animation as shader time / per-instance data, not `Assets<StandardMaterial>` writes | GPU material parameter animation | main + render | M | no |
| 4 | Persistent preprocess bind groups (Bevy #26257) | bind-group caching | render | M | no |
| 5 | Skip marker for detached cached lamp faces (confirm by trace first) | - | render | S | no |
| 6 | Gate water FFT/velocity on water visible (~0.2 ms) | - | GPU | S | no |
| 7 | Stagger env-map stages (removes a 0.7 ms spike every 8th frame) | time-slicing | GPU pacing | S | no |
| 8 | Clamp `Time<Virtual>::max_delta` (~50 ms; today up to 15 fixed ticks after a hitch) | bounded catch-up | pacing | S | no |
| 9 | Fix GPU-timed accounting (sliced passes forward-filled) | - | measurement | S | no |
| 10 | DLSS model preset (none pinned; up to ~1 ms, guess) | - | GPU | S | **yes** |
| 11 | PCSS to PCF, contact shadows 8 steps / sun only (0.3-0.8 ms in town, guess) | - | GPU | S | **yes** |
| 12 | Atmosphere LUT slicing, smaller bloom chain (~0.25 ms) | - | GPU | S | **yes** |

**Low-hanging fruit**: 6, 7, 8, 9 (small, no image change), then 2 and 5 once the trace confirms them; 10 and 11
are A/B switches for the owner to look at, no code needed for 11 (`--off softshadows,contactshadows`).

## MSAA instead of DLSS? (owner question; SPUD does this)

- **It already exists**: `Upscaling::Off` is the MSAA path, the Low preset only (`sf_sky_bevy/src/upscale.rs`,
  sky-lighting.md "Temporal resolve as the base"). The renderer is forward, so MSAA fits.
- **SPUD's 300-400 fps do not come from MSAA.** Their GPU frame is ~1.75 ms: a 2011 forward renderer with no
  volumetric clouds, froxel fog, atmosphere, GTAO or soft shadows (`superfargoth/docs/research/spud-code-2026-11-08.md:20-23`).
  Our CPU lanes (7-9 ms) cap us near 110-140 fps whatever the anti-aliasing.
- **MSAA shades every native pixel; DLSS Quality shades 44 % of them.** Measured 2026-10-12, Balmora 1440p: native
  TAA 11.9 ms, DLSS Quality 8.9. Estimate for 4x MSAA at 1440p: slower than DLSS Q by 1.5-3 ms (main pass x2.25
  pixels plus MSAA cost, minus DLSS's 2.2 ms). At the 1080p panel MSAA is about level with today's 1440p DLSS Q;
  DLSS Q at 1080p would still be cheaper.
- **What MSAA loses today**: GTAO (Bevy's SSAO needs MSAA off, `exterior.rs:795-799`); the temporal shadow filter
  (falls back to Gaussian); dithered LOD and cascade fades show as stipple; cloud, fog and distant-storm noise has
  no final resolve (clouds and froxels keep their own history filters); MSAA does nothing for specular, alpha-test
  or water shimmer.
- **Do we need DLSS without ray tracing?** No. We need *a* temporal resolve as long as the effects are stochastic,
  or the effects rebuilt to be noise-free (SPUD's no-TAA rule), which costs samples. Non-NVIDIA machines already
  get TAA, so that path must hold up anyway. Suggested: keep DLSS as the High default, treat MSAA as a
  "crisp/classic" option, and let the owner A/B it with F4 before deciding anything.

## Runs that would settle the guesses (owner)

1. One trace of `balmora_walk` (`perf suite ... --trace`, perf.md "CPU per system"): the main-thread split, the
   4.4 ms of graph time, item 5.
2. `balmora_walk` with `--point-shadows 0`: the lamp scans (item 2).
3. Standing `balmora` with `--off softshadows,contactshadows`: their GPU share (item 11).
4. One suite at the panel's 1080p with `BEVYRROWIND_WINDOW=borderless`: what this screen actually gets.
5. F4 to the MSAA path in Balmora, day and storm: the look and the ms.

## Measured A/B (2026-11-10 evening, owner-requested runs)

Same profiling `dlss` exe as `s0_baseline`, 2560x1440 + DLSS Quality; summaries and sweep tables in
`benchmarks/runs/audit_2026-11-10/` (full reports in `target/perf/au_*`). Noise is about ±0.4 ms: the walk baseline
read 10.5-11.6 ms across the evening (10.77 in the morning), so only differences above ~0.5 ms count.

**One subsystem off at a time** (`perf sweep ... --var off`; frame median ms, Δ against `none` in the same sweep):

| off | standing Balmora (×2) | Δ | walk (×1) | Δ | walk main / render busy | reading |
|---|---:|---:|---:|---:|---|---|
| none | 9.83 | | 10.53 | | 8.0 / 8.5 | |
| statics (also drops lamps, lights, doors, items) | 7.99 | -1.8 | **7.04** | **-3.5** | 4.3 / 4.5 | the cost; the frame turns GPU-bound (97 %) |
| actors | 9.25 | -0.6 | **8.32** | **-2.2** | 6.3 / 6.6 | second cost, both CPU lanes |
| shadows (sun) | 8.71 | -1.1 | 9.01 | -1.5 | 7.3 / 7.0 | render thread |
| sky (also drops the sun and its shadows) | 8.35 | -1.5 | 8.70 | -1.8 | 7.3 / 6.7 | sky itself ≈ 0.3-0.4 |
| lights | 9.30 | -0.5 | 10.00 | -0.5 | 7.4 / 8.0 | GPU, main opaque -0.5 standing |
| ssao / bloom | 9.33 / 9.42 | -0.5 / -0.4 | | | | at the noise edge |
| terrain | 9.38 | -0.45 | 10.38 | -0.15 | | main opaque -0.6 GPU |
| surface, particles (1 run) | 9.38 / 9.43 | -0.4 | 10.57 / 10.41 | 0 | | noise |
| fog, froxels, farshadows | 9.56-9.61 | -0.25 | 10.82 (fog) | 0 | | noise |
| clouds | 9.73 | -0.1 | 10.63 | 0 | | nothing measurable |
| envmap | 9.87 | 0 | | | | timed GPU -0.9 is the forward-fill artifact |
| walkers, physics | | | 10.58 / 11.04 | 0 | | nothing |

**Lamp shadows** (`BEVYRROWIND_POINT_SHADOWS=0`, ×3): day walk 11.63 → 10.88 (render -0.9, main ≤ 0.3);
**night walk 11.31 → 9.49** (render 9.7 → 7.6, main 8.6 → 7.3; views 21 → 3). The night walk is over the gate
because of them.

**Soft + contact shadows** (`--off softshadows,contactshadows`, ×3): main opaque 2.44 → 2.39 by day, 2.24 → 2.22 at
night. Free; the 0.3-0.8 ms guess (item 11) was wrong. Not checked: that the image changed.

**What this changes above:**

- Eye candy is cheap: clouds, fog, god rays, sky, particles, surface weather, far shadows are each ≤ ~0.4 ms and
  mostly unmeasurable. No inefficient subsystem there. Water has no off-switch (GPU passes ≈ 0.3 ms).
- **Statics are worth ~3 ms on the walk** (minus what the switch also removes: lamps ≈ 0.75, lights ≈ 0.5), on both
  CPU lanes. Removing items pays where precombine's merging did not, so the static scene's target is confirmed;
  with statics off the walk is GPU-bound at 7.0 ms, which is the floor it can reach.
- **Actors are a new lever: ~2.2 ms on the walk** (main -1.7, render -1.9) for 568 characters of which ~19 animate.
  Not in the options table; candidates are merging body-part skinned meshes per actor and culling or freezing
  far actors' extraction (perf research README items 4-6). Needs its own diagnosis.
- **Lamp shadows move up**: 1.8 ms at night, main thread included, so options 2 and 5 both matter.
- Option 11 is dropped. Order by measured ms on the walk: static scene (≤ 3), actors (≤ 2.2), lamp shadows at night
  (≤ 1.8), sun cascades (≤ 1.5, option 1).
