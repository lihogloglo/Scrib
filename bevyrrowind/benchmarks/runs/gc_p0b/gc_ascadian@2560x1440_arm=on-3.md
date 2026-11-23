# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1291 frames over 15.0 s · **median 11.56 ms (87 fps)** · mean 11.63 · p95 13.20 · p99 14.16 (1% low 71 fps) · worst 18.43 ms · 2 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 306 frames over 9.2 s, worst 2449.3 ms
- hitches (> 28.9 ms): **0** measured, 38 in warm-up
- timed GPU passes: 9.65 ms of a 11.63 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.56 ms, limit 11 ms
- pass worst frame 18.43 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.273 | 2.301 | 2.336 | 0.99 | 0.101 |
| dlss_super_resolution | 2.044 | 2.070 | 2.093 | 0.99 | 0.181 |
| early prepass | 1.316 | 1.333 | 1.348 | 0.99 | 0.116 |
| sun_visibility | 0.680 | 0.688 | 0.705 | 0.99 | 0.007 |
| atmosphere_luts | 0.355 | 0.359 | 0.364 | 0.99 | 0.003 |
| water_velocity | 0.326 | 0.330 | 0.333 | 0.99 | 0.002 |
| bloom | 0.320 | 0.324 | 0.340 | 0.99 | 0.062 |
| atmos_froxel_inject | 0.308 | 0.312 | 0.311 | 0.99 | 0.002 |
| water_sspr | 0.210 | 0.213 | 0.206 | 0.99 | 0.002 |
| gpu_instance_materialization | 0.202 | 0.204 | 0.249 | 0.99 | 0.008 |
| ssao | 0.186 | 0.189 | 0.197 | 0.99 | 0.008 |
| uniform_allocation | 0.138 | 0.140 | 0.183 | 0.99 | 0.028 |
| water_fft | 0.134 | 0.136 | 0.138 | 0.99 | 0.003 |
| upscaling | 0.130 | 0.131 | 0.136 | 0.99 | 0.004 |
| clouds_composite | 0.107 | 0.109 | 0.111 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.101 | 0.102 | 0.136 | 0.99 | 0.005 |
| bin_unpacking | 0.098 | 0.099 | 0.131 | 0.99 | 0.018 |
| tonemapping | 0.088 | 0.089 | 0.094 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.080 | 0.081 | 0.086 | 0.99 | 0.009 |
| clustering | 0.069 | 0.070 | 0.081 | 0.99 | 0.020 |
| render_sky | 0.063 | 0.064 | 0.065 | 0.99 | 0.002 |
| clouds_trace | 0.057 | 0.058 | 0.063 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.12 | 0.001 |
| clouds_resolve | 0.047 | 0.048 | 0.049 | 0.99 | 0.001 |
| point_shadow_atlas | 0.036 | 0.073 | 0.080 | 0.50 | 0.129 |
| range_unpacking | 0.035 | 0.035 | 0.044 | 0.99 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.230 | 0.12 | 0.004 |
| per_view_shadow_pass | 0.026 | 0.026 | 0.049 | 0.99 | 0.007 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| clouds_shadow | 0.021 | 0.021 | 0.023 | 0.99 | 0.004 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.028 | 0.99 | 0.241 |
| clouds_fog_light | 0.011 | 0.011 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.99 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.015 | 0.016 | 0.50 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.006 | 0.99 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.007 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.411 | 11.962 | 19.39 | 13440.3 |
| render/busy/ms | 9.196 | 10.742 | 17.38 | 11872.3 |
| main/busy/ms | 8.524 | 9.653 | 15.47 | 11003.9 |
| stage/render/render | 5.755 | 7.004 | 10.95 | 7429.3 |
| encode//sum | 5.249 | 7.344 | 11.56 | 6776.1 |
| render/graph/ms | 5.140 | 6.329 | 10.14 | 6635.6 |
| stage/main/post_update | 4.964 | 6.018 | 7.43 | 6408.0 |
| stage/render/graph_submit | 3.731 | 4.794 | 8.57 | 4816.3 |
| encode/submit | 2.189 | 2.755 | 4.59 | 2825.9 |
| stage/main/update | 1.802 | 2.218 | 2.54 | 2326.2 |
| pipelines/compile_ms | 1.703 | 0.000 | 134.15 | 2199.1 |
| pipelines/front_ms | 1.653 | 0.000 | 130.41 | 2134.1 |
| encode/finish_wall | 1.577 | 2.272 | 4.01 | 2035.5 |
| stage/render/camera_driver | 1.332 | 1.551 | 2.83 | 1719.4 |
| encode/pass:shadow_directional_light/sum | 1.033 | 1.763 | 3.03 | 1333.8 |
| encode/pass:early prepass/sum | 0.970 | 1.359 | 2.03 | 1252.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.929 | 1.331 | 1.97 | 1200.0 |
| stage/render/prepare_views | 0.899 | 2.529 | 3.96 | 1160.2 |
| render/acquire/ms | 0.853 | 2.475 | 3.92 | 1101.5 |
| stage/render/prepare_assets | 0.599 | 0.721 | 1.77 | 773.3 |
| bigspace/total/ms | 0.565 | 1.206 | 3.05 | 729.2 |
| stage/render/prepare_write_phase_buffers | 0.564 | 0.817 | 1.24 | 727.8 |
| stage/render/prepare_meshes | 0.548 | 0.753 | 1.26 | 707.2 |
| stage/main/fixed_main_loop | 0.524 | 0.819 | 1.50 | 676.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.458 | 0.681 | 1.38 | 591.8 |
| stage/render/prepare_resources | 0.440 | 0.525 | 0.81 | 567.7 |
| stage/main/pre_update | 0.405 | 0.514 | 1.40 | 523.2 |
| stage/main/first | 0.364 | 0.437 | 1.96 | 469.6 |
| render/present/ms | 0.361 | 0.435 | 1.88 | 466.5 |
| controller/feed/ms | 0.334 | 0.421 | 0.70 | 431.0 |
| stage/render/prepare_batch_phases | 0.327 | 0.464 | 0.64 | 421.7 |
| stage/render/prepare_flush | 0.317 | 0.523 | 0.87 | 408.9 |
| stage/render/prepare_bind_groups | 0.291 | 0.468 | 0.72 | 375.7 |
| stage/main/last | 0.279 | 0.344 | 6.59 | 360.1 |
| bigspace/low_precision/ms | 0.263 | 0.731 | 2.20 | 340.1 |
| stage/render/extract_commands | 0.202 | 0.234 | 0.58 | 261.0 |
| stage/render/create_views | 0.191 | 0.234 | 0.79 | 246.3 |
| stream/drive/ms | 0.187 | 0.274 | 0.62 | 240.9 |
| stage/render/specialize | 0.180 | 0.251 | 0.67 | 232.7 |
| anim/evaluate_cpu/ms | 0.177 | 0.225 | 0.62 | 228.5 |
| anim/evaluate/ms | 0.172 | 0.354 | 1.62 | 222.3 |
| stream/tick/ms | 0.155 | 0.234 | 0.59 | 200.3 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.142 | 0.392 | 0.90 | 183.6 |
| stage/render/queue | 0.138 | 0.177 | 0.23 | 177.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.136 | 0.218 | 0.68 | 175.4 |
| bigspace/high_precision/ms | 0.133 | 0.361 | 1.92 | 171.2 |
| controller/move/ms | 0.066 | 0.117 | 0.56 | 85.2 |
| controller/write/ms | 0.063 | 0.079 | 0.45 | 81.6 |
| encode/resolve_diagnostics/sum | 0.059 | 0.098 | 0.43 | 76.4 |
| pipelines/driver_ms | 0.050 | 0.000 | 4.26 | 65.0 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.072 | 0.20 | 58.6 |
| physics/step/ms | 0.044 | 0.073 | 0.47 | 56.3 |
| terrain/update/ms | 0.035 | 0.050 | 0.11 | 45.7 |
| stage/render/phase_sort | 0.032 | 0.050 | 0.10 | 41.3 |
| stage/render/cleanup | 0.024 | 0.032 | 0.28 | 30.6 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.040 | 0.08 | 30.5 |
| encode/camera_driver_span_end/sum | 0.021 | 0.038 | 0.22 | 27.3 |
| weather/map/ms | 0.016 | 0.021 | 0.06 | 20.1 |
| controller/avoid/ms | 0.010 | 0.018 | 0.46 | 13.2 |
| anim/apply/ms | 0.009 | 0.014 | 0.04 | 11.6 |
| weather/eval/ms | 0.009 | 0.012 | 0.08 | 11.4 |
| script/update/ms | 0.008 | 0.011 | 0.04 | 10.1 |
| script/input/ms | 0.006 | 0.008 | 0.03 | 7.3 |
| far/select/ms | 0.004 | 0.005 | 0.03 | 5.2 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.2 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.02 | 2.4 |
| stream/assets_ready/ms | 0.002 | 0.002 | 0.36 | 2.2 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.6 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 1.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 0.8 |
| terrain/select/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| anim/a2/count | count | 2 | 3 | 2 |
| anim/a3/count | count | 448 | 448 | 448 |
| anim/characters/count | count | 450 | 450 | 450 |
| anim/evaluated/count | count | 26 | 28 | 26 |
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
| crowd/joints_extracted/count | count | 744 | 1167 | 982 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.64 | 2.01 | 1.18 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.08 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.22 | 0.02 |
| encode/finish_critical | ms | 1.14 | 3.03 | 1.49 |
| encode/pass:early prepass/max | ms | 0.95 | 2.03 | 1.38 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.91 | 1.97 | 1.49 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.13 | 0.68 | 0.19 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.43 | 1.38 | 0.68 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.28 | 0.90 | 0.33 |
| encode/pass:shadow_directional_light/max | ms | 0.93 | 3.03 | 0.55 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.43 | 0.07 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 10.41 | 17.38 | 10.61 |
| gpu/timed/ms | ms | 10.40 | 11.75 | 10.61 |
| groundcover/batches/count | count | 10 | 10 | 10 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1353 | 1353 | 1353 |
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
| phases/alpha_mask3d/bins | count | 359 | 360 | 359 |
| phases/alpha_mask3d/instance_batches | count | 8 | 8 | 8 |
| phases/alpha_mask3d/instance_draws | count | 565 | 565 | 565 |
| phases/alpha_mask3d/instance_slots | count | 16.8 k | 16.8 k | 16.8 k |
| phases/alpha_mask3d/items | count | 564 | 565 | 564 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 15 | 15 | 15 |
| phases/alpha_mask3d_prepass/bins | count | 359 | 360 | 359 |
| phases/alpha_mask3d_prepass/instance_batches | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/instance_draws | count | 565 | 565 | 565 |
| phases/alpha_mask3d_prepass/instance_slots | count | 16.8 k | 16.8 k | 16.8 k |
| phases/alpha_mask3d_prepass/items | count | 564 | 565 | 564 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 27 | 27 |
| phases/opaque3d/bins | count | 1350 | 1351 | 1350 |
| phases/opaque3d/instance_batches | count | 6 | 6 | 6 |
| phases/opaque3d/instance_draws | count | 6757 | 6757 | 6757 |
| phases/opaque3d/instance_slots | count | 127.0 k | 127.0 k | 127.0 k |
| phases/opaque3d/items | count | 2832 | 2833 | 2832 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 24 | 24 |
| phases/opaque3d_prepass/bins | count | 1350 | 1351 | 1350 |
| phases/opaque3d_prepass/instance_batches | count | 6 | 6 | 6 |
| phases/opaque3d_prepass/instance_draws | count | 6757 | 6757 | 6757 |
| phases/opaque3d_prepass/instance_slots | count | 127.0 k | 127.0 k | 127.0 k |
| phases/opaque3d_prepass/items | count | 2832 | 2833 | 2832 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 12 | 115 | 88 |
| phases/shadow/bins | count | 2171 | 2189 | 2151 |
| phases/shadow/instance_batches | count | 7 | 21 | 7 |
| phases/shadow/instance_draws | count | 7315 | 21.9 k | 7315 |
| phases/shadow/instance_slots | count | 50.1 k | 150.3 k | 50.1 k |
| phases/shadow/items | count | 5346 | 5368 | 5323 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 50 | 51 | 50 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 350 | 350 | 350 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 27 | 0 |
| preprocess/bind_groups | count | 9 | 69 | 9 |
| preprocess/bind_groups_created | count | 4 | 62 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 27.82 | 43.75 | 25.96 |
| process/mem_usage | GiB | 3.14 | 3.21 | 3.20 |
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
| shadow_atlas/movers | count | 85 | 100 | 85 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 4 | 4 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 0 | 4 | 4 |
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
| skin/blocks_changed/count | count | 18 | 27 | 22 |
| skin/blocks_skipped/count | count | 191 | 200 | 186 |
| skin/blocks_walked/count | count | 19 | 27 | 24 |
| skin/joint_bytes_written/bytes | bytes | 57.6 k | 80.6 k | 59.4 k |
| skin/joints_written/count | count | 900 | 1260 | 928 |
| skin/upload_writes/count | count | 10 | 13 | 9 |
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
| system/cpu_usage | % | 33.45 | 42.97 | 31.43 |
| system/mem_usage | % | 51.21 | 51.44 | 51.44 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 93 | 196 | 169 |
| views/all/bins | count | 5589 | 5611 | 5569 |
| views/all/count | count | 10 | 10 | 10 |
| views/all/instance_batches | count | 35 | 49 | 35 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 22.0 k |
| views/all/instance_slots | count | 337.7 k | 438.0 k | 337.7 k |
| views/all/items | count | 12.1 k | 12.2 k | 12.1 k |
| views/all/pipelines | count | 103 | 104 | 103 |
| views/all/pipelines_distinct | count | 73 | 73 | 73 |
| views/all/work_items | count | 345.0 k | 450.4 k | 345.1 k |
| views/cascade/batch_sets | count | 12 | 54 | 12 |
| views/cascade/bins | count | 2037 | 2037 | 2014 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 7 | 21 | 7 |
| views/cascade/instance_draws | count | 7315 | 21.9 k | 7315 |
| views/cascade/instance_slots | count | 50.1 k | 150.3 k | 50.1 k |
| views/cascade/items | count | 5209 | 5216 | 5186 |
| views/cascade/pipelines | count | 40 | 40 | 40 |
| views/cascade/work_items | count | 50.3 k | 155.5 k | 50.3 k |
| views/lamp_face/batch_sets | count | 0 | 91 | 76 |
| views/lamp_face/bins | count | 137 | 152 | 137 |
| views/lamp_face/count | count | 6 | 6 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 137 | 152 | 137 |
| views/lamp_face/pipelines | count | 10 | 11 | 10 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 91 | 76 |
| views/main/batch_sets | count | 81 | 81 | 81 |
| views/main/bins | count | 3418 | 3422 | 3418 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 28 | 28 | 28 |
| views/main/instance_draws | count | 14.6 k | 14.6 k | 14.6 k |
| views/main/instance_slots | count | 287.6 k | 287.6 k | 287.6 k |
| views/main/items | count | 6792 | 6796 | 6792 |
| views/main/pipelines | count | 53 | 53 | 53 |
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
| anim/bones_written/count | count | 22.69 | 78 | 29.3 k |
| anim/events/count | count | 0.04 | 1 | 51 |
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
| encode//count | count | 28.25 | 56 | 36.5 k |
| encode/bind_walk/init_textures | count | 9233 | 20.9 k | 11.92 M |
| encode/bind_walk/init_textures_skipped | count | 53.1 k | 113.5 k | 68.52 M |
| encode/bind_walk/inits | count | 736 | 1529 | 950.8 k |
| encode/bind_walk/inits_skipped | count | 165 | 336 | 213.5 k |
| encode/bind_walk/merge_textures | count | 9171 | 20.8 k | 11.84 M |
| encode/bind_walk/merge_textures_skipped | count | 19.1 k | 45.4 k | 24.60 M |
| encode/bind_walk/merges | count | 140 | 292 | 180.1 k |
| encode/bind_walk/merges_skipped | count | 155 | 322 | 200.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1291 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1291 |
| encode/pass:early prepass/count | count | 1 | 2 | 1291 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1291 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1291 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1291 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 645 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1291 |
| encode/queue/command_buffers | count | 38.75 | 77 | 50.0 k |
| encode/queue/submits | count | 2 | 4 | 2582 |
| encode/queue/write_buffer/bytes | bytes | 2.75 M | 5.68 M | 3.55 G |
| encode/queue/write_buffer/calls | count | 274 | 593 | 353.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 58.4 k | 116.7 k | 75.35 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.4 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1291 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 32 |
| pipelines/created | count | 0.02 | 1 | 32 |
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

