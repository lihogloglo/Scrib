# Frame waste audit, cost-first (2026-11-16)

Read-only audit: no code change, no build, no bench run. Question asked of every per-frame item: **does the player
see this work in this situation?** Known items are not re-reported (lamp shadows by day, SH-21a caster caching,
fewer faces per lamp, the perf-verdict audit §3, hardcoded tunables); §4 only re-sizes the lamp one.

**Sources (all numbers are from these files, mean per frame unless stated):**
- W = `bevyrrowind/benchmarks/runs/f_on/balmora_walk@2560x1440-2.md` (day walk, High, DLSS Q, clean focus)
- S = `bevyrrowind/benchmarks/runs/f_on/balmora@2560x1440-2.md` (standing, same exe)
- N = `bevyrrowind/benchmarks/runs/lampsel_new/balmora_walk_night@2560x1440-2.md` (night walk; **older exe, before
  the F3 cascade hold and before the focus guard**, so use its counts, not its ms)
- C = `bevyrrowind/target/perf/w4btput_warm3/census@2560x1440-1/census_office.md` (interior; **TAA at native
  1440p, no DLSS**: it is a warm-up run, the only census report on disk; no clean DLSS census run exists)

Bound threads: W main 9.41 / render busy 9.75 / GPU timed 7.31 ms. S main 7.50 / render busy 7.95 / GPU 8.97 ms.
C main 5.80 / render busy 6.78 / main opaque alone 7.54 ms GPU (native). Target 8.3 ms.

**Limits of this audit.** Confidence labels are honest: "measured" is a counter in a report; "estimated" is my
reading of which share of a counter a gate would remove; "code read" means I read the lines cited; "grep only" means
I found no gate by grep and did not read the whole file. Industry sources are cited from memory or from our
`docs/wiki/industry/` pages and were not re-fetched in this session. I stopped at the brief's ~100k-token limit:
§5 lists what is left.

## 1. Per-frame inventory

Cost columns: W / S / N / C in ms (GPU ms for GPU rows, thread ms otherwise); "-" = not present, "?" = no counter.

### GPU passes

| item | W | S | N | C | player sees it? | gate today | verdict |
|---|---:|---:|---:|---:|---|---|---|
| DLSS super resolution | 2.04 | 2.04 | 2.08 | - (TAA 0.52) | yes | none needed | not waste |
| main opaque | 1.60 | 2.91 | 1.33 | 7.54 native | yes; includes clustered lamp shading by day (top 7) | frustum only; no occlusion (no `late_*` passes in any report) | see top 7, §3 |
| early prepass | 0.55 | 0.73 | 0.53 | 0.28 | feeds SSAO, contact shadows, DLSS | none needed | not waste |
| water chain: velocity + sspr + fft + mips + snapshot | 0.66 | 0.66 | 0.67 | - | only when water is on screen | exterior yes/no only; cost is the same in all three exterior scenes | **top 5** |
| atmosphere LUTs | 0.31 | 0.45 | 0.32 | - | only when sun, altitude or weather changes | static LUTs cached, the rest every frame | **top 6** |
| clouds: trace + composite + resolve + shadow + fog light | 0.34 | 0.36 | 0.21 | - | clear weather in every bench scene | none found | **top 8** |
| froxel fog inject + integrate | 0.17 | 0.18 | 0.13 | - | yes (distance haze) | none | not checked further |
| bloom | 0.32 | 0.31 | 0.32 | 0.36 | yes | none | standard cost, not waste |
| SSAO | 0.19 | 0.23 | 0.19 | 0.77 native | yes | none | not waste |
| sun cascade 0 (`per_view_shadow_pass`) | 0.01 | ~0.01 | ? | - | yes | every frame | not waste (§3) |
| sun cascades 1-2 (`_cascade1..3`) | 0.14 at 0.25 runs/frame | 0.17 | every frame (pre-F3 exe) | - | yes | F3 hold works | not waste (§3); weather gate missing (top 2) |
| lamp faces | ~0 | 0 views | ~0 | ~0 | n/a | vendored skip works: `views/lamp_face/work_items` median 0 | GPU not waste; CPU see §4 |
| sky probe (irradiance + radiance) | 0.08 at 0.11 runs/frame | 0.08 | 0.05 | - | slowly | time-sliced | not waste |
| `auxiliary_views` | 0.15 (CPU 0.14) | ? | 0.21 | 0.35 | ? | ? | unknown: not identified |
| `uniform_allocation` | 0.15 | 0.12 | 0.21 | 0.04 | n/a | n/a | Bevy internal |
| surface trail | 0.04 at 0.74 runs/frame | ? | 0.05 | - | no: `surface/active_cells` 0, `surface/steps` 0 | none | small waste (0.04) |
| night sky pass by day | 0.006 | ? | - | - | no | none | negligible |
| `blas_build` in the exterior | 0.02 runs/frame, ~0 ms | ? | ? | - | no (GI is interior only) | ? | negligible ms; shows the Solari scene plugin is live outdoors, main-side cost unknown |
| GI: `tlas_build` + probe update | - | - | - | 0.23 | yes (interior) | interior only | not waste |

