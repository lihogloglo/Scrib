# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5920 frames over 65.6 s · **median 10.84 ms (92 fps)** · mean 11.09 · p95 13.69 · p99 17.36 (1% low 58 fps) · worst 44.22 ms · 84 spikes over 1.5× median · 63.1 stutters/min (work 51.2/min)
- warm-up: 324 frames over 9.4 s, worst 2632.6 ms
- hitches (> 27.1 ms): **34** measured, 52 in warm-up
- timed GPU passes: 6.74 ms of a 11.09 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.84 ms, limit 11 ms
- **FAIL** frame_ms: 69 stutters (> 1.5x moving median of 60) in 1.09 min = 63.06/min, limit 1/min
- **FAIL** worst frame 44.22 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.887 | 2.034 | 2.066 | 0.93 | 0.176 |
| main_opaque_pass_3d | 1.515 | 1.633 | 2.041 | 0.93 | 0.065 |
| early prepass | 0.455 | 0.490 | 0.716 | 0.93 | 0.086 |
| bloom | 0.292 | 0.314 | 0.321 | 0.93 | 0.061 |
| atmosphere_luts | 0.281 | 0.303 | 0.364 | 0.93 | 0.003 |
| water_velocity | 0.259 | 0.279 | 0.348 | 0.93 | 0.002 |
| auxiliary_views | 0.197 | 0.212 | 0.680 | 0.93 | 0.202 |
| water_sspr | 0.187 | 0.201 | 0.203 | 0.93 | 0.002 |
| ssao | 0.174 | 0.188 | 0.191 | 0.93 | 0.007 |
| atmos_froxel_inject | 0.135 | 0.146 | 0.147 | 0.93 | 0.001 |
| clouds_trace | 0.130 | 0.140 | 0.230 | 0.93 | 0.001 |
| uniform_allocation | 0.122 | 0.132 | 0.184 | 0.93 | 0.025 |
| water_fft | 0.121 | 0.130 | 0.137 | 0.93 | 0.003 |
| upscaling | 0.107 | 0.115 | 0.123 | 0.93 | 0.004 |
| clouds_composite | 0.098 | 0.106 | 0.109 | 0.93 | 0.002 |
| bin_unpacking | 0.098 | 0.105 | 0.142 | 0.93 | 0.018 |
| main_transmissive_pass_3d | 0.079 | 0.085 | 0.178 | 0.93 | 0.008 |
| tonemapping | 0.079 | 0.085 | 0.088 | 0.93 | 0.003 |
| clustering | 0.059 | 0.064 | 0.067 | 0.93 | 0.018 |
| render_sky | 0.048 | 0.052 | 0.052 | 0.93 | 0.002 |
| clouds_resolve | 0.045 | 0.049 | 0.049 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.412 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.93 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.67 | 0.002 |
| surface_exposure_far | 0.036 | 0.043 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.93 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.027 | 0.93 | 0.125 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.665 | 12.146 | 39.51 | 57216.3 |
| render/busy/ms | 9.123 | 11.607 | 38.87 | 54005.7 |
| main/busy/ms | 9.104 | 10.849 | 28.22 | 53897.1 |
| stage/render/render | 5.732 | 7.428 | 22.48 | 33934.5 |
| stage/main/post_update | 5.709 | 7.102 | 9.75 | 33798.8 |
| render/graph/ms | 4.998 | 6.670 | 21.68 | 29589.7 |
| encode//sum | 4.483 | 6.516 | 21.32 | 26540.9 |
| stage/render/graph_submit | 3.768 | 5.099 | 20.10 | 22304.3 |
| encode/submit | 2.414 | 3.391 | 17.90 | 14291.2 |
| stage/main/update | 2.127 | 2.539 | 21.70 | 12593.3 |
| encode/finish_wall | 1.417 | 2.066 | 4.14 | 8389.9 |
| stage/render/camera_driver | 1.177 | 1.632 | 3.75 | 6970.2 |
| encode/pass:shadow_point_light/sum | 1.171 | 3.599 | 16.76 | 6934.0 |
| encode/pass:shadow_directional_light/sum | 0.831 | 1.558 | 2.60 | 4921.3 |
| stage/render/prepare_write_phase_buffers | 0.780 | 1.135 | 2.31 | 4617.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.745 | 1.096 | 1.69 | 4409.3 |
| encode/pass:early prepass/sum | 0.704 | 1.063 | 1.69 | 4169.7 |
| stage/render/prepare_assets | 0.560 | 0.720 | 8.85 | 3313.1 |
| bigspace/total/ms | 0.542 | 1.316 | 2.45 | 3207.6 |
| stage/render/prepare_resources | 0.451 | 0.546 | 1.75 | 2671.0 |
| render/present/ms | 0.449 | 0.636 | 2.68 | 2660.0 |
| stage/render/prepare_meshes | 0.441 | 0.555 | 8.20 | 2612.7 |
| stage/main/first | 0.401 | 0.526 | 6.02 | 2375.6 |
| stage/main/pre_update | 0.356 | 0.524 | 1.69 | 2110.0 |
| stage/render/specialize | 0.328 | 0.424 | 12.55 | 1944.0 |
| terrain/update/ms | 0.316 | 0.415 | 2.12 | 1868.4 |
| stage/render/queue | 0.294 | 0.425 | 7.40 | 1739.2 |
| stage/render/prepare_bind_groups | 0.288 | 0.523 | 2.29 | 1702.8 |
| terrain/select/ms | 0.282 | 0.373 | 0.92 | 1670.7 |
| controller/feed/ms | 0.270 | 0.338 | 0.74 | 1596.8 |
| stream/drive/ms | 0.249 | 0.327 | 19.90 | 1473.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.244 | 0.364 | 1.01 | 1445.2 |
| bigspace/low_precision/ms | 0.229 | 0.791 | 1.79 | 1353.1 |
| stage/main/fixed_main_loop | 0.217 | 0.370 | 1.02 | 1284.9 |
| far/select/ms | 0.200 | 0.272 | 0.58 | 1184.8 |
| stage/render/create_views | 0.195 | 0.251 | 1.32 | 1156.2 |
| stream/tick/ms | 0.191 | 0.262 | 0.79 | 1132.5 |
| stage/render/prepare_batch_phases | 0.189 | 0.273 | 0.48 | 1120.0 |
| stage/render/extract_commands | 0.170 | 0.199 | 0.53 | 1008.2 |
| stage/render/prepare_flush | 0.162 | 0.320 | 1.04 | 957.4 |
| anim/evaluate/ms | 0.156 | 0.247 | 1.52 | 926.4 |
| anim/evaluate_cpu/ms | 0.152 | 0.192 | 0.63 | 898.2 |
| bigspace/high_precision/ms | 0.151 | 0.672 | 1.63 | 893.0 |
| stage/render/prepare_views | 0.136 | 0.170 | 0.42 | 806.8 |
| controller/move/ms | 0.125 | 0.218 | 0.61 | 741.9 |
| stage/main/last | 0.117 | 0.158 | 1.72 | 690.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.104 | 0.175 | 0.74 | 618.6 |
| render/acquire/ms | 0.093 | 0.119 | 0.36 | 550.6 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.134 | 0.40 | 377.3 |
| pipelines/compile_ms | 0.049 | 0.000 | 120.43 | 289.0 |
| pipelines/front_ms | 0.047 | 0.000 | 116.66 | 277.5 |
| physics/step/ms | 0.047 | 0.087 | 0.59 | 276.0 |
| physics/insert/ms | 0.035 | 0.371 | 0.90 | 207.5 |
| stage/render/phase_sort | 0.033 | 0.059 | 0.22 | 196.1 |
| stage/render/cleanup | 0.031 | 0.090 | 0.87 | 181.2 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.45 | 178.7 |
| weather/map/ms | 0.025 | 0.033 | 0.22 | 147.9 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.59 | 85.1 |
| anim/apply/ms | 0.011 | 0.022 | 0.18 | 67.6 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.45 | 63.4 |
| controller/avoid/ms | 0.010 | 0.014 | 0.21 | 61.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.06 | 61.2 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 41.1 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 30.6 |
| stream/despawn/ms | 0.005 | 0.000 | 0.97 | 30.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 28.7 |
| script/ui/ms | 0.004 | 0.006 | 0.14 | 25.2 |
| controller/write/ms | 0.004 | 0.005 | 0.05 | 20.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 14.1 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.97 | 11.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.32 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.10 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 32 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1277 | 2262 | 595 |
| crowd/c0/meshes/count | count | 352 | 646 | 158 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5531 | 5995 | 5601 |
| crowd/c1/meshes/count | count | 1177 | 1297 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4613 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4593 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.43 | 1.66 | 0.46 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.59 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.45 | 0.01 |
| encode/finish_critical | ms | 0.96 | 2.60 | 1.47 |
| encode/pass:early prepass/max | ms | 0.71 | 1.69 | 0.91 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.69 | 0.53 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.74 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.01 | 0.19 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.60 | 1.47 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.60 | 0.70 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.45 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.39 | 38.87 | 9.68 |
| gpu/timed/ms | ms | 7.96 | 9.27 | 8.37 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 807 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 807 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1282 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3396 | 5148 | 3187 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1282 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3396 | 5148 | 3187 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 26 | 196 | 29 |
| phases/shadow/bins | count | 5048 | 9069 | 2407 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.5 k | 6358 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 254 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 204 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 553 | 58 |
| preprocess/bind_groups_created | count | 13 | 553 | 58 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 132 | 228 | 123 |
| process/cpu_usage | % | 26.70 | 40.38 | 24.52 |
| process/mem_usage | GiB | 5.53 | 5.70 | 5.70 |
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
| skin/blocks_changed/count | count | 14 | 126 | 15 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 367.9 k | 808.7 k | 494.6 k |
| skin/joints_written/count | count | 5748 | 12.6 k | 7728 |
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
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.02 | 43.20 | 32.14 |
| system/mem_usage | % | 62.48 | 63.14 | 63.07 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 74 | 245 | 79 |
| views/all/bins | count | 7612 | 11.9 k | 5905 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 252 | 301 | 75 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 34.9 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 54 | 29 |
| views/cascade/bins | count | 2802 | 6195 | 2407 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8904 | 23.6 k | 6358 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 419 | 23.1 k | 6358 |
| views/lamp_face/batch_sets | count | 5 | 188 | 0 |
| views/lamp_face/bins | count | 2174 | 2933 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 1 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2419 | 3235 | 0 |
| views/lamp_face/pipelines | count | 168 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 96 | 2867 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2860 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8399 | 13.1 k | 7608 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8566 | 13.3 k | 7916 |
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
| anim/bones_written/count | count | 59.21 | 271 | 350.5 k |
| anim/events/count | count | 0.09 | 3 | 536 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.66 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1260 | 4675 |
| encode//count | count | 35.96 | 220 | 212.9 k |
| encode/bind_walk/init_textures | count | 14.1 k | 114.7 k | 83.19 M |
| encode/bind_walk/init_textures_skipped | count | 60.6 k | 136.1 k | 358.82 M |
| encode/bind_walk/inits | count | 471 | 1115 | 2.79 M |
| encode/bind_walk/inits_skipped | count | 104 | 155 | 617.0 k |
| encode/bind_walk/merge_textures | count | 14.0 k | 114.6 k | 82.76 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 31.77 M |
| encode/bind_walk/merges | count | 102 | 332 | 605.2 k |
| encode/bind_walk/merges_skipped | count | 82.01 | 130 | 485.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5920 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5920 |
| encode/pass:early prepass/count | count | 1 | 1 | 5920 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5920 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5920 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5920 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5920 |
| encode/pass:shadow_point_light/count | count | 2.00 | 48 | 11.8 k |
| encode/queue/command_buffers | count | 47.97 | 278 | 284.0 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.88 M | 11.19 G |
| encode/queue/write_buffer/calls | count | 270 | 826 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.52 M | 1.05 G |
| encode/queue/write_buffer_with/calls | count | 11.04 | 465 | 65.4 k |
| encode/queue/write_texture/bytes | bytes | 12.1 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5920 |
| physics/inserts | count | 0.38 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.67 | 3 | 3939 |
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
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5718 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1254 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.63 | 2632.6 | main/busy/ms 2601, stage/main/post_update 2495, stage/main/update 96.50, surface/advance_cpu_ms 66.84, stage/main/pre_update 5.44, stage/main/last 1.14, surface/cell_upload_ms 0.81, stage/main/first 0.60, stream/drive/ms 0.50, stage/main/fixed_main_loop 0.38, stream/tick/ms 0.13, weather/map/ms 0.12 |
| warm-up | 3.38 | 751.6 | main/busy/ms 2743, stage/main/post_update 2518, render/thread/ms 717, render/busy/ms 545, stage/render/prepare_assets 496, stage/main/update 210, stage/render/prepare_views 169, render/acquire/ms 169, surface/advance_cpu_ms 66.84, stage/render/create_views 27.83, stage/render/render 13.18, render/graph/ms 8.81 |
| warm-up | 3.50 | 112.4 | render/thread/ms 812, render/busy/ms 588, stage/render/prepare_assets 522, stage/render/prepare_views 169, render/acquire/ms 169, main/busy/ms 163, stage/main/update 117, pipelines/compile_ms 104, stage/render/render 71.72, pipelines/front_ms 63.37, render/present/ms 54.71, pipelines/driver_ms 40.72 |
| warm-up | 3.54 | 42.0 | pipelines/compile_ms 434, pipelines/front_ms 358, pipelines/wait_ms 241, render/thread/ms 135, stage/render/render 92.46, render/busy/ms 80.83, pipelines/driver_ms 76.48, main/busy/ms 61.66, render/present/ms 53.99, stage/main/post_update 46.69, render/graph/ms 37.52, pipelines/lock_ms 29.47 |
| warm-up | 3.60 | 58.9 | pipelines/wait_ms 1272, pipelines/compile_ms 637, pipelines/front_ms 584, main/busy/ms 98.61, render/thread/ms 84.25, render/busy/ms 75.89, stage/main/post_update 58.70, pipelines/driver_ms 52.81, stage/render/render 45.12, render/graph/ms 37.30, pipelines/lock_ms 35.48, stage/main/pre_update 28.88 |
| warm-up | 3.71 | 115.8 | pipelines/wait_ms 4761, pipelines/compile_ms 1506, pipelines/front_ms 1426, render/thread/ms 151, main/busy/ms 150, render/busy/ms 145, pipelines/driver_ms 80.16, stage/main/update 76.03, stage/render/prepare_resources 48.53, stage/render/prepare_assets 39.87, stage/main/pre_update 35.52, stage/main/post_update 34.92 |
| warm-up | 3.77 | 61.4 | pipelines/wait_ms 4111, pipelines/compile_ms 2087, pipelines/front_ms 1970, render/thread/ms 162, render/busy/ms 160, pipelines/driver_ms 117, main/busy/ms 107, stage/main/update 76.38, stage/render/prepare_assets 71.21, stage/render/prepare_resources 41.99, stage/render/prepare_bind_groups 22.91, stage/main/post_update 17.23 |
| warm-up | 3.99 | 70.4 | pipelines/compile_ms 1493, pipelines/front_ms 1423, render/thread/ms 70.56, pipelines/driver_ms 69.50, render/busy/ms 68.61, main/busy/ms 56.87, stage/main/update 44.63, stage/render/prepare_assets 42.86, stream/drive/ms 38.55, encode//sum 18.27, stage/render/render 17.14, render/graph/ms 14.86 |
| warm-up | 4.08 | 73.7 | pipelines/compile_ms 659, pipelines/front_ms 628, main/busy/ms 56.20, render/thread/ms 48.27, render/busy/ms 46.08, stage/main/update 42.08, stream/drive/ms 36.67, pipelines/driver_ms 31.30, stage/render/prepare_assets 25.57, stage/render/render 14.91, render/graph/ms 12.24, stage/main/post_update 9.23 |
| warm-up | 4.12 | 34.7 | pipelines/compile_ms 707, pipelines/front_ms 684, render/thread/ms 65.07, render/busy/ms 58.21, main/busy/ms 45.72, stage/render/prepare_assets 33.71, stage/main/update 24.52, pipelines/driver_ms 22.49, stage/render/render 18.65, stream/drive/ms 15.77, render/graph/ms 11.14, stage/main/post_update 10.54 |
| warm-up | 4.21 | 97.4 | pipelines/compile_ms 540, pipelines/front_ms 523, render/thread/ms 90.18, render/busy/ms 83.41, stage/render/prepare_assets 58.62, main/busy/ms 57.65, stage/main/update 29.05, stage/render/render 18.11, pipelines/driver_ms 16.88, stage/main/post_update 16.16, stream/drive/ms 14.95, render/graph/ms 10.64 |
| warm-up | 4.29 | 72.4 | pipelines/compile_ms 976, pipelines/front_ms 949, render/thread/ms 106, render/busy/ms 104, stage/render/prepare_assets 77.36, main/busy/ms 54.78, stage/main/update 32.91, pipelines/driver_ms 26.63, stage/render/render 18.48, render/graph/ms 15.80, stage/main/post_update 13.48, stage/render/graph_submit 8.61 |
| warm-up | 4.34 | 52.2 | pipelines/compile_ms 1230, pipelines/front_ms 1193, render/thread/ms 87.31, render/busy/ms 85.43, stage/render/prepare_assets 62.24, main/busy/ms 49.13, pipelines/driver_ms 37.10, stage/main/update 32.74, stage/render/render 15.26, render/graph/ms 13.04, stage/main/post_update 7.91, encode//sum 6.45 |
| warm-up | 4.39 | 55.7 | pipelines/compile_ms 603, pipelines/front_ms 581, render/thread/ms 94.53, render/busy/ms 91.26, main/busy/ms 60.43, stage/render/render 42.71, stage/main/update 40.15, render/graph/ms 39.29, stage/render/prepare_assets 37.05, stage/render/graph_submit 33.99, encode//sum 22.18, pipelines/driver_ms 21.90 |
| warm-up | 4.45 | 52.5 | pipelines/compile_ms 491, pipelines/front_ms 474, render/thread/ms 85.53, render/busy/ms 82.20, main/busy/ms 58.78, stage/render/render 46.83, render/graph/ms 43.20, stage/main/update 36.27, stage/render/graph_submit 35.40, encode//sum 22.56, encode/submit 21.04, stage/render/prepare_assets 20.12 |
| warm-up | 4.50 | 52.1 | pipelines/compile_ms 1431, pipelines/front_ms 1385, pipelines/wait_ms 107, main/busy/ms 78.91, render/thread/ms 75.58, render/busy/ms 73.98, pipelines/driver_ms 45.97, stage/main/post_update 45.59, stage/render/prepare_assets 26.82, stage/render/render 19.61, render/graph/ms 17.48, stage/main/update 13.08 |
| warm-up | 4.59 | 92.4 | pipelines/compile_ms 1575, pipelines/front_ms 1527, pipelines/wait_ms 279, main/busy/ms 114, render/thread/ms 111, render/busy/ms 110, stage/main/post_update 64.91, pipelines/driver_ms 48.61, stage/render/prepare_assets 27.93, stage/render/render 25.72, render/graph/ms 23.61, stage/main/pre_update 22.21 |
| warm-up | 4.68 | 90.3 | pipelines/compile_ms 2182, pipelines/front_ms 2102, pipelines/wait_ms 411, render/thread/ms 151, render/busy/ms 149, main/busy/ms 85.82, pipelines/driver_ms 79.51, encode//sum 65.18, stage/render/render 49.96, render/graph/ms 47.33, stage/render/graph_submit 40.97, stage/main/post_update 31.77 |
| warm-up | 4.73 | 45.5 | pipelines/compile_ms 2082, pipelines/front_ms 2005, pipelines/wait_ms 312, main/busy/ms 132, render/thread/ms 126, render/busy/ms 123, stage/main/update 92.89, encode//sum 82.67, pipelines/driver_ms 77.07, stage/render/render 66.59, render/graph/ms 63.31, stream/drive/ms 55.64 |
| warm-up | 4.85 | 126.6 | pipelines/compile_ms 1932, pipelines/front_ms 1868, pipelines/wait_ms 326, main/busy/ms 125, render/thread/ms 98.26, stage/main/update 97.50, render/busy/ms 95.59, pipelines/driver_ms 63.02, stream/drive/ms 57.34, stage/render/render 42.28, encode//sum 39.48, render/graph/ms 38.41 |
| warm-up | 4.91 | 56.0 | pipelines/compile_ms 2347, pipelines/front_ms 2277, pipelines/wait_ms 301, render/thread/ms 104, render/busy/ms 101, main/busy/ms 99.28, pipelines/driver_ms 69.93, stage/main/update 67.11, stream/drive/ms 55.37, stage/render/prepare_assets 44.93, stage/render/render 31.09, render/graph/ms 27.57 |
| warm-up | 5.06 | 153.0 | pipelines/compile_ms 1844, pipelines/front_ms 1753, render/thread/ms 140, render/busy/ms 139, main/busy/ms 105, pipelines/driver_ms 90.73, stage/main/update 73.51, stage/render/prepare_assets 58.34, stage/render/render 56.60, stream/drive/ms 54.28, render/graph/ms 54.12, pipelines/wait_ms 50.84 |
| warm-up | 5.13 | 67.6 | pipelines/compile_ms 1877, pipelines/front_ms 1741, render/thread/ms 150, render/busy/ms 143, pipelines/driver_ms 136, main/busy/ms 94.81, stage/render/render 76.03, stage/main/update 68.44, render/graph/ms 67.81, stage/render/prepare_assets 57.97, stream/drive/ms 45.15, stage/render/camera_driver 34.78 |
| warm-up | 5.18 | 45.8 | pipelines/compile_ms 897, pipelines/front_ms 831, render/thread/ms 87.79, main/busy/ms 83.05, render/busy/ms 80.52, pipelines/driver_ms 65.48, stage/main/update 64.24, stream/drive/ms 44.53, stage/render/render 44.08, render/graph/ms 36.04, stage/render/graph_submit 31.89, encode/submit 30.61 |
| warm-up | 5.25 | 47.7 | pipelines/compile_ms 182, pipelines/front_ms 176, main/busy/ms 56.85, stage/main/update 40.85, render/thread/ms 36.61, render/busy/ms 35.23, stream/drive/ms 29.19, stage/render/render 11.29, stage/render/prepare_assets 11.23, stage/main/post_update 9.46, render/graph/ms 9.20, encode//sum 7.93 |
| warm-up | 5.35 | 31.6 | render/thread/ms 51.66, render/busy/ms 49.51, main/busy/ms 45.63, stage/main/update 20.49, stage/main/post_update 19.19, pipelines/compile_ms 19.16, pipelines/front_ms 18.57, stage/render/render 17.05, render/graph/ms 14.26, stage/render/prepare_assets 12.93, stage/render/graph_submit 10.88, encode//sum 9.20 |
| warm-up | 5.39 | 35.8 | pipelines/compile_ms 297, pipelines/front_ms 285, render/thread/ms 59.76, render/busy/ms 57.60, main/busy/ms 55.81, stage/main/update 20.10, stage/render/render 18.09, stage/main/post_update 17.62, stage/render/prepare_assets 16.11, render/graph/ms 15.13, pipelines/driver_ms 11.53, stage/render/graph_submit 11.51 |
| warm-up | 5.45 | 58.4 | pipelines/compile_ms 297, pipelines/front_ms 285, render/thread/ms 74.59, render/busy/ms 73.14, main/busy/ms 50.78, stage/render/prepare_assets 44.94, stage/main/update 21.48, stage/render/render 12.84, stage/main/post_update 11.59, pipelines/driver_ms 11.53, render/graph/ms 10.58, stage/main/first 10.38 |
| warm-up | 5.49 | 46.6 | pipelines/compile_ms 481, pipelines/front_ms 467, render/thread/ms 87.29, render/busy/ms 85.94, main/busy/ms 69.66, stage/render/prepare_assets 37.60, stage/main/post_update 28.79, stage/main/update 21.69, stage/render/render 17.55, render/graph/ms 15.66, pipelines/driver_ms 13.83, pipelines/wait_ms 13.83 |
| warm-up | 5.54 | 44.0 | pipelines/compile_ms 1925, pipelines/front_ms 1872, main/busy/ms 82.13, render/thread/ms 82.04, render/busy/ms 80.59, pipelines/driver_ms 53.48, stage/main/post_update 46.60, pipelines/wait_ms 45.50, stage/render/prepare_resources 19.82, stage/render/render 17.90, render/graph/ms 15.90, stage/main/update 15.82 |
| warm-up | 5.61 | 44.0 | pipelines/compile_ms 214, pipelines/front_ms 211, render/thread/ms 54.97, render/busy/ms 53.56, main/busy/ms 34.08, stage/render/prepare_assets 27.82, stage/main/update 15.46, stage/render/render 13.49, stage/main/post_update 11.74, render/graph/ms 11.23, encode//sum 8.19, stage/render/graph_submit 7.87 |
| warm-up | 5.69 | 34.5 | pipelines/compile_ms 611, pipelines/front_ms 597, render/thread/ms 52.69, render/busy/ms 51.26, main/busy/ms 47.44, stage/main/update 25.12, stage/render/prepare_assets 19.60, stage/main/post_update 16.66, stage/render/render 16.03, pipelines/driver_ms 14.12, render/graph/ms 13.87, stream/drive/ms 13.36 |
| warm-up | 5.72 | 32.2 | pipelines/compile_ms 616, pipelines/front_ms 606, render/thread/ms 57.89, render/busy/ms 56.51, pipelines/wait_ms 54.00, main/busy/ms 39.30, stage/render/prepare_assets 26.81, stage/main/update 19.45, stage/main/post_update 14.13, stage/render/render 13.58, stream/drive/ms 12.64, render/graph/ms 11.32 |
| warm-up | 5.75 | 30.2 | pipelines/compile_ms 450, pipelines/front_ms 442, pipelines/wait_ms 54.99, render/thread/ms 53.58, render/busy/ms 52.00, main/busy/ms 35.40, stage/render/render 19.83, stage/main/post_update 19.57, stage/render/prepare_assets 19.43, render/graph/ms 17.51, stage/render/graph_submit 12.01, stage/main/update 9.31 |
| warm-up | 5.78 | 33.2 | pipelines/compile_ms 299, pipelines/front_ms 282, render/thread/ms 56.58, render/busy/ms 54.89, main/busy/ms 35.18, stage/render/prepare_assets 25.20, stage/main/post_update 19.86, stage/render/render 19.84, render/graph/ms 17.47, pipelines/driver_ms 17.23, stage/render/graph_submit 12.12, stage/main/update 9.20 |
| warm-up | 6.40 | 33.0 | render/thread/ms 39.14, render/busy/ms 38.03, main/busy/ms 30.54, stage/render/prepare_assets 16.28, stage/main/post_update 12.70, stage/main/update 11.87, stage/render/render 10.90, render/graph/ms 9.32, encode//sum 8.63, stage/render/graph_submit 6.25, encode/submit 4.07, stream/drive/ms 3.96 |
| warm-up | 6.62 | 31.1 | pipelines/compile_ms 164, pipelines/front_ms 161, main/busy/ms 37.86, render/thread/ms 37.40, render/busy/ms 36.09, stage/main/update 19.84, stage/main/post_update 13.71, stage/render/render 11.28, render/graph/ms 9.38, encode//sum 9.04, stage/render/graph_submit 6.36, stage/render/prepare_assets 6.02 |
| warm-up | 6.68 | 30.2 | pipelines/compile_ms 89.23, pipelines/front_ms 84.16, render/thread/ms 47.41, render/busy/ms 45.69, main/busy/ms 40.35, stage/main/update 17.55, stage/main/post_update 14.90, stage/render/prepare_assets 14.20, stage/render/render 13.71, render/graph/ms 11.12, encode//sum 10.59, stage/render/graph_submit 7.19 |
| warm-up | 6.79 | 32.9 | render/thread/ms 41.38, main/busy/ms 40.63, render/busy/ms 40.18, stage/main/update 19.84, stage/render/prepare_assets 14.39, stage/main/post_update 13.10, stage/render/render 11.32, render/graph/ms 9.47, encode//sum 8.95, stage/render/graph_submit 6.70, stream/drive/ms 3.89, stage/render/prepare_write_phase_buffers 3.89 |
| warm-up | 6.99 | 39.9 | render/thread/ms 49.65, render/busy/ms 48.53, main/busy/ms 38.68, stage/render/prepare_assets 30.93, stage/main/update 24.62, stream/drive/ms 17.91, stage/render/render 10.69, stage/main/post_update 10.68, render/graph/ms 8.80, encode//sum 8.09, stage/render/graph_submit 6.37, encode/submit 4.00 |
| warm-up | 7.19 | 34.1 | pipelines/compile_ms 133, pipelines/front_ms 127, main/busy/ms 41.12, render/thread/ms 26.07, render/busy/ms 24.95, stage/main/update 20.44, stage/main/post_update 14.52, stream/drive/ms 13.15, stage/render/render 12.27, stream/spawn/ms 11.89, render/graph/ms 10.75, encode//sum 8.61 |
| warm-up | 7.27 | 38.6 | render/thread/ms 44.54, render/busy/ms 43.42, main/busy/ms 27.44, stage/main/update 13.16, pipelines/compile_ms 11.99, pipelines/front_ms 11.29, stage/render/specialize 10.84, stage/render/render 10.59, stage/main/post_update 10.04, render/graph/ms 8.92, encode//sum 8.00, stage/render/queue 7.93 |
| warm-up | 8.53 | 32.1 | render/thread/ms 36.05, render/busy/ms 35.10, main/busy/ms 18.27, stage/render/render 10.64, stage/main/post_update 10.63, render/graph/ms 9.21, stage/render/specialize 9.20, stage/render/graph_submit 7.15, encode//sum 6.92, stage/render/queue 6.45, stage/main/update 5.36, encode/submit 3.73 |
| measure | 0.41 | 31.1 | render/thread/ms 46.88, render/busy/ms 45.86, encode//sum 20.67, stage/render/render 18.94, render/graph/ms 17.34, main/busy/ms 17.23, stage/render/graph_submit 12.76, encode/pass:shadow_point_light/sum 11.99, stage/main/post_update 10.60, stage/render/specialize 9.63, encode/submit 7.70, stage/render/queue 6.53 |
| measure | 1.69 | 31.2 | render/thread/ms 36.56, render/busy/ms 35.56, main/busy/ms 17.13, stage/render/render 11.04, stage/main/post_update 10.18, render/graph/ms 9.63, stage/render/specialize 9.00, encode//sum 8.65, stage/render/graph_submit 7.11, stage/render/queue 5.98, stage/render/prepare_meshes 4.17, stage/main/update 4.15 |
| measure | 11.05 | 30.0 | render/thread/ms 38.27, render/busy/ms 37.18, main/busy/ms 18.78, stage/render/render 12.83, stage/main/post_update 11.39, render/graph/ms 11.26, stage/render/specialize 9.77, encode//sum 9.74, stage/render/graph_submit 8.59, stage/render/queue 6.37, encode/submit 5.25, encode/pass:shadow_point_light/sum 4.83 |
| measure | 15.74 | 31.0 | render/thread/ms 41.32, render/busy/ms 40.30, main/busy/ms 19.49, stage/render/render 14.64, render/graph/ms 13.13, stage/main/post_update 12.06, stage/render/graph_submit 10.37, stage/render/specialize 10.24, encode//sum 10.21, stage/render/queue 6.59, encode/submit 6.38, encode/pass:shadow_point_light/sum 5.06 |
| measure | 18.18 | 31.0 | main/busy/ms 38.28, stage/main/update 23.73, render/thread/ms 21.55, render/busy/ms 20.56, stream/drive/ms 20.12, stage/render/render 12.99, stage/main/post_update 11.65, render/graph/ms 11.59, encode//sum 10.65, stage/render/graph_submit 8.55, encode/pass:shadow_point_light/sum 6.27, encode/submit 5.93 |
| measure | 19.46 | 34.2 | render/thread/ms 45.01, render/busy/ms 43.75, main/busy/ms 22.46, encode//sum 15.89, stage/render/render 15.35, stage/main/post_update 13.69, render/graph/ms 13.60, stage/render/graph_submit 10.47, stage/render/specialize 9.96, stage/render/queue 7.01, encode/pass:shadow_point_light/sum 6.59, encode/submit 6.57 |
| measure | 23.03 | 34.5 | render/thread/ms 45.94, render/busy/ms 44.81, main/busy/ms 23.28, stage/render/render 15.84, encode//sum 14.88, render/graph/ms 13.81, stage/main/post_update 12.36, stage/render/specialize 11.34, stage/render/graph_submit 10.69, stage/render/queue 7.85, encode/pass:shadow_point_light/sum 7.31, stage/main/update 6.35 |
| measure | 24.71 | 30.6 | render/thread/ms 43.52, render/busy/ms 42.38, main/busy/ms 22.89, stage/render/render 15.41, stage/main/post_update 14.18, render/graph/ms 13.79, encode//sum 13.76, stage/render/graph_submit 10.90, stage/render/specialize 10.13, encode/pass:shadow_point_light/sum 7.60, stage/render/queue 6.93, encode/submit 6.68 |
| measure | 27.28 | 35.5 | render/thread/ms 45.00, render/busy/ms 43.98, main/busy/ms 25.09, encode//sum 17.54, stage/render/render 15.40, render/graph/ms 13.87, stage/main/post_update 13.02, stage/render/specialize 11.73, stage/render/graph_submit 10.97, encode/pass:shadow_point_light/sum 8.08, stage/render/queue 7.68, encode/submit 6.96 |
| measure | 28.87 | 30.3 | render/thread/ms 40.02, render/busy/ms 38.95, main/busy/ms 19.68, stage/render/render 13.42, stage/main/post_update 12.70, render/graph/ms 11.90, encode//sum 11.80, stage/render/specialize 9.73, stage/render/graph_submit 9.13, stage/render/queue 6.62, encode/pass:shadow_point_light/sum 5.58, encode/submit 5.32 |
| measure | 30.48 | 35.8 | render/thread/ms 42.26, render/busy/ms 41.20, main/busy/ms 21.83, stage/render/render 13.52, stage/main/post_update 12.49, render/graph/ms 12.07, stage/render/specialize 10.84, encode//sum 10.74, stage/render/graph_submit 9.53, stage/render/queue 7.65, encode/pass:shadow_point_light/sum 6.45, stage/main/update 5.80 |
| measure | 35.37 | 31.0 | render/thread/ms 38.74, render/busy/ms 37.62, main/busy/ms 20.79, stage/main/post_update 12.95, stage/render/render 12.22, render/graph/ms 10.76, stage/render/specialize 10.20, encode//sum 8.92, stage/render/graph_submit 8.03, stage/render/queue 6.72, stage/main/update 5.00, encode/submit 4.52 |
| measure | 37.20 | 33.8 | render/thread/ms 41.80, render/busy/ms 40.80, main/busy/ms 23.88, stage/render/render 12.50, stage/main/post_update 12.03, stage/render/specialize 11.32, render/graph/ms 11.07, encode//sum 10.04, stage/render/graph_submit 8.44, stage/main/update 7.67, stage/render/queue 7.16, encode/submit 4.61 |
| measure | 37.48 | 30.8 | render/thread/ms 36.80, render/busy/ms 35.64, main/busy/ms 19.40, stage/main/post_update 12.06, stage/render/render 11.17, stage/render/specialize 10.00, render/graph/ms 9.61, encode//sum 9.38, stage/render/graph_submit 7.35, stage/render/queue 6.48, encode/submit 4.32, stage/main/update 4.30 |
| measure | 40.80 | 31.0 | render/thread/ms 37.89, render/busy/ms 36.92, main/busy/ms 19.33, stage/main/post_update 11.26, stage/render/render 10.45, stage/render/specialize 9.47, render/graph/ms 9.12, encode//sum 8.61, stage/render/graph_submit 7.29, stage/render/queue 6.79, stage/main/update 4.73, encode/submit 3.91 |
| measure | 43.22 | 31.4 | render/thread/ms 38.55, render/busy/ms 37.07, main/busy/ms 18.70, stage/main/post_update 11.12, stage/render/render 10.30, stage/render/specialize 10.14, render/graph/ms 8.76, encode//sum 8.04, stage/render/queue 6.96, stage/render/graph_submit 6.89, stage/main/update 4.56, stage/render/prepare_meshes 4.16 |
| measure | 45.68 | 37.1 | render/thread/ms 39.65, render/busy/ms 38.41, main/busy/ms 18.04, stage/render/render 10.95, stage/render/specialize 10.68, stage/main/post_update 10.47, render/graph/ms 9.39, encode//sum 8.28, stage/render/graph_submit 7.37, stage/render/queue 6.75, stage/main/update 4.54, stage/render/prepare_meshes 4.29 |
| measure | 56.44 | 32.8 | render/thread/ms 36.57, render/busy/ms 35.31, main/busy/ms 15.55, stage/render/render 10.43, stage/main/post_update 9.61, stage/render/specialize 9.47, render/graph/ms 8.84, encode//sum 8.74, stage/render/graph_submit 6.74, stage/render/queue 6.70, stage/main/update 3.74, stage/render/prepare_meshes 3.70 |
| measure | 59.03 | 44.2 | render/thread/ms 55.29, render/busy/ms 53.78, main/busy/ms 28.97, stage/render/render 17.71, stage/main/post_update 16.34, render/graph/ms 15.79, encode//sum 13.31, stage/render/specialize 12.72, stage/render/graph_submit 12.71, stage/render/prepare_meshes 8.63, stage/main/update 8.58, encode/submit 8.47 |
| measure | 59.07 | 35.6 | render/thread/ms 57.92, render/busy/ms 56.77, main/busy/ms 24.13, stage/render/specialize 12.85, stage/main/post_update 12.13, stage/render/prepare_assets 12.02, stage/render/render 10.95, render/graph/ms 9.55, stage/render/prepare_meshes 9.15, encode//sum 9.09, stage/render/graph_submit 7.37, stage/main/update 7.01 |
| measure | 62.58 | 31.8 | render/thread/ms 37.47, render/busy/ms 36.41, main/busy/ms 18.50, stage/render/specialize 10.41, stage/render/render 10.31, stage/main/post_update 10.00, render/graph/ms 9.01, encode//sum 8.68, stage/render/graph_submit 6.98, stage/render/queue 6.64, stage/main/update 5.99, stage/render/prepare_meshes 4.31 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_nomat\balmora_walk@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 1%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **77%** (min 38%) · 2625 MHz (min 2625) · memory 8101 MHz · 79.3 W · 75 °C (max 80) · VRAM 7004 MiB · throttled 100% of samples

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
