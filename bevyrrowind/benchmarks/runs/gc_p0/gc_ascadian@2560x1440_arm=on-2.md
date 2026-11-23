# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 351 frames over 15.0 s · **median 38.21 ms (26 fps)** · mean 42.81 · p95 89.91 · p99 100.99 (1% low 10 fps) · worst 103.60 ms · 25 spikes over 1.5× median · 99.8 stutters/min (work 99.8/min)
- warm-up: 307 frames over 21.5 s, worst 2492.0 ms
- hitches (> 95.5 ms): **9** measured, 37 in warm-up
- timed GPU passes: 36.95 ms of a 42.81 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 38.21 ms, limit 11 ms
- **FAIL** worst frame 103.60 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 10.759 | 10.759 | 10.799 | 1.00 | 0.082 |
| early prepass | 10.027 | 10.027 | 10.108 | 1.00 | 0.126 |
| range_unpacking | 6.597 | 6.597 | 56.059 | 1.00 | 0.002 |
| dlss_super_resolution | 2.070 | 2.070 | 2.093 | 1.00 | 0.197 |
| early_mesh_preprocessing | 1.990 | 1.990 | 2.801 | 1.00 | 0.003 |
| bin_unpacking | 0.743 | 0.743 | 1.038 | 1.00 | 0.014 |
| sun_visibility | 0.701 | 0.701 | 0.710 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.647 | 0.647 | 0.672 | 1.00 | 0.009 |
| uniform_allocation | 0.379 | 0.379 | 0.507 | 1.00 | 0.022 |
| atmosphere_luts | 0.358 | 0.358 | 0.365 | 1.00 | 0.003 |
| bloom | 0.325 | 0.325 | 0.343 | 1.00 | 0.058 |
| water_velocity | 0.318 | 0.318 | 0.335 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.306 | 0.306 | 0.309 | 1.00 | 0.001 |
| clustering | 0.288 | 0.288 | 0.316 | 1.00 | 0.019 |
| water_sspr | 0.201 | 0.201 | 0.202 | 1.00 | 0.002 |
| ssao | 0.190 | 0.190 | 0.200 | 1.00 | 0.007 |
| water_fft | 0.136 | 0.136 | 0.140 | 1.00 | 0.002 |
| upscaling | 0.132 | 0.132 | 0.135 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.002 |
| tonemapping | 0.089 | 0.089 | 0.095 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.079 | 0.079 | 0.085 | 1.00 | 0.007 |
| render_sky | 0.063 | 0.063 | 0.064 | 1.00 | 0.001 |
| clouds_trace | 0.056 | 0.056 | 0.063 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.411 | 0.13 | 0.001 |
| per_view_shadow_pass | 0.049 | 0.049 | 0.095 | 1.00 | 0.006 |
| clouds_resolve | 0.047 | 0.047 | 0.047 | 1.00 | 0.001 |
| point_shadow_atlas | 0.036 | 0.073 | 0.080 | 0.50 | 0.150 |
| main_transparent_pass_3d | 0.031 | 0.031 | 0.032 | 1.00 | 0.224 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.029 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.217 | 0.228 | 0.13 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.001 |
| clouds_shadow | 0.022 | 0.022 | 0.024 | 1.00 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.012 | 1.00 | 0.003 |
| lightprobe_copy | 0.009 | 0.070 | 0.071 | 0.13 | 0.004 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.017 | 0.018 | 0.50 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.006 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.007 | 1.00 | 0.001 |
| lightprobe_downsampling_first_pass | 0.004 | 0.032 | 0.033 | 0.13 | 0.001 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 41.508 | 88.627 | 102.41 | 14569.2 |
| stage/render/prepare_views | 31.565 | 79.141 | 92.02 | 11079.3 |
| render/acquire/ms | 31.495 | 79.068 | 91.93 | 11054.7 |
| render/busy/ms | 9.621 | 11.090 | 22.87 | 3377.1 |
| main/busy/ms | 8.038 | 9.387 | 10.76 | 2821.4 |
| encode//sum | 5.896 | 8.433 | 49.26 | 2069.5 |
| stage/render/render | 5.807 | 6.959 | 19.15 | 2038.1 |
| render/graph/ms | 5.141 | 6.254 | 18.58 | 1804.6 |
| stage/main/post_update | 3.787 | 4.257 | 6.06 | 1329.3 |
| stage/render/graph_submit | 3.770 | 4.812 | 17.19 | 1323.3 |
| encode/submit | 2.182 | 2.903 | 5.20 | 766.0 |
| encode/finish_wall | 1.605 | 2.150 | 14.66 | 563.2 |
| stage/main/update | 1.591 | 1.821 | 2.37 | 558.4 |
| stage/main/fixed_main_loop | 1.325 | 2.120 | 2.63 | 465.1 |
| stage/render/camera_driver | 1.275 | 1.503 | 1.72 | 447.6 |
| encode/pass:early prepass/sum | 1.077 | 1.414 | 13.46 | 378.1 |
| encode/pass:shadow_directional_light/sum | 1.043 | 1.675 | 8.04 | 366.1 |
| encode/pass:main_opaque_pass_3d/sum | 1.016 | 1.436 | 1.76 | 356.7 |
| stage/render/prepare_assets | 0.812 | 1.187 | 2.34 | 285.1 |
| stage/render/prepare_write_phase_buffers | 0.530 | 0.750 | 1.36 | 186.0 |
| stage/render/prepare_meshes | 0.528 | 0.663 | 0.87 | 185.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.519 | 0.785 | 6.02 | 182.1 |
| stage/render/prepare_resources | 0.515 | 0.592 | 0.70 | 180.8 |
| stage/main/first | 0.477 | 0.665 | 1.25 | 167.5 |
| bigspace/total/ms | 0.432 | 0.533 | 0.97 | 151.6 |
| stage/main/pre_update | 0.411 | 0.523 | 0.94 | 144.4 |
| render/present/ms | 0.391 | 0.505 | 1.15 | 137.4 |
| stage/render/prepare_batch_phases | 0.371 | 0.512 | 0.96 | 130.3 |
| stage/render/prepare_flush | 0.330 | 0.464 | 1.50 | 115.7 |
| controller/feed/ms | 0.300 | 0.370 | 0.51 | 105.4 |
| stage/render/prepare_bind_groups | 0.263 | 0.416 | 0.53 | 92.5 |
| stage/main/last | 0.263 | 0.304 | 2.24 | 92.4 |
| stage/render/queue | 0.246 | 0.291 | 0.51 | 86.3 |
| stage/render/extract_commands | 0.208 | 0.243 | 0.30 | 73.0 |
| controller/move/ms | 0.193 | 0.320 | 0.39 | 67.6 |
| bigspace/low_precision/ms | 0.190 | 0.240 | 0.32 | 66.6 |
| stage/render/specialize | 0.185 | 0.232 | 0.27 | 65.1 |
| stage/render/create_views | 0.184 | 0.214 | 0.28 | 64.5 |
| anim/evaluate_cpu/ms | 0.182 | 0.206 | 1.43 | 63.9 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.177 | 0.432 | 6.52 | 62.3 |
| stream/drive/ms | 0.167 | 0.193 | 0.32 | 58.5 |
| anim/evaluate/ms | 0.144 | 0.165 | 1.35 | 50.5 |
| stream/tick/ms | 0.139 | 0.163 | 0.28 | 48.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.129 | 0.203 | 0.43 | 45.5 |
| physics/step/ms | 0.101 | 0.131 | 0.16 | 35.6 |
| bigspace/high_precision/ms | 0.093 | 0.113 | 0.24 | 32.6 |
| encode/resolve_diagnostics/sum | 0.062 | 0.105 | 0.18 | 21.9 |
| controller/write/ms | 0.062 | 0.076 | 0.21 | 21.8 |
| stage/render/phase_sort | 0.046 | 0.066 | 0.13 | 16.2 |
| stage/render/prepare_collect_phase_buffers | 0.041 | 0.060 | 0.67 | 14.4 |
| terrain/update/ms | 0.035 | 0.046 | 0.07 | 12.3 |
| stage/render/cleanup | 0.032 | 0.057 | 0.29 | 11.1 |
| controller/avoid/ms | 0.030 | 0.053 | 0.06 | 10.6 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.040 | 0.09 | 8.3 |
| encode/camera_driver_span_end/sum | 0.022 | 0.036 | 0.10 | 7.7 |
| weather/map/ms | 0.015 | 0.020 | 0.07 | 5.4 |
| anim/apply/ms | 0.012 | 0.014 | 0.03 | 4.1 |
| weather/eval/ms | 0.008 | 0.011 | 0.02 | 2.9 |
| script/update/ms | 0.008 | 0.010 | 0.03 | 2.8 |
| script/input/ms | 0.007 | 0.006 | 0.58 | 2.3 |
| far/select/ms | 0.004 | 0.005 | 0.01 | 1.4 |
| script/ui/ms | 0.004 | 0.005 | 0.03 | 1.3 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.00 | 0.7 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.00 | 0.5 |
| character/finish/ms | 0.001 | 0.002 | 0.00 | 0.5 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 0.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 0.3 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.2 |
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
| crowd/joints_extracted/count | count | 779 | 1167 | 615 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.69 | 14.51 | 0.66 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.02 |
| encode/finish_critical | ms | 1.21 | 14.51 | 1.27 |
| encode/pass:early prepass/max | ms | 1.04 | 13.46 | 1.08 |
| encode/pass:main_opaque_pass_3d/max | ms | 1.01 | 1.76 | 1.04 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.43 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.48 | 6.02 | 0.67 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.28 | 6.52 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.97 | 8.04 | 1.27 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.18 | 0.05 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 33.41 | 94.71 | 32.99 |
| gpu/timed/ms | ms | 33.41 | 94.71 | 32.99 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1512 | 1512 | 1512 |
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
| phases/shadow/batch_sets | count | 105 | 133 | 121 |
| phases/shadow/bins | count | 2207 | 2208 | 2195 |
| phases/shadow/instance_batches | count | 9 | 27 | 27 |
| phases/shadow/instance_draws | count | 7317 | 22.0 k | 22.0 k |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 248.6 k |
| phases/shadow/items | count | 5386 | 5388 | 5374 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 55 | 59 | 54 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 360 | 360 | 360 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 9 | 68 | 67 |
| preprocess/bind_groups_created | count | 4 | 61 | 60 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 141 | 141 | 141 |
| process/cpu_usage | % | 14.02 | 22.50 | 16.44 |
| process/mem_usage | GiB | 3.96 | 3.97 | 3.97 |
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
| shadow_atlas/mover_draws | count | 2 | 4 | 0 |
| shadow_atlas/movers | count | 100 | 100 | 100 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 2 | 4 | 2 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 2 | 4 | 2 |
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
| skin/blocks_changed/count | count | 19 | 26 | 17 |
| skin/blocks_skipped/count | count | 190 | 196 | 193 |
| skin/blocks_walked/count | count | 20 | 27 | 17 |
| skin/joint_bytes_written/bytes | bytes | 61.4 k | 80.6 k | 52.2 k |
| skin/joints_written/count | count | 960 | 1260 | 816 |
| skin/upload_writes/count | count | 8 | 11 | 8 |
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
| system/cpu_usage | % | 17.71 | 24.47 | 17.56 |
| system/mem_usage | % | 58.85 | 58.88 | 58.85 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 204 | 232 | 220 |
| views/all/bins | count | 5675 | 5676 | 5663 |
| views/all/count | count | 11 | 11 | 10 |
| views/all/instance_batches | count | 53 | 71 | 71 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 36.6 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.66 M |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 114 | 118 | 113 |
| views/all/pipelines_distinct | count | 80 | 85 | 80 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.68 M |
| views/cascade/batch_sets | count | 14 | 60 | 60 |
| views/cascade/bins | count | 2043 | 2043 | 2043 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 27 |
| views/cascade/instance_draws | count | 7317 | 22.0 k | 22.0 k |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 248.6 k |
| views/cascade/items | count | 5222 | 5222 | 5222 |
| views/cascade/pipelines | count | 43 | 43 | 43 |
| views/cascade/work_items | count | 83.0 k | 253.8 k | 253.8 k |
| views/lamp_face/batch_sets | count | 58 | 91 | 61 |
| views/lamp_face/bins | count | 164 | 167 | 152 |
| views/lamp_face/count | count | 7 | 7 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 164 | 167 | 152 |
| views/lamp_face/pipelines | count | 12 | 16 | 11 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 58 | 91 | 61 |
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
| anim/bones_written/count | count | 52.00 | 86 | 18.3 k |
| anim/events/count | count | 0.21 | 3 | 75 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 7.98 | 8 | 2800 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| encode//count | count | 28.25 | 30 | 9916 |
| encode/bind_walk/init_textures | count | 9353 | 13.1 k | 3.28 M |
| encode/bind_walk/init_textures_skipped | count | 58.0 k | 68.9 k | 20.34 M |
| encode/bind_walk/inits | count | 806 | 918 | 282.9 k |
| encode/bind_walk/inits_skipped | count | 170 | 184 | 59.5 k |
| encode/bind_walk/merge_textures | count | 9292 | 13.0 k | 3.26 M |
| encode/bind_walk/merge_textures_skipped | count | 19.1 k | 30.0 k | 6.70 M |
| encode/bind_walk/merges | count | 159 | 183 | 55.7 k |
| encode/bind_walk/merges_skipped | count | 158 | 180 | 55.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 351 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 351 |
| encode/pass:early prepass/count | count | 1 | 1 | 351 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 351 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 351 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 351 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 176 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 351 |
| encode/queue/command_buffers | count | 38.75 | 41 | 13.6 k |
| encode/queue/submits | count | 2 | 2 | 702 |
| encode/queue/write_buffer/bytes | bytes | 2.96 M | 3.51 M | 1.04 G |
| encode/queue/write_buffer/calls | count | 286 | 358 | 100.5 k |
| encode/queue/write_buffer_with/bytes | bytes | 58.4 k | 58.4 k | 20.49 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 3159 |
| encode/queue/write_texture/bytes | bytes | 75.1 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.01 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 351 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 2.38 | 3 | 836 |
| physics/steps_dropped | count | 0.19 | 4 | 65 |
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

