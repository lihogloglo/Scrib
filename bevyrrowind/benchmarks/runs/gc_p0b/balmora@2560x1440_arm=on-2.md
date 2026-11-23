# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1298 frames over 15.0 s · **median 11.66 ms (86 fps)** · mean 11.56 · p95 12.94 · p99 13.36 (1% low 75 fps) · worst 19.80 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 310 frames over 8.7 s, worst 2563.2 ms
- hitches (> 29.2 ms): **0** measured, 33 in warm-up
- timed GPU passes: 10.15 ms of a 11.56 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.66 ms, limit 11 ms
- pass worst frame 19.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.210 | 3.215 | 3.312 | 1.00 | 0.067 |
| dlss_super_resolution | 2.019 | 2.023 | 2.044 | 1.00 | 0.180 |
| sun_visibility | 0.778 | 0.779 | 0.808 | 1.00 | 0.006 |
| early prepass | 0.640 | 0.641 | 0.650 | 1.00 | 0.096 |
| main_transmissive_pass_3d | 0.395 | 0.396 | 0.407 | 1.00 | 0.008 |
| atmosphere_luts | 0.366 | 0.366 | 0.439 | 1.00 | 0.003 |
| bloom | 0.313 | 0.314 | 0.330 | 1.00 | 0.060 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.306 | 0.306 | 0.310 | 1.00 | 0.001 |
| water_sspr | 0.216 | 0.216 | 0.213 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.196 | 1.00 | 0.007 |
| clouds_trace | 0.152 | 0.152 | 0.172 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.135 | 0.136 | 1.00 | 0.002 |
| uniform_allocation | 0.116 | 0.117 | 0.148 | 1.00 | 0.019 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.113 | 0.114 | 0.121 | 1.00 | 0.004 |
| bin_unpacking | 0.090 | 0.090 | 0.114 | 1.00 | 0.014 |
| tonemapping | 0.085 | 0.085 | 0.089 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.068 | 0.068 | 0.090 | 1.00 | 0.004 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.055 | 0.055 | 0.080 | 1.00 | 0.004 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.229 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.027 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.035 | 1.00 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.025 | 1.00 | 0.188 |
| surface_exposure_far | 0.012 | 0.040 | 0.043 | 0.31 | 0.002 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.001 |
| surface_exposure | 0.011 | 0.040 | 0.041 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.561 | 11.954 | 18.83 | 13708.2 |
| render/busy/ms | 7.136 | 8.130 | 15.40 | 9262.0 |
| main/busy/ms | 6.568 | 7.807 | 9.29 | 8525.2 |
| stage/render/render | 4.672 | 5.398 | 12.43 | 6064.4 |
| encode//sum | 4.553 | 5.811 | 13.31 | 5909.6 |
| render/graph/ms | 4.091 | 4.799 | 11.61 | 5310.4 |
| stage/main/post_update | 3.729 | 4.711 | 6.04 | 4839.7 |
| stage/render/prepare_views | 3.125 | 4.779 | 5.95 | 4056.1 |
| render/acquire/ms | 3.076 | 4.723 | 5.91 | 3993.0 |
| stage/render/graph_submit | 2.937 | 3.599 | 10.52 | 3812.0 |
| encode/submit | 1.808 | 2.198 | 3.71 | 2347.1 |
| stage/main/update | 1.315 | 1.616 | 2.26 | 1706.4 |
| encode/finish_wall | 1.146 | 1.554 | 7.59 | 1487.1 |
| stage/render/camera_driver | 1.072 | 1.227 | 1.65 | 1391.3 |
| pipelines/compile_ms | 1.047 | 0.000 | 129.44 | 1358.7 |
| pipelines/front_ms | 1.019 | 0.000 | 126.20 | 1322.9 |
| encode/pass:shadow_directional_light/sum | 0.726 | 1.291 | 7.18 | 942.4 |
| encode/pass:early prepass/sum | 0.718 | 0.988 | 6.63 | 931.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.692 | 0.975 | 1.37 | 898.0 |
| stage/render/prepare_write_phase_buffers | 0.485 | 0.707 | 0.91 | 630.0 |
| stage/main/fixed_main_loop | 0.416 | 0.633 | 1.34 | 540.5 |
| stage/render/prepare_resources | 0.415 | 0.483 | 0.74 | 538.2 |
| stage/render/prepare_assets | 0.386 | 0.465 | 1.96 | 501.6 |
| stage/main/pre_update | 0.378 | 0.436 | 0.70 | 490.3 |
| render/present/ms | 0.349 | 0.412 | 1.52 | 453.2 |
| bigspace/total/ms | 0.338 | 0.640 | 1.66 | 438.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.322 | 0.458 | 0.69 | 418.4 |
| stage/main/first | 0.311 | 0.371 | 2.09 | 403.6 |
| stage/render/prepare_flush | 0.269 | 0.408 | 0.86 | 349.5 |
| stage/render/prepare_batch_phases | 0.256 | 0.360 | 0.47 | 332.2 |
| stage/render/prepare_bind_groups | 0.251 | 0.343 | 0.49 | 326.2 |
| stage/main/last | 0.246 | 0.354 | 0.82 | 319.6 |
| stage/render/prepare_meshes | 0.203 | 0.251 | 1.11 | 263.4 |
| stage/render/extract_commands | 0.193 | 0.223 | 0.31 | 250.1 |
| controller/feed/ms | 0.182 | 0.226 | 0.61 | 235.9 |
| stage/render/create_views | 0.156 | 0.186 | 0.30 | 202.4 |
| stream/drive/ms | 0.133 | 0.154 | 0.70 | 172.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.121 | 0.181 | 0.29 | 156.5 |
| bigspace/high_precision/ms | 0.119 | 0.295 | 1.33 | 153.9 |
| stream/tick/ms | 0.108 | 0.126 | 0.67 | 139.6 |
| stage/render/queue | 0.100 | 0.131 | 2.11 | 129.4 |
| bigspace/low_precision/ms | 0.088 | 0.222 | 1.39 | 114.9 |
| controller/move/ms | 0.076 | 0.161 | 0.22 | 98.6 |
| stage/render/specialize | 0.076 | 0.102 | 3.14 | 98.3 |
| anim/evaluate/ms | 0.070 | 0.105 | 0.64 | 91.3 |
| anim/evaluate_cpu/ms | 0.060 | 0.072 | 0.54 | 78.0 |
| encode/resolve_diagnostics/sum | 0.056 | 0.082 | 0.15 | 72.7 |
| controller/write/ms | 0.052 | 0.067 | 0.11 | 67.4 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.081 | 0.16 | 59.5 |
| terrain/update/ms | 0.035 | 0.047 | 0.49 | 45.9 |
| physics/step/ms | 0.029 | 0.048 | 0.13 | 37.1 |
| pipelines/driver_ms | 0.028 | 0.000 | 3.81 | 35.8 |
| stage/render/phase_sort | 0.026 | 0.044 | 0.12 | 34.0 |
| stage/render/cleanup | 0.025 | 0.033 | 0.41 | 32.9 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.037 | 0.09 | 31.6 |
| weather/map/ms | 0.023 | 0.031 | 0.08 | 29.9 |
| encode/camera_driver_span_end/sum | 0.021 | 0.033 | 0.08 | 27.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.08 | 13.2 |
| script/update/ms | 0.006 | 0.008 | 0.02 | 8.0 |
| script/input/ms | 0.005 | 0.006 | 0.01 | 6.3 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 5.7 |
| controller/avoid/ms | 0.004 | 0.007 | 0.02 | 5.3 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 4.6 |
| script/ui/ms | 0.003 | 0.004 | 0.03 | 3.8 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.01 | 2.2 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.02 | 1.7 |
| stream/promote/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| character/finish/ms | 0.001 | 0.001 | 0.00 | 1.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 145 | 181 | 102 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.57 | 7.34 | 0.59 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.08 | 0.02 |
| encode/finish_critical | ms | 0.86 | 7.34 | 0.67 |
| encode/pass:early prepass/max | ms | 0.72 | 6.63 | 0.56 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.69 | 1.37 | 0.67 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.29 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.32 | 0.69 | 0.34 |
| encode/pass:shadow_directional_light/max | ms | 0.68 | 7.18 | 0.54 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.15 | 0.06 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.77 | 15.40 | 10.69 |
| gpu/timed/ms | ms | 10.77 | 12.01 | 10.69 |
| groundcover/batches/count | count | 0 | 0 | 0 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1312 | 1312 | 1312 |
| groundcover/slots/capacity | count | 0 | 0 | 0 |
| groundcover/slots/capacity_bytes | bytes | 0 | 0 | 0 |
| groundcover/slots/live | count | 0 | 0 | 0 |
| groundcover/tiles/capped | count | 0 | 0 | 0 |
| groundcover/tiles/casting | count | 6 | 6 | 6 |
| groundcover/tiles/resident | count | 63 | 63 | 63 |
| groundcover/tiles/scattered | count | 63 | 63 | 63 |
| groundcover/tiles/unplaced | count | 0 | 0 | 0 |
| groundcover/upload/bytes | bytes | 0 | 0 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1931 | 1931 | 1931 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 217.5 k | 217.5 k | 217.5 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.3 k | 11.3 k | 11.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 248 | 248 | 248 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 319 | 319 | 319 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 248 | 248 | 248 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 319 | 319 | 319 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 875 | 875 | 875 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 875 | 875 | 875 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 41 | 4 |
| phases/shadow/bins | count | 896 | 1509 | 896 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 5861 | 17.6 k | 5861 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 3202 | 8161 | 3202 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 34 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 332 | 332 | 332 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 23.08 | 38.84 | 22.86 |
| process/mem_usage | GiB | 2.99 | 3.05 | 3.05 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2386 | 2386 | 2386 |
| scene/batch/masked/live | count | 2386 | 2386 | 2386 |
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
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1639 | 1639 | 1639 |
| scene/chains/live | count | 1639 | 1639 | 1639 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 0 | 0 | 0 |
| shadow_atlas/live_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/live_tiles | count | 0 | 0 | 0 |
| shadow_atlas/mover_draws | count | 0 | 0 | 0 |
| shadow_atlas/movers | count | 0 | 0 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 0 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/live_direct | count | 0 | 0 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 0 | 0 | 0 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/static_tiles | count | 0 | 0 | 0 |
| shadow_atlas/views | count | 0 | 0 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 5 | 7 | 5 |
| skin/blocks_skipped/count | count | 176 | 180 | 176 |
| skin/blocks_walked/count | count | 5 | 7 | 5 |
| skin/joint_bytes_written/bytes | bytes | 9984 | 12.5 k | 12.5 k |
| skin/joints_written/count | count | 156 | 196 | 196 |
| skin/upload_writes/count | count | 4 | 6 | 5 |
| started_load_count |  | 5306 | 5308 | 5308 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.00 | 37.14 | 25.80 |
| system/mem_usage | % | 49.84 | 50.02 | 50.02 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 64 | 101 | 64 |
| views/all/bins | count | 3142 | 3755 | 3142 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 17.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 9408 | 14.4 k | 9408 |
| views/all/pipelines | count | 75 | 82 | 75 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.9 k | 260.4 k | 153.9 k |
| views/cascade/batch_sets | count | 4 | 41 | 4 |
| views/cascade/bins | count | 896 | 1509 | 896 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 5861 | 17.6 k | 5861 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 3202 | 8161 | 3202 |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 49.2 k | 155.6 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 60 | 60 | 60 |
| views/main/bins | count | 2246 | 2246 | 2246 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6206 | 6206 | 6206 |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 104.8 k | 104.8 k | 104.8 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.5 k | 15.5 k | 15.5 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 130 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 0.00 | 4 | 4 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 28.55 | 31 | 37.1 k |
| encode/bind_walk/init_textures | count | 7193 | 12.1 k | 9.34 M |
| encode/bind_walk/init_textures_skipped | count | 33.9 k | 41.1 k | 44.02 M |
| encode/bind_walk/inits | count | 504 | 608 | 654.0 k |
| encode/bind_walk/inits_skipped | count | 119 | 129 | 154.8 k |
| encode/bind_walk/merge_textures | count | 7128 | 12.0 k | 9.25 M |
| encode/bind_walk/merge_textures_skipped | count | 6919 | 14.2 k | 8.98 M |
| encode/bind_walk/merges | count | 91.25 | 111 | 118.4 k |
| encode/bind_walk/merges_skipped | count | 109 | 127 | 142.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1298 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1298 |
| encode/pass:early prepass/count | count | 1 | 1 | 1298 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1298 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1298 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1298 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1298 |
| encode/queue/command_buffers | count | 38.55 | 41 | 50.0 k |
| encode/queue/submits | count | 2 | 2 | 2596 |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 3.36 M | 2.43 G |
| encode/queue/write_buffer/calls | count | 233 | 304 | 301.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 42.37 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.7 k |
| encode/queue/write_texture/bytes | bytes | 20.3 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1298 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.69 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 21 |
| pipelines/created | count | 0.02 | 1 | 21 |
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
| surface/shelter_far_redraws | count | 0.00 | 1 | 1 |
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.2 ms; slowest 33, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2563.2 | main/busy/ms 2531, stage/main/post_update 2416, stage/main/update 101, surface/advance_cpu_ms 70.55, stage/main/pre_update 10.15, stage/main/last 1.16, stage/main/first 0.74, surface/cell_upload_ms 0.74, stage/main/fixed_main_loop 0.37, stream/drive/ms 0.36, weather/map/ms 0.18, stream/tick/ms 0.13 |
| warm-up | 3.31 | 746.0 | main/busy/ms 2576, stage/main/post_update 2447, render/thread/ms 685, render/busy/ms 529, stage/render/prepare_assets 502, stage/render/prepare_views 153, render/acquire/ms 153, stage/main/update 108, surface/advance_cpu_ms 70.55, stage/render/render 13.73, stage/main/pre_update 12.15, render/graph/ms 9.29 |
| warm-up | 3.39 | 75.9 | render/thread/ms 759, render/busy/ms 548, stage/render/prepare_assets 507, stage/render/prepare_views 153, render/acquire/ms 153, main/busy/ms 82.01, stage/render/render 74.75, render/present/ms 58.15, stage/main/post_update 47.81, pipelines/compile_ms 23.00, terrain/update/ms 22.60, pipelines/front_ms 20.05 |
| warm-up | 3.43 | 43.8 | pipelines/compile_ms 179, pipelines/front_ms 124, render/thread/ms 116, stage/render/render 74.51, render/present/ms 62.10, main/busy/ms 57.13, pipelines/driver_ms 54.52, render/busy/ms 53.48, terrain/update/ms 28.50, stage/main/post_update 27.48, pipelines/wait_ms 27.10, stage/render/prepare_assets 26.78 |
| warm-up | 3.53 | 97.0 | pipelines/wait_ms 3043, pipelines/compile_ms 1071, pipelines/front_ms 954, render/thread/ms 137, render/busy/ms 129, pipelines/driver_ms 117, main/busy/ms 65.71, stage/main/post_update 42.54, stage/render/prepare_assets 40.40, stage/render/prepare_resources 40.29, stage/render/render 24.41, stage/render/prepare_flush 19.95 |
| warm-up | 3.64 | 114.2 | pipelines/wait_ms 6111, pipelines/compile_ms 2028, pipelines/front_ms 1877, render/thread/ms 207, render/busy/ms 205, pipelines/driver_ms 151, main/busy/ms 117, stage/render/prepare_resources 68.26, stage/render/prepare_assets 61.59, stage/main/update 59.89, stage/main/post_update 43.54, stage/render/render 37.43 |
| warm-up | 3.70 | 58.1 | pipelines/wait_ms 3232, pipelines/compile_ms 1989, pipelines/front_ms 1868, render/thread/ms 167, render/busy/ms 165, pipelines/driver_ms 121, main/busy/ms 88.11, stage/render/prepare_assets 87.55, stage/main/update 62.12, stage/render/render 32.30, stage/render/prepare_resources 30.99, render/graph/ms 29.62 |
| warm-up | 3.75 | 51.2 | pipelines/compile_ms 1468, pipelines/front_ms 1399, pipelines/wait_ms 316, render/thread/ms 103, render/busy/ms 102, stage/render/prepare_assets 81.02, pipelines/driver_ms 68.81, main/busy/ms 42.69, stage/main/post_update 21.94, terrain/update/ms 11.00, stage/main/update 10.16, stage/render/render 9.90 |
| warm-up | 3.80 | 29.8 | pipelines/compile_ms 701, pipelines/front_ms 658, pipelines/wait_ms 53.21, render/thread/ms 49.73, render/busy/ms 47.89, main/busy/ms 44.78, pipelines/driver_ms 42.77, stage/main/post_update 20.26, encode//sum 17.52, stage/main/update 14.50, stage/render/render 14.23, render/graph/ms 11.93 |
| warm-up | 3.84 | 36.0 | pipelines/compile_ms 496, pipelines/front_ms 476, render/thread/ms 62.00, render/busy/ms 58.80, main/busy/ms 46.19, stage/main/post_update 23.50, stage/render/render 21.15, pipelines/driver_ms 20.42, encode//sum 18.99, render/graph/ms 17.63, stage/main/update 14.13, stage/render/graph_submit 13.58 |
| warm-up | 3.90 | 59.0 | pipelines/compile_ms 445, pipelines/front_ms 420, render/thread/ms 90.30, render/busy/ms 86.92, stage/render/prepare_assets 43.50, main/busy/ms 43.43, pipelines/driver_ms 25.37, stage/render/render 24.54, render/graph/ms 20.96, encode//sum 18.01, stage/render/graph_submit 16.92, stage/main/post_update 15.61 |
| warm-up | 3.94 | 41.0 | pipelines/compile_ms 1493, pipelines/front_ms 1421, pipelines/wait_ms 178, render/thread/ms 94.55, render/busy/ms 92.17, pipelines/driver_ms 72.24, stage/render/prepare_assets 62.35, main/busy/ms 47.41, stage/render/render 21.38, stage/main/update 20.36, render/graph/ms 18.48, encode//sum 17.70 |
| warm-up | 3.99 | 52.7 | pipelines/compile_ms 1800, pipelines/front_ms 1732, pipelines/wait_ms 178, render/thread/ms 89.53, render/busy/ms 87.71, pipelines/driver_ms 68.30, stage/render/prepare_assets 67.40, main/busy/ms 38.70, stage/main/update 16.12, stage/render/render 13.46, render/graph/ms 11.17, stage/main/post_update 8.81 |
| warm-up | 4.03 | 37.1 | pipelines/compile_ms 640, pipelines/front_ms 620, render/thread/ms 85.87, render/busy/ms 84.52, stage/render/prepare_assets 67.69, main/busy/ms 33.31, pipelines/driver_ms 19.69, stage/render/render 11.88, stage/main/update 11.05, render/graph/ms 10.14, pipelines/wait_ms 8.03, pipelines/lock_ms 7.31 |
| warm-up | 4.06 | 30.5 | pipelines/compile_ms 210, pipelines/front_ms 201, render/thread/ms 64.57, render/busy/ms 62.93, stage/render/prepare_assets 39.97, main/busy/ms 30.67, encode//sum 17.13, stage/render/render 15.55, render/graph/ms 13.70, stage/main/update 11.09, stage/render/graph_submit 9.68, pipelines/driver_ms 9.33 |
| warm-up | 4.10 | 30.4 | pipelines/compile_ms 237, pipelines/front_ms 231, main/busy/ms 29.20, render/thread/ms 29.15, render/busy/ms 27.74, stage/main/update 12.49, stage/main/post_update 11.83, stage/render/render 11.81, render/graph/ms 9.85, encode//sum 8.82, stage/render/prepare_assets 8.73, stage/render/graph_submit 6.62 |
| warm-up | 4.17 | 54.3 | pipelines/compile_ms 224, pipelines/front_ms 219, render/thread/ms 57.82, render/busy/ms 55.68, main/busy/ms 45.23, stage/render/prepare_assets 22.98, stage/main/update 22.94, stage/render/render 21.11, render/graph/ms 18.46, stage/main/post_update 15.69, stage/render/graph_submit 12.16, encode//sum 7.73 |
| warm-up | 4.22 | 48.6 | pipelines/compile_ms 1257, pipelines/front_ms 1223, render/thread/ms 89.42, render/busy/ms 87.31, main/busy/ms 56.67, stage/render/prepare_assets 46.37, stage/main/update 35.78, pipelines/driver_ms 34.89, stage/render/render 21.30, render/graph/ms 18.55, pipelines/wait_ms 17.11, stage/main/post_update 13.92 |
| warm-up | 4.28 | 57.4 | pipelines/compile_ms 1744, pipelines/front_ms 1684, render/thread/ms 102, render/busy/ms 100, stage/render/prepare_assets 69.70, main/busy/ms 61.26, pipelines/driver_ms 60.39, stage/main/update 38.99, pipelines/wait_ms 18.52, stage/render/render 14.62, stage/main/post_update 12.87, render/graph/ms 12.02 |
| warm-up | 4.35 | 70.5 | pipelines/compile_ms 871, pipelines/front_ms 832, render/thread/ms 124, render/busy/ms 120, stage/render/prepare_assets 69.20, main/busy/ms 55.54, stage/render/render 39.34, pipelines/driver_ms 38.23, render/graph/ms 34.62, stage/render/graph_submit 31.11, stage/main/update 29.32, pipelines/wait_ms 16.65 |
| warm-up | 4.41 | 61.1 | pipelines/compile_ms 888, pipelines/front_ms 854, render/thread/ms 128, pipelines/wait_ms 126, render/busy/ms 124, main/busy/ms 64.22, stage/render/prepare_assets 58.30, stage/render/render 51.00, render/graph/ms 37.99, stage/render/graph_submit 34.65, pipelines/driver_ms 33.85, stage/main/update 29.38 |
| warm-up | 4.44 | 35.3 | pipelines/compile_ms 1151, pipelines/front_ms 1112, pipelines/wait_ms 112, render/thread/ms 92.84, render/busy/ms 91.26, main/busy/ms 57.05, stage/render/prepare_assets 41.35, pipelines/driver_ms 39.57, stage/render/render 22.53, stage/main/post_update 20.55, stage/main/update 19.73, render/graph/ms 12.05 |
| warm-up | 4.51 | 64.7 | pipelines/compile_ms 903, pipelines/front_ms 875, pipelines/wait_ms 185, render/thread/ms 97.18, render/busy/ms 95.88, main/busy/ms 41.57, stage/render/prepare_assets 40.01, pipelines/driver_ms 27.71, stage/main/post_update 16.85, stage/main/update 15.09, stage/render/prepare_write_phase_buffers 13.89, stage/render/prepare_resources 12.98 |
| warm-up | 4.61 | 99.5 | pipelines/compile_ms 2189, pipelines/front_ms 2129, pipelines/wait_ms 664, render/thread/ms 162, render/busy/ms 160, main/busy/ms 122, stage/main/update 74.55, pipelines/driver_ms 60.86, stage/render/prepare_assets 46.46, stage/render/prepare_resources 34.64, stage/render/prepare_write_phase_buffers 28.50, stage/render/prepare_flush 22.07 |
| warm-up | 4.69 | 78.6 | pipelines/compile_ms 2570, pipelines/front_ms 2499, pipelines/wait_ms 585, render/thread/ms 170, render/busy/ms 168, main/busy/ms 156, stage/main/update 92.44, pipelines/driver_ms 70.49, stage/render/prepare_assets 62.40, stage/render/prepare_resources 30.80, stage/main/post_update 27.49, stage/render/prepare_write_phase_buffers 23.14 |
| warm-up | 4.74 | 49.9 | pipelines/compile_ms 1486, pipelines/front_ms 1443, pipelines/wait_ms 344, render/thread/ms 119, render/busy/ms 117, main/busy/ms 99.97, stage/render/prepare_assets 73.75, stage/main/update 53.25, pipelines/driver_ms 43.01, stage/main/post_update 30.48, stage/render/render 16.92, render/graph/ms 14.06 |
| warm-up | 4.79 | 52.0 | pipelines/compile_ms 1766, pipelines/front_ms 1722, pipelines/wait_ms 761, render/thread/ms 97.44, render/busy/ms 95.75, main/busy/ms 77.00, stage/render/prepare_assets 63.66, pipelines/driver_ms 44.09, stage/main/update 43.40, stage/main/post_update 18.68, stage/render/render 14.42, render/graph/ms 11.82 |
| warm-up | 4.82 | 31.3 | pipelines/compile_ms 1404, pipelines/front_ms 1373, pipelines/wait_ms 524, render/thread/ms 79.72, render/busy/ms 78.16, main/busy/ms 48.82, stage/render/prepare_assets 44.92, pipelines/driver_ms 30.77, stage/main/update 27.41, stage/render/render 14.35, render/graph/ms 12.42, stage/main/post_update 10.57 |
| warm-up | 5.13 | 49.9 | pipelines/compile_ms 216, pipelines/front_ms 209, main/busy/ms 66.14, render/thread/ms 62.48, render/busy/ms 59.01, stage/main/update 34.77, stage/render/render 22.51, render/graph/ms 17.08, stage/render/prepare_resources 14.67, stage/render/graph_submit 13.15, stage/main/post_update 12.08, encode//sum 10.80 |
| warm-up | 5.20 | 66.8 | pipelines/compile_ms 907, pipelines/front_ms 884, pipelines/wait_ms 117, render/thread/ms 80.93, render/busy/ms 79.61, main/busy/ms 70.91, stage/main/update 35.11, stage/main/post_update 24.57, pipelines/driver_ms 22.97, stage/render/prepare_resources 21.60, stage/render/prepare_assets 19.83, stage/render/render 16.57 |
| warm-up | 5.25 | 30.3 | pipelines/compile_ms 1059, pipelines/front_ms 1029, render/thread/ms 51.72, render/busy/ms 50.26, main/busy/ms 35.85, pipelines/driver_ms 30.54, stage/render/prepare_assets 18.23, stage/main/update 17.36, stage/render/render 13.30, render/graph/ms 11.40, stage/main/post_update 8.88, encode//sum 7.95 |
| warm-up | 5.42 | 37.5 | pipelines/compile_ms 177, pipelines/front_ms 174, render/thread/ms 54.86, render/busy/ms 52.90, main/busy/ms 38.71, stage/render/prepare_assets 21.46, stage/main/update 18.16, stage/render/render 17.68, render/graph/ms 14.99, stage/main/post_update 11.76, stage/render/graph_submit 11.06, encode//sum 10.70 |
| warm-up | 5.75 | 31.9 | pipelines/compile_ms 139, pipelines/front_ms 135, main/busy/ms 38.46, render/thread/ms 36.59, render/busy/ms 35.32, stage/main/update 19.81, stage/main/post_update 12.93, stage/render/render 11.54, render/graph/ms 9.80, stage/render/prepare_assets 9.69, encode//sum 8.20, stage/render/graph_submit 6.89 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=on-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 1%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 91.9 W · 75 °C (max 76) · VRAM 5347 MiB · throttled 100% of samples

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