### Render thread (W 9.75 busy, S 7.95, C 6.78)

| item | W | S | C | player sees it? | verdict |
|---|---:|---:|---:|---|---|
| `encode/submit` (serial `queue.submit`) | 2.39 | 2.04 | 1.98 | does not follow content: C draws 524 items | **top 4** (unexplained constant) |
| `encode/finish_wall` (38-39 encoders, 10.6 thread-ms) | 2.26 | 1.81 | 1.15 | partly | known (CPU frame audit); encoders 39 W vs 31.5 S |
| `camera_driver` | 1.12 | 1.00 | 0.84 | n/a | known |
| `prepare_write_phase_buffers` | 0.77 | 0.64 | 0.46 | n/a | scales weakly with content; not attributed |
| `prepare_assets` | 0.57 | 0.46 | 0.27 | S: nothing loads, yet 0.46 | **top 3** (material churn) |
| `prepare_resources` + `prepare_bind_groups` | 0.73 | 0.65 | 0.76 | n/a | partly top 3 |
| `prepare_meshes` | 0.40 | 0.20 | 0.20 | S: `assets/Mesh/added` 0, yet 0.20 | unknown: what is prepared when no mesh changes? |
| skin extract + upload (inside extract/prepare) | 4622 joints | 4512 joints, **0 bones written** | 143 characters | no change to see in S | **top 9** |
| lamp-face views kept alive | 42 views, 2148 bins, 168 pipelines | 0 | 48 views, 3603 items | n/a | §4 |
| `prepare_views` + `render/acquire` | 0.23 | 1.99 | 6.62 | n/a | GPU back-pressure wait, not work (§3) |

### Main thread (W 9.41 busy, S 7.50, C 5.80)

| item | W | S | C | player sees it? | verdict |
|---|---:|---:|---:|---|---|
| PostUpdate total | 5.91 | 4.45 | 2.99 | S: camera still, 0 entity delta, 0 bones | visibility chain known (ledger perf 2026-11-15); C is top 10 |
| visibility chain (camera, cascades, lamps, ranges) | ~2.5-3.0 est. | ? | ? | re-tests ~31.7 k static meshes against an unchanged or barely changed frustum | known, ranked in the ledger |
| Update total | 2.23 | 1.93 | 1.45 | | |
| terrain update + select | 0.59 | 0.45 | 0 (0 patches) | selection only changes when the camera crosses a LOD ring | **top 1** |
| far-field select | 0.19 | 0.18 | ? (162 entities) | same | **top 1** |
| stream drive + tick | 0.45 | 0.38 | 0.27 | rings change every few metres | **top 1** |
| bigspace (transform chain) | 0.63, p95 2.02 | 0.42 | 0.21 | S: nothing moves | part known (static transforms); S still pays 0.42 |
| controller feed + move | 0.42 | 0.29 | ? | 20 movers W; S nothing visible moves | 21 us per mover is high; not read |
| animation evaluate | 0.20 (19 of 569 evaluated) | 0.11 (7 of 414) | 3 of 143 | yes | gate works (§3) |
| physics step + insert | 0.09 | ? | ? | 0 awake of 2643 bodies | gate works (§3) |
| First + PreUpdate + FixedMain + Last | 1.10 | 0.94 | 1.17 | ? | **not attributed by any counter**; 1.1 ms constant in all scenes |
| weather, scripts, shapes | < 0.05 | | | | not waste |

