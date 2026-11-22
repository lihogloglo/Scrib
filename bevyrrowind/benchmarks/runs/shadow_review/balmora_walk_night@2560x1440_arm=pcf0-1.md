# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6651 frames over 65.6 s · **median 9.79 ms (102 fps)** · mean 9.87 · p95 11.56 · p99 13.25 (1% low 75 fps) · worst 32.02 ms · 33 spikes over 1.5× median · 26.5 stutters/min (work 23.8/min)
- warm-up: 355 frames over 9.0 s, worst 2431.9 ms
- hitches (> 25.0 ms): **3** measured, 48 in warm-up
- timed GPU passes: 6.42 ms of a 9.87 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.79 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.51/min, limit 1/min
- pass worst frame 32.02 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.846 | 2.037 | 2.072 | 0.91 | 0.175 |
| main_opaque_pass_3d | 1.481 | 1.635 | 2.379 | 0.91 | 0.066 |
| early prepass | 0.467 | 0.515 | 0.754 | 0.91 | 0.088 |
| bloom | 0.288 | 0.318 | 0.325 | 0.91 | 0.064 |
| atmosphere_luts | 0.284 | 0.314 | 0.379 | 0.91 | 0.003 |
| water_velocity | 0.205 | 0.226 | 0.247 | 0.91 | 0.002 |
| water_sspr | 0.183 | 0.202 | 0.204 | 0.91 | 0.002 |
| ssao | 0.170 | 0.188 | 0.192 | 0.91 | 0.007 |
| point_shadow_atlas | 0.137 | 0.252 | 0.471 | 0.55 | 0.238 |
| upscaling | 0.117 | 0.129 | 0.136 | 0.91 | 0.004 |
| atmos_froxel_inject | 0.116 | 0.128 | 0.130 | 0.91 | 0.002 |
| water_fft | 0.111 | 0.123 | 0.126 | 0.91 | 0.003 |
| uniform_allocation | 0.106 | 0.117 | 0.141 | 0.91 | 0.021 |
| clouds_composite | 0.095 | 0.105 | 0.109 | 0.91 | 0.002 |
| gpu_instance_materialization | 0.086 | 0.095 | 0.099 | 0.91 | 0.004 |
| bin_unpacking | 0.083 | 0.092 | 0.113 | 0.91 | 0.016 |
| tonemapping | 0.079 | 0.087 | 0.092 | 0.91 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.073 | 0.142 | 0.91 | 0.009 |
| clustering | 0.058 | 0.064 | 0.068 | 0.91 | 0.020 |
| clouds_trace | 0.050 | 0.056 | 0.078 | 0.91 | 0.001 |
| early_mesh_preprocessing | 0.049 | 0.054 | 0.070 | 0.91 | 0.004 |
| render_sky | 0.048 | 0.053 | 0.054 | 0.91 | 0.002 |
| clouds_resolve | 0.045 | 0.050 | 0.051 | 0.91 | 0.001 |
| lightprobe_irradiance_map | 0.034 | 0.412 | 0.417 | 0.08 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.59 | 0.002 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.91 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.026 | 0.047 | 0.070 | 0.55 | 0.009 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.91 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.028 | 0.91 | 0.003 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.018 | 0.220 | 0.230 | 0.08 | 0.004 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.017 | 0.027 | 0.91 | 0.005 |
| point_shadow_atlas/uniform_allocation | 0.014 | 0.026 | 0.059 | 0.55 | 0.008 |
| main_transparent_pass_3d | 0.014 | 0.016 | 0.030 | 0.91 | 0.181 |
| point_shadow_atlas/bin_unpacking | 0.010 | 0.018 | 0.037 | 0.55 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.91 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.91 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.55 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.91 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.672 | 10.085 | 19.80 | 57675.2 |
| render/busy/ms | 8.146 | 9.551 | 19.33 | 54178.1 |
| main/busy/ms | 8.044 | 9.300 | 29.64 | 53502.8 |
| stage/render/render | 5.069 | 6.069 | 10.84 | 33717.1 |
| stage/main/post_update | 4.541 | 5.381 | 10.71 | 30201.2 |
| render/graph/ms | 4.393 | 5.300 | 9.54 | 29217.5 |
| encode//sum | 3.229 | 3.966 | 9.45 | 21479.3 |
| stage/render/graph_submit | 3.087 | 3.888 | 7.08 | 20534.3 |
| encode/submit | 1.954 | 2.801 | 5.01 | 12996.1 |
| stage/main/update | 1.813 | 2.319 | 23.70 | 12061.2 |
| stage/render/camera_driver | 1.244 | 1.514 | 2.85 | 8274.3 |
| encode/finish_wall | 1.213 | 1.583 | 2.80 | 8067.5 |
| encode/pass:early prepass/sum | 0.807 | 1.050 | 1.88 | 5370.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.801 | 1.057 | 1.99 | 5330.7 |
| encode/pass:shadow_directional_light/sum | 0.800 | 1.194 | 2.06 | 5321.7 |
| stage/render/prepare_write_phase_buffers | 0.583 | 0.884 | 2.10 | 3874.3 |
| bigspace/total/ms | 0.542 | 1.292 | 2.10 | 3605.2 |
| stage/main/fixed_main_loop | 0.510 | 0.954 | 2.38 | 3392.7 |
| stage/render/prepare_assets | 0.509 | 0.652 | 9.91 | 3382.9 |
| render/present/ms | 0.429 | 1.172 | 1.89 | 2854.3 |
| stage/render/prepare_meshes | 0.418 | 0.521 | 3.72 | 2778.3 |
| stage/render/prepare_resources | 0.417 | 0.498 | 1.85 | 2772.0 |
| stage/main/pre_update | 0.383 | 0.538 | 2.04 | 2549.9 |
| stage/main/first | 0.368 | 0.489 | 5.22 | 2444.9 |
| stage/render/prepare_flush | 0.321 | 0.533 | 1.07 | 2137.3 |
| controller/feed/ms | 0.318 | 0.403 | 0.83 | 2115.9 |
| stage/render/prepare_bind_groups | 0.301 | 0.499 | 1.41 | 2000.4 |
| stage/render/prepare_batch_phases | 0.270 | 0.410 | 0.72 | 1795.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.267 | 0.430 | 1.38 | 1777.0 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.255 | 1.141 | 1.94 | 1697.3 |
| stage/main/last | 0.251 | 0.292 | 1.43 | 1669.3 |
| bigspace/low_precision/ms | 0.225 | 0.724 | 1.74 | 1495.5 |
| stage/render/create_views | 0.200 | 0.272 | 0.58 | 1327.1 |
| stream/drive/ms | 0.195 | 0.276 | 10.19 | 1295.3 |
| stage/render/extract_commands | 0.184 | 0.216 | 0.63 | 1226.5 |
| bigspace/high_precision/ms | 0.170 | 0.694 | 1.80 | 1129.1 |
| anim/evaluate/ms | 0.157 | 0.283 | 1.72 | 1045.3 |
| stage/render/specialize | 0.156 | 0.246 | 1.80 | 1035.9 |
| anim/evaluate_cpu/ms | 0.149 | 0.189 | 0.63 | 993.5 |
| stream/tick/ms | 0.143 | 0.210 | 0.58 | 952.1 |
| stage/render/prepare_views | 0.138 | 0.180 | 1.25 | 918.4 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.109 | 0.845 | 1.69 | 728.2 |
| stage/render/queue | 0.098 | 0.171 | 1.47 | 651.9 |
| render/acquire/ms | 0.097 | 0.127 | 1.21 | 642.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.092 | 0.145 | 0.75 | 609.1 |
| stage/render/phase_sort | 0.080 | 0.110 | 0.34 | 532.9 |
| controller/write/ms | 0.074 | 0.100 | 0.37 | 489.9 |
| controller/move/ms | 0.073 | 0.224 | 0.58 | 485.8 |
| terrain/update/ms | 0.058 | 0.289 | 2.08 | 388.8 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.053 | 0.000 | 2.62 | 352.2 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.073 | 0.40 | 307.7 |
| physics/step/ms | 0.044 | 0.119 | 0.51 | 292.7 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.50 | 214.2 |
| physics/insert/ms | 0.025 | 0.279 | 0.93 | 163.2 |
| stage/render/cleanup | 0.024 | 0.033 | 0.49 | 160.0 |
| pipelines/compile_ms | 0.024 | 0.000 | 76.37 | 159.7 |
| pipelines/front_ms | 0.023 | 0.000 | 73.81 | 154.3 |
| terrain/select/ms | 0.021 | 0.256 | 0.78 | 142.5 |
| weather/map/ms | 0.018 | 0.025 | 0.07 | 116.7 |
| far/select/ms | 0.015 | 0.166 | 0.39 | 97.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.14 | 84.7 |
| anim/apply/ms | 0.011 | 0.021 | 0.49 | 74.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.56 | 72.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.17 | 69.0 |
| script/update/ms | 0.008 | 0.011 | 0.20 | 50.5 |
| controller/avoid/ms | 0.006 | 0.012 | 0.41 | 38.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.28 | 36.3 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 31.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.14 | 21.6 |
| script/ui/ms | 0.003 | 0.004 | 0.16 | 20.4 |
| stream/upload/ms | 0.003 | 0.000 | 1.01 | 18.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 12.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 6.9 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.58 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
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
| anim/evaluated/count | count | 18 | 31 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
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
| crowd/joint_blocks/count | count | 160 | 216 | 204 |
| crowd/joints_extracted/count | count | 671 | 5537 | 563 |
| crowd/joints_uploaded/count | count | 3599 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.40 | 1.38 | 0.38 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.14 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.56 | 0.01 |
| encode/finish_critical | ms | 0.96 | 2.62 | 0.94 |
| encode/pass:early prepass/max | ms | 0.81 | 1.88 | 0.80 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 1.99 | 0.94 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.75 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.38 | 0.26 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.83 | 1.94 | 0.94 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.32 | 1.69 | 0.24 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.06 | 2.62 | 0.83 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.06 | 0.75 |
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
| frame/work/ms | ms | 8.56 | 29.64 | 8.81 |
| gpu/timed/ms | ms | 7.87 | 9.37 | 8.81 |
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
| phases/opaque3d/bins | count | 766 | 971 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2479 | 3552 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 971 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2479 | 3552 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 70 | 429 | 8 |
| phases/shadow/bins | count | 826 | 1542 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2309 | 3284 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 49 | 151 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2679 | 3268 | 2346 |
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
| preprocess/bind_groups | count | 15 | 274 | 11 |
| preprocess/bind_groups_created | count | 10 | 268 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 137 | 227 | 123 |
| process/cpu_usage | % | 25.36 | 37.50 | 24.52 |
| process/mem_usage | GiB | 5.09 | 5.40 | 5.40 |
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
| shadow_atlas/live_occ_pct | % | 4.79 | 37.60 | 0 |
| shadow_atlas/live_tiles | count | 17 | 70 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 158 | 499 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 19 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 63 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 112 | 0 |
| shadow_atlas/views | count | 17 | 63 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 95 | 11 |
| skin/blocks_skipped/count | count | 146 | 205 | 193 |
| skin/blocks_walked/count | count | 14 | 27 | 11 |
| skin/joint_bytes_written/bytes | bytes | 47.1 k | 811.1 k | 41.0 k |
| skin/joints_written/count | count | 736 | 12.7 k | 640 |
| skin/upload_writes/count | count | 7 | 24 | 9 |
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
| system/cpu_usage | % | 29.54 | 42.27 | 28.91 |
| system/mem_usage | % | 53.63 | 54.39 | 54.02 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2666 | 2724 | 2724 |
| views/all/batch_sets | count | 129 | 484 | 66 |
| views/all/bins | count | 2933 | 3577 | 2948 |
| views/all/count | count | 20 | 66 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8010 | 10.5 k | 7956 |
| views/all/pipelines | count | 97 | 198 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 230.7 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 713 | 849 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2123 | 2427 | 2014 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 57 | 416 | 0 |
| views/lamp_face/bins | count | 95 | 704 | 0 |
| views/lamp_face/count | count | 17 | 63 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 129 | 864 | 0 |
| views/lamp_face/pipelines | count | 24 | 125 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 107 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2012 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7524 | 5942 |
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
| anim/bones_written/count | count | 47.55 | 231 | 316.2 k |
| anim/events/count | count | 0.08 | 3 | 562 |
| assets/Image/added | count | 0.09 | 36 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 262 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7481 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1168 | 1489 |
| encode//count | count | 27.91 | 30 | 185.6 k |
| encode/bind_walk/init_textures | count | 9091 | 23.1 k | 60.47 M |
| encode/bind_walk/init_textures_skipped | count | 47.6 k | 73.2 k | 316.49 M |
| encode/bind_walk/inits | count | 563 | 769 | 3.75 M |
| encode/bind_walk/inits_skipped | count | 167 | 250 | 1.11 M |
| encode/bind_walk/merge_textures | count | 9028 | 22.9 k | 60.05 M |
| encode/bind_walk/merge_textures_skipped | count | 10.1 k | 26.4 k | 66.97 M |
| encode/bind_walk/merges | count | 104 | 183 | 693.7 k |
| encode/bind_walk/merges_skipped | count | 149 | 224 | 992.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6651 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6651 |
| encode/pass:early prepass/count | count | 1 | 1 | 6651 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6651 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6651 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6651 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.29 | 1 | 1923 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.23 | 1 | 1555 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 310 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6651 |
| encode/queue/command_buffers | count | 38.49 | 42 | 256.0 k |
| encode/queue/submits | count | 2.01 | 4 | 13.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.54 M | 15.43 G |
| encode/queue/write_buffer/calls | count | 272 | 449 | 1.81 M |
| encode/queue/write_buffer_with/bytes | bytes | 172.4 k | 1.29 M | 1.15 G |
| encode/queue/write_buffer_with/calls | count | 10.28 | 795 | 68.4 k |
| encode/queue/write_texture/bytes | bytes | 1984 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6651 |
| physics/inserts | count | 0.34 | 87 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 163 | 2335 |
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
| stream/deactivate/items | count | 0.03 | 25 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5090 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1116 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 51, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.43 | 2431.9 | main/busy/ms 2404, stage/main/post_update 2359, stage/main/update 34.60, stage/main/pre_update 7.11, stage/main/last 1.23, surface/advance_cpu_ms 0.79, stage/main/first 0.54, stream/drive/ms 0.50, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.32, stream/tick/ms 0.12, bigspace/total/ms 0.10 |
| warm-up | 3.19 | 757.1 | main/busy/ms 2530, stage/main/post_update 2380, render/thread/ms 719, render/busy/ms 578, stage/render/prepare_assets 517, stage/render/prepare_views 137, render/acquire/ms 137, stage/main/update 133, stage/render/create_views 29.68, stage/render/render 23.86, render/graph/ms 18.41, stage/render/camera_driver 16.36 |
| warm-up | 3.31 | 116.1 | render/thread/ms 819, render/busy/ms 623, stage/render/prepare_assets 550, main/busy/ms 154, stage/render/prepare_views 138, render/acquire/ms 137, stage/main/update 108, stage/render/render 82.73, pipelines/compile_ms 59.79, render/present/ms 58.52, pipelines/driver_ms 42.32, stage/main/post_update 32.29 |
| warm-up | 3.35 | 48.5 | pipelines/compile_ms 433, pipelines/front_ms 321, render/thread/ms 132, pipelines/driver_ms 111, pipelines/wait_ms 104, stage/render/render 84.73, render/busy/ms 70.01, render/present/ms 61.14, main/busy/ms 55.06, stage/render/prepare_assets 33.92, stage/main/post_update 29.66, render/graph/ms 22.55 |
| warm-up | 3.46 | 104.4 | pipelines/wait_ms 3945, pipelines/compile_ms 1388, pipelines/front_ms 1266, render/thread/ms 134, pipelines/driver_ms 122, render/busy/ms 106, main/busy/ms 58.97, stage/render/render 38.49, render/graph/ms 29.86, stage/main/post_update 29.20, stage/render/prepare_bind_groups 24.88, stage/render/graph_submit 23.76 |
| warm-up | 3.50 | 46.7 | pipelines/wait_ms 4350, pipelines/compile_ms 1623, pipelines/front_ms 1548, render/thread/ms 139, render/busy/ms 117, main/busy/ms 117, pipelines/driver_ms 74.65, stage/main/update 65.80, stage/render/prepare_bind_groups 26.35, stage/render/prepare_resources 24.91, stage/main/post_update 21.69, stage/render/prepare_views 21.45 |
| warm-up | 3.64 | 131.5 | pipelines/compile_ms 1908, pipelines/front_ms 1826, pipelines/wait_ms 741, render/thread/ms 109, main/busy/ms 102, render/busy/ms 100, pipelines/driver_ms 81.88, stage/main/update 70.02, stage/render/prepare_assets 65.25, stage/main/post_update 18.71, stage/render/render 18.27, terrain/update/ms 14.03 |
| warm-up | 3.67 | 34.7 | pipelines/compile_ms 1650, pipelines/front_ms 1576, pipelines/wait_ms 369, render/thread/ms 106, render/busy/ms 95.95, stage/render/prepare_assets 75.79, pipelines/driver_ms 74.19, main/busy/ms 40.17, stage/render/render 21.34, encode//sum 20.62, stage/main/post_update 17.82, render/graph/ms 11.39 |
| warm-up | 3.74 | 25.4 | pipelines/compile_ms 384, pipelines/front_ms 366, pipelines/wait_ms 58.96, render/thread/ms 37.91, render/busy/ms 36.51, main/busy/ms 33.46, pipelines/driver_ms 18.68, stage/main/post_update 14.51, stage/main/update 10.70, stage/render/prepare_resources 10.14, stage/render/render 9.78, stage/render/prepare_assets 9.42 |
| warm-up | 3.78 | 39.5 | pipelines/compile_ms 210, pipelines/front_ms 197, main/busy/ms 79.44, stage/main/update 60.75, render/thread/ms 59.18, render/busy/ms 57.56, stage/render/prepare_assets 27.56, stage/render/prepare_resources 13.28, pipelines/driver_ms 12.89, pipelines/wait_ms 12.58, stage/main/post_update 11.10, stage/render/render 10.91 |
| warm-up | 3.87 | 95.5 | pipelines/compile_ms 1409, pipelines/front_ms 1337, main/busy/ms 76.91, render/thread/ms 75.10, render/busy/ms 73.45, pipelines/driver_ms 72.02, stage/main/update 61.73, stage/render/prepare_assets 46.02, pipelines/wait_ms 33.29, stage/render/render 15.64, render/graph/ms 13.76, stage/render/graph_submit 11.46 |
| warm-up | 3.92 | 45.8 | pipelines/compile_ms 1734, pipelines/front_ms 1660, pipelines/driver_ms 73.61, render/thread/ms 66.42, render/busy/ms 65.13, stage/render/prepare_assets 46.98, pipelines/wait_ms 24.61, main/busy/ms 21.55, stage/render/render 14.12, render/graph/ms 12.56, stage/render/graph_submit 10.34, stage/main/update 8.74 |
| warm-up | 3.97 | 36.7 | pipelines/compile_ms 479, pipelines/front_ms 463, render/thread/ms 48.99, stage/render/prepare_views 24.22, render/acquire/ms 24.09, render/busy/ms 23.05, main/busy/ms 21.47, stage/render/render 17.19, encode//sum 16.06, pipelines/driver_ms 15.83, render/graph/ms 14.86, stage/main/update 13.10 |
| warm-up | 4.03 | 40.5 | pipelines/compile_ms 87.35, pipelines/front_ms 83.77, render/thread/ms 44.95, render/busy/ms 40.62, main/busy/ms 32.48, stage/render/render 23.44, stage/main/update 20.09, render/graph/ms 18.63, stage/render/prepare_assets 12.49, stage/render/graph_submit 11.08, stage/main/post_update 8.58, encode//sum 7.86 |
| warm-up | 4.06 | 38.7 | pipelines/compile_ms 684, pipelines/front_ms 646, render/thread/ms 66.83, render/busy/ms 64.02, main/busy/ms 47.34, pipelines/driver_ms 37.92, stage/main/post_update 22.99, stage/render/prepare_assets 20.50, stage/render/render 20.16, render/graph/ms 16.59, stage/main/update 16.07, stage/render/graph_submit 10.49 |
| warm-up | 4.12 | 52.6 | pipelines/compile_ms 795, pipelines/front_ms 757, render/thread/ms 63.91, render/busy/ms 61.79, main/busy/ms 53.26, pipelines/driver_ms 38.41, stage/render/prepare_assets 30.01, stage/main/post_update 22.03, stage/main/update 21.42, stage/render/render 11.77, render/graph/ms 9.15, encode//sum 7.74 |
| warm-up | 4.17 | 52.4 | pipelines/compile_ms 814, pipelines/front_ms 796, render/thread/ms 65.89, render/busy/ms 64.47, main/busy/ms 56.67, stage/render/prepare_assets 45.72, stage/main/update 38.57, pipelines/driver_ms 18.37, stage/main/post_update 11.58, stage/render/render 10.39, render/graph/ms 8.67, encode//sum 6.24 |
| warm-up | 4.23 | 59.9 | pipelines/compile_ms 1170, pipelines/front_ms 1137, main/busy/ms 191, stage/main/update 149, pipelines/wait_ms 99.51, render/thread/ms 83.92, render/busy/ms 82.36, stage/render/prepare_assets 50.07, pipelines/driver_ms 32.87, stage/main/post_update 19.75, stage/main/pre_update 13.31, stage/render/render 12.84 |
| warm-up | 4.40 | 174.8 | pipelines/compile_ms 2686, pipelines/front_ms 2601, pipelines/wait_ms 461, main/busy/ms 177, stage/main/update 137, render/thread/ms 91.78, render/busy/ms 90.14, pipelines/driver_ms 85.19, stage/render/prepare_assets 51.09, stage/render/render 19.62, render/graph/ms 17.54, stage/main/post_update 16.52 |
| warm-up | 4.44 | 35.5 | pipelines/compile_ms 2609, pipelines/front_ms 2527, pipelines/wait_ms 397, pipelines/driver_ms 81.68, render/thread/ms 77.43, render/busy/ms 75.82, main/busy/ms 47.22, stage/render/prepare_assets 42.05, stage/main/update 20.18, stage/render/render 19.88, render/graph/ms 17.92, stage/main/pre_update 12.72 |
| warm-up | 4.50 | 55.7 | pipelines/compile_ms 1119, pipelines/front_ms 1080, pipelines/wait_ms 193, render/thread/ms 80.53, render/busy/ms 79.19, main/busy/ms 69.16, pipelines/driver_ms 38.78, stage/main/post_update 26.97, stage/render/prepare_assets 26.93, stage/main/update 23.44, stage/render/prepare_flush 15.69, stage/render/render 15.28 |
| warm-up | 4.53 | 36.2 | pipelines/compile_ms 668, pipelines/front_ms 643, pipelines/wait_ms 160, render/thread/ms 77.44, main/busy/ms 75.98, render/busy/ms 75.19, stage/main/update 35.38, stage/main/post_update 31.56, pipelines/driver_ms 25.00, stream/drive/ms 20.78, stage/render/prepare_assets 19.07, stage/render/render 18.88 |
| warm-up | 4.61 | 77.0 | pipelines/compile_ms 1178, pipelines/front_ms 1127, pipelines/wait_ms 131, main/busy/ms 94.46, render/thread/ms 78.59, render/busy/ms 75.65, stage/main/update 67.21, pipelines/driver_ms 51.11, stage/render/prepare_assets 36.14, stage/render/render 19.96, stage/main/post_update 16.24, render/graph/ms 15.98 |
| warm-up | 4.69 | 77.0 | pipelines/compile_ms 2045, pipelines/front_ms 1971, pipelines/wait_ms 155, render/thread/ms 88.89, render/busy/ms 86.76, main/busy/ms 82.94, pipelines/driver_ms 73.42, stage/main/update 58.50, stage/render/prepare_assets 52.66, stage/render/render 17.31, render/graph/ms 14.64, stage/main/post_update 13.37 |
| warm-up | 4.74 | 52.0 | pipelines/compile_ms 1205, pipelines/front_ms 1172, render/thread/ms 77.02, render/busy/ms 75.56, main/busy/ms 45.57, stage/render/prepare_assets 41.08, pipelines/driver_ms 32.86, pipelines/wait_ms 27.96, stage/main/update 21.21, stage/render/render 14.46, render/graph/ms 12.60, stage/main/post_update 9.34 |
| warm-up | 4.77 | 27.9 | pipelines/compile_ms 386, pipelines/front_ms 373, main/busy/ms 91.13, stage/main/update 65.99, render/thread/ms 64.22, render/busy/ms 62.53, stream/drive/ms 37.19, stage/render/render 19.23, stage/render/prepare_assets 18.83, render/graph/ms 16.62, pipelines/driver_ms 13.01, stage/render/graph_submit 12.26 |
| warm-up | 4.84 | 78.2 | pipelines/compile_ms 456, pipelines/front_ms 443, main/busy/ms 103, stage/main/update 83.29, stream/drive/ms 57.92, render/thread/ms 54.26, render/busy/ms 52.46, stage/render/render 18.60, stage/render/prepare_assets 17.15, render/graph/ms 15.84, pipelines/driver_ms 13.00, stage/main/post_update 11.46 |
| warm-up | 4.87 | 29.5 | pipelines/compile_ms 1457, pipelines/front_ms 1420, main/busy/ms 73.17, stage/main/update 58.25, render/thread/ms 46.76, render/busy/ms 45.32, stream/drive/ms 39.24, pipelines/driver_ms 36.66, stage/render/prepare_assets 17.13, stage/render/render 15.13, render/graph/ms 13.08, stage/render/graph_submit 8.82 |
| warm-up | 4.92 | 48.3 | pipelines/compile_ms 1468, pipelines/front_ms 1427, main/busy/ms 51.31, render/thread/ms 40.39, pipelines/driver_ms 40.11, render/busy/ms 39.08, stage/main/update 37.87, stream/drive/ms 19.57, stage/render/prepare_assets 15.76, stage/render/render 13.16, render/graph/ms 11.37, stage/render/graph_submit 7.84 |
| warm-up | 4.97 | 28.0 | pipelines/compile_ms 52.55, pipelines/front_ms 50.39, render/thread/ms 46.39, render/busy/ms 44.78, main/busy/ms 40.30, stage/render/render 19.87, stage/main/update 19.61, render/graph/ms 17.76, stage/main/post_update 15.06, stage/render/graph_submit 13.53, encode/submit 8.00, encode//sum 7.89 |
| warm-up | 5.02 | 26.9 | render/thread/ms 38.04, render/busy/ms 36.90, main/busy/ms 27.92, stage/render/prepare_assets 15.08, stage/main/update 13.89, stage/render/render 11.17, render/graph/ms 9.39, stage/main/post_update 8.59, encode//sum 7.85, stage/render/graph_submit 6.14, stage/render/prepare_write_phase_buffers 4.79, encode/submit 3.94 |
| warm-up | 5.10 | 30.6 | pipelines/compile_ms 291, pipelines/front_ms 279, render/thread/ms 42.12, render/busy/ms 40.69, main/busy/ms 39.30, stage/main/post_update 23.11, stage/render/render 17.74, render/graph/ms 15.73, stage/render/graph_submit 11.59, stage/main/update 11.47, pipelines/driver_ms 11.38, encode//sum 7.43 |
| warm-up | 5.19 | 25.2 | pipelines/compile_ms 638, pipelines/front_ms 621, main/busy/ms 33.16, render/thread/ms 30.17, render/busy/ms 28.81, stage/main/update 18.79, pipelines/driver_ms 17.00, stream/drive/ms 11.48, stage/render/render 11.31, render/graph/ms 9.35, stage/main/post_update 8.65, encode//sum 7.94 |
| warm-up | 5.25 | 31.8 | pipelines/compile_ms 379, pipelines/front_ms 370, render/thread/ms 44.92, render/busy/ms 42.66, main/busy/ms 40.56, stage/render/render 19.77, stage/main/update 18.93, render/graph/ms 17.07, stage/main/post_update 15.48, stage/render/prepare_assets 12.48, stage/render/graph_submit 12.29, pipelines/driver_ms 8.90 |
| warm-up | 5.28 | 25.9 | pipelines/compile_ms 1163, pipelines/front_ms 1136, render/thread/ms 42.42, render/busy/ms 41.11, main/busy/ms 27.23, pipelines/driver_ms 26.82, stage/render/prepare_assets 18.32, stage/render/render 12.57, stage/main/update 10.77, render/graph/ms 10.74, stage/main/post_update 10.50, encode//sum 7.57 |
| warm-up | 5.50 | 26.7 | pipelines/compile_ms 572, pipelines/front_ms 558, main/busy/ms 37.70, render/thread/ms 27.93, render/busy/ms 26.68, stage/main/update 22.77, stream/drive/ms 15.50, pipelines/driver_ms 13.65, stage/render/render 12.40, render/graph/ms 10.74, stage/main/post_update 8.00, stage/render/graph_submit 7.78 |
| warm-up | 5.71 | 27.9 | main/busy/ms 33.79, render/thread/ms 22.78, render/busy/ms 21.80, stage/main/update 21.71, stream/drive/ms 13.51, stage/render/render 10.24, render/graph/ms 8.64, encode//sum 7.39, stage/main/post_update 6.88, stage/render/graph_submit 5.32, stage/render/prepare_assets 4.64, stage/render/camera_driver 3.17 |
| warm-up | 5.82 | 26.9 | pipelines/compile_ms 126, pipelines/front_ms 120, render/thread/ms 37.92, render/busy/ms 36.47, main/busy/ms 31.09, stage/main/update 13.36, stage/main/post_update 12.79, stage/render/prepare_assets 12.70, stage/render/render 12.67, render/graph/ms 10.57, encode//sum 9.61, stage/render/graph_submit 6.94 |
| warm-up | 5.87 | 30.6 | render/thread/ms 37.32, render/busy/ms 36.46, main/busy/ms 25.17, stage/render/prepare_assets 17.54, stage/main/update 11.41, stage/render/render 8.98, render/graph/ms 7.55, stage/main/post_update 7.24, encode//sum 6.71, stage/render/graph_submit 4.69, stream/drive/ms 3.30, stage/render/camera_driver 2.72 |
| warm-up | 6.06 | 25.4 | render/thread/ms 38.14, render/busy/ms 36.58, main/busy/ms 36.28, stage/main/update 17.92, stage/render/render 15.82, stage/main/post_update 14.36, render/graph/ms 13.35, encode//sum 10.06, stage/render/graph_submit 8.70, stage/render/prepare_assets 7.58, encode/submit 5.17, stage/render/camera_driver 4.40 |
| warm-up | 6.11 | 28.4 | pipelines/compile_ms 141, pipelines/front_ms 134, render/thread/ms 37.67, render/busy/ms 36.60, main/busy/ms 34.41, stage/main/update 18.86, stage/render/prepare_assets 12.76, stage/render/render 9.56, stage/main/post_update 8.21, render/graph/ms 8.12, pipelines/driver_ms 6.77, encode//sum 6.36 |
| warm-up | 6.14 | 28.0 | render/thread/ms 46.84, render/busy/ms 45.67, main/busy/ms 37.03, stage/main/update 19.12, stage/render/render 18.57, render/graph/ms 16.80, pipelines/compile_ms 15.57, pipelines/front_ms 14.53, stage/render/graph_submit 12.69, stage/render/prepare_assets 11.75, stage/main/post_update 11.63, encode//sum 8.72 |
| warm-up | 6.17 | 34.5 | render/thread/ms 58.90, render/busy/ms 57.59, main/busy/ms 41.99, stage/render/render 27.77, render/graph/ms 25.91, stage/render/graph_submit 20.16, stage/main/post_update 18.59, stage/main/update 18.19, encode/finish_wall 13.41, encode/pass:point_shadow_atlas_live_indirect/sum 13.03, encode//sum 11.48, stage/render/prepare_assets 7.58 |
| warm-up | 6.20 | 28.1 | render/thread/ms 54.90, render/busy/ms 53.75, main/busy/ms 41.54, stage/render/render 20.68, stage/main/update 20.29, render/graph/ms 18.97, stage/main/post_update 15.97, stage/render/graph_submit 14.43, stage/render/prepare_assets 11.21, encode/finish_wall 9.06, encode//sum 8.99, encode/pass:point_shadow_atlas_live_indirect/sum 8.48 |
| warm-up | 6.24 | 36.5 | render/thread/ms 48.89, render/busy/ms 47.93, main/busy/ms 38.31, stage/main/update 23.04, stage/render/prepare_assets 17.62, stage/render/render 14.77, render/graph/ms 13.15, stage/render/graph_submit 10.10, stage/main/post_update 9.36, encode/finish_wall 6.74, encode/pass:point_shadow_atlas_live_indirect/sum 6.39, encode//sum 6.06 |
| warm-up | 6.35 | 33.1 | main/busy/ms 38.80, render/thread/ms 28.62, render/busy/ms 27.71, stage/main/update 24.22, stage/render/render 14.05, stream/drive/ms 13.89, render/graph/ms 12.62, stage/main/post_update 9.97, stage/render/graph_submit 9.57, encode//sum 7.40, encode/finish_wall 5.75, encode/pass:point_shadow_atlas_live_indirect/sum 5.37 |
| warm-up | 6.42 | 41.3 | pipelines/compile_ms 141, pipelines/front_ms 136, main/busy/ms 51.54, render/thread/ms 45.05, render/busy/ms 43.57, stage/main/update 32.27, stage/render/render 18.79, render/graph/ms 16.22, stage/main/post_update 13.34, stage/render/prepare_assets 12.92, stage/render/graph_submit 12.81, encode/submit 7.64 |
| warm-up | 6.45 | 31.1 | pipelines/compile_ms 152, pipelines/front_ms 147, render/thread/ms 43.62, render/busy/ms 42.60, main/busy/ms 37.46, stage/render/prepare_assets 20.20, stage/main/update 19.67, stage/render/render 11.51, stage/main/post_update 10.44, render/graph/ms 10.01, stream/drive/ms 7.81, stage/render/graph_submit 7.37 |
| measure | 18.30 | 32.0 | main/busy/ms 37.74, stage/main/update 25.54, render/thread/ms 20.73, render/busy/ms 19.74, stage/render/render 10.08, render/graph/ms 8.71, stage/main/post_update 8.43, stage/render/graph_submit 5.74, encode//sum 5.63, stage/render/prepare_assets 3.86, encode/submit 3.68, stage/render/camera_driver 2.84 |
| measure | 59.06 | 25.6 | render/thread/ms 36.10, main/busy/ms 34.96, render/busy/ms 34.65, stage/main/post_update 20.05, stage/render/render 18.07, render/graph/ms 15.94, encode//sum 12.67, stage/render/graph_submit 10.72, stage/main/update 7.87, stage/render/prepare_assets 6.80, encode/submit 6.34, stage/render/camera_driver 5.03 |
| measure | 59.09 | 27.2 | render/thread/ms 37.21, render/busy/ms 36.30, main/busy/ms 29.43, stage/main/post_update 15.89, stage/render/prepare_assets 15.18, stage/render/render 11.55, encode//sum 10.34, render/graph/ms 10.13, stage/render/graph_submit 6.19, stage/main/update 5.56, stage/main/fixed_main_loop 3.82, stage/render/camera_driver 3.75 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.soft_pcf_radius=0 --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=pcf0-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (254 samples): GPU load **85%** (min 61%) · 2613 MHz (min 2610) · memory 8101 MHz · 85.2 W · 79 °C (max 85) · VRAM 5645 MiB · throttled 100% of samples

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