## Hitches (> 28.9 ms; slowest 38, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2449.3 | main/busy/ms 2417, stage/main/post_update 2368, stage/main/update 34.07, stage/main/pre_update 9.63, stage/main/last 3.24, surface/advance_cpu_ms 0.95, stream/drive/ms 0.65, stage/main/first 0.54, stage/main/fixed_main_loop 0.38, surface/cell_upload_ms 0.29, stream/tick/ms 0.22, bigspace/total/ms 0.08 |
| warm-up | 3.25 | 800.9 | main/busy/ms 2464, stage/main/post_update 2401, render/thread/ms 741, render/busy/ms 588, stage/render/prepare_assets 554, stage/render/prepare_views 150, render/acquire/ms 149, stage/main/update 41.32, stage/render/render 20.98, render/graph/ms 16.35, stage/render/camera_driver 14.66, stage/main/pre_update 11.52 |
| warm-up | 3.34 | 86.4 | render/thread/ms 825, render/busy/ms 619, stage/render/prepare_assets 564, stage/render/prepare_views 150, render/acquire/ms 150, stage/render/render 83.75, main/busy/ms 76.59, pipelines/compile_ms 63.96, render/present/ms 56.57, stage/main/post_update 47.04, pipelines/driver_ms 38.81, terrain/update/ms 25.50 |
| warm-up | 3.39 | 53.9 | pipelines/compile_ms 375, pipelines/front_ms 281, render/thread/ms 133, pipelines/wait_ms 127, pipelines/driver_ms 93.85, stage/render/render 81.16, render/busy/ms 77.62, render/present/ms 54.77, main/busy/ms 49.50, terrain/update/ms 32.05, stage/render/prepare_assets 31.70, stage/main/post_update 26.20 |
| warm-up | 3.46 | 70.1 | pipelines/wait_ms 1639, pipelines/compile_ms 806, pipelines/front_ms 709, render/thread/ms 102, render/busy/ms 99.17, pipelines/driver_ms 96.74, main/busy/ms 76.39, stage/main/update 38.58, stage/render/prepare_assets 32.63, stage/main/post_update 31.14, stage/render/render 30.38, render/graph/ms 22.35 |
| warm-up | 3.57 | 106.4 | pipelines/wait_ms 4199, pipelines/compile_ms 1414, pipelines/front_ms 1333, render/thread/ms 154, render/busy/ms 152, main/busy/ms 138, stage/main/update 86.30, pipelines/driver_ms 81.01, stage/render/prepare_assets 62.83, stage/render/prepare_resources 45.77, stage/main/post_update 29.92, stage/render/render 18.00 |
| warm-up | 3.62 | 56.5 | pipelines/wait_ms 3296, pipelines/compile_ms 1786, pipelines/front_ms 1703, render/thread/ms 154, render/busy/ms 152, main/busy/ms 119, stage/render/prepare_assets 94.40, pipelines/driver_ms 82.90, stage/main/update 61.24, stage/render/prepare_resources 36.50, stage/main/post_update 19.66, stage/main/pre_update 15.75 |
| warm-up | 3.66 | 40.6 | pipelines/compile_ms 1648, pipelines/front_ms 1578, pipelines/wait_ms 814, render/thread/ms 90.71, render/busy/ms 88.74, pipelines/driver_ms 70.30, stage/render/prepare_assets 69.23, main/busy/ms 60.16, stage/main/post_update 19.17, stage/main/update 16.45, terrain/update/ms 11.83, stage/render/render 10.77 |
| warm-up | 3.72 | 59.7 | pipelines/compile_ms 1391, pipelines/front_ms 1347, pipelines/wait_ms 371, encode//sum 109, render/thread/ms 95.47, render/busy/ms 93.41, main/busy/ms 44.92, pipelines/driver_ms 44.14, stage/render/render 40.20, render/graph/ms 37.88, stage/render/graph_submit 34.80, stage/render/prepare_assets 33.76 |
| warm-up | 3.76 | 33.4 | pipelines/compile_ms 1109, pipelines/front_ms 1048, pipelines/wait_ms 324, encode//sum 109, render/thread/ms 90.22, render/busy/ms 87.35, pipelines/driver_ms 61.22, main/busy/ms 46.59, stage/render/render 41.20, render/graph/ms 38.67, stage/render/graph_submit 35.09, stage/main/post_update 31.72 |
| warm-up | 3.82 | 63.9 | pipelines/compile_ms 1266, pipelines/front_ms 1182, pipelines/wait_ms 256, render/thread/ms 94.92, render/busy/ms 91.40, pipelines/driver_ms 84.29, main/busy/ms 72.06, stage/main/post_update 49.16, stage/render/prepare_resources 27.57, stage/render/prepare_write_phase_buffers 20.97, stage/render/render 19.68, render/graph/ms 16.45 |
| warm-up | 3.86 | 40.9 | pipelines/compile_ms 1210, pipelines/front_ms 1148, pipelines/wait_ms 118, render/thread/ms 101, render/busy/ms 98.68, main/busy/ms 79.72, pipelines/driver_ms 61.69, stage/main/post_update 40.68, stage/render/prepare_assets 27.71, stage/render/render 20.20, stage/render/prepare_resources 18.12, render/graph/ms 17.01 |
| warm-up | 3.95 | 85.4 | pipelines/compile_ms 1474, pipelines/front_ms 1400, render/thread/ms 122, render/busy/ms 115, pipelines/wait_ms 94.82, stage/render/prepare_assets 79.77, pipelines/driver_ms 73.79, main/busy/ms 47.33, stage/render/render 25.35, render/graph/ms 17.46, stage/main/update 16.40, stage/main/pre_update 15.12 |
| warm-up | 4.03 | 66.9 | pipelines/compile_ms 362, pipelines/front_ms 348, render/thread/ms 83.60, render/busy/ms 81.31, stage/render/prepare_assets 37.89, stage/render/render 31.14, render/graph/ms 28.47, main/busy/ms 26.92, stage/render/graph_submit 24.73, encode/submit 22.92, pipelines/driver_ms 13.62, stage/main/post_update 9.71 |
| warm-up | 4.19 | 32.1 | pipelines/compile_ms 197, pipelines/front_ms 191, main/busy/ms 64.36, render/thread/ms 43.61, stage/main/post_update 41.50, render/busy/ms 41.48, stage/render/render 21.82, render/graph/ms 19.39, stage/main/update 16.83, stage/render/graph_submit 13.59, stage/render/prepare_assets 8.11, encode/submit 7.85 |
| warm-up | 4.33 | 142.8 | pipelines/compile_ms 330, pipelines/front_ms 311, pipelines/wait_ms 159, main/busy/ms 154, render/thread/ms 150, render/busy/ms 148, stage/main/post_update 78.75, stage/main/update 66.97, stage/render/prepare_assets 44.37, pipelines/lock_ms 39.39, stage/render/prepare_resources 38.31, stage/render/render 24.17 |
| warm-up | 4.40 | 64.7 | pipelines/compile_ms 2115, pipelines/front_ms 2049, pipelines/wait_ms 260, render/thread/ms 181, render/busy/ms 179, main/busy/ms 129, stage/render/prepare_assets 86.46, pipelines/lock_ms 79.23, stage/main/update 68.82, pipelines/driver_ms 65.39, stage/main/post_update 49.16, stage/render/prepare_resources 38.37 |
| warm-up | 4.48 | 83.4 | pipelines/compile_ms 2472, pipelines/front_ms 2402, pipelines/wait_ms 143, render/thread/ms 142, render/busy/ms 140, stage/render/prepare_assets 81.13, pipelines/driver_ms 69.59, main/busy/ms 56.21, pipelines/lock_ms 56.14, stage/render/render 41.23, render/graph/ms 38.82, stage/render/graph_submit 33.50 |
| warm-up | 4.55 | 74.8 | pipelines/compile_ms 721, pipelines/front_ms 696, render/thread/ms 155, render/busy/ms 153, main/busy/ms 66.04, pipelines/wait_ms 58.86, stage/render/render 48.06, stage/render/prepare_assets 45.99, render/graph/ms 45.79, stage/render/graph_submit 40.29, stage/render/prepare_resources 40.20, stage/main/update 38.93 |
| warm-up | 4.61 | 57.4 | pipelines/compile_ms 486, pipelines/front_ms 460, render/thread/ms 129, render/busy/ms 127, main/busy/ms 106, stage/main/update 62.69, stage/render/prepare_resources 46.63, stage/render/render 34.78, render/graph/ms 32.29, stage/main/post_update 29.74, pipelines/driver_ms 25.53, stage/render/graph_submit 23.43 |
| warm-up | 4.69 | 78.7 | pipelines/compile_ms 1794, pipelines/front_ms 1731, render/thread/ms 116, render/busy/ms 114, main/busy/ms 111, pipelines/wait_ms 85.71, stage/main/update 64.07, pipelines/driver_ms 62.70, stage/render/prepare_assets 53.99, stage/render/render 26.52, stage/main/post_update 25.55, render/graph/ms 24.02 |
| warm-up | 4.76 | 71.1 | pipelines/compile_ms 2053, pipelines/front_ms 1965, render/thread/ms 130, render/busy/ms 128, pipelines/wait_ms 94.42, pipelines/driver_ms 88.68, stage/render/prepare_assets 64.05, main/busy/ms 58.40, stage/main/update 28.54, stage/render/prepare_flush 26.17, stage/render/render 16.91, render/graph/ms 14.84 |
| warm-up | 4.85 | 39.8 | pipelines/compile_ms 65.80, pipelines/front_ms 63.17, main/busy/ms 58.11, render/thread/ms 46.78, render/busy/ms 45.34, stage/main/update 32.64, stage/render/render 20.39, render/graph/ms 18.28, stage/main/post_update 17.10, character/finish/ms 11.73, stage/render/camera_driver 9.60, stage/render/graph_submit 8.11 |
| warm-up | 5.06 | 30.3 | pipelines/compile_ms 515, pipelines/front_ms 496, render/thread/ms 46.70, render/busy/ms 45.28, main/busy/ms 35.80, pipelines/driver_ms 19.39, stage/main/update 17.42, stage/render/render 14.26, stage/main/post_update 12.36, render/graph/ms 12.31, stage/render/prepare_assets 8.81, encode//sum 8.81 |
| warm-up | 5.09 | 30.6 | pipelines/compile_ms 1574, pipelines/front_ms 1521, render/thread/ms 56.38, render/busy/ms 54.99, pipelines/driver_ms 53.39, main/busy/ms 47.44, stage/main/update 26.29, stage/render/render 15.01, stage/main/post_update 13.94, stage/render/prepare_resources 13.49, render/graph/ms 13.04, stage/render/prepare_assets 11.96 |
| warm-up | 5.18 | 31.5 | pipelines/compile_ms 343, pipelines/front_ms 336, main/busy/ms 43.97, render/thread/ms 40.36, render/busy/ms 38.85, stage/main/update 29.62, stage/render/render 14.54, render/graph/ms 12.55, stream/drive/ms 12.01, stage/render/prepare_assets 10.67, encode//sum 10.21, stage/render/graph_submit 9.65 |
| warm-up | 5.46 | 38.5 | render/thread/ms 58.63, render/busy/ms 56.52, main/busy/ms 37.79, stage/render/prepare_assets 21.99, stage/render/render 21.08, render/graph/ms 18.07, stage/main/post_update 17.36, encode//sum 14.37, stage/main/update 13.89, stage/render/graph_submit 13.33, encode/submit 9.17, stage/render/camera_driver 4.37 |
| warm-up | 5.50 | 40.0 | pipelines/compile_ms 179, pipelines/front_ms 173, render/thread/ms 73.44, render/busy/ms 71.26, main/busy/ms 40.49, stage/render/prepare_assets 37.01, stage/render/render 20.48, render/graph/ms 17.39, stage/main/post_update 16.23, encode//sum 15.38, stage/main/update 14.91, stage/render/graph_submit 12.32 |
| warm-up | 5.55 | 52.8 | pipelines/compile_ms 207, pipelines/front_ms 200, render/thread/ms 88.09, render/busy/ms 86.31, stage/render/render 40.76, render/graph/ms 38.43, stage/render/prepare_assets 36.22, encode//sum 36.22, stage/render/graph_submit 34.41, main/busy/ms 34.08, encode/finish_wall 26.95, encode/pass:main_transparent_pass_3d/sum 25.12 |
| warm-up | 5.59 | 41.5 | render/thread/ms 90.62, render/busy/ms 89.40, stage/render/prepare_assets 39.92, stage/render/render 36.71, main/busy/ms 36.01, encode//sum 35.94, render/graph/ms 34.91, stage/render/graph_submit 31.34, pipelines/compile_ms 28.12, pipelines/front_ms 27.11, encode/finish_wall 26.87, encode/pass:main_transparent_pass_3d/sum 25.07 |
| warm-up | 5.70 | 40.5 | pipelines/compile_ms 163, pipelines/front_ms 158, render/thread/ms 57.23, render/busy/ms 55.49, main/busy/ms 49.82, stage/main/update 24.61, stage/render/render 19.31, stage/main/post_update 16.90, render/graph/ms 16.75, encode//sum 13.14, stage/render/prepare_assets 12.54, stage/render/graph_submit 11.34 |
| warm-up | 5.73 | 30.9 | pipelines/compile_ms 199, pipelines/front_ms 192, render/thread/ms 64.81, render/busy/ms 62.87, main/busy/ms 56.00, stage/main/update 30.24, stage/render/prepare_assets 19.91, stage/render/render 15.75, stage/main/post_update 15.00, render/graph/ms 13.08, encode//sum 10.66, stage/render/prepare_meshes 10.01 |
| warm-up | 5.76 | 34.3 | pipelines/compile_ms 71.19, pipelines/front_ms 67.79, render/thread/ms 60.29, render/busy/ms 58.99, main/busy/ms 47.64, pipelines/wait_ms 32.86, stage/render/prepare_assets 30.17, stage/main/update 27.37, stage/render/render 10.63, stage/main/post_update 9.39, encode//sum 8.92, render/graph/ms 8.79 |
| warm-up | 5.87 | 32.3 | render/thread/ms 53.83, render/busy/ms 52.27, main/busy/ms 41.80, stage/main/update 23.97, pipelines/compile_ms 23.10, stage/render/prepare_assets 21.81, pipelines/front_ms 21.70, stage/render/render 15.50, render/graph/ms 13.43, stage/main/post_update 12.17, encode//sum 11.51, stage/render/graph_submit 8.93 |
| warm-up | 5.95 | 30.6 | pipelines/compile_ms 159, pipelines/front_ms 156, main/busy/ms 49.99, render/thread/ms 48.42, render/busy/ms 46.21, stage/main/update 23.80, stage/render/render 20.07, stage/main/post_update 17.58, render/graph/ms 17.35, stage/render/graph_submit 12.93, encode//sum 11.33, encode/submit 9.10 |
| warm-up | 5.99 | 42.3 | pipelines/compile_ms 181, pipelines/front_ms 176, render/thread/ms 65.99, main/busy/ms 65.26, render/busy/ms 63.93, stage/main/update 33.75, stage/render/prepare_assets 21.30, pipelines/wait_ms 21.25, stage/main/post_update 19.40, stage/render/render 19.38, render/graph/ms 16.64, encode//sum 12.65 |
| warm-up | 6.03 | 36.6 | pipelines/compile_ms 318, pipelines/front_ms 308, render/thread/ms 71.97, render/busy/ms 70.73, main/busy/ms 49.83, stage/render/prepare_assets 36.14, pipelines/wait_ms 32.74, stage/main/update 26.07, stage/render/render 13.73, stage/main/post_update 13.48, render/graph/ms 11.99, encode//sum 11.31 |
| warm-up | 6.64 | 29.5 | pipelines/compile_ms 149, pipelines/front_ms 145, main/busy/ms 40.41, render/thread/ms 32.53, render/busy/ms 31.43, stage/main/update 20.87, stage/render/render 12.51, stage/main/post_update 11.90, render/graph/ms 11.03, encode//sum 8.82, stage/render/graph_submit 8.23, stage/render/prepare_meshes 6.48 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=on-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 92%) · 2625 MHz (min 2625) · memory 8101 MHz · 88.7 W · 72 °C (max 75) · VRAM 6156 MiB · throttled 100% of samples

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
