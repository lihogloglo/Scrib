# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.render.reserve_worst_case=on (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 357 frames over 15.0 s · **median 38.05 ms (26 fps)** · mean 42.05 · p95 88.56 · p99 99.97 (1% low 10 fps) · worst 103.41 ms · 25 spikes over 1.5× median · 99.9 stutters/min (work 99.9/min)
- warm-up: 307 frames over 20.3 s, worst 2463.4 ms
- hitches (> 95.1 ms): **8** measured, 40 in warm-up
- timed GPU passes: 36.31 ms of a 42.05 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 38.05 ms, limit 11 ms
- **FAIL** worst frame 103.41 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 10.448 | 10.477 | 10.537 | 1.00 | 0.084 |
| early prepass | 9.812 | 9.840 | 10.028 | 1.00 | 0.133 |
| range_unpacking | 6.408 | 6.426 | 54.433 | 1.00 | 0.002 |
| early_mesh_preprocessing | 2.219 | 2.225 | 3.045 | 1.00 | 0.003 |
| dlss_super_resolution | 2.063 | 2.068 | 2.087 | 1.00 | 0.194 |
| sun_visibility | 0.695 | 0.697 | 0.706 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.641 | 0.643 | 0.644 | 1.00 | 0.008 |
| bin_unpacking | 0.616 | 0.618 | 0.669 | 1.00 | 0.015 |
| uniform_allocation | 0.381 | 0.382 | 0.498 | 1.00 | 0.022 |
| atmosphere_luts | 0.355 | 0.356 | 0.364 | 1.00 | 0.003 |
| water_velocity | 0.328 | 0.329 | 0.333 | 1.00 | 0.001 |
| bloom | 0.323 | 0.324 | 0.341 | 1.00 | 0.057 |
| atmos_froxel_inject | 0.304 | 0.304 | 0.308 | 1.00 | 0.001 |
| clustering | 0.282 | 0.282 | 0.306 | 1.00 | 0.020 |
| water_sspr | 0.199 | 0.200 | 0.201 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.199 | 1.00 | 0.007 |
| water_fft | 0.135 | 0.135 | 0.139 | 1.00 | 0.002 |
| upscaling | 0.132 | 0.133 | 0.136 | 1.00 | 0.004 |
| clouds_composite | 0.107 | 0.107 | 0.110 | 1.00 | 0.002 |
| tonemapping | 0.088 | 0.088 | 0.094 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.080 | 0.080 | 0.085 | 1.00 | 0.007 |
| render_sky | 0.063 | 0.063 | 0.063 | 1.00 | 0.002 |
| clouds_trace | 0.057 | 0.057 | 0.062 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.053 | 0.053 | 0.105 | 1.00 | 0.006 |
| lightprobe_irradiance_map | 0.051 | 0.408 | 0.410 | 0.13 | 0.001 |
| clouds_resolve | 0.046 | 0.046 | 0.047 | 1.00 | 0.001 |
| point_shadow_atlas | 0.037 | 0.073 | 0.080 | 0.50 | 0.140 |
| main_transparent_pass_3d | 0.030 | 0.030 | 0.032 | 1.00 | 0.222 |
| lightprobe_radiance_map | 0.028 | 0.218 | 0.228 | 0.13 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.028 | 1.00 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.024 | 1.00 | 0.001 |
| clouds_shadow | 0.022 | 0.022 | 0.024 | 1.00 | 0.003 |
| water_mips | 0.021 | 0.022 | 0.023 | 1.00 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.009 | 0.009 | 0.012 | 1.00 | 0.003 |
| lightprobe_copy | 0.009 | 0.070 | 0.071 | 0.13 | 0.004 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.017 | 0.019 | 0.50 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.006 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 1.00 | 0.001 |
| lightprobe_downsampling_first_pass | 0.004 | 0.032 | 0.034 | 0.13 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 40.777 | 87.256 | 101.93 | 14557.5 |
| stage/render/prepare_views | 30.853 | 78.246 | 93.19 | 11014.6 |
| render/acquire/ms | 30.784 | 78.173 | 93.13 | 10989.9 |
| render/busy/ms | 9.601 | 10.974 | 15.81 | 3427.6 |
| main/busy/ms | 8.002 | 9.426 | 11.54 | 2856.6 |
| encode//sum | 5.916 | 8.366 | 37.86 | 2112.1 |
| stage/render/render | 5.822 | 6.983 | 11.67 | 2078.4 |
| render/graph/ms | 5.157 | 6.306 | 10.90 | 1841.1 |
| stage/main/post_update | 3.805 | 4.222 | 5.78 | 1358.4 |
| stage/render/graph_submit | 3.795 | 4.922 | 9.46 | 1354.7 |
| encode/submit | 2.229 | 2.897 | 3.95 | 795.7 |
| encode/finish_wall | 1.585 | 2.112 | 6.19 | 565.7 |
| stage/main/update | 1.577 | 1.790 | 2.38 | 563.0 |
| stage/main/fixed_main_loop | 1.328 | 2.333 | 3.44 | 474.1 |
| stage/render/camera_driver | 1.269 | 1.451 | 1.64 | 453.0 |
| encode/pass:early prepass/sum | 1.084 | 1.507 | 1.79 | 387.1 |
| encode/pass:main_opaque_pass_3d/sum | 1.047 | 1.471 | 3.87 | 373.8 |
| encode/pass:shadow_directional_light/sum | 1.026 | 1.747 | 2.70 | 366.2 |
| stage/render/prepare_assets | 0.775 | 1.331 | 2.17 | 276.8 |
| stage/render/prepare_write_phase_buffers | 0.558 | 0.790 | 1.53 | 199.2 |
| stage/render/prepare_resources | 0.513 | 0.592 | 0.70 | 183.2 |
| stage/render/prepare_meshes | 0.509 | 0.613 | 0.81 | 181.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.498 | 0.762 | 1.12 | 177.8 |
| stage/main/first | 0.444 | 0.553 | 1.13 | 158.6 |
| bigspace/total/ms | 0.426 | 0.520 | 0.65 | 152.2 |
| stage/main/pre_update | 0.410 | 0.535 | 0.86 | 146.3 |
| render/present/ms | 0.392 | 0.476 | 0.61 | 140.0 |
| stage/render/prepare_batch_phases | 0.364 | 0.520 | 1.04 | 130.0 |
| stage/render/prepare_flush | 0.324 | 0.409 | 1.28 | 115.6 |
| controller/feed/ms | 0.311 | 0.378 | 0.48 | 111.1 |
| stage/render/prepare_bind_groups | 0.275 | 0.474 | 0.73 | 98.3 |
| stage/main/last | 0.259 | 0.304 | 0.45 | 92.3 |
| stage/render/queue | 0.240 | 0.293 | 0.37 | 85.7 |
| controller/move/ms | 0.231 | 0.490 | 1.08 | 82.6 |
| stage/render/extract_commands | 0.209 | 0.238 | 0.30 | 74.5 |
| bigspace/low_precision/ms | 0.188 | 0.235 | 0.30 | 67.1 |
| stage/render/create_views | 0.184 | 0.222 | 0.30 | 65.7 |
| stage/render/specialize | 0.180 | 0.224 | 0.34 | 64.3 |
| anim/evaluate_cpu/ms | 0.177 | 0.208 | 0.28 | 63.2 |
| stream/drive/ms | 0.170 | 0.195 | 0.41 | 60.7 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.167 | 0.421 | 3.90 | 59.7 |
| stream/tick/ms | 0.141 | 0.162 | 0.38 | 50.4 |
| anim/evaluate/ms | 0.137 | 0.165 | 0.20 | 48.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.134 | 0.204 | 0.37 | 47.9 |
| physics/step/ms | 0.099 | 0.125 | 0.18 | 35.3 |
| bigspace/high_precision/ms | 0.092 | 0.113 | 0.15 | 33.0 |
| encode/resolve_diagnostics/sum | 0.060 | 0.092 | 0.19 | 21.3 |
| controller/write/ms | 0.059 | 0.071 | 0.12 | 21.2 |
| stage/render/phase_sort | 0.052 | 0.077 | 0.19 | 18.7 |
| stage/render/prepare_collect_phase_buffers | 0.038 | 0.060 | 0.11 | 13.7 |
| terrain/update/ms | 0.035 | 0.045 | 0.07 | 12.4 |
| stage/render/cleanup | 0.030 | 0.039 | 0.54 | 10.6 |
| controller/avoid/ms | 0.030 | 0.052 | 0.07 | 10.6 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.040 | 0.09 | 8.6 |
| encode/camera_driver_span_end/sum | 0.021 | 0.034 | 0.08 | 7.7 |
| weather/map/ms | 0.015 | 0.020 | 0.04 | 5.3 |
| anim/apply/ms | 0.011 | 0.014 | 0.02 | 3.8 |
| weather/eval/ms | 0.009 | 0.012 | 0.04 | 3.1 |
| script/update/ms | 0.008 | 0.009 | 0.03 | 2.7 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 1.9 |
| far/select/ms | 0.004 | 0.005 | 0.01 | 1.3 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 1.2 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.02 | 0.7 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.00 | 0.5 |
| character/finish/ms | 0.001 | 0.002 | 0.00 | 0.4 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 0.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 0.4 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| terrain/select/ms | 0.001 | 0.001 | 0.02 | 0.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.2 |
| stage/render/prepare | 0.000 | 0.002 | 0.00 | 0.1 |
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
| anim/evaluated/count | count | 26 | 27 | 26 |
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
| crowd/joints_extracted/count | count | 765 | 1210 | 870 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.73 | 4.22 | 0.67 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.08 | 0.03 |
| encode/finish_critical | ms | 1.25 | 4.22 | 1.05 |
| encode/pass:early prepass/max | ms | 1.06 | 1.79 | 1.05 |
| encode/pass:main_opaque_pass_3d/max | ms | 1.03 | 3.87 | 0.94 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.13 | 0.37 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.48 | 1.12 | 0.52 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.28 | 3.90 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.96 | 2.70 | 0.72 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.19 | 0.07 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 33.30 | 95.96 | 32.34 |
| gpu/timed/ms | ms | 33.30 | 95.96 | 32.34 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1432 | 1432 | 1432 |
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
| phases/opaque3d/batch_sets | count | 32 | 32 | 32 |
| phases/opaque3d/bins | count | 1370 | 1370 | 1370 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2851 | 2851 | 2851 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 24 | 24 | 24 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 29 | 29 | 29 |
| phases/opaque3d_prepass/bins | count | 1370 | 1370 | 1370 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2851 | 2851 | 2851 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 136 | 14 |
| phases/shadow/bins | count | 2179 | 2195 | 2193 |
| phases/shadow/instance_batches | count | 9 | 27 | 9 |
| phases/shadow/instance_draws | count | 7317 | 22.0 k | 7317 |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| phases/shadow/items | count | 5351 | 5374 | 5372 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 53 | 56 | 56 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 358 | 358 | 358 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 5 | 68 | 5 |
| preprocess/bind_groups_created | count | 0 | 61 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 141 | 141 | 141 |
| process/cpu_usage | % | 13.33 | 23.35 | 12.50 |
| process/mem_usage | GiB | 3.84 | 3.85 | 3.85 |
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
| shadow_atlas/mover_draws | count | 2 | 4 | 4 |
| shadow_atlas/movers | count | 100 | 106 | 100 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 4 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 0 | 4 | 0 |
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
| skin/blocks_changed/count | count | 19 | 27 | 21 |
| skin/blocks_skipped/count | count | 191 | 199 | 188 |
| skin/blocks_walked/count | count | 19 | 27 | 22 |
| skin/joint_bytes_written/bytes | bytes | 61.7 k | 80.6 k | 57.3 k |
| skin/joints_written/count | count | 964 | 1260 | 896 |
| skin/upload_writes/count | count | 7 | 11 | 8 |
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
| system/cpu_usage | % | 18.44 | 24.52 | 18.55 |
| system/mem_usage | % | 53.65 | 53.69 | 53.68 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 113 | 235 | 113 |
| views/all/bins | count | 5647 | 5663 | 5661 |
| views/all/count | count | 10 | 11 | 11 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 22.0 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 113 | 116 | 116 |
| views/all/pipelines_distinct | count | 81 | 81 | 81 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.50 M |
| views/cascade/batch_sets | count | 14 | 60 | 14 |
| views/cascade/bins | count | 2027 | 2043 | 2027 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 9 |
| views/cascade/instance_draws | count | 7317 | 22.0 k | 7317 |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| views/cascade/items | count | 5206 | 5222 | 5206 |
| views/cascade/pipelines | count | 43 | 43 | 43 |
| views/cascade/work_items | count | 83.0 k | 253.8 k | 83.0 k |
| views/lamp_face/batch_sets | count | 0 | 91 | 0 |
| views/lamp_face/bins | count | 149 | 167 | 166 |
| views/lamp_face/count | count | 6 | 7 | 7 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 149 | 167 | 166 |
| views/lamp_face/pipelines | count | 10 | 13 | 13 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 91 | 0 |
| views/main/batch_sets | count | 99 | 99 | 99 |
| views/main/bins | count | 3468 | 3468 | 3468 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 14.7 k | 14.7 k | 14.7 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6840 | 6840 | 6840 |
| views/main/pipelines | count | 60 | 60 | 60 |
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
| anim/bones_written/count | count | 38.93 | 77 | 13.9 k |
| anim/events/count | count | 0.20 | 2 | 71 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 8 | 8 | 2856 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| encode//count | count | 28.25 | 30 | 10.1 k |
| encode/bind_walk/init_textures | count | 9346 | 13.1 k | 3.34 M |
| encode/bind_walk/init_textures_skipped | count | 67.7 k | 78.6 k | 24.15 M |
| encode/bind_walk/inits | count | 806 | 918 | 287.6 k |
| encode/bind_walk/inits_skipped | count | 177 | 195 | 63.3 k |
| encode/bind_walk/merge_textures | count | 9285 | 13.0 k | 3.31 M |
| encode/bind_walk/merge_textures_skipped | count | 19.1 k | 30.0 k | 6.80 M |
| encode/bind_walk/merges | count | 159 | 183 | 56.7 k |
| encode/bind_walk/merges_skipped | count | 161 | 187 | 57.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 357 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 357 |
| encode/pass:early prepass/count | count | 1 | 1 | 357 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 357 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 357 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 357 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 178 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 357 |
| encode/queue/command_buffers | count | 38.75 | 41 | 13.8 k |
| encode/queue/submits | count | 2 | 2 | 714 |
| encode/queue/write_buffer/bytes | bytes | 2.96 M | 3.51 M | 1.06 G |
| encode/queue/write_buffer/calls | count | 286 | 358 | 102.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 55.6 k | 55.6 k | 19.83 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 3213 |
| encode/queue/write_texture/bytes | bytes | 73.8 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.01 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 357 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 2.34 | 3 | 836 |
| physics/steps_dropped | count | 0.18 | 3 | 64 |
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

