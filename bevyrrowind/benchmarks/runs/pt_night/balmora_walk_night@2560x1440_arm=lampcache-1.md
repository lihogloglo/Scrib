# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5763 frames over 65.6 s · **median 10.76 ms (93 fps)** · mean 11.39 · p95 15.61 · p99 17.26 (1% low 58 fps) · worst 35.07 ms · 182 spikes over 1.5× median · 12.8 stutters/min (work 13.7/min)
- warm-up: 324 frames over 9.2 s, worst 2382.1 ms
- hitches (> 26.9 ms): **5** measured, 47 in warm-up
- timed GPU passes: 6.81 ms of a 11.39 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.76 ms, limit 11 ms
- **FAIL** frame_ms: 14 stutters (> 1.5x moving median of 60) in 1.09 min = 12.80/min, limit 1/min
- **FAIL** worst frame 35.07 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.968 | 2.030 | 2.063 | 0.97 | 0.177 |
| main_opaque_pass_3d | 1.278 | 1.318 | 1.661 | 0.97 | 0.060 |
| auxiliary_views | 0.504 | 0.519 | 1.705 | 0.97 | 0.463 |
| early prepass | 0.481 | 0.496 | 0.707 | 0.97 | 0.086 |
| bloom | 0.306 | 0.316 | 0.322 | 0.97 | 0.064 |
| atmosphere_luts | 0.299 | 0.309 | 0.371 | 0.97 | 0.003 |
| water_velocity | 0.253 | 0.261 | 0.350 | 0.97 | 0.001 |
| water_sspr | 0.195 | 0.201 | 0.203 | 0.97 | 0.002 |
| ssao | 0.182 | 0.188 | 0.193 | 0.97 | 0.007 |
| upscaling | 0.125 | 0.129 | 0.136 | 0.97 | 0.004 |
| water_fft | 0.124 | 0.128 | 0.136 | 0.97 | 0.003 |
| atmos_froxel_inject | 0.123 | 0.127 | 0.128 | 0.97 | 0.001 |
| uniform_allocation | 0.123 | 0.127 | 0.154 | 0.97 | 0.024 |
| clouds_composite | 0.101 | 0.105 | 0.108 | 0.97 | 0.002 |
| bin_unpacking | 0.095 | 0.098 | 0.120 | 0.97 | 0.017 |
| tonemapping | 0.083 | 0.086 | 0.091 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.076 | 0.078 | 0.156 | 0.97 | 0.008 |
| clustering | 0.062 | 0.064 | 0.067 | 0.97 | 0.018 |
| clouds_trace | 0.055 | 0.056 | 0.074 | 0.97 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.414 | 0.416 | 0.11 | 0.001 |
| surface_trail | 0.040 | 0.056 | 0.057 | 0.71 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.97 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.029 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.219 | 0.229 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.036 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.023 | 0.97 | 0.124 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.009 | 0.010 | 0.015 | 0.97 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.177 | 14.365 | 34.02 | 58647.2 |
| render/busy/ms | 9.646 | 13.811 | 33.29 | 55592.4 |
| main/busy/ms | 8.917 | 11.072 | 27.69 | 51387.9 |
| encode//sum | 7.565 | 15.497 | 31.05 | 43598.0 |
| stage/render/render | 6.541 | 9.930 | 30.19 | 37696.5 |
| render/graph/ms | 5.863 | 9.190 | 29.28 | 33786.7 |
| stage/main/post_update | 5.583 | 7.277 | 9.91 | 32176.4 |
| stage/render/graph_submit | 4.377 | 6.721 | 26.84 | 25226.5 |
| encode/submit | 2.741 | 4.442 | 24.77 | 15794.5 |
| stage/main/update | 2.123 | 2.568 | 21.66 | 12236.1 |
| encode/finish_wall | 1.680 | 2.674 | 4.51 | 9680.8 |
| encode/pass:shadow_point_light/sum | 1.443 | 5.054 | 8.08 | 8314.0 |
| stage/render/camera_driver | 1.431 | 2.485 | 4.34 | 8246.4 |
| encode/pass:shadow_directional_light/sum | 0.746 | 1.264 | 3.50 | 4299.8 |
| stage/render/prepare_write_phase_buffers | 0.719 | 1.087 | 2.70 | 4141.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.710 | 1.075 | 2.46 | 4093.0 |
| encode/pass:early prepass/sum | 0.685 | 1.066 | 2.30 | 3946.6 |
| stage/render/prepare_assets | 0.591 | 0.729 | 10.11 | 3408.0 |
| bigspace/total/ms | 0.572 | 1.421 | 3.60 | 3293.6 |
| stage/render/prepare_resources | 0.442 | 0.531 | 1.05 | 2546.0 |
| render/present/ms | 0.436 | 0.542 | 2.13 | 2513.7 |
| stage/render/prepare_meshes | 0.407 | 0.520 | 2.13 | 2343.2 |
| stage/main/first | 0.351 | 0.457 | 6.57 | 2022.4 |
| stage/main/pre_update | 0.348 | 0.464 | 1.55 | 2007.3 |
| terrain/update/ms | 0.330 | 0.497 | 2.81 | 1899.7 |
| terrain/select/ms | 0.296 | 0.452 | 0.94 | 1707.2 |
| stage/render/prepare_bind_groups | 0.271 | 0.479 | 1.75 | 1564.5 |
| controller/feed/ms | 0.257 | 0.324 | 0.99 | 1483.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.257 | 0.398 | 0.94 | 1480.4 |
| stream/drive/ms | 0.251 | 0.332 | 19.72 | 1448.4 |
| bigspace/low_precision/ms | 0.237 | 0.847 | 3.16 | 1365.3 |
| stage/main/fixed_main_loop | 0.219 | 0.362 | 1.50 | 1262.1 |
| far/select/ms | 0.200 | 0.266 | 0.70 | 1154.2 |
| stream/tick/ms | 0.193 | 0.260 | 0.68 | 1111.5 |
| stage/render/create_views | 0.191 | 0.257 | 1.02 | 1099.7 |
| stage/render/prepare_batch_phases | 0.189 | 0.270 | 0.56 | 1090.9 |
| stage/render/specialize | 0.187 | 0.306 | 1.72 | 1075.6 |
| stage/render/queue | 0.185 | 0.314 | 1.61 | 1065.0 |
| stage/render/extract_commands | 0.174 | 0.205 | 0.57 | 1001.2 |
| anim/evaluate/ms | 0.171 | 0.302 | 2.44 | 987.3 |
| bigspace/high_precision/ms | 0.170 | 0.879 | 3.26 | 982.1 |
| stage/render/prepare_flush | 0.170 | 0.334 | 0.84 | 979.3 |
| anim/evaluate_cpu/ms | 0.152 | 0.187 | 0.70 | 874.7 |
| stage/render/prepare_views | 0.137 | 0.169 | 0.37 | 787.8 |
| controller/move/ms | 0.136 | 0.297 | 0.67 | 786.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.120 | 0.212 | 0.69 | 689.3 |
| stage/main/last | 0.118 | 0.158 | 2.85 | 677.7 |
| render/acquire/ms | 0.094 | 0.119 | 0.32 | 541.1 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.120 | 0.33 | 324.0 |
| physics/step/ms | 0.048 | 0.083 | 0.63 | 273.9 |
| encode/point_shadow_cache_copies/sum | 0.042 | 0.148 | 0.76 | 243.7 |
| stage/render/cleanup | 0.038 | 0.125 | 0.73 | 221.8 |
| physics/insert/ms | 0.037 | 0.381 | 1.10 | 214.1 |
| stage/render/phase_sort | 0.034 | 0.059 | 0.30 | 196.7 |
| encode/resolve_diagnostics/sum | 0.028 | 0.039 | 0.49 | 163.2 |
| pipelines/compile_ms | 0.017 | 0.000 | 83.69 | 95.9 |
| pipelines/front_ms | 0.016 | 0.000 | 80.89 | 92.9 |
| weather/map/ms | 0.016 | 0.021 | 0.10 | 90.5 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.51 | 75.3 |
| anim/apply/ms | 0.010 | 0.019 | 0.20 | 59.8 |
| controller/avoid/ms | 0.010 | 0.014 | 0.15 | 59.7 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.47 | 59.1 |
| weather/eval/ms | 0.010 | 0.013 | 0.08 | 56.3 |
| script/update/ms | 0.007 | 0.009 | 0.23 | 39.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 30.8 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 29.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.17 | 28.5 |
| script/ui/ms | 0.005 | 0.006 | 0.07 | 26.7 |
| controller/write/ms | 0.004 | 0.004 | 0.23 | 20.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.82 | 14.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.54 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 3.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.80 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 25 | 43 | 14 |
| crowd/c0/joints/count | count | 1236 | 2262 | 658 |
| crowd/c0/meshes/count | count | 342 | 646 | 188 |
| crowd/c1/count | count | 147 | 155 | 147 |
| crowd/c1/joints/count | count | 5554 | 5894 | 5538 |
| crowd/c1/meshes/count | count | 1155 | 1254 | 1153 |
| crowd/c2/count | count | 400 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 204 |
| crowd/joints_extracted/count | count | 3718 | 8334 | 5731 |
| crowd/joints_uploaded/count | count | 3651 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.49 | 1.84 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.51 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.47 | 0.01 |
| encode/finish_critical | ms | 0.94 | 3.50 | 0.91 |
| encode/pass:early prepass/max | ms | 0.66 | 2.30 | 0.75 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.70 | 2.46 | 0.91 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.69 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.94 | 0.42 |
| encode/pass:shadow_directional_light/max | ms | 0.73 | 3.50 | 0.51 |
| encode/pass:shadow_point_light/max | ms | 0.75 | 1.47 | 0.80 |
| encode/point_shadow_cache_copies/max | ms | 0.07 | 0.76 | 0.05 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.49 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.98 | 33.29 | 7.89 |
| gpu/timed/ms | ms | 7.59 | 9.25 | 7.89 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 9 | 27 | 0 |
| lamp_cache/copies | count | 0 | 15 | 0 |
| lamp_cache/deferred_faces | count | 0 | 16 | 0 |
| lamp_cache/lamps | count | 6 | 8 | 0 |
| lamp_cache/mover_faces | count | 0 | 12 | 0 |
| lamp_cache/movers | count | 95 | 263 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 6 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 15 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 6 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 12 | 32 | 0 |
| lamp_cache/stock_lamps | count | 0 | 2 | 0 |
| lamp_cache/store_starved | count | 0 | 8 | 0 |
| lamp_cache/store_used | count | 5 | 24 | 0 |
| lamp_cache/views | count | 2 | 32 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2621 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 209 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 764 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 209 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 764 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1295 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3326 | 5152 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1295 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3326 | 5152 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 21 | 107 | 4 |
| phases/shadow/bins | count | 1472 | 2615 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3508 | 5455 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 30 | 131 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2641 | 3268 | 2348 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 18 | 341 | 6 |
| preprocess/bind_groups_created | count | 18 | 341 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 298 | 127 |
| process/cpu_usage | % | 24.52 | 54.03 | 23.44 |
| process/mem_usage | GiB | 5.39 | 5.57 | 5.56 |
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
| skin/blocks_changed/count | count | 13 | 109 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 279.8 k | 809.7 k | 448.5 k |
| skin/joints_written/count | count | 4372 | 12.7 k | 7008 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4940 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 429 | 528 | 396 |
| statics/materials_bound | count | 429 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 323 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1127 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.40 | 49.77 | 32.88 |
| system/mem_usage | % | 53.94 | 54.52 | 54.49 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2910 | 3012 | 3012 |
| textures/images | count | 2691 | 2735 | 2735 |
| views/all/batch_sets | count | 71 | 154 | 56 |
| views/all/bins | count | 4921 | 5517 | 4989 |
| views/all/count | count | 5 | 41 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.1 k | 18.2 k | 10.6 k |
| views/all/pipelines | count | 77 | 178 | 67 |
| views/all/pipelines_distinct | count | 64 | 74 | 63 |
| views/all/work_items | count | 9811 | 18.1 k | 8146 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1444 | 1545 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3337 | 4418 | 3025 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 320 | 4418 | 222 |
| views/lamp_face/batch_sets | count | 4 | 83 | 0 |
| views/lamp_face/bins | count | 27 | 1214 | 0 |
| views/lamp_face/count | count | 2 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 1 | 26 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 47 | 1345 | 0 |
| views/lamp_face/pipelines | count | 7 | 106 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 26 | 1109 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2892 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7960 | 13.0 k | 7616 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8260 | 13.2 k | 7924 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.4 k | 37.9 k | 30.8 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 43.70 | 193 | 251.8 k |
| anim/events/count | count | 0.09 | 2 | 504 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.70 | 152 | 4061 |
| assets/Mesh/removed | count | 0.84 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.76 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.81 | 1298 | 4675 |
| encode//count | count | 52.38 | 131 | 301.9 k |
| encode/bind_walk/init_textures | count | 15.1 k | 48.0 k | 87.10 M |
| encode/bind_walk/init_textures_skipped | count | 63.8 k | 134.8 k | 367.85 M |
| encode/bind_walk/inits | count | 494 | 789 | 2.85 M |
| encode/bind_walk/inits_skipped | count | 108 | 166 | 620.9 k |
| encode/bind_walk/merge_textures | count | 15.1 k | 47.9 k | 86.74 M |
| encode/bind_walk/merge_textures_skipped | count | 5408 | 6012 | 31.16 M |
| encode/bind_walk/merges | count | 119 | 229 | 688.6 k |
| encode/bind_walk/merges_skipped | count | 84.20 | 128 | 485.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5763 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5763 |
| encode/pass:early prepass/count | count | 1 | 1 | 5763 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5763 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5763 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5763 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5763 |
| encode/pass:shadow_point_light/count | count | 6.35 | 26 | 36.6 k |
| encode/point_shadow_cache_copies/count | count | 0.49 | 1 | 2828 |
| encode/queue/command_buffers | count | 69.24 | 168 | 399.0 k |
| encode/queue/submits | count | 2.02 | 4 | 11.6 k |
| encode/queue/write_buffer/bytes | bytes | 1.82 M | 2.71 M | 10.52 G |
| encode/queue/write_buffer/calls | count | 272 | 531 | 1.57 M |
| encode/queue/write_buffer_with/bytes | bytes | 129.3 k | 3.50 M | 745.06 M |
| encode/queue/write_buffer_with/calls | count | 11.10 | 465 | 64.0 k |
| encode/queue/write_texture/bytes | bytes | 10.9 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5763 |
| physics/inserts | count | 0.39 | 111 | 2251 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 5874 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1288 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.9 ms; slowest 52, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.38 | 2382.1 | main/busy/ms 2355, stage/main/post_update 2315, stage/main/update 30.38, stage/main/pre_update 5.68, stage/main/last 1.05, surface/advance_cpu_ms 0.76, stage/main/first 0.49, stream/drive/ms 0.48, stage/main/fixed_main_loop 0.35, surface/cell_upload_ms 0.28, stream/tick/ms 0.12, bigspace/total/ms 0.07 |
| warm-up | 3.19 | 811.6 | main/busy/ms 2527, stage/main/post_update 2340, render/thread/ms 779, render/busy/ms 610, stage/render/prepare_assets 562, stage/main/update 172, stage/render/prepare_views 165, render/acquire/ms 165, stage/render/create_views 30.02, stage/render/render 12.73, render/graph/ms 8.01, stage/main/pre_update 7.32 |
| warm-up | 3.31 | 114.8 | render/thread/ms 877, render/busy/ms 648, stage/render/prepare_assets 588, main/busy/ms 191, stage/render/prepare_views 166, render/acquire/ms 166, stage/main/update 146, pipelines/compile_ms 83.75, stage/render/render 77.37, render/present/ms 63.36, pipelines/driver_ms 51.74, stage/main/post_update 34.40 |
| warm-up | 3.34 | 29.0 | pipelines/compile_ms 371, pipelines/front_ms 249, render/thread/ms 125, pipelines/driver_ms 121, stage/render/render 85.59, render/present/ms 70.52, main/busy/ms 60.57, render/busy/ms 53.78, pipelines/wait_ms 52.73, stage/main/post_update 45.32, stage/render/prepare_assets 27.10, terrain/update/ms 25.28 |
| warm-up | 3.42 | 84.7 | pipelines/wait_ms 2302, pipelines/compile_ms 945, pipelines/front_ms 795, pipelines/driver_ms 150, render/thread/ms 94.60, render/busy/ms 81.84, stage/render/render 63.17, main/busy/ms 62.07, encode//sum 50.62, stage/main/post_update 50.61, render/graph/ms 49.90, stage/render/graph_submit 44.13 |
| warm-up | 3.53 | 112.4 | pipelines/wait_ms 4761, pipelines/compile_ms 1729, pipelines/front_ms 1583, render/thread/ms 179, render/busy/ms 176, pipelines/driver_ms 146, main/busy/ms 90.49, encode//sum 75.73, stage/render/render 62.04, render/graph/ms 58.60, stage/render/graph_submit 56.09, stage/main/update 53.96 |
| warm-up | 3.60 | 68.0 | pipelines/wait_ms 2570, pipelines/compile_ms 1845, pipelines/front_ms 1729, render/thread/ms 176, render/busy/ms 173, pipelines/driver_ms 116, main/busy/ms 92.73, stage/main/update 60.59, stage/render/prepare_assets 59.40, stage/render/prepare_resources 50.32, stage/render/render 34.87, render/graph/ms 31.92 |
| warm-up | 3.64 | 39.2 | pipelines/compile_ms 1657, pipelines/front_ms 1574, pipelines/wait_ms 123, render/thread/ms 101, render/busy/ms 99.41, pipelines/driver_ms 82.55, stage/render/prepare_assets 74.57, main/busy/ms 39.22, stage/main/post_update 19.21, stage/render/render 18.60, render/graph/ms 16.40, stage/main/update 13.25 |
| warm-up | 3.70 | 34.0 | pipelines/compile_ms 500, pipelines/front_ms 478, render/thread/ms 53.86, render/busy/ms 52.09, main/busy/ms 40.84, pipelines/driver_ms 21.98, stage/render/prepare_write_phase_buffers 20.30, stage/main/post_update 18.89, stage/render/prepare_assets 13.53, encode//sum 12.36, stage/main/update 12.31, stage/render/render 11.38 |
| warm-up | 3.83 | 80.8 | pipelines/compile_ms 1302, pipelines/front_ms 1260, main/busy/ms 56.63, render/thread/ms 53.40, render/busy/ms 52.11, stage/main/update 45.38, pipelines/driver_ms 41.41, stream/drive/ms 40.63, stage/render/prepare_assets 32.11, stage/render/render 9.59, render/graph/ms 7.92, stage/main/post_update 7.14 |
| warm-up | 3.87 | 30.9 | pipelines/compile_ms 237, pipelines/front_ms 224, render/thread/ms 29.71, render/busy/ms 28.34, main/busy/ms 25.38, stage/render/prepare_assets 15.57, stage/main/update 14.11, pipelines/driver_ms 12.74, stream/drive/ms 10.43, stage/render/render 9.26, render/graph/ms 7.64, stage/main/post_update 5.81 |
| warm-up | 3.97 | 71.9 | pipelines/compile_ms 304, pipelines/front_ms 291, render/thread/ms 65.21, render/busy/ms 63.62, main/busy/ms 53.99, stage/render/prepare_assets 37.89, stage/main/update 36.32, stream/drive/ms 31.45, stage/render/render 18.85, render/graph/ms 16.33, stage/render/graph_submit 13.51, pipelines/driver_ms 12.75 |
| warm-up | 4.00 | 30.4 | pipelines/compile_ms 284, pipelines/front_ms 271, render/thread/ms 68.07, render/busy/ms 64.08, stage/render/prepare_assets 36.70, main/busy/ms 29.65, stage/render/render 17.70, pipelines/driver_ms 13.04, render/graph/ms 12.56, stage/main/update 10.19, stage/main/post_update 9.93, stage/render/graph_submit 9.07 |
| warm-up | 4.09 | 83.6 | pipelines/compile_ms 734, pipelines/front_ms 712, render/thread/ms 73.67, render/busy/ms 69.78, main/busy/ms 55.22, stage/render/prepare_assets 48.61, stage/main/update 33.48, pipelines/driver_ms 22.61, stage/main/post_update 14.23, stage/render/render 13.52, render/graph/ms 8.91, stage/render/graph_submit 6.01 |
| warm-up | 4.17 | 83.1 | pipelines/compile_ms 1856, pipelines/front_ms 1793, render/thread/ms 118, render/busy/ms 115, stage/render/prepare_assets 84.77, main/busy/ms 64.30, pipelines/driver_ms 63.18, stage/main/update 40.38, stage/render/render 24.20, render/graph/ms 21.11, stage/render/graph_submit 18.98, encode/submit 17.23 |
| warm-up | 4.22 | 48.9 | pipelines/compile_ms 1710, pipelines/front_ms 1649, render/thread/ms 109, render/busy/ms 107, pipelines/wait_ms 90.46, pipelines/driver_ms 61.18, stage/render/prepare_assets 53.43, main/busy/ms 47.71, stage/render/render 30.34, render/graph/ms 27.26, stage/render/graph_submit 24.21, stage/main/update 18.54 |
| warm-up | 4.30 | 80.0 | pipelines/compile_ms 778, pipelines/front_ms 754, pipelines/wait_ms 143, render/thread/ms 105, render/busy/ms 104, stage/render/prepare_assets 60.30, main/busy/ms 44.75, pipelines/driver_ms 23.94, stage/main/post_update 19.41, stage/main/update 16.57, stage/render/render 15.33, render/graph/ms 13.23 |
| warm-up | 4.38 | 85.5 | pipelines/compile_ms 1823, pipelines/front_ms 1771, pipelines/wait_ms 420, render/thread/ms 146, render/busy/ms 145, main/busy/ms 100, stage/render/prepare_assets 71.94, stage/main/post_update 61.09, pipelines/driver_ms 52.33, stage/main/update 31.25, stage/render/prepare_resources 26.48, stage/render/render 12.53 |
| warm-up | 4.43 | 51.4 | pipelines/compile_ms 2585, pipelines/front_ms 2511, pipelines/wait_ms 1086, render/thread/ms 118, render/busy/ms 116, main/busy/ms 106, pipelines/driver_ms 73.68, stage/main/post_update 67.10, stage/render/prepare_assets 43.92, stage/render/prepare_resources 27.60, stage/main/update 27.01, stage/render/render 12.11 |
| warm-up | 4.57 | 130.8 | pipelines/compile_ms 2195, pipelines/front_ms 2024, pipelines/wait_ms 1166, pipelines/driver_ms 171, render/thread/ms 145, render/busy/ms 143, main/busy/ms 114, stage/main/update 65.39, stage/render/create_views 56.00, stream/drive/ms 54.58, stage/render/prepare_assets 53.02, stage/main/post_update 22.10 |
| warm-up | 4.60 | 39.2 | pipelines/compile_ms 2622, pipelines/front_ms 2415, pipelines/wait_ms 1348, pipelines/driver_ms 207, render/thread/ms 140, render/busy/ms 136, main/busy/ms 118, stage/main/update 76.04, stream/drive/ms 56.13, stage/render/create_views 55.57, stage/render/prepare_assets 42.14, stage/render/render 22.56 |
| warm-up | 4.65 | 42.5 | pipelines/compile_ms 1665, pipelines/front_ms 1595, pipelines/wait_ms 945, main/busy/ms 127, stage/main/update 100, pipelines/driver_ms 69.67, render/thread/ms 62.41, render/busy/ms 58.31, stream/drive/ms 50.61, stage/render/prepare_assets 23.00, stage/render/render 14.90, stage/main/post_update 14.44 |
| warm-up | 4.78 | 132.8 | pipelines/compile_ms 1142, pipelines/front_ms 1110, pipelines/wait_ms 265, main/busy/ms 117, stage/main/update 93.92, render/thread/ms 78.56, render/busy/ms 76.83, stream/drive/ms 49.59, stage/render/prepare_assets 44.35, pipelines/driver_ms 31.47, stage/render/render 15.02, stage/main/post_update 13.63 |
| warm-up | 4.81 | 28.1 | pipelines/compile_ms 1164, pipelines/front_ms 1132, pipelines/wait_ms 225, main/busy/ms 89.45, render/thread/ms 72.66, stage/main/update 70.95, render/busy/ms 70.80, stream/drive/ms 48.00, encode//sum 35.71, stage/render/prepare_assets 35.47, pipelines/driver_ms 32.37, stage/render/render 21.59 |
| warm-up | 4.89 | 85.5 | pipelines/compile_ms 1048, pipelines/front_ms 1018, main/busy/ms 86.22, stage/main/update 71.37, render/thread/ms 54.10, render/busy/ms 52.35, stream/drive/ms 47.48, encode//sum 36.23, pipelines/driver_ms 29.42, stage/render/render 21.18, stage/render/prepare_assets 19.67, render/graph/ms 18.98 |
| warm-up | 4.95 | 36.5 | pipelines/compile_ms 575, pipelines/front_ms 559, main/busy/ms 49.37, render/thread/ms 35.33, stage/main/update 34.23, render/busy/ms 33.54, stream/drive/ms 27.92, pipelines/driver_ms 15.82, stage/render/render 14.86, render/graph/ms 12.68, stage/main/post_update 9.76, encode//sum 9.66 |
| warm-up | 4.98 | 28.2 | pipelines/compile_ms 539, pipelines/front_ms 525, render/thread/ms 41.58, render/busy/ms 39.65, main/busy/ms 27.36, stage/render/render 17.49, render/graph/ms 15.03, pipelines/driver_ms 14.15, encode//sum 13.91, stage/main/post_update 12.31, stage/main/update 10.11, stage/render/graph_submit 9.49 |
| warm-up | 5.03 | 28.6 | pipelines/compile_ms 169, pipelines/front_ms 162, render/thread/ms 40.03, render/busy/ms 38.76, main/busy/ms 26.86, stage/render/render 14.27, render/graph/ms 12.54, stage/render/prepare_assets 12.46, stage/main/update 11.35, stage/main/post_update 10.14, encode//sum 9.96, stage/render/graph_submit 8.41 |
| warm-up | 5.08 | 29.5 | pipelines/compile_ms 358, pipelines/front_ms 348, render/thread/ms 50.87, render/busy/ms 48.77, main/busy/ms 39.24, stage/render/render 25.68, render/graph/ms 22.77, stage/main/update 19.14, encode//sum 18.23, stage/render/graph_submit 16.43, stage/main/post_update 13.77, encode/submit 12.72 |
| warm-up | 5.13 | 45.1 | pipelines/compile_ms 658, pipelines/front_ms 637, render/thread/ms 61.62, render/busy/ms 59.67, main/busy/ms 39.49, stage/render/prepare_assets 25.12, stage/render/render 23.67, render/graph/ms 21.08, pipelines/driver_ms 20.85, stage/main/update 20.18, stage/render/graph_submit 15.17, stage/main/post_update 13.19 |
| warm-up | 5.19 | 60.3 | pipelines/compile_ms 740, pipelines/front_ms 713, render/thread/ms 87.14, render/busy/ms 85.46, stage/render/prepare_assets 57.62, main/busy/ms 51.39, stage/main/update 32.73, pipelines/driver_ms 26.39, stage/render/render 17.45, stream/drive/ms 16.60, render/graph/ms 15.18, stage/main/post_update 11.18 |
| warm-up | 5.26 | 66.5 | pipelines/compile_ms 930, pipelines/front_ms 902, render/thread/ms 111, render/busy/ms 109, stage/render/prepare_assets 76.78, main/busy/ms 69.38, stage/main/post_update 33.08, stage/main/update 29.14, pipelines/driver_ms 27.93, stage/render/render 19.34, render/graph/ms 16.83, stream/drive/ms 15.39 |
| warm-up | 5.28 | 27.1 | pipelines/compile_ms 754, pipelines/front_ms 736, render/thread/ms 84.24, render/busy/ms 82.51, main/busy/ms 49.96, stage/render/prepare_assets 42.50, stage/main/post_update 36.97, encode//sum 27.64, stage/render/render 26.58, render/graph/ms 23.98, pipelines/driver_ms 18.59, stage/render/graph_submit 16.57 |
| warm-up | 5.31 | 27.2 | pipelines/compile_ms 444, pipelines/front_ms 431, render/thread/ms 51.45, render/busy/ms 49.97, main/busy/ms 30.97, encode//sum 27.36, stage/render/render 21.94, render/graph/ms 19.71, stage/main/post_update 18.33, stage/render/prepare_assets 15.99, pipelines/driver_ms 13.33, stage/render/graph_submit 12.90 |
| warm-up | 5.56 | 27.2 | pipelines/compile_ms 281, pipelines/front_ms 274, render/thread/ms 37.79, render/busy/ms 36.71, main/busy/ms 30.31, stage/main/update 14.23, stage/render/render 14.20, stage/render/prepare_assets 12.91, render/graph/ms 12.68, stage/main/post_update 11.45, encode//sum 9.82, stage/render/graph_submit 8.30 |
| warm-up | 5.62 | 29.7 | main/busy/ms 46.36, render/thread/ms 42.25, render/busy/ms 39.61, stage/main/update 31.94, pipelines/compile_ms 25.37, pipelines/front_ms 23.27, stage/render/render 22.68, render/graph/ms 19.47, stage/render/graph_submit 14.73, stream/drive/ms 13.76, stage/main/post_update 10.87, encode//sum 10.39 |
| warm-up | 5.65 | 28.5 | pipelines/compile_ms 67.76, pipelines/front_ms 62.77, main/busy/ms 45.32, render/thread/ms 44.74, render/busy/ms 42.09, stage/main/update 30.14, stage/render/render 21.63, render/graph/ms 18.35, stream/drive/ms 14.12, stage/render/graph_submit 14.02, stage/render/prepare_assets 12.62, encode//sum 10.59 |
| warm-up | 5.78 | 27.8 | pipelines/compile_ms 511, pipelines/front_ms 498, render/thread/ms 37.57, render/busy/ms 36.38, main/busy/ms 22.79, stage/render/render 14.64, stage/render/prepare_assets 14.37, pipelines/driver_ms 12.98, render/graph/ms 12.83, encode//sum 11.57, stage/main/post_update 10.50, encode/pass:shadow_point_light/sum 8.65 |
| warm-up | 5.96 | 27.5 | main/busy/ms 39.21, render/thread/ms 33.66, render/busy/ms 31.89, stage/main/update 24.79, stream/drive/ms 17.30, stage/render/render 16.95, render/graph/ms 14.51, encode//sum 10.72, stage/main/post_update 10.29, stage/render/graph_submit 10.14, encode/pass:shadow_point_light/sum 7.43, encode/submit 6.88 |
| warm-up | 6.04 | 31.0 | pipelines/compile_ms 177, pipelines/front_ms 173, render/thread/ms 43.48, render/busy/ms 41.73, main/busy/ms 32.55, stage/render/render 20.89, render/graph/ms 18.56, stage/main/post_update 14.92, encode//sum 14.71, stage/main/update 13.07, stage/render/graph_submit 12.74, encode/submit 9.11 |
| warm-up | 6.09 | 30.9 | pipelines/compile_ms 326, pipelines/front_ms 316, render/thread/ms 34.39, render/busy/ms 33.27, main/busy/ms 23.40, stage/render/prepare_assets 12.95, stage/render/render 11.69, stage/main/post_update 10.34, render/graph/ms 10.20, pipelines/driver_ms 9.90, encode//sum 9.38, stage/main/update 7.99 |
| warm-up | 6.33 | 29.3 | render/thread/ms 44.47, render/busy/ms 43.10, main/busy/ms 40.47, stage/render/render 18.33, stage/main/update 17.56, stage/main/post_update 16.81, render/graph/ms 16.59, encode//sum 11.69, stage/render/graph_submit 10.85, stage/render/prepare_assets 9.66, encode/pass:shadow_point_light/sum 9.29, encode/submit 7.33 |
| warm-up | 6.41 | 28.4 | render/thread/ms 45.98, render/busy/ms 44.45, main/busy/ms 35.40, stage/render/render 20.39, render/graph/ms 18.63, stage/main/update 15.21, stage/main/post_update 14.47, stage/render/graph_submit 13.95, stage/render/prepare_assets 12.47, encode//sum 11.10, encode/submit 10.58, encode/pass:shadow_point_light/sum 7.19 |
| warm-up | 6.46 | 33.3 | render/thread/ms 39.13, render/busy/ms 38.11, main/busy/ms 33.45, stage/main/update 16.11, stage/render/prepare_assets 15.97, stage/render/render 13.94, encode//sum 13.54, render/graph/ms 12.25, encode/pass:shadow_point_light/sum 11.10, stage/main/post_update 10.72, stage/render/graph_submit 7.36, stage/render/camera_driver 4.77 |
| warm-up | 6.55 | 27.3 | render/thread/ms 49.15, render/busy/ms 48.11, stage/render/prepare_assets 27.77, main/busy/ms 23.20, pipelines/compile_ms 13.04, stage/render/render 12.65, pipelines/front_ms 12.25, encode//sum 11.27, encode/pass:shadow_point_light/sum 11.06, render/graph/ms 11.01, stage/main/post_update 10.51, stage/main/update 8.83 |
| warm-up | 6.64 | 49.8 | render/thread/ms 49.03, render/busy/ms 47.88, main/busy/ms 40.39, stage/main/update 26.24, stage/render/prepare_assets 25.76, stream/drive/ms 17.56, stage/render/render 14.52, render/graph/ms 12.88, encode//sum 11.56, stage/main/post_update 9.79, stage/render/graph_submit 9.56, encode/pass:shadow_point_light/sum 7.58 |
| warm-up | 6.89 | 29.0 | pipelines/compile_ms 143, pipelines/front_ms 139, main/busy/ms 38.75, render/thread/ms 27.68, render/busy/ms 26.61, stage/main/update 22.46, stage/render/render 13.83, stream/drive/ms 12.90, render/graph/ms 12.32, stage/main/post_update 10.86, encode//sum 10.08, encode/pass:shadow_point_light/sum 9.63 |
| measure | 0.29 | 35.1 | render/thread/ms 46.19, render/busy/ms 44.74, stage/render/render 38.02, render/graph/ms 36.13, stage/render/graph_submit 31.34, encode/submit 27.94, main/busy/ms 17.88, encode//sum 12.08, stage/main/post_update 11.34, encode/pass:shadow_point_light/sum 5.80, stage/render/camera_driver 4.63, stage/main/update 4.24 |
| measure | 0.78 | 27.1 | main/busy/ms 32.63, render/thread/ms 31.60, render/busy/ms 30.19, stage/main/update 18.61, stage/render/render 17.55, render/graph/ms 15.72, encode//sum 15.22, stream/drive/ms 13.94, stage/render/graph_submit 10.79, stage/main/post_update 10.64, encode/submit 7.39, encode/pass:shadow_point_light/sum 7.09 |
| measure | 18.19 | 29.8 | main/busy/ms 36.93, render/thread/ms 27.19, render/busy/ms 26.20, stage/main/update 23.72, stream/drive/ms 20.04, encode//sum 18.43, stage/render/render 18.41, render/graph/ms 17.02, stage/render/graph_submit 12.59, stage/main/post_update 10.92, encode/submit 8.44, encode/pass:shadow_point_light/sum 6.02 |
| measure | 23.07 | 27.1 | encode//sum 42.96, render/thread/ms 38.06, render/busy/ms 36.94, main/busy/ms 29.62, stage/render/render 18.68, render/graph/ms 17.09, stage/main/update 15.51, stage/render/graph_submit 12.43, stage/main/post_update 10.37, stage/render/prepare_assets 8.53, encode/finish_wall 7.07, encode/pass:shadow_point_light/sum 6.12 |
| measure | 59.04 | 28.2 | render/thread/ms 36.69, render/busy/ms 35.50, main/busy/ms 26.88, stage/render/prepare_assets 14.38, stage/main/post_update 14.29, encode//sum 12.78, stage/render/render 12.20, render/graph/ms 10.61, stage/main/update 7.57, stage/render/graph_submit 7.30, encode/submit 4.04, encode/finish_wall 3.31 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=lampcache-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 0%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **71%** (min 43%) · 2625 MHz (min 2625) · memory 8101 MHz · 73.9 W · 75 °C (max 82) · VRAM 7012 MiB · throttled 100% of samples

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
