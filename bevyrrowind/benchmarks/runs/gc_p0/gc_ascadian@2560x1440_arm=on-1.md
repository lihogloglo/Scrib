# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 358 frames over 15.0 s · **median 40.57 ms (25 fps)** · mean 41.91 · p95 48.92 · p99 49.56 (1% low 20 fps) · worst 49.91 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 307 frames over 22.9 s, worst 2493.9 ms
- hitches (> 101.4 ms): **0** measured, 14 in warm-up
- timed GPU passes: 35.52 ms of a 41.91 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 40.57 ms, limit 11 ms
- **FAIL** worst frame 49.91 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 11.434 | 11.434 | 11.518 | 1.00 | 0.082 |
| early prepass | 10.907 | 10.907 | 11.066 | 1.00 | 0.126 |
| range_unpacking | 2.915 | 2.915 | 3.787 | 1.00 | 0.002 |
| early_mesh_preprocessing | 2.543 | 2.543 | 3.858 | 1.00 | 0.003 |
| dlss_super_resolution | 2.066 | 2.066 | 2.093 | 1.00 | 0.196 |
| bin_unpacking | 0.756 | 0.756 | 1.094 | 1.00 | 0.014 |
| sun_visibility | 0.701 | 0.701 | 0.705 | 1.00 | 0.006 |
| gpu_instance_materialization | 0.636 | 0.636 | 0.646 | 1.00 | 0.008 |
| uniform_allocation | 0.469 | 0.469 | 0.598 | 1.00 | 0.021 |
| atmosphere_luts | 0.359 | 0.359 | 0.365 | 1.00 | 0.003 |
| bloom | 0.325 | 0.325 | 0.343 | 1.00 | 0.059 |
| water_velocity | 0.307 | 0.307 | 0.335 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.306 | 0.306 | 0.310 | 1.00 | 0.001 |
| clustering | 0.288 | 0.288 | 0.319 | 1.00 | 0.019 |
| water_sspr | 0.201 | 0.201 | 0.202 | 1.00 | 0.002 |
| ssao | 0.190 | 0.190 | 0.200 | 1.00 | 0.007 |
| water_fft | 0.136 | 0.136 | 0.140 | 1.00 | 0.002 |
| upscaling | 0.131 | 0.131 | 0.134 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.001 |
| tonemapping | 0.089 | 0.089 | 0.095 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.077 | 0.084 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.075 | 0.075 | 0.076 | 1.00 | 0.219 |
| render_sky | 0.063 | 0.063 | 0.065 | 1.00 | 0.001 |
| point_shadow_atlas | 0.057 | 0.113 | 0.134 | 0.50 | 0.127 |
| clouds_trace | 0.055 | 0.055 | 0.058 | 1.00 | 0.001 |
| per_view_shadow_pass | 0.053 | 0.053 | 0.107 | 1.00 | 0.006 |
| lightprobe_irradiance_map | 0.052 | 0.410 | 0.412 | 0.13 | 0.001 |
| clouds_resolve | 0.046 | 0.046 | 0.047 | 1.00 | 0.001 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.029 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.219 | 0.230 | 0.13 | 0.003 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.001 |
| clouds_shadow | 0.022 | 0.022 | 0.024 | 1.00 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.012 | 1.00 | 0.003 |
| lightprobe_copy | 0.009 | 0.070 | 0.071 | 0.13 | 0.004 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.016 | 0.018 | 0.50 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.005 | 1.00 | 0.001 |
| lightprobe_downsampling_first_pass | 0.004 | 0.032 | 0.034 | 0.13 | 0.001 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.006 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.006 | 0.007 | 0.50 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.01 | 0.008 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 40.589 | 47.414 | 48.51 | 14530.8 |
| stage/render/prepare_views | 30.524 | 37.743 | 38.45 | 10927.5 |
| render/acquire/ms | 30.454 | 37.666 | 38.37 | 10902.5 |
| render/busy/ms | 9.722 | 11.236 | 12.27 | 3480.3 |
| main/busy/ms | 8.051 | 8.834 | 10.37 | 2882.2 |
| stage/render/render | 5.993 | 7.239 | 8.18 | 2145.5 |
| encode//sum | 5.992 | 8.520 | 10.87 | 2145.2 |
| render/graph/ms | 5.305 | 6.455 | 7.41 | 1899.1 |
| stage/render/graph_submit | 3.947 | 5.074 | 6.01 | 1412.9 |
| stage/main/post_update | 3.834 | 4.336 | 6.33 | 1372.7 |
| encode/submit | 2.346 | 3.040 | 4.41 | 840.0 |
| encode/finish_wall | 1.620 | 2.222 | 2.75 | 580.1 |
| stage/main/update | 1.617 | 1.848 | 2.48 | 578.9 |
| stage/main/fixed_main_loop | 1.289 | 1.574 | 2.19 | 461.6 |
| stage/render/camera_driver | 1.257 | 1.468 | 1.75 | 449.9 |
| encode/pass:early prepass/sum | 1.110 | 1.477 | 2.06 | 397.4 |
| encode/pass:main_opaque_pass_3d/sum | 1.068 | 1.502 | 1.95 | 382.2 |
| encode/pass:shadow_directional_light/sum | 1.057 | 1.723 | 2.66 | 378.6 |
| stage/render/prepare_assets | 0.755 | 0.915 | 2.28 | 270.4 |
| stage/render/prepare_write_phase_buffers | 0.538 | 0.765 | 1.24 | 192.5 |
| stage/render/prepare_meshes | 0.517 | 0.643 | 1.17 | 185.0 |
| stage/render/prepare_resources | 0.515 | 0.588 | 0.69 | 184.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.507 | 0.720 | 0.89 | 181.4 |
| stage/main/first | 0.451 | 0.601 | 0.91 | 161.4 |
| bigspace/total/ms | 0.434 | 0.532 | 0.67 | 155.2 |
| render/present/ms | 0.413 | 0.529 | 0.85 | 148.0 |
| stage/main/pre_update | 0.409 | 0.508 | 0.96 | 146.6 |
| stage/render/prepare_batch_phases | 0.360 | 0.492 | 0.65 | 129.0 |
| stage/render/prepare_flush | 0.331 | 0.486 | 0.70 | 118.5 |
| controller/feed/ms | 0.307 | 0.374 | 0.50 | 109.8 |
| stage/render/prepare_bind_groups | 0.265 | 0.416 | 0.58 | 95.0 |
| stage/main/last | 0.264 | 0.309 | 0.37 | 94.5 |
| stage/render/queue | 0.241 | 0.291 | 0.43 | 86.4 |
| stage/render/extract_commands | 0.214 | 0.272 | 0.33 | 76.7 |
| bigspace/low_precision/ms | 0.192 | 0.243 | 0.35 | 68.7 |
| stage/render/create_views | 0.189 | 0.233 | 0.58 | 67.7 |
| anim/evaluate_cpu/ms | 0.182 | 0.212 | 0.29 | 65.2 |
| stage/render/specialize | 0.179 | 0.221 | 0.28 | 64.2 |
| controller/move/ms | 0.179 | 0.254 | 0.33 | 64.1 |
| stream/drive/ms | 0.172 | 0.203 | 0.30 | 61.7 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.166 | 0.446 | 1.04 | 59.3 |
| stream/tick/ms | 0.143 | 0.168 | 0.27 | 51.3 |
| anim/evaluate/ms | 0.142 | 0.168 | 0.21 | 50.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.137 | 0.214 | 0.29 | 48.9 |
| physics/step/ms | 0.107 | 0.132 | 0.18 | 38.2 |
| bigspace/high_precision/ms | 0.095 | 0.115 | 0.23 | 33.9 |
| encode/resolve_diagnostics/sum | 0.066 | 0.103 | 0.15 | 23.5 |
| controller/write/ms | 0.062 | 0.075 | 0.11 | 22.2 |
| stage/render/phase_sort | 0.056 | 0.077 | 0.15 | 20.1 |
| stage/render/prepare_collect_phase_buffers | 0.037 | 0.059 | 0.11 | 13.3 |
| terrain/update/ms | 0.035 | 0.046 | 0.07 | 12.6 |
| controller/avoid/ms | 0.031 | 0.041 | 0.05 | 11.0 |
| stage/render/cleanup | 0.028 | 0.036 | 0.21 | 10.0 |
| encode/camera_driver_span_begin/sum | 0.026 | 0.048 | 0.09 | 9.4 |
| encode/camera_driver_span_end/sum | 0.023 | 0.038 | 0.08 | 8.3 |
| weather/map/ms | 0.015 | 0.020 | 0.03 | 5.3 |
| anim/apply/ms | 0.012 | 0.014 | 0.04 | 4.2 |
| weather/eval/ms | 0.008 | 0.011 | 0.03 | 3.0 |
| script/update/ms | 0.008 | 0.010 | 0.02 | 2.8 |
| script/input/ms | 0.005 | 0.006 | 0.01 | 1.7 |
| far/select/ms | 0.004 | 0.004 | 0.01 | 1.3 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 1.2 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.02 | 0.8 |
| character/finish/ms | 0.001 | 0.002 | 0.00 | 0.5 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 0.5 |
| stream/promote/ms | 0.001 | 0.002 | 0.00 | 0.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 0.4 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.1 |
| stage/render/prepare | 0.000 | 0.002 | 0.00 | 0.1 |
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
| anim/a2/count | count | 4 | 4 | 4 |
| anim/a3/count | count | 446 | 446 | 446 |
| anim/characters/count | count | 450 | 450 | 450 |
| anim/evaluated/count | count | 26 | 26 | 26 |
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
| crowd/joints_extracted/count | count | 782 | 1154 | 624 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.75 | 1.99 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.08 | 0.02 |
| encode/finish_critical | ms | 1.26 | 2.66 | 1.23 |
| encode/pass:early prepass/max | ms | 1.11 | 2.06 | 1.19 |
| encode/pass:main_opaque_pass_3d/max | ms | 1.05 | 1.95 | 1.19 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.13 | 0.29 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.50 | 0.89 | 0.35 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.29 | 1.04 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 1.02 | 2.66 | 1.23 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.15 | 0.07 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 35.53 | 39.80 | 38.36 |
| gpu/timed/ms | ms | 35.53 | 39.80 | 38.36 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1347 | 1347 | 1347 |
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
| phases/opaque3d/pipelines | count | 23 | 23 | 23 |
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
| phases/shadow/batch_sets | count | 52 | 121 | 105 |
| phases/shadow/bins | count | 2180 | 2210 | 2192 |
| phases/shadow/instance_batches | count | 9 | 27 | 9 |
| phases/shadow/instance_draws | count | 7317 | 22.0 k | 7317 |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| phases/shadow/items | count | 5360 | 5390 | 5371 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 53 | 56 | 53 |
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
| preprocess/bind_groups | count | 7 | 67 | 9 |
| preprocess/bind_groups_created | count | 2 | 60 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 141 | 141 | 141 |
| process/cpu_usage | % | 13.19 | 22.49 | 10.58 |
| process/mem_usage | GiB | 4.26 | 4.27 | 4.27 |
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
| shadow_atlas/mover_draws | count | 2 | 4 | 2 |
| shadow_atlas/movers | count | 85 | 100 | 100 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 1 | 4 | 4 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 1 | 4 | 4 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 6 | 7 | 6 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 6 | 7 | 6 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 19 | 26 | 16 |
| skin/blocks_skipped/count | count | 191 | 196 | 194 |
| skin/blocks_walked/count | count | 19 | 27 | 16 |
| skin/joint_bytes_written/bytes | bytes | 61.4 k | 76.8 k | 51.2 k |
| skin/joints_written/count | count | 960 | 1200 | 800 |
| skin/upload_writes/count | count | 8 | 11 | 9 |
| started_load_count |  | 5839 | 5841 | 5841 |
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
| stream/latency/s | s | 0.18 | 0.18 | 0.18 |
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
| system/cpu_usage | % | 18.20 | 27.00 | 16.67 |
| system/mem_usage | % | 62.76 | 62.82 | 62.77 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 151 | 220 | 204 |
| views/all/bins | count | 5648 | 5678 | 5660 |
| views/all/count | count | 10 | 11 | 10 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 22.0 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 112 | 115 | 112 |
| views/all/pipelines_distinct | count | 80 | 80 | 80 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.50 M |
| views/cascade/batch_sets | count | 14 | 60 | 14 |
| views/cascade/bins | count | 2043 | 2043 | 2043 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 9 |
| views/cascade/instance_draws | count | 7317 | 22.0 k | 7317 |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| views/cascade/items | count | 5222 | 5223 | 5222 |
| views/cascade/pipelines | count | 43 | 43 | 43 |
| views/cascade/work_items | count | 83.0 k | 253.8 k | 83.0 k |
| views/lamp_face/batch_sets | count | 29 | 91 | 91 |
| views/lamp_face/bins | count | 137 | 167 | 149 |
| views/lamp_face/count | count | 6 | 7 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 137 | 167 | 149 |
| views/lamp_face/pipelines | count | 10 | 13 | 10 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 29 | 91 | 91 |
| views/main/batch_sets | count | 99 | 99 | 99 |
| views/main/bins | count | 3468 | 3468 | 3468 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 14.7 k | 14.7 k | 14.7 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6840 | 6840 | 6840 |
| views/main/pipelines | count | 59 | 59 | 59 |
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
| anim/bones_written/count | count | 56.07 | 120 | 20.1 k |
| anim/events/count | count | 0.19 | 2 | 68 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 8 | 8 | 2864 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| encode//count | count | 28.25 | 30 | 10.1 k |
| encode/bind_walk/init_textures | count | 9343 | 13.1 k | 3.34 M |
| encode/bind_walk/init_textures_skipped | count | 62.2 k | 71.3 k | 22.26 M |
| encode/bind_walk/inits | count | 805 | 917 | 288.3 k |
| encode/bind_walk/inits_skipped | count | 173 | 186 | 62.0 k |
| encode/bind_walk/merge_textures | count | 9281 | 13.0 k | 3.32 M |
| encode/bind_walk/merge_textures_skipped | count | 18.4 k | 27.6 k | 6.60 M |
| encode/bind_walk/merges | count | 158 | 183 | 56.7 k |
| encode/bind_walk/merges_skipped | count | 159 | 179 | 56.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 358 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 358 |
| encode/pass:early prepass/count | count | 1 | 1 | 358 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 358 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 358 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 358 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 179 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 358 |
| encode/queue/command_buffers | count | 38.75 | 41 | 13.9 k |
| encode/queue/submits | count | 2 | 2 | 716 |
| encode/queue/write_buffer/bytes | bytes | 2.96 M | 3.51 M | 1.06 G |
| encode/queue/write_buffer/calls | count | 286 | 358 | 102.5 k |
| encode/queue/write_buffer_with/bytes | bytes | 53.9 k | 53.9 k | 19.29 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 3222 |
| encode/queue/write_texture/bytes | bytes | 73.6 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.01 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 358 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 2.51 | 3 | 900 |
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