## Hitches (> 95.1 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2463.4 | main/busy/ms 2432, stage/main/post_update 2382, stage/main/update 36.31, stage/main/pre_update 9.98, surface/advance_cpu_ms 1.33, stage/main/last 1.12, stream/drive/ms 0.58, stage/main/first 0.51, stage/main/fixed_main_loop 0.41, surface/cell_upload_ms 0.29, stream/tick/ms 0.19, bigspace/total/ms 0.10 |
| warm-up | 3.26 | 795.1 | main/busy/ms 2480, stage/main/post_update 2417, render/thread/ms 736, render/busy/ms 580, stage/render/prepare_assets 544, stage/render/prepare_views 153, render/acquire/ms 153, stage/main/update 42.72, stage/render/render 20.83, render/graph/ms 17.22, stage/render/camera_driver 15.20, stage/main/pre_update 11.72 |
| warm-up | 3.36 | 105.7 | render/thread/ms 839, render/busy/ms 628, stage/render/prepare_assets 550, stage/render/prepare_views 154, render/acquire/ms 153, main/busy/ms 110, stage/render/render 109, stage/main/post_update 75.77, render/present/ms 58.09, render/graph/ms 49.00, terrain/update/ms 24.81, pipelines/compile_ms 24.80 |
| warm-up | 3.66 | 174.5 | pipelines/wait_ms 9394, pipelines/compile_ms 2531, pipelines/front_ms 2379, pipelines/driver_ms 152, main/busy/ms 137, render/thread/ms 135, render/busy/ms 133, stage/render/prepare_assets 87.12, stage/main/update 54.06, stage/main/post_update 48.25, stage/main/pre_update 29.10, terrain/update/ms 17.91 |
| warm-up | 4.60 | 185.4 | pipelines/compile_ms 3209, pipelines/front_ms 3126, render/thread/ms 266, render/busy/ms 262, pipelines/wait_ms 155, main/busy/ms 134, stage/main/update 94.33, stage/render/prepare_assets 85.14, pipelines/driver_ms 82.87, stage/render/prepare_resources 69.04, stage/render/prepare_write_phase_buffers 48.84, stage/main/post_update 30.01 |
| warm-up | 4.80 | 134.7 | pipelines/compile_ms 2956, pipelines/front_ms 2873, pipelines/wait_ms 1227, render/thread/ms 201, render/busy/ms 199, stage/render/prepare_assets 159, pipelines/driver_ms 83.03, main/busy/ms 52.87, stage/main/update 22.96, stage/render/render 21.16, stage/main/post_update 19.81, render/graph/ms 18.87 |
| warm-up | 7.75 | 151.8 | render/thread/ms 227, stage/render/prepare_views 131, render/acquire/ms 131, render/busy/ms 95.05, stage/render/render 84.27, render/graph/ms 82.72, stage/render/graph_submit 79.79, encode/submit 76.90, main/busy/ms 23.17, encode//sum 9.57, stage/main/post_update 7.66, stage/main/update 6.43 |
| warm-up | 9.12 | 142.1 | encode//sum 490, render/thread/ms 192, stage/render/prepare_views 97.60, render/acquire/ms 97.47, render/busy/ms 94.02, stage/render/render 82.12, render/graph/ms 80.65, stage/render/graph_submit 77.65, encode/finish_wall 72.20, encode/pass:main_opaque_pass_3d/sum 70.53, encode/pass:early prepass/sum 69.53, encode/pass:main_transmissive_pass_3d/sum 68.06 |
| warm-up | 11.37 | 99.1 | render/thread/ms 175, stage/render/prepare_views 153, render/acquire/ms 153, main/busy/ms 21.50, render/busy/ms 21.29, stage/render/render 12.66, encode//sum 12.41, render/graph/ms 11.20, stage/render/graph_submit 8.77, stage/main/post_update 8.07, encode/submit 5.48, stage/main/fixed_main_loop 5.35 |
| warm-up | 12.21 | 99.8 | render/thread/ms 176, stage/render/prepare_views 151, render/acquire/ms 151, render/busy/ms 24.17, main/busy/ms 19.89, encode//sum 15.28, stage/render/render 14.15, render/graph/ms 12.57, stage/render/graph_submit 9.65, stage/main/post_update 7.95, encode/submit 5.40, stage/main/update 4.51 |
| warm-up | 13.24 | 99.7 | render/thread/ms 179, stage/render/prepare_views 154, render/acquire/ms 154, render/busy/ms 23.97, main/busy/ms 22.04, encode//sum 14.13, stage/render/render 13.53, render/graph/ms 12.04, stage/render/graph_submit 9.04, stage/main/post_update 8.54, stage/main/fixed_main_loop 5.53, encode/submit 5.15 |
| warm-up | 13.74 | 95.5 | render/thread/ms 170, stage/render/prepare_views 147, render/acquire/ms 147, render/busy/ms 22.12, main/busy/ms 19.61, stage/render/render 12.62, encode//sum 12.30, render/graph/ms 11.01, stage/render/graph_submit 8.40, stage/main/post_update 7.33, encode/submit 5.31, stage/main/update 4.38 |
| warm-up | 15.62 | 110.6 | render/thread/ms 189, stage/render/prepare_views 161, render/acquire/ms 161, render/busy/ms 27.13, main/busy/ms 22.59, stage/render/render 15.91, encode//sum 15.59, render/graph/ms 14.23, stage/render/graph_submit 11.06, stage/main/post_update 9.01, encode/submit 6.46, stage/main/fixed_main_loop 5.43 |
| warm-up | 15.82 | 106.6 | render/thread/ms 197, stage/render/prepare_views 173, render/acquire/ms 173, main/busy/ms 24.39, render/busy/ms 23.69, encode//sum 12.13, stage/render/render 11.83, render/graph/ms 10.54, stage/main/post_update 8.09, stage/render/graph_submit 8.00, stage/main/fixed_main_loop 6.58, encode/submit 4.93 |
| warm-up | 15.92 | 96.8 | render/thread/ms 201, stage/render/prepare_views 178, render/acquire/ms 177, render/busy/ms 22.55, main/busy/ms 22.35, encode//sum 12.30, stage/render/render 12.30, render/graph/ms 10.98, stage/render/graph_submit 8.22, stage/main/post_update 7.64, stage/main/fixed_main_loop 6.69, encode/submit 5.07 |
| warm-up | 16.02 | 100.8 | render/thread/ms 195, stage/render/prepare_views 170, render/acquire/ms 170, render/busy/ms 23.61, main/busy/ms 22.11, stage/render/render 13.61, encode//sum 12.91, render/graph/ms 11.99, stage/render/graph_submit 9.00, stage/main/post_update 7.68, stage/main/fixed_main_loop 6.93, encode/submit 5.49 |
| warm-up | 16.12 | 104.0 | render/thread/ms 202, stage/render/prepare_views 178, render/acquire/ms 178, render/busy/ms 22.70, main/busy/ms 22.56, stage/render/render 12.94, encode//sum 12.57, render/graph/ms 11.37, stage/main/post_update 8.77, stage/render/graph_submit 8.68, stage/main/fixed_main_loop 6.66, encode/submit 5.23 |
| warm-up | 16.22 | 98.9 | render/thread/ms 200, stage/render/prepare_views 177, render/acquire/ms 177, render/busy/ms 22.25, main/busy/ms 22.15, encode//sum 12.28, stage/render/render 12.15, render/graph/ms 10.80, stage/main/post_update 8.58, stage/render/graph_submit 8.23, stage/main/fixed_main_loop 5.85, encode/submit 5.20 |
| warm-up | 16.33 | 104.6 | render/thread/ms 201, stage/render/prepare_views 178, render/acquire/ms 178, render/busy/ms 21.84, main/busy/ms 20.56, encode//sum 12.48, stage/render/render 12.20, render/graph/ms 10.79, stage/render/graph_submit 8.02, stage/main/post_update 7.46, stage/main/fixed_main_loop 5.62, encode/submit 4.94 |
| warm-up | 16.61 | 102.9 | render/thread/ms 191, stage/render/prepare_views 168, render/acquire/ms 168, render/busy/ms 22.44, main/busy/ms 22.12, encode//sum 12.56, stage/render/render 12.31, render/graph/ms 11.04, stage/render/graph_submit 8.43, stage/main/post_update 7.92, stage/main/fixed_main_loop 5.57, encode/submit 5.16 |
| warm-up | 16.71 | 96.2 | render/thread/ms 196, stage/render/prepare_views 173, render/acquire/ms 173, render/busy/ms 23.18, main/busy/ms 22.59, stage/render/render 12.87, encode//sum 12.61, render/graph/ms 11.56, stage/render/graph_submit 8.81, stage/main/post_update 8.02, stage/main/fixed_main_loop 5.96, encode/submit 5.25 |
| warm-up | 16.81 | 104.2 | render/thread/ms 197, stage/render/prepare_views 173, render/acquire/ms 173, render/busy/ms 24.00, main/busy/ms 21.65, stage/render/render 13.22, encode//sum 12.55, render/graph/ms 11.74, stage/render/graph_submit 8.84, stage/main/post_update 7.99, stage/main/fixed_main_loop 5.70, encode/submit 4.99 |
| warm-up | 17.01 | 102.4 | render/thread/ms 193, stage/render/prepare_views 168, render/acquire/ms 167, render/busy/ms 24.88, main/busy/ms 23.32, encode//sum 12.81, stage/render/render 12.57, render/graph/ms 11.21, stage/render/graph_submit 8.57, stage/main/post_update 8.54, stage/main/fixed_main_loop 5.33, stage/render/prepare_assets 5.28 |
| warm-up | 17.13 | 117.4 | render/thread/ms 217, stage/render/prepare_views 192, render/acquire/ms 192, main/busy/ms 24.55, render/busy/ms 23.97, stage/render/render 13.15, render/graph/ms 11.85, encode//sum 11.65, stage/main/post_update 10.65, stage/render/graph_submit 9.03, encode/submit 5.50, stage/main/fixed_main_loop 5.06 |
| warm-up | 17.22 | 96.5 | render/thread/ms 210, stage/render/prepare_views 181, render/acquire/ms 181, main/busy/ms 29.61, render/busy/ms 27.95, stage/render/render 13.14, encode//sum 13.00, render/graph/ms 11.66, stage/main/post_update 10.57, stage/render/graph_submit 8.71, stage/main/fixed_main_loop 7.00, stage/render/prepare_assets 6.77 |
| warm-up | 17.32 | 96.2 | render/thread/ms 188, stage/render/prepare_views 159, render/acquire/ms 159, render/busy/ms 28.23, main/busy/ms 27.11, encode//sum 14.79, stage/render/render 13.25, render/graph/ms 11.65, stage/render/graph_submit 8.71, stage/main/post_update 8.06, stage/render/prepare_assets 7.59, stage/main/fixed_main_loop 6.37 |
| warm-up | 17.44 | 117.1 | render/thread/ms 210, stage/render/prepare_views 187, render/acquire/ms 187, render/busy/ms 21.71, main/busy/ms 20.77, encode//sum 13.19, stage/render/render 12.33, render/graph/ms 11.08, stage/render/graph_submit 8.19, stage/main/post_update 8.04, encode/submit 4.84, stage/main/fixed_main_loop 4.55 |
| warm-up | 17.54 | 100.2 | render/thread/ms 215, stage/render/prepare_views 193, render/acquire/ms 192, render/busy/ms 21.74, main/busy/ms 21.24, encode//sum 14.09, stage/render/render 12.08, render/graph/ms 10.98, stage/main/post_update 8.43, stage/render/graph_submit 8.25, stage/main/fixed_main_loop 5.06, encode/submit 4.64 |
| warm-up | 17.63 | 96.2 | render/thread/ms 194, stage/render/prepare_views 170, render/acquire/ms 169, render/busy/ms 23.76, main/busy/ms 21.21, encode//sum 14.85, stage/render/render 13.10, render/graph/ms 11.79, stage/render/graph_submit 8.97, stage/main/post_update 7.98, stage/main/fixed_main_loop 5.04, encode/submit 4.98 |
| warm-up | 17.83 | 102.8 | render/thread/ms 192, stage/render/prepare_views 168, render/acquire/ms 168, render/busy/ms 22.52, main/busy/ms 21.80, encode//sum 12.60, stage/render/render 12.46, render/graph/ms 11.10, stage/main/post_update 8.62, stage/render/graph_submit 8.39, stage/main/fixed_main_loop 5.69, encode/submit 5.06 |
| warm-up | 17.93 | 100.1 | render/thread/ms 200, stage/render/prepare_views 177, render/acquire/ms 177, render/busy/ms 22.74, main/busy/ms 21.12, encode//sum 13.13, stage/render/render 13.08, render/graph/ms 11.72, stage/render/graph_submit 8.78, stage/main/post_update 8.17, encode/submit 5.53, stage/main/fixed_main_loop 5.40 |
| warm-up | 18.04 | 113.8 | render/thread/ms 211, stage/render/prepare_views 187, render/acquire/ms 187, render/busy/ms 23.38, main/busy/ms 21.25, encode//sum 13.72, stage/render/render 13.37, render/graph/ms 11.87, stage/render/graph_submit 8.79, stage/main/post_update 8.23, stage/main/fixed_main_loop 5.39, encode/submit 5.23 |
| warm-up | 18.23 | 95.8 | render/thread/ms 187, stage/render/prepare_views 163, render/acquire/ms 163, render/busy/ms 22.49, main/busy/ms 21.32, encode//sum 12.93, stage/render/render 12.46, render/graph/ms 11.06, stage/render/graph_submit 8.24, stage/main/post_update 8.20, encode/submit 5.18, stage/main/fixed_main_loop 4.96 |
| warm-up | 18.43 | 109.5 | render/thread/ms 201, stage/render/prepare_views 176, render/acquire/ms 175, render/busy/ms 24.12, main/busy/ms 20.95, encode//sum 14.10, stage/render/render 13.58, render/graph/ms 12.07, stage/render/graph_submit 9.24, stage/main/post_update 7.83, encode/submit 5.45, stage/main/fixed_main_loop 4.66 |
| warm-up | 18.53 | 100.8 | render/thread/ms 207, stage/render/prepare_views 184, render/acquire/ms 184, render/busy/ms 22.66, main/busy/ms 20.87, encode//sum 13.46, stage/render/render 12.63, render/graph/ms 11.26, stage/render/graph_submit 8.68, stage/main/post_update 8.23, encode/submit 5.17, stage/main/fixed_main_loop 4.58 |
| warm-up | 18.64 | 106.4 | render/thread/ms 205, stage/render/prepare_views 182, render/acquire/ms 182, main/busy/ms 21.97, render/busy/ms 21.87, encode//sum 11.89, stage/render/render 11.72, render/graph/ms 10.51, stage/main/post_update 8.44, stage/render/graph_submit 8.01, stage/main/fixed_main_loop 5.01, encode/submit 4.67 |
| warm-up | 18.74 | 95.3 | render/thread/ms 199, stage/render/prepare_views 175, render/acquire/ms 175, render/busy/ms 23.59, main/busy/ms 21.90, stage/render/render 13.15, encode//sum 12.70, render/graph/ms 11.79, stage/render/graph_submit 9.00, stage/main/post_update 8.14, encode/submit 5.36, stage/main/fixed_main_loop 4.93 |
| warm-up | 18.84 | 102.1 | render/thread/ms 195, stage/render/prepare_views 169, render/acquire/ms 169, render/busy/ms 24.70, main/busy/ms 22.01, stage/render/render 14.30, encode//sum 13.02, render/graph/ms 12.65, stage/render/graph_submit 9.74, stage/main/post_update 8.76, encode/submit 6.10, stage/main/fixed_main_loop 4.92 |
| warm-up | 18.94 | 100.8 | render/thread/ms 200, stage/render/prepare_views 177, render/acquire/ms 177, render/busy/ms 22.15, main/busy/ms 20.66, stage/render/render 13.20, encode//sum 11.97, render/graph/ms 11.56, stage/render/graph_submit 8.83, stage/main/post_update 8.37, encode/submit 5.31, stage/main/fixed_main_loop 4.98 |
| warm-up | 19.04 | 102.4 | render/thread/ms 200, stage/render/prepare_views 180, render/acquire/ms 180, main/busy/ms 19.96, render/busy/ms 19.27, stage/render/render 11.07, render/graph/ms 9.73, encode//sum 9.57, stage/main/post_update 7.84, stage/render/graph_submit 7.16, stage/main/fixed_main_loop 5.07, encode/submit 4.25 |
| measure | 9.64 | 99.8 | render/thread/ms 183, stage/render/prepare_views 165, render/acquire/ms 165, render/busy/ms 17.98, main/busy/ms 17.03, stage/render/render 9.26, render/graph/ms 8.13, encode//sum 7.51, stage/main/post_update 7.38, stage/render/graph_submit 5.82, encode/submit 3.50, stage/main/fixed_main_loop 3.28 |
| measure | 9.82 | 96.0 | render/thread/ms 180, stage/render/prepare_views 161, render/acquire/ms 161, main/busy/ms 18.90, render/busy/ms 18.58, stage/render/render 9.77, render/graph/ms 8.53, stage/main/post_update 7.54, encode//sum 7.41, stage/render/graph_submit 6.04, stage/main/fixed_main_loop 5.16, encode/submit 3.54 |
| measure | 10.00 | 96.4 | render/thread/ms 182, stage/render/prepare_views 163, render/acquire/ms 163, main/busy/ms 19.47, render/busy/ms 17.56, stage/render/render 8.99, stage/main/post_update 7.79, render/graph/ms 7.74, encode//sum 7.22, stage/render/graph_submit 5.48, stage/main/fixed_main_loop 4.85, encode/submit 3.41 |
| measure | 10.37 | 100.2 | render/thread/ms 185, stage/render/prepare_views 168, render/acquire/ms 167, main/busy/ms 19.29, render/busy/ms 17.22, stage/render/render 9.19, render/graph/ms 8.09, stage/main/post_update 7.62, encode//sum 7.61, stage/render/graph_submit 5.87, stage/main/fixed_main_loop 5.05, encode/submit 3.48 |
| measure | 10.74 | 103.4 | render/thread/ms 190, stage/render/prepare_views 172, render/acquire/ms 172, main/busy/ms 17.95, render/busy/ms 17.59, stage/render/render 9.76, encode//sum 9.07, render/graph/ms 8.49, stage/main/post_update 7.38, stage/render/graph_submit 6.12, stage/main/fixed_main_loop 4.97, encode/submit 3.83 |
| measure | 10.93 | 96.9 | render/thread/ms 185, stage/render/prepare_views 164, render/acquire/ms 163, render/busy/ms 21.05, main/busy/ms 19.74, stage/render/render 10.48, render/graph/ms 9.23, encode//sum 8.82, stage/main/post_update 7.52, stage/render/graph_submit 6.74, stage/main/fixed_main_loop 5.51, encode/submit 4.00 |
| measure | 11.12 | 101.8 | render/thread/ms 187, stage/render/prepare_views 164, render/acquire/ms 164, render/busy/ms 22.00, main/busy/ms 20.84, stage/render/render 12.11, render/graph/ms 10.61, encode//sum 9.37, stage/main/post_update 8.29, stage/render/graph_submit 7.77, stage/main/fixed_main_loop 5.90, encode/submit 4.89 |
| measure | 11.50 | 102.0 | render/thread/ms 191, stage/render/prepare_views 171, render/acquire/ms 171, main/busy/ms 20.76, render/busy/ms 18.71, stage/render/render 10.22, render/graph/ms 8.96, stage/main/post_update 8.20, encode//sum 7.90, stage/render/graph_submit 6.62, stage/main/fixed_main_loop 5.48, encode/submit 4.29 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --set groundcover.render.reserve_worst_case=true --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=worst-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **99%** (min 97%) · 2640 MHz (min 2640) · memory 8101 MHz · 45.9 W · 62 °C (max 64) · VRAM 7610 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `groundcover.render.reserve_worst_case=on (command line: --set)`
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
