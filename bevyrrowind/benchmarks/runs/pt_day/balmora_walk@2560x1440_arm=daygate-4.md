# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5817 frames over 65.6 s · **median 11.02 ms (91 fps)** · mean 11.28 · p95 13.83 · p99 17.50 (1% low 57 fps) · worst 49.41 ms · 81 spikes over 1.5× median · 64.9 stutters/min (work 54.8/min)
- warm-up: 324 frames over 9.4 s, worst 2657.9 ms
- hitches (> 27.6 ms): **34** measured, 43 in warm-up
- timed GPU passes: 6.75 ms of a 11.28 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.02 ms, limit 11 ms
- **FAIL** frame_ms: 71 stutters (> 1.5x moving median of 60) in 1.09 min = 64.90/min, limit 1/min
- **FAIL** worst frame 49.41 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.898 | 2.033 | 2.064 | 0.93 | 0.173 |
| main_opaque_pass_3d | 1.553 | 1.663 | 2.038 | 0.93 | 0.077 |
| early prepass | 0.458 | 0.490 | 0.722 | 0.93 | 0.089 |
| bloom | 0.294 | 0.314 | 0.321 | 0.93 | 0.062 |
| atmosphere_luts | 0.283 | 0.303 | 0.365 | 0.93 | 0.004 |
| water_velocity | 0.261 | 0.279 | 0.347 | 0.93 | 0.002 |
| water_sspr | 0.188 | 0.201 | 0.203 | 0.93 | 0.003 |
| ssao | 0.176 | 0.188 | 0.193 | 0.93 | 0.008 |
| auxiliary_views | 0.155 | 0.166 | 0.572 | 0.93 | 0.173 |
| atmos_froxel_inject | 0.136 | 0.145 | 0.147 | 0.93 | 0.001 |
| clouds_trace | 0.131 | 0.140 | 0.233 | 0.93 | 0.001 |
| water_fft | 0.123 | 0.132 | 0.139 | 0.93 | 0.003 |
| uniform_allocation | 0.122 | 0.130 | 0.184 | 0.93 | 0.028 |
| upscaling | 0.111 | 0.119 | 0.126 | 0.93 | 0.004 |
| clouds_composite | 0.098 | 0.105 | 0.108 | 0.93 | 0.002 |
| bin_unpacking | 0.098 | 0.105 | 0.141 | 0.93 | 0.020 |
| main_transmissive_pass_3d | 0.079 | 0.085 | 0.178 | 0.93 | 0.009 |
| tonemapping | 0.079 | 0.085 | 0.088 | 0.93 | 0.004 |
| clustering | 0.060 | 0.064 | 0.068 | 0.93 | 0.018 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.93 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.93 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.93 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.67 | 0.002 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.83 | 0.002 |
| lightprobe_irradiance_map | 0.033 | 0.412 | 0.415 | 0.08 | 0.001 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.004 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.93 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.018 | 0.220 | 0.232 | 0.08 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.013 | 0.026 | 0.93 | 0.125 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.816 | 12.095 | 40.82 | 57100.9 |
| render/busy/ms | 9.268 | 11.588 | 40.20 | 53912.1 |
| main/busy/ms | 9.239 | 10.985 | 29.32 | 53743.0 |
| stage/main/post_update | 5.742 | 7.105 | 9.64 | 33400.5 |
| stage/render/render | 5.691 | 7.202 | 21.19 | 33105.6 |
| render/graph/ms | 4.949 | 6.417 | 14.89 | 28785.9 |
| encode//sum | 4.108 | 5.328 | 24.58 | 23894.8 |
| stage/render/graph_submit | 3.707 | 4.850 | 10.30 | 21561.0 |
| encode/submit | 2.347 | 3.247 | 6.66 | 13650.7 |
| stage/main/update | 2.198 | 2.643 | 22.49 | 12784.4 |
| encode/finish_wall | 1.425 | 2.077 | 4.66 | 8292.0 |
| stage/render/camera_driver | 1.188 | 1.635 | 4.53 | 6909.9 |
| encode/pass:shadow_point_light/sum | 1.007 | 3.440 | 16.85 | 5855.2 |
| encode/pass:shadow_directional_light/sum | 0.897 | 1.625 | 3.63 | 5217.4 |
| stage/render/prepare_write_phase_buffers | 0.808 | 1.177 | 2.66 | 4700.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.801 | 1.134 | 2.12 | 4661.3 |
| encode/pass:early prepass/sum | 0.772 | 1.119 | 1.74 | 4491.8 |
| stage/render/prepare_assets | 0.601 | 0.802 | 10.32 | 3497.1 |
| bigspace/total/ms | 0.544 | 1.347 | 2.15 | 3161.6 |
| stage/render/prepare_meshes | 0.471 | 0.606 | 6.93 | 2739.6 |
| stage/render/prepare_resources | 0.471 | 0.567 | 1.01 | 2739.3 |
| render/present/ms | 0.454 | 0.698 | 17.57 | 2642.5 |
| stage/main/first | 0.403 | 0.529 | 5.75 | 2346.8 |
| stage/main/pre_update | 0.367 | 0.517 | 1.47 | 2133.4 |
| stage/render/specialize | 0.347 | 0.440 | 13.44 | 2019.9 |
| stage/render/queue | 0.315 | 0.455 | 7.88 | 1831.0 |
| terrain/update/ms | 0.310 | 0.404 | 1.72 | 1803.0 |
| stage/render/prepare_bind_groups | 0.296 | 0.533 | 2.68 | 1720.7 |
| controller/feed/ms | 0.281 | 0.349 | 0.83 | 1632.8 |
| terrain/select/ms | 0.276 | 0.361 | 0.97 | 1603.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.253 | 0.371 | 1.07 | 1471.2 |
| stream/drive/ms | 0.253 | 0.342 | 20.47 | 1469.6 |
| stage/main/fixed_main_loop | 0.232 | 0.390 | 1.33 | 1350.5 |
| bigspace/low_precision/ms | 0.227 | 0.739 | 1.77 | 1321.3 |
| stage/render/create_views | 0.203 | 0.263 | 0.94 | 1181.4 |
| far/select/ms | 0.200 | 0.277 | 0.63 | 1164.3 |
| stream/tick/ms | 0.194 | 0.271 | 0.58 | 1127.5 |
| stage/render/prepare_batch_phases | 0.193 | 0.277 | 0.60 | 1120.7 |
| stage/render/extract_commands | 0.180 | 0.210 | 0.48 | 1049.8 |
| stage/render/prepare_flush | 0.169 | 0.325 | 1.11 | 985.3 |
| anim/evaluate/ms | 0.160 | 0.250 | 1.86 | 932.9 |
| anim/evaluate_cpu/ms | 0.154 | 0.193 | 0.76 | 897.2 |
| bigspace/high_precision/ms | 0.150 | 0.656 | 1.85 | 873.4 |
| controller/move/ms | 0.144 | 0.262 | 0.64 | 836.7 |
| stage/render/prepare_views | 0.139 | 0.173 | 0.40 | 805.7 |
| stage/main/last | 0.116 | 0.161 | 4.45 | 677.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.105 | 0.166 | 0.82 | 612.0 |
| render/acquire/ms | 0.094 | 0.120 | 0.32 | 546.3 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.133 | 0.45 | 363.4 |
| pipelines/compile_ms | 0.052 | 0.000 | 125.80 | 301.8 |
| physics/step/ms | 0.051 | 0.092 | 0.58 | 297.1 |
| pipelines/front_ms | 0.050 | 0.000 | 121.86 | 289.7 |
| physics/insert/ms | 0.034 | 0.376 | 1.03 | 199.7 |
| encode/resolve_diagnostics/sum | 0.033 | 0.043 | 0.79 | 190.0 |
| stage/render/cleanup | 0.032 | 0.092 | 0.55 | 184.3 |
| stage/render/phase_sort | 0.029 | 0.050 | 0.32 | 168.2 |
| weather/map/ms | 0.025 | 0.034 | 0.32 | 147.1 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.022 | 0.68 | 92.4 |
| anim/apply/ms | 0.012 | 0.021 | 0.16 | 70.4 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.45 | 67.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.23 | 61.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 59.4 |
| script/update/ms | 0.008 | 0.009 | 0.19 | 44.1 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 31.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.13 | 31.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 29.5 |
| script/ui/ms | 0.004 | 0.005 | 0.05 | 23.1 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 20.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.73 | 14.4 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.15 | 12.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.10 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.21 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.09 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 27 | 19 |
| controller/movers/count | count | 20 | 23 | 19 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1292 | 2262 | 649 |
| crowd/c0/meshes/count | count | 353 | 646 | 177 |
| crowd/c1/count | count | 145 | 155 | 146 |
| crowd/c1/joints/count | count | 5527 | 5936 | 5515 |
| crowd/c1/meshes/count | count | 1150 | 1268 | 1156 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4613 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.43 | 1.63 | 0.37 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.68 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.45 | 0.01 |
| encode/finish_critical | ms | 1.00 | 3.63 | 0.90 |
| encode/pass:early prepass/max | ms | 0.80 | 1.74 | 0.90 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 2.12 | 0.90 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.82 | 0.17 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 1.07 | 0.31 |
| encode/pass:shadow_directional_light/max | ms | 0.83 | 3.63 | 0.70 |
| encode/pass:shadow_point_light/max | ms | 0.83 | 1.52 | 0.94 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.79 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.57 | 40.20 | 9.06 |
| gpu/timed/ms | ms | 7.98 | 11.35 | 8.21 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 811 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 811 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3400 | 5094 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3400 | 5094 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 235 | 4 |
| phases/shadow/bins | count | 5028 | 8982 | 2404 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.3 k | 6351 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 253 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 629 | 4 |
| preprocess/bind_groups_created | count | 13 | 629 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 217 | 125 |
| process/cpu_usage | % | 26.51 | 38.94 | 22.60 |
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
| skin/blocks_changed/count | count | 14 | 126 | 13 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.2 k | 810.2 k | 486.7 k |
| skin/joints_written/count | count | 5768 | 12.7 k | 7604 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4916 | 5009 | 5009 |
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
| system/cpu_usage | % | 31.57 | 51.40 | 33.81 |
| system/mem_usage | % | 54.20 | 55.51 | 55.44 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 69 | 282 | 54 |
| views/all/bins | count | 7555 | 11.9 k | 5902 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.2 k | 14.0 k |
| views/all/pipelines | count | 249 | 297 | 75 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 33.5 k | 8123 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2788 | 6186 | 2404 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8904 | 23.5 k | 6351 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 409 | 22.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 197 | 0 |
| views/lamp_face/bins | count | 2170 | 2851 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2405 | 3162 | 0 |
| views/lamp_face/pipelines | count | 170 | 202 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2921 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2860 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8414 | 13.0 k | 7614 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8575 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 56.56 | 241 | 329.0 k |
| anim/events/count | count | 0.09 | 3 | 532 |
| assets/Image/added | count | 0.10 | 43 | 592 |
| assets/Image/modified | count | 0.00 | 3 | 19 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.70 | 152 | 4061 |
| assets/Mesh/removed | count | 0.83 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.73 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.80 | 1260 | 4675 |
| encode//count | count | 34.12 | 222 | 198.5 k |
| encode/bind_walk/init_textures | count | 12.9 k | 119.2 k | 75.30 M |
| encode/bind_walk/init_textures_skipped | count | 62.0 k | 133.8 k | 360.70 M |
| encode/bind_walk/inits | count | 461 | 1235 | 2.68 M |
| encode/bind_walk/inits_skipped | count | 106 | 157 | 615.2 k |
| encode/bind_walk/merge_textures | count | 12.9 k | 119.1 k | 74.87 M |
| encode/bind_walk/merge_textures_skipped | count | 5368 | 6012 | 31.22 M |
| encode/bind_walk/merges | count | 99.75 | 358 | 580.2 k |
| encode/bind_walk/merges_skipped | count | 82.92 | 131 | 482.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5817 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5817 |
| encode/pass:early prepass/count | count | 1 | 1 | 5817 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5817 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5817 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5817 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5817 |
| encode/pass:shadow_point_light/count | count | 1.54 | 48 | 8949 |
| encode/queue/command_buffers | count | 45.68 | 280 | 265.7 k |
| encode/queue/submits | count | 2.02 | 4 | 11.7 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.88 M | 10.97 G |
| encode/queue/write_buffer/calls | count | 261 | 938 | 1.52 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.2 k | 3.52 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 11.08 | 465 | 64.4 k |
| encode/queue/write_texture/bytes | bytes | 12.5 k | 8.39 M | 72.50 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 565 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5817 |
| physics/inserts | count | 0.39 | 110 | 2264 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 129 | 2335 |
| physics/steps | count | 0.68 | 3 | 3939 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5820 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1276 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.6 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2657.9 | main/busy/ms 2626, stage/main/post_update 2520, stage/main/update 96.93, surface/advance_cpu_ms 67.84, stage/main/pre_update 5.71, stage/main/last 1.00, surface/cell_upload_ms 0.84, stage/main/first 0.55, stream/drive/ms 0.54, stage/main/fixed_main_loop 0.38, stream/tick/ms 0.19, weather/map/ms 0.12 |
| warm-up | 3.47 | 813.8 | main/busy/ms 2807, stage/main/post_update 2547, render/thread/ms 781, render/busy/ms 651, stage/render/prepare_assets 603, stage/main/update 245, stage/render/prepare_views 126, render/acquire/ms 126, surface/advance_cpu_ms 67.84, stage/render/create_views 28.40, stage/render/render 13.20, render/graph/ms 8.29 |
| warm-up | 3.57 | 97.4 | render/thread/ms 862, render/busy/ms 691, stage/render/prepare_assets 630, main/busy/ms 201, stage/main/update 153, stage/render/prepare_views 127, render/acquire/ms 126, pipelines/compile_ms 67.47, stage/render/render 59.47, render/present/ms 44.72, pipelines/front_ms 39.04, stage/main/post_update 37.20 |
| warm-up | 3.61 | 41.1 | pipelines/compile_ms 400, pipelines/front_ms 340, pipelines/wait_ms 193, render/thread/ms 121, stage/render/render 79.51, render/busy/ms 73.26, pipelines/driver_ms 60.48, main/busy/ms 59.82, render/present/ms 46.51, stage/main/post_update 44.51, render/graph/ms 32.23, stage/render/prepare_assets 27.24 |
| warm-up | 3.73 | 121.0 | pipelines/wait_ms 4104, pipelines/compile_ms 1400, pipelines/front_ms 1315, render/thread/ms 158, render/busy/ms 151, pipelines/driver_ms 84.61, main/busy/ms 78.96, stage/main/post_update 66.76, stage/render/render 59.54, render/graph/ms 52.73, stage/render/graph_submit 39.34, stage/render/prepare_assets 38.16 |
| warm-up | 3.80 | 66.9 | pipelines/wait_ms 4444, pipelines/compile_ms 1516, pipelines/front_ms 1435, render/thread/ms 183, render/busy/ms 181, main/busy/ms 114, pipelines/driver_ms 80.44, stage/main/update 57.95, stage/render/prepare_resources 53.89, stage/main/post_update 49.63, stage/render/prepare_assets 48.12, stage/render/render 36.29 |
| warm-up | 3.90 | 98.6 | pipelines/compile_ms 1938, pipelines/front_ms 1837, pipelines/wait_ms 709, render/thread/ms 149, render/busy/ms 146, pipelines/driver_ms 101, main/busy/ms 92.73, stage/render/prepare_assets 65.77, stage/main/update 59.65, stage/render/render 34.10, stage/render/prepare_resources 33.59, render/graph/ms 30.42 |
| warm-up | 3.96 | 59.3 | pipelines/compile_ms 2357, pipelines/front_ms 2227, pipelines/wait_ms 242, render/thread/ms 141, render/busy/ms 139, pipelines/driver_ms 130, stage/render/prepare_assets 103, main/busy/ms 32.97, stage/render/render 30.13, render/graph/ms 27.74, encode//sum 18.09, stage/main/post_update 16.21 |
| warm-up | 4.11 | 79.5 | pipelines/compile_ms 1104, pipelines/front_ms 1057, main/busy/ms 60.48, render/thread/ms 48.63, stage/main/update 48.28, pipelines/driver_ms 47.25, stream/drive/ms 43.99, render/busy/ms 43.74, pipelines/wait_ms 43.15, stage/render/prepare_assets 22.58, stage/render/render 15.89, render/graph/ms 10.67 |
| warm-up | 4.19 | 55.9 | pipelines/compile_ms 570, pipelines/front_ms 545, render/thread/ms 59.14, render/busy/ms 57.39, main/busy/ms 39.36, stage/render/prepare_assets 37.82, pipelines/wait_ms 27.23, pipelines/lock_ms 26.55, pipelines/driver_ms 24.80, stage/main/update 21.98, stream/drive/ms 12.03, stage/render/render 11.92 |
| warm-up | 4.24 | 50.1 | pipelines/compile_ms 638, pipelines/front_ms 615, render/thread/ms 71.26, render/busy/ms 69.26, main/busy/ms 50.90, stage/render/prepare_assets 49.25, stage/main/update 31.70, pipelines/driver_ms 23.19, stream/drive/ms 21.59, stage/main/post_update 13.54, stage/render/render 12.30, render/graph/ms 10.08 |
| warm-up | 4.28 | 43.5 | pipelines/compile_ms 192, pipelines/front_ms 188, render/thread/ms 64.81, render/busy/ms 62.74, main/busy/ms 49.75, stage/render/prepare_assets 39.77, stage/main/update 28.24, stream/drive/ms 22.82, stage/render/render 14.30, stage/main/post_update 12.86, render/graph/ms 11.58, stage/render/graph_submit 8.44 |
| warm-up | 4.33 | 53.8 | pipelines/compile_ms 942, pipelines/front_ms 909, render/thread/ms 69.30, render/busy/ms 67.67, stage/render/prepare_assets 45.26, main/busy/ms 34.79, pipelines/driver_ms 33.75, stage/main/update 13.14, stage/render/render 12.77, pipelines/wait_ms 11.64, pipelines/lock_ms 11.26, render/graph/ms 10.38 |
| warm-up | 4.39 | 50.5 | pipelines/compile_ms 1625, pipelines/front_ms 1575, render/thread/ms 75.57, render/busy/ms 74.22, stage/render/prepare_assets 56.20, pipelines/driver_ms 50.00, main/busy/ms 40.41, stage/main/update 17.20, pipelines/wait_ms 13.46, pipelines/lock_ms 12.89, stage/render/render 10.29, stage/main/post_update 10.29 |
| warm-up | 4.43 | 40.4 | pipelines/compile_ms 683, pipelines/front_ms 667, render/thread/ms 71.13, render/busy/ms 69.66, main/busy/ms 51.11, stage/render/prepare_assets 50.09, stage/main/update 32.08, pipelines/driver_ms 16.24, stage/render/render 11.33, stage/main/post_update 9.84, render/graph/ms 9.49, stage/render/graph_submit 7.34 |
| warm-up | 4.50 | 71.6 | pipelines/compile_ms 377, pipelines/front_ms 365, pipelines/wait_ms 149, render/thread/ms 92.64, render/busy/ms 91.17, main/busy/ms 71.29, stage/render/prepare_assets 59.39, stage/main/update 46.27, stage/render/render 12.28, pipelines/driver_ms 12.01, stage/main/post_update 11.62, render/graph/ms 10.44 |
| warm-up | 4.58 | 78.7 | pipelines/compile_ms 850, pipelines/front_ms 824, pipelines/wait_ms 189, render/thread/ms 128, render/busy/ms 126, main/busy/ms 83.48, stage/render/prepare_assets 60.44, stage/main/update 44.28, pipelines/driver_ms 25.95, stage/main/post_update 24.40, stage/render/prepare_resources 18.85, bigspace/total/ms 13.84 |
| warm-up | 4.62 | 42.1 | pipelines/compile_ms 856, pipelines/front_ms 832, render/thread/ms 105, render/busy/ms 103, main/busy/ms 64.35, pipelines/wait_ms 41.42, stage/render/prepare_resources 33.52, stage/main/update 27.83, stage/main/post_update 27.03, pipelines/driver_ms 24.41, stage/render/prepare_assets 22.73, bigspace/total/ms 13.80 |
| warm-up | 4.68 | 60.0 | pipelines/compile_ms 2168, pipelines/front_ms 2100, pipelines/wait_ms 396, render/thread/ms 82.61, render/busy/ms 81.14, pipelines/driver_ms 67.97, main/busy/ms 44.56, stage/render/prepare_assets 28.72, stage/render/prepare_resources 19.22, stage/main/post_update 16.16, stage/main/update 15.96, stage/render/render 13.27 |
| warm-up | 4.77 | 90.1 | pipelines/compile_ms 2765, pipelines/front_ms 2680, pipelines/wait_ms 1253, render/thread/ms 127, render/busy/ms 125, main/busy/ms 103, pipelines/driver_ms 84.95, stage/main/update 68.57, stream/drive/ms 60.70, stage/render/prepare_assets 48.87, stage/render/prepare_resources 38.56, stage/render/render 21.44 |
| warm-up | 4.82 | 52.7 | pipelines/compile_ms 1195, pipelines/front_ms 1161, pipelines/wait_ms 897, render/thread/ms 130, render/busy/ms 128, main/busy/ms 96.40, stage/main/update 65.76, encode//sum 61.41, stream/drive/ms 59.82, stage/render/prepare_resources 48.01, stage/render/render 33.81, pipelines/driver_ms 33.80 |
| warm-up | 4.86 | 42.5 | pipelines/compile_ms 1060, pipelines/front_ms 1027, pipelines/wait_ms 180, main/busy/ms 118, stage/main/update 95.01, render/thread/ms 87.23, render/busy/ms 85.17, encode//sum 76.61, stream/drive/ms 53.62, stage/render/render 37.68, render/graph/ms 34.68, pipelines/driver_ms 33.20 |
| warm-up | 5.01 | 142.6 | pipelines/compile_ms 2134, pipelines/front_ms 2068, pipelines/wait_ms 719, main/busy/ms 120, stage/main/update 97.76, render/thread/ms 87.90, render/busy/ms 85.93, pipelines/driver_ms 65.28, stream/drive/ms 54.24, stage/render/prepare_assets 35.08, encode//sum 32.28, stage/render/render 26.96 |
| warm-up | 5.08 | 78.7 | pipelines/compile_ms 1838, pipelines/front_ms 1780, pipelines/wait_ms 583, render/thread/ms 106, render/busy/ms 101, main/busy/ms 94.32, stage/main/update 70.28, pipelines/driver_ms 58.09, stage/render/prepare_assets 53.61, stream/drive/ms 45.81, stage/render/render 24.23, render/graph/ms 18.64 |
| warm-up | 5.16 | 77.0 | pipelines/compile_ms 871, pipelines/front_ms 837, main/busy/ms 104, render/thread/ms 98.56, render/busy/ms 93.85, stage/main/update 74.91, stage/render/prepare_assets 54.66, stream/drive/ms 45.92, pipelines/driver_ms 33.99, stage/render/render 20.98, stage/main/post_update 18.79, render/graph/ms 15.48 |
| warm-up | 5.25 | 57.6 | pipelines/compile_ms 1038, pipelines/front_ms 1016, main/busy/ms 68.09, stage/main/update 50.84, render/thread/ms 48.04, render/busy/ms 46.59, stream/drive/ms 34.43, pipelines/driver_ms 22.46, stage/render/prepare_assets 18.73, stage/render/render 11.46, render/graph/ms 9.26, stage/main/post_update 9.08 |
| warm-up | 5.28 | 32.7 | pipelines/compile_ms 1661, pipelines/front_ms 1619, render/thread/ms 51.90, render/busy/ms 50.49, pipelines/driver_ms 42.40, main/busy/ms 39.63, stage/main/update 19.95, stage/render/prepare_assets 14.38, stage/main/post_update 12.84, stage/render/render 11.47, render/graph/ms 9.59, encode//sum 8.00 |
| warm-up | 5.36 | 31.0 | pipelines/compile_ms 342, pipelines/front_ms 329, render/thread/ms 45.91, render/busy/ms 44.06, main/busy/ms 39.51, stage/render/render 17.09, stage/main/update 16.74, stage/main/post_update 15.52, render/graph/ms 14.61, pipelines/driver_ms 13.13, stage/render/graph_submit 10.09, encode//sum 10.07 |
| warm-up | 5.43 | 41.9 | pipelines/compile_ms 352, pipelines/front_ms 342, render/thread/ms 55.49, main/busy/ms 53.11, render/busy/ms 51.72, stage/main/update 34.12, stage/render/render 23.06, render/graph/ms 18.93, stage/render/prepare_assets 15.74, stage/render/graph_submit 14.95, stage/main/post_update 13.40, stream/drive/ms 12.35 |
| warm-up | 5.48 | 48.6 | pipelines/compile_ms 729, pipelines/front_ms 708, render/thread/ms 59.48, main/busy/ms 58.84, render/busy/ms 57.05, stage/main/update 39.87, stage/render/prepare_assets 24.83, stage/render/render 21.50, pipelines/driver_ms 20.83, render/graph/ms 18.53, stream/drive/ms 16.00, stage/render/graph_submit 14.24 |
| warm-up | 5.58 | 31.9 | pipelines/compile_ms 612, pipelines/front_ms 591, render/thread/ms 52.85, render/busy/ms 51.47, main/busy/ms 46.20, stage/main/update 25.23, pipelines/driver_ms 21.11, stage/main/post_update 15.33, stage/render/render 14.99, stream/drive/ms 13.18, render/graph/ms 12.94, stage/render/graph_submit 9.46 |
| warm-up | 5.61 | 32.5 | pipelines/compile_ms 399, pipelines/front_ms 383, render/thread/ms 52.68, render/busy/ms 51.25, main/busy/ms 40.19, stage/main/update 23.50, pipelines/driver_ms 15.13, stage/render/prepare_assets 14.95, stage/render/render 13.32, stream/drive/ms 13.19, render/graph/ms 11.29, stage/main/post_update 11.21 |
| warm-up | 6.21 | 32.7 | render/thread/ms 36.94, render/busy/ms 35.97, main/busy/ms 28.62, stage/render/prepare_assets 16.04, stage/main/update 12.30, stage/main/post_update 10.46, stage/render/render 9.35, encode//sum 8.33, render/graph/ms 7.96, stage/render/graph_submit 5.73, stream/drive/ms 4.27, encode/submit 3.26 |
| warm-up | 6.46 | 30.6 | pipelines/compile_ms 142, pipelines/front_ms 137, render/thread/ms 51.91, render/busy/ms 50.73, main/busy/ms 42.97, stage/render/prepare_assets 27.98, stage/main/update 25.10, stage/main/post_update 12.60, stream/drive/ms 12.57, stage/render/render 9.78, encode//sum 8.67, render/graph/ms 7.95 |
| warm-up | 6.66 | 37.9 | main/busy/ms 46.09, render/thread/ms 30.51, render/busy/ms 29.39, stage/main/update 24.95, stage/main/post_update 15.68, stream/drive/ms 14.13, stage/render/render 12.25, render/graph/ms 10.38, pipelines/compile_ms 10.26, pipelines/front_ms 9.60, encode//sum 8.69, stage/render/graph_submit 6.73 |
| warm-up | 6.70 | 33.0 | render/thread/ms 42.25, main/busy/ms 41.54, render/busy/ms 41.13, stage/main/update 19.80, stage/main/post_update 15.95, stage/render/render 14.51, stage/render/prepare_assets 14.02, render/graph/ms 12.38, encode//sum 11.22, stage/render/graph_submit 8.14, stream/drive/ms 7.41, encode/finish_wall 4.71 |
| warm-up | 7.09 | 36.3 | main/busy/ms 44.99, render/thread/ms 31.25, render/busy/ms 29.98, stage/main/update 26.34, stream/drive/ms 14.05, stage/render/prepare_assets 12.29, stage/main/post_update 11.40, stage/render/render 10.46, render/graph/ms 8.70, encode//sum 8.35, stage/render/graph_submit 6.10, encode/submit 3.39 |
| warm-up | 7.18 | 39.8 | pipelines/compile_ms 161, pipelines/front_ms 158, render/thread/ms 50.41, render/busy/ms 48.95, main/busy/ms 27.50, stage/render/render 14.86, render/graph/ms 12.91, stage/render/specialize 12.85, stage/main/post_update 12.61, encode//sum 11.95, stage/main/update 10.38, stage/render/graph_submit 10.15 |
| warm-up | 8.56 | 30.9 | render/thread/ms 38.39, render/busy/ms 37.30, main/busy/ms 18.01, stage/render/render 11.35, stage/main/post_update 10.67, render/graph/ms 9.81, stage/render/specialize 9.68, stage/render/graph_submit 7.71, encode//sum 7.16, stage/render/queue 6.70, stage/main/update 4.73, encode/submit 4.33 |
| measure | 0.42 | 33.2 | render/thread/ms 38.96, render/busy/ms 37.82, main/busy/ms 18.26, stage/render/render 12.85, stage/main/post_update 11.45, render/graph/ms 11.24, stage/render/specialize 9.56, encode//sum 8.97, stage/render/graph_submit 8.74, stage/render/queue 6.47, encode/submit 5.57, stage/main/update 3.90 |
| measure | 1.76 | 32.1 | render/thread/ms 35.82, render/busy/ms 34.64, main/busy/ms 18.48, stage/main/post_update 11.08, stage/render/render 10.53, render/graph/ms 9.09, stage/render/specialize 8.86, encode//sum 7.74, stage/render/graph_submit 7.01, stage/render/queue 6.13, stage/render/prepare_meshes 4.54, stage/main/update 4.28 |
| measure | 3.16 | 33.6 | render/thread/ms 39.29, render/busy/ms 38.10, main/busy/ms 19.50, stage/main/post_update 12.81, stage/render/render 12.58, render/graph/ms 10.95, encode//sum 10.13, stage/render/specialize 9.62, stage/render/graph_submit 8.13, stage/render/queue 6.58, encode/pass:shadow_point_light/sum 5.35, encode/submit 4.65 |
| measure | 5.90 | 31.6 | render/thread/ms 37.22, render/busy/ms 36.18, main/busy/ms 19.69, stage/main/post_update 12.91, stage/render/render 10.53, stage/render/specialize 9.67, render/graph/ms 8.98, encode//sum 8.23, stage/render/queue 7.38, stage/render/graph_submit 6.92, stage/main/update 4.22, encode/submit 3.78 |
| measure | 10.09 | 32.6 | render/thread/ms 38.10, render/busy/ms 37.14, main/busy/ms 19.49, stage/render/render 12.64, stage/main/post_update 11.30, render/graph/ms 11.16, stage/render/specialize 9.58, encode//sum 9.00, stage/render/graph_submit 8.12, stage/render/queue 6.43, stage/main/update 5.68, encode/submit 4.95 |
| measure | 11.47 | 29.9 | render/thread/ms 36.55, render/busy/ms 35.57, main/busy/ms 17.73, stage/render/render 11.63, stage/main/post_update 11.02, render/graph/ms 10.14, stage/render/specialize 9.43, encode//sum 8.46, stage/render/graph_submit 7.66, stage/render/queue 6.33, encode/submit 4.45, stage/main/update 4.25 |
| measure | 15.73 | 31.4 | render/thread/ms 39.56, render/busy/ms 38.46, main/busy/ms 18.76, stage/render/render 12.58, stage/main/post_update 11.18, render/graph/ms 11.01, stage/render/specialize 10.36, stage/render/graph_submit 8.55, encode//sum 8.40, stage/render/queue 7.08, encode/submit 5.02, stage/main/update 4.04 |
| measure | 17.20 | 30.3 | render/thread/ms 38.87, render/busy/ms 37.91, main/busy/ms 19.04, stage/main/post_update 12.21, stage/render/render 12.03, render/graph/ms 10.47, stage/render/specialize 10.07, encode//sum 9.82, stage/render/graph_submit 7.98, stage/render/queue 7.24, encode/submit 4.64, stage/main/update 4.21 |
| measure | 18.01 | 31.9 | render/thread/ms 36.10, render/busy/ms 34.75, main/busy/ms 19.41, stage/main/post_update 11.31, stage/render/render 10.70, stage/render/specialize 9.72, encode//sum 9.26, render/graph/ms 8.90, stage/render/queue 6.78, stage/render/graph_submit 6.42, stage/main/update 4.58, encode/submit 4.00 |
| measure | 18.18 | 31.0 | main/busy/ms 39.03, stage/main/update 24.39, stream/drive/ms 20.70, render/thread/ms 20.32, render/busy/ms 19.39, stage/main/post_update 11.72, stage/render/render 11.71, render/graph/ms 10.32, encode//sum 9.60, stage/render/graph_submit 7.58, encode/submit 4.88, encode/pass:shadow_point_light/sum 4.56 |
| measure | 19.45 | 32.0 | render/thread/ms 39.64, render/busy/ms 38.72, main/busy/ms 20.37, stage/main/post_update 12.48, stage/render/render 11.74, render/graph/ms 10.34, stage/render/specialize 9.81, encode//sum 9.33, stage/render/graph_submit 7.72, stage/render/queue 7.23, stage/main/update 5.44, encode/pass:shadow_point_light/sum 5.25 |
| measure | 20.89 | 32.4 | render/thread/ms 37.29, render/busy/ms 35.84, main/busy/ms 18.54, stage/main/post_update 11.36, stage/render/render 10.94, stage/render/specialize 10.02, render/graph/ms 9.08, encode//sum 8.24, stage/render/queue 7.22, stage/render/graph_submit 6.71, stage/main/update 4.76, encode/submit 4.34 |
| measure | 23.03 | 34.1 | render/thread/ms 44.20, render/busy/ms 43.08, main/busy/ms 25.96, stage/render/render 13.48, stage/main/post_update 13.22, render/graph/ms 11.94, stage/render/specialize 11.40, encode//sum 10.16, stage/render/graph_submit 9.05, stage/render/queue 8.18, stage/main/update 6.98, encode/pass:shadow_point_light/sum 6.27 |
| measure | 24.69 | 31.3 | render/thread/ms 42.35, render/busy/ms 41.30, main/busy/ms 20.56, stage/render/render 14.47, render/graph/ms 13.00, stage/main/post_update 12.67, encode//sum 10.61, stage/render/specialize 10.08, stage/render/graph_submit 9.45, encode/pass:shadow_point_light/sum 8.41, stage/render/queue 6.95, encode/submit 6.39 |
| measure | 27.29 | 38.8 | render/thread/ms 46.18, render/busy/ms 45.00, main/busy/ms 25.96, stage/render/render 16.19, render/graph/ms 14.45, encode//sum 13.05, stage/main/post_update 12.70, stage/render/specialize 11.60, stage/render/graph_submit 11.12, encode/pass:shadow_point_light/sum 8.41, stage/render/queue 7.94, encode/submit 6.64 |
| measure | 30.48 | 32.2 | render/thread/ms 38.15, render/busy/ms 37.10, main/busy/ms 20.49, stage/main/post_update 11.91, stage/render/render 11.38, stage/render/specialize 10.69, render/graph/ms 9.87, stage/render/graph_submit 7.86, encode//sum 7.50, stage/render/queue 6.39, stage/main/update 5.25, encode/submit 4.40 |
| measure | 35.30 | 34.1 | render/thread/ms 45.94, render/busy/ms 44.83, main/busy/ms 20.74, stage/render/render 16.35, render/graph/ms 14.58, stage/main/post_update 13.23, encode//sum 10.99, stage/render/graph_submit 10.90, stage/render/specialize 10.81, stage/render/queue 8.15, encode/pass:shadow_point_light/sum 7.24, encode/submit 6.64 |
| measure | 36.79 | 30.4 | render/thread/ms 40.54, render/busy/ms 39.40, main/busy/ms 19.20, stage/render/render 13.38, stage/main/post_update 11.97, render/graph/ms 11.68, stage/render/specialize 10.06, encode//sum 9.93, stage/render/graph_submit 8.99, stage/render/queue 6.99, encode/pass:shadow_point_light/sum 5.15, encode/submit 5.04 |
| measure | 37.48 | 31.4 | render/thread/ms 40.22, render/busy/ms 39.12, main/busy/ms 19.12, stage/render/render 13.26, stage/main/post_update 12.22, render/graph/ms 11.64, stage/render/specialize 10.22, stage/render/graph_submit 9.02, encode//sum 8.93, stage/render/queue 6.69, encode/submit 5.68, stage/main/update 4.07 |
| measure | 40.82 | 35.5 | render/thread/ms 42.58, render/busy/ms 41.29, main/busy/ms 22.13, stage/main/post_update 13.62, stage/render/render 12.04, stage/render/specialize 10.89, render/graph/ms 10.01, stage/render/graph_submit 7.95, encode//sum 7.64, stage/render/queue 7.21, stage/main/update 4.66, encode/submit 4.39 |
| measure | 43.22 | 30.5 | render/thread/ms 38.25, render/busy/ms 37.20, main/busy/ms 16.98, stage/render/render 10.68, stage/render/specialize 10.21, stage/main/post_update 10.03, render/graph/ms 9.19, encode//sum 8.33, stage/render/graph_submit 7.23, stage/render/queue 6.77, stage/main/update 4.59, encode/submit 4.00 |
| measure | 45.69 | 36.0 | render/thread/ms 41.63, render/busy/ms 40.54, main/busy/ms 17.94, stage/render/render 11.59, stage/render/specialize 10.30, render/graph/ms 10.13, stage/main/post_update 9.98, encode//sum 9.16, stage/render/graph_submit 8.05, stage/render/queue 6.86, stage/render/prepare_meshes 5.14, stage/main/update 4.91 |
| measure | 56.44 | 30.4 | render/thread/ms 36.01, render/busy/ms 34.98, main/busy/ms 14.88, stage/render/render 10.40, stage/render/specialize 9.49, render/graph/ms 9.11, encode//sum 8.44, stage/main/post_update 8.39, stage/render/graph_submit 6.99, stage/render/queue 6.73, stage/main/update 4.19, encode/submit 3.91 |
| measure | 59.05 | 49.4 | render/thread/ms 56.73, render/busy/ms 55.10, main/busy/ms 30.10, stage/render/render 17.53, stage/main/post_update 16.19, render/graph/ms 15.50, stage/render/specialize 13.60, encode//sum 12.69, stage/render/graph_submit 11.79, stage/main/update 9.44, stage/render/prepare_meshes 7.31, encode/submit 6.80 |
| measure | 62.58 | 34.3 | render/thread/ms 37.42, render/busy/ms 36.30, main/busy/ms 19.22, stage/main/post_update 10.51, stage/render/render 10.38, stage/render/specialize 10.15, render/graph/ms 8.96, encode//sum 8.01, stage/render/graph_submit 6.90, stage/render/queue 6.77, stage/main/update 6.21, stage/render/prepare_meshes 4.38 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=daygate-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 13%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **75%** (min 55%) · 2625 MHz (min 2610) · memory 8101 MHz · 78.2 W · 76 °C (max 81) · VRAM 7058 MiB · throttled 100% of samples

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
- `shadow.lamp.day_gate=on (command line: --set)`
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
