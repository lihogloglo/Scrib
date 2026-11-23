# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.render.reserve_worst_case=on (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1183 frames over 15.0 s · **median 12.77 ms (78 fps)** · mean 12.69 · p95 14.40 · p99 14.77 (1% low 68 fps) · worst 17.63 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 310 frames over 9.3 s, worst 2627.2 ms
- hitches (> 31.9 ms): **0** measured, 30 in warm-up
- timed GPU passes: 11.18 ms of a 12.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.77 ms, limit 11 ms
- pass worst frame 17.63 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.229 | 3.229 | 3.318 | 1.00 | 0.109 |
| dlss_super_resolution | 2.022 | 2.022 | 2.046 | 1.00 | 0.182 |
| sun_visibility | 0.807 | 0.807 | 0.810 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.779 | 0.779 | 0.899 | 1.00 | 0.009 |
| early prepass | 0.629 | 0.629 | 0.635 | 1.00 | 0.132 |
| main_transmissive_pass_3d | 0.400 | 0.400 | 0.411 | 1.00 | 0.008 |
| atmosphere_luts | 0.371 | 0.371 | 0.379 | 1.00 | 0.003 |
| bloom | 0.316 | 0.316 | 0.331 | 1.00 | 0.059 |
| atmos_froxel_inject | 0.310 | 0.310 | 0.308 | 1.00 | 0.001 |
| water_velocity | 0.307 | 0.307 | 0.311 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.235 | 0.235 | 0.270 | 1.00 | 0.004 |
| water_sspr | 0.218 | 0.218 | 0.219 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.198 | 1.00 | 0.008 |
| clouds_trace | 0.151 | 0.151 | 0.170 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.134 | 0.138 | 1.00 | 0.002 |
| uniform_allocation | 0.120 | 0.120 | 0.153 | 1.00 | 0.021 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.113 | 0.113 | 0.120 | 1.00 | 0.004 |
| bin_unpacking | 0.098 | 0.098 | 0.123 | 1.00 | 0.015 |
| tonemapping | 0.085 | 0.085 | 0.089 | 1.00 | 0.003 |
| range_unpacking | 0.083 | 0.083 | 0.093 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.411 | 0.13 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.028 | 0.028 | 0.056 | 1.00 | 0.007 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.229 | 0.13 | 0.004 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.024 | 1.00 | 0.193 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| surface_exposure_far | 0.011 | 0.035 | 0.036 | 0.31 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| surface_exposure | 0.010 | 0.036 | 0.037 | 0.28 | 0.001 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.13 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.680 | 13.403 | 16.44 | 13817.9 |
| render/busy/ms | 7.843 | 9.110 | 15.97 | 9278.6 |
| main/busy/ms | 6.677 | 7.941 | 9.96 | 7898.8 |
| encode//sum | 5.655 | 7.379 | 10.70 | 6690.4 |
| stage/render/render | 5.233 | 6.128 | 7.61 | 6190.6 |
| render/graph/ms | 4.634 | 5.482 | 6.76 | 5482.1 |
| stage/main/post_update | 3.758 | 4.811 | 6.64 | 4445.4 |
| stage/render/prepare_views | 3.522 | 5.214 | 6.64 | 4166.8 |
| render/acquire/ms | 3.474 | 5.168 | 6.60 | 4109.7 |
| stage/render/graph_submit | 3.359 | 4.174 | 5.14 | 3973.1 |
| encode/submit | 2.015 | 2.412 | 3.19 | 2383.2 |
| encode/finish_wall | 1.372 | 1.877 | 2.56 | 1623.2 |
| stage/main/update | 1.343 | 1.590 | 2.19 | 1588.6 |
| stage/render/camera_driver | 1.190 | 1.365 | 1.60 | 1407.9 |
| pipelines/compile_ms | 0.944 | 0.000 | 127.93 | 1116.7 |
| pipelines/front_ms | 0.920 | 0.000 | 123.69 | 1088.8 |
| encode/pass:early prepass/sum | 0.869 | 1.195 | 1.88 | 1028.2 |
| encode/pass:shadow_directional_light/sum | 0.862 | 1.498 | 2.41 | 1020.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.832 | 1.166 | 1.47 | 984.7 |
| stage/render/prepare_write_phase_buffers | 0.516 | 0.762 | 0.98 | 610.4 |
| stage/main/fixed_main_loop | 0.460 | 0.670 | 1.25 | 544.0 |
| stage/render/prepare_resources | 0.431 | 0.503 | 0.61 | 509.9 |
| stage/render/prepare_assets | 0.410 | 0.496 | 1.45 | 484.9 |
| stage/main/pre_update | 0.389 | 0.462 | 1.37 | 459.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.367 | 0.510 | 0.74 | 434.5 |
| render/present/ms | 0.363 | 0.427 | 0.92 | 429.6 |
| bigspace/total/ms | 0.327 | 0.585 | 1.27 | 387.1 |
| stage/main/first | 0.309 | 0.367 | 1.89 | 365.5 |
| stage/render/prepare_flush | 0.277 | 0.423 | 1.50 | 327.3 |
| stage/render/prepare_batch_phases | 0.273 | 0.383 | 0.58 | 322.9 |
| stage/render/prepare_bind_groups | 0.272 | 0.420 | 0.58 | 322.4 |
| stage/main/last | 0.242 | 0.344 | 0.83 | 286.1 |
| stage/render/prepare_meshes | 0.226 | 0.274 | 1.11 | 267.5 |
| stage/render/extract_commands | 0.201 | 0.235 | 0.33 | 237.9 |
| controller/feed/ms | 0.188 | 0.239 | 0.58 | 222.4 |
| stage/render/create_views | 0.161 | 0.200 | 0.29 | 190.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.146 | 0.216 | 0.36 | 172.5 |
| stream/drive/ms | 0.134 | 0.159 | 0.38 | 158.3 |
| bigspace/high_precision/ms | 0.114 | 0.287 | 0.99 | 134.7 |
| stream/tick/ms | 0.107 | 0.127 | 0.33 | 126.1 |
| stage/render/queue | 0.101 | 0.137 | 2.35 | 119.5 |
| controller/move/ms | 0.089 | 0.171 | 0.26 | 105.1 |
| bigspace/low_precision/ms | 0.084 | 0.159 | 0.95 | 99.2 |
| stage/render/specialize | 0.081 | 0.107 | 3.49 | 95.6 |
| anim/evaluate/ms | 0.071 | 0.094 | 0.58 | 83.5 |
| encode/resolve_diagnostics/sum | 0.065 | 0.096 | 0.32 | 77.4 |
| anim/evaluate_cpu/ms | 0.061 | 0.074 | 0.43 | 72.5 |
| controller/write/ms | 0.052 | 0.066 | 0.18 | 61.0 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.077 | 0.21 | 54.5 |
| terrain/update/ms | 0.034 | 0.046 | 0.14 | 40.7 |
| physics/step/ms | 0.032 | 0.050 | 0.14 | 38.2 |
| stage/render/phase_sort | 0.028 | 0.050 | 0.11 | 33.5 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.041 | 0.09 | 32.3 |
| stage/render/cleanup | 0.025 | 0.032 | 0.43 | 29.6 |
| encode/camera_driver_span_end/sum | 0.024 | 0.037 | 0.13 | 28.3 |
| pipelines/driver_ms | 0.024 | 0.000 | 4.24 | 27.8 |
| weather/map/ms | 0.023 | 0.032 | 0.12 | 27.7 |
| weather/eval/ms | 0.011 | 0.014 | 0.12 | 12.9 |
| script/update/ms | 0.007 | 0.008 | 0.06 | 8.2 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 6.5 |
| anim/apply/ms | 0.005 | 0.006 | 0.16 | 5.3 |
| controller/avoid/ms | 0.004 | 0.006 | 0.02 | 5.0 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 4.3 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 4.0 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.01 | 2.0 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.4 |
| stream/promote/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.06 | 1.1 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 0.7 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.3 |
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
| crowd/joints_extracted/count | count | 148 | 181 | 180 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 38.0 k | 38.0 k | 38.0 k |
| encode//max | ms | 0.66 | 2.01 | 0.95 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.09 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.13 | 0.08 |
| encode/finish_critical | ms | 1.02 | 2.41 | 1.28 |
| encode/pass:early prepass/max | ms | 0.85 | 1.88 | 1.28 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 1.47 | 0.90 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.14 | 0.36 | 0.17 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.36 | 0.74 | 0.38 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.41 | 0.81 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.32 | 0.09 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 11.85 | 15.97 | 11.66 |
| gpu/timed/ms | ms | 11.85 | 13.23 | 11.66 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1275 | 1275 | 1275 |
| groundcover/slots/capacity | count | 658.0 k | 658.0 k | 658.0 k |
| groundcover/slots/capacity_bytes | bytes | 42.11 M | 42.11 M | 42.11 M |
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
| phases/alpha_mask3d/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d/bins | count | 258 | 258 | 258 |
| phases/alpha_mask3d/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d/instance_draws | count | 573 | 573 | 573 |
| phases/alpha_mask3d/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d/items | count | 329 | 329 | 329 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d_prepass/bins | count | 258 | 258 | 258 |
| phases/alpha_mask3d_prepass/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d_prepass/instance_draws | count | 573 | 573 | 573 |
| phases/alpha_mask3d_prepass/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d_prepass/items | count | 329 | 329 | 329 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 31 | 31 | 31 |
| phases/opaque3d/bins | count | 884 | 884 | 884 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/bins | count | 884 | 884 | 884 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 5306 | 5306 | 5306 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2792 | 2792 | 2792 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 9 | 56 | 9 |
| phases/shadow/bins | count | 911 | 1524 | 911 |
| phases/shadow/instance_batches | count | 9 | 27 | 9 |
| phases/shadow/instance_draws | count | 5866 | 17.6 k | 5866 |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| phases/shadow/items | count | 3217 | 8176 | 3217 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 40 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 346 | 346 | 346 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 24.04 | 34.62 | 24.76 |
| process/mem_usage | GiB | 3.05 | 3.11 | 3.11 |
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
| skin/blocks_changed/count | count | 5 | 7 | 6 |
| skin/blocks_skipped/count | count | 176 | 177 | 175 |
| skin/blocks_walked/count | count | 5 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 10.2 k | 12.5 k | 12.3 k |
| skin/joints_written/count | count | 160 | 196 | 192 |
| skin/upload_writes/count | count | 4 | 5 | 3 |
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
| system/cpu_usage | % | 29.18 | 39.45 | 25.62 |
| system/mem_usage | % | 50.23 | 50.40 | 50.40 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2548 | 2548 | 2548 |
| textures/images | count | 2000 | 2000 | 2000 |
| views/all/batch_sets | count | 105 | 152 | 105 |
| views/all/bins | count | 3195 | 3808 | 3195 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 17.6 k | 29.4 k | 17.6 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 9459 | 14.4 k | 9459 |
| views/all/pipelines | count | 89 | 96 | 89 |
| views/all/pipelines_distinct | count | 73 | 73 | 73 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.50 M |
| views/cascade/batch_sets | count | 9 | 56 | 9 |
| views/cascade/bins | count | 911 | 1524 | 911 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 9 |
| views/cascade/instance_draws | count | 5866 | 17.6 k | 5866 |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| views/cascade/items | count | 3217 | 8176 | 3217 |
| views/cascade/pipelines | count | 33 | 40 | 33 |
| views/cascade/work_items | count | 82.9 k | 256.7 k | 82.9 k |
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
| views/main/batch_sets | count | 96 | 96 | 96 |
| views/main/bins | count | 2284 | 2284 | 2284 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 11.8 k | 11.8 k | 11.8 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6242 | 6242 | 6242 |
| views/main/pipelines | count | 56 | 56 | 56 |
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
| world/meshes | count | 15.5 k | 15.5 k | 15.5 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.12 | 3 | 147 |
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
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.56 | 31 | 33.8 k |
| encode/bind_walk/init_textures | count | 7453 | 12.4 k | 8.82 M |
| encode/bind_walk/init_textures_skipped | count | 41.1 k | 46.9 k | 48.67 M |
| encode/bind_walk/inits | count | 652 | 779 | 771.7 k |
| encode/bind_walk/inits_skipped | count | 123 | 132 | 145.5 k |
| encode/bind_walk/merge_textures | count | 7389 | 12.3 k | 8.74 M |
| encode/bind_walk/merge_textures_skipped | count | 14.1 k | 19.9 k | 16.74 M |
| encode/bind_walk/merges | count | 135 | 162 | 159.4 k |
| encode/bind_walk/merges_skipped | count | 112 | 127 | 132.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1183 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1183 |
| encode/pass:early prepass/count | count | 1 | 1 | 1183 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1183 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1183 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1183 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1183 |
| encode/queue/command_buffers | count | 38.56 | 41 | 45.6 k |
| encode/queue/submits | count | 2 | 2 | 2366 |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 3.38 M | 2.24 G |
| encode/queue/write_buffer/calls | count | 251 | 324 | 296.4 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 38.61 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.6 k |
| encode/queue/write_texture/bytes | bytes | 22.3 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1183 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.76 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.01 | 1 | 16 |
| pipelines/created | count | 0.01 | 1 | 16 |
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

