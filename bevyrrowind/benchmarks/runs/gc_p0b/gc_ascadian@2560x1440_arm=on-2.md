# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1290 frames over 15.0 s · **median 11.73 ms (85 fps)** · mean 11.63 · p95 13.10 · p99 13.92 (1% low 72 fps) · worst 19.64 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 306 frames over 9.3 s, worst 2488.9 ms
- hitches (> 29.3 ms): **0** measured, 39 in warm-up
- timed GPU passes: 9.70 ms of a 11.63 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.73 ms, limit 11 ms
- pass worst frame 19.64 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.292 | 2.313 | 2.346 | 0.99 | 0.079 |
| dlss_super_resolution | 2.052 | 2.071 | 2.093 | 0.99 | 0.183 |
| early prepass | 1.325 | 1.337 | 1.350 | 0.99 | 0.113 |
| sun_visibility | 0.680 | 0.686 | 0.702 | 0.99 | 0.007 |
| atmosphere_luts | 0.359 | 0.362 | 0.374 | 0.99 | 0.003 |
| water_velocity | 0.327 | 0.331 | 0.333 | 0.99 | 0.002 |
| bloom | 0.322 | 0.325 | 0.340 | 0.99 | 0.064 |
| atmos_froxel_inject | 0.310 | 0.312 | 0.311 | 0.99 | 0.001 |
| gpu_instance_materialization | 0.225 | 0.227 | 0.246 | 0.99 | 0.007 |
| water_sspr | 0.200 | 0.202 | 0.205 | 0.99 | 0.002 |
| ssao | 0.188 | 0.190 | 0.197 | 0.99 | 0.007 |
| uniform_allocation | 0.140 | 0.141 | 0.184 | 0.99 | 0.023 |
| water_fft | 0.135 | 0.136 | 0.138 | 0.99 | 0.003 |
| upscaling | 0.126 | 0.128 | 0.133 | 0.99 | 0.004 |
| clouds_composite | 0.108 | 0.109 | 0.111 | 0.99 | 0.002 |
| bin_unpacking | 0.100 | 0.101 | 0.132 | 0.99 | 0.016 |
| early_mesh_preprocessing | 0.090 | 0.091 | 0.123 | 0.99 | 0.004 |
| tonemapping | 0.085 | 0.086 | 0.091 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.080 | 0.081 | 0.085 | 0.99 | 0.008 |
| clustering | 0.069 | 0.069 | 0.080 | 0.99 | 0.020 |
| render_sky | 0.063 | 0.063 | 0.065 | 0.99 | 0.002 |
| clouds_trace | 0.057 | 0.057 | 0.061 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.047 | 0.048 | 0.049 | 0.99 | 0.001 |
| point_shadow_atlas | 0.036 | 0.073 | 0.079 | 0.50 | 0.127 |
| range_unpacking | 0.036 | 0.036 | 0.047 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.227 | 0.12 | 0.004 |
| per_view_shadow_pass | 0.027 | 0.027 | 0.051 | 0.99 | 0.006 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.99 | 0.002 |
| clouds_shadow | 0.021 | 0.021 | 0.023 | 0.99 | 0.003 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.025 | 0.99 | 0.239 |
| clouds_fog_light | 0.011 | 0.011 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.014 | 0.99 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.015 | 0.016 | 0.50 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.005 | 0.99 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.004 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.436 | 11.928 | 18.37 | 13461.9 |
| render/busy/ms | 8.884 | 10.343 | 17.83 | 11460.9 |
| main/busy/ms | 8.394 | 9.528 | 10.95 | 10827.8 |
| stage/render/render | 5.584 | 6.856 | 13.99 | 7203.8 |
| render/graph/ms | 4.975 | 6.160 | 13.31 | 6417.7 |
| encode//sum | 4.924 | 6.800 | 15.30 | 6351.5 |
| stage/main/post_update | 4.836 | 5.850 | 7.29 | 6238.9 |
| stage/render/graph_submit | 3.613 | 4.685 | 11.72 | 4660.9 |
| encode/submit | 2.180 | 2.817 | 5.28 | 2811.9 |
| stage/main/update | 1.832 | 2.285 | 2.66 | 2363.8 |
| encode/finish_wall | 1.465 | 2.072 | 8.71 | 1890.1 |
| stage/render/camera_driver | 1.284 | 1.510 | 1.92 | 1656.2 |
| stage/render/prepare_views | 1.240 | 3.123 | 4.63 | 1599.5 |
| render/acquire/ms | 1.195 | 3.074 | 4.58 | 1541.2 |
| encode/pass:shadow_directional_light/sum | 0.946 | 1.629 | 8.40 | 1220.1 |
| encode/pass:early prepass/sum | 0.921 | 1.268 | 7.30 | 1188.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.900 | 1.265 | 7.53 | 1161.1 |
| stage/render/prepare_assets | 0.579 | 0.700 | 2.14 | 747.5 |
| bigspace/total/ms | 0.575 | 1.343 | 2.25 | 742.2 |
| stage/render/prepare_write_phase_buffers | 0.534 | 0.770 | 1.07 | 689.2 |
| stage/render/prepare_meshes | 0.516 | 0.772 | 1.23 | 665.6 |
| stage/main/fixed_main_loop | 0.511 | 0.817 | 1.40 | 659.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.433 | 0.621 | 2.17 | 558.0 |
| stage/render/prepare_resources | 0.428 | 0.494 | 0.67 | 552.2 |
| stage/main/pre_update | 0.389 | 0.472 | 0.88 | 501.8 |
| stage/main/first | 0.364 | 0.439 | 1.96 | 469.6 |
| render/present/ms | 0.356 | 0.434 | 1.73 | 459.8 |
| controller/feed/ms | 0.320 | 0.407 | 0.75 | 413.2 |
| stage/render/prepare_batch_phases | 0.316 | 0.444 | 0.58 | 407.0 |
| stage/render/prepare_flush | 0.312 | 0.450 | 0.96 | 402.6 |
| stage/main/last | 0.278 | 0.346 | 0.95 | 359.1 |
| stage/render/prepare_bind_groups | 0.274 | 0.436 | 0.74 | 353.9 |
| bigspace/low_precision/ms | 0.257 | 0.659 | 1.72 | 331.1 |
| stage/render/extract_commands | 0.195 | 0.227 | 0.61 | 251.7 |
| stage/render/create_views | 0.190 | 0.232 | 0.74 | 244.7 |
| stream/drive/ms | 0.180 | 0.252 | 0.69 | 232.0 |
| anim/evaluate_cpu/ms | 0.172 | 0.216 | 0.56 | 221.6 |
| anim/evaluate/ms | 0.164 | 0.336 | 1.72 | 211.2 |
| stage/render/specialize | 0.160 | 0.217 | 0.55 | 206.3 |
| bigspace/high_precision/ms | 0.154 | 0.480 | 1.91 | 198.0 |
| stream/tick/ms | 0.150 | 0.216 | 0.63 | 193.6 |
| stage/render/queue | 0.136 | 0.175 | 0.27 | 175.6 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.136 | 0.374 | 1.41 | 175.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.121 | 0.193 | 0.32 | 156.6 |
| controller/write/ms | 0.062 | 0.079 | 0.43 | 80.2 |
| controller/move/ms | 0.060 | 0.113 | 0.71 | 77.3 |
| encode/resolve_diagnostics/sum | 0.055 | 0.086 | 0.20 | 70.8 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.074 | 0.16 | 59.5 |
| physics/step/ms | 0.043 | 0.071 | 0.49 | 54.9 |
| stage/render/phase_sort | 0.041 | 0.061 | 0.13 | 53.3 |
| terrain/update/ms | 0.036 | 0.049 | 0.09 | 45.8 |
| stage/render/cleanup | 0.024 | 0.033 | 0.29 | 31.2 |
| encode/camera_driver_span_begin/sum | 0.023 | 0.038 | 0.27 | 29.4 |
| encode/camera_driver_span_end/sum | 0.020 | 0.035 | 0.11 | 26.0 |
| weather/map/ms | 0.015 | 0.020 | 0.08 | 19.5 |
| anim/apply/ms | 0.011 | 0.018 | 0.16 | 13.9 |
| controller/avoid/ms | 0.010 | 0.017 | 0.36 | 12.8 |
| weather/eval/ms | 0.009 | 0.011 | 0.05 | 11.2 |
| script/update/ms | 0.008 | 0.011 | 0.03 | 9.8 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 6.6 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 5.8 |
| far/select/ms | 0.004 | 0.005 | 0.09 | 5.3 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.02 | 2.5 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.9 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.9 |
| terrain/select/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
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
| anim/evaluated/count | count | 26.50 | 28 | 25 |
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
| crowd/joints_extracted/count | count | 759 | 1211 | 640 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.63 | 7.05 | 0.77 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.27 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.11 | 0.02 |
| encode/finish_critical | ms | 1.06 | 8.40 | 0.97 |
| encode/pass:early prepass/max | ms | 0.91 | 7.30 | 0.97 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.89 | 7.53 | 0.74 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.32 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.41 | 2.17 | 0.34 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.26 | 1.41 | 0.28 |
| encode/pass:shadow_directional_light/max | ms | 0.87 | 8.40 | 0.78 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.20 | 0.07 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 10.42 | 17.83 | 10.30 |
| gpu/timed/ms | ms | 10.41 | 11.48 | 10.30 |
| groundcover/batches/count | count | 10 | 10 | 10 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1371 | 1371 | 1371 |
| groundcover/slots/capacity | count | 94.7 k | 94.7 k | 94.7 k |
| groundcover/slots/capacity_bytes | bytes | 6.06 M | 6.06 M | 6.06 M |
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
| phases/alpha_mask3d/batch_sets | count | 15 | 15 | 15 |
| phases/alpha_mask3d/bins | count | 360 | 360 | 360 |
| phases/alpha_mask3d/instance_batches | count | 8 | 8 | 8 |
| phases/alpha_mask3d/instance_draws | count | 565 | 565 | 565 |
| phases/alpha_mask3d/instance_slots | count | 16.8 k | 16.8 k | 16.8 k |
| phases/alpha_mask3d/items | count | 565 | 565 | 565 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 15 | 15 | 15 |
| phases/alpha_mask3d_prepass/bins | count | 360 | 360 | 360 |
| phases/alpha_mask3d_prepass/instance_batches | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/instance_draws | count | 565 | 565 | 565 |
| phases/alpha_mask3d_prepass/instance_slots | count | 16.8 k | 16.8 k | 16.8 k |
| phases/alpha_mask3d_prepass/items | count | 565 | 565 | 565 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 27 | 27 |
| phases/opaque3d/bins | count | 1365 | 1365 | 1365 |
| phases/opaque3d/instance_batches | count | 6 | 6 | 6 |
| phases/opaque3d/instance_draws | count | 6757 | 6757 | 6757 |
| phases/opaque3d/instance_slots | count | 127.0 k | 127.0 k | 127.0 k |
| phases/opaque3d/items | count | 2847 | 2847 | 2847 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 24 | 24 |
| phases/opaque3d_prepass/bins | count | 1365 | 1365 | 1365 |
| phases/opaque3d_prepass/instance_batches | count | 6 | 6 | 6 |
| phases/opaque3d_prepass/instance_draws | count | 6757 | 6757 | 6757 |
| phases/opaque3d_prepass/instance_slots | count | 127.0 k | 127.0 k | 127.0 k |
| phases/opaque3d_prepass/items | count | 2847 | 2847 | 2847 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 12 | 127 | 12 |
| phases/shadow/bins | count | 2186 | 2188 | 2157 |
| phases/shadow/instance_batches | count | 7 | 21 | 7 |
| phases/shadow/instance_draws | count | 7315 | 21.9 k | 7315 |
| phases/shadow/instance_slots | count | 50.1 k | 150.3 k | 50.1 k |
| phases/shadow/items | count | 5365 | 5367 | 5336 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 51 | 54 | 50 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 348 | 348 | 348 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 5 | 149 | 5 |
| preprocess/bind_groups_created | count | 0 | 142 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 25.87 | 41.35 | 30.81 |
| process/mem_usage | GiB | 3.14 | 3.22 | 3.22 |
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
| shadow_atlas/live_occ_pct | % | 0.68 | 0.68 | 0.59 |
| shadow_atlas/live_tiles | count | 7 | 7 | 6 |
| shadow_atlas/mover_draws | count | 1 | 4 | 4 |
| shadow_atlas/movers | count | 85 | 100 | 85 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 4 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 0 | 4 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 7 | 7 | 6 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 7 | 7 | 6 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 19 | 28 | 15 |
| skin/blocks_skipped/count | count | 191 | 202 | 195 |
| skin/blocks_walked/count | count | 19 | 28 | 15 |
| skin/joint_bytes_written/bytes | bytes | 59.9 k | 80.6 k | 49.2 k |
| skin/joints_written/count | count | 936 | 1260 | 768 |
| skin/upload_writes/count | count | 8 | 12 | 10 |
| started_load_count |  | 5837 | 5838 | 5838 |
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
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
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
| system/cpu_usage | % | 32.07 | 44.32 | 33.58 |
| system/mem_usage | % | 51.28 | 51.56 | 51.56 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 93 | 208 | 93 |
| views/all/bins | count | 5636 | 5638 | 5607 |
| views/all/count | count | 11 | 11 | 10 |
| views/all/instance_batches | count | 35 | 49 | 35 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 22.0 k |
| views/all/instance_slots | count | 337.7 k | 438.0 k | 337.7 k |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 105 | 108 | 104 |
| views/all/pipelines_distinct | count | 74 | 78 | 74 |
| views/all/work_items | count | 345.1 k | 450.4 k | 345.1 k |
| views/cascade/batch_sets | count | 12 | 54 | 12 |
| views/cascade/bins | count | 2037 | 2037 | 2021 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 7 | 21 | 7 |
| views/cascade/instance_draws | count | 7315 | 21.9 k | 7315 |
| views/cascade/instance_slots | count | 50.1 k | 150.3 k | 50.1 k |
| views/cascade/items | count | 5216 | 5216 | 5200 |
| views/cascade/pipelines | count | 40 | 40 | 40 |
| views/cascade/work_items | count | 50.3 k | 155.5 k | 50.3 k |
| views/lamp_face/batch_sets | count | 0 | 91 | 0 |
| views/lamp_face/bins | count | 149 | 167 | 136 |
| views/lamp_face/count | count | 7 | 7 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 149 | 167 | 136 |
| views/lamp_face/pipelines | count | 11 | 14 | 10 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 91 | 0 |
| views/main/batch_sets | count | 81 | 81 | 81 |
| views/main/bins | count | 3450 | 3450 | 3450 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 28 | 28 | 28 |
| views/main/instance_draws | count | 14.6 k | 14.6 k | 14.6 k |
| views/main/instance_slots | count | 287.6 k | 287.6 k | 287.6 k |
| views/main/items | count | 6824 | 6824 | 6824 |
| views/main/pipelines | count | 54 | 54 | 54 |
| views/main/work_items | count | 294.8 k | 294.8 k | 294.8 k |
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
| anim/bones_written/count | count | 35.72 | 117 | 46.1 k |
| anim/events/count | count | 0.05 | 2 | 61 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.79 | 8 | 3600 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 30 | 36.4 k |
| encode/bind_walk/init_textures | count | 9234 | 13.0 k | 11.91 M |
| encode/bind_walk/init_textures_skipped | count | 60.4 k | 71.3 k | 77.88 M |
| encode/bind_walk/inits | count | 737 | 840 | 950.2 k |
| encode/bind_walk/inits_skipped | count | 173 | 192 | 223.2 k |
| encode/bind_walk/merge_textures | count | 9172 | 12.9 k | 11.83 M |
| encode/bind_walk/merge_textures_skipped | count | 17.8 k | 25.1 k | 23.02 M |
| encode/bind_walk/merges | count | 139 | 161 | 179.9 k |
| encode/bind_walk/merges_skipped | count | 159 | 183 | 205.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1290 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1290 |
| encode/pass:early prepass/count | count | 1 | 1 | 1290 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1290 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1290 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1290 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 644 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1290 |
| encode/queue/command_buffers | count | 38.75 | 41 | 50.0 k |
| encode/queue/submits | count | 2 | 2 | 2580 |
| encode/queue/write_buffer/bytes | bytes | 2.75 M | 3.50 M | 3.55 G |
| encode/queue/write_buffer/calls | count | 272 | 345 | 350.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 53.9 k | 53.9 k | 69.52 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.4 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1290 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
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