## 2. Top 10, ranked by expected ms on the bound thread

Each has: evidence, why the player does not see the work, standard fix + source, first cheap measurement.

**1. Terrain, far-field and streaming selection re-run every frame (main, W).** Measured 1.23 ms per frame on the
walk (`terrain/update` 0.31 + `terrain/select` 0.28 + `far/select` 0.19 + `stream/drive` 0.26 + `stream/tick` 0.20),
1.0 ms standing with a still camera. At 4 m/s the camera moves 4.5 cm per frame; CDLOD rings, far nodes and
streaming rings change every few metres. The only gate I found is the origin-shift one
(`superfargoth/crates/sf_terrain_bevy/src/lib.rs:814, 935`), and S still pays 0.45 ms for terrain alone, so it does
not gate the cost. Standard: update selection when the source has moved past a quantum, or time-slice it. UE5 World
Partition quantises the streaming source (`wp.Runtime.UpdateStreaming.LocationQuantization` /
`RotationQuantization`); our `docs/wiki/industry/open-world-streaming.md` covers async budgets. Expected **-0.6 to
-0.9 ms main** on the walk. Confidence: cost measured, skippable share estimated (medium). First measurement: none
needed for the cost; read the three systems for a "camera moved less than X" early-out, then one `--arm` A/B with
the main column.

