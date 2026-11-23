# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.density_scale=0 (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 377 frames over 15.0 s · **median 36.70 ms (27 fps)** · mean 39.80 · p95 85.37 · p99 93.25 (1% low 11 fps) · worst 94.95 ms · 25 spikes over 1.5× median · 100.0 stutters/min (work 100.0/min)
- warm-up: 307 frames over 19.8 s, worst 2576.4 ms
- hitches (> 91.8 ms): **7** measured, 37 in warm-up
- timed GPU passes: 34.45 ms of a 39.80 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 36.70 ms, limit 11 ms
- **FAIL** worst frame 94.95 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 10.099 | 10.099 | 10.250 | 1.00 | 0.088 |
| early prepass | 9.211 | 9.211 | 9.332 | 1.00 | 0.130 |
| range_unpacking | 5.990 | 5.990 | 50.919 | 1.00 | 0.002 |
| dlss_super_resolution | 2.055 | 2.055 | 2.078 | 1.00 | 0.195 |
| early_mesh_preprocessing | 1.742 | 1.742 | 2.034 | 1.00 | 0.003 |
| sun_visibility | 0.704 | 0.704 | 0.716 | 1.00 | 0.006 |
| bin_unpacking | 0.693 | 0.693 | 0.778 | 1.00 | 0.015 |
| gpu_instance_materialization | 0.591 | 0.591 | 0.590 | 1.00 | 0.009 |
| atmosphere_luts | 0.356 | 0.356 | 0.362 | 1.00 | 0.003 |
| uniform_allocation | 0.352 | 0.352 | 0.407 | 1.00 | 0.023 |
| water_velocity | 0.332 | 0.332 | 0.335 | 1.00 | 0.001 |
| bloom | 0.320 | 0.320 | 0.338 | 1.00 | 0.058 |
| atmos_froxel_inject | 0.304 | 0.304 | 0.308 | 1.00 | 0.001 |
| clustering | 0.285 | 0.285 | 0.315 | 1.00 | 0.019 |
| water_sspr | 0.200 | 0.200 | 0.201 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.195 | 1.00 | 0.007 |
| water_fft | 0.136 | 0.136 | 0.140 | 1.00 | 0.002 |
| upscaling | 0.122 | 0.122 | 0.126 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.112 | 1.00 | 0.002 |
| tonemapping | 0.085 | 0.085 | 0.091 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.082 | 0.082 | 0.092 | 1.00 | 0.007 |
| render_sky | 0.064 | 0.064 | 0.066 | 1.00 | 0.001 |
| clouds_trace | 0.060 | 0.060 | 0.070 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.409 | 0.411 | 0.12 | 0.001 |
| clouds_resolve | 0.047 | 0.047 | 0.049 | 1.00 | 0.001 |
| point_shadow_atlas | 0.037 | 0.073 | 0.080 | 0.50 | 0.147 |
| per_view_shadow_pass | 0.032 | 0.032 | 0.062 | 1.00 | 0.006 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.027 | 0.027 | 0.044 | 1.00 | 0.224 |
| lightprobe_radiance_map | 0.027 | 0.217 | 0.226 | 0.12 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.001 |
| clouds_shadow | 0.023 | 0.023 | 0.025 | 1.00 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| lightprobe_copy | 0.009 | 0.070 | 0.071 | 0.12 | 0.005 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.017 | 0.018 | 0.50 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.006 | 1.00 | 0.003 |
| night_sky | 0.005 | 0.005 | 0.008 | 1.00 | 0.001 |
| lightprobe_downsampling_first_pass | 0.004 | 0.032 | 0.033 | 0.12 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.01 | 0.009 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 38.529 | 84.178 | 93.80 | 14525.5 |
| stage/render/prepare_views | 28.343 | 74.222 | 85.04 | 10685.2 |
| render/acquire/ms | 28.274 | 74.155 | 84.94 | 10659.3 |
| render/busy/ms | 9.853 | 11.432 | 12.88 | 3714.7 |
| main/busy/ms | 7.918 | 9.209 | 10.18 | 2984.9 |
| encode//sum | 6.089 | 9.118 | 12.00 | 2295.4 |
| stage/render/render | 6.039 | 7.508 | 8.72 | 2276.7 |
| render/graph/ms | 5.360 | 6.794 | 8.03 | 2020.8 |
| stage/render/graph_submit | 3.975 | 5.302 | 6.71 | 1498.6 |
| stage/main/post_update | 3.749 | 4.172 | 6.06 | 1413.3 |
| encode/submit | 2.357 | 3.099 | 4.90 | 888.8 |
| encode/finish_wall | 1.632 | 2.236 | 2.95 | 615.3 |
| stage/main/update | 1.591 | 1.814 | 2.38 | 599.7 |
| stage/render/camera_driver | 1.289 | 1.489 | 1.71 | 486.1 |
| stage/main/fixed_main_loop | 1.268 | 2.002 | 2.65 | 478.2 |
| encode/pass:early prepass/sum | 1.109 | 1.548 | 1.83 | 418.0 |
| encode/pass:main_opaque_pass_3d/sum | 1.080 | 1.523 | 1.99 | 407.2 |
| encode/pass:shadow_directional_light/sum | 1.042 | 1.730 | 2.44 | 392.8 |
| stage/render/prepare_assets | 0.785 | 1.032 | 2.14 | 295.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.587 | 0.909 | 1.24 | 221.3 |
| stage/render/prepare_write_phase_buffers | 0.524 | 0.742 | 0.93 | 197.5 |
| stage/render/prepare_resources | 0.521 | 0.595 | 0.82 | 196.5 |
| stage/render/prepare_meshes | 0.519 | 0.648 | 0.84 | 195.7 |
| stage/main/first | 0.469 | 0.630 | 1.21 | 177.0 |
| bigspace/total/ms | 0.434 | 0.521 | 0.70 | 163.5 |
| render/present/ms | 0.402 | 0.490 | 0.69 | 151.5 |
| stage/main/pre_update | 0.399 | 0.491 | 0.84 | 150.3 |
| stage/render/prepare_batch_phases | 0.394 | 0.529 | 1.06 | 148.5 |
| stage/render/prepare_flush | 0.343 | 0.531 | 1.03 | 129.4 |
| controller/feed/ms | 0.301 | 0.372 | 0.47 | 113.5 |
| stage/render/prepare_bind_groups | 0.274 | 0.441 | 0.81 | 103.4 |
| stage/main/last | 0.262 | 0.311 | 1.00 | 98.6 |
| stage/render/queue | 0.241 | 0.292 | 0.36 | 91.0 |
| stage/render/extract_commands | 0.211 | 0.244 | 0.32 | 79.7 |
| controller/move/ms | 0.193 | 0.296 | 0.45 | 72.8 |
| bigspace/low_precision/ms | 0.188 | 0.240 | 0.27 | 70.7 |
| stage/render/specialize | 0.186 | 0.230 | 0.34 | 70.3 |
| stage/render/create_views | 0.186 | 0.216 | 0.27 | 70.1 |
| anim/evaluate_cpu/ms | 0.178 | 0.205 | 0.36 | 67.0 |
| stream/drive/ms | 0.175 | 0.204 | 0.27 | 66.0 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.169 | 0.462 | 1.24 | 63.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.151 | 0.244 | 0.96 | 57.0 |
| stream/tick/ms | 0.146 | 0.172 | 0.24 | 55.0 |
| anim/evaluate/ms | 0.138 | 0.167 | 0.33 | 52.1 |
| physics/step/ms | 0.096 | 0.123 | 0.14 | 36.2 |
| bigspace/high_precision/ms | 0.093 | 0.112 | 0.20 | 35.1 |
| encode/resolve_diagnostics/sum | 0.071 | 0.118 | 0.21 | 26.7 |
| controller/write/ms | 0.061 | 0.075 | 0.10 | 23.1 |
| stage/render/phase_sort | 0.043 | 0.065 | 0.11 | 16.2 |
| stage/render/prepare_collect_phase_buffers | 0.041 | 0.061 | 0.11 | 15.3 |
| terrain/update/ms | 0.035 | 0.046 | 0.09 | 13.1 |
| stage/render/cleanup | 0.031 | 0.045 | 0.28 | 11.6 |
| controller/avoid/ms | 0.030 | 0.049 | 0.07 | 11.2 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.044 | 0.12 | 10.0 |
| encode/camera_driver_span_end/sum | 0.023 | 0.041 | 0.12 | 8.8 |
| weather/map/ms | 0.015 | 0.020 | 0.03 | 5.7 |
| anim/apply/ms | 0.011 | 0.014 | 0.03 | 4.2 |
| weather/eval/ms | 0.009 | 0.011 | 0.03 | 3.3 |
| script/update/ms | 0.008 | 0.009 | 0.03 | 2.9 |
| script/input/ms | 0.005 | 0.007 | 0.01 | 2.0 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 1.6 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 1.2 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.02 | 0.8 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.00 | 0.5 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 0.5 |
| stream/promote/ms | 0.001 | 0.002 | 0.00 | 0.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 0.4 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.1 |
| stage/render/prepare | 0.000 | 0.002 | 0.01 | 0.1 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a2/count | count | 4 | 4 | 4 |
| anim/a3/count | count | 446 | 446 | 446 |
| anim/characters/count | count | 450 | 450 | 450 |
| anim/evaluated/count | count | 26 | 26 | 26 |
| controller/movers/count | count | 28 | 28 | 28 |
| crowd/c0/count | count | 30 | 30 | 30 |
| crowd/c0/joints/count | count | 1249 | 1249 | 1249 |
| crowd/c0/meshes/count | count | 398 | 398 | 398 |
| crowd/c1/count | count | 116 | 116 | 116 |
| crowd/c1/joints/count | count | 5172 | 5172 | 5172 |
| crowd/c1/meshes/count | count | 1293 | 1293 | 1293 |
| crowd/c2/count | count | 304 | 304 | 304 |
| crowd/c2/joints/count | count | 5254 | 5254 | 5254 |
| crowd/c2/meshes/count | count | 1802 | 1802 | 1802 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 210 | 210 | 210 |
| crowd/joints_extracted/count | count | 754 | 1156 | 965 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.70 | 2.43 | 0.76 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.12 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.12 | 0.02 |
| encode/finish_critical | ms | 1.29 | 2.44 | 1.23 |
| encode/pass:early prepass/max | ms | 1.10 | 1.83 | 1.23 |
| encode/pass:main_opaque_pass_3d/max | ms | 1.08 | 1.99 | 0.73 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.14 | 0.96 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.55 | 1.24 | 0.56 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.30 | 1.24 | 0.26 |
| encode/pass:shadow_directional_light/max | ms | 0.98 | 2.44 | 0.80 |
| encode/resolve_diagnostics/max | ms | 0.07 | 0.21 | 0.05 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 31.71 | 90.32 | 31.37 |
| gpu/timed/ms | ms | 31.71 | 90.32 | 31.37 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 809 | 809 | 809 |
| groundcover/slots/capacity | count | 658.0 k | 658.0 k | 658.0 k |
| groundcover/slots/capacity_bytes | bytes | 42.11 M | 42.11 M | 42.11 M |
| groundcover/slots/live | count | 0 | 0 | 0 |
| groundcover/tiles/capped | count | 0 | 0 | 0 |
| groundcover/tiles/casting | count | 5 | 5 | 5 |
| groundcover/tiles/resident | count | 63 | 63 | 63 |
| groundcover/tiles/scattered | count | 63 | 63 | 63 |
| groundcover/tiles/unplaced | count | 0 | 0 | 0 |
| groundcover/upload/bytes | bytes | 0 | 0 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2433 | 2433 | 2433 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 303.9 k | 303.9 k | 303.9 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.6 k | 12.6 k | 12.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 343.66 M | 343.66 M | 343.66 M |
| phases/alpha_mask3d/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d/bins | count | 364 | 364 | 364 |
| phases/alpha_mask3d/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d/instance_draws | count | 569 | 569 | 569 |
| phases/alpha_mask3d/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d/items | count | 569 | 569 | 569 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d_prepass/bins | count | 364 | 364 | 364 |
| phases/alpha_mask3d_prepass/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d_prepass/instance_draws | count | 569 | 569 | 569 |
| phases/alpha_mask3d_prepass/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d_prepass/items | count | 569 | 569 | 569 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 32 | 32 | 32 |
| phases/opaque3d/bins | count | 1370 | 1370 | 1370 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2851 | 2851 | 2851 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 23 | 23 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 29 | 29 | 29 |
| phases/opaque3d_prepass/bins | count | 1370 | 1370 | 1370 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2851 | 2851 | 2851 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 133 | 14 |
| phases/shadow/bins | count | 2207 | 2207 | 2195 |
| phases/shadow/instance_batches | count | 9 | 27 | 9 |
| phases/shadow/instance_draws | count | 7317 | 22.0 k | 7317 |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| phases/shadow/items | count | 5386 | 5386 | 5374 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 55 | 56 | 54 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 358 | 358 | 358 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 5 | 68 | 5 |
| preprocess/bind_groups_created | count | 0 | 61 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 141 | 141 | 141 |
| process/cpu_usage | % | 13.84 | 24.22 | 13.39 |
| process/mem_usage | GiB | 3.64 | 3.68 | 3.65 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 1720 | 1720 | 1720 |
| scene/batch/masked/live | count | 1720 | 1720 | 1720 |
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
| scene/batch/opaque/high_water | count | 19.7 k | 19.7 k | 19.7 k |
| scene/batch/opaque/live | count | 19.7 k | 19.7 k | 19.7 k |
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
| scene/cells/resident | count | 118 | 118 | 118 |
| scene/chains/high_water | count | 1663 | 1663 | 1663 |
| scene/chains/live | count | 1663 | 1663 | 1663 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/scene_bump | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/scene_live | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 4 | 4 | 4 |
| shadow_atlas/live_occ_pct | % | 0.68 | 0.68 | 0.59 |
| shadow_atlas/live_tiles | count | 7 | 7 | 6 |
| shadow_atlas/mover_draws | count | 2 | 4 | 4 |
| shadow_atlas/movers | count | 100 | 100 | 100 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 4 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 0 | 4 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 7 | 7 | 6 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 7 | 7 | 6 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 19 | 26 | 22 |
| skin/blocks_skipped/count | count | 191 | 200 | 187 |
| skin/blocks_walked/count | count | 19 | 26 | 23 |
| skin/joint_bytes_written/bytes | bytes | 59.6 k | 78.3 k | 68.9 k |
| skin/joints_written/count | count | 932 | 1224 | 1076 |
| skin/upload_writes/count | count | 8 | 12 | 8 |
| started_load_count |  | 5838 | 5840 | 5840 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 247 | 247 | 247 |
| statics/materials_bound | count | 247 | 247 | 247 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.35 | 0.35 | 0.35 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 221 | 221 | 221 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 17.40 M | 17.40 M | 17.40 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 899 | 899 | 899 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 19.03 | 29.36 | 17.54 |
| system/mem_usage | % | 58.65 | 58.72 | 58.65 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 113 | 232 | 113 |
| views/all/bins | count | 5675 | 5675 | 5663 |
| views/all/count | count | 11 | 11 | 10 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 22.0 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 114 | 115 | 113 |
| views/all/pipelines_distinct | count | 80 | 80 | 80 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.50 M |
| views/cascade/batch_sets | count | 14 | 60 | 14 |
| views/cascade/bins | count | 2043 | 2043 | 2043 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 9 |
| views/cascade/instance_draws | count | 7317 | 22.0 k | 7317 |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| views/cascade/items | count | 5222 | 5222 | 5222 |
| views/cascade/pipelines | count | 43 | 43 | 43 |
| views/cascade/work_items | count | 83.0 k | 253.8 k | 83.0 k |
| views/lamp_face/batch_sets | count | 0 | 91 | 0 |
| views/lamp_face/bins | count | 164 | 167 | 152 |
| views/lamp_face/count | count | 7 | 7 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 164 | 167 | 152 |
| views/lamp_face/pipelines | count | 12 | 13 | 11 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 91 | 0 |
| views/main/batch_sets | count | 99 | 99 | 99 |
| views/main/bins | count | 3468 | 3468 | 3468 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 14.7 k | 14.7 k | 14.7 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6840 | 6840 | 6840 |
| views/main/pipelines | count | 59 | 59 | 59 |
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
| world/meshes | count | 18.7 k | 18.7 k | 18.7 k |
| world/point_lights | count | 355 | 355 | 355 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 51.08 | 86 | 19.3 k |
| anim/events/count | count | 0.20 | 3 | 74 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 1 | 3 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 8 | 8 | 3016 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| encode//count | count | 28.25 | 30 | 10.7 k |
| encode/bind_walk/init_textures | count | 9347 | 13.1 k | 3.52 M |
| encode/bind_walk/init_textures_skipped | count | 60.4 k | 71.3 k | 22.76 M |
| encode/bind_walk/inits | count | 806 | 918 | 303.8 k |
| encode/bind_walk/inits_skipped | count | 172 | 189 | 64.9 k |
| encode/bind_walk/merge_textures | count | 9285 | 13.0 k | 3.50 M |
| encode/bind_walk/merge_textures_skipped | count | 19.1 k | 30.0 k | 7.18 M |
| encode/bind_walk/merges | count | 159 | 183 | 59.8 k |
| encode/bind_walk/merges_skipped | count | 159 | 184 | 60.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 377 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 377 |
| encode/pass:early prepass/count | count | 1 | 1 | 377 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 377 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 377 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 377 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 188 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 377 |
| encode/queue/command_buffers | count | 38.75 | 41 | 14.6 k |
| encode/queue/submits | count | 2 | 2 | 754 |
| encode/queue/write_buffer/bytes | bytes | 2.96 M | 3.50 M | 1.12 G |
| encode/queue/write_buffer/calls | count | 286 | 360 | 107.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 69.0 k | 69.0 k | 26.01 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 3393 |
| encode/queue/write_texture/bytes | bytes | 46.1 k | 8.39 M | 17.37 M |
| encode/queue/write_texture/calls | count | 0.01 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 377 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 2.23 | 3 | 842 |
| physics/steps_dropped | count | 0.15 | 3 | 58 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 91.8 ms; slowest 44, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2576.4 | main/busy/ms 2543, stage/main/post_update 2490, stage/main/update 35.42, stage/main/pre_update 14.08, stage/main/last 1.18, surface/advance_cpu_ms 0.92, stream/drive/ms 0.56, stage/main/first 0.55, stage/main/fixed_main_loop 0.41, surface/cell_upload_ms 0.30, stream/tick/ms 0.14, bigspace/total/ms 0.10 |
| warm-up | 3.35 | 776.6 | main/busy/ms 2589, stage/main/post_update 2522, render/thread/ms 716, render/busy/ms 558, stage/render/prepare_assets 522, stage/render/prepare_views 154, render/acquire/ms 154, stage/main/update 42.36, stage/render/render 22.88, render/graph/ms 17.79, stage/main/pre_update 16.23, stage/render/camera_driver 15.63 |
| warm-up | 3.50 | 147.9 | render/thread/ms 861, render/busy/ms 636, stage/render/prepare_assets 533, stage/render/prepare_views 154, render/acquire/ms 154, stage/render/render 145, pipelines/compile_ms 129, pipelines/front_ms 118, main/busy/ms 101, render/graph/ms 71.74, render/present/ms 70.95, pipelines/wait_ms 67.15 |
| warm-up | 3.61 | 107.7 | pipelines/compile_ms 1196, pipelines/front_ms 838, pipelines/driver_ms 358, pipelines/wait_ms 267, render/thread/ms 252, render/busy/ms 183, stage/render/render 132, main/busy/ms 94.86, pipelines/lock_ms 89.44, render/present/ms 68.53, stage/main/post_update 61.89, render/graph/ms 61.76 |
| warm-up | 3.71 | 99.9 | pipelines/compile_ms 2520, pipelines/front_ms 2081, pipelines/wait_ms 1655, pipelines/driver_ms 439, render/thread/ms 194, render/busy/ms 191, main/busy/ms 128, stage/render/prepare_assets 81.96, stage/main/update 76.42, stage/render/prepare_write_phase_buffers 76.34, pipelines/lock_ms 66.46, stage/main/post_update 40.67 |
| warm-up | 3.83 | 122.6 | pipelines/wait_ms 7413, pipelines/compile_ms 2902, pipelines/front_ms 2725, render/thread/ms 204, render/busy/ms 201, pipelines/driver_ms 178, main/busy/ms 157, stage/render/prepare_assets 147, stage/main/update 74.22, stage/main/pre_update 49.50, stage/main/post_update 26.16, stage/render/prepare_write_phase_buffers 24.66 |
| warm-up | 4.58 | 133.4 | pipelines/compile_ms 1411, pipelines/front_ms 1359, main/busy/ms 189, render/thread/ms 167, render/busy/ms 163, stage/main/update 138, pipelines/wait_ms 115, stage/render/prepare_write_phase_buffers 57.06, pipelines/lock_ms 56.00, pipelines/driver_ms 51.21, stage/render/render 48.62, render/graph/ms 43.02 |
| warm-up | 4.76 | 187.2 | pipelines/compile_ms 2537, pipelines/front_ms 2456, render/thread/ms 298, render/busy/ms 295, main/busy/ms 173, pipelines/wait_ms 162, stage/main/update 147, stage/render/prepare_assets 123, pipelines/driver_ms 80.98, stage/render/prepare_write_phase_buffers 56.17, pipelines/lock_ms 55.53, stage/render/render 48.99 |
| warm-up | 7.63 | 100.8 | pipelines/compile_ms 149, pipelines/front_ms 145, render/thread/ms 111, stage/render/prepare_views 90.12, render/acquire/ms 89.96, render/busy/ms 20.52, main/busy/ms 20.49, stage/render/render 11.89, stage/main/post_update 11.18, render/graph/ms 10.54, encode//sum 8.11, stage/render/graph_submit 7.78 |
| warm-up | 7.74 | 105.5 | render/thread/ms 204, stage/render/prepare_views 153, render/acquire/ms 153, render/busy/ms 50.50, stage/render/prepare_resources 29.69, pipelines/compile_ms 21.21, main/busy/ms 20.60, pipelines/front_ms 20.36, stage/render/render 11.65, render/graph/ms 10.16, stage/main/post_update 8.93, encode//sum 8.67 |
| warm-up | 14.29 | 104.8 | render/thread/ms 182, stage/render/prepare_views 159, render/acquire/ms 159, render/busy/ms 21.98, main/busy/ms 21.57, pipelines/compile_ms 19.79, pipelines/front_ms 19.11, encode//sum 12.75, stage/render/render 12.35, render/graph/ms 10.93, stage/main/post_update 8.78, stage/render/graph_submit 8.07 |
| warm-up | 14.39 | 97.2 | render/thread/ms 199, stage/render/prepare_views 175, render/acquire/ms 175, pipelines/compile_ms 90.65, pipelines/front_ms 87.57, render/busy/ms 23.63, main/busy/ms 20.86, stage/render/render 13.20, encode//sum 12.72, render/graph/ms 11.69, stage/render/graph_submit 8.74, stage/main/post_update 8.19 |
| warm-up | 14.67 | 96.7 | render/thread/ms 184, stage/render/prepare_views 159, render/acquire/ms 158, render/busy/ms 24.83, main/busy/ms 23.69, pipelines/compile_ms 17.13, pipelines/front_ms 16.22, stage/render/render 12.97, encode//sum 12.10, render/graph/ms 11.42, stage/main/post_update 8.84, stage/render/graph_submit 8.62 |
| warm-up | 14.76 | 96.1 | render/thread/ms 190, stage/render/prepare_views 165, render/acquire/ms 165, render/busy/ms 24.13, main/busy/ms 21.76, stage/render/render 13.05, encode//sum 11.75, render/graph/ms 11.48, pipelines/compile_ms 11.08, pipelines/front_ms 10.47, stage/render/graph_submit 8.55, stage/main/post_update 7.94 |
| warm-up | 14.94 | 100.8 | render/thread/ms 174, stage/render/prepare_views 147, render/acquire/ms 147, pipelines/compile_ms 97.03, pipelines/front_ms 94.82, render/busy/ms 25.67, main/busy/ms 24.17, stage/render/render 12.02, encode//sum 11.85, render/graph/ms 10.77, stage/render/graph_submit 8.12, stage/main/post_update 7.71 |
| warm-up | 15.61 | 98.6 | render/thread/ms 137, stage/render/prepare_views 116, render/acquire/ms 115, pipelines/compile_ms 104, pipelines/front_ms 102, render/busy/ms 20.63, main/busy/ms 17.44, encode//sum 11.88, stage/render/render 11.83, render/graph/ms 10.57, stage/render/graph_submit 7.70, stage/main/post_update 7.67 |
| warm-up | 15.88 | 92.6 | render/thread/ms 181, stage/render/prepare_views 156, render/acquire/ms 156, pipelines/compile_ms 100, pipelines/front_ms 96.72, render/busy/ms 24.07, main/busy/ms 22.20, encode//sum 13.48, stage/render/render 12.53, render/graph/ms 11.23, stage/render/graph_submit 8.44, stage/main/post_update 7.78 |
| warm-up | 15.98 | 98.6 | pipelines/compile_ms 201, pipelines/front_ms 194, render/thread/ms 189, stage/render/prepare_views 164, render/acquire/ms 164, render/busy/ms 24.14, main/busy/ms 21.95, encode//sum 13.37, stage/render/render 13.23, render/graph/ms 11.84, stage/render/graph_submit 8.74, stage/main/post_update 8.30 |
| warm-up | 16.17 | 97.4 | render/thread/ms 186, stage/render/prepare_views 162, render/acquire/ms 162, render/busy/ms 23.69, main/busy/ms 21.56, stage/render/render 13.13, encode//sum 13.13, render/graph/ms 11.51, stage/render/graph_submit 8.67, stage/main/post_update 8.08, encode/submit 5.32, stage/main/fixed_main_loop 5.16 |
| warm-up | 16.27 | 101.7 | render/thread/ms 196, stage/render/prepare_views 173, render/acquire/ms 173, main/busy/ms 22.88, render/busy/ms 22.85, stage/render/render 12.00, encode//sum 11.53, render/graph/ms 10.59, stage/main/post_update 8.56, stage/render/graph_submit 7.99, encode/submit 5.19, stage/main/fixed_main_loop 4.95 |
| warm-up | 16.64 | 98.1 | render/thread/ms 184, stage/render/prepare_views 161, render/acquire/ms 161, render/busy/ms 22.03, main/busy/ms 20.76, stage/render/render 12.67, encode//sum 12.53, render/graph/ms 11.28, stage/render/graph_submit 8.67, stage/main/post_update 7.93, encode/submit 5.27, stage/main/fixed_main_loop 4.83 |
| warm-up | 16.74 | 100.4 | render/thread/ms 196, stage/render/prepare_views 173, render/acquire/ms 173, render/busy/ms 21.81, main/busy/ms 20.69, stage/render/render 12.70, encode//sum 12.33, render/graph/ms 11.24, stage/render/graph_submit 8.42, stage/main/post_update 7.84, stage/main/fixed_main_loop 5.27, encode/submit 5.21 |
| warm-up | 16.84 | 98.7 | render/thread/ms 196, stage/render/prepare_views 173, render/acquire/ms 173, render/busy/ms 22.30, main/busy/ms 21.27, stage/render/render 12.37, encode//sum 11.75, render/graph/ms 10.86, stage/main/post_update 8.38, stage/render/graph_submit 8.05, stage/main/fixed_main_loop 5.17, encode/submit 4.70 |
| warm-up | 17.03 | 104.9 | render/thread/ms 189, stage/render/prepare_views 166, render/acquire/ms 166, render/busy/ms 22.65, main/busy/ms 21.88, stage/render/render 11.99, encode//sum 11.97, render/graph/ms 10.71, stage/render/graph_submit 8.14, stage/main/post_update 7.70, stage/main/fixed_main_loop 5.21, encode/submit 5.00 |
| warm-up | 17.22 | 96.6 | render/thread/ms 184, stage/render/prepare_views 153, render/acquire/ms 152, render/busy/ms 30.84, main/busy/ms 30.44, stage/render/render 13.89, render/graph/ms 12.52, encode//sum 12.35, stage/main/post_update 11.35, stage/render/prepare_assets 9.85, stage/render/graph_submit 9.80, encode/submit 5.29 |
| warm-up | 17.31 | 94.5 | render/thread/ms 187, stage/render/prepare_views 157, render/acquire/ms 157, render/busy/ms 29.37, main/busy/ms 26.48, stage/render/render 13.05, encode//sum 12.99, render/graph/ms 11.69, stage/render/prepare_assets 9.21, stage/render/graph_submit 8.96, stage/main/post_update 8.28, encode/submit 5.18 |
| warm-up | 17.41 | 93.6 | render/thread/ms 185, stage/render/prepare_views 164, render/acquire/ms 164, main/busy/ms 21.38, render/busy/ms 20.47, encode//sum 12.12, stage/render/render 11.02, render/graph/ms 9.79, stage/main/post_update 8.27, stage/render/graph_submit 7.36, stage/main/fixed_main_loop 4.75, encode/submit 4.42 |
| warm-up | 17.60 | 105.4 | render/thread/ms 194, stage/render/prepare_views 170, render/acquire/ms 170, render/busy/ms 23.19, main/busy/ms 19.63, encode//sum 14.26, stage/render/render 13.73, render/graph/ms 12.33, stage/render/graph_submit 9.26, stage/main/post_update 7.50, encode/submit 5.31, stage/main/fixed_main_loop 4.85 |
| warm-up | 17.79 | 93.6 | render/thread/ms 181, stage/render/prepare_views 158, render/acquire/ms 158, render/busy/ms 21.78, main/busy/ms 20.66, encode//sum 12.23, stage/render/render 12.11, render/graph/ms 10.71, stage/main/post_update 7.86, stage/render/graph_submit 7.76, stage/main/fixed_main_loop 5.17, encode/submit 4.82 |
| warm-up | 17.88 | 91.9 | render/thread/ms 183, stage/render/prepare_views 159, render/acquire/ms 159, render/busy/ms 22.55, main/busy/ms 20.35, encode//sum 14.15, stage/render/render 13.11, render/graph/ms 11.65, stage/render/graph_submit 8.55, stage/main/post_update 7.81, encode/submit 5.36, stage/main/fixed_main_loop 4.66 |
| warm-up | 17.98 | 99.2 | render/thread/ms 188, stage/render/prepare_views 164, render/acquire/ms 164, render/busy/ms 23.02, main/busy/ms 21.61, encode//sum 14.08, stage/render/render 13.33, render/graph/ms 11.96, stage/render/graph_submit 8.82, stage/main/post_update 8.68, stage/main/fixed_main_loop 4.98, encode/submit 4.96 |
| warm-up | 18.17 | 100.9 | render/thread/ms 189, stage/render/prepare_views 166, render/acquire/ms 166, main/busy/ms 23.17, render/busy/ms 21.59, encode//sum 11.59, stage/render/render 11.31, render/graph/ms 9.94, stage/main/post_update 8.97, stage/render/graph_submit 7.51, stage/main/fixed_main_loop 5.91, encode/submit 4.78 |
| warm-up | 18.26 | 97.1 | render/thread/ms 195, stage/render/prepare_views 172, render/acquire/ms 172, render/busy/ms 22.35, main/busy/ms 22.10, encode//sum 12.63, stage/render/render 12.09, render/graph/ms 10.79, stage/render/graph_submit 8.15, stage/main/post_update 7.85, stage/main/fixed_main_loop 5.84, encode/submit 5.19 |
| warm-up | 18.45 | 94.4 | render/thread/ms 183, stage/render/prepare_views 159, render/acquire/ms 158, render/busy/ms 23.24, main/busy/ms 20.85, stage/render/render 12.39, encode//sum 12.21, render/graph/ms 10.87, stage/render/graph_submit 8.03, stage/main/post_update 7.44, stage/main/fixed_main_loop 5.08, encode/submit 4.75 |
| warm-up | 18.54 | 94.8 | render/thread/ms 186, stage/render/prepare_views 164, render/acquire/ms 164, main/busy/ms 21.71, render/busy/ms 21.22, stage/render/render 11.69, encode//sum 11.49, render/graph/ms 10.29, stage/main/post_update 7.80, stage/render/graph_submit 7.63, stage/main/update 5.00, stage/main/fixed_main_loop 4.91 |
| warm-up | 18.91 | 93.3 | render/thread/ms 180, stage/render/prepare_views 157, render/acquire/ms 157, main/busy/ms 23.66, render/busy/ms 22.17, stage/render/render 10.83, encode//sum 10.28, render/graph/ms 9.02, stage/main/post_update 8.48, stage/render/graph_submit 6.50, stage/render/prepare_assets 5.56, stage/main/fixed_main_loop 4.76 |
| warm-up | 19.27 | 93.2 | render/thread/ms 176, stage/render/prepare_views 154, render/acquire/ms 154, main/busy/ms 22.91, render/busy/ms 21.01, encode//sum 10.58, stage/render/render 10.32, render/graph/ms 9.05, stage/main/post_update 8.51, stage/render/graph_submit 6.60, stage/render/prepare_assets 4.94, stage/main/fixed_main_loop 4.54 |
| measure | 10.90 | 94.5 | render/thread/ms 178, stage/render/prepare_views 159, render/acquire/ms 159, main/busy/ms 18.51, render/busy/ms 17.93, stage/render/render 9.83, render/graph/ms 8.51, stage/main/post_update 7.64, encode//sum 7.48, stage/render/graph_submit 5.96, stage/main/fixed_main_loop 4.14, stage/main/update 3.82 |
| measure | 10.99 | 93.2 | render/thread/ms 185, stage/render/prepare_views 166, render/acquire/ms 166, main/busy/ms 18.56, render/busy/ms 18.15, stage/render/render 9.77, render/graph/ms 8.49, stage/main/post_update 7.53, encode//sum 6.97, stage/render/graph_submit 6.14, stage/main/fixed_main_loop 4.32, encode/submit 3.45 |
| measure | 11.42 | 95.0 | render/thread/ms 178, stage/render/prepare_views 160, render/acquire/ms 160, main/busy/ms 19.14, render/busy/ms 16.93, stage/render/render 8.74, render/graph/ms 7.65, stage/main/post_update 7.60, encode//sum 6.80, stage/render/graph_submit 5.43, stage/main/update 4.21, stage/main/fixed_main_loop 3.99 |
| measure | 11.52 | 93.7 | render/thread/ms 186, stage/render/prepare_views 169, render/acquire/ms 169, main/busy/ms 17.94, render/busy/ms 16.72, stage/render/render 9.27, render/graph/ms 8.12, encode//sum 7.12, stage/main/post_update 7.06, stage/render/graph_submit 5.79, stage/main/update 4.19, stage/main/fixed_main_loop 3.83 |
| measure | 11.87 | 92.5 | render/thread/ms 179, stage/render/prepare_views 160, render/acquire/ms 160, render/busy/ms 18.11, main/busy/ms 17.78, stage/render/render 9.87, render/graph/ms 8.66, encode//sum 7.72, stage/main/post_update 7.45, stage/render/graph_submit 6.24, stage/main/fixed_main_loop 4.16, encode/submit 3.84 |
| measure | 12.22 | 93.4 | render/thread/ms 178, stage/render/prepare_views 158, render/acquire/ms 158, render/busy/ms 19.32, main/busy/ms 18.34, stage/render/render 9.99, render/graph/ms 8.77, encode//sum 7.91, stage/main/post_update 7.49, stage/render/graph_submit 6.25, stage/main/fixed_main_loop 4.14, encode/submit 3.92 |
| measure | 12.49 | 92.3 | render/thread/ms 177, stage/render/prepare_views 158, render/acquire/ms 158, main/busy/ms 18.69, render/busy/ms 18.13, encode//sum 10.09, stage/render/render 10.03, render/graph/ms 8.76, stage/main/post_update 7.86, stage/render/graph_submit 6.44, stage/main/fixed_main_loop 4.37, encode/submit 3.90 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --set groundcover.density_scale=0 --resolution 2560x1440 --bench-out target/perf\gc_p0\gc_ascadian@2560x1440_arm=empty-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **99%** (min 99%) · 2640 MHz (min 2640) · memory 8101 MHz · 44.4 W · 63 °C (max 66) · VRAM 7621 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `groundcover.density_scale=0 (command line: --set)`
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
