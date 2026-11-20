# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5870 frames over 65.6 s · **median 10.87 ms (92 fps)** · mean 11.18 · p95 14.24 · p99 15.95 (1% low 63 fps) · worst 33.07 ms · 43 spikes over 1.5× median · 15.5 stutters/min (work 14.6/min)
- warm-up: 355 frames over 9.4 s, worst 2499.9 ms
- hitches (> 27.2 ms): **2** measured, 40 in warm-up
- timed GPU passes: 6.80 ms of a 11.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.87 ms, limit 11 ms
- **FAIL** frame_ms: 17 stutters (> 1.5x moving median of 60) in 1.09 min = 15.54/min, limit 1/min
- **FAIL** worst frame 33.07 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.922 | 2.034 | 2.067 | 0.94 | 0.177 |
| main_opaque_pass_3d | 1.211 | 1.282 | 1.570 | 0.94 | 0.069 |
| auxiliary_views | 0.565 | 0.598 | 1.574 | 0.94 | 0.489 |
| early prepass | 0.490 | 0.519 | 0.730 | 0.94 | 0.091 |
| bloom | 0.300 | 0.317 | 0.323 | 0.94 | 0.065 |
| atmosphere_luts | 0.295 | 0.312 | 0.374 | 0.94 | 0.003 |
| water_velocity | 0.213 | 0.226 | 0.246 | 0.94 | 0.002 |
| water_sspr | 0.191 | 0.202 | 0.203 | 0.94 | 0.002 |
| ssao | 0.178 | 0.188 | 0.191 | 0.94 | 0.007 |
| upscaling | 0.122 | 0.130 | 0.137 | 0.94 | 0.004 |
| atmos_froxel_inject | 0.121 | 0.128 | 0.129 | 0.94 | 0.001 |
| water_fft | 0.116 | 0.123 | 0.126 | 0.94 | 0.003 |
| uniform_allocation | 0.112 | 0.118 | 0.145 | 0.94 | 0.022 |
| clouds_composite | 0.099 | 0.105 | 0.108 | 0.94 | 0.002 |
| gpu_instance_materialization | 0.092 | 0.097 | 0.101 | 0.94 | 0.004 |
| bin_unpacking | 0.090 | 0.095 | 0.121 | 0.94 | 0.017 |
| tonemapping | 0.082 | 0.086 | 0.091 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.068 | 0.072 | 0.139 | 0.94 | 0.008 |
| clustering | 0.060 | 0.063 | 0.067 | 0.94 | 0.018 |
| clouds_trace | 0.053 | 0.056 | 0.077 | 0.94 | 0.001 |
| early_mesh_preprocessing | 0.053 | 0.056 | 0.074 | 0.94 | 0.004 |
| render_sky | 0.050 | 0.053 | 0.054 | 0.94 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.94 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.68 | 0.002 |
| lightprobe_irradiance_map | 0.036 | 0.414 | 0.416 | 0.09 | 0.001 |
| atmos_froxel_integrate | 0.029 | 0.031 | 0.033 | 0.94 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.94 | 0.002 |
| range_unpacking | 0.022 | 0.024 | 0.031 | 0.94 | 0.003 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.94 | 0.002 |
| lightprobe_radiance_map | 0.019 | 0.220 | 0.232 | 0.09 | 0.004 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.94 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.039 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 0.94 | 0.198 |
| clouds_fog_light | 0.009 | 0.010 | 0.010 | 0.94 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.94 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.868 | 12.863 | 19.79 | 57927.0 |
| render/busy/ms | 9.347 | 12.347 | 19.25 | 54864.9 |
| main/busy/ms | 8.331 | 10.005 | 30.56 | 48900.5 |
| stage/render/render | 6.294 | 8.697 | 11.84 | 36944.9 |
| render/graph/ms | 5.618 | 7.984 | 10.97 | 32977.3 |
| encode//sum | 5.042 | 7.923 | 15.04 | 29593.8 |
| stage/main/post_update | 4.767 | 5.874 | 8.95 | 27981.0 |
| stage/render/graph_submit | 3.973 | 5.624 | 8.59 | 23321.2 |
| encode/submit | 2.636 | 3.898 | 7.09 | 15474.5 |
| stage/main/update | 1.836 | 2.377 | 23.90 | 10779.6 |
| stage/render/camera_driver | 1.589 | 2.384 | 4.27 | 9328.3 |
| encode/finish_wall | 1.402 | 1.883 | 3.89 | 8228.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.812 | 1.102 | 2.00 | 4767.7 |
| encode/pass:shadow_directional_light/sum | 0.803 | 1.245 | 2.00 | 4714.5 |
| encode/pass:early prepass/sum | 0.784 | 1.078 | 2.38 | 4602.6 |
| stage/render/prepare_write_phase_buffers | 0.648 | 1.038 | 1.90 | 3803.5 |
| encode/pass:shadow_point_light/sum | 0.570 | 2.231 | 4.07 | 3344.6 |
| stage/render/prepare_assets | 0.565 | 0.684 | 9.50 | 3319.0 |
| stage/main/fixed_main_loop | 0.549 | 0.925 | 1.66 | 3225.5 |
| bigspace/total/ms | 0.492 | 1.138 | 2.24 | 2886.5 |
| render/present/ms | 0.435 | 0.635 | 2.94 | 2552.5 |
| stage/render/prepare_resources | 0.428 | 0.520 | 0.93 | 2514.0 |
| stage/render/prepare_meshes | 0.428 | 0.534 | 4.24 | 2513.9 |
| stage/main/pre_update | 0.384 | 0.490 | 2.95 | 2252.1 |
| stage/main/first | 0.352 | 0.455 | 5.59 | 2065.2 |
| stage/render/prepare_bind_groups | 0.326 | 0.557 | 1.61 | 1915.2 |
| controller/feed/ms | 0.323 | 0.405 | 0.71 | 1897.2 |
| stage/render/prepare_flush | 0.293 | 0.519 | 2.70 | 1721.0 |
| stage/render/prepare_batch_phases | 0.282 | 0.419 | 2.85 | 1652.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.277 | 0.431 | 1.10 | 1624.6 |
| stage/main/last | 0.259 | 0.308 | 1.42 | 1520.4 |
| bigspace/low_precision/ms | 0.204 | 0.451 | 1.85 | 1195.4 |
| stream/drive/ms | 0.202 | 0.293 | 10.86 | 1187.2 |
| stage/render/create_views | 0.194 | 0.259 | 0.81 | 1140.2 |
| stage/render/extract_commands | 0.172 | 0.204 | 0.60 | 1008.5 |
| anim/evaluate_cpu/ms | 0.154 | 0.189 | 0.43 | 903.6 |
| anim/evaluate/ms | 0.152 | 0.223 | 1.77 | 892.0 |
| stage/render/queue | 0.147 | 0.223 | 1.48 | 860.7 |
| stream/tick/ms | 0.146 | 0.222 | 0.42 | 858.7 |
| stage/render/specialize | 0.141 | 0.233 | 1.61 | 827.7 |
| bigspace/high_precision/ms | 0.140 | 0.571 | 1.89 | 820.8 |
| stage/render/prepare_views | 0.132 | 0.174 | 0.34 | 775.0 |
| pipelines/compile_ms | 0.125 | 0.000 | 133.50 | 731.6 |
| pipelines/front_ms | 0.121 | 0.000 | 130.89 | 712.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.095 | 0.155 | 0.41 | 558.9 |
| render/acquire/ms | 0.087 | 0.114 | 0.29 | 509.6 |
| controller/move/ms | 0.075 | 0.185 | 0.49 | 441.4 |
| controller/write/ms | 0.073 | 0.100 | 0.29 | 431.4 |
| terrain/update/ms | 0.061 | 0.296 | 2.02 | 358.2 |
| physics/step/ms | 0.045 | 0.086 | 0.67 | 267.0 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.071 | 0.31 | 263.0 |
| stage/render/phase_sort | 0.038 | 0.065 | 0.22 | 225.3 |
| encode/point_shadow_cache_copies/sum | 0.032 | 0.087 | 0.32 | 185.2 |
| encode/resolve_diagnostics/sum | 0.030 | 0.042 | 0.50 | 178.7 |
| physics/insert/ms | 0.030 | 0.356 | 0.87 | 173.8 |
| terrain/select/ms | 0.024 | 0.262 | 0.51 | 142.3 |
| stage/render/cleanup | 0.024 | 0.033 | 0.65 | 139.6 |
| weather/map/ms | 0.017 | 0.024 | 0.20 | 98.0 |
| far/select/ms | 0.016 | 0.174 | 0.50 | 95.6 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.22 | 73.9 |
| anim/apply/ms | 0.011 | 0.021 | 0.19 | 65.4 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.10 | 61.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.14 | 60.9 |
| script/update/ms | 0.008 | 0.011 | 0.03 | 45.4 |
| controller/avoid/ms | 0.007 | 0.013 | 0.20 | 41.0 |
| stream/despawn/ms | 0.006 | 0.000 | 1.05 | 36.0 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 27.8 |
| script/ui/ms | 0.004 | 0.006 | 0.11 | 25.2 |
| stream/spawn/ms | 0.004 | 0.000 | 1.11 | 21.3 |
| stream/upload/ms | 0.003 | 0.000 | 1.32 | 20.1 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.19 | 19.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 13.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 31 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1277 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 148 | 156 | 149 |
| crowd/c1/joints/count | count | 5590 | 5991 | 5661 |
| crowd/c1/meshes/count | count | 1177 | 1295 | 1197 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 206 |
| crowd/joints_extracted/count | count | 695 | 6834 | 671 |
| crowd/joints_uploaded/count | count | 3649 | 6220 | 5807 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.1 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.43 | 1.40 | 0.47 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.22 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.10 | 0.01 |
| encode/finish_critical | ms | 0.97 | 2.38 | 0.95 |
| encode/pass:early prepass/max | ms | 0.79 | 2.38 | 0.84 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 2.00 | 0.95 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.41 | 0.16 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 1.10 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.00 | 0.77 |
| encode/pass:shadow_point_light/max | ms | 0.12 | 1.39 | 0.07 |
| encode/point_shadow_cache_copies/max | ms | 0.06 | 0.32 | 0.06 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.50 | 0.02 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.01 | 30.56 | 7.64 |
| gpu/timed/ms | ms | 7.72 | 9.73 | 7.64 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 9 | 25 | 0 |
| lamp_cache/copies | count | 0 | 16 | 0 |
| lamp_cache/deferred_faces | count | 0 | 16 | 0 |
| lamp_cache/lamps | count | 6 | 8 | 0 |
| lamp_cache/mover_faces | count | 0 | 12 | 0 |
| lamp_cache/movers | count | 100 | 330 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 5 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 16 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 6 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 12 | 32 | 0 |
| lamp_cache/stock_lamps | count | 0 | 3 | 0 |
| lamp_cache/store_starved | count | 0 | 6 | 0 |
| lamp_cache/store_used | count | 8 | 24 | 0 |
| lamp_cache/views | count | 6 | 32 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2607 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 168 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 681 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 393 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 168 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 681 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 393 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 762 | 971 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5843 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2471 | 3527 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 762 | 971 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5843 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2471 | 3527 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 91 | 8 |
| phases/shadow/bins | count | 756 | 1416 | 671 |
| phases/shadow/instance_batches | count | 4 | 10 | 4 |
| phases/shadow/instance_draws | count | 6657 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 106.5 k | 49.2 k |
| phases/shadow/items | count | 2224 | 3152 | 2035 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 45 | 117 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 332 | 333 | 333 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 25 | 297 | 11 |
| preprocess/bind_groups_created | count | 20 | 291 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 8 | 5 |
| preprocess/bind_groups_reused | count | 129 | 232 | 123 |
| process/cpu_usage | % | 24.35 | 36.23 | 29.76 |
| process/mem_usage | GiB | 5.36 | 5.55 | 5.55 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 3092 | 3166 | 3166 |
| scene/batch/masked/live | count | 3092 | 3166 | 3166 |
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
| scene/batch/opaque/high_water | count | 19.2 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 19.2 k | 19.9 k | 19.9 k |
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
| scene/cells/resident | count | 120 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 22.3 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 22.3 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 22.3 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 22.3 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 119 | 14 |
| skin/blocks_skipped/count | count | 148 | 206 | 192 |
| skin/blocks_walked/count | count | 14 | 29 | 14 |
| skin/joint_bytes_written/bytes | bytes | 49.4 k | 810.2 k | 47.4 k |
| skin/joints_written/count | count | 772 | 12.7 k | 740 |
| skin/upload_writes/count | count | 8 | 28 | 9 |
| started_load_count |  | 6464 | 6564 | 6564 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 229 | 333 | 180 |
| statics/materials_bound | count | 229 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 277 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.13 | 46.27 | 30.80 |
| system/mem_usage | % | 56.41 | 57.10 | 57.10 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 85 | 148 | 66 |
| views/all/bins | count | 2937 | 3352 | 2961 |
| views/all/count | count | 13 | 41 | 3 |
| views/all/instance_batches | count | 12 | 18 | 12 |
| views/all/instance_draws | count | 20.0 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 204.8 k | 147.5 k |
| views/all/items | count | 8038 | 10.2 k | 7977 |
| views/all/pipelines | count | 94 | 163 | 66 |
| views/all/pipelines_distinct | count | 66 | 74 | 61 |
| views/all/work_items | count | 154.0 k | 214.4 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 718 | 877 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6657 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2101 | 2439 | 2035 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 6 | 63 | 0 |
| views/lamp_face/bins | count | 49 | 664 | 0 |
| views/lamp_face/count | count | 10 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 12 | 0 |
| views/lamp_face/drawn | count | 2 | 22 | 0 |
| views/lamp_face/instance_batches | count | 0 | 2 | 0 |
| views/lamp_face/instance_draws | count | 0 | 24 | 0 |
| views/lamp_face/instance_slots | count | 0 | 8192 | 0 |
| views/lamp_face/items | count | 75 | 787 | 0 |
| views/lamp_face/pipelines | count | 20 | 91 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 28 | 8635 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2008 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 13.0 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5899 | 7492 | 5942 |
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
| world/meshes | count | 20.9 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 42.01 | 219 | 246.6 k |
| anim/events/count | count | 0.09 | 3 | 541 |
| assets/Image/added | count | 0.10 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.49 | 249 | 2874 |
| assets/Mesh/removed | count | 0.83 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.27 | 10 | 7474 |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 6 |
| ecs/entities_delta | count | 0.25 | 1260 | 1489 |
| encode//count | count | 53.63 | 116 | 314.8 k |
| encode/bind_walk/init_textures | count | 9533 | 24.0 k | 55.96 M |
| encode/bind_walk/init_textures_skipped | count | 52.8 k | 71.2 k | 310.18 M |
| encode/bind_walk/inits | count | 608 | 782 | 3.57 M |
| encode/bind_walk/inits_skipped | count | 141 | 235 | 830.2 k |
| encode/bind_walk/merge_textures | count | 9470 | 23.9 k | 55.59 M |
| encode/bind_walk/merge_textures_skipped | count | 10.7 k | 19.9 k | 62.86 M |
| encode/bind_walk/merges | count | 121 | 200 | 710.9 k |
| encode/bind_walk/merges_skipped | count | 132 | 228 | 774.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5870 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5870 |
| encode/pass:early prepass/count | count | 1 | 1 | 5870 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5870 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5870 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5870 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5870 |
| encode/pass:shadow_point_light/count | count | 6.42 | 22 | 37.7 k |
| encode/point_shadow_cache_copies/count | count | 0.50 | 1 | 2933 |
| encode/queue/command_buffers | count | 70.56 | 149 | 414.2 k |
| encode/queue/submits | count | 2.01 | 4 | 11.8 k |
| encode/queue/write_buffer/bytes | bytes | 2.35 M | 3.32 M | 13.78 G |
| encode/queue/write_buffer/calls | count | 288 | 484 | 1.69 M |
| encode/queue/write_buffer_with/bytes | bytes | 121.8 k | 1.24 M | 714.82 M |
| encode/queue/write_buffer_with/calls | count | 10.45 | 756 | 61.4 k |
| encode/queue/write_texture/bytes | bytes | 2170 | 639.0 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5870 |
| physics/inserts | count | 0.38 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 175 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 1 | 9 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 35 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5767 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1264 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.2 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2499.9 | main/busy/ms 2469, stage/main/post_update 2426, stage/main/update 33.52, stage/main/pre_update 6.02, stage/main/last 1.25, surface/advance_cpu_ms 0.76, stage/main/first 0.47, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.33, stream/drive/ms 0.30, bigspace/total/ms 0.12, bigspace/high_precision/ms 0.09 |
| warm-up | 3.45 | 949.2 | main/busy/ms 2654, stage/main/post_update 2451, render/thread/ms 913, render/busy/ms 831, stage/render/prepare_assets 806, stage/main/update 187, stage/render/prepare_views 79.65, render/acquire/ms 79.45, stage/render/render 10.57, stage/render/create_views 9.02, stage/main/pre_update 7.78, render/graph/ms 6.91 |
| warm-up | 3.51 | 57.8 | render/thread/ms 953, render/busy/ms 870, stage/render/prepare_assets 831, main/busy/ms 206, stage/main/update 159, stage/render/prepare_views 80.26, render/acquire/ms 79.91, pipelines/compile_ms 60.14, pipelines/front_ms 56.72, stage/main/post_update 36.18, terrain/update/ms 19.53, stage/render/render 17.85 |
| warm-up | 3.55 | 45.0 | pipelines/compile_ms 210, pipelines/front_ms 195, pipelines/wait_ms 124, render/thread/ms 83.69, render/busy/ms 80.42, main/busy/ms 64.77, stage/main/post_update 48.47, stage/render/render 45.15, render/graph/ms 41.42, stage/render/camera_driver 28.78, stage/render/prepare_assets 26.31, terrain/update/ms 25.49 |
| warm-up | 3.58 | 29.2 | pipelines/wait_ms 933, pipelines/compile_ms 408, pipelines/front_ms 380, main/busy/ms 112, stage/main/post_update 73.67, render/thread/ms 70.62, render/busy/ms 67.95, stage/render/render 41.30, render/graph/ms 38.42, stage/main/update 33.35, stage/render/camera_driver 27.89, pipelines/driver_ms 27.73 |
| warm-up | 3.71 | 133.2 | pipelines/wait_ms 6038, pipelines/compile_ms 1352, pipelines/front_ms 1277, main/busy/ms 133, render/thread/ms 116, render/busy/ms 115, pipelines/driver_ms 74.17, stage/main/post_update 61.72, stage/main/update 57.67, encode//sum 30.39, stage/render/prepare_resources 26.13, stage/render/render 25.74 |
| warm-up | 3.75 | 35.1 | pipelines/wait_ms 5344, pipelines/compile_ms 1772, pipelines/front_ms 1686, main/busy/ms 149, render/thread/ms 118, render/busy/ms 116, pipelines/driver_ms 86.36, stage/main/update 82.04, encode//sum 36.01, stage/main/post_update 33.12, stage/render/render 30.64, stage/render/prepare_resources 29.82 |
| warm-up | 3.87 | 122.5 | pipelines/compile_ms 1692, pipelines/front_ms 1614, pipelines/wait_ms 271, main/busy/ms 103, render/thread/ms 92.85, render/busy/ms 91.08, pipelines/driver_ms 78.02, stage/render/prepare_assets 63.35, stage/main/update 61.05, stage/main/fixed_main_loop 21.22, stage/main/post_update 17.43, stage/render/render 14.87 |
| warm-up | 3.93 | 37.5 | pipelines/compile_ms 988, pipelines/front_ms 932, pipelines/wait_ms 195, encode//sum 64.45, pipelines/driver_ms 55.77, render/thread/ms 51.11, render/busy/ms 49.53, main/busy/ms 39.58, stage/render/prepare_assets 24.57, stage/main/post_update 18.50, stage/render/render 16.68, render/graph/ms 14.50 |
| warm-up | 4.09 | 108.4 | pipelines/compile_ms 1065, pipelines/front_ms 1022, main/busy/ms 71.47, render/thread/ms 68.60, render/busy/ms 66.77, stage/main/update 56.18, stage/render/prepare_assets 45.64, pipelines/driver_ms 42.75, stage/render/render 13.77, pipelines/wait_ms 12.00, render/graph/ms 11.23, stage/main/post_update 8.17 |
| warm-up | 4.12 | 35.7 | pipelines/compile_ms 1504, pipelines/front_ms 1449, render/thread/ms 66.14, render/busy/ms 64.91, pipelines/driver_ms 54.68, stage/render/prepare_assets 50.09, main/busy/ms 22.37, stage/render/render 10.94, stage/main/update 9.61, render/graph/ms 9.37, pipelines/wait_ms 8.00, stage/render/graph_submit 7.24 |
| warm-up | 4.23 | 37.4 | pipelines/compile_ms 238, pipelines/front_ms 230, render/thread/ms 46.14, render/busy/ms 43.49, main/busy/ms 42.78, stage/main/update 24.16, stage/render/render 14.33, stage/render/prepare_assets 13.96, stage/main/post_update 13.52, render/graph/ms 11.58, stream/drive/ms 11.29, stage/render/graph_submit 8.01 |
| warm-up | 4.29 | 61.1 | pipelines/compile_ms 219, pipelines/front_ms 211, render/thread/ms 71.80, render/busy/ms 69.42, main/busy/ms 62.08, stage/main/update 35.29, stage/render/prepare_assets 34.59, stream/drive/ms 21.29, stage/render/render 17.84, stage/main/post_update 16.71, render/graph/ms 14.65, stage/render/graph_submit 10.86 |
| warm-up | 4.36 | 70.4 | pipelines/compile_ms 1412, pipelines/front_ms 1368, render/thread/ms 79.83, render/busy/ms 78.23, main/busy/ms 60.21, stage/render/prepare_assets 51.72, pipelines/driver_ms 43.04, stage/main/update 34.79, stream/drive/ms 22.33, stage/render/render 16.00, stage/main/post_update 14.03, render/graph/ms 13.77 |
| warm-up | 4.44 | 86.8 | pipelines/compile_ms 1681, pipelines/front_ms 1637, main/busy/ms 144, stage/main/update 127, render/thread/ms 104, render/busy/ms 101, stage/render/prepare_assets 51.81, pipelines/driver_ms 44.42, stage/render/render 26.12, render/graph/ms 23.30, stage/render/prepare_flush 18.97, stage/render/graph_submit 12.03 |
| warm-up | 4.57 | 123.2 | pipelines/compile_ms 1041, pipelines/front_ms 1015, main/busy/ms 153, stage/main/update 136, render/thread/ms 108, render/busy/ms 105, stage/render/prepare_assets 53.63, stage/render/render 26.90, pipelines/driver_ms 25.46, render/graph/ms 23.98, pipelines/wait_ms 21.16, stage/render/prepare_flush 18.97 |
| warm-up | 4.60 | 33.7 | pipelines/compile_ms 605, pipelines/front_ms 586, render/thread/ms 72.03, render/busy/ms 70.59, main/busy/ms 62.53, stage/main/update 39.25, stage/render/prepare_assets 33.98, pipelines/wait_ms 19.69, pipelines/driver_ms 19.00, stage/render/render 18.21, stage/main/post_update 11.80, render/graph/ms 11.25 |
| warm-up | 4.65 | 47.4 | pipelines/compile_ms 1433, pipelines/front_ms 1386, pipelines/wait_ms 78.43, main/busy/ms 67.94, render/thread/ms 65.93, render/busy/ms 64.13, pipelines/driver_ms 47.34, stage/main/update 45.77, stage/render/prepare_assets 18.89, stage/render/render 17.25, stage/main/post_update 12.78, render/graph/ms 10.38 |
| warm-up | 4.73 | 80.1 | pipelines/compile_ms 1429, pipelines/front_ms 1376, render/thread/ms 104, render/busy/ms 102, main/busy/ms 88.44, pipelines/wait_ms 78.01, stage/render/prepare_assets 60.47, pipelines/driver_ms 52.98, stage/main/update 51.35, stage/main/post_update 30.66, stage/render/render 13.68, render/graph/ms 11.33 |
| warm-up | 4.78 | 49.4 | pipelines/compile_ms 868, pipelines/front_ms 839, render/thread/ms 112, render/busy/ms 110, main/busy/ms 87.35, stage/render/prepare_assets 57.29, stage/main/update 38.53, stage/main/post_update 36.02, pipelines/driver_ms 28.99, stage/render/prepare_write_phase_buffers 14.55, stage/render/render 14.42, pipelines/wait_ms 13.71 |
| warm-up | 4.86 | 79.3 | pipelines/compile_ms 2357, pipelines/front_ms 2284, pipelines/wait_ms 420, render/thread/ms 112, render/busy/ms 110, main/busy/ms 94.33, pipelines/driver_ms 72.64, stage/main/post_update 38.88, stage/render/prepare_assets 33.28, stage/main/update 31.53, stage/render/prepare_resources 24.68, stage/render/prepare_write_phase_buffers 17.39 |
| warm-up | 4.90 | 42.0 | pipelines/compile_ms 2242, pipelines/front_ms 2171, pipelines/wait_ms 407, main/busy/ms 107, render/thread/ms 101, render/busy/ms 98.83, pipelines/driver_ms 71.76, stage/main/update 53.84, stage/render/prepare_assets 34.24, stage/main/post_update 32.59, stage/render/prepare_resources 24.01, stage/render/render 13.92 |
| warm-up | 4.98 | 85.3 | pipelines/compile_ms 1408, pipelines/front_ms 1356, main/busy/ms 135, stage/main/update 111, render/thread/ms 82.90, render/busy/ms 80.89, stream/drive/ms 62.55, pipelines/driver_ms 51.23, stage/render/prepare_assets 35.68, stage/render/render 18.45, pipelines/wait_ms 16.24, render/graph/ms 15.58 |
| warm-up | 5.07 | 81.9 | pipelines/compile_ms 1779, pipelines/front_ms 1718, pipelines/wait_ms 162, main/busy/ms 107, stage/main/update 85.19, render/thread/ms 83.15, render/busy/ms 81.62, stream/drive/ms 62.49, pipelines/driver_ms 61.74, stage/render/prepare_assets 37.81, stage/render/render 18.49, render/graph/ms 16.39 |
| warm-up | 5.10 | 30.8 | pipelines/compile_ms 1566, pipelines/front_ms 1521, pipelines/wait_ms 151, render/thread/ms 58.54, render/busy/ms 56.62, main/busy/ms 52.50, pipelines/driver_ms 45.16, stage/main/update 35.28, stage/render/render 18.48, stage/render/prepare_assets 16.25, render/graph/ms 16.06, stage/render/graph_submit 12.37 |
| warm-up | 5.14 | 41.2 | pipelines/compile_ms 528, pipelines/front_ms 512, render/thread/ms 52.31, render/busy/ms 50.34, main/busy/ms 49.60, stage/main/update 33.00, stage/render/render 17.44, pipelines/driver_ms 16.53, render/graph/ms 14.94, stage/render/prepare_assets 12.13, encode//sum 11.86, stage/render/graph_submit 11.21 |
| warm-up | 5.24 | 36.7 | render/thread/ms 55.99, render/busy/ms 54.10, pipelines/compile_ms 48.50, pipelines/front_ms 47.26, main/busy/ms 44.99, stage/main/update 25.79, encode//sum 23.56, stage/render/render 21.17, render/graph/ms 18.80, stage/main/post_update 13.59, stage/render/graph_submit 13.21, stage/render/prepare_flush 12.42 |
| warm-up | 5.27 | 30.6 | pipelines/compile_ms 903, pipelines/front_ms 879, render/thread/ms 59.02, render/busy/ms 57.28, main/busy/ms 38.02, pipelines/driver_ms 23.93, stage/main/update 19.81, stage/render/render 18.36, encode//sum 17.07, render/graph/ms 16.18, stage/main/post_update 11.52, stage/render/prepare_flush 11.29 |
| warm-up | 5.34 | 31.6 | pipelines/compile_ms 56.71, pipelines/front_ms 53.92, render/thread/ms 43.71, render/busy/ms 41.61, main/busy/ms 33.73, stage/render/render 16.67, render/graph/ms 14.04, stage/main/update 13.22, stage/main/post_update 12.85, stage/render/prepare_resources 10.85, encode//sum 9.97, stage/render/graph_submit 9.61 |
| warm-up | 5.38 | 39.6 | pipelines/compile_ms 671, pipelines/front_ms 649, render/thread/ms 65.14, render/busy/ms 63.34, main/busy/ms 49.27, pipelines/wait_ms 30.37, pipelines/driver_ms 22.14, stage/main/pre_update 20.18, stage/render/prepare_assets 19.86, stage/render/render 16.14, stage/main/update 13.80, render/graph/ms 13.72 |
| warm-up | 6.02 | 27.6 | render/thread/ms 39.13, render/busy/ms 37.69, main/busy/ms 30.16, stage/render/render 16.44, render/graph/ms 14.31, stage/main/update 13.80, encode//sum 13.59, stage/main/post_update 12.76, stage/render/prepare_assets 9.93, stage/render/graph_submit 9.91, encode/submit 6.85, stage/render/camera_driver 4.22 |
| warm-up | 6.08 | 30.2 | render/thread/ms 37.04, render/busy/ms 36.19, main/busy/ms 24.34, stage/render/prepare_assets 19.66, stage/main/update 9.98, encode//sum 9.56, stage/render/render 9.26, stage/main/post_update 7.80, render/graph/ms 7.78, stage/render/graph_submit 4.97, stage/main/fixed_main_loop 3.03, encode/submit 3.02 |
| warm-up | 6.28 | 30.2 | render/thread/ms 42.48, render/busy/ms 41.14, main/busy/ms 39.59, stage/render/render 20.53, stage/main/update 20.03, render/graph/ms 18.44, stage/main/post_update 14.18, stage/render/graph_submit 13.19, encode//sum 11.95, encode/submit 9.97, stage/render/prepare_assets 8.29, stage/render/camera_driver 5.05 |
| warm-up | 6.34 | 34.9 | pipelines/compile_ms 138, pipelines/front_ms 136, render/thread/ms 49.73, render/busy/ms 48.66, main/busy/ms 37.82, stage/main/update 19.49, stage/render/prepare_assets 17.73, stage/render/render 11.90, encode//sum 11.90, stage/main/post_update 10.87, render/graph/ms 10.38, stage/render/graph_submit 6.40 |
| warm-up | 6.38 | 37.2 | render/thread/ms 61.50, render/busy/ms 59.68, main/busy/ms 44.90, stage/main/update 22.89, stage/render/render 20.76, render/graph/ms 18.30, encode//sum 17.32, stage/render/prepare_assets 17.23, stage/main/post_update 13.03, stage/render/graph_submit 11.52, encode/pass:shadow_point_light/sum 11.44, pipelines/compile_ms 9.62 |
| warm-up | 6.41 | 27.7 | render/thread/ms 54.18, render/busy/ms 52.44, main/busy/ms 33.86, stage/render/render 20.35, render/graph/ms 17.94, encode//sum 16.22, stage/main/update 15.72, stage/render/prepare_assets 15.50, encode/pass:shadow_point_light/sum 11.78, stage/render/graph_submit 11.25, stage/main/post_update 10.10, pipelines/compile_ms 9.62 |
| warm-up | 6.44 | 33.7 | render/thread/ms 45.58, render/busy/ms 44.53, main/busy/ms 30.66, stage/render/prepare_assets 19.76, stage/main/update 14.47, stage/render/render 13.82, encode//sum 12.55, render/graph/ms 12.13, stage/main/post_update 9.94, encode/pass:shadow_point_light/sum 9.31, stage/render/graph_submit 7.43, stage/render/camera_driver 4.58 |
| warm-up | 6.61 | 47.2 | main/busy/ms 61.79, render/thread/ms 49.52, render/busy/ms 48.04, stage/main/update 41.60, stage/render/render 23.47, render/graph/ms 21.26, encode//sum 19.08, stream/drive/ms 16.15, stage/render/graph_submit 15.60, stage/main/post_update 13.66, encode/pass:shadow_point_light/sum 11.42, stage/render/prepare_assets 11.06 |
| warm-up | 6.65 | 37.5 | render/thread/ms 55.63, render/busy/ms 54.38, main/busy/ms 41.73, stage/main/update 21.72, stage/render/prepare_assets 21.13, stage/render/render 18.92, encode//sum 17.15, render/graph/ms 16.92, stage/main/post_update 12.46, stage/render/graph_submit 11.49, encode/pass:shadow_point_light/sum 11.25, encode/submit 6.92 |
| warm-up | 7.36 | 73.2 | render/thread/ms 84.83, render/busy/ms 83.76, stage/render/render 75.57, render/graph/ms 73.99, stage/render/graph_submit 69.48, encode/submit 66.27, main/busy/ms 20.40, encode//sum 11.72, stage/main/post_update 9.79, encode/pass:shadow_point_light/sum 8.69, stage/main/update 6.86, stage/render/camera_driver 4.40 |
| measure | 18.38 | 33.1 | main/busy/ms 41.12, render/thread/ms 26.61, stage/main/update 25.95, render/busy/ms 25.55, stage/render/render 16.09, encode//sum 15.08, render/graph/ms 14.68, stage/main/post_update 10.12, stage/render/graph_submit 9.95, encode/submit 6.81, stage/render/camera_driver 4.60, stage/render/prepare_assets 3.58 |
| measure | 59.11 | 27.4 | render/thread/ms 35.63, render/busy/ms 34.20, main/busy/ms 32.23, stage/main/post_update 17.55, stage/render/render 16.39, render/graph/ms 14.29, encode//sum 10.81, stage/render/graph_submit 9.71, stage/main/update 8.78, stage/render/prepare_assets 8.73, encode/submit 5.82, stage/render/camera_driver 4.44 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=cache-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **75%** (min 52%) · 2625 MHz (min 2625) · memory 8101 MHz · 74.8 W · 73 °C (max 79) · VRAM 6190 MiB · throttled 100% of samples

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
