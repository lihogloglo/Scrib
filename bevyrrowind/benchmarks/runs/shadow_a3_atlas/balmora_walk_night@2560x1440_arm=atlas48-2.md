# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=48 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6666 frames over 65.6 s · **median 9.73 ms (103 fps)** · mean 9.85 · p95 11.57 · p99 13.38 (1% low 75 fps) · worst 30.09 ms · 36 spikes over 1.5× median · 35.6 stutters/min (work 29.2/min)
- warm-up: 353 frames over 9.3 s, worst 2494.8 ms
- hitches (> 25.0 ms): **3** measured, 44 in warm-up
- timed GPU passes: 6.29 ms of a 9.85 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.73 ms, limit 11 ms
- **FAIL** frame_ms: 39 stutters (> 1.5x moving median of 60) in 1.09 min = 35.65/min, limit 1/min
- pass worst frame 30.09 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.940 | 2.179 | 2.596 | 0.89 | 0.174 |
| main_opaque_pass_3d | 1.193 | 1.340 | 1.881 | 0.89 | 0.066 |
| early prepass | 0.467 | 0.525 | 0.764 | 0.89 | 0.093 |
| bloom | 0.306 | 0.344 | 0.659 | 0.89 | 0.064 |
| atmosphere_luts | 0.301 | 0.338 | 0.632 | 0.89 | 0.003 |
| water_velocity | 0.218 | 0.245 | 0.397 | 0.89 | 0.002 |
| water_sspr | 0.196 | 0.220 | 0.349 | 0.89 | 0.002 |
| ssao | 0.178 | 0.200 | 0.205 | 0.89 | 0.008 |
| upscaling | 0.122 | 0.137 | 0.165 | 0.89 | 0.004 |
| atmos_froxel_inject | 0.122 | 0.137 | 0.130 | 0.89 | 0.001 |
| water_fft | 0.114 | 0.128 | 0.128 | 0.89 | 0.003 |
| uniform_allocation | 0.109 | 0.123 | 0.142 | 0.89 | 0.022 |
| point_shadow_atlas | 0.105 | 0.197 | 0.419 | 0.53 | 0.101 |
| clouds_composite | 0.095 | 0.107 | 0.109 | 0.89 | 0.002 |
| bin_unpacking | 0.085 | 0.096 | 0.112 | 0.89 | 0.017 |
| tonemapping | 0.082 | 0.092 | 0.095 | 0.89 | 0.004 |
| gpu_instance_materialization | 0.081 | 0.091 | 0.094 | 0.89 | 0.004 |
| main_transmissive_pass_3d | 0.068 | 0.076 | 0.148 | 0.89 | 0.009 |
| clustering | 0.060 | 0.068 | 0.068 | 0.89 | 0.019 |
| early_mesh_preprocessing | 0.052 | 0.059 | 0.073 | 0.89 | 0.003 |
| clouds_trace | 0.049 | 0.055 | 0.078 | 0.89 | 0.001 |
| render_sky | 0.048 | 0.054 | 0.054 | 0.89 | 0.002 |
| clouds_resolve | 0.044 | 0.050 | 0.051 | 0.89 | 0.001 |
| surface_trail | 0.034 | 0.058 | 0.057 | 0.58 | 0.002 |
| lightprobe_irradiance_map | 0.032 | 0.465 | 0.765 | 0.07 | 0.001 |
| atmos_froxel_integrate | 0.028 | 0.032 | 0.032 | 0.89 | 0.001 |
| water_snapshot | 0.024 | 0.027 | 0.033 | 0.89 | 0.002 |
| range_unpacking | 0.022 | 0.024 | 0.028 | 0.89 | 0.003 |
| water_mips | 0.021 | 0.024 | 0.024 | 0.89 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.019 | 0.026 | 0.89 | 0.005 |
| night_sky | 0.015 | 0.017 | 0.043 | 0.89 | 0.002 |
| lightprobe_radiance_map | 0.015 | 0.218 | 0.228 | 0.07 | 0.004 |
| main_transparent_pass_3d | 0.014 | 0.016 | 0.029 | 0.89 | 0.184 |
| point_shadow_atlas/early_mesh_preprocessing | 0.012 | 0.023 | 0.047 | 0.53 | 0.005 |
| point_shadow_atlas/uniform_allocation | 0.012 | 0.023 | 0.052 | 0.53 | 0.006 |
| atmosphere_environment | 0.010 | 0.012 | 0.012 | 0.89 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.89 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.009 | 0.016 | 0.031 | 0.53 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.89 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.53 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.89 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.89 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.634 | 9.974 | 27.13 | 57551.2 |
| render/busy/ms | 8.085 | 9.406 | 19.62 | 53897.5 |
| main/busy/ms | 8.060 | 9.260 | 26.50 | 53729.2 |
| stage/render/render | 5.004 | 6.056 | 16.60 | 33355.9 |
| stage/main/post_update | 4.531 | 5.335 | 11.55 | 30202.8 |
| render/graph/ms | 4.304 | 5.197 | 11.49 | 28690.9 |
| encode//sum | 3.210 | 3.941 | 7.99 | 21400.2 |
| stage/render/graph_submit | 3.061 | 3.906 | 9.43 | 20402.0 |
| encode/submit | 1.961 | 2.846 | 5.39 | 13073.1 |
| stage/main/update | 1.862 | 2.368 | 20.74 | 12409.1 |
| encode/finish_wall | 1.195 | 1.539 | 6.03 | 7963.7 |
| stage/render/camera_driver | 1.181 | 1.382 | 2.97 | 7870.3 |
| encode/pass:early prepass/sum | 0.805 | 1.057 | 2.45 | 5368.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.804 | 1.063 | 5.97 | 5361.2 |
| encode/pass:shadow_directional_light/sum | 0.804 | 1.220 | 2.11 | 5358.7 |
| stage/render/prepare_write_phase_buffers | 0.582 | 0.875 | 2.63 | 3878.7 |
| bigspace/total/ms | 0.547 | 1.287 | 3.97 | 3649.4 |
| stage/render/prepare_assets | 0.523 | 0.658 | 10.96 | 3486.5 |
| stage/main/fixed_main_loop | 0.491 | 0.890 | 2.49 | 3275.3 |
| render/present/ms | 0.456 | 1.252 | 11.21 | 3041.4 |
| stage/render/prepare_meshes | 0.430 | 0.559 | 3.87 | 2865.2 |
| stage/render/prepare_resources | 0.422 | 0.520 | 1.35 | 2810.5 |
| stage/main/pre_update | 0.375 | 0.507 | 3.17 | 2498.2 |
| stage/main/first | 0.364 | 0.479 | 5.43 | 2428.5 |
| controller/feed/ms | 0.330 | 0.431 | 0.95 | 2202.4 |
| stage/render/prepare_bind_groups | 0.299 | 0.483 | 2.10 | 1990.6 |
| stage/render/prepare_flush | 0.296 | 0.538 | 2.96 | 1971.0 |
| stage/render/prepare_batch_phases | 0.274 | 0.420 | 0.74 | 1826.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.266 | 0.433 | 0.92 | 1774.3 |
| stage/main/last | 0.256 | 0.310 | 0.83 | 1707.8 |
| bigspace/low_precision/ms | 0.235 | 0.747 | 1.70 | 1568.9 |
| stage/render/create_views | 0.219 | 0.307 | 0.69 | 1457.2 |
| stream/drive/ms | 0.207 | 0.292 | 10.53 | 1379.3 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.203 | 0.976 | 2.15 | 1355.8 |
| stage/render/extract_commands | 0.187 | 0.224 | 0.53 | 1244.5 |
| anim/evaluate/ms | 0.166 | 0.316 | 1.44 | 1105.2 |
| bigspace/high_precision/ms | 0.162 | 0.657 | 3.24 | 1083.1 |
| stage/render/specialize | 0.155 | 0.242 | 2.06 | 1033.5 |
| anim/evaluate_cpu/ms | 0.154 | 0.196 | 0.48 | 1025.1 |
| stream/tick/ms | 0.153 | 0.223 | 0.63 | 1020.7 |
| stage/render/prepare_views | 0.135 | 0.181 | 0.59 | 900.6 |
| pipelines/compile_ms | 0.132 | 0.000 | 137.67 | 878.1 |
| pipelines/front_ms | 0.128 | 0.000 | 134.30 | 855.0 |
| stage/render/queue | 0.097 | 0.165 | 1.41 | 643.7 |
| render/acquire/ms | 0.092 | 0.125 | 0.53 | 612.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.090 | 0.144 | 0.46 | 601.8 |
| stage/render/phase_sort | 0.079 | 0.110 | 0.38 | 526.9 |
| controller/write/ms | 0.073 | 0.098 | 0.46 | 489.2 |
| controller/move/ms | 0.064 | 0.160 | 0.46 | 428.7 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.061 | 0.226 | 1.30 | 406.8 |
| terrain/update/ms | 0.057 | 0.282 | 1.77 | 383.1 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.056 | 0.423 | 2.60 | 373.5 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.34 | 310.4 |
| physics/step/ms | 0.041 | 0.089 | 0.64 | 274.1 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.32 | 207.3 |
| physics/insert/ms | 0.025 | 0.270 | 0.86 | 163.9 |
| stage/render/cleanup | 0.023 | 0.035 | 0.50 | 153.8 |
| terrain/select/ms | 0.021 | 0.250 | 0.64 | 138.8 |
| weather/map/ms | 0.017 | 0.023 | 0.20 | 112.8 |
| far/select/ms | 0.015 | 0.166 | 0.36 | 100.2 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.35 | 83.5 |
| anim/apply/ms | 0.011 | 0.021 | 0.13 | 76.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.19 | 70.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.17 | 69.2 |
| script/update/ms | 0.008 | 0.012 | 0.24 | 53.6 |
| controller/avoid/ms | 0.006 | 0.012 | 0.08 | 39.4 |
| stream/despawn/ms | 0.006 | 0.000 | 1.24 | 36.9 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 33.2 |
| script/ui/ms | 0.003 | 0.005 | 0.10 | 23.3 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.37 | 23.1 |
| stream/spawn/ms | 0.003 | 0.000 | 1.12 | 22.0 |
| stream/upload/ms | 0.003 | 0.000 | 1.42 | 20.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 14.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.10 | 2.7 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
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
| anim/evaluated/count | count | 18 | 32 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
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
| crowd/joint_blocks/count | count | 160 | 215 | 204 |
| crowd/joints_extracted/count | count | 657 | 7052 | 546 |
| crowd/joints_uploaded/count | count | 3598 | 6186 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.39 | 1.29 | 0.52 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.35 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.19 | 0.01 |
| encode/finish_critical | ms | 0.95 | 5.97 | 0.98 |
| encode/pass:early prepass/max | ms | 0.80 | 2.45 | 0.98 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 5.97 | 0.71 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.46 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.92 | 0.25 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.66 | 2.15 | 0.37 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.15 | 1.30 | 0.12 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 0.84 | 2.60 | 0.26 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.11 | 0.73 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.32 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.41 | 26.50 | 8.70 |
| gpu/timed/ms | ms | 7.88 | 9.60 | 8.70 |
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
| phases/opaque3d/bins | count | 767 | 973 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2480 | 3554 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 767 | 973 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2480 | 3554 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 24 | 178 | 8 |
| phases/shadow/bins | count | 743 | 1022 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2152 | 2701 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 28 | 84 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2679 | 3183 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 14 | 265 | 11 |
| preprocess/bind_groups_created | count | 9 | 255 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 128 | 206 | 123 |
| process/cpu_usage | % | 25.59 | 44.50 | 34.88 |
| process/mem_usage | GiB | 5.12 | 5.41 | 5.39 |
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
| shadow_atlas/alloc_failures | count | 893 | 893 | 893 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 2 | 0 |
| shadow_atlas/lamps | count | 8 | 19 | 0 |
| shadow_atlas/live_occ_pct | % | 30.86 | 100 | 0 |
| shadow_atlas/live_tiles | count | 17 | 58 | 0 |
| shadow_atlas/mover_draws | count | 9 | 12 | 0 |
| shadow_atlas/movers | count | 192 | 544 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 2 | 0 |
| shadow_atlas/render/blits | count | 9 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 19 | 0 |
| shadow_atlas/render/live_direct | count | 9 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 61 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 33.98 | 0 |
| shadow_atlas/static_tiles | count | 42 | 112 | 0 |
| shadow_atlas/views | count | 17 | 61 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 13 | 124 | 11 |
| skin/blocks_skipped/count | count | 146 | 205 | 193 |
| skin/blocks_walked/count | count | 13 | 25 | 11 |
| skin/joint_bytes_written/bytes | bytes | 46.6 k | 811.1 k | 36.1 k |
| skin/joints_written/count | count | 728 | 12.7 k | 564 |
| skin/upload_writes/count | count | 7 | 24 | 10 |
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
| system/cpu_usage | % | 31.62 | 45.26 | 35.39 |
| system/mem_usage | % | 55.43 | 56.48 | 56.40 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 81 | 233 | 66 |
| views/all/bins | count | 2859 | 3161 | 2948 |
| views/all/count | count | 20 | 64 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 7970 | 9912 | 7956 |
| views/all/pipelines | count | 76 | 133 | 66 |
| views/all/pipelines_distinct | count | 64 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 231.1 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 713 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2121 | 2427 | 2014 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 146 | 0 |
| views/lamp_face/bins | count | 5 | 271 | 0 |
| views/lamp_face/count | count | 17 | 61 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 55 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 12 | 351 | 0 |
| views/lamp_face/pipelines | count | 4 | 61 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 24.9 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7528 | 5942 |
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
| anim/bones_written/count | count | 48.82 | 231 | 325.5 k |
| anim/events/count | count | 0.07 | 3 | 472 |
| assets/Image/added | count | 0.09 | 36 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 12 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 246 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7477 |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1218 | 1489 |
| encode//count | count | 27.90 | 30 | 186.0 k |
| encode/bind_walk/init_textures | count | 8829 | 22.7 k | 58.85 M |
| encode/bind_walk/init_textures_skipped | count | 51.7 k | 78.4 k | 344.89 M |
| encode/bind_walk/inits | count | 551 | 766 | 3.67 M |
| encode/bind_walk/inits_skipped | count | 142 | 234 | 948.0 k |
| encode/bind_walk/merge_textures | count | 8766 | 22.7 k | 58.44 M |
| encode/bind_walk/merge_textures_skipped | count | 11.0 k | 26.5 k | 73.48 M |
| encode/bind_walk/merges | count | 102 | 166 | 676.6 k |
| encode/bind_walk/merges_skipped | count | 130 | 228 | 869.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6666 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6666 |
| encode/pass:early prepass/count | count | 1 | 1 | 6666 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6666 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6666 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6666 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.28 | 1 | 1872 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.23 | 1 | 1542 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.07 | 1 | 436 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6666 |
| encode/queue/command_buffers | count | 38.49 | 42 | 256.6 k |
| encode/queue/submits | count | 2.01 | 4 | 13.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.30 M | 15.44 G |
| encode/queue/write_buffer/calls | count | 262 | 425 | 1.74 M |
| encode/queue/write_buffer_with/bytes | bytes | 161.8 k | 1.29 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.28 | 747 | 68.5 k |
| encode/queue/write_texture/bytes | bytes | 2068 | 1.05 M | 13.78 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 558 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6666 |
| physics/inserts | count | 0.34 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
| physics/steps | count | 0.59 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 9 |
| pipelines/created | count | 0.00 | 1 | 10 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 28 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5078 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1113 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2494.8 | main/busy/ms 2462, stage/main/post_update 2421, stage/main/update 30.70, stage/main/pre_update 6.59, stage/main/last 1.19, surface/advance_cpu_ms 0.74, stage/main/first 0.51, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.30, stream/tick/ms 0.10, bigspace/total/ms 0.09 |
| warm-up | 3.29 | 792.2 | main/busy/ms 2599, stage/main/post_update 2446, render/thread/ms 741, render/busy/ms 607, stage/render/prepare_assets 554, stage/main/update 137, stage/render/prepare_views 132, render/acquire/ms 132, stage/render/create_views 29.48, stage/render/render 13.16, render/graph/ms 9.67, stage/main/pre_update 8.34 |
| warm-up | 3.35 | 62.2 | render/thread/ms 787, render/busy/ms 650, stage/render/prepare_assets 584, main/busy/ms 160, stage/render/prepare_views 133, render/acquire/ms 133, stage/main/update 113, stage/main/post_update 34.69, stage/render/create_views 29.84, pipelines/compile_ms 27.39, pipelines/front_ms 26.29, stage/render/render 18.70 |
| warm-up | 3.38 | 34.2 | pipelines/wait_ms 182, pipelines/compile_ms 169, pipelines/front_ms 157, main/busy/ms 95.54, render/thread/ms 77.80, render/busy/ms 71.19, stage/main/post_update 51.33, pipelines/lock_ms 32.75, stage/render/render 31.47, stage/render/prepare_assets 30.80, stage/main/update 26.77, render/graph/ms 24.63 |
| warm-up | 3.55 | 162.5 | pipelines/wait_ms 4044, pipelines/compile_ms 1519, pipelines/front_ms 1424, render/thread/ms 147, render/busy/ms 141, main/busy/ms 125, pipelines/driver_ms 95.66, stage/main/post_update 51.60, stage/render/render 49.94, render/graph/ms 43.42, stage/render/prepare_resources 33.96, pipelines/lock_ms 32.62 |
| warm-up | 3.61 | 66.3 | pipelines/wait_ms 4240, pipelines/compile_ms 2045, pipelines/front_ms 1928, render/thread/ms 176, render/busy/ms 174, main/busy/ms 142, pipelines/driver_ms 117, stage/main/update 64.60, stage/render/prepare_resources 46.32, stage/render/render 40.90, render/graph/ms 38.54, stage/render/prepare_assets 37.54 |
| warm-up | 3.73 | 121.7 | pipelines/compile_ms 2284, pipelines/front_ms 2172, pipelines/wait_ms 703, render/thread/ms 149, render/busy/ms 147, pipelines/driver_ms 112, main/busy/ms 105, stage/render/prepare_assets 85.97, stage/main/update 66.13, stage/render/render 31.96, render/graph/ms 29.47, encode//sum 26.26 |
| warm-up | 3.78 | 50.3 | pipelines/compile_ms 1972, pipelines/front_ms 1879, pipelines/wait_ms 377, render/thread/ms 136, render/busy/ms 133, stage/render/prepare_assets 104, pipelines/driver_ms 92.97, main/busy/ms 31.44, stage/render/render 23.64, render/graph/ms 20.27, encode//sum 18.93, stage/main/post_update 14.19 |
| warm-up | 3.83 | 27.7 | pipelines/compile_ms 697, pipelines/front_ms 659, pipelines/wait_ms 76.98, render/thread/ms 47.84, render/busy/ms 46.07, main/busy/ms 40.50, pipelines/driver_ms 37.95, stage/main/post_update 19.63, stage/main/update 15.13, stage/render/render 14.86, render/graph/ms 12.60, stage/render/graph_submit 10.01 |
| warm-up | 3.99 | 138.9 | pipelines/compile_ms 1326, pipelines/front_ms 1249, main/busy/ms 81.00, pipelines/driver_ms 77.39, render/thread/ms 65.53, render/busy/ms 61.24, stage/main/update 59.22, pipelines/wait_ms 39.28, stage/render/prepare_assets 38.60, stage/render/render 19.75, render/graph/ms 15.32, stage/render/graph_submit 12.75 |
| warm-up | 4.04 | 44.7 | pipelines/compile_ms 2046, pipelines/front_ms 1950, pipelines/driver_ms 95.65, render/thread/ms 76.02, render/busy/ms 72.61, stage/render/prepare_assets 52.84, pipelines/wait_ms 38.95, main/busy/ms 21.27, stage/render/render 17.18, render/graph/ms 13.52, stage/render/graph_submit 10.90, stage/main/update 8.78 |
| warm-up | 4.10 | 25.8 | render/thread/ms 30.59, render/busy/ms 29.07, main/busy/ms 27.20, pipelines/compile_ms 23.32, pipelines/front_ms 22.44, stage/main/update 15.14, stage/render/render 14.37, render/graph/ms 12.67, encode//sum 8.95, stage/main/post_update 8.92, stage/render/graph_submit 8.63, encode/submit 5.51 |
| warm-up | 4.13 | 29.9 | main/busy/ms 121, stage/main/update 97.54, pipelines/compile_ms 44.51, render/thread/ms 43.46, pipelines/front_ms 42.64, render/busy/ms 41.89, stage/render/render 14.88, render/graph/ms 12.77, stage/render/prepare_assets 12.66, encode//sum 10.37, stage/main/post_update 10.22, stage/main/pre_update 10.20 |
| warm-up | 4.28 | 153.0 | pipelines/compile_ms 1744, pipelines/front_ms 1685, main/busy/ms 123, stage/main/update 96.57, render/thread/ms 62.70, render/busy/ms 61.27, pipelines/driver_ms 58.68, stage/render/prepare_assets 43.04, stage/main/pre_update 10.81, stage/render/render 10.57, pipelines/wait_ms 10.25, stage/main/post_update 10.07 |
| warm-up | 4.33 | 48.6 | pipelines/compile_ms 1722, pipelines/front_ms 1665, render/thread/ms 68.45, render/busy/ms 67.14, pipelines/driver_ms 57.69, stage/render/prepare_assets 54.00, main/busy/ms 35.61, stage/main/update 16.76, pipelines/wait_ms 10.18, stage/main/post_update 9.70, stage/render/render 8.89, render/graph/ms 7.34 |
| warm-up | 4.43 | 104.1 | render/thread/ms 126, render/busy/ms 124, pipelines/compile_ms 112, pipelines/front_ms 97.18, main/busy/ms 77.69, stage/main/update 61.86, stage/render/prepare_meshes 45.18, stage/render/render 40.73, render/graph/ms 38.08, stage/render/graph_submit 35.50, stage/render/prepare_assets 33.23, pipelines/wait_ms 29.77 |
| warm-up | 4.50 | 68.2 | pipelines/compile_ms 707, pipelines/front_ms 660, pipelines/wait_ms 167, render/thread/ms 142, render/busy/ms 140, main/busy/ms 79.77, stage/main/update 62.00, pipelines/driver_ms 47.15, stage/render/prepare_meshes 46.32, stage/render/render 43.43, render/graph/ms 40.33, stage/render/prepare_assets 39.87 |
| warm-up | 4.56 | 54.5 | pipelines/compile_ms 1071, pipelines/front_ms 1000, pipelines/wait_ms 162, render/thread/ms 95.78, render/busy/ms 94.21, pipelines/driver_ms 70.84, main/busy/ms 54.96, stage/main/update 35.54, stage/render/prepare_assets 30.30, stage/render/prepare_bind_groups 19.07, stage/render/render 14.74, stage/render/prepare_write_phase_buffers 13.32 |
| warm-up | 4.64 | 80.7 | pipelines/compile_ms 1366, pipelines/front_ms 1297, render/thread/ms 116, pipelines/wait_ms 115, render/busy/ms 113, main/busy/ms 92.36, pipelines/driver_ms 68.55, stage/main/update 33.55, stage/main/pre_update 25.39, stage/render/prepare_flush 23.29, stage/main/post_update 22.62, stage/render/prepare_assets 18.83 |
| warm-up | 4.74 | 97.9 | pipelines/compile_ms 2475, pipelines/front_ms 2375, pipelines/wait_ms 520, render/thread/ms 149, render/busy/ms 147, main/busy/ms 134, pipelines/driver_ms 100, stage/main/post_update 66.15, stage/render/prepare_assets 38.11, stage/render/prepare_resources 37.48, stage/main/update 32.05, stage/render/prepare_write_phase_buffers 26.52 |
| warm-up | 4.82 | 80.2 | pipelines/compile_ms 2348, pipelines/front_ms 2247, pipelines/wait_ms 520, render/thread/ms 141, render/busy/ms 139, main/busy/ms 136, pipelines/driver_ms 101, stage/main/update 66.54, stage/main/post_update 54.32, stage/render/prepare_assets 50.14, stage/render/prepare_resources 29.12, stage/render/prepare_write_phase_buffers 25.99 |
| warm-up | 4.88 | 69.1 | pipelines/compile_ms 2005, pipelines/front_ms 1944, pipelines/wait_ms 489, render/thread/ms 93.33, render/busy/ms 91.28, main/busy/ms 85.25, pipelines/driver_ms 60.43, stage/main/update 58.49, stage/render/prepare_assets 55.85, stage/render/render 21.05, render/graph/ms 18.17, stage/render/graph_submit 14.54 |
| warm-up | 4.95 | 63.0 | pipelines/compile_ms 1586, pipelines/front_ms 1548, pipelines/wait_ms 413, render/thread/ms 81.49, render/busy/ms 80.12, main/busy/ms 55.44, stage/render/prepare_assets 50.33, pipelines/driver_ms 38.46, stage/main/update 35.53, stage/render/render 11.88, render/graph/ms 10.08, stage/main/post_update 9.05 |
| warm-up | 4.99 | 39.2 | pipelines/compile_ms 433, pipelines/front_ms 421, main/busy/ms 88.33, render/thread/ms 77.91, render/busy/ms 76.51, stage/main/update 61.18, pipelines/wait_ms 37.25, stream/drive/ms 34.82, stage/render/prepare_assets 34.17, stage/render/render 15.52, render/graph/ms 13.58, stage/main/pre_update 13.42 |
| warm-up | 5.05 | 63.0 | pipelines/compile_ms 520, pipelines/front_ms 505, main/busy/ms 105, stage/main/update 80.09, render/thread/ms 64.16, render/busy/ms 62.74, stream/drive/ms 59.05, pipelines/wait_ms 22.91, stage/render/prepare_assets 21.88, stage/render/render 16.94, pipelines/driver_ms 15.59, render/graph/ms 14.83 |
| warm-up | 5.09 | 42.1 | pipelines/compile_ms 1780, pipelines/front_ms 1735, main/busy/ms 76.68, stage/main/update 55.97, render/thread/ms 52.62, render/busy/ms 51.27, pipelines/driver_ms 45.12, stream/drive/ms 37.60, stage/render/render 19.09, render/graph/ms 17.04, stage/render/prepare_assets 14.00, stage/render/graph_submit 12.72 |
| warm-up | 5.13 | 41.5 | pipelines/compile_ms 1430, pipelines/front_ms 1392, main/busy/ms 68.64, render/thread/ms 53.11, render/busy/ms 51.53, stage/main/update 46.58, pipelines/driver_ms 38.12, stage/render/render 22.19, render/graph/ms 19.75, stream/drive/ms 14.76, stage/render/graph_submit 13.53, stage/main/post_update 12.79 |
| warm-up | 5.18 | 46.8 | pipelines/compile_ms 499, pipelines/front_ms 476, main/busy/ms 56.71, render/thread/ms 56.46, render/busy/ms 54.90, stage/main/update 39.30, pipelines/driver_ms 22.31, stage/render/prepare_assets 18.21, pipelines/wait_ms 17.03, stage/render/render 15.90, render/graph/ms 13.59, stage/main/post_update 9.85 |
| warm-up | 5.21 | 33.0 | pipelines/compile_ms 657, pipelines/front_ms 633, render/thread/ms 59.14, render/busy/ms 57.80, main/busy/ms 48.10, stage/main/update 25.97, pipelines/driver_ms 24.97, stage/render/prepare_assets 17.21, stage/main/post_update 14.24, pipelines/wait_ms 13.19, stage/render/render 12.58, render/graph/ms 10.62 |
| warm-up | 5.35 | 25.0 | pipelines/compile_ms 82.92, pipelines/front_ms 78.83, main/busy/ms 58.67, render/thread/ms 38.80, render/busy/ms 36.14, stage/main/update 32.10, stream/drive/ms 22.32, stage/main/post_update 18.99, stage/render/render 16.76, render/graph/ms 13.38, stage/render/graph_submit 9.19, stage/render/prepare_assets 8.50 |
| warm-up | 5.39 | 41.8 | pipelines/compile_ms 280, pipelines/front_ms 269, main/busy/ms 54.36, render/thread/ms 38.41, render/busy/ms 36.97, stage/main/update 32.29, stream/drive/ms 22.22, stage/main/post_update 16.36, stage/render/render 13.92, stage/render/prepare_assets 11.87, render/graph/ms 11.69, pipelines/driver_ms 10.37 |
| warm-up | 5.90 | 27.4 | main/busy/ms 31.07, render/thread/ms 22.22, render/busy/ms 21.34, stage/main/update 20.28, stream/drive/ms 12.33, stage/render/render 8.20, stage/main/post_update 7.18, render/graph/ms 6.82, encode//sum 5.95, stage/render/prepare_assets 5.91, stage/render/graph_submit 4.70, encode/submit 2.94 |
| warm-up | 6.03 | 29.1 | render/thread/ms 42.66, render/busy/ms 41.27, main/busy/ms 31.07, stage/render/prepare_assets 18.05, stage/main/update 15.68, stage/render/render 11.41, render/graph/ms 9.49, stage/main/post_update 8.75, encode//sum 7.44, stage/render/graph_submit 5.78, stream/drive/ms 4.66, stage/render/camera_driver 3.51 |
| warm-up | 6.15 | 26.2 | pipelines/compile_ms 135, pipelines/front_ms 132, render/thread/ms 34.78, main/busy/ms 33.84, render/busy/ms 33.77, stage/main/update 21.42, stage/render/render 10.98, stage/render/prepare_assets 10.06, render/graph/ms 9.57, stage/main/post_update 7.06, encode//sum 6.69, stage/render/graph_submit 6.54 |
| warm-up | 6.21 | 28.8 | render/thread/ms 37.84, render/busy/ms 36.61, main/busy/ms 31.71, stage/main/update 19.52, stage/render/render 14.83, render/graph/ms 13.09, stage/render/prepare_assets 8.98, encode//sum 8.88, stage/render/graph_submit 8.77, stage/main/post_update 7.95, encode/submit 6.05, stage/render/prepare_meshes 4.51 |
| warm-up | 6.25 | 25.5 | main/busy/ms 43.70, render/thread/ms 35.59, render/busy/ms 33.92, stage/main/update 25.75, stage/render/render 15.62, render/graph/ms 13.55, stage/main/post_update 12.01, encode//sum 9.81, stage/render/graph_submit 9.54, encode/submit 6.34, stage/render/prepare_assets 5.06, stream/drive/ms 3.90 |
| warm-up | 6.29 | 35.4 | render/thread/ms 43.20, main/busy/ms 42.08, render/busy/ms 41.68, stage/main/update 22.64, stage/render/render 13.86, stage/main/post_update 13.15, stage/render/prepare_assets 12.61, render/graph/ms 11.84, encode//sum 8.88, stage/render/graph_submit 8.00, encode/submit 4.89, stage/render/prepare_meshes 4.24 |
| warm-up | 6.32 | 29.4 | pipelines/compile_ms 140, pipelines/front_ms 137, render/thread/ms 47.08, render/busy/ms 46.02, main/busy/ms 38.56, stage/main/update 20.24, stage/render/prepare_assets 16.82, stage/main/post_update 11.25, stage/render/render 8.85, render/graph/ms 7.32, encode//sum 6.76, stage/render/prepare_meshes 6.03 |
| warm-up | 6.36 | 37.2 | pipelines/compile_ms 140, pipelines/front_ms 137, render/thread/ms 57.67, render/busy/ms 56.17, main/busy/ms 46.02, stage/main/update 25.28, stage/render/prepare_assets 16.46, stage/render/render 13.31, stage/main/post_update 13.04, render/graph/ms 10.88, encode//sum 9.71, stage/render/prepare_meshes 9.18 |
| warm-up | 6.41 | 28.6 | render/thread/ms 34.06, render/busy/ms 33.08, main/busy/ms 30.72, stage/main/update 15.54, stage/render/prepare_assets 13.74, pipelines/compile_ms 13.55, pipelines/front_ms 12.63, stage/main/post_update 8.87, stage/render/render 8.34, render/graph/ms 6.84, encode//sum 6.22, stage/render/graph_submit 4.21 |
| warm-up | 6.55 | 36.6 | pipelines/compile_ms 131, pipelines/front_ms 128, main/busy/ms 48.29, render/thread/ms 35.85, render/busy/ms 34.36, stage/main/update 32.63, stream/drive/ms 14.09, stage/render/render 12.94, render/graph/ms 10.93, stage/main/post_update 10.54, encode//sum 9.03, stage/render/graph_submit 7.44 |
| warm-up | 6.58 | 34.7 | render/thread/ms 51.82, main/busy/ms 51.61, render/busy/ms 50.29, stage/main/update 30.81, stage/render/render 17.93, render/graph/ms 15.52, stage/main/post_update 14.55, pipelines/compile_ms 12.78, stage/render/prepare_assets 12.71, pipelines/front_ms 11.80, encode//sum 11.24, stage/render/graph_submit 10.52 |
| warm-up | 6.61 | 27.2 | render/thread/ms 49.04, render/busy/ms 47.71, main/busy/ms 43.89, stage/main/update 25.03, stage/render/prepare_assets 16.63, stage/render/render 15.77, render/graph/ms 13.61, stage/main/post_update 13.24, pipelines/compile_ms 12.78, pipelines/front_ms 11.80, encode//sum 10.18, stage/render/graph_submit 9.37 |
| warm-up | 6.82 | 25.3 | main/busy/ms 33.32, render/thread/ms 28.56, render/busy/ms 27.65, stage/main/update 17.80, stage/render/render 9.82, stage/main/post_update 9.28, render/graph/ms 8.41, encode//sum 7.79, stage/render/prepare_meshes 5.89, stage/render/graph_submit 5.34, stream/drive/ms 5.24, stage/render/prepare_assets 3.82 |
| measure | 18.35 | 29.1 | main/busy/ms 35.81, stage/main/update 22.86, render/thread/ms 20.67, render/busy/ms 19.73, stage/render/render 9.68, stage/main/post_update 8.97, render/graph/ms 8.47, encode//sum 6.32, stage/render/graph_submit 6.19, stage/render/prepare_assets 4.34, encode/submit 3.95, encode/finish_wall 2.26 |
| measure | 59.04 | 25.4 | render/thread/ms 39.42, render/busy/ms 37.83, main/busy/ms 36.95, stage/main/post_update 20.76, stage/render/render 19.69, render/graph/ms 17.41, encode//sum 12.42, stage/render/graph_submit 11.30, stage/main/update 8.48, stage/render/prepare_assets 7.91, encode/submit 6.60, stage/render/camera_driver 5.84 |
| measure | 59.07 | 30.1 | render/thread/ms 40.02, render/busy/ms 38.91, main/busy/ms 27.95, stage/render/prepare_assets 17.48, stage/main/post_update 13.83, stage/render/render 12.22, render/graph/ms 10.51, encode//sum 8.62, stage/render/graph_submit 6.39, stage/main/update 5.50, stage/render/camera_driver 3.95, stage/main/fixed_main_loop 3.70 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=atlas48-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1536 pmon samples)
- focus: clean
- nvidia-smi over the window (254 samples): GPU load **87%** (min 59%) · 2625 MHz (min 2625) · memory 8101 MHz · 83.7 W · 75 °C (max 80) · VRAM 6467 MiB · throttled 100% of samples

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
