# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.max_lamps=16 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6379 frames over 65.6 s · **median 10.15 ms (99 fps)** · mean 10.29 · p95 12.04 · p99 13.94 (1% low 72 fps) · worst 34.26 ms · 32 spikes over 1.5× median · 23.8 stutters/min (work 24.7/min)
- warm-up: 353 frames over 9.4 s, worst 2482.8 ms
- hitches (> 25.4 ms): **2** measured, 43 in warm-up
- timed GPU passes: 6.78 ms of a 10.29 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.15 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.76/min, limit 1/min
- **FAIL** worst frame 34.26 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.933 | 2.035 | 2.068 | 0.95 | 0.178 |
| main_opaque_pass_3d | 1.573 | 1.656 | 2.372 | 0.95 | 0.081 |
| early prepass | 0.490 | 0.516 | 0.753 | 0.95 | 0.092 |
| bloom | 0.301 | 0.317 | 0.324 | 0.95 | 0.063 |
| atmosphere_luts | 0.297 | 0.313 | 0.376 | 0.95 | 0.003 |
| water_velocity | 0.214 | 0.225 | 0.246 | 0.95 | 0.002 |
| water_sspr | 0.192 | 0.202 | 0.203 | 0.95 | 0.003 |
| ssao | 0.178 | 0.188 | 0.191 | 0.95 | 0.008 |
| point_shadow_atlas | 0.161 | 0.289 | 0.624 | 0.56 | 0.261 |
| upscaling | 0.123 | 0.129 | 0.135 | 0.95 | 0.004 |
| atmos_froxel_inject | 0.122 | 0.128 | 0.129 | 0.95 | 0.002 |
| water_fft | 0.117 | 0.123 | 0.126 | 0.95 | 0.003 |
| uniform_allocation | 0.111 | 0.117 | 0.142 | 0.95 | 0.025 |
| clouds_composite | 0.100 | 0.105 | 0.109 | 0.95 | 0.002 |
| gpu_instance_materialization | 0.092 | 0.096 | 0.100 | 0.95 | 0.004 |
| bin_unpacking | 0.088 | 0.093 | 0.114 | 0.95 | 0.018 |
| tonemapping | 0.083 | 0.087 | 0.092 | 0.95 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.070 | 0.132 | 0.95 | 0.009 |
| clustering | 0.061 | 0.064 | 0.068 | 0.95 | 0.020 |
| clouds_trace | 0.053 | 0.056 | 0.078 | 0.95 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.054 | 0.070 | 0.95 | 0.004 |
| render_sky | 0.050 | 0.052 | 0.053 | 0.95 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.415 | 0.417 | 0.11 | 0.001 |
| surface_trail | 0.037 | 0.057 | 0.057 | 0.64 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.029 | 0.031 | 0.95 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.028 | 0.050 | 0.078 | 0.56 | 0.012 |
| lightprobe_radiance_map | 0.025 | 0.219 | 0.230 | 0.11 | 0.004 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.95 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.95 | 0.002 |
| range_unpacking | 0.021 | 0.022 | 0.027 | 0.95 | 0.004 |
| point_shadow_atlas/uniform_allocation | 0.018 | 0.033 | 0.080 | 0.56 | 0.010 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.95 | 0.006 |
| night_sky | 0.016 | 0.017 | 0.042 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.031 | 0.95 | 0.186 |
| point_shadow_atlas/bin_unpacking | 0.013 | 0.023 | 0.047 | 0.56 | 0.007 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.56 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.95 | 0.004 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.95 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.152 | 10.790 | 20.11 | 58382.2 |
| render/busy/ms | 8.631 | 10.320 | 19.34 | 55057.1 |
| main/busy/ms | 8.294 | 9.827 | 30.85 | 52910.5 |
| stage/render/render | 5.343 | 6.349 | 13.27 | 34080.9 |
| stage/main/post_update | 4.721 | 5.730 | 9.33 | 30114.7 |
| render/graph/ms | 4.665 | 5.624 | 11.91 | 29755.3 |
| encode//sum | 3.471 | 4.203 | 9.10 | 22138.8 |
| stage/render/graph_submit | 3.298 | 4.046 | 9.14 | 21036.9 |
| encode/submit | 2.046 | 2.709 | 5.30 | 13053.9 |
| stage/main/update | 1.853 | 2.390 | 24.79 | 11818.8 |
| encode/finish_wall | 1.333 | 1.788 | 3.96 | 8502.1 |
| stage/render/camera_driver | 1.303 | 1.661 | 2.65 | 8313.7 |
| encode/pass:early prepass/sum | 0.864 | 1.113 | 1.91 | 5510.1 |
| encode/pass:shadow_directional_light/sum | 0.858 | 1.262 | 3.22 | 5470.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.854 | 1.119 | 2.10 | 5447.3 |
| stage/render/prepare_write_phase_buffers | 0.647 | 0.988 | 2.06 | 4127.9 |
| bigspace/total/ms | 0.551 | 1.354 | 3.06 | 3512.8 |
| stage/render/prepare_assets | 0.533 | 0.666 | 12.04 | 3401.0 |
| stage/main/fixed_main_loop | 0.528 | 0.941 | 2.02 | 3369.1 |
| stage/render/prepare_meshes | 0.448 | 0.561 | 4.26 | 2857.5 |
| stage/render/prepare_resources | 0.440 | 0.533 | 1.04 | 2807.4 |
| render/present/ms | 0.430 | 1.229 | 2.06 | 2744.0 |
| stage/main/pre_update | 0.391 | 0.532 | 1.72 | 2494.0 |
| stage/main/first | 0.363 | 0.484 | 5.18 | 2318.6 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.353 | 1.435 | 2.72 | 2251.6 |
| controller/feed/ms | 0.332 | 0.416 | 0.87 | 2120.6 |
| stage/render/prepare_flush | 0.322 | 0.534 | 1.74 | 2055.7 |
| stage/render/prepare_bind_groups | 0.320 | 0.534 | 1.65 | 2038.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.283 | 0.442 | 1.15 | 1807.3 |
| stage/render/prepare_batch_phases | 0.283 | 0.423 | 0.88 | 1803.7 |
| stage/main/last | 0.258 | 0.309 | 0.82 | 1644.1 |
| bigspace/low_precision/ms | 0.242 | 0.746 | 1.75 | 1541.7 |
| stream/drive/ms | 0.213 | 0.303 | 10.55 | 1356.8 |
| stage/render/create_views | 0.202 | 0.281 | 0.87 | 1286.8 |
| stage/render/extract_commands | 0.198 | 0.229 | 0.42 | 1263.2 |
| stage/render/specialize | 0.167 | 0.267 | 1.81 | 1065.3 |
| anim/evaluate/ms | 0.161 | 0.277 | 1.75 | 1028.1 |
| bigspace/high_precision/ms | 0.159 | 0.667 | 2.74 | 1013.0 |
| stream/tick/ms | 0.158 | 0.230 | 0.56 | 1006.3 |
| anim/evaluate_cpu/ms | 0.157 | 0.199 | 0.54 | 1002.2 |
| stage/render/prepare_views | 0.134 | 0.176 | 0.52 | 854.3 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.113 | 0.974 | 1.82 | 719.0 |
| stage/render/queue | 0.106 | 0.178 | 1.47 | 674.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.155 | 0.83 | 637.3 |
| pipelines/compile_ms | 0.093 | 0.000 | 126.07 | 594.3 |
| render/acquire/ms | 0.091 | 0.121 | 0.39 | 581.1 |
| pipelines/front_ms | 0.091 | 0.000 | 123.45 | 577.7 |
| stage/render/phase_sort | 0.080 | 0.111 | 0.26 | 513.3 |
| controller/move/ms | 0.077 | 0.208 | 0.39 | 492.8 |
| controller/write/ms | 0.073 | 0.098 | 0.53 | 465.8 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.073 | 0.584 | 3.19 | 464.9 |
| terrain/update/ms | 0.060 | 0.292 | 1.66 | 385.8 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.33 | 295.7 |
| physics/step/ms | 0.040 | 0.081 | 0.69 | 255.3 |
| encode/resolve_diagnostics/sum | 0.034 | 0.046 | 0.49 | 214.4 |
| physics/insert/ms | 0.026 | 0.312 | 0.84 | 166.8 |
| stage/render/cleanup | 0.024 | 0.033 | 0.37 | 152.4 |
| terrain/select/ms | 0.023 | 0.258 | 0.84 | 145.9 |
| weather/map/ms | 0.018 | 0.023 | 0.12 | 111.8 |
| far/select/ms | 0.015 | 0.167 | 0.54 | 97.2 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.54 | 88.0 |
| anim/apply/ms | 0.013 | 0.024 | 0.25 | 80.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.021 | 0.58 | 73.1 |
| weather/eval/ms | 0.011 | 0.014 | 0.05 | 68.4 |
| script/update/ms | 0.008 | 0.012 | 0.21 | 52.4 |
| controller/avoid/ms | 0.006 | 0.012 | 0.04 | 38.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.08 | 35.4 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 34.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.14 | 22.3 |
| script/ui/ms | 0.003 | 0.004 | 0.23 | 21.5 |
| stream/upload/ms | 0.003 | 0.000 | 1.15 | 20.4 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.42 | 16.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 13.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 6.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 20 |
| controller/movers/count | count | 20 | 21 | 20 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1306 | 2262 | 535 |
| crowd/c0/meshes/count | count | 355 | 646 | 144 |
| crowd/c1/count | count | 145 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5982 | 5661 |
| crowd/c1/meshes/count | count | 1166 | 1284 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 204 |
| crowd/joints_extracted/count | count | 679 | 6888 | 613 |
| crowd/joints_uploaded/count | count | 3615 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.42 | 1.29 | 0.60 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.54 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.58 | 0.01 |
| encode/finish_critical | ms | 1.04 | 3.22 | 0.93 |
| encode/pass:early prepass/max | ms | 0.86 | 1.91 | 0.93 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.86 | 2.10 | 0.77 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.83 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 1.15 | 0.30 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 1.13 | 2.72 | 1.00 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.34 | 1.82 | 1.00 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.30 | 3.19 | 1.29 |
| encode/pass:shadow_directional_light/max | ms | 0.83 | 3.22 | 0.56 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.49 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.74 | 30.85 | 8.68 |
| gpu/timed/ms | ms | 7.89 | 9.29 | 8.68 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 156 | 386 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 329 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 156 | 386 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 329 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2491 | 3536 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2491 | 3536 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 70 | 474 | 8 |
| phases/shadow/bins | count | 800 | 1411 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2314 | 3255 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 52 | 142 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3182 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 334 | 335 | 335 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 15 | 291 | 11 |
| preprocess/bind_groups_created | count | 10 | 281 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 138 | 249 | 123 |
| process/cpu_usage | % | 24.52 | 37.10 | 25.48 |
| process/mem_usage | GiB | 5.21 | 5.37 | 5.37 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 4 | 0 |
| shadow_atlas/lamps | count | 8 | 17 | 0 |
| shadow_atlas/live_occ_pct | % | 3.91 | 32.91 | 0 |
| shadow_atlas/live_tiles | count | 15 | 50 | 0 |
| shadow_atlas/mover_draws | count | 9 | 34 | 0 |
| shadow_atlas/movers | count | 145 | 486 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 9 | 34 | 0 |
| shadow_atlas/render/clears | count | 0 | 24 | 0 |
| shadow_atlas/render/lamps | count | 8 | 17 | 0 |
| shadow_atlas/render/live_direct | count | 9 | 34 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 18 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 15 | 52 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 14.06 | 38.28 | 0 |
| shadow_atlas/static_tiles | count | 42 | 99 | 0 |
| shadow_atlas/views | count | 15 | 52 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 122 | 13 |
| skin/blocks_skipped/count | count | 146 | 205 | 191 |
| skin/blocks_walked/count | count | 14 | 27 | 13 |
| skin/joint_bytes_written/bytes | bytes | 47.6 k | 810.1 k | 47.4 k |
| skin/joints_written/count | count | 744 | 12.7 k | 740 |
| skin/upload_writes/count | count | 7 | 28 | 9 |
| started_load_count |  | 6443 | 6568 | 6568 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.37 | 46.75 | 31.72 |
| system/mem_usage | % | 54.03 | 54.51 | 54.48 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 131 | 524 | 66 |
| views/all/bins | count | 2928 | 3551 | 2948 |
| views/all/count | count | 18 | 55 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8052 | 10.3 k | 7956 |
| views/all/pipelines | count | 99 | 187 | 66 |
| views/all/pipelines_distinct | count | 67 | 74 | 61 |
| views/all/work_items | count | 154.2 k | 231.0 k | 154.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 8 |
| views/cascade/bins | count | 722 | 880 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2126 | 2442 | 2014 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 51 | 461 | 0 |
| views/lamp_face/bins | count | 94 | 619 | 0 |
| views/lamp_face/count | count | 15 | 52 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 140 | 816 | 0 |
| views/lamp_face/pipelines | count | 24 | 116 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 94 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2022 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5914 | 7492 | 5942 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 104.6 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.1 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 58.88 | 283 | 375.6 k |
| anim/events/count | count | 0.09 | 3 | 567 |
| assets/Image/added | count | 0.09 | 38 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.45 | 266 | 2874 |
| assets/Mesh/removed | count | 0.76 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.17 | 10 | 7475 |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1168 | 1489 |
| encode//count | count | 27.93 | 30 | 178.2 k |
| encode/bind_walk/init_textures | count | 9446 | 25.8 k | 60.26 M |
| encode/bind_walk/init_textures_skipped | count | 60.0 k | 91.5 k | 382.68 M |
| encode/bind_walk/inits | count | 575 | 772 | 3.67 M |
| encode/bind_walk/inits_skipped | count | 179 | 275 | 1.14 M |
| encode/bind_walk/merge_textures | count | 9383 | 25.7 k | 59.86 M |
| encode/bind_walk/merge_textures_skipped | count | 10.9 k | 34.2 k | 69.24 M |
| encode/bind_walk/merges | count | 108 | 203 | 686.7 k |
| encode/bind_walk/merges_skipped | count | 157 | 246 | 1.00 M |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6379 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6379 |
| encode/pass:early prepass/count | count | 1 | 1 | 6379 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6379 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6379 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6379 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.31 | 1 | 1989 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.20 | 1 | 1262 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 348 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6379 |
| encode/queue/command_buffers | count | 38.51 | 43 | 245.6 k |
| encode/queue/submits | count | 2.01 | 4 | 12.8 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.25 M | 14.85 G |
| encode/queue/write_buffer/calls | count | 280 | 486 | 1.79 M |
| encode/queue/write_buffer_with/bytes | bytes | 156.4 k | 1.28 M | 997.59 M |
| encode/queue/write_buffer_with/calls | count | 10.34 | 807 | 65.9 k |
| encode/queue/write_texture/bytes | bytes | 1996 | 639.0 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6379 |
| physics/inserts | count | 0.35 | 109 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 175 | 2335 |
| physics/steps | count | 0.62 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 7 |
| pipelines/created | count | 0.00 | 1 | 8 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 30 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5307 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1163 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.4 ms; slowest 45, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2482.8 | main/busy/ms 2454, stage/main/post_update 2411, stage/main/update 32.68, stage/main/pre_update 6.77, stage/main/last 0.98, surface/advance_cpu_ms 0.82, stream/drive/ms 0.77, stage/main/first 0.48, surface/cell_upload_ms 0.41, stage/main/fixed_main_loop 0.35, stream/tick/ms 0.24, bigspace/total/ms 0.10 |
| warm-up | 3.24 | 754.5 | main/busy/ms 2593, stage/main/post_update 2437, render/thread/ms 721, render/busy/ms 581, stage/render/prepare_assets 519, stage/main/update 139, stage/render/prepare_views 137, render/acquire/ms 137, stage/render/create_views 28.96, stage/render/render 24.43, render/graph/ms 20.00, stage/render/camera_driver 18.25 |
| warm-up | 3.41 | 171.3 | render/thread/ms 876, render/busy/ms 642, stage/render/prepare_assets 566, main/busy/ms 169, stage/render/prepare_views 138, render/acquire/ms 137, stage/render/render 122, stage/main/update 114, pipelines/compile_ms 98.91, render/present/ms 96.82, pipelines/driver_ms 75.93, stage/main/post_update 41.45 |
| warm-up | 3.44 | 28.4 | pipelines/compile_ms 505, pipelines/front_ms 283, pipelines/driver_ms 222, render/thread/ms 182, main/busy/ms 136, stage/render/render 119, render/present/ms 95.14, pipelines/wait_ms 90.26, render/busy/ms 85.95, stage/main/update 65.14, stage/main/post_update 59.07, stage/render/prepare_assets 48.32 |
| warm-up | 3.75 | 308.4 | pipelines/wait_ms 4890, pipelines/compile_ms 3838, pipelines/front_ms 3025, pipelines/driver_ms 813, render/thread/ms 235, render/busy/ms 232, main/busy/ms 159, stage/render/prepare_assets 121, stage/render/render 100, render/graph/ms 97.28, stage/render/camera_driver 77.01, stage/main/post_update 76.11 |
| warm-up | 3.81 | 65.5 | pipelines/wait_ms 4914, pipelines/compile_ms 3761, pipelines/front_ms 3069, pipelines/driver_ms 691, render/thread/ms 270, render/busy/ms 269, stage/render/prepare_assets 171, main/busy/ms 88.63, stage/render/render 87.50, render/graph/ms 85.45, stage/render/camera_driver 76.25, stage/main/post_update 50.35 |
| warm-up | 3.85 | 39.8 | pipelines/compile_ms 1293, pipelines/front_ms 1205, pipelines/wait_ms 205, render/thread/ms 100, render/busy/ms 98.42, pipelines/driver_ms 87.69, main/busy/ms 71.74, stage/render/prepare_assets 71.70, stage/main/post_update 26.11, stage/main/pre_update 22.69, stage/render/render 18.96, render/graph/ms 16.70 |
| warm-up | 3.89 | 32.2 | pipelines/compile_ms 71.62, pipelines/front_ms 66.87, render/thread/ms 36.15, main/busy/ms 27.10, render/busy/ms 23.91, encode//sum 23.03, stage/render/render 12.60, stage/main/post_update 11.94, stage/main/update 11.26, stage/render/prepare_views 11.15, render/acquire/ms 11.05, render/graph/ms 10.93 |
| warm-up | 3.95 | 57.9 | pipelines/compile_ms 220, pipelines/front_ms 201, render/thread/ms 82.89, main/busy/ms 77.51, render/busy/ms 56.53, stage/main/update 54.45, stage/render/render 39.93, render/graph/ms 36.53, encode//sum 26.53, stage/render/prepare_views 23.85, render/acquire/ms 23.67, stage/render/camera_driver 22.13 |
| warm-up | 4.04 | 84.0 | pipelines/compile_ms 1389, pipelines/front_ms 1320, render/thread/ms 110, main/busy/ms 96.20, render/busy/ms 94.69, pipelines/wait_ms 77.99, pipelines/driver_ms 69.43, stage/main/update 66.81, stage/render/prepare_assets 47.33, stage/render/render 39.82, render/graph/ms 36.51, stage/render/camera_driver 21.74 |
| warm-up | 4.09 | 54.8 | pipelines/compile_ms 1556, pipelines/front_ms 1493, render/thread/ms 88.08, render/busy/ms 86.72, pipelines/wait_ms 83.31, stage/render/prepare_assets 68.79, pipelines/driver_ms 63.72, main/busy/ms 43.17, stage/main/update 22.63, stage/render/render 12.64, stage/main/post_update 11.36, render/graph/ms 11.02 |
| warm-up | 4.18 | 30.6 | pipelines/compile_ms 48.31, pipelines/front_ms 45.70, render/thread/ms 37.64, main/busy/ms 36.30, render/busy/ms 36.02, stage/main/update 21.02, stage/render/render 14.16, stage/render/prepare_assets 13.33, render/graph/ms 12.10, stream/drive/ms 10.25, stage/main/post_update 8.93, stage/render/graph_submit 8.70 |
| warm-up | 4.23 | 51.4 | pipelines/compile_ms 434, pipelines/front_ms 417, render/thread/ms 65.01, render/busy/ms 62.07, main/busy/ms 54.95, stage/main/update 30.93, stage/render/prepare_assets 29.36, stage/render/render 20.07, pipelines/driver_ms 16.77, render/graph/ms 16.74, stage/main/post_update 15.55, stream/drive/ms 10.71 |
| warm-up | 4.28 | 46.2 | pipelines/compile_ms 671, pipelines/front_ms 631, render/thread/ms 77.97, render/busy/ms 74.59, main/busy/ms 67.31, stage/render/prepare_assets 40.75, pipelines/driver_ms 39.90, stage/main/update 36.97, stage/render/render 21.88, render/graph/ms 18.20, stage/main/post_update 17.38, encode//sum 12.83 |
| warm-up | 4.35 | 64.5 | pipelines/compile_ms 1352, pipelines/front_ms 1299, main/busy/ms 177, stage/main/update 154, render/thread/ms 75.91, render/busy/ms 73.70, pipelines/driver_ms 52.60, stage/render/prepare_assets 51.22, stage/render/render 16.48, render/graph/ms 13.82, stream/drive/ms 12.10, encode//sum 10.06 |
| warm-up | 4.60 | 258.7 | pipelines/compile_ms 3263, pipelines/front_ms 3171, main/busy/ms 174, render/thread/ms 164, stage/main/update 155, render/busy/ms 155, pipelines/driver_ms 92.01, stage/render/prepare_assets 76.31, stage/render/render 50.03, render/graph/ms 40.89, pipelines/wait_ms 39.65, pipelines/lock_ms 37.90 |
| warm-up | 4.70 | 100.6 | pipelines/compile_ms 2249, pipelines/front_ms 2182, render/thread/ms 220, render/busy/ms 212, pipelines/wait_ms 201, main/busy/ms 102, pipelines/driver_ms 67.21, stage/render/prepare_assets 51.12, stage/render/render 45.94, stage/render/prepare_resources 45.48, stage/main/fixed_main_loop 38.14, render/graph/ms 37.09 |
| warm-up | 4.79 | 88.3 | pipelines/compile_ms 2065, pipelines/front_ms 1984, pipelines/wait_ms 1129, render/thread/ms 160, render/busy/ms 158, main/busy/ms 127, pipelines/driver_ms 81.83, stage/main/update 50.08, stage/render/prepare_assets 46.14, stage/main/fixed_main_loop 38.23, stage/render/prepare_write_phase_buffers 35.95, stage/render/prepare_bind_groups 31.28 |
| warm-up | 4.89 | 92.7 | pipelines/compile_ms 2725, pipelines/front_ms 2618, pipelines/wait_ms 1155, render/thread/ms 117, render/busy/ms 116, pipelines/driver_ms 107, stage/render/prepare_assets 83.08, main/busy/ms 64.39, stage/main/update 46.45, stage/render/render 12.78, render/graph/ms 10.74, stage/main/post_update 7.96 |
| warm-up | 4.92 | 31.2 | pipelines/compile_ms 1244, pipelines/front_ms 1198, pipelines/wait_ms 272, render/thread/ms 78.20, render/busy/ms 76.61, stage/render/prepare_assets 49.79, pipelines/driver_ms 45.73, main/busy/ms 41.53, stage/main/update 17.84, stage/render/render 11.32, stage/main/post_update 10.98, render/graph/ms 9.30 |
| warm-up | 4.96 | 42.9 | pipelines/compile_ms 941, pipelines/front_ms 911, pipelines/wait_ms 299, main/busy/ms 57.15, render/thread/ms 54.85, render/busy/ms 53.36, pipelines/driver_ms 30.13, stage/main/update 26.21, stage/main/post_update 18.85, stage/render/prepare_assets 15.59, stage/render/render 11.99, render/graph/ms 9.87 |
| warm-up | 5.01 | 53.1 | pipelines/compile_ms 1709, pipelines/front_ms 1652, pipelines/wait_ms 297, render/thread/ms 82, render/busy/ms 80.10, main/busy/ms 76.22, pipelines/driver_ms 57.39, stage/main/update 44.62, stage/render/prepare_assets 32.23, stage/main/post_update 19.65, stage/render/render 17.48, render/graph/ms 14.50 |
| warm-up | 5.07 | 55.1 | pipelines/compile_ms 1307, pipelines/front_ms 1261, main/busy/ms 134, stage/main/update 104, render/thread/ms 81.76, pipelines/wait_ms 81.37, render/busy/ms 79.96, stream/drive/ms 58.98, stage/render/prepare_assets 46.96, pipelines/driver_ms 45.88, stage/render/render 16.08, stage/main/post_update 15.18 |
| warm-up | 5.19 | 122.9 | pipelines/compile_ms 921, pipelines/front_ms 900, main/busy/ms 104, stage/main/update 80.60, render/thread/ms 75.10, render/busy/ms 73.90, stream/drive/ms 57.39, stage/render/prepare_assets 47.85, pipelines/driver_ms 20.87, stage/render/render 11.72, stage/main/post_update 11.66, render/graph/ms 9.97 |
| warm-up | 5.25 | 31.3 | pipelines/compile_ms 123, pipelines/front_ms 116, main/busy/ms 52.55, render/thread/ms 46.53, render/busy/ms 45.00, stage/main/update 35.17, stage/render/render 20.28, render/graph/ms 18.17, stage/render/graph_submit 12.69, stream/drive/ms 10.74, encode//sum 9.64, stage/main/post_update 8.84 |
| warm-up | 5.28 | 38.9 | pipelines/compile_ms 94.01, pipelines/front_ms 90.03, render/thread/ms 55.49, render/busy/ms 54.12, main/busy/ms 53.67, stage/main/update 31.31, stage/render/prepare_assets 16.87, stage/main/post_update 15.61, stage/render/render 14.58, render/graph/ms 12.44, stream/drive/ms 11.14, encode//sum 8.98 |
| warm-up | 5.32 | 31.9 | render/thread/ms 56.01, render/busy/ms 54.70, main/busy/ms 38.94, pipelines/compile_ms 21.46, stage/render/prepare_assets 21.11, pipelines/front_ms 20.94, stage/main/update 16.47, stage/main/post_update 14.35, stage/render/render 11.87, render/graph/ms 9.99, encode//sum 7.49, stage/render/graph_submit 6.70 |
| warm-up | 5.35 | 31.1 | render/thread/ms 51.64, render/busy/ms 50.34, main/busy/ms 32.19, stage/render/render 19.29, render/graph/ms 17.45, stage/render/prepare_assets 15.82, stage/main/update 15.16, stage/render/graph_submit 14.15, encode/submit 11.12, stage/main/post_update 8.95, encode//sum 8.54, stream/drive/ms 5.37 |
| warm-up | 5.45 | 32.9 | pipelines/compile_ms 669, pipelines/front_ms 649, main/busy/ms 41.51, render/thread/ms 38.74, render/busy/ms 37.04, pipelines/driver_ms 19.70, stage/main/post_update 17.43, stage/main/update 17.12, stage/render/render 15.40, render/graph/ms 13.24, stage/render/graph_submit 9.93, encode//sum 7.70 |
| warm-up | 5.51 | 25.4 | pipelines/compile_ms 192, pipelines/front_ms 187, main/busy/ms 34.22, render/thread/ms 32.94, render/busy/ms 31.47, stage/main/update 21.71, stage/render/render 14.98, render/graph/ms 12.77, stream/drive/ms 11.76, encode//sum 9.80, stage/render/graph_submit 8.23, stage/render/prepare_assets 7.99 |
| warm-up | 5.98 | 27.1 | main/busy/ms 31.41, render/thread/ms 21.06, stage/main/update 20.69, render/busy/ms 20.05, stream/drive/ms 12.88, stage/render/render 9.12, render/graph/ms 7.71, encode//sum 6.72, stage/main/post_update 6.62, stage/render/graph_submit 5.21, stage/render/prepare_assets 4.33, encode/submit 2.69 |
| warm-up | 6.10 | 27.6 | render/thread/ms 40.14, render/busy/ms 38.48, main/busy/ms 31.76, stage/render/render 15.51, stage/main/update 14.86, stage/main/post_update 13.05, render/graph/ms 13.03, pipelines/compile_ms 12.68, pipelines/front_ms 11.68, stage/render/prepare_assets 9.92, encode//sum 9.80, stage/render/graph_submit 8.60 |
| warm-up | 6.15 | 29.2 | render/thread/ms 37.39, render/busy/ms 36.38, main/busy/ms 26.09, stage/render/prepare_assets 20.29, stage/main/update 10.44, stage/main/post_update 8.82, stage/render/render 8.14, render/graph/ms 6.75, encode//sum 6.55, stage/render/graph_submit 4.46, stage/main/fixed_main_loop 2.90, stream/drive/ms 2.55 |
| warm-up | 6.25 | 28.7 | pipelines/compile_ms 139, pipelines/front_ms 136, render/thread/ms 36.69, main/busy/ms 36.02, render/busy/ms 35.21, stage/main/update 22.92, stage/render/prepare_assets 11.60, stage/render/render 11.08, render/graph/ms 9.25, encode//sum 8.06, stage/main/post_update 7.31, stage/render/graph_submit 6.73 |
| warm-up | 6.38 | 37.5 | main/busy/ms 45.99, render/thread/ms 38.72, render/busy/ms 37.41, stage/main/update 26.09, stage/render/render 16.12, render/graph/ms 14.10, stage/main/post_update 13.34, stage/render/prepare_assets 10.83, stage/render/graph_submit 10.68, encode//sum 8.74, encode/submit 7.68, stream/drive/ms 4.74 |
| warm-up | 6.41 | 31.8 | render/thread/ms 46.62, render/busy/ms 45.57, main/busy/ms 36.29, stage/main/update 18.65, stage/render/prepare_assets 15.01, stage/render/render 12.14, render/graph/ms 10.66, stage/main/post_update 8.91, stage/render/graph_submit 7.83, stage/render/prepare_meshes 6.39, encode//sum 6.17, encode/finish_wall 4.92 |
| warm-up | 6.44 | 26.7 | pipelines/compile_ms 136, pipelines/front_ms 134, render/thread/ms 49.25, render/busy/ms 48.06, main/busy/ms 41.24, stage/main/update 20.05, stage/render/render 17.19, render/graph/ms 15.68, stage/main/post_update 12.51, stage/render/graph_submit 12.51, stage/render/prepare_assets 11.93, encode/finish_wall 8.12 |
| warm-up | 6.48 | 37.4 | pipelines/compile_ms 145, pipelines/front_ms 142, render/thread/ms 58.04, render/busy/ms 56.55, main/busy/ms 47.69, stage/render/render 24.96, stage/main/update 23.70, render/graph/ms 23.02, stage/render/graph_submit 18.26, stage/main/post_update 17.96, encode/finish_wall 10.55, encode/pass:point_shadow_atlas_live_indirect/sum 10.03 |
| warm-up | 6.50 | 26.0 | render/thread/ms 56.15, render/busy/ms 54.74, main/busy/ms 41.90, stage/render/render 23.16, stage/main/update 22.14, render/graph/ms 21.21, stage/render/graph_submit 16.65, stage/main/post_update 13.66, encode/finish_wall 10.00, stage/render/prepare_assets 9.93, encode/pass:point_shadow_atlas_live_indirect/sum 9.54, pipelines/compile_ms 9.05 |
| warm-up | 6.54 | 30.7 | render/thread/ms 42.97, render/busy/ms 41.94, main/busy/ms 33.52, stage/main/update 17.19, stage/render/prepare_assets 14.93, stage/render/render 13.25, render/graph/ms 11.79, stage/main/post_update 9.83, stage/render/graph_submit 8.99, encode//sum 6.37, encode/finish_wall 6.18, encode/pass:point_shadow_atlas_live_indirect/sum 5.88 |
| warm-up | 6.66 | 31.3 | main/busy/ms 42.19, render/thread/ms 38.36, render/busy/ms 37.13, stage/main/update 24.66, pipelines/compile_ms 20.88, pipelines/front_ms 19.69, stage/render/render 17.66, render/graph/ms 15.85, stage/main/post_update 12.66, stream/drive/ms 12.62, stage/render/graph_submit 12.44, encode//sum 7.31 |
| warm-up | 6.69 | 29.8 | main/busy/ms 54.33, render/thread/ms 47.85, render/busy/ms 46.25, stage/main/update 33.53, stage/render/render 22.89, render/graph/ms 20.47, stage/main/post_update 15.44, stage/render/graph_submit 15.30, stage/render/prepare_assets 9.90, encode/finish_wall 8.77, encode/pass:point_shadow_atlas_live_indirect/sum 8.35, encode//sum 8.00 |
| warm-up | 6.74 | 46.0 | main/busy/ms 54.64, render/thread/ms 52.69, render/busy/ms 51.18, stage/main/update 34.79, stage/render/render 22.03, render/graph/ms 19.75, stage/render/prepare_assets 16.80, stage/render/graph_submit 15.59, stage/main/post_update 14.40, encode/finish_wall 9.05, encode/pass:point_shadow_atlas_live_indirect/sum 8.64, encode//sum 7.46 |
| measure | 18.31 | 34.3 | main/busy/ms 39.23, stage/main/update 26.53, render/thread/ms 21.36, render/busy/ms 20.36, stage/render/render 10.53, render/graph/ms 9.06, stage/main/post_update 8.86, stage/render/graph_submit 6.24, encode//sum 6.17, encode/submit 3.95, stage/render/prepare_assets 3.90, stage/render/camera_driver 2.67 |
| measure | 59.25 | 31.0 | render/thread/ms 37.17, render/busy/ms 36.09, main/busy/ms 24.31, stage/render/prepare_assets 16.61, stage/main/post_update 12.08, stage/render/render 10.42, render/graph/ms 9.07, encode//sum 7.38, stage/render/graph_submit 5.74, stage/main/update 5.08, encode/submit 3.53, stage/main/fixed_main_loop 3.22 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.max_lamps=16 --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=lamps16-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 0%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1197 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **81%** (min 53%) · 2622 MHz (min 2610) · memory 8101 MHz · 82.3 W · 79 °C (max 84) · VRAM 5646 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.max_lamps=16 (command line: --set)`
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
