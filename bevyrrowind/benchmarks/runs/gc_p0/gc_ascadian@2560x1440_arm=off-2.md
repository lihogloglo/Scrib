# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1426 frames over 15.0 s · **median 10.44 ms (96 fps)** · mean 10.52 · p95 12.11 · p99 13.14 (1% low 76 fps) · worst 18.27 ms · 3 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 308 frames over 9.1 s, worst 2479.6 ms
- hitches (> 26.1 ms): **0** measured, 48 in warm-up
- timed GPU passes: 8.44 ms of a 10.52 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.44 ms, limit 11 ms
- pass worst frame 18.27 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 1.994 | 2.068 | 2.106 | 0.96 | 0.063 |
| dlss_super_resolution | 1.988 | 2.061 | 2.085 | 0.96 | 0.180 |
| early prepass | 0.726 | 0.752 | 0.766 | 0.96 | 0.094 |
| sun_visibility | 0.666 | 0.691 | 0.707 | 0.96 | 0.006 |
| atmosphere_luts | 0.346 | 0.358 | 0.362 | 0.96 | 0.003 |
| water_velocity | 0.317 | 0.329 | 0.333 | 0.96 | 0.002 |
| bloom | 0.309 | 0.320 | 0.337 | 0.96 | 0.063 |
| atmos_froxel_inject | 0.295 | 0.306 | 0.310 | 0.96 | 0.001 |
| water_sspr | 0.202 | 0.210 | 0.203 | 0.96 | 0.002 |
| ssao | 0.182 | 0.188 | 0.197 | 0.96 | 0.007 |
| uniform_allocation | 0.136 | 0.141 | 0.184 | 0.96 | 0.022 |
| water_fft | 0.130 | 0.134 | 0.136 | 0.96 | 0.003 |
| upscaling | 0.118 | 0.122 | 0.127 | 0.96 | 0.004 |
| clouds_composite | 0.105 | 0.109 | 0.111 | 0.96 | 0.002 |
| bin_unpacking | 0.099 | 0.102 | 0.133 | 0.96 | 0.016 |
| tonemapping | 0.083 | 0.086 | 0.089 | 0.96 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.080 | 0.093 | 0.96 | 0.009 |
| gpu_instance_materialization | 0.077 | 0.079 | 0.091 | 0.96 | 0.004 |
| clustering | 0.067 | 0.070 | 0.080 | 0.96 | 0.019 |
| early_mesh_preprocessing | 0.062 | 0.065 | 0.096 | 0.96 | 0.004 |
| render_sky | 0.061 | 0.063 | 0.065 | 0.96 | 0.002 |
| clouds_trace | 0.055 | 0.057 | 0.067 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.411 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.046 | 0.048 | 0.050 | 0.96 | 0.001 |
| point_shadow_atlas | 0.036 | 0.073 | 0.080 | 0.49 | 0.135 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.220 | 0.231 | 0.12 | 0.004 |
| water_snapshot | 0.023 | 0.024 | 0.025 | 0.96 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.96 | 0.002 |
| range_unpacking | 0.021 | 0.022 | 0.027 | 0.96 | 0.003 |
| clouds_shadow | 0.020 | 0.021 | 0.023 | 0.96 | 0.003 |
| per_view_shadow_pass | 0.019 | 0.020 | 0.037 | 0.96 | 0.006 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.025 | 0.96 | 0.232 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.96 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.015 | 0.016 | 0.49 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.96 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.49 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.49 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.49 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.96 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.216 | 10.795 | 17.10 | 13142.1 |
| render/busy/ms | 8.251 | 9.875 | 16.62 | 11765.9 |
| main/busy/ms | 8.025 | 8.992 | 11.48 | 11442.9 |
| stage/render/render | 5.274 | 6.765 | 12.82 | 7521.3 |
| render/graph/ms | 4.653 | 5.851 | 12.22 | 6635.1 |
| stage/main/post_update | 4.564 | 5.399 | 7.38 | 6508.3 |
| encode//sum | 4.209 | 6.159 | 18.31 | 6001.4 |
| stage/render/graph_submit | 3.360 | 4.449 | 10.71 | 4790.7 |
| encode/submit | 2.086 | 2.877 | 6.23 | 2974.7 |
| stage/main/update | 1.763 | 2.094 | 2.51 | 2514.7 |
| encode/finish_wall | 1.321 | 1.895 | 7.48 | 1883.7 |
| stage/render/camera_driver | 1.230 | 1.423 | 1.83 | 1754.1 |
| encode/pass:shadow_directional_light/sum | 0.874 | 1.526 | 6.26 | 1246.8 |
| encode/pass:early prepass/sum | 0.823 | 1.156 | 7.07 | 1173.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.797 | 1.122 | 1.68 | 1136.9 |
| stage/render/prepare_views | 0.637 | 2.142 | 3.71 | 908.1 |
| render/acquire/ms | 0.594 | 2.089 | 3.68 | 846.8 |
| stage/render/prepare_assets | 0.567 | 0.696 | 1.57 | 808.6 |
| bigspace/total/ms | 0.534 | 1.200 | 2.72 | 760.8 |
| stage/render/prepare_write_phase_buffers | 0.505 | 0.740 | 1.07 | 719.7 |
| stage/main/fixed_main_loop | 0.463 | 0.791 | 1.40 | 659.5 |
| stage/render/prepare_meshes | 0.460 | 0.633 | 1.18 | 656.3 |
| stage/render/prepare_resources | 0.419 | 0.486 | 1.09 | 597.5 |
| stage/main/pre_update | 0.404 | 0.523 | 1.26 | 576.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.398 | 0.602 | 0.95 | 567.9 |
| render/present/ms | 0.371 | 0.471 | 2.45 | 529.5 |
| stage/main/first | 0.369 | 0.448 | 1.92 | 526.1 |
| controller/feed/ms | 0.323 | 0.403 | 0.81 | 460.2 |
| stage/render/prepare_flush | 0.318 | 0.547 | 0.87 | 453.7 |
| stage/render/prepare_batch_phases | 0.311 | 0.429 | 0.61 | 443.6 |
| stage/main/last | 0.277 | 0.338 | 0.73 | 395.3 |
| stage/render/prepare_bind_groups | 0.267 | 0.398 | 0.62 | 381.0 |
| bigspace/low_precision/ms | 0.231 | 0.646 | 1.62 | 329.9 |
| stage/render/create_views | 0.189 | 0.228 | 0.66 | 269.4 |
| stream/drive/ms | 0.180 | 0.251 | 0.56 | 257.4 |
| stage/render/extract_commands | 0.173 | 0.204 | 0.54 | 247.0 |
| pipelines/compile_ms | 0.172 | 0.000 | 123.36 | 245.4 |
| pipelines/front_ms | 0.168 | 0.000 | 120.39 | 239.8 |
| anim/evaluate_cpu/ms | 0.168 | 0.198 | 0.32 | 239.2 |
| stage/render/specialize | 0.158 | 0.221 | 0.62 | 225.0 |
| anim/evaluate/ms | 0.155 | 0.298 | 1.67 | 221.2 |
| stream/tick/ms | 0.150 | 0.217 | 0.50 | 213.7 |
| bigspace/high_precision/ms | 0.147 | 0.546 | 2.36 | 210.3 |
| stage/render/queue | 0.131 | 0.172 | 0.26 | 186.2 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.120 | 0.342 | 0.99 | 171.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.107 | 0.175 | 0.50 | 152.6 |
| controller/write/ms | 0.065 | 0.083 | 0.55 | 92.9 |
| controller/move/ms | 0.050 | 0.096 | 0.54 | 71.5 |
| encode/resolve_diagnostics/sum | 0.047 | 0.083 | 0.45 | 66.9 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.072 | 0.27 | 63.8 |
| physics/step/ms | 0.037 | 0.066 | 0.50 | 52.2 |
| terrain/update/ms | 0.035 | 0.047 | 0.09 | 49.6 |
| stage/render/phase_sort | 0.031 | 0.049 | 0.08 | 44.5 |
| stage/render/cleanup | 0.024 | 0.032 | 0.29 | 33.7 |
| encode/camera_driver_span_begin/sum | 0.019 | 0.037 | 0.11 | 27.8 |
| encode/camera_driver_span_end/sum | 0.017 | 0.034 | 0.38 | 24.9 |
| weather/map/ms | 0.016 | 0.022 | 0.05 | 22.8 |
| anim/apply/ms | 0.010 | 0.013 | 0.03 | 14.3 |
| controller/avoid/ms | 0.010 | 0.019 | 0.38 | 13.8 |
| weather/eval/ms | 0.009 | 0.012 | 0.06 | 13.1 |
| script/update/ms | 0.007 | 0.010 | 0.42 | 10.5 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 6.5 |
| script/ui/ms | 0.004 | 0.006 | 0.07 | 6.3 |
| far/select/ms | 0.004 | 0.005 | 0.04 | 6.1 |
| pipelines/driver_ms | 0.004 | 0.000 | 2.97 | 5.6 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.01 | 2.9 |
| stream/assets_ready/ms | 0.002 | 0.002 | 0.01 | 2.2 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.0 |
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
| crowd/joints_extracted/count | count | 643 | 1113 | 714 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.7 k | 43.7 k | 43.7 k |
| encode//max | ms | 0.53 | 7.41 | 0.64 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.11 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.38 | 0.02 |
| encode/finish_critical | ms | 0.97 | 7.41 | 1.36 |
| encode/pass:early prepass/max | ms | 0.82 | 7.07 | 0.72 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.68 | 1.02 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.50 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.38 | 0.95 | 0.30 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.22 | 0.99 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 6.26 | 1.36 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.45 | 0.04 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 9.38 | 16.62 | 9.36 |
| gpu/timed/ms | ms | 9.37 | 10.63 | 9.36 |
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
| phases/shadow/bins | count | 2180 | 2180 | 2180 |
| phases/shadow/instance_batches | count | 4 | 12 | 12 |
| phases/shadow/instance_draws | count | 7312 | 21.9 k | 21.9 k |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5352 | 5359 | 5352 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 48 | 48 | 48 |
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
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 9 | 67 | 67 |
| preprocess/bind_groups_created | count | 4 | 60 | 60 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 27.10 | 43.27 | 31.28 |
| process/mem_usage | GiB | 3.10 | 3.21 | 3.21 |
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
| shadow_atlas/movers | count | 100 | 100 | 100 |
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
| skin/blocks_changed/count | count | 16 | 25 | 17 |
| skin/blocks_skipped/count | count | 194 | 200 | 192 |
| skin/blocks_walked/count | count | 16 | 25 | 18 |
| skin/joint_bytes_written/bytes | bytes | 49.9 k | 76.8 k | 51.5 k |
| skin/joints_written/count | count | 780 | 1200 | 804 |
| skin/upload_writes/count | count | 10 | 14 | 12 |
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
| system/cpu_usage | % | 31.49 | 41.13 | 35.14 |
| system/mem_usage | % | 58.52 | 58.88 | 58.88 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2950 | 2950 | 2950 |
| textures/images | count | 2500 | 2500 | 2500 |
| views/all/batch_sets | count | 146 | 167 | 167 |
| views/all/bins | count | 5610 | 5610 | 5610 |
| views/all/count | count | 10 | 10 | 10 |
| views/all/instance_batches | count | 12 | 20 | 20 |
| views/all/instance_draws | count | 21.9 k | 36.6 k | 36.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 245.8 k |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 98 | 98 | 98 |
| views/all/pipelines_distinct | count | 69 | 69 | 69 |
| views/all/work_items | count | 154.9 k | 258.2 k | 258.2 k |
| views/cascade/batch_sets | count | 9 | 45 | 45 |
| views/cascade/bins | count | 2028 | 2028 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 12 |
| views/cascade/instance_draws | count | 7312 | 21.9 k | 21.9 k |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 5200 | 5207 | 5200 |
| views/cascade/pipelines | count | 37 | 37 | 37 |
| views/cascade/work_items | count | 49.3 k | 152.7 k | 152.6 k |
| views/lamp_face/batch_sets | count | 61 | 91 | 61 |
| views/lamp_face/bins | count | 152 | 152 | 152 |
| views/lamp_face/count | count | 6 | 6 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 152 | 152 | 152 |
| views/lamp_face/pipelines | count | 11 | 11 | 11 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 61 | 91 | 61 |
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
| anim/bones_written/count | count | 38.70 | 79 | 55.2 k |
| anim/events/count | count | 0.03 | 1 | 49 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.52 | 8 | 3600 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 30 | 40.3 k |
| encode/bind_walk/init_textures | count | 9084 | 12.8 k | 12.95 M |
| encode/bind_walk/init_textures_skipped | count | 47.6 k | 56.7 k | 67.85 M |
| encode/bind_walk/inits | count | 653 | 742 | 931.6 k |
| encode/bind_walk/inits_skipped | count | 166 | 182 | 237.1 k |
| encode/bind_walk/merge_textures | count | 9022 | 12.7 k | 12.86 M |
| encode/bind_walk/merge_textures_skipped | count | 7519 | 13.0 k | 10.72 M |
| encode/bind_walk/merges | count | 115 | 132 | 164.4 k |
| encode/bind_walk/merges_skipped | count | 153 | 174 | 218.6 k |
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
| encode/queue/write_buffer/bytes | bytes | 2.71 M | 3.48 M | 3.87 G |
| encode/queue/write_buffer/calls | count | 264 | 336 | 376.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 53.9 k | 53.9 k | 76.84 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 12.8 k |
| encode/queue/write_texture/bytes | bytes | 18.5 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1426 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.63 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 1 |
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