**2. Sun and moon cascades are drawn whenever the light is above the horizon, whatever the weather (render + main,
not in the bench).** Code read: `casts_shadows` depends on elevation only
(`superfargoth/crates/sf_sky/src/sky.rs:242-248`), and `write_light` enables the maps on that flag alone
(`superfargoth/crates/sf_sky_bevy/src/lib.rs:678, 689`). Eight of Morrowind's ten weathers (overcast, foggy, rain,
thunder, ash, blight, snow, blizzard) dim or hide the direct sun, the same "one light drowned by another" case as
lamps at noon, with ambient as the winner. At night the moon's two cascades hold 2734 items (N; that exe redrew
both every frame, the F3 hold now holds index 1). Cost while it applies: all shadows off moved the walk 10.99 ->
9.37 (`lean_shadowoff`, ledger perf 2026-11-15, old exe); after F3 the cascade share is smaller, and the main
thread pays the cascade caster scan (1.27 thread-ms, ledger 2026-11-15). Expected **-1.0 to -1.5 ms render and
-0.4 main in those weathers, 0 in the bench (clear)**. Standard: shadow cost follows the light's visible
contribution (the owner's own 1.23 rule for lamps "by perceptual value"); engines fade and drop the cascades as the
direct-to-ambient ratio falls. I have no primary source to hand for a shipped engine doing it by weather: treat the
source as owed. Confidence: gate absent = high (code read); how much our weather dims the direct sun = **not
verified**. First measurement: log sun illuminance against ambient per weather; then `--off shadows` under
`--weather overcast` with thread columns.

**3. NIF material controllers rewrite `StandardMaterial` assets every frame, seen or not (render, all scenes).**
Measured `assets/StandardMaterial/modified` 3.85 per frame W, 3.43 S, 4.17 N. `animate_materials` steps shared
materials at 30 Hz on the global clock per loaded model, with `materials.get_mut` and no visibility test
(`bevyrrowind/crates/mw_assets/src/animation.rs:38-45, 73, 89, 101, 116`). Each write is an asset change: render
world re-prepare, bind-group slot rewrite, change scans on main (0.59 thread-ms asset/material scans, ledger
2026-11-15). The 30 Hz cap already admits the cost ("cutting ... churn by 3x"). The player sees a scrolling UV or a
pulse only on screen, and a scroll needs no CPU write at all. Standard: time-driven UV scroll in the shader (UE
Panner node; one time uniform), CPU controllers only for visible objects (OpenMW updates controllers in the cull
traversal). Expected **-0.2 to -0.5 ms render** (upper bound = `prepare_assets` 0.57 + part of
`prepare_bind_groups` 0.27). Confidence low-medium: the share of `prepare_assets` is unknown. First measurement: no
switch found; a one-line env freeze of `animate_materials` and an `--arm` A/B on `stage/render/prepare_assets`.

**4. `queue.submit` costs 2.0-2.4 ms on the render thread whatever is on screen (render, all scenes).** Measured
`encode/submit` 2.39 W, 2.04 S, 1.98 C. C draws 524 main-view items against 8466 on the walk and pays the same, so
the cost is not draws. The ledger records it as "2.38 unchanged" by F1-F4 and files it under "fewer command
buffers", which the census number contradicts (C has far fewer passes). Something constant is paid each frame:
candidates are per-frame buffer writes flushed at submit, or the fixed set of encoders. This is the largest
unexplained constant on the bound thread. Expected: **up to -1.0 ms render if content-independent work is found;
unknown otherwise**. Confidence low on cause, high on the number. First measurement: `light_lab` (near-empty) and a
`--off` everything run: if submit stays ~2 ms, it is a fixed cost, then count bytes written through
`write_buffer`/`write_texture` per frame.

**5. The water chain is a fixed 0.66 ms of GPU in every exterior frame (GPU, S).** Measured: `water_velocity` +
`water_sspr` + `water_fft` + `water_mips` + `water_snapshot` = 0.66 W, 0.66 S, 0.67 N, the same from 60 m above the
river and in a street. A cost that ignores how much water is visible is not gated on it; `render.rs` extracts per
`WaterCamera` and has no visibility term (grep only: `superfargoth/crates/sf_water_bevy/src/render.rs:288, 357,
384`). Standing is GPU-bound and needs -0.67 ms, so this is the whole gap. Standard: reflections and velocity only
for water pixels, nothing when no water primitive is in view (UE5 Single Layer Water; pixel-projected reflections,
Cichocki, SIGGRAPH 2017 "Optimized pixel-projected reflections for planar reflectors"); spectrum update can run
under frame rate. Expected **-0.3 to -0.6 ms GPU** when little or no water is on screen, less when it fills the
view. Confidence: cost high, missing gate medium. First measurement: `--off waterssr` exists (ledger 2026-11-15
measured W-SSPR + Gerstner at +0.45 walk / +0.55 standing at native); re-run it interleaved on S with the GPU
column.

**6. Atmosphere LUTs are recomputed every frame (GPU, S).** Measured 0.31 W, 0.45 S, 0.32 N at 0.98 runs/frame.
Only transmittance and multiscattering are cached (`superfargoth/vendor/bevy_pbr/src/atmosphere/node.rs:35, 66,
121`); the sky-view and aerial-perspective LUTs run each frame. Standing, the camera is still and the sun moves a
few thousandths of a degree per frame. The P1 regression (0.36 -> 0.93, then P1b) is still not back to 0.36 on S.
Standard: Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique" (EGSR 2020) reports
all LUTs together at a fraction of our cost on older hardware; UE time-slices sky capture (`Real Time Capture Time
Slice`). Update the sky-view LUT on sun or altitude change, the aerial LUT on camera movement. Expected **-0.25 to
-0.4 ms GPU standing**. Confidence medium. First measurement: `--off sky` is not usable (it also removes DLSS,
ledger 2026-11-12); needs a LUT-freeze switch.

**7. 286 unshadowed lamps are shaded per pixel at noon (GPU, S).** Different from the known shadow item: this is
the lighting itself. `world/point_lights` 286 W / 210 S; the wiki states "No light budget or draw distance exists
beyond the shadow budget" (`docs/wiki/engine/lights.md:24`) and reach is 4 x radius. Under a noon exposure a lamp's
contribution is below one display step past a short distance, so most clustered light evaluations change no pixel.
Only stale numbers exist: ~0.5 ms GPU standing for ~290 lamps (ledger 2026-10-20), `--off lights` 10.53 -> 10.00
walk (2026-11-10, one run, frame delta). Standard: cull or shrink lights by contribution threshold in
clustered shading (Olsson et al., "Clustered Deferred and Forward Shading", HPG 2012); UE `MaxDrawDistance` +
`MaxDistanceFadeRange` per light; OpenMW `maximum light distance` + `light fade start`. Expected **-0.3 to -0.5 ms
GPU standing by day**. Confidence low (stale). Owner note: "lit look over correctness" applies at night and
indoors; this gate is day only. First measurement: `--off lights` on S and W by day, interleaved, GPU column.

**8. The cloud chain runs in full under a clear sky (GPU, S).** Measured 0.34 W, 0.36 S, 0.21 N (composite alone
0.11 in all three). Every bench scene is "clear". If coverage is near zero the trace, shadow and fog-light passes
produce nothing. Standard: Schneider, "The Real-time Volumetric Cloudscapes of Horizon Zero Dawn" (SIGGRAPH 2015):
quarter resolution, 1 pixel in 16 per frame, early-out on coverage; see `docs/wiki/industry/clouds-and-weather.md`.
Expected **-0.1 to -0.3 ms GPU** depending on what "clear" holds. Confidence low: I did not read what coverage
clear weather has. First measurement: `--off clouds` exists (`lean_clouds`: standing 10.46 -> 10.20, old exe).

**9. Characters too small to read are fully skinned, and unchanged poses are uploaded again (render + main, W).**
Measured W: 569 characters, 386 under 20 reference px (C2) with 1820 skinned meshes and 6398 joints, 146 C1; 19
evaluated. S: 414 characters, 4512 joints extracted and uploaded per frame with `anim/bones_written` 0. No tier
machinery exists (`superfargoth/crates/sf_character_bevy/src/crowd.rs:1-13`; gauges `:142-143`). The CROWD-1
verdict "far NPCs off -0.3, noise, tiers not built" (ledger animation 2026-11-12) was a frame delta on a max()
frame, not interleaved, before the focus guard: under the 2026-11-16 rule it is **inconclusive, not "no win"**.
Standard: UE Animation Budget Allocator and Update Rate Optimisation, skeletal LODs, no re-skin for an unchanged
pose; see `docs/wiki/industry/character-animation.md`. Expected **-0.3 to -0.6 ms render, -0.2 to -0.4 main**.
Confidence medium-low (whether `joints_uploaded` is a real upload or a buffer total was not read). First
measurement: `--off farnpcs` in an `--arm` A/B with thread columns.

**10. Inside an interior, the exterior stays in the ECS and in the schedules (main, C).** Measured in the Census
Office: 15.7 k entities, 8725 meshes, 143 characters, 156 point lights, 30 resident cells, 4955 far lights, 162 far
entities, while the main view holds 524 items; main busy 5.80 ms (PostUpdate 2.99, Update 1.45), stream 0.27. The
player sees none of the exterior indoors. Not read in code: why it stays (door prefetch is the likely reason).
Standard: keep the data resident but out of every query; Bevy 0.20's `Disabled` component with default query
filters does this natively; Bethesda and OpenMW unload or preload cells outside the scene graph; UE hidden
streaming levels do not tick. Expected **-2 to -3 ms main in interiors**. Not the bound thread there today (GPU),
so it buys pacing and battery, not the median. Confidence medium. First measurement: a clean DLSS census run (none
exists), then entity counts by worldspace through BRP.

## 3. Looks wasteful, is not (do not chase)

- **Lamp faces on the GPU.** 42-48 views, but `work_items` median 0 and `per_view_shadow_pass` 0.010 ms: the
  vendored skip works. The cost is CPU and stutter (§4).
- **Sun cascades 1-2 after F3.** `_cascade1` runs 0.25 per frame, `_cascade2/3` 0.01; cascade work items median
  519 W, 36 S. Cascade 0 every frame is what every engine does.
- **No occlusion culling** (no `late_*` passes; 8466 main items in a street). Bevy's two-phase culling saves GPU
  raster only; the walk is CPU-bound and GPU sits at 7.3 ms. Worth it only once CPU is under the GPU.
- **Animation evaluation.** 19 of 569 characters evaluated, 0.20 ms: the LOD gate works.
- **Physics.** 0.05 ms step, 0 awake of 2643 bodies.
- **Sky probe.** Time-sliced to 0.11 runs per frame, 0.08 ms.
- **Precipitation occlusion and shelter maps.** `views/other` 0 in every run; shelter redraws 0.01 per frame.
- **GI in exteriors.** No GI pass outdoors; 0.23 ms indoors.
- **`prepare_views` 1.0 S / 3.3 C and `render/acquire` 1.0 S / 3.3 C.** Back-pressure from a GPU-bound frame, not
  work; they vanish on the walk (0.14 / 0.09).
- **Bloom 0.32 and SSAO 0.19.** Standard costs at this resolution.
- **DLSS Performance instead of Quality.** Already measured slower or equal (ledger 2026-11-15).

## 4. Known item re-sized: lamp shadows by day

Smaller on the GPU than assumed, different in kind. GPU: ~0.01 ms (above). What the 42 day views cost: (a) main,
the lamp caster scan, 1.16 thread-ms (ledger 2026-11-15; the ledger's own estimate for caching it is 0.4-0.6 ms);
(b) render, 42 retained views with 2148 bins and 168 pipelines, and ~8 more encoders than standing (39 vs 31.5),
not separately timed; (c) stutters: redraw frames reach 2917 work items and 191 batch sets (W max), the "18x
3.3-4 ms" of the stutter census. So gating lamp shadows by day should be judged on main ms and stutters/min, not on
GPU or the median. The interior has the same shape: 48 views, 3603 items, median 0 drawn.

## 5. Not done (hand-off)

- No clean census run with DLSS exists; C's 7.54 ms main opaque (native) is unattributed between 8 sampled lamp
  cubes, GI and 156 clustered lights. Run `census` with `--off shadows` and `--off gi` arms.
- Night: no run after F3 and the focus guard. Moon cascade cost today is unmeasured.
- Not read: the water node body, cloud coverage in clear weather, `controller/feed`, what `auxiliary_views` and
  `prepare_meshes` do on a still frame, why the Solari scene plugin builds a BLAS outdoors, the 1.1 ms of
  First/PreUpdate/FixedMain/Last that no counter attributes.
- Wiki conflict to fix: `docs/wiki/engine/shadows.md` "Lamp cache" describes `sf_shadow_cache_bevy` as live;
  `frame-and-views.md` and ledger perf 2026-11-12 say it is compiled out (`POINT_SHADOW_CACHE = false`). The
  second is the one backed by a code line.
- Bench note: the nvidia-smi line of W reads "throttled 100% of samples" at 77 W. If that flag means power or
  thermal limiting, every GPU ms here is a throttled-GPU number; worth one look by whoever owns the bench.
- Sources were not re-fetched; item 2 has no primary source yet.
