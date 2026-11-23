# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 369 frames over 15.0 s · **median 37.36 ms (27 fps)** · mean 40.67 · p95 65.35 · p99 98.03 (1% low 10 fps) · worst 102.62 ms · 25 spikes over 1.5× median · 100.0 stutters/min (work 100.0/min)
- warm-up: 309 frames over 19.3 s, worst 2560.9 ms
- hitches (> 93.4 ms): **11** measured, 31 in warm-up
- timed GPU passes: 34.73 ms of a 40.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 37.36 ms, limit 11 ms
- **FAIL** worst frame 102.62 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 10.446 | 10.446 | 10.542 | 1.00 | 0.081 |
| early prepass | 9.711 | 9.711 | 9.792 | 1.00 | 0.128 |
| range_unpacking | 5.288 | 5.288 | 32.505 | 1.00 | 0.002 |
| dlss_super_resolution | 2.071 | 2.071 | 2.097 | 1.00 | 0.194 |
| early_mesh_preprocessing | 1.878 | 1.878 | 2.635 | 1.00 | 0.003 |
| sun_visibility | 0.708 | 0.708 | 0.718 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.646 | 0.646 | 0.676 | 1.00 | 0.008 |
| bin_unpacking | 0.514 | 0.514 | 0.711 | 1.00 | 0.015 |
| uniform_allocation | 0.388 | 0.388 | 0.517 | 1.00 | 0.023 |
| atmosphere_luts | 0.358 | 0.358 | 0.364 | 1.00 | 0.003 |
| water_velocity | 0.332 | 0.332 | 0.335 | 1.00 | 0.001 |
| bloom | 0.325 | 0.325 | 0.342 | 1.00 | 0.059 |
| atmos_froxel_inject | 0.311 | 0.311 | 0.309 | 1.00 | 0.001 |
| clustering | 0.289 | 0.289 | 0.323 | 1.00 | 0.019 |
| water_sspr | 0.202 | 0.202 | 0.202 | 1.00 | 0.002 |
| ssao | 0.190 | 0.190 | 0.196 | 1.00 | 0.007 |
| water_fft | 0.137 | 0.137 | 0.140 | 1.00 | 0.002 |
| upscaling | 0.133 | 0.133 | 0.137 | 1.00 | 0.004 |
| clouds_composite | 0.109 | 0.109 | 0.112 | 1.00 | 0.002 |
| tonemapping | 0.091 | 0.091 | 0.096 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.081 | 0.081 | 0.085 | 1.00 | 0.006 |
| render_sky | 0.064 | 0.064 | 0.066 | 1.00 | 0.001 |
| clouds_trace | 0.060 | 0.060 | 0.070 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.053 | 0.053 | 0.105 | 1.00 | 0.006 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.048 | 0.048 | 0.049 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.037 | 0.037 | 0.047 | 1.00 | 0.216 |
| point_shadow_atlas | 0.036 | 0.072 | 0.078 | 0.50 | 0.130 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.216 | 0.223 | 0.12 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.001 |
| clouds_shadow | 0.023 | 0.023 | 0.026 | 1.00 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| lightprobe_copy | 0.009 | 0.070 | 0.071 | 0.12 | 0.004 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.015 | 0.017 | 0.50 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.006 | 1.00 | 0.003 |
| night_sky | 0.005 | 0.005 | 0.009 | 1.00 | 0.001 |
| lightprobe_downsampling_first_pass | 0.004 | 0.032 | 0.034 | 0.12 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.01 | 0.008 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 39.406 | 64.100 | 101.41 | 14540.9 |
| stage/render/prepare_views | 29.402 | 55.613 | 92.71 | 10849.2 |
| render/acquire/ms | 29.329 | 55.527 | 92.64 | 10822.2 |
| render/busy/ms | 9.681 | 11.220 | 17.01 | 3572.3 |
| main/busy/ms | 7.963 | 8.995 | 12.51 | 2938.5 |
| encode//sum | 6.034 | 8.856 | 12.62 | 2226.4 |
| stage/render/render | 5.912 | 7.139 | 12.98 | 2181.7 |
| render/graph/ms | 5.247 | 6.416 | 12.15 | 1936.1 |
| stage/render/graph_submit | 3.903 | 5.070 | 10.83 | 1440.3 |
| stage/main/post_update | 3.761 | 4.236 | 6.45 | 1387.8 |
| encode/submit | 2.349 | 3.111 | 8.14 | 866.8 |
| stage/main/update | 1.666 | 1.913 | 2.49 | 614.9 |
| encode/finish_wall | 1.568 | 2.143 | 3.15 | 578.7 |
| stage/render/camera_driver | 1.249 | 1.458 | 2.14 | 460.8 |
| stage/main/fixed_main_loop | 1.246 | 1.695 | 2.75 | 459.8 |
| encode/pass:early prepass/sum | 1.045 | 1.450 | 1.77 | 385.7 |
| encode/pass:main_opaque_pass_3d/sum | 1.019 | 1.482 | 1.81 | 375.9 |
| encode/pass:shadow_directional_light/sum | 1.007 | 1.711 | 2.56 | 371.4 |
| stage/render/prepare_assets | 0.770 | 0.972 | 4.51 | 284.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.579 | 0.873 | 1.08 | 213.6 |
| stage/render/prepare_write_phase_buffers | 0.527 | 0.739 | 1.35 | 194.4 |
| stage/render/prepare_resources | 0.519 | 0.595 | 0.64 | 191.6 |
| stage/render/prepare_meshes | 0.507 | 0.657 | 0.86 | 187.2 |
| stage/main/first | 0.452 | 0.629 | 1.31 | 166.6 |
| bigspace/total/ms | 0.424 | 0.529 | 0.63 | 156.3 |
| stage/main/pre_update | 0.402 | 0.506 | 2.69 | 148.3 |
| render/present/ms | 0.397 | 0.472 | 0.81 | 146.4 |
| stage/render/prepare_batch_phases | 0.373 | 0.517 | 1.02 | 137.6 |
| stage/render/prepare_flush | 0.332 | 0.532 | 1.34 | 122.4 |
| controller/feed/ms | 0.304 | 0.376 | 0.54 | 112.3 |
| stage/render/prepare_bind_groups | 0.272 | 0.460 | 0.64 | 100.3 |
| stage/main/last | 0.254 | 0.298 | 0.40 | 93.6 |
| stage/render/queue | 0.249 | 0.307 | 0.38 | 91.8 |
| stage/render/extract_commands | 0.208 | 0.239 | 0.28 | 76.7 |
| controller/move/ms | 0.190 | 0.305 | 0.45 | 70.3 |
| stage/render/create_views | 0.186 | 0.226 | 0.30 | 68.7 |
| anim/evaluate_cpu/ms | 0.185 | 0.210 | 0.30 | 68.1 |
| bigspace/low_precision/ms | 0.184 | 0.237 | 0.32 | 68.1 |
| stage/render/specialize | 0.183 | 0.222 | 0.33 | 67.4 |
| stream/drive/ms | 0.177 | 0.210 | 0.62 | 65.3 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.171 | 0.503 | 1.41 | 63.0 |
| stream/tick/ms | 0.147 | 0.177 | 0.31 | 54.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.147 | 0.238 | 0.90 | 54.1 |
| anim/evaluate/ms | 0.141 | 0.169 | 0.20 | 52.0 |
| physics/step/ms | 0.100 | 0.132 | 0.17 | 37.1 |
| bigspace/high_precision/ms | 0.093 | 0.115 | 0.18 | 34.3 |
| encode/resolve_diagnostics/sum | 0.065 | 0.101 | 0.44 | 23.8 |
| controller/write/ms | 0.061 | 0.074 | 0.10 | 22.4 |
| stage/render/phase_sort | 0.045 | 0.066 | 0.11 | 16.8 |
| stage/render/prepare_collect_phase_buffers | 0.040 | 0.058 | 0.41 | 14.6 |
| terrain/update/ms | 0.035 | 0.045 | 0.10 | 12.8 |
| stage/render/cleanup | 0.030 | 0.039 | 0.29 | 11.1 |
| controller/avoid/ms | 0.029 | 0.043 | 0.07 | 10.6 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.041 | 0.08 | 8.9 |
| encode/camera_driver_span_end/sum | 0.023 | 0.039 | 0.15 | 8.6 |
| weather/map/ms | 0.015 | 0.020 | 0.04 | 5.4 |
| anim/apply/ms | 0.010 | 0.013 | 0.02 | 3.8 |
| weather/eval/ms | 0.009 | 0.012 | 0.02 | 3.2 |
| script/update/ms | 0.008 | 0.010 | 0.04 | 2.9 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 1.9 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 1.4 |
| script/ui/ms | 0.004 | 0.005 | 0.06 | 1.4 |
| stream/poll_models/ms | 0.004 | 0.003 | 0.47 | 1.3 |
| character/finish/ms | 0.001 | 0.002 | 0.00 | 0.5 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.00 | 0.5 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 0.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 0.4 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.2 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| stage/render/prepare | 0.000 | 0.003 | 0.00 | 0.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.1 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/chains/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/plan/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/wanted/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 3 | 3 | 3 |
| anim/a3/count | count | 447 | 447 | 447 |
| anim/characters/count | count | 450 | 450 | 450 |
| anim/evaluated/count | count | 27 | 27 | 27 |
| controller/movers/count | count | 28 | 28 | 28 |
| crowd/c0/count | count | 30 | 30 | 30 |
| crowd/c0/joints/count | count | 1249 | 1249 | 1249 |
| crowd/c0/meshes/count | count | 398 | 398 | 398 |
| crowd/c1/count | count | 116 | 116 | 116 |
| crowd/c1/joints/count | count | 5172 | 5172 | 5172 |
| crowd/c1/meshes/count | count | 1293 | 1293 | 1293 |
| crowd/c2/count | count | 304 | 304 | 304 |
| crowd/c2/joints/count | count | 5254 | 5254 | 5254 |
| crowd/c2/meshes/count | count | 1802 | 1802 | 1802 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 210 | 210 | 210 |
| crowd/joints_extracted/count | count | 772 | 1133 | 738 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.75 | 2.08 | 0.78 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.08 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.15 | 0.02 |
| encode/finish_critical | ms | 1.22 | 2.56 | 1.07 |
| encode/pass:early prepass/max | ms | 1.04 | 1.77 | 0.92 |
| encode/pass:main_opaque_pass_3d/max | ms | 1.01 | 1.81 | 1.07 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.14 | 0.90 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.55 | 1.08 | 0.50 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.28 | 1.41 | 0.20 |
| encode/pass:shadow_directional_light/max | ms | 0.94 | 2.56 | 1.05 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.44 | 0.05 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 32.44 | 94.93 | 31.50 |
| gpu/timed/ms | ms | 32.44 | 94.93 | 31.50 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1359 | 1359 | 1359 |
| groundcover/slots/capacity | count | 658.0 k | 658.0 k | 658.0 k |
| groundcover/slots/capacity_bytes | bytes | 42.11 M | 42.11 M | 42.11 M |
| groundcover/slots/live | count | 42.2 k | 42.2 k | 42.2 k |
| groundcover/tiles/capped | count | 0 | 0 | 0 |
| groundcover/tiles/casting | count | 5 | 5 | 5 |
| groundcover/tiles/resident | count | 63 | 63 | 63 |
| groundcover/tiles/scattered | count | 63 | 63 | 63 |
| groundcover/tiles/unplaced | count | 0 | 0 | 0 |
| groundcover/upload/bytes | bytes | 0 | 0 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2433 | 2433 | 2433 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 303.9 k | 303.9 k | 303.9 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.6 k | 12.6 k | 12.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 343.66 M | 343.66 M | 343.66 M |
| phases/alpha_mask3d/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d/bins | count | 364 | 364 | 364 |
| phases/alpha_mask3d/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d/instance_draws | count | 569 | 569 | 569 |
| phases/alpha_mask3d/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d/items | count | 569 | 569 | 569 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d_prepass/bins | count | 364 | 364 | 364 |
| phases/alpha_mask3d_prepass/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d_prepass/instance_draws | count | 569 | 569 | 569 |
| phases/alpha_mask3d_prepass/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d_prepass/items | count | 569 | 569 | 569 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 31 | 31 | 31 |
| phases/opaque3d/bins | count | 1369 | 1369 | 1369 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2851 | 2851 | 2851 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 28 | 28 |
| phases/opaque3d_prepass/bins | count | 1369 | 1369 | 1369 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2851 | 2851 | 2851 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 90 | 133 | 133 |
| phases/shadow/bins | count | 2179 | 2195 | 2191 |
| phases/shadow/instance_batches | count | 9 | 27 | 27 |
| phases/shadow/instance_draws | count | 7317 | 22.0 k | 22.0 k |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 248.6 k |
| phases/shadow/items | count | 5359 | 5375 | 5371 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 53 | 55 | 55 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 360 | 360 | 360 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 9 | 68 | 68 |
| preprocess/bind_groups_created | count | 4 | 61 | 61 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 14.22 | 22.22 | 11.01 |
| process/mem_usage | GiB | 3.61 | 3.64 | 3.61 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 1720 | 1720 | 1720 |
| scene/batch/masked/live | count | 1720 | 1720 | 1720 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 19.7 k | 19.7 k | 19.7 k |
| scene/batch/opaque/live | count | 19.7 k | 19.7 k | 19.7 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 118 | 118 | 118 |
| scene/chains/high_water | count | 1663 | 1663 | 1663 |
| scene/chains/live | count | 1663 | 1663 | 1663 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/scene_bump | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/scene_live | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 4 | 4 | 4 |
| shadow_atlas/live_occ_pct | % | 0.59 | 0.68 | 0.68 |
| shadow_atlas/live_tiles | count | 6 | 7 | 7 |
| shadow_atlas/mover_draws | count | 0 | 4 | 0 |
| shadow_atlas/movers | count | 97 | 112 | 100 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 2 | 4 | 3 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 2 | 4 | 3 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 6 | 7 | 7 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 6 | 7 | 7 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 20 | 26 | 20 |
| skin/blocks_skipped/count | count | 190 | 198 | 190 |
| skin/blocks_walked/count | count | 20 | 27 | 20 |
| skin/joint_bytes_written/bytes | bytes | 61.2 k | 80.6 k | 57.6 k |
| skin/joints_written/count | count | 956 | 1260 | 900 |
| skin/upload_writes/count | count | 10 | 12 | 9 |
| started_load_count |  | 5838 | 5840 | 5840 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 247 | 247 | 247 |
| statics/materials_bound | count | 247 | 247 | 247 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.15 | 0.15 | 0.15 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 221 | 221 | 221 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 17.40 M | 17.40 M | 17.40 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 899 | 899 | 899 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 18.47 | 26.56 | 18.05 |
| system/mem_usage | % | 58.18 | 58.27 | 58.19 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 187 | 230 | 230 |
| views/all/bins | count | 5645 | 5661 | 5657 |
| views/all/count | count | 10 | 11 | 11 |
| views/all/instance_batches | count | 53 | 71 | 71 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 36.6 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.66 M |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 110 | 112 | 112 |
| views/all/pipelines_distinct | count | 78 | 78 | 78 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.68 M |
| views/cascade/batch_sets | count | 14 | 60 | 60 |
| views/cascade/bins | count | 2043 | 2043 | 2027 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 27 |
| views/cascade/instance_draws | count | 7317 | 22.0 k | 22.0 k |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 248.6 k |
| views/cascade/items | count | 5223 | 5224 | 5207 |
| views/cascade/pipelines | count | 43 | 43 | 43 |
| views/cascade/work_items | count | 83.0 k | 253.8 k | 253.8 k |
| views/lamp_face/batch_sets | count | 58 | 91 | 73 |
| views/lamp_face/bins | count | 137 | 164 | 164 |
| views/lamp_face/count | count | 6 | 7 | 7 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 137 | 164 | 164 |
| views/lamp_face/pipelines | count | 10 | 12 | 12 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 58 | 91 | 73 |
| views/main/batch_sets | count | 97 | 97 | 97 |
| views/main/bins | count | 3466 | 3466 | 3466 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 14.7 k | 14.7 k | 14.7 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6840 | 6840 | 6840 |
| views/main/pipelines | count | 57 | 57 | 57 |
| views/main/work_items | count | 1.42 M | 1.42 M | 1.42 M |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 18.7 k | 18.7 k | 18.7 k |
| world/point_lights | count | 355 | 355 | 355 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 34.75 | 74 | 12.8 k |
| anim/events/count | count | 0.22 | 2 | 82 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 1 | 3 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 8 | 8 | 2952 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 28.25 | 30 | 10.4 k |
| encode/bind_walk/init_textures | count | 9360 | 13.1 k | 3.45 M |
| encode/bind_walk/init_textures_skipped | count | 60.4 k | 71.3 k | 22.29 M |
| encode/bind_walk/inits | count | 802 | 913 | 295.9 k |
| encode/bind_walk/inits_skipped | count | 172 | 192 | 63.6 k |
| encode/bind_walk/merge_textures | count | 9298 | 13.0 k | 3.43 M |
| encode/bind_walk/merge_textures_skipped | count | 19.1 k | 30.0 k | 7.05 M |
| encode/bind_walk/merges | count | 159 | 183 | 58.5 k |
| encode/bind_walk/merges_skipped | count | 159 | 187 | 58.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 369 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 369 |
| encode/pass:early prepass/count | count | 1 | 1 | 369 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 369 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 369 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 369 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 185 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 369 |
| encode/queue/command_buffers | count | 38.75 | 41 | 14.3 k |
| encode/queue/submits | count | 2 | 2 | 738 |
| encode/queue/write_buffer/bytes | bytes | 2.96 M | 3.51 M | 1.09 G |
| encode/queue/write_buffer/calls | count | 282 | 353 | 104.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 53.9 k | 53.9 k | 19.88 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 3321 |
| encode/queue/write_texture/bytes | bytes | 47.1 k | 8.39 M | 17.37 M |
| encode/queue/write_texture/calls | count | 0.01 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 369 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 2.31 | 3 | 853 |
| physics/steps_dropped | count | 0.13 | 3 | 47 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
| scene/load/deferred | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 93.4 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2560.9 | main/busy/ms 2508, stage/main/post_update 2457, stage/main/update 36.49, stage/main/pre_update 10.78, stage/main/last 1.22, surface/advance_cpu_ms 1.00, stage/main/fixed_main_loop 0.59, stage/main/first 0.52, stream/drive/ms 0.40, surface/cell_upload_ms 0.28, stream/tick/ms 0.13, bigspace/total/ms 0.11 |
| warm-up | 3.30 | 743.0 | main/busy/ms 2555, stage/main/post_update 2491, render/thread/ms 684, render/busy/ms 530, stage/render/prepare_assets 505, stage/render/prepare_views 151, render/acquire/ms 151, stage/main/update 42.68, stage/render/render 12.50, stage/main/pre_update 12.37, render/graph/ms 8.33, stage/render/prepare_resources 7.09 |
| warm-up | 3.42 | 115.3 | render/thread/ms 797, render/busy/ms 584, stage/render/prepare_assets 509, stage/render/prepare_views 151, render/acquire/ms 151, stage/render/render 112, main/busy/ms 98.19, pipelines/compile_ms 97.47, pipelines/front_ms 92.06, stage/main/post_update 76.59, render/present/ms 62.33, render/graph/ms 47.45 |
| warm-up | 3.59 | 107.3 | pipelines/wait_ms 3103, pipelines/compile_ms 2116, pipelines/front_ms 1724, pipelines/driver_ms 392, main/busy/ms 180, render/thread/ms 160, render/busy/ms 157, stage/main/post_update 80.11, stage/main/update 75.61, stage/render/prepare_resources 57.33, stage/render/prepare_assets 43.88, pipelines/lock_ms 42.68 |
| warm-up | 3.74 | 144.2 | pipelines/wait_ms 6803, pipelines/compile_ms 3202, pipelines/front_ms 3040, main/busy/ms 198, render/thread/ms 177, render/busy/ms 174, pipelines/driver_ms 161, stage/main/post_update 87.79, stage/main/update 85.65, stage/render/prepare_assets 74.66, stage/render/prepare_resources 48.81, pipelines/lock_ms 21.19 |
| warm-up | 4.64 | 109.6 | pipelines/compile_ms 1697, pipelines/front_ms 1656, render/thread/ms 173, render/busy/ms 171, main/busy/ms 113, pipelines/wait_ms 112, stage/render/prepare_assets 56.11, stage/main/update 55.15, pipelines/driver_ms 41.54, stage/main/post_update 35.10, stage/render/prepare_flush 34.29, stage/render/render 29.74 |
| warm-up | 4.75 | 104.9 | pipelines/compile_ms 3149, pipelines/front_ms 3074, pipelines/wait_ms 536, render/thread/ms 210, render/busy/ms 208, main/busy/ms 164, pipelines/driver_ms 74.51, stage/render/prepare_assets 72.91, stage/main/post_update 69.57, stage/main/update 58.44, stage/render/prepare_flush 32.59, stage/render/render 30.00 |
| warm-up | 4.90 | 147.0 | pipelines/compile_ms 4123, pipelines/front_ms 3992, pipelines/wait_ms 1507, render/thread/ms 247, render/busy/ms 245, stage/render/prepare_assets 150, pipelines/driver_ms 131, main/busy/ms 130, stage/main/post_update 61.77, stage/main/update 39.00, stage/render/prepare_flush 34.43, stage/render/prepare_write_phase_buffers 21.98 |
| warm-up | 7.13 | 159.3 | pipelines/compile_ms 185, render/thread/ms 176, pipelines/front_ms 118, stage/render/render 106, render/present/ms 94.68, pipelines/driver_ms 66.87, stage/render/prepare_views 50.49, render/acquire/ms 50.38, render/busy/ms 30.68, main/busy/ms 24.21, encode//sum 11.00, render/graph/ms 10.57 |
| warm-up | 7.63 | 169.6 | render/thread/ms 249, stage/render/prepare_views 129, render/acquire/ms 129, render/busy/ms 120, stage/render/render 93.22, render/graph/ms 91.71, encode//sum 90.11, stage/render/graph_submit 89.32, encode/finish_wall 82.84, encode/pass:early prepass/sum 81.10, main/busy/ms 34.97, pipelines/compile_ms 31.75 |
| warm-up | 8.05 | 254.1 | render/thread/ms 332, render/busy/ms 190, pipelines/compile_ms 188, stage/render/render 173, render/graph/ms 172, stage/render/graph_submit 169, encode/submit 166, stage/render/prepare_views 140, render/acquire/ms 140, pipelines/front_ms 112, pipelines/driver_ms 75.57, main/busy/ms 33.62 |
| warm-up | 10.20 | 133.1 | encode//sum 375, render/thread/ms 180, pipelines/compile_ms 154, pipelines/front_ms 111, stage/render/prepare_views 100, render/acquire/ms 100, render/busy/ms 77.79, stage/render/render 66.77, render/graph/ms 64.17, stage/render/graph_submit 61.36, encode/finish_wall 55.43, encode/pass:main_opaque_pass_3d/sum 53.67 |
| warm-up | 14.94 | 111.8 | render/thread/ms 192, stage/render/prepare_views 167, render/acquire/ms 167, render/busy/ms 24.50, main/busy/ms 19.47, encode//sum 14.50, stage/render/render 14.25, render/graph/ms 12.32, stage/render/graph_submit 9.52, stage/main/post_update 7.43, encode/submit 5.39, stage/main/update 4.80 |
| warm-up | 15.11 | 94.7 | render/thread/ms 168, stage/render/prepare_views 143, render/acquire/ms 142, main/busy/ms 25.81, render/busy/ms 25.00, encode//sum 12.99, stage/render/render 11.69, render/graph/ms 10.44, stage/render/graph_submit 7.72, stage/main/post_update 7.53, stage/main/update 7.20, stage/render/prepare_assets 6.33 |
| warm-up | 15.36 | 98.0 | render/thread/ms 161, stage/render/prepare_views 138, render/acquire/ms 138, render/busy/ms 22.51, main/busy/ms 21.61, encode//sum 12.44, stage/render/render 11.46, render/graph/ms 10.21, stage/main/post_update 7.78, stage/render/graph_submit 7.77, stage/main/update 4.84, encode/submit 4.80 |
| warm-up | 15.46 | 94.3 | render/thread/ms 190, stage/render/prepare_views 167, render/acquire/ms 166, render/busy/ms 22.64, main/busy/ms 20.51, encode//sum 12.44, stage/render/render 11.85, render/graph/ms 10.62, stage/render/graph_submit 7.76, stage/main/post_update 7.44, encode/submit 4.66, stage/main/update 4.37 |
| warm-up | 15.70 | 98.7 | render/thread/ms 163, stage/render/prepare_views 141, render/acquire/ms 141, render/busy/ms 21.37, main/busy/ms 21.17, stage/render/render 11.94, encode//sum 10.75, render/graph/ms 10.62, stage/main/post_update 8.36, stage/render/graph_submit 7.95, encode/submit 4.96, stage/main/update 4.51 |
| warm-up | 16.04 | 96.9 | render/thread/ms 188, stage/render/prepare_views 166, render/acquire/ms 165, render/busy/ms 21.41, main/busy/ms 21.27, stage/render/render 11.17, encode//sum 10.94, render/graph/ms 9.91, stage/main/post_update 8.33, stage/render/graph_submit 7.33, stage/main/update 4.64, encode/submit 4.58 |
| warm-up | 16.20 | 94.7 | render/thread/ms 157, stage/render/prepare_views 132, render/acquire/ms 132, render/busy/ms 23.61, main/busy/ms 23.05, encode//sum 13.35, stage/render/render 12.16, render/graph/ms 10.80, stage/main/post_update 8.44, stage/render/graph_submit 7.94, stage/main/update 4.77, stage/main/fixed_main_loop 4.56 |
| warm-up | 16.39 | 101.3 | render/thread/ms 186, stage/render/prepare_views 166, render/acquire/ms 166, render/busy/ms 19.39, main/busy/ms 19.12, stage/render/render 11.17, encode//sum 11.03, render/graph/ms 9.95, stage/main/post_update 7.73, stage/render/graph_submit 7.38, encode/submit 4.57, stage/main/update 4.44 |
| warm-up | 16.56 | 95.0 | render/thread/ms 165, stage/render/prepare_views 140, render/acquire/ms 140, render/busy/ms 24.04, main/busy/ms 22.11, encode//sum 12.49, stage/render/render 12.14, render/graph/ms 10.86, stage/main/post_update 7.92, stage/render/graph_submit 7.88, stage/main/update 4.73, stage/render/prepare_assets 4.52 |
| warm-up | 16.65 | 95.1 | render/thread/ms 188, stage/render/prepare_views 163, render/acquire/ms 163, render/busy/ms 23.42, main/busy/ms 23.33, stage/render/render 11.48, encode//sum 11.04, render/graph/ms 10.20, stage/main/post_update 8.88, stage/render/graph_submit 7.29, stage/main/update 4.97, stage/render/prepare_assets 4.41 |
| warm-up | 16.76 | 103.8 | render/thread/ms 196, stage/render/prepare_views 173, render/acquire/ms 173, main/busy/ms 23.17, render/busy/ms 22.92, encode//sum 11.92, stage/render/render 11.71, render/graph/ms 10.44, stage/main/post_update 8.56, stage/render/graph_submit 7.54, stage/main/update 5.20, encode/submit 4.56 |
| warm-up | 16.92 | 94.9 | render/thread/ms 158, stage/render/prepare_views 134, render/acquire/ms 134, render/busy/ms 23.35, main/busy/ms 22.15, stage/render/render 12.31, encode//sum 12.19, render/graph/ms 10.90, stage/main/post_update 8.79, stage/render/graph_submit 8.10, stage/main/update 4.66, encode/submit 4.49 |
| warm-up | 17.11 | 100.1 | render/thread/ms 190, stage/render/prepare_views 166, render/acquire/ms 166, render/busy/ms 23.07, main/busy/ms 22.90, stage/render/render 11.35, encode//sum 11.07, render/graph/ms 10.05, stage/main/post_update 8.05, stage/render/graph_submit 7.48, stage/render/prepare_assets 5.17, encode/submit 4.68 |
| warm-up | 17.21 | 94.6 | render/thread/ms 192, stage/render/prepare_views 172, render/acquire/ms 172, main/busy/ms 23.74, render/busy/ms 19.95, stage/main/post_update 11.34, stage/render/render 11.12, encode//sum 10.95, render/graph/ms 9.82, stage/render/graph_submit 7.03, stage/main/update 4.93, stage/main/fixed_main_loop 4.53 |
| warm-up | 17.37 | 93.4 | render/thread/ms 162, stage/render/prepare_views 133, render/acquire/ms 133, render/busy/ms 28.21, main/busy/ms 26.48, stage/render/render 12.23, encode//sum 11.57, render/graph/ms 10.99, stage/render/prepare_assets 9.36, stage/render/graph_submit 8.36, stage/main/post_update 7.81, encode/submit 5.17 |
| warm-up | 17.47 | 96.8 | render/thread/ms 188, stage/render/prepare_views 165, render/acquire/ms 165, main/busy/ms 22.05, render/busy/ms 21.81, encode//sum 11.32, stage/render/render 11.07, render/graph/ms 9.84, stage/main/post_update 8.06, stage/render/graph_submit 7.31, encode/submit 4.67, stage/main/fixed_main_loop 4.42 |
| warm-up | 17.57 | 100.0 | render/thread/ms 194, stage/render/prepare_views 173, render/acquire/ms 173, main/busy/ms 21.96, render/busy/ms 20.89, encode//sum 11.58, stage/render/render 11.34, render/graph/ms 10.10, stage/main/post_update 8.55, stage/render/graph_submit 7.42, stage/main/fixed_main_loop 4.61, encode/submit 4.58 |
| warm-up | 17.84 | 99.8 | render/thread/ms 188, stage/render/prepare_views 166, render/acquire/ms 166, main/busy/ms 22.11, render/busy/ms 20.82, stage/render/render 11.24, encode//sum 11.10, render/graph/ms 9.94, stage/main/post_update 7.89, stage/render/graph_submit 7.26, stage/main/update 4.97, encode/submit 4.33 |
| warm-up | 18.09 | 95.2 | render/thread/ms 165, stage/render/prepare_views 140, render/acquire/ms 140, render/busy/ms 24.20, main/busy/ms 22.68, stage/render/render 12.21, encode//sum 11.54, render/graph/ms 10.93, stage/render/graph_submit 8.06, stage/main/post_update 7.59, encode/submit 4.79, stage/render/prepare_assets 4.69 |
| measure | 9.57 | 96.6 | render/thread/ms 163, stage/render/prepare_views 144, render/acquire/ms 144, render/busy/ms 17.95, main/busy/ms 17.30, stage/render/render 9.41, render/graph/ms 8.21, stage/main/post_update 7.67, encode//sum 7.07, stage/render/graph_submit 6.00, encode/submit 3.55, stage/main/update 3.16 |
| measure | 9.73 | 94.1 | render/thread/ms 157, stage/render/prepare_views 139, render/acquire/ms 138, render/busy/ms 18.30, main/busy/ms 17.69, stage/render/render 9.52, render/graph/ms 8.29, encode//sum 7.71, stage/main/post_update 7.60, stage/render/graph_submit 5.80, stage/main/fixed_main_loop 3.94, stage/main/update 3.48 |
| measure | 9.90 | 101.8 | render/thread/ms 163, stage/render/prepare_views 145, render/acquire/ms 145, main/busy/ms 18.08, render/busy/ms 18.00, stage/render/render 9.09, render/graph/ms 7.85, stage/main/post_update 7.72, encode//sum 7.64, stage/render/graph_submit 5.59, stage/main/fixed_main_loop 3.71, encode/submit 3.35 |
| measure | 10.23 | 102.6 | render/thread/ms 168, stage/render/prepare_views 150, render/acquire/ms 150, main/busy/ms 18.49, render/busy/ms 17.11, stage/render/render 8.85, render/graph/ms 7.75, stage/main/post_update 7.67, encode//sum 7.23, stage/render/graph_submit 5.44, stage/main/fixed_main_loop 4.07, stage/main/update 3.36 |
| measure | 10.40 | 97.9 | render/thread/ms 160, stage/render/prepare_views 141, render/acquire/ms 141, render/busy/ms 18.31, main/busy/ms 17.60, stage/render/render 9.58, render/graph/ms 8.35, encode//sum 8.26, stage/main/post_update 7.41, stage/render/graph_submit 5.95, stage/main/fixed_main_loop 4.25, encode/submit 3.41 |
| measure | 10.56 | 102.6 | render/thread/ms 164, stage/render/prepare_views 145, render/acquire/ms 145, render/busy/ms 18.40, main/busy/ms 18.27, stage/render/render 9.91, render/graph/ms 8.64, encode//sum 8.27, stage/main/post_update 7.79, stage/render/graph_submit 6.31, stage/main/fixed_main_loop 3.96, encode/submit 3.83 |
| measure | 10.73 | 97.0 | render/thread/ms 162, stage/render/prepare_views 142, render/acquire/ms 141, render/busy/ms 19.60, main/busy/ms 19.24, stage/render/render 10.23, render/graph/ms 8.98, encode//sum 8.62, stage/main/post_update 8.46, stage/render/graph_submit 6.49, stage/main/fixed_main_loop 4.23, encode/submit 3.79 |
| measure | 10.89 | 97.0 | render/thread/ms 159, stage/render/prepare_views 140, render/acquire/ms 140, render/busy/ms 18.40, main/busy/ms 17.85, stage/render/render 9.54, render/graph/ms 8.36, encode//sum 7.49, stage/main/post_update 7.29, stage/render/graph_submit 6.14, stage/main/fixed_main_loop 4.18, encode/submit 3.91 |
| measure | 11.05 | 96.1 | render/thread/ms 161, stage/render/prepare_views 142, render/acquire/ms 142, render/busy/ms 18.78, main/busy/ms 17.11, stage/render/render 10.57, render/graph/ms 9.26, encode//sum 8.25, stage/main/post_update 7.63, stage/render/graph_submit 6.73, encode/submit 3.89, stage/main/fixed_main_loop 3.83 |
| measure | 11.21 | 98.3 | render/thread/ms 160, stage/render/prepare_views 141, render/acquire/ms 141, main/busy/ms 19.89, render/busy/ms 18.76, stage/render/render 10.13, encode//sum 9.15, render/graph/ms 8.85, stage/main/post_update 7.79, stage/render/graph_submit 6.55, stage/main/fixed_main_loop 4.63, stage/main/update 4.22 |
| measure | 11.38 | 96.0 | render/thread/ms 161, stage/render/prepare_views 141, render/acquire/ms 141, render/busy/ms 19.23, main/busy/ms 17.96, stage/render/render 10.83, encode//sum 9.85, render/graph/ms 9.55, stage/main/post_update 7.18, stage/render/graph_submit 6.94, stage/main/fixed_main_loop 4.24, encode/submit 4.07 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0\gc_ascadian@2560x1440_arm=on-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **99%** (min 99%) · 2625 MHz (min 2625) · memory 8101 MHz · 47.5 W · 64 °C (max 66) · VRAM 7631 MiB · throttled 100% of samples

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
- `sf.graphics_preset=high (preset: level requested)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=high (preset: level requested)`
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
