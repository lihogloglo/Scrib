# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6693 frames over 65.6 s · **median 9.71 ms (103 fps)** · mean 9.81 · p95 11.45 · p99 13.15 (1% low 76 fps) · worst 31.91 ms · 31 spikes over 1.5× median · 25.6 stutters/min (work 20.1/min)
- warm-up: 352 frames over 9.0 s, worst 2443.6 ms
- hitches (> 25.0 ms): **2** measured, 41 in warm-up
- timed GPU passes: 6.50 ms of a 9.81 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.71 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 31.91 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.867 | 2.036 | 2.070 | 0.92 | 0.175 |
| main_opaque_pass_3d | 1.504 | 1.640 | 2.374 | 0.92 | 0.063 |
| early prepass | 0.471 | 0.514 | 0.753 | 0.92 | 0.088 |
| bloom | 0.291 | 0.318 | 0.325 | 0.92 | 0.064 |
| atmosphere_luts | 0.287 | 0.313 | 0.378 | 0.92 | 0.003 |
| water_velocity | 0.206 | 0.225 | 0.247 | 0.92 | 0.002 |
| water_sspr | 0.185 | 0.202 | 0.203 | 0.92 | 0.002 |
| ssao | 0.172 | 0.188 | 0.191 | 0.92 | 0.008 |
| point_shadow_atlas | 0.137 | 0.250 | 0.473 | 0.55 | 0.238 |
| upscaling | 0.119 | 0.129 | 0.136 | 0.92 | 0.004 |
| atmos_froxel_inject | 0.117 | 0.128 | 0.130 | 0.92 | 0.001 |
| water_fft | 0.112 | 0.122 | 0.126 | 0.92 | 0.003 |
| uniform_allocation | 0.106 | 0.115 | 0.140 | 0.92 | 0.021 |
| clouds_composite | 0.096 | 0.105 | 0.109 | 0.92 | 0.002 |
| bin_unpacking | 0.084 | 0.091 | 0.112 | 0.92 | 0.015 |
| gpu_instance_materialization | 0.083 | 0.091 | 0.095 | 0.92 | 0.004 |
| tonemapping | 0.079 | 0.087 | 0.092 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.064 | 0.070 | 0.140 | 0.92 | 0.008 |
| clustering | 0.059 | 0.064 | 0.068 | 0.92 | 0.020 |
| clouds_trace | 0.051 | 0.055 | 0.079 | 0.92 | 0.001 |
| early_mesh_preprocessing | 0.049 | 0.054 | 0.070 | 0.92 | 0.003 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.92 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.413 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.59 | 0.002 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.92 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.026 | 0.047 | 0.071 | 0.55 | 0.009 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.220 | 0.231 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.92 | 0.003 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.017 | 0.027 | 0.92 | 0.005 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.92 | 0.183 |
| point_shadow_atlas/uniform_allocation | 0.014 | 0.026 | 0.059 | 0.55 | 0.007 |
| point_shadow_atlas/bin_unpacking | 0.010 | 0.019 | 0.039 | 0.55 | 0.005 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.92 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.92 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.55 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.622 | 9.970 | 20.31 | 57705.7 |
| render/busy/ms | 8.103 | 9.465 | 19.68 | 54236.7 |
| main/busy/ms | 8.044 | 9.319 | 25.43 | 53836.8 |
| stage/render/render | 5.046 | 5.974 | 11.17 | 33773.7 |
| stage/main/post_update | 4.530 | 5.384 | 10.01 | 30317.4 |
| render/graph/ms | 4.374 | 5.216 | 9.79 | 29274.8 |
| encode//sum | 3.166 | 3.921 | 11.61 | 21188.7 |
| stage/render/graph_submit | 3.074 | 3.820 | 7.15 | 20576.4 |
| encode/submit | 1.937 | 2.730 | 4.60 | 12965.8 |
| stage/main/update | 1.829 | 2.318 | 20.18 | 12244.1 |
| stage/render/camera_driver | 1.238 | 1.489 | 2.84 | 8284.9 |
| encode/finish_wall | 1.216 | 1.590 | 2.93 | 8141.6 |
| encode/pass:early prepass/sum | 0.817 | 1.064 | 2.16 | 5470.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.817 | 1.073 | 2.05 | 5464.9 |
| encode/pass:shadow_directional_light/sum | 0.808 | 1.214 | 2.25 | 5404.9 |
| stage/render/prepare_write_phase_buffers | 0.572 | 0.864 | 1.48 | 3829.7 |
| bigspace/total/ms | 0.551 | 1.320 | 2.30 | 3684.7 |
| stage/render/prepare_assets | 0.499 | 0.640 | 11.76 | 3342.9 |
| stage/main/fixed_main_loop | 0.495 | 0.893 | 3.26 | 3313.6 |
| render/present/ms | 0.423 | 1.169 | 1.98 | 2829.7 |
| stage/render/prepare_meshes | 0.421 | 0.525 | 3.58 | 2820.6 |
| stage/render/prepare_resources | 0.413 | 0.492 | 1.05 | 2766.7 |
| stage/main/pre_update | 0.388 | 0.561 | 3.00 | 2596.1 |
| stage/main/first | 0.367 | 0.483 | 5.45 | 2453.3 |
| stage/render/prepare_flush | 0.321 | 0.524 | 1.07 | 2145.4 |
| controller/feed/ms | 0.311 | 0.392 | 0.78 | 2083.9 |
| stage/render/prepare_bind_groups | 0.301 | 0.491 | 1.56 | 2014.8 |
| stage/render/prepare_batch_phases | 0.271 | 0.414 | 0.84 | 1814.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.265 | 0.439 | 1.54 | 1772.1 |
| stage/main/last | 0.255 | 0.299 | 0.82 | 1705.9 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.249 | 1.137 | 2.00 | 1667.1 |
| bigspace/low_precision/ms | 0.232 | 0.765 | 1.83 | 1555.6 |
| stream/drive/ms | 0.198 | 0.278 | 10.19 | 1322.8 |
| stage/render/create_views | 0.197 | 0.267 | 0.55 | 1317.8 |
| stage/render/extract_commands | 0.185 | 0.216 | 0.67 | 1240.7 |
| bigspace/high_precision/ms | 0.171 | 0.737 | 2.00 | 1141.3 |
| anim/evaluate/ms | 0.156 | 0.269 | 1.44 | 1041.4 |
| stage/render/specialize | 0.154 | 0.244 | 1.62 | 1028.4 |
| anim/evaluate_cpu/ms | 0.150 | 0.190 | 0.66 | 1001.3 |
| stream/tick/ms | 0.146 | 0.211 | 0.48 | 975.4 |
| stage/render/prepare_views | 0.137 | 0.177 | 1.18 | 919.6 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.111 | 0.843 | 1.61 | 745.8 |
| pipelines/compile_ms | 0.104 | 0.000 | 124.88 | 699.3 |
| pipelines/front_ms | 0.101 | 0.000 | 121.94 | 679.1 |
| stage/render/queue | 0.096 | 0.168 | 1.50 | 645.3 |
| render/acquire/ms | 0.096 | 0.125 | 1.12 | 639.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.089 | 0.140 | 0.75 | 593.2 |
| stage/render/phase_sort | 0.080 | 0.109 | 0.40 | 534.9 |
| controller/write/ms | 0.073 | 0.100 | 0.50 | 491.0 |
| controller/move/ms | 0.065 | 0.169 | 0.49 | 437.4 |
| terrain/update/ms | 0.059 | 0.287 | 1.48 | 392.1 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.053 | 0.000 | 2.67 | 355.7 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.073 | 0.28 | 308.3 |
| physics/step/ms | 0.041 | 0.092 | 0.58 | 277.3 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.48 | 211.0 |
| stage/render/cleanup | 0.024 | 0.032 | 0.47 | 157.3 |
| physics/insert/ms | 0.023 | 0.266 | 0.85 | 153.1 |
| terrain/select/ms | 0.021 | 0.254 | 0.45 | 140.9 |
| weather/map/ms | 0.018 | 0.025 | 0.25 | 119.3 |
| far/select/ms | 0.014 | 0.164 | 0.46 | 96.0 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.48 | 85.9 |
| anim/apply/ms | 0.011 | 0.020 | 0.22 | 75.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.37 | 71.6 |
| weather/eval/ms | 0.011 | 0.014 | 0.08 | 70.7 |
| script/update/ms | 0.008 | 0.012 | 0.07 | 53.4 |
| controller/avoid/ms | 0.006 | 0.012 | 0.17 | 39.1 |
| stream/despawn/ms | 0.005 | 0.000 | 0.96 | 35.8 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 32.2 |
| script/ui/ms | 0.004 | 0.005 | 0.03 | 23.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.11 | 21.1 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.32 | 20.2 |
| stream/upload/ms | 0.003 | 0.000 | 1.00 | 18.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 7.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.19 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.6 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 4.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 16 |
| controller/movers/count | count | 19 | 21 | 16 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 357 | 646 | 162 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5539 | 5942 | 5613 |
| crowd/c1/meshes/count | count | 1181 | 1272 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 205 |
| crowd/joints_extracted/count | count | 672 | 7026 | 410 |
| crowd/joints_uploaded/count | count | 3607 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.39 | 1.70 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.48 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.37 | 0.01 |
| encode/finish_critical | ms | 0.97 | 2.67 | 1.11 |
| encode/pass:early prepass/max | ms | 0.82 | 2.16 | 0.89 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 2.05 | 1.11 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.75 | 0.16 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.54 | 0.46 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.83 | 2.00 | 0.77 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.32 | 1.61 | 0.17 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.09 | 2.67 | 0.84 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.25 | 0.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.48 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.52 | 25.43 | 8.77 |
| gpu/timed/ms | ms | 7.83 | 9.40 | 8.77 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 318 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 318 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 767 | 973 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2479 | 3555 | 2455 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 767 | 973 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2479 | 3555 | 2455 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 74 | 446 | 8 |
| phases/shadow/bins | count | 837 | 1558 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2302 | 3300 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 49 | 154 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2679 | 3256 | 2345 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 161 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 15 | 262 | 11 |
| preprocess/bind_groups_created | count | 10 | 256 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 135 | 223 | 123 |
| process/cpu_usage | % | 25.29 | 40.18 | 34.45 |
| process/mem_usage | GiB | 5.10 | 5.40 | 5.40 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 2 | 0 |
| shadow_atlas/lamps | count | 8 | 19 | 0 |
| shadow_atlas/live_occ_pct | % | 4.49 | 37.60 | 0 |
| shadow_atlas/live_tiles | count | 17 | 70 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 162 | 499 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 19 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 61 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 112 | 0 |
| shadow_atlas/views | count | 17 | 61 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 123 | 10 |
| skin/blocks_skipped/count | count | 145 | 207 | 195 |
| skin/blocks_walked/count | count | 14 | 25 | 10 |
| skin/joint_bytes_written/bytes | bytes | 47.6 k | 807.7 k | 34.0 k |
| skin/joints_written/count | count | 744 | 12.6 k | 532 |
| skin/upload_writes/count | count | 7 | 24 | 11 |
| started_load_count |  | 6439 | 6563 | 6563 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.53 | 53.94 | 45.88 |
| system/mem_usage | % | 53.11 | 54.38 | 54.38 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 132 | 505 | 66 |
| views/all/bins | count | 2934 | 3563 | 2948 |
| views/all/count | count | 20 | 64 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8009 | 10.5 k | 7962 |
| views/all/pipelines | count | 96 | 199 | 66 |
| views/all/pipelines_distinct | count | 65 | 73 | 61 |
| views/all/work_items | count | 154.1 k | 231.0 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 713 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2121 | 2427 | 2022 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 61 | 414 | 0 |
| views/lamp_face/bins | count | 98 | 720 | 0 |
| views/lamp_face/count | count | 17 | 61 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 135 | 880 | 0 |
| views/lamp_face/pipelines | count | 25 | 128 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 108 | 25.1 k | 0 |
| views/main/batch_sets | count | 57 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5906 | 7530 | 5940 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 104.6 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.3 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.76 | 237 | 319.7 k |
| anim/events/count | count | 0.08 | 3 | 516 |
| assets/Image/added | count | 0.09 | 33 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 255 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7475 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1168 | 1489 |
| encode//count | count | 27.90 | 30 | 186.7 k |
| encode/bind_walk/init_textures | count | 9086 | 23.1 k | 60.81 M |
| encode/bind_walk/init_textures_skipped | count | 54.5 k | 78.4 k | 364.77 M |
| encode/bind_walk/inits | count | 561 | 770 | 3.76 M |
| encode/bind_walk/inits_skipped | count | 174 | 256 | 1.16 M |
| encode/bind_walk/merge_textures | count | 9023 | 23.0 k | 60.39 M |
| encode/bind_walk/merge_textures_skipped | count | 9357 | 21.2 k | 62.63 M |
| encode/bind_walk/merges | count | 104 | 178 | 697.4 k |
| encode/bind_walk/merges_skipped | count | 152 | 228 | 1.02 M |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6693 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6693 |
| encode/pass:early prepass/count | count | 1 | 1 | 6693 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6693 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6693 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6693 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.28 | 1 | 1903 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.24 | 1 | 1601 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 312 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6693 |
| encode/queue/command_buffers | count | 38.48 | 42 | 257.6 k |
| encode/queue/submits | count | 2.01 | 4 | 13.5 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.31 M | 15.53 G |
| encode/queue/write_buffer/calls | count | 270 | 449 | 1.81 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.8 k | 1.29 M | 1.14 G |
| encode/queue/write_buffer_with/calls | count | 10.27 | 774 | 68.8 k |
| encode/queue/write_texture/bytes | bytes | 1971 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6693 |
| physics/inserts | count | 0.33 | 105 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
| physics/steps | count | 0.59 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 1 | 9 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 35 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5058 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1109 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.44 | 2443.6 | main/busy/ms 2414, stage/main/post_update 2372, stage/main/update 31.70, stage/main/pre_update 7.14, stage/main/last 1.06, surface/advance_cpu_ms 0.83, stage/main/first 0.57, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.34, surface/cell_upload_ms 0.31, bigspace/total/ms 0.10, stream/tick/ms 0.08 |
| warm-up | 3.17 | 727.4 | main/busy/ms 2547, stage/main/post_update 2394, render/thread/ms 692, render/busy/ms 563, stage/render/prepare_assets 514, stage/main/update 136, stage/render/prepare_views 127, render/acquire/ms 126, stage/render/create_views 29.41, stage/render/render 10.88, stage/main/pre_update 9.18, render/graph/ms 7.29 |
| warm-up | 3.28 | 104.2 | render/thread/ms 779, render/busy/ms 600, stage/render/prepare_assets 538, main/busy/ms 151, stage/render/prepare_views 127, render/acquire/ms 127, pipelines/compile_ms 118, stage/main/update 109, pipelines/front_ms 81.71, stage/render/render 66.02, render/present/ms 52.31, pipelines/driver_ms 36.06 |
| warm-up | 3.30 | 25.3 | pipelines/compile_ms 385, pipelines/front_ms 316, render/thread/ms 111, pipelines/wait_ms 79.22, stage/render/render 73.56, pipelines/driver_ms 68.86, main/busy/ms 64.44, render/busy/ms 58.00, render/present/ms 52.55, stage/main/post_update 36.02, stage/render/prepare_assets 24.71, terrain/update/ms 21.71 |
| warm-up | 3.48 | 175.8 | pipelines/wait_ms 4904, pipelines/compile_ms 1841, pipelines/front_ms 1723, render/thread/ms 175, render/busy/ms 169, main/busy/ms 148, pipelines/driver_ms 118, stage/main/update 84.35, encode//sum 56.04, stage/render/render 51.33, render/graph/ms 46.61, stage/main/post_update 37.32 |
| warm-up | 3.54 | 66.0 | pipelines/wait_ms 5002, pipelines/compile_ms 2427, pipelines/front_ms 2303, render/thread/ms 214, render/busy/ms 210, main/busy/ms 126, pipelines/driver_ms 123, stage/render/prepare_assets 80.56, stage/main/update 76.12, encode//sum 58.18, stage/render/render 43.67, render/graph/ms 41.08 |
| warm-up | 3.58 | 35.6 | pipelines/compile_ms 934, pipelines/front_ms 890, pipelines/wait_ms 179, render/thread/ms 95.99, render/busy/ms 93.92, stage/render/prepare_assets 70.62, pipelines/driver_ms 43.32, main/busy/ms 37.48, stage/main/post_update 16.31, terrain/update/ms 14.99, stage/render/render 14.68, stage/main/update 13.59 |
| warm-up | 3.61 | 32.0 | pipelines/compile_ms 825, pipelines/front_ms 781, pipelines/wait_ms 113, render/thread/ms 62.88, render/busy/ms 61.41, stage/render/prepare_assets 44.64, pipelines/driver_ms 43.66, main/busy/ms 40.32, stage/main/post_update 15.50, stage/main/update 14.08, terrain/update/ms 9.61, stage/render/render 8.12 |
| warm-up | 3.65 | 37.0 | pipelines/compile_ms 783, pipelines/front_ms 742, pipelines/wait_ms 112, render/thread/ms 65.51, render/busy/ms 63.75, main/busy/ms 60.11, pipelines/driver_ms 40.90, stage/main/post_update 32.75, stage/render/prepare_assets 22.63, encode//sum 21.60, stage/main/update 17.51, stage/render/render 16.46 |
| warm-up | 3.69 | 39.9 | pipelines/compile_ms 465, pipelines/front_ms 449, main/busy/ms 103, render/thread/ms 73.15, render/busy/ms 71.30, stage/main/update 62.45, stage/main/post_update 33.50, stage/render/render 22.32, encode//sum 22.21, stage/render/prepare_assets 20.28, render/graph/ms 20.10, stage/render/graph_submit 17.26 |
| warm-up | 3.79 | 101.6 | pipelines/compile_ms 1950, pipelines/front_ms 1867, pipelines/wait_ms 133, main/busy/ms 83.22, pipelines/driver_ms 82.59, render/thread/ms 78.52, render/busy/ms 76.67, stage/main/update 58.79, stage/render/prepare_assets 48.23, stage/render/render 16.71, stage/main/post_update 16.00, render/graph/ms 14.18 |
| warm-up | 3.84 | 47.0 | pipelines/compile_ms 2162, pipelines/front_ms 2080, pipelines/wait_ms 143, pipelines/driver_ms 82.03, render/thread/ms 70.85, render/busy/ms 68.68, stage/render/prepare_assets 49.37, main/busy/ms 22.17, stage/render/render 13.69, render/graph/ms 11.11, terrain/update/ms 9.32, stage/main/update 8.62 |
| warm-up | 3.91 | 39.5 | pipelines/compile_ms 86.51, pipelines/front_ms 81.64, render/thread/ms 44.54, render/busy/ms 42.92, main/busy/ms 25.45, stage/render/prepare_resources 18.44, stage/main/update 12.98, stage/render/render 12.87, render/graph/ms 10.57, stage/main/post_update 9.17, stage/render/graph_submit 7.23, encode//sum 6.68 |
| warm-up | 4.13 | 205.0 | pipelines/compile_ms 1763, pipelines/front_ms 1723, main/busy/ms 119, stage/main/update 103, render/thread/ms 94.77, render/busy/ms 92.88, stage/render/prepare_assets 74.98, pipelines/driver_ms 39.71, stage/render/render 13.99, render/graph/ms 11.69, stage/render/graph_submit 8.70, encode//sum 8.04 |
| warm-up | 4.21 | 50.0 | pipelines/compile_ms 202, pipelines/front_ms 190, main/busy/ms 87.09, stage/main/update 67.21, render/thread/ms 59.03, render/busy/ms 56.55, stage/render/render 36.02, render/graph/ms 32.89, pipelines/wait_ms 28.43, stage/render/graph_submit 20.88, stage/main/post_update 13.39, pipelines/driver_ms 12.19 |
| warm-up | 4.29 | 87.0 | pipelines/compile_ms 184, pipelines/front_ms 172, render/thread/ms 82.96, render/busy/ms 81.07, main/busy/ms 76.67, stage/main/update 60.07, stage/render/prepare_assets 34.67, stage/render/render 29.74, render/graph/ms 27.31, pipelines/wait_ms 26.39, stage/render/graph_submit 15.44, stage/render/camera_driver 11.67 |
| warm-up | 4.37 | 75.5 | pipelines/compile_ms 1860, pipelines/front_ms 1798, pipelines/wait_ms 211, render/thread/ms 117, render/busy/ms 115, main/busy/ms 64.61, pipelines/driver_ms 62.56, stage/render/prepare_assets 41.88, stage/main/update 36.73, stage/render/prepare_write_phase_buffers 36.63, stage/main/pre_update 14.68, stage/render/render 12.66 |
| warm-up | 4.40 | 30.3 | pipelines/compile_ms 2437, pipelines/front_ms 2356, pipelines/wait_ms 212, render/thread/ms 100, render/busy/ms 98.18, pipelines/driver_ms 80.74, main/busy/ms 63.35, stage/render/prepare_write_phase_buffers 37.49, stage/main/update 36.51, stage/render/render 19.03, stage/render/prepare_assets 18.77, render/graph/ms 16.10 |
| warm-up | 4.49 | 89.8 | pipelines/compile_ms 751, pipelines/front_ms 721, render/thread/ms 107, render/busy/ms 105, main/busy/ms 94.18, stage/main/update 46.89, pipelines/wait_ms 45.51, stage/main/post_update 32.46, stage/render/prepare_assets 31.14, pipelines/driver_ms 30.17, stage/render/render 22.97, render/graph/ms 19.89 |
| warm-up | 4.60 | 112.2 | pipelines/compile_ms 2386, pipelines/front_ms 2313, pipelines/wait_ms 267, render/thread/ms 157, render/busy/ms 155, main/busy/ms 97.61, pipelines/driver_ms 72.76, stage/render/prepare_assets 60.37, stage/main/update 48.84, stage/render/render 45.13, render/graph/ms 42.60, stage/render/graph_submit 38.73 |
| warm-up | 4.66 | 55.3 | pipelines/compile_ms 2445, pipelines/front_ms 2380, pipelines/wait_ms 242, render/thread/ms 123, render/busy/ms 121, main/busy/ms 81.59, pipelines/driver_ms 65.56, stage/main/update 60.09, stage/render/render 56.76, render/graph/ms 53.69, stage/render/prepare_assets 49.29, stage/render/graph_submit 38.14 |
| warm-up | 4.74 | 88.9 | pipelines/compile_ms 613, pipelines/front_ms 598, render/thread/ms 97.77, render/busy/ms 95.79, main/busy/ms 87.59, stage/main/update 65.27, pipelines/wait_ms 42.28, stage/render/prepare_assets 40.35, stage/render/render 30.16, render/graph/ms 27.34, pipelines/driver_ms 15.26, pipelines/lock_ms 14.96 |
| warm-up | 4.78 | 36.0 | pipelines/compile_ms 476, pipelines/front_ms 460, main/busy/ms 90.10, render/thread/ms 84.50, render/busy/ms 81.94, stage/main/update 64.87, pipelines/wait_ms 46.46, stream/drive/ms 35.23, stage/render/prepare_assets 32.62, stage/render/render 17.29, pipelines/driver_ms 15.28, pipelines/lock_ms 14.82 |
| warm-up | 4.85 | 70.5 | pipelines/compile_ms 1786, pipelines/front_ms 1735, main/busy/ms 107, stage/main/update 83.79, stream/drive/ms 59.47, render/thread/ms 58.54, pipelines/wait_ms 57.62, render/busy/ms 56.04, pipelines/driver_ms 50.97, pipelines/lock_ms 35.19, stage/render/prepare_assets 18.00, stage/render/render 15.55 |
| warm-up | 4.89 | 44.0 | pipelines/compile_ms 1775, pipelines/front_ms 1726, main/busy/ms 74.20, stage/main/update 58.20, render/thread/ms 51.65, pipelines/driver_ms 49.52, render/busy/ms 49.49, stream/drive/ms 36.73, pipelines/wait_ms 35.88, pipelines/lock_ms 35.22, stage/render/render 19.27, render/graph/ms 16.65 |
| warm-up | 4.94 | 45.7 | pipelines/compile_ms 323, pipelines/front_ms 312, render/thread/ms 51.92, main/busy/ms 50.21, render/busy/ms 49.75, stage/main/update 34.77, stage/render/render 18.78, stage/render/prepare_assets 17.70, render/graph/ms 16.19, stream/drive/ms 13.00, stage/render/graph_submit 12.13, pipelines/driver_ms 11.30 |
| warm-up | 4.98 | 27.4 | pipelines/compile_ms 407, pipelines/front_ms 395, render/thread/ms 39.69, main/busy/ms 38.16, render/busy/ms 37.88, stage/main/update 20.22, pipelines/wait_ms 14.94, stage/render/render 14.74, render/graph/ms 12.83, pipelines/driver_ms 12.77, stage/main/post_update 12.09, encode//sum 9.56 |
| warm-up | 5.01 | 26.6 | pipelines/compile_ms 45.49, render/thread/ms 45.42, render/busy/ms 43.61, pipelines/front_ms 43.06, main/busy/ms 39.92, stage/main/update 19.66, stage/main/post_update 14.10, stage/render/render 13.37, stage/render/prepare_assets 11.18, render/graph/ms 11.17, encode//sum 9.51, stage/render/graph_submit 7.11 |
| warm-up | 5.10 | 26.8 | pipelines/compile_ms 119, pipelines/front_ms 116, main/busy/ms 37.61, render/thread/ms 35.05, render/busy/ms 33.60, stage/main/update 23.07, stage/render/render 15.44, stream/drive/ms 13.62, render/graph/ms 13.61, stage/render/graph_submit 10.27, stage/main/post_update 9.29, encode//sum 6.73 |
| warm-up | 5.14 | 36.8 | pipelines/compile_ms 246, pipelines/front_ms 237, render/thread/ms 51.25, main/busy/ms 50.25, render/busy/ms 49.90, stage/main/update 25.79, stage/render/render 16.41, render/graph/ms 14.69, stage/main/post_update 13.20, stream/drive/ms 12.78, stage/render/graph_submit 10.53, stage/render/prepare_assets 9.02 |
| warm-up | 5.18 | 25.3 | render/thread/ms 37.95, render/busy/ms 36.79, pipelines/compile_ms 36.28, pipelines/front_ms 34.97, main/busy/ms 29.85, pipelines/wait_ms 26.54, stage/render/render 14.25, stage/main/post_update 13.16, render/graph/ms 12.39, stage/main/update 9.33, stage/render/graph_submit 8.45, encode//sum 7.09 |
| warm-up | 5.73 | 25.9 | render/thread/ms 39.07, render/busy/ms 37.54, main/busy/ms 29.88, pipelines/compile_ms 16.61, stage/render/render 15.51, pipelines/front_ms 15.41, render/graph/ms 13.39, stage/main/update 12.54, stage/main/post_update 11.63, stage/render/prepare_assets 11.09, encode//sum 10.33, stage/render/graph_submit 9.58 |
| warm-up | 5.93 | 27.4 | main/busy/ms 33.23, render/thread/ms 32.92, render/busy/ms 31.77, stage/main/update 20.46, pipelines/compile_ms 12.74, stage/render/render 12.65, pipelines/front_ms 11.76, render/graph/ms 11.17, encode//sum 8.11, stage/main/post_update 8.03, stage/render/prepare_assets 7.68, stage/render/graph_submit 7.59 |
| warm-up | 5.99 | 29.7 | main/busy/ms 38.32, render/thread/ms 37.64, render/busy/ms 36.25, stage/main/update 19.91, stage/render/render 14.82, stage/main/post_update 13.51, render/graph/ms 12.83, encode//sum 10.01, stage/render/graph_submit 7.89, stage/render/camera_driver 4.68, stage/render/prepare_assets 4.64, encode/submit 4.57 |
| warm-up | 6.02 | 33.0 | render/thread/ms 51.68, render/busy/ms 50.25, main/busy/ms 42.73, stage/main/update 23.16, stage/render/render 17.46, render/graph/ms 15.39, stage/render/prepare_assets 12.39, stage/main/post_update 12.07, stage/render/graph_submit 10.87, encode//sum 8.75, encode/finish_wall 6.40, stage/render/prepare_meshes 5.69 |
| warm-up | 6.06 | 38.1 | pipelines/compile_ms 134, pipelines/front_ms 132, render/thread/ms 52.62, render/busy/ms 51.70, main/busy/ms 36.66, stage/render/prepare_assets 20.82, stage/main/update 20.28, stage/render/render 14.77, render/graph/ms 13.38, stage/render/graph_submit 10.58, stage/main/post_update 8.27, encode/finish_wall 7.58 |
| warm-up | 6.09 | 33.2 | pipelines/compile_ms 151, pipelines/front_ms 148, render/thread/ms 57.96, render/busy/ms 56.78, main/busy/ms 37.50, stage/render/render 25.05, render/graph/ms 23.37, stage/render/graph_submit 19.21, stage/main/update 17.71, stage/render/prepare_assets 15.35, stage/main/post_update 13.69, encode/finish_wall 10.45 |
| warm-up | 6.15 | 34.9 | render/thread/ms 42.58, render/busy/ms 41.60, main/busy/ms 30.32, stage/main/update 16.67, stage/render/prepare_assets 16.22, stage/render/render 13.38, render/graph/ms 11.85, stage/render/graph_submit 9.31, stage/main/post_update 8.49, encode/finish_wall 6.69, encode//sum 6.62, encode/pass:point_shadow_atlas_live_indirect/sum 6.51 |
| warm-up | 6.30 | 42.3 | main/busy/ms 54.95, render/thread/ms 41.12, render/busy/ms 39.44, stage/main/update 38.70, stage/render/render 18.45, render/graph/ms 15.96, stream/drive/ms 15.79, stage/render/graph_submit 12.57, stage/main/post_update 11.75, pipelines/compile_ms 10.66, pipelines/front_ms 10.04, stage/render/prepare_assets 8.46 |
| warm-up | 6.34 | 34.4 | render/thread/ms 47.31, render/busy/ms 45.95, main/busy/ms 42.48, stage/main/update 26.35, stage/render/prepare_assets 16.54, stage/render/render 16.00, render/graph/ms 13.67, stage/main/post_update 10.34, stage/render/graph_submit 10.21, encode//sum 8.47, stream/drive/ms 6.46, encode/submit 5.16 |
| warm-up | 6.36 | 26.1 | render/thread/ms 41.24, render/busy/ms 40.41, main/busy/ms 36.29, stage/main/update 16.88, stage/render/prepare_assets 16.84, stage/main/post_update 11.38, stage/render/render 11.35, render/graph/ms 9.76, stage/render/graph_submit 7.16, encode//sum 6.66, stage/render/prepare_meshes 5.17, stream/drive/ms 4.83 |
| measure | 18.29 | 27.7 | main/busy/ms 34.86, stage/main/update 22.00, render/thread/ms 20.84, render/busy/ms 19.88, stage/render/render 10.11, render/graph/ms 8.89, stage/main/post_update 8.49, stage/render/graph_submit 5.93, encode//sum 5.63, encode/submit 3.86, stage/render/prepare_assets 3.71, stage/render/camera_driver 2.83 |
| measure | 59.06 | 31.9 | render/thread/ms 37.70, render/busy/ms 36.51, main/busy/ms 26.84, stage/render/prepare_assets 16.37, stage/main/post_update 12.01, stage/render/render 11.22, render/graph/ms 9.76, encode//sum 7.66, stage/render/graph_submit 6.60, stage/main/update 6.44, stage/main/fixed_main_loop 4.60, encode/submit 3.97 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.soft_pcf_radius=0 --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=pcf0-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **86%** (min 63%) · 2619 MHz (min 2610) · memory 8101 MHz · 86.1 W · 80 °C (max 85) · VRAM 5640 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set)`
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