## Hitches (> 95.5 ms; slowest 46, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2492.0 | main/busy/ms 2458, stage/main/post_update 2409, stage/main/update 35.54, stage/main/pre_update 9.47, stage/main/last 1.09, surface/advance_cpu_ms 0.77, stage/main/first 0.58, stream/drive/ms 0.52, stage/main/fixed_main_loop 0.41, surface/cell_upload_ms 0.39, stream/tick/ms 0.15, bigspace/total/ms 0.10 |
| warm-up | 3.36 | 864.1 | main/busy/ms 2504, stage/main/post_update 2442, render/thread/ms 802, render/busy/ms 650, stage/render/prepare_assets 611, stage/render/prepare_views 149, render/acquire/ms 148, stage/main/update 42.91, stage/render/render 21.55, render/graph/ms 16.64, stage/render/camera_driver 14.33, stage/main/pre_update 11.44 |
| warm-up | 3.49 | 134.3 | render/thread/ms 934, render/busy/ms 714, stage/render/prepare_assets 621, stage/render/prepare_views 149, render/acquire/ms 149, stage/render/render 130, main/busy/ms 103, stage/main/post_update 74.81, render/present/ms 72.00, render/graph/ms 56.05, pipelines/compile_ms 47.70, pipelines/front_ms 45.72 |
| warm-up | 3.60 | 108.6 | pipelines/compile_ms 1533, pipelines/front_ms 863, pipelines/driver_ms 670, pipelines/wait_ms 438, render/thread/ms 239, render/busy/ms 169, stage/render/render 166, render/graph/ms 94.13, main/busy/ms 82.25, render/present/ms 69.50, stage/main/post_update 54.55, stage/render/graph_submit 53.72 |
| warm-up | 3.87 | 213.8 | pipelines/wait_ms 8747, pipelines/compile_ms 3173, pipelines/front_ms 3001, pipelines/driver_ms 172, main/busy/ms 166, render/thread/ms 166, render/busy/ms 164, stage/render/prepare_assets 92.51, stage/main/update 57.12, stage/main/pre_update 43.33, stage/render/prepare_resources 41.55, stage/main/post_update 41.29 |
| warm-up | 4.60 | 148.9 | pipelines/compile_ms 1256, pipelines/front_ms 1206, main/busy/ms 167, render/thread/ms 160, render/busy/ms 157, stage/main/update 92.36, stage/render/prepare_assets 89.03, stage/main/post_update 64.97, pipelines/driver_ms 50.62, pipelines/wait_ms 43.11, pipelines/lock_ms 32.99, stage/render/render 28.89 |
| warm-up | 4.73 | 129.0 | pipelines/compile_ms 2579, pipelines/front_ms 2502, render/thread/ms 253, render/busy/ms 251, pipelines/wait_ms 196, stage/render/prepare_assets 156, main/busy/ms 150, stage/main/update 100, pipelines/driver_ms 77.05, pipelines/lock_ms 55.90, stage/main/post_update 32.74, stage/render/create_views 28.15 |
| warm-up | 4.86 | 126.8 | pipelines/compile_ms 2695, pipelines/front_ms 2593, render/thread/ms 241, render/busy/ms 240, pipelines/wait_ms 219, stage/render/prepare_assets 165, pipelines/driver_ms 102, main/busy/ms 64.73, pipelines/lock_ms 44.43, stage/main/update 32.19, stage/render/create_views 27.72, stage/render/render 22.44 |
| warm-up | 7.31 | 101.0 | render/thread/ms 110, stage/render/prepare_views 76.59, render/acquire/ms 76.46, main/busy/ms 32.25, render/busy/ms 32.18, stage/main/update 17.35, pipelines/compile_ms 12.88, stage/render/render 12.24, pipelines/front_ms 12.08, encode//sum 11.45, render/graph/ms 10.52, stage/main/post_update 9.25 |
| warm-up | 7.95 | 141.3 | render/thread/ms 227, pipelines/compile_ms 172, stage/render/prepare_views 144, render/acquire/ms 143, pipelines/front_ms 124, render/busy/ms 82.61, stage/render/render 70.81, render/graph/ms 69.37, stage/render/camera_driver 61.58, pipelines/driver_ms 48.69, main/busy/ms 27.22, encode//sum 9.52 |
| warm-up | 17.07 | 111.6 | render/thread/ms 192, stage/render/prepare_views 167, render/acquire/ms 167, render/busy/ms 24.07, main/busy/ms 20.48, stage/render/render 13.75, encode//sum 12.57, render/graph/ms 12.29, stage/render/graph_submit 9.12, stage/main/post_update 8.14, encode/submit 5.87, stage/main/update 4.35 |
| warm-up | 17.17 | 100.3 | render/thread/ms 209, stage/render/prepare_views 184, render/acquire/ms 184, render/busy/ms 23.49, main/busy/ms 23.28, stage/render/render 13.77, encode//sum 13.69, render/graph/ms 12.18, stage/main/post_update 10.80, stage/render/graph_submit 9.01, encode/submit 5.42, stage/main/update 4.96 |
| warm-up | 17.37 | 100.0 | render/thread/ms 191, stage/render/prepare_views 159, render/acquire/ms 159, render/busy/ms 31.17, main/busy/ms 30.06, stage/render/prepare_assets 12.82, stage/render/render 11.74, encode//sum 11.45, render/graph/ms 10.33, stage/main/fixed_main_loop 9.70, stage/main/post_update 8.38, stage/render/graph_submit 7.84 |
| warm-up | 17.47 | 101.3 | render/thread/ms 199, stage/render/prepare_views 173, render/acquire/ms 173, render/busy/ms 24.77, main/busy/ms 23.12, encode//sum 12.59, stage/render/render 11.96, render/graph/ms 10.58, stage/render/graph_submit 7.95, stage/main/post_update 7.91, stage/render/prepare_assets 6.35, stage/main/fixed_main_loop 5.30 |
| warm-up | 17.57 | 95.8 | render/thread/ms 195, stage/render/prepare_views 171, render/acquire/ms 170, render/busy/ms 23.26, main/busy/ms 21.26, stage/render/render 12.76, encode//sum 12.60, render/graph/ms 11.49, stage/render/graph_submit 8.76, stage/main/post_update 7.84, encode/submit 4.86, stage/main/fixed_main_loop 4.62 |
| warm-up | 17.66 | 97.0 | render/thread/ms 190, stage/render/prepare_views 165, render/acquire/ms 164, render/busy/ms 24.86, main/busy/ms 20.99, stage/render/render 13.87, render/graph/ms 12.49, encode//sum 12.44, stage/render/graph_submit 9.71, stage/main/post_update 7.39, encode/submit 5.60, stage/main/fixed_main_loop 4.51 |
| warm-up | 17.76 | 101.1 | render/thread/ms 195, stage/render/prepare_views 170, render/acquire/ms 170, render/busy/ms 24.63, main/busy/ms 23.38, stage/render/render 12.54, encode//sum 11.95, render/graph/ms 11.11, stage/render/graph_submit 8.50, stage/main/post_update 8.08, stage/render/prepare_assets 5.58, encode/submit 5.22 |
| warm-up | 17.95 | 96.5 | render/thread/ms 188, stage/render/prepare_views 164, render/acquire/ms 164, render/busy/ms 23.38, main/busy/ms 21.41, encode//sum 12.95, stage/render/render 12.95, render/graph/ms 11.62, stage/render/graph_submit 8.77, stage/main/post_update 7.63, stage/main/fixed_main_loop 6.27, encode/submit 5.01 |
| warm-up | 18.05 | 96.5 | render/thread/ms 190, stage/render/prepare_views 165, render/acquire/ms 165, render/busy/ms 24.74, main/busy/ms 23.54, stage/render/render 12.93, encode//sum 12.05, render/graph/ms 11.51, stage/render/graph_submit 8.82, stage/main/post_update 8.05, stage/main/fixed_main_loop 6.32, encode/submit 5.33 |
| warm-up | 18.16 | 108.4 | render/thread/ms 202, stage/render/prepare_views 178, render/acquire/ms 178, render/busy/ms 23.56, main/busy/ms 23.49, stage/render/render 11.72, encode//sum 11.55, render/graph/ms 10.36, stage/main/post_update 8.08, stage/render/graph_submit 7.64, stage/main/fixed_main_loop 5.61, stage/render/prepare_assets 5.16 |
| warm-up | 18.35 | 97.8 | render/thread/ms 191, stage/render/prepare_views 165, render/acquire/ms 165, render/busy/ms 24.72, main/busy/ms 21.48, encode//sum 14.05, stage/render/render 13.72, render/graph/ms 12.43, stage/render/graph_submit 9.16, stage/main/post_update 7.99, stage/main/fixed_main_loop 5.88, encode/submit 5.05 |
| warm-up | 18.45 | 97.9 | render/thread/ms 193, stage/render/prepare_views 167, render/acquire/ms 167, render/busy/ms 25.07, main/busy/ms 21.45, encode//sum 13.30, stage/render/render 12.65, render/graph/ms 11.40, stage/render/graph_submit 8.36, stage/main/post_update 7.85, stage/main/fixed_main_loop 5.12, stage/render/prepare_assets 5.01 |
| warm-up | 18.55 | 100.9 | render/thread/ms 196, stage/render/prepare_views 171, render/acquire/ms 171, render/busy/ms 25.04, main/busy/ms 23.40, encode//sum 14.88, stage/render/render 12.89, render/graph/ms 11.58, stage/render/graph_submit 8.67, stage/main/post_update 8.24, encode/submit 5.45, stage/main/fixed_main_loop 5.25 |
| warm-up | 18.65 | 99.2 | render/thread/ms 198, stage/render/prepare_views 172, render/acquire/ms 172, render/busy/ms 25.26, main/busy/ms 23.68, encode//sum 14.61, stage/render/render 13.14, render/graph/ms 11.82, stage/render/graph_submit 8.87, stage/main/post_update 8.30, encode/submit 5.30, stage/main/fixed_main_loop 5.11 |
| warm-up | 18.75 | 97.6 | render/thread/ms 194, stage/render/prepare_views 169, render/acquire/ms 169, render/busy/ms 24.76, main/busy/ms 22.71, encode//sum 14.72, stage/render/render 13.27, render/graph/ms 11.96, stage/render/graph_submit 8.96, stage/main/post_update 8.29, stage/main/update 4.87, stage/main/fixed_main_loop 4.87 |
| warm-up | 18.94 | 103.7 | render/thread/ms 194, stage/render/prepare_views 171, render/acquire/ms 171, render/busy/ms 22.10, main/busy/ms 21.29, stage/render/render 12.69, encode//sum 12.47, render/graph/ms 11.31, stage/render/graph_submit 8.68, stage/main/post_update 8.36, encode/submit 5.47, stage/main/fixed_main_loop 4.89 |
| warm-up | 19.04 | 95.6 | render/thread/ms 196, stage/render/prepare_views 174, render/acquire/ms 174, render/busy/ms 21.67, main/busy/ms 21.49, stage/render/render 11.74, encode//sum 11.10, render/graph/ms 10.45, stage/main/post_update 8.26, stage/render/graph_submit 7.74, stage/main/fixed_main_loop 5.07, encode/submit 4.76 |
| warm-up | 19.14 | 99.3 | render/thread/ms 192, stage/render/prepare_views 168, render/acquire/ms 168, render/busy/ms 23.47, main/busy/ms 21.64, stage/render/render 12.85, encode//sum 12.16, render/graph/ms 11.53, stage/render/graph_submit 8.70, stage/main/post_update 8.04, encode/submit 5.05, stage/main/fixed_main_loop 5.04 |
| warm-up | 19.24 | 97.4 | render/thread/ms 194, stage/render/prepare_views 169, render/acquire/ms 169, render/busy/ms 24.45, main/busy/ms 21.65, stage/render/render 13.33, encode//sum 13.22, render/graph/ms 12.05, stage/render/graph_submit 9.23, stage/main/post_update 7.91, encode/submit 5.31, stage/main/fixed_main_loop 5.14 |
| warm-up | 19.34 | 101.8 | render/thread/ms 197, stage/render/prepare_views 173, render/acquire/ms 172, render/busy/ms 23.40, main/busy/ms 22.72, encode//sum 12.90, stage/render/render 12.48, render/graph/ms 11.24, stage/main/post_update 8.85, stage/render/graph_submit 8.30, stage/main/fixed_main_loop 5.50, encode/submit 4.88 |
| warm-up | 19.44 | 98.1 | render/thread/ms 197, stage/render/prepare_views 174, render/acquire/ms 174, render/busy/ms 22.76, main/busy/ms 21.96, encode//sum 13.62, stage/render/render 13.12, render/graph/ms 11.83, stage/render/graph_submit 8.75, stage/main/post_update 8.68, stage/main/fixed_main_loop 5.32, encode/submit 5.24 |
| warm-up | 19.54 | 100.6 | render/thread/ms 196, stage/render/prepare_views 171, render/acquire/ms 171, render/busy/ms 24.01, main/busy/ms 21.35, encode//sum 14.57, stage/render/render 13.71, render/graph/ms 12.27, stage/render/graph_submit 9.26, stage/main/post_update 7.92, stage/main/fixed_main_loop 5.30, encode/submit 5.08 |
| warm-up | 19.63 | 97.2 | render/thread/ms 195, stage/render/prepare_views 171, render/acquire/ms 171, render/busy/ms 23.40, main/busy/ms 21.64, stage/render/render 13.05, encode//sum 12.94, render/graph/ms 11.57, stage/render/graph_submit 8.77, stage/main/post_update 8.03, stage/main/fixed_main_loop 5.34, encode/submit 4.98 |
| warm-up | 19.74 | 103.2 | render/thread/ms 198, stage/render/prepare_views 174, render/acquire/ms 174, render/busy/ms 23.16, main/busy/ms 22.28, stage/render/render 12.88, encode//sum 11.61, render/graph/ms 11.55, stage/render/graph_submit 8.63, stage/main/post_update 8.16, encode/submit 5.46, stage/main/fixed_main_loop 5.41 |
| warm-up | 19.84 | 105.4 | render/thread/ms 206, stage/render/prepare_views 181, render/acquire/ms 181, main/busy/ms 25.46, render/busy/ms 24.64, stage/render/render 12.97, encode//sum 12.45, render/graph/ms 11.72, stage/main/post_update 8.94, stage/render/graph_submit 8.58, stage/main/fixed_main_loop 5.58, encode/submit 5.19 |
| warm-up | 19.94 | 98.9 | render/thread/ms 202, stage/render/prepare_views 177, render/acquire/ms 176, main/busy/ms 25.50, render/busy/ms 24.52, stage/render/render 12.16, encode//sum 11.74, render/graph/ms 10.89, stage/main/post_update 8.72, stage/render/graph_submit 8.01, stage/main/fixed_main_loop 5.74, stage/render/prepare_assets 5.39 |
| warm-up | 20.04 | 97.0 | render/thread/ms 193, stage/render/prepare_views 170, render/acquire/ms 170, main/busy/ms 22.67, render/busy/ms 22.28, stage/render/render 11.93, encode//sum 10.60, render/graph/ms 10.50, stage/main/post_update 8.51, stage/render/graph_submit 7.92, stage/main/fixed_main_loop 5.82, encode/submit 4.57 |
| measure | 9.56 | 100.9 | render/thread/ms 187, stage/render/prepare_views 169, render/acquire/ms 169, main/busy/ms 18.69, render/busy/ms 17.29, stage/render/render 9.01, render/graph/ms 7.84, stage/main/post_update 7.77, encode//sum 7.08, stage/render/graph_submit 5.61, stage/main/fixed_main_loop 4.04, stage/main/update 3.55 |
| measure | 9.93 | 99.4 | render/thread/ms 182, stage/render/prepare_views 163, render/acquire/ms 163, main/busy/ms 18.84, render/busy/ms 17.72, stage/render/render 9.37, render/graph/ms 8.22, stage/main/post_update 7.87, encode//sum 7.18, stage/render/graph_submit 5.97, stage/main/fixed_main_loop 4.55, encode/submit 3.45 |
| measure | 10.30 | 101.5 | render/thread/ms 190, stage/render/prepare_views 171, render/acquire/ms 171, main/busy/ms 17.96, render/busy/ms 17.43, stage/render/render 9.33, render/graph/ms 8.14, stage/main/post_update 7.24, encode//sum 6.69, stage/render/graph_submit 5.64, stage/main/fixed_main_loop 4.50, stage/main/update 3.37 |
| measure | 10.68 | 101.1 | render/thread/ms 193, stage/render/prepare_views 174, render/acquire/ms 173, main/busy/ms 20.12, render/busy/ms 18.49, stage/render/render 9.99, stage/main/post_update 8.86, render/graph/ms 8.68, encode//sum 7.79, stage/render/graph_submit 6.29, stage/main/fixed_main_loop 4.32, encode/submit 3.84 |
| measure | 10.87 | 96.0 | render/thread/ms 184, stage/render/prepare_views 164, render/acquire/ms 163, render/busy/ms 20.20, main/busy/ms 18.58, stage/render/render 10.75, render/graph/ms 9.30, encode//sum 8.31, stage/main/post_update 7.74, stage/render/graph_submit 6.92, stage/main/fixed_main_loop 4.63, encode/submit 4.04 |
| measure | 11.06 | 101.1 | render/thread/ms 189, stage/render/prepare_views 170, render/acquire/ms 169, main/busy/ms 19.00, render/busy/ms 18.59, stage/render/render 9.89, render/graph/ms 8.59, encode//sum 7.93, stage/main/post_update 7.63, stage/render/graph_submit 6.35, stage/main/fixed_main_loop 4.53, encode/submit 4.02 |
| measure | 11.24 | 97.7 | render/thread/ms 182, stage/render/prepare_views 160, render/acquire/ms 160, render/busy/ms 21.15, main/busy/ms 19.50, stage/render/render 10.48, render/graph/ms 9.11, encode//sum 8.64, stage/main/post_update 7.77, stage/render/graph_submit 6.48, stage/main/fixed_main_loop 4.65, encode/submit 3.88 |
| measure | 11.44 | 103.6 | render/thread/ms 191, stage/render/prepare_views 171, render/acquire/ms 171, main/busy/ms 19.28, render/busy/ms 19.07, stage/render/render 10.92, render/graph/ms 9.74, encode//sum 9.54, stage/main/post_update 7.51, stage/render/graph_submit 7.40, stage/main/fixed_main_loop 4.87, encode/submit 4.28 |
| measure | 11.62 | 96.6 | render/thread/ms 177, stage/render/prepare_views 156, render/acquire/ms 156, render/busy/ms 21.04, main/busy/ms 19.75, stage/render/render 11.41, encode//sum 10.96, render/graph/ms 9.98, stage/main/post_update 7.76, stage/render/graph_submit 7.29, stage/main/fixed_main_loop 4.68, encode/submit 4.40 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0\gc_ascadian@2560x1440_arm=on-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **99%** (min 99%) · 2625 MHz (min 2625) · memory 8101 MHz · 45.9 W · 65 °C (max 68) · VRAM 7555 MiB · throttled 100% of samples

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