## Hitches (> 26.1 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2479.6 | main/busy/ms 2446, stage/main/post_update 2401, stage/main/update 31.63, stage/main/pre_update 8.85, surface/advance_cpu_ms 1.39, stage/main/last 1.27, stage/main/first 0.62, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.33, surface/cell_upload_ms 0.32, bigspace/total/ms 0.11, stream/tick/ms 0.09 |
| warm-up | 3.35 | 870.7 | main/busy/ms 2492, stage/main/post_update 2433, render/thread/ms 806, render/busy/ms 666, stage/render/prepare_assets 615, stage/render/prepare_views 138, render/acquire/ms 137, stage/main/update 40.03, stage/render/create_views 31.03, stage/render/render 11.50, stage/main/pre_update 10.72, render/graph/ms 7.65 |
| warm-up | 3.43 | 78.8 | render/thread/ms 882, render/busy/ms 684, stage/render/prepare_assets 620, stage/render/prepare_views 138, render/acquire/ms 138, pipelines/compile_ms 110, main/busy/ms 80.28, stage/render/render 76.01, pipelines/front_ms 69.91, render/present/ms 60.53, stage/main/post_update 45.82, pipelines/driver_ms 39.70 |
| warm-up | 3.50 | 69.5 | pipelines/compile_ms 507, pipelines/front_ms 431, pipelines/wait_ms 202, render/thread/ms 144, stage/render/render 101, render/busy/ms 79.71, pipelines/driver_ms 76.32, render/present/ms 63.36, main/busy/ms 53.33, render/graph/ms 36.06, terrain/update/ms 29.62, stage/main/post_update 26.40 |
| warm-up | 3.65 | 149.3 | pipelines/wait_ms 4794, pipelines/compile_ms 1585, pipelines/front_ms 1478, render/thread/ms 186, render/busy/ms 178, main/busy/ms 140, pipelines/driver_ms 107, stage/render/render 74.09, encode//sum 69.05, render/graph/ms 65.43, stage/main/update 60.16, stage/render/graph_submit 39.08 |
| warm-up | 3.74 | 90.5 | pipelines/wait_ms 4792, pipelines/compile_ms 2301, pipelines/front_ms 2177, render/thread/ms 203, render/busy/ms 200, main/busy/ms 143, pipelines/driver_ms 124, encode//sum 73.30, stage/render/prepare_assets 65.45, stage/main/update 62.98, stage/render/render 59.80, render/graph/ms 55.92 |
| warm-up | 3.78 | 44.3 | pipelines/compile_ms 1553, pipelines/front_ms 1479, pipelines/wait_ms 267, render/thread/ms 126, render/busy/ms 124, pipelines/driver_ms 73.78, stage/render/prepare_assets 68.06, main/busy/ms 45.55, stage/render/render 39.10, render/graph/ms 36.16, stage/render/graph_submit 27.34, encode//sum 19.86 |
| warm-up | 3.83 | 43.0 | pipelines/compile_ms 672, pipelines/front_ms 645, render/thread/ms 82.28, render/busy/ms 80.53, pipelines/wait_ms 78.35, main/busy/ms 47.25, stage/render/prepare_assets 31.18, pipelines/driver_ms 27.12, stage/render/render 21.33, encode//sum 19.08, render/graph/ms 18.99, stage/main/post_update 18.64 |
| warm-up | 3.85 | 28.3 | pipelines/compile_ms 1285, pipelines/front_ms 1227, pipelines/wait_ms 116, render/thread/ms 67.48, render/busy/ms 66.09, pipelines/driver_ms 58.45, main/busy/ms 43.55, stage/render/prepare_assets 25.11, stage/main/post_update 18.80, stage/render/prepare_resources 18.28, stage/main/update 10.09, stage/render/render 9.96 |
| warm-up | 3.90 | 42.1 | pipelines/compile_ms 1274, pipelines/front_ms 1212, pipelines/wait_ms 107, render/thread/ms 67.04, render/busy/ms 64.94, pipelines/driver_ms 61.67, main/busy/ms 51.51, stage/main/post_update 26.74, stage/render/prepare_resources 22.62, stage/render/render 15.54, stage/main/update 13.16, render/graph/ms 12.44 |
| warm-up | 3.93 | 35.5 | pipelines/compile_ms 829, pipelines/front_ms 785, render/thread/ms 73.04, render/busy/ms 70.56, main/busy/ms 48.31, pipelines/driver_ms 44.50, stage/main/post_update 28.33, stage/render/prepare_assets 21.67, stage/render/render 19.35, stage/render/prepare_resources 17.63, render/graph/ms 15.93, stage/main/update 12.69 |
| warm-up | 3.97 | 40.2 | pipelines/compile_ms 662, pipelines/front_ms 626, render/thread/ms 71.47, render/busy/ms 69.24, stage/render/prepare_assets 41.55, pipelines/driver_ms 36.64, main/busy/ms 28.55, stage/render/render 18.50, render/graph/ms 15.70, stage/render/graph_submit 11.80, stage/main/post_update 10.81, encode//sum 10.11 |
| warm-up | 4.05 | 79.9 | pipelines/compile_ms 1065, pipelines/front_ms 941, pipelines/driver_ms 123, render/thread/ms 117, render/busy/ms 115, stage/render/prepare_assets 95.52, main/busy/ms 28.25, pipelines/wait_ms 17.26, stage/render/render 13.57, render/graph/ms 11.26, stage/main/post_update 8.52, stage/main/update 8.48 |
| warm-up | 4.10 | 44.6 | pipelines/compile_ms 1100, pipelines/front_ms 974, pipelines/driver_ms 125, render/thread/ms 120, render/busy/ms 119, stage/render/prepare_assets 105, main/busy/ms 33.63, pipelines/wait_ms 18.54, stage/main/update 14.24, stage/render/render 9.70, stage/main/post_update 8.29, render/graph/ms 7.77 |
| warm-up | 4.13 | 29.4 | pipelines/compile_ms 298, pipelines/front_ms 288, render/thread/ms 70.32, render/busy/ms 68.68, stage/render/prepare_assets 53.79, main/busy/ms 27.61, stage/main/update 13.97, stage/render/render 10.90, pipelines/driver_ms 10.26, render/graph/ms 8.90, encode//sum 6.90, stage/main/post_update 6.34 |
| warm-up | 4.20 | 28.1 | pipelines/compile_ms 101, pipelines/front_ms 96.43, render/thread/ms 43.59, render/busy/ms 41.96, main/busy/ms 37.31, stage/main/update 24.23, stage/render/prepare_assets 19.43, stage/render/render 13.37, render/graph/ms 11.43, stage/main/post_update 9.02, stage/render/graph_submit 7.84, encode//sum 6.24 |
| warm-up | 4.28 | 32.3 | pipelines/compile_ms 879, pipelines/front_ms 850, render/thread/ms 54.17, render/busy/ms 52.06, main/busy/ms 45.89, pipelines/driver_ms 29.52, stage/main/update 22.61, stage/render/render 16.90, stage/main/post_update 14.97, render/graph/ms 14.46, stage/render/prepare_resources 11.64, stage/render/prepare_assets 11.34 |
| warm-up | 4.34 | 54.4 | pipelines/compile_ms 155, pipelines/front_ms 149, render/thread/ms 83.17, render/busy/ms 81.58, main/busy/ms 75.18, stage/main/update 37.06, stage/render/render 35.49, render/graph/ms 33.54, stage/main/post_update 31.64, stage/render/graph_submit 24.66, stage/render/prepare_assets 23.92, encode/finish_wall 20.31 |
| warm-up | 4.37 | 34.6 | pipelines/compile_ms 304, pipelines/front_ms 294, render/thread/ms 85.85, render/busy/ms 84.28, main/busy/ms 79.77, stage/main/update 41.42, stage/render/prepare_assets 35.45, stage/render/render 34.75, render/graph/ms 30.60, stage/main/post_update 30.40, stage/render/graph_submit 23.59, encode/finish_wall 19.42 |
| warm-up | 4.45 | 79.9 | pipelines/compile_ms 1546, pipelines/front_ms 1506, render/thread/ms 112, render/busy/ms 110, main/busy/ms 63.80, pipelines/driver_ms 39.45, stage/render/prepare_assets 30.26, stage/main/update 27.56, stage/render/prepare_resources 20.50, stage/main/post_update 19.69, stage/render/prepare_bind_groups 17.09, stage/render/render 15.21 |
| warm-up | 4.57 | 119.2 | pipelines/compile_ms 3092, pipelines/front_ms 3006, pipelines/wait_ms 214, render/thread/ms 196, render/busy/ms 195, main/busy/ms 110, pipelines/driver_ms 86.64, stage/render/prepare_resources 86.40, stage/main/update 59.48, stage/render/prepare_bind_groups 27.24, stage/main/post_update 23.34, stage/render/prepare_assets 21.52 |
| warm-up | 4.64 | 69.8 | pipelines/compile_ms 2421, pipelines/front_ms 2353, pipelines/wait_ms 222, render/thread/ms 185, render/busy/ms 183, main/busy/ms 128, stage/main/update 74.11, stage/render/prepare_resources 68.75, pipelines/driver_ms 68.31, stage/render/prepare_assets 59.50, stage/main/pre_update 22.80, stage/main/post_update 20.84 |
| warm-up | 4.72 | 79.7 | pipelines/compile_ms 1729, pipelines/front_ms 1680, render/thread/ms 145, render/busy/ms 143, stage/render/prepare_assets 110, main/busy/ms 104, pipelines/wait_ms 102, stage/main/update 51.01, pipelines/driver_ms 48.36, stage/main/post_update 27.33, character/finish/ms 15.80, stage/render/render 15.60 |
| warm-up | 4.82 | 97.7 | pipelines/compile_ms 2118, pipelines/front_ms 2055, pipelines/wait_ms 247, render/thread/ms 173, render/busy/ms 171, stage/render/prepare_assets 129, main/busy/ms 81.71, pipelines/driver_ms 63.12, stage/main/update 41.97, stage/render/render 19.70, stage/main/post_update 19.48, render/graph/ms 16.93 |
| warm-up | 4.85 | 31.8 | pipelines/compile_ms 1719, pipelines/front_ms 1670, pipelines/wait_ms 175, render/thread/ms 126, render/busy/ms 123, stage/render/prepare_assets 75.12, main/busy/ms 54.83, pipelines/driver_ms 48.77, stage/main/update 25.27, stage/render/render 24.74, render/graph/ms 21.88, stage/render/graph_submit 17.57 |
| warm-up | 4.90 | 26.6 | pipelines/compile_ms 243, pipelines/front_ms 234, render/thread/ms 46.37, render/busy/ms 45.08, main/busy/ms 37.53, stage/main/update 15.75, stage/main/post_update 14.49, stage/render/render 13.72, render/graph/ms 11.76, stage/render/prepare_resources 10.39, stage/render/prepare_assets 9.43, pipelines/driver_ms 8.38 |
| warm-up | 4.99 | 31.2 | pipelines/compile_ms 215, pipelines/front_ms 209, main/busy/ms 34.56, render/thread/ms 30.18, render/busy/ms 28.64, stage/main/update 18.21, stage/render/render 10.80, stage/render/prepare_assets 10.06, stage/main/post_update 9.20, render/graph/ms 8.63, encode//sum 6.85, pipelines/driver_ms 5.68 |
| warm-up | 5.19 | 43.7 | pipelines/compile_ms 1140, pipelines/front_ms 1114, render/thread/ms 62.82, render/busy/ms 61.51, main/busy/ms 41.57, pipelines/driver_ms 25.57, stage/main/post_update 23.18, stage/render/prepare_assets 21.06, stage/render/prepare_resources 16.78, stage/main/update 11.88, stage/render/render 11.58, render/graph/ms 9.66 |
| warm-up | 5.25 | 40.1 | pipelines/compile_ms 433, pipelines/front_ms 419, render/thread/ms 56.96, render/busy/ms 55.45, main/busy/ms 52.64, stage/main/update 23.30, stage/main/post_update 19.71, stage/render/render 18.02, render/graph/ms 16.08, pipelines/driver_ms 13.74, stage/render/prepare_resources 12.60, stage/render/graph_submit 8.76 |
| warm-up | 5.28 | 29.5 | pipelines/compile_ms 261, pipelines/front_ms 252, render/thread/ms 64.15, render/busy/ms 62.74, main/busy/ms 60.86, stage/main/update 26.32, stage/main/post_update 15.73, stage/render/prepare_assets 13.98, stage/render/render 13.35, stage/render/prepare_resources 13.33, render/graph/ms 11.53, character/finish/ms 10.92 |
| warm-up | 5.33 | 47.8 | pipelines/compile_ms 1002, pipelines/front_ms 974, render/thread/ms 70.80, render/busy/ms 69.35, main/busy/ms 65.79, stage/main/post_update 28.43, pipelines/driver_ms 27.16, stage/render/prepare_assets 22.96, stage/main/update 17.96, stage/render/prepare_resources 17.15, stage/render/render 12.82, render/graph/ms 10.75 |
| warm-up | 5.38 | 31.8 | pipelines/compile_ms 532, pipelines/front_ms 516, render/thread/ms 47.62, render/busy/ms 45.89, main/busy/ms 41.36, stage/render/prepare_assets 20.78, stage/main/update 16.91, pipelines/driver_ms 15.86, stage/main/post_update 15.75, stage/render/render 14.25, render/graph/ms 11.87, stage/render/graph_submit 8.57 |
| warm-up | 5.55 | 32.8 | pipelines/compile_ms 358, pipelines/front_ms 349, render/thread/ms 52.79, render/busy/ms 50.59, main/busy/ms 35.84, stage/render/render 17.88, stage/render/prepare_assets 17.80, stage/main/update 16.07, render/graph/ms 14.71, stage/main/post_update 13.76, encode//sum 11.30, stage/render/graph_submit 10.83 |
| warm-up | 5.58 | 27.0 | pipelines/compile_ms 185, pipelines/front_ms 181, render/thread/ms 55.06, render/busy/ms 52.40, main/busy/ms 37.72, stage/render/prepare_assets 23.48, stage/render/render 16.44, stage/main/post_update 14.60, stage/main/update 14.28, render/graph/ms 12.85, encode//sum 10.91, stage/render/graph_submit 8.26 |
| warm-up | 5.60 | 28.8 | pipelines/compile_ms 341, pipelines/front_ms 332, render/thread/ms 51.13, render/busy/ms 49.41, main/busy/ms 38.65, stage/render/prepare_assets 27.60, stage/main/update 16.16, stage/main/post_update 13.29, stage/render/render 11.90, render/graph/ms 9.70, pipelines/driver_ms 9.44, encode//sum 8.53 |
| warm-up | 5.64 | 34.1 | pipelines/compile_ms 341, pipelines/front_ms 332, render/thread/ms 59.54, render/busy/ms 57.97, main/busy/ms 36.92, stage/render/prepare_assets 24.87, stage/render/render 17.29, stage/main/post_update 15.89, render/graph/ms 15.23, stage/main/update 13.86, stage/render/graph_submit 11.68, encode//sum 9.81 |
| warm-up | 5.67 | 28.5 | render/thread/ms 57.82, render/busy/ms 55.88, main/busy/ms 38.28, stage/render/render 19.51, stage/render/prepare_assets 18.91, render/graph/ms 16.73, stage/main/post_update 16.40, stage/main/update 12.41, stage/render/graph_submit 12.32, encode//sum 11.56, encode/submit 8.36, stage/main/fixed_main_loop 4.25 |
| warm-up | 5.70 | 30.3 | render/thread/ms 53.19, render/busy/ms 51.87, main/busy/ms 37.45, stage/render/prepare_assets 32.15, stage/main/update 15.28, stage/main/post_update 11.95, stage/render/render 10.99, render/graph/ms 8.98, encode//sum 8.23, stage/render/graph_submit 5.96, stage/main/fixed_main_loop 4.36, scene/load/wanted/ms 3.73 |
| warm-up | 5.75 | 29.2 | render/thread/ms 48.81, render/busy/ms 47.78, main/busy/ms 36.39, stage/render/prepare_assets 17.12, stage/main/update 14.52, stage/main/post_update 12.78, pipelines/compile_ms 12.75, stage/render/prepare_bind_groups 11.91, pipelines/front_ms 11.70, stage/render/render 10.77, render/graph/ms 9.21, encode//sum 7.10 |
| warm-up | 5.81 | 40.2 | render/thread/ms 60.64, render/busy/ms 58.83, main/busy/ms 51.60, pipelines/compile_ms 38.89, pipelines/front_ms 36.72, stage/main/update 30.61, stage/render/prepare_assets 20.03, stage/render/render 15.40, stage/main/post_update 14.33, render/graph/ms 12.99, encode//sum 9.14, stage/render/graph_submit 8.89 |
| warm-up | 5.85 | 33.4 | render/thread/ms 68.33, pipelines/compile_ms 67.72, render/busy/ms 66.65, pipelines/front_ms 64.20, main/busy/ms 53.61, stage/render/prepare_assets 29.27, stage/main/update 28.59, pipelines/wait_ms 23.87, stage/main/post_update 15.38, stage/render/render 13.36, render/graph/ms 10.98, encode//sum 9.14 |
| warm-up | 5.88 | 36.2 | render/thread/ms 64.42, render/busy/ms 63.25, main/busy/ms 49.97, stage/render/prepare_assets 33.94, pipelines/compile_ms 28.83, stage/main/update 28.56, pipelines/front_ms 27.49, pipelines/wait_ms 23.73, stage/main/post_update 12.06, stage/render/render 10.43, render/graph/ms 8.79, encode//sum 8.47 |
| warm-up | 6.04 | 35.4 | main/busy/ms 46.76, render/thread/ms 46.31, render/busy/ms 44.76, stage/main/post_update 19.53, stage/main/update 19.21, stage/render/render 16.98, render/graph/ms 14.53, stage/render/prepare_assets 12.46, encode//sum 10.10, stage/render/graph_submit 9.34, stage/render/prepare_meshes 6.10, encode/submit 5.94 |
| warm-up | 6.09 | 44.0 | pipelines/compile_ms 99.44, pipelines/front_ms 90.36, render/thread/ms 71.41, render/busy/ms 69.65, main/busy/ms 66.26, stage/main/update 30.26, stage/render/prepare_assets 28.61, stage/main/post_update 20.22, stage/render/render 18.23, render/graph/ms 15.37, encode//sum 10.49, stage/render/graph_submit 9.28 |
| warm-up | 6.12 | 32.4 | pipelines/compile_ms 440, pipelines/front_ms 422, render/thread/ms 71.27, render/busy/ms 69.85, main/busy/ms 56.42, stage/render/prepare_assets 39.24, stage/main/update 28.37, pipelines/driver_ms 18.32, stage/render/render 13.84, stage/main/post_update 13.06, render/graph/ms 11.83, encode//sum 9.78 |
| warm-up | 6.27 | 27.3 | render/thread/ms 43.55, render/busy/ms 42.24, main/busy/ms 31.29, stage/render/render 15.82, render/graph/ms 14.06, stage/main/update 13.57, stage/main/post_update 12.89, encode//sum 10.44, stage/render/graph_submit 9.87, stage/render/prepare_assets 7.71, stage/render/prepare_meshes 7.46, encode/submit 6.12 |
| warm-up | 6.49 | 26.7 | render/thread/ms 40.25, render/busy/ms 38.94, main/busy/ms 32.94, stage/render/render 18.94, render/graph/ms 16.90, stage/main/update 15.97, stage/render/graph_submit 12.00, stage/main/post_update 11.68, encode//sum 10.33, encode/submit 6.59, stage/render/prepare_meshes 6.15, encode/finish_wall 5.46 |
| warm-up | 6.70 | 29.2 | main/busy/ms 40.69, render/thread/ms 32.12, render/busy/ms 31.05, stage/main/update 20.57, stage/render/render 11.93, stage/main/post_update 11.85, encode//sum 10.75, render/graph/ms 10.35, stage/render/graph_submit 6.29, stage/render/prepare_assets 6.14, stage/render/prepare_meshes 6.02, character/finish/ms 5.50 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0\gc_ascadian@2560x1440_arm=off-2 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 16%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **97%** (min 89%) · 2625 MHz (min 2625) · memory 8101 MHz · 88.9 W · 76 °C (max 79) · VRAM 5698 MiB · throttled 100% of samples

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
