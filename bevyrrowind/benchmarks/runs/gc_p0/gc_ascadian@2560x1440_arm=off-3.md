# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1417 frames over 15.0 s · **median 10.40 ms (96 fps)** · mean 10.59 · p95 12.49 · p99 13.85 (1% low 72 fps) · worst 19.86 ms · 8 spikes over 1.5× median · 28.0 stutters/min (work 8.0/min)
- warm-up: 310 frames over 9.3 s, worst 2476.3 ms
- hitches (> 26.0 ms): **0** measured, 52 in warm-up
- timed GPU passes: 7.98 ms of a 10.59 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.40 ms, limit 11 ms
- pass worst frame 19.86 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 1.885 | 2.067 | 2.101 | 0.91 | 0.083 |
| dlss_super_resolution | 1.878 | 2.060 | 2.083 | 0.91 | 0.177 |
| early prepass | 0.684 | 0.751 | 0.764 | 0.91 | 0.097 |
| sun_visibility | 0.639 | 0.701 | 0.713 | 0.91 | 0.007 |
| atmosphere_luts | 0.325 | 0.357 | 0.361 | 0.91 | 0.004 |
| water_velocity | 0.301 | 0.330 | 0.333 | 0.91 | 0.002 |
| bloom | 0.291 | 0.320 | 0.324 | 0.91 | 0.064 |
| atmos_froxel_inject | 0.278 | 0.305 | 0.309 | 0.91 | 0.002 |
| water_sspr | 0.184 | 0.201 | 0.202 | 0.91 | 0.002 |
| ssao | 0.171 | 0.187 | 0.189 | 0.91 | 0.008 |
| uniform_allocation | 0.128 | 0.141 | 0.184 | 0.91 | 0.027 |
| water_fft | 0.123 | 0.135 | 0.136 | 0.91 | 0.003 |
| upscaling | 0.111 | 0.121 | 0.126 | 0.91 | 0.004 |
| clouds_composite | 0.099 | 0.108 | 0.111 | 0.91 | 0.002 |
| bin_unpacking | 0.092 | 0.101 | 0.131 | 0.91 | 0.018 |
| gpu_instance_materialization | 0.083 | 0.091 | 0.094 | 0.91 | 0.004 |
| tonemapping | 0.078 | 0.085 | 0.089 | 0.91 | 0.004 |
| main_transmissive_pass_3d | 0.072 | 0.079 | 0.091 | 0.91 | 0.009 |
| clustering | 0.063 | 0.069 | 0.076 | 0.91 | 0.020 |
| render_sky | 0.058 | 0.063 | 0.065 | 0.91 | 0.002 |
| early_mesh_preprocessing | 0.057 | 0.063 | 0.093 | 0.91 | 0.005 |
| clouds_trace | 0.052 | 0.057 | 0.068 | 0.91 | 0.001 |
| lightprobe_irradiance_map | 0.049 | 0.412 | 0.416 | 0.12 | 0.001 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.91 | 0.001 |
| point_shadow_atlas | 0.034 | 0.072 | 0.076 | 0.48 | 0.129 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.91 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.219 | 0.229 | 0.12 | 0.004 |
| water_snapshot | 0.021 | 0.023 | 0.025 | 0.91 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| range_unpacking | 0.020 | 0.021 | 0.027 | 0.91 | 0.004 |
| clouds_shadow | 0.019 | 0.021 | 0.024 | 0.91 | 0.004 |
| per_view_shadow_pass | 0.018 | 0.020 | 0.037 | 0.91 | 0.006 |
| main_transparent_pass_3d | 0.014 | 0.016 | 0.026 | 0.91 | 0.241 |
| clouds_fog_light | 0.010 | 0.011 | 0.012 | 0.91 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.015 | 0.016 | 0.48 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.004 | 0.91 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.91 | 0.003 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.007 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.230 | 11.019 | 19.14 | 13078.3 |
| render/busy/ms | 8.486 | 10.067 | 18.19 | 12024.4 |
| main/busy/ms | 8.186 | 9.195 | 10.79 | 11599.1 |
| stage/render/render | 5.375 | 6.973 | 11.77 | 7616.0 |
| render/graph/ms | 4.707 | 5.806 | 10.31 | 6669.8 |
| stage/main/post_update | 4.653 | 5.495 | 6.75 | 6592.7 |
| encode//sum | 4.174 | 5.620 | 10.82 | 5914.9 |
| stage/render/graph_submit | 3.355 | 4.360 | 8.48 | 4754.7 |
| encode/submit | 2.053 | 3.168 | 7.10 | 2908.5 |
| stage/main/update | 1.845 | 2.138 | 2.59 | 2614.7 |
| encode/finish_wall | 1.391 | 2.000 | 3.27 | 1970.9 |
| stage/render/camera_driver | 1.288 | 1.488 | 2.71 | 1824.7 |
| encode/pass:shadow_directional_light/sum | 0.932 | 1.593 | 2.26 | 1320.5 |
| encode/pass:early prepass/sum | 0.884 | 1.173 | 1.79 | 1253.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.863 | 1.164 | 1.87 | 1222.7 |
| stage/render/prepare_assets | 0.593 | 0.743 | 1.53 | 840.5 |
| bigspace/total/ms | 0.554 | 1.275 | 2.03 | 784.4 |
| stage/render/prepare_write_phase_buffers | 0.538 | 0.781 | 1.25 | 762.1 |
| stage/render/prepare_meshes | 0.516 | 0.743 | 1.28 | 730.9 |
| stage/main/fixed_main_loop | 0.459 | 0.744 | 1.52 | 649.7 |
| stage/render/prepare_resources | 0.431 | 0.500 | 1.01 | 610.5 |
| render/present/ms | 0.417 | 1.286 | 1.84 | 590.5 |
| stage/main/pre_update | 0.415 | 0.566 | 0.96 | 588.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.400 | 0.600 | 1.32 | 566.9 |
| stage/render/prepare_views | 0.372 | 1.562 | 2.96 | 527.1 |
| stage/main/first | 0.358 | 0.443 | 2.01 | 506.7 |
| controller/feed/ms | 0.332 | 0.416 | 0.82 | 470.4 |
| render/acquire/ms | 0.327 | 1.511 | 2.90 | 463.3 |
| stage/render/prepare_batch_phases | 0.325 | 0.451 | 0.76 | 461.1 |
| stage/render/prepare_flush | 0.314 | 0.529 | 1.06 | 445.4 |
| stage/render/prepare_bind_groups | 0.287 | 0.437 | 0.70 | 406.1 |
| stage/main/last | 0.272 | 0.332 | 0.70 | 384.9 |
| bigspace/low_precision/ms | 0.259 | 0.778 | 1.61 | 367.3 |
| stream/drive/ms | 0.194 | 0.283 | 0.61 | 275.1 |
| stage/render/create_views | 0.191 | 0.230 | 0.80 | 270.4 |
| stage/render/extract_commands | 0.190 | 0.221 | 0.65 | 269.2 |
| stage/render/specialize | 0.169 | 0.233 | 0.66 | 239.3 |
| anim/evaluate_cpu/ms | 0.168 | 0.198 | 0.32 | 237.7 |
| stream/tick/ms | 0.161 | 0.243 | 0.58 | 228.0 |
| anim/evaluate/ms | 0.152 | 0.290 | 1.09 | 215.5 |
| bigspace/high_precision/ms | 0.142 | 0.499 | 1.47 | 201.8 |
| stage/render/queue | 0.134 | 0.175 | 0.30 | 189.9 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.113 | 0.314 | 0.45 | 160.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.111 | 0.181 | 0.29 | 157.8 |
| controller/write/ms | 0.066 | 0.082 | 0.46 | 93.3 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.072 | 0.24 | 66.5 |
| controller/move/ms | 0.046 | 0.086 | 0.44 | 64.5 |
| encode/resolve_diagnostics/sum | 0.043 | 0.077 | 0.20 | 60.5 |
| physics/step/ms | 0.036 | 0.064 | 0.48 | 50.5 |
| terrain/update/ms | 0.035 | 0.050 | 0.08 | 49.9 |
| stage/render/phase_sort | 0.031 | 0.048 | 0.18 | 43.3 |
| stage/render/cleanup | 0.025 | 0.035 | 0.46 | 35.9 |
| encode/camera_driver_span_begin/sum | 0.018 | 0.035 | 0.32 | 25.5 |
| weather/map/ms | 0.016 | 0.021 | 0.07 | 22.5 |
| encode/camera_driver_span_end/sum | 0.016 | 0.031 | 0.29 | 22.2 |
| anim/apply/ms | 0.010 | 0.013 | 0.03 | 14.7 |
| controller/avoid/ms | 0.009 | 0.019 | 0.04 | 13.5 |
| weather/eval/ms | 0.009 | 0.012 | 0.03 | 12.9 |
| script/update/ms | 0.009 | 0.011 | 0.39 | 12.1 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.2 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 5.7 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 5.5 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.01 | 3.1 |
| stream/assets_ready/ms | 0.002 | 0.002 | 0.02 | 2.2 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.8 |
| stream/promote/ms | 0.001 | 0.002 | 0.01 | 1.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.2 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.03 | 1.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| terrain/select/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| crowd/joints_extracted/count | count | 687 | 1154 | 629 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.7 k | 43.7 k | 43.7 k |
| encode//max | ms | 0.52 | 1.58 | 0.55 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.32 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.02 |
| encode/finish_critical | ms | 1.02 | 2.26 | 0.87 |
| encode/pass:early prepass/max | ms | 0.87 | 1.77 | 0.87 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.86 | 1.51 | 0.71 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.27 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.38 | 1.28 | 0.38 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.22 | 0.45 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.84 | 2.26 | 0.79 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.20 | 0.04 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 9.38 | 18.19 | 9.37 |
| gpu/timed/ms | ms | 9.37 | 9.99 | 9.37 |
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
| phases/shadow/batch_sets | count | 9 | 106 | 9 |
| phases/shadow/bins | count | 2165 | 2179 | 2165 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 7312 | 21.9 k | 7312 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
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
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 5 | 67 | 5 |
| preprocess/bind_groups_created | count | 0 | 60 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 25.96 | 37.44 | 23.56 |
| process/mem_usage | GiB | 2.96 | 3.02 | 3.02 |
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
| shadow_atlas/mover_draws | count | 0 | 4 | 0 |
| shadow_atlas/movers | count | 85 | 100 | 85 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 4 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 0 | 4 | 0 |
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
| skin/blocks_skipped/count | count | 192 | 203 | 193 |
| skin/blocks_walked/count | count | 18 | 27 | 17 |
| skin/joint_bytes_written/bytes | bytes | 54.8 k | 80.6 k | 53.8 k |
| skin/joints_written/count | count | 856 | 1260 | 840 |
| skin/upload_writes/count | count | 7 | 11 | 8 |
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
| system/cpu_usage | % | 32.27 | 43.29 | 30.06 |
| system/mem_usage | % | 56.39 | 56.54 | 56.54 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2950 | 2950 | 2950 |
| textures/images | count | 2500 | 2500 | 2500 |
| views/all/batch_sets | count | 70 | 167 | 70 |
| views/all/bins | count | 5595 | 5609 | 5595 |
| views/all/count | count | 10 | 10 | 10 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 21.9 k | 36.6 k | 21.9 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 12.1 k | 12.2 k | 12.1 k |
| views/all/pipelines | count | 97 | 98 | 97 |
| views/all/pipelines_distinct | count | 69 | 69 | 69 |
| views/all/work_items | count | 154.8 k | 258.2 k | 154.8 k |
| views/cascade/batch_sets | count | 9 | 45 | 9 |
| views/cascade/bins | count | 2028 | 2028 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 7312 | 21.9 k | 7312 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 5207 | 5207 | 5207 |
| views/cascade/pipelines | count | 37 | 37 | 37 |
| views/cascade/work_items | count | 49.3 k | 152.7 k | 49.3 k |
| views/lamp_face/batch_sets | count | 0 | 91 | 0 |
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
| views/lamp_face/work_items | count | 0 | 91 | 0 |
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
| anim/bones_written/count | count | 30.90 | 65 | 43.8 k |
| anim/events/count | count | 0.04 | 1 | 53 |
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
| encode//count | count | 28.25 | 58 | 40.0 k |
| encode/bind_walk/init_textures | count | 9081 | 20.7 k | 12.87 M |
| encode/bind_walk/init_textures_skipped | count | 52.4 k | 110.9 k | 74.30 M |
| encode/bind_walk/inits | count | 653 | 1365 | 925.4 k |
| encode/bind_walk/inits_skipped | count | 170 | 346 | 240.5 k |
| encode/bind_walk/merge_textures | count | 9019 | 20.5 k | 12.78 M |
| encode/bind_walk/merge_textures_skipped | count | 8728 | 23.5 k | 12.37 M |
| encode/bind_walk/merges | count | 115 | 240 | 163.0 k |
| encode/bind_walk/merges_skipped | count | 156 | 324 | 220.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1417 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1417 |
| encode/pass:early prepass/count | count | 1 | 2 | 1417 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1417 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1417 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1417 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 708 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1417 |
| encode/queue/command_buffers | count | 38.75 | 79 | 54.9 k |
| encode/queue/submits | count | 2 | 4 | 2834 |
| encode/queue/write_buffer/bytes | bytes | 2.72 M | 5.84 M | 3.86 G |
| encode/queue/write_buffer/calls | count | 261 | 568 | 370.2 k |
| encode/queue/write_buffer_with/bytes | bytes | 53.9 k | 107.8 k | 76.36 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 12.8 k |
| encode/queue/write_texture/bytes | bytes | 18.6 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1417 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 901 |
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

