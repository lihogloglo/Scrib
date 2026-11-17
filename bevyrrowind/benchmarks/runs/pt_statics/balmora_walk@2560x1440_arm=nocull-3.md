# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5853 frames over 65.7 s · **median 10.85 ms (92 fps)** · mean 11.22 · p95 14.13 · p99 17.97 (1% low 56 fps) · worst 46.48 ms · 91 spikes over 1.5× median · 57.6 stutters/min (work 62.1/min)
- warm-up: 324 frames over 9.3 s, worst 2569.9 ms
- hitches (> 27.1 ms): **31** measured, 51 in warm-up
- timed GPU passes: 6.89 ms of a 11.22 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.85 ms, limit 11 ms
- **FAIL** frame_ms: 63 stutters (> 1.5x moving median of 60) in 1.09 min = 57.58/min, limit 1/min
- **FAIL** worst frame 46.48 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.921 | 2.037 | 2.068 | 0.94 | 0.176 |
| main_opaque_pass_3d | 1.551 | 1.644 | 2.058 | 0.94 | 0.065 |
| early prepass | 0.473 | 0.502 | 0.735 | 0.94 | 0.084 |
| bloom | 0.296 | 0.314 | 0.319 | 0.94 | 0.063 |
| atmosphere_luts | 0.287 | 0.304 | 0.366 | 0.94 | 0.003 |
| water_velocity | 0.262 | 0.278 | 0.347 | 0.94 | 0.002 |
| water_sspr | 0.190 | 0.202 | 0.203 | 0.94 | 0.002 |
| ssao | 0.178 | 0.188 | 0.193 | 0.94 | 0.007 |
| auxiliary_views | 0.170 | 0.181 | 0.605 | 0.94 | 0.181 |
| uniform_allocation | 0.147 | 0.156 | 0.212 | 0.94 | 0.027 |
| atmos_froxel_inject | 0.138 | 0.146 | 0.148 | 0.94 | 0.001 |
| clouds_trace | 0.133 | 0.141 | 0.234 | 0.94 | 0.001 |
| water_fft | 0.123 | 0.130 | 0.138 | 0.94 | 0.003 |
| upscaling | 0.110 | 0.116 | 0.124 | 0.94 | 0.004 |
| bin_unpacking | 0.104 | 0.110 | 0.147 | 0.94 | 0.019 |
| clouds_composite | 0.100 | 0.106 | 0.109 | 0.94 | 0.002 |
| main_transmissive_pass_3d | 0.081 | 0.086 | 0.179 | 0.94 | 0.008 |
| tonemapping | 0.080 | 0.085 | 0.088 | 0.94 | 0.004 |
| clustering | 0.060 | 0.064 | 0.067 | 0.94 | 0.018 |
| render_sky | 0.049 | 0.052 | 0.053 | 0.94 | 0.002 |
| clouds_resolve | 0.046 | 0.048 | 0.049 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.413 | 0.417 | 0.10 | 0.001 |
| surface_exposure | 0.041 | 0.043 | 0.044 | 0.94 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.69 | 0.002 |
| early_mesh_preprocessing | 0.038 | 0.040 | 0.058 | 0.94 | 0.005 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.83 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.94 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.94 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.94 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.221 | 0.233 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.030 | 0.94 | 0.174 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.94 | 0.002 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.94 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.831 | 12.557 | 39.96 | 57539.7 |
| render/busy/ms | 9.286 | 12.017 | 39.41 | 54351.3 |
| main/busy/ms | 8.700 | 10.503 | 28.59 | 50921.2 |
| stage/render/render | 5.669 | 7.376 | 16.94 | 33183.0 |
| stage/main/post_update | 5.293 | 6.679 | 11.47 | 30982.0 |
| render/graph/ms | 4.899 | 6.578 | 15.72 | 28674.7 |
| encode//sum | 4.328 | 6.241 | 23.42 | 25331.3 |
| stage/render/graph_submit | 3.622 | 4.851 | 11.07 | 21199.4 |
| encode/submit | 2.290 | 3.198 | 7.06 | 13403.1 |
| stage/main/update | 2.145 | 2.558 | 22.27 | 12554.0 |
| encode/finish_wall | 1.406 | 2.017 | 5.01 | 8229.6 |
| stage/render/camera_driver | 1.223 | 1.748 | 4.57 | 7158.7 |
| encode/pass:shadow_point_light/sum | 1.065 | 3.583 | 17.62 | 6231.7 |
| encode/pass:shadow_directional_light/sum | 0.847 | 1.581 | 2.51 | 4958.0 |
| stage/render/prepare_write_phase_buffers | 0.799 | 1.215 | 2.69 | 4675.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.755 | 1.091 | 2.12 | 4421.7 |
| encode/pass:early prepass/sum | 0.732 | 1.078 | 1.85 | 4283.3 |
| stage/render/prepare_assets | 0.567 | 0.727 | 9.91 | 3321.5 |
| stage/render/prepare_meshes | 0.497 | 0.678 | 3.57 | 2906.5 |
| bigspace/total/ms | 0.475 | 1.034 | 2.49 | 2782.2 |
| stage/render/prepare_resources | 0.452 | 0.547 | 0.89 | 2644.1 |
| render/present/ms | 0.451 | 0.725 | 2.22 | 2642.3 |
| stage/main/first | 0.392 | 0.518 | 5.97 | 2292.7 |
| stage/render/specialize | 0.366 | 0.470 | 12.99 | 2142.2 |
| stage/main/pre_update | 0.355 | 0.502 | 2.79 | 2078.1 |
| stage/render/queue | 0.320 | 0.473 | 8.20 | 1873.3 |
| terrain/update/ms | 0.309 | 0.399 | 2.06 | 1808.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.302 | 0.442 | 1.13 | 1767.5 |
| stage/render/prepare_bind_groups | 0.299 | 0.565 | 2.62 | 1749.8 |
| terrain/select/ms | 0.276 | 0.358 | 0.95 | 1613.4 |
| controller/feed/ms | 0.272 | 0.342 | 0.69 | 1594.4 |
| stage/render/prepare_batch_phases | 0.262 | 0.370 | 0.67 | 1531.1 |
| stream/drive/ms | 0.240 | 0.321 | 19.95 | 1404.2 |
| stage/main/fixed_main_loop | 0.219 | 0.367 | 1.11 | 1279.2 |
| stage/render/create_views | 0.197 | 0.254 | 0.95 | 1155.7 |
| far/select/ms | 0.196 | 0.267 | 0.77 | 1145.0 |
| bigspace/low_precision/ms | 0.192 | 0.556 | 1.70 | 1124.2 |
| stream/tick/ms | 0.183 | 0.247 | 0.50 | 1073.9 |
| stage/render/extract_commands | 0.170 | 0.200 | 0.43 | 995.6 |
| stage/render/prepare_flush | 0.164 | 0.312 | 0.96 | 958.5 |
| anim/evaluate/ms | 0.151 | 0.250 | 1.98 | 885.8 |
| anim/evaluate_cpu/ms | 0.148 | 0.188 | 0.70 | 867.6 |
| stage/render/prepare_views | 0.136 | 0.168 | 0.73 | 798.8 |
| controller/move/ms | 0.125 | 0.220 | 0.51 | 732.4 |
| bigspace/high_precision/ms | 0.122 | 0.270 | 2.15 | 716.4 |
| stage/main/last | 0.118 | 0.169 | 1.59 | 692.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.101 | 0.165 | 0.78 | 592.6 |
| render/acquire/ms | 0.093 | 0.117 | 0.68 | 546.1 |
| stage/render/cleanup | 0.074 | 0.194 | 0.74 | 435.9 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.133 | 0.33 | 366.4 |
| stage/render/phase_sort | 0.062 | 0.092 | 0.96 | 363.1 |
| pipelines/compile_ms | 0.051 | 0.000 | 127.50 | 301.2 |
| pipelines/front_ms | 0.049 | 0.000 | 123.67 | 289.1 |
| physics/step/ms | 0.047 | 0.086 | 0.40 | 277.9 |
| physics/insert/ms | 0.034 | 0.369 | 0.84 | 198.2 |
| encode/resolve_diagnostics/sum | 0.033 | 0.052 | 0.53 | 195.9 |
| weather/map/ms | 0.026 | 0.034 | 0.19 | 149.4 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.52 | 86.0 |
| anim/apply/ms | 0.011 | 0.021 | 0.45 | 65.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.60 | 64.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.12 | 62.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 60.4 |
| script/update/ms | 0.007 | 0.009 | 0.20 | 40.8 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 30.0 |
| stream/despawn/ms | 0.005 | 0.000 | 0.89 | 29.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 27.3 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 24.7 |
| controller/write/ms | 0.003 | 0.005 | 0.08 | 20.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.85 | 14.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.22 | 12.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.23 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.10 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1277 | 2262 | 595 |
| crowd/c0/meshes/count | count | 352 | 646 | 158 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5940 | 5601 |
| crowd/c1/meshes/count | count | 1179 | 1270 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.44 | 1.61 | 0.50 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.52 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.60 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.51 | 1.05 |
| encode/pass:early prepass/max | ms | 0.75 | 1.85 | 1.05 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 2.12 | 0.92 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.78 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.29 | 1.13 | 0.27 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.51 | 0.90 |
| encode/pass:shadow_point_light/max | ms | 0.82 | 1.47 | 0.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.10 | 39.41 | 8.91 |
| gpu/timed/ms | ms | 8.02 | 11.36 | 8.24 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 249 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3916 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3916 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 4098 | 4330 | 4330 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 4098 | 4330 | 4330 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 234 | 31 |
| phases/shadow/bins | count | 14.2 k | 19.1 k | 12.1 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 40.4 k | 56.7 k | 39.9 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 208 | 254 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 20 | 635 | 60 |
| preprocess/bind_groups_created | count | 20 | 635 | 60 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 135 | 232 | 125 |
| process/cpu_usage | % | 25.48 | 39.42 | 21.15 |
| process/mem_usage | GiB | 5.54 | 5.70 | 5.70 |
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
| skin/blocks_changed/count | count | 13 | 128 | 14 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.4 k | 808.8 k | 489.5 k |
| skin/joints_written/count | count | 5772 | 12.6 k | 7648 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5010 | 5010 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.43 | 38.46 | 32.66 |
| system/mem_usage | % | 67.12 | 67.74 | 67.68 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 74 | 285 | 81 |
| views/all/bins | count | 23.0 k | 27.6 k | 21.7 k |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 69.0 k | 85.3 k | 69.4 k |
| views/all/pipelines | count | 256 | 304 | 77 |
| views/all/pipelines_distinct | count | 72 | 76 | 61 |
| views/all/work_items | count | 41.4 k | 83.5 k | 70.1 k |
| views/cascade/batch_sets | count | 10 | 52 | 31 |
| views/cascade/bins | count | 12.0 k | 15.3 k | 12.1 k |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 39.0 k | 53.1 k | 39.9 k |
| views/cascade/pipelines | count | 38 | 56 | 31 |
| views/cascade/work_items | count | 11.7 k | 53.1 k | 39.9 k |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 3154 | 4332 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3394 | 4628 | 0 |
| views/lamp_face/pipelines | count | 170 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 4091 | 0 |
| views/main/batch_sets | count | 51 | 55 | 50 |
| views/main/bins | count | 8678 | 9602 | 9602 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 47 | 51 | 46 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 50.15 | 227 | 293.5 k |
| anim/events/count | count | 0.09 | 3 | 518 |
| assets/Image/added | count | 0.10 | 40 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.83 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.70 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.80 | 1296 | 4675 |
| encode//count | count | 34.88 | 220 | 204.1 k |
| encode/bind_walk/init_textures | count | 13.4 k | 119.2 k | 78.72 M |
| encode/bind_walk/init_textures_skipped | count | 60.9 k | 149.3 k | 356.48 M |
| encode/bind_walk/inits | count | 530 | 1342 | 3.10 M |
| encode/bind_walk/inits_skipped | count | 130 | 204 | 761.9 k |
| encode/bind_walk/merge_textures | count | 13.4 k | 119.1 k | 78.30 M |
| encode/bind_walk/merge_textures_skipped | count | 5503 | 6026 | 32.21 M |
| encode/bind_walk/merges | count | 103 | 357 | 603.9 k |
| encode/bind_walk/merges_skipped | count | 108 | 156 | 631.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5853 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5853 |
| encode/pass:early prepass/count | count | 1 | 1 | 5853 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5853 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5853 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5853 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5853 |
| encode/pass:shadow_point_light/count | count | 1.73 | 48 | 10.1 k |
| encode/queue/command_buffers | count | 46.62 | 278 | 272.9 k |
| encode/queue/submits | count | 2.02 | 4 | 11.8 k |
| encode/queue/write_buffer/bytes | bytes | 2.34 M | 6.64 M | 13.68 G |
| encode/queue/write_buffer/calls | count | 274 | 948 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.0 k | 3.52 M | 1.05 G |
| encode/queue/write_buffer_with/calls | count | 11.07 | 465 | 64.8 k |
| encode/queue/write_texture/bytes | bytes | 12.2 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5853 |
| physics/inserts | count | 0.38 | 104 | 2246 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 161 | 2335 |
| physics/steps | count | 0.67 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5784 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1268 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2569.9 | main/busy/ms 2539, stage/main/post_update 2429, stage/main/update 101, surface/advance_cpu_ms 71.81, stage/main/pre_update 5.31, stage/main/last 1.30, surface/cell_upload_ms 0.74, stage/main/first 0.53, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.37, weather/map/ms 0.14, stream/tick/ms 0.14 |
| warm-up | 3.31 | 740.5 | main/busy/ms 2680, stage/main/post_update 2454, render/thread/ms 701, render/busy/ms 538, stage/render/prepare_assets 490, stage/main/update 210, stage/render/prepare_views 160, render/acquire/ms 160, surface/advance_cpu_ms 71.81, stage/render/create_views 28.50, stage/render/render 12.03, render/graph/ms 7.48 |
| warm-up | 3.42 | 113.9 | render/thread/ms 797, render/busy/ms 580, stage/render/prepare_assets 516, main/busy/ms 162, stage/render/prepare_views 160, render/acquire/ms 160, pipelines/compile_ms 147, stage/main/update 113, pipelines/front_ms 98.48, stage/render/render 72.56, render/present/ms 57.82, pipelines/driver_ms 48.94 |
| warm-up | 3.60 | 160.2 | pipelines/wait_ms 4736, pipelines/compile_ms 1471, pipelines/front_ms 1368, render/thread/ms 136, render/busy/ms 134, main/busy/ms 126, pipelines/driver_ms 102, stage/main/post_update 51.12, stage/render/prepare_assets 36.73, stage/main/pre_update 36.19, stage/main/update 34.92, stage/render/prepare_resources 33.41 |
| warm-up | 3.65 | 52.6 | pipelines/wait_ms 5004, pipelines/compile_ms 1858, pipelines/front_ms 1742, render/thread/ms 165, render/busy/ms 163, main/busy/ms 153, pipelines/driver_ms 117, stage/main/update 79.22, stage/render/prepare_assets 51.88, stage/render/prepare_resources 42.66, stage/main/pre_update 40.59, stage/main/post_update 25.82 |
| warm-up | 3.75 | 95.8 | pipelines/compile_ms 1496, pipelines/front_ms 1408, pipelines/wait_ms 309, render/thread/ms 103, render/busy/ms 101, main/busy/ms 90.43, pipelines/driver_ms 87.35, stage/render/prepare_assets 60.44, stage/main/update 58.10, stage/render/render 17.54, stage/main/post_update 17.43, render/graph/ms 15.27 |
| warm-up | 3.94 | 54.2 | pipelines/compile_ms 279, pipelines/front_ms 269, render/thread/ms 58.98, render/busy/ms 57.30, main/busy/ms 47.04, stage/render/prepare_assets 41.02, stage/main/update 35.90, stream/drive/ms 31.75, stage/render/render 9.58, pipelines/driver_ms 9.41, render/graph/ms 7.46, stage/main/post_update 6.24 |
| warm-up | 4.03 | 79.0 | pipelines/compile_ms 1066, pipelines/front_ms 1020, main/busy/ms 54.99, render/thread/ms 52.24, render/busy/ms 50.24, pipelines/driver_ms 45.73, stage/main/update 41.39, stream/drive/ms 35.94, stage/render/prepare_assets 28.39, stage/render/render 15.67, render/graph/ms 13.14, stage/render/graph_submit 10.49 |
| warm-up | 4.13 | 89.6 | pipelines/compile_ms 476, pipelines/front_ms 460, render/thread/ms 62.40, render/busy/ms 60.30, main/busy/ms 42.66, stage/render/prepare_assets 39.59, stage/main/update 20.85, pipelines/driver_ms 16.00, stage/main/post_update 15.22, stream/drive/ms 14.48, stage/render/render 14.48, render/graph/ms 11.91 |
| warm-up | 4.17 | 43.2 | pipelines/compile_ms 765, pipelines/front_ms 740, render/thread/ms 79.81, render/busy/ms 78.14, stage/render/prepare_assets 56.07, main/busy/ms 40.27, pipelines/driver_ms 25.42, stage/main/update 21.07, stage/render/render 11.94, stage/main/post_update 11.82, render/graph/ms 9.77, stage/render/graph_submit 7.10 |
| warm-up | 4.30 | 127.6 | pipelines/compile_ms 1049, pipelines/front_ms 826, pipelines/driver_ms 223, render/thread/ms 126, render/busy/ms 125, main/busy/ms 63.94, stage/render/prepare_assets 59.17, stage/render/render 57.26, render/graph/ms 55.15, stage/render/camera_driver 49.94, stage/main/update 45.33, stage/main/post_update 10.66 |
| warm-up | 4.37 | 70.9 | pipelines/compile_ms 1707, pipelines/front_ms 1467, pipelines/driver_ms 240, render/thread/ms 159, render/busy/ms 156, stage/render/render 79.58, render/graph/ms 76.01, stage/render/prepare_assets 65.94, main/busy/ms 65.33, stage/render/camera_driver 51.42, stage/main/update 43.59, stage/main/post_update 12.15 |
| warm-up | 4.43 | 52.8 | pipelines/compile_ms 1380, pipelines/front_ms 1338, pipelines/wait_ms 194, render/thread/ms 102, render/busy/ms 99.29, main/busy/ms 53.60, stage/render/prepare_assets 43.11, pipelines/driver_ms 41.82, stage/render/render 35.39, render/graph/ms 31.55, stage/main/post_update 25.07, pipelines/lock_ms 24.40 |
| warm-up | 4.52 | 90.7 | pipelines/compile_ms 516, pipelines/front_ms 498, pipelines/wait_ms 396, render/thread/ms 120, render/busy/ms 118, main/busy/ms 70.06, stage/main/post_update 45.94, stage/render/prepare_assets 32.38, pipelines/lock_ms 22.28, stage/render/render 21.75, stage/render/prepare_resources 21.26, encode//sum 20.61 |
| warm-up | 4.60 | 79.1 | pipelines/compile_ms 2050, pipelines/front_ms 1985, pipelines/wait_ms 471, render/thread/ms 153, render/busy/ms 150, pipelines/driver_ms 65.31, main/busy/ms 59.14, stage/render/prepare_assets 46.38, stage/main/post_update 34.19, stage/render/render 29.31, stage/render/prepare_resources 28.14, encode//sum 27.54 |
| warm-up | 4.65 | 51.5 | pipelines/compile_ms 2674, pipelines/front_ms 2586, pipelines/wait_ms 343, main/busy/ms 123, render/thread/ms 121, render/busy/ms 118, stage/main/update 101, pipelines/driver_ms 88.13, encode//sum 73.04, stream/drive/ms 60.84, stage/render/render 43.20, render/graph/ms 40.30 |
| warm-up | 4.77 | 127.4 | pipelines/compile_ms 2165, pipelines/front_ms 2101, pipelines/wait_ms 296, main/busy/ms 129, stage/main/update 103, render/thread/ms 96.90, render/busy/ms 95.11, pipelines/driver_ms 64.45, encode//sum 61.42, stream/drive/ms 61.25, stage/render/prepare_assets 41.46, stage/render/render 35.23 |
| warm-up | 4.82 | 47.1 | pipelines/compile_ms 1906, pipelines/front_ms 1855, pipelines/wait_ms 220, main/busy/ms 125, render/thread/ms 92.20, render/busy/ms 90.59, stage/main/update 85.58, stream/drive/ms 77.17, pipelines/driver_ms 50.37, stage/render/prepare_assets 42.51, stage/main/post_update 26.15, stage/render/render 25.29 |
| warm-up | 4.95 | 128.3 | pipelines/compile_ms 882, pipelines/front_ms 857, main/busy/ms 132, render/thread/ms 86.97, stage/main/update 86.18, render/busy/ms 85.18, stream/drive/ms 77.86, pipelines/wait_ms 39.75, stage/main/post_update 31.90, stage/render/prepare_assets 30.27, stage/render/render 26.45, pipelines/driver_ms 25.21 |
| warm-up | 4.99 | 44.9 | pipelines/compile_ms 2019, pipelines/front_ms 1963, main/busy/ms 99.51, render/thread/ms 72.17, render/busy/ms 70.44, pipelines/wait_ms 65.86, stage/main/update 61.84, pipelines/driver_ms 56.63, stream/drive/ms 50.02, stage/render/prepare_assets 38.02, stage/main/post_update 24.28, stage/render/render 15.07 |
| warm-up | 5.08 | 86.5 | pipelines/compile_ms 2334, pipelines/front_ms 2270, main/busy/ms 100, stage/main/update 68.16, pipelines/driver_ms 63.18, render/thread/ms 61.73, render/busy/ms 59.97, stream/drive/ms 49.29, stage/render/prepare_assets 31.61, pipelines/wait_ms 29.93, stage/main/post_update 19.68, stage/render/render 14.78 |
| warm-up | 5.11 | 33.4 | pipelines/compile_ms 835, pipelines/front_ms 811, main/busy/ms 96.11, stage/main/update 65.14, render/thread/ms 60.69, render/busy/ms 58.92, stream/drive/ms 50.89, pipelines/driver_ms 24.03, stage/render/prepare_assets 23.55, stage/render/render 22.44, render/graph/ms 19.96, stage/main/post_update 19.62 |
| warm-up | 5.21 | 99.8 | pipelines/compile_ms 555, pipelines/front_ms 529, main/busy/ms 100, render/thread/ms 72.80, render/busy/ms 71.22, stage/main/update 71.05, stream/drive/ms 51.09, pipelines/driver_ms 25.74, stage/render/prepare_assets 24.84, stage/render/render 21.58, stage/main/post_update 20.42, render/graph/ms 19.46 |
| warm-up | 5.26 | 44.5 | pipelines/compile_ms 451, pipelines/front_ms 431, render/thread/ms 82.20, render/busy/ms 80.75, main/busy/ms 61.02, stage/render/prepare_assets 30.29, stage/main/update 28.06, stage/main/post_update 23.85, pipelines/driver_ms 20.85, stage/render/render 13.43, render/graph/ms 11.37, stage/render/specialize 11.02 |
| warm-up | 5.32 | 59.8 | pipelines/compile_ms 1510, pipelines/front_ms 1475, render/thread/ms 78.56, render/busy/ms 77.14, main/busy/ms 61.90, stage/main/post_update 35.98, pipelines/driver_ms 34.45, stage/render/prepare_assets 17.60, stage/render/prepare_write_phase_buffers 15.28, bigspace/total/ms 14.42, stage/main/update 14.13, stage/render/render 13.26 |
| warm-up | 5.35 | 32.7 | pipelines/compile_ms 2306, pipelines/front_ms 2250, render/thread/ms 69.63, render/busy/ms 68.11, pipelines/driver_ms 55.92, main/busy/ms 52.89, stage/main/post_update 29.27, stage/render/render 16.81, render/graph/ms 14.48, bigspace/total/ms 14.40, stage/render/prepare_write_phase_buffers 13.78, stage/main/update 12.57 |
| warm-up | 5.39 | 40.1 | pipelines/compile_ms 906, pipelines/front_ms 879, render/thread/ms 64.33, render/busy/ms 62.86, main/busy/ms 42.48, pipelines/driver_ms 27.02, stage/main/post_update 20.38, stage/render/prepare_resources 18.14, stage/render/render 15.57, stage/main/update 14.31, render/graph/ms 13.44, stage/render/graph_submit 10.39 |
| warm-up | 5.43 | 35.7 | pipelines/compile_ms 307, pipelines/front_ms 297, render/thread/ms 64.28, render/busy/ms 62.88, main/busy/ms 45.05, stage/main/post_update 21.47, stage/render/prepare_resources 18.34, stage/main/update 15.81, stage/render/prepare_assets 14.60, stage/render/render 11.63, pipelines/driver_ms 10.01, render/graph/ms 9.50 |
| warm-up | 5.46 | 31.1 | pipelines/compile_ms 573, pipelines/front_ms 557, render/thread/ms 51.05, render/busy/ms 49.61, pipelines/wait_ms 47.16, main/busy/ms 35.63, stage/render/prepare_assets 18.89, stage/main/update 16.00, pipelines/driver_ms 15.60, stage/main/post_update 11.84, stage/render/render 11.47, render/graph/ms 9.46 |
| warm-up | 5.61 | 30.4 | pipelines/compile_ms 214, pipelines/front_ms 204, render/thread/ms 44.04, render/busy/ms 42.21, main/busy/ms 41.62, stage/main/post_update 19.39, stage/render/render 18.00, render/graph/ms 15.67, stage/main/update 14.70, stage/render/graph_submit 11.86, pipelines/driver_ms 9.13, encode//sum 7.15 |
| warm-up | 5.65 | 39.9 | pipelines/compile_ms 130, pipelines/front_ms 123, render/thread/ms 59.58, render/busy/ms 58.05, main/busy/ms 43.88, stage/render/prepare_assets 25.20, stage/main/update 21.58, stage/render/render 14.19, stage/main/post_update 13.55, render/graph/ms 12.09, stage/render/graph_submit 8.78, encode//sum 7.96 |
| warm-up | 5.71 | 32.4 | pipelines/compile_ms 210, pipelines/front_ms 203, render/thread/ms 53.79, render/busy/ms 52.49, main/busy/ms 39.50, stage/main/update 21.14, stage/render/phase_sort 16.68, stream/drive/ms 12.78, stage/main/post_update 12.12, stage/render/prepare_assets 11.80, stage/render/render 11.41, render/graph/ms 9.58 |
| warm-up | 5.74 | 35.8 | pipelines/compile_ms 437, pipelines/front_ms 424, render/thread/ms 63.08, render/busy/ms 61.75, main/busy/ms 47.15, stage/main/update 26.96, stage/render/prepare_assets 25.50, stage/main/post_update 14.10, stream/drive/ms 13.47, pipelines/driver_ms 12.83, stage/render/render 12.45, stage/render/phase_sort 12.39 |
| warm-up | 5.81 | 40.2 | pipelines/compile_ms 1586, pipelines/front_ms 1543, render/thread/ms 55.13, render/busy/ms 53.57, pipelines/driver_ms 42.78, main/busy/ms 32.30, stage/render/prepare_assets 24.26, stage/main/post_update 14.22, stage/render/render 12.24, stage/main/update 10.96, render/graph/ms 9.85, encode//sum 9.01 |
| warm-up | 5.90 | 29.8 | pipelines/compile_ms 293, pipelines/front_ms 279, render/thread/ms 50.60, render/busy/ms 49.30, stage/render/prepare_assets 29.87, main/busy/ms 25.98, pipelines/driver_ms 14.70, stage/main/post_update 11.95, stage/render/render 11.74, encode//sum 9.51, render/graph/ms 9.46, stage/main/update 8.07 |
| warm-up | 6.01 | 33.2 | pipelines/compile_ms 458, pipelines/front_ms 433, render/thread/ms 52.01, render/busy/ms 50.50, main/busy/ms 36.50, pipelines/driver_ms 24.66, stage/render/prepare_assets 23.40, stage/main/update 17.63, stage/main/post_update 12.97, stage/render/render 12.68, stream/drive/ms 10.55, render/graph/ms 9.86 |
| warm-up | 6.48 | 33.3 | render/thread/ms 40.22, render/busy/ms 39.09, main/busy/ms 26.76, stage/render/prepare_assets 14.44, stage/main/post_update 11.96, stage/main/update 9.99, stage/render/render 9.18, render/graph/ms 7.74, encode//sum 6.75, stage/render/graph_submit 5.61, stage/render/specialize 5.55, encode/submit 3.78 |
| warm-up | 6.69 | 33.8 | render/thread/ms 55.50, render/busy/ms 53.64, main/busy/ms 37.89, stage/render/render 16.88, stage/main/update 16.09, render/graph/ms 14.59, stage/main/post_update 14.41, stage/render/specialize 14.04, stage/render/graph_submit 11.09, encode//sum 10.08, encode/submit 7.50, stage/render/queue 6.42 |
| warm-up | 6.75 | 31.9 | pipelines/compile_ms 72.86, pipelines/front_ms 69.17, render/thread/ms 42.45, render/busy/ms 41.31, main/busy/ms 27.98, stage/render/prepare_assets 18.63, stage/main/update 13.50, stage/main/post_update 9.04, stage/render/render 7.54, encode//sum 6.74, render/graph/ms 5.94, stage/render/specialize 5.14 |
| warm-up | 6.84 | 32.3 | main/busy/ms 45.38, render/thread/ms 43.63, render/busy/ms 42.53, stage/main/update 29.21, stage/render/render 14.14, stream/drive/ms 13.38, stage/render/prepare_assets 13.37, render/graph/ms 12.26, encode//sum 11.76, stage/main/post_update 11.16, stage/render/graph_submit 8.32, encode/submit 4.42 |
| warm-up | 7.24 | 35.6 | main/busy/ms 42.49, render/thread/ms 32.51, render/busy/ms 31.50, stage/main/update 23.85, stream/drive/ms 13.32, stage/render/prepare_assets 12.91, stage/main/post_update 11.31, stage/render/render 8.22, render/graph/ms 6.56, encode//sum 6.11, stage/render/graph_submit 4.41, stage/render/specialize 2.49 |
| warm-up | 7.29 | 31.6 | render/thread/ms 44.64, render/busy/ms 43.52, main/busy/ms 26.34, stage/render/specialize 11.83, stage/render/render 11.08, stage/main/update 10.95, stage/main/post_update 10.69, render/graph/ms 9.52, encode//sum 8.51, stage/render/queue 7.66, stage/render/graph_submit 7.36, stage/render/prepare_assets 4.25 |
| measure | 0.40 | 29.5 | render/thread/ms 45.77, render/busy/ms 44.74, encode//sum 19.99, stage/render/render 18.98, render/graph/ms 16.87, main/busy/ms 15.63, stage/render/graph_submit 12.99, encode/pass:shadow_point_light/sum 11.18, stage/render/specialize 8.98, stage/main/post_update 8.64, encode/submit 8.03, stage/render/queue 6.68 |
| measure | 9.61 | 30.8 | render/thread/ms 38.88, render/busy/ms 37.85, main/busy/ms 20.65, stage/render/render 13.33, stage/main/post_update 12.87, render/graph/ms 11.66, stage/render/specialize 10.17, encode//sum 9.78, stage/render/graph_submit 8.58, encode/pass:shadow_point_light/sum 6.52, stage/render/queue 6.18, encode/submit 5.21 |
| measure | 15.74 | 31.1 | render/thread/ms 36.98, render/busy/ms 35.98, main/busy/ms 18.37, stage/main/post_update 11.35, stage/render/render 11.34, render/graph/ms 9.89, stage/render/specialize 9.75, encode//sum 8.75, stage/render/graph_submit 7.45, stage/render/queue 7.17, encode/submit 4.39, stage/main/update 4.25 |
| measure | 18.00 | 28.8 | render/thread/ms 36.98, render/busy/ms 35.92, main/busy/ms 18.32, stage/main/post_update 11.06, stage/render/render 11.06, stage/render/specialize 10.00, render/graph/ms 9.50, encode//sum 8.92, stage/render/queue 7.04, stage/render/graph_submit 6.97, stage/main/update 4.56, encode/submit 4.21 |
| measure | 18.18 | 32.7 | main/busy/ms 39.38, stage/main/update 24.51, render/thread/ms 22.01, render/busy/ms 20.98, stream/drive/ms 20.24, stage/render/render 12.35, stage/main/post_update 12.10, render/graph/ms 10.76, encode//sum 9.85, stage/render/graph_submit 7.56, encode/submit 4.74, encode/pass:shadow_point_light/sum 3.94 |
| measure | 19.50 | 35.3 | render/thread/ms 42.42, render/busy/ms 41.23, main/busy/ms 24.18, stage/main/post_update 14.04, stage/render/render 12.51, stage/render/specialize 12.11, render/graph/ms 10.85, encode//sum 9.75, stage/render/queue 8.61, stage/render/graph_submit 7.97, stage/main/update 5.46, encode/submit 4.98 |
| measure | 23.05 | 33.5 | render/thread/ms 45.14, render/busy/ms 44.05, main/busy/ms 25.90, stage/render/render 13.98, stage/main/post_update 13.80, render/graph/ms 12.29, stage/render/specialize 12.12, encode//sum 11.12, stage/render/graph_submit 9.52, stage/render/queue 8.23, stage/main/update 6.21, encode/submit 5.48 |
| measure | 23.10 | 29.0 | render/thread/ms 41.57, render/busy/ms 40.47, main/busy/ms 33.82, encode//sum 15.81, stage/render/render 15.11, stage/main/update 13.86, stage/main/post_update 13.61, render/graph/ms 13.37, encode/pass:shadow_point_light/sum 10.23, stage/render/graph_submit 9.04, stage/render/prepare_assets 8.55, encode/submit 5.10 |
| measure | 24.78 | 30.6 | render/thread/ms 42.51, render/busy/ms 41.32, main/busy/ms 20.52, stage/render/render 14.41, encode//sum 13.26, render/graph/ms 12.58, stage/main/post_update 11.61, stage/render/specialize 10.08, stage/render/graph_submit 9.32, stage/render/queue 7.04, encode/pass:shadow_point_light/sum 6.59, encode/submit 5.66 |
| measure | 27.29 | 36.7 | render/thread/ms 45.44, render/busy/ms 44.33, main/busy/ms 27.47, stage/render/render 15.63, stage/main/post_update 14.77, render/graph/ms 13.84, encode//sum 13.48, stage/render/specialize 11.92, stage/render/graph_submit 11.07, stage/render/queue 8.45, encode/submit 7.30, encode/pass:shadow_point_light/sum 7.25 |
| measure | 28.95 | 31.7 | render/thread/ms 41.07, render/busy/ms 40.05, main/busy/ms 18.38, stage/render/render 13.76, encode//sum 13.22, render/graph/ms 12.14, stage/main/post_update 11.35, stage/render/specialize 10.90, stage/render/graph_submit 8.84, stage/render/queue 7.13, encode/pass:shadow_point_light/sum 6.66, encode/submit 5.44 |
| measure | 30.58 | 31.5 | render/thread/ms 42.30, render/busy/ms 41.21, main/busy/ms 18.49, stage/render/render 14.60, render/graph/ms 12.92, encode//sum 12.25, stage/main/post_update 11.18, stage/render/specialize 10.25, stage/render/graph_submit 9.73, stage/render/queue 7.51, encode/submit 6.13, encode/pass:shadow_point_light/sum 6.09 |
| measure | 32.14 | 31.2 | render/thread/ms 41.80, render/busy/ms 40.73, main/busy/ms 19.58, stage/render/render 14.39, encode//sum 12.73, render/graph/ms 12.70, stage/main/post_update 11.93, stage/render/specialize 10.08, stage/render/graph_submit 9.46, stage/render/queue 7.73, encode/pass:shadow_point_light/sum 7.02, encode/submit 5.59 |
| measure | 35.36 | 34.0 | render/thread/ms 41.30, render/busy/ms 40.09, main/busy/ms 21.79, stage/main/post_update 13.42, stage/render/render 12.89, encode//sum 12.59, stage/render/specialize 11.37, render/graph/ms 11.17, stage/render/graph_submit 8.62, stage/render/queue 8.29, encode/submit 4.98, stage/main/update 4.90 |
| measure | 37.48 | 29.3 | render/thread/ms 37.19, render/busy/ms 36.09, main/busy/ms 17.15, stage/render/render 11.20, stage/main/post_update 10.48, stage/render/specialize 9.97, render/graph/ms 9.61, encode//sum 9.22, stage/render/graph_submit 7.35, stage/render/queue 6.72, stage/main/update 4.15, encode/submit 3.96 |
| measure | 40.80 | 32.2 | render/thread/ms 37.08, render/busy/ms 36.08, main/busy/ms 18.58, stage/main/post_update 10.89, stage/render/specialize 10.43, stage/render/render 9.57, render/graph/ms 8.13, encode//sum 7.87, stage/render/queue 6.79, stage/render/graph_submit 6.13, stage/main/update 4.33, encode/submit 3.75 |
| measure | 43.23 | 32.0 | render/thread/ms 36.20, render/busy/ms 35.21, main/busy/ms 18.41, stage/main/post_update 11.69, stage/render/render 10.27, stage/render/specialize 10.24, encode//sum 9.21, render/graph/ms 8.72, stage/render/queue 7.30, stage/render/graph_submit 6.80, stage/main/update 3.85, encode/submit 3.60 |
| measure | 45.66 | 34.4 | render/thread/ms 39.74, render/busy/ms 38.82, main/busy/ms 16.30, stage/render/render 11.47, stage/render/specialize 10.98, render/graph/ms 10.09, stage/main/post_update 9.38, stage/render/graph_submit 8.02, encode//sum 7.93, stage/render/queue 7.63, stage/main/update 4.30, encode/submit 4.04 |
| measure | 56.43 | 28.9 | render/thread/ms 34.33, render/busy/ms 33.29, main/busy/ms 13.82, stage/render/render 9.65, stage/render/specialize 9.59, render/graph/ms 8.28, stage/main/post_update 7.80, encode//sum 7.60, stage/render/queue 7.44, stage/render/graph_submit 6.13, encode/submit 3.73, stage/main/update 3.67 |
| measure | 59.05 | 46.5 | render/thread/ms 57.49, render/busy/ms 55.72, main/busy/ms 31.26, stage/render/render 19.71, stage/main/post_update 18.24, render/graph/ms 17.48, stage/render/graph_submit 14.19, stage/render/specialize 13.21, encode//sum 12.61, encode/submit 8.79, stage/main/update 8.19, stage/render/queue 6.88 |
| measure | 59.08 | 31.8 | render/thread/ms 60.37, render/busy/ms 59.21, main/busy/ms 24.33, stage/render/prepare_assets 15.48, stage/render/specialize 13.87, stage/main/post_update 11.75, stage/render/render 11.39, render/graph/ms 9.97, encode//sum 8.87, stage/render/graph_submit 7.74, stage/render/queue 7.14, stage/main/update 7.08 |
| measure | 62.57 | 33.8 | render/thread/ms 38.33, render/busy/ms 37.31, main/busy/ms 18.97, stage/render/render 11.45, stage/render/specialize 11.22, stage/main/post_update 10.17, render/graph/ms 10.08, encode//sum 8.82, stage/render/graph_submit 7.96, stage/render/queue 6.92, stage/main/update 6.00, encode/submit 4.25 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=nocull-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **77%** (min 43%) · 2620 MHz (min 2610) · memory 8101 MHz · 78.9 W · 77 °C (max 82) · VRAM 7023 MiB · throttled 100% of samples

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
