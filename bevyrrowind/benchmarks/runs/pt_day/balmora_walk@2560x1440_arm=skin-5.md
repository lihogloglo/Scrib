# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5983 frames over 65.6 s · **median 10.65 ms (94 fps)** · mean 10.97 · p95 13.64 · p99 17.65 (1% low 57 fps) · worst 49.28 ms · 93 spikes over 1.5× median · 62.2 stutters/min (work 58.5/min)
- warm-up: 323 frames over 9.6 s, worst 2750.9 ms
- hitches (> 26.6 ms): **36** measured, 45 in warm-up
- timed GPU passes: 6.67 ms of a 10.97 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.65 ms, limit 11 ms
- **FAIL** frame_ms: 68 stutters (> 1.5x moving median of 60) in 1.09 min = 62.15/min, limit 1/min
- **FAIL** worst frame 49.28 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.888 | 2.040 | 2.073 | 0.93 | 0.175 |
| main_opaque_pass_3d | 1.514 | 1.636 | 2.049 | 0.93 | 0.061 |
| early prepass | 0.464 | 0.502 | 0.736 | 0.93 | 0.089 |
| bloom | 0.291 | 0.315 | 0.321 | 0.93 | 0.063 |
| atmosphere_luts | 0.280 | 0.303 | 0.365 | 0.93 | 0.003 |
| water_velocity | 0.258 | 0.278 | 0.347 | 0.93 | 0.002 |
| water_sspr | 0.186 | 0.201 | 0.203 | 0.93 | 0.002 |
| ssao | 0.174 | 0.188 | 0.191 | 0.93 | 0.007 |
| atmos_froxel_inject | 0.135 | 0.146 | 0.148 | 0.93 | 0.001 |
| clouds_trace | 0.130 | 0.140 | 0.232 | 0.93 | 0.001 |
| auxiliary_views | 0.122 | 0.132 | 0.541 | 0.93 | 0.134 |
| water_fft | 0.121 | 0.130 | 0.137 | 0.93 | 0.003 |
| uniform_allocation | 0.119 | 0.129 | 0.181 | 0.93 | 0.024 |
| upscaling | 0.109 | 0.118 | 0.125 | 0.93 | 0.004 |
| clouds_composite | 0.098 | 0.106 | 0.109 | 0.93 | 0.002 |
| bin_unpacking | 0.095 | 0.103 | 0.140 | 0.93 | 0.018 |
| main_transmissive_pass_3d | 0.079 | 0.086 | 0.179 | 0.93 | 0.008 |
| tonemapping | 0.078 | 0.085 | 0.087 | 0.93 | 0.003 |
| clustering | 0.059 | 0.064 | 0.067 | 0.93 | 0.018 |
| render_sky | 0.048 | 0.052 | 0.052 | 0.93 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.043 | 0.412 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.93 | 0.001 |
| surface_trail | 0.037 | 0.057 | 0.057 | 0.65 | 0.002 |
| surface_exposure_far | 0.035 | 0.043 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.031 | 0.046 | 0.93 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.023 | 0.219 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.026 | 0.93 | 0.127 |
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
| render/thread/ms | 9.400 | 11.756 | 44.34 | 56239.5 |
| main/busy/ms | 9.120 | 10.963 | 25.28 | 54563.9 |
| render/busy/ms | 8.863 | 11.253 | 43.69 | 53027.8 |
| stage/main/post_update | 5.683 | 7.125 | 10.63 | 34001.4 |
| stage/render/render | 5.495 | 7.120 | 14.99 | 32874.3 |
| render/graph/ms | 4.763 | 6.371 | 14.08 | 28499.5 |
| encode//sum | 3.944 | 5.330 | 20.17 | 23596.6 |
| stage/render/graph_submit | 3.585 | 4.875 | 9.84 | 21448.0 |
| encode/submit | 2.280 | 3.248 | 6.87 | 13641.0 |
| stage/main/update | 2.153 | 2.598 | 19.10 | 12879.0 |
| encode/finish_wall | 1.371 | 2.025 | 4.29 | 8202.7 |
| stage/render/camera_driver | 1.124 | 1.571 | 4.14 | 6725.4 |
| encode/pass:shadow_directional_light/sum | 0.870 | 1.557 | 3.35 | 5202.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.787 | 1.096 | 1.99 | 4711.0 |
| encode/pass:shadow_point_light/sum | 0.779 | 3.215 | 17.50 | 4663.7 |
| stage/render/prepare_write_phase_buffers | 0.760 | 1.090 | 2.39 | 4548.6 |
| encode/pass:early prepass/sum | 0.755 | 1.072 | 2.90 | 4519.5 |
| stage/render/prepare_assets | 0.572 | 0.736 | 11.69 | 3421.3 |
| bigspace/total/ms | 0.561 | 1.345 | 2.28 | 3355.0 |
| stage/render/prepare_resources | 0.445 | 0.535 | 1.25 | 2664.9 |
| stage/render/prepare_meshes | 0.443 | 0.565 | 6.95 | 2652.7 |
| render/present/ms | 0.442 | 0.655 | 2.65 | 2646.2 |
| stage/main/first | 0.404 | 0.531 | 5.17 | 2418.2 |
| stage/main/pre_update | 0.360 | 0.523 | 1.72 | 2153.1 |
| stage/render/specialize | 0.320 | 0.413 | 13.19 | 1913.2 |
| terrain/update/ms | 0.311 | 0.403 | 1.94 | 1863.3 |
| stage/render/queue | 0.288 | 0.413 | 13.47 | 1724.7 |
| terrain/select/ms | 0.277 | 0.361 | 1.01 | 1658.9 |
| stage/render/prepare_bind_groups | 0.277 | 0.485 | 2.53 | 1658.0 |
| controller/feed/ms | 0.256 | 0.328 | 0.90 | 1531.9 |
| stream/drive/ms | 0.248 | 0.329 | 16.96 | 1484.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.238 | 0.353 | 1.02 | 1426.0 |
| bigspace/low_precision/ms | 0.234 | 0.737 | 1.89 | 1398.0 |
| stage/main/fixed_main_loop | 0.220 | 0.379 | 1.67 | 1316.1 |
| stage/render/create_views | 0.198 | 0.261 | 1.19 | 1187.4 |
| far/select/ms | 0.198 | 0.266 | 0.58 | 1186.1 |
| stream/tick/ms | 0.192 | 0.260 | 0.58 | 1149.8 |
| stage/render/prepare_batch_phases | 0.192 | 0.269 | 0.50 | 1147.2 |
| stage/render/extract_commands | 0.169 | 0.201 | 0.44 | 1013.1 |
| stage/render/prepare_flush | 0.164 | 0.300 | 1.76 | 984.1 |
| bigspace/high_precision/ms | 0.163 | 0.699 | 1.97 | 975.7 |
| anim/evaluate/ms | 0.162 | 0.276 | 1.53 | 969.7 |
| anim/evaluate_cpu/ms | 0.151 | 0.192 | 0.72 | 904.6 |
| stage/render/prepare_views | 0.139 | 0.176 | 0.39 | 831.0 |
| controller/move/ms | 0.138 | 0.313 | 0.96 | 824.8 |
| stage/main/last | 0.119 | 0.162 | 3.43 | 713.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.166 | 0.77 | 597.1 |
| render/acquire/ms | 0.095 | 0.123 | 0.31 | 565.5 |
| stage/render/prepare_collect_phase_buffers | 0.065 | 0.138 | 0.53 | 388.4 |
| pipelines/compile_ms | 0.049 | 0.000 | 125.62 | 296.1 |
| physics/step/ms | 0.048 | 0.091 | 0.57 | 289.6 |
| pipelines/front_ms | 0.047 | 0.000 | 120.92 | 283.8 |
| physics/insert/ms | 0.033 | 0.363 | 0.96 | 198.7 |
| stage/render/phase_sort | 0.033 | 0.057 | 0.30 | 197.5 |
| stage/render/cleanup | 0.031 | 0.091 | 0.75 | 187.8 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.66 | 180.1 |
| weather/map/ms | 0.025 | 0.033 | 0.16 | 151.3 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.58 | 88.4 |
| anim/apply/ms | 0.011 | 0.021 | 0.17 | 67.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.51 | 66.4 |
| controller/avoid/ms | 0.010 | 0.014 | 0.15 | 62.7 |
| weather/eval/ms | 0.010 | 0.014 | 0.15 | 62.1 |
| script/update/ms | 0.007 | 0.009 | 0.23 | 41.6 |
| script/input/ms | 0.005 | 0.006 | 0.19 | 31.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.06 | 30.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.15 | 29.3 |
| script/ui/ms | 0.005 | 0.006 | 0.22 | 28.4 |
| controller/write/ms | 0.004 | 0.004 | 0.18 | 21.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 13.8 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.70 | 12.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.16 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.09 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1301 | 2262 | 661 |
| crowd/c0/meshes/count | count | 353 | 646 | 173 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5928 | 5535 |
| crowd/c1/meshes/count | count | 1168 | 1270 | 1168 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 682 | 6895 | 563 |
| crowd/joints_uploaded/count | count | 4596 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 2.75 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.58 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.51 | 0.01 |
| encode/finish_critical | ms | 0.98 | 3.35 | 1.12 |
| encode/pass:early prepass/max | ms | 0.77 | 2.90 | 0.72 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.99 | 1.12 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.77 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.02 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 3.35 | 0.65 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.76 | 1.02 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.66 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.29 | 43.69 | 8.81 |
| gpu/timed/ms | ms | 7.91 | 11.29 | 8.22 |
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
| phases/alpha_mask3d/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1292 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3415 | 5114 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1292 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3415 | 5114 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 232 | 4 |
| phases/shadow/bins | count | 4990 | 8706 | 2405 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.3 k | 6352 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 252 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
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
| preprocess/bind_groups | count | 12 | 623 | 4 |
| preprocess/bind_groups_created | count | 12 | 623 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 239 | 125 |
| process/cpu_usage | % | 26.79 | 40.87 | 24.52 |
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
| skin/blocks_changed/count | count | 14 | 128 | 14 |
| skin/blocks_skipped/count | count | 186 | 234 | 224 |
| skin/blocks_walked/count | count | 14 | 20 | 14 |
| skin/joint_bytes_written/bytes | bytes | 46.3 k | 813.1 k | 37.4 k |
| skin/joints_written/count | count | 724 | 12.7 k | 584 |
| skin/upload_writes/count | count | 7 | 26 | 8 |
| started_load_count |  | 4919 | 5014 | 5014 |
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
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.61 | 67.93 | 36.03 |
| system/mem_usage | % | 61.14 | 61.68 | 61.61 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 64 | 279 | 54 |
| views/all/bins | count | 7561 | 11.6 k | 5903 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.2 k | 14.0 k |
| views/all/pipelines | count | 246 | 299 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 33.9 k | 8123 |
| views/cascade/batch_sets | count | 9 | 51 | 4 |
| views/cascade/bins | count | 2788 | 6194 | 2405 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8907 | 23.5 k | 6352 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 409 | 23.1 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 2139 | 2724 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2396 | 3021 | 0 |
| views/lamp_face/pipelines | count | 165 | 203 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2945 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2886 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8442 | 13.0 k | 7614 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8602 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 52.45 | 233 | 313.8 k |
| anim/events/count | count | 0.09 | 3 | 548 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.62 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1260 | 4676 |
| encode//count | count | 32.84 | 220 | 196.5 k |
| encode/bind_walk/init_textures | count | 12.2 k | 119.2 k | 72.91 M |
| encode/bind_walk/init_textures_skipped | count | 67.0 k | 162.0 k | 400.63 M |
| encode/bind_walk/inits | count | 455 | 1215 | 2.72 M |
| encode/bind_walk/inits_skipped | count | 109 | 160 | 652.7 k |
| encode/bind_walk/merge_textures | count | 12.1 k | 119.1 k | 72.48 M |
| encode/bind_walk/merge_textures_skipped | count | 5367 | 6012 | 32.11 M |
| encode/bind_walk/merges | count | 97.57 | 356 | 583.8 k |
| encode/bind_walk/merges_skipped | count | 84.29 | 132 | 504.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5983 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5983 |
| encode/pass:early prepass/count | count | 1 | 1 | 5983 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5983 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5983 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5983 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5983 |
| encode/pass:shadow_point_light/count | count | 1.22 | 48 | 7309 |
| encode/queue/command_buffers | count | 44.07 | 278 | 263.7 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.55 M | 4.44 M | 9.26 G |
| encode/queue/write_buffer/calls | count | 261 | 933 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.1 k | 3.52 M | 1.07 G |
| encode/queue/write_buffer_with/calls | count | 11.02 | 465 | 65.9 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5983 |
| physics/inserts | count | 0.37 | 105 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3938 |
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
| stream/spawn/items | count | 0.06 | 18 | 378 |
| stream/upload/bytes | bytes | 5658 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1241 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.75 | 2750.9 | main/busy/ms 2715, stage/main/post_update 2609, stage/main/update 97.38, surface/advance_cpu_ms 67.60, stage/main/pre_update 4.88, stage/main/last 1.35, surface/cell_upload_ms 0.75, stream/drive/ms 0.62, stage/main/first 0.51, stage/main/fixed_main_loop 0.40, stream/tick/ms 0.14, weather/map/ms 0.12 |
| warm-up | 3.51 | 757.8 | main/busy/ms 2870, stage/main/post_update 2635, render/thread/ms 720, render/busy/ms 543, stage/render/prepare_assets 491, stage/main/update 220, stage/render/prepare_views 172, render/acquire/ms 172, surface/advance_cpu_ms 67.60, stage/render/create_views 28.31, stage/render/render 15.35, render/graph/ms 8.55 |
| warm-up | 3.71 | 197.4 | render/thread/ms 899, render/busy/ms 611, stage/render/prepare_assets 541, main/busy/ms 189, stage/render/prepare_views 173, render/acquire/ms 172, stage/render/render 132, stage/main/update 131, render/present/ms 115, stage/main/post_update 40.16, encode//sum 31.24, stage/render/create_views 28.91 |
| warm-up | 3.93 | 194.2 | pipelines/wait_ms 4192, pipelines/compile_ms 2719, pipelines/front_ms 2340, pipelines/driver_ms 379, main/busy/ms 155, render/thread/ms 127, render/busy/ms 125, stage/render/prepare_assets 92.13, stage/main/update 89.87, stage/main/post_update 39.08, stage/render/render 22.81, render/graph/ms 19.96 |
| warm-up | 4.04 | 114.3 | pipelines/wait_ms 4203, pipelines/compile_ms 2872, pipelines/front_ms 2479, pipelines/driver_ms 394, render/thread/ms 211, render/busy/ms 209, stage/render/prepare_assets 168, main/busy/ms 67.01, stage/render/render 33.19, render/graph/ms 30.57, stage/render/graph_submit 27.75, encode/submit 23.83 |
| warm-up | 4.09 | 30.5 | pipelines/compile_ms 841, pipelines/front_ms 792, pipelines/wait_ms 184, render/thread/ms 48.64, pipelines/driver_ms 48.38, render/busy/ms 46.74, main/busy/ms 28.73, encode//sum 18.86, stage/render/render 16.19, stage/main/post_update 15.66, render/graph/ms 13.93, stage/render/prepare_assets 11.59 |
| warm-up | 4.23 | 92.4 | pipelines/compile_ms 978, pipelines/front_ms 942, main/busy/ms 58.41, render/thread/ms 53.38, render/busy/ms 51.51, stage/main/update 42.87, stage/render/prepare_assets 37.62, stream/drive/ms 37.41, pipelines/driver_ms 35.65, pipelines/wait_ms 26.65, terrain/update/ms 10.27, stage/main/post_update 10.20 |
| warm-up | 4.30 | 39.0 | pipelines/compile_ms 338, pipelines/front_ms 321, render/thread/ms 46.00, render/busy/ms 43.88, main/busy/ms 39.40, stage/render/render 19.54, stage/render/prepare_assets 18.98, stage/main/update 17.47, render/graph/ms 17.35, pipelines/driver_ms 16.56, stage/render/graph_submit 14.31, stage/main/post_update 12.08 |
| warm-up | 4.34 | 40.7 | pipelines/compile_ms 380, pipelines/front_ms 362, main/busy/ms 73.84, render/thread/ms 62.35, render/busy/ms 59.61, stage/main/update 55.84, stage/render/prepare_assets 40.24, stream/drive/ms 26.93, pipelines/driver_ms 18.46, stage/render/render 14.99, render/graph/ms 12.26, stage/render/graph_submit 10.13 |
| warm-up | 4.41 | 78.3 | pipelines/compile_ms 124, pipelines/front_ms 118, main/busy/ms 84.66, render/thread/ms 73.61, render/busy/ms 69.55, stage/main/update 57.51, stage/render/prepare_assets 48.91, stream/drive/ms 28.62, stage/main/post_update 20.10, stage/render/render 14.12, render/graph/ms 11.23, stage/render/graph_submit 9.06 |
| warm-up | 4.49 | 76.2 | pipelines/compile_ms 957, pipelines/front_ms 923, render/thread/ms 94.22, render/busy/ms 91.12, main/busy/ms 69.20, stage/render/prepare_assets 63.80, stage/main/update 36.89, pipelines/driver_ms 33.85, stage/main/post_update 21.42, stage/render/prepare_resources 9.05, stage/render/render 8.84, render/graph/ms 6.97 |
| warm-up | 4.60 | 105.4 | pipelines/compile_ms 2236, pipelines/front_ms 2169, render/thread/ms 135, render/busy/ms 133, stage/render/prepare_assets 69.69, pipelines/driver_ms 66.94, main/busy/ms 65.91, stage/main/update 40.00, stage/render/render 39.26, render/graph/ms 37.04, stage/render/camera_driver 28.39, pipelines/wait_ms 28.15 |
| warm-up | 4.70 | 105.0 | pipelines/compile_ms 2084, pipelines/front_ms 2025, pipelines/wait_ms 190, render/thread/ms 174, render/busy/ms 171, main/busy/ms 72.66, stage/render/render 63.24, render/graph/ms 59.94, pipelines/driver_ms 58.86, stage/render/prepare_assets 47.19, stage/render/camera_driver 46.28, stage/main/update 32.08 |
| warm-up | 4.80 | 101.1 | pipelines/compile_ms 2160, pipelines/front_ms 2093, pipelines/wait_ms 388, render/thread/ms 183, render/busy/ms 180, main/busy/ms 76.24, pipelines/driver_ms 67.20, stage/render/render 54.00, render/graph/ms 50.59, stage/render/prepare_assets 40.22, stage/render/prepare_write_phase_buffers 34.85, encode//sum 31.62 |
| warm-up | 4.88 | 78.7 | pipelines/compile_ms 2533, pipelines/front_ms 2443, pipelines/wait_ms 493, render/thread/ms 164, render/busy/ms 162, main/busy/ms 96.28, pipelines/driver_ms 90.12, stage/render/render 51.15, render/graph/ms 48.66, stage/render/prepare_write_phase_buffers 45.60, stage/main/update 42.86, stage/main/post_update 34.15 |
| warm-up | 4.95 | 70.7 | pipelines/compile_ms 1604, pipelines/front_ms 1545, pipelines/wait_ms 334, render/thread/ms 139, render/busy/ms 137, main/busy/ms 96.81, encode//sum 62.02, pipelines/driver_ms 59.33, stage/render/render 52.62, render/graph/ms 49.68, stage/main/update 45.19, stage/render/prepare_write_phase_buffers 36.37 |
| warm-up | 4.99 | 36.1 | pipelines/compile_ms 1283, pipelines/front_ms 1242, pipelines/wait_ms 134, render/thread/ms 92.55, render/busy/ms 90.10, encode//sum 56.72, main/busy/ms 42.69, pipelines/driver_ms 41.04, stage/render/render 37.58, render/graph/ms 34.36, stage/render/graph_submit 20.96, stage/main/update 18.42 |
| warm-up | 5.08 | 63.6 | pipelines/compile_ms 1048, pipelines/front_ms 1023, pipelines/wait_ms 118, main/busy/ms 97.75, stage/main/update 72.61, stream/drive/ms 49.51, render/thread/ms 41.42, render/busy/ms 39.60, pipelines/driver_ms 25.16, stage/render/render 14.87, stage/render/prepare_assets 14.20, stage/main/post_update 13.11 |
| warm-up | 5.16 | 80.2 | pipelines/compile_ms 1059, pipelines/front_ms 1029, main/busy/ms 123, stage/main/update 98.44, pipelines/wait_ms 73.64, render/thread/ms 66.89, render/busy/ms 65.21, stream/drive/ms 54.85, pipelines/driver_ms 29.66, stage/render/prepare_assets 25.89, stage/render/render 14.75, stage/main/post_update 12.40 |
| warm-up | 5.33 | 165.7 | pipelines/compile_ms 2050, pipelines/front_ms 1987, render/thread/ms 138, render/busy/ms 137, main/busy/ms 115, stage/main/update 89.40, stage/render/prepare_assets 69.39, pipelines/driver_ms 62.89, stream/drive/ms 53.59, stage/render/render 40.94, render/graph/ms 38.87, stage/render/graph_submit 35.32 |
| warm-up | 5.44 | 92.6 | pipelines/compile_ms 216, pipelines/front_ms 207, main/busy/ms 80.50, stage/main/update 60.78, render/thread/ms 57.34, render/busy/ms 55.95, stream/drive/ms 42.78, stage/render/prepare_assets 24.11, stage/render/render 20.01, render/graph/ms 17.98, stage/render/graph_submit 14.53, stage/main/post_update 12.53 |
| warm-up | 5.52 | 52.3 | pipelines/compile_ms 708, pipelines/front_ms 685, main/busy/ms 59.88, stage/main/update 45.42, render/thread/ms 39.63, render/busy/ms 38.32, stream/drive/ms 33.51, pipelines/driver_ms 23.52, stage/render/prepare_assets 13.40, stage/render/render 10.03, render/graph/ms 8.09, stage/main/post_update 7.81 |
| warm-up | 5.65 | 50.0 | pipelines/compile_ms 191, pipelines/front_ms 184, render/thread/ms 68.85, render/busy/ms 67.39, main/busy/ms 42.34, stage/render/prepare_assets 23.25, stage/main/post_update 18.15, stage/main/update 18.00, stage/render/render 13.85, render/graph/ms 11.63, encode//sum 9.23, stage/render/prepare_resources 8.58 |
| warm-up | 5.72 | 49.1 | pipelines/compile_ms 1877, pipelines/front_ms 1830, render/thread/ms 52.41, render/busy/ms 51.05, pipelines/driver_ms 47.27, main/busy/ms 39.85, stage/render/prepare_assets 26.58, stage/main/update 22.46, stage/render/render 12.19, stage/main/post_update 10.64, render/graph/ms 10.21, stage/render/graph_submit 7.42 |
| warm-up | 5.82 | 30.2 | render/thread/ms 46.30, render/busy/ms 45.08, main/busy/ms 34.86, stage/render/prepare_assets 23.59, stage/main/update 16.93, stage/main/post_update 11.77, stage/render/render 9.96, stream/drive/ms 8.63, encode//sum 8.50, render/graph/ms 8.03, stage/render/graph_submit 4.92, stage/render/camera_driver 2.94 |
| warm-up | 5.85 | 31.8 | pipelines/compile_ms 364, pipelines/front_ms 354, render/thread/ms 50.59, render/busy/ms 49.50, stage/render/prepare_assets 30.92, main/busy/ms 29.90, stage/main/update 13.74, stage/render/render 10.21, stage/main/post_update 10.12, pipelines/driver_ms 9.54, render/graph/ms 8.45, encode//sum 8.21 |
| warm-up | 5.93 | 35.4 | pipelines/compile_ms 563, pipelines/front_ms 550, render/thread/ms 41.93, render/busy/ms 40.62, main/busy/ms 39.38, stage/main/update 21.41, stage/render/prepare_assets 21.17, pipelines/driver_ms 12.72, stage/main/post_update 12.33, stream/drive/ms 11.38, stage/render/render 11.15, encode//sum 9.28 |
| warm-up | 6.04 | 34.4 | pipelines/compile_ms 553, pipelines/front_ms 526, render/thread/ms 50.87, render/busy/ms 49.49, stage/render/prepare_assets 28.17, main/busy/ms 27.47, pipelines/driver_ms 27.11, stage/render/render 12.15, stage/main/post_update 11.84, render/graph/ms 10.02, stage/main/update 9.72, encode//sum 8.36 |
| warm-up | 6.57 | 31.5 | render/thread/ms 37.97, render/busy/ms 37.03, main/busy/ms 27.94, stage/render/prepare_assets 17.48, stage/main/update 11.93, stage/main/post_update 9.73, stage/render/render 7.81, encode//sum 7.09, render/graph/ms 6.44, stage/render/graph_submit 4.46, stream/drive/ms 4.18, stage/render/specialize 3.04 |
| warm-up | 6.79 | 29.9 | render/thread/ms 47.41, render/busy/ms 46.02, main/busy/ms 41.63, stage/main/update 17.30, stage/main/post_update 16.32, stage/render/render 12.57, stage/render/prepare_assets 12.28, render/graph/ms 10.27, encode//sum 9.47, stage/render/graph_submit 6.35, stream/drive/ms 5.86, stage/render/prepare_write_phase_buffers 4.57 |
| warm-up | 6.89 | 34.0 | main/busy/ms 42.12, render/thread/ms 31.45, render/busy/ms 30.31, stage/main/update 24.17, stream/drive/ms 14.38, stage/main/post_update 13.48, stage/render/render 12.71, render/graph/ms 10.86, encode//sum 7.96, stage/render/graph_submit 7.93, stage/render/prepare_assets 5.79, encode/submit 4.59 |
| warm-up | 6.94 | 29.6 | pipelines/compile_ms 155, pipelines/front_ms 151, main/busy/ms 40.13, render/thread/ms 36.91, render/busy/ms 35.85, stage/main/update 20.37, stage/render/prepare_assets 15.09, stage/main/post_update 11.27, stage/render/render 10.50, render/graph/ms 8.87, encode//sum 7.01, stage/render/graph_submit 6.09 |
| warm-up | 7.13 | 28.8 | render/thread/ms 40.13, render/busy/ms 39.11, main/busy/ms 23.75, stage/render/prepare_assets 21.97, stage/main/post_update 12.08, stage/render/render 10.13, render/graph/ms 8.48, stage/main/update 7.40, encode//sum 7.16, stage/render/graph_submit 6.09, encode/submit 3.87, stream/drive/ms 2.48 |
| warm-up | 7.31 | 35.9 | main/busy/ms 42.97, render/thread/ms 27.65, render/busy/ms 26.71, stage/main/update 23.88, stream/drive/ms 13.15, stage/render/prepare_assets 12.30, stage/main/post_update 11.94, stage/render/render 8.50, render/graph/ms 7.16, encode//sum 7.05, stage/render/graph_submit 5.13, encode/submit 2.90 |
| warm-up | 7.41 | 31.9 | render/thread/ms 38.75, render/busy/ms 37.61, main/busy/ms 18.58, pipelines/compile_ms 15.61, pipelines/front_ms 14.56, stage/render/render 11.00, stage/main/post_update 10.00, stage/render/specialize 9.85, render/graph/ms 9.47, encode//sum 9.33, stage/render/graph_submit 7.30, stage/render/queue 7.06 |
| warm-up | 8.73 | 30.9 | render/thread/ms 37.22, render/busy/ms 36.20, main/busy/ms 18.19, stage/render/render 11.56, stage/main/post_update 10.80, render/graph/ms 10.14, stage/render/specialize 9.46, encode//sum 8.74, stage/render/graph_submit 7.89, stage/render/queue 6.17, stage/main/update 4.96, encode/finish_wall 4.03 |
| measure | 0.43 | 34.1 | render/thread/ms 38.06, render/busy/ms 37.05, main/busy/ms 17.07, stage/render/render 12.58, render/graph/ms 11.23, stage/main/post_update 10.66, stage/render/specialize 9.37, stage/render/graph_submit 9.08, encode//sum 7.50, stage/render/queue 6.12, encode/submit 6.12, stage/main/update 3.90 |
| measure | 0.75 | 29.2 | main/busy/ms 36.07, render/thread/ms 27.71, render/busy/ms 26.49, stage/main/update 18.65, stage/main/post_update 13.96, stream/drive/ms 13.84, stage/render/render 12.14, render/graph/ms 10.57, encode//sum 8.86, stage/render/graph_submit 7.60, stage/render/prepare_assets 6.08, encode/submit 4.64 |
| measure | 1.75 | 31.6 | render/thread/ms 37.12, render/busy/ms 36.16, main/busy/ms 19.82, stage/render/render 11.87, stage/main/post_update 11.61, render/graph/ms 10.44, stage/render/specialize 9.25, encode//sum 7.71, stage/render/graph_submit 7.67, stage/render/queue 5.68, encode/submit 4.60, stage/render/prepare_meshes 4.48 |
| measure | 3.18 | 32.7 | render/thread/ms 42.91, render/busy/ms 41.93, main/busy/ms 21.31, stage/render/render 15.88, render/graph/ms 14.40, stage/main/post_update 13.50, encode//sum 11.81, stage/render/graph_submit 10.46, encode/pass:shadow_point_light/sum 9.93, stage/render/specialize 9.90, encode/submit 6.56, stage/render/queue 6.38 |
| measure | 4.61 | 29.5 | render/thread/ms 35.44, render/busy/ms 34.49, main/busy/ms 19.15, stage/main/post_update 12.59, stage/render/specialize 10.07, stage/render/render 9.89, render/graph/ms 8.49, encode//sum 7.18, stage/render/graph_submit 6.56, stage/render/queue 6.31, stage/main/update 4.06, encode/submit 3.66 |
| measure | 5.97 | 29.2 | render/thread/ms 35.34, render/busy/ms 34.44, main/busy/ms 18.67, stage/main/post_update 11.94, stage/render/render 10.29, stage/render/specialize 9.73, render/graph/ms 8.87, encode//sum 7.57, stage/render/graph_submit 6.51, stage/render/queue 6.33, stage/main/update 4.04, encode/submit 3.54 |
| measure | 7.30 | 29.3 | render/thread/ms 36.26, render/busy/ms 35.22, main/busy/ms 17.17, stage/render/render 11.82, stage/main/post_update 10.75, render/graph/ms 10.34, stage/render/specialize 9.31, encode//sum 8.38, stage/render/graph_submit 7.54, stage/render/queue 6.00, encode/submit 4.37, stage/main/update 3.93 |
| measure | 9.96 | 31.7 | render/thread/ms 36.88, render/busy/ms 35.96, main/busy/ms 19.81, stage/main/post_update 12.15, stage/render/render 10.86, stage/render/specialize 10.07, render/graph/ms 9.39, encode//sum 8.83, stage/render/graph_submit 6.97, stage/render/queue 6.71, stage/main/update 5.19, encode/submit 3.94 |
| measure | 11.27 | 29.6 | render/thread/ms 35.60, render/busy/ms 34.48, main/busy/ms 19.76, stage/main/post_update 11.67, stage/render/render 10.46, stage/render/specialize 10.34, render/graph/ms 8.87, encode//sum 6.80, stage/render/graph_submit 6.75, stage/render/queue 5.86, stage/main/update 4.46, encode/submit 3.95 |
| measure | 15.71 | 31.3 | render/thread/ms 40.25, render/busy/ms 39.26, main/busy/ms 19.42, stage/render/render 13.00, stage/main/post_update 11.64, render/graph/ms 11.57, stage/render/specialize 10.32, encode//sum 9.39, stage/render/graph_submit 8.90, stage/render/queue 7.19, encode/pass:shadow_point_light/sum 5.73, encode/submit 5.48 |
| measure | 17.09 | 29.8 | render/thread/ms 37.43, render/busy/ms 36.36, main/busy/ms 19.41, stage/main/post_update 12.32, stage/render/render 11.81, render/graph/ms 10.40, stage/render/specialize 9.67, encode//sum 9.63, stage/render/graph_submit 7.09, stage/render/queue 6.86, encode/pass:shadow_point_light/sum 4.59, stage/main/update 4.58 |
| measure | 18.00 | 28.8 | render/thread/ms 37.49, render/busy/ms 36.49, main/busy/ms 18.59, stage/render/render 11.85, stage/main/post_update 11.56, render/graph/ms 10.51, stage/render/specialize 10.03, encode//sum 9.78, stage/render/graph_submit 7.67, stage/render/queue 6.55, encode/submit 5.11, encode/pass:shadow_point_light/sum 4.14 |
| measure | 23.03 | 39.0 | render/thread/ms 45.90, render/busy/ms 44.67, main/busy/ms 26.50, stage/render/specialize 13.56, stage/main/post_update 13.32, stage/render/render 11.70, render/graph/ms 10.00, encode//sum 9.17, stage/render/queue 8.90, stage/render/graph_submit 7.70, stage/main/update 6.64, encode/submit 4.29 |
| measure | 23.08 | 28.9 | render/thread/ms 39.42, render/busy/ms 38.25, main/busy/ms 34.34, stage/main/post_update 16.11, stage/render/render 14.30, stage/main/update 12.56, render/graph/ms 12.42, encode//sum 11.82, stage/render/prepare_assets 9.18, stage/render/graph_submit 8.67, encode/submit 4.96, encode/finish_wall 3.78 |
| measure | 24.55 | 32.0 | render/thread/ms 38.62, render/busy/ms 37.66, main/busy/ms 20.87, stage/main/post_update 12.44, stage/render/render 11.64, stage/render/specialize 10.55, render/graph/ms 10.28, encode//sum 8.50, stage/render/graph_submit 7.95, stage/render/queue 7.11, stage/main/update 5.35, encode/submit 4.50 |
| measure | 27.34 | 37.2 | render/thread/ms 51.37, render/busy/ms 50.19, main/busy/ms 29.26, stage/render/render 17.97, render/graph/ms 16.29, stage/main/post_update 15.32, encode//sum 13.77, stage/render/specialize 13.65, stage/render/graph_submit 12.95, encode/pass:shadow_point_light/sum 10.04, stage/render/queue 8.53, encode/submit 8.37 |
| measure | 28.77 | 30.5 | render/thread/ms 36.76, render/busy/ms 35.60, main/busy/ms 19.05, stage/main/post_update 12.21, stage/render/render 10.90, stage/render/specialize 9.70, render/graph/ms 9.23, encode//sum 8.56, stage/render/graph_submit 6.96, stage/render/queue 6.85, encode/submit 4.47, stage/main/update 4.16 |
| measure | 30.48 | 33.9 | render/thread/ms 41.57, render/busy/ms 40.39, main/busy/ms 21.94, stage/render/render 13.14, stage/main/post_update 12.71, render/graph/ms 11.60, stage/render/specialize 11.01, encode//sum 9.59, stage/render/graph_submit 8.83, encode/pass:shadow_point_light/sum 7.71, stage/render/queue 7.50, stage/main/update 5.60 |
| measure | 32.05 | 30.0 | render/thread/ms 38.17, render/busy/ms 37.19, main/busy/ms 19.84, stage/main/post_update 12.12, stage/render/render 11.75, render/graph/ms 10.36, stage/render/specialize 9.99, encode//sum 8.52, stage/render/graph_submit 7.83, stage/render/queue 7.21, encode/pass:shadow_point_light/sum 5.91, encode/submit 4.88 |
| measure | 35.35 | 33.2 | render/thread/ms 39.85, render/busy/ms 38.61, main/busy/ms 20.75, stage/main/post_update 13.01, stage/render/render 12.55, render/graph/ms 10.94, stage/render/specialize 10.28, stage/render/graph_submit 8.48, encode//sum 7.93, stage/render/queue 6.75, encode/submit 5.17, stage/main/update 4.52 |
| measure | 37.49 | 30.4 | render/thread/ms 37.59, render/busy/ms 36.35, main/busy/ms 19.84, stage/main/post_update 11.55, stage/render/render 11.45, stage/render/specialize 10.07, render/graph/ms 9.79, encode//sum 9.26, stage/render/graph_submit 7.69, stage/render/queue 6.45, stage/main/update 4.65, encode/submit 4.26 |
| measure | 40.76 | 31.9 | render/thread/ms 38.50, render/busy/ms 37.52, main/busy/ms 19.39, stage/main/post_update 11.52, stage/render/render 10.72, stage/render/specialize 10.26, render/graph/ms 9.30, encode//sum 8.21, stage/render/graph_submit 7.35, stage/render/queue 6.19, stage/main/update 4.51, encode/submit 4.05 |
| measure | 43.21 | 30.0 | render/thread/ms 36.83, render/busy/ms 35.77, main/busy/ms 16.96, stage/main/post_update 10.37, stage/render/render 10.28, stage/render/specialize 10.20, render/graph/ms 8.86, encode//sum 7.48, stage/render/graph_submit 7.09, stage/render/queue 6.38, stage/main/update 3.88, encode/submit 3.77 |
| measure | 45.69 | 34.4 | render/thread/ms 39.05, render/busy/ms 37.95, main/busy/ms 18.51, stage/render/render 11.21, stage/main/post_update 10.66, stage/render/specialize 10.29, render/graph/ms 9.74, encode//sum 9.49, stage/render/graph_submit 7.80, stage/render/queue 6.35, stage/main/update 4.33, stage/render/prepare_meshes 4.32 |
| measure | 56.43 | 29.7 | render/thread/ms 36.27, render/busy/ms 35.14, main/busy/ms 16.80, stage/render/render 10.27, stage/main/post_update 9.85, stage/render/specialize 9.57, render/graph/ms 8.93, encode//sum 8.16, stage/render/graph_submit 7.03, stage/render/queue 6.90, encode/submit 4.31, stage/main/update 4.24 |
| measure | 59.03 | 49.3 | render/thread/ms 57.56, render/busy/ms 56.03, main/busy/ms 29.21, stage/main/post_update 15.62, stage/render/render 13.86, stage/render/queue 13.74, stage/render/specialize 12.87, render/graph/ms 11.93, encode//sum 10.14, stage/render/graph_submit 8.71, stage/main/update 7.95, stage/render/prepare_meshes 7.41 |
| measure | 59.09 | 33.6 | render/thread/ms 38.72, render/busy/ms 37.64, main/busy/ms 21.18, stage/render/prepare_assets 19.52, stage/main/post_update 11.86, stage/render/render 10.05, encode//sum 9.61, render/graph/ms 8.76, stage/render/graph_submit 6.73, stage/main/update 4.63, encode/submit 4.05, encode/finish_wall 2.75 |
| measure | 62.58 | 30.6 | render/thread/ms 37.15, render/busy/ms 36.19, main/busy/ms 17.92, stage/render/specialize 10.62, stage/render/render 10.14, stage/main/post_update 9.55, render/graph/ms 8.91, encode//sum 7.54, stage/render/graph_submit 6.83, stage/render/queue 6.59, stage/main/update 5.45, stage/render/prepare_meshes 3.82 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=skin-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 17%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (250 samples): GPU load **78%** (min 52%) · 2624 MHz (min 2610) · memory 8101 MHz · 79.5 W · 75 °C (max 81) · VRAM 6785 MiB · throttled 100% of samples

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
- `render.skin.skip_unchanged=on (command line: --set)`
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
