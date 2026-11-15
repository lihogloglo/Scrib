# Lean ladder 2026-11-15: how fast with the niceties off

Build: `main` e82dfdf4 (clean tree), `cargo build --profile profiling -p bevyrrowind --features dlss`, untraced.
RTX 4060 Laptop, 2560x1440, High + DLSS Quality unless stated, 3 runs per scenario, median of runs. Command shape
(from `bevyrrowind/`): `target/profiling/bevyrrowind.exe perf suite lean_<cfg> balmora_walk balmora --repeat 3
--resolution 2560x1440 --env BEVYRROWIND_DLSS=quality [--off ...] --save benchmarks/runs/lean_<cfg>`. Folders
`benchmarks/runs/lean_<cfg>/`. Busy = mean ms per frame (`main/busy`, `render/busy`), GPU = nvidia-smi load % / Bevy-timed
GPU ms. Noise is about 0.5 ms; the base walk's own runs spread 13.05-14.87.

Rungs are cumulative from `pbr` down to `shadowoff`:
`pbr` = `--off normalmaps,orm,parallax`; `clouds` = pbr + clouds; `fog` = + fog,froxels; `envmap` = + envmap (sky probe);
`post` = + ssao,bloom,gi,surface; `shadowlite` = post + softshadows,contactshadows,farshadows; `shadowoff` = post + shadows.
`pom` = base + `--off parallax` only; `cap512` = base + `--texture-cap 512` (the Low preset's S1 cap; High is 2048).
Presets: `low` / `ultra` = `BEVYRROWIND_QUALITY` with DLSS Quality forced (Low normally runs without DLSS); `dlssperf` = High +
DLSS Performance. Medium was dropped to stay in the config budget (Low and High bracket it).

| config | walk median / p99 / stutters/min | standing median / p99 | walk main / render / GPU | standing main / render / GPU | top GPU passes (standing, ms) |
|---|---|---|---|---|---|
| base (High) | **14.15** / 21.87 / 27.4 | **11.41** / 14.27 | 9.83 / 12.56 / 73% 8.04 | 7.81 / 9.47 / 96% 9.03 | main_opaque 2.97, DLSS 2.11, prepass 0.62, atmos LUTs 0.34, bloom 0.32, transmissive 0.29, water_velocity 0.25, water_sspr 0.23, ssao 0.20 |
| pbr (normal+ORM+POM off) | 11.42 / 16.68 / 32.9 | 10.46 / 12.78 | 8.85 / 9.75 / 84% 7.24 | 7.12 / 7.58 / 98% 8.74 | main_opaque 2.64, DLSS 2.20, prepass 0.60 |
| pom (parallax off only) | 12.95 / 19.14 / 19.2 | 10.81 / 13.77 | 9.29 / 11.55 / 77% 7.79 | 7.37 / 8.35 / 98% 8.83 | main_opaque 2.68 |
| cap512 (texture cap 512) | 13.85 / 19.98 / 14.6 | 11.14 / 13.67 | 9.63 / 12.47 / 73% 7.99 | 7.59 / 9.29 / 97% 9.18 | as base |
| clouds | 11.01 / 15.92 / 29.2 | 10.20 / 12.13 | 8.62 / 9.49 / 85% 7.07 | 6.97 / 7.40 / 98% 8.47 | |
| fog (+fog, froxels) | 10.91 / 16.69 / 25.6 | 9.66 / 12.08 | 8.66 / 9.56 / 81% 6.83 | 7.14 / 7.54 / 97% 7.97 | |
| envmap (+sky probe) | 11.19 / 16.88 / 30.2 | 9.18 / 11.72 | 8.84 / 9.83 / 75% 6.67 | 7.13 / 7.38 / 96% 7.28 | main_opaque 2.07, DLSS 2.00 |
| post (+ssao, bloom, gi, surface) | 10.92 / 17.04 / 31.1 | 8.61 / 11.06 | 8.77 / 9.47 / 69% 5.88 | 7.01 / 7.08 / 93% 6.24 | |
| shadowlite (+soft, contact, far shadows) | 10.99 / 18.37 / 25.6 | 8.53 / 11.31 | 9.12 / 9.86 / 64% 5.52 | 6.89 / 7.03 / 90% 5.72 | |
| **shadowoff** (post + all shadows off) | **9.37** / 14.80 / 44.8 (2 runs) | **7.77** / 10.96 | 8.11 / 7.84 / 69% 5.60 | 6.55 / 6.31 / 91% 5.95 | DLSS 1.94, main_opaque 1.71, prepass 0.50, atmos LUTs 0.35, water_velocity 0.23, transmissive 0.21, water_sspr 0.21, water_fft 0.13, upscaling 0.12 |
| shadowoff, `balmora_night` | | 7.52 / 10.35 | | 6.40 / 6.07 / 92% 5.76 | DLSS 1.90, main_opaque 1.54 |
| low (DLSS Q forced) | 9.98 / 13.66 / 12.8 | 8.78 / 11.83 | 7.46 / 8.89 / 72% 5.65 | 6.37 / 7.14 / 93% 6.96 | main_opaque 2.17, DLSS 1.98 |
| ultra (no RT) | **out of GPU memory, 3/3 runs** | 11.70 / 14.64 | | 7.81 / 9.74 / 96% 9.04 | main_opaque 3.04, DLSS 2.00, ssao 0.32 |
| dlssperf (High, DLSS Performance) | 13.81 / 20.24 / 19.2 | 11.64 / 14.36 | 9.65 / 12.54 / 75% 8.31 | 7.44 / 9.07 / 97% 9.53 | main_opaque 2.87, DLSS 2.71 |

Walk top GPU passes, base: DLSS 2.19, main_opaque 1.74, prepass 0.54, bloom 0.34, atmos LUTs 0.33, uniform_allocation 0.32,
water_velocity 0.30, bin_unpacking 0.25, water_sspr 0.23, ssao 0.20. Leanest (`shadowoff`): DLSS 2.06, main_opaque 1.20,
prepass 0.46, atmos LUTs 0.31, water_velocity 0.28, water_sspr 0.21, upscaling 0.14, auxiliary_views 0.14.

## Readings

- **PBR maps cost 2.7 ms on the walk (14.15 -> 11.42) and 0.95 ms standing (11.41 -> 10.46).** On the walk it is render-thread CPU,
  not GPU: `render/busy` 12.56 -> 9.75, `render/graph` 8.2 -> 5.5 ms per frame, timed GPU only 8.04 -> 7.24. Batch sets
  157 -> 111 (all views; main view 92 -> 60), opaque3d pipelines 27 -> 21, distinct pipelines 97 -> 82, and pipeline compiles
  keep running during the walk (`pipelines/compile_ms` 0.84 vs 0.23 per frame). The extra material variants split batches in
  every view. Standing it is GPU shading (main_opaque 2.97 -> 2.64). POM alone: walk -1.20, standing -0.60; normal + ORM by
  difference: walk ~-1.5, standing ~-0.35.
- **Texture residency/size (S1 cap) costs nothing measurable:** cap 512 cuts texture GPU memory 2822 -> 1728 MiB and VRAM
  6.3 -> 5.3 GB, with frame time within noise (-0.3 ms on both). Turning the maps off does not free their textures (2822 MiB stays).
- **Ultra cannot run the walk on 8 GB:** 4354 MiB of live textures (BC5 3 images = 1520 MiB, the terrain layer arrays
  uncapped) -> wgpu "Out of Memory" in all 3 walk runs (`target/perf/lean_ultra/balmora_walk@2560x1440-*/log.txt`).
- Sky, fog, probe and post niceties only help standing (GPU bound): 10.46 -> 8.61 together (clouds -0.26, fog/froxels -0.54,
  sky probe -0.48, ssao/bloom/gi/surface -0.57). The walk stays at ~11 ms through all of them (render-thread bound).
- Shadow refinements (soft, contact, far) are noise on both. All shadows off: walk -1.55 (render busy 9.47 -> 7.84, fewer views),
  standing -0.84.
- **Leanest config (`shadowoff`): walk 9.37 ms, standing 7.77, night 7.52.** The walk is CPU-bound with main and render even
  (main 8.11, render 7.84, GPU 69%); standing is near GPU bound (91%, DLSS 1.94 + main opaque 1.71 the top passes).
  Stutters go up when the frame gets shorter (44.8/min: the 1.5x-moving-median stutter rule counts the same spikes as larger).
- The Low preset with DLSS Q (walk 9.98, standing 8.78) lands next to the leanest ladder rung, keeping shadows.
- DLSS Performance does not help: walk -0.34 (noise, CPU bound), standing +0.23; the DLSS pass itself costs more (2.71 vs 2.11).
- Regression not attributed: even with PBR off, the walk is 11.42 vs 9.96 on 2026-11-12 (`m_base`) and standing 10.46 vs 9.42;
  batch sets 111 vs 84 and new GPU passes `water_velocity` + `water_sspr` (~0.5 ms) since then. Base walk +4.2 ms over 3 days.

Failed runs: `shadowoff` walk #3 GPU DeviceLost (driver), not re-run (#1 and #2 both 9.37); `ultra` walk 3/3 OOM (above).
None hit the render-layer exhaustion panic (ROADMAP 1.1). No busy-machine re-runs were needed; the suite waited for quiet
CPU before `ultra` standing and `shadowoff` walk.
