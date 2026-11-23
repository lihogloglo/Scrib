# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1424 frames over 15.0 s · **median 10.46 ms (96 fps)** · mean 10.53 · p95 12.12 · p99 13.09 (1% low 76 fps) · worst 15.12 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 309 frames over 8.9 s, worst 2467.1 ms
- hitches (> 26.2 ms): **0** measured, 47 in warm-up
- timed GPU passes: 8.38 ms of a 10.53 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.46 ms, limit 11 ms
- pass worst frame 15.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.981 | 2.064 | 2.087 | 0.96 | 0.178 |
| main_opaque_pass_3d | 1.973 | 2.055 | 2.102 | 0.96 | 0.065 |
| early prepass | 0.721 | 0.751 | 0.767 | 0.96 | 0.093 |
| sun_visibility | 0.665 | 0.693 | 0.706 | 0.96 | 0.006 |
| atmosphere_luts | 0.343 | 0.357 | 0.370 | 0.96 | 0.003 |
| water_velocity | 0.317 | 0.330 | 0.333 | 0.96 | 0.002 |
| bloom | 0.308 | 0.321 | 0.329 | 0.96 | 0.064 |
| atmos_froxel_inject | 0.295 | 0.308 | 0.309 | 0.96 | 0.001 |
| water_sspr | 0.196 | 0.204 | 0.202 | 0.96 | 0.002 |
| ssao | 0.181 | 0.188 | 0.194 | 0.96 | 0.008 |
| uniform_allocation | 0.132 | 0.138 | 0.184 | 0.96 | 0.023 |
| water_fft | 0.129 | 0.134 | 0.135 | 0.96 | 0.003 |
| upscaling | 0.118 | 0.123 | 0.127 | 0.96 | 0.004 |
| clouds_composite | 0.104 | 0.109 | 0.111 | 0.96 | 0.002 |
| bin_unpacking | 0.099 | 0.103 | 0.134 | 0.96 | 0.016 |
| gpu_instance_materialization | 0.082 | 0.086 | 0.092 | 0.96 | 0.004 |
| tonemapping | 0.082 | 0.085 | 0.090 | 0.96 | 0.004 |
| main_transmissive_pass_3d | 0.076 | 0.079 | 0.091 | 0.96 | 0.008 |
| clustering | 0.067 | 0.070 | 0.081 | 0.96 | 0.020 |
| render_sky | 0.061 | 0.063 | 0.066 | 0.96 | 0.002 |
| early_mesh_preprocessing | 0.059 | 0.062 | 0.093 | 0.96 | 0.004 |
| clouds_trace | 0.055 | 0.058 | 0.069 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.049 | 0.414 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.046 | 0.048 | 0.050 | 0.96 | 0.001 |
| point_shadow_atlas | 0.035 | 0.072 | 0.078 | 0.48 | 0.131 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.031 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.218 | 0.227 | 0.12 | 0.004 |
| water_snapshot | 0.022 | 0.023 | 0.025 | 0.96 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.96 | 0.002 |
| range_unpacking | 0.021 | 0.022 | 0.028 | 0.96 | 0.003 |
| clouds_shadow | 0.020 | 0.021 | 0.024 | 0.96 | 0.003 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.039 | 0.96 | 0.005 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.027 | 0.96 | 0.235 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.96 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.012 | 0.96 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.015 | 0.017 | 0.48 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.96 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.48 | 0.001 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.48 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.96 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.224 | 10.774 | 13.00 | 13134.4 |
| render/busy/ms | 8.357 | 9.916 | 11.81 | 11900.3 |
| main/busy/ms | 8.085 | 9.089 | 11.58 | 11512.8 |
| stage/render/render | 5.272 | 6.782 | 9.88 | 7507.9 |
| render/graph/ms | 4.644 | 5.825 | 8.30 | 6613.6 |
| stage/main/post_update | 4.574 | 5.432 | 6.44 | 6513.8 |
| encode//sum | 4.153 | 6.143 | 13.11 | 5914.3 |
| stage/render/graph_submit | 3.336 | 4.466 | 7.14 | 4749.8 |
| encode/submit | 2.070 | 2.952 | 6.61 | 2948.1 |
| stage/main/update | 1.800 | 2.149 | 2.50 | 2563.0 |
| encode/finish_wall | 1.315 | 1.885 | 4.94 | 1872.8 |
| stage/render/camera_driver | 1.245 | 1.436 | 1.81 | 1772.9 |
| encode/pass:shadow_directional_light/sum | 0.867 | 1.502 | 4.01 | 1234.8 |
| encode/pass:early prepass/sum | 0.832 | 1.144 | 3.41 | 1184.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.806 | 1.153 | 2.15 | 1147.1 |
| stage/render/prepare_assets | 0.573 | 0.733 | 1.63 | 815.6 |
| pipelines/compile_ms | 0.541 | 0.000 | 124.19 | 771.0 |
| bigspace/total/ms | 0.538 | 1.231 | 2.46 | 765.6 |
| stage/render/prepare_views | 0.536 | 1.972 | 3.07 | 763.3 |
| pipelines/front_ms | 0.527 | 0.000 | 121.32 | 750.4 |
| stage/render/prepare_meshes | 0.520 | 0.729 | 1.16 | 740.4 |
| stage/render/prepare_write_phase_buffers | 0.512 | 0.749 | 1.02 | 729.5 |
| render/acquire/ms | 0.491 | 1.926 | 3.04 | 699.5 |
| stage/main/fixed_main_loop | 0.480 | 0.825 | 1.41 | 683.5 |
| stage/render/prepare_resources | 0.425 | 0.501 | 0.70 | 605.2 |
| stage/main/pre_update | 0.412 | 0.601 | 1.50 | 586.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.395 | 0.612 | 0.95 | 562.9 |
| render/present/ms | 0.375 | 0.479 | 1.88 | 534.6 |
| stage/main/first | 0.362 | 0.447 | 2.12 | 516.1 |
| controller/feed/ms | 0.329 | 0.417 | 0.67 | 468.1 |
| stage/render/prepare_batch_phases | 0.319 | 0.450 | 0.66 | 454.4 |
| stage/render/prepare_flush | 0.314 | 0.539 | 0.82 | 446.7 |
| stage/render/prepare_bind_groups | 0.274 | 0.414 | 0.92 | 390.7 |
| stage/main/last | 0.272 | 0.327 | 3.76 | 387.4 |
| bigspace/low_precision/ms | 0.246 | 0.702 | 1.49 | 349.7 |
| stage/render/create_views | 0.194 | 0.247 | 0.71 | 276.0 |
| stream/drive/ms | 0.187 | 0.259 | 0.59 | 266.4 |
| stage/render/extract_commands | 0.178 | 0.208 | 0.43 | 252.8 |
| anim/evaluate_cpu/ms | 0.171 | 0.202 | 0.39 | 243.2 |
| stage/render/specialize | 0.165 | 0.230 | 0.66 | 234.6 |
| stream/tick/ms | 0.156 | 0.223 | 0.56 | 222.5 |
| anim/evaluate/ms | 0.154 | 0.300 | 1.17 | 220.0 |
| bigspace/high_precision/ms | 0.137 | 0.378 | 1.94 | 195.7 |
| stage/render/queue | 0.133 | 0.173 | 0.24 | 189.5 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.112 | 0.311 | 1.98 | 159.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.110 | 0.182 | 0.56 | 156.3 |
| controller/move/ms | 0.067 | 0.141 | 0.64 | 94.9 |
| controller/write/ms | 0.064 | 0.082 | 0.52 | 90.9 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.20 | 66.5 |
| encode/resolve_diagnostics/sum | 0.044 | 0.079 | 0.18 | 63.2 |
| physics/step/ms | 0.038 | 0.072 | 0.46 | 54.3 |
| terrain/update/ms | 0.035 | 0.048 | 0.11 | 49.9 |
| stage/render/phase_sort | 0.034 | 0.051 | 0.27 | 48.6 |
| stage/render/cleanup | 0.024 | 0.032 | 0.43 | 34.4 |
| encode/camera_driver_span_begin/sum | 0.018 | 0.035 | 0.10 | 26.2 |
| encode/camera_driver_span_end/sum | 0.017 | 0.034 | 0.28 | 23.5 |
| weather/map/ms | 0.016 | 0.021 | 0.46 | 22.4 |
| pipelines/driver_ms | 0.014 | 0.000 | 3.53 | 20.6 |
| anim/apply/ms | 0.010 | 0.013 | 0.12 | 14.3 |
| controller/avoid/ms | 0.010 | 0.018 | 0.47 | 14.2 |
| weather/eval/ms | 0.009 | 0.012 | 0.03 | 13.2 |
| script/update/ms | 0.008 | 0.012 | 0.35 | 11.2 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 7.3 |
| script/ui/ms | 0.005 | 0.007 | 0.27 | 7.2 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 6.0 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.02 | 2.9 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.03 | 2.0 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 1.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
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
| crowd/joints_extracted/count | count | 763 | 1168 | 928 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.7 k | 43.7 k | 43.7 k |
| encode//max | ms | 0.51 | 4.15 | 0.57 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.10 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.02 |
| encode/finish_critical | ms | 0.98 | 4.15 | 0.80 |
| encode/pass:early prepass/max | ms | 0.83 | 3.41 | 0.69 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 2.15 | 0.71 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.56 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.37 | 0.95 | 0.33 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.21 | 1.98 | 0.32 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 4.01 | 0.80 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.18 | 0.06 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 9.37 | 11.81 | 9.44 |
| gpu/timed/ms | ms | 9.36 | 10.59 | 9.44 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2431 | 2431 | 2431 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 301.1 k | 301.1 k | 301.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.6 k | 12.6 k | 12.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 343.66 M | 343.66 M | 343.66 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 354 | 355 | 354 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 559 | 559 | 559 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 559 | 560 | 559 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 354 | 355 | 354 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 559 | 559 | 559 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 559 | 560 | 559 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 1361 | 1376 | 1361 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 6753 | 6753 | 6753 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2843 | 2858 | 2843 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 20 | 20 | 20 |
| phases/opaque3d_prepass/bins | count | 1361 | 1376 | 1361 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 6753 | 6753 | 6753 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2843 | 2858 | 2843 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 47 | 135 | 85 |
| phases/shadow/bins | count | 2165 | 2216 | 2162 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 7312 | 21.9 k | 7312 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 5345 | 5399 | 5341 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 47 | 57 | 46 |
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
| precache/throttled | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 9 | 181 | 9 |
| preprocess/bind_groups_created | count | 4 | 174 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 25.46 | 42.58 | 16.83 |
| process/mem_usage | GiB | 3.02 | 3.17 | 3.17 |
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
| shadow_atlas/live_occ_pct | % | 0.59 | 0.88 | 0.59 |
| shadow_atlas/live_tiles | count | 6 | 9 | 6 |
| shadow_atlas/mover_draws | count | 1 | 4 | 2 |
| shadow_atlas/movers | count | 85 | 100 | 85 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 1 | 4 | 4 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 1 | 4 | 4 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 6 | 9 | 6 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 6 | 9 | 6 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 19 | 27 | 21 |
| skin/blocks_skipped/count | count | 191 | 198 | 189 |
| skin/blocks_walked/count | count | 19 | 27 | 21 |
| skin/joint_bytes_written/bytes | bytes | 59.4 k | 80.6 k | 68.4 k |
| skin/joints_written/count | count | 928 | 1260 | 1068 |
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
| system/cpu_usage | % | 33.11 | 44.02 | 33.03 |
| system/mem_usage | % | 50.90 | 51.39 | 51.39 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2950 | 2950 | 2950 |
| textures/images | count | 2500 | 2500 | 2500 |
| views/all/batch_sets | count | 108 | 196 | 146 |
| views/all/bins | count | 5595 | 5646 | 5592 |
| views/all/count | count | 10 | 13 | 10 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 21.9 k | 36.6 k | 21.9 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 12.1 k | 12.2 k | 12.1 k |
| views/all/pipelines | count | 96 | 106 | 95 |
| views/all/pipelines_distinct | count | 68 | 72 | 68 |
| views/all/work_items | count | 154.9 k | 258.2 k | 154.9 k |
| views/cascade/batch_sets | count | 9 | 45 | 9 |
| views/cascade/bins | count | 2028 | 2028 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 7312 | 21.9 k | 7312 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 5207 | 5208 | 5207 |
| views/cascade/pipelines | count | 37 | 37 | 37 |
| views/cascade/work_items | count | 49.3 k | 152.7 k | 49.3 k |
| views/lamp_face/batch_sets | count | 29 | 91 | 76 |
| views/lamp_face/bins | count | 137 | 188 | 134 |
| views/lamp_face/count | count | 6 | 9 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 137 | 192 | 134 |
| views/lamp_face/pipelines | count | 10 | 20 | 9 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 29 | 91 | 76 |
| views/main/batch_sets | count | 61 | 61 | 61 |
| views/main/bins | count | 3430 | 3462 | 3430 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 14.6 k | 14.6 k | 14.6 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6804 | 6836 | 6804 |
| views/main/pipelines | count | 49 | 49 | 49 |
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
| anim/bones_written/count | count | 42.26 | 78 | 60.2 k |
| anim/events/count | count | 0.05 | 2 | 72 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.53 | 8 | 3600 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 30 | 40.2 k |
| encode/bind_walk/init_textures | count | 9082 | 12.8 k | 12.93 M |
| encode/bind_walk/init_textures_skipped | count | 50.0 k | 59.1 k | 71.21 M |
| encode/bind_walk/inits | count | 653 | 743 | 930.0 k |
| encode/bind_walk/inits_skipped | count | 168 | 189 | 238.6 k |
| encode/bind_walk/merge_textures | count | 9020 | 12.7 k | 12.84 M |
| encode/bind_walk/merge_textures_skipped | count | 8730 | 17.8 k | 12.43 M |
| encode/bind_walk/merges | count | 115 | 132 | 163.9 k |
| encode/bind_walk/merges_skipped | count | 155 | 184 | 220.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1424 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1424 |
| encode/pass:early prepass/count | count | 1 | 1 | 1424 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1424 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1424 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1424 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 712 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1424 |
| encode/queue/command_buffers | count | 38.75 | 41 | 55.2 k |
| encode/queue/submits | count | 2 | 2 | 2848 |
| encode/queue/write_buffer/bytes | bytes | 2.73 M | 3.48 M | 3.89 G |
| encode/queue/write_buffer/calls | count | 262 | 335 | 373.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 67.3 k | 67.3 k | 95.88 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 12.8 k |
| encode/queue/write_texture/bytes | bytes | 18.5 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1424 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.63 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.01 | 1 | 11 |
| pipelines/created | count | 0.01 | 1 | 11 |
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
| warm-up | 2.47 | 2467.1 | main/busy/ms 2435, stage/main/post_update 2387, stage/main/update 35.28, stage/main/pre_update 9.23, stage/main/last 1.30, surface/advance_cpu_ms 0.94, stream/drive/ms 0.82, stage/main/first 0.49, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.30, stream/tick/ms 0.21, bigspace/total/ms 0.09 |
| warm-up | 3.19 | 720.6 | main/busy/ms 2482, stage/main/post_update 2420, render/thread/ms 659, render/busy/ms 522, stage/render/prepare_assets 469, stage/render/prepare_views 134, render/acquire/ms 134, stage/main/update 41.29, stage/render/create_views 32.73, pipelines/compile_ms 19.85, pipelines/front_ms 18.27, stage/render/render 12.02 |
| warm-up | 3.26 | 74.6 | render/thread/ms 731, render/busy/ms 539, stage/render/prepare_assets 472, stage/render/prepare_views 135, render/acquire/ms 135, pipelines/compile_ms 125, main/busy/ms 118, pipelines/front_ms 84.63, stage/render/render 74.33, render/present/ms 57.08, stage/main/post_update 55.47, stage/main/last 40.95 |
| warm-up | 3.31 | 50.2 | pipelines/compile_ms 237, pipelines/front_ms 184, render/thread/ms 120, main/busy/ms 91.81, stage/render/render 80.55, render/busy/ms 63.56, render/present/ms 55.63, pipelines/driver_ms 52.40, stage/main/last 41.22, stage/main/post_update 35.69, terrain/update/ms 32.37, pipelines/wait_ms 29.00 |
| warm-up | 3.42 | 111.7 | pipelines/wait_ms 3424, pipelines/compile_ms 1068, pipelines/front_ms 995, render/thread/ms 157, render/busy/ms 153, pipelines/driver_ms 73.63, main/busy/ms 46.19, stage/render/prepare_bind_groups 41.37, stage/render/prepare_resources 39.98, stage/render/prepare_assets 35.34, stage/render/render 32.56, render/graph/ms 27.40 |
| warm-up | 3.50 | 79.5 | pipelines/wait_ms 4833, pipelines/compile_ms 1686, pipelines/front_ms 1583, render/thread/ms 182, render/busy/ms 167, main/busy/ms 114, pipelines/driver_ms 103, stage/main/update 68.06, stage/render/prepare_resources 59.93, stage/render/prepare_bind_groups 41.46, stage/render/render 25.54, stage/main/post_update 25.30 |
| warm-up | 3.61 | 104.2 | pipelines/compile_ms 2047, pipelines/front_ms 1945, pipelines/wait_ms 1625, render/thread/ms 156, render/busy/ms 143, main/busy/ms 108, pipelines/driver_ms 102, stage/render/prepare_assets 74.28, stage/main/update 63.11, stage/render/render 26.46, stage/render/prepare_resources 23.62, stage/main/post_update 23.56 |
| warm-up | 3.65 | 42.9 | pipelines/compile_ms 1675, pipelines/front_ms 1603, pipelines/wait_ms 276, render/thread/ms 123, render/busy/ms 121, stage/render/prepare_assets 75.41, pipelines/driver_ms 72.12, main/busy/ms 60.76, stage/render/render 23.74, stage/main/update 22.43, render/graph/ms 21.58, encode//sum 18.26 |
| warm-up | 3.68 | 29.0 | pipelines/compile_ms 409, pipelines/front_ms 393, pipelines/wait_ms 87.89, render/thread/ms 65.13, render/busy/ms 63.68, main/busy/ms 62.50, stage/render/prepare_assets 25.12, stage/main/update 19.57, encode//sum 18.66, stage/main/post_update 15.58, pipelines/driver_ms 15.40, stage/main/pre_update 13.03 |
| warm-up | 3.73 | 46.8 | pipelines/compile_ms 1068, pipelines/front_ms 1001, pipelines/wait_ms 207, pipelines/driver_ms 67.64, render/thread/ms 58.83, render/busy/ms 57.58, main/busy/ms 49.44, stage/main/post_update 20.65, stage/render/prepare_assets 18.64, stage/render/prepare_resources 12.27, stage/main/update 11.42, stage/render/render 11.13 |
| warm-up | 3.77 | 40.4 | pipelines/compile_ms 1069, pipelines/front_ms 1002, pipelines/wait_ms 208, render/thread/ms 71.94, render/busy/ms 69.64, pipelines/driver_ms 67.06, main/busy/ms 58.21, stage/main/post_update 26.79, stage/render/render 24.41, render/graph/ms 21.50, stage/main/update 20.09, stage/render/graph_submit 17.08 |
| warm-up | 3.82 | 52.7 | pipelines/compile_ms 1027, pipelines/front_ms 972, render/thread/ms 87.91, render/busy/ms 84.68, pipelines/driver_ms 55.30, main/busy/ms 44.50, stage/render/prepare_assets 36.10, stage/render/render 31.83, render/graph/ms 28.37, pipelines/wait_ms 23.84, stage/render/graph_submit 23.53, stage/main/update 16.51 |
| warm-up | 3.86 | 40.0 | pipelines/compile_ms 1280, pipelines/front_ms 1203, render/thread/ms 87.03, render/busy/ms 84.06, pipelines/driver_ms 77.34, stage/render/prepare_assets 50.64, pipelines/wait_ms 50.02, main/busy/ms 31.29, stage/render/render 24.18, render/graph/ms 20.71, stage/render/graph_submit 15.66, stage/main/post_update 10.01 |
| warm-up | 3.93 | 73.4 | pipelines/compile_ms 772, pipelines/front_ms 733, render/thread/ms 109, render/busy/ms 107, stage/render/prepare_assets 61.24, pipelines/driver_ms 39.09, main/busy/ms 29.94, pipelines/wait_ms 27.40, stage/render/create_views 24.68, stage/render/render 15.66, render/graph/ms 13.08, stage/main/update 10.94 |
| warm-up | 4.17 | 53.0 | pipelines/compile_ms 1226, pipelines/front_ms 1183, main/busy/ms 91.76, render/thread/ms 68.41, render/busy/ms 64.13, stage/main/update 57.18, pipelines/driver_ms 42.68, stage/render/render 42.51, render/graph/ms 37.68, stage/main/post_update 26.84, stage/render/camera_driver 18.63, pipelines/wait_ms 14.48 |
| warm-up | 4.32 | 146.3 | pipelines/compile_ms 1016, pipelines/front_ms 976, main/busy/ms 186, render/thread/ms 171, render/busy/ms 166, stage/main/update 139, pipelines/wait_ms 138, stage/render/render 53.81, render/graph/ms 47.97, pipelines/driver_ms 39.68, stage/render/prepare_assets 39.25, stage/main/post_update 32.73 |
| warm-up | 4.42 | 99.6 | pipelines/compile_ms 2567, pipelines/front_ms 2484, pipelines/wait_ms 224, render/thread/ms 214, render/busy/ms 212, main/busy/ms 157, stage/render/prepare_assets 107, stage/main/update 106, pipelines/driver_ms 83.04, stage/render/render 31.16, render/graph/ms 28.06, stage/render/prepare_resources 27.63 |
| warm-up | 4.49 | 72.6 | pipelines/compile_ms 2142, pipelines/front_ms 2078, render/thread/ms 165, render/busy/ms 164, stage/render/prepare_assets 121, pipelines/wait_ms 118, main/busy/ms 74.27, pipelines/driver_ms 64.05, stage/main/update 32.74, stage/render/render 24.02, render/graph/ms 21.82, stage/render/graph_submit 18.81 |
| warm-up | 4.55 | 57.6 | pipelines/compile_ms 413, pipelines/front_ms 397, render/thread/ms 126, render/busy/ms 123, main/busy/ms 71.80, stage/render/prepare_assets 56.50, pipelines/wait_ms 45.88, stage/main/update 38.89, stage/render/render 22.98, render/graph/ms 20.97, stage/render/prepare_write_phase_buffers 19.19, stage/render/graph_submit 17.65 |
| warm-up | 4.61 | 61.1 | pipelines/compile_ms 1456, pipelines/front_ms 1412, render/thread/ms 115, render/busy/ms 112, main/busy/ms 81.34, pipelines/driver_ms 44.62, pipelines/wait_ms 39.90, stage/main/update 38.38, stage/render/prepare_write_phase_buffers 34.59, stage/render/prepare_assets 24.69, stage/render/render 23.24, render/graph/ms 21.06 |
| warm-up | 4.64 | 28.6 | pipelines/compile_ms 1410, pipelines/front_ms 1367, render/thread/ms 86.32, render/busy/ms 84.90, main/busy/ms 61.60, pipelines/driver_ms 42.91, stage/main/update 28.37, pipelines/wait_ms 25.06, stage/render/prepare_assets 23.55, stage/render/prepare_write_phase_buffers 18.09, stage/render/render 17.65, render/graph/ms 15.66 |
| warm-up | 4.66 | 28.4 | pipelines/compile_ms 265, pipelines/front_ms 256, render/thread/ms 53.78, render/busy/ms 52.40, main/busy/ms 49.47, stage/main/update 29.02, stage/render/render 15.59, render/graph/ms 13.88, stage/main/post_update 12.28, pipelines/wait_ms 11.88, stage/render/prepare_assets 10.13, stage/render/graph_submit 9.99 |
| warm-up | 4.70 | 32.8 | pipelines/compile_ms 852, pipelines/front_ms 829, render/thread/ms 57.99, render/busy/ms 56.55, main/busy/ms 41.39, pipelines/wait_ms 24.01, pipelines/driver_ms 22.61, stage/main/update 21.37, stage/render/prepare_assets 16.26, stage/render/render 15.66, render/graph/ms 13.63, stage/main/post_update 13.25 |
| warm-up | 4.74 | 29.9 | pipelines/compile_ms 361, pipelines/front_ms 348, main/busy/ms 49.74, render/thread/ms 31.01, render/busy/ms 29.19, stage/main/update 25.37, stage/render/render 17.77, render/graph/ms 15.38, stage/main/post_update 15.13, pipelines/driver_ms 12.69, stage/render/graph_submit 11.87, encode/submit 8.40 |
| warm-up | 4.77 | 27.9 | pipelines/compile_ms 376, pipelines/front_ms 363, main/busy/ms 37.26, render/thread/ms 35.20, render/busy/ms 33.54, stage/main/update 19.98, stage/render/render 14.62, pipelines/driver_ms 13.01, render/graph/ms 12.37, stage/render/prepare_assets 11.04, stage/main/post_update 10.05, stage/render/graph_submit 8.86 |
| warm-up | 4.83 | 27.3 | pipelines/compile_ms 329, pipelines/front_ms 318, render/thread/ms 40.29, render/busy/ms 38.86, main/busy/ms 30.59, stage/main/post_update 16.07, stage/render/render 12.84, stage/render/prepare_assets 11.47, pipelines/driver_ms 10.66, render/graph/ms 10.32, stage/main/update 9.50, encode//sum 8.45 |
| warm-up | 5.04 | 28.9 | pipelines/compile_ms 661, pipelines/front_ms 638, main/busy/ms 54.27, render/thread/ms 49.36, render/busy/ms 47.98, stage/main/update 23.91, pipelines/driver_ms 23.61, stage/render/render 18.52, stage/main/post_update 18.46, render/graph/ms 16.62, encode//sum 10.96, stage/render/graph_submit 10.75 |
| warm-up | 5.08 | 40.3 | pipelines/compile_ms 1726, pipelines/front_ms 1670, render/thread/ms 59.33, render/busy/ms 57.76, main/busy/ms 56.88, pipelines/driver_ms 56.41, stage/main/update 29.59, stage/render/render 19.27, stage/main/post_update 17.57, render/graph/ms 17.13, encode//sum 11.07, stage/render/graph_submit 10.93 |
| warm-up | 5.11 | 27.3 | pipelines/compile_ms 1540, pipelines/front_ms 1496, render/thread/ms 57.44, render/busy/ms 55.88, main/busy/ms 46.62, pipelines/driver_ms 43.70, stage/main/update 24.89, stage/render/prepare_assets 20.36, stage/render/render 13.24, stage/main/post_update 12.87, render/graph/ms 11.23, encode//sum 8.11 |
| warm-up | 5.15 | 40.4 | render/thread/ms 64.28, render/busy/ms 62.63, main/busy/ms 43.99, pipelines/compile_ms 42.48, pipelines/front_ms 40.92, stage/main/update 19.02, stage/main/post_update 15.41, stage/render/prepare_assets 14.84, stage/render/render 14.04, stage/render/prepare_resources 13.64, render/graph/ms 11.92, encode//sum 8.60 |
| warm-up | 5.21 | 45.3 | pipelines/compile_ms 630, pipelines/front_ms 614, render/thread/ms 62.03, render/busy/ms 60.58, main/busy/ms 45.26, stage/render/prepare_assets 29.31, stage/main/update 21.16, pipelines/driver_ms 16.47, stage/main/post_update 14.25, stage/render/render 13.15, render/graph/ms 11.13, encode//sum 8.27 |
| warm-up | 5.26 | 28.4 | render/thread/ms 48.78, render/busy/ms 47.36, main/busy/ms 33.36, stage/render/prepare_assets 19.93, stage/main/update 15.78, stage/render/render 12.10, stage/main/post_update 10.11, render/graph/ms 10.05, encode//sum 8.91, stage/render/graph_submit 6.94, encode/submit 4.85, stage/main/fixed_main_loop 4.22 |
| warm-up | 5.29 | 26.4 | pipelines/compile_ms 1065, pipelines/front_ms 1039, render/thread/ms 51.43, render/busy/ms 50.01, main/busy/ms 30.14, pipelines/driver_ms 25.67, stage/render/prepare_assets 17.86, stage/render/render 14.53, render/graph/ms 12.41, stage/main/update 11.04, stage/main/post_update 10.20, encode//sum 8.29 |
| warm-up | 5.39 | 39.9 | pipelines/compile_ms 762, pipelines/front_ms 745, render/thread/ms 56.96, render/busy/ms 55.50, main/busy/ms 40.27, stage/render/prepare_assets 25.75, stage/main/update 19.47, pipelines/driver_ms 17.19, stage/render/render 13.93, stage/main/post_update 11.65, render/graph/ms 11.53, encode//sum 10.13 |
| warm-up | 5.42 | 31.3 | pipelines/compile_ms 950, pipelines/front_ms 925, render/thread/ms 66.20, render/busy/ms 64.51, main/busy/ms 48.40, stage/render/prepare_assets 29.75, pipelines/driver_ms 24.87, stage/main/update 21.39, stage/render/render 17.22, render/graph/ms 14.51, stage/main/post_update 13.69, encode//sum 13.20 |
| warm-up | 5.46 | 37.1 | pipelines/compile_ms 373, pipelines/front_ms 359, render/thread/ms 64.02, render/busy/ms 62.46, main/busy/ms 42.98, stage/render/prepare_assets 30.29, stage/render/render 17.52, stage/main/update 16.70, render/graph/ms 15.23, encode//sum 14.77, stage/main/post_update 14.56, pipelines/driver_ms 13.66 |
| warm-up | 5.49 | 32.9 | render/thread/ms 66.13, render/busy/ms 64.36, main/busy/ms 39.91, stage/render/prepare_assets 32.16, stage/render/render 17.62, stage/main/post_update 17.30, render/graph/ms 14.98, stage/main/update 14.21, encode//sum 13.86, stage/render/graph_submit 10.65, encode/submit 7.15, stage/main/fixed_main_loop 4.17 |
| warm-up | 5.52 | 32.9 | render/thread/ms 60.56, render/busy/ms 59.18, main/busy/ms 40.90, stage/render/prepare_assets 35.97, stage/main/post_update 15.26, stage/main/update 14.62, stage/render/render 12.01, encode//sum 10.20, render/graph/ms 9.79, stage/render/graph_submit 6.57, stage/main/fixed_main_loop 6.38, encode/submit 3.81 |
| warm-up | 5.63 | 39.3 | pipelines/compile_ms 78.79, pipelines/front_ms 74.22, render/thread/ms 56.97, render/busy/ms 54.93, main/busy/ms 39.02, stage/main/update 20.45, stage/render/prepare_assets 17.58, stage/render/render 15.58, render/graph/ms 12.99, stage/main/post_update 12.47, encode//sum 9.48, stage/render/graph_submit 8.39 |
| warm-up | 5.65 | 26.4 | render/thread/ms 61.01, render/busy/ms 58.89, main/busy/ms 46.28, pipelines/compile_ms 34.86, pipelines/front_ms 32.86, stage/main/update 26.01, stage/render/prepare_assets 18.56, stage/render/render 17.49, render/graph/ms 14.73, stage/main/post_update 11.83, encode//sum 10.44, stage/render/graph_submit 9.63 |
| warm-up | 5.70 | 43.6 | render/thread/ms 62.19, render/busy/ms 60.82, main/busy/ms 54.56, stage/main/update 36.31, stage/render/prepare_assets 30.68, stage/render/render 12.42, render/graph/ms 10.45, stage/main/post_update 10.39, encode//sum 8.66, character/finish/ms 8.36, stage/render/graph_submit 7.22, stream/drive/ms 5.94 |
| warm-up | 5.77 | 27.7 | pipelines/compile_ms 365, pipelines/front_ms 357, render/thread/ms 45.11, render/busy/ms 43.65, main/busy/ms 35.89, stage/main/update 21.09, stage/render/render 14.37, stage/render/prepare_assets 13.00, render/graph/ms 12.19, encode//sum 9.44, stage/main/post_update 8.70, pipelines/driver_ms 8.67 |
| warm-up | 5.86 | 32.7 | pipelines/compile_ms 122, pipelines/front_ms 113, main/busy/ms 48.63, render/thread/ms 47.16, render/busy/ms 45.14, stage/render/render 21.56, stage/main/update 21.31, stage/main/post_update 19.22, render/graph/ms 18.86, stage/render/graph_submit 13.72, encode//sum 10.33, encode/submit 9.01 |
| warm-up | 5.90 | 39.8 | pipelines/compile_ms 96.10, pipelines/front_ms 88.60, render/thread/ms 65.83, render/busy/ms 63.78, main/busy/ms 59.02, stage/main/update 27.88, stage/render/render 22.15, stage/render/prepare_assets 20.21, render/graph/ms 19.21, stage/main/post_update 19.13, stage/render/graph_submit 13.59, encode//sum 10.84 |
| warm-up | 5.94 | 35.0 | pipelines/compile_ms 177, pipelines/front_ms 172, render/thread/ms 69.32, render/busy/ms 68.06, main/busy/ms 46.03, stage/render/prepare_assets 36.24, stage/main/update 23.18, stage/main/post_update 13.50, stage/render/render 12.72, render/graph/ms 10.65, encode//sum 9.22, stage/render/prepare_meshes 7.11 |
| warm-up | 6.40 | 27.2 | main/busy/ms 41.37, render/thread/ms 26.27, render/busy/ms 25.07, stage/main/update 14.24, stage/main/post_update 13.60, stage/render/render 12.05, render/graph/ms 10.31, encode//sum 9.99, stage/main/last 7.99, stage/render/graph_submit 7.00, stream/drive/ms 4.69, stage/render/prepare_meshes 4.05 |
| warm-up | 6.53 | 28.4 | main/busy/ms 42.34, render/thread/ms 34.58, render/busy/ms 33.55, stage/main/update 20.78, stage/render/render 13.21, stage/main/post_update 13.02, render/graph/ms 11.81, encode//sum 9.31, stage/render/graph_submit 6.75, stage/render/prepare_meshes 6.59, stage/render/prepare_assets 5.97, character/finish/ms 5.96 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=off-3 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 0%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **97%** (min 88%) · 2625 MHz (min 2625) · memory 8101 MHz · 87.3 W · 73 °C (max 75) · VRAM 5644 MiB · throttled 100% of samples

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
