# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: contactshadows · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=custom (preset: derived from its members); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=off (command line: --off contactshadows); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=off (command line: --off contactshadows); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6728 frames over 65.6 s · **median 9.66 ms (104 fps)** · mean 9.76 · p95 11.26 · p99 13.06 (1% low 77 fps) · worst 30.40 ms · 28 spikes over 1.5× median · 25.6 stutters/min (work 20.1/min)
- warm-up: 355 frames over 8.9 s, worst 2319.6 ms
- hitches (> 25.0 ms): **2** measured, 44 in warm-up
- timed GPU passes: 6.57 ms of a 9.76 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.66 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 30.40 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.899 | 2.036 | 2.070 | 0.93 | 0.175 |
| main_opaque_pass_3d | 1.458 | 1.564 | 2.366 | 0.93 | 0.064 |
| early prepass | 0.484 | 0.519 | 0.754 | 0.93 | 0.087 |
| bloom | 0.296 | 0.317 | 0.326 | 0.93 | 0.063 |
| atmosphere_luts | 0.292 | 0.314 | 0.379 | 0.93 | 0.003 |
| water_velocity | 0.211 | 0.226 | 0.247 | 0.93 | 0.002 |
| water_sspr | 0.189 | 0.202 | 0.204 | 0.93 | 0.002 |
| ssao | 0.175 | 0.188 | 0.191 | 0.93 | 0.007 |
| point_shadow_atlas | 0.153 | 0.278 | 0.472 | 0.55 | 0.238 |
| upscaling | 0.120 | 0.129 | 0.135 | 0.93 | 0.004 |
| atmos_froxel_inject | 0.119 | 0.128 | 0.130 | 0.93 | 0.001 |
| water_fft | 0.115 | 0.123 | 0.126 | 0.93 | 0.003 |
| uniform_allocation | 0.109 | 0.117 | 0.141 | 0.93 | 0.020 |
| clouds_composite | 0.098 | 0.105 | 0.109 | 0.93 | 0.002 |
| gpu_instance_materialization | 0.089 | 0.095 | 0.099 | 0.93 | 0.004 |
| bin_unpacking | 0.086 | 0.092 | 0.112 | 0.93 | 0.015 |
| tonemapping | 0.081 | 0.087 | 0.092 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.065 | 0.070 | 0.136 | 0.93 | 0.008 |
| clustering | 0.060 | 0.064 | 0.068 | 0.93 | 0.019 |
| clouds_trace | 0.052 | 0.056 | 0.078 | 0.93 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.055 | 0.071 | 0.93 | 0.003 |
| render_sky | 0.049 | 0.052 | 0.053 | 0.93 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.043 | 0.414 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.60 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.93 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.027 | 0.049 | 0.071 | 0.55 | 0.009 |
| lightprobe_radiance_map | 0.023 | 0.220 | 0.230 | 0.10 | 0.004 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.93 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.93 | 0.003 |
| main_transparent_pass_3d | 0.016 | 0.018 | 0.031 | 0.93 | 0.181 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.93 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.043 | 0.93 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.016 | 0.029 | 0.056 | 0.55 | 0.008 |
| point_shadow_atlas/bin_unpacking | 0.011 | 0.020 | 0.036 | 0.55 | 0.005 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.93 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.93 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.55 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.93 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.590 | 9.864 | 19.62 | 57793.2 |
| render/busy/ms | 8.066 | 9.334 | 18.71 | 54271.1 |
| main/busy/ms | 7.975 | 9.194 | 30.00 | 53657.4 |
| stage/render/render | 5.028 | 5.915 | 11.59 | 33828.7 |
| stage/main/post_update | 4.513 | 5.344 | 10.55 | 30366.5 |
| render/graph/ms | 4.368 | 5.170 | 10.49 | 29386.4 |
| encode//sum | 3.122 | 3.874 | 11.09 | 21008.1 |
| stage/render/graph_submit | 3.085 | 3.782 | 7.75 | 20754.7 |
| encode/submit | 1.951 | 2.714 | 4.62 | 13127.4 |
| stage/main/update | 1.787 | 2.250 | 23.75 | 12024.9 |
| stage/render/camera_driver | 1.222 | 1.481 | 3.38 | 8218.9 |
| encode/finish_wall | 1.212 | 1.587 | 3.20 | 8151.2 |
| encode/pass:early prepass/sum | 0.814 | 1.060 | 2.33 | 5477.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.813 | 1.059 | 2.23 | 5470.6 |
| encode/pass:shadow_directional_light/sum | 0.802 | 1.204 | 2.41 | 5398.1 |
| stage/render/prepare_write_phase_buffers | 0.585 | 0.881 | 1.60 | 3933.9 |
| bigspace/total/ms | 0.548 | 1.302 | 2.30 | 3687.1 |
| stage/render/prepare_assets | 0.501 | 0.642 | 10.53 | 3370.3 |
| stage/main/fixed_main_loop | 0.496 | 0.923 | 2.01 | 3337.9 |
| stage/render/prepare_meshes | 0.422 | 0.525 | 3.24 | 2841.9 |
| render/present/ms | 0.419 | 1.178 | 1.96 | 2817.6 |
| stage/render/prepare_resources | 0.406 | 0.478 | 0.95 | 2728.8 |
| stage/main/pre_update | 0.382 | 0.553 | 3.08 | 2570.6 |
| stage/main/first | 0.368 | 0.485 | 5.40 | 2475.5 |
| controller/feed/ms | 0.317 | 0.400 | 0.81 | 2134.9 |
| stage/render/prepare_flush | 0.316 | 0.528 | 1.10 | 2126.7 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.302 | 1.158 | 2.07 | 2032.9 |
| stage/render/prepare_bind_groups | 0.294 | 0.477 | 2.38 | 1978.4 |
| stage/render/prepare_batch_phases | 0.263 | 0.394 | 0.73 | 1767.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.261 | 0.432 | 1.03 | 1757.2 |
| stage/main/last | 0.252 | 0.295 | 1.68 | 1697.5 |
| bigspace/low_precision/ms | 0.240 | 0.767 | 1.93 | 1615.7 |
| stage/render/create_views | 0.197 | 0.267 | 0.56 | 1322.3 |
| stream/drive/ms | 0.196 | 0.269 | 10.27 | 1318.2 |
| stage/render/extract_commands | 0.180 | 0.210 | 0.46 | 1208.7 |
| anim/evaluate/ms | 0.164 | 0.312 | 1.46 | 1106.1 |
| bigspace/high_precision/ms | 0.162 | 0.655 | 1.97 | 1092.7 |
| anim/evaluate_cpu/ms | 0.152 | 0.184 | 0.66 | 1021.9 |
| stage/render/specialize | 0.152 | 0.242 | 1.75 | 1019.4 |
| stream/tick/ms | 0.145 | 0.208 | 0.53 | 977.9 |
| stage/render/prepare_views | 0.145 | 0.192 | 1.34 | 972.4 |
| render/acquire/ms | 0.105 | 0.136 | 1.31 | 704.5 |
| stage/render/queue | 0.095 | 0.167 | 1.77 | 636.4 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.091 | 0.845 | 1.50 | 611.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.088 | 0.138 | 0.71 | 590.5 |
| stage/render/phase_sort | 0.079 | 0.108 | 0.37 | 529.0 |
| controller/write/ms | 0.074 | 0.101 | 0.40 | 495.0 |
| controller/move/ms | 0.068 | 0.193 | 0.62 | 455.2 |
| terrain/update/ms | 0.058 | 0.287 | 1.90 | 392.0 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.050 | 0.000 | 3.05 | 333.8 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.070 | 0.30 | 303.3 |
| physics/step/ms | 0.042 | 0.098 | 0.56 | 279.3 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.43 | 206.5 |
| stage/render/cleanup | 0.023 | 0.031 | 0.46 | 153.8 |
| physics/insert/ms | 0.023 | 0.282 | 0.85 | 153.0 |
| pipelines/compile_ms | 0.022 | 0.000 | 69.38 | 146.5 |
| terrain/select/ms | 0.021 | 0.254 | 0.87 | 144.7 |
| pipelines/front_ms | 0.021 | 0.000 | 66.97 | 141.5 |
| weather/map/ms | 0.017 | 0.024 | 0.27 | 115.9 |
| far/select/ms | 0.014 | 0.162 | 0.46 | 94.1 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.021 | 0.48 | 83.8 |
| anim/apply/ms | 0.011 | 0.019 | 0.42 | 76.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.06 | 69.5 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.58 | 68.5 |
| script/update/ms | 0.008 | 0.011 | 0.22 | 52.1 |
| controller/avoid/ms | 0.006 | 0.012 | 0.40 | 39.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.26 | 35.7 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 33.2 |
| script/ui/ms | 0.003 | 0.005 | 0.04 | 21.9 |
| stream/spawn/ms | 0.003 | 0.000 | 1.11 | 21.4 |
| stream/upload/ms | 0.003 | 0.000 | 0.95 | 18.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 12.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.07 | 6.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.6 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.42 | 5.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.42 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.5 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1313 | 2262 | 641 |
| crowd/c0/meshes/count | count | 357 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5997 | 5555 |
| crowd/c1/meshes/count | count | 1181 | 1299 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 204 |
| crowd/joints_extracted/count | count | 694 | 7057 | 559 |
| crowd/joints_uploaded/count | count | 3603 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.38 | 1.19 | 0.59 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.48 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.58 | 0.02 |
| encode/finish_critical | ms | 0.97 | 3.05 | 1.31 |
| encode/pass:early prepass/max | ms | 0.82 | 2.33 | 0.62 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 2.23 | 0.62 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.71 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 1.03 | 0.27 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.78 | 2.07 | 0.80 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.38 | 1.50 | 0.17 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.16 | 3.05 | 1.00 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.41 | 1.31 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.43 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.50 | 30.00 | 8.99 |
| gpu/timed/ms | ms | 7.78 | 9.49 | 8.67 |
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
| phases/alpha_mask3d/items | count | 322 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 322 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2481 | 3554 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2481 | 3554 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 69 | 498 | 27 |
| phases/shadow/bins | count | 874 | 1576 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 98.3 k |
| phases/shadow/items | count | 2349 | 3352 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 54 | 148 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2679 | 3182 | 2346 |
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
| preprocess/bind_groups | count | 15 | 271 | 43 |
| preprocess/bind_groups_created | count | 10 | 264 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 6 |
| preprocess/bind_groups_reused | count | 137 | 216 | 123 |
| process/cpu_usage | % | 25.30 | 39.29 | 25 |
| process/mem_usage | GiB | 5.17 | 5.41 | 5.41 |
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
| shadow_atlas/live_occ_pct | % | 11.52 | 37.40 | 0 |
| shadow_atlas/live_tiles | count | 18 | 67 | 0 |
| shadow_atlas/mover_draws | count | 10 | 38 | 0 |
| shadow_atlas/movers | count | 144 | 468 | 0 |
| shadow_atlas/moving_lamps | count | 1 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 38 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 19 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 38 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 61 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 107 | 0 |
| shadow_atlas/views | count | 17 | 60 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 121 | 11 |
| skin/blocks_skipped/count | count | 144 | 208 | 192 |
| skin/blocks_walked/count | count | 14 | 29 | 12 |
| skin/joint_bytes_written/bytes | bytes | 48.9 k | 808.7 k | 36.4 k |
| skin/joints_written/count | count | 764 | 12.6 k | 568 |
| skin/upload_writes/count | count | 8 | 27 | 11 |
| started_load_count |  | 6441 | 6565 | 6565 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.03 | 0.07 | 0.06 |
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
| system/cpu_usage | % | 29.31 | 41.09 | 35.10 |
| system/mem_usage | % | 46.54 | 47.26 | 47.26 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 128 | 553 | 85 |
| views/all/bins | count | 2938 | 3583 | 2948 |
| views/all/count | count | 20 | 64 | 3 |
| views/all/instance_batches | count | 12 | 22 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 196.6 k |
| views/all/items | count | 8063 | 10.6 k | 7956 |
| views/all/pipelines | count | 101 | 197 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 230.7 k | 205.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 27 |
| views/cascade/bins | count | 715 | 851 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2122 | 2454 | 2014 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.8 k | 100.3 k |
| views/lamp_face/batch_sets | count | 57 | 466 | 0 |
| views/lamp_face/bins | count | 121 | 738 | 0 |
| views/lamp_face/count | count | 17 | 61 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 176 | 906 | 0 |
| views/lamp_face/pipelines | count | 29 | 122 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 121 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7528 | 5942 |
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
| anim/bones_written/count | count | 49.83 | 208 | 335.3 k |
| anim/events/count | count | 0.09 | 3 | 582 |
| assets/Image/added | count | 0.09 | 33 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 258 | 2874 |
| assets/Mesh/removed | count | 0.72 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.11 | 10 | 7475 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.22 | 1260 | 1489 |
| encode//count | count | 27.90 | 30 | 187.7 k |
| encode/bind_walk/init_textures | count | 9116 | 23.1 k | 61.33 M |
| encode/bind_walk/init_textures_skipped | count | 53.1 k | 73.3 k | 357.37 M |
| encode/bind_walk/inits | count | 569 | 773 | 3.83 M |
| encode/bind_walk/inits_skipped | count | 172 | 243 | 1.16 M |
| encode/bind_walk/merge_textures | count | 9054 | 23.0 k | 60.91 M |
| encode/bind_walk/merge_textures_skipped | count | 10.1 k | 26.5 k | 67.88 M |
| encode/bind_walk/merges | count | 106 | 187 | 715.9 k |
| encode/bind_walk/merges_skipped | count | 153 | 228 | 1.03 M |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6728 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6728 |
| encode/pass:early prepass/count | count | 1 | 1 | 6728 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6728 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6728 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6728 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.36 | 1 | 2418 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.17 | 1 | 1134 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 275 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6728 |
| encode/queue/command_buffers | count | 38.48 | 42 | 258.9 k |
| encode/queue/submits | count | 2.01 | 4 | 13.5 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.26 M | 15.64 G |
| encode/queue/write_buffer/calls | count | 276 | 467 | 1.86 M |
| encode/queue/write_buffer_with/bytes | bytes | 171.1 k | 1.29 M | 1.15 G |
| encode/queue/write_buffer_with/calls | count | 9.27 | 782 | 62.4 k |
| encode/queue/write_texture/bytes | bytes | 1961 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6728 |
| physics/inserts | count | 0.33 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
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
| stream/deactivate/items | count | 0.03 | 35 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5032 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1103 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 46, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.32 | 2319.6 | main/busy/ms 2291, stage/main/post_update 2248, stage/main/update 31.99, stage/main/pre_update 6.74, surface/advance_cpu_ms 1.09, stage/main/last 1.02, stream/drive/ms 0.51, stage/main/first 0.48, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.28, stream/tick/ms 0.13, bigspace/total/ms 0.07 |
| warm-up | 3.05 | 730.1 | main/busy/ms 2424, stage/main/post_update 2277, render/thread/ms 697, render/busy/ms 558, stage/render/prepare_assets 507, stage/render/prepare_views 137, render/acquire/ms 137, stage/main/update 131, stage/render/create_views 29.00, stage/render/render 11.90, stage/main/pre_update 8.45, render/graph/ms 7.82 |
| warm-up | 3.15 | 104.8 | render/thread/ms 784, render/busy/ms 595, stage/render/prepare_assets 529, main/busy/ms 154, stage/render/prepare_views 138, render/acquire/ms 137, stage/main/update 104, pipelines/compile_ms 99.40, stage/render/render 67.70, pipelines/front_ms 61.43, render/present/ms 51.37, stage/main/post_update 38.70 |
| warm-up | 3.31 | 136.7 | pipelines/wait_ms 4215, pipelines/compile_ms 1734, pipelines/front_ms 1621, render/thread/ms 133, render/busy/ms 128, pipelines/driver_ms 113, main/busy/ms 87.62, encode//sum 60.10, stage/main/post_update 56.47, stage/render/prepare_write_phase_buffers 42.02, stage/render/render 34.44, render/graph/ms 28.68 |
| warm-up | 3.45 | 118.0 | pipelines/compile_ms 982, pipelines/front_ms 932, pipelines/wait_ms 292, main/busy/ms 93.11, render/thread/ms 81.42, render/busy/ms 79.62, stage/main/update 66.35, stage/render/prepare_assets 56.40, pipelines/driver_ms 50.07, stage/main/post_update 19.93, terrain/update/ms 14.55, stage/render/render 10.78 |
| warm-up | 3.50 | 47.4 | pipelines/compile_ms 1600, pipelines/front_ms 1524, pipelines/wait_ms 307, render/thread/ms 106, render/busy/ms 104, stage/render/prepare_assets 80.27, pipelines/driver_ms 75.99, main/busy/ms 47.14, stage/main/post_update 17.56, stage/render/render 15.60, stage/main/update 15.17, encode//sum 13.25 |
| warm-up | 3.60 | 33.4 | pipelines/compile_ms 89.13, pipelines/front_ms 85.07, main/busy/ms 45.06, render/thread/ms 42.25, render/busy/ms 40.43, stage/main/update 29.20, stage/render/render 17.15, render/graph/ms 15.15, stage/render/prepare_assets 13.43, stage/render/graph_submit 10.16, stage/main/post_update 9.63, terrain/update/ms 8.40 |
| warm-up | 3.65 | 58.4 | pipelines/compile_ms 1089, pipelines/front_ms 1044, pipelines/wait_ms 115, main/busy/ms 88.56, render/thread/ms 80.03, render/busy/ms 78.03, stage/main/update 75.30, stage/render/prepare_assets 51.09, pipelines/driver_ms 44.43, stage/render/render 15.32, render/graph/ms 12.90, stage/render/graph_submit 9.77 |
| warm-up | 3.73 | 76.4 | pipelines/compile_ms 1844, pipelines/front_ms 1763, pipelines/wait_ms 126, render/thread/ms 86.57, render/busy/ms 84.59, pipelines/driver_ms 80.56, main/busy/ms 73.92, stage/main/update 63.65, stage/render/prepare_assets 59.42, stage/render/render 18.88, render/graph/ms 16.39, stage/render/graph_submit 13.25 |
| warm-up | 3.82 | 86.2 | pipelines/compile_ms 1458, pipelines/front_ms 1368, render/thread/ms 102, render/busy/ms 100, pipelines/driver_ms 90.11, stage/render/prepare_assets 82.07, pipelines/wait_ms 16.67, main/busy/ms 16.21, stage/render/render 14.40, render/graph/ms 12.85, stage/render/graph_submit 10.57, stage/main/update 6.41 |
| warm-up | 3.91 | 29.8 | pipelines/compile_ms 826, pipelines/front_ms 791, render/thread/ms 36.98, render/busy/ms 35.04, pipelines/driver_ms 34.92, main/busy/ms 31.63, stage/main/update 16.24, stage/render/render 13.04, stage/render/prepare_assets 12.84, render/graph/ms 10.51, pipelines/wait_ms 8.69, stage/main/post_update 8.24 |
| warm-up | 3.97 | 52.3 | pipelines/compile_ms 204, pipelines/front_ms 198, render/thread/ms 61.26, render/busy/ms 59.49, main/busy/ms 44.56, stage/render/prepare_assets 32.77, stage/main/post_update 17.93, stage/render/render 15.10, stage/main/update 14.92, render/graph/ms 12.74, stage/render/graph_submit 9.46, encode//sum 9.18 |
| warm-up | 4.02 | 56.3 | pipelines/compile_ms 250, pipelines/front_ms 242, main/busy/ms 140, stage/main/update 113, render/thread/ms 80.22, render/busy/ms 78.75, stage/render/prepare_assets 48.56, stage/main/post_update 16.07, stage/render/render 12.66, render/graph/ms 10.71, stage/render/graph_submit 8.01, encode//sum 7.93 |
| warm-up | 4.19 | 164.7 | pipelines/compile_ms 2322, pipelines/front_ms 2260, main/busy/ms 149, stage/main/update 136, render/thread/ms 107, render/busy/ms 105, pipelines/driver_ms 62.28, stage/render/prepare_assets 59.83, stage/render/render 32.66, render/graph/ms 30.45, stage/render/camera_driver 19.13, stage/render/graph_submit 10.82 |
| warm-up | 4.24 | 57.9 | pipelines/compile_ms 2120, pipelines/front_ms 2063, render/thread/ms 109, render/busy/ms 106, stage/render/prepare_assets 58.59, pipelines/driver_ms 56.50, main/busy/ms 51.53, stage/main/update 37.23, stage/render/render 35.95, render/graph/ms 33.38, stage/render/camera_driver 19.45, stage/render/graph_submit 13.45 |
| warm-up | 4.32 | 77.5 | pipelines/compile_ms 172, pipelines/front_ms 165, render/thread/ms 118, render/busy/ms 116, main/busy/ms 52.81, stage/render/prepare_assets 38.72, stage/main/update 27.45, stage/render/prepare_resources 25.14, stage/render/render 13.31, stage/render/prepare_flush 13.26, stage/render/prepare_write_phase_buffers 12.01, stream/drive/ms 11.53 |
| warm-up | 4.37 | 51.9 | pipelines/compile_ms 2232, pipelines/front_ms 2171, pipelines/wait_ms 543, render/thread/ms 123, render/busy/ms 121, pipelines/driver_ms 61.40, main/busy/ms 51.74, stage/render/prepare_resources 30.65, stage/render/prepare_write_phase_buffers 30.43, stage/main/update 27.82, stage/render/prepare_assets 25.38, stage/render/prepare_flush 13.47 |
| warm-up | 4.46 | 59.0 | pipelines/compile_ms 530, pipelines/front_ms 506, pipelines/wait_ms 373, main/busy/ms 104, stage/main/update 59.12, render/thread/ms 55.49, render/busy/ms 51.19, stage/render/render 27.75, pipelines/driver_ms 24.06, render/graph/ms 22.73, stream/drive/ms 20.65, stage/main/post_update 15.15 |
| warm-up | 4.57 | 109.9 | pipelines/compile_ms 1645, pipelines/front_ms 1580, pipelines/wait_ms 357, main/busy/ms 113, render/thread/ms 82.82, render/busy/ms 78.57, stage/main/update 67.92, pipelines/driver_ms 65.00, stage/render/prepare_assets 40.60, stage/render/render 28.08, render/graph/ms 23.16, pipelines/lock_ms 15.16 |
| warm-up | 4.67 | 106.6 | pipelines/compile_ms 1985, pipelines/front_ms 1927, pipelines/wait_ms 336, render/thread/ms 122, render/busy/ms 120, stage/render/prepare_assets 74.71, main/busy/ms 68.47, pipelines/driver_ms 58.81, stage/main/update 44.38, stage/render/prepare_resources 24.82, stage/render/render 12.67, pipelines/lock_ms 11.16 |
| warm-up | 4.71 | 38.9 | pipelines/compile_ms 884, pipelines/front_ms 865, render/thread/ms 114, render/busy/ms 112, main/busy/ms 77.79, stage/main/update 47.59, stage/render/prepare_assets 42.05, stage/render/render 25.47, stream/drive/ms 25.38, stage/render/prepare_resources 24.71, render/graph/ms 22.79, pipelines/driver_ms 19.12 |
| warm-up | 4.76 | 44.3 | pipelines/compile_ms 515, pipelines/front_ms 497, main/busy/ms 109, stage/main/update 86.01, render/thread/ms 61.70, render/busy/ms 59.45, stream/drive/ms 44.20, stage/render/render 26.53, render/graph/ms 23.71, stage/render/graph_submit 19.20, pipelines/driver_ms 17.99, stage/main/post_update 13.78 |
| warm-up | 4.84 | 81.2 | pipelines/compile_ms 760, pipelines/front_ms 737, main/busy/ms 101, stage/main/update 85.44, render/thread/ms 60.34, render/busy/ms 58.83, stream/drive/ms 46.92, pipelines/driver_ms 23.83, stage/render/prepare_assets 21.90, stage/render/render 16.24, render/graph/ms 14.08, stage/render/graph_submit 9.88 |
| warm-up | 4.87 | 36.6 | pipelines/compile_ms 899, pipelines/front_ms 870, main/busy/ms 62.71, render/thread/ms 57.19, render/busy/ms 55.75, stage/main/update 45.42, pipelines/driver_ms 29.10, stream/drive/ms 27.76, stage/render/prepare_assets 22.11, stage/render/render 13.95, render/graph/ms 11.76, stage/main/post_update 9.57 |
| warm-up | 4.93 | 35.7 | pipelines/compile_ms 355, pipelines/front_ms 345, render/thread/ms 55.41, render/busy/ms 53.78, main/busy/ms 42.31, stage/main/update 21.11, stage/main/post_update 14.89, stage/render/render 13.41, render/graph/ms 11.41, pipelines/driver_ms 10.78, stage/render/prepare_assets 10.73, encode//sum 8.01 |
| warm-up | 4.99 | 30.2 | pipelines/compile_ms 230, pipelines/front_ms 224, render/thread/ms 41.07, render/busy/ms 39.03, main/busy/ms 32.12, stage/main/update 16.56, stage/render/render 12.76, stage/render/prepare_assets 12.40, render/graph/ms 10.92, stage/main/post_update 9.47, stage/render/graph_submit 7.64, encode//sum 6.94 |
| warm-up | 5.11 | 25.4 | render/thread/ms 33.47, render/busy/ms 31.96, main/busy/ms 31.57, pipelines/compile_ms 28.66, pipelines/front_ms 27.36, stage/main/update 16.87, stage/render/render 13.38, render/graph/ms 11.48, stage/main/post_update 10.58, stream/drive/ms 8.82, stage/render/graph_submit 8.19, encode//sum 7.04 |
| warm-up | 5.38 | 29.0 | pipelines/compile_ms 112, pipelines/front_ms 105, main/busy/ms 42.75, render/thread/ms 33.23, render/busy/ms 31.52, stage/main/update 27.91, stage/render/render 17.62, stream/drive/ms 16.68, render/graph/ms 15.43, stage/render/graph_submit 11.60, stage/main/post_update 9.61, pipelines/driver_ms 7.77 |
| warm-up | 5.48 | 25.7 | render/thread/ms 38.89, render/busy/ms 37.56, pipelines/compile_ms 29.47, pipelines/front_ms 27.79, main/busy/ms 27.40, stage/render/render 15.16, render/graph/ms 13.01, stage/main/update 11.02, stage/main/post_update 10.65, stage/render/graph_submit 8.74, encode//sum 8.06, encode/submit 6.17 |
| warm-up | 5.60 | 25.1 | pipelines/compile_ms 407, pipelines/front_ms 396, main/busy/ms 37.39, render/thread/ms 30.62, render/busy/ms 29.09, stage/main/update 19.76, stage/render/render 16.16, render/graph/ms 14.05, pipelines/driver_ms 10.59, stage/render/graph_submit 9.87, stage/main/post_update 9.35, encode//sum 8.66 |
| warm-up | 5.69 | 31.0 | pipelines/compile_ms 314, pipelines/front_ms 304, main/busy/ms 40.64, render/thread/ms 27.12, render/busy/ms 25.73, stage/main/update 24.15, stream/drive/ms 13.90, stage/render/render 11.04, stage/main/post_update 10.25, pipelines/driver_ms 9.76, render/graph/ms 9.39, encode//sum 7.19 |
| warm-up | 5.79 | 29.4 | pipelines/compile_ms 302, pipelines/front_ms 293, render/thread/ms 44.50, render/busy/ms 42.96, main/busy/ms 39.05, stage/render/render 17.43, stage/main/update 17.10, render/graph/ms 15.09, stage/main/post_update 14.52, stage/render/prepare_assets 14.12, stage/render/graph_submit 11.08, encode//sum 9.51 |
| warm-up | 5.82 | 32.1 | pipelines/compile_ms 302, pipelines/front_ms 293, render/thread/ms 44.67, render/busy/ms 43.68, main/busy/ms 30.62, stage/render/prepare_assets 23.04, stage/main/update 13.30, stage/render/render 10.11, stage/main/post_update 9.03, pipelines/driver_ms 8.97, render/graph/ms 8.49, encode//sum 7.13 |
| warm-up | 5.93 | 25.1 | render/thread/ms 39.18, render/busy/ms 37.97, main/busy/ms 36.46, stage/main/update 21.89, stage/render/render 11.76, stage/render/prepare_assets 11.03, render/graph/ms 10.12, stage/main/post_update 8.29, stage/render/graph_submit 7.02, encode//sum 6.73, stage/render/prepare_meshes 5.84, encode/submit 4.85 |
| warm-up | 6.00 | 26.6 | render/thread/ms 37.16, render/busy/ms 36.08, main/busy/ms 34.02, stage/main/update 20.94, stage/render/render 14.39, render/graph/ms 12.92, stage/render/graph_submit 9.11, encode//sum 8.86, stage/main/post_update 8.48, stage/render/prepare_assets 8.37, encode/finish_wall 5.06, stage/render/prepare_meshes 4.69 |
| warm-up | 6.04 | 28.0 | pipelines/compile_ms 135, pipelines/front_ms 131, main/busy/ms 36.48, render/thread/ms 36.36, render/busy/ms 34.74, stage/main/update 16.61, stage/render/render 16.33, stage/main/post_update 14.15, render/graph/ms 14.05, encode//sum 9.27, stage/render/graph_submit 9.25, stage/render/prepare_assets 5.77 |
| warm-up | 6.11 | 45.4 | render/thread/ms 51.75, render/busy/ms 50.57, main/busy/ms 43.75, pipelines/compile_ms 27.31, stage/main/update 26.49, pipelines/front_ms 25.86, stage/render/prepare_assets 17.08, stage/render/render 15.89, render/graph/ms 14.27, stage/render/graph_submit 10.18, stage/main/post_update 9.34, encode//sum 7.73 |
| warm-up | 6.15 | 37.4 | render/thread/ms 65.54, render/busy/ms 63.74, main/busy/ms 41.46, stage/render/render 22.87, stage/main/update 21.29, render/graph/ms 20.48, stage/render/prepare_assets 18.61, stage/render/graph_submit 16.25, stage/main/post_update 12.24, pipelines/compile_ms 12.18, pipelines/front_ms 11.52, encode/finish_wall 11.25 |
| warm-up | 6.18 | 28.4 | render/thread/ms 56.82, render/busy/ms 55.03, main/busy/ms 38.03, stage/render/render 21.98, render/graph/ms 19.39, stage/main/update 18.17, stage/render/graph_submit 15.15, stage/render/prepare_assets 12.76, stage/main/post_update 12.09, encode/finish_wall 10.41, encode/pass:point_shadow_atlas_live_indirect/sum 10.00, encode//sum 9.23 |
| warm-up | 6.21 | 36.6 | render/thread/ms 49.25, render/busy/ms 48.23, main/busy/ms 32.34, stage/render/prepare_assets 19.16, stage/main/update 16.54, stage/render/render 14.98, render/graph/ms 13.29, stage/render/graph_submit 10.54, stage/main/post_update 9.02, encode/finish_wall 7.30, encode/pass:point_shadow_atlas_live_indirect/sum 6.96, encode//sum 6.95 |
| warm-up | 6.37 | 38.1 | main/busy/ms 49.22, render/thread/ms 42.73, render/busy/ms 41.29, stage/main/update 31.84, stage/render/render 19.13, render/graph/ms 16.71, stream/drive/ms 13.38, stage/render/graph_submit 12.71, stage/main/post_update 12.49, encode//sum 8.24, encode/finish_wall 7.11, encode/pass:point_shadow_atlas_live_indirect/sum 6.70 |
| warm-up | 6.41 | 40.9 | render/thread/ms 62.39, render/busy/ms 61.16, main/busy/ms 56.26, stage/main/update 32.10, stage/render/render 24.79, render/graph/ms 22.39, stage/main/post_update 17.37, stage/render/graph_submit 17.09, stage/render/prepare_assets 14.66, encode/finish_wall 11.75, encode//sum 10.63, encode/pass:point_shadow_atlas_static_capture/sum 7.89 |
| warm-up | 6.44 | 27.7 | pipelines/compile_ms 131, pipelines/front_ms 128, render/thread/ms 57.23, render/busy/ms 56.07, main/busy/ms 48.46, stage/main/update 27.01, stage/render/render 21.37, render/graph/ms 19.38, stage/render/prepare_assets 17.92, stage/render/graph_submit 15.45, stage/main/post_update 14.60, encode/finish_wall 10.65 |
| warm-up | 6.47 | 28.5 | pipelines/compile_ms 145, pipelines/front_ms 141, render/thread/ms 43.96, render/busy/ms 42.87, main/busy/ms 38.65, stage/main/update 17.76, stage/render/render 16.03, render/graph/ms 14.39, stage/render/prepare_assets 13.42, stage/main/post_update 13.00, stage/render/graph_submit 11.36, encode//sum 6.95 |
| measure | 18.32 | 30.4 | main/busy/ms 38.83, stage/main/update 25.63, render/thread/ms 21.94, render/busy/ms 21.00, stage/render/render 10.97, render/graph/ms 9.70, stage/main/post_update 8.40, stage/render/graph_submit 6.91, encode//sum 5.81, encode/submit 4.02, stage/render/prepare_assets 3.80, encode/finish_wall 2.93 |
| measure | 59.18 | 28.3 | render/thread/ms 35.36, render/busy/ms 34.31, main/busy/ms 25.01, stage/render/prepare_assets 15.24, stage/main/post_update 12.09, stage/render/render 10.07, render/graph/ms 8.45, encode//sum 7.76, stage/render/graph_submit 4.72, stage/main/update 4.65, stage/main/fixed_main_loop 3.96, stage/render/camera_driver 3.53 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=nocontact-2 --bench-baseline none --off contactshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 1%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **85%** (min 67%) · 2618 MHz (min 2610) · memory 8101 MHz · 85.3 W · 80 °C (max 85) · VRAM 5564 MiB · throttled 100% of samples

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
- `sg.post=high (preset: level requested)`
- `sg.shadows=custom (preset: derived from its members)`
- `sg.sky=high (preset: level requested)`
- `sg.textures=high (preset: level requested)`
- `sg.view_distance=high (preset: level requested)`
- `sg.water=high (preset: level requested)`
- `shadow.contact=off (command line: --off contactshadows)`
- `shadow.contact.length_m=0.3 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.steps=16 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.thickness_m=0.1 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.filter=auto (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.lamp.contact=off (command line: --off contactshadows)`
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
