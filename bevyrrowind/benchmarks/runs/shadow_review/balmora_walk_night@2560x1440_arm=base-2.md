# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6669 frames over 65.6 s · **median 9.72 ms (103 fps)** · mean 9.84 · p95 11.52 · p99 13.25 (1% low 75 fps) · worst 34.85 ms · 32 spikes over 1.5× median · 29.2 stutters/min (work 23.8/min)
- warm-up: 353 frames over 9.2 s, worst 2532.1 ms
- hitches (> 25.0 ms): **2** measured, 48 in warm-up
- timed GPU passes: 6.45 ms of a 9.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.72 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.25/min, limit 1/min
- **FAIL** worst frame 34.85 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.845 | 2.039 | 2.074 | 0.90 | 0.175 |
| main_opaque_pass_3d | 1.496 | 1.653 | 2.377 | 0.90 | 0.062 |
| early prepass | 0.466 | 0.515 | 0.754 | 0.90 | 0.088 |
| bloom | 0.288 | 0.318 | 0.325 | 0.90 | 0.063 |
| atmosphere_luts | 0.286 | 0.316 | 0.380 | 0.90 | 0.003 |
| water_velocity | 0.204 | 0.226 | 0.247 | 0.90 | 0.002 |
| water_sspr | 0.183 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.171 | 0.189 | 0.193 | 0.90 | 0.007 |
| point_shadow_atlas | 0.149 | 0.275 | 0.481 | 0.54 | 0.236 |
| upscaling | 0.117 | 0.130 | 0.136 | 0.90 | 0.004 |
| atmos_froxel_inject | 0.116 | 0.128 | 0.130 | 0.90 | 0.001 |
| water_fft | 0.112 | 0.124 | 0.127 | 0.90 | 0.003 |
| uniform_allocation | 0.107 | 0.118 | 0.142 | 0.90 | 0.020 |
| clouds_composite | 0.095 | 0.105 | 0.109 | 0.90 | 0.002 |
| gpu_instance_materialization | 0.087 | 0.096 | 0.100 | 0.90 | 0.004 |
| bin_unpacking | 0.084 | 0.093 | 0.115 | 0.90 | 0.016 |
| tonemapping | 0.079 | 0.087 | 0.092 | 0.90 | 0.004 |
| main_transmissive_pass_3d | 0.063 | 0.070 | 0.133 | 0.90 | 0.008 |
| clustering | 0.059 | 0.065 | 0.068 | 0.90 | 0.019 |
| clouds_trace | 0.050 | 0.056 | 0.078 | 0.90 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.053 | 0.070 | 0.90 | 0.003 |
| render_sky | 0.048 | 0.053 | 0.054 | 0.90 | 0.002 |
| clouds_resolve | 0.045 | 0.050 | 0.051 | 0.90 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.59 | 0.002 |
| lightprobe_irradiance_map | 0.033 | 0.415 | 0.417 | 0.08 | 0.001 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.90 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.027 | 0.050 | 0.070 | 0.54 | 0.009 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.90 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.90 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.028 | 0.90 | 0.003 |
| lightprobe_radiance_map | 0.017 | 0.220 | 0.230 | 0.08 | 0.004 |
| point_shadow_atlas/uniform_allocation | 0.016 | 0.029 | 0.059 | 0.54 | 0.008 |
| main_transparent_pass_3d | 0.015 | 0.017 | 0.030 | 0.90 | 0.180 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.017 | 0.026 | 0.90 | 0.005 |
| point_shadow_atlas/bin_unpacking | 0.011 | 0.021 | 0.037 | 0.54 | 0.006 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.90 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.90 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.54 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.90 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.666 | 9.977 | 18.87 | 57792.4 |
| render/busy/ms | 8.144 | 9.434 | 18.35 | 54314.2 |
| main/busy/ms | 8.012 | 9.201 | 30.03 | 53435.3 |
| stage/render/render | 5.051 | 6.052 | 11.44 | 33687.2 |
| stage/main/post_update | 4.526 | 5.334 | 9.54 | 30182.6 |
| render/graph/ms | 4.379 | 5.253 | 10.08 | 29206.5 |
| encode//sum | 3.195 | 3.942 | 6.74 | 21306.0 |
| stage/render/graph_submit | 3.085 | 3.881 | 6.89 | 20571.3 |
| encode/submit | 1.958 | 2.833 | 4.68 | 13056.6 |
| stage/main/update | 1.813 | 2.287 | 24.09 | 12090.2 |
| stage/render/camera_driver | 1.233 | 1.490 | 3.08 | 8221.8 |
| encode/finish_wall | 1.210 | 1.564 | 2.91 | 8069.8 |
| encode/pass:early prepass/sum | 0.821 | 1.063 | 2.17 | 5475.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.818 | 1.076 | 1.84 | 5458.0 |
| encode/pass:shadow_directional_light/sum | 0.807 | 1.216 | 1.81 | 5382.5 |
| stage/render/prepare_write_phase_buffers | 0.611 | 0.909 | 1.75 | 4077.5 |
| bigspace/total/ms | 0.556 | 1.304 | 2.57 | 3711.3 |
| stage/render/prepare_assets | 0.501 | 0.644 | 9.79 | 3343.4 |
| stage/main/fixed_main_loop | 0.493 | 0.926 | 1.92 | 3290.6 |
| render/present/ms | 0.425 | 1.185 | 1.95 | 2837.6 |
| stage/render/prepare_meshes | 0.422 | 0.525 | 3.42 | 2811.4 |
| stage/render/prepare_resources | 0.418 | 0.494 | 0.86 | 2786.1 |
| stage/main/pre_update | 0.385 | 0.552 | 2.98 | 2568.8 |
| stage/main/first | 0.366 | 0.485 | 5.89 | 2439.6 |
| stage/render/prepare_flush | 0.321 | 0.528 | 1.57 | 2143.6 |
| controller/feed/ms | 0.317 | 0.400 | 0.78 | 2115.9 |
| stage/render/prepare_bind_groups | 0.298 | 0.493 | 1.46 | 1990.4 |
| stage/render/prepare_batch_phases | 0.267 | 0.403 | 0.74 | 1779.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.259 | 0.427 | 1.14 | 1728.2 |
| stage/main/last | 0.252 | 0.298 | 0.95 | 1677.5 |
| bigspace/low_precision/ms | 0.248 | 0.778 | 1.83 | 1655.7 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.240 | 1.021 | 2.04 | 1603.3 |
| stage/render/create_views | 0.200 | 0.270 | 0.73 | 1332.1 |
| stream/drive/ms | 0.198 | 0.278 | 10.71 | 1317.8 |
| stage/render/extract_commands | 0.183 | 0.214 | 0.50 | 1221.9 |
| bigspace/high_precision/ms | 0.162 | 0.648 | 1.86 | 1080.2 |
| anim/evaluate/ms | 0.161 | 0.273 | 1.53 | 1073.1 |
| anim/evaluate_cpu/ms | 0.155 | 0.195 | 0.64 | 1035.5 |
| stage/render/specialize | 0.150 | 0.238 | 2.03 | 1001.6 |
| stream/tick/ms | 0.145 | 0.213 | 0.50 | 969.5 |
| stage/render/prepare_views | 0.138 | 0.177 | 1.56 | 918.4 |
| stage/render/queue | 0.096 | 0.165 | 2.22 | 640.8 |
| render/acquire/ms | 0.096 | 0.124 | 1.53 | 640.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.089 | 0.141 | 0.67 | 593.7 |
| stage/render/phase_sort | 0.081 | 0.110 | 0.42 | 538.0 |
| controller/write/ms | 0.073 | 0.100 | 0.39 | 486.7 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.072 | 0.348 | 1.17 | 481.5 |
| controller/move/ms | 0.067 | 0.227 | 0.55 | 448.3 |
| terrain/update/ms | 0.059 | 0.290 | 1.68 | 394.5 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.056 | 0.000 | 2.77 | 372.8 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.073 | 0.29 | 307.1 |
| physics/step/ms | 0.039 | 0.086 | 0.52 | 259.8 |
| encode/resolve_diagnostics/sum | 0.033 | 0.045 | 0.59 | 216.9 |
| pipelines/compile_ms | 0.024 | 0.000 | 76.03 | 160.3 |
| physics/insert/ms | 0.024 | 0.261 | 0.83 | 157.2 |
| stage/render/cleanup | 0.023 | 0.032 | 0.41 | 155.5 |
| pipelines/front_ms | 0.023 | 0.000 | 73.37 | 154.6 |
| terrain/select/ms | 0.021 | 0.256 | 0.74 | 143.2 |
| weather/map/ms | 0.017 | 0.024 | 0.12 | 114.3 |
| far/select/ms | 0.014 | 0.164 | 0.51 | 94.9 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.42 | 83.4 |
| anim/apply/ms | 0.011 | 0.020 | 0.22 | 76.7 |
| encode/camera_driver_span_end/sum | 0.010 | 0.020 | 0.46 | 69.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.06 | 68.9 |
| script/update/ms | 0.008 | 0.011 | 0.10 | 51.6 |
| controller/avoid/ms | 0.006 | 0.012 | 0.20 | 40.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.22 | 35.3 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 31.2 |
| stream/spawn/ms | 0.003 | 0.000 | 1.12 | 22.6 |
| script/ui/ms | 0.003 | 0.005 | 0.14 | 22.5 |
| stream/upload/ms | 0.003 | 0.000 | 1.05 | 18.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.59 | 13.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 6.7 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.84 | 5.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.2 |
| stage/render/prepare | 0.000 | 0.002 | 0.03 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.5 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5507 | 5947 | 5613 |
| crowd/c1/meshes/count | count | 1166 | 1282 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 206 |
| crowd/joints_extracted/count | count | 689 | 5506 | 586 |
| crowd/joints_uploaded/count | count | 3607 | 6220 | 5807 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.39 | 1.41 | 0.65 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.42 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.46 | 0.02 |
| encode/finish_critical | ms | 0.97 | 2.77 | 1.03 |
| encode/pass:early prepass/max | ms | 0.82 | 2.17 | 0.83 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 1.84 | 1.03 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.67 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.14 | 0.26 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.80 | 2.04 | 0.51 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.26 | 1.17 | 0.16 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.04 | 2.77 | 1.00 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 1.81 | 0.53 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.53 | 30.03 | 9.06 |
| gpu/timed/ms | ms | 8.04 | 9.26 | 9.06 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
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
| phases/opaque3d/bins | count | 765 | 971 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2485 | 3527 | 2457 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 971 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2485 | 3527 | 2457 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 56 | 431 | 8 |
| phases/shadow/bins | count | 777 | 1337 | 671 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2273 | 3090 | 2035 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 46 | 133 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2679 | 3183 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 15 | 267 | 11 |
| preprocess/bind_groups_created | count | 10 | 257 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 141 | 217 | 123 |
| process/cpu_usage | % | 25 | 37.98 | 24.76 |
| process/mem_usage | GiB | 5.12 | 5.39 | 5.39 |
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
| shadow_atlas/lamps | count | 9 | 20 | 0 |
| shadow_atlas/live_occ_pct | % | 3.42 | 26.27 | 0 |
| shadow_atlas/live_tiles | count | 12 | 51 | 0 |
| shadow_atlas/mover_draws | count | 9 | 39 | 0 |
| shadow_atlas/movers | count | 145 | 541 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 2 | 0 |
| shadow_atlas/render/blits | count | 9 | 39 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 9 | 20 | 0 |
| shadow_atlas/render/live_direct | count | 9 | 39 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 16 | 56 | 0 |
| shadow_atlas/resizes | count | 33 | 43 | 43 |
| shadow_atlas/static_occ_pct | % | 16.41 | 39.36 | 0 |
| shadow_atlas/static_tiles | count | 42 | 118 | 0 |
| shadow_atlas/views | count | 16 | 56 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 93 | 13 |
| skin/blocks_skipped/count | count | 144 | 207 | 193 |
| skin/blocks_walked/count | count | 14 | 25 | 13 |
| skin/joint_bytes_written/bytes | bytes | 49.4 k | 808.6 k | 42.5 k |
| skin/joints_written/count | count | 772 | 12.6 k | 664 |
| skin/upload_writes/count | count | 8 | 27 | 11 |
| started_load_count |  | 6441 | 6565 | 6565 |
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
| system/cpu_usage | % | 29.21 | 41.24 | 33.96 |
| system/mem_usage | % | 53.75 | 54.64 | 54.62 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 116 | 483 | 66 |
| views/all/bins | count | 2939 | 3378 | 2961 |
| views/all/count | count | 19 | 59 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8049 | 10.3 k | 7979 |
| views/all/pipelines | count | 93 | 182 | 66 |
| views/all/pipelines_distinct | count | 65 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 230.1 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 724 | 866 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2130 | 2457 | 2035 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.8 k | 49.2 k |
| views/lamp_face/batch_sets | count | 39 | 398 | 0 |
| views/lamp_face/bins | count | 49 | 536 | 0 |
| views/lamp_face/count | count | 16 | 56 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 86 | 724 | 0 |
| views/lamp_face/pipelines | count | 20 | 107 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 73 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2016 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7492 | 5944 |
| views/main/pipelines | count | 47 | 49 | 46 |
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
| anim/bones_written/count | count | 48.31 | 212 | 322.2 k |
| anim/events/count | count | 0.08 | 3 | 520 |
| assets/Image/added | count | 0.09 | 39 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 242 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7477 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1154 | 1489 |
| encode//count | count | 27.91 | 30 | 186.1 k |
| encode/bind_walk/init_textures | count | 8728 | 23.0 k | 58.21 M |
| encode/bind_walk/init_textures_skipped | count | 54.9 k | 83.7 k | 366.44 M |
| encode/bind_walk/inits | count | 570 | 773 | 3.80 M |
| encode/bind_walk/inits_skipped | count | 173 | 259 | 1.16 M |
| encode/bind_walk/merge_textures | count | 8665 | 22.9 k | 57.79 M |
| encode/bind_walk/merge_textures_skipped | count | 10.7 k | 26.5 k | 71.49 M |
| encode/bind_walk/merges | count | 106 | 181 | 707.8 k |
| encode/bind_walk/merges_skipped | count | 153 | 229 | 1.02 M |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6669 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6669 |
| encode/pass:early prepass/count | count | 1 | 1 | 6669 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6669 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6669 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6669 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.29 | 1 | 1928 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.23 | 1 | 1535 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 333 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6669 |
| encode/queue/command_buffers | count | 38.49 | 42 | 256.7 k |
| encode/queue/submits | count | 2.01 | 4 | 13.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.43 M | 15.51 G |
| encode/queue/write_buffer/calls | count | 278 | 450 | 1.85 M |
| encode/queue/write_buffer_with/bytes | bytes | 165.8 k | 1.30 M | 1.11 G |
| encode/queue/write_buffer_with/calls | count | 10.28 | 735 | 68.6 k |
| encode/queue/write_texture/bytes | bytes | 1821 | 589.8 k | 12.15 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 556 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6669 |
| physics/inserts | count | 0.34 | 106 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
| physics/steps | count | 0.59 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 26 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5076 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1113 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 50, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2532.1 | main/busy/ms 2504, stage/main/post_update 2459, stage/main/update 34.48, stage/main/pre_update 7.16, stage/main/last 1.15, surface/advance_cpu_ms 0.83, stage/main/first 0.59, stream/drive/ms 0.50, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.30, stream/tick/ms 0.13, bigspace/total/ms 0.10 |
| warm-up | 3.27 | 733.0 | main/busy/ms 2643, stage/main/post_update 2482, render/thread/ms 700, render/busy/ms 563, stage/render/prepare_assets 514, stage/main/update 144, stage/render/prepare_views 134, render/acquire/ms 134, stage/render/create_views 29.32, stage/render/render 10.64, stage/main/pre_update 9.26, render/graph/ms 6.73 |
| warm-up | 3.37 | 105.8 | render/thread/ms 789, render/busy/ms 600, stage/render/prepare_assets 537, main/busy/ms 163, stage/render/prepare_views 135, render/acquire/ms 134, stage/main/update 115, pipelines/compile_ms 106, stage/render/render 68.84, pipelines/front_ms 65.82, render/present/ms 54.49, pipelines/driver_ms 39.76 |
| warm-up | 3.55 | 162.8 | pipelines/wait_ms 4362, pipelines/compile_ms 1623, pipelines/front_ms 1455, pipelines/driver_ms 168, render/thread/ms 156, render/busy/ms 150, main/busy/ms 104, encode//sum 80.55, stage/render/render 61.01, stage/main/post_update 54.97, render/graph/ms 48.32, stage/render/graph_submit 41.94 |
| warm-up | 3.59 | 38.0 | pipelines/wait_ms 4779, pipelines/compile_ms 1730, pipelines/front_ms 1576, render/thread/ms 174, render/busy/ms 172, pipelines/driver_ms 155, main/busy/ms 139, stage/main/update 88.42, encode//sum 85.11, stage/render/render 55.66, render/graph/ms 46.98, stage/render/graph_submit 43.37 |
| warm-up | 3.70 | 111.9 | pipelines/compile_ms 1382, pipelines/front_ms 1323, pipelines/wait_ms 600, render/thread/ms 111, render/busy/ms 109, main/busy/ms 86.06, stage/render/prepare_assets 65.70, stage/main/update 59.47, pipelines/driver_ms 58.30, stage/main/post_update 19.41, stage/render/render 18.24, render/graph/ms 15.71 |
| warm-up | 3.75 | 44.3 | pipelines/compile_ms 1678, pipelines/front_ms 1596, pipelines/wait_ms 203, render/thread/ms 117, render/busy/ms 115, stage/render/prepare_assets 84.59, pipelines/driver_ms 82.16, main/busy/ms 40.27, stage/render/render 23.75, render/graph/ms 21.42, stage/main/post_update 18.45, encode//sum 14.14 |
| warm-up | 3.78 | 29.7 | pipelines/compile_ms 708, pipelines/front_ms 666, render/thread/ms 70.62, render/busy/ms 68.88, pipelines/wait_ms 46.89, stage/render/prepare_assets 43.84, pipelines/driver_ms 41.15, main/busy/ms 40.49, stage/main/post_update 18.61, stage/render/render 17.17, render/graph/ms 15.06, encode//sum 14.78 |
| warm-up | 3.80 | 25.2 | pipelines/compile_ms 878, pipelines/front_ms 845, pipelines/wait_ms 79.39, render/thread/ms 49.07, render/busy/ms 47.55, main/busy/ms 37.38, pipelines/driver_ms 32.74, stage/render/prepare_assets 23.37, stage/main/post_update 18.52, stage/main/update 13.17, stage/render/render 9.64, terrain/update/ms 8.55 |
| warm-up | 3.94 | 110.4 | pipelines/compile_ms 1245, pipelines/front_ms 1193, main/busy/ms 79.40, render/thread/ms 70.38, render/busy/ms 68.31, stage/main/update 56.78, pipelines/driver_ms 51.82, stage/render/prepare_assets 41.90, stage/render/render 17.59, stage/main/post_update 15.08, render/graph/ms 15.01, stage/render/graph_submit 12.07 |
| warm-up | 3.98 | 41.8 | pipelines/compile_ms 1654, pipelines/front_ms 1585, render/thread/ms 76.61, render/busy/ms 75.04, pipelines/driver_ms 68.83, stage/render/prepare_assets 56.54, main/busy/ms 31.13, stage/main/update 18.50, stage/render/render 13.19, render/graph/ms 11.28, stage/render/graph_submit 8.57, stage/main/post_update 6.92 |
| warm-up | 4.00 | 25.7 | pipelines/compile_ms 543, pipelines/front_ms 519, render/thread/ms 45.71, render/busy/ms 44.34, stage/render/prepare_assets 27.17, pipelines/driver_ms 24.19, main/busy/ms 21.77, stage/render/render 11.91, render/graph/ms 10.28, stage/main/update 9.85, stage/render/graph_submit 7.83, pipelines/wait_ms 6.18 |
| warm-up | 4.11 | 56.6 | pipelines/compile_ms 197, pipelines/front_ms 189, render/thread/ms 57.72, render/busy/ms 55.61, main/busy/ms 43.69, stage/main/update 26.55, stage/render/prepare_assets 24.50, stage/render/render 20.51, render/graph/ms 17.54, stage/main/post_update 12.95, stage/render/graph_submit 11.54, stream/drive/ms 9.95 |
| warm-up | 4.14 | 35.2 | pipelines/compile_ms 393, pipelines/front_ms 379, render/thread/ms 61.29, render/busy/ms 59.22, main/busy/ms 42.20, stage/render/prepare_assets 30.47, stage/main/update 20.27, stage/render/render 16.45, stage/main/post_update 15.01, pipelines/driver_ms 13.83, render/graph/ms 13.43, stage/render/graph_submit 9.68 |
| warm-up | 4.22 | 71.2 | pipelines/compile_ms 1300, pipelines/front_ms 1263, main/busy/ms 140, stage/main/update 120, render/thread/ms 62.91, render/busy/ms 61.35, stage/render/prepare_assets 42.31, pipelines/driver_ms 36.79, stage/main/post_update 12.60, stage/render/render 11.40, stream/drive/ms 10.18, render/graph/ms 9.45 |
| warm-up | 4.37 | 151.0 | pipelines/compile_ms 2014, pipelines/front_ms 1955, main/busy/ms 180, stage/main/update 165, render/thread/ms 95.22, render/busy/ms 84.97, stage/render/prepare_assets 59.96, pipelines/driver_ms 58.92, stage/render/render 25.39, render/graph/ms 14.76, stream/drive/ms 10.71, pipelines/wait_ms 10.40 |
| warm-up | 4.43 | 58.9 | pipelines/compile_ms 1041, pipelines/front_ms 1008, render/thread/ms 90.00, main/busy/ms 87.82, render/busy/ms 79.61, pipelines/wait_ms 74.36, stage/main/update 67.17, stage/render/prepare_assets 47.24, pipelines/driver_ms 33.39, stage/render/render 27.00, render/graph/ms 16.00, stage/main/post_update 12.58 |
| warm-up | 4.46 | 31.8 | pipelines/compile_ms 191, pipelines/front_ms 183, pipelines/wait_ms 83.66, render/thread/ms 63.91, render/busy/ms 62.40, main/busy/ms 48.20, stage/main/update 28.87, stage/render/render 21.93, stage/render/prepare_assets 20.89, stage/main/post_update 12.16, render/graph/ms 11.28, pipelines/driver_ms 8.52 |
| warm-up | 4.54 | 83.7 | pipelines/compile_ms 292, pipelines/front_ms 281, main/busy/ms 101, render/thread/ms 98.26, render/busy/ms 96.81, pipelines/wait_ms 76.13, stage/main/post_update 52.79, stage/main/update 28.70, stage/render/render 28.30, stage/render/prepare_resources 23.62, render/graph/ms 17.75, stage/render/prepare_assets 15.66 |
| warm-up | 4.61 | 72.7 | pipelines/compile_ms 2396, pipelines/front_ms 2303, pipelines/wait_ms 417, render/thread/ms 129, render/busy/ms 128, main/busy/ms 110, pipelines/driver_ms 92.86, stage/main/post_update 55.59, stage/render/prepare_resources 53.07, stage/main/update 22.66, stage/render/prepare_assets 20.98, stage/main/last 19.27 |
| warm-up | 4.67 | 55.4 | pipelines/compile_ms 2687, pipelines/front_ms 2583, pipelines/wait_ms 410, pipelines/driver_ms 104, render/thread/ms 95.43, render/busy/ms 94.10, main/busy/ms 66.09, stage/render/prepare_resources 31.74, stage/main/update 29.47, stage/render/prepare_assets 28.78, stage/main/post_update 12.99, stage/main/last 12.29 |
| warm-up | 4.75 | 82.2 | pipelines/compile_ms 1128, pipelines/front_ms 1091, pipelines/wait_ms 286, render/thread/ms 104, render/busy/ms 103, main/busy/ms 83.24, stage/render/prepare_assets 66.04, stage/main/update 56.01, pipelines/driver_ms 37.86, stage/render/render 17.47, render/graph/ms 15.37, stage/main/post_update 13.21 |
| warm-up | 4.81 | 61.4 | pipelines/compile_ms 1924, pipelines/front_ms 1871, pipelines/wait_ms 1104, render/thread/ms 111, render/busy/ms 109, main/busy/ms 109, stage/main/update 84.41, stage/render/prepare_assets 66.11, pipelines/driver_ms 52.78, stage/render/render 21.57, render/graph/ms 19.04, stage/render/graph_submit 13.99 |
| warm-up | 4.93 | 122.4 | pipelines/compile_ms 2283, pipelines/front_ms 2218, pipelines/wait_ms 894, main/busy/ms 132, render/thread/ms 124, render/busy/ms 123, stage/main/update 102, stage/render/prepare_assets 92.92, pipelines/driver_ms 65.46, stream/drive/ms 54.92, stage/render/render 16.49, stage/main/post_update 15.57 |
| warm-up | 4.97 | 37.6 | pipelines/compile_ms 1414, pipelines/front_ms 1368, render/thread/ms 115, render/busy/ms 113, main/busy/ms 100, stage/render/prepare_assets 81.65, stage/main/update 70.20, stream/drive/ms 54.96, pipelines/driver_ms 46.34, pipelines/wait_ms 23.68, stage/main/post_update 16.01, stage/render/render 12.72 |
| warm-up | 5.00 | 26.8 | pipelines/compile_ms 665, pipelines/front_ms 643, render/thread/ms 56.81, render/busy/ms 55.06, main/busy/ms 53.94, stage/main/update 34.32, pipelines/driver_ms 21.77, stage/render/render 21.24, render/graph/ms 18.84, stage/render/graph_submit 15.20, stage/render/prepare_assets 11.26, stage/main/post_update 10.60 |
| warm-up | 5.04 | 37.6 | pipelines/compile_ms 261, pipelines/front_ms 252, render/thread/ms 48.68, render/busy/ms 46.99, main/busy/ms 45.22, stage/main/update 30.54, stage/render/render 21.03, render/graph/ms 18.77, stage/render/graph_submit 15.11, stage/render/prepare_assets 10.91, pipelines/driver_ms 9.54, encode/submit 8.41 |
| warm-up | 5.09 | 31.1 | render/thread/ms 49.36, render/busy/ms 47.48, pipelines/compile_ms 43.40, pipelines/front_ms 39.87, main/busy/ms 38.70, stage/main/update 19.89, stage/render/render 15.26, render/graph/ms 13.03, stage/main/post_update 12.76, stage/render/graph_submit 9.66, encode//sum 8.24, stage/render/prepare_assets 7.85 |
| warm-up | 5.14 | 27.8 | pipelines/compile_ms 566, pipelines/front_ms 552, render/thread/ms 43.44, render/busy/ms 42.07, main/busy/ms 33.74, pipelines/driver_ms 14.71, stage/render/render 14.41, stage/main/update 13.78, stage/main/post_update 13.09, render/graph/ms 12.67, stage/render/prepare_assets 9.87, stage/render/graph_submit 9.34 |
| warm-up | 5.17 | 25.8 | pipelines/compile_ms 499, pipelines/front_ms 482, render/thread/ms 47.45, render/busy/ms 46.16, main/busy/ms 40.93, stage/main/update 17.55, pipelines/driver_ms 16.66, stage/render/render 14.36, stage/main/post_update 14.26, render/graph/ms 12.64, stage/render/prepare_assets 9.66, stage/render/graph_submit 8.94 |
| warm-up | 5.19 | 25.8 | pipelines/compile_ms 503, pipelines/front_ms 484, render/thread/ms 43.13, render/busy/ms 41.86, main/busy/ms 39.55, pipelines/driver_ms 18.39, stage/main/update 17.51, stage/render/render 13.82, render/graph/ms 12.13, stage/main/post_update 11.88, stage/render/prepare_assets 10.53, stage/render/graph_submit 8.51 |
| warm-up | 5.27 | 80.3 | pipelines/compile_ms 627, pipelines/front_ms 608, render/thread/ms 99.04, render/busy/ms 97.77, main/busy/ms 75.73, stage/main/update 30.25, stage/main/post_update 26.40, stage/render/prepare_bind_groups 20.34, stage/render/prepare_flush 19.25, pipelines/driver_ms 18.33, stage/render/prepare_write_phase_buffers 16.82, stage/render/prepare_resources 16.70 |
| warm-up | 5.30 | 27.0 | pipelines/compile_ms 982, pipelines/front_ms 955, render/thread/ms 98.59, render/busy/ms 97.31, main/busy/ms 73.10, pipelines/driver_ms 27.58, stage/main/update 27.01, stage/main/post_update 25.28, stage/render/prepare_bind_groups 20.84, stage/render/prepare_flush 19.45, stage/render/prepare_write_phase_buffers 17.06, stage/render/prepare_resources 16.04 |
| warm-up | 5.64 | 27.4 | pipelines/compile_ms 103, pipelines/front_ms 96.10, render/thread/ms 45.23, render/busy/ms 43.72, main/busy/ms 40.15, stage/main/post_update 19.58, stage/render/render 19.54, render/graph/ms 17.46, stage/main/update 14.21, stage/render/graph_submit 12.98, stage/render/prepare_assets 10.13, encode//sum 7.08 |
| warm-up | 5.67 | 28.0 | pipelines/compile_ms 322, pipelines/front_ms 299, render/thread/ms 44.68, render/busy/ms 43.33, main/busy/ms 37.24, pipelines/driver_ms 22.37, stage/main/update 17.49, stage/render/prepare_assets 15.76, stage/main/post_update 12.66, stage/render/render 12.40, render/graph/ms 10.48, encode//sum 6.84 |
| warm-up | 5.84 | 30.8 | pipelines/compile_ms 607, pipelines/front_ms 591, main/busy/ms 40.51, render/thread/ms 30.54, render/busy/ms 29.35, stage/main/update 27.50, stream/drive/ms 18.12, pipelines/driver_ms 16.67, stage/render/render 12.28, render/graph/ms 10.69, stage/main/post_update 7.46, encode//sum 7.31 |
| warm-up | 5.91 | 32.7 | render/thread/ms 43.90, render/busy/ms 42.27, main/busy/ms 33.58, stage/render/render 15.62, stage/main/update 14.52, stage/render/prepare_assets 13.69, stage/main/post_update 13.58, render/graph/ms 13.04, encode//sum 9.21, stage/render/graph_submit 8.50, pipelines/compile_ms 7.97, pipelines/front_ms 7.58 |
| warm-up | 5.97 | 32.8 | pipelines/compile_ms 491, pipelines/front_ms 477, render/thread/ms 38.54, render/busy/ms 37.51, main/busy/ms 28.61, stage/render/prepare_assets 17.99, pipelines/driver_ms 14.12, stage/main/update 12.85, stage/main/post_update 9.62, stage/render/render 9.40, render/graph/ms 7.95, encode//sum 7.54 |
| warm-up | 6.05 | 27.6 | render/thread/ms 40.71, render/busy/ms 39.28, main/busy/ms 38.28, stage/main/update 24.11, stage/render/render 16.68, render/graph/ms 14.78, stage/render/graph_submit 10.50, encode//sum 10.46, stage/render/prepare_assets 10.32, stage/main/post_update 9.46, encode/submit 7.06, stream/drive/ms 4.35 |
| warm-up | 6.17 | 35.4 | pipelines/compile_ms 280, pipelines/front_ms 271, render/thread/ms 48.23, render/busy/ms 46.87, main/busy/ms 45.82, stage/main/update 25.62, stage/render/render 16.54, stage/main/post_update 15.23, render/graph/ms 14.47, stage/render/prepare_assets 10.74, encode//sum 10.65, stage/render/graph_submit 8.71 |
| warm-up | 6.19 | 28.6 | pipelines/compile_ms 280, pipelines/front_ms 271, render/thread/ms 53.18, render/busy/ms 52.00, main/busy/ms 41.61, stage/main/update 20.82, stage/render/prepare_assets 16.40, stage/render/render 16.27, stage/main/post_update 14.64, render/graph/ms 14.40, stage/render/graph_submit 10.21, encode//sum 8.92 |
| warm-up | 6.22 | 29.4 | render/thread/ms 45.68, render/busy/ms 44.69, main/busy/ms 31.04, stage/render/render 16.17, stage/render/prepare_assets 14.87, render/graph/ms 14.63, stage/main/update 14.17, stage/render/graph_submit 11.85, stage/main/post_update 10.82, encode/finish_wall 7.58, encode/pass:point_shadow_atlas_live_indirect/sum 7.37, encode//sum 6.52 |
| warm-up | 6.29 | 43.6 | render/thread/ms 60.71, render/busy/ms 59.37, main/busy/ms 48.01, stage/main/update 25.76, stage/render/render 22.84, render/graph/ms 20.77, stage/render/graph_submit 15.49, stage/main/post_update 14.47, stage/render/prepare_assets 11.36, encode//sum 10.82, encode/finish_wall 10.03, encode/pass:point_shadow_atlas_live_indirect/sum 9.57 |
| warm-up | 6.32 | 25.4 | render/thread/ms 59.97, render/busy/ms 58.74, main/busy/ms 42.61, stage/render/render 20.69, stage/main/update 20.07, render/graph/ms 18.76, stage/main/post_update 15.04, stage/render/graph_submit 14.61, stage/render/prepare_assets 12.32, encode//sum 10.89, encode/finish_wall 9.28, encode/pass:point_shadow_atlas_live_indirect/sum 8.71 |
| warm-up | 6.35 | 36.5 | render/thread/ms 47.41, render/busy/ms 46.39, main/busy/ms 31.67, stage/render/render 16.91, stage/main/update 16.31, render/graph/ms 15.38, stage/render/prepare_assets 14.88, stage/render/graph_submit 12.64, stage/main/post_update 8.85, encode/finish_wall 8.41, encode//sum 8.28, encode/pass:point_shadow_atlas_live_indirect/sum 8.13 |
| warm-up | 6.52 | 54.7 | main/busy/ms 61.80, render/thread/ms 48.87, render/busy/ms 47.62, stage/main/update 44.60, stage/render/render 22.16, stream/drive/ms 20.09, render/graph/ms 20.02, stage/render/graph_submit 16.46, stage/main/post_update 11.56, encode/finish_wall 10.88, encode/pass:point_shadow_atlas_live_indirect/sum 10.30, stage/render/prepare_assets 10.13 |
| warm-up | 6.56 | 35.4 | render/thread/ms 56.58, render/busy/ms 55.38, main/busy/ms 44.08, stage/main/update 25.95, stage/render/render 20.53, stage/render/prepare_assets 19.58, render/graph/ms 18.33, stage/render/graph_submit 14.61, stage/main/post_update 10.85, encode/finish_wall 9.72, encode/pass:point_shadow_atlas_live_indirect/sum 9.12, encode//sum 8.70 |
| warm-up | 6.59 | 29.1 | pipelines/compile_ms 143, pipelines/front_ms 139, render/thread/ms 45.25, render/busy/ms 44.25, main/busy/ms 32.11, stage/render/prepare_assets 18.74, stage/main/update 16.51, stage/render/render 14.69, render/graph/ms 13.12, stage/render/graph_submit 10.36, stage/main/post_update 9.20, encode/finish_wall 6.91 |
| measure | 18.31 | 34.8 | main/busy/ms 38.84, stage/main/update 25.76, render/thread/ms 21.61, render/busy/ms 20.61, stage/render/render 11.03, render/graph/ms 9.77, stage/main/post_update 9.19, stage/render/graph_submit 7.09, encode//sum 5.83, encode/submit 3.72, stage/render/prepare_assets 3.61, encode/finish_wall 3.39 |
| measure | 59.05 | 27.6 | render/thread/ms 36.97, render/busy/ms 35.92, main/busy/ms 25.29, stage/render/prepare_assets 15.73, stage/main/post_update 13.06, stage/render/render 10.85, render/graph/ms 9.37, encode//sum 8.98, stage/render/graph_submit 5.79, stage/main/update 5.05, stage/render/camera_driver 3.37, stage/main/fixed_main_loop 3.21 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1197 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **86%** (min 58%) · 2613 MHz (min 2610) · memory 8101 MHz · 85.4 W · 80 °C (max 85) · VRAM 5671 MiB · throttled 100% of samples

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
