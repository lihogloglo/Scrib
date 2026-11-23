# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.render.reserve_worst_case=on (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1207 frames over 15.0 s · **median 12.53 ms (80 fps)** · mean 12.43 · p95 13.96 · p99 14.42 (1% low 69 fps) · worst 17.12 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 307 frames over 9.7 s, worst 2474.3 ms
- hitches (> 31.3 ms): **0** measured, 39 in warm-up
- timed GPU passes: 10.58 ms of a 12.43 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.53 ms, limit 11 ms
- pass worst frame 17.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.312 | 2.312 | 2.348 | 1.00 | 0.082 |
| dlss_super_resolution | 2.071 | 2.071 | 2.094 | 1.00 | 0.190 |
| early prepass | 1.327 | 1.327 | 1.339 | 1.00 | 0.126 |
| gpu_instance_materialization | 0.711 | 0.711 | 0.907 | 1.00 | 0.009 |
| sun_visibility | 0.684 | 0.684 | 0.705 | 1.00 | 0.006 |
| atmosphere_luts | 0.367 | 0.367 | 0.429 | 1.00 | 0.003 |
| early_mesh_preprocessing | 0.344 | 0.344 | 0.375 | 1.00 | 0.004 |
| water_velocity | 0.330 | 0.330 | 0.333 | 1.00 | 0.002 |
| bloom | 0.323 | 0.323 | 0.339 | 1.00 | 0.064 |
| atmos_froxel_inject | 0.309 | 0.309 | 0.310 | 1.00 | 0.001 |
| water_sspr | 0.202 | 0.202 | 0.203 | 1.00 | 0.002 |
| ssao | 0.189 | 0.189 | 0.198 | 1.00 | 0.007 |
| uniform_allocation | 0.145 | 0.145 | 0.189 | 1.00 | 0.022 |
| water_fft | 0.135 | 0.135 | 0.139 | 1.00 | 0.002 |
| upscaling | 0.129 | 0.129 | 0.134 | 1.00 | 0.004 |
| bin_unpacking | 0.110 | 0.110 | 0.142 | 1.00 | 0.016 |
| clouds_composite | 0.108 | 0.108 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.089 | 0.089 | 0.094 | 1.00 | 0.003 |
| range_unpacking | 0.088 | 0.088 | 0.101 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.081 | 0.081 | 0.085 | 1.00 | 0.008 |
| clustering | 0.069 | 0.069 | 0.079 | 1.00 | 0.019 |
| render_sky | 0.063 | 0.063 | 0.065 | 1.00 | 0.002 |
| clouds_trace | 0.057 | 0.057 | 0.061 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.409 | 0.411 | 0.13 | 0.001 |
| clouds_resolve | 0.047 | 0.047 | 0.049 | 1.00 | 0.001 |
| point_shadow_atlas | 0.037 | 0.073 | 0.080 | 0.50 | 0.135 |
| per_view_shadow_pass | 0.030 | 0.030 | 0.057 | 1.00 | 0.007 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.236 | 0.13 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| clouds_shadow | 0.021 | 0.021 | 0.023 | 1.00 | 0.003 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.016 | 1.00 | 0.234 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.015 | 0.017 | 0.50 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.007 | 1.00 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.004 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.214 | 12.790 | 15.97 | 13535.7 |
| render/busy/ms | 8.779 | 10.077 | 13.07 | 10596.3 |
| main/busy/ms | 8.058 | 9.453 | 13.20 | 9726.2 |
| stage/render/render | 5.465 | 6.479 | 10.14 | 6596.0 |
| encode//sum | 4.972 | 6.537 | 9.35 | 6001.2 |
| render/graph/ms | 4.849 | 5.778 | 9.40 | 5852.5 |
| stage/main/post_update | 4.650 | 5.845 | 7.28 | 5612.3 |
| stage/render/graph_submit | 3.465 | 4.334 | 6.31 | 4182.8 |
| stage/render/prepare_views | 2.127 | 4.189 | 5.23 | 2567.3 |
| encode/submit | 2.087 | 2.585 | 5.78 | 2519.1 |
| render/acquire/ms | 2.078 | 4.124 | 5.10 | 2507.7 |
| stage/main/update | 1.630 | 2.087 | 3.66 | 1967.2 |
| encode/finish_wall | 1.399 | 1.880 | 2.60 | 1688.1 |
| stage/render/camera_driver | 1.301 | 1.532 | 6.26 | 1570.1 |
| encode/pass:early prepass/sum | 0.902 | 1.206 | 1.70 | 1088.1 |
| encode/pass:shadow_directional_light/sum | 0.881 | 1.545 | 2.44 | 1063.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.878 | 1.220 | 1.69 | 1059.8 |
| stage/render/prepare_assets | 0.584 | 0.703 | 1.67 | 705.0 |
| stage/render/prepare_write_phase_buffers | 0.541 | 0.791 | 1.02 | 653.3 |
| stage/main/fixed_main_loop | 0.539 | 0.774 | 1.32 | 650.5 |
| bigspace/total/ms | 0.531 | 0.919 | 2.02 | 640.4 |
| stage/render/prepare_meshes | 0.500 | 0.651 | 1.23 | 603.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.439 | 0.629 | 0.97 | 529.6 |
| stage/render/prepare_resources | 0.437 | 0.507 | 0.68 | 527.1 |
| stage/main/pre_update | 0.389 | 0.472 | 0.82 | 469.0 |
| stage/main/first | 0.369 | 0.440 | 1.97 | 445.8 |
| render/present/ms | 0.358 | 0.422 | 2.11 | 431.7 |
| controller/feed/ms | 0.317 | 0.411 | 0.82 | 382.6 |
| stage/render/prepare_batch_phases | 0.313 | 0.431 | 0.72 | 377.7 |
| stage/render/prepare_flush | 0.312 | 0.469 | 0.83 | 376.1 |
| stage/main/last | 0.295 | 0.388 | 5.70 | 355.9 |
| stage/render/prepare_bind_groups | 0.275 | 0.449 | 0.74 | 332.5 |
| bigspace/low_precision/ms | 0.233 | 0.422 | 1.62 | 281.0 |
| stage/render/extract_commands | 0.197 | 0.232 | 0.33 | 238.2 |
| stage/render/create_views | 0.186 | 0.222 | 0.58 | 225.0 |
| anim/evaluate_cpu/ms | 0.174 | 0.220 | 0.64 | 210.0 |
| stream/drive/ms | 0.169 | 0.223 | 0.73 | 203.9 |
| anim/evaluate/ms | 0.166 | 0.350 | 1.65 | 200.4 |
| stage/render/specialize | 0.162 | 0.215 | 0.60 | 195.8 |
| stage/render/queue | 0.144 | 0.182 | 0.27 | 173.5 |
| stream/tick/ms | 0.140 | 0.188 | 0.68 | 169.0 |
| bigspace/high_precision/ms | 0.134 | 0.334 | 1.52 | 161.8 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.132 | 0.332 | 0.95 | 159.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.120 | 0.185 | 0.50 | 144.3 |
| controller/move/ms | 0.075 | 0.135 | 0.61 | 90.3 |
| controller/write/ms | 0.060 | 0.078 | 0.12 | 72.1 |
| encode/resolve_diagnostics/sum | 0.056 | 0.086 | 0.16 | 67.0 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.079 | 0.26 | 56.4 |
| physics/step/ms | 0.044 | 0.069 | 0.18 | 53.1 |
| stage/render/phase_sort | 0.042 | 0.061 | 0.14 | 50.9 |
| terrain/update/ms | 0.035 | 0.045 | 0.44 | 42.5 |
| stage/render/cleanup | 0.024 | 0.034 | 0.22 | 29.6 |
| encode/camera_driver_span_begin/sum | 0.023 | 0.038 | 0.09 | 27.2 |
| encode/camera_driver_span_end/sum | 0.021 | 0.037 | 0.37 | 24.8 |
| weather/map/ms | 0.015 | 0.020 | 0.05 | 18.0 |
| anim/apply/ms | 0.012 | 0.018 | 0.05 | 14.7 |
| controller/avoid/ms | 0.011 | 0.017 | 0.11 | 12.7 |
| weather/eval/ms | 0.009 | 0.011 | 0.38 | 10.5 |
| script/update/ms | 0.007 | 0.009 | 0.06 | 8.7 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 6.3 |
| far/select/ms | 0.004 | 0.005 | 0.17 | 5.0 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 4.8 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.02 | 2.3 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.4 |
| stream/promote/ms | 0.001 | 0.002 | 0.00 | 1.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 0.8 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.3 |
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
| crowd/joints_extracted/count | count | 772 | 1145 | 540 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.65 | 1.77 | 1.55 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.37 | 0.03 |
| encode/finish_critical | ms | 1.06 | 2.44 | 2.15 |
| encode/pass:early prepass/max | ms | 0.91 | 1.70 | 1.04 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.89 | 1.69 | 1.16 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.50 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.43 | 0.97 | 0.65 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.25 | 0.95 | 0.40 |
| encode/pass:shadow_directional_light/max | ms | 0.83 | 2.44 | 2.15 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.16 | 0.08 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 11.22 | 13.20 | 11.18 |
| gpu/timed/ms | ms | 11.22 | 12.67 | 11.07 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 1407 | 1407 | 1407 |
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
| phases/shadow/bins | count | 2179 | 2209 | 2207 |
| phases/shadow/instance_batches | count | 9 | 27 | 27 |
| phases/shadow/instance_draws | count | 7317 | 22.0 k | 22.0 k |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 248.6 k |
| phases/shadow/items | count | 5358 | 5388 | 5386 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 53 | 59 | 55 |
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
| preprocess/bind_groups | count | 9 | 68 | 68 |
| preprocess/bind_groups_created | count | 4 | 61 | 61 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 25 | 36.32 | 23.21 |
| process/mem_usage | GiB | 3.03 | 3.18 | 3.18 |
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
| shadow_atlas/mover_draws | count | 2 | 4 | 0 |
| shadow_atlas/movers | count | 100 | 100 | 100 |
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
| skin/blocks_changed/count | count | 19 | 26 | 15 |
| skin/blocks_skipped/count | count | 191 | 198 | 195 |
| skin/blocks_walked/count | count | 19 | 26 | 15 |
| skin/joint_bytes_written/bytes | bytes | 61.2 k | 80.4 k | 54.5 k |
| skin/joints_written/count | count | 956 | 1256 | 852 |
| skin/upload_writes/count | count | 9 | 13 | 11 |
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
| system/cpu_usage | % | 29.98 | 42.73 | 33.66 |
| system/mem_usage | % | 50.16 | 50.62 | 50.62 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 187 | 230 | 230 |
| views/all/bins | count | 5645 | 5675 | 5673 |
| views/all/count | count | 10 | 11 | 11 |
| views/all/instance_batches | count | 53 | 71 | 71 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 36.6 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.66 M |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 110 | 116 | 112 |
| views/all/pipelines_distinct | count | 78 | 84 | 78 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.68 M |
| views/cascade/batch_sets | count | 14 | 60 | 60 |
| views/cascade/bins | count | 2027 | 2043 | 2043 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 27 |
| views/cascade/instance_draws | count | 7317 | 22.0 k | 22.0 k |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 248.6 k |
| views/cascade/items | count | 5206 | 5222 | 5222 |
| views/cascade/pipelines | count | 43 | 43 | 43 |
| views/cascade/work_items | count | 83.0 k | 253.8 k | 253.8 k |
| views/lamp_face/batch_sets | count | 43 | 91 | 73 |
| views/lamp_face/bins | count | 149 | 167 | 164 |
| views/lamp_face/count | count | 6 | 7 | 7 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 149 | 167 | 164 |
| views/lamp_face/pipelines | count | 10 | 16 | 12 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 43 | 91 | 73 |
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
| anim/bones_written/count | count | 47.27 | 116 | 57.1 k |
| anim/events/count | count | 0.05 | 2 | 64 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.98 | 8 | 3600 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 30 | 34.1 k |
| encode/bind_walk/init_textures | count | 9351 | 13.1 k | 11.29 M |
| encode/bind_walk/init_textures_skipped | count | 57.3 k | 66.5 k | 69.21 M |
| encode/bind_walk/inits | count | 802 | 914 | 967.7 k |
| encode/bind_walk/inits_skipped | count | 169 | 185 | 204.4 k |
| encode/bind_walk/merge_textures | count | 9289 | 13.0 k | 11.21 M |
| encode/bind_walk/merge_textures_skipped | count | 18.5 k | 27.6 k | 22.28 M |
| encode/bind_walk/merges | count | 159 | 183 | 191.5 k |
| encode/bind_walk/merges_skipped | count | 157 | 180 | 189.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1207 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1207 |
| encode/pass:early prepass/count | count | 1 | 1 | 1207 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1207 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1207 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1207 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 604 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1207 |
| encode/queue/command_buffers | count | 38.75 | 41 | 46.8 k |
| encode/queue/submits | count | 2 | 2 | 2414 |
| encode/queue/write_buffer/bytes | bytes | 2.77 M | 3.49 M | 3.34 G |
| encode/queue/write_buffer/calls | count | 281 | 353 | 339.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 67.3 k | 67.3 k | 81.26 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 10.9 k |
| encode/queue/write_texture/bytes | bytes | 21.8 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1207 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.75 | 1 | 900 |
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

