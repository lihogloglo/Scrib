# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1187 frames over 15.0 s · **median 12.70 ms (79 fps)** · mean 12.64 · p95 14.04 · p99 14.64 (1% low 68 fps) · worst 17.76 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 312 frames over 9.4 s, worst 2772.0 ms
- hitches (> 31.7 ms): **0** measured, 28 in warm-up
- timed GPU passes: 11.14 ms of a 12.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.70 ms, limit 11 ms
- pass worst frame 17.76 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.215 | 3.215 | 3.308 | 1.00 | 0.092 |
| dlss_super_resolution | 2.022 | 2.022 | 2.044 | 1.00 | 0.178 |
| sun_visibility | 0.784 | 0.784 | 0.810 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.778 | 0.778 | 0.900 | 1.00 | 0.008 |
| early prepass | 0.627 | 0.627 | 0.634 | 1.00 | 0.124 |
| main_transmissive_pass_3d | 0.397 | 0.397 | 0.408 | 1.00 | 0.007 |
| atmosphere_luts | 0.376 | 0.376 | 0.386 | 1.00 | 0.003 |
| bloom | 0.315 | 0.315 | 0.331 | 1.00 | 0.058 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.307 | 1.00 | 0.001 |
| water_velocity | 0.307 | 0.307 | 0.311 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.238 | 0.238 | 0.270 | 1.00 | 0.004 |
| water_sspr | 0.217 | 0.217 | 0.210 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.169 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.135 | 0.139 | 1.00 | 0.002 |
| uniform_allocation | 0.121 | 0.121 | 0.154 | 1.00 | 0.018 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.113 | 0.113 | 0.118 | 1.00 | 0.004 |
| bin_unpacking | 0.098 | 0.098 | 0.123 | 1.00 | 0.013 |
| tonemapping | 0.085 | 0.085 | 0.089 | 1.00 | 0.003 |
| range_unpacking | 0.084 | 0.084 | 0.094 | 1.00 | 0.003 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.230 | 0.12 | 0.003 |
| per_view_shadow_pass | 0.026 | 0.026 | 0.052 | 1.00 | 0.006 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.022 | 1.00 | 0.183 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| surface_exposure_far | 0.011 | 0.034 | 0.035 | 0.31 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| surface_exposure | 0.010 | 0.035 | 0.035 | 0.28 | 0.001 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.650 | 13.089 | 16.62 | 13828.9 |
| render/busy/ms | 7.431 | 8.474 | 16.15 | 8820.1 |
| main/busy/ms | 6.468 | 7.668 | 10.58 | 7677.5 |
| encode//sum | 5.040 | 6.522 | 11.25 | 5981.9 |
| stage/render/render | 4.930 | 5.718 | 7.42 | 5852.2 |
| render/graph/ms | 4.342 | 5.091 | 6.49 | 5153.6 |
| stage/render/prepare_views | 3.913 | 5.407 | 6.92 | 4644.9 |
| render/acquire/ms | 3.863 | 5.353 | 6.85 | 4585.5 |
| stage/main/post_update | 3.576 | 4.484 | 6.62 | 4244.6 |
| stage/render/graph_submit | 3.132 | 3.838 | 4.98 | 3717.2 |
| encode/submit | 1.899 | 2.285 | 3.02 | 2254.1 |
| stage/main/update | 1.311 | 1.512 | 2.57 | 1556.3 |
| encode/finish_wall | 1.253 | 1.681 | 2.53 | 1487.5 |
| stage/render/camera_driver | 1.124 | 1.298 | 2.16 | 1334.6 |
| pipelines/compile_ms | 1.007 | 0.000 | 124.26 | 1194.8 |
| pipelines/front_ms | 0.980 | 0.000 | 121.92 | 1163.5 |
| encode/pass:early prepass/sum | 0.805 | 1.098 | 1.41 | 955.3 |
| encode/pass:shadow_directional_light/sum | 0.791 | 1.416 | 2.10 | 938.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.782 | 1.076 | 1.55 | 928.5 |
| stage/render/prepare_write_phase_buffers | 0.482 | 0.698 | 0.89 | 572.7 |
| stage/main/fixed_main_loop | 0.460 | 0.664 | 1.24 | 546.4 |
| stage/render/prepare_resources | 0.421 | 0.493 | 0.75 | 499.8 |
| stage/render/prepare_assets | 0.392 | 0.476 | 1.53 | 465.5 |
| stage/main/pre_update | 0.386 | 0.450 | 1.03 | 457.8 |
| render/present/ms | 0.357 | 0.422 | 0.71 | 423.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.341 | 0.478 | 0.90 | 404.4 |
| stage/main/first | 0.316 | 0.378 | 1.71 | 375.1 |
| bigspace/total/ms | 0.304 | 0.465 | 1.79 | 360.6 |
| stage/render/prepare_flush | 0.269 | 0.364 | 0.86 | 318.9 |
| stage/render/prepare_batch_phases | 0.267 | 0.387 | 0.63 | 317.1 |
| stage/render/prepare_bind_groups | 0.256 | 0.370 | 0.48 | 304.1 |
| stage/main/last | 0.246 | 0.365 | 0.87 | 291.9 |
| stage/render/prepare_meshes | 0.222 | 0.274 | 1.17 | 263.4 |
| stage/render/extract_commands | 0.192 | 0.220 | 0.37 | 227.7 |
| controller/feed/ms | 0.175 | 0.220 | 0.56 | 208.0 |
| stage/render/create_views | 0.154 | 0.187 | 0.33 | 182.2 |
| stream/drive/ms | 0.134 | 0.157 | 0.53 | 158.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.123 | 0.185 | 0.41 | 146.2 |
| stream/tick/ms | 0.107 | 0.126 | 0.27 | 127.2 |
| stage/render/queue | 0.105 | 0.144 | 2.35 | 124.7 |
| bigspace/high_precision/ms | 0.102 | 0.166 | 1.22 | 121.5 |
| controller/move/ms | 0.089 | 0.162 | 0.31 | 105.1 |
| bigspace/low_precision/ms | 0.077 | 0.103 | 1.00 | 92.0 |
| stage/render/specialize | 0.077 | 0.102 | 3.59 | 91.8 |
| anim/evaluate/ms | 0.066 | 0.079 | 0.59 | 78.6 |
| anim/evaluate_cpu/ms | 0.060 | 0.072 | 0.59 | 71.5 |
| encode/resolve_diagnostics/sum | 0.059 | 0.088 | 0.27 | 69.7 |
| controller/write/ms | 0.053 | 0.068 | 0.13 | 62.8 |
| stage/render/prepare_collect_phase_buffers | 0.044 | 0.076 | 0.26 | 52.3 |
| terrain/update/ms | 0.035 | 0.046 | 0.07 | 41.1 |
| physics/step/ms | 0.031 | 0.048 | 0.16 | 37.2 |
| pipelines/driver_ms | 0.026 | 0.000 | 3.69 | 31.2 |
| stage/render/cleanup | 0.026 | 0.034 | 0.49 | 31.1 |
| stage/render/phase_sort | 0.025 | 0.046 | 0.23 | 29.4 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.038 | 0.22 | 29.4 |
| weather/map/ms | 0.024 | 0.033 | 0.09 | 28.5 |
| encode/camera_driver_span_end/sum | 0.021 | 0.034 | 0.17 | 25.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.05 | 12.7 |
| script/update/ms | 0.007 | 0.008 | 0.08 | 7.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 6.0 |
| controller/avoid/ms | 0.005 | 0.007 | 0.03 | 5.6 |
| anim/apply/ms | 0.004 | 0.005 | 0.13 | 5.2 |
| far/select/ms | 0.004 | 0.004 | 0.14 | 4.3 |
| script/ui/ms | 0.003 | 0.004 | 0.07 | 3.6 |
| stream/assets_ready/ms | 0.002 | 0.002 | 0.40 | 2.1 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.02 | 2.0 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| stream/promote/ms | 0.001 | 0.002 | 0.02 | 1.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.0 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.7 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.6 |
| stage/render/prepare | 0.000 | 0.002 | 0.06 | 0.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.3 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/chains/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/plan/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/wanted/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 148 | 181 | 148 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.60 | 1.72 | 0.54 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.22 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.17 | 0.01 |
| encode/finish_critical | ms | 0.94 | 2.10 | 0.78 |
| encode/pass:early prepass/max | ms | 0.81 | 1.41 | 0.66 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.55 | 0.78 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.41 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.34 | 0.90 | 0.41 |
| encode/pass:shadow_directional_light/max | ms | 0.74 | 2.10 | 0.64 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.27 | 0.05 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.78 | 16.15 | 11.48 |
| gpu/timed/ms | ms | 11.78 | 13.20 | 11.48 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1416 | 1416 | 1416 |
| groundcover/slots/capacity | count | 658.0 k | 658.0 k | 658.0 k |
| groundcover/slots/capacity_bytes | bytes | 42.11 M | 42.11 M | 42.11 M |
| groundcover/slots/live | count | 0 | 0 | 0 |
| groundcover/tiles/capped | count | 0 | 0 | 0 |
| groundcover/tiles/casting | count | 6 | 6 | 6 |
| groundcover/tiles/resident | count | 63 | 63 | 63 |
| groundcover/tiles/scattered | count | 63 | 63 | 63 |
| groundcover/tiles/unplaced | count | 0 | 0 | 0 |
| groundcover/upload/bytes | bytes | 0 | 0 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1931 | 1931 | 1931 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 217.5 k | 217.5 k | 217.5 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.3 k | 11.3 k | 11.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
| phases/alpha_mask3d/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d/bins | count | 258 | 258 | 258 |
| phases/alpha_mask3d/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d/instance_draws | count | 573 | 573 | 573 |
| phases/alpha_mask3d/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d/items | count | 329 | 329 | 329 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d_prepass/bins | count | 258 | 258 | 258 |
| phases/alpha_mask3d_prepass/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d_prepass/instance_draws | count | 573 | 573 | 573 |
| phases/alpha_mask3d_prepass/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d_prepass/items | count | 329 | 329 | 329 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 31 | 31 | 31 |
| phases/opaque3d/bins | count | 884 | 884 | 884 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/bins | count | 884 | 884 | 884 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 9 | 56 | 9 |
| phases/shadow/bins | count | 911 | 1524 | 911 |
| phases/shadow/instance_batches | count | 9 | 27 | 9 |
| phases/shadow/instance_draws | count | 5866 | 17.6 k | 5866 |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| phases/shadow/items | count | 3217 | 8176 | 3217 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 40 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 346 | 346 | 346 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 23.21 | 32.69 | 14.88 |
| process/mem_usage | GiB | 3.02 | 3.08 | 3.08 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2386 | 2386 | 2386 |
| scene/batch/masked/live | count | 2386 | 2386 | 2386 |
| scene/batch/masked/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/masked_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_wind/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_wind/count | count | 0 | 0 | 0 |
| scene/batch/masked_wind/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_wind/live | count | 0 | 0 | 0 |
| scene/batch/masked_wind/overflow | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/masked_wind_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/opaque_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/count | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/live | count | 0 | 0 | 0 |
| scene/batch/opaque_wind/overflow | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/count | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/live | count | 0 | 0 | 0 |
| scene/batch/opaque_wind_two_sided/overflow | count | 0 | 0 | 0 |
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1639 | 1639 | 1639 |
| scene/chains/live | count | 1639 | 1639 | 1639 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 0 | 0 | 0 |
| shadow_atlas/live_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/live_tiles | count | 0 | 0 | 0 |
| shadow_atlas/mover_draws | count | 0 | 0 | 0 |
| shadow_atlas/movers | count | 0 | 0 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 0 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/live_direct | count | 0 | 0 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 0 | 0 | 0 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/static_tiles | count | 0 | 0 | 0 |
| shadow_atlas/views | count | 0 | 0 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 5 | 7 | 5 |
| skin/blocks_skipped/count | count | 176 | 178 | 176 |
| skin/blocks_walked/count | count | 5 | 7 | 5 |
| skin/joint_bytes_written/bytes | bytes | 10.5 k | 12.5 k | 10.2 k |
| skin/joints_written/count | count | 164 | 196 | 160 |
| skin/upload_writes/count | count | 3 | 5 | 4 |
| started_load_count |  | 5306 | 5308 | 5308 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.88 | 42.09 | 26.28 |
| system/mem_usage | % | 55.76 | 56.02 | 56.02 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 105 | 152 | 105 |
| views/all/bins | count | 3195 | 3808 | 3195 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 17.6 k | 29.4 k | 17.6 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 9459 | 14.4 k | 9459 |
| views/all/pipelines | count | 89 | 96 | 89 |
| views/all/pipelines_distinct | count | 73 | 73 | 73 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.50 M |
| views/cascade/batch_sets | count | 9 | 56 | 9 |
| views/cascade/bins | count | 911 | 1524 | 911 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 9 |
| views/cascade/instance_draws | count | 5866 | 17.6 k | 5866 |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| views/cascade/items | count | 3217 | 8176 | 3217 |
| views/cascade/pipelines | count | 33 | 40 | 33 |
| views/cascade/work_items | count | 82.9 k | 256.7 k | 82.9 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 96 | 96 | 96 |
| views/main/bins | count | 2284 | 2284 | 2284 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 11.8 k | 11.8 k | 11.8 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6242 | 6242 | 6242 |
| views/main/pipelines | count | 56 | 56 | 56 |
| views/main/work_items | count | 1.42 M | 1.42 M | 1.42 M |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.5 k | 15.5 k | 15.5 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.12 | 2 | 141 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 0.00 | 4 | 4 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.56 | 31 | 33.9 k |
| encode/bind_walk/init_textures | count | 7457 | 12.4 k | 8.85 M |
| encode/bind_walk/init_textures_skipped | count | 41.1 k | 46.9 k | 48.84 M |
| encode/bind_walk/inits | count | 652 | 779 | 774.4 k |
| encode/bind_walk/inits_skipped | count | 123 | 133 | 146.3 k |
| encode/bind_walk/merge_textures | count | 7392 | 12.3 k | 8.77 M |
| encode/bind_walk/merge_textures_skipped | count | 14.2 k | 19.9 k | 16.80 M |
| encode/bind_walk/merges | count | 135 | 162 | 160.0 k |
| encode/bind_walk/merges_skipped | count | 112 | 128 | 133.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1187 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1187 |
| encode/pass:early prepass/count | count | 1 | 1 | 1187 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1187 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1187 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1187 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1187 |
| encode/queue/command_buffers | count | 38.56 | 41 | 45.8 k |
| encode/queue/submits | count | 2 | 2 | 2374 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.24 G |
| encode/queue/write_buffer/calls | count | 250 | 323 | 297.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 38.74 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.7 k |
| encode/queue/write_texture/bytes | bytes | 22.2 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1187 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.76 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 19 |
| pipelines/created | count | 0.02 | 1 | 18 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
| scene/load/deferred | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 1 |
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.7 ms; slowest 28, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.77 | 2772.0 | main/busy/ms 2740, stage/main/post_update 2626, stage/main/update 99.90, surface/advance_cpu_ms 68.42, stage/main/pre_update 9.75, stage/main/last 1.02, surface/cell_upload_ms 0.75, stage/main/first 0.56, weather/map/ms 0.49, stage/main/fixed_main_loop 0.49, stream/drive/ms 0.30, bigspace/total/ms 0.08 |
| warm-up | 3.56 | 792.7 | main/busy/ms 2789, stage/main/post_update 2661, render/thread/ms 731, render/busy/ms 576, stage/render/prepare_assets 551, stage/render/prepare_views 152, render/acquire/ms 152, stage/main/update 106, surface/advance_cpu_ms 68.42, stage/render/render 11.90, stage/main/pre_update 11.89, render/graph/ms 7.66 |
| warm-up | 3.67 | 106.9 | render/thread/ms 835, render/busy/ms 626, stage/render/prepare_assets 555, stage/render/prepare_views 152, render/acquire/ms 152, pipelines/compile_ms 106, main/busy/ms 105, stage/render/render 103, pipelines/front_ms 101, stage/main/post_update 69.28, render/present/ms 56.86, render/graph/ms 44.24 |
| warm-up | 3.76 | 87.2 | pipelines/compile_ms 1386, pipelines/front_ms 982, pipelines/driver_ms 404, pipelines/wait_ms 235, render/thread/ms 190, render/busy/ms 132, stage/render/render 128, main/busy/ms 87.24, pipelines/lock_ms 75.96, render/graph/ms 69.87, stage/main/post_update 56.90, render/present/ms 56.67 |
| warm-up | 3.89 | 135.8 | pipelines/wait_ms 3925, pipelines/compile_ms 2993, pipelines/front_ms 2480, pipelines/driver_ms 513, render/thread/ms 219, render/busy/ms 214, main/busy/ms 164, pipelines/lock_ms 84.60, stage/main/update 72.29, stage/main/post_update 69.89, stage/render/render 67.80, encode//sum 63.02 |
| warm-up | 4.04 | 143.9 | pipelines/wait_ms 8588, pipelines/compile_ms 3454, pipelines/front_ms 3243, render/thread/ms 266, render/busy/ms 261, pipelines/driver_ms 211, main/busy/ms 180, stage/render/prepare_assets 96.27, stage/main/update 81.47, stage/main/post_update 76.71, stage/render/render 45.52, render/graph/ms 41.54 |
| warm-up | 4.08 | 37.4 | pipelines/wait_ms 5288, pipelines/compile_ms 2277, pipelines/front_ms 2155, render/thread/ms 167, render/busy/ms 164, pipelines/driver_ms 122, stage/render/prepare_assets 92.95, main/busy/ms 72.50, stage/main/post_update 41.10, stage/render/prepare_meshes 35.52, stage/main/update 23.46, stage/render/render 19.59 |
| warm-up | 4.17 | 39.0 | pipelines/compile_ms 1192, pipelines/front_ms 1128, pipelines/wait_ms 430, pipelines/driver_ms 63.33, render/thread/ms 62.90, render/busy/ms 60.68, main/busy/ms 47.07, stage/main/post_update 32.73, stage/render/render 20.99, render/graph/ms 18.50, stage/render/graph_submit 15.36, stage/render/prepare_resources 10.59 |
| warm-up | 4.28 | 88.2 | pipelines/compile_ms 1041, pipelines/front_ms 985, render/thread/ms 97.79, render/busy/ms 91.05, pipelines/driver_ms 56.16, stage/render/prepare_assets 53.67, main/busy/ms 45.53, stage/render/render 32.19, render/graph/ms 24.60, pipelines/wait_ms 20.03, stage/render/graph_submit 16.92, stage/main/update 15.98 |
| warm-up | 4.32 | 36.5 | pipelines/compile_ms 1532, pipelines/front_ms 1465, render/thread/ms 110, render/busy/ms 102, pipelines/driver_ms 66.82, stage/render/prepare_assets 60.91, main/busy/ms 43.52, stage/render/render 32.54, pipelines/wait_ms 24.28, render/graph/ms 24.17, stage/main/post_update 17.24, stage/render/graph_submit 16.49 |
| warm-up | 4.40 | 82.1 | pipelines/compile_ms 856, pipelines/front_ms 831, render/thread/ms 115, render/busy/ms 112, stage/render/prepare_assets 83.40, main/busy/ms 52.65, pipelines/driver_ms 24.66, stage/main/post_update 22.78, stage/render/render 18.80, render/graph/ms 15.51, pipelines/wait_ms 13.78, pipelines/lock_ms 12.95 |
| warm-up | 4.56 | 37.8 | pipelines/compile_ms 679, pipelines/front_ms 656, render/thread/ms 63.66, main/busy/ms 60.34, render/busy/ms 60.15, stage/main/update 34.40, stage/render/render 23.91, pipelines/driver_ms 22.19, render/graph/ms 19.63, stage/render/prepare_assets 19.43, stage/main/post_update 17.14, pipelines/wait_ms 13.00 |
| warm-up | 4.61 | 48.6 | pipelines/compile_ms 364, pipelines/front_ms 352, render/thread/ms 81.15, render/busy/ms 79.13, main/busy/ms 61.85, stage/render/prepare_assets 47.97, stage/main/update 37.50, stage/render/render 14.79, stage/main/post_update 13.44, pipelines/driver_ms 12.06, render/graph/ms 11.70, stage/render/graph_submit 8.14 |
| warm-up | 4.66 | 51.1 | pipelines/compile_ms 145, pipelines/front_ms 139, render/thread/ms 94.24, render/busy/ms 92.17, stage/render/prepare_assets 68.94, main/busy/ms 66.48, stage/main/update 36.74, stage/main/post_update 18.80, stage/render/render 14.82, render/graph/ms 12.31, stage/render/graph_submit 8.92, encode//sum 8.43 |
| warm-up | 4.71 | 48.2 | pipelines/compile_ms 97.07, render/thread/ms 95.53, render/busy/ms 93.75, pipelines/front_ms 92.27, main/busy/ms 64.42, stage/render/prepare_assets 64.12, stage/main/update 37.02, stage/main/post_update 16.92, stage/render/render 14.35, render/graph/ms 12.20, stage/render/graph_submit 9.00, encode//sum 8.04 |
| warm-up | 4.75 | 49.3 | pipelines/compile_ms 1287, pipelines/front_ms 1252, render/thread/ms 93.94, render/busy/ms 92.39, pipelines/wait_ms 86.78, main/busy/ms 55.98, stage/render/prepare_assets 55.61, pipelines/driver_ms 35.70, stage/main/post_update 22.44, stage/main/update 21.54, stage/render/render 13.27, render/graph/ms 11.26 |
| warm-up | 4.83 | 74.1 | pipelines/compile_ms 1995, pipelines/front_ms 1937, pipelines/wait_ms 444, render/thread/ms 120, render/busy/ms 119, pipelines/driver_ms 58.06, main/busy/ms 48.29, stage/render/prepare_assets 45.14, stage/main/post_update 20.76, stage/render/prepare_write_phase_buffers 17.24, stage/render/prepare_resources 16.13, stage/main/update 15.47 |
| warm-up | 4.89 | 62.4 | pipelines/compile_ms 1023, pipelines/front_ms 974, pipelines/wait_ms 527, render/thread/ms 134, render/busy/ms 132, main/busy/ms 108, pipelines/driver_ms 48.19, stage/render/prepare_resources 45.72, stage/main/update 45.37, stage/main/post_update 40.41, stage/render/prepare_assets 28.88, stage/render/prepare_write_phase_buffers 19.46 |
| warm-up | 4.98 | 84.6 | pipelines/compile_ms 2128, pipelines/front_ms 2055, pipelines/wait_ms 509, main/busy/ms 153, render/thread/ms 106, render/busy/ms 104, pipelines/driver_ms 72.52, stage/main/update 65.81, stage/main/post_update 49.29, stage/render/prepare_assets 39.92, stage/render/prepare_resources 32.17, stage/main/pre_update 26.29 |
| warm-up | 5.06 | 84.3 | pipelines/compile_ms 2663, pipelines/front_ms 2585, pipelines/wait_ms 447, render/thread/ms 112, render/busy/ms 110, main/busy/ms 95.93, pipelines/driver_ms 78.00, stage/render/prepare_assets 50.53, stage/main/update 45.21, stage/main/post_update 24.23, stage/main/pre_update 16.09, stage/render/render 14.30 |
| warm-up | 5.10 | 38.2 | pipelines/compile_ms 856, pipelines/front_ms 825, pipelines/wait_ms 108, render/thread/ms 103, render/busy/ms 102, main/busy/ms 60.91, stage/render/prepare_assets 34.20, pipelines/driver_ms 31.05, stage/main/update 29.12, stage/main/post_update 16.78, stage/render/render 15.93, stage/render/prepare_write_phase_buffers 14.40 |
| warm-up | 5.15 | 47.9 | pipelines/compile_ms 913, pipelines/front_ms 887, pipelines/wait_ms 147, render/thread/ms 82.59, render/busy/ms 81.07, main/busy/ms 46.48, stage/render/prepare_assets 41.11, pipelines/driver_ms 25.96, stage/main/update 23.08, stage/render/render 16.39, render/graph/ms 14.30, stage/main/post_update 10.48 |
| warm-up | 5.27 | 33.8 | pipelines/compile_ms 170, pipelines/front_ms 163, main/busy/ms 32.79, render/thread/ms 31.32, render/busy/ms 30.10, stage/main/update 16.41, stage/render/render 12.43, stage/main/post_update 10.61, render/graph/ms 10.55, encode//sum 8.78, stage/render/graph_submit 6.90, pipelines/driver_ms 6.47 |
| warm-up | 5.45 | 44.6 | pipelines/compile_ms 232, pipelines/front_ms 226, render/thread/ms 60.41, render/busy/ms 59.04, main/busy/ms 43.10, stage/main/update 28.16, stage/render/render 16.05, render/graph/ms 14.14, stage/render/prepare_resources 12.75, stage/render/graph_submit 10.74, encode//sum 9.43, stage/main/post_update 9.24 |
| warm-up | 5.50 | 55.6 | pipelines/compile_ms 277, pipelines/front_ms 270, render/thread/ms 92.16, render/busy/ms 90.67, main/busy/ms 67.17, stage/main/update 40.14, pipelines/wait_ms 31.36, stage/render/prepare_assets 25.53, stage/render/prepare_resources 20.26, stage/main/post_update 20.17, stage/render/prepare_write_phase_buffers 15.31, stage/render/render 14.36 |
| warm-up | 5.54 | 35.7 | pipelines/compile_ms 975, pipelines/front_ms 948, render/thread/ms 88.06, render/busy/ms 86.27, main/busy/ms 65.24, pipelines/wait_ms 31.86, stage/main/update 28.78, pipelines/driver_ms 27.30, stage/render/prepare_assets 27.29, stage/main/post_update 25.17, stage/render/render 18.98, render/graph/ms 16.78 |
| warm-up | 5.81 | 48.0 | render/thread/ms 71.96, render/busy/ms 70.28, main/busy/ms 47.39, stage/render/prepare_assets 31.06, stage/main/update 22.91, stage/render/render 17.48, stage/main/post_update 15.46, render/graph/ms 14.86, encode//sum 12.18, stage/render/graph_submit 10.73, stage/render/prepare_meshes 7.66, encode/submit 7.29 |
| warm-up | 6.61 | 32.0 | render/thread/ms 47.96, render/busy/ms 45.32, main/busy/ms 40.48, stage/render/render 21.27, render/graph/ms 18.29, stage/main/update 16.97, stage/main/post_update 16.19, encode//sum 14.06, stage/render/graph_submit 12.85, stage/render/prepare_assets 10.33, encode/submit 8.34, stream/drive/ms 5.09 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=on-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.0 W · 76 °C (max 79) · VRAM 7221 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `render.bloom=on (preset: sg.post high`
- `industry: screen-space-effects research 4.11 (bloom)`
- `ROADMAP 1.3 (temporal resolve))`
- `render.bloom.intensity=0.1 (preset: sg.post high`
- `industry: screen-space-effects research 4.11 (bloom)`
- `ROADMAP 1.3 (temporal resolve))`
- `render.gi.probes=on (preset: sg.lighting high`
- `measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level)`
- `gi.md slice 1)`
- `render.ssao.quality=medium (preset: sg.lighting high`
- `measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level)`
- `gi.md slice 1)`
- `render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value))`
- `sf.graphics_preset=high (preset: level requested)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=high (preset: level requested)`
- `sg.shadows=high (preset: level requested)`
- `sg.sky=high (preset: level requested)`
- `sg.textures=high (preset: level requested)`
- `sg.view_distance=high (preset: level requested)`
- `sg.water=high (preset: level requested)`
- `shadow.contact=on (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.length_m=0.3 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.steps=16 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.thickness_m=0.1 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.filter=auto (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.lamp.contact=on (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.lamp.count=8 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01`
- `framework default 15-25 m))`
- `shadow.lamp.map_size_px=512 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.cascades=3 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.distance_m=205 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.overlap=0.16 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.soft=on (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.soft_size_texels=10 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.split_lambda=0.8 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `sky.clouds.map_per_sample=on (preset: sg.sky high`
- `industry: weather.md (distant weather))`
- `surface.puddles=on (preset: sg.effects high`
- `industry: surface-weather.md "Budget")`
- `surface.shelter=on (preset: sg.effects high`
- `industry: surface-weather.md "Budget")`
- `texture.max_size_px=2048 (preset: sg.textures high`
- `measured: docs/ledger/textures.md 2026-11-13 (cap)`
- `textures.md §9 T4d (parallax))`
- `texture.parallax.layers=32 (preset: sg.textures high`
- `measured: docs/ledger/textures.md 2026-11-13 (cap)`
- `textures.md §9 T4d (parallax))`
- `texture.parallax.relief_steps=5 (preset: sg.textures high`
- `measured: docs/ledger/textures.md 2026-11-13 (cap)`
- `textures.md §9 T4d (parallax))`
- `water.quality=high (preset: sg.water high`
- `industry: water.md "Performance budgets")`
