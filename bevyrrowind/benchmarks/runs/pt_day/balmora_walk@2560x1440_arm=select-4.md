# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6305 frames over 65.6 s · **median 10.16 ms (98 fps)** · mean 10.41 · p95 12.86 · p99 16.26 (1% low 62 fps) · worst 46.04 ms · 87 spikes over 1.5× median · 66.7 stutters/min (work 58.5/min)
- warm-up: 324 frames over 9.2 s, worst 2598.7 ms
- hitches (> 25.4 ms): **33** measured, 53 in warm-up
- timed GPU passes: 6.40 ms of a 10.41 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.16 ms, limit 11 ms
- **FAIL** frame_ms: 73 stutters (> 1.5x moving median of 60) in 1.09 min = 66.72/min, limit 1/min
- **FAIL** worst frame 46.04 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.824 | 2.037 | 2.071 | 0.90 | 0.178 |
| main_opaque_pass_3d | 1.454 | 1.624 | 2.046 | 0.90 | 0.058 |
| early prepass | 0.443 | 0.495 | 0.736 | 0.90 | 0.086 |
| bloom | 0.282 | 0.315 | 0.321 | 0.90 | 0.063 |
| atmosphere_luts | 0.272 | 0.304 | 0.366 | 0.90 | 0.003 |
| water_velocity | 0.251 | 0.280 | 0.347 | 0.90 | 0.002 |
| water_sspr | 0.181 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.168 | 0.188 | 0.193 | 0.90 | 0.007 |
| atmos_froxel_inject | 0.130 | 0.146 | 0.148 | 0.90 | 0.001 |
| clouds_trace | 0.125 | 0.140 | 0.231 | 0.90 | 0.001 |
| water_fft | 0.120 | 0.134 | 0.140 | 0.90 | 0.003 |
| uniform_allocation | 0.114 | 0.127 | 0.179 | 0.90 | 0.023 |
| upscaling | 0.106 | 0.118 | 0.125 | 0.90 | 0.004 |
| clouds_composite | 0.095 | 0.106 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.093 | 0.104 | 0.142 | 0.90 | 0.018 |
| auxiliary_views | 0.091 | 0.102 | 0.416 | 0.90 | 0.102 |
| main_transmissive_pass_3d | 0.077 | 0.086 | 0.179 | 0.90 | 0.008 |
| tonemapping | 0.076 | 0.085 | 0.089 | 0.90 | 0.004 |
| clustering | 0.057 | 0.064 | 0.068 | 0.90 | 0.018 |
| render_sky | 0.046 | 0.052 | 0.053 | 0.90 | 0.002 |
| clouds_resolve | 0.043 | 0.048 | 0.049 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.039 | 0.413 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.039 | 0.044 | 0.044 | 0.89 | 0.001 |
| surface_trail | 0.035 | 0.056 | 0.057 | 0.62 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.80 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.90 | 0.003 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.047 | 0.90 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.030 | 0.031 | 0.90 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.90 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.220 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.90 | 0.002 |
| night_sky | 0.012 | 0.014 | 0.033 | 0.90 | 0.001 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.90 | 0.123 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.90 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.89 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.988 | 11.020 | 39.53 | 56669.1 |
| main/busy/ms | 8.522 | 10.138 | 25.96 | 53732.1 |
| render/busy/ms | 8.427 | 10.538 | 38.97 | 53132.0 |
| stage/main/post_update | 5.346 | 6.663 | 11.74 | 33707.6 |
| stage/render/render | 5.240 | 6.645 | 15.36 | 33041.3 |
| render/graph/ms | 4.489 | 5.911 | 14.16 | 28303.7 |
| encode//sum | 3.633 | 4.777 | 24.22 | 22907.1 |
| stage/render/graph_submit | 3.362 | 4.497 | 10.35 | 21200.4 |
| encode/submit | 2.139 | 3.103 | 6.43 | 13487.2 |
| stage/main/update | 1.931 | 2.325 | 19.31 | 12176.8 |
| encode/finish_wall | 1.314 | 1.894 | 4.49 | 8285.4 |
| stage/render/camera_driver | 1.074 | 1.467 | 3.75 | 6772.3 |
| encode/pass:shadow_directional_light/sum | 0.859 | 1.516 | 3.36 | 5417.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.768 | 1.059 | 1.95 | 4840.2 |
| encode/pass:early prepass/sum | 0.756 | 1.047 | 1.81 | 4767.4 |
| stage/render/prepare_write_phase_buffers | 0.708 | 1.014 | 2.55 | 4465.7 |
| encode/pass:shadow_point_light/sum | 0.589 | 2.759 | 16.37 | 3715.3 |
| stage/render/prepare_assets | 0.540 | 0.705 | 10.50 | 3405.0 |
| bigspace/total/ms | 0.517 | 1.241 | 2.26 | 3260.6 |
| render/present/ms | 0.467 | 1.215 | 2.26 | 2943.5 |
| stage/render/prepare_resources | 0.438 | 0.524 | 0.92 | 2759.9 |
| stage/render/prepare_meshes | 0.432 | 0.536 | 7.62 | 2722.3 |
| stage/main/first | 0.396 | 0.516 | 5.42 | 2494.2 |
| stage/main/pre_update | 0.354 | 0.520 | 2.82 | 2230.9 |
| stage/render/specialize | 0.309 | 0.413 | 11.24 | 1950.5 |
| stage/render/queue | 0.277 | 0.401 | 7.64 | 1748.0 |
| stage/render/prepare_bind_groups | 0.267 | 0.464 | 2.37 | 1685.7 |
| controller/feed/ms | 0.258 | 0.326 | 0.84 | 1629.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.230 | 0.338 | 0.94 | 1452.4 |
| bigspace/low_precision/ms | 0.212 | 0.647 | 1.83 | 1333.9 |
| stage/main/fixed_main_loop | 0.204 | 0.360 | 1.10 | 1284.8 |
| stream/drive/ms | 0.202 | 0.281 | 17.16 | 1272.7 |
| stage/render/create_views | 0.195 | 0.251 | 1.13 | 1227.7 |
| stage/render/prepare_batch_phases | 0.185 | 0.264 | 0.46 | 1166.1 |
| stage/render/extract_commands | 0.168 | 0.198 | 0.46 | 1059.7 |
| stage/render/prepare_flush | 0.163 | 0.315 | 1.29 | 1024.9 |
| anim/evaluate/ms | 0.155 | 0.246 | 2.52 | 978.2 |
| anim/evaluate_cpu/ms | 0.152 | 0.184 | 0.61 | 960.7 |
| stream/tick/ms | 0.146 | 0.211 | 0.65 | 921.9 |
| bigspace/high_precision/ms | 0.144 | 0.587 | 1.86 | 910.6 |
| stage/render/prepare_views | 0.138 | 0.170 | 0.63 | 867.0 |
| controller/move/ms | 0.135 | 0.324 | 0.56 | 853.1 |
| stage/main/last | 0.116 | 0.155 | 1.69 | 729.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.095 | 0.156 | 0.61 | 600.8 |
| render/acquire/ms | 0.094 | 0.120 | 0.33 | 593.6 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.123 | 0.33 | 360.7 |
| terrain/update/ms | 0.056 | 0.285 | 1.74 | 355.0 |
| pipelines/compile_ms | 0.046 | 0.000 | 122.55 | 289.3 |
| pipelines/front_ms | 0.044 | 0.000 | 118.69 | 277.7 |
| physics/step/ms | 0.043 | 0.084 | 0.58 | 273.1 |
| encode/resolve_diagnostics/sum | 0.034 | 0.056 | 0.59 | 212.0 |
| stage/render/cleanup | 0.031 | 0.093 | 0.96 | 198.4 |
| stage/render/phase_sort | 0.031 | 0.054 | 0.55 | 194.6 |
| physics/insert/ms | 0.031 | 0.331 | 1.00 | 193.2 |
| weather/map/ms | 0.025 | 0.033 | 0.15 | 157.1 |
| terrain/select/ms | 0.023 | 0.254 | 1.04 | 142.7 |
| far/select/ms | 0.016 | 0.170 | 0.47 | 99.2 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.021 | 0.52 | 94.0 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.53 | 74.2 |
| anim/apply/ms | 0.011 | 0.019 | 0.08 | 70.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.15 | 67.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.21 | 65.3 |
| script/update/ms | 0.007 | 0.009 | 0.14 | 42.9 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 33.9 |
| script/ui/ms | 0.005 | 0.006 | 0.03 | 30.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.03 | 29.6 |
| stream/spawn/ms | 0.004 | 0.000 | 1.11 | 28.1 |
| controller/write/ms | 0.004 | 0.005 | 0.05 | 22.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.5 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.06 | 11.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 6.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.09 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.3 |
| stage/render/prepare | 0.001 | 0.002 | 0.12 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.06 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 561 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 20 | 36 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 15 |
| crowd/c0/joints/count | count | 1306 | 2262 | 707 |
| crowd/c0/meshes/count | count | 355 | 646 | 189 |
| crowd/c1/count | count | 145 | 156 | 145 |
| crowd/c1/joints/count | count | 5531 | 6023 | 5457 |
| crowd/c1/meshes/count | count | 1173 | 1297 | 1144 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4605 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4584 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.40 | 1.52 | 0.36 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.52 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.53 | 0.02 |
| encode/finish_critical | ms | 0.95 | 3.36 | 0.81 |
| encode/pass:early prepass/max | ms | 0.77 | 1.81 | 0.80 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.95 | 0.63 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.61 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.94 | 0.26 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 3.36 | 0.81 |
| encode/pass:shadow_point_light/max | ms | 0.77 | 1.78 | 0.87 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.76 | 38.97 | 8.21 |
| gpu/timed/ms | ms | 7.85 | 9.25 | 8.21 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 823 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 823 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1276 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3428 | 5139 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 1276 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3428 | 5139 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 203 | 4 |
| phases/shadow/bins | count | 5031 | 8796 | 2421 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.1 k | 6368 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 247 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 559 | 6 |
| preprocess/bind_groups_created | count | 11 | 559 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 223 | 123 |
| process/cpu_usage | % | 26.92 | 39.42 | 24.04 |
| process/mem_usage | GiB | 5.46 | 5.63 | 5.63 |
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
| skin/blocks_changed/count | count | 13 | 132 | 15 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 367.1 k | 812.5 k | 493.3 k |
| skin/joints_written/count | count | 5736 | 12.7 k | 7708 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4916 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.45 | 39.91 | 29.49 |
| system/mem_usage | % | 54.93 | 55.53 | 55.48 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 61 | 250 | 54 |
| views/all/bins | count | 7609 | 11.7 k | 5919 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.1 k | 14.0 k |
| views/all/pipelines | count | 248 | 294 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.6 k | 34.9 k | 8119 |
| views/cascade/batch_sets | count | 9 | 45 | 4 |
| views/cascade/bins | count | 2808 | 6187 | 2421 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8917 | 23.2 k | 6368 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 409 | 22.7 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2147 | 2751 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2398 | 3048 | 0 |
| views/lamp_face/pipelines | count | 168 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2932 | 0 |
| views/main/batch_sets | count | 49 | 52 | 50 |
| views/main/bins | count | 2846 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8496 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 48 | 46 |
| views/main/work_items | count | 8655 | 13.2 k | 7918 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 50.88 | 232 | 320.8 k |
| anim/events/count | count | 0.08 | 3 | 508 |
| assets/Image/added | count | 0.09 | 37 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.64 | 201 | 4061 |
| assets/Mesh/removed | count | 0.77 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.43 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.74 | 1272 | 4675 |
| encode//count | count | 31.77 | 220 | 200.3 k |
| encode/bind_walk/init_textures | count | 11.5 k | 114.7 k | 72.59 M |
| encode/bind_walk/init_textures_skipped | count | 61.0 k | 133.2 k | 384.77 M |
| encode/bind_walk/inits | count | 449 | 1125 | 2.83 M |
| encode/bind_walk/inits_skipped | count | 105 | 159 | 663.7 k |
| encode/bind_walk/merge_textures | count | 11.4 k | 114.6 k | 72.14 M |
| encode/bind_walk/merge_textures_skipped | count | 5360 | 6012 | 33.79 M |
| encode/bind_walk/merges | count | 96.02 | 334 | 605.4 k |
| encode/bind_walk/merges_skipped | count | 82.83 | 132 | 522.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6305 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6305 |
| encode/pass:early prepass/count | count | 1 | 1 | 6305 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6305 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6305 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6305 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6305 |
| encode/pass:shadow_point_light/count | count | 0.96 | 48 | 6046 |
| encode/queue/command_buffers | count | 42.74 | 278 | 269.5 k |
| encode/queue/submits | count | 2.02 | 4 | 12.7 k |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 4.88 M | 11.80 G |
| encode/queue/write_buffer/calls | count | 250 | 838 | 1.58 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.8 k | 3.52 M | 1.12 G |
| encode/queue/write_buffer_with/calls | count | 10.92 | 612 | 68.8 k |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 71.91 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6305 |
| physics/inserts | count | 0.36 | 106 | 2247 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 175 | 2335 |
| physics/steps | count | 0.62 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 32 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5369 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 54 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1177 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.4 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2598.7 | main/busy/ms 2561, stage/main/post_update 2456, stage/main/update 95.88, surface/advance_cpu_ms 66.49, stage/main/pre_update 4.98, stage/main/last 1.20, surface/cell_upload_ms 0.77, stage/main/first 0.57, stream/drive/ms 0.48, stage/main/fixed_main_loop 0.38, weather/map/ms 0.14, stream/tick/ms 0.12 |
| warm-up | 3.37 | 772.3 | main/busy/ms 2714, stage/main/post_update 2486, render/thread/ms 738, render/busy/ms 568, stage/render/prepare_assets 519, stage/main/update 213, stage/render/prepare_views 167, render/acquire/ms 167, surface/advance_cpu_ms 66.49, stage/render/create_views 28.50, stage/render/render 12.20, render/graph/ms 7.78 |
| warm-up | 3.49 | 114.9 | render/thread/ms 836, render/busy/ms 610, stage/render/prepare_assets 545, main/busy/ms 176, stage/render/prepare_views 167, render/acquire/ms 167, pipelines/compile_ms 125, stage/main/update 121, pipelines/front_ms 79.80, stage/render/render 74.07, render/present/ms 58.60, pipelines/driver_ms 45.35 |
| warm-up | 3.52 | 29.6 | pipelines/compile_ms 367, pipelines/front_ms 291, render/thread/ms 119, pipelines/driver_ms 75.96, stage/render/render 75.83, render/busy/ms 59.98, render/present/ms 57.80, main/busy/ms 45.35, terrain/update/ms 30.36, pipelines/wait_ms 27.79, stage/main/post_update 27.46, stage/render/prepare_assets 27.35 |
| warm-up | 3.55 | 38.3 | pipelines/wait_ms 555, pipelines/compile_ms 534, pipelines/front_ms 486, main/busy/ms 67.56, render/thread/ms 51.30, stage/main/post_update 48.83, pipelines/driver_ms 48.38, render/busy/ms 46.98, stage/render/render 19.98, render/graph/ms 15.45, stage/render/prepare_assets 13.51, terrain/update/ms 13.43 |
| warm-up | 3.69 | 135.9 | pipelines/wait_ms 4861, pipelines/compile_ms 1695, pipelines/front_ms 1580, main/busy/ms 168, render/thread/ms 142, render/busy/ms 129, pipelines/driver_ms 115, stage/main/post_update 57.38, stage/main/update 51.10, stage/main/pre_update 48.00, stage/render/prepare_flush 47.51, stage/render/prepare_assets 26.49 |
| warm-up | 3.77 | 78.3 | pipelines/wait_ms 4551, pipelines/compile_ms 1974, pipelines/front_ms 1854, render/thread/ms 177, render/busy/ms 163, main/busy/ms 151, pipelines/driver_ms 120, stage/render/prepare_assets 64.55, stage/main/update 53.40, stage/main/pre_update 47.89, stage/render/prepare_flush 47.38, stage/main/post_update 36.90 |
| warm-up | 3.80 | 29.4 | pipelines/compile_ms 892, pipelines/front_ms 857, pipelines/wait_ms 269, render/thread/ms 91.65, render/busy/ms 89.58, stage/render/prepare_assets 69.31, main/busy/ms 60.09, pipelines/driver_ms 34.48, stage/main/post_update 23.78, stage/main/fixed_main_loop 20.57, stage/render/render 13.22, stage/main/update 11.46 |
| warm-up | 3.83 | 30.6 | pipelines/compile_ms 934, pipelines/front_ms 886, pipelines/wait_ms 96.64, render/thread/ms 50.69, render/busy/ms 49.17, pipelines/driver_ms 48.27, main/busy/ms 46.57, stage/render/prepare_assets 33.95, stage/main/post_update 20.55, stage/main/fixed_main_loop 14.19, terrain/update/ms 10.18, stage/main/update 7.98 |
| warm-up | 3.86 | 35.8 | pipelines/compile_ms 1237, pipelines/front_ms 1177, pipelines/wait_ms 189, pipelines/driver_ms 59.89, render/thread/ms 58.40, render/busy/ms 56.68, main/busy/ms 32.34, stage/render/prepare_assets 23.29, stage/main/post_update 22.02, stage/render/prepare_resources 11.59, stage/render/render 10.89, render/graph/ms 8.48 |
| warm-up | 3.91 | 30.8 | pipelines/compile_ms 279, pipelines/front_ms 272, render/thread/ms 46.40, render/busy/ms 44.45, stage/render/prepare_assets 19.51, main/busy/ms 19.02, encode//sum 18.08, stage/render/render 14.26, render/graph/ms 12.12, stage/main/post_update 9.10, terrain/update/ms 8.82, stage/render/graph_submit 8.39 |
| warm-up | 4.02 | 82.0 | pipelines/compile_ms 1284, pipelines/front_ms 1238, main/busy/ms 69.16, render/thread/ms 61.18, render/busy/ms 57.60, stage/main/update 48.41, pipelines/driver_ms 46.46, stream/drive/ms 42.95, stage/render/prepare_assets 38.67, stage/render/render 10.35, stage/main/post_update 8.45, render/graph/ms 8.42 |
| warm-up | 4.09 | 39.6 | pipelines/compile_ms 541, pipelines/front_ms 520, render/thread/ms 49.89, render/busy/ms 48.29, main/busy/ms 46.61, stage/main/update 33.63, stream/drive/ms 29.56, stage/render/render 23.26, render/graph/ms 21.22, pipelines/driver_ms 20.81, pipelines/wait_ms 20.67, stage/render/prepare_assets 19.07 |
| warm-up | 4.14 | 46.3 | pipelines/compile_ms 716, pipelines/front_ms 693, render/thread/ms 64.81, render/busy/ms 63.32, main/busy/ms 60.96, stage/render/prepare_assets 43.30, stage/main/update 28.23, stage/main/post_update 25.64, stream/drive/ms 24.46, pipelines/driver_ms 23.40, pipelines/wait_ms 19.33, stage/render/render 11.98 |
| warm-up | 4.19 | 50.5 | pipelines/compile_ms 295, pipelines/front_ms 282, render/thread/ms 70.01, render/busy/ms 67.71, main/busy/ms 64.69, stage/render/prepare_assets 41.29, stage/main/update 29.98, stage/main/post_update 27.08, stream/drive/ms 25.93, stage/render/render 15.87, render/graph/ms 12.78, pipelines/driver_ms 12.41 |
| warm-up | 4.24 | 52.8 | pipelines/compile_ms 750, pipelines/front_ms 717, render/thread/ms 62.90, render/busy/ms 60.43, stage/render/prepare_assets 37.10, pipelines/driver_ms 32.67, main/busy/ms 32.28, stage/main/update 15.10, stage/render/render 13.26, render/graph/ms 10.08, stage/main/post_update 8.69, stage/render/graph_submit 7.18 |
| warm-up | 4.31 | 67.1 | pipelines/compile_ms 1185, pipelines/front_ms 1139, render/thread/ms 81.00, render/busy/ms 78.98, main/busy/ms 73.80, stage/main/update 55.10, stage/render/prepare_assets 48.82, pipelines/driver_ms 45.98, stage/render/render 12.33, stage/main/post_update 10.71, render/graph/ms 9.99, stage/render/graph_submit 7.48 |
| warm-up | 4.40 | 96.4 | pipelines/compile_ms 1421, pipelines/front_ms 1372, render/thread/ms 115, render/busy/ms 112, main/busy/ms 84.86, stage/render/prepare_assets 58.54, stage/main/update 53.31, pipelines/driver_ms 48.88, stage/main/post_update 23.95, stage/render/render 23.92, render/graph/ms 20.46, stage/render/graph_submit 17.68 |
| warm-up | 4.47 | 67.2 | pipelines/compile_ms 1745, pipelines/front_ms 1691, pipelines/wait_ms 183, render/thread/ms 130, render/busy/ms 127, main/busy/ms 54.61, pipelines/driver_ms 53.88, stage/render/prepare_assets 38.35, stage/render/prepare_bind_groups 29.77, stage/main/post_update 24.13, stage/render/render 23.19, render/graph/ms 19.80 |
| warm-up | 4.58 | 110.1 | pipelines/compile_ms 2159, pipelines/front_ms 2098, pipelines/wait_ms 743, render/thread/ms 160, render/busy/ms 159, pipelines/driver_ms 61.16, main/busy/ms 52.06, stage/render/prepare_assets 29.78, stage/render/prepare_resources 27.85, stage/render/prepare_bind_groups 26.90, stage/render/render 24.03, render/graph/ms 21.75 |
| warm-up | 4.66 | 75.5 | pipelines/compile_ms 2409, pipelines/front_ms 2330, pipelines/wait_ms 816, render/thread/ms 170, render/busy/ms 168, pipelines/driver_ms 79.29, main/busy/ms 76.66, encode//sum 59.44, stage/render/render 38.47, render/graph/ms 36.03, stage/main/post_update 34.36, stage/render/prepare_resources 31.59 |
| warm-up | 4.74 | 83.5 | pipelines/compile_ms 2625, pipelines/front_ms 2522, pipelines/wait_ms 1636, render/thread/ms 133, render/busy/ms 131, pipelines/driver_ms 103, main/busy/ms 80.91, encode//sum 59.32, stage/render/render 33.09, stage/main/update 30.99, render/graph/ms 30.64, stage/main/post_update 23.96 |
| warm-up | 4.84 | 74.5 | pipelines/compile_ms 1039, pipelines/front_ms 1011, pipelines/wait_ms 225, main/busy/ms 107, stage/main/update 79.57, stream/drive/ms 55.43, render/thread/ms 45.26, render/busy/ms 42.67, pipelines/driver_ms 27.71, stage/render/render 15.89, stage/main/post_update 15.87, render/graph/ms 12.08 |
| warm-up | 4.93 | 84.0 | pipelines/compile_ms 1673, pipelines/front_ms 1618, pipelines/wait_ms 447, main/busy/ms 124, stage/main/update 96.85, render/thread/ms 73.79, render/busy/ms 70.90, pipelines/driver_ms 54.74, stream/drive/ms 52.84, stage/render/prepare_assets 28.37, stage/render/render 19.36, render/graph/ms 15.00 |
| warm-up | 5.08 | 154.9 | pipelines/compile_ms 2097, pipelines/front_ms 2031, pipelines/wait_ms 370, render/thread/ms 143, render/busy/ms 141, main/busy/ms 115, stage/main/update 90.67, stage/render/prepare_assets 69.07, pipelines/driver_ms 66.59, stream/drive/ms 51.57, stage/render/prepare_write_phase_buffers 31.40, stage/render/render 20.31 |
| warm-up | 5.13 | 47.1 | pipelines/compile_ms 1362, pipelines/front_ms 1310, render/thread/ms 133, render/busy/ms 130, main/busy/ms 88.30, stage/main/update 63.02, stage/render/prepare_assets 52.98, pipelines/driver_ms 51.34, stream/drive/ms 38.02, stage/render/render 33.49, stage/render/prepare_write_phase_buffers 31.24, render/graph/ms 29.92 |
| warm-up | 5.18 | 53.9 | pipelines/compile_ms 324, pipelines/front_ms 302, main/busy/ms 77.41, render/thread/ms 65.52, render/busy/ms 62.55, stage/main/update 57.56, stream/drive/ms 37.82, stage/render/render 28.29, render/graph/ms 24.74, stage/render/prepare_assets 22.93, stage/render/graph_submit 21.89, pipelines/driver_ms 21.69 |
| warm-up | 5.21 | 31.0 | main/busy/ms 69.80, stage/main/update 54.14, render/thread/ms 50.80, render/busy/ms 49.39, stream/drive/ms 37.02, stage/render/prepare_assets 21.95, pipelines/compile_ms 21.38, pipelines/front_ms 20.89, stage/render/render 13.58, render/graph/ms 11.57, stage/main/post_update 8.79, encode//sum 8.78 |
| warm-up | 5.27 | 54.4 | pipelines/compile_ms 562, pipelines/front_ms 545, main/busy/ms 63.32, stage/main/update 47.84, render/thread/ms 46.49, render/busy/ms 45.15, stream/drive/ms 37.30, pipelines/driver_ms 17.31, stage/render/prepare_assets 16.97, stage/render/render 12.88, render/graph/ms 11.01, encode//sum 9.20 |
| warm-up | 5.37 | 32.3 | pipelines/compile_ms 133, pipelines/front_ms 129, render/thread/ms 49.47, render/busy/ms 48.11, main/busy/ms 35.85, stage/render/render 18.05, render/graph/ms 16.15, encode//sum 16.02, stage/main/update 14.90, stage/main/post_update 14.68, stage/render/graph_submit 12.15, stage/render/prepare_write_phase_buffers 7.98 |
| warm-up | 5.40 | 30.3 | pipelines/compile_ms 409, pipelines/front_ms 398, render/thread/ms 54.75, render/busy/ms 53.27, main/busy/ms 39.50, stage/render/render 18.89, render/graph/ms 16.75, stage/main/update 16.44, stage/main/post_update 14.44, encode//sum 13.99, stage/render/graph_submit 12.96, stage/render/prepare_assets 11.62 |
| warm-up | 5.43 | 29.8 | pipelines/compile_ms 773, pipelines/front_ms 752, render/thread/ms 45.90, render/busy/ms 44.49, main/busy/ms 38.85, pipelines/driver_ms 21.81, stage/render/prepare_assets 17.10, stage/main/update 15.73, stage/render/render 13.83, render/graph/ms 11.67, stage/main/post_update 11.13, encode//sum 10.54 |
| warm-up | 5.48 | 41.4 | pipelines/compile_ms 666, pipelines/front_ms 644, render/thread/ms 57.20, render/busy/ms 55.85, main/busy/ms 35.47, stage/render/prepare_assets 33.42, pipelines/driver_ms 21.28, stage/main/post_update 14.12, stage/main/update 11.98, stage/render/render 11.17, encode//sum 9.69, render/graph/ms 9.15 |
| warm-up | 5.58 | 37.8 | pipelines/compile_ms 411, pipelines/front_ms 399, render/thread/ms 46.26, render/busy/ms 44.80, main/busy/ms 31.35, stage/render/prepare_assets 23.04, stage/main/update 15.46, stage/render/render 12.06, pipelines/driver_ms 11.89, render/graph/ms 10.15, stage/main/post_update 9.47, encode//sum 8.91 |
| warm-up | 5.66 | 31.2 | pipelines/compile_ms 573, pipelines/front_ms 560, render/thread/ms 50.23, render/busy/ms 48.79, main/busy/ms 39.57, stage/render/prepare_assets 23.16, stage/main/update 20.13, stage/main/post_update 13.35, pipelines/driver_ms 12.79, stage/render/render 10.88, stream/drive/ms 8.76, render/graph/ms 8.68 |
| warm-up | 5.69 | 29.7 | pipelines/compile_ms 221, pipelines/front_ms 216, render/thread/ms 52.83, render/busy/ms 51.39, main/busy/ms 36.16, stage/render/prepare_assets 25.13, stage/main/update 16.25, stage/main/post_update 14.10, stage/render/render 10.98, render/graph/ms 8.86, stream/drive/ms 8.20, encode//sum 7.00 |
| warm-up | 5.76 | 40.9 | pipelines/compile_ms 234, pipelines/front_ms 219, render/thread/ms 60.26, render/busy/ms 58.75, stage/render/prepare_assets 35.85, main/busy/ms 29.29, pipelines/driver_ms 14.63, stage/render/render 12.18, stage/main/update 12.18, stage/main/post_update 11.16, render/graph/ms 9.97, encode//sum 7.58 |
| warm-up | 6.29 | 32.9 | render/thread/ms 41.17, render/busy/ms 40.08, main/busy/ms 29.17, stage/render/prepare_assets 15.73, stage/main/update 12.62, stage/main/post_update 9.80, stage/render/render 8.97, encode//sum 7.63, render/graph/ms 7.33, stage/render/specialize 5.27, stage/render/graph_submit 5.19, stream/drive/ms 4.71 |
| warm-up | 6.52 | 29.2 | render/thread/ms 46.10, render/busy/ms 44.45, main/busy/ms 40.14, stage/main/post_update 17.40, stage/render/render 16.72, stage/main/update 15.73, render/graph/ms 14.32, stage/render/graph_submit 10.44, encode//sum 9.05, stage/render/prepare_assets 8.24, encode/submit 6.39, stream/drive/ms 5.40 |
| warm-up | 6.62 | 43.0 | main/busy/ms 49.09, render/thread/ms 33.38, render/busy/ms 32.34, stage/main/update 28.47, stage/main/post_update 14.92, stream/drive/ms 14.25, stage/render/render 12.39, render/graph/ms 10.56, encode//sum 8.55, stage/render/prepare_assets 7.77, stage/render/graph_submit 7.67, encode/submit 4.12 |
| warm-up | 7.04 | 31.5 | pipelines/compile_ms 137, pipelines/front_ms 134, main/busy/ms 40.68, render/thread/ms 28.47, render/busy/ms 27.41, stage/main/update 23.15, stage/render/prepare_assets 13.33, stream/drive/ms 13.09, stage/main/post_update 10.61, stage/render/render 7.74, encode//sum 6.64, render/graph/ms 6.23 |
| warm-up | 7.08 | 34.2 | render/thread/ms 43.91, render/busy/ms 42.84, main/busy/ms 26.72, pipelines/compile_ms 14.49, pipelines/front_ms 13.71, stage/render/specialize 12.67, stage/main/update 10.82, stage/render/render 10.81, stage/main/post_update 10.80, render/graph/ms 9.25, stage/render/queue 7.35, stage/render/graph_submit 7.20 |
| warm-up | 8.32 | 30.1 | render/thread/ms 36.90, render/busy/ms 35.71, main/busy/ms 17.18, stage/render/render 11.03, stage/main/post_update 9.98, render/graph/ms 9.37, stage/render/specialize 9.20, stage/render/graph_submit 7.33, encode//sum 6.67, stage/render/queue 6.20, stage/main/update 4.99, encode/submit 4.10 |
| measure | 0.38 | 28.9 | render/thread/ms 32.90, render/busy/ms 31.92, main/busy/ms 15.60, stage/main/post_update 9.89, stage/render/render 9.62, stage/render/specialize 8.86, render/graph/ms 8.24, encode//sum 6.41, stage/render/graph_submit 6.28, stage/render/queue 6.01, encode/submit 3.38, stage/main/update 3.29 |
| measure | 1.65 | 32.0 | render/thread/ms 38.38, render/busy/ms 37.34, main/busy/ms 25.00, stage/main/post_update 14.77, stage/render/specialize 10.19, stage/render/render 9.91, render/graph/ms 8.50, encode//sum 7.13, stage/render/queue 6.47, stage/main/update 6.31, stage/render/graph_submit 6.21, stage/render/prepare_meshes 4.09 |
| measure | 3.03 | 29.7 | render/thread/ms 36.92, render/busy/ms 35.91, main/busy/ms 18.27, stage/main/post_update 11.98, stage/render/render 11.37, render/graph/ms 9.97, stage/render/specialize 8.84, encode//sum 8.60, stage/render/graph_submit 7.42, stage/render/queue 5.99, encode/pass:shadow_point_light/sum 5.77, stage/render/prepare_meshes 4.99 |
| measure | 9.44 | 30.3 | render/thread/ms 35.23, render/busy/ms 34.18, main/busy/ms 20.35, stage/main/post_update 11.14, stage/render/render 10.12, stage/render/specialize 10.12, render/graph/ms 8.70, encode//sum 7.15, stage/render/queue 6.76, stage/render/graph_submit 6.56, stage/main/update 6.45, encode/submit 3.73 |
| measure | 15.82 | 29.0 | render/thread/ms 37.14, render/busy/ms 36.07, main/busy/ms 17.38, stage/render/render 11.69, stage/main/post_update 10.98, render/graph/ms 10.28, stage/render/specialize 9.95, stage/render/graph_submit 8.13, encode//sum 7.16, stage/render/queue 6.89, encode/submit 5.49, stage/main/update 4.03 |
| measure | 17.16 | 29.4 | render/thread/ms 37.73, render/busy/ms 36.67, main/busy/ms 19.82, stage/main/post_update 12.27, stage/render/render 11.72, render/graph/ms 10.26, stage/render/specialize 9.58, encode//sum 8.59, stage/render/graph_submit 7.73, stage/render/queue 6.91, encode/submit 4.99, stage/main/update 4.60 |
| measure | 20.66 | 32.5 | render/thread/ms 34.40, render/busy/ms 33.37, main/busy/ms 19.39, stage/main/post_update 11.51, stage/render/specialize 10.34, stage/render/render 9.09, encode//sum 7.99, render/graph/ms 7.72, stage/render/queue 6.50, stage/render/graph_submit 5.78, stage/main/update 4.84, stage/render/prepare_meshes 3.29 |
| measure | 23.00 | 32.6 | render/thread/ms 41.37, render/busy/ms 40.42, main/busy/ms 22.94, stage/render/render 11.96, stage/main/post_update 11.83, stage/render/specialize 11.59, render/graph/ms 10.62, encode//sum 10.43, stage/render/graph_submit 8.21, stage/render/queue 7.96, stage/main/update 5.72, encode/submit 4.90 |
| measure | 24.43 | 29.9 | render/thread/ms 37.19, render/busy/ms 36.15, main/busy/ms 19.12, stage/main/post_update 12.22, stage/render/render 10.94, stage/render/specialize 10.11, render/graph/ms 9.59, encode//sum 7.98, stage/render/graph_submit 7.26, stage/render/queue 6.39, encode/submit 4.60, stage/main/update 4.03 |
| measure | 27.28 | 35.8 | render/thread/ms 46.44, render/busy/ms 45.35, main/busy/ms 25.54, stage/render/render 16.93, render/graph/ms 15.41, encode//sum 15.35, stage/main/post_update 13.07, stage/render/graph_submit 12.04, encode/pass:shadow_point_light/sum 11.82, stage/render/specialize 11.49, encode/submit 7.95, stage/render/queue 7.95 |
| measure | 28.72 | 29.6 | render/thread/ms 37.13, render/busy/ms 36.10, main/busy/ms 19.03, stage/main/post_update 12.50, stage/render/render 10.91, stage/render/specialize 9.84, render/graph/ms 9.51, encode//sum 7.90, stage/render/graph_submit 7.22, stage/render/queue 6.69, encode/submit 4.33, stage/main/update 3.78 |
| measure | 30.62 | 34.4 | render/thread/ms 40.55, render/busy/ms 39.54, main/busy/ms 20.67, stage/render/render 13.09, stage/main/post_update 12.48, render/graph/ms 11.54, stage/render/specialize 10.82, stage/render/graph_submit 8.94, encode//sum 8.76, stage/render/queue 7.29, encode/pass:shadow_point_light/sum 5.97, encode/submit 5.46 |
| measure | 35.96 | 33.0 | render/thread/ms 35.84, render/busy/ms 34.90, main/busy/ms 21.53, stage/main/post_update 11.57, stage/render/render 10.62, stage/render/specialize 10.01, render/graph/ms 9.22, encode//sum 7.16, stage/render/graph_submit 7.12, stage/render/queue 6.29, stage/main/update 4.82, encode/submit 3.62 |
| measure | 37.26 | 31.6 | render/thread/ms 40.94, render/busy/ms 39.98, main/busy/ms 21.96, stage/main/post_update 11.63, stage/render/specialize 11.52, stage/render/render 11.40, render/graph/ms 9.97, stage/render/graph_submit 7.99, stage/render/queue 7.42, encode//sum 7.16, stage/main/update 6.87, encode/submit 4.13 |
| measure | 37.48 | 30.0 | render/thread/ms 36.24, render/busy/ms 35.18, main/busy/ms 18.30, stage/render/render 10.63, stage/main/post_update 10.46, stage/render/specialize 10.22, render/graph/ms 8.97, encode//sum 7.96, stage/render/graph_submit 6.96, stage/render/queue 6.27, stage/main/update 5.24, encode/submit 3.89 |
| measure | 41.17 | 33.4 | render/thread/ms 39.90, render/busy/ms 38.83, main/busy/ms 17.86, stage/render/specialize 10.85, stage/render/render 10.61, stage/main/post_update 10.50, render/graph/ms 9.17, stage/render/graph_submit 7.41, encode//sum 7.34, stage/render/queue 6.99, encode/submit 4.28, stage/main/update 4.19 |
| measure | 43.85 | 29.6 | render/thread/ms 36.68, render/busy/ms 35.61, main/busy/ms 16.64, stage/render/render 10.68, stage/main/post_update 10.09, stage/render/specialize 9.94, render/graph/ms 9.26, stage/render/graph_submit 7.50, stage/render/queue 6.65, encode//sum 6.32, encode/submit 4.33, stage/main/update 3.97 |
| measure | 46.13 | 32.3 | render/thread/ms 38.39, render/busy/ms 37.10, main/busy/ms 17.53, stage/render/render 10.55, stage/render/specialize 10.42, stage/main/post_update 9.96, render/graph/ms 9.12, encode//sum 8.05, stage/render/graph_submit 7.03, stage/render/queue 6.25, stage/main/update 4.90, stage/render/prepare_meshes 4.22 |
| measure | 59.25 | 46.0 | render/thread/ms 57.26, render/busy/ms 55.71, main/busy/ms 32.70, stage/main/post_update 19.59, stage/render/render 18.21, render/graph/ms 16.11, stage/render/graph_submit 12.98, stage/render/specialize 10.16, encode//sum 9.60, stage/main/update 8.69, stage/render/prepare_meshes 8.00, encode/submit 7.16 |
| measure | 59.28 | 29.6 | render/thread/ms 59.29, render/busy/ms 58.25, main/busy/ms 25.62, stage/render/prepare_assets 16.72, stage/main/post_update 13.59, stage/render/specialize 10.36, stage/render/render 10.12, stage/render/prepare_meshes 8.89, render/graph/ms 8.79, encode//sum 7.05, stage/render/graph_submit 6.86, stage/render/queue 6.75 |
| measure | 63.20 | 34.0 | render/thread/ms 36.33, render/busy/ms 35.34, main/busy/ms 18.69, stage/render/specialize 10.32, stage/render/render 10.16, stage/main/post_update 9.89, render/graph/ms 8.91, encode//sum 7.91, stage/render/graph_submit 6.72, stage/render/queue 6.20, stage/main/update 5.72, stage/render/prepare_meshes 4.35 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=select-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 14%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **81%** (min 51%) · 2621 MHz (min 2610) · memory 8101 MHz · 83.0 W · 77 °C (max 82) · VRAM 7054 MiB · throttled 100% of samples

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
- `select.gate=on (command line: --set)`
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
