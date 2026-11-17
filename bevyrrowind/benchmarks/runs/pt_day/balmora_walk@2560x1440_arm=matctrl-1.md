# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6155 frames over 65.6 s · **median 10.40 ms (96 fps)** · mean 10.67 · p95 13.21 · p99 16.19 (1% low 62 fps) · worst 45.58 ms · 75 spikes over 1.5× median · 56.7 stutters/min (work 49.4/min)
- warm-up: 323 frames over 9.1 s, worst 2588.3 ms
- hitches (> 26.0 ms): **33** measured, 50 in warm-up
- timed GPU passes: 6.44 ms of a 10.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.40 ms, limit 11 ms
- **FAIL** frame_ms: 62 stutters (> 1.5x moving median of 60) in 1.09 min = 56.67/min, limit 1/min
- **FAIL** worst frame 45.58 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.833 | 2.036 | 2.071 | 0.90 | 0.172 |
| main_opaque_pass_3d | 1.470 | 1.633 | 2.054 | 0.90 | 0.068 |
| early prepass | 0.442 | 0.491 | 0.719 | 0.90 | 0.088 |
| bloom | 0.283 | 0.314 | 0.321 | 0.90 | 0.062 |
| atmosphere_luts | 0.275 | 0.305 | 0.367 | 0.90 | 0.003 |
| water_velocity | 0.251 | 0.279 | 0.347 | 0.90 | 0.002 |
| water_sspr | 0.182 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.170 | 0.188 | 0.193 | 0.90 | 0.007 |
| atmos_froxel_inject | 0.131 | 0.145 | 0.147 | 0.90 | 0.001 |
| clouds_trace | 0.126 | 0.140 | 0.234 | 0.90 | 0.001 |
| water_fft | 0.117 | 0.130 | 0.137 | 0.90 | 0.003 |
| uniform_allocation | 0.115 | 0.128 | 0.179 | 0.90 | 0.025 |
| upscaling | 0.105 | 0.117 | 0.124 | 0.90 | 0.004 |
| auxiliary_views | 0.095 | 0.106 | 0.474 | 0.90 | 0.106 |
| clouds_composite | 0.095 | 0.106 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.094 | 0.105 | 0.143 | 0.90 | 0.018 |
| tonemapping | 0.077 | 0.085 | 0.088 | 0.90 | 0.003 |
| main_transmissive_pass_3d | 0.076 | 0.084 | 0.177 | 0.90 | 0.008 |
| clustering | 0.057 | 0.064 | 0.067 | 0.90 | 0.018 |
| render_sky | 0.046 | 0.052 | 0.053 | 0.90 | 0.002 |
| clouds_resolve | 0.044 | 0.049 | 0.049 | 0.90 | 0.001 |
| surface_exposure | 0.039 | 0.044 | 0.045 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.039 | 0.414 | 0.417 | 0.09 | 0.001 |
| surface_trail | 0.035 | 0.056 | 0.057 | 0.62 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.80 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.90 | 0.003 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.046 | 0.90 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.90 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.90 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.220 | 0.231 | 0.09 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.90 | 0.002 |
| night_sky | 0.012 | 0.014 | 0.033 | 0.90 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.90 | 0.124 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.90 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.005 | 10.995 | 39.70 | 55424.4 |
| main/busy/ms | 8.976 | 10.622 | 28.80 | 55247.9 |
| render/busy/ms | 8.436 | 10.473 | 39.16 | 51923.2 |
| stage/main/post_update | 5.580 | 6.897 | 9.46 | 34347.3 |
| stage/render/render | 5.357 | 6.823 | 15.26 | 32973.5 |
| render/graph/ms | 4.600 | 6.086 | 14.14 | 28313.4 |
| encode//sum | 3.957 | 5.240 | 25.22 | 24356.4 |
| stage/render/graph_submit | 3.460 | 4.666 | 10.32 | 21293.4 |
| encode/submit | 2.219 | 3.199 | 6.51 | 13657.8 |
| stage/main/update | 2.144 | 2.556 | 22.29 | 13194.2 |
| encode/finish_wall | 1.327 | 1.911 | 4.78 | 8170.8 |
| stage/render/camera_driver | 1.088 | 1.500 | 3.75 | 6696.2 |
| encode/pass:shadow_directional_light/sum | 0.839 | 1.514 | 2.25 | 5166.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.759 | 1.059 | 1.72 | 4669.4 |
| encode/pass:early prepass/sum | 0.747 | 1.042 | 1.89 | 4598.2 |
| stage/render/prepare_write_phase_buffers | 0.725 | 1.055 | 2.88 | 4464.6 |
| encode/pass:shadow_point_light/sum | 0.603 | 2.722 | 17.62 | 3709.6 |
| bigspace/total/ms | 0.602 | 1.405 | 2.45 | 3708.4 |
| stage/render/prepare_assets | 0.514 | 0.668 | 9.39 | 3165.0 |
| render/present/ms | 0.475 | 1.236 | 4.96 | 2923.0 |
| stage/render/prepare_resources | 0.450 | 0.539 | 1.14 | 2772.4 |
| stage/render/prepare_meshes | 0.421 | 0.524 | 6.83 | 2593.5 |
| stage/main/first | 0.396 | 0.517 | 5.97 | 2435.2 |
| stage/main/pre_update | 0.359 | 0.535 | 2.65 | 2207.3 |
| terrain/update/ms | 0.308 | 0.401 | 2.10 | 1895.9 |
| stage/render/prepare_bind_groups | 0.277 | 0.495 | 2.55 | 1705.2 |
| terrain/select/ms | 0.275 | 0.360 | 0.99 | 1689.9 |
| controller/feed/ms | 0.264 | 0.330 | 0.79 | 1627.5 |
| stream/drive/ms | 0.259 | 0.338 | 20.13 | 1596.2 |
| stage/render/specialize | 0.254 | 0.329 | 12.55 | 1561.8 |
| bigspace/low_precision/ms | 0.248 | 0.952 | 1.97 | 1523.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.239 | 0.354 | 0.95 | 1468.3 |
| stage/render/queue | 0.223 | 0.301 | 7.50 | 1369.7 |
| stage/main/fixed_main_loop | 0.210 | 0.365 | 1.08 | 1292.4 |
| stream/tick/ms | 0.201 | 0.269 | 0.60 | 1236.7 |
| far/select/ms | 0.194 | 0.262 | 0.67 | 1196.7 |
| stage/render/create_views | 0.192 | 0.250 | 0.92 | 1184.2 |
| anim/evaluate/ms | 0.188 | 0.582 | 1.61 | 1159.0 |
| stage/render/prepare_batch_phases | 0.186 | 0.263 | 0.78 | 1144.0 |
| bigspace/high_precision/ms | 0.184 | 0.909 | 2.15 | 1135.1 |
| stage/render/extract_commands | 0.169 | 0.198 | 0.45 | 1041.7 |
| pipelines/compile_ms | 0.165 | 0.000 | 136.47 | 1014.0 |
| stage/render/prepare_flush | 0.164 | 0.302 | 0.97 | 1008.7 |
| pipelines/front_ms | 0.160 | 0.000 | 133.67 | 984.6 |
| anim/evaluate_cpu/ms | 0.152 | 0.188 | 0.77 | 936.8 |
| controller/move/ms | 0.144 | 0.298 | 0.60 | 885.1 |
| stage/render/prepare_views | 0.138 | 0.175 | 0.56 | 850.5 |
| stage/main/last | 0.114 | 0.155 | 1.57 | 702.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.101 | 0.170 | 0.92 | 624.5 |
| render/acquire/ms | 0.094 | 0.123 | 0.50 | 578.1 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.124 | 0.27 | 356.5 |
| physics/step/ms | 0.046 | 0.086 | 0.59 | 281.2 |
| stage/render/phase_sort | 0.033 | 0.057 | 0.35 | 205.6 |
| stage/render/cleanup | 0.033 | 0.102 | 0.77 | 202.8 |
| physics/insert/ms | 0.031 | 0.346 | 0.86 | 190.6 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.65 | 188.5 |
| weather/map/ms | 0.025 | 0.032 | 0.23 | 152.6 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.26 | 88.4 |
| anim/apply/ms | 0.012 | 0.021 | 0.44 | 73.6 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.50 | 68.0 |
| controller/avoid/ms | 0.010 | 0.013 | 0.26 | 62.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 62.2 |
| script/update/ms | 0.007 | 0.010 | 0.12 | 45.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.01 | 36.5 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 33.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.22 | 31.8 |
| pipelines/driver_ms | 0.005 | 0.000 | 4.34 | 29.5 |
| script/ui/ms | 0.004 | 0.006 | 0.20 | 27.6 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 20.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1306 | 2262 | 661 |
| crowd/c0/meshes/count | count | 355 | 646 | 173 |
| crowd/c1/count | count | 144 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5995 | 5535 |
| crowd/c1/meshes/count | count | 1166 | 1297 | 1168 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4598 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4584 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.42 | 1.88 | 0.36 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.26 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.50 | 0.01 |
| encode/finish_critical | ms | 0.95 | 2.25 | 0.98 |
| encode/pass:early prepass/max | ms | 0.76 | 1.89 | 0.98 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.72 | 0.61 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.92 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.95 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 2.25 | 0.90 |
| encode/pass:shadow_point_light/max | ms | 0.77 | 1.39 | 0.80 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.65 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.12 | 39.16 | 8.20 |
| gpu/timed/ms | ms | 7.86 | 11.34 | 8.20 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 818 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 818 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1290 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3415 | 5113 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1290 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3415 | 5113 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 12 | 198 | 4 |
| phases/shadow/bins | count | 4990 | 8879 | 2413 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6366 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 254 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 12 | 557 | 6 |
| preprocess/bind_groups_created | count | 12 | 557 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 222 | 123 |
| process/cpu_usage | % | 26.44 | 40.87 | 27.88 |
| process/mem_usage | GiB | 5.48 | 5.65 | 5.65 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 0 | 0 | 0 |
| scene/batch/masked/count | count | 0 | 0 | 0 |
| scene/batch/masked/high_water | count | 0 | 0 | 0 |
| scene/batch/masked/live | count | 0 | 0 | 0 |
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
| scene/batch/opaque/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque/count | count | 0 | 0 | 0 |
| scene/batch/opaque/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque/live | count | 0 | 0 | 0 |
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
| scene/batches/count | count | 0 | 0 | 0 |
| scene/cells/resident | count | 0 | 0 | 0 |
| scene/chains/high_water | count | 0 | 0 | 0 |
| scene/chains/live | count | 0 | 0 | 0 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 0 | 0 | 0 |
| scene/slots/batch_capacity | count | 0 | 0 | 0 |
| scene/slots/batch_capacity_bytes | bytes | 0 | 0 | 0 |
| scene/slots/batch_live | count | 0 | 0 | 0 |
| scene/slots/scene_bump | count | 0 | 0 | 0 |
| scene/slots/scene_live | count | 0 | 0 | 0 |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 120 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 365.1 k | 813.1 k | 494.3 k |
| skin/joints_written/count | count | 5704 | 12.7 k | 7724 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4915 | 5009 | 5009 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.85 | 44.26 | 29.39 |
| system/mem_usage | % | 54.46 | 55.04 | 54.99 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 61 | 247 | 54 |
| views/all/bins | count | 7638 | 11.7 k | 5911 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 248 | 302 | 75 |
| views/all/pipelines_distinct | count | 70 | 75 | 61 |
| views/all/work_items | count | 10.7 k | 35.8 k | 8119 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2789 | 6173 | 2413 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8901 | 23.5 k | 6366 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 411 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2124 | 2825 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2380 | 3119 | 0 |
| views/lamp_face/pipelines | count | 164 | 216 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2846 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2884 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8462 | 13.0 k | 7610 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8618 | 13.2 k | 7918 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 54.94 | 282 | 338.1 k |
| anim/events/count | count | 0.09 | 3 | 560 |
| assets/Image/added | count | 0.10 | 38 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.22 | 11 | 7533 |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.76 | 1260 | 4675 |
| encode//count | count | 31.96 | 220 | 196.7 k |
| encode/bind_walk/init_textures | count | 11.7 k | 114.7 k | 71.77 M |
| encode/bind_walk/init_textures_skipped | count | 74.0 k | 162.2 k | 455.74 M |
| encode/bind_walk/inits | count | 452 | 1120 | 2.78 M |
| encode/bind_walk/inits_skipped | count | 116 | 180 | 714.0 k |
| encode/bind_walk/merge_textures | count | 11.6 k | 114.6 k | 71.32 M |
| encode/bind_walk/merge_textures_skipped | count | 5363 | 6012 | 33.01 M |
| encode/bind_walk/merges | count | 96.31 | 333 | 592.8 k |
| encode/bind_walk/merges_skipped | count | 88.26 | 135 | 543.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6155 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6155 |
| encode/pass:early prepass/count | count | 1 | 1 | 6155 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6155 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6155 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6155 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6155 |
| encode/pass:shadow_point_light/count | count | 1.01 | 48 | 6188 |
| encode/queue/command_buffers | count | 42.98 | 278 | 264.5 k |
| encode/queue/submits | count | 2.02 | 4 | 12.4 k |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 4.99 M | 11.49 G |
| encode/queue/write_buffer/calls | count | 254 | 832 | 1.57 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.5 k | 3.52 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 10.96 | 465 | 67.5 k |
| encode/queue/write_texture/bytes | bytes | 11.5 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6155 |
| physics/inserts | count | 0.37 | 110 | 2261 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 161 | 2335 |
| physics/steps | count | 0.64 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 2 | 12 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5500 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1206 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.0 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2588.3 | main/busy/ms 2557, stage/main/post_update 2440, stage/main/update 109, surface/advance_cpu_ms 76.47, stage/main/pre_update 5.04, stage/main/last 1.08, surface/cell_upload_ms 0.86, stage/main/first 0.51, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.34, weather/map/ms 0.16, stream/tick/ms 0.12 |
| warm-up | 3.33 | 740.7 | main/busy/ms 2743, stage/main/post_update 2468, render/thread/ms 708, render/busy/ms 590, stage/render/prepare_assets 561, stage/main/update 261, stage/render/prepare_views 114, render/acquire/ms 114, surface/advance_cpu_ms 76.47, stage/render/render 12.58, stage/render/create_views 9.29, render/graph/ms 8.17 |
| warm-up | 3.45 | 122.3 | render/thread/ms 812, render/busy/ms 631, stage/render/prepare_assets 589, main/busy/ms 207, stage/main/update 156, stage/render/prepare_views 115, render/acquire/ms 115, pipelines/compile_ms 110, stage/render/render 80.58, render/present/ms 66.56, pipelines/front_ms 57.22, pipelines/driver_ms 52.34 |
| warm-up | 3.60 | 128.6 | pipelines/wait_ms 3805, pipelines/compile_ms 1078, pipelines/front_ms 988, render/thread/ms 116, main/busy/ms 115, render/busy/ms 113, pipelines/driver_ms 89.77, stage/main/post_update 87.50, stage/render/prepare_resources 35.02, pipelines/lock_ms 32.35, stage/main/update 21.07, terrain/update/ms 20.66 |
| warm-up | 3.66 | 62.6 | pipelines/wait_ms 3967, pipelines/compile_ms 1817, pipelines/front_ms 1720, render/thread/ms 156, render/busy/ms 154, main/busy/ms 132, pipelines/driver_ms 97.71, stage/main/update 69.81, stage/main/post_update 57.44, stage/render/prepare_resources 51.71, stage/render/prepare_assets 46.71, pipelines/lock_ms 25.10 |
| warm-up | 3.74 | 71.3 | pipelines/compile_ms 1571, pipelines/front_ms 1489, pipelines/wait_ms 422, render/thread/ms 121, render/busy/ms 119, pipelines/driver_ms 82.10, main/busy/ms 80.88, stage/render/prepare_assets 77.46, stage/main/update 55.93, stage/render/prepare_resources 19.13, stage/main/post_update 17.91, terrain/update/ms 14.92 |
| warm-up | 3.80 | 31.5 | pipelines/compile_ms 649, pipelines/front_ms 616, pipelines/wait_ms 187, render/thread/ms 56.28, render/busy/ms 54.98, main/busy/ms 42.86, pipelines/driver_ms 33.18, stage/main/post_update 25.31, stage/render/prepare_assets 18.36, stage/render/prepare_resources 17.67, terrain/update/ms 9.87, encode//sum 8.46 |
| warm-up | 3.96 | 88.8 | pipelines/compile_ms 1888, pipelines/front_ms 1817, pipelines/driver_ms 71.02, render/thread/ms 69.16, render/busy/ms 67.62, main/busy/ms 62.64, pipelines/wait_ms 55.93, stage/main/update 46.94, stream/drive/ms 42.41, stage/render/prepare_assets 39.84, stage/render/render 19.05, render/graph/ms 17.11 |
| warm-up | 4.03 | 44.1 | pipelines/compile_ms 367, pipelines/front_ms 330, encode//sum 86.84, render/thread/ms 63.88, render/busy/ms 61.18, main/busy/ms 38.19, pipelines/driver_ms 37.40, stage/render/render 31.40, render/graph/ms 28.21, stage/render/prepare_assets 25.71, stage/render/graph_submit 25.61, stage/main/update 22.46 |
| warm-up | 4.08 | 45.2 | pipelines/compile_ms 522, pipelines/front_ms 502, render/thread/ms 80.16, render/busy/ms 77.19, main/busy/ms 53.00, stage/render/prepare_assets 30.05, stage/main/update 27.87, pipelines/driver_ms 20.31, stage/main/post_update 18.37, stage/render/render 15.97, render/graph/ms 10.96, stage/render/create_views 9.60 |
| warm-up | 4.15 | 71.9 | pipelines/compile_ms 296, pipelines/front_ms 286, render/thread/ms 104, render/busy/ms 102, main/busy/ms 81.84, stage/render/prepare_assets 56.30, stage/main/update 52.22, stage/main/post_update 21.35, stage/render/render 13.82, pipelines/driver_ms 10.13, render/graph/ms 9.72, stage/render/create_views 9.64 |
| warm-up | 4.21 | 57.7 | pipelines/compile_ms 1108, pipelines/front_ms 1081, render/thread/ms 101, render/busy/ms 99.64, stage/render/prepare_assets 75.43, main/busy/ms 58.66, stage/main/update 37.37, pipelines/driver_ms 27.71, stage/render/render 15.10, render/graph/ms 13.11, stage/main/post_update 12.30, stage/render/graph_submit 10.79 |
| warm-up | 4.26 | 52.5 | pipelines/compile_ms 2123, pipelines/front_ms 2064, pipelines/wait_ms 100, render/thread/ms 66.53, render/busy/ms 64.91, pipelines/driver_ms 58.42, stage/render/prepare_assets 35.07, main/busy/ms 30.81, stage/render/render 15.21, stage/main/update 13.45, render/graph/ms 13.09, pipelines/lock_ms 12.11 |
| warm-up | 4.34 | 84.0 | pipelines/compile_ms 1220, pipelines/front_ms 1183, pipelines/wait_ms 206, render/thread/ms 105, render/busy/ms 103, encode//sum 42.86, main/busy/ms 39.29, pipelines/driver_ms 37.76, stage/render/render 27.38, stage/main/update 26.00, render/graph/ms 24.66, stage/render/prepare_assets 18.62 |
| warm-up | 4.38 | 39.5 | pipelines/compile_ms 215, pipelines/front_ms 208, pipelines/wait_ms 114, render/thread/ms 110, render/busy/ms 108, main/busy/ms 48.50, encode//sum 45.06, stage/render/render 29.54, render/graph/ms 26.90, stage/main/update 25.23, stage/render/prepare_resources 18.75, stage/render/prepare_assets 17.53 |
| warm-up | 4.48 | 98.3 | pipelines/compile_ms 2281, pipelines/front_ms 2189, pipelines/wait_ms 636, render/thread/ms 122, render/busy/ms 120, pipelines/driver_ms 92.60, main/busy/ms 60.16, encode//sum 54.65, stage/render/render 52.74, render/graph/ms 49.85, stage/render/prepare_write_phase_buffers 31.82, stage/render/graph_submit 25.15 |
| warm-up | 4.52 | 39.3 | pipelines/compile_ms 2489, pipelines/front_ms 2388, pipelines/wait_ms 636, render/thread/ms 122, render/busy/ms 120, pipelines/driver_ms 100, main/busy/ms 61.36, encode//sum 54.57, stage/render/render 49.77, render/graph/ms 46.84, stage/render/prepare_write_phase_buffers 36.22, stage/main/update 27.95 |
| warm-up | 4.57 | 51.1 | pipelines/compile_ms 890, pipelines/front_ms 863, main/busy/ms 104, stage/main/update 80.78, render/thread/ms 58.51, render/busy/ms 57.21, stream/drive/ms 43.62, stage/render/prepare_assets 30.40, pipelines/wait_ms 27.85, pipelines/driver_ms 27.68, stage/main/post_update 12.60, encode//sum 9.45 |
| warm-up | 4.67 | 102.1 | pipelines/compile_ms 1985, pipelines/front_ms 1933, pipelines/wait_ms 593, main/busy/ms 106, stage/main/update 88.10, render/thread/ms 59.57, render/busy/ms 58.07, pipelines/driver_ms 51.58, stream/drive/ms 49.04, stage/render/prepare_assets 42.08, stage/render/render 9.31, stage/main/post_update 8.69 |
| warm-up | 4.71 | 37.1 | pipelines/compile_ms 1574, pipelines/front_ms 1534, pipelines/wait_ms 569, main/busy/ms 92.74, render/thread/ms 63.05, render/busy/ms 61.46, stage/main/update 59.91, stream/drive/ms 50.48, pipelines/driver_ms 40.60, stage/render/prepare_assets 28.84, stage/main/post_update 17.59, stage/render/render 11.70 |
| warm-up | 4.84 | 127.7 | pipelines/compile_ms 488, pipelines/front_ms 471, main/busy/ms 139, render/thread/ms 113, render/busy/ms 112, stage/main/update 60.25, stage/main/post_update 55.85, stream/drive/ms 47.55, anim/evaluate/ms 25.62, stage/render/prepare_bind_groups 22.54, stage/render/prepare_assets 19.08, pipelines/driver_ms 16.50 |
| warm-up | 4.91 | 71.2 | pipelines/compile_ms 2419, pipelines/front_ms 2343, main/busy/ms 130, render/thread/ms 128, render/busy/ms 127, pipelines/driver_ms 76.28, stage/main/update 65.54, stream/drive/ms 49.57, stage/main/post_update 47.66, stage/render/prepare_assets 42.95, pipelines/wait_ms 39.77, anim/evaluate/ms 25.42 |
| warm-up | 4.99 | 76.6 | pipelines/compile_ms 2507, pipelines/front_ms 2431, main/busy/ms 89.61, render/thread/ms 87.64, render/busy/ms 86.13, pipelines/driver_ms 76.03, stage/main/update 67.30, stage/render/prepare_assets 61.23, stream/drive/ms 47.58, pipelines/wait_ms 45.06, stage/main/post_update 12.63, stage/render/render 11.91 |
| warm-up | 5.09 | 70.2 | pipelines/compile_ms 932, pipelines/front_ms 905, main/busy/ms 78.29, stage/main/update 59.50, render/thread/ms 51.73, render/busy/ms 49.40, stream/drive/ms 37.97, pipelines/driver_ms 27.15, stage/render/prepare_assets 22.18, stage/render/render 13.81, stage/main/post_update 12.14, render/graph/ms 11.03 |
| warm-up | 5.12 | 30.9 | pipelines/compile_ms 944, pipelines/front_ms 916, render/thread/ms 52.64, render/busy/ms 51.31, main/busy/ms 38.99, pipelines/driver_ms 28.19, stage/render/prepare_assets 23.77, stage/main/update 22.04, stage/main/post_update 10.49, stage/render/render 9.43, render/graph/ms 7.82, encode//sum 7.42 |
| warm-up | 5.17 | 33.4 | pipelines/compile_ms 190, pipelines/front_ms 181, render/thread/ms 44.31, render/busy/ms 42.89, main/busy/ms 31.33, stage/render/prepare_assets 15.81, stage/main/update 15.68, stage/render/render 12.23, render/graph/ms 10.10, stage/main/post_update 10.04, pipelines/driver_ms 8.94, encode//sum 8.58 |
| warm-up | 5.21 | 40.5 | pipelines/compile_ms 292, pipelines/front_ms 278, render/thread/ms 64.12, render/busy/ms 62.52, main/busy/ms 46.78, stage/main/update 28.05, stage/render/prepare_assets 17.61, pipelines/driver_ms 13.45, stage/main/post_update 12.64, stage/render/render 11.76, render/graph/ms 9.75, stage/render/queue 8.71 |
| warm-up | 5.25 | 43.1 | pipelines/compile_ms 121, pipelines/front_ms 116, render/thread/ms 64.77, render/busy/ms 63.23, main/busy/ms 54.94, stage/main/update 34.12, stage/render/prepare_assets 17.82, stage/main/post_update 14.51, stage/render/render 10.58, stage/render/queue 9.17, render/graph/ms 8.76, encode//sum 8.35 |
| warm-up | 5.29 | 35.0 | render/thread/ms 60.38, render/busy/ms 59.11, main/busy/ms 47.75, stage/render/prepare_assets 27.74, stage/main/update 27.02, stage/main/post_update 13.73, stage/render/render 10.57, encode//sum 8.65, render/graph/ms 8.58, stream/drive/ms 6.28, stage/render/graph_submit 5.55, stage/render/prepare_resources 4.27 |
| warm-up | 5.32 | 33.4 | pipelines/compile_ms 1145, pipelines/front_ms 1116, render/thread/ms 59.03, render/busy/ms 57.56, main/busy/ms 48.05, pipelines/driver_ms 29.50, stage/render/prepare_assets 20.80, stage/main/update 19.23, stage/render/render 17.78, render/graph/ms 15.79, stage/main/post_update 13.47, stage/render/graph_submit 12.65 |
| warm-up | 5.47 | 33.2 | render/thread/ms 41.02, render/busy/ms 39.49, main/busy/ms 30.35, stage/render/prepare_assets 20.69, stage/main/update 15.57, stage/render/render 11.91, stage/main/post_update 9.85, render/graph/ms 9.84, stream/drive/ms 9.67, encode//sum 8.64, stage/render/graph_submit 7.63, encode/submit 5.30 |
| warm-up | 6.09 | 30.9 | render/thread/ms 43.18, render/busy/ms 41.39, main/busy/ms 33.21, stage/render/render 16.02, stage/main/post_update 15.54, render/graph/ms 13.61, stage/main/update 12.72, encode//sum 12.28, pipelines/compile_ms 11.58, pipelines/front_ms 10.50, stage/render/graph_submit 9.63, encode/submit 6.56 |
| warm-up | 6.14 | 32.3 | render/thread/ms 37.82, render/busy/ms 36.76, main/busy/ms 25.96, stage/render/prepare_assets 16.66, stage/main/update 11.19, stage/main/post_update 10.01, stage/render/render 9.93, encode//sum 8.95, render/graph/ms 8.22, stage/render/graph_submit 5.92, stream/drive/ms 3.55, encode/submit 3.51 |
| warm-up | 6.40 | 29.5 | pipelines/compile_ms 90.51, pipelines/front_ms 85.81, render/thread/ms 46.01, render/busy/ms 44.56, main/busy/ms 41.15, stage/main/update 17.41, stage/main/post_update 15.85, stage/render/render 14.83, stage/render/prepare_assets 14.25, render/graph/ms 12.92, stage/render/graph_submit 9.37, encode//sum 8.91 |
| warm-up | 6.47 | 34.1 | pipelines/compile_ms 165, pipelines/front_ms 160, main/busy/ms 42.81, render/thread/ms 33.51, render/busy/ms 31.58, stage/main/update 23.25, stage/render/render 14.49, stream/drive/ms 14.38, stage/main/post_update 13.87, render/graph/ms 11.83, encode//sum 9.66, stage/render/graph_submit 8.60 |
| warm-up | 6.88 | 36.6 | pipelines/compile_ms 144, pipelines/front_ms 140, main/busy/ms 42.03, render/thread/ms 25.35, render/busy/ms 24.26, stage/main/update 22.92, stage/main/post_update 13.55, stage/render/render 11.09, render/graph/ms 9.50, encode//sum 7.75, stage/render/prepare_assets 6.65, stage/render/graph_submit 6.23 |
| warm-up | 6.94 | 37.9 | render/thread/ms 50.50, render/busy/ms 49.24, main/busy/ms 31.85, pipelines/compile_ms 20.82, pipelines/front_ms 19.61, stage/main/update 14.63, stage/render/prepare_assets 11.21, stage/main/post_update 10.92, stage/render/specialize 10.89, stage/render/render 10.72, render/graph/ms 8.90, encode//sum 7.84 |
| warm-up | 8.27 | 32.9 | render/thread/ms 38.09, render/busy/ms 37.02, main/busy/ms 19.99, stage/render/render 11.63, stage/main/post_update 11.16, stage/render/specialize 10.23, render/graph/ms 10.09, encode//sum 7.91, stage/render/graph_submit 7.86, stage/render/queue 6.28, stage/main/update 6.07, encode/submit 4.29 |
| measure | 0.43 | 29.9 | render/thread/ms 38.89, render/busy/ms 37.77, main/busy/ms 16.70, stage/render/render 13.49, render/graph/ms 11.93, stage/main/post_update 9.98, stage/render/graph_submit 9.51, stage/render/specialize 9.00, encode//sum 8.21, encode/submit 6.57, stage/render/queue 6.30, stage/main/update 4.12 |
| measure | 1.73 | 36.3 | render/thread/ms 39.27, render/busy/ms 37.96, main/busy/ms 18.35, stage/render/render 12.42, stage/main/post_update 11.57, render/graph/ms 10.73, stage/render/specialize 9.51, stage/render/graph_submit 8.03, encode//sum 7.96, stage/render/queue 6.55, stage/render/prepare_meshes 5.04, encode/submit 4.55 |
| measure | 4.51 | 29.9 | render/thread/ms 37.67, render/busy/ms 36.53, main/busy/ms 19.70, stage/render/render 12.44, stage/main/post_update 12.17, render/graph/ms 10.86, stage/render/specialize 9.25, encode//sum 9.19, stage/render/graph_submit 8.22, stage/render/queue 6.26, encode/submit 5.30, encode/pass:shadow_point_light/sum 4.94 |
| measure | 5.83 | 30.7 | render/thread/ms 33.74, render/busy/ms 32.65, main/busy/ms 20.53, stage/main/post_update 12.77, stage/render/render 9.67, stage/render/specialize 9.38, render/graph/ms 8.12, encode//sum 7.57, stage/render/queue 6.28, stage/render/graph_submit 6.15, stage/main/update 4.55, encode/submit 3.43 |
| measure | 7.12 | 29.5 | render/thread/ms 39.81, render/busy/ms 38.90, main/busy/ms 20.04, stage/main/post_update 12.38, stage/render/render 11.92, render/graph/ms 10.60, stage/render/specialize 9.66, encode//sum 9.05, stage/render/graph_submit 7.72, stage/render/queue 6.11, encode/submit 4.40, stage/main/update 3.94 |
| measure | 9.64 | 30.1 | render/thread/ms 35.36, render/busy/ms 34.32, main/busy/ms 19.03, stage/main/post_update 12.23, stage/render/render 10.03, stage/render/specialize 9.66, render/graph/ms 8.50, encode//sum 7.14, stage/render/graph_submit 6.52, stage/render/queue 6.49, encode/submit 3.91, stage/main/update 3.84 |
| measure | 10.93 | 29.6 | render/thread/ms 35.95, render/busy/ms 34.97, main/busy/ms 19.82, stage/main/post_update 10.73, stage/render/render 10.45, stage/render/specialize 9.98, render/graph/ms 9.05, encode//sum 7.87, stage/render/graph_submit 6.94, stage/render/queue 6.32, stage/main/update 4.18, encode/submit 3.95 |
| measure | 15.71 | 31.3 | render/thread/ms 40.03, render/busy/ms 38.84, main/busy/ms 19.94, stage/main/post_update 12.77, stage/render/render 12.75, render/graph/ms 11.21, stage/render/specialize 10.75, encode//sum 9.18, stage/render/graph_submit 8.41, stage/render/queue 7.03, encode/submit 5.38, encode/pass:shadow_point_light/sum 4.34 |
| measure | 18.01 | 30.6 | render/thread/ms 34.41, render/busy/ms 33.31, main/busy/ms 17.86, stage/main/post_update 11.04, stage/render/render 10.06, stage/render/specialize 9.55, encode//sum 9.17, render/graph/ms 8.54, stage/render/graph_submit 6.38, stage/render/queue 6.11, stage/main/update 3.94, encode/submit 3.78 |
| measure | 18.18 | 31.3 | main/busy/ms 38.27, stage/main/update 24.51, stream/drive/ms 20.45, render/thread/ms 18.79, render/busy/ms 17.64, stage/render/render 11.02, stage/main/post_update 10.93, render/graph/ms 9.34, encode//sum 8.18, stage/render/graph_submit 7.11, encode/submit 4.71, encode/pass:shadow_point_light/sum 2.73 |
| measure | 19.37 | 31.4 | render/thread/ms 34.41, render/busy/ms 33.49, main/busy/ms 19.10, stage/main/post_update 11.77, stage/render/specialize 9.93, stage/render/render 9.49, encode//sum 8.22, render/graph/ms 8.12, stage/render/queue 6.38, stage/render/graph_submit 6.25, stage/main/update 4.64, encode/submit 3.72 |
| measure | 20.71 | 32.3 | render/thread/ms 34.38, render/busy/ms 33.38, main/busy/ms 18.04, stage/main/post_update 10.99, stage/render/render 9.74, stage/render/specialize 9.64, encode//sum 8.71, render/graph/ms 8.35, stage/render/queue 6.63, stage/render/graph_submit 6.21, stage/main/update 4.53, encode/submit 3.61 |
| measure | 23.00 | 33.5 | render/thread/ms 39.95, render/busy/ms 38.76, main/busy/ms 23.38, stage/main/post_update 12.32, stage/render/render 10.82, stage/render/specialize 10.70, render/graph/ms 9.25, encode//sum 8.44, stage/render/queue 7.67, stage/render/graph_submit 7.16, stage/main/update 6.34, encode/submit 3.79 |
| measure | 24.43 | 31.5 | render/thread/ms 36.31, render/busy/ms 35.02, main/busy/ms 20.03, stage/main/post_update 11.89, stage/render/render 10.75, stage/render/specialize 9.75, render/graph/ms 9.04, encode//sum 8.66, stage/render/graph_submit 7.10, stage/render/queue 6.38, stage/main/update 5.35, encode/submit 3.80 |
| measure | 27.28 | 36.2 | render/thread/ms 42.99, render/busy/ms 41.87, main/busy/ms 25.61, stage/render/render 14.44, encode//sum 14.31, render/graph/ms 12.89, stage/main/post_update 12.37, stage/render/specialize 11.82, stage/render/graph_submit 10.24, stage/render/queue 7.61, encode/submit 6.63, stage/main/update 6.45 |
| measure | 28.67 | 31.5 | render/thread/ms 36.64, render/busy/ms 35.50, main/busy/ms 19.26, stage/main/post_update 11.76, stage/render/render 11.31, encode//sum 9.80, render/graph/ms 9.71, stage/render/specialize 9.64, stage/render/graph_submit 7.61, stage/render/queue 6.58, stage/main/update 4.74, encode/submit 4.40 |
| measure | 30.45 | 33.3 | render/thread/ms 37.67, render/busy/ms 36.63, main/busy/ms 20.99, stage/main/post_update 12.92, stage/render/render 11.03, stage/render/specialize 10.41, render/graph/ms 9.54, encode//sum 8.01, stage/render/graph_submit 7.22, stage/render/queue 6.92, stage/main/update 5.25, encode/submit 3.93 |
| measure | 35.34 | 33.8 | render/thread/ms 40.61, render/busy/ms 39.53, main/busy/ms 20.36, stage/render/render 12.78, stage/main/post_update 12.78, render/graph/ms 11.31, stage/render/specialize 10.48, encode//sum 9.57, stage/render/graph_submit 8.38, stage/render/queue 7.48, encode/submit 4.68, stage/main/update 4.50 |
| measure | 36.81 | 30.8 | render/thread/ms 39.94, render/busy/ms 38.86, main/busy/ms 19.22, stage/render/render 13.42, stage/main/post_update 12.23, render/graph/ms 11.88, encode//sum 10.05, stage/render/specialize 9.69, stage/render/graph_submit 8.72, stage/render/queue 7.00, encode/pass:shadow_point_light/sum 5.62, encode/submit 5.25 |
| measure | 37.48 | 30.6 | render/thread/ms 37.64, render/busy/ms 36.43, main/busy/ms 20.17, stage/main/post_update 12.35, stage/render/render 12.16, render/graph/ms 10.49, stage/render/specialize 10.01, encode//sum 9.40, stage/render/graph_submit 7.92, stage/render/queue 6.39, encode/submit 4.84, stage/main/update 4.79 |
| measure | 40.80 | 34.5 | render/thread/ms 39.01, render/busy/ms 37.84, main/busy/ms 19.49, stage/main/post_update 11.42, stage/render/render 10.77, stage/render/specialize 9.93, render/graph/ms 9.23, encode//sum 7.98, stage/render/graph_submit 7.28, stage/render/queue 6.94, stage/main/update 4.83, encode/submit 4.05 |
| measure | 43.23 | 33.2 | render/thread/ms 38.52, render/busy/ms 37.48, main/busy/ms 18.09, stage/render/render 11.19, stage/main/post_update 10.99, stage/render/specialize 9.95, render/graph/ms 9.67, encode//sum 8.81, stage/render/graph_submit 7.86, stage/render/queue 7.05, stage/main/update 4.10, encode/submit 4.09 |
| measure | 45.67 | 32.1 | render/thread/ms 40.59, render/busy/ms 39.60, main/busy/ms 18.89, stage/render/render 11.84, stage/main/post_update 11.35, render/graph/ms 10.50, stage/render/specialize 10.29, encode//sum 8.66, stage/render/graph_submit 8.22, stage/render/queue 6.94, stage/main/update 4.61, encode/submit 4.60 |
| measure | 56.44 | 29.9 | render/thread/ms 37.29, render/busy/ms 36.33, main/busy/ms 15.06, stage/render/render 10.48, stage/render/specialize 9.70, render/graph/ms 9.21, stage/main/post_update 8.94, encode//sum 8.20, stage/render/graph_submit 7.18, stage/render/queue 6.90, stage/main/update 4.07, encode/submit 3.67 |
| measure | 59.03 | 45.6 | render/thread/ms 54.75, render/busy/ms 53.26, main/busy/ms 32.30, stage/main/post_update 17.29, stage/render/render 16.84, render/graph/ms 14.72, encode//sum 13.20, stage/render/specialize 12.80, stage/render/graph_submit 11.59, stage/main/update 9.34, stage/render/prepare_meshes 7.30, encode/submit 6.78 |
| measure | 62.57 | 30.7 | render/thread/ms 37.83, render/busy/ms 36.68, main/busy/ms 18.99, stage/render/render 10.90, stage/render/specialize 10.53, encode//sum 10.14, stage/main/post_update 9.90, render/graph/ms 9.54, stage/render/graph_submit 7.56, stage/render/queue 6.49, stage/main/update 6.14, stage/render/prepare_meshes 3.99 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=matctrl-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **79%** (min 56%) · 2620 MHz (min 2610) · memory 8101 MHz · 81.8 W · 78 °C (max 83) · VRAM 7056 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `mw.matctrl.gate=on (command line: --set)`
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
- `sf.graphics_preset=custom (preset: derived from its members)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=custom (preset: derived from its members)`
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
