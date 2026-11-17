# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5533 frames over 65.6 s · **median 10.83 ms (92 fps)** · mean 11.86 · p95 16.71 · p99 19.20 (1% low 52 fps) · worst 42.73 ms · 311 spikes over 1.5× median · 14.6 stutters/min (work 14.6/min)
- warm-up: 324 frames over 9.3 s, worst 2516.3 ms
- hitches (> 27.1 ms): **3** measured, 49 in warm-up
- timed GPU passes: 6.96 ms of a 11.86 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.83 ms, limit 11 ms
- **FAIL** frame_ms: 16 stutters (> 1.5x moving median of 60) in 1.09 min = 14.62/min, limit 1/min
- **FAIL** worst frame 42.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.006 | 2.031 | 2.064 | 0.99 | 0.176 |
| main_opaque_pass_3d | 1.294 | 1.311 | 1.652 | 0.99 | 0.069 |
| auxiliary_views | 0.520 | 0.527 | 1.488 | 0.99 | 0.480 |
| early prepass | 0.500 | 0.506 | 0.722 | 0.99 | 0.086 |
| bloom | 0.311 | 0.315 | 0.321 | 0.99 | 0.063 |
| atmosphere_luts | 0.306 | 0.310 | 0.373 | 0.99 | 0.003 |
| water_velocity | 0.256 | 0.260 | 0.349 | 0.99 | 0.002 |
| water_sspr | 0.199 | 0.201 | 0.203 | 0.99 | 0.002 |
| ssao | 0.186 | 0.188 | 0.193 | 0.99 | 0.007 |
| water_fft | 0.127 | 0.129 | 0.137 | 0.99 | 0.003 |
| upscaling | 0.127 | 0.128 | 0.136 | 0.99 | 0.004 |
| atmos_froxel_inject | 0.126 | 0.127 | 0.128 | 0.99 | 0.002 |
| uniform_allocation | 0.125 | 0.127 | 0.155 | 0.99 | 0.025 |
| clouds_composite | 0.103 | 0.105 | 0.108 | 0.99 | 0.002 |
| bin_unpacking | 0.097 | 0.098 | 0.120 | 0.99 | 0.018 |
| tonemapping | 0.086 | 0.087 | 0.092 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.078 | 0.154 | 0.99 | 0.008 |
| clustering | 0.063 | 0.063 | 0.067 | 0.99 | 0.019 |
| clouds_trace | 0.056 | 0.056 | 0.074 | 0.99 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.99 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.048 | 0.414 | 0.416 | 0.12 | 0.001 |
| surface_trail | 0.042 | 0.056 | 0.057 | 0.74 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.025 | 0.220 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.017 | 0.038 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.023 | 0.99 | 0.124 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.647 | 15.226 | 41.69 | 58907.2 |
| render/busy/ms | 10.118 | 14.675 | 41.11 | 55982.0 |
| encode//sum | 9.341 | 21.205 | 41.65 | 51683.7 |
| main/busy/ms | 9.242 | 11.962 | 27.65 | 51135.6 |
| stage/render/render | 6.882 | 10.486 | 37.83 | 38079.0 |
| render/graph/ms | 6.204 | 9.725 | 37.15 | 34324.5 |
| stage/main/post_update | 5.863 | 8.059 | 14.10 | 32437.7 |
| stage/render/graph_submit | 4.681 | 7.378 | 35.06 | 25898.2 |
| encode/submit | 2.811 | 4.341 | 5.50 | 15550.7 |
| stage/main/update | 2.141 | 2.604 | 21.82 | 11848.8 |
| encode/finish_wall | 1.912 | 3.365 | 30.16 | 10579.2 |
| encode/pass:shadow_point_light/sum | 1.580 | 4.699 | 7.30 | 8739.4 |
| stage/render/camera_driver | 1.467 | 2.469 | 8.46 | 8117.6 |
| encode/pass:shadow_directional_light/sum | 0.784 | 1.312 | 2.16 | 4340.2 |
| stage/render/prepare_write_phase_buffers | 0.757 | 1.116 | 2.19 | 4188.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.708 | 1.066 | 1.78 | 3917.5 |
| encode/pass:early prepass/sum | 0.695 | 1.078 | 28.79 | 3845.6 |
| stage/render/prepare_assets | 0.610 | 0.757 | 8.69 | 3373.3 |
| bigspace/total/ms | 0.572 | 1.415 | 3.70 | 3166.5 |
| stage/render/prepare_resources | 0.455 | 0.552 | 1.43 | 2519.1 |
| render/present/ms | 0.436 | 0.543 | 1.91 | 2411.3 |
| stage/render/prepare_meshes | 0.414 | 0.535 | 2.33 | 2292.6 |
| stage/main/pre_update | 0.360 | 0.488 | 1.36 | 1993.4 |
| stage/main/first | 0.350 | 0.454 | 6.43 | 1939.1 |
| terrain/update/ms | 0.332 | 0.494 | 2.57 | 1835.9 |
| terrain/select/ms | 0.297 | 0.447 | 1.14 | 1645.8 |
| stage/render/prepare_bind_groups | 0.280 | 0.496 | 1.96 | 1551.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.272 | 0.428 | 0.98 | 1504.4 |
| controller/feed/ms | 0.260 | 0.323 | 0.83 | 1440.2 |
| stream/drive/ms | 0.251 | 0.336 | 19.67 | 1386.0 |
| bigspace/low_precision/ms | 0.234 | 0.839 | 3.18 | 1296.1 |
| stage/main/fixed_main_loop | 0.230 | 0.369 | 1.36 | 1274.2 |
| stage/render/specialize | 0.211 | 0.342 | 1.96 | 1164.9 |
| far/select/ms | 0.202 | 0.269 | 0.78 | 1118.7 |
| stage/render/queue | 0.196 | 0.336 | 1.61 | 1083.3 |
| stage/render/create_views | 0.194 | 0.261 | 0.87 | 1072.7 |
| stream/tick/ms | 0.192 | 0.258 | 0.56 | 1060.2 |
| stage/render/prepare_batch_phases | 0.190 | 0.268 | 0.58 | 1049.2 |
| stage/render/extract_commands | 0.178 | 0.210 | 0.52 | 983.2 |
| bigspace/high_precision/ms | 0.172 | 0.861 | 2.91 | 954.3 |
| stage/render/prepare_flush | 0.170 | 0.337 | 0.94 | 943.3 |
| anim/evaluate/ms | 0.168 | 0.266 | 2.56 | 931.4 |
| anim/evaluate_cpu/ms | 0.156 | 0.199 | 0.66 | 861.8 |
| pipelines/compile_ms | 0.141 | 0.000 | 132.95 | 777.9 |
| pipelines/front_ms | 0.137 | 0.000 | 130.11 | 757.9 |
| stage/render/prepare_views | 0.136 | 0.168 | 0.38 | 755.1 |
| controller/move/ms | 0.133 | 0.284 | 0.67 | 735.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.128 | 0.230 | 0.69 | 709.7 |
| stage/main/last | 0.121 | 0.170 | 3.03 | 670.2 |
| render/acquire/ms | 0.093 | 0.118 | 0.33 | 513.9 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.122 | 0.52 | 315.9 |
| physics/step/ms | 0.049 | 0.084 | 0.54 | 271.0 |
| encode/point_shadow_cache_copies/sum | 0.047 | 0.172 | 0.57 | 260.9 |
| stage/render/cleanup | 0.044 | 0.137 | 0.91 | 241.1 |
| physics/insert/ms | 0.042 | 0.411 | 1.50 | 234.1 |
| stage/render/phase_sort | 0.032 | 0.055 | 0.26 | 179.0 |
| encode/resolve_diagnostics/sum | 0.028 | 0.038 | 0.67 | 156.2 |
| weather/map/ms | 0.016 | 0.021 | 0.20 | 90.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.45 | 74.3 |
| anim/apply/ms | 0.011 | 0.021 | 0.60 | 63.1 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.49 | 57.7 |
| controller/avoid/ms | 0.010 | 0.013 | 0.25 | 57.0 |
| weather/eval/ms | 0.010 | 0.013 | 0.20 | 54.0 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 39.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.11 | 31.2 |
| script/input/ms | 0.005 | 0.007 | 0.18 | 29.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.19 | 28.9 |
| script/ui/ms | 0.004 | 0.005 | 0.45 | 22.8 |
| pipelines/driver_ms | 0.004 | 0.000 | 3.87 | 20.0 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 19.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 14.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.69 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.5 |
| stage/render/prepare | 0.000 | 0.002 | 0.02 | 2.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1236 | 2262 | 535 |
| crowd/c0/meshes/count | count | 346 | 646 | 144 |
| crowd/c1/count | count | 148 | 154 | 149 |
| crowd/c1/joints/count | count | 5629 | 5894 | 5661 |
| crowd/c1/meshes/count | count | 1180 | 1254 | 1197 |
| crowd/c2/count | count | 400 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1824 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 165 | 216 | 205 |
| crowd/joints_extracted/count | count | 3717 | 8334 | 5763 |
| crowd/joints_uploaded/count | count | 3671 | 6206 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.53 | 29.10 | 0.54 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.45 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.49 | 0.01 |
| encode/finish_critical | ms | 0.96 | 29.10 | 1.36 |
| encode/pass:early prepass/max | ms | 0.69 | 28.79 | 1.00 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.72 | 1.78 | 1.10 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.69 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.98 | 0.28 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 2.16 | 1.36 |
| encode/pass:shadow_point_light/max | ms | 0.76 | 1.58 | 0.68 |
| encode/point_shadow_cache_copies/max | ms | 0.08 | 0.57 | 0.13 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.67 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.13 | 41.11 | 8.84 |
| gpu/timed/ms | ms | 7.60 | 9.22 | 7.68 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 9 | 28 | 0 |
| lamp_cache/copies | count | 0 | 16 | 0 |
| lamp_cache/deferred_faces | count | 0 | 15 | 0 |
| lamp_cache/lamps | count | 5 | 8 | 0 |
| lamp_cache/mover_faces | count | 0 | 12 | 0 |
| lamp_cache/movers | count | 100 | 271 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 5 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 16 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 7 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 12 | 33 | 0 |
| lamp_cache/stock_lamps | count | 0 | 3 | 0 |
| lamp_cache/store_starved | count | 0 | 8 | 0 |
| lamp_cache/store_used | count | 3 | 24 | 0 |
| lamp_cache/views | count | 1 | 33 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2621 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 228 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 758 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 228 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 758 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1298 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3315 | 5146 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1298 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3315 | 5146 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 90 | 19 |
| phases/shadow/bins | count | 1477 | 3287 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3404 | 6181 | 3033 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 26 | 145 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2641 | 3268 | 2347 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 22 | 303 | 37 |
| preprocess/bind_groups_created | count | 22 | 303 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 272 | 127 |
| process/cpu_usage | % | 24.06 | 35.10 | 32.21 |
| process/mem_usage | GiB | 5.38 | 5.56 | 5.55 |
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
| skin/blocks_changed/count | count | 12 | 111 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 280.4 k | 810.6 k | 444.2 k |
| skin/joints_written/count | count | 4381 | 12.7 k | 6940 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 323 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1126 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.16 | 37.05 | 32.37 |
| system/mem_usage | % | 54.46 | 55.06 | 55.01 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 72 | 141 | 71 |
| views/all/bins | count | 4912 | 6079 | 4989 |
| views/all/count | count | 4 | 41 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.0 k | 19.0 k | 10.6 k |
| views/all/pipelines | count | 74 | 194 | 67 |
| views/all/pipelines_distinct | count | 63 | 75 | 63 |
| views/all/work_items | count | 10.1 k | 18.5 k | 11.0 k |
| views/cascade/batch_sets | count | 8 | 27 | 19 |
| views/cascade/bins | count | 1438 | 1560 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3297 | 4448 | 3033 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 315 | 4448 | 3033 |
| views/lamp_face/batch_sets | count | 0 | 66 | 0 |
| views/lamp_face/bins | count | 20 | 1776 | 0 |
| views/lamp_face/count | count | 1 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 25 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 23 | 2213 | 0 |
| views/lamp_face/pipelines | count | 3 | 121 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 1225 | 0 |
| views/main/batch_sets | count | 51 | 55 | 52 |
| views/main/bins | count | 2902 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7940 | 13.0 k | 7616 |
| views/main/pipelines | count | 48 | 51 | 48 |
| views/main/work_items | count | 8245 | 13.3 k | 7924 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.3 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 52.07 | 243 | 288.1 k |
| anim/events/count | count | 0.10 | 3 | 552 |
| assets/Image/added | count | 0.11 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.73 | 152 | 4061 |
| assets/Mesh/removed | count | 0.88 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.92 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.84 | 1304 | 4675 |
| encode//count | count | 53.14 | 127 | 294.0 k |
| encode/bind_walk/init_textures | count | 15.9 k | 44.7 k | 87.73 M |
| encode/bind_walk/init_textures_skipped | count | 59.5 k | 118.4 k | 329.44 M |
| encode/bind_walk/inits | count | 500 | 787 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 104 | 149 | 572.9 k |
| encode/bind_walk/merge_textures | count | 15.8 k | 44.6 k | 87.38 M |
| encode/bind_walk/merge_textures_skipped | count | 5421 | 6012 | 30.00 M |
| encode/bind_walk/merges | count | 121 | 218 | 667.0 k |
| encode/bind_walk/merges_skipped | count | 81.74 | 124 | 452.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5533 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5533 |
| encode/pass:early prepass/count | count | 1 | 1 | 5533 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5533 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5533 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5533 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5533 |
| encode/pass:shadow_point_light/count | count | 6.54 | 25 | 36.2 k |
| encode/point_shadow_cache_copies/count | count | 0.48 | 1 | 2645 |
| encode/queue/command_buffers | count | 70.17 | 163 | 388.3 k |
| encode/queue/submits | count | 2.02 | 4 | 11.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.82 M | 2.68 M | 10.06 G |
| encode/queue/write_buffer/calls | count | 276 | 491 | 1.53 M |
| encode/queue/write_buffer_with/bytes | bytes | 128.5 k | 3.50 M | 711.22 M |
| encode/queue/write_buffer_with/calls | count | 11.19 | 465 | 61.9 k |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5533 |
| physics/inserts | count | 0.41 | 111 | 2258 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 161 | 2335 |
| physics/steps | count | 0.71 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 7 |
| pipelines/created | count | 0.00 | 1 | 8 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 1 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6118 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1341 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 52, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2516.3 | main/busy/ms 2486, stage/main/post_update 2438, stage/main/update 39.52, stage/main/pre_update 5.31, stage/main/last 1.12, surface/advance_cpu_ms 0.73, stage/main/first 0.62, stage/main/fixed_main_loop 0.49, stream/drive/ms 0.46, surface/cell_upload_ms 0.33, stream/tick/ms 0.16, bigspace/total/ms 0.08 |
| warm-up | 3.29 | 768.9 | main/busy/ms 2631, stage/main/post_update 2462, render/thread/ms 736, render/busy/ms 608, stage/render/prepare_assets 560, stage/main/update 154, stage/render/prepare_views 125, render/acquire/ms 125, stage/render/create_views 30.29, stage/render/render 10.95, stage/main/pre_update 7.26, render/graph/ms 6.66 |
| warm-up | 3.39 | 109.5 | render/thread/ms 829, render/busy/ms 646, stage/render/prepare_assets 585, main/busy/ms 167, stage/render/prepare_views 126, render/acquire/ms 125, pipelines/compile_ms 121, stage/main/update 119, pipelines/front_ms 73.14, stage/render/render 71.19, render/present/ms 57.81, pipelines/driver_ms 47.62 |
| warm-up | 3.44 | 46.4 | pipelines/compile_ms 371, pipelines/front_ms 278, render/thread/ms 116, pipelines/driver_ms 93.24, stage/render/render 76.72, render/present/ms 62.75, main/busy/ms 62.58, render/busy/ms 53.03, stage/main/post_update 45.94, pipelines/wait_ms 33.31, stage/render/prepare_assets 26.42, terrain/update/ms 25.85 |
| warm-up | 3.57 | 125.4 | pipelines/wait_ms 3401, pipelines/compile_ms 1406, pipelines/front_ms 1280, main/busy/ms 148, render/thread/ms 144, render/busy/ms 131, pipelines/driver_ms 126, stage/main/post_update 66.76, stage/render/render 53.62, stage/main/update 53.58, render/graph/ms 38.25, stage/render/prepare_flush 33.89 |
| warm-up | 3.63 | 63.7 | pipelines/wait_ms 3622, pipelines/compile_ms 1854, pipelines/front_ms 1736, render/thread/ms 181, render/busy/ms 175, main/busy/ms 141, pipelines/driver_ms 118, stage/render/prepare_assets 63.41, stage/main/update 56.76, stage/main/post_update 44.40, stage/render/render 43.16, render/graph/ms 35.18 |
| warm-up | 3.66 | 30.8 | pipelines/compile_ms 1028, pipelines/front_ms 976, pipelines/wait_ms 254, render/thread/ms 88.83, render/busy/ms 87.18, stage/render/prepare_assets 67.68, pipelines/driver_ms 52.51, main/busy/ms 48.96, stage/main/post_update 17.35, terrain/update/ms 15.24, stage/main/update 12.94, stage/render/render 12.54 |
| warm-up | 3.69 | 30.6 | pipelines/compile_ms 417, pipelines/front_ms 397, render/thread/ms 56.70, render/busy/ms 55.13, stage/render/prepare_assets 36.76, main/busy/ms 35.31, pipelines/driver_ms 20.09, stage/main/post_update 15.14, stage/main/update 11.64, stage/render/render 10.90, terrain/update/ms 10.74, render/graph/ms 9.08 |
| warm-up | 3.72 | 28.9 | pipelines/compile_ms 98.55, pipelines/front_ms 91.40, render/thread/ms 55.34, render/busy/ms 53.36, main/busy/ms 32.91, stage/render/prepare_assets 28.39, encode//sum 22.02, stage/main/post_update 16.28, stage/render/render 13.07, render/graph/ms 11.04, terrain/update/ms 9.46, stage/main/update 8.97 |
| warm-up | 3.86 | 56.2 | pipelines/compile_ms 825, pipelines/front_ms 797, render/thread/ms 47.55, main/busy/ms 46.46, render/busy/ms 45.84, stage/main/update 35.13, stream/drive/ms 31.15, stage/render/prepare_assets 29.72, pipelines/driver_ms 28.73, stage/render/render 10.51, render/graph/ms 8.80, encode//sum 6.51 |
| warm-up | 3.95 | 82.6 | pipelines/compile_ms 394, pipelines/front_ms 377, render/thread/ms 54.55, render/busy/ms 52.71, main/busy/ms 48.17, stage/main/update 38.21, stream/drive/ms 34.00, stage/render/prepare_assets 24.28, stage/render/render 23.69, render/graph/ms 21.12, pipelines/driver_ms 16.48, stage/render/graph_submit 12.79 |
| warm-up | 4.04 | 72.1 | pipelines/compile_ms 307, pipelines/front_ms 293, render/thread/ms 60.50, render/busy/ms 56.35, stage/render/prepare_assets 35.06, main/busy/ms 30.62, stage/render/render 16.02, pipelines/driver_ms 13.24, stage/main/update 11.48, render/graph/ms 11.35, stage/main/post_update 9.90, stage/render/graph_submit 8.78 |
| warm-up | 4.09 | 41.5 | pipelines/compile_ms 460, pipelines/front_ms 441, render/thread/ms 77.02, render/busy/ms 72.83, stage/render/prepare_assets 47.75, main/busy/ms 47.12, stage/main/update 31.11, stage/render/render 19.98, pipelines/driver_ms 18.82, render/graph/ms 14.99, stage/main/post_update 8.84, stage/render/camera_driver 7.87 |
| warm-up | 4.12 | 33.8 | pipelines/compile_ms 324, pipelines/front_ms 311, render/thread/ms 55.05, render/busy/ms 53.32, main/busy/ms 52.60, stage/main/update 32.70, stage/render/prepare_assets 27.80, stage/render/render 17.38, render/graph/ms 14.87, pipelines/driver_ms 13.35, stage/main/post_update 12.43, stage/render/camera_driver 8.07 |
| warm-up | 4.15 | 36.1 | pipelines/compile_ms 703, pipelines/front_ms 679, render/thread/ms 51.21, render/busy/ms 49.63, main/busy/ms 29.17, pipelines/driver_ms 24.19, stage/render/prepare_assets 15.43, stage/render/prepare_resources 15.11, stage/main/post_update 13.05, stage/render/render 10.86, stage/main/update 10.39, render/graph/ms 8.75 |
| warm-up | 4.22 | 50.4 | pipelines/compile_ms 513, pipelines/front_ms 500, render/thread/ms 53.49, render/busy/ms 51.72, main/busy/ms 49.71, stage/main/post_update 26.08, stage/render/prepare_bind_groups 19.12, stage/main/update 17.43, stage/render/render 14.42, pipelines/driver_ms 13.29, stage/render/prepare_assets 12.13, render/graph/ms 12.06 |
| warm-up | 4.28 | 57.3 | pipelines/compile_ms 488, pipelines/front_ms 473, render/thread/ms 94.05, render/busy/ms 91.89, main/busy/ms 69.79, stage/main/post_update 35.30, stage/render/prepare_assets 27.03, stage/render/render 26.40, render/graph/ms 23.66, stage/main/update 23.38, stage/render/prepare_bind_groups 19.12, pipelines/driver_ms 14.45 |
| warm-up | 4.33 | 56.7 | pipelines/compile_ms 1344, pipelines/front_ms 1303, render/thread/ms 90.55, render/busy/ms 88.30, main/busy/ms 71.46, pipelines/driver_ms 40.59, stage/render/prepare_assets 33.87, stage/main/post_update 32.49, stage/render/render 28.95, stage/main/update 26.88, render/graph/ms 25.56, stage/render/camera_driver 15.97 |
| warm-up | 4.43 | 96.4 | pipelines/compile_ms 2140, pipelines/front_ms 2067, main/busy/ms 129, render/thread/ms 97.03, render/busy/ms 95.05, stage/main/update 88.51, pipelines/driver_ms 73.16, stream/drive/ms 70.45, pipelines/wait_ms 59.26, stage/render/prepare_assets 52.36, stage/main/post_update 24.22, stage/render/render 19.33 |
| warm-up | 4.54 | 106.6 | pipelines/compile_ms 2769, pipelines/front_ms 2681, pipelines/wait_ms 132, main/busy/ms 117, render/thread/ms 104, render/busy/ms 102, pipelines/driver_ms 88.35, stage/main/update 78.88, stream/drive/ms 68.91, stage/render/prepare_assets 68.54, stage/main/post_update 19.06, stage/render/render 12.41 |
| warm-up | 4.57 | 27.5 | pipelines/compile_ms 1964, pipelines/front_ms 1907, pipelines/wait_ms 149, main/busy/ms 116, stage/main/update 83.62, render/thread/ms 69.05, render/busy/ms 67.04, pipelines/driver_ms 57.17, stream/drive/ms 45.56, stage/render/prepare_assets 37.88, stage/main/post_update 20.28, stage/render/render 17.34 |
| warm-up | 4.69 | 126.1 | pipelines/compile_ms 1848, pipelines/front_ms 1790, pipelines/wait_ms 267, main/busy/ms 110, stage/main/update 90.58, render/thread/ms 69.01, render/busy/ms 67.19, pipelines/driver_ms 58.69, stream/drive/ms 50.91, stage/render/prepare_assets 29.65, stage/render/render 17.35, render/graph/ms 15.01 |
| warm-up | 4.73 | 41.3 | pipelines/compile_ms 1884, pipelines/front_ms 1822, pipelines/wait_ms 197, render/thread/ms 81.56, render/busy/ms 79.89, main/busy/ms 77.71, pipelines/driver_ms 62.31, stage/main/update 58.16, stream/drive/ms 43.39, stage/render/prepare_assets 30.35, stage/render/render 19.07, render/graph/ms 16.94 |
| warm-up | 4.82 | 82.1 | pipelines/compile_ms 542, pipelines/front_ms 522, main/busy/ms 112, render/thread/ms 79.65, render/busy/ms 77.68, stage/main/update 70.52, stream/drive/ms 47.01, stage/main/post_update 30.22, stage/render/render 23.87, render/graph/ms 21.33, pipelines/driver_ms 20.77, stage/render/prepare_assets 16.97 |
| warm-up | 4.86 | 42.3 | pipelines/compile_ms 264, pipelines/front_ms 253, main/busy/ms 109, render/thread/ms 69.67, stage/main/update 68.44, render/busy/ms 68.14, stream/drive/ms 45.86, stage/main/post_update 30.87, stage/render/prepare_assets 22.62, stage/render/render 15.82, render/graph/ms 13.69, stage/render/prepare_resources 11.66 |
| warm-up | 4.92 | 59.7 | pipelines/compile_ms 1314, pipelines/front_ms 1276, main/busy/ms 78.90, stage/main/update 60.14, render/thread/ms 56.64, render/busy/ms 54.98, pipelines/driver_ms 38.23, stream/drive/ms 37.08, stage/render/prepare_assets 18.38, stage/render/render 13.09, stage/main/post_update 11.50, render/graph/ms 10.98 |
| warm-up | 4.96 | 42.8 | pipelines/compile_ms 1351, pipelines/front_ms 1314, render/thread/ms 63.87, render/busy/ms 61.95, main/busy/ms 49.15, pipelines/driver_ms 37.29, stage/main/update 31.91, stage/render/prepare_assets 21.51, stage/render/render 16.18, render/graph/ms 13.62, encode//sum 12.05, stage/main/post_update 10.85 |
| warm-up | 5.00 | 42.9 | pipelines/compile_ms 685, pipelines/front_ms 663, render/thread/ms 69.81, render/busy/ms 67.88, main/busy/ms 43.62, encode//sum 34.80, stage/render/prepare_assets 25.52, stage/main/update 24.24, pipelines/driver_ms 21.85, stage/render/render 20.83, render/graph/ms 18.34, stage/render/graph_submit 13.16 |
| warm-up | 5.06 | 29.1 | pipelines/compile_ms 410, pipelines/front_ms 401, render/thread/ms 46.13, render/busy/ms 44.07, main/busy/ms 33.40, pipelines/wait_ms 30.58, stage/render/render 19.97, render/graph/ms 17.38, stage/main/update 15.94, encode//sum 14.30, stage/render/graph_submit 12.13, stage/main/post_update 11.49 |
| warm-up | 5.09 | 30.6 | pipelines/compile_ms 162, pipelines/front_ms 156, render/thread/ms 51.67, render/busy/ms 49.23, main/busy/ms 35.29, stage/render/render 23.94, render/graph/ms 20.87, encode//sum 18.72, stage/main/update 18.06, stage/render/graph_submit 14.31, stage/main/post_update 11.30, encode/submit 10.58 |
| warm-up | 5.13 | 48.0 | pipelines/compile_ms 665, pipelines/front_ms 643, render/thread/ms 63.75, render/busy/ms 61.47, main/busy/ms 39.86, stage/render/render 25.01, render/graph/ms 22.21, pipelines/driver_ms 21.51, stage/main/update 19.08, stage/render/prepare_assets 17.79, stage/render/graph_submit 16.87, encode//sum 15.34 |
| warm-up | 5.19 | 57.8 | pipelines/compile_ms 961, pipelines/front_ms 932, render/thread/ms 89.36, render/busy/ms 85.62, main/busy/ms 65.05, stage/main/update 29.32, pipelines/driver_ms 28.47, stage/render/render 24.37, stage/main/post_update 24.27, pipelines/wait_ms 21.73, stage/render/prepare_assets 20.76, render/graph/ms 20.11 |
| warm-up | 5.24 | 43.3 | pipelines/compile_ms 445, pipelines/front_ms 432, render/thread/ms 86.98, render/busy/ms 83.84, main/busy/ms 66.22, stage/main/post_update 31.20, stage/main/update 24.82, stage/render/prepare_assets 23.29, pipelines/wait_ms 20.87, stage/render/render 18.03, render/graph/ms 14.39, encode//sum 13.38 |
| warm-up | 5.27 | 31.6 | render/thread/ms 66.18, render/busy/ms 64.40, main/busy/ms 38.64, stage/main/post_update 23.16, stage/render/prepare_assets 22.63, stage/render/render 18.71, render/graph/ms 16.28, encode//sum 16.04, stage/render/graph_submit 11.47, stage/main/update 9.41, encode/submit 8.17, encode/pass:shadow_point_light/sum 7.03 |
| warm-up | 5.30 | 33.8 | pipelines/compile_ms 207, pipelines/front_ms 201, render/thread/ms 61.75, render/busy/ms 59.89, main/busy/ms 36.11, stage/render/render 20.25, stage/main/post_update 18.96, render/graph/ms 17.80, encode//sum 13.43, stage/render/graph_submit 12.77, stage/main/update 11.35, stage/render/prepare_assets 10.88 |
| warm-up | 5.39 | 30.1 | pipelines/compile_ms 572, pipelines/front_ms 560, render/thread/ms 44.03, render/busy/ms 42.89, pipelines/wait_ms 30.24, main/busy/ms 25.67, stage/render/prepare_assets 20.20, stage/render/render 14.32, render/graph/ms 12.83, pipelines/driver_ms 11.60, encode//sum 10.89, stage/main/post_update 10.54 |
| warm-up | 5.46 | 30.0 | render/thread/ms 40.92, render/busy/ms 39.45, main/busy/ms 31.70, stage/render/prepare_assets 18.67, stage/main/update 17.54, stage/render/render 14.45, render/graph/ms 12.26, stream/drive/ms 12.05, encode//sum 9.52, stage/main/post_update 8.99, stage/render/graph_submit 8.88, encode/submit 6.63 |
| warm-up | 5.52 | 33.4 | render/thread/ms 48.34, render/busy/ms 47.07, main/busy/ms 25.30, pipelines/compile_ms 24.45, pipelines/front_ms 22.94, stage/render/prepare_assets 21.97, stage/render/render 15.21, render/graph/ms 13.45, encode//sum 10.58, stage/main/post_update 10.43, stage/main/update 10.28, stage/render/graph_submit 9.35 |
| warm-up | 5.54 | 27.7 | pipelines/compile_ms 93.78, pipelines/front_ms 88.26, render/thread/ms 51.79, render/busy/ms 50.56, stage/render/prepare_assets 30.78, main/busy/ms 23.15, stage/render/render 11.91, render/graph/ms 10.06, encode//sum 9.95, stage/main/update 8.82, stage/main/post_update 8.72, stage/render/graph_submit 6.63 |
| warm-up | 5.57 | 31.2 | pipelines/compile_ms 244, pipelines/front_ms 230, render/thread/ms 51.06, render/busy/ms 49.79, pipelines/wait_ms 33.91, stage/render/prepare_assets 27.45, main/busy/ms 21.98, pipelines/driver_ms 14.06, stage/render/render 13.33, encode//sum 11.94, render/graph/ms 11.38, stage/main/post_update 8.49 |
| warm-up | 5.98 | 28.1 | main/busy/ms 30.90, render/thread/ms 26.01, render/busy/ms 24.99, stage/main/update 19.94, stream/drive/ms 13.72, stage/render/render 12.67, render/graph/ms 11.18, encode//sum 10.23, stage/render/graph_submit 7.36, encode/pass:shadow_point_light/sum 7.10, stage/main/post_update 6.48, stage/render/prepare_assets 5.73 |
| warm-up | 6.09 | 27.7 | pipelines/compile_ms 135, pipelines/front_ms 131, render/thread/ms 39.12, render/busy/ms 37.69, main/busy/ms 30.67, stage/render/render 19.33, render/graph/ms 17.29, stage/main/post_update 13.71, encode//sum 13.53, stage/render/graph_submit 12.66, stage/main/update 9.74, encode/submit 8.65 |
| warm-up | 6.12 | 29.5 | pipelines/compile_ms 145, pipelines/front_ms 140, render/thread/ms 41.36, render/busy/ms 40.43, main/busy/ms 25.07, stage/render/prepare_assets 16.39, stage/render/render 11.76, encode//sum 10.84, render/graph/ms 10.44, stage/main/update 9.55, stage/main/post_update 8.18, stage/render/graph_submit 7.32 |
| warm-up | 6.22 | 30.1 | render/thread/ms 41.71, render/busy/ms 40.57, main/busy/ms 32.43, stage/main/update 17.84, stage/render/render 16.53, render/graph/ms 14.76, stage/render/prepare_assets 12.67, encode//sum 12.31, stage/main/post_update 10.76, encode/pass:shadow_point_light/sum 10.49, stage/render/graph_submit 9.72, encode/submit 6.04 |
| warm-up | 6.35 | 28.7 | render/thread/ms 40.77, main/busy/ms 39.98, render/busy/ms 39.12, stage/render/render 19.89, stage/main/update 19.04, render/graph/ms 17.63, encode//sum 14.48, stage/main/post_update 14.16, stage/render/graph_submit 11.47, encode/pass:shadow_point_light/sum 10.28, encode/submit 7.25, stage/render/prepare_assets 7.11 |
| warm-up | 6.38 | 28.8 | render/thread/ms 44.89, render/busy/ms 43.47, main/busy/ms 38.08, stage/main/update 16.67, stage/render/prepare_assets 15.86, stage/render/render 14.93, stage/main/post_update 14.20, render/graph/ms 12.93, encode//sum 12.57, encode/pass:shadow_point_light/sum 9.84, stage/render/graph_submit 8.34, encode/submit 4.88 |
| warm-up | 6.45 | 34.0 | pipelines/compile_ms 141, pipelines/front_ms 138, main/busy/ms 37.25, render/thread/ms 33.13, render/busy/ms 31.32, stage/main/update 22.34, stage/render/render 15.63, stream/drive/ms 13.84, render/graph/ms 13.17, encode//sum 11.15, stage/main/post_update 10.17, stage/render/graph_submit 9.09 |
| warm-up | 6.51 | 30.5 | render/thread/ms 44.29, render/busy/ms 42.75, main/busy/ms 39.25, stage/main/update 23.17, stage/render/render 21.04, render/graph/ms 19.19, encode//sum 14.81, pipelines/compile_ms 14.80, stage/render/graph_submit 14.35, pipelines/front_ms 13.79, encode/pass:shadow_point_light/sum 12.38, stage/render/prepare_assets 11.82 |
| warm-up | 6.92 | 30.6 | render/thread/ms 36.70, render/busy/ms 35.65, main/busy/ms 34.05, stage/main/update 14.60, stage/render/render 14.53, stage/render/prepare_assets 13.60, render/graph/ms 12.91, encode//sum 12.13, stage/main/post_update 11.07, stage/render/graph_submit 8.90, encode/pass:shadow_point_light/sum 8.60, encode/submit 5.69 |
| measure | 7.31 | 42.7 | render/thread/ms 54.09, render/busy/ms 53.03, encode//sum 47.03, stage/render/render 46.52, render/graph/ms 45.20, stage/render/graph_submit 40.87, encode/finish_wall 32.11, encode/pass:early prepass/sum 29.14, main/busy/ms 19.57, stage/main/post_update 12.78, encode/submit 8.83, encode/pass:shadow_point_light/sum 7.11 |
| measure | 18.19 | 30.8 | encode//sum 41.45, main/busy/ms 39.61, render/thread/ms 28.21, render/busy/ms 26.95, stage/main/update 24.27, stream/drive/ms 19.93, stage/render/render 18.00, render/graph/ms 16.42, stage/main/post_update 12.55, stage/render/graph_submit 12.26, encode/submit 6.21, encode/finish_wall 6.11 |
| measure | 59.01 | 27.4 | render/thread/ms 35.25, render/busy/ms 33.62, main/busy/ms 30.17, stage/render/render 19.23, encode//sum 19.03, stage/main/post_update 17.93, render/graph/ms 17.15, stage/render/graph_submit 12.89, stage/main/update 7.98, encode/submit 7.72, encode/finish_wall 5.29, stage/render/prepare_assets 5.13 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=lampcache-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **69%** (min 43%) · 2625 MHz (min 2625) · memory 8101 MHz · 71.5 W · 74 °C (max 81) · VRAM 6792 MiB · throttled 100% of samples

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