## Hitches (> 101.4 ms; slowest 14, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2493.9 | main/busy/ms 2459, stage/main/post_update 2411, stage/main/update 33.70, stage/main/pre_update 10.40, surface/advance_cpu_ms 1.14, stage/main/last 1.04, stage/main/first 0.64, stream/drive/ms 0.56, surface/cell_upload_ms 0.45, stage/main/fixed_main_loop 0.39, stream/tick/ms 0.17, bigspace/total/ms 0.08 |
| warm-up | 3.31 | 818.4 | main/busy/ms 2513, stage/main/post_update 2446, render/thread/ms 756, render/busy/ms 603, stage/render/prepare_assets 567, stage/render/prepare_views 149, render/acquire/ms 148, stage/main/update 40.71, stage/render/render 23.78, render/graph/ms 18.25, stage/render/camera_driver 16.33, stage/main/pre_update 12.06 |
| warm-up | 3.56 | 176.9 | pipelines/compile_ms 1031, pipelines/front_ms 959, pipelines/wait_ms 234, render/thread/ms 227, render/busy/ms 212, main/busy/ms 87.27, stage/render/prepare_assets 81.90, stage/render/render 74.01, pipelines/driver_ms 72.10, stage/main/post_update 63.66, stage/render/prepare_resources 60.31, render/graph/ms 58.43 |
| warm-up | 3.67 | 113.1 | pipelines/wait_ms 2889, pipelines/compile_ms 2542, pipelines/front_ms 2347, render/thread/ms 257, render/busy/ms 243, pipelines/driver_ms 195, main/busy/ms 122, stage/render/prepare_assets 83.50, stage/render/prepare_resources 81.10, stage/main/update 65.84, stage/render/render 42.89, stage/main/post_update 35.57 |
| warm-up | 3.82 | 150.2 | pipelines/wait_ms 8977, pipelines/compile_ms 3566, pipelines/front_ms 3340, render/thread/ms 243, render/busy/ms 241, pipelines/driver_ms 226, main/busy/ms 145, stage/render/prepare_assets 138, stage/main/update 66.29, stage/main/post_update 44.69, stage/main/fixed_main_loop 29.05, stage/render/prepare_resources 28.47 |
| warm-up | 4.61 | 114.6 | pipelines/compile_ms 1157, pipelines/front_ms 1119, render/thread/ms 138, render/busy/ms 137, main/busy/ms 124, stage/main/post_update 104, pipelines/wait_ms 73.78, stage/render/prepare_assets 49.47, pipelines/driver_ms 38.11, stage/render/prepare_write_phase_buffers 37.53, bigspace/total/ms 33.65, stage/render/prepare_meshes 14.24 |
| warm-up | 4.84 | 103.0 | pipelines/compile_ms 1902, pipelines/front_ms 1842, pipelines/wait_ms 236, render/thread/ms 154, render/busy/ms 151, main/busy/ms 123, stage/render/prepare_assets 71.80, pipelines/driver_ms 59.77, stage/main/post_update 56.83, stage/main/update 47.82, stage/render/prepare_resources 30.18, stage/render/render 24.31 |
| warm-up | 7.44 | 101.7 | render/thread/ms 167, stage/render/prepare_views 110, render/acquire/ms 110, render/busy/ms 55.94, main/busy/ms 45.54, stage/render/prepare_assets 25.06, stage/main/update 20.98, stage/render/render 12.40, render/graph/ms 10.92, stage/main/post_update 10.51, stage/render/prepare_meshes 9.71, stage/main/fixed_main_loop 8.95 |
| warm-up | 7.92 | 149.2 | render/thread/ms 227, encode//sum 164, stage/render/prepare_views 118, render/acquire/ms 118, render/busy/ms 108, stage/render/render 89.65, render/graph/ms 88.26, stage/render/graph_submit 85.39, encode/finish_wall 80.18, encode/pass:early prepass/sum 78.84, main/busy/ms 34.96, stage/main/update 13.37 |
| warm-up | 9.02 | 165.2 | render/thread/ms 240, render/busy/ms 178, stage/render/prepare_assets 159, stage/render/prepare_views 61.64, render/acquire/ms 61.54, main/busy/ms 29.59, stage/main/update 12.13, stage/render/render 9.06, encode//sum 8.65, stage/main/post_update 8.38, render/graph/ms 7.83, stage/render/graph_submit 5.50 |
| warm-up | 9.69 | 237.8 | encode//sum 1384, render/thread/ms 321, render/busy/ms 177, stage/render/render 167, render/graph/ms 166, stage/render/graph_submit 163, encode/finish_wall 157, encode/pass:main_transmissive_pass_3d/sum 145, stage/render/prepare_views 143, render/acquire/ms 143, main/busy/ms 21.43, encode/pass:main_transparent_pass_3d/sum 9.22 |
| warm-up | 10.39 | 101.9 | render/thread/ms 189, stage/render/prepare_views 167, render/acquire/ms 167, main/busy/ms 23.23, render/busy/ms 21.36, stage/render/render 11.58, render/graph/ms 10.28, encode//sum 9.41, stage/main/post_update 8.78, stage/render/graph_submit 7.49, stage/main/update 7.40, encode/submit 4.51 |
| warm-up | 12.19 | 102.8 | render/thread/ms 185, stage/render/prepare_views 159, render/acquire/ms 159, main/busy/ms 25.28, render/busy/ms 25.24, encode//sum 13.57, stage/render/render 12.44, render/graph/ms 11.09, stage/render/graph_submit 8.56, stage/main/post_update 8.33, stage/render/prepare_assets 6.09, stage/main/update 5.97 |
| warm-up | 18.56 | 102.2 | render/thread/ms 184, stage/render/prepare_views 160, render/acquire/ms 160, main/busy/ms 23.23, render/busy/ms 22.83, stage/render/render 12.39, encode//sum 11.58, render/graph/ms 11.05, stage/main/post_update 9.68, stage/render/graph_submit 8.44, encode/submit 5.32, stage/main/update 4.99 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0\gc_ascadian@2560x1440_arm=on-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **99%** (min 99%) · 2625 MHz (min 2625) · memory 8101 MHz · 46.5 W · 65 °C (max 67) · VRAM 7547 MiB · throttled 100% of samples

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
