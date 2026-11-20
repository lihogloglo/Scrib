# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=48 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6569 frames over 65.6 s · **median 9.87 ms (101 fps)** · mean 9.99 · p95 11.61 · p99 13.75 (1% low 73 fps) · worst 32.63 ms · 33 spikes over 1.5× median · 32.0 stutters/min (work 27.4/min)
- warm-up: 353 frames over 9.0 s, worst 2545.2 ms
- hitches (> 25.0 ms): **4** measured, 43 in warm-up
- timed GPU passes: 6.48 ms of a 9.99 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.87 ms, limit 11 ms
- **FAIL** frame_ms: 35 stutters (> 1.5x moving median of 60) in 1.09 min = 31.99/min, limit 1/min
- pass worst frame 32.63 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.978 | 2.187 | 2.614 | 0.90 | 0.173 |
| main_opaque_pass_3d | 1.229 | 1.358 | 1.937 | 0.90 | 0.064 |
| early prepass | 0.484 | 0.535 | 0.778 | 0.90 | 0.092 |
| bloom | 0.311 | 0.344 | 0.659 | 0.90 | 0.065 |
| atmosphere_luts | 0.302 | 0.334 | 0.509 | 0.90 | 0.003 |
| water_velocity | 0.219 | 0.242 | 0.370 | 0.90 | 0.002 |
| water_sspr | 0.199 | 0.220 | 0.227 | 0.90 | 0.002 |
| ssao | 0.178 | 0.197 | 0.203 | 0.90 | 0.007 |
| point_shadow_atlas | 0.144 | 0.263 | 0.479 | 0.55 | 0.125 |
| atmos_froxel_inject | 0.125 | 0.138 | 0.130 | 0.90 | 0.001 |
| upscaling | 0.124 | 0.137 | 0.163 | 0.90 | 0.004 |
| water_fft | 0.118 | 0.130 | 0.127 | 0.90 | 0.003 |
| uniform_allocation | 0.113 | 0.125 | 0.143 | 0.90 | 0.022 |
| clouds_composite | 0.097 | 0.107 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.089 | 0.099 | 0.115 | 0.90 | 0.017 |
| gpu_instance_materialization | 0.088 | 0.097 | 0.100 | 0.90 | 0.004 |
| tonemapping | 0.084 | 0.092 | 0.094 | 0.90 | 0.004 |
| main_transmissive_pass_3d | 0.070 | 0.077 | 0.142 | 0.90 | 0.008 |
| clustering | 0.062 | 0.068 | 0.068 | 0.90 | 0.019 |
| early_mesh_preprocessing | 0.053 | 0.059 | 0.073 | 0.90 | 0.004 |
| clouds_trace | 0.050 | 0.056 | 0.078 | 0.90 | 0.001 |
| render_sky | 0.050 | 0.055 | 0.055 | 0.90 | 0.002 |
| clouds_resolve | 0.045 | 0.050 | 0.051 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.036 | 0.464 | 0.779 | 0.08 | 0.001 |
| surface_trail | 0.036 | 0.059 | 0.057 | 0.60 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.031 | 0.032 | 0.90 | 0.001 |
| water_snapshot | 0.024 | 0.026 | 0.035 | 0.90 | 0.002 |
| range_unpacking | 0.022 | 0.024 | 0.028 | 0.90 | 0.003 |
| water_mips | 0.021 | 0.024 | 0.024 | 0.90 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.018 | 0.033 | 0.067 | 0.55 | 0.009 |
| lightprobe_radiance_map | 0.017 | 0.219 | 0.228 | 0.08 | 0.004 |
| per_view_shadow_pass | 0.017 | 0.019 | 0.027 | 0.90 | 0.005 |
| main_transparent_pass_3d | 0.017 | 0.018 | 0.030 | 0.90 | 0.189 |
| point_shadow_atlas/early_mesh_preprocessing | 0.016 | 0.030 | 0.058 | 0.55 | 0.005 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.90 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.013 | 0.023 | 0.043 | 0.55 | 0.006 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.90 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.90 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.90 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.55 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.08 | 0.004 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.90 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.788 | 10.131 | 24.33 | 57730.1 |
| render/busy/ms | 8.235 | 9.562 | 19.00 | 54095.0 |
| main/busy/ms | 8.118 | 9.277 | 28.96 | 53327.4 |
| stage/render/render | 5.111 | 6.111 | 20.50 | 33573.3 |
| stage/main/post_update | 4.593 | 5.390 | 10.16 | 30172.4 |
| render/graph/ms | 4.402 | 5.253 | 12.67 | 28919.6 |
| encode//sum | 3.260 | 4.020 | 14.43 | 21413.3 |
| stage/render/graph_submit | 3.139 | 3.929 | 9.62 | 20619.0 |
| encode/submit | 2.005 | 2.868 | 5.93 | 13172.8 |
| stage/main/update | 1.855 | 2.350 | 23.60 | 12183.4 |
| encode/finish_wall | 1.230 | 1.590 | 6.50 | 8081.6 |
| stage/render/camera_driver | 1.200 | 1.420 | 2.94 | 7883.6 |
| encode/pass:early prepass/sum | 0.832 | 1.086 | 5.67 | 5465.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.822 | 1.086 | 1.76 | 5402.3 |
| encode/pass:shadow_directional_light/sum | 0.822 | 1.234 | 3.31 | 5399.4 |
| stage/render/prepare_write_phase_buffers | 0.613 | 0.964 | 1.74 | 4024.4 |
| bigspace/total/ms | 0.545 | 1.294 | 2.27 | 3577.0 |
| stage/render/prepare_assets | 0.518 | 0.651 | 11.61 | 3406.0 |
| stage/main/fixed_main_loop | 0.490 | 0.870 | 1.99 | 3219.9 |
| render/present/ms | 0.461 | 1.282 | 14.69 | 3029.3 |
| stage/render/prepare_meshes | 0.439 | 0.556 | 3.49 | 2882.8 |
| stage/render/prepare_resources | 0.424 | 0.525 | 5.07 | 2786.1 |
| stage/main/pre_update | 0.377 | 0.521 | 5.21 | 2473.7 |
| stage/main/first | 0.367 | 0.480 | 5.62 | 2407.6 |
| controller/feed/ms | 0.329 | 0.430 | 0.96 | 2160.8 |
| stage/render/prepare_bind_groups | 0.303 | 0.489 | 1.95 | 1990.9 |
| stage/render/prepare_flush | 0.295 | 0.525 | 1.47 | 1939.1 |
| stage/render/prepare_batch_phases | 0.273 | 0.417 | 0.69 | 1793.6 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.271 | 1.064 | 6.42 | 1781.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.264 | 0.434 | 0.89 | 1734.3 |
| stage/main/last | 0.256 | 0.308 | 0.81 | 1680.8 |
| bigspace/low_precision/ms | 0.237 | 0.724 | 1.59 | 1554.2 |
| stage/render/create_views | 0.219 | 0.312 | 0.64 | 1439.1 |
| stream/drive/ms | 0.202 | 0.293 | 10.67 | 1325.1 |
| stage/render/extract_commands | 0.188 | 0.227 | 0.56 | 1233.6 |
| anim/evaluate/ms | 0.168 | 0.300 | 1.56 | 1102.8 |
| bigspace/high_precision/ms | 0.160 | 0.655 | 1.93 | 1049.2 |
| anim/evaluate_cpu/ms | 0.160 | 0.203 | 0.51 | 1048.7 |
| stage/render/specialize | 0.159 | 0.250 | 1.82 | 1042.0 |
| stream/tick/ms | 0.149 | 0.226 | 0.59 | 977.5 |
| stage/render/prepare_views | 0.136 | 0.182 | 0.55 | 893.4 |
| pipelines/compile_ms | 0.135 | 0.000 | 136.18 | 890.0 |
| pipelines/front_ms | 0.132 | 0.000 | 132.86 | 864.9 |
| stage/render/queue | 0.098 | 0.168 | 1.79 | 643.0 |
| render/acquire/ms | 0.092 | 0.125 | 0.49 | 605.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.090 | 0.145 | 0.80 | 594.3 |
| stage/render/phase_sort | 0.083 | 0.117 | 0.51 | 543.9 |
| controller/write/ms | 0.073 | 0.100 | 0.43 | 482.5 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.072 | 0.795 | 1.35 | 472.5 |
| controller/move/ms | 0.063 | 0.162 | 0.46 | 416.5 |
| terrain/update/ms | 0.059 | 0.285 | 1.82 | 385.4 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.055 | 0.289 | 3.35 | 359.4 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.38 | 301.5 |
| physics/step/ms | 0.038 | 0.077 | 0.51 | 248.2 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.40 | 206.0 |
| physics/insert/ms | 0.025 | 0.294 | 0.90 | 166.4 |
| stage/render/cleanup | 0.023 | 0.035 | 0.32 | 153.7 |
| terrain/select/ms | 0.022 | 0.252 | 0.61 | 141.8 |
| weather/map/ms | 0.017 | 0.023 | 0.23 | 112.6 |
| far/select/ms | 0.015 | 0.168 | 0.43 | 98.8 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.021 | 0.15 | 81.8 |
| anim/apply/ms | 0.012 | 0.022 | 0.31 | 78.6 |
| weather/eval/ms | 0.011 | 0.014 | 0.07 | 70.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.28 | 70.0 |
| script/update/ms | 0.008 | 0.012 | 0.43 | 52.8 |
| controller/avoid/ms | 0.006 | 0.012 | 0.14 | 41.4 |
| stream/despawn/ms | 0.006 | 0.000 | 1.06 | 36.2 |
| script/input/ms | 0.005 | 0.006 | 0.11 | 32.3 |
| pipelines/driver_ms | 0.004 | 0.000 | 3.54 | 25.1 |
| stream/spawn/ms | 0.004 | 0.000 | 1.18 | 23.5 |
| script/ui/ms | 0.004 | 0.005 | 0.04 | 23.3 |
| stream/upload/ms | 0.003 | 0.000 | 1.19 | 18.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 7.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 2 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1300 | 2262 | 593 |
| crowd/c0/meshes/count | count | 355 | 646 | 156 |
| crowd/c1/count | count | 145 | 159 | 148 |
| crowd/c1/joints/count | count | 5477 | 6140 | 5603 |
| crowd/c1/meshes/count | count | 1150 | 1356 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 215 | 206 |
| crowd/joints_extracted/count | count | 690 | 6932 | 394 |
| crowd/joints_uploaded/count | count | 3613 | 6186 | 5807 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.40 | 5.74 | 0.53 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.15 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.01 |
| encode/finish_critical | ms | 0.98 | 6.42 | 0.93 |
| encode/pass:early prepass/max | ms | 0.84 | 5.67 | 0.93 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 1.76 | 0.77 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.80 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.89 | 0.27 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.77 | 6.42 | 0.38 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.18 | 1.35 | 1.03 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 0.97 | 3.35 | 0.35 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 3.31 | 0.87 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.40 | 0.02 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.51 | 28.96 | 8.17 |
| gpu/timed/ms | ms | 8.01 | 9.59 | 8.17 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 769 | 971 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2485 | 3555 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 769 | 971 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2485 | 3555 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 216 | 8 |
| phases/shadow/bins | count | 762 | 1192 | 671 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2202 | 2813 | 2035 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 96 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2702 | 3183 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 330 | 331 | 331 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 14 | 274 | 11 |
| preprocess/bind_groups_created | count | 9 | 264 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 129 | 227 | 123 |
| process/cpu_usage | % | 25.06 | 38.76 | 22.60 |
| process/mem_usage | GiB | 5.14 | 5.44 | 5.44 |
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
| shadow_atlas/alloc_failures | count | 24 | 24 | 24 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 1 | 0 |
| shadow_atlas/lamps | count | 8 | 20 | 0 |
| shadow_atlas/live_occ_pct | % | 37.50 | 100 | 0 |
| shadow_atlas/live_tiles | count | 13 | 56 | 0 |
| shadow_atlas/mover_draws | count | 9 | 12 | 0 |
| shadow_atlas/movers | count | 141 | 497 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 9 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 20 | 0 |
| shadow_atlas/render/live_direct | count | 9 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 15 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 16 | 57 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 33.98 | 0 |
| shadow_atlas/static_tiles | count | 42 | 107 | 0 |
| shadow_atlas/views | count | 16 | 57 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 121 | 9 |
| skin/blocks_skipped/count | count | 146 | 204 | 197 |
| skin/blocks_walked/count | count | 14 | 28 | 9 |
| skin/joint_bytes_written/bytes | bytes | 48.6 k | 808.2 k | 26.1 k |
| skin/joints_written/count | count | 760 | 12.6 k | 408 |
| skin/upload_writes/count | count | 6 | 27 | 9 |
| started_load_count |  | 6443 | 6567 | 6567 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.50 | 46.41 | 30.57 |
| system/mem_usage | % | 55.47 | 56.52 | 56.45 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 85 | 275 | 66 |
| views/all/bins | count | 2907 | 3194 | 2961 |
| views/all/count | count | 19 | 60 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 7998 | 10.2 k | 7977 |
| views/all/pipelines | count | 79 | 145 | 66 |
| views/all/pipelines_distinct | count | 64 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 231.0 k | 154.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 8 |
| views/cascade/bins | count | 719 | 850 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2126 | 2390 | 2035 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 2 | 205 | 0 |
| views/lamp_face/bins | count | 14 | 465 | 0 |
| views/lamp_face/count | count | 16 | 57 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 55 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 28 | 603 | 0 |
| views/lamp_face/pipelines | count | 7 | 72 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 4 | 25.0 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2014 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7530 | 5942 |
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
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 51.84 | 320 | 340.5 k |
| anim/events/count | count | 0.08 | 3 | 521 |
| assets/Image/added | count | 0.09 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.44 | 282 | 2874 |
| assets/Mesh/removed | count | 0.74 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.14 | 10 | 7475 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1063 | 1489 |
| encode//count | count | 27.91 | 30 | 183.4 k |
| encode/bind_walk/init_textures | count | 8817 | 22.7 k | 57.92 M |
| encode/bind_walk/init_textures_skipped | count | 52.1 k | 78.4 k | 342.09 M |
| encode/bind_walk/inits | count | 566 | 773 | 3.72 M |
| encode/bind_walk/inits_skipped | count | 143 | 229 | 942.4 k |
| encode/bind_walk/merge_textures | count | 8754 | 22.7 k | 57.50 M |
| encode/bind_walk/merge_textures_skipped | count | 10.1 k | 26.5 k | 66.33 M |
| encode/bind_walk/merges | count | 107 | 187 | 702.1 k |
| encode/bind_walk/merges_skipped | count | 132 | 225 | 864.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6569 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6569 |
| encode/pass:early prepass/count | count | 1 | 1 | 6569 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6569 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6569 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6569 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.34 | 1 | 2241 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.18 | 1 | 1194 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.06 | 1 | 367 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6569 |
| encode/queue/command_buffers | count | 38.51 | 42 | 252.9 k |
| encode/queue/submits | count | 2.01 | 4 | 13.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.31 M | 15.24 G |
| encode/queue/write_buffer/calls | count | 274 | 454 | 1.80 M |
| encode/queue/write_buffer_with/bytes | bytes | 158.2 k | 1.29 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 10.30 | 855 | 67.7 k |
| encode/queue/write_texture/bytes | bytes | 2009 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6569 |
| physics/inserts | count | 0.34 | 92 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 175 | 2335 |
| physics/steps | count | 0.60 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 12 |
| pipelines/created | count | 0.00 | 1 | 13 |
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
| stream/upload/bytes | bytes | 5153 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1130 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2545.2 | main/busy/ms 2512, stage/main/post_update 2470, stage/main/update 31.05, stage/main/pre_update 6.99, stage/main/last 1.22, surface/advance_cpu_ms 0.74, stage/main/first 0.45, stream/drive/ms 0.40, stage/main/fixed_main_loop 0.40, surface/cell_upload_ms 0.28, stream/tick/ms 0.09, bigspace/total/ms 0.09 |
| warm-up | 3.10 | 557.8 | main/busy/ms 2661, stage/main/post_update 2503, render/thread/ms 522, render/busy/ms 430, stage/render/prepare_assets 387, stage/main/update 140, stage/render/prepare_views 89.55, render/acquire/ms 89.37, stage/render/render 17.03, stage/render/create_views 16.62, render/graph/ms 13.92, stage/render/camera_driver 12.20 |
| warm-up | 3.17 | 64.9 | render/thread/ms 570, render/busy/ms 476, stage/render/prepare_assets 418, main/busy/ms 176, stage/main/update 116, stage/render/prepare_views 90.40, render/acquire/ms 90.02, stage/main/post_update 45.09, terrain/update/ms 25.86, stage/render/render 23.05, render/graph/ms 18.44, stage/render/create_views 17.23 |
| warm-up | 3.21 | 40.5 | pipelines/compile_ms 155, pipelines/front_ms 135, pipelines/wait_ms 119, render/thread/ms 85.81, main/busy/ms 84.47, render/busy/ms 77.48, stage/main/post_update 63.24, stage/render/render 37.50, stage/render/prepare_assets 31.71, terrain/update/ms 31.54, render/graph/ms 29.03, stage/render/graph_submit 20.89 |
| warm-up | 3.33 | 123.3 | pipelines/wait_ms 4628, pipelines/compile_ms 1171, pipelines/front_ms 1085, main/busy/ms 122, render/thread/ms 107, render/busy/ms 98.82, pipelines/driver_ms 85.41, stage/main/post_update 61.72, stage/render/render 37.09, stage/main/last 35.71, stage/render/prepare_resources 32.40, render/graph/ms 28.64 |
| warm-up | 3.44 | 108.3 | pipelines/wait_ms 6215, pipelines/compile_ms 2387, pipelines/front_ms 2226, main/busy/ms 183, render/thread/ms 175, render/busy/ms 172, pipelines/driver_ms 161, stage/main/update 61.70, stage/main/fixed_main_loop 56.28, stage/render/prepare_resources 49.90, stage/main/last 35.53, stage/render/render 26.55 |
| warm-up | 3.52 | 84.2 | pipelines/wait_ms 1870, pipelines/compile_ms 1789, pipelines/front_ms 1663, render/thread/ms 173, render/busy/ms 171, main/busy/ms 146, pipelines/driver_ms 125, stage/render/prepare_assets 74.44, stage/main/update 69.66, stage/main/fixed_main_loop 49.79, stage/render/render 26.21, render/graph/ms 23.89 |
| warm-up | 3.56 | 32.7 | pipelines/compile_ms 705, pipelines/front_ms 667, pipelines/wait_ms 293, render/thread/ms 97.17, render/busy/ms 95.07, stage/render/prepare_assets 76.28, main/busy/ms 46.56, pipelines/driver_ms 37.96, stage/main/post_update 20.34, stage/main/update 17.53, terrain/update/ms 13.93, encode//sum 12.68 |
| warm-up | 3.59 | 29.9 | pipelines/compile_ms 770, pipelines/front_ms 741, pipelines/wait_ms 131, render/thread/ms 58.16, render/busy/ms 55.36, main/busy/ms 39.09, stage/render/prepare_assets 33.20, pipelines/driver_ms 28.72, stage/main/post_update 17.71, stage/main/update 15.98, stage/render/render 14.88, render/graph/ms 12.72 |
| warm-up | 3.69 | 53.0 | pipelines/compile_ms 492, pipelines/front_ms 471, main/busy/ms 89.92, render/thread/ms 72.11, render/busy/ms 70.08, stage/main/update 68.57, pipelines/wait_ms 51.12, stage/render/prepare_assets 40.49, pipelines/driver_ms 20.54, stage/render/render 12.92, stage/main/post_update 12.82, render/graph/ms 10.36 |
| warm-up | 3.77 | 86.0 | pipelines/compile_ms 1651, pipelines/front_ms 1596, render/thread/ms 84.20, render/busy/ms 82.50, main/busy/ms 76.80, stage/main/update 62.28, stage/render/prepare_assets 60.91, pipelines/wait_ms 56.75, pipelines/driver_ms 55.21, stage/render/render 13.63, render/graph/ms 11.40, stage/main/post_update 9.37 |
| warm-up | 3.84 | 64.9 | pipelines/compile_ms 1557, pipelines/front_ms 1503, render/thread/ms 83.87, render/busy/ms 82.55, pipelines/driver_ms 53.67, stage/render/prepare_assets 42.01, stage/render/prepare_resources 26.32, main/busy/ms 18.13, stage/render/render 11.12, render/graph/ms 9.58, pipelines/wait_ms 9.33, stage/main/update 8.32 |
| warm-up | 3.92 | 39.0 | pipelines/compile_ms 62.69, pipelines/front_ms 58.87, render/thread/ms 51.13, render/busy/ms 46.69, main/busy/ms 34.45, stage/render/render 19.03, stage/main/post_update 15.49, render/graph/ms 13.93, stage/main/update 13.92, stage/render/prepare_assets 12.16, stage/render/graph_submit 7.89, encode//sum 7.16 |
| warm-up | 3.96 | 35.9 | pipelines/compile_ms 436, pipelines/front_ms 419, main/busy/ms 116, stage/main/update 87.17, render/thread/ms 62.74, render/busy/ms 60.59, stage/render/prepare_assets 23.99, pipelines/driver_ms 17.85, stage/render/render 16.51, stage/main/pre_update 15.31, render/graph/ms 13.53, encode//sum 9.30 |
| warm-up | 4.13 | 172.5 | pipelines/compile_ms 1737, pipelines/front_ms 1686, main/busy/ms 117, render/thread/ms 96.15, render/busy/ms 94.39, stage/main/update 87.35, pipelines/driver_ms 50.69, stage/render/prepare_assets 50.07, stage/render/render 36.53, render/graph/ms 34.19, encode//sum 32.96, stage/render/graph_submit 30.33 |
| warm-up | 4.19 | 61.7 | pipelines/compile_ms 1469, pipelines/front_ms 1429, render/thread/ms 126, render/busy/ms 125, stage/render/render 74.57, main/busy/ms 73.85, render/graph/ms 72.62, stage/render/graph_submit 59.56, stage/main/update 56.77, stage/render/prepare_assets 41.02, pipelines/driver_ms 40.73, pipelines/wait_ms 39.10 |
| warm-up | 4.26 | 67.6 | pipelines/compile_ms 422, pipelines/front_ms 407, render/thread/ms 107, render/busy/ms 105, main/busy/ms 90.55, stage/main/update 75.42, stage/render/render 55.48, render/graph/ms 52.80, stage/render/graph_submit 39.30, pipelines/wait_ms 36.17, pipelines/lock_ms 35.65, stage/render/prepare_assets 34.60 |
| warm-up | 4.34 | 86.0 | pipelines/compile_ms 1360, pipelines/front_ms 1316, pipelines/wait_ms 125, render/thread/ms 125, render/busy/ms 122, main/busy/ms 116, stage/main/update 66.33, pipelines/driver_ms 44.35, stage/render/prepare_assets 41.39, stage/render/render 30.59, stage/main/post_update 29.06, stage/render/prepare_flush 26.29 |
| warm-up | 4.38 | 33.7 | pipelines/compile_ms 1557, pipelines/front_ms 1506, pipelines/wait_ms 388, render/thread/ms 96.78, main/busy/ms 96.43, render/busy/ms 94.23, pipelines/driver_ms 50.88, stage/main/update 46.93, stage/render/render 29.44, stage/main/post_update 28.40, stage/render/prepare_flush 26.36, render/graph/ms 24.56 |
| warm-up | 4.42 | 43.6 | pipelines/compile_ms 569, pipelines/front_ms 549, pipelines/wait_ms 275, main/busy/ms 65.77, render/thread/ms 60.03, render/busy/ms 58.66, stage/main/update 35.16, stream/drive/ms 26.56, stage/render/prepare_assets 25.87, pipelines/driver_ms 20.59, stage/main/fixed_main_loop 16.39, stage/render/render 12.88 |
| warm-up | 4.51 | 86.7 | pipelines/compile_ms 1714, pipelines/front_ms 1635, render/thread/ms 109, main/busy/ms 105, render/busy/ms 104, pipelines/wait_ms 98.27, pipelines/driver_ms 79.18, stage/main/update 67.38, stage/render/prepare_assets 33.98, stream/drive/ms 27.54, stage/render/prepare_resources 16.21, stage/main/fixed_main_loop 16.05 |
| warm-up | 4.57 | 62.5 | pipelines/compile_ms 2371, pipelines/front_ms 2274, pipelines/wait_ms 239, main/busy/ms 118, render/thread/ms 114, render/busy/ms 108, pipelines/driver_ms 96.81, stage/main/update 86.22, stage/render/prepare_assets 42.87, stage/render/render 22.48, render/graph/ms 18.08, stage/main/post_update 16.07 |
| warm-up | 4.68 | 106.2 | pipelines/compile_ms 1463, pipelines/front_ms 1425, pipelines/wait_ms 184, render/thread/ms 104, render/busy/ms 101, main/busy/ms 95.10, stage/render/prepare_assets 61.62, stage/main/update 61.33, pipelines/driver_ms 37.83, stage/render/render 25.50, render/graph/ms 21.45, stage/main/post_update 19.63 |
| warm-up | 4.75 | 78.9 | pipelines/compile_ms 982, pipelines/front_ms 958, render/thread/ms 114, render/busy/ms 112, main/busy/ms 88.20, stage/render/prepare_assets 62.77, stage/main/update 52.36, pipelines/wait_ms 32.76, pipelines/driver_ms 23.86, stage/render/render 22.84, stage/main/post_update 22.78, stream/drive/ms 21.33 |
| warm-up | 4.81 | 51.0 | pipelines/compile_ms 904, pipelines/front_ms 876, main/busy/ms 98.18, render/thread/ms 90.14, render/busy/ms 87.99, stage/main/update 69.78, stage/render/prepare_assets 43.47, stream/drive/ms 40.49, pipelines/driver_ms 28.08, stage/render/render 17.50, stage/main/post_update 16.73, render/graph/ms 15.20 |
| warm-up | 4.83 | 29.4 | pipelines/compile_ms 673, pipelines/front_ms 651, main/busy/ms 89.17, stage/main/update 69.59, render/thread/ms 56.01, render/busy/ms 54.54, stream/drive/ms 47.73, pipelines/driver_ms 21.33, stage/render/prepare_assets 20.35, stage/render/render 14.54, render/graph/ms 12.35, stage/main/post_update 10.61 |
| warm-up | 4.89 | 56.2 | pipelines/compile_ms 1234, pipelines/front_ms 1196, main/busy/ms 65.94, stage/main/update 49.55, render/thread/ms 42.01, render/busy/ms 40.75, pipelines/driver_ms 38.47, stream/drive/ms 28.28, stage/render/prepare_assets 14.31, stage/render/render 11.34, render/graph/ms 9.53, stage/main/post_update 9.43 |
| warm-up | 5.07 | 32.1 | pipelines/compile_ms 525, pipelines/front_ms 510, render/thread/ms 47.09, render/busy/ms 45.43, main/busy/ms 42.45, stage/render/render 24.99, stage/main/post_update 23.73, render/graph/ms 23.16, stage/render/graph_submit 19.55, pipelines/driver_ms 15.18, stage/main/update 11.13, encode//sum 7.71 |
| warm-up | 5.11 | 38.2 | pipelines/compile_ms 564, pipelines/front_ms 541, render/thread/ms 64.73, main/busy/ms 64.33, render/busy/ms 61.10, stage/render/render 33.26, stage/main/post_update 31.88, render/graph/ms 29.31, stage/render/graph_submit 23.98, stage/main/update 22.71, pipelines/driver_ms 22.46, stream/drive/ms 13.74 |
| warm-up | 5.15 | 38.8 | pipelines/compile_ms 243, pipelines/front_ms 228, render/thread/ms 67.85, render/busy/ms 64.43, main/busy/ms 57.04, stage/main/post_update 25.77, stage/main/update 24.04, stage/render/render 20.56, stage/render/prepare_assets 18.43, render/graph/ms 16.78, pipelines/driver_ms 14.91, stream/drive/ms 13.46 |
| warm-up | 5.20 | 57.4 | render/thread/ms 86.71, render/busy/ms 85.28, main/busy/ms 56.87, stage/main/post_update 35.22, pipelines/compile_ms 33.37, stage/render/prepare_resources 32.48, pipelines/front_ms 32.09, stage/render/prepare_assets 17.85, stage/main/update 14.83, stage/render/render 14.68, render/graph/ms 12.82, stage/render/graph_submit 9.68 |
| warm-up | 5.24 | 32.1 | pipelines/compile_ms 432, pipelines/front_ms 419, render/thread/ms 79.37, render/busy/ms 78.02, main/busy/ms 55.07, stage/render/prepare_resources 32.49, stage/main/post_update 27.57, stage/render/render 14.96, stage/main/update 13.37, render/graph/ms 13.14, pipelines/driver_ms 13.12, stage/render/prepare_write_phase_buffers 11.71 |
| warm-up | 5.28 | 42.8 | pipelines/compile_ms 2124, pipelines/front_ms 2075, render/thread/ms 50.14, render/busy/ms 49.10, pipelines/driver_ms 48.84, main/busy/ms 36.51, stage/render/prepare_assets 23.28, pipelines/wait_ms 13.54, stage/main/update 12.53, stage/render/render 10.57, stage/main/post_update 9.22, render/graph/ms 8.98 |
| warm-up | 5.45 | 31.9 | main/busy/ms 34.15, render/thread/ms 24.66, render/busy/ms 23.58, pipelines/compile_ms 22.07, stage/main/update 20.95, pipelines/front_ms 20.90, stream/drive/ms 13.31, stage/main/post_update 9.18, stage/render/render 8.57, render/graph/ms 7.02, encode//sum 6.18, stage/render/graph_submit 4.63 |
| warm-up | 5.68 | 27.0 | main/busy/ms 30.29, render/thread/ms 21.56, stage/main/update 20.59, render/busy/ms 19.64, stream/drive/ms 12.68, stage/render/render 10.07, render/graph/ms 7.72, stage/main/post_update 6.58, encode//sum 6.30, stage/render/graph_submit 4.94, stage/render/prepare_assets 4.33, encode/submit 2.83 |
| warm-up | 5.98 | 27.1 | render/thread/ms 43.03, render/busy/ms 41.72, main/busy/ms 39.29, stage/main/update 19.80, stage/render/render 17.08, render/graph/ms 15.01, stage/main/post_update 14.15, encode//sum 10.86, stage/render/graph_submit 10.54, stage/render/prepare_assets 8.07, encode/submit 6.80, stage/render/camera_driver 4.30 |
| warm-up | 6.00 | 25.4 | pipelines/compile_ms 141, pipelines/front_ms 138, render/thread/ms 44.72, render/busy/ms 43.33, main/busy/ms 42.42, stage/main/update 21.15, stage/render/render 16.27, render/graph/ms 14.24, stage/main/post_update 13.48, stage/render/graph_submit 9.78, encode//sum 9.14, stage/render/prepare_assets 8.56 |
| warm-up | 6.04 | 32.6 | pipelines/compile_ms 141, pipelines/front_ms 138, render/thread/ms 45.55, render/busy/ms 44.62, main/busy/ms 38.11, stage/main/update 19.88, stage/render/prepare_assets 17.60, stage/main/post_update 10.05, stage/render/render 8.92, render/graph/ms 7.49, encode//sum 6.17, stage/render/prepare_meshes 5.72 |
| warm-up | 6.07 | 33.2 | render/thread/ms 55.11, render/busy/ms 53.59, main/busy/ms 40.11, stage/main/update 20.83, stage/render/prepare_assets 17.29, stage/render/render 14.25, stage/main/post_update 12.44, render/graph/ms 12.02, encode//sum 9.54, pipelines/compile_ms 9.25, pipelines/front_ms 8.53, stage/render/graph_submit 7.60 |
| warm-up | 6.12 | 30.0 | render/thread/ms 38.61, render/busy/ms 37.70, main/busy/ms 30.87, stage/render/prepare_assets 16.71, stage/main/update 16.56, stage/render/render 8.58, stage/main/post_update 7.74, render/graph/ms 7.25, encode//sum 6.55, stage/render/graph_submit 4.48, stage/render/prepare_meshes 4.21, stream/drive/ms 3.69 |
| warm-up | 6.26 | 36.5 | main/busy/ms 45.92, render/thread/ms 33.85, render/busy/ms 32.77, stage/main/update 29.39, stream/drive/ms 12.90, stage/main/post_update 12.24, stage/render/render 11.32, pipelines/compile_ms 10.20, render/graph/ms 9.70, pipelines/front_ms 9.50, encode//sum 7.56, stage/render/prepare_meshes 6.85 |
| warm-up | 6.29 | 31.4 | main/busy/ms 52.09, render/thread/ms 48.39, render/busy/ms 46.74, stage/main/update 31.24, stage/main/post_update 14.80, stage/render/render 14.50, stage/render/prepare_assets 13.54, render/graph/ms 12.28, encode//sum 9.22, stage/render/prepare_meshes 7.83, stage/render/graph_submit 7.77, stream/drive/ms 6.06 |
| warm-up | 6.32 | 32.8 | render/thread/ms 45.02, render/busy/ms 43.45, main/busy/ms 42.55, stage/main/update 25.89, stage/render/prepare_assets 18.04, stage/render/render 11.66, stage/main/post_update 10.22, render/graph/ms 9.73, encode//sum 7.85, stage/render/graph_submit 6.41, stream/drive/ms 6.31, stage/render/prepare_meshes 6.07 |
| measure | 18.36 | 32.6 | main/busy/ms 39.18, stage/main/update 25.66, render/thread/ms 21.79, render/busy/ms 20.84, stage/render/render 10.69, stage/main/post_update 9.41, render/graph/ms 9.41, stage/render/graph_submit 6.66, encode//sum 6.14, encode/submit 4.31, stage/render/prepare_assets 3.92, stage/render/camera_driver 2.60 |
| measure | 23.03 | 26.6 | render/thread/ms 31.05, render/busy/ms 29.84, main/busy/ms 28.59, stage/render/render 14.90, stage/main/update 13.64, render/graph/ms 13.00, stage/main/post_update 10.23, encode//sum 9.88, stage/render/graph_submit 9.46, stage/render/prepare_assets 8.33, encode/submit 5.84, encode/finish_wall 3.70 |
| measure | 50.96 | 25.8 | render/thread/ms 34.53, stage/render/render 27.02, render/busy/ms 19.10, main/busy/ms 17.24, render/present/ms 15.23, render/graph/ms 11.29, encode//sum 10.52, stage/main/post_update 9.54, stage/render/graph_submit 8.22, encode/submit 5.17, stage/main/update 4.16, encode/finish_wall 3.12 |
| measure | 59.12 | 30.2 | render/thread/ms 36.99, render/busy/ms 36.01, main/busy/ms 24.28, stage/render/prepare_assets 17.88, stage/main/post_update 11.51, stage/render/render 8.51, render/graph/ms 7.27, encode//sum 6.90, stage/main/update 5.35, stage/render/graph_submit 4.76, stage/main/fixed_main_loop 3.11, encode/submit 2.64 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=atlas48-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1512 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **87%** (min 62%) · 2625 MHz (min 2625) · memory 8101 MHz · 83.2 W · 74 °C (max 79) · VRAM 6469 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.enabled=on (command line: --set)`
- `shadow.lamp.atlas.max_lamps=48 (command line: --set)`
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