## Hitches (> 29.3 ms; slowest 39, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2488.9 | main/busy/ms 2456, stage/main/post_update 2408, stage/main/update 34.71, stage/main/pre_update 9.58, stage/main/last 1.06, surface/advance_cpu_ms 0.73, stage/main/first 0.59, stream/drive/ms 0.43, stage/main/fixed_main_loop 0.38, surface/cell_upload_ms 0.29, bigspace/total/ms 0.12, stream/tick/ms 0.10 |
| warm-up | 3.24 | 755.9 | main/busy/ms 2503, stage/main/post_update 2441, render/thread/ms 693, render/busy/ms 541, stage/render/prepare_assets 506, stage/render/prepare_views 149, render/acquire/ms 149, stage/main/update 41.25, stage/render/render 20.97, render/graph/ms 16.64, stage/render/camera_driver 14.55, stage/main/pre_update 11.86 |
| warm-up | 3.37 | 124.2 | render/thread/ms 815, render/busy/ms 579, stage/render/prepare_assets 516, stage/render/prepare_views 150, render/acquire/ms 149, stage/render/render 119, pipelines/compile_ms 96.46, render/present/ms 86.69, main/busy/ms 82.63, pipelines/driver_ms 71.86, stage/main/post_update 50.54, render/graph/ms 29.90 |
| warm-up | 3.43 | 63.1 | pipelines/compile_ms 589, pipelines/front_ms 361, pipelines/driver_ms 228, render/thread/ms 183, stage/render/render 123, pipelines/wait_ms 116, render/present/ms 92.10, render/busy/ms 90.16, main/busy/ms 57.10, stage/render/prepare_assets 35.85, stage/main/post_update 30.76, terrain/update/ms 30.64 |
| warm-up | 3.47 | 36.7 | pipelines/compile_ms 671, pipelines/wait_ms 522, pipelines/front_ms 492, pipelines/driver_ms 178, main/busy/ms 151, render/thread/ms 94.83, render/busy/ms 84.40, stage/main/update 64.65, stage/render/prepare_assets 47.78, stage/main/fixed_main_loop 35.47, stage/render/render 29.92, stage/main/post_update 25.54 |
| warm-up | 3.65 | 176.6 | pipelines/wait_ms 6593, pipelines/compile_ms 2541, pipelines/front_ms 2408, main/busy/ms 173, pipelines/driver_ms 133, render/thread/ms 110, render/busy/ms 108, stage/render/prepare_assets 85.84, stage/main/update 66.85, stage/main/pre_update 44.71, stage/main/fixed_main_loop 33.33, stage/main/post_update 25.08 |
| warm-up | 3.71 | 34.3 | pipelines/compile_ms 489, pipelines/front_ms 468, pipelines/wait_ms 225, render/thread/ms 57.63, render/busy/ms 56.28, main/busy/ms 42.56, stage/render/prepare_assets 28.15, stage/main/post_update 26.61, pipelines/driver_ms 21.83, stage/main/update 10.91, terrain/update/ms 10.39, stage/render/render 8.96 |
| warm-up | 3.75 | 44.0 | pipelines/compile_ms 541, pipelines/front_ms 523, pipelines/wait_ms 146, render/thread/ms 74.74, render/busy/ms 73.04, main/busy/ms 59.65, stage/main/post_update 41.62, encode//sum 27.66, stage/render/render 26.26, render/graph/ms 24.04, pipelines/driver_ms 17.77, stage/render/prepare_resources 17.31 |
| warm-up | 3.86 | 110.3 | pipelines/compile_ms 2040, pipelines/front_ms 1948, pipelines/wait_ms 371, render/thread/ms 148, render/busy/ms 145, main/busy/ms 123, pipelines/driver_ms 92.57, stage/main/post_update 90.62, stage/render/prepare_write_phase_buffers 65.83, bigspace/total/ms 58.93, stage/render/render 33.36, encode//sum 31.12 |
| warm-up | 4.00 | 142.4 | pipelines/compile_ms 3120, pipelines/front_ms 2907, pipelines/wait_ms 366, render/thread/ms 244, render/busy/ms 242, pipelines/driver_ms 213, stage/render/prepare_assets 125, main/busy/ms 119, stage/main/post_update 77.59, stage/render/prepare_write_phase_buffers 64.20, bigspace/total/ms 58.99, stage/render/render 30.59 |
| warm-up | 4.04 | 41.3 | pipelines/compile_ms 1455, pipelines/front_ms 1316, render/thread/ms 179, render/busy/ms 177, stage/render/prepare_assets 146, pipelines/driver_ms 139, pipelines/wait_ms 58.28, main/busy/ms 39.85, stage/render/render 24.63, render/graph/ms 22.39, stage/render/graph_submit 19.67, stage/main/update 14.62 |
| warm-up | 4.10 | 58.4 | pipelines/compile_ms 202, pipelines/front_ms 181, render/thread/ms 97.09, render/busy/ms 89.79, stage/render/prepare_assets 68.17, main/busy/ms 22.33, pipelines/driver_ms 20.63, stage/render/render 18.11, render/graph/ms 10.38, encode/submit 9.13, stage/main/post_update 8.40, pipelines/wait_ms 8.13 |
| warm-up | 4.22 | 32.0 | pipelines/compile_ms 1017, pipelines/front_ms 977, render/thread/ms 54.69, render/busy/ms 52.97, main/busy/ms 44.20, pipelines/driver_ms 40.22, stage/main/post_update 24.12, stage/main/update 15.41, stage/render/render 14.67, stage/render/prepare_assets 14.41, stage/render/prepare_flush 12.95, render/graph/ms 12.75 |
| warm-up | 4.35 | 84.2 | main/busy/ms 143, pipelines/compile_ms 116, pipelines/front_ms 112, render/thread/ms 78.96, render/busy/ms 77.07, stage/main/update 64.88, stage/main/post_update 46.16, stage/render/prepare_bind_groups 27.67, stage/render/render 15.86, stage/render/prepare_resources 14.83, stage/main/pre_update 14.41, render/graph/ms 13.34 |
| warm-up | 4.49 | 138.3 | pipelines/compile_ms 2433, pipelines/front_ms 2364, main/busy/ms 162, render/thread/ms 156, render/busy/ms 154, pipelines/driver_ms 68.30, stage/main/post_update 66.95, stage/main/update 65.25, stage/render/prepare_assets 47.91, stage/render/prepare_resources 34.07, stage/render/prepare_bind_groups 28.64, stage/render/render 25.76 |
| warm-up | 4.59 | 107.1 | pipelines/compile_ms 3593, pipelines/front_ms 3485, pipelines/wait_ms 293, render/thread/ms 188, render/busy/ms 185, main/busy/ms 140, pipelines/driver_ms 108, stage/main/post_update 61.21, stage/render/prepare_assets 53.12, stage/main/update 51.66, stage/render/prepare_resources 49.53, stage/render/render 28.74 |
| warm-up | 4.73 | 132.9 | pipelines/compile_ms 3585, pipelines/front_ms 3483, pipelines/wait_ms 777, render/thread/ms 222, render/busy/ms 220, stage/render/prepare_assets 128, main/busy/ms 107, pipelines/driver_ms 102, stage/main/update 50.72, stage/main/post_update 32.43, stage/render/prepare_resources 29.05, stage/render/prepare_write_phase_buffers 21.18 |
| warm-up | 4.80 | 75.3 | pipelines/compile_ms 2945, pipelines/front_ms 2870, pipelines/wait_ms 600, render/thread/ms 205, render/busy/ms 203, stage/render/prepare_assets 135, pipelines/driver_ms 75.11, main/busy/ms 54.25, stage/main/update 32.59, stage/render/prepare_resources 25.11, stage/render/render 24.36, render/graph/ms 22.05 |
| warm-up | 4.85 | 47.0 | pipelines/compile_ms 709, pipelines/front_ms 688, render/thread/ms 119, render/busy/ms 117, pipelines/wait_ms 93.97, main/busy/ms 60.82, stage/render/render 38.47, stage/main/update 35.79, render/graph/ms 35.06, stage/render/graph_submit 30.41, stage/render/prepare_assets 28.62, stage/render/prepare_resources 25.10 |
| warm-up | 4.89 | 38.6 | pipelines/compile_ms 206, pipelines/front_ms 196, render/thread/ms 80.88, render/busy/ms 78.12, main/busy/ms 59.49, stage/main/update 32.29, stage/render/render 30.11, render/graph/ms 26.91, stage/render/graph_submit 22.27, stage/render/prepare_assets 20.60, encode/submit 18.11, stage/main/post_update 17.61 |
| warm-up | 4.92 | 31.7 | pipelines/compile_ms 515, pipelines/front_ms 500, render/thread/ms 65.50, render/busy/ms 64.00, main/busy/ms 49.78, stage/main/update 25.42, stage/render/prepare_assets 20.94, stage/render/render 17.23, stage/main/post_update 15.61, render/graph/ms 15.32, pipelines/driver_ms 14.87, stage/render/graph_submit 10.90 |
| warm-up | 4.95 | 29.9 | pipelines/compile_ms 421, pipelines/front_ms 410, render/thread/ms 58.67, render/busy/ms 57.16, main/busy/ms 46.32, stage/main/update 22.46, stage/main/post_update 16.39, stage/render/render 15.79, stage/render/prepare_assets 15.11, render/graph/ms 13.98, pipelines/driver_ms 10.68, stage/render/graph_submit 9.50 |
| warm-up | 4.98 | 31.1 | render/thread/ms 58.17, render/busy/ms 56.65, main/busy/ms 50.19, stage/main/update 24.86, stage/main/post_update 16.14, stage/render/render 14.19, stage/render/prepare_resources 13.08, render/graph/ms 12.41, stage/render/prepare_assets 11.47, encode//sum 8.60, stage/render/graph_submit 8.57, encode/submit 6.08 |
| warm-up | 5.20 | 39.5 | pipelines/compile_ms 218, pipelines/front_ms 210, render/thread/ms 48.33, main/busy/ms 47.65, render/busy/ms 47.02, stage/main/update 21.07, pipelines/wait_ms 15.45, stage/render/render 14.99, stage/main/post_update 14.15, render/graph/ms 13.09, stage/render/prepare_resources 13.07, encode//sum 8.44 |
| warm-up | 5.31 | 36.0 | pipelines/compile_ms 1374, pipelines/front_ms 1335, render/thread/ms 55.89, render/busy/ms 54.41, main/busy/ms 52.23, pipelines/driver_ms 38.25, stage/main/update 30.58, stage/render/render 22.25, render/graph/ms 20.16, stage/main/post_update 14.92, stage/render/graph_submit 14.80, stream/drive/ms 12.55 |
| warm-up | 5.41 | 32.0 | pipelines/compile_ms 124, pipelines/front_ms 118, render/thread/ms 49.93, render/busy/ms 48.14, main/busy/ms 40.08, stage/render/prepare_assets 21.63, stage/render/render 16.98, stage/main/post_update 16.85, stage/main/update 15.03, render/graph/ms 14.24, stage/render/graph_submit 10.32, encode//sum 8.40 |
| warm-up | 5.64 | 31.3 | render/thread/ms 48.85, render/busy/ms 46.97, main/busy/ms 37.46, stage/render/render 19.47, render/graph/ms 16.70, stage/render/prepare_assets 15.49, stage/main/update 15.46, stage/main/post_update 14.19, encode//sum 12.83, stage/render/graph_submit 11.80, encode/submit 8.48, stage/render/camera_driver 4.62 |
| warm-up | 5.68 | 42.7 | render/thread/ms 68.44, render/busy/ms 66.68, main/busy/ms 49.26, stage/render/prepare_assets 30.24, stage/main/update 24.22, stage/render/render 18.90, render/graph/ms 16.06, stage/main/post_update 14.36, encode//sum 11.65, stage/render/graph_submit 10.91, encode/submit 6.85, stage/render/prepare_meshes 5.63 |
| warm-up | 5.71 | 31.3 | render/thread/ms 69.05, render/busy/ms 67.70, main/busy/ms 48.09, stage/render/prepare_assets 38.52, stage/main/update 20.73, stage/main/post_update 14.45, stage/render/render 14.04, render/graph/ms 11.93, encode//sum 9.97, stage/render/graph_submit 8.37, stage/main/fixed_main_loop 5.98, encode/submit 4.94 |
| warm-up | 5.75 | 41.1 | pipelines/compile_ms 375, pipelines/front_ms 365, render/thread/ms 68.91, render/busy/ms 67.50, stage/render/prepare_assets 43.46, main/busy/ms 40.74, stage/main/post_update 14.36, stage/main/update 14.32, stage/render/render 13.25, render/graph/ms 11.04, pipelines/driver_ms 10.13, encode//sum 10.05 |
| warm-up | 5.88 | 45.3 | pipelines/compile_ms 161, pipelines/front_ms 153, render/thread/ms 64.40, render/busy/ms 62.60, main/busy/ms 47.12, stage/render/prepare_assets 20.76, stage/main/update 20.70, stage/main/post_update 17.34, stage/render/render 15.29, render/graph/ms 12.76, encode//sum 11.33, stage/render/prepare_meshes 8.12 |
| warm-up | 5.92 | 36.8 | pipelines/compile_ms 132, pipelines/front_ms 126, render/thread/ms 77.91, render/busy/ms 75.95, main/busy/ms 58.70, stage/main/update 28.84, stage/render/render 20.50, stage/render/prepare_assets 19.58, stage/main/post_update 18.68, render/graph/ms 17.87, stage/render/graph_submit 12.30, encode//sum 11.52 |
| warm-up | 5.96 | 38.4 | pipelines/compile_ms 217, pipelines/front_ms 210, render/thread/ms 71.25, render/busy/ms 69.78, main/busy/ms 52.53, stage/main/update 30.85, stage/render/prepare_assets 29.25, stage/render/render 16.13, render/graph/ms 14.19, stage/main/post_update 12.99, stage/render/graph_submit 10.36, encode//sum 8.93 |
| warm-up | 5.99 | 29.7 | pipelines/compile_ms 186, pipelines/front_ms 180, render/thread/ms 64.70, render/busy/ms 63.34, main/busy/ms 46.08, stage/render/prepare_assets 27.22, stage/main/update 24.52, stage/main/post_update 13.59, stage/render/render 12.86, render/graph/ms 10.99, encode//sum 8.21, stage/render/graph_submit 7.60 |
| warm-up | 6.06 | 39.7 | pipelines/compile_ms 1051, pipelines/front_ms 1022, render/thread/ms 57.71, render/busy/ms 55.77, main/busy/ms 49.14, stage/main/update 31.70, pipelines/driver_ms 28.42, stage/render/render 21.55, render/graph/ms 18.80, stage/render/prepare_assets 17.22, stage/render/graph_submit 13.56, encode//sum 10.51 |
| warm-up | 6.17 | 43.6 | pipelines/compile_ms 341, pipelines/front_ms 323, render/thread/ms 64.40, render/busy/ms 62.54, main/busy/ms 59.23, stage/main/update 31.88, stage/render/render 21.61, render/graph/ms 19.13, stage/main/post_update 18.51, pipelines/driver_ms 18.02, stage/render/graph_submit 14.10, stage/render/prepare_assets 11.79 |
| warm-up | 6.21 | 42.4 | pipelines/compile_ms 344, pipelines/front_ms 325, render/thread/ms 80.77, render/busy/ms 78.91, main/busy/ms 68.10, stage/main/update 34.01, stage/render/render 23.56, stage/render/prepare_assets 23.38, render/graph/ms 21.10, stage/main/post_update 19.28, pipelines/driver_ms 18.58, stage/render/graph_submit 15.93 |
| warm-up | 6.29 | 74.9 | pipelines/compile_ms 1033, pipelines/front_ms 995, render/thread/ms 112, render/busy/ms 111, main/busy/ms 52.19, stage/render/render 48.48, render/graph/ms 46.59, stage/render/graph_submit 42.31, encode//sum 40.58, stage/render/prepare_assets 39.82, pipelines/driver_ms 37.64, encode/finish_wall 35.05 |
| warm-up | 6.33 | 43.9 | pipelines/compile_ms 1570, pipelines/front_ms 1519, render/thread/ms 115, render/busy/ms 113, main/busy/ms 58.87, stage/render/render 55.31, render/graph/ms 53.01, pipelines/driver_ms 51.15, stage/render/graph_submit 47.16, encode//sum 41.30, encode/finish_wall 34.39, encode/pass:main_transparent_pass_3d/sum 31.66 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=on-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 94%) · 2625 MHz (min 2625) · memory 8101 MHz · 89.0 W · 72 °C (max 75) · VRAM 6148 MiB · throttled 100% of samples

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
