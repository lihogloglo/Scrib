# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5901 frames over 65.6 s · **median 10.81 ms (93 fps)** · mean 11.13 · p95 13.86 · p99 18.08 (1% low 55 fps) · worst 43.12 ms · 80 spikes over 1.5× median · 63.1 stutters/min (work 65.8/min)
- warm-up: 325 frames over 9.3 s, worst 2472.7 ms
- hitches (> 27.0 ms): **31** measured, 52 in warm-up
- timed GPU passes: 6.75 ms of a 11.13 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.81 ms, limit 11 ms
- **FAIL** frame_ms: 69 stutters (> 1.5x moving median of 60) in 1.09 min = 63.06/min, limit 1/min
- **FAIL** worst frame 43.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.896 | 2.037 | 2.072 | 0.93 | 0.177 |
| main_opaque_pass_3d | 1.513 | 1.625 | 2.032 | 0.93 | 0.063 |
| early prepass | 0.459 | 0.493 | 0.720 | 0.93 | 0.084 |
| bloom | 0.293 | 0.315 | 0.319 | 0.93 | 0.064 |
| atmosphere_luts | 0.284 | 0.305 | 0.367 | 0.93 | 0.003 |
| water_velocity | 0.260 | 0.280 | 0.347 | 0.93 | 0.002 |
| water_sspr | 0.188 | 0.202 | 0.203 | 0.93 | 0.002 |
| ssao | 0.175 | 0.188 | 0.193 | 0.93 | 0.007 |
| auxiliary_views | 0.144 | 0.155 | 0.565 | 0.93 | 0.155 |
| uniform_allocation | 0.144 | 0.154 | 0.213 | 0.93 | 0.025 |
| atmos_froxel_inject | 0.136 | 0.146 | 0.147 | 0.93 | 0.001 |
| clouds_trace | 0.131 | 0.141 | 0.231 | 0.93 | 0.001 |
| water_fft | 0.123 | 0.132 | 0.139 | 0.93 | 0.003 |
| upscaling | 0.109 | 0.117 | 0.124 | 0.93 | 0.004 |
| bin_unpacking | 0.103 | 0.111 | 0.150 | 0.93 | 0.018 |
| clouds_composite | 0.098 | 0.105 | 0.108 | 0.93 | 0.002 |
| main_transmissive_pass_3d | 0.079 | 0.085 | 0.178 | 0.93 | 0.009 |
| tonemapping | 0.079 | 0.085 | 0.088 | 0.93 | 0.003 |
| clustering | 0.059 | 0.064 | 0.067 | 0.93 | 0.019 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.93 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.93 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.044 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.414 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.67 | 0.002 |
| early_mesh_preprocessing | 0.037 | 0.040 | 0.058 | 0.93 | 0.004 |
| surface_exposure_far | 0.035 | 0.042 | 0.044 | 0.83 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.220 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.93 | 0.177 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.93 | 0.002 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.730 | 12.285 | 37.23 | 57419.4 |
| render/busy/ms | 9.199 | 11.750 | 36.66 | 54281.5 |
| main/busy/ms | 8.717 | 10.472 | 25.41 | 51438.2 |
| stage/render/render | 5.566 | 7.125 | 14.97 | 32843.2 |
| stage/main/post_update | 5.261 | 6.597 | 10.49 | 31046.9 |
| render/graph/ms | 4.810 | 6.325 | 13.85 | 28382.8 |
| encode//sum | 4.195 | 5.711 | 25.22 | 24753.8 |
| stage/render/graph_submit | 3.555 | 4.727 | 11.06 | 20975.3 |
| encode/submit | 2.240 | 3.118 | 7.03 | 13215.9 |
| stage/main/update | 2.170 | 2.591 | 19.40 | 12807.1 |
| encode/finish_wall | 1.374 | 1.969 | 5.80 | 8107.4 |
| stage/render/camera_driver | 1.200 | 1.695 | 3.94 | 7083.8 |
| encode/pass:shadow_point_light/sum | 0.897 | 3.227 | 16.00 | 5293.9 |
| encode/pass:shadow_directional_light/sum | 0.845 | 1.536 | 3.99 | 4985.5 |
| stage/render/prepare_write_phase_buffers | 0.773 | 1.179 | 2.42 | 4563.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.749 | 1.085 | 1.65 | 4418.8 |
| encode/pass:early prepass/sum | 0.726 | 1.065 | 1.96 | 4284.8 |
| stage/render/prepare_assets | 0.571 | 0.734 | 12.90 | 3367.2 |
| stage/render/prepare_meshes | 0.535 | 0.741 | 3.93 | 3154.2 |
| bigspace/total/ms | 0.492 | 1.129 | 2.77 | 2901.2 |
| stage/render/prepare_resources | 0.453 | 0.546 | 1.06 | 2673.9 |
| render/present/ms | 0.436 | 0.613 | 1.85 | 2572.5 |
| stage/main/first | 0.400 | 0.526 | 5.98 | 2363.1 |
| stage/main/pre_update | 0.361 | 0.527 | 1.52 | 2133.1 |
| stage/render/specialize | 0.355 | 0.455 | 13.22 | 2094.3 |
| stage/render/queue | 0.310 | 0.458 | 9.45 | 1829.0 |
| terrain/update/ms | 0.308 | 0.396 | 2.33 | 1818.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.300 | 0.440 | 1.16 | 1769.6 |
| stage/render/prepare_bind_groups | 0.291 | 0.531 | 2.32 | 1717.8 |
| terrain/select/ms | 0.275 | 0.355 | 0.90 | 1620.8 |
| stage/render/prepare_batch_phases | 0.265 | 0.377 | 0.61 | 1564.6 |
| controller/feed/ms | 0.263 | 0.333 | 0.80 | 1553.3 |
| stream/drive/ms | 0.243 | 0.317 | 17.17 | 1434.2 |
| stage/main/fixed_main_loop | 0.222 | 0.376 | 1.61 | 1311.5 |
| bigspace/low_precision/ms | 0.200 | 0.558 | 2.38 | 1182.8 |
| stage/render/create_views | 0.199 | 0.256 | 1.03 | 1172.0 |
| far/select/ms | 0.198 | 0.267 | 0.68 | 1168.8 |
| stream/tick/ms | 0.188 | 0.250 | 0.65 | 1108.7 |
| stage/render/extract_commands | 0.169 | 0.198 | 0.47 | 999.1 |
| stage/render/prepare_flush | 0.169 | 0.315 | 1.20 | 995.9 |
| anim/evaluate/ms | 0.153 | 0.259 | 1.32 | 903.3 |
| anim/evaluate_cpu/ms | 0.149 | 0.188 | 0.69 | 880.7 |
| stage/render/prepare_views | 0.139 | 0.172 | 0.61 | 821.8 |
| bigspace/high_precision/ms | 0.129 | 0.365 | 1.66 | 761.0 |
| controller/move/ms | 0.128 | 0.276 | 0.67 | 756.6 |
| stage/main/last | 0.119 | 0.169 | 1.77 | 699.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.165 | 1.15 | 591.4 |
| render/acquire/ms | 0.096 | 0.120 | 0.57 | 565.3 |
| stage/render/cleanup | 0.081 | 0.211 | 0.74 | 479.8 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.137 | 0.35 | 380.0 |
| stage/render/phase_sort | 0.063 | 0.092 | 0.30 | 369.1 |
| physics/step/ms | 0.048 | 0.088 | 0.62 | 285.7 |
| pipelines/compile_ms | 0.048 | 0.000 | 119.81 | 283.7 |
| pipelines/front_ms | 0.046 | 0.000 | 116.27 | 272.8 |
| physics/insert/ms | 0.035 | 0.376 | 0.89 | 208.0 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.72 | 180.3 |
| weather/map/ms | 0.025 | 0.033 | 0.23 | 146.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.55 | 87.7 |
| anim/apply/ms | 0.012 | 0.019 | 0.42 | 69.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.81 | 66.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.12 | 62.4 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 60.3 |
| script/update/ms | 0.007 | 0.009 | 0.21 | 40.2 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 31.4 |
| stream/despawn/ms | 0.005 | 0.000 | 0.93 | 30.7 |
| script/ui/ms | 0.005 | 0.006 | 0.09 | 28.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 28.1 |
| controller/write/ms | 0.003 | 0.005 | 0.26 | 20.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.59 | 14.1 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.74 | 10.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.59 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.20 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 593 |
| crowd/c0/meshes/count | count | 353 | 646 | 156 |
| crowd/c1/count | count | 146 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5892 | 5603 |
| crowd/c1/meshes/count | count | 1181 | 1255 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4606 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.43 | 1.74 | 0.51 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.55 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.81 | 0.01 |
| encode/finish_critical | ms | 0.97 | 3.99 | 0.98 |
| encode/pass:early prepass/max | ms | 0.73 | 1.96 | 0.58 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 1.65 | 0.98 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 1.15 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 1.16 | 0.36 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 3.99 | 0.37 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.39 | 0.86 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.72 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.09 | 36.66 | 8.57 |
| gpu/timed/ms | ms | 7.96 | 11.30 | 8.31 |
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
| phases/alpha_mask3d/items | count | 3905 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3905 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 4108 | 4330 | 4330 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 4108 | 4330 | 4330 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 205 | 5 |
| phases/shadow/bins | count | 14.4 k | 19.1 k | 12.1 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 40.3 k | 56.7 k | 39.9 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 251 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 19 | 567 | 6 |
| preprocess/bind_groups_created | count | 19 | 567 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 246 | 125 |
| process/cpu_usage | % | 24.76 | 37.91 | 25.24 |
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
| skin/blocks_changed/count | count | 14 | 121 | 13 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 367.4 k | 811.3 k | 493.6 k |
| skin/joints_written/count | count | 5740 | 12.7 k | 7712 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5011 | 5011 |
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
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
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
| system/cpu_usage | % | 30.75 | 44.65 | 33.06 |
| system/mem_usage | % | 56.98 | 57.58 | 57.52 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 71 | 254 | 55 |
| views/all/bins | count | 23.1 k | 27.7 k | 21.7 k |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 69.0 k | 85.3 k | 69.4 k |
| views/all/pipelines | count | 251 | 297 | 77 |
| views/all/pipelines_distinct | count | 70 | 76 | 61 |
| views/all/work_items | count | 41.4 k | 82.5 k | 42.2 k |
| views/cascade/batch_sets | count | 11 | 54 | 5 |
| views/cascade/bins | count | 12.0 k | 15.3 k | 12.1 k |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 39.0 k | 53.1 k | 39.9 k |
| views/cascade/pipelines | count | 38 | 55 | 31 |
| views/cascade/work_items | count | 11.7 k | 52.9 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 3129 | 4305 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3387 | 4595 | 0 |
| views/lamp_face/pipelines | count | 167 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 4123 | 0 |
| views/main/batch_sets | count | 51 | 55 | 50 |
| views/main/bins | count | 8674 | 9602 | 9602 |
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
| anim/bones_written/count | count | 55.16 | 231 | 325.5 k |
| anim/events/count | count | 0.10 | 3 | 563 |
| assets/Image/added | count | 0.10 | 42 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.67 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1298 | 4675 |
| encode//count | count | 33.82 | 220 | 199.6 k |
| encode/bind_walk/init_textures | count | 12.8 k | 114.7 k | 75.55 M |
| encode/bind_walk/init_textures_skipped | count | 62.8 k | 144.4 k | 370.38 M |
| encode/bind_walk/inits | count | 523 | 1244 | 3.09 M |
| encode/bind_walk/inits_skipped | count | 132 | 201 | 778.7 k |
| encode/bind_walk/merge_textures | count | 12.7 k | 114.6 k | 75.12 M |
| encode/bind_walk/merge_textures_skipped | count | 5503 | 6026 | 32.47 M |
| encode/bind_walk/merges | count | 102 | 337 | 599.6 k |
| encode/bind_walk/merges_skipped | count | 109 | 156 | 642.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5901 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5901 |
| encode/pass:early prepass/count | count | 1 | 1 | 5901 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5901 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5901 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5901 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5901 |
| encode/pass:shadow_point_light/count | count | 1.46 | 48 | 8633 |
| encode/queue/command_buffers | count | 45.30 | 278 | 267.3 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 2.34 M | 3.48 M | 13.79 G |
| encode/queue/write_buffer/calls | count | 267 | 848 | 1.57 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.8 k | 3.53 M | 1.05 G |
| encode/queue/write_buffer_with/calls | count | 11.05 | 465 | 65.2 k |
| encode/queue/write_texture/bytes | bytes | 12.1 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5901 |
| physics/inserts | count | 0.38 | 111 | 2266 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
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
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5737 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1258 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.0 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2472.7 | main/busy/ms 2439, stage/main/post_update 2331, stage/main/update 98.46, surface/advance_cpu_ms 67.97, stage/main/pre_update 5.07, stage/main/last 1.20, surface/cell_upload_ms 0.72, stream/drive/ms 0.61, stage/main/first 0.49, stage/main/fixed_main_loop 0.37, weather/map/ms 0.17, stream/tick/ms 0.16 |
| warm-up | 3.29 | 820.0 | main/busy/ms 2615, stage/main/post_update 2358, render/thread/ms 786, render/busy/ms 616, stage/render/prepare_assets 557, stage/main/update 242, stage/render/prepare_views 165, render/acquire/ms 165, surface/advance_cpu_ms 67.97, stage/render/create_views 28.10, stage/render/render 13.93, stage/render/prepare_bind_groups 10.52 |
| warm-up | 3.40 | 111.4 | render/thread/ms 880, render/busy/ms 660, stage/render/prepare_assets 585, main/busy/ms 200, stage/render/prepare_views 166, render/acquire/ms 165, stage/main/update 148, stage/render/render 69.68, pipelines/compile_ms 56.05, render/present/ms 54.59, pipelines/driver_ms 39.19, stage/main/post_update 38.33 |
| warm-up | 3.45 | 45.8 | pipelines/compile_ms 373, pipelines/front_ms 276, render/thread/ms 125, pipelines/driver_ms 97.27, stage/render/render 78.99, pipelines/wait_ms 71.29, main/busy/ms 69.93, render/busy/ms 65.71, render/present/ms 58.28, stage/main/post_update 48.79, stage/render/prepare_assets 29.61, terrain/update/ms 28.02 |
| warm-up | 3.49 | 36.4 | pipelines/wait_ms 806, pipelines/compile_ms 634, pipelines/front_ms 561, main/busy/ms 108, stage/main/post_update 77.96, pipelines/driver_ms 72.83, render/thread/ms 62.60, render/busy/ms 52.17, stage/render/render 28.58, render/graph/ms 17.56, stage/render/prepare_assets 14.82, terrain/update/ms 13.62 |
| warm-up | 3.58 | 92.4 | pipelines/wait_ms 4459, pipelines/compile_ms 1164, pipelines/front_ms 1099, main/busy/ms 182, render/thread/ms 92.40, render/busy/ms 89.95, pipelines/driver_ms 65.33, stage/main/update 61.08, stage/main/pre_update 57.41, stage/main/post_update 50.45, stage/render/prepare_assets 23.35, stage/render/prepare_write_phase_buffers 18.25 |
| warm-up | 3.71 | 132.9 | pipelines/wait_ms 4188, pipelines/compile_ms 2326, pipelines/front_ms 2192, main/busy/ms 135, pipelines/driver_ms 133, render/thread/ms 130, render/busy/ms 127, stage/render/prepare_assets 59.16, stage/main/update 56.50, stage/main/pre_update 51.23, stage/render/render 22.92, render/graph/ms 20.51 |
| warm-up | 3.76 | 44.9 | pipelines/compile_ms 2349, pipelines/front_ms 2187, pipelines/wait_ms 627, pipelines/driver_ms 162, render/thread/ms 112, render/busy/ms 109, stage/render/prepare_assets 77.83, main/busy/ms 54.19, stage/render/render 26.16, stage/main/pre_update 23.18, render/graph/ms 23.00, encode//sum 19.91 |
| warm-up | 3.79 | 33.6 | pipelines/compile_ms 1202, pipelines/front_ms 1079, pipelines/wait_ms 164, pipelines/driver_ms 123, render/thread/ms 74.89, render/busy/ms 72.07, main/busy/ms 55.15, stage/render/prepare_assets 50.94, stage/main/pre_update 28.05, encode//sum 19.77, stage/main/post_update 16.24, stage/render/render 15.37 |
| warm-up | 3.82 | 31.6 | pipelines/compile_ms 396, pipelines/front_ms 343, render/thread/ms 62.78, render/busy/ms 60.98, pipelines/driver_ms 53.74, main/busy/ms 36.45, stage/render/prepare_assets 23.15, stage/main/post_update 20.49, stage/render/render 11.29, stage/render/prepare_resources 9.59, stage/render/prepare_write_phase_buffers 9.21, render/graph/ms 8.94 |
| warm-up | 3.97 | 92.5 | pipelines/compile_ms 1572, pipelines/front_ms 1508, pipelines/driver_ms 64.26, main/busy/ms 63.20, render/thread/ms 53.66, render/busy/ms 52.37, stage/main/update 49.52, stream/drive/ms 43.66, stage/render/prepare_assets 33.41, stage/render/render 13.69, render/graph/ms 12.06, stage/main/post_update 8.75 |
| warm-up | 4.02 | 29.3 | pipelines/compile_ms 309, pipelines/front_ms 295, render/thread/ms 37.57, render/busy/ms 34.23, main/busy/ms 30.95, stage/main/update 17.82, stage/render/prepare_assets 16.58, pipelines/driver_ms 14.79, stage/render/render 12.60, stream/drive/ms 11.05, render/graph/ms 8.94, stage/main/post_update 7.34 |
| warm-up | 4.11 | 64.7 | render/thread/ms 68.27, render/busy/ms 66.74, main/busy/ms 47.05, stage/render/prepare_assets 39.72, stage/main/update 28.83, stream/drive/ms 24.07, stage/render/render 20.35, render/graph/ms 17.94, pipelines/compile_ms 16.89, pipelines/front_ms 16.53, stage/render/graph_submit 14.50, encode/submit 12.71 |
| warm-up | 4.22 | 87.3 | pipelines/compile_ms 1060, pipelines/front_ms 1029, render/thread/ms 72.85, render/busy/ms 71.12, main/busy/ms 61.85, stage/render/prepare_assets 54.61, stage/main/update 37.91, pipelines/driver_ms 30.98, stage/main/post_update 15.15, stage/render/render 9.87, render/graph/ms 7.89, encode//sum 5.77 |
| warm-up | 4.30 | 82.6 | pipelines/compile_ms 1400, pipelines/front_ms 1353, render/thread/ms 120, render/busy/ms 116, stage/render/prepare_assets 87.22, main/busy/ms 60.44, pipelines/driver_ms 46.50, stage/main/update 38.83, stage/render/render 22.88, render/graph/ms 19.55, stage/render/graph_submit 16.99, encode/submit 14.82 |
| warm-up | 4.36 | 53.1 | pipelines/compile_ms 750, pipelines/front_ms 721, pipelines/wait_ms 256, render/thread/ms 112, render/busy/ms 109, main/busy/ms 53.53, stage/render/prepare_assets 50.72, pipelines/driver_ms 28.85, stage/main/post_update 27.20, stage/render/render 25.63, render/graph/ms 21.94, stage/render/prepare_resources 18.40 |
| warm-up | 4.44 | 83.2 | pipelines/compile_ms 1132, pipelines/front_ms 1097, pipelines/wait_ms 385, render/thread/ms 116, render/busy/ms 114, main/busy/ms 56.69, stage/render/prepare_assets 41.27, pipelines/driver_ms 35.32, stage/main/post_update 26.63, stage/render/prepare_resources 25.06, stage/render/render 17.77, encode//sum 16.45 |
| warm-up | 4.53 | 91.4 | pipelines/compile_ms 2531, pipelines/front_ms 2454, pipelines/wait_ms 404, render/thread/ms 156, render/busy/ms 154, pipelines/driver_ms 77.36, encode//sum 60.00, main/busy/ms 57.83, stage/render/prepare_assets 50.30, stage/render/render 46.07, render/graph/ms 43.41, stage/main/update 25.91 |
| warm-up | 4.62 | 88.3 | pipelines/compile_ms 2325, pipelines/front_ms 2248, pipelines/wait_ms 461, render/thread/ms 162, render/busy/ms 161, pipelines/driver_ms 77.61, encode//sum 64.94, main/busy/ms 61.39, stage/render/render 46.88, stage/render/prepare_assets 46.66, render/graph/ms 44.44, stage/main/post_update 33.25 |
| warm-up | 4.66 | 41.3 | pipelines/compile_ms 1157, pipelines/front_ms 1115, pipelines/wait_ms 230, render/thread/ms 113, render/busy/ms 112, main/busy/ms 95.61, stage/main/update 67.11, stream/drive/ms 58.84, pipelines/driver_ms 41.75, stage/render/prepare_assets 35.89, stage/render/render 19.92, encode//sum 19.13 |
| warm-up | 4.74 | 75.5 | pipelines/compile_ms 1982, pipelines/front_ms 1920, pipelines/wait_ms 393, main/busy/ms 113, stage/main/update 83.20, render/thread/ms 71.69, render/busy/ms 69.35, pipelines/driver_ms 62.09, stream/drive/ms 59.77, stage/render/render 21.31, render/graph/ms 18.37, stage/main/post_update 16.85 |
| warm-up | 4.81 | 77.3 | pipelines/compile_ms 2014, pipelines/front_ms 1952, pipelines/wait_ms 578, main/busy/ms 126, stage/main/update 95.80, render/thread/ms 90.42, render/busy/ms 88.19, pipelines/driver_ms 61.80, stream/drive/ms 52.78, stage/render/prepare_assets 30.41, stage/render/render 26.69, render/graph/ms 23.58 |
| warm-up | 4.99 | 177.7 | pipelines/compile_ms 2197, pipelines/front_ms 2045, pipelines/wait_ms 244, render/thread/ms 166, render/busy/ms 164, pipelines/driver_ms 151, main/busy/ms 114, stage/render/prepare_assets 110, stage/main/update 88.70, stream/drive/ms 52.17, stage/render/render 25.53, render/graph/ms 22.74 |
| warm-up | 5.04 | 43.5 | pipelines/compile_ms 1604, pipelines/front_ms 1466, render/thread/ms 147, render/busy/ms 144, pipelines/driver_ms 138, stage/render/prepare_assets 95.32, main/busy/ms 91.66, stage/main/update 64.64, stream/drive/ms 42.63, stage/render/render 32.85, render/graph/ms 29.37, pipelines/wait_ms 23.69 |
| warm-up | 5.10 | 68.7 | pipelines/compile_ms 109, pipelines/front_ms 102, main/busy/ms 82.84, render/thread/ms 66.06, render/busy/ms 63.56, stage/main/update 60.71, stream/drive/ms 42.23, stage/render/prepare_assets 26.45, stage/render/render 26.01, render/graph/ms 22.95, stage/render/graph_submit 20.07, encode/submit 18.80 |
| warm-up | 5.19 | 55.4 | pipelines/compile_ms 905, pipelines/front_ms 880, main/busy/ms 60.15, render/thread/ms 47.14, stage/main/update 45.79, render/busy/ms 45.35, stream/drive/ms 34.75, pipelines/driver_ms 25.37, stage/render/render 13.41, stage/render/prepare_assets 12.33, render/graph/ms 11.24, stage/main/post_update 8.38 |
| warm-up | 5.31 | 31.3 | pipelines/compile_ms 190, pipelines/front_ms 184, render/thread/ms 52.61, render/busy/ms 51.02, main/busy/ms 38.91, stage/main/update 17.42, stage/main/post_update 14.30, stage/render/render 14.03, stage/render/prepare_assets 13.66, render/graph/ms 11.74, encode//sum 10.02, stage/render/graph_submit 8.63 |
| warm-up | 5.36 | 48.8 | pipelines/compile_ms 126, pipelines/front_ms 122, render/thread/ms 64.77, render/busy/ms 63.58, main/busy/ms 38.53, stage/render/prepare_assets 33.07, stage/main/update 19.77, stage/render/render 11.81, stage/main/post_update 11.78, render/graph/ms 9.99, encode//sum 9.34, stage/render/graph_submit 7.10 |
| warm-up | 5.44 | 33.8 | pipelines/compile_ms 760, pipelines/front_ms 737, render/thread/ms 49.78, render/busy/ms 47.78, main/busy/ms 39.31, stage/main/post_update 23.53, pipelines/driver_ms 23.11, stage/render/render 19.11, render/graph/ms 15.80, stage/main/update 11.55, stage/render/graph_submit 11.29, encode//sum 9.62 |
| warm-up | 5.51 | 42.2 | pipelines/compile_ms 854, pipelines/front_ms 833, render/thread/ms 56.13, render/busy/ms 54.77, main/busy/ms 32.33, stage/render/prepare_assets 29.99, pipelines/driver_ms 21.55, stage/main/update 14.14, stage/main/post_update 12.43, stage/render/render 11.66, render/graph/ms 9.77, encode//sum 7.59 |
| warm-up | 5.57 | 31.4 | pipelines/compile_ms 131, pipelines/front_ms 125, render/thread/ms 53.09, render/busy/ms 51.33, main/busy/ms 42.47, stage/main/update 22.58, stage/render/prepare_assets 19.16, stage/main/post_update 13.61, stage/render/render 11.83, render/graph/ms 9.64, encode//sum 7.70, stage/render/graph_submit 6.72 |
| warm-up | 5.60 | 30.4 | pipelines/compile_ms 201, pipelines/front_ms 195, render/thread/ms 56.13, render/busy/ms 54.43, main/busy/ms 43.35, stage/main/update 23.96, stage/render/prepare_assets 22.55, stage/main/post_update 13.73, stream/drive/ms 13.10, stage/render/render 12.68, render/graph/ms 10.38, encode//sum 7.64 |
| warm-up | 5.63 | 37.5 | pipelines/compile_ms 407, pipelines/front_ms 394, render/thread/ms 57.91, render/busy/ms 56.20, main/busy/ms 37.43, stage/render/prepare_assets 25.92, stage/main/update 20.23, stream/drive/ms 13.19, pipelines/driver_ms 12.87, stage/render/render 12.60, stage/main/post_update 12.11, render/graph/ms 10.38 |
| warm-up | 5.68 | 28.8 | pipelines/compile_ms 569, pipelines/front_ms 547, render/thread/ms 45.66, render/busy/ms 44.28, main/busy/ms 26.50, stage/render/prepare_assets 22.94, pipelines/driver_ms 21.96, stage/render/render 13.64, render/graph/ms 11.43, stage/main/post_update 10.60, stage/main/update 10.47, encode//sum 9.48 |
| warm-up | 6.22 | 32.9 | render/thread/ms 47.46, render/busy/ms 46.34, main/busy/ms 30.82, stage/render/prepare_assets 18.71, stage/main/update 14.33, stage/main/post_update 11.12, stage/render/render 7.93, stage/render/specialize 7.31, encode//sum 6.48, render/graph/ms 6.44, stream/drive/ms 4.52, stage/render/graph_submit 4.43 |
| warm-up | 6.31 | 29.1 | pipelines/compile_ms 146, pipelines/front_ms 142, render/thread/ms 38.65, render/busy/ms 37.63, main/busy/ms 31.04, stage/main/update 15.57, stage/main/post_update 11.47, stage/render/prepare_assets 10.16, stage/render/render 8.21, render/graph/ms 6.78, stage/render/specialize 6.65, encode//sum 6.60 |
| warm-up | 6.43 | 29.3 | render/thread/ms 45.47, render/busy/ms 44.27, main/busy/ms 36.29, stage/main/update 17.94, stage/render/specialize 15.00, stage/main/post_update 13.15, stage/render/render 10.35, render/graph/ms 8.75, encode//sum 7.16, stage/render/graph_submit 6.42, stage/render/queue 4.89, stream/drive/ms 4.89 |
| warm-up | 6.47 | 33.3 | render/thread/ms 54.51, render/busy/ms 53.36, main/busy/ms 40.55, stage/render/specialize 18.39, stage/main/update 17.57, stage/main/post_update 14.90, stage/render/render 8.87, stage/render/prepare_assets 8.55, encode//sum 7.49, render/graph/ms 7.34, stage/render/queue 6.77, stage/render/graph_submit 5.19 |
| warm-up | 6.57 | 33.0 | main/busy/ms 41.14, render/thread/ms 35.65, render/busy/ms 34.26, stage/main/update 23.76, stream/drive/ms 14.91, stage/main/post_update 12.90, stage/render/render 10.50, encode//sum 8.94, render/graph/ms 8.47, stage/render/graph_submit 5.84, stage/render/prepare_assets 5.57, stage/render/specialize 4.84 |
| warm-up | 6.60 | 29.1 | render/thread/ms 43.70, render/busy/ms 42.09, main/busy/ms 37.14, stage/main/update 16.58, stage/render/render 15.36, stage/main/post_update 14.77, render/graph/ms 12.91, encode//sum 10.35, stage/render/prepare_assets 9.70, stage/render/graph_submit 9.17, encode/submit 6.09, stream/drive/ms 3.78 |
| warm-up | 6.65 | 28.6 | render/thread/ms 42.61, render/busy/ms 41.62, main/busy/ms 30.13, stage/render/prepare_assets 17.79, stage/main/update 14.55, stage/main/post_update 10.19, stage/render/render 9.06, render/graph/ms 7.52, encode//sum 7.14, stage/render/graph_submit 5.43, stage/render/specialize 5.17, stream/drive/ms 3.48 |
| warm-up | 7.04 | 38.8 | pipelines/compile_ms 130, pipelines/front_ms 126, main/busy/ms 44.79, render/thread/ms 33.28, render/busy/ms 32.19, stage/main/update 24.66, stream/drive/ms 13.48, stage/render/prepare_assets 12.69, stage/main/post_update 12.52, stage/render/render 8.71, encode//sum 7.80, render/graph/ms 7.10 |
| warm-up | 7.12 | 36.5 | render/thread/ms 48.40, render/busy/ms 47.14, main/busy/ms 21.86, stage/render/specialize 16.31, stage/main/post_update 12.53, stage/render/render 12.44, render/graph/ms 10.69, encode//sum 8.91, stage/render/graph_submit 8.21, stage/render/queue 7.39, stage/main/update 6.30, encode/submit 4.59 |
| measure | 0.39 | 38.1 | render/thread/ms 44.66, render/busy/ms 43.54, encode//sum 23.57, stage/render/render 18.69, render/graph/ms 16.97, main/busy/ms 16.04, encode/pass:shadow_point_light/sum 12.49, stage/render/graph_submit 12.26, stage/main/post_update 9.30, stage/render/specialize 8.88, encode/submit 6.77, stage/render/queue 6.25 |
| measure | 1.66 | 28.9 | render/thread/ms 35.51, render/busy/ms 34.50, main/busy/ms 16.23, stage/render/render 12.26, render/graph/ms 10.74, stage/main/post_update 9.76, stage/render/specialize 9.68, encode//sum 9.61, stage/render/graph_submit 7.93, stage/render/queue 5.60, encode/submit 4.79, stage/main/update 3.86 |
| measure | 3.12 | 29.3 | render/thread/ms 38.71, render/busy/ms 37.60, main/busy/ms 17.67, stage/render/render 13.09, render/graph/ms 11.50, stage/main/post_update 11.13, encode//sum 10.84, stage/render/specialize 10.03, stage/render/graph_submit 8.63, stage/render/queue 6.38, encode/submit 5.26, encode/pass:shadow_point_light/sum 4.82 |
| measure | 7.13 | 30.7 | render/thread/ms 40.28, render/busy/ms 39.26, main/busy/ms 19.01, stage/render/render 14.14, render/graph/ms 12.53, stage/main/post_update 12.15, encode//sum 10.69, stage/render/specialize 10.32, stage/render/graph_submit 8.68, stage/render/queue 6.74, encode/pass:shadow_point_light/sum 6.34, encode/submit 5.27 |
| measure | 11.16 | 28.9 | render/thread/ms 34.96, render/busy/ms 33.88, main/busy/ms 17.30, stage/render/render 10.96, stage/main/post_update 10.74, stage/render/specialize 9.58, render/graph/ms 9.27, encode//sum 8.31, stage/render/graph_submit 7.02, stage/render/queue 6.35, encode/submit 3.91, stage/main/update 3.85 |
| measure | 15.70 | 30.4 | render/thread/ms 37.84, render/busy/ms 36.77, main/busy/ms 18.40, stage/render/render 11.92, stage/main/post_update 11.75, stage/render/specialize 10.43, render/graph/ms 10.38, encode//sum 9.20, stage/render/graph_submit 7.79, stage/render/queue 6.92, encode/submit 4.47, stage/main/update 4.04 |
| measure | 19.45 | 30.0 | render/thread/ms 39.42, render/busy/ms 38.33, main/busy/ms 20.26, stage/render/render 12.28, stage/main/post_update 11.91, render/graph/ms 10.69, stage/render/specialize 10.21, encode//sum 9.79, stage/render/graph_submit 7.80, stage/render/queue 7.51, stage/main/update 5.39, encode/submit 5.05 |
| measure | 23.04 | 35.1 | render/thread/ms 43.91, render/busy/ms 42.64, main/busy/ms 26.78, stage/main/post_update 14.33, stage/render/render 12.88, stage/render/specialize 12.43, encode//sum 11.20, render/graph/ms 11.16, stage/render/graph_submit 8.40, stage/render/queue 7.96, stage/main/update 6.42, encode/submit 5.12 |
| measure | 24.64 | 28.8 | render/thread/ms 38.00, render/busy/ms 36.88, main/busy/ms 18.81, stage/render/render 12.20, stage/main/post_update 11.97, render/graph/ms 10.53, stage/render/specialize 9.93, encode//sum 9.59, stage/render/graph_submit 7.80, stage/render/queue 7.17, encode/submit 5.00, stage/main/update 4.40 |
| measure | 27.30 | 38.9 | render/thread/ms 51.82, render/busy/ms 50.70, main/busy/ms 28.15, stage/render/render 17.82, render/graph/ms 16.15, stage/main/post_update 15.62, encode//sum 15.03, stage/render/specialize 13.58, stage/render/graph_submit 12.38, stage/render/queue 9.80, encode/pass:shadow_point_light/sum 9.59, encode/submit 7.82 |
| measure | 28.88 | 31.0 | render/thread/ms 39.01, render/busy/ms 38.02, main/busy/ms 19.10, stage/render/render 12.36, stage/main/post_update 11.88, render/graph/ms 10.85, stage/render/specialize 10.48, encode//sum 9.44, stage/render/graph_submit 8.30, stage/render/queue 7.29, encode/submit 5.09, stage/main/update 4.38 |
| measure | 30.48 | 37.9 | render/thread/ms 43.73, render/busy/ms 42.50, main/busy/ms 22.10, stage/render/render 13.24, stage/main/post_update 12.63, stage/render/specialize 11.42, render/graph/ms 11.30, encode//sum 9.54, stage/render/graph_submit 8.63, stage/render/queue 8.33, stage/main/update 5.27, encode/submit 4.98 |
| measure | 32.00 | 30.8 | render/thread/ms 38.94, render/busy/ms 37.81, main/busy/ms 18.46, stage/render/render 12.11, stage/main/post_update 11.55, render/graph/ms 10.49, stage/render/specialize 10.44, encode//sum 9.20, stage/render/graph_submit 7.94, stage/render/queue 6.79, encode/submit 4.52, stage/main/update 4.46 |
| measure | 35.36 | 32.2 | render/thread/ms 41.30, render/busy/ms 40.20, main/busy/ms 21.76, stage/main/post_update 14.48, stage/render/render 13.56, render/graph/ms 11.95, stage/render/specialize 10.88, encode//sum 10.16, stage/render/graph_submit 9.01, stage/render/queue 7.29, encode/submit 5.13, stage/main/update 4.49 |
| measure | 37.48 | 29.3 | render/thread/ms 38.59, render/busy/ms 37.65, main/busy/ms 19.06, stage/render/render 12.73, encode//sum 12.41, stage/main/post_update 11.51, render/graph/ms 11.21, stage/render/specialize 10.37, stage/render/graph_submit 8.41, stage/render/queue 6.50, encode/pass:shadow_point_light/sum 5.62, encode/submit 5.18 |
| measure | 40.79 | 33.1 | render/thread/ms 36.89, render/busy/ms 35.80, main/busy/ms 19.54, stage/main/post_update 11.28, stage/render/specialize 10.43, stage/render/render 9.54, render/graph/ms 8.08, encode//sum 7.15, stage/render/queue 6.72, stage/render/graph_submit 6.09, stage/main/update 4.87, encode/submit 3.67 |
| measure | 43.22 | 29.6 | render/thread/ms 37.38, render/busy/ms 36.27, main/busy/ms 18.31, stage/main/post_update 11.15, stage/render/render 11.06, stage/render/specialize 10.48, render/graph/ms 9.47, encode//sum 7.67, stage/render/graph_submit 7.60, stage/render/queue 6.77, stage/main/update 4.44, encode/submit 4.43 |
| measure | 45.68 | 33.5 | render/thread/ms 40.70, render/busy/ms 39.64, main/busy/ms 16.81, stage/render/render 12.01, stage/render/specialize 11.69, render/graph/ms 10.49, stage/main/post_update 9.74, stage/render/graph_submit 8.42, encode//sum 8.18, stage/render/queue 6.95, encode/finish_wall 4.39, stage/main/update 4.31 |
| measure | 59.02 | 43.1 | render/thread/ms 56.05, render/busy/ms 54.64, main/busy/ms 29.21, stage/render/render 19.71, render/graph/ms 17.82, stage/main/post_update 15.86, stage/render/graph_submit 14.51, encode//sum 11.42, stage/render/specialize 10.71, stage/main/update 8.95, encode/finish_wall 7.59, encode/submit 7.12 |
| measure | 59.06 | 34.6 | render/thread/ms 58.42, render/busy/ms 57.38, main/busy/ms 22.78, stage/render/prepare_assets 18.58, stage/render/specialize 11.16, stage/main/post_update 10.50, stage/render/render 10.17, render/graph/ms 8.86, encode//sum 8.12, stage/main/update 7.06, stage/render/graph_submit 6.75, stage/render/queue 6.53 |
| measure | 62.57 | 34.2 | render/thread/ms 37.84, render/busy/ms 36.36, main/busy/ms 18.86, stage/render/specialize 11.60, stage/render/render 11.29, stage/main/post_update 9.65, render/graph/ms 9.47, encode//sum 8.76, stage/render/graph_submit 7.14, stage/render/queue 6.97, stage/main/update 5.79, encode/submit 3.92 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=nocull-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 0%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **78%** (min 40%) · 2622 MHz (min 2610) · memory 8101 MHz · 80.7 W · 78 °C (max 83) · VRAM 7023 MiB · throttled 100% of samples

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
