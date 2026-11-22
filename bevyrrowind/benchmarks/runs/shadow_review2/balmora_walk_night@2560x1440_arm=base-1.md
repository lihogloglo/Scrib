# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6690 frames over 65.6 s · **median 9.70 ms (103 fps)** · mean 9.81 · p95 11.46 · p99 13.33 (1% low 75 fps) · worst 28.67 ms · 38 spikes over 1.5× median · 32.9 stutters/min (work 25.6/min)
- warm-up: 353 frames over 9.3 s, worst 2521.8 ms
- hitches (> 25.0 ms): **3** measured, 48 in warm-up
- timed GPU passes: 6.59 ms of a 9.81 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.70 ms, limit 11 ms
- **FAIL** frame_ms: 36 stutters (> 1.5x moving median of 60) in 1.09 min = 32.91/min, limit 1/min
- pass worst frame 28.67 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.880 | 2.039 | 2.073 | 0.92 | 0.175 |
| main_opaque_pass_3d | 1.530 | 1.659 | 2.374 | 0.92 | 0.062 |
| early prepass | 0.479 | 0.519 | 0.755 | 0.92 | 0.088 |
| bloom | 0.293 | 0.318 | 0.326 | 0.92 | 0.064 |
| atmosphere_luts | 0.291 | 0.315 | 0.379 | 0.92 | 0.003 |
| water_velocity | 0.208 | 0.226 | 0.247 | 0.92 | 0.002 |
| water_sspr | 0.187 | 0.202 | 0.204 | 0.92 | 0.002 |
| ssao | 0.174 | 0.188 | 0.193 | 0.92 | 0.007 |
| point_shadow_atlas | 0.152 | 0.280 | 0.472 | 0.54 | 0.246 |
| upscaling | 0.119 | 0.129 | 0.135 | 0.92 | 0.004 |
| atmos_froxel_inject | 0.118 | 0.128 | 0.130 | 0.92 | 0.001 |
| water_fft | 0.114 | 0.124 | 0.126 | 0.92 | 0.003 |
| uniform_allocation | 0.106 | 0.115 | 0.140 | 0.92 | 0.020 |
| clouds_composite | 0.097 | 0.105 | 0.109 | 0.92 | 0.002 |
| gpu_instance_materialization | 0.085 | 0.092 | 0.096 | 0.92 | 0.004 |
| bin_unpacking | 0.084 | 0.091 | 0.112 | 0.92 | 0.015 |
| tonemapping | 0.080 | 0.087 | 0.092 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.071 | 0.146 | 0.92 | 0.008 |
| clustering | 0.059 | 0.064 | 0.068 | 0.92 | 0.020 |
| clouds_trace | 0.052 | 0.056 | 0.078 | 0.92 | 0.001 |
| early_mesh_preprocessing | 0.049 | 0.053 | 0.070 | 0.92 | 0.003 |
| render_sky | 0.049 | 0.053 | 0.054 | 0.92 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.415 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.60 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.92 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.027 | 0.050 | 0.073 | 0.54 | 0.009 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.220 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.92 | 0.003 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.031 | 0.92 | 0.185 |
| point_shadow_atlas/uniform_allocation | 0.016 | 0.029 | 0.057 | 0.54 | 0.008 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.92 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.92 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.011 | 0.020 | 0.036 | 0.54 | 0.005 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.92 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.92 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.54 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.651 | 10.054 | 19.15 | 57878.0 |
| render/busy/ms | 8.123 | 9.534 | 18.51 | 54341.6 |
| main/busy/ms | 8.012 | 9.244 | 25.12 | 53601.6 |
| stage/render/render | 5.063 | 6.030 | 11.96 | 33868.6 |
| stage/main/post_update | 4.529 | 5.380 | 11.73 | 30297.8 |
| render/graph/ms | 4.394 | 5.260 | 11.42 | 29395.6 |
| encode//sum | 3.130 | 3.867 | 10.99 | 20942.5 |
| stage/render/graph_submit | 3.096 | 3.846 | 9.96 | 20714.9 |
| encode/submit | 1.968 | 2.782 | 5.48 | 13163.2 |
| stage/main/update | 1.794 | 2.267 | 19.99 | 12003.3 |
| stage/render/camera_driver | 1.237 | 1.513 | 3.01 | 8272.7 |
| encode/finish_wall | 1.208 | 1.587 | 8.00 | 8079.3 |
| encode/pass:early prepass/sum | 0.806 | 1.043 | 7.36 | 5389.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.804 | 1.059 | 1.70 | 5379.2 |
| encode/pass:shadow_directional_light/sum | 0.799 | 1.205 | 2.46 | 5345.9 |
| stage/render/prepare_write_phase_buffers | 0.572 | 0.880 | 1.83 | 3828.7 |
| bigspace/total/ms | 0.538 | 1.287 | 2.10 | 3596.3 |
| stage/main/fixed_main_loop | 0.511 | 0.982 | 2.34 | 3419.2 |
| stage/render/prepare_assets | 0.500 | 0.640 | 11.09 | 3344.3 |
| stage/render/prepare_meshes | 0.434 | 0.539 | 3.70 | 2902.1 |
| render/present/ms | 0.421 | 1.176 | 2.10 | 2819.3 |
| stage/render/prepare_resources | 0.410 | 0.494 | 1.78 | 2744.3 |
| stage/main/pre_update | 0.382 | 0.549 | 6.14 | 2552.4 |
| stage/main/first | 0.365 | 0.489 | 5.42 | 2443.3 |
| controller/feed/ms | 0.318 | 0.404 | 0.87 | 2125.6 |
| stage/render/prepare_flush | 0.316 | 0.519 | 2.39 | 2116.1 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.310 | 1.155 | 2.10 | 2072.4 |
| stage/render/prepare_bind_groups | 0.297 | 0.490 | 2.59 | 1985.7 |
| stage/render/prepare_batch_phases | 0.274 | 0.412 | 1.06 | 1836.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.263 | 0.429 | 1.30 | 1760.2 |
| stage/main/last | 0.255 | 0.301 | 1.38 | 1705.7 |
| bigspace/low_precision/ms | 0.229 | 0.718 | 1.71 | 1533.1 |
| stage/render/create_views | 0.200 | 0.274 | 0.75 | 1335.6 |
| stream/drive/ms | 0.197 | 0.277 | 10.22 | 1320.7 |
| stage/render/extract_commands | 0.182 | 0.216 | 0.73 | 1217.1 |
| bigspace/high_precision/ms | 0.163 | 0.669 | 1.79 | 1093.5 |
| anim/evaluate/ms | 0.161 | 0.295 | 1.70 | 1074.6 |
| stage/render/specialize | 0.153 | 0.243 | 1.75 | 1020.5 |
| anim/evaluate_cpu/ms | 0.150 | 0.187 | 0.66 | 1002.3 |
| stage/render/prepare_views | 0.148 | 0.206 | 1.53 | 992.7 |
| stream/tick/ms | 0.146 | 0.210 | 0.45 | 974.2 |
| render/acquire/ms | 0.107 | 0.146 | 1.50 | 717.1 |
| stage/render/queue | 0.096 | 0.166 | 2.16 | 644.1 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.088 | 0.820 | 1.51 | 588.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.088 | 0.137 | 0.63 | 586.1 |
| stage/render/phase_sort | 0.079 | 0.109 | 0.38 | 526.2 |
| controller/move/ms | 0.078 | 0.255 | 0.83 | 518.8 |
| controller/write/ms | 0.073 | 0.101 | 0.47 | 491.2 |
| terrain/update/ms | 0.059 | 0.289 | 1.56 | 392.7 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.050 | 0.000 | 3.45 | 334.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.071 | 0.29 | 304.5 |
| physics/step/ms | 0.044 | 0.117 | 0.68 | 292.9 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 1.39 | 210.4 |
| pipelines/compile_ms | 0.025 | 0.000 | 82.66 | 167.0 |
| pipelines/front_ms | 0.024 | 0.000 | 80.48 | 161.5 |
| stage/render/cleanup | 0.023 | 0.031 | 0.54 | 157.1 |
| physics/insert/ms | 0.023 | 0.281 | 0.82 | 155.1 |
| terrain/select/ms | 0.021 | 0.255 | 0.83 | 142.1 |
| weather/map/ms | 0.017 | 0.024 | 0.28 | 116.5 |
| far/select/ms | 0.014 | 0.162 | 0.55 | 95.3 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.54 | 84.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.25 | 76.2 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 71.7 |
| encode/camera_driver_span_end/sum | 0.010 | 0.020 | 0.40 | 69.8 |
| script/update/ms | 0.008 | 0.012 | 0.31 | 52.9 |
| controller/avoid/ms | 0.006 | 0.012 | 0.07 | 38.3 |
| stream/despawn/ms | 0.005 | 0.000 | 0.94 | 34.9 |
| script/input/ms | 0.005 | 0.006 | 0.38 | 31.8 |
| script/ui/ms | 0.003 | 0.005 | 0.04 | 22.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.18 | 21.8 |
| stream/upload/ms | 0.003 | 0.000 | 1.01 | 18.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 13.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.4 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.14 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
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
| anim/evaluated/count | count | 19 | 33 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1313 | 2262 | 641 |
| crowd/c0/meshes/count | count | 357 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5997 | 5555 |
| crowd/c1/meshes/count | count | 1184 | 1299 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 204 |
| crowd/joints_extracted/count | count | 689 | 7019 | 678 |
| crowd/joints_uploaded/count | count | 3598 | 6207 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.39 | 7.40 | 0.55 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.54 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.40 | 0.02 |
| encode/finish_critical | ms | 0.96 | 7.40 | 1.16 |
| encode/pass:early prepass/max | ms | 0.81 | 7.36 | 0.92 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 1.70 | 0.99 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.63 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.30 | 0.26 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.80 | 2.10 | 0.74 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.37 | 1.51 | 0.20 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.16 | 3.45 | 0.95 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.46 | 1.16 |
| encode/resolve_diagnostics/max | ms | 0.03 | 1.39 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.52 | 25.12 | 8.72 |
| gpu/timed/ms | ms | 7.96 | 9.47 | 8.72 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
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
| phases/alpha_mask3d/items | count | 324 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 324 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2481 | 3554 | 2458 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2481 | 3554 | 2458 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 75 | 519 | 27 |
| phases/shadow/bins | count | 878 | 1551 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 98.3 k |
| phases/shadow/items | count | 2342 | 3313 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 55 | 154 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2698 | 3182 | 2346 |
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
| preprocess/bind_groups | count | 15 | 267 | 41 |
| preprocess/bind_groups_created | count | 10 | 255 | 35 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 6 |
| preprocess/bind_groups_reused | count | 136 | 212 | 125 |
| process/cpu_usage | % | 25.48 | 47.27 | 37.02 |
| process/mem_usage | GiB | 5.17 | 5.42 | 5.42 |
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
| shadow_atlas/live_occ_pct | % | 11.72 | 37.60 | 0 |
| shadow_atlas/live_tiles | count | 18 | 70 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 147 | 458 | 0 |
| shadow_atlas/moving_lamps | count | 1 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 19 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 61 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 107 | 0 |
| shadow_atlas/views | count | 17 | 61 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 123 | 13 |
| skin/blocks_skipped/count | count | 145 | 203 | 190 |
| skin/blocks_walked/count | count | 14 | 27 | 14 |
| skin/joint_bytes_written/bytes | bytes | 48.6 k | 807.6 k | 41.2 k |
| skin/joints_written/count | count | 760 | 12.6 k | 644 |
| skin/upload_writes/count | count | 8 | 24 | 9 |
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
| system/cpu_usage | % | 29.88 | 47.42 | 39.34 |
| system/mem_usage | % | 47.41 | 47.94 | 47.94 |
| terrain/patches | count | 511 | 520 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2666 | 2724 | 2724 |
| views/all/batch_sets | count | 132 | 572 | 85 |
| views/all/bins | count | 2936 | 3566 | 2948 |
| views/all/count | count | 20 | 64 | 3 |
| views/all/instance_batches | count | 12 | 22 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 196.6 k |
| views/all/items | count | 8062 | 10.6 k | 7960 |
| views/all/pipelines | count | 100 | 199 | 66 |
| views/all/pipelines_distinct | count | 66 | 73 | 61 |
| views/all/work_items | count | 154.1 k | 231.2 k | 205.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 27 |
| views/cascade/bins | count | 717 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2121 | 2454 | 2014 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 60 | 506 | 0 |
| views/lamp_face/bins | count | 115 | 713 | 0 |
| views/lamp_face/count | count | 17 | 61 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 160 | 866 | 0 |
| views/lamp_face/pipelines | count | 28 | 128 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 120 | 25.2 k | 0 |
| views/main/batch_sets | count | 57 | 61 | 58 |
| views/main/bins | count | 2012 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7528 | 5946 |
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
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 49.81 | 224 | 333.3 k |
| anim/events/count | count | 0.08 | 3 | 568 |
| assets/Image/added | count | 0.09 | 37 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 12 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 265 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7477 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1154 | 1489 |
| encode//count | count | 27.91 | 30 | 186.7 k |
| encode/bind_walk/init_textures | count | 9104 | 23.5 k | 60.91 M |
| encode/bind_walk/init_textures_skipped | count | 48.7 k | 70.7 k | 325.99 M |
| encode/bind_walk/inits | count | 567 | 768 | 3.80 M |
| encode/bind_walk/inits_skipped | count | 169 | 243 | 1.13 M |
| encode/bind_walk/merge_textures | count | 9041 | 23.4 k | 60.49 M |
| encode/bind_walk/merge_textures_skipped | count | 8531 | 18.7 k | 57.07 M |
| encode/bind_walk/merges | count | 106 | 189 | 712.2 k |
| encode/bind_walk/merges_skipped | count | 150 | 223 | 1.00 M |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6690 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6690 |
| encode/pass:early prepass/count | count | 1 | 1 | 6690 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6690 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6690 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6690 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.36 | 1 | 2440 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.16 | 1 | 1099 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 269 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6690 |
| encode/queue/command_buffers | count | 38.49 | 42 | 257.5 k |
| encode/queue/submits | count | 2.01 | 4 | 13.5 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.61 M | 15.55 G |
| encode/queue/write_buffer/calls | count | 273 | 462 | 1.83 M |
| encode/queue/write_buffer_with/bytes | bytes | 171.1 k | 1.29 M | 1.14 G |
| encode/queue/write_buffer_with/calls | count | 10.28 | 804 | 68.7 k |
| encode/queue/write_texture/bytes | bytes | 2060 | 1.05 M | 13.78 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 558 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6690 |
| physics/inserts | count | 0.33 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
| physics/steps | count | 0.59 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 20 | 219 |
| stream/deactivate/items | count | 0.03 | 27 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5060 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1109 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 51, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2521.8 | main/busy/ms 2392, stage/main/post_update 2340, stage/main/update 35.88, stage/main/pre_update 7.15, stage/main/last 5.72, surface/advance_cpu_ms 0.88, stage/main/first 0.65, stream/drive/ms 0.55, stage/main/fixed_main_loop 0.39, surface/cell_upload_ms 0.29, stream/tick/ms 0.15, bigspace/total/ms 0.08 |
| warm-up | 3.38 | 853.5 | main/busy/ms 2541, stage/main/post_update 2386, render/thread/ms 896, render/busy/ms 758, stage/render/prepare_assets 702, stage/render/prepare_views 136, render/acquire/ms 136, stage/main/update 133, stage/render/create_views 31.80, stage/render/render 11.52, stage/main/pre_update 8.74, render/graph/ms 7.62 |
| warm-up | 3.48 | 105.8 | render/thread/ms 984, render/busy/ms 796, stage/render/prepare_assets 725, main/busy/ms 171, stage/render/prepare_views 136, render/acquire/ms 136, stage/main/update 103, stage/render/render 67.15, stage/main/post_update 57.20, pipelines/compile_ms 56.61, render/present/ms 51.50, pipelines/driver_ms 38.46 |
| warm-up | 3.64 | 139.7 | pipelines/wait_ms 3477, pipelines/compile_ms 1629, pipelines/front_ms 1519, render/thread/ms 129, render/busy/ms 122, pipelines/driver_ms 110, main/busy/ms 92.56, stage/main/post_update 58.69, encode//sum 56.68, stage/render/prepare_flush 30.41, stage/render/render 25.64, stage/main/update 25.51 |
| warm-up | 3.68 | 41.4 | pipelines/wait_ms 4547, pipelines/compile_ms 1765, pipelines/front_ms 1656, render/thread/ms 151, render/busy/ms 149, main/busy/ms 127, pipelines/driver_ms 109, stage/main/update 72.12, encode//sum 66.26, stage/render/prepare_resources 38.44, stage/render/prepare_flush 34.24, stage/main/post_update 32.49 |
| warm-up | 3.81 | 125.4 | pipelines/compile_ms 1574, pipelines/front_ms 1480, pipelines/wait_ms 1238, render/thread/ms 121, render/busy/ms 119, main/busy/ms 94.17, pipelines/driver_ms 93.37, stage/render/prepare_assets 79.46, stage/main/update 56.62, stage/main/post_update 16.59, stage/render/prepare_resources 15.19, terrain/update/ms 14.41 |
| warm-up | 3.85 | 44.2 | pipelines/compile_ms 1683, pipelines/front_ms 1580, pipelines/wait_ms 151, render/thread/ms 124, render/busy/ms 122, pipelines/driver_ms 103, stage/render/prepare_assets 101, main/busy/ms 42.66, encode//sum 17.41, stage/main/post_update 15.79, stage/render/render 14.72, render/graph/ms 12.40 |
| warm-up | 3.88 | 32.3 | pipelines/compile_ms 643, pipelines/front_ms 609, render/thread/ms 73.01, render/busy/ms 71.11, main/busy/ms 44.61, stage/render/prepare_assets 41.83, pipelines/driver_ms 34.03, pipelines/wait_ms 29.35, stage/render/render 19.56, encode//sum 17.86, render/graph/ms 17.49, stage/main/post_update 16.42 |
| warm-up | 3.92 | 31.4 | pipelines/compile_ms 406, pipelines/front_ms 382, render/thread/ms 59.86, render/busy/ms 58.18, main/busy/ms 40.48, pipelines/driver_ms 23.57, stage/main/update 20.48, stage/render/prepare_assets 16.20, stage/render/render 15.98, pipelines/wait_ms 15.76, stage/main/post_update 15.34, render/graph/ms 13.79 |
| warm-up | 3.95 | 32.4 | pipelines/compile_ms 454, pipelines/front_ms 433, pipelines/wait_ms 104, main/busy/ms 75.48, render/thread/ms 59.57, render/busy/ms 57.54, stage/main/update 48.40, pipelines/driver_ms 21.16, stage/main/post_update 15.92, stage/render/render 14.20, stage/render/prepare_write_phase_buffers 14.11, render/graph/ms 11.57 |
| warm-up | 4.05 | 98.9 | pipelines/compile_ms 1299, pipelines/front_ms 1244, pipelines/wait_ms 105, render/thread/ms 82.80, main/busy/ms 80.73, render/busy/ms 80.53, stage/main/update 59.69, pipelines/driver_ms 54.73, stage/render/prepare_assets 45.05, stage/render/render 17.29, render/graph/ms 14.38, stage/render/graph_submit 11.24 |
| warm-up | 4.09 | 43.3 | pipelines/compile_ms 1825, pipelines/front_ms 1756, render/thread/ms 75.31, render/busy/ms 73.45, pipelines/driver_ms 69.32, stage/render/prepare_assets 52.48, main/busy/ms 37.97, pipelines/wait_ms 29.46, stage/main/update 26.48, stage/render/render 13.63, render/graph/ms 11.38, stage/render/graph_submit 8.89 |
| warm-up | 4.12 | 29.6 | pipelines/compile_ms 986, pipelines/front_ms 952, render/thread/ms 43.16, render/busy/ms 41.94, pipelines/driver_ms 34.79, pipelines/wait_ms 30.04, stage/render/prepare_assets 29.21, main/busy/ms 19.61, stage/main/update 9.40, stage/render/render 9.20, render/graph/ms 7.84, stage/render/graph_submit 5.91 |
| warm-up | 4.17 | 40.1 | pipelines/compile_ms 280, pipelines/front_ms 267, render/thread/ms 44.28, stage/render/prepare_views 24.32, render/acquire/ms 24.20, main/busy/ms 22.35, render/busy/ms 18.26, encode//sum 16.23, stage/render/render 13.36, pipelines/driver_ms 12.48, render/graph/ms 11.11, stage/main/update 10.93 |
| warm-up | 4.22 | 30.5 | render/thread/ms 44.15, render/busy/ms 42.47, main/busy/ms 34.30, pipelines/compile_ms 33.83, pipelines/front_ms 31.60, stage/main/post_update 14.74, stage/render/render 14.73, stage/main/update 12.52, render/graph/ms 12.29, stage/render/prepare_assets 11.08, stage/render/graph_submit 8.33, encode//sum 8.26 |
| warm-up | 4.26 | 44.1 | pipelines/compile_ms 383, pipelines/front_ms 368, render/thread/ms 58.49, render/busy/ms 56.70, main/busy/ms 33.52, stage/render/prepare_assets 26.35, pipelines/driver_ms 15.30, stage/render/render 14.64, stage/main/update 13.21, stage/main/post_update 12.48, render/graph/ms 12.16, stage/render/graph_submit 9.07 |
| warm-up | 4.31 | 46.8 | pipelines/compile_ms 1211, pipelines/front_ms 1174, render/thread/ms 61.19, render/busy/ms 59.72, stage/render/prepare_assets 39.66, pipelines/driver_ms 37.11, main/busy/ms 33.08, stage/main/update 15.06, stage/render/render 11.85, stage/main/post_update 10.29, render/graph/ms 9.93, stage/render/graph_submit 7.17 |
| warm-up | 4.36 | 46.9 | pipelines/compile_ms 1252, pipelines/front_ms 1218, main/busy/ms 149, stage/main/update 131, render/thread/ms 67.49, render/busy/ms 65.41, stage/render/prepare_assets 38.85, pipelines/driver_ms 34.38, stage/render/render 13.35, render/graph/ms 10.76, stage/main/post_update 10.71, stage/render/graph_submit 6.79 |
| warm-up | 4.52 | 164.3 | pipelines/compile_ms 1463, pipelines/front_ms 1424, main/busy/ms 165, stage/main/update 151, render/thread/ms 82.19, render/busy/ms 79.99, pipelines/wait_ms 58.15, stage/render/prepare_assets 50.71, pipelines/driver_ms 39.67, stage/render/render 14.67, pipelines/lock_ms 13.30, render/graph/ms 11.97 |
| warm-up | 4.57 | 45.0 | pipelines/compile_ms 2307, pipelines/front_ms 2236, pipelines/wait_ms 151, render/thread/ms 78.08, render/busy/ms 76.51, pipelines/driver_ms 71.06, main/busy/ms 63.37, stage/render/prepare_assets 49.31, stage/main/update 46.26, pipelines/lock_ms 18.46, stage/render/render 13.01, stream/drive/ms 11.26 |
| warm-up | 4.64 | 73.9 | pipelines/compile_ms 1396, pipelines/front_ms 1347, pipelines/wait_ms 228, render/thread/ms 101, render/busy/ms 99.85, main/busy/ms 87.54, pipelines/driver_ms 48.08, stage/main/post_update 38.59, stage/render/prepare_assets 37.76, stage/render/prepare_resources 25.68, stage/main/update 21.81, stage/render/prepare_write_phase_buffers 13.78 |
| warm-up | 4.70 | 58.1 | pipelines/wait_ms 222, pipelines/compile_ms 189, pipelines/front_ms 181, render/thread/ms 120, render/busy/ms 119, main/busy/ms 111, stage/main/post_update 60.81, stage/render/prepare_assets 27.64, stage/main/update 26.90, stage/render/prepare_resources 26.75, stage/render/prepare_flush 24.49, stage/render/prepare_write_phase_buffers 20.47 |
| warm-up | 4.77 | 69.2 | pipelines/compile_ms 2367, pipelines/front_ms 2288, pipelines/wait_ms 327, render/thread/ms 99.04, render/busy/ms 97.75, main/busy/ms 81.35, pipelines/driver_ms 78.86, stage/render/prepare_assets 41.15, stage/main/post_update 39.71, stage/main/update 31.14, stage/render/prepare_flush 23.88, stream/drive/ms 18.46 |
| warm-up | 4.82 | 53.0 | pipelines/compile_ms 2633, pipelines/front_ms 2548, pipelines/wait_ms 240, render/thread/ms 91.86, render/busy/ms 90.23, pipelines/driver_ms 85.63, main/busy/ms 72.46, stage/render/prepare_assets 61.79, stage/main/update 32.07, stage/main/post_update 27.05, stage/render/render 15.30, render/graph/ms 12.62 |
| warm-up | 4.88 | 64.0 | pipelines/compile_ms 842, pipelines/front_ms 809, pipelines/wait_ms 126, render/thread/ms 98.07, main/busy/ms 97.83, render/busy/ms 96.37, stage/main/update 66.17, stage/render/prepare_assets 48.96, pipelines/driver_ms 32.80, stage/main/post_update 18.71, stage/render/prepare_flush 17.88, stage/render/render 14.47 |
| warm-up | 4.97 | 87.1 | pipelines/compile_ms 1852, pipelines/front_ms 1804, pipelines/wait_ms 536, render/thread/ms 106, render/busy/ms 104, main/busy/ms 79.61, stage/main/update 56.47, stage/render/prepare_assets 52.34, pipelines/driver_ms 47.88, stage/render/prepare_flush 18.81, stage/render/render 14.47, render/graph/ms 12.50 |
| warm-up | 5.00 | 26.2 | pipelines/compile_ms 1634, pipelines/front_ms 1599, pipelines/wait_ms 415, main/busy/ms 98.89, render/thread/ms 77.90, render/busy/ms 75.96, stage/main/update 75.65, stream/drive/ms 46.66, pipelines/driver_ms 34.71, stage/render/prepare_assets 34.60, stage/render/render 19.98, render/graph/ms 17.61 |
| warm-up | 5.09 | 90.4 | pipelines/compile_ms 515, pipelines/front_ms 496, main/busy/ms 107, stage/main/update 86.51, stream/drive/ms 60.70, render/thread/ms 52.21, render/busy/ms 50.19, pipelines/driver_ms 19.40, stage/render/render 18.34, render/graph/ms 15.78, stage/render/prepare_assets 15.75, stage/main/post_update 12.25 |
| warm-up | 5.12 | 29.9 | pipelines/compile_ms 239, pipelines/front_ms 228, main/busy/ms 72.27, stage/main/update 54.51, render/thread/ms 51.28, render/busy/ms 48.99, stream/drive/ms 28.54, stage/render/render 18.53, stage/render/prepare_assets 16.09, render/graph/ms 15.55, stage/render/graph_submit 11.61, pipelines/driver_ms 11.25 |
| warm-up | 5.17 | 52.5 | pipelines/compile_ms 985, pipelines/front_ms 955, main/busy/ms 57.41, render/thread/ms 48.21, render/busy/ms 46.14, stage/main/update 40.99, pipelines/driver_ms 30.51, stage/render/render 16.90, stage/render/prepare_assets 16.61, stream/drive/ms 14.54, render/graph/ms 14.24, stage/render/graph_submit 10.89 |
| warm-up | 5.20 | 31.9 | pipelines/compile_ms 1149, pipelines/front_ms 1116, render/thread/ms 54.62, render/busy/ms 53.24, main/busy/ms 43.76, pipelines/driver_ms 33.08, stage/main/update 25.19, stage/render/prepare_assets 18.05, stage/render/render 12.81, stage/main/post_update 11.04, render/graph/ms 10.77, encode//sum 8.35 |
| warm-up | 5.23 | 32.7 | pipelines/compile_ms 201, pipelines/front_ms 196, render/thread/ms 54.38, render/busy/ms 52.83, main/busy/ms 43.62, stage/main/update 23.44, stage/render/render 14.18, render/graph/ms 12.08, stage/main/post_update 12.04, stage/render/prepare_assets 11.69, encode//sum 8.91, stage/render/graph_submit 8.55 |
| warm-up | 5.37 | 27.6 | pipelines/compile_ms 1081, pipelines/front_ms 1048, render/thread/ms 43.37, render/busy/ms 41.85, main/busy/ms 35.62, pipelines/driver_ms 33.03, stage/render/render 22.92, render/graph/ms 20.91, stage/render/graph_submit 17.06, stage/main/post_update 16.27, stage/main/update 13.79, encode/submit 11.71 |
| warm-up | 5.46 | 30.6 | pipelines/compile_ms 198, pipelines/front_ms 193, main/busy/ms 36.60, render/thread/ms 34.01, render/busy/ms 32.44, stage/main/update 21.16, stream/drive/ms 14.97, stage/render/render 13.38, render/graph/ms 11.06, stage/main/post_update 10.18, encode//sum 8.02, stage/render/prepare_assets 7.99 |
| warm-up | 5.70 | 34.1 | pipelines/compile_ms 199, pipelines/front_ms 193, main/busy/ms 44.86, render/thread/ms 29.78, render/busy/ms 28.07, stage/main/update 23.68, stream/drive/ms 15.46, stage/main/post_update 15.43, stage/render/render 14.45, render/graph/ms 12.05, stage/render/graph_submit 8.61, encode//sum 6.54 |
| warm-up | 5.72 | 26.4 | pipelines/compile_ms 571, pipelines/front_ms 555, render/thread/ms 39.94, main/busy/ms 38.73, render/busy/ms 37.61, stage/main/post_update 20.45, stage/render/render 18.49, pipelines/driver_ms 16.60, render/graph/ms 16.12, stage/render/graph_submit 12.48, stage/main/update 10.94, stage/render/prepare_assets 8.29 |
| warm-up | 5.98 | 31.7 | main/busy/ms 35.70, render/thread/ms 24.86, stage/main/update 24.01, render/busy/ms 23.61, stream/drive/ms 16.80, stage/render/render 11.22, render/graph/ms 9.37, encode//sum 7.84, stage/main/post_update 6.90, stage/render/graph_submit 6.34, stage/render/prepare_assets 6.20, encode/submit 3.57 |
| warm-up | 6.08 | 26.4 | render/thread/ms 42.65, render/busy/ms 41.02, main/busy/ms 34.64, stage/main/update 16.14, stage/render/render 15.90, render/graph/ms 13.67, stage/main/post_update 13.04, stage/render/prepare_assets 10.63, encode//sum 10.06, stage/render/graph_submit 8.87, encode/submit 6.34, stage/render/camera_driver 4.56 |
| warm-up | 6.11 | 29.7 | render/thread/ms 45.98, render/busy/ms 44.88, main/busy/ms 32.72, stage/render/prepare_assets 19.85, stage/main/update 16.22, stage/main/post_update 10.77, stage/render/render 10.36, render/graph/ms 8.63, encode//sum 8.13, stream/drive/ms 4.85, stage/render/graph_submit 4.61, stage/render/prepare_meshes 4.58 |
| warm-up | 6.23 | 27.7 | pipelines/compile_ms 143, pipelines/front_ms 138, render/thread/ms 35.30, render/busy/ms 34.23, main/busy/ms 33.40, stage/main/update 21.55, stage/render/prepare_assets 10.35, stage/render/render 9.20, render/graph/ms 7.76, stage/main/post_update 6.57, encode//sum 6.17, stage/render/graph_submit 5.44 |
| warm-up | 6.33 | 25.9 | main/busy/ms 40.83, render/thread/ms 36.61, render/busy/ms 35.25, stage/main/update 20.62, stage/render/render 16.58, stage/main/post_update 14.74, render/graph/ms 14.63, stage/render/graph_submit 9.96, encode//sum 8.65, encode/submit 7.11, stage/render/prepare_assets 5.24, stage/render/camera_driver 4.47 |
| warm-up | 6.36 | 29.8 | render/thread/ms 42.67, main/busy/ms 41.94, render/busy/ms 41.31, stage/main/update 21.63, stage/render/render 15.76, render/graph/ms 13.73, stage/main/post_update 13.71, stage/render/prepare_assets 10.45, stage/render/graph_submit 8.93, encode//sum 8.88, encode/submit 6.01, stage/render/prepare_meshes 4.96 |
| warm-up | 6.40 | 35.8 | render/thread/ms 49.68, render/busy/ms 48.61, main/busy/ms 38.73, stage/main/update 19.58, stage/render/prepare_assets 16.65, stage/render/render 12.38, render/graph/ms 10.91, stage/main/post_update 9.95, stage/render/graph_submit 8.04, stage/render/prepare_meshes 6.26, encode//sum 5.91, encode/finish_wall 5.34 |
| warm-up | 6.44 | 39.0 | render/thread/ms 63.99, render/busy/ms 62.62, main/busy/ms 43.01, stage/render/render 22.80, stage/main/update 21.02, render/graph/ms 20.70, stage/render/graph_submit 16.41, stage/render/prepare_assets 15.77, stage/main/post_update 12.66, encode/finish_wall 12.02, encode/pass:point_shadow_atlas_live_indirect/sum 11.64, encode//sum 7.32 |
| warm-up | 6.50 | 34.4 | pipelines/compile_ms 145, pipelines/front_ms 143, render/thread/ms 44.66, render/busy/ms 43.41, main/busy/ms 32.54, stage/main/update 16.42, stage/render/prepare_assets 16.05, stage/render/render 14.37, render/graph/ms 12.85, stage/main/post_update 10.22, stage/render/graph_submit 10.12, encode/finish_wall 6.94 |
| warm-up | 6.65 | 47.8 | main/busy/ms 57.11, render/thread/ms 41.09, stage/main/update 40.32, render/busy/ms 39.78, pipelines/compile_ms 18.84, pipelines/front_ms 17.67, stage/render/render 15.12, stream/drive/ms 14.48, render/graph/ms 13.15, stage/render/prepare_assets 11.70, stage/main/post_update 11.29, stage/render/graph_submit 9.85 |
| warm-up | 6.68 | 32.3 | render/thread/ms 48.13, render/busy/ms 46.93, main/busy/ms 43.34, stage/main/update 25.59, stage/render/prepare_assets 18.18, stage/render/render 14.59, render/graph/ms 12.85, stage/main/post_update 10.45, stage/render/graph_submit 9.41, encode//sum 7.18, stream/drive/ms 6.53, stage/render/prepare_meshes 6.05 |
| warm-up | 6.71 | 28.1 | render/thread/ms 41.72, render/busy/ms 40.75, main/busy/ms 32.00, stage/main/update 17.12, stage/render/prepare_assets 16.32, stage/render/render 12.08, render/graph/ms 10.61, stage/main/post_update 8.23, stage/render/graph_submit 8.03, stream/drive/ms 6.78, encode//sum 6.29, stage/render/prepare_meshes 5.54 |
| measure | 1.54 | 28.4 | render/thread/ms 32.21, render/busy/ms 30.69, main/busy/ms 27.38, stage/main/update 13.87, stage/render/render 11.37, stage/render/prepare_assets 10.77, render/graph/ms 9.35, stage/main/post_update 9.17, stage/render/graph_submit 6.81, encode//sum 5.94, encode/submit 4.91, stage/render/camera_driver 2.40 |
| measure | 18.29 | 27.5 | main/busy/ms 34.20, stage/main/update 21.91, render/thread/ms 21.14, render/busy/ms 20.12, stage/render/render 10.55, render/graph/ms 9.18, stage/main/post_update 8.30, encode//sum 7.17, stage/render/graph_submit 6.05, stage/render/prepare_assets 3.88, encode/submit 3.72, stage/render/camera_driver 3.00 |
| measure | 59.11 | 28.7 | render/thread/ms 36.57, render/busy/ms 35.54, main/busy/ms 23.49, stage/render/prepare_assets 16.48, stage/main/post_update 11.41, stage/render/render 9.71, render/graph/ms 8.39, encode//sum 7.55, stage/render/graph_submit 5.06, stage/main/update 4.63, stage/main/fixed_main_loop 3.33, stage/render/camera_driver 3.14 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1197 pmon samples)
- focus: clean
- nvidia-smi over the window (254 samples): GPU load **86%** (min 67%) · 2614 MHz (min 2610) · memory 8101 MHz · 86.0 W · 80 °C (max 85) · VRAM 5577 MiB · throttled 100% of samples

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
