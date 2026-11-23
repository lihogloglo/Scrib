# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1426 frames over 15.0 s · **median 10.46 ms (96 fps)** · mean 10.53 · p95 12.08 · p99 13.20 (1% low 76 fps) · worst 21.62 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 308 frames over 8.9 s, worst 2553.0 ms
- hitches (> 26.2 ms): **0** measured, 47 in warm-up
- timed GPU passes: 8.34 ms of a 10.53 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.46 ms, limit 11 ms
- pass worst frame 21.62 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.970 | 2.063 | 2.087 | 0.96 | 0.179 |
| main_opaque_pass_3d | 1.964 | 2.057 | 2.095 | 0.96 | 0.064 |
| early prepass | 0.716 | 0.749 | 0.764 | 0.96 | 0.094 |
| sun_visibility | 0.661 | 0.692 | 0.706 | 0.96 | 0.006 |
| atmosphere_luts | 0.343 | 0.359 | 0.372 | 0.96 | 0.003 |
| water_velocity | 0.315 | 0.330 | 0.333 | 0.96 | 0.002 |
| bloom | 0.306 | 0.321 | 0.338 | 0.96 | 0.064 |
| atmos_froxel_inject | 0.294 | 0.308 | 0.310 | 0.96 | 0.001 |
| water_sspr | 0.198 | 0.207 | 0.210 | 0.96 | 0.002 |
| ssao | 0.180 | 0.189 | 0.197 | 0.96 | 0.007 |
| uniform_allocation | 0.134 | 0.140 | 0.184 | 0.96 | 0.022 |
| water_fft | 0.130 | 0.136 | 0.137 | 0.96 | 0.002 |
| upscaling | 0.116 | 0.122 | 0.127 | 0.96 | 0.004 |
| clouds_composite | 0.104 | 0.109 | 0.111 | 0.96 | 0.002 |
| bin_unpacking | 0.094 | 0.098 | 0.131 | 0.96 | 0.016 |
| tonemapping | 0.081 | 0.085 | 0.089 | 0.96 | 0.003 |
| gpu_instance_materialization | 0.080 | 0.084 | 0.093 | 0.96 | 0.004 |
| main_transmissive_pass_3d | 0.076 | 0.080 | 0.092 | 0.96 | 0.008 |
| clustering | 0.067 | 0.070 | 0.080 | 0.96 | 0.019 |
| render_sky | 0.061 | 0.063 | 0.065 | 0.96 | 0.002 |
| early_mesh_preprocessing | 0.060 | 0.063 | 0.092 | 0.96 | 0.004 |
| clouds_trace | 0.054 | 0.057 | 0.067 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.050 | 0.410 | 0.415 | 0.12 | 0.001 |
| clouds_resolve | 0.046 | 0.048 | 0.050 | 0.96 | 0.001 |
| point_shadow_atlas | 0.035 | 0.072 | 0.079 | 0.49 | 0.127 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.030 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.218 | 0.226 | 0.12 | 0.004 |
| water_snapshot | 0.022 | 0.023 | 0.025 | 0.96 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.96 | 0.002 |
| range_unpacking | 0.021 | 0.022 | 0.027 | 0.96 | 0.003 |
| clouds_shadow | 0.020 | 0.021 | 0.023 | 0.96 | 0.003 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.037 | 0.96 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.026 | 0.96 | 0.235 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.96 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.015 | 0.016 | 0.49 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.96 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.49 | 0.001 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.49 | 0.001 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.49 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.96 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.003 | 0.003 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.226 | 10.751 | 20.50 | 13156.5 |
| render/busy/ms | 8.286 | 9.899 | 18.54 | 11816.5 |
| main/busy/ms | 8.010 | 8.982 | 10.74 | 11422.0 |
| stage/render/render | 5.288 | 6.841 | 15.51 | 7540.9 |
| render/graph/ms | 4.661 | 5.976 | 14.83 | 6646.7 |
| stage/main/post_update | 4.576 | 5.429 | 6.60 | 6525.9 |
| encode//sum | 4.203 | 6.310 | 13.22 | 5993.6 |
| stage/render/graph_submit | 3.359 | 4.561 | 13.56 | 4790.4 |
| encode/submit | 2.078 | 2.915 | 5.87 | 2962.6 |
| stage/main/update | 1.767 | 2.076 | 2.54 | 2519.6 |
| encode/finish_wall | 1.329 | 1.941 | 10.53 | 1895.0 |
| stage/render/camera_driver | 1.239 | 1.432 | 1.69 | 1767.4 |
| encode/pass:shadow_directional_light/sum | 0.862 | 1.538 | 3.58 | 1229.2 |
| encode/pass:early prepass/sum | 0.824 | 1.146 | 10.43 | 1174.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.811 | 1.161 | 1.63 | 1157.1 |
| stage/render/prepare_views | 0.608 | 2.116 | 3.49 | 866.3 |
| stage/render/prepare_assets | 0.572 | 0.707 | 1.75 | 815.5 |
| render/acquire/ms | 0.563 | 2.064 | 3.43 | 803.0 |
| bigspace/total/ms | 0.541 | 1.252 | 2.47 | 772.1 |
| stage/render/prepare_write_phase_buffers | 0.500 | 0.730 | 0.95 | 712.9 |
| stage/render/prepare_meshes | 0.486 | 0.644 | 1.41 | 692.4 |
| stage/main/fixed_main_loop | 0.452 | 0.769 | 1.28 | 644.1 |
| stage/render/prepare_resources | 0.419 | 0.489 | 1.26 | 597.2 |
| stage/main/pre_update | 0.405 | 0.540 | 1.34 | 576.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.402 | 0.611 | 1.53 | 573.3 |
| render/present/ms | 0.377 | 0.507 | 1.69 | 537.1 |
| pipelines/compile_ms | 0.362 | 0.000 | 121.64 | 516.8 |
| stage/main/first | 0.357 | 0.442 | 3.25 | 509.7 |
| pipelines/front_ms | 0.353 | 0.000 | 119.17 | 503.9 |
| controller/feed/ms | 0.321 | 0.408 | 0.77 | 458.0 |
| stage/render/prepare_flush | 0.310 | 0.512 | 0.85 | 442.8 |
| stage/render/prepare_batch_phases | 0.310 | 0.433 | 0.62 | 442.0 |
| stage/main/last | 0.271 | 0.325 | 0.75 | 386.0 |
| stage/render/prepare_bind_groups | 0.269 | 0.407 | 0.57 | 383.0 |
| bigspace/low_precision/ms | 0.238 | 0.675 | 1.59 | 340.0 |
| stage/render/create_views | 0.189 | 0.233 | 0.66 | 269.0 |
| stream/drive/ms | 0.183 | 0.251 | 0.59 | 261.0 |
| stage/render/extract_commands | 0.177 | 0.207 | 0.60 | 252.3 |
| anim/evaluate_cpu/ms | 0.165 | 0.198 | 0.53 | 235.0 |
| stage/render/specialize | 0.160 | 0.221 | 0.63 | 227.7 |
| anim/evaluate/ms | 0.155 | 0.299 | 1.08 | 221.5 |
| stream/tick/ms | 0.153 | 0.215 | 0.56 | 217.8 |
| bigspace/high_precision/ms | 0.148 | 0.561 | 1.77 | 210.6 |
| stage/render/queue | 0.130 | 0.170 | 0.35 | 185.9 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.118 | 0.349 | 1.06 | 167.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.110 | 0.181 | 0.75 | 157.3 |
| controller/write/ms | 0.064 | 0.082 | 0.51 | 91.7 |
| encode/resolve_diagnostics/sum | 0.046 | 0.082 | 0.39 | 66.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.070 | 0.26 | 65.7 |
| controller/move/ms | 0.044 | 0.084 | 0.44 | 63.4 |
| physics/step/ms | 0.036 | 0.066 | 0.52 | 51.9 |
| terrain/update/ms | 0.036 | 0.049 | 0.09 | 50.7 |
| stage/render/phase_sort | 0.035 | 0.052 | 0.09 | 49.5 |
| stage/render/cleanup | 0.024 | 0.032 | 0.43 | 33.7 |
| encode/camera_driver_span_begin/sum | 0.019 | 0.035 | 0.19 | 26.7 |
| encode/camera_driver_span_end/sum | 0.017 | 0.036 | 0.27 | 24.5 |
| weather/map/ms | 0.016 | 0.021 | 0.15 | 22.8 |
| anim/apply/ms | 0.010 | 0.012 | 0.09 | 14.2 |
| controller/avoid/ms | 0.009 | 0.019 | 0.04 | 13.3 |
| weather/eval/ms | 0.009 | 0.012 | 0.08 | 13.3 |
| pipelines/driver_ms | 0.009 | 0.000 | 3.29 | 12.9 |
| script/update/ms | 0.007 | 0.011 | 0.03 | 10.4 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.1 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 7.1 |
| far/select/ms | 0.004 | 0.005 | 0.04 | 5.8 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.03 | 2.9 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 2.0 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.02 | 1.9 |
| stream/promote/ms | 0.001 | 0.002 | 0.07 | 1.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.39 | 1.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| terrain/select/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/evaluated/count | count | 26.50 | 27 | 26 |
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
| crowd/joints_extracted/count | count | 704 | 1163 | 652 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.7 k | 43.7 k | 43.7 k |
| encode//max | ms | 0.52 | 10.44 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.19 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.27 | 0.01 |
| encode/finish_critical | ms | 0.97 | 10.44 | 1.15 |
| encode/pass:early prepass/max | ms | 0.81 | 10.43 | 0.78 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 1.63 | 0.98 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.75 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.38 | 1.53 | 0.34 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.21 | 1.06 | 0.18 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 3.58 | 1.15 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.39 | 0.03 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 9.37 | 18.54 | 9.35 |
| gpu/timed/ms | ms | 9.36 | 10.60 | 9.35 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2431 | 2431 | 2431 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 301.1 k | 301.1 k | 301.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.6 k | 12.6 k | 12.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 343.66 M | 343.66 M | 343.66 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 354 | 354 | 354 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 559 | 559 | 559 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 559 | 559 | 559 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 354 | 354 | 354 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 559 | 559 | 559 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 559 | 559 | 559 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 1361 | 1361 | 1361 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 6753 | 6753 | 6753 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2843 | 2843 | 2843 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 20 | 20 | 20 |
| phases/opaque3d_prepass/bins | count | 1361 | 1361 | 1361 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 6753 | 6753 | 6753 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2843 | 2843 | 2843 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 85 | 106 | 106 |
| phases/shadow/bins | count | 2165 | 2179 | 2165 |
| phases/shadow/instance_batches | count | 4 | 12 | 12 |
| phases/shadow/instance_draws | count | 7312 | 21.9 k | 21.9 k |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5344 | 5358 | 5344 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 47 | 48 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 342 | 342 | 342 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 9 | 67 | 67 |
| preprocess/bind_groups_created | count | 4 | 60 | 60 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 28.71 | 46.63 | 29.28 |
| process/mem_usage | GiB | 3.11 | 3.22 | 3.22 |
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
| shadow_atlas/live_occ_pct | % | 0.59 | 0.59 | 0.59 |
| shadow_atlas/live_tiles | count | 6 | 6 | 6 |
| shadow_atlas/mover_draws | count | 1 | 4 | 0 |
| shadow_atlas/movers | count | 85 | 100 | 85 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 2 | 4 | 2 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 2 | 4 | 2 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 6 | 6 | 6 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 6 | 6 | 6 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 17 | 26 | 16 |
| skin/blocks_skipped/count | count | 192 | 202 | 193 |
| skin/blocks_walked/count | count | 18 | 27 | 17 |
| skin/joint_bytes_written/bytes | bytes | 55.8 k | 76.8 k | 49.9 k |
| skin/joints_written/count | count | 872 | 1200 | 780 |
| skin/upload_writes/count | count | 8 | 11 | 8 |
| started_load_count |  | 5819 | 5820 | 5820 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
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
| system/cpu_usage | % | 31.93 | 40.88 | 29.90 |
| system/mem_usage | % | 50.95 | 51.38 | 51.38 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2950 | 2950 | 2950 |
| textures/images | count | 2500 | 2500 | 2500 |
| views/all/batch_sets | count | 146 | 167 | 167 |
| views/all/bins | count | 5595 | 5609 | 5595 |
| views/all/count | count | 10 | 10 | 10 |
| views/all/instance_batches | count | 12 | 20 | 20 |
| views/all/instance_draws | count | 21.9 k | 36.6 k | 36.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 245.8 k |
| views/all/items | count | 12.1 k | 12.2 k | 12.1 k |
| views/all/pipelines | count | 97 | 98 | 97 |
| views/all/pipelines_distinct | count | 69 | 69 | 69 |
| views/all/work_items | count | 154.9 k | 258.2 k | 258.2 k |
| views/cascade/batch_sets | count | 9 | 45 | 45 |
| views/cascade/bins | count | 2028 | 2028 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 12 |
| views/cascade/instance_draws | count | 7312 | 21.9 k | 21.9 k |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 5207 | 5207 | 5207 |
| views/cascade/pipelines | count | 37 | 37 | 37 |
| views/cascade/work_items | count | 49.3 k | 152.7 k | 152.7 k |
| views/lamp_face/batch_sets | count | 58 | 91 | 61 |
| views/lamp_face/bins | count | 137 | 151 | 137 |
| views/lamp_face/count | count | 6 | 6 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 137 | 151 | 137 |
| views/lamp_face/pipelines | count | 10 | 11 | 10 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 58 | 91 | 61 |
| views/main/batch_sets | count | 61 | 61 | 61 |
| views/main/bins | count | 3430 | 3430 | 3430 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 14.6 k | 14.6 k | 14.6 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6804 | 6804 | 6804 |
| views/main/pipelines | count | 50 | 50 | 50 |
| views/main/work_items | count | 105.5 k | 105.5 k | 105.5 k |
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
| anim/bones_written/count | count | 30.92 | 66 | 44.1 k |
| anim/events/count | count | 0.03 | 1 | 49 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.53 | 8 | 3608 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 30 | 40.3 k |
| encode/bind_walk/init_textures | count | 9084 | 12.8 k | 12.95 M |
| encode/bind_walk/init_textures_skipped | count | 52.4 k | 61.5 k | 74.78 M |
| encode/bind_walk/inits | count | 653 | 742 | 931.3 k |
| encode/bind_walk/inits_skipped | count | 169 | 182 | 241.4 k |
| encode/bind_walk/merge_textures | count | 9022 | 12.7 k | 12.86 M |
| encode/bind_walk/merge_textures_skipped | count | 8734 | 17.8 k | 12.46 M |
| encode/bind_walk/merges | count | 115 | 132 | 164.1 k |
| encode/bind_walk/merges_skipped | count | 155 | 176 | 221.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1426 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1426 |
| encode/pass:early prepass/count | count | 1 | 1 | 1426 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1426 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1426 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1426 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 714 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1426 |
| encode/queue/command_buffers | count | 38.75 | 41 | 55.3 k |
| encode/queue/submits | count | 2 | 2 | 2852 |
| encode/queue/write_buffer/bytes | bytes | 2.72 M | 3.47 M | 3.88 G |
| encode/queue/write_buffer/calls | count | 261 | 333 | 372.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 55.6 k | 55.6 k | 79.22 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 12.8 k |
| encode/queue/write_texture/bytes | bytes | 18.5 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1426 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.63 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 7 |
| pipelines/created | count | 0.00 | 1 | 7 |
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

