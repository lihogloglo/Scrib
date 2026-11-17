# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: precombine · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5388 frames over 65.6 s · **median 11.75 ms (85 fps)** · mean 12.18 · p95 15.29 · p99 19.28 (1% low 52 fps) · worst 62.72 ms · 74 spikes over 1.5× median · 54.8 stutters/min (work 43.9/min)
- warm-up: 324 frames over 9.2 s, worst 2634.8 ms
- hitches (> 29.4 ms): **31** measured, 32 in warm-up
- timed GPU passes: 7.07 ms of a 12.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.75 ms, limit 11 ms
- **FAIL** frame_ms: 60 stutters (> 1.5x moving median of 60) in 1.09 min = 54.84/min, limit 1/min
- **FAIL** worst frame 62.72 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.009 | 2.037 | 2.072 | 0.99 | 0.178 |
| main_opaque_pass_3d | 1.589 | 1.611 | 2.010 | 0.99 | 0.076 |
| early prepass | 0.460 | 0.466 | 0.678 | 0.99 | 0.089 |
| bloom | 0.311 | 0.315 | 0.322 | 0.99 | 0.062 |
| atmosphere_luts | 0.300 | 0.304 | 0.367 | 0.99 | 0.003 |
| water_velocity | 0.272 | 0.276 | 0.347 | 0.99 | 0.002 |
| water_sspr | 0.199 | 0.202 | 0.203 | 0.99 | 0.003 |
| ssao | 0.186 | 0.188 | 0.193 | 0.99 | 0.007 |
| atmos_froxel_inject | 0.144 | 0.146 | 0.147 | 0.99 | 0.001 |
| clouds_trace | 0.139 | 0.141 | 0.227 | 0.99 | 0.001 |
| water_fft | 0.131 | 0.132 | 0.140 | 0.99 | 0.003 |
| uniform_allocation | 0.129 | 0.131 | 0.179 | 0.99 | 0.029 |
| auxiliary_views | 0.129 | 0.130 | 0.531 | 0.99 | 0.151 |
| upscaling | 0.117 | 0.118 | 0.127 | 0.99 | 0.004 |
| bin_unpacking | 0.106 | 0.107 | 0.142 | 0.99 | 0.020 |
| clouds_composite | 0.104 | 0.106 | 0.109 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.084 | 0.085 | 0.178 | 0.99 | 0.009 |
| tonemapping | 0.084 | 0.085 | 0.088 | 0.99 | 0.003 |
| clustering | 0.063 | 0.064 | 0.067 | 0.99 | 0.019 |
| render_sky | 0.051 | 0.052 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.412 | 0.416 | 0.12 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.049 | 0.99 | 0.001 |
| surface_exposure | 0.043 | 0.044 | 0.045 | 0.99 | 0.001 |
| surface_trail | 0.042 | 0.056 | 0.057 | 0.75 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.039 | 0.061 | 0.99 | 0.005 |
| surface_exposure_far | 0.036 | 0.042 | 0.045 | 0.86 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.229 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.028 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.014 | 0.014 | 0.033 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.024 | 0.99 | 0.124 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.022 | 0.99 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| main/busy/ms | 10.575 | 12.923 | 28.23 | 56979.6 |
| render/thread/ms | 10.070 | 12.400 | 54.35 | 54255.0 |
| render/busy/ms | 9.532 | 11.883 | 53.75 | 51358.8 |
| stage/main/post_update | 7.043 | 8.950 | 13.35 | 37950.1 |
| stage/render/render | 5.750 | 7.253 | 37.22 | 30980.8 |
| render/graph/ms | 5.028 | 6.488 | 36.49 | 27090.3 |
| encode//sum | 4.028 | 5.188 | 19.87 | 21700.7 |
| stage/render/graph_submit | 3.801 | 4.937 | 35.21 | 20482.3 |
| encode/submit | 2.420 | 3.259 | 34.07 | 13037.6 |
| stage/main/update | 2.216 | 2.724 | 19.61 | 11942.0 |
| encode/finish_wall | 1.420 | 2.106 | 4.67 | 7653.5 |
| stage/render/camera_driver | 1.168 | 1.598 | 4.04 | 6295.7 |
| encode/pass:shadow_point_light/sum | 0.898 | 3.248 | 16.39 | 4840.7 |
| encode/pass:shadow_directional_light/sum | 0.891 | 1.589 | 2.74 | 4798.6 |
| stage/render/prepare_write_phase_buffers | 0.822 | 1.192 | 2.93 | 4431.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.799 | 1.131 | 1.68 | 4302.4 |
| encode/pass:early prepass/sum | 0.778 | 1.113 | 1.96 | 4190.8 |
| stage/render/prepare_assets | 0.604 | 0.775 | 11.36 | 3252.4 |
| bigspace/total/ms | 0.574 | 1.374 | 2.33 | 3091.0 |
| stage/render/prepare_meshes | 0.539 | 0.683 | 8.58 | 2906.5 |
| stage/render/prepare_resources | 0.460 | 0.568 | 1.12 | 2481.2 |
| render/present/ms | 0.441 | 0.581 | 2.24 | 2374.3 |
| stage/main/first | 0.404 | 0.532 | 5.15 | 2174.6 |
| stage/render/specialize | 0.399 | 0.450 | 19.57 | 2149.2 |
| stage/main/pre_update | 0.368 | 0.523 | 1.58 | 1983.1 |
| stage/render/queue | 0.366 | 0.520 | 13.46 | 1969.6 |
| terrain/update/ms | 0.310 | 0.403 | 1.93 | 1669.7 |
| stage/render/prepare_bind_groups | 0.293 | 0.526 | 2.30 | 1579.7 |
| controller/feed/ms | 0.277 | 0.342 | 1.02 | 1491.2 |
| terrain/select/ms | 0.276 | 0.361 | 0.87 | 1485.2 |
| stream/drive/ms | 0.248 | 0.335 | 17.47 | 1333.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.246 | 0.362 | 0.92 | 1326.3 |
| stage/main/fixed_main_loop | 0.240 | 0.383 | 1.24 | 1292.5 |
| bigspace/low_precision/ms | 0.222 | 0.714 | 2.08 | 1197.8 |
| stage/render/create_views | 0.208 | 0.273 | 7.96 | 1120.0 |
| far/select/ms | 0.197 | 0.269 | 0.83 | 1059.4 |
| stage/render/prepare_batch_phases | 0.192 | 0.277 | 0.68 | 1036.9 |
| stream/tick/ms | 0.185 | 0.252 | 0.70 | 997.5 |
| stage/render/extract_commands | 0.180 | 0.209 | 0.60 | 970.4 |
| stage/render/prepare_flush | 0.169 | 0.375 | 0.91 | 909.9 |
| anim/evaluate/ms | 0.158 | 0.260 | 2.18 | 850.7 |
| anim/evaluate_cpu/ms | 0.153 | 0.194 | 0.68 | 824.8 |
| bigspace/high_precision/ms | 0.150 | 0.633 | 1.75 | 806.6 |
| stage/render/prepare_views | 0.140 | 0.178 | 0.56 | 754.7 |
| controller/move/ms | 0.135 | 0.315 | 0.75 | 727.1 |
| stage/main/last | 0.123 | 0.165 | 0.69 | 664.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.105 | 0.172 | 0.76 | 566.1 |
| render/acquire/ms | 0.097 | 0.125 | 0.51 | 521.9 |
| stage/render/prepare_collect_phase_buffers | 0.069 | 0.142 | 0.52 | 371.6 |
| stage/render/cleanup | 0.059 | 0.144 | 0.55 | 317.0 |
| pipelines/compile_ms | 0.057 | 0.000 | 127.77 | 304.7 |
| pipelines/front_ms | 0.054 | 0.000 | 123.92 | 292.2 |
| physics/step/ms | 0.052 | 0.090 | 0.41 | 280.8 |
| physics/insert/ms | 0.039 | 0.404 | 0.97 | 209.3 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.67 | 174.3 |
| stage/render/phase_sort | 0.029 | 0.049 | 0.44 | 157.2 |
| weather/map/ms | 0.025 | 0.033 | 0.25 | 133.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.023 | 0.40 | 83.0 |
| anim/apply/ms | 0.012 | 0.022 | 0.17 | 64.7 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.74 | 63.1 |
| controller/avoid/ms | 0.010 | 0.014 | 0.25 | 55.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.05 | 55.2 |
| script/update/ms | 0.008 | 0.009 | 0.30 | 40.8 |
| stream/despawn/ms | 0.006 | 0.000 | 1.00 | 31.3 |
| stream/spawn/ms | 0.006 | 0.000 | 1.17 | 29.7 |
| script/input/ms | 0.005 | 0.007 | 0.21 | 29.3 |
| script/ui/ms | 0.004 | 0.005 | 0.19 | 22.7 |
| controller/write/ms | 0.003 | 0.005 | 0.20 | 18.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 14.1 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.62 | 12.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.29 | 5.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.39 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.23 | 1.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1269 | 2262 | 593 |
| crowd/c0/meshes/count | count | 353 | 646 | 156 |
| crowd/c1/count | count | 146 | 157 | 148 |
| crowd/c1/joints/count | count | 5524 | 6037 | 5603 |
| crowd/c1/meshes/count | count | 1157 | 1311 | 1185 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4599 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 83.3 k | 91.8 k | 84.3 k |
| encode//max | ms | 0.42 | 1.41 | 0.52 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.40 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.74 | 0.01 |
| encode/finish_critical | ms | 1.00 | 2.74 | 1.09 |
| encode/pass:early prepass/max | ms | 0.80 | 1.96 | 0.84 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 1.68 | 1.09 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.76 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.92 | 0.32 |
| encode/pass:shadow_directional_light/max | ms | 0.84 | 2.74 | 0.85 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.58 | 0.83 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.67 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 10.72 | 53.75 | 8.84 |
| gpu/timed/ms | ms | 7.88 | 8.78 | 8.18 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.6 k | 12.8 k | 11.5 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1367 | 1534 | 1424 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 5274 | 7382 | 5165 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1367 | 1534 | 1424 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 5274 | 7382 | 5165 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 18 | 207 | 4 |
| phases/shadow/bins | count | 4442 | 7957 | 2604 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 16.8 k | 44.6 k | 12.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 244 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 219 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 571 | 6 |
| preprocess/bind_groups_created | count | 14 | 571 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 213 | 123 |
| process/cpu_usage | % | 27.40 | 40.38 | 29.73 |
| process/mem_usage | GiB | 5.43 | 5.60 | 5.60 |
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
| skin/blocks_changed/count | count | 14 | 121 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 201 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.1 k | 810.5 k | 492.5 k |
| skin/joints_written/count | count | 5720 | 12.7 k | 7696 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 5218 | 5306 | 5306 |
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
| stream/latency/s | s | 0.04 | 0.13 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 742 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.94 | 39.25 | 34.05 |
| system/mem_usage | % | 67.13 | 67.60 | 67.55 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 68 | 256 | 54 |
| views/all/bins | count | 7193 | 10.8 k | 6246 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 28.9 k | 59.3 k | 23.6 k |
| views/all/pipelines | count | 249 | 289 | 74 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 14.7 k | 57.2 k | 12.1 k |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2933 | 5979 | 2604 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 15.0 k | 42.0 k | 12.0 k |
| views/cascade/pipelines | count | 36 | 53 | 28 |
| views/cascade/work_items | count | 697 | 39.9 k | 182 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 1448 | 2143 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1870 | 2820 | 0 |
| views/lamp_face/pipelines | count | 166 | 199 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2196 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 3149 | 3712 | 3642 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 12.0 k | 17.5 k | 11.6 k |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 12.2 k | 17.7 k | 11.9 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 55.0 k | 61.4 k | 55.9 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 51.31 | 253 | 276.5 k |
| anim/events/count | count | 0.10 | 3 | 557 |
| assets/Image/added | count | 0.11 | 43 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.52 | 189 | 2822 |
| assets/Mesh/removed | count | 0.88 | 1146 | 4740 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.98 | 11 | 21.5 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.73 | 1303 | 9344 |
| encode//count | count | 33.65 | 220 | 181.3 k |
| encode/bind_walk/init_textures | count | 12.6 k | 114.7 k | 67.95 M |
| encode/bind_walk/init_textures_skipped | count | 65.3 k | 138.5 k | 351.80 M |
| encode/bind_walk/inits | count | 462 | 1130 | 2.49 M |
| encode/bind_walk/inits_skipped | count | 110 | 157 | 591.3 k |
| encode/bind_walk/merge_textures | count | 12.5 k | 114.6 k | 67.56 M |
| encode/bind_walk/merge_textures_skipped | count | 5378 | 6012 | 28.97 M |
| encode/bind_walk/merges | count | 98.96 | 335 | 533.2 k |
| encode/bind_walk/merges_skipped | count | 85.49 | 129 | 460.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5388 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5388 |
| encode/pass:early prepass/count | count | 1 | 1 | 5388 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5388 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5388 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5388 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5388 |
| encode/pass:shadow_point_light/count | count | 1.41 | 48 | 7599 |
| encode/queue/command_buffers | count | 45.07 | 278 | 242.9 k |
| encode/queue/submits | count | 2.02 | 4 | 10.9 k |
| encode/queue/write_buffer/bytes | bytes | 1.95 M | 6.63 M | 10.50 G |
| encode/queue/write_buffer/calls | count | 260 | 854 | 1.40 M |
| encode/queue/write_buffer_with/bytes | bytes | 172.9 k | 1.30 M | 931.74 M |
| encode/queue/write_buffer_with/calls | count | 10.55 | 576 | 56.9 k |
| encode/queue/write_texture/bytes | bytes | 13.2 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5388 |
| physics/inserts | count | 0.43 | 105 | 2308 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 161 | 2335 |
| physics/steps | count | 0.73 | 3 | 3934 |
| physics/steps_dropped | count | 0.00 | 1 | 4 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 16 | 351 |
| stream/upload/bytes | bytes | 6283 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1377 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.4 ms; slowest 63, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.63 | 2634.8 | main/busy/ms 2604, stage/main/post_update 2496, stage/main/update 99.62, surface/advance_cpu_ms 68.42, stage/main/pre_update 4.64, stage/main/last 1.28, surface/cell_upload_ms 0.99, stream/drive/ms 0.52, stage/main/first 0.48, stage/main/fixed_main_loop 0.34, stream/tick/ms 0.15, weather/map/ms 0.13 |
| warm-up | 3.35 | 712.3 | main/busy/ms 2784, stage/main/post_update 2519, render/thread/ms 679, render/busy/ms 566, stage/render/prepare_assets 537, stage/main/update 251, stage/render/prepare_views 110, render/acquire/ms 109, surface/advance_cpu_ms 68.42, stage/render/render 12.34, stage/render/create_views 9.48, render/graph/ms 7.30 |
| warm-up | 3.46 | 111.8 | render/thread/ms 773, render/busy/ms 608, stage/render/prepare_assets 564, main/busy/ms 203, stage/main/update 156, pipelines/compile_ms 120, stage/render/prepare_views 110, render/acquire/ms 110, pipelines/front_ms 84.13, stage/render/render 68.98, render/present/ms 54.97, stage/main/post_update 36.33 |
| warm-up | 3.51 | 51.4 | pipelines/wait_ms 543, pipelines/compile_ms 506, pipelines/front_ms 435, render/thread/ms 144, stage/render/render 98.97, render/busy/ms 89.71, main/busy/ms 71.95, pipelines/driver_ms 71.02, stage/main/post_update 53.52, render/present/ms 52.81, render/graph/ms 45.09, stage/render/prepare_assets 28.31 |
| warm-up | 3.64 | 132.3 | pipelines/wait_ms 5028, pipelines/compile_ms 1546, pipelines/front_ms 1444, render/thread/ms 167, render/busy/ms 163, pipelines/driver_ms 101, main/busy/ms 81.06, stage/main/post_update 54.26, stage/render/prepare_assets 53.46, stage/render/render 49.91, pipelines/lock_ms 49.70, render/graph/ms 46.48 |
| warm-up | 3.69 | 51.4 | pipelines/wait_ms 4775, pipelines/compile_ms 2121, pipelines/front_ms 2002, render/thread/ms 160, render/busy/ms 158, pipelines/driver_ms 118, main/busy/ms 104, stage/render/prepare_assets 74.04, stage/main/update 58.28, stage/render/prepare_resources 34.57, stage/main/post_update 26.39, pipelines/lock_ms 25.56 |
| warm-up | 3.81 | 111.8 | pipelines/compile_ms 1507, pipelines/front_ms 1430, pipelines/wait_ms 338, render/thread/ms 121, render/busy/ms 119, main/busy/ms 90.39, stage/render/prepare_assets 81.37, pipelines/driver_ms 76.89, stage/main/update 61.79, stage/main/post_update 20.89, stage/render/render 19.61, terrain/update/ms 18.43 |
| warm-up | 3.86 | 51.5 | pipelines/compile_ms 1611, pipelines/front_ms 1555, pipelines/wait_ms 197, render/thread/ms 128, render/busy/ms 125, stage/render/prepare_assets 91.97, pipelines/driver_ms 56.21, main/busy/ms 33.91, stage/render/render 27.62, render/graph/ms 25.21, stage/render/graph_submit 22.25, stage/main/post_update 16.97 |
| warm-up | 4.02 | 42.9 | pipelines/compile_ms 631, pipelines/front_ms 606, main/busy/ms 51.42, stage/main/update 42.86, stream/drive/ms 39.22, render/thread/ms 25.01, pipelines/driver_ms 24.99, render/busy/ms 23.64, stage/render/prepare_assets 10.43, stage/render/render 8.52, render/graph/ms 6.76, encode//sum 5.64 |
| warm-up | 4.11 | 30.0 | pipelines/compile_ms 134, pipelines/front_ms 128, render/thread/ms 36.61, render/busy/ms 34.70, main/busy/ms 22.47, stage/render/prepare_assets 18.20, stage/main/post_update 10.91, stage/render/render 9.55, render/graph/ms 7.49, stage/main/update 6.70, pipelines/driver_ms 5.77, stage/render/graph_submit 5.07 |
| warm-up | 4.26 | 35.4 | pipelines/compile_ms 211, pipelines/front_ms 206, render/thread/ms 39.66, render/busy/ms 37.55, main/busy/ms 32.58, stage/main/update 18.16, stage/render/render 13.36, render/graph/ms 10.68, stage/main/post_update 8.73, stage/render/prepare_assets 7.95, stage/render/graph_submit 7.67, encode//sum 6.20 |
| warm-up | 4.30 | 46.0 | pipelines/compile_ms 1427, pipelines/front_ms 1383, render/thread/ms 68.00, render/busy/ms 65.82, main/busy/ms 64.12, pipelines/wait_ms 55.18, stage/main/update 48.84, pipelines/driver_ms 44.32, stage/render/render 24.72, render/graph/ms 21.89, stage/render/graph_submit 14.20, encode//sum 14.15 |
| warm-up | 4.36 | 55.8 | pipelines/compile_ms 1294, pipelines/front_ms 1253, render/thread/ms 78.84, pipelines/wait_ms 77.56, render/busy/ms 76.76, main/busy/ms 65.49, stage/main/update 49.04, pipelines/driver_ms 40.96, stage/render/render 27.64, render/graph/ms 24.85, stage/render/prepare_assets 19.48, encode//sum 17.65 |
| warm-up | 4.41 | 49.8 | pipelines/compile_ms 183, pipelines/front_ms 177, render/thread/ms 82.38, render/busy/ms 80.60, pipelines/wait_ms 59.41, encode//sum 56.04, main/busy/ms 33.07, stage/render/render 26.14, render/graph/ms 23.78, stage/render/prepare_assets 20.38, stage/main/update 19.90, stage/render/graph_submit 17.33 |
| warm-up | 4.47 | 64.6 | pipelines/compile_ms 1585, pipelines/front_ms 1530, pipelines/wait_ms 153, render/thread/ms 105, render/busy/ms 102, encode//sum 77.31, pipelines/driver_ms 54.35, stage/render/render 45.15, render/graph/ms 42.21, main/busy/ms 30.96, stage/render/graph_submit 23.77, stage/main/update 18.52 |
| warm-up | 4.70 | 33.6 | pipelines/compile_ms 630, pipelines/front_ms 610, render/thread/ms 55.41, render/busy/ms 53.51, main/busy/ms 30.66, stage/render/prepare_resources 19.55, pipelines/driver_ms 19.54, stage/render/render 17.44, stage/main/post_update 16.70, render/graph/ms 12.18, stage/render/graph_submit 8.97, stage/main/update 8.45 |
| warm-up | 4.75 | 38.7 | pipelines/compile_ms 410, pipelines/front_ms 401, render/thread/ms 49.95, render/busy/ms 48.29, main/busy/ms 27.88, stage/render/render 21.83, render/graph/ms 19.81, stage/render/camera_driver 13.57, stage/main/update 10.63, stage/render/prepare_assets 10.57, pipelines/driver_ms 8.94, stage/main/post_update 8.92 |
| warm-up | 4.84 | 41.7 | pipelines/compile_ms 397, pipelines/front_ms 385, render/thread/ms 56.33, render/busy/ms 54.88, main/busy/ms 41.13, stage/main/update 19.63, stage/render/prepare_resources 17.64, stage/render/render 15.38, stage/main/post_update 13.83, render/graph/ms 13.31, stream/drive/ms 12.77, pipelines/driver_ms 12.69 |
| warm-up | 4.92 | 48.5 | pipelines/compile_ms 1501, pipelines/front_ms 1462, main/busy/ms 60.38, render/thread/ms 52.34, render/busy/ms 50.66, pipelines/driver_ms 38.58, stage/main/update 37.24, stage/render/prepare_assets 17.49, stage/render/render 14.85, stage/main/post_update 14.01, render/graph/ms 12.43, encode//sum 9.39 |
| warm-up | 4.96 | 48.9 | pipelines/compile_ms 1293, pipelines/front_ms 1265, render/thread/ms 62.02, render/busy/ms 60.68, main/busy/ms 38.08, stage/render/prepare_assets 30.97, pipelines/driver_ms 28.39, stage/main/update 21.96, stage/render/render 10.61, encode//sum 8.84, stage/main/post_update 8.61, render/graph/ms 8.59 |
| warm-up | 5.18 | 56.2 | pipelines/compile_ms 220, pipelines/front_ms 212, main/busy/ms 58.16, render/thread/ms 46.74, render/busy/ms 45.03, stage/main/post_update 32.63, stage/main/update 19.30, stage/render/render 17.39, render/graph/ms 14.77, stage/render/prepare_assets 13.26, encode//sum 10.71, stream/drive/ms 9.85 |
| warm-up | 5.23 | 52.6 | pipelines/compile_ms 1066, pipelines/front_ms 1034, render/thread/ms 60.68, render/busy/ms 59.19, stage/render/prepare_assets 36.27, main/busy/ms 34.15, pipelines/driver_ms 32.70, stage/main/post_update 13.80, stage/render/render 12.78, stage/main/update 12.19, render/graph/ms 10.36, pipelines/wait_ms 10.36 |
| warm-up | 5.26 | 29.7 | pipelines/compile_ms 846, pipelines/front_ms 822, render/thread/ms 60.36, render/busy/ms 58.74, main/busy/ms 41.84, stage/render/prepare_assets 30.10, pipelines/driver_ms 24.63, stage/main/post_update 15.98, stage/main/update 15.91, stage/render/render 13.07, render/graph/ms 10.78, encode//sum 9.05 |
| warm-up | 5.32 | 54.1 | pipelines/compile_ms 688, pipelines/front_ms 669, render/thread/ms 74.07, render/busy/ms 72.22, main/busy/ms 39.80, stage/render/prepare_assets 33.26, stage/main/post_update 18.49, pipelines/driver_ms 18.46, stage/main/update 14.13, stage/render/render 13.14, render/graph/ms 10.65, encode//sum 8.84 |
| warm-up | 5.39 | 48.7 | pipelines/compile_ms 470, pipelines/front_ms 456, render/thread/ms 46.21, render/busy/ms 44.86, main/busy/ms 30.38, stage/render/prepare_assets 24.45, pipelines/driver_ms 13.91, stage/main/update 12.27, stage/main/post_update 10.93, stage/render/render 10.71, pipelines/wait_ms 8.90, pipelines/lock_ms 8.81 |
| warm-up | 5.61 | 36.1 | pipelines/compile_ms 202, pipelines/front_ms 198, main/busy/ms 36.95, render/thread/ms 35.21, render/busy/ms 33.61, stage/main/post_update 16.61, stage/render/render 14.49, stage/main/update 13.66, render/graph/ms 11.93, encode//sum 10.46, stage/render/graph_submit 8.70, stage/render/prepare_assets 7.52 |
| warm-up | 5.70 | 33.7 | pipelines/compile_ms 192, pipelines/front_ms 187, main/busy/ms 44.43, stage/main/update 32.87, stream/drive/ms 29.04, render/thread/ms 22.50, render/busy/ms 21.30, stage/render/render 8.79, encode//sum 7.69, stage/main/post_update 7.03, render/graph/ms 7.00, stage/render/prepare_assets 5.20 |
| warm-up | 5.78 | 46.4 | pipelines/compile_ms 150, pipelines/front_ms 145, main/busy/ms 46.81, stage/main/update 36.59, stream/drive/ms 32.99, render/thread/ms 24.96, render/busy/ms 23.79, stage/render/prepare_assets 9.35, stage/render/render 8.51, stage/main/post_update 7.57, encode//sum 7.26, render/graph/ms 6.96 |
| warm-up | 5.85 | 31.1 | pipelines/compile_ms 284, pipelines/front_ms 274, main/busy/ms 39.70, render/thread/ms 29.02, stage/main/update 28.15, render/busy/ms 27.60, stream/drive/ms 24.35, stage/render/prepare_assets 11.41, stage/render/render 10.57, pipelines/driver_ms 9.56, encode//sum 8.85, render/graph/ms 8.29 |
| warm-up | 5.90 | 33.7 | main/busy/ms 36.92, render/thread/ms 30.02, render/busy/ms 28.85, stage/main/update 27.16, stream/drive/ms 19.54, stage/render/render 12.19, encode//sum 10.52, render/graph/ms 10.28, stage/render/graph_submit 7.53, stage/main/post_update 6.40, stage/render/prepare_assets 6.16, encode/submit 4.17 |
| warm-up | 5.96 | 41.2 | pipelines/compile_ms 165, pipelines/front_ms 158, main/busy/ms 44.91, stage/main/update 32.87, render/thread/ms 24.11, stream/drive/ms 23.43, render/busy/ms 22.74, stage/render/render 10.45, render/graph/ms 8.54, stage/main/post_update 8.01, encode//sum 7.38, pipelines/driver_ms 6.56 |
| warm-up | 8.18 | 39.1 | render/thread/ms 47.56, render/busy/ms 46.61, main/busy/ms 21.31, stage/render/specialize 15.02, stage/main/post_update 11.53, stage/render/render 10.22, stage/render/queue 9.31, render/graph/ms 8.89, stage/render/graph_submit 6.82, encode//sum 6.71, stage/main/update 6.13, stage/render/prepare_meshes 4.74 |
| measure | 0.45 | 43.0 | render/thread/ms 49.87, render/busy/ms 48.58, main/busy/ms 19.70, stage/render/specialize 16.19, stage/main/post_update 12.25, stage/render/render 11.63, stage/render/queue 10.62, render/graph/ms 10.18, encode//sum 7.96, stage/render/graph_submit 7.60, encode/submit 4.57, stage/main/update 4.57 |
| measure | 0.93 | 42.5 | render/thread/ms 50.49, render/busy/ms 49.51, stage/render/render 43.01, render/graph/ms 41.65, stage/render/graph_submit 39.06, encode/submit 36.41, main/busy/ms 19.68, stage/main/post_update 13.08, encode//sum 7.73, stage/main/update 4.28, encode/pass:shadow_point_light/sum 4.12, encode/finish_wall 2.68 |
| measure | 1.86 | 46.4 | render/thread/ms 50.05, render/busy/ms 48.94, main/busy/ms 19.65, stage/render/specialize 16.67, stage/main/post_update 12.78, stage/render/queue 10.85, stage/render/render 10.19, render/graph/ms 8.72, encode//sum 7.27, stage/render/graph_submit 6.50, stage/render/prepare_meshes 5.79, stage/main/update 4.49 |
| measure | 2.73 | 31.2 | render/thread/ms 40.36, render/busy/ms 39.15, encode//sum 24.22, main/busy/ms 21.31, stage/render/render 21.08, render/graph/ms 19.21, encode/pass:shadow_point_light/sum 18.03, stage/main/post_update 14.05, stage/render/graph_submit 13.63, encode/submit 8.41, stage/render/create_views 8.13, stage/render/camera_driver 5.46 |
| measure | 3.39 | 47.4 | render/thread/ms 55.78, render/busy/ms 54.64, main/busy/ms 23.18, stage/render/specialize 17.33, stage/main/post_update 14.03, stage/render/render 13.46, render/graph/ms 11.96, stage/render/queue 11.82, encode//sum 9.08, stage/render/graph_submit 8.91, encode/pass:shadow_point_light/sum 5.87, stage/main/update 5.77 |
| measure | 4.95 | 49.3 | render/thread/ms 52.33, render/busy/ms 51.25, main/busy/ms 24.46, stage/render/specialize 17.80, stage/main/post_update 15.03, stage/render/queue 11.63, stage/render/render 10.83, render/graph/ms 9.38, stage/render/graph_submit 7.24, encode//sum 7.14, stage/render/prepare_meshes 5.94, stage/main/update 5.45 |
| measure | 6.41 | 46.5 | render/thread/ms 52.43, render/busy/ms 51.38, main/busy/ms 23.51, stage/render/specialize 18.02, stage/main/post_update 15.37, stage/render/render 11.37, stage/render/queue 11.20, render/graph/ms 9.90, encode//sum 8.08, stage/render/graph_submit 7.57, stage/render/prepare_meshes 5.83, stage/main/update 5.20 |
| measure | 7.88 | 47.4 | render/thread/ms 51.68, render/busy/ms 50.48, main/busy/ms 21.61, stage/render/specialize 17.40, stage/main/post_update 13.96, stage/render/queue 11.79, stage/render/render 10.04, render/graph/ms 8.52, encode//sum 6.53, stage/render/prepare_meshes 6.46, stage/render/graph_submit 6.34, stage/main/update 4.14 |
| measure | 9.39 | 44.9 | render/thread/ms 53.50, render/busy/ms 52.55, main/busy/ms 21.77, stage/render/specialize 17.46, stage/main/post_update 14.28, stage/render/render 12.03, stage/render/queue 11.76, render/graph/ms 10.66, encode//sum 8.20, stage/render/graph_submit 8.13, stage/render/prepare_meshes 5.87, stage/main/update 4.86 |
| measure | 10.96 | 48.5 | render/thread/ms 52.40, render/busy/ms 51.39, main/busy/ms 23.08, stage/render/specialize 17.37, stage/main/post_update 15.69, stage/render/queue 11.90, stage/render/render 10.59, render/graph/ms 9.20, encode//sum 7.47, stage/render/graph_submit 6.65, stage/render/prepare_meshes 6.24, stage/main/update 4.35 |
| measure | 15.76 | 51.5 | render/thread/ms 56.31, render/busy/ms 55.20, main/busy/ms 22.79, stage/render/specialize 18.66, stage/main/post_update 15.32, stage/render/queue 12.85, stage/render/render 11.20, render/graph/ms 9.72, encode//sum 7.51, stage/render/graph_submit 7.34, stage/render/prepare_meshes 6.67, stage/main/update 4.60 |
| measure | 17.31 | 47.3 | render/thread/ms 52.51, render/busy/ms 51.39, main/busy/ms 21.92, stage/render/specialize 17.17, stage/main/post_update 15.17, stage/render/queue 11.56, stage/render/render 10.58, render/graph/ms 9.06, encode//sum 7.68, stage/render/graph_submit 6.68, stage/render/prepare_meshes 6.65, encode/submit 4.10 |
| measure | 18.03 | 49.8 | render/thread/ms 55.38, render/busy/ms 54.31, main/busy/ms 23.11, stage/render/specialize 17.76, stage/main/post_update 15.23, stage/render/queue 12.61, stage/render/render 11.36, render/graph/ms 9.86, encode//sum 7.71, stage/render/graph_submit 7.41, stage/render/prepare_meshes 6.31, encode/submit 4.41 |
| measure | 18.17 | 31.3 | main/busy/ms 39.48, render/thread/ms 23.92, render/busy/ms 22.98, stage/main/update 21.70, stream/drive/ms 17.73, stage/main/post_update 14.82, stage/render/render 12.41, render/graph/ms 10.91, encode//sum 8.42, stage/render/graph_submit 8.04, encode/submit 4.56, encode/finish_wall 3.54 |
| measure | 19.67 | 50.3 | render/thread/ms 53.97, render/busy/ms 52.90, main/busy/ms 23.30, stage/render/specialize 17.90, stage/main/post_update 15.17, stage/render/queue 11.91, stage/render/render 10.58, render/graph/ms 9.16, encode//sum 7.65, stage/render/graph_submit 6.99, stage/render/prepare_meshes 6.80, stage/main/update 5.24 |
| measure | 21.27 | 52.0 | render/thread/ms 55.79, render/busy/ms 54.72, main/busy/ms 23.39, stage/render/specialize 17.15, stage/main/post_update 15.99, stage/render/queue 12.12, stage/render/render 12.03, render/graph/ms 10.27, encode//sum 8.95, stage/render/graph_submit 7.53, stage/render/prepare_meshes 7.32, encode/submit 4.59 |
| measure | 23.08 | 56.0 | render/thread/ms 62.67, render/busy/ms 61.44, main/busy/ms 30.37, stage/render/specialize 19.04, stage/main/post_update 18.14, stage/render/render 14.49, stage/render/queue 13.89, render/graph/ms 12.97, encode//sum 10.05, stage/render/graph_submit 9.52, stage/render/prepare_meshes 7.77, stage/main/update 6.25 |
| measure | 24.83 | 49.0 | render/thread/ms 56.24, render/busy/ms 55.14, main/busy/ms 24.33, stage/render/specialize 17.15, stage/main/post_update 15.76, stage/render/render 13.12, stage/render/queue 12.01, render/graph/ms 11.62, stage/render/graph_submit 8.77, encode//sum 8.64, stage/render/prepare_meshes 6.95, stage/main/update 5.57 |
| measure | 27.32 | 59.5 | render/thread/ms 66.72, render/busy/ms 65.59, main/busy/ms 29.00, stage/render/specialize 19.96, stage/main/post_update 17.05, stage/render/render 16.63, render/graph/ms 15.01, stage/render/queue 13.56, encode//sum 12.19, stage/render/graph_submit 11.84, encode/pass:shadow_point_light/sum 8.71, stage/render/prepare_meshes 8.23 |
| measure | 29.02 | 51.8 | render/thread/ms 56.73, render/busy/ms 55.59, main/busy/ms 26.20, stage/render/specialize 17.90, stage/main/post_update 17.19, stage/render/queue 12.95, stage/render/render 11.02, render/graph/ms 9.47, encode//sum 8.16, stage/render/prepare_meshes 7.75, stage/render/graph_submit 7.29, stage/main/update 5.88 |
| measure | 30.73 | 52.4 | render/thread/ms 58.33, render/busy/ms 57.13, main/busy/ms 24.62, stage/render/specialize 17.64, stage/main/post_update 16.95, stage/render/render 13.48, stage/render/queue 12.18, encode//sum 12.18, render/graph/ms 12.02, stage/render/graph_submit 9.20, stage/render/prepare_meshes 7.38, encode/pass:shadow_point_light/sum 7.07 |
| measure | 32.45 | 52.8 | render/thread/ms 54.47, render/busy/ms 53.28, main/busy/ms 24.38, stage/render/specialize 17.76, stage/main/post_update 16.60, stage/render/queue 12.21, stage/render/render 10.62, render/graph/ms 9.04, encode//sum 7.84, stage/render/prepare_meshes 7.07, stage/render/graph_submit 6.98, stage/main/update 4.71 |
| measure | 35.38 | 54.0 | render/thread/ms 59.60, render/busy/ms 58.49, main/busy/ms 25.50, stage/render/specialize 20.08, stage/main/post_update 17.28, stage/render/queue 12.71, stage/render/render 12.70, render/graph/ms 11.06, encode//sum 9.28, stage/render/graph_submit 8.50, stage/render/prepare_meshes 6.72, encode/submit 4.79 |
| measure | 37.03 | 51.3 | render/thread/ms 58.39, render/busy/ms 57.26, main/busy/ms 24.77, stage/main/post_update 17.02, stage/render/specialize 16.64, stage/render/render 13.29, stage/render/queue 11.77, render/graph/ms 11.70, encode//sum 10.00, stage/render/prepare_meshes 9.05, stage/render/graph_submit 8.96, encode/pass:shadow_point_light/sum 5.98 |
| measure | 37.50 | 49.5 | render/thread/ms 55.34, render/busy/ms 54.25, main/busy/ms 25.27, stage/render/specialize 17.63, stage/main/post_update 17.61, stage/render/render 13.00, stage/render/queue 12.20, render/graph/ms 11.46, encode//sum 10.47, stage/render/graph_submit 8.83, encode/submit 5.45, stage/render/prepare_meshes 5.25 |
| measure | 40.83 | 46.2 | render/thread/ms 52.88, render/busy/ms 51.56, main/busy/ms 23.02, stage/render/specialize 16.33, stage/main/post_update 15.09, stage/render/queue 11.49, stage/render/render 11.24, render/graph/ms 9.59, encode//sum 7.92, stage/render/graph_submit 7.64, stage/render/prepare_meshes 5.11, stage/main/update 4.67 |
| measure | 43.23 | 49.9 | render/thread/ms 54.19, render/busy/ms 53.19, main/busy/ms 21.45, stage/render/specialize 17.65, stage/main/post_update 14.24, stage/render/queue 12.77, stage/render/render 11.35, render/graph/ms 10.01, encode//sum 8.08, stage/render/graph_submit 8.07, stage/render/prepare_meshes 5.44, encode/submit 4.26 |
| measure | 45.68 | 50.7 | render/thread/ms 54.74, render/busy/ms 53.59, main/busy/ms 21.75, stage/render/specialize 18.64, stage/main/post_update 13.24, stage/render/queue 11.52, stage/render/render 11.24, render/graph/ms 9.67, encode//sum 7.91, stage/render/graph_submit 7.52, stage/render/prepare_meshes 5.75, stage/main/update 5.01 |
| measure | 56.45 | 42.2 | render/thread/ms 50.03, render/busy/ms 48.99, main/busy/ms 17.26, stage/render/specialize 16.05, stage/render/queue 10.87, stage/render/render 10.70, stage/main/post_update 10.32, render/graph/ms 9.45, encode//sum 7.64, stage/render/graph_submit 7.42, stage/render/prepare_meshes 5.45, stage/main/update 4.53 |
| measure | 59.05 | 62.7 | render/thread/ms 73.43, render/busy/ms 72.19, main/busy/ms 33.76, stage/main/post_update 20.56, stage/render/prepare_assets 17.63, stage/render/specialize 16.70, stage/render/render 13.89, render/graph/ms 12.21, stage/render/queue 11.49, encode//sum 11.39, stage/render/graph_submit 8.66, stage/main/update 7.41 |
| measure | 62.60 | 46.4 | render/thread/ms 50.60, render/busy/ms 49.66, main/busy/ms 22.08, stage/render/specialize 16.69, stage/main/post_update 12.64, stage/render/queue 11.14, stage/render/render 10.78, render/graph/ms 9.58, encode//sum 7.80, stage/render/graph_submit 7.41, stage/main/update 6.39, stage/render/prepare_meshes 6.25 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=noprecomb-1 --bench-baseline none --off precombine
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 1%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **69%** (min 31%) · 2622 MHz (min 2610) · memory 8101 MHz · 74.3 W · 76 °C (max 83) · VRAM 6753 MiB · throttled 100% of samples

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
