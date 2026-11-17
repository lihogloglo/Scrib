# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5951 frames over 65.6 s · **median 10.69 ms (94 fps)** · mean 11.03 · p95 14.62 · p99 16.41 (1% low 61 fps) · worst 28.08 ms · 85 spikes over 1.5× median · 18.3 stutters/min (work 20.1/min)
- warm-up: 325 frames over 9.1 s, worst 2506.2 ms
- hitches (> 26.7 ms): **2** measured, 47 in warm-up
- timed GPU passes: 6.68 ms of a 11.03 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.69 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- pass worst frame 28.08 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.979 | 2.029 | 2.063 | 0.98 | 0.174 |
| main_opaque_pass_3d | 1.273 | 1.305 | 1.652 | 0.98 | 0.062 |
| early prepass | 0.477 | 0.489 | 0.702 | 0.98 | 0.086 |
| auxiliary_views | 0.346 | 0.355 | 1.193 | 0.98 | 0.316 |
| bloom | 0.308 | 0.316 | 0.322 | 0.98 | 0.063 |
| atmosphere_luts | 0.303 | 0.310 | 0.374 | 0.98 | 0.003 |
| water_velocity | 0.256 | 0.262 | 0.349 | 0.98 | 0.002 |
| water_sspr | 0.196 | 0.201 | 0.203 | 0.98 | 0.002 |
| ssao | 0.184 | 0.188 | 0.193 | 0.98 | 0.007 |
| upscaling | 0.126 | 0.130 | 0.137 | 0.98 | 0.004 |
| water_fft | 0.126 | 0.129 | 0.138 | 0.98 | 0.003 |
| atmos_froxel_inject | 0.124 | 0.127 | 0.128 | 0.98 | 0.001 |
| uniform_allocation | 0.124 | 0.127 | 0.155 | 0.98 | 0.025 |
| clouds_composite | 0.102 | 0.105 | 0.108 | 0.98 | 0.002 |
| bin_unpacking | 0.096 | 0.098 | 0.121 | 0.98 | 0.018 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.076 | 0.078 | 0.155 | 0.98 | 0.008 |
| clustering | 0.062 | 0.064 | 0.067 | 0.98 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.075 | 0.98 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.98 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.413 | 0.415 | 0.11 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.037 | 0.98 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.98 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.219 | 0.230 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.98 | 0.002 |
| night_sky | 0.016 | 0.017 | 0.041 | 0.98 | 0.001 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.026 | 0.98 | 0.125 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.98 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.694 | 13.207 | 24.56 | 57691.9 |
| render/busy/ms | 9.181 | 12.659 | 23.87 | 54636.4 |
| main/busy/ms | 8.984 | 11.006 | 25.14 | 53466.0 |
| stage/render/render | 5.945 | 8.747 | 15.94 | 35378.4 |
| stage/main/post_update | 5.621 | 7.271 | 9.20 | 33451.2 |
| render/graph/ms | 5.247 | 7.963 | 14.93 | 31226.5 |
| encode//sum | 4.340 | 7.363 | 21.41 | 25825.9 |
| stage/render/graph_submit | 3.906 | 5.881 | 10.34 | 23246.6 |
| encode/submit | 2.532 | 3.925 | 6.14 | 15065.7 |
| stage/main/update | 2.123 | 2.574 | 18.92 | 12635.4 |
| encode/pass:shadow_point_light/sum | 1.781 | 5.526 | 16.80 | 10599.2 |
| encode/finish_wall | 1.412 | 2.011 | 4.38 | 8400.7 |
| stage/render/camera_driver | 1.287 | 2.060 | 4.55 | 7659.7 |
| stage/render/prepare_write_phase_buffers | 0.780 | 1.132 | 2.48 | 4640.7 |
| encode/pass:shadow_directional_light/sum | 0.734 | 1.238 | 2.59 | 4370.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.728 | 1.073 | 1.82 | 4332.0 |
| encode/pass:early prepass/sum | 0.715 | 1.070 | 1.95 | 4257.9 |
| stage/render/prepare_assets | 0.572 | 0.720 | 9.33 | 3403.3 |
| bigspace/total/ms | 0.536 | 1.361 | 2.43 | 3189.5 |
| stage/render/prepare_resources | 0.447 | 0.533 | 1.38 | 2659.5 |
| render/present/ms | 0.419 | 0.523 | 2.29 | 2492.7 |
| stage/render/prepare_meshes | 0.408 | 0.521 | 2.33 | 2426.8 |
| stage/main/first | 0.389 | 0.516 | 5.07 | 2315.6 |
| stage/main/pre_update | 0.348 | 0.491 | 1.83 | 2070.8 |
| terrain/update/ms | 0.310 | 0.397 | 2.63 | 1845.3 |
| terrain/select/ms | 0.277 | 0.355 | 0.96 | 1646.0 |
| stage/render/prepare_bind_groups | 0.262 | 0.449 | 2.30 | 1561.4 |
| controller/feed/ms | 0.257 | 0.320 | 0.99 | 1532.2 |
| stage/render/specialize | 0.254 | 0.386 | 2.28 | 1514.2 |
| stream/drive/ms | 0.245 | 0.326 | 16.85 | 1457.9 |
| stage/render/queue | 0.237 | 0.378 | 2.16 | 1408.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.234 | 0.343 | 1.05 | 1392.7 |
| bigspace/low_precision/ms | 0.218 | 0.642 | 1.93 | 1294.5 |
| stage/main/fixed_main_loop | 0.215 | 0.360 | 0.98 | 1277.1 |
| far/select/ms | 0.199 | 0.269 | 0.51 | 1186.3 |
| stage/render/create_views | 0.190 | 0.245 | 0.94 | 1132.5 |
| stream/tick/ms | 0.189 | 0.258 | 0.79 | 1126.5 |
| stage/render/prepare_batch_phases | 0.186 | 0.264 | 0.62 | 1109.5 |
| stage/render/extract_commands | 0.171 | 0.200 | 0.44 | 1015.1 |
| anim/evaluate/ms | 0.168 | 0.280 | 2.17 | 998.8 |
| stage/render/prepare_flush | 0.164 | 0.336 | 0.97 | 974.3 |
| anim/evaluate_cpu/ms | 0.159 | 0.204 | 0.68 | 947.8 |
| bigspace/high_precision/ms | 0.154 | 0.623 | 1.72 | 916.4 |
| stage/render/prepare_views | 0.137 | 0.171 | 0.46 | 815.2 |
| controller/move/ms | 0.126 | 0.219 | 0.58 | 750.6 |
| stage/main/last | 0.114 | 0.156 | 1.54 | 677.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.097 | 0.156 | 0.75 | 576.1 |
| render/acquire/ms | 0.095 | 0.121 | 0.40 | 562.7 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.128 | 0.55 | 358.8 |
| physics/step/ms | 0.047 | 0.084 | 0.55 | 277.6 |
| physics/insert/ms | 0.034 | 0.370 | 0.89 | 205.1 |
| stage/render/phase_sort | 0.033 | 0.057 | 0.25 | 197.5 |
| stage/render/cleanup | 0.032 | 0.096 | 0.84 | 188.8 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.54 | 176.8 |
| pipelines/compile_ms | 0.017 | 0.000 | 89.34 | 101.1 |
| pipelines/front_ms | 0.016 | 0.000 | 86.23 | 97.8 |
| weather/map/ms | 0.016 | 0.021 | 0.21 | 94.7 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.06 | 83.2 |
| anim/apply/ms | 0.011 | 0.021 | 0.16 | 68.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.47 | 63.5 |
| controller/avoid/ms | 0.010 | 0.014 | 0.17 | 62.5 |
| weather/eval/ms | 0.010 | 0.013 | 0.23 | 58.7 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 40.3 |
| script/input/ms | 0.005 | 0.006 | 0.20 | 31.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.09 | 30.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 28.2 |
| script/ui/ms | 0.005 | 0.006 | 0.55 | 27.8 |
| controller/write/ms | 0.003 | 0.004 | 0.13 | 20.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.68 | 14.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.8 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.12 | 3.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 42 | 13 |
| crowd/c0/joints/count | count | 1247 | 2204 | 583 |
| crowd/c0/meshes/count | count | 349 | 634 | 162 |
| crowd/c1/count | count | 147 | 156 | 148 |
| crowd/c1/joints/count | count | 5584 | 5992 | 5613 |
| crowd/c1/meshes/count | count | 1183 | 1308 | 1179 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 215 | 204 |
| crowd/joints_extracted/count | count | 3677 | 7862 | 5731 |
| crowd/joints_uploaded/count | count | 3636 | 6186 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.40 | 1.24 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.06 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.47 | 0.02 |
| encode/finish_critical | ms | 0.94 | 2.59 | 1.29 |
| encode/pass:early prepass/max | ms | 0.74 | 1.95 | 1.00 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.74 | 1.82 | 0.55 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.75 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 1.05 | 0.18 |
| encode/pass:shadow_directional_light/max | ms | 0.73 | 2.59 | 1.29 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.40 | 0.87 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.54 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.25 | 25.14 | 8.93 |
| gpu/timed/ms | ms | 7.50 | 10.80 | 7.73 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 786 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 786 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1294 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3339 | 5145 | 3193 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1294 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3339 | 5145 | 3193 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 197 | 19 |
| phases/shadow/bins | count | 3660 | 4504 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5844 | 7618 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 233 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 559 | 40 |
| preprocess/bind_groups_created | count | 14 | 559 | 40 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 136 | 301 | 124 |
| process/cpu_usage | % | 26.71 | 39.64 | 35.29 |
| process/mem_usage | GiB | 5.45 | 5.62 | 5.62 |
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
| skin/blocks_changed/count | count | 13 | 90 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 215 | 204 |
| skin/joint_bytes_written/bytes | bytes | 283.6 k | 808.2 k | 452.9 k |
| skin/joints_written/count | count | 4432 | 12.6 k | 7076 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4920 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 431 | 528 | 396 |
| statics/materials_bound | count | 431 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.49 | 39.96 | 30.66 |
| system/mem_usage | % | 54.36 | 54.91 | 54.86 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 72 | 248 | 71 |
| views/all/bins | count | 6194 | 7359 | 4989 |
| views/all/count | count | 45 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 20.4 k | 10.6 k |
| views/all/pipelines | count | 251 | 281 | 67 |
| views/all/pipelines_distinct | count | 71 | 75 | 63 |
| views/all/work_items | count | 10.5 k | 18.4 k | 11.0 k |
| views/cascade/batch_sets | count | 9 | 27 | 19 |
| views/cascade/bins | count | 1445 | 1557 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3385 | 4437 | 3025 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 330 | 4434 | 3025 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2287 | 2993 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2505 | 3392 | 0 |
| views/lamp_face/pipelines | count | 180 | 207 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2849 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2890 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8246 | 13.1 k | 7622 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8415 | 13.3 k | 7930 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 58.23 | 292 | 346.5 k |
| anim/events/count | count | 0.09 | 4 | 539 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.64 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 60 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1263 | 4675 |
| encode//count | count | 41.35 | 219 | 246.1 k |
| encode/bind_walk/init_textures | count | 17.0 k | 112.9 k | 101.42 M |
| encode/bind_walk/init_textures_skipped | count | 70.7 k | 146.5 k | 420.88 M |
| encode/bind_walk/inits | count | 489 | 1122 | 2.91 M |
| encode/bind_walk/inits_skipped | count | 113 | 171 | 674.1 k |
| encode/bind_walk/merge_textures | count | 17.0 k | 112.9 k | 101.04 M |
| encode/bind_walk/merge_textures_skipped | count | 5397 | 6012 | 32.12 M |
| encode/bind_walk/merges | count | 110 | 339 | 656.0 k |
| encode/bind_walk/merges_skipped | count | 86.70 | 127 | 515.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5951 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5951 |
| encode/pass:early prepass/count | count | 1 | 1 | 5951 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5951 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5951 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5951 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5951 |
| encode/pass:shadow_point_light/count | count | 3.60 | 48 | 21.4 k |
| encode/queue/command_buffers | count | 54.96 | 277 | 327.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.0 k |
| encode/queue/write_buffer/bytes | bytes | 1.82 M | 2.96 M | 10.85 G |
| encode/queue/write_buffer/calls | count | 281 | 835 | 1.67 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.3 k | 3.52 M | 1.01 G |
| encode/queue/write_buffer_with/calls | count | 11.03 | 465 | 65.7 k |
| encode/queue/write_texture/bytes | bytes | 10.5 k | 8.39 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5951 |
| physics/inserts | count | 0.38 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
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
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5689 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1247 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.7 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2506.2 | main/busy/ms 2473, stage/main/post_update 2431, stage/main/update 32.14, stage/main/pre_update 5.65, stage/main/last 1.29, stream/drive/ms 0.77, surface/advance_cpu_ms 0.72, stage/main/first 0.51, stage/main/fixed_main_loop 0.34, surface/cell_upload_ms 0.32, stream/tick/ms 0.23, bigspace/total/ms 0.09 |
| warm-up | 3.27 | 767.0 | main/busy/ms 2620, stage/main/post_update 2455, render/thread/ms 729, render/busy/ms 600, stage/render/prepare_assets 550, stage/main/update 149, stage/render/prepare_views 126, render/acquire/ms 126, stage/render/create_views 29.82, stage/render/render 11.66, stage/main/pre_update 7.52, render/graph/ms 6.91 |
| warm-up | 3.38 | 104.3 | render/thread/ms 818, render/busy/ms 637, stage/render/prepare_assets 574, main/busy/ms 167, stage/render/prepare_views 126, render/acquire/ms 126, stage/main/update 121, pipelines/compile_ms 117, pipelines/front_ms 76.84, stage/render/render 68.02, render/present/ms 54.72, pipelines/driver_ms 40.03 |
| warm-up | 3.42 | 38.6 | pipelines/compile_ms 397, pipelines/front_ms 316, render/thread/ms 125, pipelines/wait_ms 106, stage/render/render 86.62, pipelines/driver_ms 80.82, render/busy/ms 68.97, main/busy/ms 62.92, render/present/ms 55.87, stage/main/post_update 47.65, pipelines/lock_ms 29.64, render/graph/ms 29.52 |
| warm-up | 3.49 | 72.5 | pipelines/wait_ms 1990, pipelines/compile_ms 852, pipelines/front_ms 768, render/thread/ms 101, render/busy/ms 94.82, main/busy/ms 89.94, pipelines/driver_ms 83.62, stage/main/post_update 57.17, pipelines/lock_ms 41.06, stage/render/render 33.98, render/graph/ms 27.44, stage/render/prepare_bind_groups 23.42 |
| warm-up | 3.58 | 91.6 | pipelines/wait_ms 5824, pipelines/compile_ms 1566, pipelines/front_ms 1451, render/thread/ms 154, render/busy/ms 152, main/busy/ms 115, pipelines/driver_ms 114, stage/render/prepare_resources 73.86, stage/main/update 62.81, stage/main/post_update 32.29, stage/render/prepare_assets 26.17, stage/render/prepare_bind_groups 23.70 |
| warm-up | 3.63 | 54.4 | pipelines/wait_ms 4039, pipelines/compile_ms 1386, pipelines/front_ms 1290, render/thread/ms 142, render/busy/ms 140, pipelines/driver_ms 96.18, main/busy/ms 83.66, stage/render/prepare_resources 65.24, stage/main/update 54.30, stage/render/prepare_assets 53.27, stage/main/post_update 18.64, encode//sum 16.52 |
| warm-up | 3.68 | 40.8 | pipelines/compile_ms 795, pipelines/front_ms 753, pipelines/wait_ms 115, render/thread/ms 83.61, render/busy/ms 81.62, stage/render/prepare_assets 60.93, pipelines/driver_ms 42.20, main/busy/ms 36.54, stage/main/post_update 16.32, stage/main/update 12.89, stage/render/render 12.67, terrain/update/ms 10.84 |
| warm-up | 3.72 | 44.9 | pipelines/compile_ms 1203, pipelines/front_ms 1146, pipelines/wait_ms 171, render/thread/ms 74.00, render/busy/ms 72.32, pipelines/driver_ms 57.22, main/busy/ms 48.50, stage/render/prepare_assets 42.57, stage/main/post_update 17.56, stage/render/render 16.69, stage/main/update 15.12, render/graph/ms 14.61 |
| warm-up | 3.77 | 28.4 | pipelines/compile_ms 783, pipelines/front_ms 754, pipelines/wait_ms 161, main/busy/ms 42.12, render/thread/ms 41.01, render/busy/ms 37.13, pipelines/driver_ms 28.97, stage/main/post_update 20.16, stage/main/update 17.18, stage/render/render 16.32, render/graph/ms 12.34, terrain/update/ms 11.28 |
| warm-up | 3.80 | 30.6 | pipelines/compile_ms 245, pipelines/front_ms 237, main/busy/ms 74.16, stage/main/update 57.52, render/thread/ms 51.12, render/busy/ms 46.87, stream/drive/ms 46.62, stage/render/prepare_assets 20.21, stage/render/render 17.06, render/graph/ms 12.59, terrain/update/ms 11.78, stage/main/post_update 11.70 |
| warm-up | 3.87 | 72.6 | pipelines/compile_ms 1194, pipelines/front_ms 1146, main/busy/ms 65.45, render/thread/ms 61.93, render/busy/ms 59.80, stage/main/update 51.19, pipelines/driver_ms 47.47, stream/drive/ms 46.64, stage/render/prepare_assets 41.14, pipelines/wait_ms 18.51, stage/render/render 13.18, render/graph/ms 10.61 |
| warm-up | 3.91 | 29.0 | pipelines/compile_ms 457, pipelines/front_ms 443, render/thread/ms 31.54, render/busy/ms 30.21, main/busy/ms 26.55, stage/render/prepare_assets 16.21, stage/main/update 14.49, pipelines/driver_ms 14.05, stream/drive/ms 10.49, stage/render/render 8.99, render/graph/ms 7.27, encode//sum 6.28 |
| warm-up | 3.97 | 32.1 | pipelines/compile_ms 118, pipelines/front_ms 109, render/thread/ms 50.26, render/busy/ms 48.69, main/busy/ms 34.09, stage/render/prepare_assets 18.53, stage/render/prepare_resources 15.65, stage/main/post_update 15.22, stage/main/update 12.31, pipelines/driver_ms 8.58, stage/render/render 8.45, render/graph/ms 6.82 |
| warm-up | 4.02 | 50.4 | pipelines/compile_ms 77.34, pipelines/front_ms 73.95, render/thread/ms 70.99, render/busy/ms 69.24, main/busy/ms 47.34, stage/render/prepare_assets 31.49, stage/main/update 21.77, stage/main/post_update 17.85, stage/render/prepare_resources 17.34, stage/render/render 10.44, render/graph/ms 8.44, encode//sum 6.34 |
| warm-up | 4.07 | 43.6 | render/thread/ms 71.84, render/busy/ms 69.98, pipelines/compile_ms 68.28, pipelines/front_ms 65.26, main/busy/ms 57.92, stage/main/update 40.09, stage/render/prepare_assets 38.22, stage/render/render 11.45, stage/main/post_update 10.96, render/graph/ms 9.19, stage/render/prepare_resources 8.11, stage/render/graph_submit 6.82 |
| warm-up | 4.17 | 104.2 | pipelines/compile_ms 2274, pipelines/front_ms 2208, pipelines/wait_ms 153, render/thread/ms 112, render/busy/ms 110, main/busy/ms 71.96, pipelines/driver_ms 65.88, stage/main/update 48.25, stage/render/prepare_assets 47.22, stage/render/render 29.53, render/graph/ms 26.95, stage/render/prepare_resources 16.66 |
| warm-up | 4.22 | 45.4 | pipelines/compile_ms 2358, pipelines/front_ms 2287, pipelines/wait_ms 196, render/thread/ms 120, render/busy/ms 118, pipelines/driver_ms 70.21, main/busy/ms 47.45, stage/render/prepare_assets 46.17, stage/render/render 38.22, render/graph/ms 35.58, encode//sum 34.77, stage/main/update 24.58 |
| warm-up | 4.30 | 81.7 | pipelines/compile_ms 808, pipelines/front_ms 774, render/thread/ms 115, render/busy/ms 112, pipelines/wait_ms 80.57, main/busy/ms 72.66, encode//sum 58.16, stage/render/render 36.70, pipelines/driver_ms 34.37, render/graph/ms 33.98, stage/main/update 32.84, stage/main/post_update 30.16 |
| warm-up | 4.37 | 73.1 | pipelines/compile_ms 1730, pipelines/front_ms 1671, render/thread/ms 137, render/busy/ms 135, main/busy/ms 94.35, pipelines/wait_ms 87.04, pipelines/driver_ms 58.67, stage/render/render 43.51, encode//sum 39.34, stage/main/post_update 36.52, stage/main/update 33.14, render/graph/ms 33.10 |
| warm-up | 4.50 | 125.5 | pipelines/compile_ms 3124, pipelines/front_ms 2856, pipelines/wait_ms 385, pipelines/driver_ms 268, render/thread/ms 166, render/busy/ms 164, main/busy/ms 108, stage/main/update 58.63, stage/render/create_views 54.62, stage/render/render 40.70, stage/render/prepare_assets 38.36, render/graph/ms 30.76 |
| warm-up | 4.54 | 43.2 | pipelines/compile_ms 2653, pipelines/front_ms 2402, pipelines/wait_ms 447, pipelines/driver_ms 251, render/thread/ms 134, render/busy/ms 132, main/busy/ms 124, stage/main/update 92.56, stage/render/create_views 53.87, stream/drive/ms 46.44, stage/render/prepare_assets 43.33, stage/render/render 25.17 |
| warm-up | 4.62 | 78.0 | pipelines/compile_ms 857, pipelines/front_ms 833, pipelines/wait_ms 217, main/busy/ms 119, stage/main/update 89.99, render/thread/ms 83.43, render/busy/ms 81.70, stream/drive/ms 73.55, stage/render/render 39.67, render/graph/ms 37.10, stage/render/prepare_assets 25.14, pipelines/driver_ms 24.05 |
| warm-up | 4.70 | 88.2 | pipelines/compile_ms 508, pipelines/front_ms 488, pipelines/wait_ms 134, render/thread/ms 115, render/busy/ms 113, main/busy/ms 89.77, stage/main/update 61.89, stage/render/render 51.28, stream/drive/ms 48.59, render/graph/ms 48.39, stage/render/camera_driver 31.36, pipelines/driver_ms 20.07 |
| warm-up | 4.75 | 48.6 | pipelines/compile_ms 959, pipelines/front_ms 930, render/thread/ms 101, render/busy/ms 99.67, pipelines/wait_ms 95.38, main/busy/ms 91.27, stage/main/update 72.31, stream/drive/ms 59.37, stage/render/prepare_assets 33.86, pipelines/driver_ms 29.14, stage/render/render 26.55, render/graph/ms 23.89 |
| warm-up | 4.90 | 144.3 | pipelines/compile_ms 2633, pipelines/front_ms 2490, pipelines/driver_ms 144, render/thread/ms 124, render/busy/ms 122, main/busy/ms 103, stage/main/update 78.04, pipelines/wait_ms 70.14, stage/render/prepare_assets 64.83, stream/drive/ms 58.16, stage/render/prepare_write_phase_buffers 32.65, stage/main/post_update 15.73 |
| warm-up | 4.93 | 32.2 | pipelines/compile_ms 2271, pipelines/front_ms 2136, pipelines/driver_ms 134, render/thread/ms 109, render/busy/ms 104, main/busy/ms 73.70, stage/render/prepare_assets 51.38, stage/main/update 49.72, pipelines/wait_ms 44.01, stream/drive/ms 33.21, stage/render/prepare_write_phase_buffers 32.14, stage/render/render 15.83 |
| warm-up | 4.98 | 54.5 | pipelines/compile_ms 481, pipelines/front_ms 466, main/busy/ms 71.01, render/thread/ms 59.35, render/busy/ms 54.78, stage/main/update 54.71, pipelines/wait_ms 43.50, stream/drive/ms 32.88, stage/render/prepare_assets 27.01, stage/render/render 16.04, pipelines/driver_ms 15.24, render/graph/ms 10.95 |
| warm-up | 5.02 | 37.6 | pipelines/compile_ms 94.03, pipelines/front_ms 89.55, render/thread/ms 60.98, render/busy/ms 59.75, main/busy/ms 44.52, stage/render/prepare_assets 32.24, stage/main/update 28.30, stage/render/render 11.01, encode//sum 10.57, render/graph/ms 9.21, stage/main/post_update 8.81, stage/render/graph_submit 5.52 |
| warm-up | 5.15 | 32.6 | pipelines/compile_ms 538, pipelines/front_ms 519, render/thread/ms 48.23, render/busy/ms 46.81, main/busy/ms 26.89, stage/render/prepare_assets 24.71, pipelines/driver_ms 18.97, stage/render/render 16.25, render/graph/ms 14.36, stage/main/update 12.77, stage/render/graph_submit 10.21, stage/main/post_update 9.35 |
| warm-up | 5.19 | 43.3 | pipelines/compile_ms 222, pipelines/front_ms 209, render/thread/ms 71.91, render/busy/ms 69.91, main/busy/ms 49.69, stage/render/prepare_assets 42.75, stage/main/update 18.28, stage/render/render 16.79, render/graph/ms 13.99, stage/main/post_update 13.32, pipelines/driver_ms 12.72, stage/main/pre_update 10.74 |
| warm-up | 5.25 | 59.8 | render/thread/ms 96.14, render/busy/ms 94.20, main/busy/ms 75.77, pipelines/compile_ms 53.96, pipelines/front_ms 51.17, stage/render/prepare_assets 33.72, stage/render/prepare_write_phase_buffers 26.28, stage/main/update 26.24, stage/main/post_update 21.67, stage/main/pre_update 19.44, stage/render/render 15.24, render/graph/ms 12.57 |
| warm-up | 5.29 | 45.0 | pipelines/compile_ms 183, pipelines/front_ms 177, render/thread/ms 95.42, render/busy/ms 94.15, main/busy/ms 75.13, stage/main/post_update 34.99, stage/render/prepare_write_phase_buffers 28.40, stage/main/update 25.29, stage/render/prepare_assets 24.84, stage/render/render 13.86, render/graph/ms 12.01, stage/main/pre_update 10.22 |
| warm-up | 5.35 | 51.8 | pipelines/compile_ms 1793, pipelines/front_ms 1742, pipelines/wait_ms 112, render/thread/ms 89.85, render/busy/ms 88.25, main/busy/ms 64.18, pipelines/driver_ms 50.93, stage/main/post_update 40.80, stage/render/render 20.88, stage/render/prepare_resources 18.66, render/graph/ms 18.23, stage/render/prepare_assets 18.12 |
| warm-up | 5.38 | 32.2 | pipelines/compile_ms 2448, pipelines/front_ms 2383, pipelines/wait_ms 113, render/thread/ms 77.06, render/busy/ms 75.41, pipelines/driver_ms 64.70, main/busy/ms 52.70, stage/main/post_update 27.12, stage/render/render 19.79, render/graph/ms 17.26, stage/render/prepare_resources 17.08, stage/main/update 16.27 |
| warm-up | 5.43 | 48.3 | pipelines/compile_ms 1042, pipelines/front_ms 1020, render/thread/ms 59.50, render/busy/ms 58.37, stage/render/prepare_assets 36.18, main/busy/ms 32.92, pipelines/driver_ms 21.97, stage/main/post_update 12.59, stage/main/update 12.21, stage/render/render 10.29, encode//sum 9.54, render/graph/ms 8.77 |
| warm-up | 5.52 | 28.8 | render/thread/ms 43.86, render/busy/ms 42.68, main/busy/ms 27.25, stage/render/prepare_assets 18.34, stage/render/render 14.06, render/graph/ms 12.36, encode//sum 12.09, stage/main/post_update 11.98, stage/main/update 10.71, stage/render/graph_submit 7.93, encode/pass:shadow_point_light/sum 5.26, encode/submit 4.97 |
| warm-up | 5.59 | 45.0 | render/thread/ms 59.71, render/busy/ms 58.05, main/busy/ms 35.30, stage/render/prepare_assets 28.83, stage/render/render 19.09, stage/main/update 18.91, render/graph/ms 16.62, pipelines/compile_ms 14.16, pipelines/front_ms 13.43, stage/render/graph_submit 12.28, encode//sum 11.77, stage/main/post_update 10.79 |
| warm-up | 6.10 | 30.8 | main/busy/ms 34.06, stage/main/update 22.63, render/thread/ms 20.77, render/busy/ms 19.77, stream/drive/ms 17.03, stage/render/render 9.93, encode//sum 8.84, render/graph/ms 8.50, stage/main/post_update 7.96, stage/render/graph_submit 5.84, encode/pass:shadow_point_light/sum 4.62, stage/render/prepare_assets 4.38 |
| warm-up | 6.23 | 30.8 | render/thread/ms 40.17, render/busy/ms 39.12, main/busy/ms 28.05, stage/render/prepare_assets 16.64, stage/main/update 10.47, stage/render/render 10.18, stage/main/post_update 10.12, encode//sum 8.78, render/graph/ms 8.68, stage/render/graph_submit 6.02, encode/pass:shadow_point_light/sum 5.35, stage/main/pre_update 3.77 |
| warm-up | 6.45 | 29.3 | render/thread/ms 45.13, render/busy/ms 43.49, main/busy/ms 42.62, stage/main/post_update 17.90, stage/main/update 17.38, stage/render/render 16.58, render/graph/ms 14.58, encode//sum 11.32, stage/render/graph_submit 9.97, stage/render/prepare_assets 9.93, encode/pass:shadow_point_light/sum 7.59, encode/submit 6.87 |
| warm-up | 6.48 | 28.8 | render/thread/ms 47.92, render/busy/ms 46.44, main/busy/ms 41.10, stage/render/render 17.73, stage/main/update 17.28, stage/main/post_update 16.90, render/graph/ms 15.67, stage/render/prepare_assets 14.54, encode//sum 13.20, stage/render/graph_submit 10.64, encode/pass:shadow_point_light/sum 9.12, encode/submit 7.22 |
| warm-up | 6.51 | 28.4 | pipelines/compile_ms 156, pipelines/front_ms 152, render/thread/ms 43.10, render/busy/ms 42.01, main/busy/ms 31.73, stage/render/prepare_assets 18.14, stage/main/update 13.65, stage/render/render 13.26, stage/main/post_update 12.42, render/graph/ms 11.55, encode//sum 11.25, encode/pass:shadow_point_light/sum 8.04 |
| warm-up | 6.59 | 33.0 | render/thread/ms 42.84, render/busy/ms 41.77, main/busy/ms 39.25, stage/main/update 19.80, stage/render/prepare_assets 15.31, stage/main/post_update 13.48, stage/render/render 11.56, encode//sum 9.92, render/graph/ms 9.86, stage/render/graph_submit 6.10, encode/pass:shadow_point_light/sum 5.74, stream/drive/ms 3.69 |
| warm-up | 6.71 | 27.5 | pipelines/compile_ms 149, pipelines/front_ms 146, render/thread/ms 46.17, render/busy/ms 44.93, main/busy/ms 24.47, stage/render/prepare_assets 24.40, stage/render/render 11.67, stage/main/post_update 10.30, stage/main/update 10.22, encode//sum 9.97, render/graph/ms 9.81, encode/pass:shadow_point_light/sum 7.07 |
| warm-up | 6.77 | 43.9 | render/thread/ms 45.34, render/busy/ms 44.09, main/busy/ms 37.21, stage/render/prepare_assets 26.77, stage/main/update 23.70, pipelines/compile_ms 17.95, pipelines/front_ms 17.16, stream/drive/ms 16.75, stage/render/render 10.52, stage/main/post_update 9.50, render/graph/ms 8.80, encode//sum 6.97 |
| warm-up | 6.99 | 32.2 | main/busy/ms 39.13, render/thread/ms 28.47, render/busy/ms 27.36, stage/main/update 21.36, stage/render/prepare_assets 12.09, stage/main/post_update 11.14, stage/render/render 9.64, render/graph/ms 8.05, encode//sum 6.30, stage/render/graph_submit 6.09, stream/drive/ms 3.77, encode/submit 3.54 |
| measure | 18.17 | 27.5 | main/busy/ms 36.06, render/thread/ms 23.90, render/busy/ms 22.83, stage/main/update 21.22, stream/drive/ms 17.11, stage/render/render 15.31, encode//sum 14.26, render/graph/ms 13.80, stage/main/post_update 12.05, stage/render/graph_submit 10.37, encode/pass:shadow_point_light/sum 9.46, encode/submit 6.82 |
| measure | 23.11 | 28.1 | render/thread/ms 38.65, render/busy/ms 37.53, main/busy/ms 31.29, encode//sum 18.16, stage/render/render 15.58, render/graph/ms 13.82, stage/main/post_update 13.37, stage/main/update 12.66, encode/pass:shadow_point_light/sum 10.46, stage/render/graph_submit 9.70, stage/render/prepare_assets 9.55, encode/submit 5.24 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 0%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **72%** (min 45%) · 2625 MHz (min 2625) · memory 8101 MHz · 75.9 W · 76 °C (max 82) · VRAM 6758 MiB · throttled 100% of samples

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
