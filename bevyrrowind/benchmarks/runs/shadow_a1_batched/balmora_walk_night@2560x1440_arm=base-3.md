# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5998 frames over 65.6 s · **median 10.53 ms (95 fps)** · mean 10.94 · p95 13.63 · p99 15.35 (1% low 65 fps) · worst 30.00 ms · 49 spikes over 1.5× median · 33.8 stutters/min (work 29.2/min)
- warm-up: 354 frames over 9.4 s, worst 2500.9 ms
- hitches (> 26.3 ms): **2** measured, 41 in warm-up
- timed GPU passes: 6.98 ms of a 10.94 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.53 ms, limit 11 ms
- **FAIL** frame_ms: 37 stutters (> 1.5x moving median of 60) in 1.09 min = 33.82/min, limit 1/min
- pass worst frame 30.00 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.113 | 2.154 | 2.448 | 0.98 | 0.183 |
| main_opaque_pass_3d | 1.307 | 1.332 | 1.804 | 0.98 | 0.084 |
| early prepass | 0.520 | 0.530 | 0.744 | 0.98 | 0.095 |
| bloom | 0.335 | 0.341 | 0.665 | 0.98 | 0.066 |
| atmosphere_luts | 0.319 | 0.325 | 0.461 | 0.98 | 0.004 |
| water_velocity | 0.235 | 0.239 | 0.248 | 0.98 | 0.002 |
| auxiliary_views | 0.212 | 0.216 | 0.980 | 0.98 | 0.205 |
| water_sspr | 0.208 | 0.212 | 0.203 | 0.98 | 0.003 |
| ssao | 0.194 | 0.198 | 0.204 | 0.98 | 0.008 |
| atmos_froxel_inject | 0.132 | 0.134 | 0.129 | 0.98 | 0.002 |
| upscaling | 0.130 | 0.133 | 0.148 | 0.98 | 0.005 |
| water_fft | 0.124 | 0.127 | 0.127 | 0.98 | 0.003 |
| uniform_allocation | 0.117 | 0.120 | 0.143 | 0.98 | 0.026 |
| clouds_composite | 0.104 | 0.106 | 0.109 | 0.98 | 0.002 |
| bin_unpacking | 0.094 | 0.095 | 0.114 | 0.98 | 0.020 |
| gpu_instance_materialization | 0.093 | 0.095 | 0.099 | 0.98 | 0.004 |
| tonemapping | 0.091 | 0.092 | 0.097 | 0.98 | 0.004 |
| main_transmissive_pass_3d | 0.075 | 0.077 | 0.142 | 0.98 | 0.009 |
| clustering | 0.063 | 0.065 | 0.068 | 0.98 | 0.020 |
| early_mesh_preprocessing | 0.055 | 0.057 | 0.072 | 0.98 | 0.005 |
| clouds_trace | 0.055 | 0.056 | 0.076 | 0.98 | 0.001 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.448 | 0.754 | 0.11 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.98 | 0.001 |
| surface_trail | 0.041 | 0.058 | 0.057 | 0.70 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 0.98 | 0.001 |
| water_snapshot | 0.026 | 0.026 | 0.032 | 0.98 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.219 | 0.230 | 0.11 | 0.005 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.98 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.028 | 0.98 | 0.004 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.026 | 0.98 | 0.006 |
| night_sky | 0.016 | 0.016 | 0.039 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.98 | 0.202 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.98 | 0.004 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.533 | 12.138 | 28.12 | 57176.1 |
| render/busy/ms | 9.031 | 11.628 | 21.08 | 54170.1 |
| main/busy/ms | 8.886 | 10.589 | 28.35 | 53300.0 |
| stage/render/render | 5.784 | 7.760 | 20.66 | 34690.6 |
| stage/main/post_update | 5.153 | 6.375 | 10.56 | 30906.5 |
| render/graph/ms | 5.100 | 7.040 | 13.30 | 30589.8 |
| encode//sum | 4.526 | 6.487 | 28.43 | 27148.3 |
| stage/render/graph_submit | 3.654 | 5.000 | 9.55 | 21914.5 |
| encode/submit | 2.341 | 3.403 | 5.76 | 14041.3 |
| stage/main/update | 1.926 | 2.469 | 21.55 | 11553.0 |
| stage/render/camera_driver | 1.390 | 1.999 | 4.17 | 8338.9 |
| encode/finish_wall | 1.355 | 1.757 | 4.22 | 8128.8 |
| encode/pass:shadow_directional_light/sum | 0.836 | 1.253 | 2.20 | 5016.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.825 | 1.121 | 1.95 | 4946.6 |
| encode/pass:early prepass/sum | 0.824 | 1.117 | 2.71 | 4940.1 |
| stage/render/prepare_write_phase_buffers | 0.667 | 1.045 | 1.84 | 3997.8 |
| bigspace/total/ms | 0.566 | 1.350 | 2.85 | 3396.9 |
| stage/render/prepare_assets | 0.558 | 0.693 | 10.75 | 3346.8 |
| stage/main/fixed_main_loop | 0.557 | 0.925 | 1.93 | 3340.9 |
| stage/render/prepare_meshes | 0.473 | 0.602 | 4.02 | 2836.4 |
| stage/render/prepare_resources | 0.459 | 0.565 | 1.05 | 2754.9 |
| render/present/ms | 0.413 | 0.514 | 12.26 | 2474.5 |
| stage/main/pre_update | 0.406 | 0.564 | 2.07 | 2434.9 |
| stage/main/first | 0.401 | 0.525 | 5.52 | 2403.8 |
| encode/pass:shadow_point_light/sum | 0.358 | 2.183 | 4.82 | 2144.7 |
| controller/feed/ms | 0.346 | 0.441 | 0.85 | 2077.1 |
| stage/render/prepare_bind_groups | 0.335 | 0.563 | 3.49 | 2006.9 |
| stage/render/prepare_flush | 0.301 | 0.530 | 2.18 | 1806.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.299 | 0.464 | 1.03 | 1795.5 |
| stage/render/prepare_batch_phases | 0.290 | 0.447 | 0.79 | 1738.2 |
| stage/main/last | 0.257 | 0.312 | 2.06 | 1539.9 |
| bigspace/low_precision/ms | 0.247 | 0.776 | 1.74 | 1479.6 |
| stream/drive/ms | 0.214 | 0.312 | 10.82 | 1280.7 |
| stage/render/create_views | 0.204 | 0.265 | 0.99 | 1221.9 |
| stage/render/specialize | 0.182 | 0.277 | 2.39 | 1094.3 |
| stage/render/extract_commands | 0.176 | 0.209 | 0.48 | 1055.8 |
| stage/render/queue | 0.175 | 0.256 | 2.22 | 1049.4 |
| anim/evaluate/ms | 0.164 | 0.292 | 1.43 | 984.0 |
| bigspace/high_precision/ms | 0.159 | 0.679 | 2.29 | 954.5 |
| anim/evaluate_cpu/ms | 0.157 | 0.206 | 0.59 | 940.7 |
| stream/tick/ms | 0.155 | 0.233 | 0.51 | 932.5 |
| stage/render/prepare_views | 0.136 | 0.178 | 0.42 | 813.1 |
| pipelines/compile_ms | 0.129 | 0.000 | 142.72 | 776.2 |
| pipelines/front_ms | 0.126 | 0.000 | 139.53 | 755.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.108 | 0.175 | 0.62 | 648.9 |
| render/acquire/ms | 0.089 | 0.118 | 0.37 | 531.5 |
| controller/move/ms | 0.077 | 0.199 | 0.53 | 459.9 |
| controller/write/ms | 0.076 | 0.101 | 0.43 | 453.4 |
| terrain/update/ms | 0.062 | 0.296 | 2.01 | 374.5 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.077 | 0.39 | 294.4 |
| physics/step/ms | 0.044 | 0.084 | 0.69 | 266.2 |
| stage/render/phase_sort | 0.042 | 0.069 | 0.36 | 250.3 |
| encode/resolve_diagnostics/sum | 0.034 | 0.046 | 0.39 | 203.0 |
| physics/insert/ms | 0.031 | 0.351 | 0.95 | 188.7 |
| terrain/select/ms | 0.024 | 0.261 | 0.74 | 145.2 |
| stage/render/cleanup | 0.024 | 0.036 | 1.45 | 145.2 |
| weather/map/ms | 0.018 | 0.024 | 0.17 | 106.5 |
| far/select/ms | 0.017 | 0.177 | 0.45 | 100.2 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.17 | 89.0 |
| anim/apply/ms | 0.012 | 0.023 | 0.14 | 72.1 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.32 | 71.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.19 | 65.9 |
| script/update/ms | 0.009 | 0.012 | 0.31 | 51.6 |
| controller/avoid/ms | 0.007 | 0.012 | 0.12 | 39.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.15 | 36.7 |
| script/input/ms | 0.005 | 0.006 | 0.13 | 31.1 |
| script/ui/ms | 0.004 | 0.006 | 0.26 | 25.8 |
| stream/spawn/ms | 0.004 | 0.000 | 1.09 | 22.4 |
| stream/upload/ms | 0.004 | 0.000 | 1.54 | 22.3 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.39 | 20.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.09 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 34 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 26 | 42 | 11 |
| crowd/c0/joints/count | count | 1302 | 2218 | 489 |
| crowd/c0/meshes/count | count | 368 | 633 | 128 |
| crowd/c1/count | count | 145 | 157 | 150 |
| crowd/c1/joints/count | count | 5531 | 6003 | 5707 |
| crowd/c1/meshes/count | count | 1203 | 1299 | 1213 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 162 | 216 | 205 |
| crowd/joints_extracted/count | count | 707 | 5775 | 480 |
| crowd/joints_uploaded/count | count | 3619 | 6207 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.2 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.47 | 1.58 | 0.60 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.17 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.32 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.20 | 0.94 |
| encode/pass:early prepass/max | ms | 0.81 | 2.13 | 0.94 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 1.95 | 0.83 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.62 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 1.03 | 0.38 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.20 | 0.59 |
| encode/pass:shadow_point_light/max | ms | 0.13 | 1.36 | 0.71 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.39 | 0.02 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.04 | 28.35 | 8.31 |
| gpu/timed/ms | ms | 7.76 | 9.17 | 8.02 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 158 | 386 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 340 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 158 | 386 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 340 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 764 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2469 | 3527 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 764 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2469 | 3527 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 159 | 8 |
| phases/shadow/bins | count | 1258 | 1669 | 671 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2939 | 3631 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 162 | 202 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 221 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 332 | 333 | 333 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 17 | 469 | 11 |
| preprocess/bind_groups_created | count | 12 | 463 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 126 | 231 | 123 |
| process/cpu_usage | % | 25.96 | 40.87 | 24.04 |
| process/mem_usage | GiB | 5.39 | 5.57 | 5.57 |
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
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 15 | 104 | 10 |
| skin/blocks_skipped/count | count | 146 | 203 | 195 |
| skin/blocks_walked/count | count | 15 | 26 | 10 |
| skin/joint_bytes_written/bytes | bytes | 49.2 k | 808.1 k | 31.7 k |
| skin/joints_written/count | count | 768 | 12.6 k | 496 |
| skin/upload_writes/count | count | 7 | 25 | 8 |
| started_load_count |  | 6440 | 6564 | 6564 |
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
| system/cpu_usage | % | 33.38 | 43.42 | 36.29 |
| system/mem_usage | % | 56.31 | 57.00 | 56.96 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 78 | 218 | 66 |
| views/all/bins | count | 3142 | 3856 | 2961 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 8417 | 10.6 k | 7969 |
| views/all/pipelines | count | 209 | 249 | 66 |
| views/all/pipelines_distinct | count | 70 | 73 | 61 |
| views/all/work_items | count | 154.0 k | 206.7 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 713 | 879 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2109 | 2445 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 130 | 0 |
| views/lamp_face/bins | count | 565 | 859 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 24 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 784 | 1187 | 0 |
| views/lamp_face/pipelines | count | 136 | 174 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 701 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 1998 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5906 | 7492 | 5942 |
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
| world/meshes | count | 21.1 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 42.83 | 227 | 256.9 k |
| anim/events/count | count | 0.10 | 3 | 572 |
| assets/Image/added | count | 0.10 | 36 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.48 | 284 | 2874 |
| assets/Mesh/removed | count | 0.81 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.25 | 10 | 7476 |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.25 | 1133 | 1490 |
| encode//count | count | 36.46 | 204 | 218.7 k |
| encode/bind_walk/init_textures | count | 9430 | 31.7 k | 56.56 M |
| encode/bind_walk/init_textures_skipped | count | 51.8 k | 87.2 k | 310.95 M |
| encode/bind_walk/inits | count | 563 | 1372 | 3.38 M |
| encode/bind_walk/inits_skipped | count | 138 | 414 | 825.1 k |
| encode/bind_walk/merge_textures | count | 9367 | 31.6 k | 56.18 M |
| encode/bind_walk/merge_textures_skipped | count | 10.4 k | 19.4 k | 62.36 M |
| encode/bind_walk/merges | count | 102 | 282 | 613.7 k |
| encode/bind_walk/merges_skipped | count | 127 | 390 | 762.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 5998 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 5998 |
| encode/pass:early prepass/count | count | 1 | 2 | 5998 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 5998 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 5998 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 5998 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 5998 |
| encode/pass:shadow_point_light/count | count | 2.12 | 44 | 12.7 k |
| encode/queue/command_buffers | count | 48.60 | 258 | 291.5 k |
| encode/queue/submits | count | 2.01 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 2.34 M | 4.53 M | 14.02 G |
| encode/queue/write_buffer/calls | count | 269 | 716 | 1.61 M |
| encode/queue/write_buffer_with/bytes | bytes | 161.8 k | 1.28 M | 970.75 M |
| encode/queue/write_buffer_with/calls | count | 10.42 | 861 | 62.5 k |
| encode/queue/write_texture/bytes | bytes | 2123 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 5998 |
| physics/inserts | count | 0.37 | 106 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 175 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 9 |
| pipelines/created | count | 0.00 | 1 | 9 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 30 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5644 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1237 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.3 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2500.9 | main/busy/ms 2469, stage/main/post_update 2427, stage/main/update 32.34, stage/main/pre_update 6.50, stage/main/last 1.22, surface/advance_cpu_ms 0.74, stage/main/first 0.55, stage/main/fixed_main_loop 0.50, surface/cell_upload_ms 0.39, stream/drive/ms 0.38, stream/tick/ms 0.11, bigspace/total/ms 0.09 |
| warm-up | 3.30 | 795.3 | main/busy/ms 2615, stage/main/post_update 2450, render/thread/ms 758, render/busy/ms 592, stage/render/prepare_assets 549, stage/render/prepare_views 163, render/acquire/ms 163, stage/main/update 147, stage/render/create_views 25.78, stage/render/render 10.06, stage/main/pre_update 8.46, render/graph/ms 6.64 |
| warm-up | 3.36 | 63.1 | render/thread/ms 804, render/busy/ms 637, stage/render/prepare_assets 581, main/busy/ms 171, stage/render/prepare_views 163, render/acquire/ms 163, stage/main/update 121, pipelines/compile_ms 60.90, pipelines/front_ms 56.89, stage/main/post_update 36.44, stage/render/create_views 26.38, terrain/update/ms 17.79 |
| warm-up | 3.40 | 44.3 | pipelines/compile_ms 275, pipelines/front_ms 257, pipelines/wait_ms 223, render/thread/ms 88.70, render/busy/ms 85.72, main/busy/ms 74.29, stage/main/post_update 55.10, stage/render/render 43.10, render/graph/ms 39.50, stage/render/prepare_assets 32.73, stage/render/graph_submit 32.64, terrain/update/ms 23.97 |
| warm-up | 3.55 | 144.4 | pipelines/wait_ms 5715, pipelines/compile_ms 1537, pipelines/front_ms 1426, render/thread/ms 165, render/busy/ms 163, main/busy/ms 137, stage/main/post_update 116, pipelines/driver_ms 110, bigspace/total/ms 65.20, stage/render/render 63.06, render/graph/ms 60.00, stage/render/graph_submit 43.25 |
| warm-up | 3.58 | 33.0 | pipelines/wait_ms 5652, pipelines/compile_ms 1393, pipelines/front_ms 1294, main/busy/ms 164, render/thread/ms 153, render/busy/ms 148, pipelines/driver_ms 99.01, stage/main/post_update 85.10, stage/main/update 60.12, bigspace/total/ms 54.78, stage/render/prepare_write_phase_buffers 42.54, stage/render/render 34.62 |
| warm-up | 3.70 | 120.1 | pipelines/compile_ms 1179, pipelines/front_ms 1122, pipelines/wait_ms 288, main/busy/ms 102, render/thread/ms 100, render/busy/ms 95.14, stage/render/prepare_assets 75.05, stage/main/update 67.56, pipelines/driver_ms 57.33, stage/main/post_update 22.84, terrain/update/ms 19.50, stage/render/render 16.00 |
| warm-up | 3.73 | 28.6 | pipelines/compile_ms 1816, pipelines/front_ms 1718, pipelines/wait_ms 232, pipelines/driver_ms 98.22, render/thread/ms 94.89, render/busy/ms 93.14, stage/render/prepare_assets 75.46, main/busy/ms 46.85, stage/main/post_update 21.76, stage/main/update 18.23, terrain/update/ms 12.09, stage/render/render 11.72 |
| warm-up | 3.76 | 27.2 | pipelines/compile_ms 802, pipelines/front_ms 753, pipelines/wait_ms 126, render/thread/ms 49.94, pipelines/driver_ms 49.06, render/busy/ms 48.45, main/busy/ms 38.94, stage/render/prepare_assets 30.65, stage/main/post_update 20.73, terrain/update/ms 11.47, stage/main/update 10.48, stage/render/render 9.62 |
| warm-up | 3.82 | 29.0 | pipelines/compile_ms 493, pipelines/front_ms 478, main/busy/ms 77.70, stage/main/update 58.62, render/thread/ms 40.00, render/busy/ms 38.37, stage/render/render 21.64, render/graph/ms 19.55, pipelines/driver_ms 14.34, stage/main/post_update 14.25, stage/render/graph_submit 10.39, stage/render/prepare_assets 8.48 |
| warm-up | 3.93 | 109.6 | pipelines/compile_ms 1030, pipelines/front_ms 985, main/busy/ms 77.08, render/thread/ms 70.54, render/busy/ms 68.78, stage/main/update 60.86, pipelines/driver_ms 44.39, stage/render/prepare_assets 43.32, stage/render/render 20.63, render/graph/ms 18.57, stage/render/graph_submit 9.73, stage/main/post_update 9.62 |
| warm-up | 3.97 | 37.8 | pipelines/compile_ms 1660, pipelines/front_ms 1594, render/thread/ms 69.34, render/busy/ms 67.73, pipelines/driver_ms 66.14, stage/render/prepare_assets 54.52, main/busy/ms 24.60, pipelines/wait_ms 19.77, stage/main/update 11.73, stage/render/render 10.06, render/graph/ms 8.42, stage/render/graph_submit 6.52 |
| warm-up | 4.07 | 33.8 | pipelines/compile_ms 74.65, pipelines/front_ms 71.04, main/busy/ms 42.15, render/thread/ms 37.35, render/busy/ms 34.93, stage/main/update 23.93, stage/render/render 17.33, render/graph/ms 14.16, stage/main/post_update 12.27, stage/render/prepare_assets 11.68, stream/drive/ms 10.60, stage/render/graph_submit 10.32 |
| warm-up | 4.12 | 57.8 | pipelines/compile_ms 246, pipelines/front_ms 239, render/thread/ms 70.01, main/busy/ms 69.85, render/busy/ms 67.83, stage/main/update 35.82, stage/render/prepare_assets 30.86, stage/main/post_update 25.41, stage/render/render 24.57, stream/drive/ms 22.17, render/graph/ms 21.65, stage/render/graph_submit 17.29 |
| warm-up | 4.20 | 75.7 | pipelines/compile_ms 1191, pipelines/front_ms 1152, main/busy/ms 156, stage/main/update 124, render/thread/ms 85.74, render/busy/ms 83.83, stage/render/prepare_assets 49.02, pipelines/driver_ms 38.49, stream/drive/ms 22.90, stage/main/post_update 22.02, stage/render/render 21.98, render/graph/ms 19.45 |
| warm-up | 4.38 | 183.7 | pipelines/compile_ms 2208, pipelines/front_ms 2117, main/busy/ms 122, render/thread/ms 119, render/busy/ms 118, stage/main/update 105, stage/render/prepare_assets 98.51, pipelines/driver_ms 91.37, stage/render/render 11.67, render/graph/ms 9.47, stage/main/post_update 8.17, encode//sum 7.58 |
| warm-up | 4.42 | 40.4 | pipelines/compile_ms 1318, pipelines/front_ms 1256, render/thread/ms 112, render/busy/ms 110, stage/render/prepare_assets 77.16, main/busy/ms 62.98, pipelines/driver_ms 61.88, stage/main/update 39.22, stage/render/render 18.94, render/graph/ms 17.07, stage/render/graph_submit 14.65, stage/main/post_update 14.21 |
| warm-up | 4.49 | 66.1 | main/busy/ms 84.13, render/thread/ms 74.67, render/busy/ms 72.99, pipelines/compile_ms 68.70, pipelines/front_ms 66.03, stage/main/update 48.57, stage/main/post_update 25.23, stage/render/prepare_assets 23.98, stage/render/render 22.31, render/graph/ms 20.23, stage/render/graph_submit 17.63, encode//sum 10.19 |
| warm-up | 4.55 | 62.4 | render/thread/ms 95.26, render/busy/ms 93.61, main/busy/ms 78.14, stage/render/prepare_assets 47.15, stage/main/update 42.26, stage/main/post_update 18.70, stage/render/prepare_resources 15.35, stage/render/render 13.70, render/graph/ms 11.50, encode//sum 11.09, stage/main/pre_update 8.95, stage/render/graph_submit 8.77 |
| warm-up | 4.64 | 91.4 | pipelines/compile_ms 2903, pipelines/front_ms 2772, pipelines/wait_ms 626, pipelines/driver_ms 131, render/thread/ms 131, render/busy/ms 129, main/busy/ms 108, stage/main/update 58.98, stage/render/prepare_assets 46.57, stage/render/specialize 28.03, stage/main/pre_update 21.02, stage/render/prepare_resources 20.54 |
| warm-up | 4.70 | 52.4 | pipelines/compile_ms 3260, pipelines/front_ms 3113, pipelines/wait_ms 976, pipelines/driver_ms 147, render/thread/ms 127, render/busy/ms 125, main/busy/ms 104, stage/main/update 52.66, stage/render/specialize 28.36, stage/render/prepare_assets 27.45, stage/main/post_update 27.30, stage/render/prepare_resources 26.66 |
| warm-up | 4.78 | 81.5 | pipelines/compile_ms 837, pipelines/front_ms 802, pipelines/wait_ms 566, render/thread/ms 125, render/busy/ms 123, main/busy/ms 97.12, stage/main/update 61.72, stage/render/prepare_assets 49.46, pipelines/driver_ms 34.91, stage/render/render 28.48, render/graph/ms 26.08, encode//sum 25.35 |
| warm-up | 4.87 | 88.1 | pipelines/compile_ms 1633, pipelines/front_ms 1575, pipelines/wait_ms 787, render/thread/ms 139, render/busy/ms 138, main/busy/ms 122, stage/main/update 85.39, stage/render/prepare_assets 63.28, pipelines/driver_ms 58.45, stage/render/render 29.84, render/graph/ms 27.26, stage/render/prepare_write_phase_buffers 22.09 |
| warm-up | 4.94 | 73.8 | pipelines/compile_ms 2539, pipelines/front_ms 2461, pipelines/wait_ms 653, main/busy/ms 151, render/thread/ms 117, stage/main/update 113, render/busy/ms 112, pipelines/driver_ms 77.47, stream/drive/ms 64.06, stage/render/prepare_assets 43.32, stage/render/render 33.63, encode//sum 29.26 |
| warm-up | 5.02 | 78.3 | pipelines/compile_ms 1949, pipelines/front_ms 1897, pipelines/wait_ms 135, main/busy/ms 107, render/thread/ms 92.38, render/busy/ms 87.50, stage/main/update 81.30, stream/drive/ms 64.03, pipelines/driver_ms 51.96, encode//sum 38.71, stage/render/render 31.74, stage/render/prepare_assets 29.53 |
| warm-up | 5.05 | 35.0 | pipelines/compile_ms 784, pipelines/front_ms 763, render/thread/ms 71.65, render/busy/ms 69.20, main/busy/ms 66.84, pipelines/wait_ms 55.52, stage/main/update 47.45, encode//sum 37.61, stage/render/render 35.56, render/graph/ms 32.62, stage/render/graph_submit 24.75, pipelines/driver_ms 20.89 |
| warm-up | 5.10 | 51.5 | pipelines/compile_ms 654, pipelines/front_ms 634, main/busy/ms 62.67, render/thread/ms 62.11, render/busy/ms 60.05, stage/main/update 44.12, stage/render/render 28.06, render/graph/ms 25.53, stage/render/graph_submit 20.87, pipelines/driver_ms 19.80, encode//sum 19.61, pipelines/wait_ms 17.25 |
| warm-up | 5.22 | 41.2 | pipelines/compile_ms 201, pipelines/front_ms 194, render/thread/ms 62.74, render/busy/ms 61.27, main/busy/ms 53.21, stage/main/update 29.12, stage/render/prepare_write_phase_buffers 20.32, stage/render/render 17.81, render/graph/ms 15.83, stage/main/post_update 14.39, stage/render/graph_submit 11.85, encode//sum 10.71 |
| warm-up | 5.26 | 40.9 | pipelines/compile_ms 767, pipelines/front_ms 746, render/thread/ms 68.49, render/busy/ms 67.11, main/busy/ms 50.12, stage/main/update 30.67, pipelines/driver_ms 21.26, stage/render/prepare_write_phase_buffers 20.62, stage/render/prepare_assets 19.22, stage/render/render 14.99, render/graph/ms 12.98, encode//sum 11.60 |
| warm-up | 5.31 | 32.3 | pipelines/compile_ms 221, pipelines/front_ms 214, render/thread/ms 52.24, render/busy/ms 47.90, main/busy/ms 38.67, stage/render/render 20.03, stage/main/post_update 17.96, render/graph/ms 15.22, stage/render/prepare_resources 12.70, stage/main/update 11.87, stage/render/graph_submit 10.62, encode//sum 10.49 |
| warm-up | 5.34 | 30.2 | pipelines/compile_ms 1221, pipelines/front_ms 1190, render/thread/ms 56.72, render/busy/ms 55.27, main/busy/ms 33.91, pipelines/driver_ms 31.00, stage/render/prepare_resources 21.11, stage/render/render 15.45, stage/main/update 14.42, render/graph/ms 13.44, encode//sum 10.65, stage/main/post_update 10.20 |
| warm-up | 6.02 | 29.8 | render/thread/ms 40.48, render/busy/ms 39.37, main/busy/ms 30.08, stage/render/prepare_assets 20.73, stage/main/update 13.29, stage/main/post_update 10.51, stage/render/render 9.17, encode//sum 8.42, render/graph/ms 7.72, stage/render/graph_submit 4.71, stream/drive/ms 3.62, stage/render/camera_driver 2.90 |
| warm-up | 6.10 | 26.8 | render/thread/ms 39.76, main/busy/ms 39.19, render/busy/ms 38.75, stage/main/update 22.38, stage/render/render 16.18, encode//sum 14.58, render/graph/ms 14.45, stage/main/post_update 11.87, stage/render/graph_submit 8.78, stage/render/prepare_assets 7.66, stage/render/camera_driver 5.52, encode/submit 5.22 |
| warm-up | 6.24 | 30.1 | render/thread/ms 45.55, render/busy/ms 44.04, main/busy/ms 38.45, stage/main/update 20.20, stage/render/render 19.27, encode//sum 16.93, render/graph/ms 16.72, stage/main/post_update 13.62, stage/render/graph_submit 11.47, stage/render/prepare_assets 10.23, encode/submit 7.72, stage/render/camera_driver 5.09 |
| warm-up | 6.30 | 33.3 | pipelines/compile_ms 136, pipelines/front_ms 134, render/thread/ms 45.12, render/busy/ms 44.17, main/busy/ms 35.67, stage/main/update 18.18, stage/render/prepare_assets 16.96, stage/render/render 11.21, encode//sum 11.21, render/graph/ms 9.79, stage/main/post_update 8.57, stage/render/prepare_meshes 6.48 |
| warm-up | 6.35 | 30.1 | render/thread/ms 48.14, render/busy/ms 46.35, main/busy/ms 37.86, stage/render/render 19.81, stage/main/update 17.87, render/graph/ms 17.57, stage/main/post_update 13.72, encode//sum 12.68, stage/render/graph_submit 11.74, pipelines/compile_ms 10.65, pipelines/front_ms 9.83, encode/submit 8.62 |
| warm-up | 6.41 | 33.0 | render/thread/ms 37.21, render/busy/ms 36.18, main/busy/ms 34.47, stage/main/update 18.97, stage/render/prepare_assets 15.59, stage/render/render 11.12, encode//sum 10.01, render/graph/ms 9.54, stage/main/post_update 9.44, stage/render/graph_submit 5.40, stream/drive/ms 4.36, stage/render/camera_driver 4.00 |
| warm-up | 6.52 | 33.1 | pipelines/compile_ms 147, pipelines/front_ms 143, main/busy/ms 45.88, render/thread/ms 35.48, render/busy/ms 33.87, stage/main/update 29.72, stream/drive/ms 14.40, stage/render/render 14.11, render/graph/ms 11.86, stage/main/post_update 11.66, encode//sum 10.86, stage/render/graph_submit 8.28 |
| warm-up | 6.55 | 28.6 | main/busy/ms 49.17, render/thread/ms 46.07, render/busy/ms 44.51, stage/main/update 29.59, stage/render/render 16.59, render/graph/ms 14.18, stage/main/post_update 13.25, encode//sum 13.06, stage/render/prepare_assets 12.73, pipelines/compile_ms 11.12, pipelines/front_ms 10.06, stage/render/graph_submit 9.93 |
| warm-up | 6.59 | 34.7 | render/thread/ms 46.22, render/busy/ms 44.78, main/busy/ms 43.68, stage/main/update 26.77, stage/render/prepare_assets 17.71, stage/render/render 14.60, render/graph/ms 12.47, encode//sum 11.45, stage/main/post_update 10.34, stage/render/graph_submit 8.52, stream/drive/ms 5.84, encode/submit 5.45 |
| warm-up | 6.61 | 27.1 | render/thread/ms 40.55, render/busy/ms 39.53, main/busy/ms 35.60, stage/render/prepare_assets 19.08, stage/main/update 16.96, stage/main/post_update 11.19, stage/render/render 9.84, render/graph/ms 8.36, encode//sum 7.87, stage/render/graph_submit 5.11, stage/render/prepare_meshes 4.68, stream/drive/ms 4.64 |
| measure | 18.38 | 30.0 | main/busy/ms 38.74, stage/main/update 24.06, render/thread/ms 20.88, render/busy/ms 19.92, stage/render/render 10.23, stage/main/post_update 9.58, render/graph/ms 8.81, encode//sum 7.42, stage/render/graph_submit 6.53, encode/submit 4.43, stage/render/prepare_assets 3.87, encode/finish_wall 2.19 |
| measure | 59.16 | 29.8 | render/thread/ms 38.04, render/busy/ms 36.95, main/busy/ms 25.65, stage/render/prepare_assets 16.72, stage/main/post_update 12.56, stage/render/render 10.42, encode//sum 9.74, render/graph/ms 8.89, stage/main/update 5.79, stage/render/graph_submit 5.40, stage/render/camera_driver 3.34, encode/submit 3.06 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 16%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1449 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **79%** (min 52%) · 2625 MHz (min 2625) · memory 8101 MHz · 76.6 W · 72 °C (max 78) · VRAM 6259 MiB · throttled 100% of samples

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