## Hitches (> 26.0 ms; slowest 52, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2476.3 | main/busy/ms 2446, stage/main/post_update 2403, stage/main/update 30.68, stage/main/pre_update 8.89, stage/main/last 1.02, surface/advance_cpu_ms 0.73, stage/main/first 0.55, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.30, stream/drive/ms 0.29, bigspace/total/ms 0.08, stream/tick/ms 0.07 |
| warm-up | 3.29 | 817.5 | main/busy/ms 2493, stage/main/post_update 2436, render/thread/ms 757, render/busy/ms 621, stage/render/prepare_assets 569, stage/render/prepare_views 134, render/acquire/ms 134, stage/main/update 37.70, stage/render/create_views 29.22, stage/render/render 12.56, stage/main/pre_update 10.84, render/graph/ms 8.81 |
| warm-up | 3.37 | 78.4 | render/thread/ms 833, render/busy/ms 642, stage/render/prepare_assets 573, stage/render/prepare_views 135, render/acquire/ms 134, pipelines/compile_ms 108, main/busy/ms 85.04, stage/render/render 75.38, pipelines/front_ms 66.26, stage/main/post_update 58.85, render/present/ms 56.06, pipelines/driver_ms 41.57 |
| warm-up | 3.41 | 40.9 | pipelines/compile_ms 264, pipelines/front_ms 171, render/thread/ms 113, pipelines/driver_ms 93.70, stage/render/render 71.32, main/busy/ms 58.64, render/busy/ms 56.95, render/present/ms 55.87, stage/main/post_update 38.00, terrain/update/ms 31.08, stage/render/prepare_assets 24.67, render/graph/ms 13.74 |
| warm-up | 3.46 | 42.3 | pipelines/compile_ms 548, pipelines/front_ms 460, pipelines/wait_ms 297, pipelines/driver_ms 87.46, render/thread/ms 71.00, render/busy/ms 61.45, main/busy/ms 59.48, stage/render/prepare_assets 30.11, stage/main/post_update 29.44, stage/render/render 23.22, terrain/update/ms 14.31, render/graph/ms 12.96 |
| warm-up | 3.58 | 122.0 | pipelines/wait_ms 4471, pipelines/compile_ms 1292, pipelines/front_ms 1200, main/busy/ms 154, render/thread/ms 150, render/busy/ms 142, pipelines/driver_ms 91.96, stage/main/update 85.57, stage/main/post_update 42.20, stage/render/prepare_write_phase_buffers 40.24, stage/render/render 33.14, stage/render/prepare_assets 28.80 |
| warm-up | 3.68 | 99.7 | pipelines/wait_ms 5570, pipelines/compile_ms 2549, pipelines/front_ms 2428, render/thread/ms 198, render/busy/ms 195, main/busy/ms 135, pipelines/driver_ms 121, stage/main/update 83.38, stage/render/prepare_assets 78.99, stage/render/prepare_write_phase_buffers 40.30, stage/main/post_update 35.59, stage/render/render 34.15 |
| warm-up | 3.75 | 50.2 | pipelines/compile_ms 440, pipelines/front_ms 423, pipelines/wait_ms 72.61, render/thread/ms 58.47, render/busy/ms 56.58, main/busy/ms 50.18, stage/render/prepare_assets 35.48, stage/main/post_update 18.36, pipelines/driver_ms 16.73, stage/main/pre_update 14.00, stage/render/render 13.04, render/graph/ms 10.61 |
| warm-up | 3.80 | 43.0 | pipelines/compile_ms 747, pipelines/front_ms 700, pipelines/wait_ms 211, render/thread/ms 73.97, render/busy/ms 71.73, main/busy/ms 52.58, pipelines/driver_ms 47.44, stage/render/prepare_assets 23.47, stage/main/update 20.44, stage/main/post_update 19.32, stage/render/prepare_resources 17.23, encode//sum 16.97 |
| warm-up | 3.85 | 31.1 | pipelines/compile_ms 614, pipelines/front_ms 587, main/busy/ms 45.16, render/thread/ms 42.36, render/busy/ms 39.39, pipelines/driver_ms 26.51, stage/main/post_update 23.51, stage/render/render 15.31, stage/main/update 14.34, render/graph/ms 12.74, stage/render/graph_submit 9.46, terrain/update/ms 9.45 |
| warm-up | 3.89 | 44.8 | pipelines/compile_ms 1190, pipelines/front_ms 1131, render/thread/ms 65.57, render/busy/ms 62.27, pipelines/driver_ms 59.29, main/busy/ms 37.87, stage/render/prepare_assets 32.98, stage/render/render 16.68, stage/main/update 16.39, render/graph/ms 13.61, stage/main/post_update 12.07, pipelines/wait_ms 11.69 |
| warm-up | 3.93 | 34.5 | pipelines/compile_ms 1416, pipelines/front_ms 1350, render/thread/ms 73.16, render/busy/ms 70.69, pipelines/driver_ms 65.91, stage/render/prepare_assets 42.27, main/busy/ms 29.87, pipelines/wait_ms 22.16, stage/render/render 20.01, render/graph/ms 17.02, stage/main/update 13.56, stage/render/graph_submit 13.18 |
| warm-up | 4.02 | 91.9 | pipelines/compile_ms 622, pipelines/front_ms 595, render/thread/ms 121, render/busy/ms 119, stage/render/prepare_assets 97.49, main/busy/ms 27.85, pipelines/driver_ms 26.93, stage/render/render 15.34, pipelines/wait_ms 14.49, render/graph/ms 13.25, stage/render/graph_submit 10.45, stage/main/update 10.16 |
| warm-up | 4.08 | 32.3 | pipelines/compile_ms 284, pipelines/front_ms 276, render/thread/ms 54.84, render/busy/ms 53.40, stage/render/prepare_assets 39.56, main/busy/ms 17.20, stage/render/render 10.30, render/graph/ms 8.70, pipelines/driver_ms 8.32, stage/main/update 7.32, stage/render/graph_submit 6.14, encode//sum 5.83 |
| warm-up | 4.16 | 35.8 | render/thread/ms 41.99, render/busy/ms 40.51, main/busy/ms 37.42, stage/render/render 18.11, stage/main/update 17.30, stage/main/post_update 17.21, render/graph/ms 16.15, stage/render/graph_submit 10.80, encode//sum 10.25, stage/render/prepare_flush 10.20, stage/render/prepare_assets 7.31, encode/submit 6.18 |
| warm-up | 4.25 | 41.2 | pipelines/compile_ms 716, pipelines/front_ms 681, render/thread/ms 60.71, render/busy/ms 59.23, main/busy/ms 46.47, pipelines/driver_ms 34.86, stage/render/prepare_assets 26.76, stage/render/prepare_resources 17.21, pipelines/wait_ms 15.63, stage/main/update 15.44, stage/main/post_update 14.46, stage/main/pre_update 12.49 |
| warm-up | 4.31 | 63.4 | pipelines/compile_ms 728, pipelines/front_ms 694, render/thread/ms 101, render/busy/ms 99.32, main/busy/ms 83.64, pipelines/driver_ms 33.93, stage/main/post_update 33.15, stage/render/prepare_assets 30.71, stage/main/update 23.23, stage/main/pre_update 20.24, stage/render/prepare_resources 18.43, stage/render/prepare_write_phase_buffers 16.84 |
| warm-up | 4.45 | 136.7 | pipelines/compile_ms 1888, pipelines/front_ms 1834, render/thread/ms 196, render/busy/ms 193, main/busy/ms 143, stage/main/update 70.06, stage/render/prepare_resources 61.47, pipelines/driver_ms 53.98, stage/render/prepare_assets 38.96, stage/main/post_update 34.84, stage/render/render 33.44, stage/render/prepare_flush 32.75 |
| warm-up | 4.57 | 118.0 | pipelines/compile_ms 3605, pipelines/front_ms 3512, pipelines/wait_ms 251, render/thread/ms 246, render/busy/ms 244, main/busy/ms 167, pipelines/driver_ms 93.31, stage/render/prepare_resources 91.42, stage/main/update 91.20, stage/render/prepare_assets 74.47, stage/main/post_update 41.22, stage/render/render 32.23 |
| warm-up | 4.68 | 116.4 | pipelines/compile_ms 3631, pipelines/front_ms 3535, pipelines/wait_ms 810, render/thread/ms 226, render/busy/ms 224, stage/render/prepare_assets 148, pipelines/driver_ms 96.07, main/busy/ms 93.41, stage/main/update 46.74, stage/render/prepare_resources 32.85, stage/main/post_update 32.81, stage/render/render 15.74 |
| warm-up | 4.76 | 76.2 | pipelines/compile_ms 2683, pipelines/front_ms 2607, pipelines/wait_ms 841, render/thread/ms 188, render/busy/ms 187, stage/render/prepare_assets 126, pipelines/driver_ms 75.95, main/busy/ms 48.09, stage/main/update 29.58, stage/render/prepare_write_phase_buffers 28.35, stage/render/render 16.24, render/graph/ms 14.18 |
| warm-up | 4.80 | 46.6 | pipelines/compile_ms 891, pipelines/front_ms 866, pipelines/wait_ms 279, render/thread/ms 119, render/busy/ms 116, main/busy/ms 55.39, stage/main/update 34.98, stage/render/prepare_assets 33.98, stage/render/prepare_write_phase_buffers 33.55, stage/render/render 28.61, render/graph/ms 25.52, pipelines/driver_ms 25.34 |
| warm-up | 4.84 | 36.8 | render/thread/ms 80.44, pipelines/compile_ms 79.59, render/busy/ms 77.32, pipelines/front_ms 74.32, main/busy/ms 53.41, stage/render/render 34.97, render/graph/ms 31.63, stage/main/update 30.91, stage/render/graph_submit 26.70, encode/submit 19.05, stage/render/prepare_assets 15.14, stage/main/post_update 12.87 |
| warm-up | 4.87 | 32.9 | pipelines/compile_ms 567, pipelines/front_ms 550, render/thread/ms 66.43, render/busy/ms 64.59, main/busy/ms 51.84, stage/main/update 27.37, stage/render/render 23.39, render/graph/ms 21.09, pipelines/driver_ms 17.02, stage/render/graph_submit 15.91, stage/render/prepare_assets 15.80, stage/main/post_update 14.61 |
| warm-up | 4.91 | 31.7 | pipelines/compile_ms 598, pipelines/front_ms 579, render/thread/ms 59.68, render/busy/ms 57.96, main/busy/ms 51.71, stage/main/update 30.23, pipelines/driver_ms 18.43, stage/render/prepare_assets 15.73, stage/render/render 15.19, stage/main/post_update 13.04, render/graph/ms 12.96, stage/render/graph_submit 9.13 |
| warm-up | 4.98 | 28.6 | pipelines/compile_ms 535, pipelines/front_ms 518, main/busy/ms 37.64, render/thread/ms 35.08, render/busy/ms 33.03, stage/main/update 20.21, pipelines/driver_ms 17.03, stage/render/render 14.33, render/graph/ms 12.02, stage/main/post_update 10.30, stage/render/prepare_assets 10.23, stage/render/graph_submit 8.63 |
| warm-up | 5.14 | 36.8 | pipelines/compile_ms 427, pipelines/front_ms 416, main/busy/ms 61.08, render/thread/ms 34.62, render/busy/ms 31.34, stage/main/post_update 27.47, stage/main/update 26.06, stage/render/render 21.09, render/graph/ms 17.36, stage/render/graph_submit 12.83, pipelines/driver_ms 11.39, pipelines/wait_ms 10.14 |
| warm-up | 5.18 | 43.8 | pipelines/compile_ms 431, pipelines/front_ms 420, render/thread/ms 51.48, render/busy/ms 50.11, main/busy/ms 37.58, stage/main/update 19.11, stage/render/render 14.84, render/graph/ms 12.81, pipelines/driver_ms 11.83, stage/render/prepare_assets 11.07, stage/main/post_update 10.59, pipelines/wait_ms 10.30 |
| warm-up | 5.21 | 27.4 | pipelines/compile_ms 749, pipelines/front_ms 728, render/thread/ms 58.20, render/busy/ms 56.85, main/busy/ms 26.91, pipelines/driver_ms 21.62, stage/render/prepare_assets 16.11, stage/render/prepare_resources 13.15, stage/render/render 12.87, stage/main/update 11.03, render/graph/ms 10.85, encode//sum 9.11 |
| warm-up | 5.26 | 52.6 | pipelines/compile_ms 1557, pipelines/front_ms 1511, render/thread/ms 74.17, render/busy/ms 72.73, pipelines/driver_ms 46.12, main/busy/ms 45.39, stage/render/prepare_resources 23.46, stage/main/update 16.38, stage/render/prepare_assets 14.76, stage/render/prepare_write_phase_buffers 13.47, stage/main/post_update 12.93, stage/render/render 12.58 |
| warm-up | 5.29 | 26.2 | pipelines/compile_ms 1200, pipelines/front_ms 1165, render/thread/ms 71.84, render/busy/ms 70.44, main/busy/ms 55.41, pipelines/driver_ms 35.51, stage/main/update 24.28, stage/render/prepare_resources 19.98, stage/render/render 16.93, render/graph/ms 15.06, stage/main/post_update 14.03, stage/render/prepare_write_phase_buffers 12.60 |
| warm-up | 5.32 | 37.0 | pipelines/compile_ms 994, pipelines/front_ms 968, render/thread/ms 53.56, render/busy/ms 52.25, main/busy/ms 46.08, pipelines/driver_ms 25.45, stage/main/update 25.36, stage/render/render 17.69, pipelines/wait_ms 15.71, render/graph/ms 15.61, stage/main/post_update 11.13, stage/render/prepare_assets 10.40 |
| warm-up | 5.61 | 27.4 | render/thread/ms 38.12, main/busy/ms 37.40, render/busy/ms 36.25, stage/render/render 16.84, stage/main/post_update 15.95, stage/main/update 15.06, render/graph/ms 14.17, encode//sum 10.16, stage/render/graph_submit 9.28, stage/render/prepare_assets 8.55, encode/submit 5.78, stage/render/camera_driver 4.18 |
| warm-up | 5.64 | 34.9 | pipelines/compile_ms 173, pipelines/front_ms 166, render/thread/ms 54.82, render/busy/ms 52.76, main/busy/ms 36.15, stage/render/prepare_assets 20.28, stage/render/render 17.77, stage/main/post_update 15.51, render/graph/ms 14.52, encode//sum 13.05, stage/main/update 12.91, stage/render/graph_submit 8.94 |
| warm-up | 5.68 | 34.5 | pipelines/compile_ms 173, pipelines/front_ms 166, render/thread/ms 63.69, render/busy/ms 61.70, main/busy/ms 45.05, stage/render/prepare_assets 25.26, stage/render/render 20.81, stage/main/post_update 19.31, render/graph/ms 17.32, stage/main/update 14.53, encode//sum 12.19, stage/render/graph_submit 11.76 |
| warm-up | 5.73 | 50.6 | pipelines/compile_ms 185, pipelines/front_ms 179, render/thread/ms 79.01, render/busy/ms 77.05, main/busy/ms 49.24, stage/render/prepare_assets 38.92, stage/render/render 22.90, render/graph/ms 19.56, stage/main/post_update 18.92, stage/main/update 18.16, stage/render/graph_submit 14.31, encode//sum 12.99 |
| warm-up | 5.78 | 27.9 | pipelines/compile_ms 194, pipelines/front_ms 191, render/thread/ms 48.32, render/busy/ms 47.15, main/busy/ms 32.08, stage/render/prepare_assets 31.52, stage/main/update 14.69, stage/main/post_update 9.78, stage/render/render 9.27, encode//sum 7.88, render/graph/ms 7.70, stage/render/graph_submit 5.29 |
| warm-up | 5.85 | 28.2 | main/busy/ms 43.08, render/thread/ms 37.73, render/busy/ms 36.38, stage/main/update 22.66, pipelines/compile_ms 21.66, pipelines/front_ms 20.71, stage/main/post_update 14.63, stage/render/render 12.57, render/graph/ms 10.84, encode//sum 8.19, stage/render/graph_submit 6.78, stage/render/prepare_meshes 6.13 |
| warm-up | 5.89 | 49.3 | pipelines/compile_ms 166, pipelines/front_ms 156, render/thread/ms 68.51, render/busy/ms 66.58, main/busy/ms 59.90, stage/main/update 36.11, stage/render/prepare_assets 21.92, stage/main/post_update 16.17, stage/render/render 16.01, render/graph/ms 13.41, stage/render/prepare_meshes 10.32, pipelines/driver_ms 9.74 |
| warm-up | 5.94 | 40.7 | pipelines/compile_ms 201, pipelines/front_ms 189, render/thread/ms 84.70, render/busy/ms 82.84, main/busy/ms 61.34, stage/render/prepare_assets 38.38, stage/main/update 34.51, stage/render/render 17.34, stage/main/post_update 15.55, render/graph/ms 14.58, pipelines/driver_ms 12.23, stage/render/graph_submit 9.11 |
| warm-up | 5.97 | 37.0 | render/thread/ms 73.09, render/busy/ms 71.63, pipelines/compile_ms 63.92, pipelines/front_ms 60.22, main/busy/ms 55.98, stage/render/prepare_assets 35.63, stage/main/update 31.05, stage/render/render 15.61, stage/main/post_update 13.62, render/graph/ms 13.55, stage/render/graph_submit 10.06, encode//sum 8.82 |
| warm-up | 6.00 | 30.5 | pipelines/compile_ms 228, pipelines/front_ms 220, render/thread/ms 63.81, render/busy/ms 62.26, main/busy/ms 60.28, stage/main/update 38.04, stage/render/prepare_assets 22.00, stage/render/render 16.20, render/graph/ms 14.21, stage/main/post_update 13.58, stage/render/graph_submit 10.53, character/finish/ms 8.30 |
| warm-up | 6.03 | 28.6 | pipelines/compile_ms 243, pipelines/front_ms 234, render/thread/ms 55.30, render/busy/ms 53.86, main/busy/ms 44.90, stage/main/update 23.45, stage/render/prepare_assets 18.04, stage/render/render 14.27, stage/main/post_update 14.10, render/graph/ms 12.22, pipelines/driver_ms 9.40, stage/render/graph_submit 8.29 |
| warm-up | 6.15 | 46.6 | pipelines/compile_ms 544, pipelines/front_ms 523, render/thread/ms 60.35, main/busy/ms 59.77, render/busy/ms 58.58, stage/main/update 27.69, stage/render/render 22.77, stage/main/post_update 22.18, pipelines/driver_ms 20.93, render/graph/ms 20.31, stage/render/prepare_assets 17.17, stage/render/graph_submit 15.21 |
| warm-up | 6.19 | 39.7 | pipelines/compile_ms 137, pipelines/front_ms 126, render/thread/ms 75.01, render/busy/ms 73.15, main/busy/ms 66.72, stage/render/prepare_assets 27.10, stage/main/update 25.95, stage/main/post_update 23.70, stage/render/render 20.84, render/graph/ms 17.95, stage/render/graph_submit 12.62, pipelines/driver_ms 10.97 |
| warm-up | 6.23 | 42.0 | pipelines/compile_ms 670, pipelines/front_ms 647, render/thread/ms 76.13, render/busy/ms 74.50, main/busy/ms 65.46, stage/render/prepare_assets 37.28, stage/main/update 33.24, pipelines/driver_ms 23.74, stage/main/post_update 16.09, stage/render/render 15.61, render/graph/ms 13.06, stage/render/graph_submit 9.37 |
| warm-up | 6.26 | 27.4 | pipelines/compile_ms 846, pipelines/front_ms 817, render/thread/ms 65.52, render/busy/ms 64.04, main/busy/ms 54.53, stage/render/prepare_assets 35.46, stage/main/update 29.63, pipelines/driver_ms 29.37, stage/main/post_update 16.25, stage/render/render 14.27, render/graph/ms 12.17, stage/render/graph_submit 8.56 |
| warm-up | 6.29 | 34.2 | pipelines/compile_ms 424, pipelines/front_ms 410, render/thread/ms 57.57, main/busy/ms 56.16, render/busy/ms 55.94, stage/main/update 24.65, stage/main/post_update 23.10, stage/render/render 20.47, render/graph/ms 18.18, stage/render/prepare_assets 17.19, pipelines/driver_ms 14.87, stage/render/graph_submit 13.47 |
| warm-up | 6.32 | 30.2 | pipelines/compile_ms 671, pipelines/front_ms 654, main/busy/ms 59.31, render/thread/ms 58.80, render/busy/ms 57.23, stage/main/update 29.57, stage/render/render 21.08, stage/main/post_update 20.96, render/graph/ms 18.73, pipelines/driver_ms 17.06, stage/render/graph_submit 13.86, encode//sum 13.35 |
| warm-up | 6.35 | 29.3 | pipelines/compile_ms 1199, pipelines/front_ms 1170, render/thread/ms 54.04, render/busy/ms 52.62, main/busy/ms 47.63, pipelines/driver_ms 29.28, stage/main/update 24.29, stage/render/prepare_assets 16.02, stage/render/render 14.19, stage/main/post_update 14.00, render/graph/ms 12.16, encode//sum 9.61 |
| warm-up | 6.41 | 30.4 | render/thread/ms 47.24, render/busy/ms 45.77, main/busy/ms 39.29, stage/main/update 22.99, stage/render/render 17.66, render/graph/ms 15.42, encode//sum 14.71, stage/render/prepare_assets 10.79, stage/render/graph_submit 10.48, stage/main/post_update 10.22, stage/render/prepare_meshes 7.66, encode/submit 6.70 |
| warm-up | 6.80 | 29.9 | main/busy/ms 35.35, render/thread/ms 34.86, render/busy/ms 33.83, stage/main/update 18.68, stage/render/render 9.93, stage/main/post_update 9.52, stage/render/prepare_assets 8.94, encode//sum 8.09, render/graph/ms 8.05, stage/render/prepare_meshes 7.64, character/finish/ms 6.06, stage/render/graph_submit 4.99 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0\gc_ascadian@2560x1440_arm=off-3 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **96%** (min 90%) · 2625 MHz (min 2625) · memory 8101 MHz · 87.0 W · 74 °C (max 77) · VRAM 5885 MiB · throttled 100% of samples

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