## Hitches (> 31.3 ms; slowest 39, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2474.3 | main/busy/ms 2441, stage/main/post_update 2391, stage/main/update 36.59, stage/main/pre_update 9.15, stage/main/last 1.25, surface/advance_cpu_ms 1.17, stage/main/fixed_main_loop 0.55, surface/cell_upload_ms 0.50, stage/main/first 0.45, stream/drive/ms 0.31, stream/tick/ms 0.08, bigspace/total/ms 0.07 |
| warm-up | 3.23 | 759.4 | main/busy/ms 2490, stage/main/post_update 2426, render/thread/ms 696, render/busy/ms 543, stage/render/prepare_assets 509, stage/render/prepare_views 151, render/acquire/ms 151, stage/main/update 43.44, stage/render/render 19.13, render/graph/ms 15.38, stage/render/camera_driver 13.63, stage/main/pre_update 11.13 |
| warm-up | 3.37 | 133.7 | render/thread/ms 827, render/busy/ms 595, stage/render/prepare_assets 516, stage/render/prepare_views 152, render/acquire/ms 151, stage/render/render 130, main/busy/ms 98.55, render/present/ms 80.73, stage/main/post_update 73.45, pipelines/compile_ms 50.39, pipelines/front_ms 48.07, render/graph/ms 47.58 |
| warm-up | 3.47 | 100.9 | pipelines/compile_ms 1506, pipelines/front_ms 770, pipelines/driver_ms 735, render/thread/ms 230, pipelines/wait_ms 190, stage/render/render 159, render/busy/ms 139, pipelines/lock_ms 110, main/busy/ms 95.34, render/present/ms 79.51, render/graph/ms 78.29, stage/main/post_update 59.69 |
| warm-up | 3.63 | 162.1 | pipelines/wait_ms 9421, pipelines/compile_ms 4068, pipelines/front_ms 3101, pipelines/driver_ms 967, render/thread/ms 259, render/busy/ms 245, main/busy/ms 184, pipelines/lock_ms 118, stage/main/update 83.22, stage/render/prepare_assets 81.29, stage/render/prepare_write_phase_buffers 59.30, stage/render/render 53.82 |
| warm-up | 3.82 | 187.4 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 3974, pipelines/front_ms 3609, pipelines/driver_ms 364, render/thread/ms 343, render/busy/ms 340, main/busy/ms 168, stage/render/prepare_assets 158, encode//sum 105, stage/main/update 85.66, stage/render/prepare_write_phase_buffers 59.33, stage/render/prepare_meshes 44.49 |
| warm-up | 3.85 | 31.7 | pipelines/compile_ms 1747, pipelines/front_ms 1597, pipelines/wait_ms 1234, render/thread/ms 213, render/busy/ms 211, pipelines/driver_ms 150, encode//sum 116, stage/render/prepare_assets 114, main/busy/ms 52.70, stage/render/prepare_meshes 44.73, stage/render/render 32.86, render/graph/ms 30.69 |
| warm-up | 3.95 | 54.9 | pipelines/compile_ms 508, pipelines/front_ms 488, pipelines/wait_ms 208, render/thread/ms 81.33, render/busy/ms 74.15, main/busy/ms 58.85, stage/render/render 39.05, stage/main/post_update 32.93, render/graph/ms 23.08, pipelines/driver_ms 20.08, stage/render/graph_submit 18.96, encode//sum 18.51 |
| warm-up | 4.02 | 65.3 | pipelines/compile_ms 1175, pipelines/front_ms 1128, render/thread/ms 116, render/busy/ms 108, main/busy/ms 72.69, pipelines/driver_ms 46.60, stage/render/prepare_assets 44.34, stage/render/render 43.61, stage/main/post_update 34.57, render/graph/ms 26.37, stage/render/graph_submit 19.51, stage/main/update 16.92 |
| warm-up | 4.11 | 92.7 | pipelines/compile_ms 1886, pipelines/front_ms 1784, render/thread/ms 153, render/busy/ms 150, pipelines/driver_ms 102, pipelines/wait_ms 91.60, stage/render/prepare_assets 82.59, stage/render/render 54.83, main/busy/ms 52.76, render/graph/ms 51.37, stage/render/graph_submit 41.82, encode//sum 31.80 |
| warm-up | 4.14 | 33.5 | pipelines/compile_ms 1107, pipelines/front_ms 1042, render/thread/ms 123, render/busy/ms 121, pipelines/wait_ms 77.04, stage/render/prepare_assets 67.58, pipelines/driver_ms 64.68, stage/render/render 44.38, render/graph/ms 42.33, stage/render/graph_submit 35.71, main/busy/ms 30.52, encode//sum 29.07 |
| warm-up | 4.18 | 32.1 | pipelines/compile_ms 519, pipelines/front_ms 506, render/thread/ms 63.28, render/busy/ms 61.99, stage/render/prepare_assets 45.46, main/busy/ms 31.53, pipelines/driver_ms 13.06, stage/render/render 10.79, stage/main/update 9.38, render/graph/ms 9.16, stage/main/post_update 7.62, encode//sum 7.54 |
| warm-up | 4.22 | 48.8 | pipelines/compile_ms 342, pipelines/front_ms 330, render/thread/ms 78.70, render/busy/ms 52.68, main/busy/ms 37.44, stage/render/render 26.34, stage/render/prepare_views 23.61, render/acquire/ms 23.47, render/graph/ms 23.16, stage/render/prepare_assets 22.68, stage/render/graph_submit 19.77, encode//sum 19.13 |
| warm-up | 4.42 | 57.2 | pipelines/compile_ms 282, pipelines/front_ms 273, render/thread/ms 72.38, render/busy/ms 70.36, main/busy/ms 67.16, stage/render/render 45.01, render/graph/ms 42.78, stage/main/post_update 37.98, stage/render/camera_driver 28.45, pipelines/wait_ms 27.38, stage/main/update 18.57, stage/render/graph_submit 10.42 |
| warm-up | 4.52 | 103.3 | pipelines/compile_ms 645, pipelines/front_ms 610, pipelines/wait_ms 206, main/busy/ms 194, render/thread/ms 157, render/busy/ms 155, stage/main/update 120, stage/render/render 49.41, stage/main/post_update 47.19, render/graph/ms 46.37, stage/render/prepare_write_phase_buffers 46.27, pipelines/driver_ms 35.49 |
| warm-up | 4.70 | 182.1 | pipelines/compile_ms 2976, pipelines/front_ms 2884, pipelines/wait_ms 286, render/thread/ms 239, render/busy/ms 237, main/busy/ms 181, stage/main/update 133, stage/render/prepare_assets 117, pipelines/driver_ms 92.18, stage/render/prepare_write_phase_buffers 44.66, stage/render/render 32.85, render/graph/ms 29.85 |
| warm-up | 4.80 | 93.1 | pipelines/compile_ms 2719, pipelines/front_ms 2648, render/thread/ms 229, render/busy/ms 227, pipelines/wait_ms 158, stage/render/prepare_assets 143, pipelines/driver_ms 71.39, main/busy/ms 65.80, stage/main/update 39.24, pipelines/lock_ms 35.96, stage/render/prepare_flush 34.66, stage/render/render 31.23 |
| warm-up | 4.85 | 50.4 | pipelines/compile_ms 186, pipelines/front_ms 176, render/thread/ms 138, render/busy/ms 137, main/busy/ms 72.94, pipelines/wait_ms 64.12, stage/render/prepare_assets 51.05, stage/main/update 37.59, stage/render/prepare_flush 33.80, stage/render/render 30.60, render/graph/ms 28.52, pipelines/lock_ms 24.22 |
| warm-up | 4.88 | 38.8 | pipelines/compile_ms 1248, pipelines/front_ms 1207, render/thread/ms 85.24, render/busy/ms 83.67, main/busy/ms 80.71, stage/main/update 42.71, pipelines/driver_ms 41.16, stage/render/render 32.18, render/graph/ms 30.15, stage/render/prepare_assets 23.80, pipelines/wait_ms 20.00, stage/render/graph_submit 18.96 |
| warm-up | 4.92 | 36.0 | pipelines/compile_ms 1666, pipelines/front_ms 1613, render/thread/ms 71.77, render/busy/ms 70.21, main/busy/ms 64.27, pipelines/driver_ms 52.99, stage/main/update 36.32, pipelines/wait_ms 28.47, stage/render/render 21.23, render/graph/ms 19.08, stage/main/post_update 17.36, stage/render/prepare_assets 16.66 |
| warm-up | 4.96 | 35.1 | pipelines/compile_ms 533, pipelines/front_ms 515, render/thread/ms 66.78, render/busy/ms 65.15, main/busy/ms 53.88, stage/main/update 29.42, pipelines/wait_ms 22.96, stage/render/render 20.97, render/graph/ms 18.85, pipelines/driver_ms 17.75, stage/main/post_update 15.16, stage/render/graph_submit 13.56 |
| warm-up | 5.18 | 47.8 | pipelines/compile_ms 409, pipelines/front_ms 400, main/busy/ms 55.24, render/thread/ms 46.23, render/busy/ms 45.04, stage/main/post_update 31.27, stage/main/update 14.39, stage/render/render 12.68, stage/render/prepare_resources 11.46, render/graph/ms 10.86, pipelines/driver_ms 9.02, encode//sum 8.21 |
| warm-up | 5.36 | 36.2 | pipelines/compile_ms 1401, pipelines/front_ms 1369, main/busy/ms 52.07, render/thread/ms 43.90, render/busy/ms 41.57, pipelines/driver_ms 32.02, stage/main/update 31.96, stage/render/render 15.14, stage/render/prepare_assets 12.73, stage/main/post_update 12.38, render/graph/ms 12.29, stream/drive/ms 12.06 |
| warm-up | 5.40 | 37.5 | pipelines/compile_ms 68.82, pipelines/front_ms 66.62, render/thread/ms 59.97, render/busy/ms 58.22, main/busy/ms 44.40, stage/main/update 21.33, stage/render/prepare_assets 20.51, stage/render/render 16.70, stage/main/post_update 15.97, render/graph/ms 14.22, encode//sum 10.75, stage/render/graph_submit 9.89 |
| warm-up | 5.44 | 40.4 | pipelines/compile_ms 193, pipelines/front_ms 186, render/thread/ms 73.23, render/busy/ms 71.08, main/busy/ms 42.64, stage/render/render 24.36, render/graph/ms 21.09, stage/render/prepare_assets 19.59, stage/main/post_update 16.59, stage/main/update 16.11, stage/render/graph_submit 15.02, encode//sum 13.85 |
| warm-up | 5.48 | 37.0 | pipelines/compile_ms 405, pipelines/front_ms 393, render/thread/ms 72.57, render/busy/ms 70.69, main/busy/ms 37.24, stage/render/prepare_assets 30.68, stage/render/render 19.94, render/graph/ms 17.18, stage/main/post_update 13.35, stage/render/graph_submit 11.88, pipelines/driver_ms 11.63, encode//sum 11.55 |
| warm-up | 5.52 | 39.1 | pipelines/compile_ms 276, pipelines/front_ms 269, render/thread/ms 72.85, render/busy/ms 71.26, stage/render/prepare_assets 48.06, main/busy/ms 44.38, stage/main/update 17.35, stage/main/post_update 16.32, stage/render/render 13.16, render/graph/ms 11.10, encode//sum 8.42, stage/render/graph_submit 7.39 |
| warm-up | 5.55 | 34.9 | pipelines/compile_ms 931, pipelines/front_ms 904, render/thread/ms 71.41, render/busy/ms 70.06, main/busy/ms 46.70, stage/render/prepare_assets 45.69, pipelines/driver_ms 26.78, stage/main/update 20.62, stage/main/post_update 18.26, stage/render/render 13.08, encode//sum 12.39, render/graph/ms 11.08 |
| warm-up | 5.64 | 33.7 | render/thread/ms 57.29, render/busy/ms 56.02, main/busy/ms 43.58, pipelines/compile_ms 33.83, pipelines/front_ms 32.43, stage/main/update 20.49, stage/main/post_update 14.94, stage/render/render 13.32, stage/render/prepare_write_phase_buffers 13.12, render/graph/ms 11.64, stage/render/graph_submit 8.23, encode//sum 8.18 |
| warm-up | 5.84 | 35.3 | render/thread/ms 56.19, render/busy/ms 54.46, main/busy/ms 35.11, stage/render/prepare_assets 26.49, stage/main/update 15.64, stage/render/render 14.98, render/graph/ms 12.62, stage/main/post_update 11.16, encode//sum 10.61, stage/render/graph_submit 9.49, encode/submit 7.01, stage/render/prepare_bind_groups 5.15 |
| warm-up | 5.95 | 45.5 | render/thread/ms 57.24, render/busy/ms 55.59, main/busy/ms 46.83, stage/main/update 22.90, stage/main/post_update 17.65, pipelines/compile_ms 16.82, pipelines/front_ms 16.20, stage/render/render 15.94, stage/render/prepare_assets 15.68, render/graph/ms 13.34, encode//sum 11.68, stage/render/prepare_meshes 8.77 |
| warm-up | 6.01 | 41.1 | pipelines/compile_ms 84.07, pipelines/front_ms 79.88, render/thread/ms 55.07, render/busy/ms 53.33, main/busy/ms 45.76, stage/main/update 24.57, stage/render/prepare_assets 21.58, stage/render/render 15.96, render/graph/ms 13.83, stage/main/post_update 13.68, stage/render/graph_submit 9.92, encode//sum 8.65 |
| warm-up | 6.20 | 35.0 | pipelines/compile_ms 533, pipelines/front_ms 509, main/busy/ms 56.93, render/thread/ms 54.54, render/busy/ms 52.72, stage/main/update 26.44, pipelines/driver_ms 23.63, stage/render/render 21.76, stage/main/post_update 19.95, render/graph/ms 19.37, stage/render/graph_submit 14.27, encode//sum 13.71 |
| warm-up | 6.25 | 46.2 | pipelines/compile_ms 720, pipelines/front_ms 692, render/thread/ms 72.54, render/busy/ms 70.77, main/busy/ms 66.20, stage/main/update 32.97, pipelines/driver_ms 27.61, stage/render/prepare_assets 25.89, stage/render/render 21.69, stage/main/post_update 19.98, render/graph/ms 19.27, stage/render/graph_submit 14.04 |
| warm-up | 6.29 | 41.2 | pipelines/compile_ms 621, pipelines/front_ms 604, render/thread/ms 79.46, render/busy/ms 78.12, main/busy/ms 46.63, stage/render/prepare_assets 38.24, stage/main/update 22.74, pipelines/driver_ms 16.77, stage/render/render 14.77, stage/main/post_update 13.95, render/graph/ms 12.93, stage/render/prepare_bind_groups 9.51 |
| warm-up | 6.37 | 32.4 | pipelines/compile_ms 499, pipelines/front_ms 484, render/thread/ms 47.39, render/busy/ms 46.03, main/busy/ms 39.78, stage/main/update 18.22, stage/render/render 16.35, pipelines/driver_ms 14.64, render/graph/ms 14.37, stage/main/post_update 13.25, encode//sum 12.52, stage/render/graph_submit 10.17 |
| warm-up | 6.40 | 32.8 | pipelines/compile_ms 338, pipelines/front_ms 324, render/thread/ms 59.37, render/busy/ms 58.00, main/busy/ms 46.04, stage/main/update 21.92, stage/render/render 19.58, render/graph/ms 17.67, stage/main/post_update 16.62, pipelines/driver_ms 14.92, stage/render/graph_submit 13.71, encode//sum 9.72 |
| warm-up | 6.47 | 40.7 | pipelines/compile_ms 409, pipelines/front_ms 392, render/thread/ms 67.97, render/busy/ms 66.58, main/busy/ms 36.93, stage/render/prepare_assets 20.08, pipelines/driver_ms 17.30, stage/main/post_update 15.30, stage/main/update 15.20, stage/render/render 13.62, stage/render/prepare_resources 13.35, render/graph/ms 11.68 |
| warm-up | 6.62 | 31.8 | main/busy/ms 38.02, render/thread/ms 35.69, render/busy/ms 34.66, stage/main/update 19.75, stage/main/post_update 15.08, stage/render/render 13.50, render/graph/ms 12.02, encode//sum 11.50, stage/render/graph_submit 7.67, stage/render/prepare_assets 6.29, stage/render/prepare_meshes 5.91, stream/drive/ms 4.25 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --set groundcover.render.reserve_worst_case=true --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=worst-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 86.9 W · 73 °C (max 75) · VRAM 7645 MiB · throttled 100% of samples

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
