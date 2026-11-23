# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.render.reserve_worst_case=on (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 348 frames over 15.0 s · **median 38.61 ms (26 fps)** · mean 43.22 · p95 90.35 · p99 98.53 (1% low 10 fps) · worst 100.55 ms · 25 spikes over 1.5× median · 99.7 stutters/min (work 99.7/min)
- warm-up: 308 frames over 20.8 s, worst 2497.9 ms
- hitches (> 96.5 ms): **5** measured, 22 in warm-up
- timed GPU passes: 37.02 ms of a 43.22 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 38.61 ms, limit 11 ms
- **FAIL** worst frame 100.55 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 10.858 | 10.858 | 11.022 | 1.00 | 0.084 |
| early prepass | 10.030 | 10.030 | 10.156 | 1.00 | 0.133 |
| range_unpacking | 6.497 | 6.497 | 54.590 | 1.00 | 0.002 |
| early_mesh_preprocessing | 2.227 | 2.227 | 2.718 | 1.00 | 0.003 |
| dlss_super_resolution | 2.065 | 2.065 | 2.089 | 1.00 | 0.198 |
| sun_visibility | 0.697 | 0.697 | 0.705 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.638 | 0.638 | 0.645 | 1.00 | 0.008 |
| bin_unpacking | 0.576 | 0.576 | 0.671 | 1.00 | 0.015 |
| uniform_allocation | 0.397 | 0.397 | 0.516 | 1.00 | 0.022 |
| atmosphere_luts | 0.356 | 0.356 | 0.362 | 1.00 | 0.003 |
| bloom | 0.325 | 0.325 | 0.342 | 1.00 | 0.058 |
| water_velocity | 0.323 | 0.323 | 0.332 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.305 | 0.305 | 0.308 | 1.00 | 0.001 |
| clustering | 0.286 | 0.286 | 0.317 | 1.00 | 0.019 |
| water_sspr | 0.200 | 0.200 | 0.201 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.198 | 1.00 | 0.007 |
| water_fft | 0.137 | 0.137 | 0.140 | 1.00 | 0.002 |
| upscaling | 0.134 | 0.134 | 0.137 | 1.00 | 0.004 |
| clouds_composite | 0.107 | 0.107 | 0.110 | 1.00 | 0.002 |
| tonemapping | 0.089 | 0.089 | 0.095 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.079 | 0.079 | 0.084 | 1.00 | 0.007 |
| render_sky | 0.063 | 0.063 | 0.064 | 1.00 | 0.002 |
| clouds_trace | 0.056 | 0.056 | 0.060 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.053 | 0.053 | 0.104 | 1.00 | 0.006 |
| lightprobe_irradiance_map | 0.051 | 0.407 | 0.409 | 0.13 | 0.001 |
| clouds_resolve | 0.046 | 0.046 | 0.047 | 1.00 | 0.001 |
| point_shadow_atlas | 0.036 | 0.073 | 0.080 | 0.50 | 0.140 |
| main_transparent_pass_3d | 0.030 | 0.030 | 0.032 | 1.00 | 0.224 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.218 | 0.224 | 0.13 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 1.00 | 0.001 |
| clouds_shadow | 0.022 | 0.022 | 0.024 | 1.00 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| lightprobe_copy | 0.009 | 0.070 | 0.071 | 0.13 | 0.004 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.016 | 0.018 | 0.50 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.006 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.007 | 1.00 | 0.002 |
| lightprobe_downsampling_first_pass | 0.004 | 0.032 | 0.034 | 0.13 | 0.001 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.01 | 0.007 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 41.935 | 89.218 | 99.51 | 14593.2 |
| stage/render/prepare_views | 32.070 | 79.837 | 90.47 | 11160.4 |
| render/acquire/ms | 32.002 | 79.767 | 90.40 | 11136.9 |
| render/busy/ms | 9.540 | 10.969 | 12.70 | 3320.0 |
| main/busy/ms | 8.007 | 9.571 | 11.39 | 2786.6 |
| stage/render/render | 5.800 | 6.988 | 8.64 | 2018.4 |
| encode//sum | 5.757 | 8.006 | 10.62 | 2003.4 |
| render/graph/ms | 5.134 | 6.314 | 7.88 | 1786.6 |
| stage/main/post_update | 3.779 | 4.234 | 6.22 | 1315.1 |
| stage/render/graph_submit | 3.760 | 4.881 | 6.49 | 1308.5 |
| encode/submit | 2.199 | 2.923 | 4.76 | 765.1 |
| stage/main/update | 1.583 | 1.805 | 2.22 | 550.8 |
| encode/finish_wall | 1.575 | 2.145 | 2.84 | 548.2 |
| stage/main/fixed_main_loop | 1.333 | 2.309 | 3.19 | 463.8 |
| stage/render/camera_driver | 1.278 | 1.487 | 1.66 | 444.6 |
| encode/pass:early prepass/sum | 1.093 | 1.494 | 1.94 | 380.4 |
| encode/pass:main_opaque_pass_3d/sum | 1.059 | 1.421 | 1.93 | 368.4 |
| encode/pass:shadow_directional_light/sum | 1.048 | 1.807 | 2.78 | 364.8 |
| stage/render/prepare_assets | 0.790 | 1.371 | 2.18 | 274.9 |
| stage/render/prepare_write_phase_buffers | 0.529 | 0.731 | 0.89 | 184.0 |
| stage/render/prepare_resources | 0.517 | 0.593 | 0.76 | 179.9 |
| stage/render/prepare_meshes | 0.506 | 0.645 | 0.80 | 176.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.504 | 0.741 | 0.96 | 175.5 |
| stage/main/first | 0.461 | 0.658 | 1.33 | 160.3 |
| bigspace/total/ms | 0.424 | 0.523 | 0.65 | 147.7 |
| stage/main/pre_update | 0.412 | 0.576 | 0.90 | 143.5 |
| render/present/ms | 0.392 | 0.497 | 1.23 | 136.3 |
| stage/render/prepare_batch_phases | 0.365 | 0.504 | 1.07 | 127.1 |
| stage/render/prepare_flush | 0.319 | 0.485 | 0.94 | 111.1 |
| controller/feed/ms | 0.313 | 0.387 | 0.50 | 108.9 |
| stage/render/prepare_bind_groups | 0.270 | 0.456 | 0.59 | 94.1 |
| stage/main/last | 0.262 | 0.300 | 3.06 | 91.3 |
| stage/render/queue | 0.240 | 0.286 | 0.38 | 83.4 |
| controller/move/ms | 0.233 | 0.522 | 0.70 | 81.2 |
| stage/render/extract_commands | 0.210 | 0.242 | 0.95 | 73.2 |
| stage/render/create_views | 0.188 | 0.236 | 0.66 | 65.3 |
| bigspace/low_precision/ms | 0.186 | 0.237 | 0.31 | 64.7 |
| anim/evaluate_cpu/ms | 0.178 | 0.213 | 1.04 | 62.0 |
| stage/render/specialize | 0.175 | 0.208 | 0.32 | 61.1 |
| stream/drive/ms | 0.170 | 0.198 | 0.33 | 59.0 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.163 | 0.428 | 0.97 | 56.8 |
| stream/tick/ms | 0.141 | 0.166 | 0.30 | 49.1 |
| anim/evaluate/ms | 0.140 | 0.176 | 1.00 | 48.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.129 | 0.189 | 0.29 | 45.0 |
| physics/step/ms | 0.101 | 0.134 | 0.17 | 35.3 |
| bigspace/high_precision/ms | 0.092 | 0.110 | 0.25 | 31.9 |
| encode/resolve_diagnostics/sum | 0.064 | 0.096 | 0.41 | 22.2 |
| controller/write/ms | 0.061 | 0.074 | 0.12 | 21.2 |
| stage/render/phase_sort | 0.041 | 0.062 | 0.13 | 14.4 |
| stage/render/prepare_collect_phase_buffers | 0.039 | 0.060 | 0.12 | 13.6 |
| terrain/update/ms | 0.035 | 0.044 | 0.15 | 12.1 |
| stage/render/cleanup | 0.031 | 0.046 | 0.34 | 10.9 |
| controller/avoid/ms | 0.031 | 0.054 | 0.14 | 10.7 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.044 | 0.14 | 8.6 |
| encode/camera_driver_span_end/sum | 0.023 | 0.040 | 0.13 | 7.9 |
| weather/map/ms | 0.016 | 0.020 | 0.08 | 5.5 |
| anim/apply/ms | 0.010 | 0.017 | 0.03 | 3.5 |
| weather/eval/ms | 0.009 | 0.012 | 0.03 | 3.1 |
| script/update/ms | 0.008 | 0.010 | 0.02 | 2.7 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 1.9 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 1.3 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 1.1 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.01 | 0.7 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.04 | 0.5 |
| character/finish/ms | 0.001 | 0.002 | 0.03 | 0.5 |
| stream/promote/ms | 0.001 | 0.002 | 0.00 | 0.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 0.3 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| stage/render/prepare | 0.000 | 0.002 | 0.00 | 0.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.1 |
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
| anim/a2/count | count | 3 | 3 | 3 |
| anim/a3/count | count | 447 | 447 | 447 |
| anim/characters/count | count | 450 | 450 | 450 |
| anim/evaluated/count | count | 26.50 | 28 | 28 |
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
| crowd/joints_extracted/count | count | 756 | 1167 | 748 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.71 | 2.75 | 0.71 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.14 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.13 | 0.03 |
| encode/finish_critical | ms | 1.25 | 2.78 | 0.99 |
| encode/pass:early prepass/max | ms | 1.09 | 1.94 | 0.83 |
| encode/pass:main_opaque_pass_3d/max | ms | 1.05 | 1.93 | 0.73 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.29 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.49 | 0.96 | 0.48 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.29 | 0.97 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.97 | 2.78 | 0.99 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.41 | 0.07 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 33.88 | 92.34 | 32.81 |
| gpu/timed/ms | ms | 33.88 | 92.34 | 32.81 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1380 | 1380 | 1380 |
| groundcover/slots/capacity | count | 658.0 k | 658.0 k | 658.0 k |
| groundcover/slots/capacity_bytes | bytes | 42.11 M | 42.11 M | 42.11 M |
| groundcover/slots/live | count | 42.2 k | 42.2 k | 42.2 k |
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
| phases/alpha_mask3d/bins | count | 364 | 365 | 364 |
| phases/alpha_mask3d/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d/instance_draws | count | 569 | 569 | 569 |
| phases/alpha_mask3d/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d/items | count | 569 | 570 | 569 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d_prepass/bins | count | 364 | 365 | 364 |
| phases/alpha_mask3d_prepass/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d_prepass/instance_draws | count | 569 | 569 | 569 |
| phases/alpha_mask3d_prepass/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d_prepass/items | count | 569 | 570 | 569 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 32 | 32 | 32 |
| phases/opaque3d/bins | count | 1370 | 1385 | 1370 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2851 | 2866 | 2851 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 24 | 24 | 24 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 29 | 29 | 29 |
| phases/opaque3d_prepass/bins | count | 1370 | 1385 | 1370 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2851 | 2866 | 2851 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 58.50 | 136 | 14 |
| phases/shadow/bins | count | 2192 | 2195 | 2193 |
| phases/shadow/instance_batches | count | 9 | 27 | 9 |
| phases/shadow/instance_draws | count | 7317 | 22.0 k | 7317 |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| phases/shadow/items | count | 5365 | 5375 | 5373 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 53 | 59 | 56 |
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
| preprocess/bind_groups | count | 9 | 123 | 5 |
| preprocess/bind_groups_created | count | 4 | 118 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 141 | 141 | 141 |
| process/cpu_usage | % | 14.17 | 20.42 | 14.17 |
| process/mem_usage | GiB | 3.83 | 3.85 | 3.85 |
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
| shadow_atlas/live_occ_pct | % | 0.59 | 0.68 | 0.68 |
| shadow_atlas/live_tiles | count | 6 | 7 | 7 |
| shadow_atlas/mover_draws | count | 1 | 4 | 4 |
| shadow_atlas/movers | count | 100 | 106 | 100 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 1 | 4 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 1 | 4 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 6 | 7 | 7 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 6 | 7 | 7 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 19 | 27 | 18 |
| skin/blocks_skipped/count | count | 191 | 200 | 192 |
| skin/blocks_walked/count | count | 19 | 27 | 18 |
| skin/joint_bytes_written/bytes | bytes | 59.5 k | 80.6 k | 54.0 k |
| skin/joints_written/count | count | 930 | 1260 | 844 |
| skin/upload_writes/count | count | 8 | 10 | 7 |
| started_load_count |  | 5839 | 5840 | 5840 |
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
| stream/latency/s | s | 0.16 | 0.16 | 0.16 |
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
| system/cpu_usage | % | 18.14 | 30.01 | 17.52 |
| system/mem_usage | % | 53.60 | 53.70 | 53.70 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 158 | 235 | 113 |
| views/all/bins | count | 5660 | 5693 | 5661 |
| views/all/count | count | 10 | 11 | 11 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 22.0 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 113 | 119 | 116 |
| views/all/pipelines_distinct | count | 81 | 86 | 81 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.50 M |
| views/cascade/batch_sets | count | 14 | 60 | 14 |
| views/cascade/bins | count | 2027 | 2043 | 2027 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 9 |
| views/cascade/instance_draws | count | 7317 | 22.0 k | 7317 |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| views/cascade/items | count | 5207 | 5223 | 5207 |
| views/cascade/pipelines | count | 43 | 43 | 43 |
| views/cascade/work_items | count | 83.0 k | 253.8 k | 83.0 k |
| views/lamp_face/batch_sets | count | 21.50 | 91 | 0 |
| views/lamp_face/bins | count | 149 | 167 | 166 |
| views/lamp_face/count | count | 6 | 7 | 7 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 149 | 170 | 166 |
| views/lamp_face/pipelines | count | 10 | 16 | 13 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 21.50 | 91 | 0 |
| views/main/batch_sets | count | 99 | 99 | 99 |
| views/main/bins | count | 3468 | 3500 | 3468 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 14.7 k | 14.7 k | 14.7 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6840 | 6872 | 6840 |
| views/main/pipelines | count | 60 | 60 | 60 |
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
| anim/bones_written/count | count | 39.74 | 118 | 13.8 k |
| anim/events/count | count | 0.18 | 2 | 61 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 8 | 8 | 2784 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| encode//count | count | 28.25 | 30 | 9830 |
| encode/bind_walk/init_textures | count | 9350 | 13.1 k | 3.25 M |
| encode/bind_walk/init_textures_skipped | count | 67.7 k | 78.6 k | 23.55 M |
| encode/bind_walk/inits | count | 806 | 918 | 280.4 k |
| encode/bind_walk/inits_skipped | count | 177 | 195 | 61.7 k |
| encode/bind_walk/merge_textures | count | 9288 | 13.0 k | 3.23 M |
| encode/bind_walk/merge_textures_skipped | count | 19.1 k | 30.0 k | 6.63 M |
| encode/bind_walk/merges | count | 159 | 183 | 55.2 k |
| encode/bind_walk/merges_skipped | count | 161 | 187 | 56.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 348 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 348 |
| encode/pass:early prepass/count | count | 1 | 1 | 348 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 348 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 348 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 348 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 174 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 348 |
| encode/queue/command_buffers | count | 38.75 | 41 | 13.5 k |
| encode/queue/submits | count | 2 | 2 | 696 |
| encode/queue/write_buffer/bytes | bytes | 2.96 M | 3.50 M | 1.03 G |
| encode/queue/write_buffer/calls | count | 286 | 358 | 99.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 53.9 k | 53.9 k | 18.75 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 3132 |
| encode/queue/write_texture/bytes | bytes | 75.7 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.01 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 348 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 2.41 | 3 | 838 |
| physics/steps_dropped | count | 0.18 | 4 | 64 |
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

