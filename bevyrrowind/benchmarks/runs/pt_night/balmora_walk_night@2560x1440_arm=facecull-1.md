# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.face_cull=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6263 frames over 65.6 s · **median 10.20 ms (98 fps)** · mean 10.48 · p95 12.96 · p99 14.81 (1% low 68 fps) · worst 31.52 ms · 45 spikes over 1.5× median · 21.9 stutters/min (work 20.1/min)
- warm-up: 323 frames over 9.4 s, worst 2389.2 ms
- hitches (> 25.5 ms): **2** measured, 52 in warm-up
- timed GPU passes: 6.49 ms of a 10.48 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.20 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.93/min, limit 1/min
- pass worst frame 31.52 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.970 | 2.030 | 2.063 | 0.97 | 0.171 |
| main_opaque_pass_3d | 1.263 | 1.301 | 1.659 | 0.97 | 0.059 |
| early prepass | 0.475 | 0.489 | 0.719 | 0.97 | 0.086 |
| bloom | 0.306 | 0.316 | 0.322 | 0.97 | 0.062 |
| atmosphere_luts | 0.301 | 0.310 | 0.372 | 0.97 | 0.003 |
| water_velocity | 0.255 | 0.263 | 0.349 | 0.97 | 0.002 |
| water_sspr | 0.196 | 0.201 | 0.203 | 0.97 | 0.002 |
| auxiliary_views | 0.194 | 0.199 | 0.821 | 0.97 | 0.180 |
| ssao | 0.183 | 0.188 | 0.193 | 0.97 | 0.007 |
| water_fft | 0.126 | 0.130 | 0.138 | 0.97 | 0.003 |
| upscaling | 0.126 | 0.129 | 0.136 | 0.97 | 0.004 |
| atmos_froxel_inject | 0.124 | 0.127 | 0.129 | 0.97 | 0.001 |
| uniform_allocation | 0.122 | 0.126 | 0.155 | 0.97 | 0.024 |
| clouds_composite | 0.102 | 0.105 | 0.108 | 0.97 | 0.002 |
| bin_unpacking | 0.095 | 0.097 | 0.119 | 0.97 | 0.017 |
| tonemapping | 0.084 | 0.087 | 0.091 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.079 | 0.156 | 0.97 | 0.008 |
| clustering | 0.062 | 0.064 | 0.067 | 0.97 | 0.020 |
| clouds_trace | 0.054 | 0.056 | 0.074 | 0.97 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.412 | 0.416 | 0.11 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.66 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.97 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.029 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.023 | 0.218 | 0.227 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.040 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.97 | 0.124 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.97 | 0.004 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.069 | 11.651 | 23.13 | 56801.7 |
| main/busy/ms | 8.896 | 10.649 | 29.35 | 55717.2 |
| render/busy/ms | 8.558 | 11.151 | 22.55 | 53595.6 |
| stage/main/post_update | 5.535 | 7.008 | 9.27 | 34664.6 |
| stage/render/render | 5.476 | 7.462 | 15.18 | 34299.3 |
| render/graph/ms | 4.782 | 6.749 | 14.33 | 29952.6 |
| encode//sum | 4.094 | 5.669 | 18.00 | 25643.6 |
| stage/render/graph_submit | 3.587 | 4.990 | 9.66 | 22462.6 |
| encode/submit | 2.302 | 3.408 | 5.76 | 14416.7 |
| stage/main/update | 2.128 | 2.559 | 22.62 | 13328.5 |
| encode/finish_wall | 1.324 | 1.750 | 3.89 | 8289.3 |
| stage/render/camera_driver | 1.144 | 1.736 | 4.59 | 7165.4 |
| encode/pass:shadow_point_light/sum | 1.067 | 4.240 | 15.90 | 6684.8 |
| encode/pass:shadow_directional_light/sum | 0.766 | 1.242 | 1.95 | 4797.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.752 | 1.078 | 1.96 | 4712.9 |
| encode/pass:early prepass/sum | 0.743 | 1.074 | 1.78 | 4650.9 |
| stage/render/prepare_write_phase_buffers | 0.709 | 1.013 | 1.94 | 4438.8 |
| bigspace/total/ms | 0.573 | 1.397 | 2.25 | 3591.5 |
| stage/render/prepare_assets | 0.563 | 0.710 | 9.14 | 3526.1 |
| stage/render/prepare_resources | 0.441 | 0.522 | 0.88 | 2763.7 |
| render/present/ms | 0.419 | 0.528 | 2.04 | 2621.8 |
| stage/main/first | 0.396 | 0.517 | 6.46 | 2477.6 |
| stage/render/prepare_meshes | 0.383 | 0.481 | 2.38 | 2396.0 |
| stage/main/pre_update | 0.348 | 0.491 | 1.66 | 2176.9 |
| terrain/update/ms | 0.314 | 0.412 | 2.09 | 1966.6 |
| terrain/select/ms | 0.281 | 0.370 | 1.03 | 1758.1 |
| controller/feed/ms | 0.261 | 0.329 | 1.17 | 1636.9 |
| stream/drive/ms | 0.248 | 0.330 | 20.41 | 1555.2 |
| stage/render/prepare_bind_groups | 0.246 | 0.404 | 2.22 | 1542.4 |
| stage/render/specialize | 0.242 | 0.371 | 2.28 | 1513.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.237 | 0.350 | 0.82 | 1483.7 |
| stage/render/queue | 0.228 | 0.360 | 2.52 | 1425.2 |
| bigspace/low_precision/ms | 0.226 | 0.734 | 1.74 | 1416.4 |
| stage/main/fixed_main_loop | 0.204 | 0.358 | 1.13 | 1277.3 |
| far/select/ms | 0.198 | 0.262 | 0.65 | 1241.3 |
| stream/tick/ms | 0.193 | 0.263 | 0.60 | 1210.3 |
| stage/render/create_views | 0.188 | 0.240 | 0.90 | 1178.0 |
| stage/render/prepare_batch_phases | 0.184 | 0.265 | 0.64 | 1155.3 |
| bigspace/high_precision/ms | 0.181 | 0.835 | 1.75 | 1136.4 |
| anim/evaluate/ms | 0.177 | 0.372 | 1.72 | 1109.5 |
| stage/render/extract_commands | 0.166 | 0.193 | 0.45 | 1042.7 |
| stage/render/prepare_flush | 0.162 | 0.337 | 1.03 | 1013.2 |
| anim/evaluate_cpu/ms | 0.153 | 0.188 | 0.71 | 959.1 |
| controller/move/ms | 0.142 | 0.327 | 0.63 | 891.7 |
| stage/render/prepare_views | 0.136 | 0.170 | 0.62 | 853.7 |
| stage/main/last | 0.112 | 0.151 | 1.61 | 703.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.169 | 0.64 | 625.2 |
| render/acquire/ms | 0.093 | 0.121 | 0.33 | 584.2 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.130 | 0.51 | 388.0 |
| physics/step/ms | 0.043 | 0.080 | 0.53 | 272.3 |
| stage/render/phase_sort | 0.035 | 0.061 | 0.26 | 219.0 |
| physics/insert/ms | 0.033 | 0.350 | 1.20 | 205.0 |
| stage/render/cleanup | 0.031 | 0.095 | 0.86 | 194.0 |
| encode/resolve_diagnostics/sum | 0.029 | 0.039 | 0.56 | 183.9 |
| weather/map/ms | 0.016 | 0.020 | 0.11 | 97.9 |
| pipelines/compile_ms | 0.015 | 0.000 | 84.89 | 96.6 |
| pipelines/front_ms | 0.015 | 0.000 | 81.73 | 93.3 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.69 | 89.9 |
| anim/apply/ms | 0.011 | 0.019 | 0.46 | 71.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.45 | 67.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.07 | 65.4 |
| weather/eval/ms | 0.010 | 0.013 | 0.20 | 60.6 |
| script/update/ms | 0.007 | 0.009 | 0.13 | 42.5 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 31.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.04 | 30.6 |
| stream/spawn/ms | 0.004 | 0.000 | 1.07 | 28.1 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 26.9 |
| controller/write/ms | 0.004 | 0.004 | 0.03 | 22.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 13.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 6.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.22 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.42 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.12 | 3.5 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.15 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.08 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 603 |
| crowd/c0/meshes/count | count | 353 | 646 | 161 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5593 |
| crowd/c1/meshes/count | count | 1159 | 1281 | 1180 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 205 |
| crowd/joints_extracted/count | count | 3664 | 8263 | 5775 |
| crowd/joints_uploaded/count | count | 3633 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.41 | 3.06 | 0.46 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.69 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.45 | 0.01 |
| encode/finish_critical | ms | 0.95 | 3.06 | 0.87 |
| encode/pass:early prepass/max | ms | 0.77 | 1.78 | 0.72 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.96 | 0.87 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.64 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.82 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 1.95 | 0.70 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.38 | 0.68 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.56 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.98 | 29.35 | 8.33 |
| gpu/timed/ms | ms | 7.41 | 10.97 | 7.74 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 803 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 803 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1285 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3383 | 5143 | 3193 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1285 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3383 | 5143 | 3193 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 175 | 4 |
| phases/shadow/bins | count | 3710 | 4389 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6080 | 7301 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 197 | 238 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 270 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 10 | 511 | 9 |
| preprocess/bind_groups_created | count | 10 | 511 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 249 | 124 |
| process/cpu_usage | % | 26.86 | 38.46 | 23.44 |
| process/mem_usage | GiB | 5.41 | 5.58 | 5.58 |
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
| skin/blocks_changed/count | count | 12 | 104 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 279.8 k | 807.6 k | 443.6 k |
| skin/joints_written/count | count | 4372 | 12.6 k | 6932 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5009 | 5009 |
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
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.11 | 45.20 | 30.72 |
| system/mem_usage | % | 54.01 | 54.54 | 54.53 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 64 | 226 | 56 |
| views/all/bins | count | 6218 | 7232 | 5002 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 20.1 k | 10.7 k |
| views/all/pipelines | count | 246 | 287 | 67 |
| views/all/pipelines_distinct | count | 71 | 75 | 63 |
| views/all/work_items | count | 10.3 k | 17.7 k | 8152 |
| views/cascade/batch_sets | count | 9 | 27 | 4 |
| views/cascade/bins | count | 1442 | 1560 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3393 | 4424 | 3038 |
| views/cascade/pipelines | count | 23 | 27 | 19 |
| views/cascade/work_items | count | 328 | 4424 | 222 |
| views/lamp_face/batch_sets | count | 0 | 167 | 0 |
| views/lamp_face/bins | count | 2441 | 2946 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 24 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2711 | 3204 | 0 |
| views/lamp_face/pipelines | count | 176 | 211 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 2632 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2866 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8378 | 13.0 k | 7622 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8545 | 13.2 k | 7930 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 46.55 | 242 | 291.6 k |
| anim/events/count | count | 0.08 | 2 | 483 |
| assets/Image/added | count | 0.09 | 30 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.65 | 152 | 4061 |
| assets/Mesh/removed | count | 0.77 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.46 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.75 | 1354 | 4675 |
| encode//count | count | 35.04 | 203 | 219.4 k |
| encode/bind_walk/init_textures | count | 13.3 k | 104.1 k | 83.08 M |
| encode/bind_walk/init_textures_skipped | count | 56.5 k | 135.9 k | 353.89 M |
| encode/bind_walk/inits | count | 457 | 1045 | 2.86 M |
| encode/bind_walk/inits_skipped | count | 102 | 160 | 638.0 k |
| encode/bind_walk/merge_textures | count | 13.2 k | 104.0 k | 82.68 M |
| encode/bind_walk/merge_textures_skipped | count | 5379 | 6012 | 33.69 M |
| encode/bind_walk/merges | count | 100 | 314 | 627.1 k |
| encode/bind_walk/merges_skipped | count | 81.26 | 125 | 508.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6263 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6263 |
| encode/pass:early prepass/count | count | 1 | 1 | 6263 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6263 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6263 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6263 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6263 |
| encode/pass:shadow_point_light/count | count | 2.03 | 44 | 12.7 k |
| encode/queue/command_buffers | count | 47.08 | 257 | 294.9 k |
| encode/queue/submits | count | 2.02 | 4 | 12.6 k |
| encode/queue/write_buffer/bytes | bytes | 1.80 M | 2.68 M | 11.26 G |
| encode/queue/write_buffer/calls | count | 255 | 764 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 166.3 k | 3.53 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 10.93 | 465 | 68.5 k |
| encode/queue/write_texture/bytes | bytes | 10.1 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6263 |
| physics/inserts | count | 0.36 | 111 | 2257 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 145 | 2335 |
| physics/steps | count | 0.63 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 18 | 378 |
| stream/upload/bytes | bytes | 5405 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1185 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.5 ms; slowest 54, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.39 | 2389.2 | main/busy/ms 2361, stage/main/post_update 2319, stage/main/update 33.12, stage/main/pre_update 5.17, stage/main/last 1.01, surface/advance_cpu_ms 0.73, stage/main/first 0.47, stream/drive/ms 0.43, stage/main/fixed_main_loop 0.41, surface/cell_upload_ms 0.30, stream/tick/ms 0.12, bigspace/total/ms 0.09 |
| warm-up | 3.12 | 733.7 | main/busy/ms 2489, stage/main/post_update 2341, render/thread/ms 695, render/busy/ms 524, stage/render/prepare_assets 475, stage/render/prepare_views 168, render/acquire/ms 167, stage/main/update 134, stage/render/create_views 28.09, stage/render/render 13.59, render/graph/ms 8.81, stage/render/camera_driver 7.00 |
| warm-up | 3.30 | 174.2 | render/thread/ms 852, render/busy/ms 588, stage/render/prepare_assets 524, stage/render/prepare_views 168, render/acquire/ms 168, main/busy/ms 155, pipelines/compile_ms 118, stage/render/render 111, stage/main/update 106, render/present/ms 96.15, pipelines/driver_ms 78.88, pipelines/front_ms 39.57 |
| warm-up | 3.53 | 216.1 | pipelines/wait_ms 4403, pipelines/compile_ms 2045, pipelines/front_ms 1767, pipelines/driver_ms 278, render/thread/ms 159, render/busy/ms 156, stage/render/prepare_assets 128, main/busy/ms 103, stage/main/update 53.60, stage/main/post_update 40.70, pipelines/lock_ms 22.76, stage/render/render 20.06 |
| warm-up | 3.64 | 106.2 | pipelines/wait_ms 4603, pipelines/compile_ms 2498, pipelines/front_ms 2298, render/thread/ms 243, render/busy/ms 240, pipelines/driver_ms 200, encode//sum 198, stage/render/prepare_assets 181, main/busy/ms 115, stage/main/pre_update 56.26, stage/render/render 50.83, render/graph/ms 48.22 |
| warm-up | 3.72 | 84.1 | pipelines/compile_ms 1674, pipelines/front_ms 1569, pipelines/wait_ms 403, encode//sum 197, render/thread/ms 181, render/busy/ms 178, main/busy/ms 129, stage/render/prepare_assets 124, pipelines/driver_ms 105, stage/main/pre_update 55.66, stage/render/render 47.18, render/graph/ms 44.42 |
| warm-up | 3.77 | 26.2 | pipelines/compile_ms 531, pipelines/front_ms 512, main/busy/ms 40.84, render/thread/ms 40.09, render/busy/ms 32.97, stage/main/post_update 27.37, pipelines/driver_ms 19.23, stage/render/render 17.18, encode//sum 14.76, render/graph/ms 13.92, terrain/update/ms 11.03, pipelines/wait_ms 10.62 |
| warm-up | 3.80 | 29.6 | pipelines/compile_ms 1056, pipelines/front_ms 1026, render/thread/ms 51.99, pipelines/wait_ms 40.91, main/busy/ms 34.87, pipelines/driver_ms 30.39, render/busy/ms 27.64, stage/main/post_update 25.72, stage/render/prepare_views 23.03, render/acquire/ms 22.86, encode//sum 16.14, stage/render/render 14.75 |
| warm-up | 3.83 | 34.8 | pipelines/compile_ms 1460, pipelines/front_ms 1418, render/thread/ms 62, pipelines/driver_ms 41.87, pipelines/wait_ms 38.78, render/busy/ms 35.78, main/busy/ms 26.77, stage/render/prepare_views 25.02, render/acquire/ms 24.86, stage/render/prepare_assets 18.92, stage/main/post_update 16.31, stage/render/render 10.52 |
| warm-up | 3.93 | 71.2 | pipelines/compile_ms 801, pipelines/front_ms 770, main/busy/ms 59.49, render/thread/ms 54.95, render/busy/ms 53.51, stage/main/update 44.27, stage/render/prepare_assets 40.05, stream/drive/ms 39.58, pipelines/driver_ms 30.93, stage/main/post_update 10.35, stage/render/render 8.35, render/graph/ms 6.76 |
| warm-up | 3.98 | 28.6 | pipelines/compile_ms 131, pipelines/front_ms 120, render/thread/ms 40.68, render/busy/ms 39.19, main/busy/ms 37.88, stage/render/prepare_assets 19.05, stage/main/update 17.76, stage/render/render 12.35, stream/drive/ms 11.37, pipelines/driver_ms 10.53, render/graph/ms 10.33, stage/render/graph_submit 7.62 |
| warm-up | 4.01 | 29.6 | pipelines/compile_ms 120, pipelines/front_ms 114, main/busy/ms 49.70, render/thread/ms 49.46, render/busy/ms 48.04, stage/main/update 31.23, stream/drive/ms 26.93, stage/render/prepare_assets 22.57, stage/render/render 10.50, render/graph/ms 8.45, stage/render/prepare_bind_groups 7.75, stage/main/first 7.26 |
| warm-up | 4.07 | 60.4 | pipelines/compile_ms 279, pipelines/front_ms 269, render/thread/ms 70.23, render/busy/ms 68.82, main/busy/ms 58.30, stage/render/prepare_assets 42.15, stage/main/update 33.35, stream/drive/ms 28.63, stage/main/post_update 15.29, stage/render/render 11.26, pipelines/driver_ms 9.85, render/graph/ms 9.30 |
| warm-up | 4.11 | 45.1 | pipelines/compile_ms 272, pipelines/front_ms 263, render/thread/ms 86.16, render/busy/ms 84.82, main/busy/ms 66.08, stage/render/prepare_assets 37.17, stage/main/update 31.71, stage/render/prepare_batch_phases 21.65, stage/main/post_update 19.61, stage/render/render 12.90, render/graph/ms 10.98, pipelines/driver_ms 9.48 |
| warm-up | 4.19 | 74.6 | pipelines/compile_ms 1558, pipelines/front_ms 1505, render/thread/ms 91.64, render/busy/ms 90.01, main/busy/ms 89.57, stage/main/update 62.95, pipelines/driver_ms 52.88, stage/render/prepare_assets 42.40, stage/render/prepare_batch_phases 19.63, stage/main/post_update 14.82, stage/render/render 12.32, render/graph/ms 10.23 |
| warm-up | 4.25 | 63.6 | pipelines/compile_ms 1859, pipelines/front_ms 1795, render/thread/ms 97.28, render/busy/ms 95.14, main/busy/ms 75.24, stage/render/prepare_assets 64.97, pipelines/driver_ms 63.87, stage/main/update 47.87, stage/main/post_update 18.83, stage/render/render 15.05, render/graph/ms 12.38, pipelines/wait_ms 9.14 |
| warm-up | 4.30 | 51.7 | pipelines/compile_ms 647, pipelines/front_ms 624, render/thread/ms 94.16, render/busy/ms 92.18, main/busy/ms 61.48, pipelines/wait_ms 45.19, stage/render/prepare_assets 37.04, stage/main/update 23.61, pipelines/driver_ms 22.93, stage/main/post_update 21.86, stage/render/render 16.47, stage/render/prepare_write_phase_buffers 15.43 |
| warm-up | 4.42 | 114.3 | pipelines/compile_ms 1418, pipelines/front_ms 1379, render/thread/ms 149, render/busy/ms 147, main/busy/ms 124, pipelines/wait_ms 97.71, stage/render/prepare_assets 61.19, stage/main/post_update 60.36, stage/main/update 49.72, pipelines/driver_ms 38.66, stage/render/render 17.52, encode//sum 16.45 |
| warm-up | 4.53 | 111.9 | pipelines/compile_ms 3177, pipelines/front_ms 3085, pipelines/wait_ms 1785, render/thread/ms 193, render/busy/ms 191, main/busy/ms 145, stage/main/post_update 97.15, pipelines/driver_ms 91.79, stage/render/prepare_assets 76.66, stage/main/update 42.28, encode//sum 29.37, stage/render/render 23.90 |
| warm-up | 4.61 | 76.2 | pipelines/compile_ms 2530, pipelines/front_ms 2429, pipelines/wait_ms 1923, render/thread/ms 149, render/busy/ms 146, pipelines/driver_ms 101, encode//sum 98.00, main/busy/ms 80.40, stage/main/post_update 61.97, stage/render/render 39.38, render/graph/ms 36.77, stage/render/prepare_assets 28.71 |
| warm-up | 4.63 | 28.0 | pipelines/compile_ms 1527, pipelines/front_ms 1443, pipelines/wait_ms 637, main/busy/ms 119, stage/main/update 90.16, pipelines/driver_ms 84.49, encode//sum 83.53, render/thread/ms 81.83, render/busy/ms 80.01, stream/drive/ms 55.29, stage/render/render 32.88, render/graph/ms 30.46 |
| warm-up | 4.76 | 127.9 | pipelines/compile_ms 1824, pipelines/front_ms 1760, pipelines/wait_ms 860, main/busy/ms 120, stage/main/update 94.61, render/thread/ms 66.88, render/busy/ms 64.93, pipelines/driver_ms 64.34, stream/drive/ms 55.91, stage/render/prepare_assets 35.09, stage/render/render 17.80, render/graph/ms 15.17 |
| warm-up | 4.82 | 54.2 | pipelines/compile_ms 1719, pipelines/front_ms 1674, pipelines/wait_ms 422, main/busy/ms 110, render/thread/ms 89.50, render/busy/ms 87.28, stage/main/update 75.00, stream/drive/ms 62.29, pipelines/driver_ms 45.45, stage/render/prepare_assets 43.11, stage/render/render 20.63, render/graph/ms 17.79 |
| warm-up | 4.91 | 96.8 | pipelines/compile_ms 1234, pipelines/front_ms 1195, main/busy/ms 110, render/thread/ms 86.47, render/busy/ms 84.77, stage/main/update 78.11, stream/drive/ms 63.91, stage/render/prepare_assets 39.64, pipelines/driver_ms 38.58, pipelines/wait_ms 33.81, stage/render/render 19.20, render/graph/ms 16.80 |
| warm-up | 4.98 | 67.8 | pipelines/compile_ms 1441, pipelines/front_ms 1392, main/busy/ms 94.50, render/thread/ms 83.65, render/busy/ms 82.10, stage/main/update 67.58, stage/render/prepare_assets 50.39, stream/drive/ms 50.11, pipelines/driver_ms 48.94, pipelines/wait_ms 33.83, stage/main/post_update 15.88, stage/render/render 15.69 |
| warm-up | 5.05 | 69.1 | pipelines/compile_ms 1808, pipelines/front_ms 1755, main/busy/ms 86.15, render/thread/ms 72.50, render/busy/ms 71.03, stage/main/update 63.73, pipelines/driver_ms 52.78, stream/drive/ms 48.44, stage/render/prepare_assets 45.33, stage/main/post_update 14.51, stage/render/render 13.10, render/graph/ms 11.04 |
| warm-up | 5.08 | 25.6 | pipelines/compile_ms 703, pipelines/front_ms 685, main/busy/ms 69.82, stage/main/update 53.69, render/thread/ms 50.19, render/busy/ms 48.23, stream/drive/ms 33.36, stage/render/prepare_assets 24.27, pipelines/driver_ms 17.66, stage/render/render 14.40, render/graph/ms 11.80, encode//sum 10.22 |
| warm-up | 5.15 | 76.5 | pipelines/compile_ms 563, pipelines/front_ms 543, main/busy/ms 73.95, stage/main/update 57.98, render/thread/ms 53.28, render/busy/ms 51.28, stream/drive/ms 33.58, stage/render/prepare_assets 23.47, pipelines/driver_ms 19.30, stage/render/render 15.53, render/graph/ms 12.87, encode//sum 9.60 |
| warm-up | 5.19 | 36.7 | pipelines/compile_ms 626, pipelines/front_ms 604, render/thread/ms 63.58, render/busy/ms 61.98, main/busy/ms 47.47, stage/render/prepare_assets 27.51, stage/main/update 24.20, pipelines/driver_ms 21.53, stage/main/post_update 15.77, stage/render/render 13.12, render/graph/ms 11.15, encode//sum 9.27 |
| warm-up | 5.21 | 26.5 | pipelines/compile_ms 112, pipelines/front_ms 107, render/thread/ms 52.52, render/busy/ms 51.01, main/busy/ms 42.07, stage/main/post_update 17.91, stage/main/update 17.46, stage/render/prepare_assets 16.51, stage/render/render 12.67, encode//sum 11.14, render/graph/ms 10.65, stage/render/graph_submit 6.47 |
| warm-up | 5.27 | 29.9 | pipelines/compile_ms 567, pipelines/front_ms 552, render/thread/ms 50.00, render/busy/ms 48.45, main/busy/ms 30.69, stage/render/render 17.76, render/graph/ms 15.52, pipelines/driver_ms 15.12, stage/main/update 13.35, stage/render/graph_submit 11.29, stage/main/post_update 10.99, encode//sum 10.85 |
| warm-up | 5.30 | 30.8 | pipelines/compile_ms 299, pipelines/front_ms 290, render/thread/ms 55.84, render/busy/ms 53.96, main/busy/ms 39.09, stage/render/render 23.65, render/graph/ms 20.97, stage/main/post_update 15.33, stage/render/graph_submit 14.27, stage/main/update 12.98, encode//sum 11.75, stage/render/prepare_bind_groups 10.91 |
| warm-up | 5.35 | 50.3 | pipelines/compile_ms 887, pipelines/front_ms 865, render/thread/ms 68.18, render/busy/ms 66.37, main/busy/ms 41.23, stage/render/prepare_assets 26.10, stage/render/render 23.42, pipelines/driver_ms 22.03, render/graph/ms 20.92, stage/main/post_update 15.88, stage/render/graph_submit 14.65, stage/main/update 14.50 |
| warm-up | 5.40 | 44.6 | pipelines/compile_ms 1076, pipelines/front_ms 1046, render/thread/ms 75.54, render/busy/ms 73.93, main/busy/ms 46.29, stage/render/prepare_assets 43.74, pipelines/driver_ms 30.24, stage/main/update 25.98, stage/render/render 17.69, stream/drive/ms 16.18, render/graph/ms 15.20, stage/main/post_update 13.16 |
| warm-up | 5.43 | 32.0 | pipelines/compile_ms 357, pipelines/front_ms 341, render/thread/ms 64.88, render/busy/ms 63.44, main/busy/ms 44.64, stage/render/prepare_assets 35.24, stage/main/update 22.69, pipelines/driver_ms 16.23, stage/render/render 14.46, stream/drive/ms 14.42, stage/main/post_update 12.63, render/graph/ms 12.12 |
| warm-up | 5.48 | 28.4 | pipelines/compile_ms 587, pipelines/front_ms 572, render/thread/ms 45.09, render/busy/ms 43.74, main/busy/ms 23.79, stage/render/prepare_assets 21.13, pipelines/driver_ms 14.94, stage/render/render 12.81, render/graph/ms 10.84, stage/main/post_update 10.25, encode//sum 9.89, stage/main/update 8.34 |
| warm-up | 5.53 | 26.8 | pipelines/compile_ms 185, pipelines/front_ms 180, render/thread/ms 42.94, render/busy/ms 41.63, main/busy/ms 25.07, stage/render/prepare_assets 19.42, stage/render/render 13.26, stage/main/post_update 11.41, render/graph/ms 11.31, encode//sum 9.36, stage/main/update 8.28, stage/render/graph_submit 7.01 |
| warm-up | 5.58 | 30.1 | pipelines/compile_ms 564, pipelines/front_ms 551, render/thread/ms 43.63, render/busy/ms 42.25, main/busy/ms 25.35, stage/render/prepare_assets 19.47, stage/render/render 15.08, render/graph/ms 12.96, pipelines/driver_ms 12.96, stage/main/post_update 11.36, encode//sum 11.00, stage/main/update 9.00 |
| warm-up | 5.63 | 29.2 | pipelines/compile_ms 167, pipelines/front_ms 162, render/thread/ms 44.16, render/busy/ms 42.96, main/busy/ms 25.56, stage/render/prepare_assets 22.83, stage/render/render 13.46, render/graph/ms 11.73, stage/main/post_update 11.31, encode//sum 10.62, stage/main/update 9.38, stage/render/graph_submit 7.26 |
| warm-up | 5.74 | 26.7 | pipelines/compile_ms 317, pipelines/front_ms 310, render/thread/ms 35.93, render/busy/ms 34.84, main/busy/ms 28.10, stage/render/prepare_assets 15.89, stage/main/update 15.62, stage/render/render 11.95, render/graph/ms 9.94, encode//sum 9.43, stream/drive/ms 9.27, stage/main/post_update 8.11 |
| warm-up | 5.82 | 37.9 | main/busy/ms 37.81, render/thread/ms 32.72, render/busy/ms 31.39, pipelines/compile_ms 23.85, pipelines/front_ms 22.56, stage/main/update 21.25, stage/main/post_update 12.91, stream/drive/ms 12.75, stage/render/render 12.11, render/graph/ms 10.22, encode//sum 9.87, stage/render/prepare_assets 9.78 |
| warm-up | 6.07 | 25.6 | main/busy/ms 29.55, render/thread/ms 21.59, render/busy/ms 20.49, pipelines/compile_ms 20.44, pipelines/front_ms 19.44, stage/main/update 19.08, stream/drive/ms 11.95, stage/render/render 10.06, render/graph/ms 8.45, stage/main/post_update 7.93, encode//sum 7.93, stage/render/graph_submit 5.65 |
| warm-up | 6.14 | 29.7 | render/thread/ms 40.65, render/busy/ms 39.21, main/busy/ms 30.95, stage/render/render 15.64, stage/main/update 14.33, render/graph/ms 13.42, stage/main/post_update 12.37, encode//sum 11.85, stage/render/graph_submit 9.38, encode/pass:shadow_point_light/sum 8.28, encode/submit 6.02, stream/drive/ms 5.83 |
| warm-up | 6.19 | 30.0 | render/thread/ms 34.97, render/busy/ms 33.96, main/busy/ms 22.18, stage/render/prepare_assets 12.67, stage/render/render 9.96, render/graph/ms 8.53, stage/main/post_update 8.37, stage/main/update 8.02, encode//sum 7.92, stage/render/graph_submit 5.91, encode/pass:shadow_point_light/sum 4.45, encode/submit 3.87 |
| warm-up | 6.42 | 28.9 | pipelines/compile_ms 147, pipelines/front_ms 144, render/thread/ms 39.38, main/busy/ms 38.69, render/busy/ms 38.07, stage/render/render 19.04, stage/main/update 17.80, render/graph/ms 17.12, stage/main/post_update 14.91, encode//sum 13.21, stage/render/graph_submit 11.03, encode/pass:shadow_point_light/sum 10.24 |
| warm-up | 6.45 | 27.2 | pipelines/compile_ms 171, pipelines/front_ms 167, render/thread/ms 44.75, render/busy/ms 43.35, main/busy/ms 40.97, stage/main/update 17.53, stage/main/post_update 16.30, stage/render/render 15.49, render/graph/ms 13.52, stage/render/prepare_assets 13.44, encode//sum 10.64, encode/pass:shadow_point_light/sum 8.67 |
| warm-up | 6.48 | 26.2 | render/thread/ms 40.21, render/busy/ms 38.91, main/busy/ms 33.69, pipelines/compile_ms 24.11, pipelines/front_ms 22.51, stage/main/update 15.31, stage/render/prepare_assets 14.72, stage/render/render 12.07, stage/main/post_update 11.58, render/graph/ms 10.42, encode//sum 8.65, stage/render/graph_submit 6.94 |
| warm-up | 6.50 | 27.0 | render/thread/ms 43.81, render/busy/ms 42.13, main/busy/ms 37.20, stage/render/render 20.27, render/graph/ms 18.05, stage/main/update 15.73, stage/main/post_update 14.83, stage/render/graph_submit 12.60, encode//sum 11.93, stage/render/prepare_assets 10.30, encode/submit 9.47, encode/pass:shadow_point_light/sum 6.68 |
| warm-up | 6.55 | 27.3 | render/thread/ms 38.68, render/busy/ms 37.55, main/busy/ms 35.01, stage/main/update 17.25, stage/render/prepare_assets 13.83, stage/main/post_update 12.96, stage/render/render 11.08, render/graph/ms 9.18, encode//sum 8.77, encode/pass:shadow_point_light/sum 5.84, stage/render/graph_submit 5.54, stream/drive/ms 4.29 |
| warm-up | 6.74 | 31.6 | render/thread/ms 46.86, render/busy/ms 45.57, main/busy/ms 41.43, stage/main/update 25.32, stage/render/prepare_assets 22.51, stream/drive/ms 17.74, stage/render/render 15.21, render/graph/ms 13.08, stage/main/post_update 11.73, encode//sum 11.08, stage/render/graph_submit 9.08, encode/pass:shadow_point_light/sum 8.16 |
| warm-up | 6.76 | 26.9 | render/thread/ms 49.94, render/busy/ms 48.91, main/busy/ms 29.21, stage/render/prepare_assets 27.71, stage/main/update 13.15, stage/render/render 12.05, stage/main/post_update 11.49, encode//sum 10.50, render/graph/ms 10.34, stage/render/graph_submit 7.13, encode/pass:shadow_point_light/sum 7.08, stream/drive/ms 6.32 |
| warm-up | 7.00 | 34.0 | pipelines/compile_ms 141, pipelines/front_ms 137, main/busy/ms 41.27, render/thread/ms 31.61, render/busy/ms 30.62, stage/main/update 22.20, stream/drive/ms 14.05, stage/render/prepare_assets 12.57, stage/main/post_update 12.55, stage/render/render 11.63, render/graph/ms 10.19, encode//sum 7.86 |
| measure | 18.15 | 31.5 | main/busy/ms 38.05, stage/main/update 24.76, stream/drive/ms 20.64, render/thread/ms 16.82, render/busy/ms 15.85, stage/main/post_update 10.41, stage/render/render 9.50, render/graph/ms 8.12, encode//sum 6.36, stage/render/graph_submit 5.95, encode/submit 3.63, encode/finish_wall 2.34 |
| measure | 59.02 | 25.9 | render/thread/ms 35.32, render/busy/ms 33.82, main/busy/ms 30.85, stage/render/render 19.52, stage/main/post_update 17.90, render/graph/ms 17.45, encode//sum 13.15, stage/render/graph_submit 12.77, stage/main/update 8.64, encode/submit 7.55, encode/finish_wall 5.36, stage/render/prepare_assets 5.19 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.face_cull=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=facecull-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 14%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **74%** (min 45%) · 2625 MHz (min 2625) · memory 8101 MHz · 78.1 W · 77 °C (max 83) · VRAM 7006 MiB · throttled 100% of samples

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
- `shadow.lamp.face_cull=on (command line: --set)`
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