## Hitches (> 26.2 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2553.0 | main/busy/ms 2517, stage/main/post_update 2470, stage/main/update 34.34, stage/main/pre_update 9.17, surface/advance_cpu_ms 1.11, stage/main/last 1.02, surface/cell_upload_ms 0.47, stage/main/fixed_main_loop 0.47, stage/main/first 0.46, stream/drive/ms 0.41, stream/tick/ms 0.09, bigspace/total/ms 0.08 |
| warm-up | 3.25 | 701.1 | main/busy/ms 2564, stage/main/post_update 2503, render/thread/ms 643, render/busy/ms 516, stage/render/prepare_assets 465, stage/render/prepare_views 124, render/acquire/ms 124, stage/main/update 42.29, stage/render/create_views 29.21, stage/render/render 14.27, stage/main/pre_update 11.15, render/graph/ms 10.14 |
| warm-up | 3.33 | 74.2 | render/thread/ms 714, render/busy/ms 536, stage/render/prepare_assets 470, stage/render/prepare_views 125, render/acquire/ms 125, main/busy/ms 117, pipelines/compile_ms 83.23, stage/render/render 73.81, stage/main/post_update 60.17, render/present/ms 54.22, pipelines/front_ms 44.57, pipelines/driver_ms 38.67 |
| warm-up | 3.39 | 65.9 | pipelines/compile_ms 367, pipelines/wait_ms 319, pipelines/front_ms 262, render/thread/ms 135, main/busy/ms 114, pipelines/driver_ms 105, stage/render/render 98.70, render/busy/ms 72.72, render/present/ms 62.06, stage/main/post_update 45.00, stage/main/last 35.04, render/graph/ms 35.03 |
| warm-up | 3.54 | 143.9 | pipelines/wait_ms 4496, pipelines/compile_ms 1582, pipelines/front_ms 1427, render/thread/ms 192, render/busy/ms 181, main/busy/ms 176, pipelines/driver_ms 155, stage/main/update 80.28, stage/render/render 75.11, encode//sum 70.49, render/graph/ms 62.40, stage/render/graph_submit 57.89 |
| warm-up | 3.62 | 85.1 | pipelines/wait_ms 5627, pipelines/compile_ms 2867, pipelines/front_ms 2689, render/thread/ms 204, render/busy/ms 201, pipelines/driver_ms 179, main/busy/ms 177, stage/main/update 82.17, stage/render/prepare_assets 78.12, encode//sum 72.88, stage/main/pre_update 52.01, stage/render/render 43.27 |
| warm-up | 3.66 | 39.6 | pipelines/compile_ms 1704, pipelines/front_ms 1607, pipelines/wait_ms 1466, render/thread/ms 106, render/busy/ms 104, pipelines/driver_ms 97.63, stage/render/prepare_assets 76.71, main/busy/ms 62.25, stage/main/post_update 24.65, stage/render/render 19.09, render/graph/ms 16.33, terrain/update/ms 15.52 |
| warm-up | 3.69 | 30.2 | pipelines/compile_ms 219, pipelines/front_ms 205, pipelines/wait_ms 62.91, render/thread/ms 56.89, render/busy/ms 55.06, main/busy/ms 37.04, stage/render/prepare_assets 30.28, stage/main/post_update 16.56, stage/render/render 16.26, pipelines/driver_ms 14.34, encode//sum 14.20, render/graph/ms 13.92 |
| warm-up | 3.78 | 40.4 | pipelines/compile_ms 748, pipelines/front_ms 718, render/thread/ms 58.12, render/busy/ms 55.28, main/busy/ms 40.12, pipelines/driver_ms 30.10, pipelines/wait_ms 22.44, stage/render/render 18.19, stage/main/update 16.85, stage/main/post_update 16.47, render/graph/ms 15.00, stage/render/prepare_resources 13.98 |
| warm-up | 3.83 | 48.1 | pipelines/compile_ms 965, pipelines/front_ms 915, render/thread/ms 82.71, render/busy/ms 77.48, pipelines/driver_ms 49.90, main/busy/ms 41.67, stage/render/prepare_assets 31.89, stage/render/render 20.27, render/graph/ms 16.80, stage/main/pre_update 13.22, stage/render/graph_submit 13.15, stage/render/prepare_resources 12.77 |
| warm-up | 3.88 | 53.6 | pipelines/compile_ms 1557, pipelines/front_ms 1477, render/thread/ms 95.92, render/busy/ms 90.77, pipelines/driver_ms 80.60, stage/render/prepare_assets 52.79, main/busy/ms 36.07, pipelines/wait_ms 23.70, stage/render/render 23.40, render/graph/ms 19.96, stage/render/graph_submit 16.57, stage/main/pre_update 11.18 |
| warm-up | 3.97 | 87.0 | pipelines/compile_ms 1443, pipelines/front_ms 1221, pipelines/driver_ms 223, render/thread/ms 135, render/busy/ms 133, stage/render/prepare_assets 105, pipelines/wait_ms 29.48, main/busy/ms 29.31, stage/render/render 21.03, render/graph/ms 18.22, stage/render/graph_submit 15.25, stage/main/post_update 11.22 |
| warm-up | 4.00 | 31.9 | pipelines/compile_ms 784, pipelines/front_ms 599, pipelines/driver_ms 185, render/thread/ms 115, render/busy/ms 113, stage/render/prepare_assets 96.89, main/busy/ms 26.24, stage/render/render 12.03, pipelines/wait_ms 10.86, render/graph/ms 10.15, stage/main/post_update 9.76, stage/render/graph_submit 7.49 |
| warm-up | 4.18 | 30.6 | pipelines/compile_ms 896, pipelines/front_ms 860, render/thread/ms 48.62, render/busy/ms 45.96, main/busy/ms 42.73, pipelines/driver_ms 35.98, stage/main/update 18.69, stage/render/render 18.15, stage/main/post_update 16.45, render/graph/ms 15.16, stage/render/prepare_assets 15.03, stage/render/prepare_write_phase_buffers 7.99 |
| warm-up | 4.25 | 68.2 | pipelines/compile_ms 906, pipelines/front_ms 871, render/thread/ms 87.80, render/busy/ms 84.98, main/busy/ms 70.20, pipelines/driver_ms 35.42, stage/render/render 31.47, stage/main/post_update 29.99, render/graph/ms 28.33, stage/main/update 26.69, stage/render/prepare_write_phase_buffers 24.35, stage/render/prepare_assets 20.07 |
| warm-up | 4.38 | 135.3 | pipelines/compile_ms 1342, pipelines/front_ms 1299, render/thread/ms 186, render/busy/ms 184, main/busy/ms 134, stage/main/update 86.03, stage/render/prepare_resources 55.63, pipelines/driver_ms 43.58, stage/render/render 40.31, render/graph/ms 37.72, stage/main/post_update 31.41, stage/render/prepare_write_phase_buffers 24.44 |
| warm-up | 4.52 | 136.5 | pipelines/compile_ms 3945, pipelines/front_ms 3823, pipelines/wait_ms 690, render/thread/ms 261, render/busy/ms 259, main/busy/ms 175, pipelines/driver_ms 122, stage/main/update 106, stage/render/prepare_resources 76.06, stage/render/prepare_assets 70.85, stage/main/post_update 49.96, stage/render/render 29.87 |
| warm-up | 4.64 | 122.1 | pipelines/compile_ms 4027, pipelines/front_ms 3894, pipelines/wait_ms 1010, render/thread/ms 247, render/busy/ms 245, stage/render/prepare_assets 150, pipelines/driver_ms 133, main/busy/ms 120, stage/main/update 47.49, stage/main/post_update 40.84, stage/render/prepare_resources 22.87, stage/render/render 22.43 |
| warm-up | 4.68 | 36.1 | pipelines/compile_ms 1807, pipelines/front_ms 1739, pipelines/wait_ms 565, render/thread/ms 148, render/busy/ms 147, stage/render/prepare_assets 104, pipelines/driver_ms 68.05, main/busy/ms 64.42, stage/main/update 28.01, stage/render/render 20.30, render/graph/ms 18.05, stage/render/graph_submit 14.59 |
| warm-up | 4.71 | 35.1 | pipelines/compile_ms 837, pipelines/front_ms 808, pipelines/wait_ms 243, render/thread/ms 67.91, render/busy/ms 66.40, main/busy/ms 52.34, stage/main/update 32.05, pipelines/driver_ms 28.74, stage/render/prepare_assets 22.90, stage/render/render 14.98, render/graph/ms 12.68, stage/main/post_update 10.96 |
| warm-up | 4.74 | 31.3 | pipelines/compile_ms 843, pipelines/front_ms 815, render/thread/ms 62.93, render/busy/ms 61.34, main/busy/ms 56.90, pipelines/wait_ms 48.38, stage/main/update 29.58, pipelines/driver_ms 27.78, stage/main/post_update 18.70, stage/render/render 17.43, stage/render/prepare_assets 15.98, render/graph/ms 15.19 |
| warm-up | 4.77 | 30.8 | pipelines/compile_ms 500, pipelines/front_ms 484, render/thread/ms 58.58, render/busy/ms 56.97, main/busy/ms 53.64, pipelines/wait_ms 49.50, stage/main/update 24.32, stage/main/post_update 20.80, stage/render/prepare_assets 19.28, pipelines/driver_ms 16.24, stage/render/render 15.06, render/graph/ms 13.05 |
| warm-up | 4.80 | 29.6 | pipelines/compile_ms 71.97, pipelines/front_ms 68.42, render/thread/ms 56.97, render/busy/ms 55.36, main/busy/ms 53.90, stage/main/update 29.99, stage/main/post_update 15.98, stage/render/prepare_assets 15.93, stage/render/render 13.53, render/graph/ms 11.52, encode//sum 8.60, stage/render/graph_submit 7.74 |
| warm-up | 4.84 | 32.2 | pipelines/compile_ms 448, pipelines/front_ms 438, render/thread/ms 55.99, render/busy/ms 54.50, main/busy/ms 52.87, stage/main/update 26.56, stage/render/prepare_assets 18.08, stage/main/post_update 16.88, stage/render/render 13.37, render/graph/ms 11.29, pipelines/driver_ms 10.49, encode//sum 9.44 |
| warm-up | 4.95 | 30.6 | pipelines/compile_ms 613, pipelines/front_ms 595, render/thread/ms 40.65, render/busy/ms 36.19, main/busy/ms 29.50, stage/render/render 28.31, render/graph/ms 23.38, pipelines/driver_ms 18.69, stage/render/camera_driver 13.40, stage/main/update 13.30, stage/render/graph_submit 9.84, stage/main/post_update 9.61 |
| warm-up | 5.04 | 31.6 | pipelines/compile_ms 77.05, pipelines/front_ms 72.68, render/thread/ms 45.05, render/busy/ms 43.62, main/busy/ms 42.78, stage/main/update 20.98, stage/render/render 16.74, stage/main/post_update 16.00, render/graph/ms 14.88, stage/render/graph_submit 10.61, stage/render/prepare_flush 9.08, encode//sum 7.69 |
| warm-up | 5.08 | 33.2 | render/thread/ms 57.30, main/busy/ms 57.26, render/busy/ms 55.76, stage/main/post_update 23.60, stage/render/render 20.09, stage/main/update 18.97, pipelines/compile_ms 15.58, pipelines/front_ms 14.65, render/graph/ms 14.04, bigspace/total/ms 11.43, stage/render/graph_submit 10.59, stage/main/pre_update 9.87 |
| warm-up | 5.14 | 34.6 | pipelines/compile_ms 1010, pipelines/front_ms 984, render/thread/ms 52.82, render/busy/ms 50.06, main/busy/ms 45.78, pipelines/driver_ms 26.24, stage/main/update 23.85, stage/render/render 14.22, render/graph/ms 12.40, stage/main/post_update 11.33, stage/render/prepare_write_phase_buffers 9.31, encode//sum 8.93 |
| warm-up | 5.17 | 27.9 | pipelines/compile_ms 1433, pipelines/front_ms 1393, render/thread/ms 54.51, render/busy/ms 53.05, main/busy/ms 51.77, pipelines/driver_ms 39.90, stage/main/update 25.67, stage/render/render 14.54, render/graph/ms 12.54, stage/main/post_update 12.37, stage/render/graph_submit 8.83, stage/render/prepare_write_phase_buffers 8.62 |
| warm-up | 5.21 | 44.7 | pipelines/compile_ms 1123, pipelines/front_ms 1090, render/thread/ms 63.07, render/busy/ms 61.01, main/busy/ms 49.81, pipelines/driver_ms 33.35, stage/main/update 25.40, stage/render/prepare_assets 15.41, stage/render/render 14.94, stage/main/post_update 13.22, render/graph/ms 12.79, stage/render/graph_submit 8.96 |
| warm-up | 5.24 | 28.3 | pipelines/compile_ms 1093, pipelines/front_ms 1065, render/thread/ms 63.89, render/busy/ms 61.95, main/busy/ms 49.83, pipelines/driver_ms 28.71, stage/main/update 28.37, stage/render/prepare_assets 15.80, stage/render/render 15.08, render/graph/ms 13.00, stage/main/post_update 12.80, stage/render/graph_submit 9.02 |
| warm-up | 5.27 | 27.4 | pipelines/compile_ms 802, pipelines/front_ms 781, render/thread/ms 52.54, render/busy/ms 50.83, main/busy/ms 48.42, stage/main/update 24.35, pipelines/driver_ms 20.43, stage/render/render 20.10, render/graph/ms 17.64, pipelines/wait_ms 16.36, stage/main/post_update 14.32, stage/render/graph_submit 12.81 |
| warm-up | 5.32 | 26.3 | pipelines/compile_ms 192, pipelines/front_ms 188, render/thread/ms 46.99, render/busy/ms 45.74, main/busy/ms 33.66, stage/render/prepare_assets 24.32, stage/main/update 13.21, stage/main/post_update 12.39, stage/render/render 11.03, render/graph/ms 9.32, encode//sum 7.97, stage/render/graph_submit 5.90 |
| warm-up | 5.38 | 40.8 | pipelines/compile_ms 626, pipelines/front_ms 604, render/thread/ms 62.00, render/busy/ms 59.84, main/busy/ms 45.01, stage/render/prepare_assets 25.46, stage/render/render 22.04, pipelines/driver_ms 21.94, render/graph/ms 18.97, stage/main/update 16.99, stage/main/post_update 16.07, stage/render/graph_submit 13.67 |
| warm-up | 5.54 | 32.7 | render/thread/ms 48.05, render/busy/ms 46.23, main/busy/ms 44.53, stage/main/update 20.67, stage/render/render 19.97, render/graph/ms 17.63, stage/main/post_update 16.30, stage/render/graph_submit 13.36, encode//sum 9.22, stage/render/prepare_assets 8.51, encode/submit 8.29, stream/drive/ms 5.15 |
| warm-up | 5.57 | 32.9 | render/thread/ms 59.95, render/busy/ms 57.96, main/busy/ms 56.41, pipelines/compile_ms 47.83, pipelines/front_ms 46.11, stage/main/update 32.25, stage/render/render 19.90, render/graph/ms 17.41, stage/render/prepare_assets 15.01, stage/main/post_update 13.21, stage/render/graph_submit 12.99, character/finish/ms 11.39 |
| warm-up | 5.62 | 54.9 | render/thread/ms 81.57, render/busy/ms 80.07, main/busy/ms 52.48, pipelines/compile_ms 47.83, pipelines/front_ms 46.11, stage/render/prepare_assets 44.37, stage/main/update 30.49, stage/render/render 12.18, stage/main/post_update 10.69, render/graph/ms 10.23, encode//sum 8.90, character/finish/ms 8.63 |
| warm-up | 5.67 | 33.2 | render/thread/ms 44.85, render/busy/ms 43.71, main/busy/ms 31.91, stage/render/prepare_assets 19.16, stage/main/update 14.54, stage/render/render 13.16, render/graph/ms 11.42, encode//sum 9.69, stage/main/post_update 9.52, stage/render/graph_submit 8.24, encode/submit 5.33, stream/drive/ms 3.41 |
| warm-up | 5.70 | 32.4 | pipelines/compile_ms 379, pipelines/front_ms 367, render/thread/ms 61.50, render/busy/ms 59.52, main/busy/ms 37.99, stage/render/prepare_assets 22.81, stage/render/render 19.50, stage/main/post_update 16.80, render/graph/ms 16.61, stage/main/update 15.51, stage/render/graph_submit 12.50, pipelines/driver_ms 11.64 |
| warm-up | 5.75 | 29.0 | render/thread/ms 45.59, render/busy/ms 44.46, pipelines/compile_ms 36.13, pipelines/front_ms 34.48, main/busy/ms 32.44, stage/render/prepare_assets 23.38, stage/main/update 15.39, stage/main/post_update 9.80, stage/render/render 9.09, encode//sum 8.10, render/graph/ms 7.64, stage/render/graph_submit 4.99 |
| warm-up | 5.87 | 33.6 | pipelines/compile_ms 175, pipelines/front_ms 171, render/thread/ms 45.29, render/busy/ms 43.62, main/busy/ms 40.36, stage/main/update 24.26, stage/render/render 16.00, stage/render/prepare_assets 15.20, render/graph/ms 13.64, stage/main/post_update 10.71, stage/render/graph_submit 9.82, encode//sum 8.70 |
| warm-up | 5.94 | 30.9 | pipelines/compile_ms 154, pipelines/front_ms 148, main/busy/ms 52.83, render/thread/ms 41.52, render/busy/ms 39.75, stage/main/update 28.98, stage/render/render 17.96, stage/main/post_update 15.69, render/graph/ms 15.49, encode//sum 10.59, stage/render/graph_submit 10.56, encode/submit 7.18 |
| warm-up | 5.99 | 48.6 | pipelines/compile_ms 129, pipelines/front_ms 119, render/thread/ms 63.83, render/busy/ms 61.91, main/busy/ms 50.24, stage/render/prepare_assets 25.65, stage/main/update 24.46, stage/render/render 17.92, stage/main/post_update 16.55, render/graph/ms 15.31, stage/render/graph_submit 10.36, encode//sum 9.97 |
| warm-up | 6.02 | 31.9 | pipelines/compile_ms 297, pipelines/front_ms 282, render/thread/ms 67.68, render/busy/ms 66.26, stage/render/prepare_assets 39.55, main/busy/ms 31.45, pipelines/driver_ms 14.99, stage/render/render 12.92, stage/main/update 12.27, stage/main/post_update 12.02, render/graph/ms 10.99, encode//sum 8.50 |
| warm-up | 6.08 | 28.1 | pipelines/compile_ms 165, pipelines/front_ms 160, render/thread/ms 41.80, render/busy/ms 40.48, main/busy/ms 40.20, stage/main/update 21.71, stage/render/render 15.98, render/graph/ms 13.70, stage/main/post_update 12.53, pipelines/wait_ms 10.53, encode//sum 9.57, stage/render/graph_submit 9.29 |
| warm-up | 6.14 | 27.0 | render/thread/ms 36.52, render/busy/ms 34.76, main/busy/ms 30.90, stage/render/render 16.69, stage/main/update 14.50, render/graph/ms 14.42, stage/main/post_update 11.30, encode//sum 11.29, stage/render/graph_submit 9.91, encode/submit 6.66, stage/render/camera_driver 4.30, stage/render/prepare_assets 4.20 |
| warm-up | 6.58 | 26.8 | main/busy/ms 35.83, render/thread/ms 35.51, render/busy/ms 34.38, stage/main/update 14.86, stage/render/render 12.38, stage/main/post_update 11.70, pipelines/compile_ms 11.06, render/graph/ms 10.60, pipelines/front_ms 10.32, encode//sum 8.57, stage/render/prepare_meshes 6.97, stage/render/prepare_assets 6.71 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=off-1 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 3% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **97%** (min 93%) · 2625 MHz (min 2625) · memory 8101 MHz · 88.1 W · 72 °C (max 75) · VRAM 5711 MiB · throttled 100% of samples

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