## Hitches (> 31.9 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.63 | 2627.2 | main/busy/ms 2590, stage/main/post_update 2470, stage/main/update 107, surface/advance_cpu_ms 76.35, stage/main/pre_update 9.31, stage/main/last 1.07, surface/cell_upload_ms 0.74, stage/main/first 0.57, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.29, weather/map/ms 0.25, bigspace/total/ms 0.09 |
| warm-up | 3.42 | 794.2 | main/busy/ms 2637, stage/main/post_update 2503, render/thread/ms 729, render/busy/ms 570, stage/render/prepare_assets 535, stage/render/prepare_views 156, render/acquire/ms 156, stage/main/update 114, surface/advance_cpu_ms 76.35, stage/render/render 22.90, render/graph/ms 18.49, stage/render/camera_driver 16.18 |
| warm-up | 3.58 | 155.6 | render/thread/ms 883, render/busy/ms 636, stage/render/prepare_assets 546, pipelines/compile_ms 173, pipelines/front_ms 165, stage/render/prepare_views 157, render/acquire/ms 156, stage/render/render 151, render/present/ms 89.71, main/busy/ms 72.01, render/graph/ms 59.51, stage/main/post_update 46.52 |
| warm-up | 3.73 | 149.4 | pipelines/compile_ms 2260, pipelines/front_ms 1420, pipelines/driver_ms 840, render/thread/ms 289, pipelines/wait_ms 263, stage/render/render 206, render/busy/ms 200, render/graph/ms 117, stage/render/camera_driver 88.51, render/present/ms 87.63, pipelines/lock_ms 81.63, main/busy/ms 72.43 |
| warm-up | 3.83 | 105.5 | pipelines/wait_ms 7922, pipelines/compile_ms 3568, pipelines/front_ms 2615, pipelines/driver_ms 953, render/thread/ms 232, render/busy/ms 230, main/busy/ms 152, stage/render/prepare_assets 100, stage/render/render 83.40, render/graph/ms 80.29, stage/render/camera_driver 75.95, pipelines/lock_ms 75.28 |
| warm-up | 3.92 | 92.4 | pipelines/wait_ms 11.2 k, pipelines/compile_ms 2348, pipelines/front_ms 2181, render/thread/ms 180, render/busy/ms 179, pipelines/driver_ms 167, main/busy/ms 137, stage/render/prepare_assets 127, stage/main/update 58.60, stage/main/post_update 31.38, stage/render/prepare_flush 28.92, pipelines/lock_ms 25.81 |
| warm-up | 3.96 | 32.7 | pipelines/wait_ms 3625, pipelines/compile_ms 1142, pipelines/front_ms 1086, render/thread/ms 114, render/busy/ms 112, stage/render/prepare_assets 72.66, pipelines/driver_ms 56.30, main/busy/ms 56.23, stage/main/post_update 27.30, encode//sum 20.62, stage/render/render 15.03, terrain/update/ms 13.85 |
| warm-up | 4.00 | 43.6 | pipelines/wait_ms 622, pipelines/compile_ms 549, pipelines/front_ms 524, render/thread/ms 72.97, render/busy/ms 70.89, main/busy/ms 56.02, stage/main/post_update 27.82, pipelines/driver_ms 25.35, encode//sum 23.65, stage/render/prepare_resources 19.01, stage/render/render 17.85, render/graph/ms 15.54 |
| warm-up | 4.06 | 54.4 | pipelines/compile_ms 897, pipelines/front_ms 847, pipelines/wait_ms 649, render/thread/ms 95.71, render/busy/ms 93.45, main/busy/ms 75.60, pipelines/driver_ms 50.66, stage/main/post_update 48.19, stage/render/prepare_flush 23.21, stage/render/prepare_resources 21.09, stage/main/update 19.07, stage/render/prepare_write_phase_buffers 16.11 |
| warm-up | 4.10 | 46.3 | pipelines/compile_ms 1490, pipelines/front_ms 1406, pipelines/wait_ms 632, render/thread/ms 98.15, render/busy/ms 95.15, main/busy/ms 85.72, pipelines/driver_ms 83.26, stage/main/post_update 52.87, stage/main/update 24.73, stage/render/prepare_flush 22.79, stage/render/render 18.46, stage/render/prepare_resources 16.71 |
| warm-up | 4.20 | 95.1 | pipelines/compile_ms 2092, pipelines/front_ms 1979, pipelines/wait_ms 677, render/thread/ms 131, render/busy/ms 125, pipelines/driver_ms 113, main/busy/ms 67.00, stage/render/prepare_assets 62.05, stage/render/render 38.58, render/graph/ms 32.29, stage/main/post_update 31.57, stage/render/graph_submit 25.54 |
| warm-up | 4.25 | 54.1 | pipelines/compile_ms 1797, pipelines/front_ms 1716, pipelines/wait_ms 224, render/thread/ms 137, render/busy/ms 133, stage/render/prepare_assets 82.22, pipelines/driver_ms 81.22, main/busy/ms 47.59, stage/render/render 39.91, render/graph/ms 35.09, stage/render/graph_submit 29.04, stage/main/post_update 18.93 |
| warm-up | 4.35 | 103.0 | pipelines/compile_ms 1072, pipelines/front_ms 1044, render/thread/ms 153, render/busy/ms 151, stage/render/prepare_assets 124, main/busy/ms 39.47, pipelines/wait_ms 33.82, pipelines/driver_ms 28.83, stage/render/render 19.29, stage/main/post_update 19.20, render/graph/ms 17.19, stage/render/graph_submit 14.41 |
| warm-up | 4.43 | 45.4 | pipelines/compile_ms 984, pipelines/front_ms 957, render/thread/ms 67.89, render/busy/ms 40.77, main/busy/ms 27.67, pipelines/driver_ms 26.62, stage/render/prepare_views 25.24, render/acquire/ms 25.11, stage/render/render 20.55, encode//sum 18.81, render/graph/ms 18.00, stage/render/prepare_assets 17.24 |
| warm-up | 4.53 | 65.1 | pipelines/compile_ms 813, pipelines/front_ms 782, render/thread/ms 74.99, render/busy/ms 72.20, stage/render/render 40.12, main/busy/ms 37.87, render/graph/ms 36.50, pipelines/driver_ms 30.61, stage/render/camera_driver 22.92, stage/main/update 20.61, stage/render/prepare_assets 19.74, stage/render/graph_submit 12.36 |
| warm-up | 4.56 | 33.2 | pipelines/compile_ms 813, pipelines/front_ms 782, render/thread/ms 91.42, render/busy/ms 88.96, main/busy/ms 58.86, stage/main/update 39.86, stage/render/render 39.07, render/graph/ms 36.04, stage/render/prepare_assets 32.67, pipelines/driver_ms 30.61, stage/render/camera_driver 22.98, stage/render/graph_submit 11.79 |
| warm-up | 4.62 | 57.2 | render/thread/ms 79.94, render/busy/ms 78.09, stage/render/prepare_assets 53.82, main/busy/ms 53.17, stage/main/update 34.87, stage/render/render 13.70, render/graph/ms 11.53, stage/main/post_update 9.08, stage/render/graph_submit 8.22, encode//sum 7.69, encode/submit 5.49, stage/main/fixed_main_loop 3.67 |
| warm-up | 4.70 | 48.9 | pipelines/compile_ms 483, pipelines/front_ms 467, render/thread/ms 76.79, render/busy/ms 75.27, main/busy/ms 58.51, stage/main/update 26.49, stage/render/render 25.54, render/graph/ms 23.74, stage/render/prepare_assets 22.28, stage/main/post_update 15.92, pipelines/driver_ms 15.76, stage/render/prepare_flush 9.14 |
| warm-up | 4.75 | 44.6 | pipelines/compile_ms 1164, pipelines/front_ms 1130, render/thread/ms 87.89, render/busy/ms 86.28, pipelines/wait_ms 84.70, main/busy/ms 59.19, stage/render/prepare_assets 42.55, stage/main/update 39.40, pipelines/driver_ms 34.10, pipelines/lock_ms 23.94, stage/render/render 14.26, render/graph/ms 12.29 |
| warm-up | 4.80 | 59.2 | pipelines/compile_ms 1034, pipelines/front_ms 1004, pipelines/wait_ms 98.90, render/thread/ms 98.41, render/busy/ms 97.05, main/busy/ms 64.11, stage/render/prepare_assets 61.27, stage/main/update 30.20, pipelines/driver_ms 30.05, stage/main/post_update 23.18, pipelines/lock_ms 22.52, stage/render/prepare_flush 13.20 |
| warm-up | 4.88 | 44.9 | pipelines/compile_ms 1044, pipelines/front_ms 1010, pipelines/wait_ms 89.05, render/thread/ms 71.52, render/busy/ms 70.23, main/busy/ms 45.71, pipelines/driver_ms 34.07, stage/main/post_update 21.03, encode//sum 20.73, stage/render/prepare_assets 19.11, stage/main/update 15.54, stage/render/render 15.50 |
| warm-up | 4.97 | 92.6 | pipelines/compile_ms 1260, pipelines/front_ms 1218, pipelines/wait_ms 197, render/thread/ms 134, render/busy/ms 133, main/busy/ms 104, stage/main/update 53.83, pipelines/driver_ms 42.32, stage/render/prepare_resources 39.01, stage/render/prepare_assets 30.22, stage/main/post_update 29.56, stage/render/prepare_meshes 17.91 |
| warm-up | 5.03 | 62.9 | pipelines/compile_ms 1974, pipelines/front_ms 1913, pipelines/wait_ms 273, render/thread/ms 149, render/busy/ms 147, main/busy/ms 106, pipelines/driver_ms 61.66, stage/main/update 58.98, stage/render/prepare_assets 54.23, stage/render/prepare_resources 33.31, stage/main/post_update 20.78, stage/render/prepare_meshes 18.84 |
| warm-up | 5.08 | 48.9 | pipelines/compile_ms 2073, pipelines/front_ms 2023, pipelines/wait_ms 217, render/thread/ms 105, render/busy/ms 103, stage/render/prepare_assets 62.34, main/busy/ms 61.27, pipelines/driver_ms 50.17, stage/main/update 31.69, stage/render/render 19.72, render/graph/ms 17.27, stage/main/post_update 14.23 |
| warm-up | 5.35 | 39.7 | pipelines/compile_ms 408, pipelines/front_ms 397, render/thread/ms 50.17, render/busy/ms 48.78, main/busy/ms 41.77, stage/main/post_update 21.52, stage/render/prepare_resources 15.47, stage/render/render 15.28, render/graph/ms 13.39, stage/main/update 13.17, pipelines/wait_ms 12.97, pipelines/driver_ms 10.36 |
| warm-up | 5.41 | 33.7 | pipelines/compile_ms 409, pipelines/front_ms 393, render/thread/ms 55.84, render/busy/ms 54.08, main/busy/ms 45.21, stage/main/post_update 17.34, pipelines/driver_ms 15.96, stage/render/render 13.66, stage/main/update 13.47, render/graph/ms 11.44, stage/render/prepare_assets 10.59, stage/render/prepare_resources 10.16 |
| warm-up | 5.48 | 73.3 | pipelines/compile_ms 1245, pipelines/front_ms 1208, render/thread/ms 104, render/busy/ms 102, main/busy/ms 93.58, stage/render/prepare_write_phase_buffers 54.56, stage/main/update 52.21, pipelines/wait_ms 39.12, pipelines/driver_ms 36.79, stage/main/post_update 22.00, stage/render/render 13.19, stage/main/pre_update 13.04 |
| warm-up | 5.52 | 34.9 | pipelines/compile_ms 2271, pipelines/front_ms 2212, render/thread/ms 105, render/busy/ms 104, main/busy/ms 86.58, pipelines/driver_ms 59.01, stage/main/update 55.24, stage/render/prepare_write_phase_buffers 54.26, pipelines/wait_ms 35.61, stage/render/prepare_assets 27.02, stage/main/post_update 17.09, stage/render/render 11.57 |
| warm-up | 5.75 | 35.5 | render/thread/ms 61.47, render/busy/ms 59.76, main/busy/ms 33.73, stage/render/render 20.53, stage/render/prepare_assets 19.67, render/graph/ms 17.84, stage/main/update 14.14, encode//sum 13.89, stage/render/graph_submit 13.06, stage/main/post_update 12.95, encode/submit 8.52, stage/render/prepare_meshes 6.91 |
| warm-up | 5.83 | 50.8 | pipelines/compile_ms 146, pipelines/front_ms 142, render/thread/ms 71.43, render/busy/ms 70.40, stage/render/render 34.01, render/graph/ms 32.73, encode//sum 31.39, stage/render/graph_submit 30.10, stage/render/prepare_assets 28.39, main/busy/ms 27.25, encode/finish_wall 24.91, encode/pass:main_transparent_pass_3d/sum 23.63 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --set groundcover.render.reserve_worst_case=true --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=worst-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 88.2 W · 73 °C (max 76) · VRAM 7153 MiB · throttled 100% of samples

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
