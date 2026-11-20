# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.batched_pass=on (command line: --set); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6277 frames over 65.6 s · **median 10.31 ms (97 fps)** · mean 10.46 · p95 12.27 · p99 14.07 (1% low 71 fps) · worst 33.04 ms · 34 spikes over 1.5× median · 25.6 stutters/min (work 28.3/min)
- warm-up: 355 frames over 9.5 s, worst 2544.5 ms
- hitches (> 25.8 ms): **4** measured, 50 in warm-up
- timed GPU passes: 6.79 ms of a 10.46 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.31 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- **FAIL** worst frame 33.04 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.063 | 2.198 | 2.692 | 0.94 | 0.176 |
| main_opaque_pass_3d | 1.264 | 1.347 | 1.896 | 0.94 | 0.077 |
| early prepass | 0.508 | 0.541 | 0.833 | 0.94 | 0.094 |
| bloom | 0.319 | 0.340 | 0.492 | 0.94 | 0.065 |
| atmosphere_luts | 0.310 | 0.330 | 0.466 | 0.94 | 0.003 |
| water_velocity | 0.232 | 0.247 | 0.404 | 0.94 | 0.002 |
| water_sspr | 0.211 | 0.225 | 0.354 | 0.94 | 0.002 |
| batched_point_shadows | 0.209 | 0.379 | 0.779 | 0.55 | 0.141 |
| ssao | 0.183 | 0.195 | 0.204 | 0.94 | 0.008 |
| atmos_froxel_inject | 0.129 | 0.137 | 0.130 | 0.94 | 0.002 |
| upscaling | 0.126 | 0.134 | 0.153 | 0.94 | 0.005 |
| water_fft | 0.122 | 0.130 | 0.127 | 0.94 | 0.003 |
| uniform_allocation | 0.119 | 0.127 | 0.147 | 0.94 | 0.024 |
| clouds_composite | 0.102 | 0.109 | 0.109 | 0.94 | 0.002 |
| bin_unpacking | 0.092 | 0.098 | 0.121 | 0.94 | 0.018 |
| gpu_instance_materialization | 0.090 | 0.096 | 0.099 | 0.94 | 0.004 |
| tonemapping | 0.086 | 0.092 | 0.094 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.070 | 0.075 | 0.139 | 0.94 | 0.009 |
| clustering | 0.067 | 0.071 | 0.068 | 0.94 | 0.019 |
| early_mesh_preprocessing | 0.056 | 0.060 | 0.077 | 0.94 | 0.004 |
| clouds_trace | 0.053 | 0.056 | 0.079 | 0.94 | 0.001 |
| render_sky | 0.051 | 0.054 | 0.054 | 0.94 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.490 | 1.021 | 0.08 | 0.001 |
| surface_trail | 0.040 | 0.063 | 0.057 | 0.64 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.032 | 0.032 | 0.94 | 0.001 |
| batched_point_shadows/uniform_allocation | 0.028 | 0.050 | 0.088 | 0.55 | 0.013 |
| batched_point_shadows/early_mesh_preprocessing | 0.026 | 0.048 | 0.073 | 0.55 | 0.007 |
| water_snapshot | 0.025 | 0.026 | 0.035 | 0.94 | 0.002 |
| range_unpacking | 0.023 | 0.024 | 0.031 | 0.94 | 0.003 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.94 | 0.002 |
| lightprobe_radiance_map | 0.019 | 0.219 | 0.229 | 0.08 | 0.004 |
| per_view_shadow_pass | 0.019 | 0.020 | 0.027 | 0.94 | 0.006 |
| night_sky | 0.016 | 0.017 | 0.043 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.017 | 0.031 | 0.94 | 0.192 |
| batched_point_shadows/bin_unpacking | 0.015 | 0.028 | 0.051 | 0.55 | 0.009 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.94 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| batched_point_shadows/range_unpacking | 0.004 | 0.007 | 0.007 | 0.55 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.94 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.117 | 10.812 | 26.44 | 57229.0 |
| render/busy/ms | 8.585 | 10.313 | 19.84 | 53890.5 |
| main/busy/ms | 8.388 | 9.768 | 26.75 | 52650.4 |
| stage/render/render | 5.463 | 6.618 | 22.37 | 34293.9 |
| render/graph/ms | 4.774 | 5.921 | 10.72 | 29967.8 |
| stage/main/post_update | 4.738 | 5.712 | 9.84 | 29740.7 |
| stage/render/graph_submit | 3.472 | 4.566 | 7.52 | 21796.4 |
| encode//sum | 3.455 | 4.222 | 7.19 | 21689.8 |
| encode/submit | 2.139 | 2.872 | 5.13 | 13424.9 |
| stage/main/update | 1.903 | 2.405 | 19.93 | 11943.2 |
| encode/finish_wall | 1.404 | 2.157 | 3.78 | 8813.6 |
| stage/render/camera_driver | 1.245 | 1.457 | 3.06 | 7815.7 |
| encode/pass:early prepass/sum | 0.829 | 1.085 | 1.88 | 5203.5 |
| encode/pass:shadow_directional_light/sum | 0.829 | 1.237 | 2.02 | 5202.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.822 | 1.102 | 1.73 | 5162.0 |
| encode/pass:shadow_point_light/sum | 0.697 | 1.972 | 3.14 | 4374.6 |
| stage/render/prepare_write_phase_buffers | 0.667 | 1.034 | 1.96 | 4187.7 |
| bigspace/total/ms | 0.576 | 1.357 | 3.44 | 3615.0 |
| stage/main/fixed_main_loop | 0.543 | 0.955 | 2.47 | 3405.3 |
| stage/render/prepare_assets | 0.542 | 0.683 | 12.65 | 3401.1 |
| stage/render/prepare_meshes | 0.452 | 0.579 | 8.15 | 2839.0 |
| render/present/ms | 0.441 | 1.219 | 16.15 | 2771.0 |
| stage/render/prepare_resources | 0.435 | 0.549 | 2.17 | 2733.0 |
| stage/main/pre_update | 0.400 | 0.550 | 2.27 | 2508.5 |
| stage/main/first | 0.362 | 0.472 | 5.87 | 2269.8 |
| stage/render/prepare_bind_groups | 0.349 | 0.612 | 1.84 | 2189.7 |
| controller/feed/ms | 0.343 | 0.439 | 0.83 | 2153.6 |
| stage/render/prepare_flush | 0.302 | 0.533 | 1.48 | 1893.5 |
| stage/render/prepare_batch_phases | 0.286 | 0.445 | 0.99 | 1796.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.281 | 0.450 | 0.99 | 1765.9 |
| stage/main/last | 0.256 | 0.313 | 0.89 | 1608.7 |
| bigspace/low_precision/ms | 0.246 | 0.786 | 2.09 | 1544.6 |
| stream/drive/ms | 0.208 | 0.309 | 10.57 | 1308.6 |
| stage/render/create_views | 0.204 | 0.275 | 0.81 | 1283.2 |
| stage/render/extract_commands | 0.176 | 0.209 | 0.60 | 1104.7 |
| anim/evaluate/ms | 0.169 | 0.299 | 1.89 | 1063.2 |
| bigspace/high_precision/ms | 0.168 | 0.716 | 2.69 | 1054.3 |
| anim/evaluate_cpu/ms | 0.157 | 0.200 | 0.52 | 988.3 |
| stream/tick/ms | 0.152 | 0.227 | 0.72 | 957.2 |
| stage/render/queue | 0.150 | 0.234 | 1.57 | 942.4 |
| stage/render/specialize | 0.144 | 0.235 | 1.66 | 902.8 |
| stage/render/prepare_views | 0.137 | 0.180 | 0.63 | 862.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.099 | 0.159 | 0.68 | 621.8 |
| render/acquire/ms | 0.090 | 0.120 | 0.55 | 567.5 |
| controller/move/ms | 0.080 | 0.202 | 0.52 | 501.6 |
| controller/write/ms | 0.075 | 0.102 | 0.43 | 468.7 |
| terrain/update/ms | 0.061 | 0.287 | 1.84 | 380.5 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.073 | 0.34 | 287.8 |
| physics/step/ms | 0.043 | 0.088 | 1.21 | 267.3 |
| encode/point_shadow_cache_copies/sum | 0.034 | 0.082 | 0.45 | 214.9 |
| stage/render/phase_sort | 0.034 | 0.058 | 0.41 | 213.9 |
| encode/resolve_diagnostics/sum | 0.034 | 0.046 | 0.36 | 212.3 |
| pipelines/compile_ms | 0.029 | 0.000 | 87.77 | 182.9 |
| pipelines/front_ms | 0.028 | 0.000 | 84.66 | 176.4 |
| physics/insert/ms | 0.027 | 0.317 | 0.86 | 170.7 |
| stage/render/cleanup | 0.025 | 0.038 | 0.70 | 155.3 |
| terrain/select/ms | 0.023 | 0.253 | 0.64 | 142.8 |
| weather/map/ms | 0.018 | 0.025 | 0.37 | 111.6 |
| far/select/ms | 0.016 | 0.170 | 0.52 | 101.3 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.26 | 82.1 |
| anim/apply/ms | 0.012 | 0.021 | 0.35 | 75.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.021 | 0.11 | 69.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.21 | 67.5 |
| script/update/ms | 0.008 | 0.012 | 0.34 | 52.7 |
| controller/avoid/ms | 0.007 | 0.013 | 0.21 | 41.2 |
| stream/despawn/ms | 0.006 | 0.000 | 1.31 | 36.4 |
| script/input/ms | 0.005 | 0.006 | 0.19 | 32.6 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 29.1 |
| stream/spawn/ms | 0.004 | 0.000 | 1.08 | 22.4 |
| stream/upload/ms | 0.003 | 0.000 | 1.24 | 20.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.69 | 13.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.7 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.11 | 6.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.07 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 561 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 19 | 34 | 17 |
| controller/movers/count | count | 20 | 23 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 583 |
| crowd/c0/meshes/count | count | 354 | 646 | 162 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5940 | 5613 |
| crowd/c1/meshes/count | count | 1167 | 1270 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 206 |
| crowd/joints_extracted/count | count | 708 | 6836 | 634 |
| crowd/joints_uploaded/count | count | 3622 | 6193 | 5807 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.43 | 1.20 | 0.53 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.26 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.11 | 0.01 |
| encode/finish_critical | ms | 1.05 | 3.14 | 0.88 |
| encode/pass:early prepass/max | ms | 0.83 | 1.88 | 0.79 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 1.73 | 0.75 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.68 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 0.99 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.02 | 0.88 |
| encode/pass:shadow_point_light/max | ms | 1.15 | 3.14 | 0.26 |
| encode/point_shadow_cache_copies/max | ms | 0.06 | 0.45 | 0.06 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.36 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.68 | 26.75 | 8.41 |
| gpu/timed/ms | ms | 8.06 | 10.24 | 8.41 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 10 | 29 | 0 |
| lamp_cache/copies | count | 7 | 15 | 0 |
| lamp_cache/deferred_faces | count | 0 | 16 | 0 |
| lamp_cache/lamps | count | 6 | 8 | 0 |
| lamp_cache/mover_faces | count | 7 | 12 | 0 |
| lamp_cache/movers | count | 119 | 290 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 6 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 15 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 7 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 13 | 36 | 0 |
| lamp_cache/stock_lamps | count | 0 | 2 | 0 |
| lamp_cache/store_starved | count | 0 | 8 | 0 |
| lamp_cache/store_used | count | 10 | 24 | 0 |
| lamp_cache/views | count | 10 | 36 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 156 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 335 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 156 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 335 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2477 | 3515 | 2448 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2477 | 3515 | 2448 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 28 | 89 | 8 |
| phases/shadow/bins | count | 812 | 1246 | 671 |
| phases/shadow/instance_batches | count | 4 | 10 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 106.5 k | 49.2 k |
| phases/shadow/items | count | 2316 | 2932 | 2035 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 52 | 120 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 179 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 327 | 328 | 328 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 27 | 300 | 7 |
| preprocess/bind_groups_created | count | 22 | 294 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 8 | 5 |
| preprocess/bind_groups_reused | count | 136 | 257 | 127 |
| process/cpu_usage | % | 24.32 | 38.46 | 23.56 |
| process/mem_usage | GiB | 5.24 | 5.44 | 5.44 |
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
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 118 | 14 |
| skin/blocks_skipped/count | count | 145 | 206 | 192 |
| skin/blocks_walked/count | count | 14 | 29 | 14 |
| skin/joint_bytes_written/bytes | bytes | 50.7 k | 811.3 k | 42.0 k |
| skin/joints_written/count | count | 792 | 12.7 k | 656 |
| skin/upload_writes/count | count | 8 | 24 | 12 |
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
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.24 | 41.35 | 34.13 |
| system/mem_usage | % | 56.08 | 56.57 | 56.53 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2666 | 2724 | 2724 |
| views/all/batch_sets | count | 86 | 146 | 66 |
| views/all/bins | count | 2920 | 3426 | 2961 |
| views/all/count | count | 16 | 41 | 3 |
| views/all/instance_batches | count | 12 | 18 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 204.8 k | 147.5 k |
| views/all/items | count | 8006 | 10.2 k | 7961 |
| views/all/pipelines | count | 100 | 162 | 66 |
| views/all/pipelines_distinct | count | 66 | 74 | 61 |
| views/all/work_items | count | 154.0 k | 214.4 k | 153.9 k |
| views/cascade/batch_sets | count | 11 | 34 | 8 |
| views/cascade/bins | count | 716 | 842 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2100 | 2424 | 2035 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 12 | 58 | 0 |
| views/lamp_face/bins | count | 108 | 424 | 0 |
| views/lamp_face/count | count | 13 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 12 | 0 |
| views/lamp_face/drawn | count | 8 | 24 | 0 |
| views/lamp_face/instance_batches | count | 0 | 2 | 0 |
| views/lamp_face/instance_draws | count | 0 | 26 | 0 |
| views/lamp_face/instance_slots | count | 0 | 8192 | 0 |
| views/lamp_face/items | count | 144 | 582 | 0 |
| views/lamp_face/pipelines | count | 26 | 94 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 77 | 8543 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2018 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5906 | 7452 | 5926 |
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
| world/meshes | count | 21.1 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 42.46 | 204 | 266.5 k |
| anim/events/count | count | 0.08 | 3 | 505 |
| assets/Image/added | count | 0.09 | 39 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.46 | 264 | 2874 |
| assets/Mesh/removed | count | 0.77 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.19 | 10 | 7477 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.24 | 1050 | 1490 |
| encode//count | count | 27.94 | 30 | 175.4 k |
| encode/bind_walk/init_textures | count | 10.0 k | 26.5 k | 62.94 M |
| encode/bind_walk/init_textures_skipped | count | 52.2 k | 73.6 k | 327.56 M |
| encode/bind_walk/inits | count | 600 | 766 | 3.76 M |
| encode/bind_walk/inits_skipped | count | 137 | 228 | 857.6 k |
| encode/bind_walk/merge_textures | count | 9964 | 26.4 k | 62.55 M |
| encode/bind_walk/merge_textures_skipped | count | 8629 | 16.8 k | 54.17 M |
| encode/bind_walk/merges | count | 122 | 203 | 766.5 k |
| encode/bind_walk/merges_skipped | count | 125 | 222 | 787.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6277 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6277 |
| encode/pass:early prepass/count | count | 1 | 1 | 6277 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6277 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6277 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6277 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6277 |
| encode/pass:shadow_point_light/count | count | 0.57 | 1 | 3567 |
| encode/point_shadow_cache_copies/count | count | 0.55 | 1 | 3448 |
| encode/queue/command_buffers | count | 39.07 | 43 | 245.2 k |
| encode/queue/submits | count | 2.01 | 4 | 12.6 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.43 M | 14.64 G |
| encode/queue/write_buffer/calls | count | 289 | 474 | 1.81 M |
| encode/queue/write_buffer_with/bytes | bytes | 122.4 k | 1.25 M | 768.21 M |
| encode/queue/write_buffer_with/calls | count | 10.36 | 801 | 65.0 k |
| encode/queue/write_texture/bytes | bytes | 2029 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6277 |
| physics/inserts | count | 0.36 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 175 | 2335 |
| physics/steps | count | 0.63 | 2 | 3939 |
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
| stream/deactivate/items | count | 0.04 | 35 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5393 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1182 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.8 ms; slowest 54, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2544.5 | main/busy/ms 2509, stage/main/post_update 2467, stage/main/update 31.90, stage/main/pre_update 7.25, stage/main/last 1.04, surface/advance_cpu_ms 0.72, stage/main/first 0.47, stage/main/fixed_main_loop 0.39, stream/drive/ms 0.32, surface/cell_upload_ms 0.31, bigspace/total/ms 0.11, stream/tick/ms 0.08 |
| warm-up | 3.31 | 762.6 | main/busy/ms 2657, stage/main/post_update 2494, render/thread/ms 718, render/busy/ms 594, stage/render/prepare_assets 547, stage/main/update 146, stage/render/prepare_views 122, render/acquire/ms 122, stage/render/create_views 27.76, stage/render/render 11.75, stage/main/pre_update 9.18, render/graph/ms 8.26 |
| warm-up | 3.36 | 57.7 | render/thread/ms 759, render/busy/ms 633, stage/render/prepare_assets 573, main/busy/ms 168, stage/render/prepare_views 123, render/acquire/ms 122, stage/main/update 119, pipelines/compile_ms 41.86, pipelines/front_ms 39.99, stage/main/post_update 37.95, stage/render/create_views 28.21, terrain/update/ms 21.99 |
| warm-up | 3.42 | 55.2 | pipelines/wait_ms 353, pipelines/compile_ms 349, pipelines/front_ms 327, render/thread/ms 93.98, render/busy/ms 90.78, main/busy/ms 62.94, stage/render/render 51.29, render/graph/ms 47.75, stage/main/post_update 44.81, terrain/update/ms 27.41, pipelines/lock_ms 27.21, stage/render/prepare_assets 26.95 |
| warm-up | 3.53 | 113.9 | pipelines/wait_ms 4650, pipelines/compile_ms 1274, pipelines/front_ms 1202, render/thread/ms 161, render/busy/ms 158, main/busy/ms 94.98, pipelines/driver_ms 71.64, stage/render/render 66.00, render/graph/ms 63.06, stage/main/post_update 49.91, encode//sum 42.20, stage/render/prepare_resources 35.97 |
| warm-up | 3.60 | 61.8 | pipelines/wait_ms 4929, pipelines/compile_ms 1150, pipelines/front_ms 1076, render/thread/ms 155, render/busy/ms 153, main/busy/ms 142, pipelines/driver_ms 74.52, stage/main/update 60.62, stage/render/prepare_resources 56.28, encode//sum 46.51, stage/main/post_update 35.07, stage/main/pre_update 33.47 |
| warm-up | 3.69 | 98.6 | pipelines/compile_ms 1553, pipelines/front_ms 1464, pipelines/wait_ms 970, main/busy/ms 108, render/thread/ms 98.84, render/busy/ms 97.03, pipelines/driver_ms 88.92, stage/main/update 64.65, stage/render/prepare_assets 55.67, stage/main/post_update 27.94, stage/render/prepare_resources 23.09, terrain/update/ms 19.34 |
| warm-up | 3.72 | 29.6 | pipelines/compile_ms 1421, pipelines/front_ms 1352, pipelines/wait_ms 340, render/thread/ms 79.24, render/busy/ms 77.76, pipelines/driver_ms 68.75, stage/render/prepare_assets 60.07, main/busy/ms 40.33, stage/main/post_update 19.42, terrain/update/ms 15.98, stage/main/update 13.66, stage/render/render 11.05 |
| warm-up | 3.76 | 35.3 | pipelines/compile_ms 537, pipelines/front_ms 513, render/thread/ms 60.52, render/busy/ms 59.06, main/busy/ms 55.54, stage/render/prepare_assets 35.14, pipelines/driver_ms 23.99, stage/main/post_update 17.93, stage/render/render 12.56, stage/main/update 11.14, terrain/update/ms 11.12, stage/main/pre_update 10.82 |
| warm-up | 3.79 | 29.8 | pipelines/compile_ms 752, pipelines/front_ms 722, main/busy/ms 64.00, render/thread/ms 61.44, render/busy/ms 59.88, pipelines/driver_ms 30.12, stage/render/prepare_assets 28.34, pipelines/wait_ms 19.07, stage/main/post_update 17.56, stage/render/render 13.55, stage/main/first 12.09, render/graph/ms 11.35 |
| warm-up | 3.82 | 35.3 | pipelines/compile_ms 995, pipelines/front_ms 962, encode//sum 62.82, render/thread/ms 58.89, main/busy/ms 58.88, render/busy/ms 57.02, pipelines/wait_ms 53.87, pipelines/driver_ms 33.15, stage/main/post_update 29.85, stage/render/render 22.23, render/graph/ms 19.77, stage/render/prepare_assets 17.62 |
| warm-up | 3.86 | 37.8 | pipelines/compile_ms 1200, pipelines/front_ms 1140, pipelines/wait_ms 198, main/busy/ms 86.76, render/thread/ms 66.29, render/busy/ms 64.36, encode//sum 62.09, pipelines/driver_ms 59.59, stage/main/update 55.74, stage/render/prepare_assets 29.85, stage/render/render 24.15, stage/main/post_update 23.23 |
| warm-up | 3.95 | 84.4 | pipelines/compile_ms 1361, pipelines/front_ms 1287, pipelines/wait_ms 162, pipelines/driver_ms 73.87, main/busy/ms 70.45, render/thread/ms 67.96, render/busy/ms 66.40, stage/main/update 56.55, stage/render/prepare_assets 48.47, stage/render/render 11.98, render/graph/ms 9.56, stage/main/post_update 7.37 |
| warm-up | 3.99 | 47.2 | pipelines/compile_ms 1355, pipelines/front_ms 1302, render/thread/ms 63.61, render/busy/ms 62.10, pipelines/driver_ms 52.17, stage/render/prepare_assets 45.38, main/busy/ms 20.04, pipelines/wait_ms 19.70, stage/render/render 13.46, render/graph/ms 11.77, stage/main/update 8.55, stage/render/graph_submit 8.15 |
| warm-up | 4.06 | 39.0 | pipelines/compile_ms 361, pipelines/front_ms 343, render/thread/ms 54.06, render/busy/ms 28.18, stage/render/prepare_views 23.78, render/acquire/ms 23.68, main/busy/ms 23.10, pipelines/driver_ms 17.96, stage/render/render 16.02, encode//sum 14.82, render/graph/ms 13.39, stage/main/update 12.00 |
| warm-up | 4.12 | 35.9 | pipelines/compile_ms 245, pipelines/front_ms 235, main/busy/ms 49.21, render/thread/ms 38.36, render/busy/ms 36.44, stage/main/update 23.85, stage/render/render 16.95, render/graph/ms 14.43, stage/main/post_update 13.13, stage/render/prepare_assets 12.36, stage/render/graph_submit 10.92, stream/drive/ms 10.04 |
| warm-up | 4.21 | 89.4 | pipelines/compile_ms 910, pipelines/front_ms 873, render/thread/ms 92.36, render/busy/ms 85.08, main/busy/ms 63.99, pipelines/driver_ms 36.83, stage/render/render 36.50, stage/render/prepare_assets 36.13, stage/main/update 29.01, render/graph/ms 28.44, stage/main/post_update 18.55, stage/render/graph_submit 17.36 |
| warm-up | 4.26 | 53.3 | pipelines/compile_ms 1645, pipelines/front_ms 1578, main/busy/ms 143, stage/main/update 111, render/thread/ms 97.10, render/busy/ms 90.39, pipelines/driver_ms 67.08, stage/render/prepare_assets 48.27, stage/render/render 31.62, render/graph/ms 24.26, stage/main/post_update 19.73, stream/drive/ms 16.79 |
| warm-up | 4.46 | 196.1 | pipelines/compile_ms 1847, pipelines/front_ms 1783, main/busy/ms 155, stage/main/update 136, render/thread/ms 113, render/busy/ms 112, stage/render/prepare_assets 71.21, pipelines/driver_ms 64.03, stage/render/prepare_resources 22.95, pipelines/wait_ms 18.80, stage/render/render 12.20, pipelines/lock_ms 10.70 |
| warm-up | 4.52 | 63.5 | pipelines/compile_ms 1078, pipelines/front_ms 1046, render/thread/ms 131, render/busy/ms 129, main/busy/ms 89.98, stage/render/prepare_assets 76.34, pipelines/wait_ms 73.05, stage/main/update 52.67, pipelines/driver_ms 32.07, stage/render/prepare_resources 24.99, stage/main/fixed_main_loop 15.00, stage/render/render 14.56 |
| warm-up | 4.61 | 84.7 | pipelines/compile_ms 234, pipelines/front_ms 225, render/thread/ms 123, render/busy/ms 122, main/busy/ms 107, pipelines/wait_ms 63.47, stage/main/post_update 44.92, stage/render/prepare_resources 41.80, stage/main/update 31.99, stage/render/prepare_assets 30.57, stage/render/prepare_write_phase_buffers 24.03, stage/main/fixed_main_loop 19.43 |
| warm-up | 4.66 | 53.6 | pipelines/compile_ms 1026, pipelines/front_ms 996, pipelines/wait_ms 139, render/thread/ms 125, render/busy/ms 124, main/busy/ms 114, stage/main/post_update 66.04, stage/render/prepare_resources 40.85, stage/main/update 31.54, pipelines/driver_ms 29.79, stage/render/prepare_write_phase_buffers 27.69, stage/render/prepare_assets 16.11 |
| warm-up | 4.75 | 90.9 | pipelines/compile_ms 2859, pipelines/front_ms 2775, pipelines/wait_ms 1335, render/thread/ms 127, render/busy/ms 126, main/busy/ms 118, pipelines/driver_ms 84.53, stage/main/post_update 75.35, stage/render/prepare_resources 35.96, stage/main/update 26.38, stage/render/prepare_assets 20.86, stage/render/render 18.89 |
| warm-up | 4.82 | 67.1 | pipelines/compile_ms 2414, pipelines/front_ms 2317, pipelines/wait_ms 1254, main/busy/ms 142, render/thread/ms 141, render/busy/ms 140, pipelines/driver_ms 96.86, stage/main/post_update 69.18, stage/render/prepare_resources 60.66, stage/main/update 60.44, stage/render/prepare_assets 23.29, stage/render/render 21.30 |
| warm-up | 4.90 | 79.8 | pipelines/compile_ms 1431, pipelines/front_ms 1358, pipelines/wait_ms 631, main/busy/ms 118, render/thread/ms 114, render/busy/ms 113, pipelines/driver_ms 73.30, stage/main/update 71.71, stage/render/prepare_assets 51.68, stage/main/post_update 32.67, stage/render/prepare_resources 27.78, stage/render/render 19.11 |
| warm-up | 4.95 | 48.9 | pipelines/compile_ms 2057, pipelines/front_ms 1996, pipelines/wait_ms 1430, render/thread/ms 87.38, render/busy/ms 85.96, main/busy/ms 71.35, pipelines/driver_ms 61.01, stage/render/prepare_assets 54.58, stage/main/update 41.74, stage/main/post_update 17.27, stage/render/render 11.74, render/graph/ms 9.75 |
| warm-up | 4.98 | 30.0 | pipelines/compile_ms 1455, pipelines/front_ms 1418, pipelines/wait_ms 871, main/busy/ms 82.43, stage/main/update 59.59, render/thread/ms 56.68, render/busy/ms 55.23, stream/drive/ms 45.21, pipelines/driver_ms 36.83, stage/render/prepare_assets 23.89, stage/render/render 13.36, stage/main/post_update 12.71 |
| warm-up | 5.05 | 71.5 | pipelines/compile_ms 678, pipelines/front_ms 663, main/busy/ms 95.89, stage/main/update 74.42, render/thread/ms 50.56, stream/drive/ms 50.35, render/busy/ms 48.52, pipelines/wait_ms 40.05, stage/render/render 19.48, render/graph/ms 16.88, stage/render/prepare_assets 14.49, pipelines/driver_ms 14.36 |
| warm-up | 5.09 | 39.0 | pipelines/compile_ms 803, pipelines/front_ms 785, main/busy/ms 60.35, render/thread/ms 51.07, render/busy/ms 49.15, stage/main/update 41.78, pipelines/wait_ms 26.75, pipelines/driver_ms 18.40, stage/render/prepare_assets 17.41, stage/render/render 17.34, render/graph/ms 14.67, stream/drive/ms 10.61 |
| warm-up | 5.12 | 30.8 | pipelines/compile_ms 667, pipelines/front_ms 642, render/thread/ms 50.86, render/busy/ms 49.00, main/busy/ms 47.77, stage/main/update 28.45, pipelines/driver_ms 24.22, stage/render/prepare_assets 13.32, stage/render/render 13.23, render/graph/ms 11.21, stage/main/post_update 10.89, stage/render/graph_submit 7.58 |
| warm-up | 5.18 | 31.1 | pipelines/compile_ms 220, pipelines/front_ms 215, render/thread/ms 50.07, render/busy/ms 48.69, main/busy/ms 40.62, stage/main/update 20.36, stage/render/render 15.53, render/graph/ms 13.61, stage/main/post_update 12.56, stage/render/prepare_assets 9.76, stage/render/graph_submit 9.31, encode//sum 8.31 |
| warm-up | 5.21 | 27.6 | pipelines/compile_ms 70.48, pipelines/front_ms 65.37, render/thread/ms 51.31, render/busy/ms 49.74, main/busy/ms 43.40, stage/main/update 19.85, stage/render/render 17.11, stage/main/post_update 16.27, render/graph/ms 15.09, stage/render/prepare_assets 11.02, stage/render/graph_submit 10.31, encode//sum 8.85 |
| warm-up | 5.23 | 28.6 | pipelines/compile_ms 44.55, render/thread/ms 43.54, render/busy/ms 41.95, pipelines/front_ms 41.19, main/busy/ms 37.79, stage/main/update 17.82, stage/render/render 14.44, stage/main/post_update 13.14, render/graph/ms 12.52, stage/render/prepare_assets 12.39, stage/render/graph_submit 8.73, encode//sum 8.68 |
| warm-up | 5.36 | 32.4 | pipelines/compile_ms 55.35, pipelines/front_ms 52.63, main/busy/ms 50.38, render/thread/ms 47.42, render/busy/ms 45.29, stage/main/update 22.55, stage/render/render 19.54, stage/main/post_update 18.24, render/graph/ms 13.88, stage/render/graph_submit 9.94, stage/render/prepare_assets 8.57, encode//sum 8.38 |
| warm-up | 5.38 | 27.2 | pipelines/compile_ms 485, pipelines/front_ms 470, render/thread/ms 50.27, render/busy/ms 48.19, main/busy/ms 44.57, pipelines/wait_ms 28.32, stage/main/update 19.80, stage/render/render 16.48, pipelines/driver_ms 15.36, stage/main/post_update 14.59, stage/render/prepare_assets 12.09, render/graph/ms 10.90 |
| warm-up | 5.44 | 40.6 | pipelines/compile_ms 394, pipelines/front_ms 385, main/busy/ms 43.17, render/thread/ms 37.01, render/busy/ms 35.79, stage/main/update 22.32, stream/drive/ms 14.00, stage/main/post_update 13.11, stage/render/render 11.55, render/graph/ms 9.82, pipelines/driver_ms 9.21, stage/render/prepare_resources 8.31 |
| warm-up | 5.47 | 31.2 | pipelines/compile_ms 1237, pipelines/front_ms 1208, render/thread/ms 52.83, render/busy/ms 51.49, main/busy/ms 38.96, pipelines/driver_ms 28.93, stage/render/prepare_resources 19.08, stage/main/update 16.55, stage/main/post_update 15.08, stage/render/render 12.17, render/graph/ms 10.36, encode//sum 7.13 |
| warm-up | 5.50 | 29.4 | pipelines/compile_ms 1233, pipelines/front_ms 1205, render/thread/ms 50.29, render/busy/ms 48.92, main/busy/ms 33.97, pipelines/driver_ms 28.66, stage/main/update 15.11, stage/render/prepare_assets 14.48, stage/render/render 12.17, stage/render/prepare_resources 12.02, stage/main/post_update 11.56, render/graph/ms 10.39 |
| warm-up | 5.97 | 28.0 | main/busy/ms 34.53, render/thread/ms 22.31, stage/main/update 21.98, render/busy/ms 21.38, stream/drive/ms 14.67, stage/render/render 10.16, render/graph/ms 8.62, encode//sum 8.12, stage/main/post_update 7.36, stage/render/graph_submit 5.60, stage/render/prepare_assets 4.59, encode/submit 2.94 |
| warm-up | 6.13 | 30.6 | render/thread/ms 39.27, render/busy/ms 38.25, main/busy/ms 26.63, stage/render/prepare_assets 19.35, stage/main/update 11.85, stage/render/render 9.53, render/graph/ms 7.95, stage/main/post_update 7.06, encode//sum 6.71, stage/render/graph_submit 5.46, stream/drive/ms 3.74, encode/submit 3.52 |
| warm-up | 6.23 | 25.9 | render/thread/ms 37.80, render/busy/ms 36.77, main/busy/ms 35.06, stage/main/update 19.48, stage/render/prepare_assets 13.79, stage/main/post_update 10.21, stage/render/render 8.72, render/graph/ms 7.38, encode//sum 6.26, stage/render/graph_submit 4.62, stage/render/prepare_meshes 4.52, stream/drive/ms 4.17 |
| warm-up | 6.30 | 29.2 | render/thread/ms 37.63, render/busy/ms 36.48, main/busy/ms 32.72, stage/main/update 19.77, stage/render/render 13.93, render/graph/ms 11.87, encode//sum 9.90, stage/render/prepare_assets 9.88, stage/main/post_update 8.41, stage/render/graph_submit 8.02, encode/submit 5.19, stage/render/prepare_meshes 4.38 |
| warm-up | 6.39 | 35.1 | render/thread/ms 40.73, render/busy/ms 39.68, main/busy/ms 39.18, stage/main/update 20.72, stage/render/prepare_assets 14.72, stage/main/post_update 10.90, stage/render/render 10.26, render/graph/ms 8.79, encode//sum 7.53, stage/render/graph_submit 5.73, stream/drive/ms 4.96, stage/render/prepare_meshes 4.64 |
| warm-up | 6.43 | 41.7 | render/thread/ms 64.51, render/busy/ms 62.81, main/busy/ms 49.10, stage/main/update 23.26, stage/render/render 22.72, render/graph/ms 20.43, stage/main/post_update 17.22, stage/render/prepare_assets 16.02, stage/render/graph_submit 15.84, encode//sum 9.26, encode/submit 8.83, stage/render/prepare_meshes 7.33 |
| warm-up | 6.46 | 27.4 | render/thread/ms 60.29, render/busy/ms 58.65, main/busy/ms 40.46, stage/render/render 25.45, render/graph/ms 23.13, stage/main/update 19.61, stage/render/graph_submit 18.37, stage/main/post_update 14.36, stage/render/prepare_assets 12.13, encode//sum 9.57, encode/submit 9.32, encode/finish_wall 9.13 |
| warm-up | 6.50 | 36.5 | pipelines/compile_ms 156, pipelines/front_ms 150, render/thread/ms 46.69, render/busy/ms 45.63, main/busy/ms 32.03, stage/render/prepare_assets 19.28, stage/main/update 16.38, stage/render/render 14.22, render/graph/ms 12.53, stage/main/post_update 10.03, stage/render/graph_submit 9.58, encode//sum 8.33 |
| warm-up | 6.65 | 34.0 | main/busy/ms 50.86, render/thread/ms 36.84, render/busy/ms 35.39, stage/main/update 34.60, stage/render/render 18.61, render/graph/ms 16.19, stream/drive/ms 15.58, stage/render/graph_submit 12.76, stage/main/post_update 10.97, encode//sum 8.85, encode/finish_wall 7.83, encode/pass:shadow_point_light/sum 7.34 |
| warm-up | 6.69 | 39.4 | pipelines/compile_ms 142, pipelines/front_ms 137, render/thread/ms 53.79, render/busy/ms 52.44, main/busy/ms 46.48, stage/main/update 27.04, stage/render/render 23.71, render/graph/ms 20.91, stage/render/graph_submit 16.37, stage/main/post_update 13.51, stage/render/prepare_assets 11.84, encode//sum 10.62 |
| warm-up | 6.72 | 26.7 | pipelines/compile_ms 142, pipelines/front_ms 137, render/thread/ms 53.32, render/busy/ms 52.05, main/busy/ms 39.61, stage/main/update 22.69, stage/render/render 22.59, render/graph/ms 20.31, stage/render/graph_submit 16.52, stage/render/prepare_assets 14.89, stage/main/post_update 10.44, encode/finish_wall 9.27 |
| warm-up | 6.75 | 33.1 | render/thread/ms 45.13, render/busy/ms 44.09, main/busy/ms 38.40, stage/main/update 20.78, stage/render/render 17.04, stage/render/prepare_assets 16.38, render/graph/ms 15.57, stage/render/graph_submit 12.73, stage/main/post_update 10.00, encode/finish_wall 9.00, encode/pass:shadow_point_light/sum 8.60, stream/drive/ms 7.94 |
| measure | 2.58 | 27.7 | render/thread/ms 36.43, stage/render/render 28.20, render/busy/ms 19.53, main/busy/ms 19.17, render/present/ms 16.71, render/graph/ms 10.88, stage/main/post_update 10.46, encode//sum 8.09, stage/render/graph_submit 7.64, stage/main/update 4.70, encode/submit 4.51, encode/finish_wall 3.21 |
| measure | 18.32 | 28.7 | main/busy/ms 35.53, stage/main/update 21.96, render/thread/ms 20.59, render/busy/ms 19.62, stage/render/render 10.42, stage/main/post_update 9.57, render/graph/ms 9.17, stage/render/graph_submit 6.71, encode//sum 6.46, encode/submit 4.20, stage/render/prepare_assets 3.43, encode/finish_wall 2.55 |
| measure | 23.04 | 26.1 | render/thread/ms 31.63, render/busy/ms 30.47, main/busy/ms 29.46, stage/render/render 14.89, stage/main/update 13.35, render/graph/ms 13.22, stage/main/post_update 11.72, encode//sum 10.93, stage/render/graph_submit 9.65, stage/render/prepare_assets 8.64, encode/submit 6.01, encode/finish_wall 3.71 |
| measure | 59.17 | 33.0 | render/thread/ms 36.47, render/busy/ms 35.35, main/busy/ms 23.96, stage/render/prepare_assets 15.90, stage/render/render 11.36, stage/main/post_update 10.67, render/graph/ms 9.90, encode//sum 9.09, stage/render/graph_submit 6.68, stage/main/update 6.09, encode/submit 4.00, stage/main/fixed_main_loop 3.49 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=batched-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1449 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **84%** (min 64%) · 2625 MHz (min 2625) · memory 8101 MHz · 80.2 W · 75 °C (max 79) · VRAM 6207 MiB · throttled 100% of samples

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
- `shadow.lamp.batched_pass=on (command line: --set)`
- `shadow.lamp.cache.enabled=on (command line: --set)`
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
