# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5955 frames over 65.6 s · **median 10.78 ms (93 fps)** · mean 11.02 · p95 13.52 · p99 16.82 (1% low 59 fps) · worst 44.25 ms · 65 spikes over 1.5× median · 53.0 stutters/min (work 51.2/min)
- warm-up: 323 frames over 9.2 s, worst 2561.3 ms
- hitches (> 27.0 ms): **32** measured, 54 in warm-up
- timed GPU passes: 6.48 ms of a 11.02 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.78 ms, limit 11 ms
- **FAIL** frame_ms: 58 stutters (> 1.5x moving median of 60) in 1.09 min = 53.01/min, limit 1/min
- **FAIL** worst frame 44.25 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.847 | 2.036 | 2.071 | 0.91 | 0.174 |
| main_opaque_pass_3d | 1.471 | 1.622 | 2.047 | 0.91 | 0.078 |
| early prepass | 0.443 | 0.488 | 0.717 | 0.91 | 0.089 |
| bloom | 0.286 | 0.315 | 0.322 | 0.91 | 0.062 |
| atmosphere_luts | 0.276 | 0.304 | 0.366 | 0.91 | 0.004 |
| water_velocity | 0.254 | 0.280 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.183 | 0.202 | 0.203 | 0.91 | 0.003 |
| ssao | 0.172 | 0.189 | 0.194 | 0.91 | 0.008 |
| atmos_froxel_inject | 0.132 | 0.146 | 0.148 | 0.91 | 0.002 |
| clouds_trace | 0.128 | 0.141 | 0.232 | 0.91 | 0.001 |
| water_fft | 0.119 | 0.132 | 0.138 | 0.91 | 0.003 |
| uniform_allocation | 0.118 | 0.130 | 0.183 | 0.91 | 0.028 |
| upscaling | 0.107 | 0.118 | 0.126 | 0.91 | 0.004 |
| auxiliary_views | 0.103 | 0.113 | 0.489 | 0.91 | 0.121 |
| clouds_composite | 0.096 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.095 | 0.105 | 0.142 | 0.91 | 0.020 |
| main_transmissive_pass_3d | 0.077 | 0.085 | 0.179 | 0.91 | 0.009 |
| tonemapping | 0.077 | 0.085 | 0.088 | 0.91 | 0.003 |
| clustering | 0.058 | 0.064 | 0.068 | 0.91 | 0.019 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.91 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.65 | 0.003 |
| surface_exposure_far | 0.034 | 0.043 | 0.045 | 0.81 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.033 | 0.91 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.031 | 0.033 | 0.91 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.046 | 0.91 | 0.005 |
| water_snapshot | 0.027 | 0.030 | 0.034 | 0.91 | 0.002 |
| lightprobe_irradiance_map | 0.027 | 0.413 | 0.416 | 0.07 | 0.001 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.014 | 0.219 | 0.229 | 0.07 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.91 | 0.125 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.014 | 0.015 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.521 | 11.663 | 37.67 | 56696.0 |
| main/busy/ms | 9.161 | 10.817 | 25.92 | 54554.9 |
| render/busy/ms | 8.967 | 11.144 | 37.12 | 53397.2 |
| stage/main/post_update | 5.683 | 7.048 | 12.96 | 33840.1 |
| stage/render/render | 5.511 | 6.899 | 14.82 | 32815.2 |
| render/graph/ms | 4.762 | 6.162 | 13.82 | 28357.1 |
| encode//sum | 3.984 | 5.096 | 28.63 | 23725.2 |
| stage/render/graph_submit | 3.571 | 4.725 | 10.52 | 21266.2 |
| encode/submit | 2.247 | 3.217 | 7.15 | 13382.3 |
| stage/main/update | 2.194 | 2.594 | 19.30 | 13065.6 |
| encode/finish_wall | 1.402 | 2.056 | 5.11 | 8351.8 |
| stage/render/camera_driver | 1.137 | 1.551 | 3.80 | 6772.6 |
| encode/pass:shadow_directional_light/sum | 0.910 | 1.617 | 2.66 | 5421.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.801 | 1.123 | 1.63 | 4769.3 |
| encode/pass:early prepass/sum | 0.791 | 1.112 | 2.21 | 4710.5 |
| stage/render/prepare_write_phase_buffers | 0.759 | 1.113 | 2.50 | 4522.1 |
| encode/pass:shadow_point_light/sum | 0.682 | 2.973 | 16.79 | 4058.8 |
| stage/render/prepare_assets | 0.581 | 0.758 | 10.96 | 3457.3 |
| bigspace/total/ms | 0.553 | 1.344 | 4.29 | 3294.2 |
| stage/render/prepare_meshes | 0.467 | 0.605 | 6.98 | 2781.2 |
| render/present/ms | 0.461 | 1.191 | 2.29 | 2744.9 |
| stage/render/prepare_resources | 0.459 | 0.551 | 1.02 | 2735.4 |
| stage/main/first | 0.398 | 0.522 | 5.62 | 2369.3 |
| stage/main/pre_update | 0.370 | 0.524 | 1.72 | 2203.9 |
| stage/render/specialize | 0.338 | 0.427 | 12.14 | 2014.0 |
| terrain/update/ms | 0.307 | 0.403 | 2.30 | 1829.9 |
| stage/render/queue | 0.302 | 0.428 | 7.63 | 1798.9 |
| stage/render/prepare_bind_groups | 0.296 | 0.523 | 3.02 | 1760.9 |
| controller/feed/ms | 0.281 | 0.352 | 1.01 | 1673.2 |
| terrain/select/ms | 0.273 | 0.359 | 0.92 | 1625.2 |
| stream/drive/ms | 0.257 | 0.345 | 17.28 | 1529.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.251 | 0.365 | 1.18 | 1494.6 |
| bigspace/low_precision/ms | 0.231 | 0.767 | 1.77 | 1377.9 |
| stage/main/fixed_main_loop | 0.223 | 0.376 | 1.31 | 1328.6 |
| stage/render/create_views | 0.201 | 0.260 | 0.99 | 1198.3 |
| stream/tick/ms | 0.199 | 0.276 | 0.69 | 1186.6 |
| far/select/ms | 0.197 | 0.270 | 0.57 | 1174.8 |
| stage/render/prepare_batch_phases | 0.192 | 0.275 | 0.49 | 1142.6 |
| stage/render/extract_commands | 0.179 | 0.208 | 0.46 | 1065.7 |
| anim/evaluate/ms | 0.165 | 0.278 | 1.67 | 984.1 |
| stage/render/prepare_flush | 0.165 | 0.334 | 0.86 | 983.5 |
| bigspace/high_precision/ms | 0.157 | 0.678 | 3.58 | 936.2 |
| anim/evaluate_cpu/ms | 0.154 | 0.188 | 0.69 | 919.2 |
| controller/move/ms | 0.149 | 0.354 | 0.61 | 886.5 |
| stage/render/prepare_views | 0.137 | 0.169 | 0.47 | 816.3 |
| stage/main/last | 0.116 | 0.163 | 1.60 | 692.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.106 | 0.167 | 0.72 | 628.7 |
| render/acquire/ms | 0.093 | 0.119 | 0.40 | 553.9 |
| stage/render/prepare_collect_phase_buffers | 0.066 | 0.137 | 0.51 | 390.1 |
| pipelines/compile_ms | 0.051 | 0.000 | 129.75 | 302.9 |
| pipelines/front_ms | 0.049 | 0.000 | 125.71 | 290.6 |
| physics/step/ms | 0.048 | 0.088 | 0.59 | 284.1 |
| stage/render/cleanup | 0.034 | 0.107 | 0.91 | 200.3 |
| physics/insert/ms | 0.033 | 0.370 | 0.93 | 199.1 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.53 | 191.2 |
| stage/render/phase_sort | 0.028 | 0.048 | 0.24 | 164.6 |
| weather/map/ms | 0.025 | 0.033 | 0.21 | 150.5 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.022 | 0.60 | 94.5 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.52 | 72.6 |
| anim/apply/ms | 0.012 | 0.021 | 0.10 | 69.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.24 | 62.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.20 | 61.9 |
| script/update/ms | 0.008 | 0.009 | 0.09 | 46.3 |
| script/input/ms | 0.006 | 0.007 | 0.15 | 34.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.15 | 30.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.15 | 28.8 |
| script/ui/ms | 0.004 | 0.005 | 0.18 | 24.4 |
| controller/write/ms | 0.003 | 0.005 | 0.05 | 20.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.66 | 13.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.03 | 12.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 19 |
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
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4617 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4596 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.43 | 2.02 | 0.55 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.60 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.52 | 0.02 |
| encode/finish_critical | ms | 0.99 | 2.66 | 1.53 |
| encode/pass:early prepass/max | ms | 0.81 | 2.21 | 1.02 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 1.63 | 0.85 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.72 | 0.18 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 1.18 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.84 | 2.66 | 1.53 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.88 | 0.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.42 | 37.12 | 9.87 |
| gpu/timed/ms | ms | 7.88 | 8.92 | 8.21 |
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
| phases/alpha_mask3d/items | count | 820 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 820 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1281 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3420 | 5143 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1281 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3420 | 5143 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 203 | 29 |
| phases/shadow/bins | count | 5035 | 8795 | 2421 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6376 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 252 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 563 | 61 |
| preprocess/bind_groups_created | count | 13 | 563 | 61 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 232 | 120 |
| process/cpu_usage | % | 26.44 | 40.36 | 24.66 |
| process/mem_usage | GiB | 5.49 | 5.66 | 5.66 |
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
| skin/blocks_changed/count | count | 13 | 130 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 368.9 k | 811.0 k | 498.4 k |
| skin/joints_written/count | count | 5764 | 12.7 k | 7788 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5011 | 5011 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.85 | 52.57 | 29.43 |
| system/mem_usage | % | 67.19 | 67.80 | 67.74 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 64 | 252 | 79 |
| views/all/bins | count | 7608 | 11.7 k | 5919 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 250 | 299 | 75 |
| views/all/pipelines_distinct | count | 70 | 73 | 61 |
| views/all/work_items | count | 10.7 k | 33.8 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 54 | 29 |
| views/cascade/bins | count | 2808 | 6184 | 2421 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8916 | 23.5 k | 6376 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 409 | 23.2 k | 6376 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2148 | 2751 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2402 | 3046 | 0 |
| views/lamp_face/pipelines | count | 168 | 207 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2933 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2862 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8478 | 13.0 k | 7618 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8638 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 51.24 | 270 | 305.2 k |
| anim/events/count | count | 0.09 | 3 | 542 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.64 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1272 | 4675 |
| encode//count | count | 32.24 | 220 | 192.0 k |
| encode/bind_walk/init_textures | count | 11.8 k | 114.7 k | 70.21 M |
| encode/bind_walk/init_textures_skipped | count | 66.4 k | 143.7 k | 395.38 M |
| encode/bind_walk/inits | count | 453 | 1129 | 2.70 M |
| encode/bind_walk/inits_skipped | count | 109 | 163 | 646.7 k |
| encode/bind_walk/merge_textures | count | 11.7 k | 114.6 k | 69.78 M |
| encode/bind_walk/merge_textures_skipped | count | 5365 | 6012 | 31.95 M |
| encode/bind_walk/merges | count | 96.65 | 334 | 575.6 k |
| encode/bind_walk/merges_skipped | count | 83.99 | 133 | 500.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5955 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5955 |
| encode/pass:early prepass/count | count | 1 | 1 | 5955 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5955 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5955 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5955 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5955 |
| encode/pass:shadow_point_light/count | count | 1.07 | 48 | 6374 |
| encode/queue/command_buffers | count | 43.33 | 278 | 258.0 k |
| encode/queue/submits | count | 2.02 | 4 | 12.0 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.63 M | 11.22 G |
| encode/queue/write_buffer/calls | count | 254 | 843 | 1.51 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.8 k | 3.52 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 11.03 | 465 | 65.7 k |
| encode/queue/write_texture/bytes | bytes | 12.1 k | 8.39 M | 71.91 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5955 |
| physics/inserts | count | 0.38 | 94 | 2261 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5685 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1246 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.0 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2561.3 | main/busy/ms 2531, stage/main/post_update 2427, stage/main/update 95.71, surface/advance_cpu_ms 66.91, stage/main/pre_update 5.14, stage/main/last 1.04, surface/cell_upload_ms 0.75, stream/drive/ms 0.58, stage/main/first 0.48, stage/main/fixed_main_loop 0.47, script/input/ms 0.24, stream/tick/ms 0.14 |
| warm-up | 3.21 | 649.1 | main/busy/ms 2672, stage/main/post_update 2453, render/thread/ms 612, render/busy/ms 500, stage/render/prepare_assets 472, stage/main/update 204, stage/render/prepare_views 110, render/acquire/ms 109, surface/advance_cpu_ms 66.91, stage/render/render 12.42, stage/render/create_views 8.25, render/graph/ms 7.85 |
| warm-up | 3.33 | 115.8 | render/thread/ms 710, render/busy/ms 542, stage/render/prepare_assets 497, main/busy/ms 162, pipelines/compile_ms 133, stage/main/update 113, stage/render/prepare_views 110, render/acquire/ms 110, pipelines/front_ms 83.40, stage/render/render 75.28, render/present/ms 58.32, pipelines/driver_ms 50.06 |
| warm-up | 3.37 | 44.6 | pipelines/compile_ms 434, pipelines/front_ms 352, render/thread/ms 126, stage/render/render 84.46, pipelines/driver_ms 82.30, pipelines/wait_ms 78.50, main/busy/ms 72.18, render/busy/ms 64.10, render/present/ms 61.75, stage/main/post_update 55.18, stage/render/prepare_assets 26.56, terrain/update/ms 26.33 |
| warm-up | 3.51 | 136.9 | pipelines/wait_ms 4071, pipelines/compile_ms 1521, pipelines/front_ms 1351, pipelines/driver_ms 170, render/thread/ms 153, main/busy/ms 147, render/busy/ms 144, stage/main/update 63.24, stage/main/post_update 60.90, stage/render/render 49.64, render/graph/ms 39.90, terrain/update/ms 27.22 |
| warm-up | 3.58 | 70.1 | pipelines/wait_ms 4230, pipelines/compile_ms 1897, pipelines/front_ms 1727, render/thread/ms 184, render/busy/ms 181, pipelines/driver_ms 169, main/busy/ms 125, stage/main/update 73.55, stage/render/prepare_assets 60.03, stage/render/render 34.09, render/graph/ms 30.34, stage/main/post_update 29.46 |
| warm-up | 3.62 | 38.0 | pipelines/compile_ms 1378, pipelines/front_ms 1295, pipelines/wait_ms 252, render/thread/ms 94.56, render/busy/ms 92.63, pipelines/driver_ms 82.74, stage/render/prepare_assets 73.68, main/busy/ms 43.99, stage/main/post_update 19.97, stage/main/update 18.22, terrain/update/ms 14.52, stage/render/render 11.29 |
| warm-up | 3.68 | 39.5 | pipelines/compile_ms 750, pipelines/front_ms 725, pipelines/wait_ms 201, render/thread/ms 61.27, render/busy/ms 58.91, main/busy/ms 46.09, stage/main/update 24.60, pipelines/driver_ms 24.58, encode//sum 17.92, stage/render/prepare_assets 17.83, stage/main/post_update 16.97, stage/render/render 12.77 |
| warm-up | 3.84 | 82.6 | pipelines/compile_ms 1186, pipelines/front_ms 1136, render/thread/ms 64.91, main/busy/ms 64.78, render/busy/ms 59.24, stage/main/update 50.38, pipelines/driver_ms 49.95, stream/drive/ms 45.24, stage/render/prepare_assets 42.21, stage/render/render 14.25, stage/main/post_update 9.81, render/graph/ms 8.40 |
| warm-up | 3.87 | 36.0 | pipelines/compile_ms 1043, pipelines/front_ms 1002, render/thread/ms 72.79, render/busy/ms 69.24, main/busy/ms 52.09, pipelines/driver_ms 41.21, stage/main/update 36.00, stage/render/prepare_assets 35.39, stage/render/render 29.33, render/graph/ms 25.33, stage/render/graph_submit 19.35, stage/main/post_update 10.52 |
| warm-up | 3.91 | 40.1 | pipelines/compile_ms 558, pipelines/front_ms 540, render/thread/ms 59.94, render/busy/ms 56.50, main/busy/ms 49.37, stage/main/update 34.93, stage/render/render 29.12, render/graph/ms 25.28, stage/render/prepare_assets 23.40, pipelines/wait_ms 19.90, pipelines/lock_ms 19.46, stage/render/graph_submit 19.43 |
| warm-up | 4.01 | 72.9 | pipelines/compile_ms 280, pipelines/front_ms 269, main/busy/ms 80.11, render/thread/ms 71.85, render/busy/ms 69.92, stage/main/update 37.59, stream/drive/ms 27.88, stage/render/prepare_assets 25.56, stage/main/post_update 24.62, stage/render/prepare_write_phase_buffers 15.57, pipelines/driver_ms 11.18, bigspace/total/ms 10.92 |
| warm-up | 4.08 | 64.1 | pipelines/compile_ms 700, pipelines/front_ms 673, render/thread/ms 95.30, main/busy/ms 93.20, render/busy/ms 92.93, stage/main/update 45.94, stage/render/prepare_assets 29.97, pipelines/driver_ms 26.83, stage/main/post_update 24.19, stage/render/render 17.56, stage/render/prepare_write_phase_buffers 15.51, stage/render/prepare_batch_phases 14.98 |
| warm-up | 4.21 | 132.7 | pipelines/compile_ms 2013, pipelines/front_ms 1936, render/thread/ms 141, render/busy/ms 138, pipelines/driver_ms 76.55, main/busy/ms 73.32, stage/render/prepare_assets 71.36, stage/main/update 46.32, stage/render/prepare_bind_groups 29.64, stage/render/render 18.53, stage/main/post_update 15.18, render/graph/ms 14.23 |
| warm-up | 4.28 | 71.4 | pipelines/compile_ms 2039, pipelines/front_ms 1937, pipelines/wait_ms 199, render/thread/ms 157, render/busy/ms 156, pipelines/driver_ms 102, main/busy/ms 73.24, stage/render/prepare_assets 71.26, stage/main/post_update 31.36, stage/render/prepare_resources 30.67, stage/render/prepare_bind_groups 28.64, stage/main/update 16.61 |
| warm-up | 4.35 | 67.9 | pipelines/compile_ms 709, pipelines/front_ms 658, pipelines/wait_ms 263, render/thread/ms 116, render/busy/ms 115, main/busy/ms 75.49, pipelines/driver_ms 50.74, encode//sum 43.39, stage/render/prepare_assets 38.81, stage/render/prepare_resources 37.36, stage/main/post_update 35.18, stage/render/render 20.57 |
| warm-up | 4.43 | 84.8 | pipelines/compile_ms 1671, pipelines/front_ms 1624, pipelines/wait_ms 342, render/thread/ms 133, render/busy/ms 132, main/busy/ms 63.92, stage/render/prepare_assets 60.46, pipelines/driver_ms 46.33, encode//sum 46.20, stage/main/post_update 35.50, stage/render/render 23.19, render/graph/ms 20.89 |
| warm-up | 4.50 | 65.0 | pipelines/compile_ms 2880, pipelines/front_ms 2791, pipelines/wait_ms 1188, render/thread/ms 143, render/busy/ms 142, pipelines/driver_ms 89.55, main/busy/ms 84.62, stage/render/prepare_assets 53.03, stage/main/post_update 37.50, stage/main/update 27.73, stage/render/prepare_resources 25.51, stage/render/render 20.88 |
| warm-up | 4.54 | 40.1 | pipelines/compile_ms 1465, pipelines/front_ms 1413, pipelines/wait_ms 925, render/thread/ms 87.54, render/busy/ms 85.86, main/busy/ms 63.10, pipelines/driver_ms 51.71, stage/render/prepare_assets 26.33, stage/main/update 22.63, stage/main/post_update 20.15, stage/render/render 16.96, stage/render/prepare_resources 15.24 |
| warm-up | 4.58 | 37.6 | pipelines/compile_ms 283, pipelines/front_ms 273, main/busy/ms 89.68, stage/main/update 68.70, stream/drive/ms 59.33, render/thread/ms 57.59, render/busy/ms 56.25, stage/render/prepare_assets 23.19, pipelines/wait_ms 11.65, stage/main/post_update 11.65, stage/render/render 11.10, pipelines/driver_ms 9.95 |
| warm-up | 4.65 | 79.0 | pipelines/compile_ms 1322, pipelines/front_ms 1280, pipelines/wait_ms 633, main/busy/ms 111, stage/main/update 91.90, render/thread/ms 63.74, render/busy/ms 61.76, stream/drive/ms 59.44, pipelines/driver_ms 41.92, stage/render/render 17.86, stage/render/prepare_assets 16.36, render/graph/ms 14.89 |
| warm-up | 4.71 | 53.8 | pipelines/compile_ms 2152, pipelines/front_ms 2090, pipelines/wait_ms 956, main/busy/ms 125, stage/main/update 103, render/thread/ms 63.28, pipelines/driver_ms 61.67, render/busy/ms 61.16, stream/drive/ms 52.25, stage/render/prepare_assets 25.86, stage/render/render 19.19, render/graph/ms 16.26 |
| warm-up | 4.84 | 127.8 | pipelines/compile_ms 1186, pipelines/front_ms 1154, pipelines/wait_ms 327, main/busy/ms 104, render/thread/ms 81.08, render/busy/ms 79.50, stage/main/update 79.40, stream/drive/ms 52.73, stage/render/prepare_assets 41.99, pipelines/driver_ms 32.73, stage/render/render 15.67, stage/main/post_update 13.67 |
| warm-up | 4.90 | 59.6 | pipelines/compile_ms 830, pipelines/front_ms 804, main/busy/ms 99.93, render/thread/ms 92.88, render/busy/ms 90.86, stage/main/update 70.30, stream/drive/ms 48.97, stage/render/prepare_assets 42.69, pipelines/driver_ms 26.11, stage/render/render 20.23, render/graph/ms 17.70, stage/main/post_update 16.74 |
| warm-up | 4.98 | 87.7 | pipelines/compile_ms 2280, pipelines/front_ms 2217, main/busy/ms 108, render/thread/ms 78.33, stage/main/update 77.25, render/busy/ms 76.27, pipelines/driver_ms 62.37, stream/drive/ms 49.13, stage/render/prepare_assets 41.79, stage/main/post_update 17.53, stage/render/render 17.09, render/graph/ms 14.52 |
| warm-up | 5.02 | 35.1 | pipelines/compile_ms 1963, pipelines/front_ms 1909, main/busy/ms 73.28, render/thread/ms 62.28, render/busy/ms 60.41, pipelines/driver_ms 54.54, stage/main/update 53.48, stage/render/prepare_assets 33.53, stream/drive/ms 33.27, pipelines/wait_ms 18.50, stage/render/render 13.96, stage/main/post_update 12.10 |
| warm-up | 5.07 | 54.7 | pipelines/compile_ms 445, pipelines/front_ms 431, main/busy/ms 66.43, render/thread/ms 52.12, stage/main/update 50.66, render/busy/ms 50.51, stream/drive/ms 33.10, stage/render/prepare_assets 20.36, pipelines/wait_ms 14.86, pipelines/driver_ms 14.35, stage/render/render 13.51, render/graph/ms 11.41 |
| warm-up | 5.10 | 30.8 | pipelines/compile_ms 276, pipelines/front_ms 268, render/thread/ms 49.81, render/busy/ms 48.43, main/busy/ms 38.91, stage/main/update 20.50, stage/render/prepare_assets 14.99, stage/render/render 12.12, stage/main/post_update 11.69, render/graph/ms 10.29, pipelines/driver_ms 8.35, encode//sum 7.84 |
| warm-up | 5.19 | 33.6 | pipelines/compile_ms 376, pipelines/front_ms 364, render/thread/ms 50.16, render/busy/ms 48.50, main/busy/ms 38.84, stage/main/update 17.06, stage/main/post_update 15.05, stage/render/render 14.67, stage/render/prepare_assets 13.26, pipelines/driver_ms 12.62, render/graph/ms 12.44, stage/render/graph_submit 8.32 |
| warm-up | 5.27 | 45.2 | pipelines/compile_ms 880, pipelines/front_ms 848, render/thread/ms 60.17, render/busy/ms 57.76, main/busy/ms 40.23, pipelines/driver_ms 31.83, stage/render/prepare_assets 23.71, stage/main/update 20.59, stage/render/render 20.40, render/graph/ms 17.93, stage/render/graph_submit 14.20, stage/main/post_update 11.55 |
| warm-up | 5.30 | 39.2 | pipelines/compile_ms 447, pipelines/front_ms 431, render/thread/ms 67.15, render/busy/ms 65.67, main/busy/ms 39.60, stage/render/prepare_assets 38.10, stage/main/update 21.99, stage/render/render 17.16, pipelines/driver_ms 15.18, render/graph/ms 14.95, stage/render/graph_submit 11.96, stage/main/post_update 8.42 |
| warm-up | 5.39 | 32.8 | pipelines/compile_ms 254, pipelines/front_ms 246, render/thread/ms 53.27, render/busy/ms 51.25, main/busy/ms 36.16, stage/render/prepare_assets 19.96, stage/main/post_update 14.50, stage/render/render 14.44, stage/main/update 13.26, render/graph/ms 11.95, stage/render/graph_submit 8.65, encode//sum 8.62 |
| warm-up | 5.44 | 45.0 | pipelines/compile_ms 428, pipelines/front_ms 417, render/thread/ms 61.98, render/busy/ms 60.58, main/busy/ms 37.65, stage/render/prepare_assets 37.20, stage/main/update 15.64, stage/main/post_update 13.90, stage/render/render 11.46, pipelines/driver_ms 11.09, render/graph/ms 9.26, stream/drive/ms 7.99 |
| warm-up | 5.54 | 31.9 | render/thread/ms 51.04, render/busy/ms 49.63, main/busy/ms 37.25, stage/render/prepare_assets 24.28, stage/main/post_update 17.70, stage/render/render 13.64, stage/main/update 13.47, render/graph/ms 11.44, encode//sum 10.08, stage/render/graph_submit 7.68, encode/submit 4.60, stage/render/camera_driver 3.61 |
| warm-up | 5.57 | 31.1 | render/thread/ms 52.82, render/busy/ms 51.53, stage/render/prepare_assets 28.90, main/busy/ms 28.06, stage/main/post_update 12.37, stage/render/render 11.60, render/graph/ms 9.60, stage/main/update 8.76, encode//sum 7.91, stage/render/graph_submit 6.63, encode/submit 3.75, encode/finish_wall 2.86 |
| warm-up | 5.61 | 36.0 | render/thread/ms 56.86, render/busy/ms 55.42, stage/render/prepare_assets 32.21, main/busy/ms 29.90, pipelines/compile_ms 28.73, pipelines/front_ms 26.92, stage/render/render 14.01, stage/main/post_update 13.77, render/graph/ms 11.75, stage/main/update 9.70, stage/render/graph_submit 8.64, encode//sum 7.44 |
| warm-up | 5.67 | 30.1 | pipelines/compile_ms 606, pipelines/front_ms 576, render/thread/ms 51.34, render/busy/ms 49.83, pipelines/driver_ms 30.42, stage/render/prepare_assets 28.75, main/busy/ms 25.37, stage/render/render 13.76, render/graph/ms 11.18, stage/main/post_update 11.05, encode//sum 8.42, stage/main/update 8.17 |
| warm-up | 6.44 | 33.1 | render/thread/ms 43.69, render/busy/ms 42.48, main/busy/ms 40.97, stage/main/update 18.54, stage/main/post_update 16.60, stage/render/render 14.90, render/graph/ms 12.69, encode//sum 10.31, stage/render/prepare_assets 9.67, stage/render/graph_submit 8.86, stream/drive/ms 6.29, encode/submit 4.82 |
| warm-up | 6.53 | 30.8 | main/busy/ms 43.15, render/thread/ms 29.53, render/busy/ms 28.30, stage/main/update 23.63, stage/main/post_update 14.05, stream/drive/ms 13.68, stage/render/render 11.38, encode//sum 9.63, render/graph/ms 9.51, stage/render/graph_submit 6.35, stage/render/prepare_assets 5.51, encode/submit 3.67 |
| warm-up | 6.76 | 30.5 | pipelines/compile_ms 162, pipelines/front_ms 158, render/thread/ms 40.84, render/busy/ms 39.78, main/busy/ms 27.67, stage/render/prepare_assets 20.71, stage/main/post_update 12.55, stage/main/update 11.08, stage/render/render 10.85, render/graph/ms 9.25, encode//sum 7.92, stage/render/graph_submit 6.92 |
| warm-up | 6.97 | 38.6 | main/busy/ms 43.16, render/thread/ms 27.25, render/busy/ms 26.22, stage/main/update 24.63, stream/drive/ms 13.23, stage/main/post_update 11.65, stage/render/prepare_assets 11.11, stage/render/render 9.06, render/graph/ms 7.66, encode//sum 7.51, stage/render/graph_submit 5.56, pipelines/compile_ms 5.44 |
| warm-up | 7.04 | 31.6 | render/thread/ms 42.40, render/busy/ms 41.29, main/busy/ms 21.19, stage/main/post_update 11.66, stage/render/render 11.26, stage/render/specialize 10.28, render/graph/ms 9.78, stage/render/graph_submit 7.61, encode//sum 7.55, stage/render/queue 6.95, stage/main/update 5.99, stage/render/prepare_assets 5.08 |
| warm-up | 8.34 | 31.5 | render/thread/ms 37.88, render/busy/ms 36.78, main/busy/ms 18.91, stage/render/render 11.21, stage/main/post_update 10.57, render/graph/ms 9.45, stage/render/specialize 9.42, stage/render/graph_submit 7.22, encode//sum 6.89, stage/render/queue 6.67, stage/main/update 5.82, stage/render/prepare_meshes 3.99 |
| measure | 0.43 | 31.3 | render/thread/ms 42.68, render/busy/ms 41.41, main/busy/ms 19.59, encode//sum 19.29, stage/render/render 15.76, render/graph/ms 13.96, stage/main/post_update 12.52, stage/render/graph_submit 11.41, stage/render/specialize 9.62, stage/render/queue 6.93, encode/submit 6.36, encode/finish_wall 5.10 |
| measure | 1.77 | 31.4 | render/thread/ms 37.69, render/busy/ms 36.62, main/busy/ms 18.75, stage/render/render 11.64, stage/main/post_update 10.89, render/graph/ms 10.19, encode//sum 9.14, stage/render/specialize 9.13, stage/render/graph_submit 7.34, stage/render/queue 6.23, stage/main/update 4.69, stage/render/prepare_meshes 4.51 |
| measure | 3.23 | 30.7 | render/thread/ms 40.57, render/busy/ms 39.58, main/busy/ms 19.27, stage/render/render 13.63, stage/main/post_update 12.74, render/graph/ms 12.10, encode//sum 9.78, stage/render/specialize 9.46, stage/render/graph_submit 8.79, stage/render/queue 6.64, encode/pass:shadow_point_light/sum 6.02, encode/submit 5.66 |
| measure | 6.00 | 33.4 | render/thread/ms 37.16, render/busy/ms 36.05, main/busy/ms 20.40, stage/main/post_update 13.54, stage/render/render 10.43, stage/render/specialize 9.87, render/graph/ms 8.84, encode//sum 7.89, stage/render/queue 6.92, stage/render/graph_submit 6.83, stage/main/update 4.26, encode/submit 3.71 |
| measure | 10.02 | 31.4 | render/thread/ms 35.45, render/busy/ms 34.38, main/busy/ms 18.57, stage/main/post_update 11.10, stage/render/render 10.65, stage/render/specialize 9.24, render/graph/ms 9.10, encode//sum 7.81, stage/render/graph_submit 6.97, stage/render/queue 6.37, stage/main/update 4.64, encode/submit 3.95 |
| measure | 11.33 | 31.6 | render/thread/ms 36.41, render/busy/ms 35.44, main/busy/ms 18.16, stage/main/post_update 11.55, stage/render/render 10.38, stage/render/specialize 9.58, render/graph/ms 8.91, encode//sum 7.97, stage/render/queue 7.25, stage/render/graph_submit 6.81, stage/main/update 4.10, encode/submit 3.93 |
| measure | 15.73 | 31.3 | render/thread/ms 39.09, render/busy/ms 38.01, main/busy/ms 18.01, stage/render/render 12.06, stage/main/post_update 11.17, render/graph/ms 10.54, stage/render/specialize 9.87, encode//sum 9.03, stage/render/graph_submit 8.13, stage/render/queue 7.69, encode/submit 5.03, stage/main/update 4.27 |
| measure | 17.12 | 33.4 | render/thread/ms 39.79, render/busy/ms 38.81, main/busy/ms 20.02, stage/main/post_update 11.87, stage/render/render 11.42, render/graph/ms 10.08, stage/render/specialize 9.92, encode//sum 9.39, stage/render/graph_submit 7.50, stage/render/queue 7.38, encode/submit 4.98, stage/main/update 4.62 |
| measure | 23.05 | 35.7 | render/thread/ms 41.23, render/busy/ms 40.25, main/busy/ms 24.27, stage/main/post_update 12.51, stage/render/specialize 11.82, stage/render/render 11.50, render/graph/ms 10.09, encode//sum 9.43, stage/render/queue 8.09, stage/render/graph_submit 7.78, stage/main/update 6.72, encode/submit 4.29 |
| measure | 24.56 | 32.7 | render/thread/ms 38.59, render/busy/ms 37.59, main/busy/ms 19.81, stage/main/post_update 12.11, stage/render/render 11.36, stage/render/specialize 10.07, render/graph/ms 9.97, encode//sum 7.91, stage/render/graph_submit 7.57, stage/render/queue 7.24, stage/main/update 4.96, encode/submit 4.44 |
| measure | 27.30 | 37.1 | render/thread/ms 46.26, render/busy/ms 45.17, main/busy/ms 25.76, stage/render/render 16.29, render/graph/ms 14.65, stage/main/post_update 13.73, stage/render/specialize 11.72, stage/render/graph_submit 11.41, encode//sum 11.33, stage/render/queue 7.83, encode/pass:shadow_point_light/sum 7.79, encode/submit 7.09 |
| measure | 28.79 | 32.1 | render/thread/ms 41.58, render/busy/ms 40.64, main/busy/ms 22.01, stage/main/post_update 14.24, stage/render/render 13.12, render/graph/ms 11.56, stage/render/specialize 10.75, encode//sum 9.90, stage/render/graph_submit 8.53, stage/render/queue 7.41, encode/submit 5.41, stage/main/update 4.77 |
| measure | 30.48 | 34.7 | render/thread/ms 40.82, render/busy/ms 39.63, main/busy/ms 22.36, stage/main/post_update 13.37, stage/render/render 12.64, render/graph/ms 10.89, stage/render/specialize 10.37, stage/render/graph_submit 8.64, encode//sum 8.50, stage/render/queue 7.27, stage/main/update 5.93, encode/submit 4.59 |
| measure | 31.95 | 31.8 | render/thread/ms 39.99, render/busy/ms 38.90, main/busy/ms 19.89, stage/main/post_update 12.71, stage/render/render 12.07, render/graph/ms 10.45, stage/render/specialize 10.41, encode//sum 8.63, stage/render/graph_submit 8.06, stage/render/queue 7.09, encode/submit 4.80, stage/main/update 4.57 |
| measure | 35.35 | 35.8 | render/thread/ms 39.78, render/busy/ms 38.77, main/busy/ms 21.98, stage/main/post_update 14.27, stage/render/specialize 11.18, stage/render/render 10.64, render/graph/ms 9.20, encode//sum 9.05, stage/render/queue 7.18, stage/render/graph_submit 7.16, stage/main/update 4.57, encode/submit 3.82 |
| measure | 40.80 | 33.3 | render/thread/ms 37.48, render/busy/ms 36.51, main/busy/ms 19.50, stage/main/post_update 11.72, stage/render/specialize 10.24, stage/render/render 9.42, render/graph/ms 8.10, encode//sum 7.50, stage/render/queue 6.45, stage/render/graph_submit 6.13, stage/main/update 4.68, encode/submit 3.68 |
| measure | 43.22 | 33.7 | render/thread/ms 37.14, render/busy/ms 36.12, main/busy/ms 17.00, stage/render/render 10.47, stage/render/specialize 10.42, stage/main/post_update 10.06, render/graph/ms 9.07, encode//sum 8.55, stage/render/graph_submit 7.22, stage/render/queue 6.71, stage/main/update 4.31, encode/submit 3.82 |
| measure | 45.67 | 35.4 | render/thread/ms 41.20, render/busy/ms 40.10, main/busy/ms 18.56, stage/render/render 11.42, stage/render/specialize 10.90, stage/main/post_update 10.68, render/graph/ms 9.71, encode//sum 7.83, stage/render/graph_submit 7.64, stage/render/queue 6.87, stage/render/prepare_meshes 5.07, stage/main/update 4.66 |
| measure | 59.02 | 44.2 | render/thread/ms 55.13, render/busy/ms 53.60, main/busy/ms 34.06, stage/main/post_update 20.76, stage/render/render 17.80, render/graph/ms 15.53, stage/render/graph_submit 12.56, stage/render/specialize 12.46, encode//sum 12.24, stage/main/update 9.04, stage/render/prepare_meshes 7.43, encode/submit 7.05 |
| measure | 59.06 | 33.6 | render/thread/ms 57.75, render/busy/ms 56.66, main/busy/ms 27.37, stage/main/post_update 14.45, stage/render/prepare_assets 14.00, stage/render/specialize 12.37, stage/render/render 10.80, render/graph/ms 9.48, stage/render/prepare_meshes 8.21, encode//sum 8.02, stage/main/update 7.50, stage/render/graph_submit 7.33 |
| measure | 62.58 | 30.8 | render/thread/ms 39.04, render/busy/ms 38.08, main/busy/ms 20.45, stage/render/render 12.03, render/graph/ms 10.62, stage/main/post_update 10.35, stage/render/specialize 10.24, encode//sum 8.99, stage/render/graph_submit 8.28, stage/render/queue 6.65, stage/main/update 6.06, encode/submit 4.24 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 1%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 52%) · 2620 MHz (min 2610) · memory 8101 MHz · 80.0 W · 78 °C (max 83) · VRAM 7019 MiB · throttled 100% of samples

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