## Hitches (> 96.5 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2497.9 | main/busy/ms 2466, stage/main/post_update 2417, stage/main/update 34.99, stage/main/pre_update 10.18, stage/main/last 1.18, surface/advance_cpu_ms 1.12, stage/main/first 0.61, stream/drive/ms 0.54, stage/main/fixed_main_loop 0.39, surface/cell_upload_ms 0.30, stream/tick/ms 0.14, bigspace/total/ms 0.10 |
| warm-up | 3.26 | 757.5 | main/busy/ms 2512, stage/main/post_update 2450, render/thread/ms 697, render/busy/ms 537, stage/render/prepare_assets 513, stage/render/prepare_views 157, render/acquire/ms 157, stage/main/update 41.57, stage/main/pre_update 12.13, stage/render/render 11.25, stage/render/prepare_resources 7.85, render/graph/ms 6.90 |
| warm-up | 3.37 | 114.0 | render/thread/ms 808, pipelines/compile_ms 708, render/busy/ms 585, stage/render/prepare_assets 517, pipelines/front_ms 396, pipelines/driver_ms 312, stage/render/prepare_views 158, render/acquire/ms 157, stage/render/render 110, main/busy/ms 109, stage/main/post_update 73.81, render/present/ms 66.25 |
| warm-up | 3.59 | 168.0 | pipelines/wait_ms 3427, pipelines/compile_ms 1687, pipelines/front_ms 1543, render/thread/ms 221, render/busy/ms 217, pipelines/driver_ms 144, main/busy/ms 117, stage/render/prepare_resources 66.83, stage/main/update 66.75, stage/render/prepare_assets 50.68, stage/render/prepare_write_phase_buffers 43.99, stage/main/post_update 33.55 |
| warm-up | 4.56 | 161.7 | pipelines/compile_ms 2862, pipelines/front_ms 2787, main/busy/ms 215, render/thread/ms 201, render/busy/ms 199, pipelines/wait_ms 166, stage/main/update 124, stage/render/prepare_assets 76.34, pipelines/driver_ms 75.51, stage/main/post_update 58.64, stage/render/prepare_resources 44.56, stage/render/prepare_flush 29.30 |
| warm-up | 4.69 | 130.3 | pipelines/compile_ms 4957, pipelines/front_ms 4823, pipelines/wait_ms 862, render/thread/ms 249, render/busy/ms 247, stage/render/prepare_assets 172, main/busy/ms 145, pipelines/driver_ms 134, stage/main/update 91.54, stage/main/post_update 26.83, stage/render/prepare_resources 19.19, stage/render/render 18.47 |
| warm-up | 6.95 | 99.4 | render/thread/ms 167, pipelines/compile_ms 147, pipelines/front_ms 143, stage/render/prepare_views 134, render/acquire/ms 133, main/busy/ms 35.05, render/busy/ms 31.46, stage/main/update 18.83, stage/render/render 15.66, render/graph/ms 13.11, encode//sum 13.07, stage/main/post_update 9.86 |
| warm-up | 14.98 | 142.8 | render/thread/ms 225, stage/render/prepare_views 141, render/acquire/ms 141, render/busy/ms 83.17, stage/render/render 73.91, render/graph/ms 72.67, stage/render/camera_driver 64.14, main/busy/ms 17.97, encode//sum 13.62, stage/render/graph_submit 8.34, stage/main/post_update 8.11, encode/submit 4.80 |
| warm-up | 16.56 | 110.5 | render/thread/ms 193, stage/render/prepare_views 167, render/acquire/ms 167, render/busy/ms 24.82, main/busy/ms 21.35, stage/render/render 13.79, encode//sum 11.91, render/graph/ms 11.72, stage/render/graph_submit 9.03, stage/main/post_update 8.90, encode/submit 5.38, stage/main/fixed_main_loop 4.82 |
| warm-up | 16.67 | 107.3 | render/thread/ms 215, stage/render/prepare_views 191, render/acquire/ms 191, main/busy/ms 23.68, render/busy/ms 23.09, encode//sum 12.05, stage/render/render 12.02, render/graph/ms 10.59, stage/main/post_update 8.67, stage/render/graph_submit 8.12, stage/main/fixed_main_loop 5.96, encode/submit 4.81 |
| warm-up | 16.86 | 98.5 | render/thread/ms 188, stage/render/prepare_views 164, render/acquire/ms 164, render/busy/ms 22.56, main/busy/ms 21.13, encode//sum 13.06, stage/render/render 13.01, render/graph/ms 11.65, stage/render/graph_submit 8.84, stage/main/post_update 8.72, stage/main/fixed_main_loop 5.61, encode/submit 5.22 |
| warm-up | 17.15 | 103.0 | render/thread/ms 195, stage/render/prepare_views 170, render/acquire/ms 170, render/busy/ms 24.18, main/busy/ms 24.01, stage/render/render 12.30, encode//sum 12.27, render/graph/ms 11.02, stage/main/post_update 9.82, stage/render/graph_submit 8.41, stage/main/fixed_main_loop 5.75, encode/submit 5.07 |
| warm-up | 17.25 | 102.8 | render/thread/ms 202, stage/render/prepare_views 177, render/acquire/ms 177, render/busy/ms 24.31, main/busy/ms 21.41, stage/render/render 12.41, encode//sum 11.87, render/graph/ms 11.07, stage/render/graph_submit 8.29, stage/main/post_update 7.31, stage/main/fixed_main_loop 5.78, stage/render/prepare_assets 4.57 |
| warm-up | 17.45 | 105.5 | render/thread/ms 192, stage/render/prepare_views 168, render/acquire/ms 168, main/busy/ms 23.57, render/busy/ms 22.90, stage/render/render 11.96, encode//sum 11.74, render/graph/ms 10.60, stage/main/post_update 8.08, stage/render/graph_submit 8.00, stage/main/fixed_main_loop 6.30, encode/submit 4.99 |
| warm-up | 17.83 | 98.2 | render/thread/ms 190, stage/render/prepare_views 167, render/acquire/ms 167, render/busy/ms 22.50, main/busy/ms 20.69, encode//sum 12.28, stage/render/render 12.19, render/graph/ms 10.97, stage/render/graph_submit 8.24, stage/main/post_update 7.74, stage/main/fixed_main_loop 5.15, encode/submit 5.09 |
| warm-up | 17.92 | 97.2 | render/thread/ms 193, stage/render/prepare_views 169, render/acquire/ms 169, render/busy/ms 23.53, main/busy/ms 22.86, encode//sum 12.74, stage/render/render 12.28, render/graph/ms 11.04, stage/render/graph_submit 8.28, stage/main/post_update 8.23, stage/main/fixed_main_loop 6.16, encode/submit 4.86 |
| warm-up | 18.03 | 100.8 | render/thread/ms 195, stage/render/prepare_views 170, render/acquire/ms 170, render/busy/ms 24.68, main/busy/ms 22.80, stage/render/render 12.94, encode//sum 12.56, render/graph/ms 11.52, stage/render/graph_submit 8.73, stage/main/post_update 8.29, stage/main/fixed_main_loop 5.74, encode/submit 4.97 |
| warm-up | 18.22 | 105.4 | render/thread/ms 195, stage/render/prepare_views 170, render/acquire/ms 169, render/busy/ms 24.98, main/busy/ms 23.26, stage/render/render 12.67, render/graph/ms 11.25, encode//sum 11.21, stage/render/graph_submit 8.73, stage/main/post_update 7.88, encode/submit 5.63, stage/render/prepare_assets 5.61 |
| warm-up | 18.32 | 101.4 | render/thread/ms 204, stage/render/prepare_views 181, render/acquire/ms 181, render/busy/ms 22.52, main/busy/ms 21.26, stage/render/render 12.26, encode//sum 11.79, render/graph/ms 10.82, stage/main/post_update 8.31, stage/render/graph_submit 8.15, encode/submit 5.09, stage/main/fixed_main_loop 4.76 |
| warm-up | 18.61 | 105.0 | render/thread/ms 190, stage/render/prepare_views 166, render/acquire/ms 166, render/busy/ms 23.43, main/busy/ms 21.94, stage/render/render 12.21, encode//sum 11.71, render/graph/ms 10.91, stage/main/post_update 8.37, stage/render/graph_submit 8.36, encode/submit 5.17, stage/main/fixed_main_loop 5.03 |
| warm-up | 18.80 | 100.9 | render/thread/ms 188, stage/render/prepare_views 165, render/acquire/ms 165, render/busy/ms 22.18, main/busy/ms 20.15, encode//sum 12.55, stage/render/render 12.54, render/graph/ms 11.20, stage/render/graph_submit 8.09, stage/main/post_update 7.96, stage/main/fixed_main_loop 4.92, encode/submit 4.45 |
| warm-up | 19.00 | 105.8 | render/thread/ms 194, stage/render/prepare_views 169, render/acquire/ms 169, render/busy/ms 23.86, main/busy/ms 23.41, stage/render/render 12.44, encode//sum 11.08, render/graph/ms 11.05, stage/main/post_update 8.94, stage/render/graph_submit 8.52, encode/submit 5.43, stage/main/fixed_main_loop 5.33 |
| measure | 9.52 | 100.6 | render/thread/ms 190, stage/render/prepare_views 172, render/acquire/ms 172, main/busy/ms 18.78, render/busy/ms 17.51, stage/render/render 9.23, render/graph/ms 8.16, stage/main/post_update 7.35, encode//sum 7.00, stage/render/graph_submit 5.73, stage/main/fixed_main_loop 4.65, stage/main/update 3.65 |
| measure | 9.88 | 100.4 | render/thread/ms 188, stage/render/prepare_views 170, render/acquire/ms 170, main/busy/ms 18.05, render/busy/ms 17.50, stage/render/render 9.47, render/graph/ms 8.35, encode//sum 7.59, stage/main/post_update 7.50, stage/render/graph_submit 5.78, stage/main/fixed_main_loop 4.27, encode/submit 3.38 |
| measure | 10.62 | 98.4 | render/thread/ms 187, stage/render/prepare_views 168, render/acquire/ms 168, main/busy/ms 18.95, render/busy/ms 18.52, stage/render/render 9.96, render/graph/ms 8.76, encode//sum 8.26, stage/main/post_update 7.43, stage/render/graph_submit 6.43, stage/main/fixed_main_loop 5.00, encode/submit 3.90 |
| measure | 10.99 | 99.8 | render/thread/ms 185, stage/render/prepare_views 166, render/acquire/ms 166, main/busy/ms 19.60, render/busy/ms 18.30, stage/render/render 10.03, render/graph/ms 8.74, encode//sum 7.91, stage/main/post_update 7.84, stage/render/graph_submit 6.32, stage/main/fixed_main_loop 4.88, encode/submit 3.92 |
| measure | 11.36 | 98.6 | render/thread/ms 187, stage/render/prepare_views 168, render/acquire/ms 168, main/busy/ms 18.57, render/busy/ms 18.00, stage/render/render 10.33, encode//sum 9.61, render/graph/ms 9.10, stage/main/post_update 7.48, stage/render/graph_submit 6.73, stage/main/fixed_main_loop 4.89, encode/submit 3.95 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --set groundcover.render.reserve_worst_case=true --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=worst-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **99%** (min 99%) · 2640 MHz (min 2640) · memory 8101 MHz · 45.0 W · 61 °C (max 64) · VRAM 7584 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `groundcover.render.reserve_worst_case=on (command line: --set)`
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
