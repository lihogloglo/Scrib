# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=48 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6476 frames over 65.6 s · **median 10.01 ms (100 fps)** · mean 10.14 · p95 11.84 · p99 14.01 (1% low 71 fps) · worst 34.04 ms · 37 spikes over 1.5× median · 34.7 stutters/min (work 27.4/min)
- warm-up: 353 frames over 9.2 s, worst 2541.3 ms
- hitches (> 25.0 ms): **3** measured, 51 in warm-up
- timed GPU passes: 6.48 ms of a 10.14 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.01 ms, limit 11 ms
- **FAIL** frame_ms: 38 stutters (> 1.5x moving median of 60) in 1.09 min = 34.73/min, limit 1/min
- **FAIL** worst frame 34.04 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.977 | 2.175 | 2.562 | 0.91 | 0.176 |
| main_opaque_pass_3d | 1.223 | 1.345 | 1.859 | 0.91 | 0.074 |
| early prepass | 0.485 | 0.533 | 0.793 | 0.91 | 0.091 |
| bloom | 0.314 | 0.345 | 0.670 | 0.91 | 0.065 |
| atmosphere_luts | 0.306 | 0.336 | 0.630 | 0.91 | 0.003 |
| water_velocity | 0.219 | 0.240 | 0.249 | 0.91 | 0.002 |
| water_sspr | 0.198 | 0.218 | 0.219 | 0.91 | 0.002 |
| ssao | 0.178 | 0.196 | 0.204 | 0.91 | 0.008 |
| point_shadow_atlas | 0.151 | 0.279 | 0.508 | 0.54 | 0.126 |
| atmos_froxel_inject | 0.126 | 0.138 | 0.131 | 0.91 | 0.002 |
| upscaling | 0.125 | 0.137 | 0.158 | 0.91 | 0.004 |
| water_fft | 0.116 | 0.127 | 0.127 | 0.91 | 0.003 |
| uniform_allocation | 0.111 | 0.122 | 0.144 | 0.91 | 0.024 |
| clouds_composite | 0.098 | 0.108 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.092 | 0.101 | 0.116 | 0.91 | 0.018 |
| gpu_instance_materialization | 0.089 | 0.098 | 0.101 | 0.91 | 0.004 |
| tonemapping | 0.085 | 0.093 | 0.098 | 0.91 | 0.004 |
| main_transmissive_pass_3d | 0.068 | 0.074 | 0.144 | 0.91 | 0.009 |
| clustering | 0.061 | 0.067 | 0.068 | 0.91 | 0.020 |
| early_mesh_preprocessing | 0.052 | 0.057 | 0.072 | 0.91 | 0.004 |
| render_sky | 0.051 | 0.056 | 0.055 | 0.91 | 0.002 |
| clouds_trace | 0.051 | 0.056 | 0.078 | 0.91 | 0.001 |
| clouds_resolve | 0.045 | 0.050 | 0.051 | 0.91 | 0.001 |
| surface_trail | 0.037 | 0.060 | 0.057 | 0.61 | 0.002 |
| lightprobe_irradiance_map | 0.033 | 0.474 | 0.767 | 0.07 | 0.001 |
| atmos_froxel_integrate | 0.029 | 0.032 | 0.033 | 0.91 | 0.001 |
| water_snapshot | 0.024 | 0.026 | 0.034 | 0.91 | 0.002 |
| range_unpacking | 0.022 | 0.024 | 0.029 | 0.91 | 0.003 |
| water_mips | 0.021 | 0.023 | 0.023 | 0.91 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.020 | 0.037 | 0.074 | 0.54 | 0.010 |
| point_shadow_atlas/early_mesh_preprocessing | 0.017 | 0.031 | 0.055 | 0.54 | 0.006 |
| per_view_shadow_pass | 0.017 | 0.019 | 0.026 | 0.91 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.018 | 0.030 | 0.91 | 0.188 |
| night_sky | 0.015 | 0.017 | 0.043 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.015 | 0.219 | 0.231 | 0.07 | 0.004 |
| point_shadow_atlas/bin_unpacking | 0.014 | 0.025 | 0.047 | 0.54 | 0.007 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.91 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.91 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.54 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.91 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.934 | 10.373 | 26.59 | 57853.8 |
| render/busy/ms | 8.393 | 9.834 | 20.01 | 54353.8 |
| main/busy/ms | 8.229 | 9.514 | 31.07 | 53289.6 |
| stage/render/render | 5.198 | 6.218 | 19.21 | 33659.8 |
| stage/main/post_update | 4.654 | 5.477 | 9.71 | 30141.3 |
| render/graph/ms | 4.501 | 5.386 | 10.02 | 29146.0 |
| encode//sum | 3.389 | 4.167 | 9.12 | 21944.2 |
| stage/render/graph_submit | 3.222 | 4.036 | 7.84 | 20865.9 |
| encode/submit | 2.040 | 2.926 | 4.79 | 13212.4 |
| stage/main/update | 1.880 | 2.371 | 25.64 | 12175.8 |
| encode/finish_wall | 1.267 | 1.635 | 3.47 | 8203.8 |
| stage/render/camera_driver | 1.215 | 1.427 | 3.08 | 7867.1 |
| encode/pass:early prepass/sum | 0.851 | 1.110 | 1.65 | 5510.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.848 | 1.119 | 2.28 | 5493.9 |
| encode/pass:shadow_directional_light/sum | 0.847 | 1.258 | 2.37 | 5485.7 |
| stage/render/prepare_write_phase_buffers | 0.638 | 1.008 | 1.93 | 4129.8 |
| bigspace/total/ms | 0.550 | 1.328 | 2.39 | 3558.7 |
| stage/render/prepare_assets | 0.530 | 0.664 | 11.76 | 3431.0 |
| stage/main/fixed_main_loop | 0.501 | 0.909 | 2.73 | 3247.5 |
| render/present/ms | 0.450 | 1.259 | 13.62 | 2911.8 |
| stage/render/prepare_resources | 0.434 | 0.538 | 4.21 | 2810.7 |
| stage/render/prepare_meshes | 0.432 | 0.549 | 9.00 | 2794.8 |
| stage/main/pre_update | 0.381 | 0.517 | 3.47 | 2467.5 |
| stage/main/first | 0.368 | 0.487 | 5.36 | 2383.0 |
| controller/feed/ms | 0.334 | 0.435 | 0.83 | 2161.6 |
| stage/render/prepare_bind_groups | 0.311 | 0.507 | 2.24 | 2011.2 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.304 | 1.131 | 3.21 | 1969.7 |
| stage/render/prepare_flush | 0.297 | 0.524 | 4.37 | 1924.1 |
| stage/render/prepare_batch_phases | 0.279 | 0.427 | 0.75 | 1804.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.276 | 0.444 | 0.94 | 1784.9 |
| stage/main/last | 0.262 | 0.316 | 1.03 | 1697.4 |
| bigspace/low_precision/ms | 0.236 | 0.713 | 1.73 | 1527.1 |
| stage/render/create_views | 0.218 | 0.311 | 0.85 | 1413.3 |
| stream/drive/ms | 0.212 | 0.301 | 10.70 | 1369.9 |
| stage/render/extract_commands | 0.191 | 0.226 | 0.56 | 1235.2 |
| anim/evaluate/ms | 0.166 | 0.301 | 1.52 | 1075.0 |
| bigspace/high_precision/ms | 0.162 | 0.688 | 1.74 | 1051.3 |
| stage/render/specialize | 0.162 | 0.257 | 1.88 | 1048.2 |
| anim/evaluate_cpu/ms | 0.157 | 0.204 | 0.53 | 1014.5 |
| stream/tick/ms | 0.157 | 0.232 | 0.65 | 1014.3 |
| stage/render/prepare_views | 0.134 | 0.178 | 0.48 | 869.8 |
| stage/render/queue | 0.101 | 0.169 | 1.64 | 655.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.096 | 0.153 | 0.56 | 619.9 |
| render/acquire/ms | 0.091 | 0.124 | 0.41 | 588.3 |
| stage/render/phase_sort | 0.082 | 0.114 | 0.41 | 528.9 |
| controller/write/ms | 0.073 | 0.099 | 0.40 | 471.6 |
| controller/move/ms | 0.067 | 0.208 | 0.41 | 437.1 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.060 | 0.277 | 1.24 | 391.2 |
| terrain/update/ms | 0.058 | 0.284 | 1.84 | 377.5 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.057 | 0.310 | 3.33 | 371.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.71 | 297.0 |
| physics/step/ms | 0.039 | 0.081 | 0.52 | 251.4 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.26 | 205.8 |
| pipelines/compile_ms | 0.027 | 0.000 | 82.26 | 175.3 |
| physics/insert/ms | 0.027 | 0.308 | 0.95 | 172.8 |
| pipelines/front_ms | 0.026 | 0.000 | 79.56 | 169.3 |
| stage/render/cleanup | 0.024 | 0.034 | 1.05 | 153.9 |
| terrain/select/ms | 0.022 | 0.251 | 0.64 | 139.5 |
| weather/map/ms | 0.018 | 0.024 | 0.36 | 116.0 |
| far/select/ms | 0.015 | 0.170 | 0.54 | 98.6 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.23 | 84.9 |
| anim/apply/ms | 0.012 | 0.023 | 0.16 | 78.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.16 | 70.5 |
| weather/eval/ms | 0.011 | 0.014 | 0.17 | 68.6 |
| script/update/ms | 0.008 | 0.012 | 0.15 | 52.6 |
| controller/avoid/ms | 0.006 | 0.012 | 0.14 | 38.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.07 | 38.1 |
| script/input/ms | 0.005 | 0.006 | 0.27 | 33.9 |
| stream/spawn/ms | 0.004 | 0.000 | 1.15 | 23.4 |
| script/ui/ms | 0.003 | 0.004 | 0.27 | 22.0 |
| stream/upload/ms | 0.003 | 0.000 | 1.36 | 20.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 13.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.33 | 7.6 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.95 | 6.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.5 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 18 | 33 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 583 |
| crowd/c0/meshes/count | count | 357 | 646 | 162 |
| crowd/c1/count | count | 145 | 156 | 148 |
| crowd/c1/joints/count | count | 5512 | 5991 | 5613 |
| crowd/c1/meshes/count | count | 1166 | 1295 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 205 |
| crowd/joints_extracted/count | count | 668 | 5573 | 610 |
| crowd/joints_uploaded/count | count | 3585 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.42 | 1.39 | 0.45 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.23 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.16 | 0.01 |
| encode/finish_critical | ms | 1.01 | 3.33 | 0.90 |
| encode/pass:early prepass/max | ms | 0.85 | 1.65 | 0.90 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.86 | 2.28 | 0.68 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.56 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.94 | 0.27 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.78 | 3.21 | 0.59 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.21 | 1.24 | 0.11 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.00 | 3.33 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.83 | 2.37 | 0.62 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.26 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.58 | 31.07 | 8.57 |
| gpu/timed/ms | ms | 7.98 | 9.36 | 8.57 |
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
| phases/alpha_mask3d/items | count | 323 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 323 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2494 | 3524 | 2457 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2494 | 3524 | 2457 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 206 | 8 |
| phases/shadow/bins | count | 751 | 1167 | 671 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2196 | 2860 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 35 | 100 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2679 | 3182 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 15 | 271 | 11 |
| preprocess/bind_groups_created | count | 10 | 265 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 132 | 228 | 123 |
| process/cpu_usage | % | 25.29 | 41.24 | 29.81 |
| process/mem_usage | GiB | 5.16 | 5.44 | 5.44 |
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
| shadow_atlas/alloc_failures | count | 590 | 590 | 590 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 1 | 0 |
| shadow_atlas/lamps | count | 8 | 21 | 0 |
| shadow_atlas/live_occ_pct | % | 41.60 | 100 | 0 |
| shadow_atlas/live_tiles | count | 16 | 58 | 0 |
| shadow_atlas/mover_draws | count | 8 | 12 | 0 |
| shadow_atlas/movers | count | 148 | 471 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 4 | 0 |
| shadow_atlas/render/blits | count | 8 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 21 | 0 |
| shadow_atlas/render/lamps | count | 8 | 21 | 0 |
| shadow_atlas/render/live_direct | count | 8 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 15 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 56 | 0 |
| shadow_atlas/resizes | count | 30 | 39 | 39 |
| shadow_atlas/static_occ_pct | % | 16.41 | 33.98 | 0 |
| shadow_atlas/static_tiles | count | 42 | 108 | 0 |
| shadow_atlas/views | count | 17 | 56 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 95 | 13 |
| skin/blocks_skipped/count | count | 145 | 211 | 192 |
| skin/blocks_walked/count | count | 14 | 27 | 13 |
| skin/joint_bytes_written/bytes | bytes | 47.4 k | 810.6 k | 40.4 k |
| skin/joints_written/count | count | 740 | 12.7 k | 632 |
| skin/upload_writes/count | count | 7 | 27 | 8 |
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
| system/cpu_usage | % | 31.70 | 44.58 | 35.88 |
| system/mem_usage | % | 55.53 | 56.55 | 56.54 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 2999 | 2999 |
| textures/images | count | 2666 | 2721 | 2721 |
| views/all/batch_sets | count | 85 | 267 | 66 |
| views/all/bins | count | 2904 | 3177 | 2961 |
| views/all/count | count | 20 | 59 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8006 | 10.1 k | 7971 |
| views/all/pipelines | count | 81 | 147 | 66 |
| views/all/pipelines_distinct | count | 65 | 76 | 61 |
| views/all/work_items | count | 154.1 k | 230.6 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 724 | 858 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2123 | 2414 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 9 | 191 | 0 |
| views/lamp_face/bins | count | 24 | 454 | 0 |
| views/lamp_face/count | count | 17 | 56 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 66 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 36 | 571 | 0 |
| views/lamp_face/pipelines | count | 8 | 74 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 17 | 25.0 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2004 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7486 | 5944 |
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
| anim/bones_written/count | count | 53.77 | 228 | 348.2 k |
| anim/events/count | count | 0.08 | 3 | 547 |
| assets/Image/added | count | 0.09 | 41 | 586 |
| assets/Image/modified | count | 0.00 | 9 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.44 | 260 | 2874 |
| assets/Mesh/removed | count | 0.75 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.15 | 10 | 7475 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1063 | 1489 |
| encode//count | count | 27.92 | 30 | 180.8 k |
| encode/bind_walk/init_textures | count | 8791 | 22.5 k | 56.93 M |
| encode/bind_walk/init_textures_skipped | count | 52.7 k | 73.1 k | 341.05 M |
| encode/bind_walk/inits | count | 570 | 775 | 3.69 M |
| encode/bind_walk/inits_skipped | count | 144 | 232 | 935.3 k |
| encode/bind_walk/merge_textures | count | 8728 | 22.4 k | 56.52 M |
| encode/bind_walk/merge_textures_skipped | count | 8264 | 16.1 k | 53.52 M |
| encode/bind_walk/merges | count | 108 | 190 | 698.8 k |
| encode/bind_walk/merges_skipped | count | 131 | 223 | 850.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6476 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6476 |
| encode/pass:early prepass/count | count | 1 | 1 | 6476 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6476 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6476 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6476 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.37 | 1 | 2423 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.14 | 1 | 931 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.06 | 1 | 367 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6476 |
| encode/queue/command_buffers | count | 38.51 | 42 | 249.4 k |
| encode/queue/submits | count | 2.01 | 4 | 13.0 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.45 M | 15.06 G |
| encode/queue/write_buffer/calls | count | 276 | 456 | 1.79 M |
| encode/queue/write_buffer_with/bytes | bytes | 157.9 k | 1.29 M | 1.02 G |
| encode/queue/write_buffer_with/calls | count | 10.32 | 789 | 66.8 k |
| encode/queue/write_texture/bytes | bytes | 2037 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6476 |
| physics/inserts | count | 0.35 | 87 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 177 | 2335 |
| physics/steps | count | 0.61 | 2 | 3938 |
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
| stream/deactivate/items | count | 0.03 | 37 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5227 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1146 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 54, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2541.3 | main/busy/ms 2511, stage/main/post_update 2467, stage/main/update 31.84, stage/main/pre_update 7.92, stage/main/last 1.03, surface/advance_cpu_ms 0.76, stage/main/first 0.57, stream/drive/ms 0.51, stage/main/fixed_main_loop 0.38, surface/cell_upload_ms 0.29, stream/tick/ms 0.16, bigspace/total/ms 0.08 |
| warm-up | 3.06 | 519.0 | main/busy/ms 2638, stage/main/post_update 2493, render/thread/ms 481, render/busy/ms 391, stage/render/prepare_assets 348, stage/main/update 126, stage/render/prepare_views 87.90, render/acquire/ms 87.47, stage/render/render 16.43, stage/render/create_views 15.75, render/graph/ms 13.18, stage/render/camera_driver 11.40 |
| warm-up | 3.12 | 58.6 | render/thread/ms 522, render/busy/ms 430, stage/render/prepare_assets 374, main/busy/ms 176, stage/main/update 99.09, stage/render/prepare_views 88.13, render/acquire/ms 87.65, stage/main/post_update 37.34, stage/main/pre_update 28.49, stage/render/render 24.03, render/graph/ms 18.12, terrain/update/ms 17.58 |
| warm-up | 3.17 | 47.6 | main/busy/ms 109, render/thread/ms 60.70, render/busy/ms 55.72, stage/main/update 44.17, pipelines/compile_ms 40.01, pipelines/front_ms 36.47, stage/main/pre_update 29.79, stage/render/prepare_assets 27.61, terrain/update/ms 23.48, stage/main/post_update 22.76, stage/render/render 15.40, stage/render/cleanup 13.12 |
| warm-up | 3.22 | 51.7 | pipelines/wait_ms 490, pipelines/compile_ms 450, pipelines/front_ms 421, main/busy/ms 105, stage/main/update 58.03, render/thread/ms 43.04, render/busy/ms 38.77, stage/main/post_update 31.31, pipelines/driver_ms 28.03, terrain/update/ms 13.17, stage/render/cleanup 13.03, stage/render/render 12.14 |
| warm-up | 3.36 | 144.4 | pipelines/wait_ms 8035, pipelines/compile_ms 1879, pipelines/front_ms 1769, main/busy/ms 157, render/thread/ms 148, render/busy/ms 145, pipelines/driver_ms 110, stage/main/post_update 90.10, stage/render/prepare_resources 38.31, stage/main/update 37.76, stage/render/prepare_batch_phases 36.78, stage/main/pre_update 19.00 |
| warm-up | 3.39 | 29.5 | pipelines/wait_ms 7718, pipelines/compile_ms 1843, pipelines/front_ms 1735, main/busy/ms 187, render/thread/ms 148, render/busy/ms 146, pipelines/driver_ms 108, stage/main/post_update 76.51, stage/main/update 73.00, stage/render/prepare_resources 37.16, stage/render/prepare_batch_phases 35.25, stage/main/pre_update 24.98 |
| warm-up | 3.52 | 124.6 | pipelines/compile_ms 1664, pipelines/front_ms 1586, pipelines/wait_ms 403, main/busy/ms 97.51, render/thread/ms 94.03, render/busy/ms 92.42, pipelines/driver_ms 78.59, stage/render/prepare_assets 75.64, stage/main/update 64.28, stage/main/post_update 17.25, terrain/update/ms 10.09, stage/render/render 9.64 |
| warm-up | 3.58 | 61.7 | pipelines/compile_ms 2220, pipelines/front_ms 2097, pipelines/wait_ms 515, render/thread/ms 128, render/busy/ms 126, pipelines/driver_ms 123, stage/render/prepare_assets 103, main/busy/ms 38.27, stage/render/render 17.09, stage/main/update 16.66, stage/main/post_update 15.14, encode//sum 14.75 |
| warm-up | 3.60 | 25.4 | pipelines/compile_ms 1476, pipelines/front_ms 1384, pipelines/wait_ms 304, pipelines/driver_ms 92.35, render/thread/ms 81.72, render/busy/ms 79.07, stage/render/prepare_assets 48.76, main/busy/ms 38.59, encode//sum 27.40, stage/render/render 22.63, render/graph/ms 19.56, stage/render/graph_submit 15.64 |
| warm-up | 3.66 | 35.8 | main/busy/ms 72.22, pipelines/compile_ms 66.81, pipelines/front_ms 60.87, render/thread/ms 56.76, render/busy/ms 53.04, stage/main/update 49.43, stage/render/render 20.31, stage/main/post_update 18.24, render/graph/ms 17.84, stage/render/graph_submit 14.45, stage/render/prepare_resources 12.26, encode/submit 10.97 |
| warm-up | 3.76 | 94.8 | pipelines/compile_ms 778, pipelines/front_ms 738, render/thread/ms 88.99, render/busy/ms 86.82, main/busy/ms 75.74, stage/main/update 52.86, stage/render/prepare_assets 41.88, pipelines/driver_ms 39.66, stage/render/render 18.76, render/graph/ms 15.94, stage/main/post_update 14.94, stage/render/prepare_resources 10.47 |
| warm-up | 3.79 | 29.6 | pipelines/compile_ms 1411, pipelines/front_ms 1351, render/thread/ms 76.47, render/busy/ms 73.00, pipelines/driver_ms 60.19, stage/render/prepare_assets 50.96, main/busy/ms 29.18, stage/render/render 17.49, render/graph/ms 13.51, stage/main/update 11.28, stage/main/post_update 8.86, stage/render/graph_submit 7.53 |
| warm-up | 3.83 | 43.6 | pipelines/compile_ms 910, pipelines/front_ms 873, render/thread/ms 48.11, render/busy/ms 45.07, pipelines/driver_ms 37.52, stage/render/prepare_assets 32.12, main/busy/ms 17.78, stage/render/render 10.60, stage/main/update 7.45, render/graph/ms 7.30, encode//sum 5.34, stage/render/graph_submit 5.09 |
| warm-up | 3.89 | 38.6 | pipelines/compile_ms 278, pipelines/front_ms 267, render/thread/ms 41.20, render/busy/ms 39.48, main/busy/ms 35.52, stage/main/update 25.40, stage/render/render 18.39, render/graph/ms 15.67, stage/render/prepare_assets 11.39, stage/render/graph_submit 11.29, pipelines/driver_ms 10.28, encode//sum 8.83 |
| warm-up | 3.93 | 36.5 | pipelines/compile_ms 253, pipelines/front_ms 243, render/thread/ms 57.47, render/busy/ms 55.11, main/busy/ms 38.12, stage/main/update 23.85, stage/render/prepare_assets 21.97, stage/render/render 19.94, render/graph/ms 16.46, stage/render/graph_submit 11.28, encode//sum 10.27, pipelines/driver_ms 9.40 |
| warm-up | 3.97 | 46.4 | pipelines/compile_ms 222, pipelines/front_ms 214, render/thread/ms 49.22, render/busy/ms 47.38, main/busy/ms 29.55, stage/render/prepare_assets 28.11, stage/main/update 14.49, stage/render/render 11.77, render/graph/ms 9.52, stage/main/post_update 8.52, pipelines/driver_ms 7.55, encode//sum 7.37 |
| warm-up | 4.05 | 73.4 | pipelines/compile_ms 1261, pipelines/front_ms 1179, main/busy/ms 111, stage/main/update 96.48, render/thread/ms 86.77, render/busy/ms 85.39, pipelines/driver_ms 82.06, stage/render/prepare_assets 71.61, stage/render/render 10.38, render/graph/ms 8.76, stage/main/post_update 7.56, stage/render/graph_submit 6.58 |
| warm-up | 4.15 | 102.3 | pipelines/compile_ms 1275, pipelines/front_ms 1195, main/busy/ms 143, stage/main/update 127, render/thread/ms 113, render/busy/ms 111, stage/render/prepare_assets 91.33, pipelines/driver_ms 79.92, stage/render/render 12.02, render/graph/ms 10.18, stage/main/post_update 8.43, stage/render/graph_submit 7.57 |
| warm-up | 4.22 | 68.0 | pipelines/compile_ms 408, pipelines/front_ms 395, main/busy/ms 117, render/thread/ms 101, render/busy/ms 99.82, pipelines/wait_ms 79.53, stage/main/update 73.85, stage/render/prepare_assets 52.95, stage/render/render 19.26, stage/main/pre_update 18.55, pipelines/driver_ms 12.62, stage/main/post_update 11.57 |
| warm-up | 4.28 | 64.3 | pipelines/compile_ms 1274, pipelines/front_ms 1227, pipelines/wait_ms 111, main/busy/ms 96.84, render/thread/ms 84.32, render/busy/ms 82.85, stage/main/update 51.65, pipelines/driver_ms 46.93, stage/render/prepare_assets 31.68, stage/render/render 20.87, stage/main/pre_update 18.97, stage/main/post_update 13.87 |
| warm-up | 4.37 | 86.6 | pipelines/compile_ms 2247, pipelines/front_ms 2174, pipelines/wait_ms 245, render/thread/ms 85.29, render/busy/ms 83.70, pipelines/driver_ms 72.79, main/busy/ms 71.71, stage/main/post_update 44.25, stage/render/prepare_assets 23.34, stage/main/update 20.13, stage/render/prepare_write_phase_buffers 19.23, stage/render/render 12.86 |
| warm-up | 4.45 | 79.6 | pipelines/compile_ms 1626, pipelines/front_ms 1579, pipelines/wait_ms 217, render/thread/ms 125, render/busy/ms 123, main/busy/ms 91.51, stage/main/post_update 49.28, pipelines/driver_ms 47.80, stage/render/prepare_assets 34.14, stage/main/update 30.37, stage/render/prepare_write_phase_buffers 19.51, stage/render/prepare_resources 19.50 |
| warm-up | 4.54 | 90.1 | pipelines/compile_ms 1659, pipelines/front_ms 1612, render/thread/ms 157, render/busy/ms 154, pipelines/wait_ms 132, main/busy/ms 88.62, pipelines/driver_ms 46.38, stage/render/prepare_assets 46.31, stage/main/update 45.06, stage/render/prepare_resources 35.95, stage/main/post_update 24.76, stream/drive/ms 18.48 |
| warm-up | 4.61 | 72.3 | pipelines/compile_ms 2813, pipelines/front_ms 2735, pipelines/wait_ms 346, render/thread/ms 125, render/busy/ms 123, main/busy/ms 98.75, pipelines/driver_ms 77.82, stage/main/update 65.43, stage/render/prepare_assets 45.43, stage/render/prepare_resources 24.79, stage/main/post_update 17.86, stage/render/render 17.50 |
| warm-up | 4.73 | 116.9 | pipelines/compile_ms 2092, pipelines/front_ms 2037, pipelines/wait_ms 258, render/thread/ms 119, render/busy/ms 117, stage/render/prepare_assets 91.90, main/busy/ms 85.24, stage/main/update 58.93, pipelines/driver_ms 54.90, stage/render/render 17.94, render/graph/ms 15.66, stage/main/post_update 11.90 |
| warm-up | 4.77 | 46.2 | pipelines/compile_ms 877, pipelines/front_ms 854, render/thread/ms 115, render/busy/ms 114, stage/render/prepare_assets 80.35, main/busy/ms 51.70, pipelines/wait_ms 41.67, stage/main/update 24.85, pipelines/driver_ms 23.87, stage/render/render 15.52, render/graph/ms 13.71, stage/main/post_update 12.44 |
| warm-up | 4.81 | 35.0 | pipelines/compile_ms 527, pipelines/front_ms 507, main/busy/ms 92.55, stage/main/update 69.70, render/thread/ms 68.84, render/busy/ms 67.54, stream/drive/ms 36.21, stage/render/prepare_assets 21.73, pipelines/driver_ms 20.26, stage/render/render 16.90, render/graph/ms 14.93, stage/main/post_update 13.56 |
| warm-up | 4.89 | 83.1 | main/busy/ms 117, pipelines/compile_ms 106, pipelines/front_ms 101, stage/main/update 96.78, stream/drive/ms 64.95, render/thread/ms 61.64, render/busy/ms 60.24, stage/render/prepare_assets 21.74, stage/render/render 16.29, render/graph/ms 14.31, stage/main/post_update 12.64, pipelines/wait_ms 10.25 |
| warm-up | 4.94 | 47.6 | pipelines/compile_ms 1330, pipelines/front_ms 1300, main/busy/ms 79.03, stage/main/update 62.64, render/thread/ms 58.69, render/busy/ms 57.20, stream/drive/ms 34.87, pipelines/driver_ms 29.93, stage/render/render 21.27, render/graph/ms 19.12, stage/render/prepare_assets 17.97, stage/render/graph_submit 14.81 |
| warm-up | 4.98 | 39.7 | pipelines/compile_ms 1776, pipelines/front_ms 1732, render/thread/ms 53.54, render/busy/ms 52.13, main/busy/ms 48.02, pipelines/driver_ms 43.89, stage/main/update 32.88, stage/render/render 20.08, render/graph/ms 17.94, stage/render/prepare_assets 15.10, stage/render/graph_submit 14.18, encode//sum 8.77 |
| warm-up | 5.18 | 28.2 | pipelines/compile_ms 663, pipelines/front_ms 644, render/thread/ms 45.14, render/busy/ms 43.71, main/busy/ms 34.68, pipelines/driver_ms 19.42, stage/main/post_update 15.09, stage/render/render 13.75, stage/main/update 12.83, render/graph/ms 11.93, stage/render/prepare_assets 11.28, stage/render/graph_submit 8.86 |
| warm-up | 5.21 | 25.6 | pipelines/compile_ms 663, pipelines/front_ms 644, render/thread/ms 47.85, render/busy/ms 46.48, main/busy/ms 35.93, pipelines/driver_ms 19.42, stage/main/post_update 17.72, stage/render/render 12.55, stage/main/update 11.80, stage/render/prepare_flush 11.22, render/graph/ms 10.76, stage/render/prepare_assets 9.13 |
| warm-up | 5.24 | 34.7 | pipelines/compile_ms 488, pipelines/front_ms 473, render/thread/ms 55.67, render/busy/ms 54.00, main/busy/ms 44.91, stage/main/post_update 21.57, stage/render/render 20.69, render/graph/ms 18.48, stage/main/update 16.73, pipelines/driver_ms 14.81, stage/render/graph_submit 13.84, stage/render/prepare_flush 10.99 |
| warm-up | 5.28 | 33.7 | pipelines/compile_ms 509, pipelines/front_ms 492, render/thread/ms 55.76, render/busy/ms 54.17, main/busy/ms 43.01, stage/render/render 19.28, stage/main/post_update 17.50, stage/render/prepare_assets 17.27, render/graph/ms 17.15, pipelines/driver_ms 16.82, stage/main/update 16.33, stage/render/graph_submit 12.71 |
| warm-up | 5.32 | 40.8 | render/thread/ms 48.50, render/busy/ms 47.39, main/busy/ms 30.66, stage/render/prepare_assets 28.61, pipelines/compile_ms 20.56, pipelines/front_ms 18.55, stage/main/update 10.70, stage/render/render 10.46, stage/main/post_update 10.38, render/graph/ms 8.94, encode//sum 7.97, stage/render/graph_submit 6.09 |
| warm-up | 5.35 | 27.1 | pipelines/compile_ms 213, pipelines/front_ms 207, render/thread/ms 48.58, render/busy/ms 47.30, main/busy/ms 36.51, stage/render/prepare_assets 19.57, stage/main/update 17.25, stage/render/render 17.17, render/graph/ms 15.10, stage/main/post_update 12.10, stage/render/graph_submit 10.89, encode//sum 9.03 |
| warm-up | 5.46 | 29.5 | main/busy/ms 32.23, render/thread/ms 22.24, render/busy/ms 21.28, stage/main/update 20.17, stream/drive/ms 13.82, stage/render/render 8.94, stage/main/post_update 7.93, render/graph/ms 7.73, encode//sum 6.25, stage/render/graph_submit 5.40, stage/render/prepare_assets 4.67, encode/submit 3.49 |
| warm-up | 5.55 | 31.2 | pipelines/compile_ms 353, pipelines/front_ms 335, render/thread/ms 43.93, render/busy/ms 41.82, stage/render/render 26.98, main/busy/ms 26.96, render/graph/ms 24.45, pipelines/driver_ms 17.31, stage/render/graph_submit 12.30, stage/main/post_update 11.23, stage/render/camera_driver 11.18, stage/main/update 11.04 |
| warm-up | 5.60 | 28.4 | pipelines/compile_ms 270, pipelines/front_ms 252, render/thread/ms 39.36, render/busy/ms 37.77, main/busy/ms 27.41, stage/render/prepare_assets 18.18, pipelines/driver_ms 18.16, stage/render/render 12.22, stage/main/update 10.62, render/graph/ms 10.06, stage/main/post_update 9.78, stage/render/graph_submit 7.22 |
| warm-up | 5.74 | 31.6 | pipelines/compile_ms 412, pipelines/front_ms 402, main/busy/ms 42.11, render/thread/ms 26.10, stage/main/update 23.37, render/busy/ms 22.67, stage/render/render 15.00, stream/drive/ms 13.92, stage/main/post_update 11.27, render/graph/ms 11.12, pipelines/driver_ms 9.94, stage/render/graph_submit 7.78 |
| warm-up | 5.82 | 27.2 | pipelines/compile_ms 216, pipelines/front_ms 212, render/thread/ms 35.81, render/busy/ms 34.40, main/busy/ms 30.37, stage/main/update 13.87, stage/render/render 11.96, stage/main/post_update 10.42, render/graph/ms 9.85, encode//sum 8.92, stage/render/prepare_assets 7.98, stage/render/graph_submit 6.64 |
| warm-up | 5.91 | 26.5 | pipelines/compile_ms 1253, pipelines/front_ms 1216, render/thread/ms 39.89, render/busy/ms 38.83, pipelines/driver_ms 37.16, main/busy/ms 32.63, stage/main/update 16.23, stage/render/prepare_assets 16.02, stage/main/post_update 11.56, stage/render/render 10.47, render/graph/ms 8.93, encode//sum 8.01 |
| warm-up | 6.05 | 25.7 | pipelines/compile_ms 143, pipelines/front_ms 139, render/thread/ms 36.84, render/busy/ms 35.78, main/busy/ms 35.72, stage/main/update 20.52, stage/render/render 12.76, render/graph/ms 11.22, stage/main/post_update 10.78, encode//sum 8.55, stage/render/prepare_assets 8.55, stage/render/graph_submit 7.42 |
| warm-up | 6.13 | 29.8 | main/busy/ms 44.04, render/thread/ms 43.74, render/busy/ms 42.32, stage/main/update 24.48, stage/render/render 14.91, render/graph/ms 12.99, stage/main/post_update 12.36, stage/render/prepare_assets 10.41, encode//sum 8.65, stage/render/graph_submit 8.43, encode/submit 5.46, stage/render/prepare_meshes 4.71 |
| warm-up | 6.17 | 36.1 | render/thread/ms 47.24, render/busy/ms 46.16, main/busy/ms 41.26, stage/render/prepare_assets 21.18, stage/main/update 21.10, stage/main/post_update 11.44, stage/render/render 8.93, render/graph/ms 7.40, encode//sum 6.06, stage/render/prepare_meshes 4.91, stage/render/graph_submit 4.65, stream/drive/ms 3.90 |
| warm-up | 6.22 | 45.5 | pipelines/compile_ms 80.15, pipelines/front_ms 76.85, render/thread/ms 68.73, render/busy/ms 66.90, main/busy/ms 61.13, stage/main/post_update 35.27, stage/render/render 32.24, render/graph/ms 29.76, stage/render/graph_submit 23.61, stage/main/update 19.57, stage/render/prepare_assets 16.62, encode/finish_wall 15.76 |
| warm-up | 6.24 | 25.1 | pipelines/compile_ms 138, pipelines/front_ms 132, render/thread/ms 61.80, render/busy/ms 59.91, main/busy/ms 59.65, stage/main/post_update 33.66, stage/render/render 33.02, render/graph/ms 30.49, stage/render/graph_submit 23.98, stage/main/update 18.99, encode/finish_wall 15.97, pipelines/wait_ms 14.40 |
| warm-up | 6.28 | 36.3 | pipelines/compile_ms 58.17, pipelines/front_ms 55.07, render/thread/ms 44.85, render/busy/ms 43.76, main/busy/ms 36.85, stage/render/prepare_assets 20.51, stage/main/update 19.33, stage/render/render 10.68, stage/main/post_update 10.68, render/graph/ms 9.00, encode//sum 7.29, stage/render/graph_submit 5.84 |
| warm-up | 6.43 | 35.0 | main/busy/ms 58.60, stage/main/update 41.45, render/thread/ms 34.83, render/busy/ms 33.46, stream/drive/ms 15.78, stage/render/render 14.84, render/graph/ms 12.82, stage/main/post_update 12.61, encode//sum 9.29, stage/render/graph_submit 8.93, stage/render/prepare_assets 6.02, encode/submit 5.63 |
| warm-up | 6.48 | 49.9 | main/busy/ms 50.24, render/thread/ms 47.60, render/busy/ms 46.23, stage/main/update 32.27, stage/render/prepare_assets 19.85, stage/render/render 13.83, stage/main/post_update 12.84, render/graph/ms 11.78, encode//sum 8.92, stage/render/graph_submit 8.10, stream/drive/ms 6.29, encode/submit 4.97 |
| measure | 18.31 | 34.0 | main/busy/ms 40.76, stage/main/update 27.56, render/thread/ms 23.25, render/busy/ms 22.25, stage/render/render 11.10, render/graph/ms 9.74, stage/main/post_update 9.01, encode//sum 7.01, stage/render/graph_submit 6.77, encode/submit 3.98, stage/render/prepare_assets 3.70, encode/finish_wall 2.83 |
| measure | 59.15 | 25.8 | render/thread/ms 37.32, render/busy/ms 35.97, main/busy/ms 31.25, stage/render/render 15.27, stage/main/post_update 14.66, render/graph/ms 13.31, stage/render/prepare_assets 11.91, encode//sum 11.62, stage/main/update 10.28, stage/render/graph_submit 9.06, encode/submit 5.58, stage/render/camera_driver 4.04 |
| measure | 59.18 | 31.7 | render/thread/ms 37.37, render/busy/ms 36.38, main/busy/ms 24.24, stage/render/prepare_assets 20.06, stage/main/post_update 10.83, stage/render/render 9.34, encode//sum 8.11, render/graph/ms 8.00, stage/main/update 5.75, stage/render/graph_submit 5.20, stage/main/fixed_main_loop 3.93, encode/submit 2.87 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=atlas48-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1536 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **85%** (min 55%) · 2625 MHz (min 2625) · memory 8101 MHz · 81.8 W · 73 °C (max 78) · VRAM 6468 MiB · throttled 100% of samples

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
