# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6568 frames over 65.6 s · **median 9.88 ms (101 fps)** · mean 9.99 · p95 11.57 · p99 13.57 (1% low 74 fps) · worst 35.28 ms · 32 spikes over 1.5× median · 30.2 stutters/min (work 28.3/min)
- warm-up: 353 frames over 9.1 s, worst 2386.0 ms
- hitches (> 25.0 ms): **3** measured, 47 in warm-up
- timed GPU passes: 6.64 ms of a 9.99 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.88 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 1.09 min = 30.16/min, limit 1/min
- **FAIL** worst frame 35.28 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.912 | 2.036 | 2.070 | 0.94 | 0.179 |
| main_opaque_pass_3d | 1.530 | 1.630 | 2.355 | 0.94 | 0.063 |
| early prepass | 0.481 | 0.513 | 0.752 | 0.94 | 0.089 |
| bloom | 0.298 | 0.318 | 0.325 | 0.94 | 0.065 |
| atmosphere_luts | 0.294 | 0.313 | 0.379 | 0.94 | 0.003 |
| water_velocity | 0.211 | 0.225 | 0.247 | 0.94 | 0.002 |
| water_sspr | 0.197 | 0.210 | 0.211 | 0.94 | 0.002 |
| ssao | 0.177 | 0.188 | 0.191 | 0.94 | 0.007 |
| point_shadow_atlas | 0.128 | 0.226 | 0.432 | 0.57 | 0.201 |
| atmos_froxel_inject | 0.121 | 0.128 | 0.130 | 0.94 | 0.001 |
| upscaling | 0.120 | 0.128 | 0.135 | 0.94 | 0.004 |
| water_fft | 0.116 | 0.123 | 0.126 | 0.94 | 0.003 |
| uniform_allocation | 0.109 | 0.116 | 0.139 | 0.94 | 0.021 |
| clouds_composite | 0.099 | 0.105 | 0.109 | 0.94 | 0.002 |
| gpu_instance_materialization | 0.091 | 0.097 | 0.101 | 0.94 | 0.004 |
| bin_unpacking | 0.086 | 0.092 | 0.113 | 0.94 | 0.016 |
| tonemapping | 0.081 | 0.086 | 0.091 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.067 | 0.071 | 0.132 | 0.94 | 0.008 |
| clustering | 0.060 | 0.064 | 0.067 | 0.94 | 0.020 |
| clouds_trace | 0.052 | 0.055 | 0.079 | 0.94 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.054 | 0.071 | 0.94 | 0.003 |
| render_sky | 0.049 | 0.053 | 0.054 | 0.94 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.94 | 0.001 |
| surface_trail | 0.035 | 0.057 | 0.057 | 0.62 | 0.002 |
| lightprobe_irradiance_map | 0.035 | 0.414 | 0.417 | 0.09 | 0.001 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.94 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.025 | 0.044 | 0.066 | 0.57 | 0.009 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.94 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.94 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.94 | 0.003 |
| lightprobe_radiance_map | 0.019 | 0.219 | 0.230 | 0.09 | 0.004 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.94 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.031 | 0.94 | 0.184 |
| point_shadow_atlas/uniform_allocation | 0.013 | 0.023 | 0.058 | 0.57 | 0.006 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.94 | 0.003 |
| point_shadow_atlas/bin_unpacking | 0.010 | 0.017 | 0.039 | 0.57 | 0.004 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.94 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.57 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.94 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.783 | 10.070 | 34.16 | 57689.7 |
| render/busy/ms | 8.277 | 9.535 | 33.57 | 54365.4 |
| main/busy/ms | 8.205 | 9.318 | 34.93 | 53889.6 |
| stage/render/render | 5.131 | 6.028 | 31.21 | 33703.4 |
| stage/main/post_update | 4.632 | 5.388 | 31.19 | 30423.6 |
| render/graph/ms | 4.471 | 5.287 | 30.45 | 29362.9 |
| encode//sum | 3.300 | 4.274 | 30.02 | 21671.1 |
| stage/render/graph_submit | 3.169 | 3.885 | 29.26 | 20813.0 |
| encode/submit | 1.998 | 2.730 | 6.07 | 13119.7 |
| stage/main/update | 1.870 | 2.324 | 23.30 | 12285.1 |
| stage/render/camera_driver | 1.239 | 1.454 | 2.98 | 8140.3 |
| encode/finish_wall | 1.236 | 1.647 | 26.92 | 8115.4 |
| encode/pass:early prepass/sum | 0.828 | 1.123 | 26.56 | 5436.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.822 | 1.116 | 2.00 | 5398.7 |
| encode/pass:shadow_directional_light/sum | 0.822 | 1.268 | 2.41 | 5398.3 |
| stage/render/prepare_write_phase_buffers | 0.593 | 0.861 | 1.66 | 3894.6 |
| bigspace/total/ms | 0.572 | 1.346 | 2.31 | 3757.3 |
| stage/render/prepare_assets | 0.510 | 0.669 | 9.83 | 3346.7 |
| stage/main/fixed_main_loop | 0.508 | 0.930 | 1.87 | 3339.6 |
| stage/render/prepare_meshes | 0.425 | 0.525 | 3.32 | 2790.7 |
| stage/render/prepare_resources | 0.424 | 0.504 | 0.83 | 2785.8 |
| render/present/ms | 0.412 | 1.097 | 2.01 | 2704.6 |
| stage/main/pre_update | 0.387 | 0.550 | 1.99 | 2540.9 |
| stage/main/first | 0.366 | 0.475 | 5.74 | 2404.1 |
| stage/render/prepare_flush | 0.330 | 0.551 | 3.84 | 2167.1 |
| controller/feed/ms | 0.325 | 0.420 | 0.86 | 2137.6 |
| stage/render/prepare_bind_groups | 0.305 | 0.488 | 1.53 | 2004.0 |
| stage/render/prepare_batch_phases | 0.282 | 0.446 | 0.81 | 1849.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.274 | 0.455 | 1.07 | 1802.6 |
| stage/main/last | 0.258 | 0.307 | 1.24 | 1691.7 |
| bigspace/low_precision/ms | 0.248 | 0.781 | 1.98 | 1631.9 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.212 | 0.992 | 1.99 | 1392.4 |
| stream/drive/ms | 0.205 | 0.283 | 9.89 | 1348.1 |
| stage/render/create_views | 0.201 | 0.266 | 0.64 | 1322.1 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.197 | 1.001 | 1.60 | 1294.0 |
| stage/render/extract_commands | 0.184 | 0.217 | 0.49 | 1207.7 |
| bigspace/high_precision/ms | 0.172 | 0.776 | 1.79 | 1130.9 |
| anim/evaluate/ms | 0.166 | 0.291 | 27.02 | 1091.5 |
| anim/evaluate_cpu/ms | 0.154 | 0.189 | 0.62 | 1013.1 |
| stage/render/specialize | 0.153 | 0.245 | 1.69 | 1003.9 |
| stream/tick/ms | 0.152 | 0.215 | 0.51 | 996.0 |
| stage/render/prepare_views | 0.136 | 0.181 | 0.56 | 894.6 |
| stage/render/queue | 0.098 | 0.168 | 1.62 | 641.7 |
| render/acquire/ms | 0.094 | 0.125 | 0.49 | 619.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.091 | 0.145 | 0.62 | 600.3 |
| stage/render/phase_sort | 0.082 | 0.113 | 0.33 | 538.0 |
| controller/move/ms | 0.075 | 0.208 | 0.43 | 492.1 |
| controller/write/ms | 0.075 | 0.102 | 0.36 | 489.5 |
| terrain/update/ms | 0.060 | 0.292 | 1.97 | 394.4 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.054 | 0.000 | 2.76 | 353.1 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.075 | 0.28 | 311.1 |
| physics/step/ms | 0.039 | 0.083 | 0.75 | 257.1 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.53 | 209.9 |
| pipelines/compile_ms | 0.026 | 0.000 | 83.55 | 169.3 |
| pipelines/front_ms | 0.025 | 0.000 | 80.37 | 163.2 |
| physics/insert/ms | 0.024 | 0.279 | 0.86 | 155.4 |
| stage/render/cleanup | 0.023 | 0.031 | 0.42 | 149.7 |
| terrain/select/ms | 0.022 | 0.258 | 0.80 | 146.0 |
| weather/map/ms | 0.018 | 0.025 | 0.17 | 116.7 |
| far/select/ms | 0.015 | 0.164 | 0.63 | 96.3 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.47 | 83.0 |
| anim/apply/ms | 0.012 | 0.020 | 0.45 | 77.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 71.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.43 | 70.1 |
| script/update/ms | 0.008 | 0.013 | 0.04 | 54.4 |
| controller/avoid/ms | 0.006 | 0.012 | 0.22 | 39.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.04 | 35.8 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 33.0 |
| script/ui/ms | 0.004 | 0.005 | 0.11 | 23.3 |
| stream/spawn/ms | 0.003 | 0.000 | 1.11 | 21.4 |
| stream/upload/ms | 0.003 | 0.000 | 0.98 | 18.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.71 | 13.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.22 | 7.6 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.18 | 6.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.21 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 2 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 559 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1313 | 2262 | 595 |
| crowd/c0/meshes/count | count | 357 | 646 | 158 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5507 | 5951 | 5601 |
| crowd/c1/meshes/count | count | 1165 | 1289 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6362 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1819 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 205 |
| crowd/joints_extracted/count | count | 691 | 6917 | 501 |
| crowd/joints_uploaded/count | count | 3591 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.43 | 26.75 | 0.56 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.47 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.43 | 0.01 |
| encode/finish_critical | ms | 0.99 | 26.75 | 0.96 |
| encode/pass:early prepass/max | ms | 0.81 | 26.56 | 0.96 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 2 | 0.92 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.62 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 1.07 | 0.36 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.79 | 1.99 | 1.28 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.85 | 1.60 | 0.19 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.15 | 2.76 | 0.98 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.41 | 0.80 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.02 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.58 | 34.93 | 8.90 |
| gpu/timed/ms | ms | 7.78 | 9.40 | 8.72 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 149 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 315 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 149 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 315 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 769 | 961 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2486 | 3512 | 2457 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 769 | 961 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2486 | 3512 | 2457 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 80 | 297 | 27 |
| phases/shadow/bins | count | 809 | 1318 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 8 |
| phases/shadow/instance_draws | count | 6557 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 98.3 k |
| phases/shadow/items | count | 2310 | 3009 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 53 | 129 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2709 | 3182 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 334 | 335 | 335 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 15 | 299 | 43 |
| preprocess/bind_groups_created | count | 10 | 287 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 6 |
| preprocess/bind_groups_reused | count | 139 | 219 | 123 |
| process/cpu_usage | % | 25.48 | 40.87 | 30.36 |
| process/mem_usage | GiB | 5.11 | 5.29 | 5.29 |
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
| shadow_atlas/empty_faces | count | 0 | 1 | 0 |
| shadow_atlas/lamps | count | 8 | 20 | 0 |
| shadow_atlas/live_occ_pct | % | 3.81 | 32.42 | 0 |
| shadow_atlas/live_tiles | count | 16 | 55 | 0 |
| shadow_atlas/mover_draws | count | 10 | 34 | 0 |
| shadow_atlas/movers | count | 137 | 367 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 34 | 0 |
| shadow_atlas/render/clears | count | 0 | 16 | 0 |
| shadow_atlas/render/lamps | count | 8 | 20 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 34 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 10 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 16 | 54 | 0 |
| shadow_atlas/resizes | count | 30 | 39 | 39 |
| shadow_atlas/static_occ_pct | % | 16.89 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 107 | 0 |
| shadow_atlas/views | count | 16 | 54 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 120 | 11 |
| skin/blocks_skipped/count | count | 145 | 206 | 194 |
| skin/blocks_walked/count | count | 14 | 25 | 11 |
| skin/joint_bytes_written/bytes | bytes | 48.9 k | 808.1 k | 33.3 k |
| skin/joints_written/count | count | 764 | 12.6 k | 520 |
| skin/upload_writes/count | count | 7 | 24 | 9 |
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
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.58 | 56.77 | 47.09 |
| system/mem_usage | % | 46.92 | 48.19 | 48.06 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 138 | 350 | 85 |
| views/all/bins | count | 2936 | 3256 | 2948 |
| views/all/count | count | 19 | 57 | 3 |
| views/all/instance_batches | count | 12 | 22 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 196.6 k |
| views/all/items | count | 8036 | 10.1 k | 7966 |
| views/all/pipelines | count | 100 | 172 | 66 |
| views/all/pipelines_distinct | count | 66 | 73 | 61 |
| views/all/work_items | count | 154.1 k | 230.9 k | 205.0 k |
| views/cascade/batch_sets | count | 13 | 33 | 27 |
| views/cascade/bins | count | 716 | 842 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6557 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2118 | 2393 | 2022 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 57 | 284 | 0 |
| views/lamp_face/bins | count | 108 | 563 | 0 |
| views/lamp_face/count | count | 16 | 54 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 157 | 713 | 0 |
| views/lamp_face/pipelines | count | 28 | 104 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 120 | 25.1 k | 0 |
| views/main/batch_sets | count | 57 | 61 | 58 |
| views/main/bins | count | 2014 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7460 | 5944 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.5 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 49.43 | 216 | 324.7 k |
| anim/events/count | count | 0.09 | 3 | 582 |
| assets/Image/added | count | 0.09 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.44 | 253 | 2874 |
| assets/Mesh/removed | count | 0.74 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.14 | 10 | 7476 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1260 | 1489 |
| encode//count | count | 27.91 | 30 | 183.3 k |
| encode/bind_walk/init_textures | count | 9317 | 25.1 k | 61.20 M |
| encode/bind_walk/init_textures_skipped | count | 49.3 k | 76.1 k | 323.75 M |
| encode/bind_walk/inits | count | 556 | 770 | 3.65 M |
| encode/bind_walk/inits_skipped | count | 167 | 244 | 1.10 M |
| encode/bind_walk/merge_textures | count | 9254 | 24.9 k | 60.78 M |
| encode/bind_walk/merge_textures_skipped | count | 11.8 k | 29.3 k | 77.74 M |
| encode/bind_walk/merges | count | 104 | 183 | 680.0 k |
| encode/bind_walk/merges_skipped | count | 148 | 227 | 971.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6568 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6568 |
| encode/pass:early prepass/count | count | 1 | 1 | 6568 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6568 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6568 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6568 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.26 | 1 | 1723 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.28 | 1 | 1854 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 289 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6568 |
| encode/queue/command_buffers | count | 38.51 | 42 | 253.0 k |
| encode/queue/submits | count | 2.01 | 4 | 13.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.31 M | 3.26 M | 15.15 G |
| encode/queue/write_buffer/calls | count | 268 | 467 | 1.76 M |
| encode/queue/write_buffer_with/bytes | bytes | 157.5 k | 1.29 M | 1.03 G |
| encode/queue/write_buffer_with/calls | count | 10.30 | 768 | 67.6 k |
| encode/queue/write_texture/bytes | bytes | 1849 | 589.8 k | 12.15 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 556 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6568 |
| physics/inserts | count | 0.34 | 95 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 163 | 2335 |
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
| stream/deactivate/items | count | 0.03 | 25 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5154 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1130 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 50, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.39 | 2386.0 | main/busy/ms 2357, stage/main/post_update 2314, stage/main/update 32.05, stage/main/pre_update 7.57, stage/main/last 1.07, surface/advance_cpu_ms 0.77, stream/drive/ms 0.53, stage/main/first 0.47, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.28, stream/tick/ms 0.15, bigspace/total/ms 0.13 |
| warm-up | 3.15 | 760.4 | main/busy/ms 2488, stage/main/post_update 2337, render/thread/ms 728, render/busy/ms 589, stage/render/prepare_assets 502, stage/render/prepare_views 136, render/acquire/ms 136, stage/main/update 133, stage/render/create_views 28.60, stage/render/prepare_batch_phases 25.98, stage/render/render 23.59, render/graph/ms 19.39 |
| warm-up | 3.31 | 161.8 | render/thread/ms 874, render/busy/ms 648, stage/render/prepare_assets 548, pipelines/compile_ms 217, main/busy/ms 159, stage/render/prepare_views 137, render/acquire/ms 136, pipelines/front_ms 123, stage/render/render 113, stage/main/update 107, pipelines/driver_ms 93.91, render/present/ms 88.90 |
| warm-up | 3.35 | 36.9 | pipelines/compile_ms 643, pipelines/front_ms 469, render/thread/ms 180, pipelines/driver_ms 174, stage/render/render 117, main/busy/ms 112, render/busy/ms 91.10, render/present/ms 88.96, pipelines/wait_ms 85.95, stage/main/update 75.13, stage/render/prepare_assets 47.52, render/graph/ms 26.79 |
| warm-up | 3.51 | 164.8 | pipelines/wait_ms 3627, pipelines/compile_ms 1885, pipelines/front_ms 1688, pipelines/driver_ms 197, main/busy/ms 115, render/thread/ms 112, render/busy/ms 108, stage/main/update 75.23, stage/render/prepare_assets 67.29, stage/render/render 33.78, stage/main/post_update 29.46, render/graph/ms 28.87 |
| warm-up | 3.55 | 37.9 | pipelines/wait_ms 3561, pipelines/compile_ms 1901, pipelines/front_ms 1764, pipelines/driver_ms 137, render/thread/ms 111, render/busy/ms 109, stage/render/prepare_assets 90.69, main/busy/ms 57.61, stage/main/post_update 28.16, terrain/update/ms 16.17, stage/main/update 11.79, stage/render/render 10.51 |
| warm-up | 3.58 | 30.5 | pipelines/compile_ms 759, pipelines/front_ms 728, render/thread/ms 62.39, render/busy/ms 60.77, pipelines/wait_ms 54.90, main/busy/ms 43.06, stage/render/prepare_assets 41.40, pipelines/driver_ms 30.72, stage/main/post_update 15.95, terrain/update/ms 15.50, stage/main/update 10.17, stage/main/fixed_main_loop 8.78 |
| warm-up | 3.61 | 30.2 | pipelines/compile_ms 367, pipelines/front_ms 350, pipelines/wait_ms 80.50, render/thread/ms 57.66, render/busy/ms 55.79, main/busy/ms 36.32, encode//sum 20.34, stage/render/render 19.85, stage/render/prepare_assets 18.83, stage/main/post_update 18.79, render/graph/ms 17.90, pipelines/driver_ms 16.73 |
| warm-up | 3.63 | 26.0 | pipelines/compile_ms 267, pipelines/front_ms 254, pipelines/wait_ms 135, render/thread/ms 53.97, render/busy/ms 51.76, main/busy/ms 37.78, stage/main/post_update 21.07, stage/render/render 20.74, encode//sum 19.20, render/graph/ms 18.60, pipelines/driver_ms 13.21, stage/main/update 11.54 |
| warm-up | 3.71 | 71.0 | pipelines/compile_ms 788, pipelines/front_ms 753, pipelines/wait_ms 105, render/thread/ms 94.23, render/busy/ms 91.56, main/busy/ms 81.95, stage/main/update 53.53, pipelines/driver_ms 35.47, stage/render/prepare_write_phase_buffers 28.50, stage/main/post_update 21.87, stage/render/prepare_resources 17.34, stage/render/render 15.59 |
| warm-up | 3.78 | 75.8 | pipelines/compile_ms 2537, pipelines/front_ms 2424, pipelines/wait_ms 218, render/thread/ms 124, render/busy/ms 122, pipelines/driver_ms 113, main/busy/ms 93.19, stage/main/update 62.99, stage/render/prepare_assets 51.84, stage/main/post_update 20.86, stage/render/prepare_write_phase_buffers 20.73, stage/render/prepare_resources 16.35 |
| warm-up | 3.87 | 84.9 | pipelines/compile_ms 2485, pipelines/front_ms 2382, pipelines/wait_ms 208, render/thread/ms 122, render/busy/ms 120, pipelines/driver_ms 103, stage/render/prepare_assets 66.97, main/busy/ms 41.27, stage/render/prepare_resources 35.02, stage/main/update 22.44, stage/render/render 14.86, render/graph/ms 13.08 |
| warm-up | 3.90 | 35.0 | pipelines/compile_ms 600, pipelines/front_ms 574, render/thread/ms 94.80, render/busy/ms 93.17, stage/render/prepare_assets 36.63, stage/render/prepare_resources 34.47, main/busy/ms 25.41, pipelines/driver_ms 25.35, stage/render/render 18.66, render/graph/ms 16.86, stage/render/graph_submit 14.71, stage/main/update 10.55 |
| warm-up | 4.01 | 44.7 | main/busy/ms 59.67, render/thread/ms 51.83, stage/main/update 45.34, render/busy/ms 45.11, pipelines/compile_ms 27.07, pipelines/front_ms 25.43, stage/render/render 23.25, stream/drive/ms 20.17, stage/render/prepare_assets 17.59, render/graph/ms 15.90, stage/main/post_update 7.90, stage/render/graph_submit 7.70 |
| warm-up | 4.08 | 72.2 | pipelines/compile_ms 891, pipelines/front_ms 867, main/busy/ms 162, stage/main/update 145, render/thread/ms 78.34, render/busy/ms 71.34, stage/render/prepare_assets 40.11, stage/render/render 24.42, pipelines/driver_ms 24.16, stream/drive/ms 20.46, render/graph/ms 16.70, stage/render/graph_submit 8.94 |
| warm-up | 4.25 | 172.8 | pipelines/compile_ms 2299, pipelines/front_ms 2238, main/busy/ms 136, stage/main/update 120, render/thread/ms 110, render/busy/ms 108, stage/render/prepare_assets 64.10, pipelines/driver_ms 61.82, stage/render/prepare_flush 23.38, stage/render/render 12.21, render/graph/ms 9.99, stage/main/post_update 8.46 |
| warm-up | 4.29 | 31.9 | pipelines/compile_ms 1465, pipelines/front_ms 1425, render/thread/ms 96.06, render/busy/ms 94.40, main/busy/ms 50.01, stage/render/prepare_assets 45.08, pipelines/driver_ms 40.30, stage/main/update 34.42, stage/render/prepare_flush 24.96, stage/render/render 17.06, render/graph/ms 14.94, stage/render/graph_submit 11.65 |
| warm-up | 4.33 | 40.9 | pipelines/compile_ms 68.04, pipelines/front_ms 65.00, render/thread/ms 60.60, render/busy/ms 58.98, main/busy/ms 53.29, stage/main/update 34.52, stage/render/prepare_assets 30.69, stage/render/render 16.46, render/graph/ms 14.41, stage/render/graph_submit 11.54, stage/main/post_update 10.75, encode/submit 7.39 |
| warm-up | 4.39 | 61.2 | render/thread/ms 84.33, render/busy/ms 83.03, main/busy/ms 75.66, stage/main/update 39.69, pipelines/compile_ms 38.38, pipelines/front_ms 36.33, stage/render/prepare_assets 35.03, stage/main/post_update 20.13, stage/render/prepare_batch_phases 14.49, stage/render/render 13.27, render/graph/ms 11.52, encode//sum 7.56 |
| warm-up | 4.45 | 63.6 | pipelines/compile_ms 364, pipelines/front_ms 353, render/thread/ms 94.76, render/busy/ms 93.15, main/busy/ms 91.39, stage/main/update 39.74, stage/render/prepare_assets 38.49, stage/main/post_update 32.38, stage/render/render 15.27, stage/render/prepare_batch_phases 14.67, render/graph/ms 12.63, pipelines/driver_ms 11.48 |
| warm-up | 4.49 | 42.6 | pipelines/compile_ms 2303, pipelines/front_ms 2233, render/thread/ms 73.10, render/busy/ms 71.25, pipelines/driver_ms 69.98, main/busy/ms 69.16, stage/render/prepare_assets 36.66, stage/main/post_update 29.54, stage/main/update 24.32, pipelines/wait_ms 15.39, stage/render/render 13.65, render/graph/ms 10.64 |
| warm-up | 4.56 | 69.7 | pipelines/compile_ms 2494, pipelines/front_ms 2415, render/thread/ms 84.48, render/busy/ms 82.76, pipelines/driver_ms 78.26, main/busy/ms 57.82, stage/render/prepare_assets 47.92, pipelines/wait_ms 42.18, stage/main/update 22.33, stage/main/post_update 21.32, stage/render/render 14.43, render/graph/ms 12.05 |
| warm-up | 4.62 | 55.3 | pipelines/compile_ms 697, pipelines/front_ms 671, main/busy/ms 111, pipelines/wait_ms 106, render/thread/ms 103, render/busy/ms 101, stage/main/update 74.40, stage/render/prepare_assets 45.81, pipelines/driver_ms 25.84, stage/render/render 22.85, render/graph/ms 20.05, stage/main/post_update 16.88 |
| warm-up | 4.79 | 167.4 | pipelines/compile_ms 2630, pipelines/front_ms 2481, main/busy/ms 159, pipelines/driver_ms 149, render/thread/ms 142, render/busy/ms 140, stage/main/update 125, pipelines/wait_ms 117, stage/render/prepare_assets 86.31, stream/drive/ms 61.16, stage/render/render 22.63, render/graph/ms 20.10 |
| warm-up | 4.82 | 34.1 | pipelines/compile_ms 2783, pipelines/front_ms 2631, pipelines/driver_ms 153, render/thread/ms 123, render/busy/ms 122, main/busy/ms 100, stage/render/prepare_assets 84.87, stage/main/update 76.62, stream/drive/ms 60.00, pipelines/wait_ms 50.33, stage/render/render 16.39, render/graph/ms 14.66 |
| warm-up | 4.85 | 31.7 | pipelines/compile_ms 382, pipelines/front_ms 369, render/thread/ms 55.30, render/busy/ms 53.71, main/busy/ms 52.44, stage/main/update 34.24, stage/render/render 17.42, render/graph/ms 15.64, pipelines/wait_ms 14.04, pipelines/driver_ms 13.21, stage/render/graph_submit 12.50, stage/render/prepare_assets 10.93 |
| warm-up | 4.89 | 34.0 | pipelines/compile_ms 126, pipelines/front_ms 120, render/thread/ms 52.48, main/busy/ms 51.68, render/busy/ms 50.98, stage/main/update 34.12, stage/render/prepare_assets 14.57, stage/render/render 13.48, render/graph/ms 11.72, stage/main/post_update 8.81, stage/render/graph_submit 8.61, encode//sum 7.65 |
| warm-up | 4.91 | 26.4 | pipelines/compile_ms 78.62, pipelines/front_ms 75.01, render/thread/ms 48.95, render/busy/ms 47.72, main/busy/ms 38.18, stage/main/update 20.52, stage/render/prepare_assets 15.59, stage/render/render 12.32, render/graph/ms 10.69, stage/main/post_update 9.16, stage/render/graph_submit 7.41, encode//sum 6.69 |
| warm-up | 4.98 | 25.7 | pipelines/compile_ms 745, pipelines/front_ms 718, render/thread/ms 41.76, render/busy/ms 40.04, main/busy/ms 37.98, pipelines/driver_ms 26.66, stage/render/render 20.62, stage/main/update 19.22, render/graph/ms 18.22, stage/render/graph_submit 13.87, stage/main/post_update 11.97, encode//sum 7.83 |
| warm-up | 5.07 | 37.4 | pipelines/compile_ms 403, pipelines/front_ms 393, render/thread/ms 45.94, render/busy/ms 44.35, main/busy/ms 37.80, stage/main/update 18.62, stage/render/prepare_write_phase_buffers 16.85, stage/main/post_update 14.79, stage/render/render 12.95, render/graph/ms 10.92, pipelines/driver_ms 10.54, stage/render/graph_submit 7.80 |
| warm-up | 5.10 | 28.9 | pipelines/compile_ms 84.80, pipelines/front_ms 81.24, render/thread/ms 56.76, render/busy/ms 55.49, main/busy/ms 39.30, pipelines/wait_ms 34.44, stage/main/update 20.53, stage/render/prepare_write_phase_buffers 16.69, stage/render/render 12.24, render/graph/ms 10.57, stage/main/post_update 9.31, encode//sum 8.15 |
| warm-up | 5.13 | 28.2 | render/thread/ms 52.54, main/busy/ms 51.95, render/busy/ms 50.76, pipelines/wait_ms 33.15, pipelines/compile_ms 32.35, pipelines/front_ms 30.82, stage/main/update 28.40, stage/render/render 17.87, render/graph/ms 15.74, stage/render/graph_submit 12.43, stream/drive/ms 12.28, encode//sum 11.90 |
| warm-up | 5.19 | 64.3 | pipelines/compile_ms 368, pipelines/front_ms 357, render/thread/ms 80.36, render/busy/ms 78.62, main/busy/ms 45.17, stage/main/update 26.93, stage/render/prepare_resources 17.94, stage/render/render 17.65, stage/render/prepare_assets 17.40, render/graph/ms 15.25, stream/drive/ms 12.20, stage/render/prepare_bind_groups 12.04 |
| warm-up | 5.59 | 26.3 | pipelines/compile_ms 205, pipelines/front_ms 192, render/thread/ms 32.13, render/busy/ms 30.98, main/busy/ms 24.72, stage/render/prepare_assets 14.14, pipelines/driver_ms 12.96, stage/main/post_update 10.29, stage/render/render 9.82, stage/main/update 9.03, render/graph/ms 8.20, encode//sum 7.06 |
| warm-up | 5.72 | 26.7 | pipelines/compile_ms 714, pipelines/front_ms 691, main/busy/ms 37.55, stage/main/update 25.78, render/thread/ms 24.92, render/busy/ms 23.76, pipelines/driver_ms 23.04, stream/drive/ms 17.36, stage/render/render 11.82, render/graph/ms 10.09, stage/main/post_update 7.13, stage/render/graph_submit 6.52 |
| warm-up | 5.84 | 27.9 | render/thread/ms 40.58, render/busy/ms 39.09, main/busy/ms 29.71, stage/render/render 14.53, stage/main/update 13.39, render/graph/ms 12.39, stage/main/post_update 12.29, stage/render/prepare_assets 11.95, encode//sum 8.57, stage/render/graph_submit 8.44, encode/submit 5.33, stage/render/camera_driver 3.68 |
| warm-up | 5.89 | 30.7 | pipelines/compile_ms 435, pipelines/front_ms 422, render/thread/ms 35.85, render/busy/ms 34.97, main/busy/ms 24.49, stage/render/prepare_assets 19.53, pipelines/driver_ms 12.93, stage/main/update 10.04, stage/render/render 8.14, stage/main/post_update 7.66, render/graph/ms 6.71, encode//sum 6.48 |
| warm-up | 6.08 | 25.0 | main/busy/ms 39.01, render/thread/ms 38.65, render/busy/ms 37.26, stage/main/update 20.87, stage/render/render 14.52, render/graph/ms 12.60, stage/main/post_update 12.50, encode//sum 9.23, stage/render/prepare_assets 8.84, stage/render/graph_submit 8.67, encode/submit 5.69, stage/render/prepare_meshes 3.94 |
| warm-up | 6.11 | 31.2 | main/busy/ms 42.97, render/thread/ms 42.70, render/busy/ms 41.25, stage/main/update 23.26, stage/render/render 14.30, stage/main/post_update 13.23, render/graph/ms 12.45, stage/render/prepare_assets 10.97, encode//sum 9.36, stage/render/graph_submit 8.63, encode/submit 5.89, stage/render/prepare_meshes 4.84 |
| warm-up | 6.14 | 33.3 | render/thread/ms 47.92, render/busy/ms 46.87, main/busy/ms 39.31, stage/main/update 18.54, stage/render/prepare_assets 15.27, stage/render/render 13.29, stage/main/post_update 11.80, render/graph/ms 11.67, stage/render/graph_submit 8.31, encode//sum 7.41, stage/render/prepare_meshes 5.49, encode/finish_wall 5.17 |
| warm-up | 6.19 | 41.9 | pipelines/compile_ms 151, pipelines/front_ms 144, render/thread/ms 65.85, render/busy/ms 64.79, main/busy/ms 45.43, stage/render/render 21.22, stage/main/update 20.10, render/graph/ms 19.22, stage/render/prepare_assets 18.96, stage/main/post_update 15.55, stage/render/graph_submit 14.36, encode/finish_wall 10.91 |
| warm-up | 6.21 | 26.0 | pipelines/compile_ms 151, pipelines/front_ms 144, render/thread/ms 59.78, render/busy/ms 58.67, main/busy/ms 42.56, stage/render/render 20.23, stage/main/update 19.11, render/graph/ms 18.44, stage/main/post_update 15.88, stage/render/prepare_assets 15.27, stage/render/graph_submit 14.29, encode/finish_wall 10.20 |
| warm-up | 6.25 | 35.0 | render/thread/ms 47.09, render/busy/ms 46.00, main/busy/ms 34.47, stage/main/update 16.94, stage/render/prepare_assets 15.78, stage/render/render 15.65, render/graph/ms 14.17, pipelines/compile_ms 14.10, pipelines/front_ms 13.49, stage/render/graph_submit 11.46, stage/main/post_update 11.27, encode/finish_wall 7.59 |
| warm-up | 6.40 | 44.6 | main/busy/ms 55.55, render/thread/ms 46.65, render/busy/ms 44.81, stage/main/update 37.89, stage/render/render 21.01, render/graph/ms 18.47, stage/render/graph_submit 14.98, stream/drive/ms 14.20, stage/main/post_update 11.71, stage/render/prepare_assets 10.37, encode/finish_wall 8.45, encode//sum 8.21 |
| warm-up | 6.44 | 34.3 | pipelines/compile_ms 138, pipelines/front_ms 135, render/thread/ms 52.66, render/busy/ms 51.21, main/busy/ms 42.62, stage/main/update 24.82, stage/render/prepare_assets 19.25, stage/render/render 18.20, render/graph/ms 16.12, stage/render/graph_submit 12.74, stage/main/post_update 10.89, encode//sum 8.10 |
| warm-up | 6.46 | 25.6 | pipelines/compile_ms 138, pipelines/front_ms 135, render/thread/ms 42.98, render/busy/ms 41.82, main/busy/ms 36.32, stage/render/prepare_assets 19.16, stage/main/update 17.73, stage/render/render 13.99, render/graph/ms 12.39, stage/main/post_update 11.45, stage/render/graph_submit 9.97, stream/drive/ms 6.42 |
| warm-up | 6.95 | 57.0 | render/thread/ms 65.50, render/busy/ms 64.44, stage/render/render 57.50, render/graph/ms 56.06, encode//sum 53.66, stage/render/graph_submit 53.52, encode/finish_wall 49.18, encode/pass:early prepass/sum 48.68, main/busy/ms 17.62, stage/main/post_update 8.80, stage/main/update 5.95, encode/submit 4.37 |
| measure | 0.86 | 35.3 | render/thread/ms 42.52, main/busy/ms 42.15, render/busy/ms 41.51, stage/render/render 35.89, stage/main/post_update 35.46, render/graph/ms 34.60, encode//sum 32.80, stage/render/graph_submit 32.02, encode/finish_wall 27.97, encode/pass:early prepass/sum 27.22, anim/evaluate/ms 27.11, encode/submit 4.09 |
| measure | 18.35 | 32.4 | main/busy/ms 39.04, stage/main/update 25.31, render/thread/ms 21.33, render/busy/ms 20.46, stage/render/render 10.93, render/graph/ms 9.51, stage/main/post_update 8.76, stage/render/graph_submit 6.41, encode//sum 6.18, encode/submit 3.92, stage/render/prepare_assets 3.65, stage/render/camera_driver 2.97 |
| measure | 59.14 | 28.0 | render/thread/ms 37.11, render/busy/ms 36.06, main/busy/ms 25.44, stage/render/prepare_assets 15.48, stage/main/post_update 13.06, stage/render/render 11.62, render/graph/ms 10.02, encode//sum 9.62, stage/render/graph_submit 6.05, stage/main/update 4.94, stage/render/camera_driver 3.78, encode/finish_wall 3.15 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **84%** (min 59%) · 2618 MHz (min 2610) · memory 8101 MHz · 84.7 W · 80 °C (max 85) · VRAM 5516 MiB · throttled 100% of samples

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
