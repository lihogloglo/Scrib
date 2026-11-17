# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5949 frames over 65.6 s · **median 10.73 ms (93 fps)** · mean 11.03 · p95 13.61 · p99 17.45 (1% low 57 fps) · worst 46.21 ms · 86 spikes over 1.5× median · 65.8 stutters/min (work 52.1/min)
- warm-up: 324 frames over 9.3 s, worst 2572.2 ms
- hitches (> 26.8 ms): **32** measured, 47 in warm-up
- timed GPU passes: 6.78 ms of a 11.03 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.73 ms, limit 11 ms
- **FAIL** frame_ms: 72 stutters (> 1.5x moving median of 60) in 1.09 min = 65.81/min, limit 1/min
- **FAIL** worst frame 46.21 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.909 | 2.034 | 2.064 | 0.94 | 0.176 |
| main_opaque_pass_3d | 1.535 | 1.636 | 2.046 | 0.94 | 0.071 |
| early prepass | 0.462 | 0.493 | 0.717 | 0.94 | 0.087 |
| bloom | 0.295 | 0.314 | 0.321 | 0.94 | 0.062 |
| atmosphere_luts | 0.284 | 0.303 | 0.365 | 0.94 | 0.003 |
| water_velocity | 0.261 | 0.278 | 0.347 | 0.94 | 0.002 |
| water_sspr | 0.197 | 0.210 | 0.211 | 0.94 | 0.002 |
| ssao | 0.177 | 0.188 | 0.193 | 0.94 | 0.007 |
| auxiliary_views | 0.146 | 0.155 | 0.582 | 0.94 | 0.161 |
| atmos_froxel_inject | 0.137 | 0.146 | 0.147 | 0.94 | 0.001 |
| clouds_trace | 0.131 | 0.140 | 0.231 | 0.94 | 0.001 |
| water_fft | 0.126 | 0.134 | 0.141 | 0.94 | 0.003 |
| uniform_allocation | 0.122 | 0.130 | 0.182 | 0.94 | 0.027 |
| upscaling | 0.112 | 0.119 | 0.126 | 0.94 | 0.004 |
| clouds_composite | 0.099 | 0.106 | 0.109 | 0.94 | 0.002 |
| bin_unpacking | 0.098 | 0.104 | 0.141 | 0.94 | 0.019 |
| main_transmissive_pass_3d | 0.080 | 0.085 | 0.178 | 0.94 | 0.009 |
| tonemapping | 0.079 | 0.085 | 0.088 | 0.94 | 0.004 |
| clustering | 0.060 | 0.064 | 0.068 | 0.94 | 0.018 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.94 | 0.002 |
| clouds_resolve | 0.046 | 0.049 | 0.049 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.411 | 0.415 | 0.11 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.94 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.67 | 0.002 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.83 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.94 | 0.003 |
| early_mesh_preprocessing | 0.029 | 0.031 | 0.047 | 0.94 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.94 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.220 | 0.232 | 0.11 | 0.004 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.94 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.94 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.027 | 0.94 | 0.125 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.94 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.452 | 11.689 | 40.83 | 56228.5 |
| main/busy/ms | 9.207 | 10.890 | 27.98 | 54770.2 |
| render/busy/ms | 8.906 | 11.133 | 40.23 | 52983.4 |
| stage/main/post_update | 5.708 | 7.062 | 10.86 | 33958.4 |
| stage/render/render | 5.632 | 7.210 | 15.11 | 33504.4 |
| render/graph/ms | 4.893 | 6.442 | 13.79 | 29106.7 |
| encode//sum | 4.207 | 5.639 | 24.90 | 25025.4 |
| stage/render/graph_submit | 3.674 | 4.895 | 9.88 | 21855.2 |
| encode/submit | 2.331 | 3.290 | 6.60 | 13870.0 |
| stage/main/update | 2.206 | 2.636 | 21.90 | 13124.9 |
| encode/finish_wall | 1.404 | 2.014 | 4.33 | 8351.8 |
| stage/render/camera_driver | 1.165 | 1.628 | 3.99 | 6930.7 |
| encode/pass:shadow_point_light/sum | 0.939 | 3.344 | 17.63 | 5588.8 |
| encode/pass:shadow_directional_light/sum | 0.873 | 1.570 | 2.57 | 5194.9 |
| stage/render/prepare_write_phase_buffers | 0.772 | 1.114 | 2.36 | 4594.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.772 | 1.100 | 1.79 | 4594.1 |
| encode/pass:early prepass/sum | 0.759 | 1.101 | 2.07 | 4515.5 |
| bigspace/total/ms | 0.602 | 1.451 | 2.35 | 3583.4 |
| stage/render/prepare_assets | 0.532 | 0.693 | 10.50 | 3164.7 |
| stage/render/prepare_resources | 0.468 | 0.561 | 1.07 | 2785.9 |
| render/present/ms | 0.451 | 0.639 | 4.48 | 2682.7 |
| stage/render/prepare_meshes | 0.447 | 0.550 | 8.51 | 2657.6 |
| stage/main/first | 0.408 | 0.533 | 5.94 | 2429.5 |
| stage/main/pre_update | 0.370 | 0.553 | 3.27 | 2198.2 |
| terrain/update/ms | 0.311 | 0.411 | 1.88 | 1850.6 |
| stage/render/prepare_bind_groups | 0.291 | 0.520 | 2.32 | 1732.2 |
| terrain/select/ms | 0.277 | 0.369 | 0.95 | 1647.5 |
| controller/feed/ms | 0.272 | 0.341 | 0.82 | 1618.9 |
| stage/render/specialize | 0.268 | 0.345 | 11.77 | 1592.8 |
| stream/drive/ms | 0.265 | 0.350 | 19.88 | 1579.4 |
| bigspace/low_precision/ms | 0.252 | 0.978 | 1.88 | 1496.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.248 | 0.369 | 0.99 | 1473.2 |
| stage/render/queue | 0.238 | 0.329 | 8.41 | 1416.9 |
| stage/main/fixed_main_loop | 0.221 | 0.374 | 1.04 | 1317.1 |
| stream/tick/ms | 0.206 | 0.281 | 0.68 | 1224.0 |
| far/select/ms | 0.200 | 0.273 | 0.67 | 1192.6 |
| stage/render/create_views | 0.199 | 0.259 | 0.90 | 1183.2 |
| stage/render/prepare_batch_phases | 0.191 | 0.272 | 0.68 | 1135.6 |
| bigspace/high_precision/ms | 0.180 | 0.895 | 1.90 | 1069.8 |
| anim/evaluate/ms | 0.180 | 0.379 | 1.58 | 1069.4 |
| stage/render/extract_commands | 0.176 | 0.205 | 0.42 | 1049.3 |
| anim/evaluate_cpu/ms | 0.160 | 0.199 | 0.85 | 951.8 |
| stage/render/prepare_flush | 0.159 | 0.325 | 1.00 | 943.8 |
| controller/move/ms | 0.148 | 0.270 | 0.58 | 879.4 |
| stage/render/prepare_views | 0.139 | 0.176 | 0.47 | 828.4 |
| stage/main/last | 0.116 | 0.157 | 2.56 | 691.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.105 | 0.172 | 0.78 | 623.9 |
| render/acquire/ms | 0.095 | 0.124 | 0.42 | 562.3 |
| stage/render/prepare_collect_phase_buffers | 0.069 | 0.137 | 0.56 | 409.2 |
| pipelines/compile_ms | 0.050 | 0.000 | 127.85 | 297.4 |
| physics/step/ms | 0.048 | 0.089 | 0.54 | 287.5 |
| pipelines/front_ms | 0.048 | 0.000 | 123.57 | 284.7 |
| stage/render/cleanup | 0.033 | 0.101 | 0.79 | 198.4 |
| physics/insert/ms | 0.033 | 0.358 | 0.91 | 195.4 |
| encode/resolve_diagnostics/sum | 0.033 | 0.043 | 0.71 | 193.9 |
| stage/render/phase_sort | 0.031 | 0.053 | 0.26 | 181.6 |
| weather/map/ms | 0.025 | 0.034 | 0.22 | 151.6 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.022 | 0.73 | 92.8 |
| anim/apply/ms | 0.012 | 0.021 | 0.40 | 70.5 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.70 | 70.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.09 | 60.9 |
| controller/avoid/ms | 0.010 | 0.013 | 0.17 | 60.9 |
| script/update/ms | 0.007 | 0.010 | 0.05 | 44.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.27 | 37.2 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 32.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.08 | 31.9 |
| script/ui/ms | 0.004 | 0.006 | 0.48 | 25.8 |
| controller/write/ms | 0.003 | 0.004 | 0.15 | 19.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.70 | 13.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.28 | 12.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.58 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 353 | 646 | 162 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5539 | 5937 | 5613 |
| crowd/c1/meshes/count | count | 1166 | 1281 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 237 |
| crowd/joints_extracted/count | count | 4606 | 10.1 k | 6494 |
| crowd/joints_uploaded/count | count | 4592 | 6835 | 6494 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.43 | 1.64 | 0.38 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.73 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.70 | 0.01 |
| encode/finish_critical | ms | 0.99 | 2.57 | 1.18 |
| encode/pass:early prepass/max | ms | 0.78 | 2.07 | 0.95 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.79 | 1.79 | 1.18 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.78 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 0.99 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 2.57 | 0.91 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.60 | 0.73 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.71 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.38 | 40.23 | 8.41 |
| gpu/timed/ms | ms | 7.97 | 11.38 | 8.29 |
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
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3403 | 5113 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3403 | 5113 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 206 | 4 |
| phases/shadow/bins | count | 5044 | 8939 | 2412 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6353 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 203 | 261 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 565 | 6 |
| preprocess/bind_groups_created | count | 14 | 565 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 217 | 123 |
| process/cpu_usage | % | 26.92 | 41.35 | 32.23 |
| process/mem_usage | GiB | 5.47 | 5.64 | 5.64 |
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
| skin/blocks_changed/count | count | 14 | 124 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 237 |
| skin/joint_bytes_written/bytes | bytes | 366.3 k | 808.7 k | 494.3 k |
| skin/joints_written/count | count | 5724 | 12.6 k | 7724 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4920 | 5013 | 5013 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.89 | 48.14 | 29.99 |
| system/mem_usage | % | 54.05 | 54.59 | 54.53 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 68 | 253 | 54 |
| views/all/bins | count | 7620 | 11.8 k | 5910 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 248 | 308 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 35.4 k | 8119 |
| views/cascade/batch_sets | count | 9 | 49 | 4 |
| views/cascade/bins | count | 2800 | 6208 | 2412 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8905 | 23.5 k | 6353 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 411 | 22.4 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 2186 | 2893 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2431 | 3190 | 0 |
| views/lamp_face/pipelines | count | 166 | 211 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2874 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2860 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8427 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8588 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 53.72 | 205 | 319.6 k |
| anim/events/count | count | 0.09 | 3 | 541 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.27 | 11 | 7527 |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1245 | 4675 |
| encode//count | count | 33.99 | 220 | 202.2 k |
| encode/bind_walk/init_textures | count | 12.8 k | 114.7 k | 76.38 M |
| encode/bind_walk/init_textures_skipped | count | 60.7 k | 141.1 k | 361.11 M |
| encode/bind_walk/inits | count | 460 | 1121 | 2.73 M |
| encode/bind_walk/inits_skipped | count | 105 | 266 | 623.5 k |
| encode/bind_walk/merge_textures | count | 12.8 k | 114.6 k | 75.95 M |
| encode/bind_walk/merge_textures_skipped | count | 5367 | 12.0 k | 31.93 M |
| encode/bind_walk/merges | count | 99.24 | 334 | 590.4 k |
| encode/bind_walk/merges_skipped | count | 82.50 | 238 | 490.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 5949 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 5949 |
| encode/pass:early prepass/count | count | 1 | 2 | 5949 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 5949 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 5949 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 5949 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 5949 |
| encode/pass:shadow_point_light/count | count | 1.51 | 48 | 8966 |
| encode/queue/command_buffers | count | 45.52 | 278 | 270.8 k |
| encode/queue/submits | count | 2.02 | 4 | 12.0 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.99 M | 11.16 G |
| encode/queue/write_buffer/calls | count | 259 | 846 | 1.54 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.8 k | 3.52 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 11.03 | 465 | 65.6 k |
| encode/queue/write_texture/bytes | bytes | 12.1 k | 8.39 M | 71.91 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 5949 |
| physics/inserts | count | 0.38 | 92 | 2274 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 161 | 2335 |
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
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 18 | 378 |
| stream/upload/bytes | bytes | 5690 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1248 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.8 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2572.2 | main/busy/ms 2539, stage/main/post_update 2430, stage/main/update 99.63, surface/advance_cpu_ms 70.15, stage/main/pre_update 5.91, stage/main/last 1.29, surface/cell_upload_ms 0.77, stream/drive/ms 0.62, stage/main/first 0.57, stage/main/fixed_main_loop 0.44, weather/map/ms 0.23, stream/tick/ms 0.17 |
| warm-up | 3.42 | 845.9 | main/busy/ms 2716, stage/main/post_update 2455, render/thread/ms 812, render/busy/ms 679, stage/render/prepare_assets 630, stage/main/update 246, stage/render/prepare_views 130, render/acquire/ms 130, surface/advance_cpu_ms 70.15, stage/render/create_views 28.47, stage/render/render 11.63, stage/main/pre_update 7.88 |
| warm-up | 3.52 | 102.2 | render/thread/ms 896, render/busy/ms 717, stage/render/prepare_assets 655, main/busy/ms 196, stage/main/update 151, stage/render/prepare_views 130, render/acquire/ms 130, pipelines/compile_ms 111, pipelines/front_ms 75.96, stage/render/render 63.17, render/present/ms 48.92, pipelines/driver_ms 35.43 |
| warm-up | 3.56 | 41.2 | pipelines/compile_ms 317, pipelines/front_ms 255, pipelines/wait_ms 169, render/thread/ms 122, stage/render/render 80.69, render/busy/ms 64.21, pipelines/driver_ms 62.12, main/busy/ms 58.98, render/present/ms 56.49, stage/main/post_update 42.64, stage/render/prepare_assets 25.46, terrain/update/ms 23.33 |
| warm-up | 3.67 | 104.1 | pipelines/wait_ms 2646, pipelines/compile_ms 1385, pipelines/front_ms 1247, pipelines/driver_ms 139, render/thread/ms 137, render/busy/ms 125, main/busy/ms 71.35, stage/main/post_update 56.30, stage/render/render 54.06, render/graph/ms 41.51, stage/render/graph_submit 32.72, stage/render/prepare_write_phase_buffers 27.80 |
| warm-up | 3.71 | 48.2 | pipelines/wait_ms 4410, pipelines/compile_ms 1477, pipelines/front_ms 1349, render/thread/ms 144, render/busy/ms 141, pipelines/driver_ms 128, main/busy/ms 124, stage/main/update 72.12, stage/main/post_update 34.93, stage/render/render 33.06, stage/render/prepare_assets 32.95, render/graph/ms 30.49 |
| warm-up | 3.84 | 126.6 | pipelines/wait_ms 2154, pipelines/compile_ms 1541, pipelines/front_ms 1445, render/thread/ms 114, main/busy/ms 112, render/busy/ms 112, pipelines/driver_ms 95.76, stage/render/prepare_assets 78.18, stage/main/update 77.66, stage/main/post_update 19.20, terrain/update/ms 14.91, stage/render/render 13.34 |
| warm-up | 3.89 | 49.6 | pipelines/compile_ms 2492, pipelines/front_ms 2376, pipelines/wait_ms 387, render/thread/ms 119, render/busy/ms 117, pipelines/driver_ms 116, stage/render/prepare_assets 98.46, main/busy/ms 36.20, encode//sum 19.89, stage/main/post_update 16.92, stage/render/render 12.85, stage/main/update 11.96 |
| warm-up | 3.93 | 39.9 | pipelines/compile_ms 1344, pipelines/front_ms 1297, pipelines/wait_ms 169, render/thread/ms 86.47, render/busy/ms 83.66, pipelines/driver_ms 47.55, stage/render/prepare_assets 43.56, encode//sum 33.75, stage/render/render 33.31, render/graph/ms 30.38, main/busy/ms 30.31, stage/render/graph_submit 25.48 |
| warm-up | 4.06 | 66.0 | pipelines/compile_ms 925, pipelines/front_ms 892, render/thread/ms 59.43, render/busy/ms 57.40, main/busy/ms 55.86, stage/main/update 43.48, stream/drive/ms 38.66, stage/render/prepare_assets 36.93, pipelines/driver_ms 32.29, stage/render/render 15.24, render/graph/ms 12.93, stage/render/graph_submit 10.44 |
| warm-up | 4.11 | 31.5 | pipelines/compile_ms 306, pipelines/front_ms 290, render/thread/ms 56.57, render/busy/ms 53.42, main/busy/ms 32.49, stage/render/prepare_assets 24.60, stage/render/render 21.76, render/graph/ms 18.09, stage/main/update 16.33, pipelines/driver_ms 16.11, stage/render/graph_submit 14.49, pipelines/wait_ms 10.94 |
| warm-up | 4.23 | 93.7 | pipelines/compile_ms 451, pipelines/front_ms 436, main/busy/ms 51.40, render/thread/ms 44.01, render/busy/ms 43.08, stage/main/update 33.31, stage/render/prepare_assets 32.94, stream/drive/ms 28.56, pipelines/driver_ms 15.01, stage/main/post_update 10.92, stage/render/render 6.23, render/graph/ms 5.07 |
| warm-up | 4.26 | 31.8 | pipelines/compile_ms 525, pipelines/front_ms 506, render/thread/ms 70.16, render/busy/ms 67.26, main/busy/ms 41.08, stage/render/prepare_assets 38.88, stage/main/update 21.37, pipelines/driver_ms 18.39, stage/render/render 12.01, stage/main/post_update 10.58, render/graph/ms 9.73, encode//sum 6.38 |
| warm-up | 4.35 | 95.6 | pipelines/compile_ms 1342, pipelines/front_ms 1297, render/thread/ms 86.75, render/busy/ms 74.89, main/busy/ms 68.58, stage/render/prepare_assets 49.85, stage/main/update 45.71, pipelines/driver_ms 45.34, stage/render/render 20.15, stage/main/post_update 13.95, render/present/ms 10.52, render/graph/ms 9.02 |
| warm-up | 4.44 | 86.3 | pipelines/compile_ms 1770, pipelines/front_ms 1703, render/thread/ms 131, render/busy/ms 119, stage/render/prepare_assets 84.41, main/busy/ms 70.64, pipelines/driver_ms 67.28, stage/main/update 36.15, stage/render/render 33.88, stage/main/post_update 24.55, render/graph/ms 21.47, stage/render/graph_submit 19.38 |
| warm-up | 4.49 | 44.3 | pipelines/compile_ms 777, pipelines/front_ms 737, render/thread/ms 106, render/busy/ms 103, pipelines/wait_ms 87.21, main/busy/ms 57.43, stage/render/prepare_assets 55.39, pipelines/driver_ms 40.19, stage/main/post_update 31.89, stage/render/render 26.56, render/graph/ms 23.10, stage/render/graph_submit 19.52 |
| warm-up | 4.57 | 85.1 | pipelines/compile_ms 1931, pipelines/front_ms 1871, pipelines/wait_ms 174, render/thread/ms 108, render/busy/ms 106, main/busy/ms 75.87, pipelines/driver_ms 59.94, stage/render/prepare_assets 54.10, stage/main/post_update 35.70, stage/main/update 32.33, stage/render/render 15.63, encode//sum 14.59 |
| warm-up | 4.62 | 47.7 | pipelines/compile_ms 2191, pipelines/front_ms 2132, pipelines/wait_ms 524, render/thread/ms 117, render/busy/ms 115, main/busy/ms 69.24, pipelines/driver_ms 59.26, stage/render/prepare_assets 53.70, encode//sum 49.22, stage/main/update 33.03, stage/main/post_update 29.46, stage/render/render 25.46 |
| warm-up | 4.69 | 75.2 | pipelines/compile_ms 1300, pipelines/front_ms 1262, pipelines/wait_ms 952, render/thread/ms 105, render/busy/ms 103, encode//sum 53.93, main/busy/ms 44.52, pipelines/driver_ms 38.01, stage/render/prepare_assets 37.35, stage/render/render 27.33, render/graph/ms 24.91, stage/main/post_update 19.35 |
| warm-up | 4.79 | 96.1 | pipelines/wait_ms 2388, pipelines/compile_ms 2136, pipelines/front_ms 2076, render/thread/ms 150, render/busy/ms 148, main/busy/ms 128, stage/main/update 79.90, pipelines/driver_ms 59.99, stream/drive/ms 58.99, stage/render/prepare_write_phase_buffers 39.35, stage/render/prepare_assets 37.86, stage/main/post_update 28.95 |
| warm-up | 4.84 | 51.0 | pipelines/wait_ms 2735, pipelines/compile_ms 2298, pipelines/front_ms 2235, main/busy/ms 142, render/thread/ms 117, render/busy/ms 114, stage/main/update 93.16, pipelines/driver_ms 63.37, stream/drive/ms 59.04, stage/render/prepare_write_phase_buffers 38.44, stage/main/post_update 26.15, stage/render/prepare_assets 22.93 |
| warm-up | 4.91 | 65.3 | pipelines/compile_ms 1535, pipelines/front_ms 1495, pipelines/wait_ms 1034, main/busy/ms 128, stage/main/update 87.44, render/thread/ms 70.69, render/busy/ms 67.28, stream/drive/ms 47.57, pipelines/driver_ms 40.53, stage/main/post_update 25.85, stage/render/prepare_assets 23.41, stage/render/render 21.75 |
| warm-up | 5.02 | 112.2 | pipelines/compile_ms 1783, pipelines/front_ms 1727, pipelines/wait_ms 1652, main/busy/ms 106, render/thread/ms 80.41, render/busy/ms 78.40, stage/main/update 74.62, pipelines/driver_ms 56.60, stream/drive/ms 48.35, stage/render/prepare_assets 37.78, stage/main/post_update 23.70, stage/render/render 19.46 |
| warm-up | 5.06 | 44.4 | pipelines/compile_ms 1722, pipelines/front_ms 1662, pipelines/wait_ms 1493, main/busy/ms 84.24, render/thread/ms 71.14, render/busy/ms 69.59, stage/main/update 63.98, pipelines/driver_ms 59.55, stream/drive/ms 48.24, stage/render/prepare_assets 41.44, stage/render/render 14.69, render/graph/ms 12.68 |
| warm-up | 5.15 | 91.7 | pipelines/compile_ms 1206, pipelines/front_ms 1164, main/busy/ms 94.16, render/thread/ms 75.24, render/busy/ms 73.66, stage/main/update 71.12, stream/drive/ms 48.05, pipelines/driver_ms 41.84, stage/render/prepare_assets 38.31, stage/render/render 25.04, render/graph/ms 23.04, stage/render/graph_submit 20.38 |
| warm-up | 5.25 | 65.1 | pipelines/compile_ms 531, pipelines/front_ms 513, main/busy/ms 71.04, stage/main/update 54.87, render/thread/ms 47.37, render/busy/ms 45.82, stream/drive/ms 36.66, stage/render/prepare_assets 19.63, pipelines/driver_ms 18.41, stage/render/render 12.62, render/graph/ms 10.35, encode//sum 9.80 |
| warm-up | 5.28 | 35.5 | pipelines/compile_ms 360, pipelines/front_ms 348, render/thread/ms 56.63, render/busy/ms 55.07, main/busy/ms 43.44, stage/main/update 20.13, stage/main/post_update 16.45, stage/render/prepare_assets 16.02, pipelines/driver_ms 12.33, stage/render/render 10.74, render/graph/ms 8.83, stage/render/prepare_resources 8.24 |
| warm-up | 5.37 | 31.5 | pipelines/compile_ms 1332, pipelines/front_ms 1295, render/thread/ms 50.13, render/busy/ms 48.48, pipelines/driver_ms 36.37, main/busy/ms 34.16, stage/render/render 19.95, render/graph/ms 17.86, stage/main/update 14.34, stage/main/post_update 13.40, stage/render/graph_submit 12.41, stage/render/prepare_assets 10.05 |
| warm-up | 5.43 | 39.2 | pipelines/compile_ms 520, pipelines/front_ms 500, render/thread/ms 50.47, render/busy/ms 48.80, main/busy/ms 39.92, stage/render/prepare_assets 22.22, stage/main/update 20.95, pipelines/driver_ms 19.45, stage/render/render 14.12, render/graph/ms 11.84, stage/main/post_update 11.56, encode//sum 8.86 |
| warm-up | 5.49 | 31.8 | pipelines/compile_ms 555, pipelines/front_ms 537, render/thread/ms 51.55, render/busy/ms 49.70, main/busy/ms 45.24, stage/main/update 29.81, stage/render/render 19.48, pipelines/driver_ms 18.08, render/graph/ms 13.88, stage/render/prepare_assets 10.48, stage/main/post_update 9.98, encode//sum 9.87 |
| warm-up | 5.53 | 40.9 | pipelines/compile_ms 99.66, pipelines/front_ms 93.60, render/thread/ms 60.94, render/busy/ms 59.18, main/busy/ms 49.62, stage/main/update 31.82, stage/render/prepare_assets 21.31, stage/render/render 19.15, render/graph/ms 13.62, stage/main/post_update 11.76, encode//sum 9.59, stage/render/graph_submit 8.97 |
| warm-up | 5.71 | 33.3 | pipelines/compile_ms 400, pipelines/front_ms 393, render/thread/ms 53.81, render/busy/ms 52.50, main/busy/ms 31.89, stage/render/prepare_assets 30.03, stage/main/update 13.29, stage/main/post_update 12.87, stage/render/render 10.03, encode//sum 8.33, render/graph/ms 8.04, pipelines/driver_ms 6.89 |
| warm-up | 5.74 | 33.3 | render/thread/ms 56.04, render/busy/ms 54.78, stage/render/prepare_assets 34.86, main/busy/ms 27.56, stage/main/post_update 12.28, stage/render/render 11.24, stage/main/update 9.48, render/graph/ms 9.28, encode//sum 8.17, stage/render/graph_submit 6.46, encode/submit 3.87, stage/render/camera_driver 2.66 |
| warm-up | 6.31 | 30.4 | render/thread/ms 41.43, render/busy/ms 40.44, main/busy/ms 29.94, stage/render/prepare_assets 19.35, stage/main/update 13.18, stage/main/post_update 9.72, stage/render/render 8.42, encode//sum 7.79, render/graph/ms 7.08, stage/render/graph_submit 5.14, stream/drive/ms 4.81, stage/render/specialize 3.97 |
| warm-up | 6.53 | 30.9 | render/thread/ms 41.29, render/busy/ms 39.97, main/busy/ms 39.74, stage/main/post_update 16.96, stage/render/render 16.49, stage/main/update 15.79, render/graph/ms 14.25, encode//sum 13.05, stage/render/graph_submit 9.49, stream/drive/ms 5.70, encode/submit 5.56, stage/render/prepare_assets 5.52 |
| warm-up | 6.63 | 34.5 | main/busy/ms 44.20, render/thread/ms 29.03, render/busy/ms 27.59, stage/main/update 25.46, stream/drive/ms 14.34, stage/main/post_update 13.74, stage/render/render 11.03, render/graph/ms 8.92, encode//sum 8.34, stage/render/graph_submit 5.90, stage/render/prepare_assets 5.84, encode/submit 3.46 |
| warm-up | 6.66 | 31.1 | main/busy/ms 40.77, render/thread/ms 40.11, render/busy/ms 38.25, stage/main/update 19.54, stage/main/post_update 15.02, stage/render/render 13.64, stage/render/prepare_assets 11.93, render/graph/ms 10.88, encode//sum 9.07, stage/render/graph_submit 7.21, encode/submit 4.51, stream/drive/ms 4.47 |
| warm-up | 7.05 | 32.2 | main/busy/ms 39.68, render/thread/ms 23.51, render/busy/ms 22.41, stage/main/update 21.87, stream/drive/ms 14.57, stream/spawn/ms 13.21, stage/main/post_update 12.66, stage/render/render 9.99, encode//sum 8.84, render/graph/ms 8.49, stage/render/graph_submit 6.05, stage/render/prepare_assets 4.86 |
| warm-up | 7.14 | 39.6 | pipelines/compile_ms 141, pipelines/front_ms 138, render/thread/ms 42.90, render/busy/ms 41.81, main/busy/ms 26.85, stage/main/update 12.26, stage/main/post_update 10.63, stage/render/specialize 10.53, stage/render/render 10.47, encode//sum 8.87, render/graph/ms 8.80, stage/render/queue 7.57 |
| warm-up | 8.41 | 31.3 | render/thread/ms 37.22, render/busy/ms 35.95, main/busy/ms 18.08, stage/render/render 11.19, stage/main/post_update 10.56, stage/render/specialize 9.76, render/graph/ms 9.52, stage/render/graph_submit 7.45, encode//sum 6.84, stage/render/queue 6.29, stage/main/update 4.92, encode/submit 4.19 |
| measure | 0.42 | 33.9 | render/thread/ms 36.20, render/busy/ms 35.04, main/busy/ms 18.00, stage/render/render 12.04, stage/main/post_update 11.15, render/graph/ms 10.41, encode//sum 10.09, stage/render/specialize 9.39, stage/render/graph_submit 8.01, stage/render/queue 5.70, encode/submit 5.22, stage/main/update 3.68 |
| measure | 1.73 | 31.2 | render/thread/ms 38.38, render/busy/ms 37.30, main/busy/ms 18.96, stage/render/render 12.32, stage/main/post_update 11.33, render/graph/ms 10.83, stage/render/specialize 9.27, encode//sum 8.59, stage/render/graph_submit 8.07, stage/render/queue 6.36, encode/submit 5.00, stage/main/update 4.69 |
| measure | 3.17 | 32.3 | render/thread/ms 46.15, render/busy/ms 45.09, main/busy/ms 19.43, stage/render/render 14.87, render/graph/ms 13.26, stage/main/post_update 12.29, encode//sum 11.07, stage/render/specialize 9.58, encode/pass:shadow_point_light/sum 9.56, stage/render/graph_submit 9.34, stage/render/queue 6.33, encode/submit 5.70 |
| measure | 5.96 | 30.5 | render/thread/ms 33.91, render/busy/ms 32.85, main/busy/ms 19.90, stage/main/post_update 12.36, stage/render/render 9.88, stage/render/specialize 8.68, render/graph/ms 8.30, encode//sum 7.64, stage/render/queue 6.82, stage/render/graph_submit 6.32, stage/main/update 4.04, encode/submit 3.35 |
| measure | 11.21 | 30.3 | render/thread/ms 36.15, render/busy/ms 35.01, main/busy/ms 17.86, stage/render/render 11.73, stage/main/post_update 10.82, render/graph/ms 10.14, stage/render/specialize 9.34, encode//sum 8.38, stage/render/graph_submit 7.83, stage/render/queue 5.92, encode/submit 4.53, stage/main/update 4.22 |
| measure | 15.72 | 32.3 | render/thread/ms 42.06, render/busy/ms 41.13, main/busy/ms 19.49, stage/render/render 13.22, stage/main/post_update 11.89, render/graph/ms 11.87, stage/render/specialize 11.00, encode//sum 9.47, stage/render/graph_submit 8.76, stage/render/queue 7.31, encode/pass:shadow_point_light/sum 6.29, encode/submit 5.33 |
| measure | 17.09 | 34.3 | render/thread/ms 38.47, render/busy/ms 37.46, main/busy/ms 18.30, stage/render/render 12.18, render/graph/ms 10.75, stage/main/post_update 10.74, stage/render/specialize 9.63, encode//sum 9.31, stage/render/graph_submit 8.01, stage/render/queue 7.06, encode/pass:shadow_point_light/sum 4.75, encode/submit 4.70 |
| measure | 18.01 | 31.0 | render/thread/ms 37.70, render/busy/ms 36.60, main/busy/ms 18.62, stage/render/render 11.65, stage/main/post_update 11.58, render/graph/ms 10.08, stage/render/specialize 9.32, encode//sum 8.89, stage/render/graph_submit 7.63, stage/render/queue 6.38, encode/submit 4.43, stage/main/update 3.94 |
| measure | 18.17 | 31.5 | main/busy/ms 37.56, stage/main/update 23.79, render/thread/ms 20.27, stream/drive/ms 20.13, render/busy/ms 19.24, stage/render/render 11.74, stage/main/post_update 11.26, render/graph/ms 10.12, encode//sum 8.99, stage/render/graph_submit 7.34, encode/submit 4.57, encode/pass:shadow_point_light/sum 3.57 |
| measure | 19.46 | 37.0 | render/thread/ms 43.69, render/busy/ms 42.62, main/busy/ms 20.58, stage/render/render 14.07, encode//sum 13.37, render/graph/ms 12.49, stage/main/post_update 12.21, stage/render/specialize 9.91, stage/render/graph_submit 9.39, encode/pass:shadow_point_light/sum 7.02, stage/render/queue 6.94, stage/main/update 5.72 |
| measure | 20.89 | 30.1 | render/thread/ms 38.12, render/busy/ms 37.08, main/busy/ms 18.37, stage/render/render 11.84, stage/main/post_update 11.36, render/graph/ms 10.37, stage/render/specialize 9.58, encode//sum 8.20, stage/render/graph_submit 7.64, stage/render/queue 7.14, encode/submit 4.71, stage/main/update 4.38 |
| measure | 23.02 | 34.0 | render/thread/ms 41.96, render/busy/ms 40.93, main/busy/ms 24.79, stage/main/post_update 12.81, stage/render/render 12.07, encode//sum 11.50, stage/render/specialize 11.44, render/graph/ms 10.57, stage/render/graph_submit 8.16, stage/render/queue 8.05, stage/main/update 6.58, encode/submit 4.86 |
| measure | 27.29 | 37.6 | render/thread/ms 46.82, render/busy/ms 45.67, main/busy/ms 26.17, stage/render/render 17.26, render/graph/ms 15.71, encode//sum 14.80, stage/main/post_update 13.26, stage/render/graph_submit 12.07, stage/render/specialize 11.21, encode/pass:shadow_point_light/sum 10.04, stage/render/queue 7.73, encode/submit 7.58 |
| measure | 28.80 | 30.3 | render/thread/ms 40.28, render/busy/ms 39.17, main/busy/ms 21.07, stage/main/post_update 13.95, stage/render/render 13.78, encode//sum 12.28, render/graph/ms 12.24, stage/render/specialize 9.84, stage/render/graph_submit 9.27, encode/pass:shadow_point_light/sum 7.61, stage/render/queue 6.28, encode/submit 5.58 |
| measure | 30.47 | 34.2 | render/thread/ms 43.48, render/busy/ms 42.36, main/busy/ms 26.47, stage/render/render 13.72, stage/main/post_update 13.44, render/graph/ms 12.21, stage/render/specialize 11.99, stage/render/graph_submit 9.97, encode//sum 9.96, stage/render/queue 7.53, stage/main/update 6.59, encode/finish_wall 5.46 |
| measure | 35.35 | 32.6 | render/thread/ms 39.75, render/busy/ms 38.70, main/busy/ms 19.92, stage/render/render 12.17, stage/main/post_update 12.08, render/graph/ms 10.67, stage/render/specialize 10.14, encode//sum 8.44, stage/render/graph_submit 8.17, stage/render/queue 6.83, stage/main/update 4.83, encode/submit 4.63 |
| measure | 37.48 | 30.5 | render/thread/ms 40.31, render/busy/ms 39.24, main/busy/ms 19.73, stage/render/render 13.70, stage/main/post_update 12.66, render/graph/ms 12.17, encode//sum 11.05, stage/render/specialize 10.07, stage/render/graph_submit 9.50, stage/render/queue 6.67, encode/submit 5.35, encode/pass:shadow_point_light/sum 4.95 |
| measure | 40.82 | 35.6 | render/thread/ms 40.31, render/busy/ms 39.15, main/busy/ms 18.81, stage/render/render 10.97, stage/main/post_update 10.55, stage/render/specialize 10.22, render/graph/ms 9.39, encode//sum 8.00, stage/render/graph_submit 7.41, stage/render/queue 7.03, stage/main/update 4.51, encode/submit 4.01 |
| measure | 43.22 | 32.3 | render/thread/ms 38.63, render/busy/ms 37.53, main/busy/ms 17.34, stage/render/render 10.95, stage/main/post_update 10.43, stage/render/specialize 10.29, render/graph/ms 9.51, encode//sum 8.06, stage/render/graph_submit 7.61, stage/render/queue 6.89, stage/main/update 4.13, encode/finish_wall 3.87 |
| measure | 45.68 | 35.5 | render/thread/ms 41.38, render/busy/ms 40.28, main/busy/ms 19.19, stage/render/render 11.25, stage/render/specialize 10.82, stage/main/post_update 10.74, render/graph/ms 9.86, encode//sum 9.12, stage/render/graph_submit 7.82, stage/render/queue 7.38, stage/render/prepare_meshes 5.18, stage/main/update 5.16 |
| measure | 56.44 | 32.3 | render/thread/ms 38.84, render/busy/ms 37.75, main/busy/ms 15.74, stage/render/render 10.73, stage/render/specialize 9.46, render/graph/ms 9.35, stage/main/post_update 9.19, stage/render/queue 8.62, encode//sum 8.10, stage/render/graph_submit 7.19, stage/main/update 4.07, stage/render/prepare_meshes 3.85 |
| measure | 59.04 | 46.2 | render/thread/ms 57.72, render/busy/ms 56.11, main/busy/ms 31.69, stage/main/post_update 18.81, stage/render/render 18.16, render/graph/ms 15.91, encode//sum 13.18, stage/render/graph_submit 12.55, stage/render/specialize 10.87, stage/render/prepare_meshes 8.95, stage/main/update 8.47, encode/submit 7.55 |
| measure | 59.07 | 30.5 | render/thread/ms 60.48, render/busy/ms 59.28, main/busy/ms 25.78, stage/render/prepare_assets 14.97, stage/main/post_update 13.57, stage/render/render 11.42, stage/render/specialize 10.89, render/graph/ms 9.85, stage/render/prepare_meshes 9.59, encode//sum 9.21, stage/render/graph_submit 7.59, stage/main/update 6.85 |
| measure | 62.57 | 32.4 | render/thread/ms 40.52, render/busy/ms 39.41, main/busy/ms 20.32, stage/render/render 12.17, stage/main/post_update 11.05, stage/render/specialize 11.02, render/graph/ms 10.72, encode//sum 9.01, stage/render/graph_submit 8.41, stage/render/queue 6.54, stage/main/update 6.51, stage/render/prepare_meshes 4.40 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=matctrl-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 55%) · 2624 MHz (min 2610) · memory 8101 MHz · 79.6 W · 77 °C (max 82) · VRAM 7058 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `mw.matctrl.gate=on (command line: --set)`
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
