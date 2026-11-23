# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1416 frames over 15.0 s · **median 10.48 ms (95 fps)** · mean 10.60 · p95 12.48 · p99 13.90 (1% low 72 fps) · worst 17.50 ms · 5 spikes over 1.5× median · 24.0 stutters/min (work 4.0/min)
- warm-up: 310 frames over 9.0 s, worst 2483.3 ms
- hitches (> 26.2 ms): **0** measured, 47 in warm-up
- timed GPU passes: 8.05 ms of a 10.60 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.48 ms, limit 11 ms
- pass worst frame 17.50 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.900 | 2.063 | 2.086 | 0.92 | 0.179 |
| main_opaque_pass_3d | 1.887 | 2.049 | 2.090 | 0.92 | 0.076 |
| early prepass | 0.690 | 0.749 | 0.764 | 0.92 | 0.098 |
| sun_visibility | 0.640 | 0.695 | 0.710 | 0.92 | 0.007 |
| atmosphere_luts | 0.330 | 0.359 | 0.406 | 0.92 | 0.003 |
| water_velocity | 0.304 | 0.330 | 0.333 | 0.92 | 0.002 |
| bloom | 0.295 | 0.321 | 0.327 | 0.92 | 0.062 |
| atmos_froxel_inject | 0.285 | 0.310 | 0.310 | 0.92 | 0.001 |
| water_sspr | 0.192 | 0.208 | 0.202 | 0.92 | 0.002 |
| ssao | 0.173 | 0.188 | 0.191 | 0.92 | 0.008 |
| uniform_allocation | 0.132 | 0.143 | 0.186 | 0.92 | 0.026 |
| water_fft | 0.125 | 0.135 | 0.137 | 0.92 | 0.003 |
| upscaling | 0.113 | 0.123 | 0.127 | 0.92 | 0.004 |
| clouds_composite | 0.100 | 0.109 | 0.111 | 0.92 | 0.002 |
| bin_unpacking | 0.095 | 0.103 | 0.133 | 0.92 | 0.018 |
| tonemapping | 0.078 | 0.085 | 0.089 | 0.92 | 0.004 |
| gpu_instance_materialization | 0.078 | 0.085 | 0.093 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.073 | 0.080 | 0.091 | 0.92 | 0.009 |
| clustering | 0.063 | 0.068 | 0.079 | 0.92 | 0.019 |
| render_sky | 0.058 | 0.063 | 0.065 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.057 | 0.062 | 0.092 | 0.92 | 0.004 |
| clouds_trace | 0.053 | 0.057 | 0.068 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.410 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.044 | 0.048 | 0.050 | 0.92 | 0.001 |
| point_shadow_atlas | 0.035 | 0.072 | 0.077 | 0.48 | 0.127 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.030 | 0.92 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.219 | 0.229 | 0.12 | 0.004 |
| water_snapshot | 0.022 | 0.024 | 0.025 | 0.92 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.92 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.92 | 0.003 |
| clouds_shadow | 0.019 | 0.021 | 0.024 | 0.92 | 0.003 |
| per_view_shadow_pass | 0.018 | 0.020 | 0.037 | 0.92 | 0.006 |
| main_transparent_pass_3d | 0.013 | 0.014 | 0.025 | 0.92 | 0.234 |
| clouds_fog_light | 0.010 | 0.011 | 0.012 | 0.92 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.015 | 0.016 | 0.48 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.92 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.007 | 0.92 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.228 | 10.949 | 18.17 | 13067.1 |
| render/busy/ms | 8.434 | 9.974 | 16.30 | 11942.7 |
| main/busy/ms | 8.144 | 9.200 | 12.88 | 11531.3 |
| stage/render/render | 5.374 | 7.053 | 11.07 | 7609.7 |
| render/graph/ms | 4.702 | 5.806 | 8.87 | 6658.4 |
| stage/main/post_update | 4.651 | 5.493 | 7.20 | 6586.2 |
| encode//sum | 4.374 | 6.272 | 12.87 | 6193.3 |
| stage/render/graph_submit | 3.370 | 4.388 | 6.70 | 4771.3 |
| encode/submit | 2.073 | 3.125 | 5.50 | 2935.0 |
| stage/main/update | 1.813 | 2.112 | 2.45 | 2567.6 |
| encode/finish_wall | 1.387 | 1.983 | 3.67 | 1964.0 |
| stage/render/camera_driver | 1.269 | 1.463 | 2.41 | 1796.6 |
| encode/pass:shadow_directional_light/sum | 0.918 | 1.563 | 2.60 | 1299.4 |
| encode/pass:early prepass/sum | 0.869 | 1.180 | 2.41 | 1230.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.844 | 1.166 | 2.35 | 1194.6 |
| stage/render/prepare_assets | 0.584 | 0.712 | 1.63 | 826.7 |
| stage/render/prepare_write_phase_buffers | 0.545 | 0.779 | 1.03 | 771.0 |
| bigspace/total/ms | 0.544 | 1.281 | 2.18 | 769.8 |
| stage/render/prepare_meshes | 0.501 | 0.715 | 1.20 | 709.1 |
| pipelines/compile_ms | 0.460 | 0.000 | 122.76 | 651.2 |
| stage/main/fixed_main_loop | 0.453 | 0.739 | 1.45 | 641.1 |
| pipelines/front_ms | 0.449 | 0.000 | 120.34 | 635.3 |
| stage/render/prepare_resources | 0.430 | 0.505 | 1.14 | 608.7 |
| render/present/ms | 0.422 | 1.287 | 2.06 | 597.8 |
| stage/render/prepare_views | 0.416 | 1.708 | 2.58 | 588.7 |
| stage/main/pre_update | 0.414 | 0.550 | 0.94 | 586.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.414 | 0.645 | 1.15 | 585.9 |
| render/acquire/ms | 0.372 | 1.661 | 2.54 | 526.7 |
| stage/main/first | 0.354 | 0.427 | 2.29 | 501.1 |
| controller/feed/ms | 0.328 | 0.405 | 0.67 | 463.9 |
| stage/render/prepare_batch_phases | 0.320 | 0.440 | 0.82 | 453.2 |
| stage/render/prepare_flush | 0.314 | 0.525 | 1.12 | 444.9 |
| stage/render/prepare_bind_groups | 0.274 | 0.428 | 0.74 | 388.7 |
| stage/main/last | 0.274 | 0.321 | 4.61 | 388.6 |
| bigspace/low_precision/ms | 0.250 | 0.751 | 1.84 | 354.2 |
| stream/drive/ms | 0.190 | 0.264 | 0.55 | 269.3 |
| stage/render/create_views | 0.189 | 0.232 | 0.67 | 267.7 |
| stage/render/extract_commands | 0.185 | 0.216 | 0.66 | 261.7 |
| anim/evaluate_cpu/ms | 0.167 | 0.200 | 0.56 | 237.0 |
| stage/render/specialize | 0.163 | 0.226 | 0.64 | 230.3 |
| stream/tick/ms | 0.159 | 0.228 | 0.52 | 225.1 |
| anim/evaluate/ms | 0.148 | 0.277 | 1.08 | 209.5 |
| bigspace/high_precision/ms | 0.143 | 0.515 | 1.57 | 202.7 |
| stage/render/queue | 0.134 | 0.173 | 0.30 | 189.6 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.121 | 0.339 | 1.53 | 171.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.117 | 0.197 | 1.42 | 165.1 |
| controller/write/ms | 0.063 | 0.080 | 0.43 | 89.6 |
| encode/resolve_diagnostics/sum | 0.047 | 0.088 | 1.46 | 66.9 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.071 | 0.25 | 64.5 |
| controller/move/ms | 0.045 | 0.084 | 0.14 | 63.2 |
| terrain/update/ms | 0.035 | 0.049 | 0.19 | 49.7 |
| physics/step/ms | 0.035 | 0.064 | 0.37 | 49.6 |
| stage/render/phase_sort | 0.033 | 0.051 | 0.20 | 46.9 |
| stage/render/cleanup | 0.025 | 0.033 | 0.37 | 34.9 |
| encode/camera_driver_span_begin/sum | 0.019 | 0.036 | 0.08 | 26.6 |
| encode/camera_driver_span_end/sum | 0.018 | 0.035 | 1.44 | 25.3 |
| weather/map/ms | 0.016 | 0.021 | 0.05 | 22.5 |
| pipelines/driver_ms | 0.011 | 0.000 | 3.43 | 15.8 |
| anim/apply/ms | 0.010 | 0.013 | 0.03 | 14.4 |
| weather/eval/ms | 0.009 | 0.012 | 0.03 | 13.4 |
| controller/avoid/ms | 0.009 | 0.018 | 0.04 | 13.3 |
| script/update/ms | 0.008 | 0.011 | 0.05 | 11.1 |
| script/input/ms | 0.005 | 0.006 | 0.37 | 7.7 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 5.9 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 5.8 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.02 | 3.1 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.4 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 1.0 |
| terrain/select/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
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
| anim/evaluated/count | count | 26.50 | 27 | 27 |
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
| crowd/joints_extracted/count | count | 682 | 1154 | 656 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.7 k | 43.7 k | 43.7 k |
| encode//max | ms | 0.53 | 2.72 | 0.43 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.07 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 1.44 | 0.01 |
| encode/finish_critical | ms | 1.01 | 2.72 | 0.90 |
| encode/pass:early prepass/max | ms | 0.86 | 2.41 | 0.90 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.84 | 2.35 | 0.84 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 1.42 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.39 | 1.15 | 0.39 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.22 | 1.53 | 0.15 |
| encode/pass:shadow_directional_light/max | ms | 0.84 | 2.60 | 0.89 |
| encode/resolve_diagnostics/max | ms | 0.04 | 1.46 | 0.02 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 9.36 | 16.30 | 9.35 |
| gpu/timed/ms | ms | 9.35 | 10.61 | 9.35 |
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
| phases/opaque3d/batch_sets | count | 24 | 24 | 24 |
| phases/opaque3d/bins | count | 1362 | 1362 | 1362 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 6753 | 6753 | 6753 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2843 | 2843 | 2843 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 22 | 22 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 21 | 21 | 21 |
| phases/opaque3d_prepass/bins | count | 1362 | 1362 | 1362 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 6753 | 6753 | 6753 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2843 | 2843 | 2843 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 9 | 106 | 106 |
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
| precache/throttled | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 5 | 67 | 67 |
| preprocess/bind_groups_created | count | 0 | 60 | 60 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 141 | 141 | 141 |
| process/cpu_usage | % | 26.44 | 38.94 | 28.83 |
| process/mem_usage | GiB | 2.99 | 3.15 | 3.15 |
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
| shadow_atlas/render/blits | count | 0 | 4 | 2 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 0 | 4 | 2 |
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
| skin/blocks_changed/count | count | 17 | 27 | 16 |
| skin/blocks_skipped/count | count | 193 | 202 | 194 |
| skin/blocks_walked/count | count | 17 | 27 | 16 |
| skin/joint_bytes_written/bytes | bytes | 54.8 k | 76.8 k | 51.5 k |
| skin/joints_written/count | count | 856 | 1200 | 804 |
| skin/upload_writes/count | count | 7 | 10 | 9 |
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
| system/cpu_usage | % | 31.94 | 42.68 | 29.76 |
| system/mem_usage | % | 50.87 | 51.26 | 51.26 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2950 | 2950 | 2950 |
| textures/images | count | 2500 | 2500 | 2500 |
| views/all/batch_sets | count | 72 | 169 | 169 |
| views/all/bins | count | 5597 | 5611 | 5597 |
| views/all/count | count | 10 | 10 | 10 |
| views/all/instance_batches | count | 12 | 20 | 20 |
| views/all/instance_draws | count | 21.9 k | 36.6 k | 36.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 245.8 k |
| views/all/items | count | 12.1 k | 12.2 k | 12.1 k |
| views/all/pipelines | count | 99 | 100 | 99 |
| views/all/pipelines_distinct | count | 71 | 71 | 71 |
| views/all/work_items | count | 154.8 k | 258.2 k | 258.2 k |
| views/cascade/batch_sets | count | 9 | 45 | 45 |
| views/cascade/bins | count | 2028 | 2028 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 12 |
| views/cascade/instance_draws | count | 7312 | 21.9 k | 21.9 k |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 5207 | 5207 | 5207 |
| views/cascade/pipelines | count | 37 | 37 | 37 |
| views/cascade/work_items | count | 49.3 k | 152.7 k | 152.7 k |
| views/lamp_face/batch_sets | count | 0 | 91 | 61 |
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
| views/lamp_face/work_items | count | 0 | 91 | 61 |
| views/main/batch_sets | count | 63 | 63 | 63 |
| views/main/bins | count | 3432 | 3432 | 3432 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 14.6 k | 14.6 k | 14.6 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6804 | 6804 | 6804 |
| views/main/pipelines | count | 52 | 52 | 52 |
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
| anim/bones_written/count | count | 30.93 | 66 | 43.8 k |
| anim/events/count | count | 0.04 | 2 | 51 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.54 | 8 | 3600 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 56 | 40.0 k |
| encode/bind_walk/init_textures | count | 9082 | 20.6 k | 12.86 M |
| encode/bind_walk/init_textures_skipped | count | 57.3 k | 120.6 k | 81.12 M |
| encode/bind_walk/inits | count | 657 | 1361 | 930.4 k |
| encode/bind_walk/inits_skipped | count | 173 | 352 | 245.3 k |
| encode/bind_walk/merge_textures | count | 9020 | 20.5 k | 12.77 M |
| encode/bind_walk/merge_textures_skipped | count | 8730 | 23.5 k | 12.36 M |
| encode/bind_walk/merges | count | 115 | 240 | 162.9 k |
| encode/bind_walk/merges_skipped | count | 157 | 326 | 222.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1416 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1416 |
| encode/pass:early prepass/count | count | 1 | 2 | 1416 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1416 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1416 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1416 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 708 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1416 |
| encode/queue/command_buffers | count | 38.75 | 77 | 54.9 k |
| encode/queue/submits | count | 2 | 4 | 2832 |
| encode/queue/write_buffer/bytes | bytes | 2.72 M | 5.90 M | 3.85 G |
| encode/queue/write_buffer/calls | count | 267 | 581 | 378.5 k |
| encode/queue/write_buffer_with/bytes | bytes | 55.6 k | 111.1 k | 78.66 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 12.7 k |
| encode/queue/write_texture/bytes | bytes | 18.6 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1416 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.01 | 1 | 10 |
| pipelines/created | count | 0.01 | 1 | 10 |
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
| warm-up | 2.48 | 2483.3 | main/busy/ms 2453, stage/main/post_update 2406, stage/main/update 33.62, stage/main/pre_update 9.82, stage/main/last 1.01, surface/advance_cpu_ms 0.73, stream/drive/ms 0.63, stage/main/first 0.62, stage/main/fixed_main_loop 0.47, surface/cell_upload_ms 0.30, stream/tick/ms 0.21, bigspace/total/ms 0.07 |
| warm-up | 3.21 | 724.8 | main/busy/ms 2496, stage/main/post_update 2435, render/thread/ms 662, render/busy/ms 525, stage/render/prepare_assets 475, stage/render/prepare_views 134, render/acquire/ms 134, stage/main/update 41.35, stage/render/create_views 29.82, stage/render/render 11.78, stage/main/pre_update 11.56, render/graph/ms 7.89 |
| warm-up | 3.29 | 81.4 | render/thread/ms 740, render/busy/ms 549, stage/render/prepare_assets 478, stage/render/prepare_views 134, render/acquire/ms 134, stage/render/render 79.82, main/busy/ms 68.97, render/present/ms 57.14, stage/main/post_update 40.80, stage/render/create_views 30.13, pipelines/compile_ms 24.22, pipelines/front_ms 21.97 |
| warm-up | 3.36 | 70.4 | pipelines/compile_ms 496, pipelines/front_ms 400, render/thread/ms 147, pipelines/wait_ms 143, stage/render/render 105, pipelines/driver_ms 96.67, render/busy/ms 90.30, render/present/ms 55.65, main/busy/ms 55.09, render/graph/ms 48.14, stage/main/post_update 33.51, terrain/update/ms 27.09 |
| warm-up | 3.49 | 133.5 | pipelines/wait_ms 6803, pipelines/compile_ms 1816, pipelines/front_ms 1640, render/thread/ms 190, render/busy/ms 184, pipelines/driver_ms 176, main/busy/ms 125, stage/main/update 72.73, stage/render/render 50.28, render/graph/ms 47.36, stage/main/post_update 43.05, stage/render/prepare_assets 42.53 |
| warm-up | 3.57 | 74.6 | pipelines/wait_ms 7081, pipelines/compile_ms 2499, pipelines/front_ms 2357, render/thread/ms 193, render/busy/ms 187, pipelines/driver_ms 142, main/busy/ms 135, stage/render/prepare_assets 81.30, stage/main/update 76.32, stage/main/post_update 39.81, stage/render/prepare_flush 37.67, stage/render/prepare_resources 33.18 |
| warm-up | 3.62 | 48.8 | pipelines/compile_ms 1561, pipelines/front_ms 1486, pipelines/wait_ms 553, render/thread/ms 99.59, render/busy/ms 97.43, stage/render/prepare_assets 79.50, pipelines/driver_ms 74.75, main/busy/ms 62.83, stage/main/post_update 28.60, terrain/update/ms 16.92, stage/main/update 12.03, stage/render/render 11.01 |
| warm-up | 3.65 | 36.7 | pipelines/compile_ms 446, pipelines/front_ms 426, pipelines/wait_ms 133, render/thread/ms 62.93, render/busy/ms 61.35, main/busy/ms 44.94, stage/render/prepare_assets 44.57, stage/main/post_update 23.86, pipelines/driver_ms 19.38, terrain/update/ms 11.32, stage/main/update 9.29, stage/render/render 9.19 |
| warm-up | 3.69 | 33.7 | pipelines/compile_ms 655, pipelines/front_ms 628, render/thread/ms 66.95, render/busy/ms 65.31, main/busy/ms 51.44, stage/render/prepare_assets 28.29, stage/main/post_update 27.04, pipelines/wait_ms 27.00, pipelines/driver_ms 26.96, stage/render/render 16.83, encode//sum 15.83, render/graph/ms 14.69 |
| warm-up | 3.76 | 43.6 | pipelines/compile_ms 1035, pipelines/front_ms 955, pipelines/wait_ms 132, pipelines/driver_ms 80.00, render/thread/ms 64.37, main/busy/ms 62.19, render/busy/ms 61.69, stage/main/post_update 30.87, stage/main/update 23.79, stage/render/prepare_write_phase_buffers 20.75, stage/render/render 17.51, render/graph/ms 14.02 |
| warm-up | 3.81 | 51.4 | pipelines/compile_ms 1518, pipelines/front_ms 1411, pipelines/wait_ms 174, pipelines/driver_ms 107, render/thread/ms 88.38, render/busy/ms 85.05, main/busy/ms 47.51, stage/render/prepare_assets 35.79, stage/main/update 22.99, stage/render/render 20.51, stage/render/prepare_write_phase_buffers 17.21, render/graph/ms 16.33 |
| warm-up | 3.84 | 34.1 | pipelines/compile_ms 904, pipelines/front_ms 858, render/thread/ms 80.18, render/busy/ms 74.38, stage/render/prepare_assets 46.43, pipelines/driver_ms 46.31, pipelines/wait_ms 43.83, main/busy/ms 23.42, stage/render/render 21.99, render/graph/ms 15.62, stage/render/graph_submit 10.65, encode//sum 9.40 |
| warm-up | 3.89 | 49.2 | pipelines/compile_ms 636, pipelines/front_ms 612, render/thread/ms 78.60, render/busy/ms 73.53, stage/render/prepare_assets 49.16, main/busy/ms 37.99, pipelines/driver_ms 23.92, stage/render/render 18.43, pipelines/wait_ms 13.10, render/graph/ms 12.83, pipelines/lock_ms 11.58, stage/main/update 9.02 |
| warm-up | 3.95 | 56.3 | pipelines/compile_ms 636, pipelines/front_ms 615, render/thread/ms 102, render/busy/ms 99.89, stage/render/prepare_assets 78.86, main/busy/ms 41.19, pipelines/driver_ms 21.30, stage/render/render 14.28, pipelines/wait_ms 12.64, render/graph/ms 11.98, pipelines/lock_ms 11.27, stage/main/post_update 10.57 |
| warm-up | 3.98 | 27.8 | pipelines/compile_ms 283, pipelines/front_ms 274, render/thread/ms 80.93, render/busy/ms 79.24, stage/render/prepare_assets 57.14, main/busy/ms 26.06, stage/render/render 18.03, render/graph/ms 16.10, stage/render/graph_submit 12.84, encode/submit 11.22, stage/main/post_update 9.69, stage/main/update 9.18 |
| warm-up | 4.06 | 34.1 | pipelines/compile_ms 1126, pipelines/front_ms 1089, render/thread/ms 49.77, render/busy/ms 48.27, pipelines/driver_ms 36.61, main/busy/ms 32.20, stage/main/update 21.33, stage/render/render 21.16, render/graph/ms 18.36, stage/render/prepare_assets 15.97, stage/render/graph_submit 11.55, pipelines/wait_ms 8.21 |
| warm-up | 4.14 | 33.1 | pipelines/compile_ms 926, pipelines/front_ms 898, render/thread/ms 49.26, render/busy/ms 47.86, main/busy/ms 32.67, pipelines/driver_ms 27.34, stage/render/prepare_assets 25.15, stage/main/update 14.15, stage/render/render 11.28, stage/main/post_update 11.26, render/graph/ms 9.49, encode//sum 6.92 |
| warm-up | 4.18 | 45.5 | pipelines/compile_ms 150, pipelines/front_ms 144, render/thread/ms 75.61, render/busy/ms 73.68, main/busy/ms 39.22, stage/render/prepare_assets 30.23, stage/render/prepare_resources 23.59, stage/main/update 15.96, stage/main/post_update 15.57, stage/render/render 12.11, render/graph/ms 10.24, stage/render/graph_submit 7.71 |
| warm-up | 4.31 | 130.2 | pipelines/compile_ms 1195, pipelines/front_ms 1155, render/thread/ms 169, render/busy/ms 166, main/busy/ms 123, stage/main/update 60.85, stage/render/prepare_resources 56.28, stage/main/post_update 48.73, pipelines/driver_ms 39.93, stage/render/prepare_assets 37.93, stage/render/prepare_flush 28.58, stage/render/render 27.69 |
| warm-up | 4.42 | 109.4 | pipelines/compile_ms 3146, pipelines/front_ms 3050, render/thread/ms 223, render/busy/ms 216, main/busy/ms 186, pipelines/wait_ms 111, pipelines/driver_ms 95.48, stage/main/update 89.84, stage/render/prepare_assets 81.01, stage/main/post_update 55.75, stage/render/prepare_flush 41.59, stage/render/prepare_resources 39.62 |
| warm-up | 4.55 | 126.8 | pipelines/compile_ms 3452, pipelines/front_ms 3346, pipelines/wait_ms 288, render/thread/ms 217, render/busy/ms 210, stage/render/prepare_assets 156, main/busy/ms 111, pipelines/driver_ms 106, stage/main/update 47.47, stage/main/post_update 20.75, stage/main/pre_update 18.71, stage/render/render 16.72 |
| warm-up | 4.59 | 46.0 | pipelines/compile_ms 1725, pipelines/front_ms 1667, pipelines/wait_ms 201, render/thread/ms 163, render/busy/ms 161, stage/render/prepare_assets 117, main/busy/ms 61.04, pipelines/driver_ms 58.19, stage/main/update 27.03, stage/render/render 22.01, render/graph/ms 19.87, stage/render/graph_submit 16.27 |
| warm-up | 4.63 | 33.7 | pipelines/compile_ms 1257, pipelines/front_ms 1218, render/thread/ms 76.34, render/busy/ms 74.66, main/busy/ms 62.13, pipelines/wait_ms 44.72, pipelines/driver_ms 39.37, stage/main/update 35.07, stage/render/prepare_assets 23.09, stage/render/render 22.36, render/graph/ms 19.87, stage/main/post_update 16.02 |
| warm-up | 4.66 | 33.3 | pipelines/compile_ms 1017, pipelines/front_ms 985, render/thread/ms 63.59, render/busy/ms 62.06, main/busy/ms 61.16, pipelines/wait_ms 44.30, stage/main/update 33.91, pipelines/driver_ms 31.50, stage/render/prepare_assets 16.28, stage/render/render 15.70, stage/main/post_update 15.57, render/graph/ms 13.36 |
| warm-up | 4.69 | 30.5 | pipelines/compile_ms 112, pipelines/front_ms 108, render/thread/ms 60.10, render/busy/ms 58.64, main/busy/ms 56.46, stage/main/update 28.84, stage/render/render 16.90, stage/main/post_update 15.97, render/graph/ms 15.00, stage/render/prepare_assets 14.63, stage/render/graph_submit 10.48, encode//sum 8.07 |
| warm-up | 4.72 | 26.8 | pipelines/compile_ms 108, pipelines/front_ms 103, render/thread/ms 53.82, render/busy/ms 52.38, main/busy/ms 47.76, stage/main/update 25.87, stage/render/render 16.52, render/graph/ms 14.68, stage/main/post_update 14.59, stage/render/prepare_assets 12.07, stage/render/graph_submit 10.05, encode//sum 8.30 |
| warm-up | 4.75 | 29.6 | pipelines/compile_ms 227, pipelines/front_ms 221, main/busy/ms 56.51, render/thread/ms 53.14, render/busy/ms 51.69, stage/main/update 28.04, stage/main/post_update 19.49, stage/render/render 16.32, render/graph/ms 14.13, stage/render/prepare_assets 12.37, stage/render/graph_submit 10.11, encode//sum 8.21 |
| warm-up | 4.78 | 34.0 | pipelines/compile_ms 652, pipelines/front_ms 636, main/busy/ms 60.56, render/thread/ms 52.60, render/busy/ms 51.03, stage/main/update 31.70, stage/main/post_update 17.97, stage/render/render 16.45, pipelines/driver_ms 16.19, render/graph/ms 13.82, stage/render/prepare_assets 11.12, encode//sum 10.38 |
| warm-up | 4.97 | 26.4 | pipelines/compile_ms 243, pipelines/front_ms 235, main/busy/ms 38.81, render/thread/ms 33.17, render/busy/ms 31.76, stage/main/update 21.32, stage/render/render 16.18, render/graph/ms 14.22, stage/main/post_update 10.50, stage/render/graph_submit 9.02, pipelines/driver_ms 8.50, encode//sum 8.26 |
| warm-up | 5.03 | 27.6 | pipelines/compile_ms 462, pipelines/front_ms 451, render/thread/ms 47.47, render/busy/ms 46.07, main/busy/ms 43.81, stage/main/update 19.97, stage/main/post_update 14.59, stage/render/prepare_assets 12.64, stage/render/render 11.66, pipelines/driver_ms 11.64, render/graph/ms 9.73, stage/render/prepare_resources 8.92 |
| warm-up | 5.07 | 42.0 | pipelines/compile_ms 1074, pipelines/front_ms 1043, render/thread/ms 66.37, render/busy/ms 64.94, main/busy/ms 57.13, stage/main/update 31.80, pipelines/driver_ms 31.59, stage/render/prepare_resources 19.49, stage/main/post_update 16.01, stage/render/render 13.88, render/graph/ms 12.08, stage/render/prepare_assets 10.10 |
| warm-up | 5.10 | 33.5 | pipelines/compile_ms 1444, pipelines/front_ms 1402, render/thread/ms 71.93, render/busy/ms 70.44, main/busy/ms 55.98, pipelines/driver_ms 41.12, stage/main/update 33.02, stage/render/prepare_assets 19.34, stage/render/render 15.38, stage/main/post_update 15.10, render/graph/ms 13.18, stage/render/prepare_resources 12.14 |
| warm-up | 5.14 | 42.8 | pipelines/compile_ms 652, pipelines/front_ms 633, render/thread/ms 72.71, render/busy/ms 71.07, main/busy/ms 55.64, stage/main/post_update 27.65, stage/main/update 20.30, pipelines/driver_ms 19.59, stage/render/prepare_assets 18.22, stage/render/render 15.00, stage/render/prepare_resources 13.71, render/graph/ms 12.23 |
| warm-up | 5.20 | 34.0 | pipelines/compile_ms 203, pipelines/front_ms 196, render/thread/ms 52.15, render/busy/ms 50.59, main/busy/ms 45.28, stage/main/update 25.47, stage/render/prepare_assets 23.66, stage/render/render 14.19, render/graph/ms 12.12, stage/main/post_update 10.76, stage/render/graph_submit 8.87, encode//sum 8.22 |
| warm-up | 5.43 | 33.9 | render/thread/ms 52.20, render/busy/ms 50.44, main/busy/ms 36.05, stage/render/render 18.53, stage/render/prepare_assets 18.06, render/graph/ms 15.73, stage/main/post_update 14.66, stage/main/update 13.60, encode//sum 12.79, stage/render/graph_submit 10.66, encode/submit 6.99, stage/render/camera_driver 4.68 |
| warm-up | 5.46 | 32.7 | render/thread/ms 61.23, render/busy/ms 59.60, main/busy/ms 40.24, stage/render/prepare_assets 30.32, stage/render/render 17.06, stage/main/post_update 15.98, stage/main/update 15.07, render/graph/ms 14.43, encode//sum 12.27, stage/render/graph_submit 9.90, encode/submit 6.38, stage/main/fixed_main_loop 5.03 |
| warm-up | 5.51 | 45.7 | render/thread/ms 72.99, render/busy/ms 71.07, main/busy/ms 44.65, pipelines/compile_ms 41.10, pipelines/front_ms 39.31, stage/render/prepare_assets 36.52, stage/render/render 20.69, stage/main/update 18.64, render/graph/ms 17.87, stage/main/post_update 15.91, stage/render/graph_submit 13.01, encode//sum 11.77 |
| warm-up | 5.56 | 27.7 | render/thread/ms 49.54, render/busy/ms 48.45, stage/render/prepare_assets 35.05, main/busy/ms 32.70, stage/main/update 14.34, stage/main/post_update 10.13, stage/render/render 7.90, encode//sum 6.61, render/graph/ms 6.34, stage/render/graph_submit 4.28, stage/main/fixed_main_loop 3.17, scene/load/wanted/ms 2.97 |
| warm-up | 5.62 | 26.6 | main/busy/ms 41.79, render/thread/ms 40.92, render/busy/ms 39.43, pipelines/compile_ms 22.85, stage/main/update 22.34, pipelines/front_ms 21.70, stage/render/render 13.98, stage/main/post_update 13.91, render/graph/ms 12.23, encode//sum 9.12, stage/render/graph_submit 8.63, stage/render/prepare_meshes 5.73 |
| warm-up | 5.66 | 41.0 | render/thread/ms 61.00, render/busy/ms 59.17, main/busy/ms 51.62, pipelines/compile_ms 47.45, pipelines/front_ms 44.99, stage/main/update 31.57, stage/render/prepare_assets 18.56, stage/render/render 16.60, render/graph/ms 14.01, stage/main/post_update 13.65, encode//sum 9.76, stage/render/prepare_meshes 9.34 |
| warm-up | 5.69 | 30.0 | render/thread/ms 65.64, render/busy/ms 63.90, main/busy/ms 54.29, pipelines/compile_ms 47.45, pipelines/front_ms 44.99, stage/render/prepare_assets 29.56, stage/main/update 29.05, stage/main/post_update 14.85, stage/render/render 14.55, render/graph/ms 11.99, encode//sum 9.10, stage/render/graph_submit 7.60 |
| warm-up | 5.73 | 35.4 | pipelines/compile_ms 165, pipelines/front_ms 161, render/thread/ms 60.21, render/busy/ms 58.84, main/busy/ms 52.84, stage/render/prepare_assets 31.33, stage/main/update 29.76, stage/render/render 12.39, stage/main/post_update 11.81, render/graph/ms 10.48, stream/drive/ms 8.64, encode//sum 7.74 |
| warm-up | 5.76 | 28.1 | pipelines/compile_ms 370, pipelines/front_ms 358, render/thread/ms 59.91, render/busy/ms 58.48, main/busy/ms 56.84, stage/main/update 38.64, stage/render/prepare_assets 22.97, stage/render/render 14.93, render/graph/ms 12.95, pipelines/driver_ms 12.82, stage/main/post_update 10.57, character/finish/ms 9.40 |
| warm-up | 5.87 | 35.7 | render/thread/ms 48.54, render/busy/ms 46.79, main/busy/ms 42.06, pipelines/wait_ms 38.85, stage/render/render 17.88, stage/main/post_update 17.57, stage/main/update 17.45, render/graph/ms 15.43, stage/render/prepare_assets 14.09, pipelines/compile_ms 11.75, stage/render/graph_submit 10.92, pipelines/front_ms 10.56 |
| warm-up | 5.93 | 40.3 | pipelines/compile_ms 257, pipelines/front_ms 245, render/thread/ms 55.70, render/busy/ms 54.40, main/busy/ms 50.09, stage/render/prepare_assets 29.41, stage/main/update 24.67, stage/main/post_update 14.95, pipelines/driver_ms 12.57, stage/render/render 12.05, render/graph/ms 10.15, encode//sum 9.05 |
| warm-up | 5.99 | 29.6 | pipelines/compile_ms 191, pipelines/front_ms 186, render/thread/ms 50.65, render/busy/ms 48.69, main/busy/ms 45.83, stage/main/update 23.75, stage/render/render 19.71, render/graph/ms 17.44, stage/main/post_update 15.23, stage/render/prepare_assets 13.75, stage/render/graph_submit 13.09, encode//sum 11.43 |
| warm-up | 6.25 | 28.2 | render/thread/ms 40.51, render/busy/ms 39.15, main/busy/ms 36.44, stage/render/render 18.32, stage/main/update 17.16, render/graph/ms 16.54, stage/main/post_update 14.50, pipelines/compile_ms 13.76, pipelines/front_ms 13.06, encode//sum 12.19, stage/render/graph_submit 11.30, encode/submit 7.41 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\gc_ascadian@2560x1440_arm=off-2 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (285 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **97%** (min 85%) · 2625 MHz (min 2625) · memory 8101 MHz · 86.9 W · 73 °C (max 75) · VRAM 5890 MiB · throttled 100% of samples

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
