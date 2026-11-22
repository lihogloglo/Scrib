# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.max_lamps=16 (command line: --set); shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6594 frames over 65.6 s · **median 9.84 ms (102 fps)** · mean 9.96 · p95 11.52 · p99 13.41 (1% low 75 fps) · worst 35.71 ms · 25 spikes over 1.5× median · 21.0 stutters/min (work 23.8/min)
- warm-up: 353 frames over 9.1 s, worst 2484.6 ms
- hitches (> 25.0 ms): **2** measured, 42 in warm-up
- timed GPU passes: 6.62 ms of a 9.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.84 ms, limit 11 ms
- **FAIL** frame_ms: 23 stutters (> 1.5x moving median of 60) in 1.09 min = 21.02/min, limit 1/min
- **FAIL** worst frame 35.71 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.905 | 2.035 | 2.067 | 0.94 | 0.178 |
| main_opaque_pass_3d | 1.532 | 1.636 | 2.380 | 0.94 | 0.065 |
| early prepass | 0.484 | 0.516 | 0.754 | 0.94 | 0.090 |
| bloom | 0.297 | 0.318 | 0.325 | 0.94 | 0.065 |
| atmosphere_luts | 0.293 | 0.313 | 0.376 | 0.94 | 0.003 |
| water_velocity | 0.211 | 0.225 | 0.246 | 0.94 | 0.002 |
| water_sspr | 0.189 | 0.202 | 0.204 | 0.94 | 0.002 |
| ssao | 0.176 | 0.188 | 0.191 | 0.94 | 0.008 |
| upscaling | 0.120 | 0.128 | 0.135 | 0.94 | 0.004 |
| atmos_froxel_inject | 0.120 | 0.128 | 0.130 | 0.94 | 0.001 |
| point_shadow_atlas | 0.117 | 0.211 | 0.428 | 0.56 | 0.207 |
| water_fft | 0.115 | 0.123 | 0.127 | 0.94 | 0.003 |
| uniform_allocation | 0.111 | 0.119 | 0.143 | 0.94 | 0.021 |
| clouds_composite | 0.098 | 0.105 | 0.109 | 0.94 | 0.002 |
| bin_unpacking | 0.089 | 0.095 | 0.116 | 0.94 | 0.017 |
| gpu_instance_materialization | 0.086 | 0.091 | 0.096 | 0.94 | 0.004 |
| tonemapping | 0.081 | 0.087 | 0.092 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.067 | 0.072 | 0.133 | 0.94 | 0.009 |
| clustering | 0.060 | 0.064 | 0.068 | 0.94 | 0.020 |
| clouds_trace | 0.052 | 0.055 | 0.077 | 0.94 | 0.001 |
| early_mesh_preprocessing | 0.050 | 0.054 | 0.070 | 0.94 | 0.003 |
| render_sky | 0.049 | 0.053 | 0.054 | 0.94 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.416 | 0.418 | 0.11 | 0.001 |
| surface_trail | 0.035 | 0.057 | 0.057 | 0.62 | 0.002 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.030 | 0.94 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.024 | 0.043 | 0.063 | 0.56 | 0.009 |
| lightprobe_radiance_map | 0.023 | 0.219 | 0.231 | 0.11 | 0.004 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.94 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.94 | 0.002 |
| range_unpacking | 0.021 | 0.022 | 0.026 | 0.94 | 0.003 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.94 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.043 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.031 | 0.94 | 0.188 |
| point_shadow_atlas/uniform_allocation | 0.012 | 0.021 | 0.055 | 0.56 | 0.006 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.94 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.94 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.009 | 0.016 | 0.035 | 0.56 | 0.004 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.56 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.94 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.780 | 10.193 | 18.75 | 57895.4 |
| render/busy/ms | 8.257 | 9.670 | 18.28 | 54448.5 |
| main/busy/ms | 8.132 | 9.408 | 31.49 | 53621.3 |
| stage/render/render | 5.125 | 6.053 | 11.30 | 33796.7 |
| stage/main/post_update | 4.579 | 5.428 | 10.06 | 30197.0 |
| render/graph/ms | 4.451 | 5.305 | 9.87 | 29352.8 |
| encode//sum | 3.301 | 4.198 | 7.52 | 21764.9 |
| stage/render/graph_submit | 3.145 | 3.873 | 7.03 | 20741.3 |
| encode/submit | 1.988 | 2.734 | 4.77 | 13112.0 |
| stage/main/update | 1.844 | 2.332 | 24.72 | 12160.2 |
| stage/render/camera_driver | 1.243 | 1.516 | 3.13 | 8198.0 |
| encode/finish_wall | 1.231 | 1.623 | 3.13 | 8119.2 |
| encode/pass:early prepass/sum | 0.830 | 1.099 | 1.87 | 5476.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.824 | 1.100 | 2.01 | 5435.7 |
| encode/pass:shadow_directional_light/sum | 0.818 | 1.233 | 2.42 | 5391.8 |
| stage/render/prepare_write_phase_buffers | 0.593 | 0.889 | 2.19 | 3908.9 |
| bigspace/total/ms | 0.566 | 1.344 | 2.12 | 3731.3 |
| stage/main/fixed_main_loop | 0.520 | 0.963 | 2.41 | 3431.1 |
| stage/render/prepare_assets | 0.509 | 0.658 | 10.56 | 3353.7 |
| stage/render/prepare_meshes | 0.433 | 0.536 | 3.70 | 2854.8 |
| stage/render/prepare_resources | 0.426 | 0.510 | 2.02 | 2806.9 |
| render/present/ms | 0.425 | 1.164 | 1.97 | 2802.5 |
| stage/main/pre_update | 0.390 | 0.559 | 1.88 | 2571.4 |
| stage/main/first | 0.367 | 0.486 | 5.22 | 2420.7 |
| stage/render/prepare_flush | 0.325 | 0.533 | 2.37 | 2144.7 |
| controller/feed/ms | 0.324 | 0.418 | 0.95 | 2135.7 |
| stage/render/prepare_bind_groups | 0.303 | 0.495 | 1.47 | 2000.9 |
| stage/render/prepare_batch_phases | 0.278 | 0.427 | 0.84 | 1834.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.272 | 0.448 | 1.00 | 1790.4 |
| stage/main/last | 0.251 | 0.299 | 0.83 | 1652.6 |
| bigspace/low_precision/ms | 0.247 | 0.783 | 1.67 | 1629.9 |
| stream/drive/ms | 0.201 | 0.286 | 10.17 | 1322.6 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.200 | 1.000 | 1.64 | 1321.3 |
| stage/render/create_views | 0.199 | 0.271 | 0.67 | 1309.8 |
| stage/render/extract_commands | 0.187 | 0.221 | 0.56 | 1230.6 |
| bigspace/high_precision/ms | 0.168 | 0.714 | 1.60 | 1105.2 |
| anim/evaluate/ms | 0.163 | 0.299 | 2.36 | 1071.5 |
| anim/evaluate_cpu/ms | 0.154 | 0.196 | 0.66 | 1014.2 |
| stage/render/specialize | 0.153 | 0.244 | 1.66 | 1007.2 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.151 | 0.874 | 1.84 | 996.0 |
| stream/tick/ms | 0.147 | 0.217 | 0.54 | 968.6 |
| stage/render/prepare_views | 0.140 | 0.182 | 1.51 | 922.5 |
| render/acquire/ms | 0.098 | 0.128 | 1.46 | 644.5 |
| stage/render/queue | 0.097 | 0.173 | 1.83 | 640.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.092 | 0.147 | 0.85 | 604.6 |
| stage/render/phase_sort | 0.083 | 0.116 | 0.39 | 546.8 |
| controller/write/ms | 0.074 | 0.102 | 0.42 | 489.9 |
| controller/move/ms | 0.071 | 0.178 | 0.58 | 470.6 |
| terrain/update/ms | 0.059 | 0.293 | 1.40 | 392.2 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.056 | 0.000 | 2.96 | 370.1 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.075 | 0.29 | 314.1 |
| physics/step/ms | 0.047 | 0.128 | 0.66 | 310.2 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.50 | 213.5 |
| pipelines/compile_ms | 0.025 | 0.000 | 77.83 | 165.7 |
| physics/insert/ms | 0.024 | 0.289 | 0.84 | 160.6 |
| pipelines/front_ms | 0.024 | 0.000 | 74.37 | 159.3 |
| stage/render/cleanup | 0.023 | 0.032 | 0.46 | 154.4 |
| terrain/select/ms | 0.022 | 0.260 | 0.93 | 145.6 |
| weather/map/ms | 0.017 | 0.023 | 0.31 | 113.9 |
| far/select/ms | 0.015 | 0.165 | 0.54 | 97.9 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.44 | 85.5 |
| anim/apply/ms | 0.011 | 0.019 | 0.37 | 75.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.46 | 70.7 |
| weather/eval/ms | 0.011 | 0.014 | 0.11 | 70.3 |
| script/update/ms | 0.008 | 0.012 | 0.26 | 52.8 |
| controller/avoid/ms | 0.006 | 0.012 | 0.04 | 40.0 |
| stream/despawn/ms | 0.006 | 0.000 | 1.23 | 36.6 |
| script/input/ms | 0.005 | 0.006 | 0.19 | 31.8 |
| script/ui/ms | 0.004 | 0.005 | 0.31 | 23.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.14 | 22.1 |
| stream/upload/ms | 0.003 | 0.000 | 1.46 | 20.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.46 | 6.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.28 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 145 | 156 | 148 |
| crowd/c1/joints/count | count | 5515 | 5991 | 5613 |
| crowd/c1/meshes/count | count | 1164 | 1295 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 205 |
| crowd/joints_extracted/count | count | 678 | 5525 | 602 |
| crowd/joints_uploaded/count | count | 3604 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.41 | 1.49 | 0.65 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.44 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.46 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.96 | 1.17 |
| encode/pass:early prepass/max | ms | 0.83 | 1.87 | 0.52 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 2.01 | 1.02 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.85 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.00 | 0.43 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.74 | 1.84 | 0.49 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.45 | 1.64 | 0.94 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.09 | 2.96 | 1.02 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.42 | 1.17 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.50 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.60 | 31.49 | 8.66 |
| gpu/timed/ms | ms | 7.78 | 9.49 | 8.66 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 153 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 153 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2478 | 3527 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2478 | 3527 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 56 | 399 | 27 |
| phases/shadow/bins | count | 759 | 1271 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 98.3 k |
| phases/shadow/items | count | 2257 | 2954 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 48 | 136 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2679 | 3183 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 15 | 273 | 43 |
| preprocess/bind_groups_created | count | 10 | 261 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 6 |
| preprocess/bind_groups_reused | count | 138 | 245 | 123 |
| process/cpu_usage | % | 25.48 | 39.90 | 33.16 |
| process/mem_usage | GiB | 5.07 | 5.37 | 5.37 |
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
| shadow_atlas/lamps | count | 8 | 17 | 0 |
| shadow_atlas/live_occ_pct | % | 2.73 | 30.86 | 0 |
| shadow_atlas/live_tiles | count | 13 | 56 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 147 | 445 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 17 | 0 |
| shadow_atlas/render/lamps | count | 8 | 17 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 16 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 15 | 56 | 0 |
| shadow_atlas/resizes | count | 31 | 40 | 40 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 102 | 0 |
| shadow_atlas/views | count | 15 | 56 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 95 | 13 |
| skin/blocks_skipped/count | count | 146 | 206 | 192 |
| skin/blocks_walked/count | count | 14 | 28 | 13 |
| skin/joint_bytes_written/bytes | bytes | 47.6 k | 812.2 k | 39.9 k |
| skin/joints_written/count | count | 744 | 12.7 k | 624 |
| skin/upload_writes/count | count | 8 | 23 | 10 |
| started_load_count |  | 6441 | 6565 | 6565 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.48 | 48.10 | 33.12 |
| system/mem_usage | % | 56.11 | 56.88 | 56.88 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 116 | 450 | 85 |
| views/all/bins | count | 2918 | 3321 | 2948 |
| views/all/count | count | 18 | 59 | 3 |
| views/all/instance_batches | count | 12 | 22 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 196.6 k |
| views/all/items | count | 8009 | 10.2 k | 7964 |
| views/all/pipelines | count | 95 | 179 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.0 k | 231.0 k | 205.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 27 |
| views/cascade/bins | count | 711 | 851 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2118 | 2427 | 2022 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 41 | 382 | 0 |
| views/lamp_face/bins | count | 60 | 536 | 0 |
| views/lamp_face/count | count | 15 | 56 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 100 | 615 | 0 |
| views/lamp_face/pipelines | count | 22 | 109 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 73 | 25.1 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2008 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5909 | 7492 | 5942 |
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
| anim/bones_written/count | count | 50.57 | 244 | 333.4 k |
| anim/events/count | count | 0.07 | 3 | 491 |
| assets/Image/added | count | 0.09 | 36 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.44 | 273 | 2874 |
| assets/Mesh/removed | count | 0.74 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.13 | 10 | 7477 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1154 | 1489 |
| encode//count | count | 27.91 | 30 | 184.1 k |
| encode/bind_walk/init_textures | count | 9108 | 23.0 k | 60.06 M |
| encode/bind_walk/init_textures_skipped | count | 59.3 k | 79.1 k | 391.26 M |
| encode/bind_walk/inits | count | 556 | 770 | 3.67 M |
| encode/bind_walk/inits_skipped | count | 175 | 265 | 1.16 M |
| encode/bind_walk/merge_textures | count | 9046 | 23.0 k | 59.65 M |
| encode/bind_walk/merge_textures_skipped | count | 9696 | 26.4 k | 63.94 M |
| encode/bind_walk/merges | count | 102 | 182 | 671.5 k |
| encode/bind_walk/merges_skipped | count | 150 | 228 | 991.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6594 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6594 |
| encode/pass:early prepass/count | count | 1 | 1 | 6594 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6594 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6594 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6594 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.19 | 1 | 1271 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.33 | 1 | 2191 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 321 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6594 |
| encode/queue/command_buffers | count | 38.50 | 42 | 253.9 k |
| encode/queue/submits | count | 2.01 | 4 | 13.3 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.45 M | 15.28 G |
| encode/queue/write_buffer/calls | count | 268 | 457 | 1.77 M |
| encode/queue/write_buffer_with/bytes | bytes | 159.7 k | 1.27 M | 1.05 G |
| encode/queue/write_buffer_with/calls | count | 10.29 | 828 | 67.9 k |
| encode/queue/write_texture/bytes | bytes | 2001 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6594 |
| physics/inserts | count | 0.34 | 95 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
| physics/steps | count | 0.60 | 2 | 3939 |
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
| stream/deactivate/items | count | 0.03 | 31 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5134 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1126 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 44, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2484.6 | main/busy/ms 2456, stage/main/post_update 2412, stage/main/update 33.00, stage/main/pre_update 7.08, stage/main/last 1.04, surface/advance_cpu_ms 0.78, stage/main/first 0.66, stream/drive/ms 0.48, stage/main/fixed_main_loop 0.43, surface/cell_upload_ms 0.35, stream/tick/ms 0.11, bigspace/total/ms 0.08 |
| warm-up | 3.24 | 759.2 | main/busy/ms 2589, stage/main/post_update 2435, render/thread/ms 726, render/busy/ms 585, stage/render/prepare_assets 521, stage/render/prepare_views 137, stage/main/update 137, render/acquire/ms 137, stage/render/create_views 28.94, stage/render/render 26.29, render/graph/ms 21.50, stage/render/camera_driver 19.48 |
| warm-up | 3.42 | 173.6 | render/thread/ms 882, render/busy/ms 646, stage/render/prepare_assets 568, main/busy/ms 164, stage/render/prepare_views 138, render/acquire/ms 137, stage/render/render 126, stage/main/update 112, pipelines/compile_ms 100, render/present/ms 99.08, pipelines/driver_ms 80.53, stage/main/post_update 37.98 |
| warm-up | 3.46 | 42.0 | pipelines/compile_ms 792, pipelines/front_ms 548, pipelines/driver_ms 244, pipelines/wait_ms 200, render/thread/ms 197, main/busy/ms 160, stage/render/render 133, render/present/ms 109, stage/main/update 90.05, render/busy/ms 87.34, stage/main/post_update 56.65, stage/render/prepare_assets 47.45 |
| warm-up | 3.69 | 226.6 | pipelines/wait_ms 3733, pipelines/compile_ms 2940, pipelines/front_ms 2370, pipelines/driver_ms 570, render/thread/ms 171, main/busy/ms 168, render/busy/ms 156, stage/main/update 91.76, stage/render/prepare_assets 81.84, stage/render/render 76.50, render/graph/ms 61.57, stage/render/camera_driver 57.43 |
| warm-up | 3.73 | 41.6 | pipelines/wait_ms 3534, pipelines/compile_ms 2248, pipelines/front_ms 1842, pipelines/driver_ms 406, render/thread/ms 171, render/busy/ms 168, stage/render/prepare_assets 99.40, stage/render/render 60.64, main/busy/ms 59.29, render/graph/ms 58.33, stage/render/camera_driver 41.78, stage/main/post_update 22.79 |
| warm-up | 3.81 | 33.1 | pipelines/compile_ms 351, pipelines/front_ms 325, pipelines/wait_ms 155, render/thread/ms 53.74, render/busy/ms 52.25, main/busy/ms 47.15, pipelines/driver_ms 25.16, stage/main/update 22.98, stage/main/post_update 21.00, stage/render/prepare_flush 16.48, stage/render/render 15.86, stage/render/prepare_resources 14.26 |
| warm-up | 3.86 | 51.4 | pipelines/compile_ms 1095, pipelines/front_ms 1035, pipelines/wait_ms 177, main/busy/ms 106, render/thread/ms 80.02, stage/main/update 78.70, render/busy/ms 78.13, pipelines/driver_ms 59.77, stage/render/prepare_flush 23.11, stage/main/post_update 22.79, stage/render/prepare_assets 21.15, stage/render/render 20.07 |
| warm-up | 3.97 | 115.9 | pipelines/compile_ms 2292, pipelines/front_ms 2201, pipelines/wait_ms 209, render/thread/ms 104, render/busy/ms 100, pipelines/driver_ms 91.09, main/busy/ms 86.70, stage/main/update 64.37, stage/render/prepare_assets 59.76, stage/render/render 21.96, render/graph/ms 17.28, stage/main/post_update 15.72 |
| warm-up | 4.02 | 44.1 | pipelines/compile_ms 1767, pipelines/front_ms 1706, pipelines/wait_ms 124, render/thread/ms 85.48, render/busy/ms 81.98, pipelines/driver_ms 60.76, stage/render/prepare_assets 60.65, main/busy/ms 26.39, stage/render/render 18.14, render/graph/ms 14.44, stage/main/update 10.49, pipelines/lock_ms 10.02 |
| warm-up | 4.11 | 33.8 | main/busy/ms 111, stage/main/update 99.17, render/thread/ms 50.28, render/busy/ms 48.64, pipelines/compile_ms 40.27, pipelines/front_ms 38.52, stage/render/render 14.10, render/graph/ms 11.95, stage/render/prepare_assets 11.90, stage/render/prepare_resources 9.14, stage/render/graph_submit 8.23, encode//sum 7.33 |
| warm-up | 4.26 | 143.7 | pipelines/compile_ms 1668, pipelines/front_ms 1629, main/busy/ms 117, stage/main/update 99.79, render/thread/ms 84.27, render/busy/ms 82.73, stage/render/prepare_assets 49.12, pipelines/driver_ms 38.85, stage/render/render 14.29, render/graph/ms 11.80, stage/render/prepare_resources 9.26, stage/main/post_update 9.03 |
| warm-up | 4.37 | 88.2 | pipelines/compile_ms 176, pipelines/front_ms 169, main/busy/ms 82.46, render/thread/ms 79.91, render/busy/ms 78.09, stage/main/update 57.73, stage/render/prepare_assets 40.86, pipelines/wait_ms 31.74, stage/render/render 17.20, stage/main/post_update 17.09, render/graph/ms 14.87, stage/render/graph_submit 8.27 |
| warm-up | 4.44 | 69.7 | pipelines/compile_ms 712, pipelines/front_ms 689, render/thread/ms 106, render/busy/ms 104, main/busy/ms 76.36, stage/render/prepare_assets 65.46, stage/main/update 57.65, pipelines/wait_ms 33.91, pipelines/driver_ms 22.61, stage/render/render 16.80, render/graph/ms 14.59, stage/main/post_update 12.58 |
| warm-up | 4.46 | 26.7 | pipelines/compile_ms 793, pipelines/front_ms 768, render/thread/ms 67.98, render/busy/ms 64.86, main/busy/ms 63.88, stage/render/prepare_assets 31.10, stage/main/update 27.29, pipelines/driver_ms 24.86, stage/render/render 20.31, render/graph/ms 16.88, pipelines/wait_ms 16.36, stage/main/post_update 16.03 |
| warm-up | 4.52 | 60.1 | pipelines/compile_ms 1596, pipelines/front_ms 1545, pipelines/wait_ms 179, main/busy/ms 65.75, pipelines/driver_ms 51.47, render/thread/ms 50.86, render/busy/ms 47.93, stage/main/update 27.17, stage/render/render 23.07, render/graph/ms 19.55, stage/main/post_update 17.60, stage/render/graph_submit 13.49 |
| warm-up | 4.57 | 46.9 | pipelines/compile_ms 1809, pipelines/front_ms 1749, pipelines/wait_ms 188, render/thread/ms 68.27, render/busy/ms 66.97, pipelines/driver_ms 59.89, main/busy/ms 55.63, stage/main/update 35.07, stream/drive/ms 21.68, stage/render/prepare_assets 20.82, stage/render/render 16.23, render/graph/ms 14.13 |
| warm-up | 4.66 | 89.2 | pipelines/compile_ms 824, pipelines/front_ms 790, render/thread/ms 124, render/busy/ms 122, main/busy/ms 88.11, stage/render/prepare_assets 57.65, stage/main/update 51.55, pipelines/driver_ms 34.74, stage/main/post_update 25.53, pipelines/wait_ms 25.03, stream/drive/ms 21.89, stage/render/prepare_write_phase_buffers 20.45 |
| warm-up | 4.72 | 61.2 | pipelines/compile_ms 1491, pipelines/front_ms 1435, render/thread/ms 131, render/busy/ms 128, main/busy/ms 102, pipelines/wait_ms 95.50, stage/render/prepare_assets 73.76, stage/main/update 64.41, pipelines/driver_ms 56.26, stage/main/post_update 25.88, stage/render/prepare_write_phase_buffers 17.80, stage/render/render 15.59 |
| warm-up | 4.87 | 150.4 | pipelines/compile_ms 2963, pipelines/front_ms 2831, render/thread/ms 144, render/busy/ms 142, pipelines/driver_ms 132, pipelines/wait_ms 127, stage/render/prepare_assets 111, main/busy/ms 76.88, stage/main/update 52.59, stage/render/render 14.78, render/graph/ms 12.57, stage/main/post_update 12.01 |
| warm-up | 4.91 | 38.6 | pipelines/compile_ms 2525, pipelines/front_ms 2403, render/thread/ms 131, render/busy/ms 130, pipelines/driver_ms 121, stage/render/prepare_assets 83.82, main/busy/ms 62.67, pipelines/wait_ms 42.42, stage/main/update 40.40, stage/render/render 25.69, render/graph/ms 21.36, stream/drive/ms 16.16 |
| warm-up | 4.94 | 31.8 | pipelines/compile_ms 913, pipelines/front_ms 878, main/busy/ms 80.36, stage/main/update 64.01, render/thread/ms 59.18, render/busy/ms 57.71, stream/drive/ms 38.40, pipelines/driver_ms 34.26, pipelines/wait_ms 30.46, stage/render/render 25.09, render/graph/ms 20.97, stage/render/graph_submit 14.68 |
| warm-up | 4.99 | 45.9 | pipelines/compile_ms 230, pipelines/front_ms 224, main/busy/ms 81.01, stage/main/update 61.98, stream/drive/ms 48.82, render/thread/ms 43.95, render/busy/ms 42.63, pipelines/wait_ms 21.26, stage/render/prepare_assets 14.52, stage/render/render 12.31, stage/main/post_update 12.19, render/graph/ms 10.61 |
| warm-up | 5.04 | 49.1 | main/busy/ms 71.95, render/thread/ms 50.51, stage/main/update 50.13, render/busy/ms 48.97, pipelines/compile_ms 27.27, pipelines/front_ms 26.08, stream/drive/ms 25.90, stage/render/render 14.38, stage/main/post_update 14.12, render/graph/ms 12.28, stage/render/prepare_assets 11.01, encode//sum 9.74 |
| warm-up | 5.08 | 40.8 | pipelines/compile_ms 1301, pipelines/front_ms 1270, render/thread/ms 52.47, render/busy/ms 51.08, main/busy/ms 47.91, stage/main/update 31.52, pipelines/driver_ms 30.87, stage/render/prepare_assets 17.35, stage/render/render 13.16, render/graph/ms 11.23, encode//sum 9.43, stage/main/post_update 8.98 |
| warm-up | 5.13 | 32.9 | pipelines/compile_ms 67.64, pipelines/front_ms 63.06, render/thread/ms 49.47, render/busy/ms 47.81, main/busy/ms 41.32, stage/main/update 22.41, stage/render/render 16.11, render/graph/ms 13.86, stage/main/post_update 12.47, stage/render/graph_submit 8.84, encode//sum 8.28, stage/render/prepare_assets 6.31 |
| warm-up | 5.26 | 28.6 | pipelines/compile_ms 557, pipelines/front_ms 538, render/thread/ms 44.87, main/busy/ms 44.31, render/busy/ms 42.82, pipelines/driver_ms 18.23, stage/main/update 17.27, stage/render/render 16.64, stage/main/post_update 16.44, render/graph/ms 14.67, stage/render/graph_submit 10.63, encode//sum 8.08 |
| warm-up | 5.32 | 40.6 | pipelines/compile_ms 618, pipelines/front_ms 601, main/busy/ms 44.72, render/thread/ms 36.38, render/busy/ms 35.03, stage/main/update 26.08, pipelines/driver_ms 17.32, stage/render/prepare_assets 12.81, stage/render/render 12.46, stream/drive/ms 12.34, stage/main/post_update 10.97, render/graph/ms 10.66 |
| warm-up | 5.74 | 26.3 | pipelines/compile_ms 197, pipelines/front_ms 191, render/thread/ms 35.65, render/busy/ms 34.42, main/busy/ms 26.42, stage/render/prepare_assets 13.91, stage/main/post_update 10.80, stage/render/render 10.58, stage/main/update 9.61, render/graph/ms 8.60, encode//sum 7.17, pipelines/driver_ms 6.12 |
| warm-up | 5.88 | 28.4 | pipelines/compile_ms 194, pipelines/front_ms 190, main/busy/ms 38.65, render/thread/ms 26.27, render/busy/ms 25.17, stage/main/update 24.95, stream/drive/ms 15.36, stage/render/render 13.06, render/graph/ms 11.34, encode//sum 8.74, stage/main/post_update 8.15, stage/render/graph_submit 7.00 |
| warm-up | 6.02 | 27.8 | render/thread/ms 42.08, render/busy/ms 40.72, main/busy/ms 33.97, stage/main/update 16.36, stage/render/prepare_assets 16.14, stage/render/render 15.00, render/graph/ms 13.02, stage/main/post_update 11.52, stage/render/graph_submit 9.11, encode//sum 8.56, encode/submit 6.30, stream/drive/ms 4.09 |
| warm-up | 6.05 | 30.4 | render/thread/ms 41.52, render/busy/ms 40.58, main/busy/ms 27.88, stage/render/prepare_assets 24.06, stage/main/update 14.21, stage/render/render 8.33, stage/main/post_update 7.60, render/graph/ms 6.96, encode//sum 6.62, stream/drive/ms 5.36, stage/render/graph_submit 4.65, stage/main/fixed_main_loop 2.64 |
| warm-up | 6.26 | 30.7 | render/thread/ms 42.86, render/busy/ms 41.55, main/busy/ms 39.52, stage/main/update 18.63, stage/main/post_update 15.98, stage/render/render 14.64, render/graph/ms 12.67, encode//sum 10.44, stage/render/prepare_assets 9.46, stage/render/graph_submit 7.45, stage/render/prepare_meshes 5.65, stage/render/camera_driver 4.80 |
| warm-up | 6.31 | 33.9 | render/thread/ms 40.11, render/busy/ms 39.00, main/busy/ms 38.63, stage/main/update 20.98, stage/render/prepare_assets 14.15, stage/render/render 10.80, stage/main/post_update 9.59, render/graph/ms 9.09, encode//sum 7.27, stage/render/graph_submit 5.68, stream/drive/ms 5.01, stage/render/prepare_bind_groups 4.01 |
| warm-up | 6.35 | 41.8 | pipelines/compile_ms 154, pipelines/front_ms 149, render/thread/ms 63.77, render/busy/ms 62.39, main/busy/ms 47.23, stage/render/render 26.96, render/graph/ms 24.93, stage/main/update 23.17, stage/render/graph_submit 20.04, stage/main/post_update 15.44, stage/render/prepare_assets 13.87, encode/finish_wall 13.22 |
| warm-up | 6.38 | 26.1 | pipelines/compile_ms 159, pipelines/front_ms 153, render/thread/ms 60.51, render/busy/ms 59.17, main/busy/ms 40.34, stage/render/render 29.11, render/graph/ms 27.19, stage/render/graph_submit 22.57, stage/main/update 18.91, encode/finish_wall 15.58, encode/pass:point_shadow_atlas_live_indirect/sum 15.38, stage/main/post_update 14.27 |
| warm-up | 6.41 | 35.2 | render/thread/ms 45.95, render/busy/ms 45.05, main/busy/ms 30.08, stage/render/prepare_assets 19.86, stage/render/render 14.74, stage/main/update 14.67, render/graph/ms 13.33, pipelines/compile_ms 11.52, pipelines/front_ms 10.93, stage/render/graph_submit 10.74, stage/main/post_update 8.99, encode/finish_wall 7.80 |
| warm-up | 6.44 | 27.2 | render/thread/ms 50.58, render/busy/ms 49.51, main/busy/ms 39.49, stage/main/update 19.46, stage/render/render 19.08, render/graph/ms 17.34, stage/render/prepare_assets 16.60, stage/main/post_update 14.33, stage/render/graph_submit 13.98, encode/finish_wall 9.23, encode//sum 7.66, pipelines/compile_ms 6.39 |
| warm-up | 6.56 | 30.2 | main/busy/ms 44.15, render/thread/ms 32.04, render/busy/ms 30.63, stage/main/update 30.05, stage/render/render 15.30, stream/drive/ms 14.19, render/graph/ms 13.19, stage/main/post_update 9.78, stage/render/graph_submit 9.59, encode//sum 7.57, encode/finish_wall 5.26, encode/pass:point_shadow_atlas_live_indirect/sum 4.90 |
| warm-up | 6.59 | 33.4 | pipelines/compile_ms 129, pipelines/front_ms 125, main/busy/ms 48.91, render/thread/ms 46.55, render/busy/ms 45.03, stage/main/update 30.09, stage/render/render 19.32, render/graph/ms 16.80, stage/render/graph_submit 12.43, stage/main/post_update 12.38, stage/render/prepare_assets 10.54, encode//sum 9.05 |
| warm-up | 6.62 | 31.9 | pipelines/compile_ms 146, pipelines/front_ms 140, render/thread/ms 48.66, render/busy/ms 47.22, main/busy/ms 45.31, stage/main/update 27.97, stage/render/render 16.30, stage/render/prepare_assets 16.12, render/graph/ms 13.99, stage/render/graph_submit 10.47, stage/main/post_update 9.88, encode//sum 8.25 |
| warm-up | 6.65 | 29.3 | render/thread/ms 41.46, render/busy/ms 40.48, main/busy/ms 38.09, stage/main/update 21.29, stage/render/prepare_assets 17.61, pipelines/compile_ms 16.80, pipelines/front_ms 15.26, stage/render/render 11.07, render/graph/ms 9.54, stage/main/post_update 9.25, stream/drive/ms 7.83, stage/render/graph_submit 6.76 |
| measure | 18.36 | 35.7 | main/busy/ms 42.11, stage/main/update 27.08, render/thread/ms 22.69, render/busy/ms 21.74, stage/render/render 11.23, render/graph/ms 9.81, stage/main/post_update 9.58, stage/render/graph_submit 6.81, encode//sum 6.72, encode/submit 4.37, stage/render/prepare_assets 4.02, stage/render/camera_driver 2.84 |
| measure | 59.18 | 29.1 | render/thread/ms 37.43, render/busy/ms 36.44, main/busy/ms 27.12, stage/render/prepare_assets 16.19, stage/main/post_update 14.40, stage/render/render 11.15, render/graph/ms 9.64, encode//sum 9.06, stage/render/graph_submit 5.34, stage/main/update 5.25, stage/render/camera_driver 4.12, stage/main/fixed_main_loop 3.47 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.soft_pcf_radius=0 --set shadow.lamp.atlas.max_lamps=16 --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=pcf0lamps16-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **84%** (min 60%) · 2619 MHz (min 2610) · memory 8101 MHz · 84.2 W · 79 °C (max 84) · VRAM 5521 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.max_lamps=16 (command line: --set)`
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
