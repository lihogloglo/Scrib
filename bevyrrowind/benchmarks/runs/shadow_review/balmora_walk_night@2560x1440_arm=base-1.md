# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6643 frames over 65.6 s · **median 9.79 ms (102 fps)** · mean 9.88 · p95 11.56 · p99 13.20 (1% low 76 fps) · worst 35.94 ms · 30 spikes over 1.5× median · 27.4 stutters/min (work 27.4/min)
- warm-up: 354 frames over 9.0 s, worst 2387.2 ms
- hitches (> 25.0 ms): **4** measured, 43 in warm-up
- timed GPU passes: 6.39 ms of a 9.88 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.79 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- **FAIL** worst frame 35.94 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.836 | 2.033 | 2.066 | 0.90 | 0.174 |
| main_opaque_pass_3d | 1.478 | 1.636 | 2.369 | 0.90 | 0.063 |
| early prepass | 0.465 | 0.515 | 0.754 | 0.90 | 0.088 |
| bloom | 0.287 | 0.317 | 0.325 | 0.90 | 0.064 |
| atmosphere_luts | 0.283 | 0.313 | 0.377 | 0.90 | 0.003 |
| water_velocity | 0.203 | 0.225 | 0.247 | 0.90 | 0.002 |
| water_sspr | 0.182 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.170 | 0.188 | 0.191 | 0.90 | 0.008 |
| point_shadow_atlas | 0.137 | 0.252 | 0.474 | 0.54 | 0.237 |
| upscaling | 0.117 | 0.129 | 0.136 | 0.90 | 0.004 |
| atmos_froxel_inject | 0.116 | 0.128 | 0.129 | 0.90 | 0.001 |
| water_fft | 0.111 | 0.123 | 0.126 | 0.90 | 0.003 |
| uniform_allocation | 0.105 | 0.116 | 0.142 | 0.90 | 0.021 |
| clouds_composite | 0.095 | 0.105 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.083 | 0.092 | 0.114 | 0.90 | 0.016 |
| gpu_instance_materialization | 0.082 | 0.091 | 0.095 | 0.90 | 0.004 |
| tonemapping | 0.078 | 0.087 | 0.092 | 0.90 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.073 | 0.144 | 0.90 | 0.008 |
| clustering | 0.058 | 0.064 | 0.068 | 0.90 | 0.019 |
| clouds_trace | 0.050 | 0.056 | 0.079 | 0.90 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.054 | 0.070 | 0.90 | 0.003 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.90 | 0.002 |
| clouds_resolve | 0.045 | 0.050 | 0.051 | 0.90 | 0.001 |
| surface_trail | 0.033 | 0.057 | 0.057 | 0.59 | 0.002 |
| lightprobe_irradiance_map | 0.031 | 0.415 | 0.417 | 0.08 | 0.001 |
| atmos_froxel_integrate | 0.026 | 0.029 | 0.031 | 0.90 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.026 | 0.048 | 0.071 | 0.54 | 0.010 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.90 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.90 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.90 | 0.003 |
| lightprobe_radiance_map | 0.017 | 0.220 | 0.230 | 0.08 | 0.004 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.017 | 0.026 | 0.90 | 0.005 |
| main_transparent_pass_3d | 0.014 | 0.016 | 0.030 | 0.90 | 0.183 |
| point_shadow_atlas/uniform_allocation | 0.014 | 0.026 | 0.060 | 0.54 | 0.007 |
| point_shadow_atlas/bin_unpacking | 0.010 | 0.019 | 0.039 | 0.54 | 0.005 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.90 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.90 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.54 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.90 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.649 | 10.008 | 21.45 | 57456.5 |
| render/busy/ms | 8.126 | 9.468 | 20.93 | 53983.5 |
| main/busy/ms | 8.058 | 9.261 | 28.71 | 53529.9 |
| stage/render/render | 5.068 | 6.060 | 10.98 | 33669.7 |
| stage/main/post_update | 4.527 | 5.367 | 8.38 | 30074.0 |
| render/graph/ms | 4.390 | 5.269 | 9.79 | 29162.9 |
| encode//sum | 3.222 | 3.992 | 8.32 | 21403.2 |
| stage/render/graph_submit | 3.087 | 3.900 | 6.71 | 20509.1 |
| encode/submit | 1.951 | 2.837 | 4.13 | 12960.6 |
| stage/main/update | 1.838 | 2.308 | 23.51 | 12207.4 |
| stage/render/camera_driver | 1.241 | 1.495 | 2.96 | 8246.9 |
| encode/finish_wall | 1.222 | 1.606 | 3.10 | 8118.8 |
| encode/pass:early prepass/sum | 0.819 | 1.067 | 2.04 | 5442.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.814 | 1.064 | 1.88 | 5407.2 |
| encode/pass:shadow_directional_light/sum | 0.814 | 1.224 | 2.80 | 5407.0 |
| stage/render/prepare_write_phase_buffers | 0.585 | 0.880 | 1.97 | 3888.1 |
| bigspace/total/ms | 0.550 | 1.307 | 2.07 | 3654.5 |
| stage/main/fixed_main_loop | 0.511 | 0.977 | 2.56 | 3397.6 |
| stage/render/prepare_assets | 0.506 | 0.654 | 11.76 | 3363.8 |
| render/present/ms | 0.429 | 1.187 | 2.06 | 2849.6 |
| stage/render/prepare_resources | 0.415 | 0.488 | 1.53 | 2753.8 |
| stage/render/prepare_meshes | 0.412 | 0.513 | 3.44 | 2737.0 |
| stage/main/pre_update | 0.386 | 0.532 | 2.69 | 2562.9 |
| stage/main/first | 0.364 | 0.482 | 5.96 | 2415.8 |
| stage/render/prepare_flush | 0.319 | 0.525 | 1.41 | 2116.7 |
| controller/feed/ms | 0.315 | 0.395 | 0.83 | 2089.8 |
| stage/render/prepare_bind_groups | 0.301 | 0.496 | 1.66 | 2000.5 |
| stage/render/prepare_batch_phases | 0.270 | 0.416 | 0.78 | 1794.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.263 | 0.430 | 0.80 | 1746.1 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.257 | 1.125 | 2.12 | 1710.4 |
| stage/main/last | 0.254 | 0.295 | 1.25 | 1687.4 |
| bigspace/low_precision/ms | 0.231 | 0.694 | 1.50 | 1531.3 |
| stream/drive/ms | 0.203 | 0.277 | 10.14 | 1350.6 |
| stage/render/create_views | 0.198 | 0.268 | 0.59 | 1313.6 |
| stage/render/extract_commands | 0.183 | 0.214 | 0.66 | 1215.6 |
| bigspace/high_precision/ms | 0.173 | 0.759 | 1.62 | 1150.6 |
| anim/evaluate/ms | 0.160 | 0.298 | 1.46 | 1063.1 |
| stage/render/specialize | 0.152 | 0.239 | 1.63 | 1012.7 |
| stream/tick/ms | 0.150 | 0.212 | 0.52 | 996.7 |
| anim/evaluate_cpu/ms | 0.149 | 0.190 | 0.65 | 990.3 |
| stage/render/prepare_views | 0.135 | 0.173 | 1.31 | 896.4 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.109 | 0.839 | 1.54 | 722.2 |
| stage/render/queue | 0.097 | 0.169 | 1.82 | 647.5 |
| render/acquire/ms | 0.094 | 0.124 | 1.26 | 623.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.090 | 0.143 | 0.61 | 596.5 |
| stage/render/phase_sort | 0.081 | 0.111 | 0.32 | 537.6 |
| controller/move/ms | 0.076 | 0.224 | 0.86 | 503.3 |
| controller/write/ms | 0.074 | 0.101 | 0.29 | 490.0 |
| terrain/update/ms | 0.059 | 0.289 | 1.86 | 391.3 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.053 | 0.000 | 2.71 | 349.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.073 | 0.38 | 307.8 |
| physics/step/ms | 0.044 | 0.118 | 0.57 | 295.1 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.52 | 212.1 |
| pipelines/compile_ms | 0.024 | 0.000 | 77.84 | 162.0 |
| physics/insert/ms | 0.024 | 0.285 | 0.96 | 156.9 |
| pipelines/front_ms | 0.024 | 0.000 | 74.78 | 156.1 |
| stage/render/cleanup | 0.023 | 0.031 | 0.45 | 154.2 |
| terrain/select/ms | 0.022 | 0.255 | 0.92 | 143.9 |
| weather/map/ms | 0.018 | 0.024 | 0.07 | 118.3 |
| far/select/ms | 0.015 | 0.164 | 0.36 | 96.8 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.30 | 84.2 |
| anim/apply/ms | 0.011 | 0.021 | 0.38 | 74.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 70.0 |
| encode/camera_driver_span_end/sum | 0.010 | 0.020 | 0.33 | 69.4 |
| script/update/ms | 0.008 | 0.011 | 0.25 | 52.2 |
| controller/avoid/ms | 0.006 | 0.012 | 0.03 | 38.0 |
| stream/despawn/ms | 0.005 | 0.000 | 0.95 | 35.9 |
| script/input/ms | 0.005 | 0.006 | 0.09 | 32.1 |
| script/ui/ms | 0.003 | 0.005 | 0.24 | 22.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.12 | 21.5 |
| stream/upload/ms | 0.003 | 0.000 | 0.99 | 18.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.69 | 13.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.06 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 18 | 34 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 357 | 646 | 162 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5539 | 5942 | 5613 |
| crowd/c1/meshes/count | count | 1181 | 1272 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 205 |
| crowd/joints_extracted/count | count | 662 | 5537 | 463 |
| crowd/joints_uploaded/count | count | 3607 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.40 | 1.52 | 0.59 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.30 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.33 | 0.01 |
| encode/finish_critical | ms | 0.97 | 2.80 | 1.06 |
| encode/pass:early prepass/max | ms | 0.82 | 2.04 | 1.06 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 1.88 | 0.81 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.61 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.80 | 0.39 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.84 | 2.12 | 0.77 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.31 | 1.54 | 0.21 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.08 | 2.71 | 0.94 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.80 | 0.48 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.52 | 0.05 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.54 | 28.71 | 8.95 |
| gpu/timed/ms | ms | 7.85 | 9.41 | 8.89 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 154 | 386 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 386 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 767 | 971 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2479 | 3552 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 767 | 971 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2479 | 3552 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 75 | 417 | 8 |
| phases/shadow/bins | count | 836 | 1551 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2309 | 3288 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 50 | 153 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2679 | 3183 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 15 | 275 | 11 |
| preprocess/bind_groups_created | count | 10 | 265 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 137 | 226 | 123 |
| process/cpu_usage | % | 26.01 | 39.42 | 37.02 |
| process/mem_usage | GiB | 5.11 | 5.43 | 5.43 |
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
| shadow_atlas/empty_faces | count | 0 | 2 | 0 |
| shadow_atlas/lamps | count | 8 | 19 | 0 |
| shadow_atlas/live_occ_pct | % | 4.88 | 37.60 | 0 |
| shadow_atlas/live_tiles | count | 17 | 70 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 160 | 490 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 19 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 61 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 112 | 0 |
| shadow_atlas/views | count | 17 | 61 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 95 | 11 |
| skin/blocks_skipped/count | count | 146 | 205 | 194 |
| skin/blocks_walked/count | count | 14 | 25 | 11 |
| skin/joint_bytes_written/bytes | bytes | 46.8 k | 807.7 k | 35.3 k |
| skin/joints_written/count | count | 732 | 12.6 k | 552 |
| skin/upload_writes/count | count | 7 | 28 | 10 |
| started_load_count |  | 6439 | 6563 | 6563 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.46 | 37.55 | 34.80 |
| system/mem_usage | % | 53.20 | 54.19 | 54.15 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 133 | 472 | 66 |
| views/all/bins | count | 2933 | 3567 | 2948 |
| views/all/count | count | 20 | 64 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8016 | 10.5 k | 7964 |
| views/all/pipelines | count | 97 | 202 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 231.1 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 714 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2122 | 2427 | 2022 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 61 | 404 | 0 |
| views/lamp_face/bins | count | 99 | 713 | 0 |
| views/lamp_face/count | count | 17 | 61 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 138 | 867 | 0 |
| views/lamp_face/pipelines | count | 25 | 127 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 110 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5906 | 7524 | 5942 |
| views/main/pipelines | count | 46 | 49 | 46 |
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
| world/meshes | count | 21.3 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.99 | 227 | 318.8 k |
| anim/events/count | count | 0.08 | 3 | 524 |
| assets/Image/added | count | 0.09 | 32 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 257 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7467 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1260 | 1489 |
| encode//count | count | 27.91 | 30 | 185.4 k |
| encode/bind_walk/init_textures | count | 9085 | 23.1 k | 60.35 M |
| encode/bind_walk/init_textures_skipped | count | 47.0 k | 70.7 k | 312.16 M |
| encode/bind_walk/inits | count | 563 | 778 | 3.74 M |
| encode/bind_walk/inits_skipped | count | 167 | 251 | 1.11 M |
| encode/bind_walk/merge_textures | count | 9023 | 23.0 k | 59.94 M |
| encode/bind_walk/merge_textures_skipped | count | 9453 | 23.8 k | 62.80 M |
| encode/bind_walk/merges | count | 104 | 181 | 693.3 k |
| encode/bind_walk/merges_skipped | count | 149 | 220 | 988.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6643 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6643 |
| encode/pass:early prepass/count | count | 1 | 1 | 6643 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6643 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6643 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6643 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.29 | 1 | 1935 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.23 | 1 | 1559 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 302 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6643 |
| encode/queue/command_buffers | count | 38.49 | 42 | 255.7 k |
| encode/queue/submits | count | 2.01 | 4 | 13.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.18 M | 15.41 G |
| encode/queue/write_buffer/calls | count | 272 | 460 | 1.81 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.2 k | 1.29 M | 1.13 G |
| encode/queue/write_buffer_with/calls | count | 10.28 | 780 | 68.3 k |
| encode/queue/write_texture/bytes | bytes | 1986 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6643 |
| physics/inserts | count | 0.34 | 87 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 147 | 2335 |
| physics/steps | count | 0.59 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 24 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5096 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1117 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.39 | 2387.2 | main/busy/ms 2353, stage/main/post_update 2310, stage/main/update 32.01, stage/main/pre_update 7.26, stage/main/last 1.14, surface/advance_cpu_ms 0.80, stage/main/first 0.58, stage/main/fixed_main_loop 0.54, stream/drive/ms 0.43, surface/cell_upload_ms 0.28, stream/tick/ms 0.12, bigspace/total/ms 0.07 |
| warm-up | 3.13 | 746.5 | main/busy/ms 2487, stage/main/post_update 2334, render/thread/ms 709, render/busy/ms 581, stage/render/prepare_assets 526, stage/main/update 136, stage/render/prepare_views 125, render/acquire/ms 125, stage/render/create_views 29.54, stage/render/render 16.05, render/graph/ms 12.18, stage/render/camera_driver 10.32 |
| warm-up | 3.24 | 105.3 | render/thread/ms 796, render/busy/ms 618, stage/render/prepare_assets 549, main/busy/ms 154, stage/render/prepare_views 126, render/acquire/ms 125, stage/main/update 109, stage/render/render 72.80, pipelines/compile_ms 57.79, render/present/ms 53.25, pipelines/driver_ms 35.80, stage/main/post_update 33.97 |
| warm-up | 3.37 | 120.6 | pipelines/wait_ms 2833, pipelines/compile_ms 1021, pipelines/front_ms 932, render/thread/ms 100, render/busy/ms 97.61, pipelines/driver_ms 89.82, main/busy/ms 60.48, stage/main/post_update 47.81, encode//sum 43.75, stage/render/prepare_flush 34.03, stage/render/prepare_resources 33.58, stage/render/render 14.26 |
| warm-up | 3.48 | 112.1 | pipelines/wait_ms 4648, pipelines/compile_ms 1902, pipelines/front_ms 1775, render/thread/ms 200, render/busy/ms 198, pipelines/driver_ms 127, main/busy/ms 120, encode//sum 65.94, stage/render/render 60.11, render/graph/ms 57.63, stage/render/prepare_resources 55.92, stage/render/graph_submit 55.15 |
| warm-up | 3.55 | 62.4 | pipelines/wait_ms 1912, pipelines/compile_ms 1561, pipelines/front_ms 1469, render/thread/ms 170, render/busy/ms 167, main/busy/ms 119, pipelines/driver_ms 91.35, stage/render/prepare_assets 77.61, stage/render/render 57.85, stage/main/update 57.59, render/graph/ms 55.45, stage/render/graph_submit 52.72 |
| warm-up | 3.58 | 37.2 | pipelines/compile_ms 1355, pipelines/front_ms 1286, pipelines/wait_ms 151, render/thread/ms 93.93, render/busy/ms 92.24, stage/render/prepare_assets 73.34, pipelines/driver_ms 68.62, main/busy/ms 39.48, stage/main/post_update 16.91, stage/main/update 12.96, stage/render/render 11.67, terrain/update/ms 10.31 |
| warm-up | 3.62 | 34.8 | pipelines/compile_ms 1277, pipelines/front_ms 1214, pipelines/wait_ms 107, render/thread/ms 67.97, render/busy/ms 65.61, pipelines/driver_ms 62.67, main/busy/ms 55.07, stage/render/prepare_assets 32.03, stage/main/post_update 22.28, encode//sum 16.20, stage/main/pre_update 14.27, stage/render/render 13.03 |
| warm-up | 3.69 | 45.9 | pipelines/compile_ms 637, pipelines/front_ms 612, pipelines/wait_ms 99.39, main/busy/ms 78.72, render/thread/ms 61.05, render/busy/ms 58.64, stage/main/update 54.40, pipelines/driver_ms 24.55, stage/render/render 23.19, render/graph/ms 20.52, stage/main/post_update 17.61, stage/render/graph_submit 16.56 |
| warm-up | 3.78 | 88.6 | pipelines/compile_ms 1113, pipelines/front_ms 1061, render/thread/ms 97.76, render/busy/ms 95.59, pipelines/wait_ms 84.01, main/busy/ms 78.26, stage/main/update 60.81, pipelines/driver_ms 52.53, stage/render/prepare_assets 48.20, stage/render/render 26.59, render/graph/ms 23.98, stage/render/graph_submit 19.37 |
| warm-up | 3.80 | 25.3 | pipelines/compile_ms 1556, pipelines/front_ms 1486, render/thread/ms 70.73, pipelines/driver_ms 69.88, render/busy/ms 69.20, stage/render/prepare_assets 47.45, main/busy/ms 32.30, pipelines/wait_ms 21.39, stage/main/update 19.96, stage/render/render 14.66, render/graph/ms 12.70, stage/render/graph_submit 9.74 |
| warm-up | 3.85 | 48.0 | pipelines/compile_ms 1111, pipelines/front_ms 1065, pipelines/driver_ms 46.35, render/thread/ms 42.18, render/busy/ms 40.79, stage/render/prepare_assets 26.89, pipelines/wait_ms 21.28, main/busy/ms 18.55, stage/render/render 9.49, stage/main/update 8.27, render/graph/ms 7.79, encode//sum 6.58 |
| warm-up | 3.92 | 56.9 | pipelines/compile_ms 281, pipelines/front_ms 268, render/thread/ms 62.91, stage/render/prepare_views 42.25, render/acquire/ms 42.12, main/busy/ms 21.21, render/busy/ms 19.57, encode//sum 16.30, stage/render/render 13.51, pipelines/driver_ms 12.94, stage/main/update 12.22, render/graph/ms 11.72 |
| warm-up | 4.01 | 53.5 | pipelines/compile_ms 923, pipelines/front_ms 892, render/thread/ms 53.92, render/busy/ms 51.72, main/busy/ms 32.44, pipelines/driver_ms 30.95, stage/render/prepare_assets 21.65, stage/render/render 21.32, render/graph/ms 18.56, stage/main/update 15.13, stage/main/post_update 9.28, stage/render/graph_submit 9.10 |
| warm-up | 4.05 | 38.6 | pipelines/compile_ms 1173, pipelines/front_ms 1133, main/busy/ms 164, stage/main/update 143, render/thread/ms 68.10, render/busy/ms 65.16, pipelines/driver_ms 39.49, stage/render/prepare_assets 27.94, stage/render/render 24.61, render/graph/ms 21.28, stage/main/post_update 11.37, stage/render/graph_submit 11.04 |
| warm-up | 4.29 | 238.6 | pipelines/compile_ms 3065, pipelines/front_ms 2979, pipelines/wait_ms 256, main/busy/ms 182, stage/main/update 165, render/thread/ms 112, render/busy/ms 110, pipelines/driver_ms 86.27, stage/render/prepare_assets 85.12, pipelines/lock_ms 34.48, stage/render/render 13.08, render/graph/ms 10.63 |
| warm-up | 4.34 | 49.6 | pipelines/compile_ms 2633, pipelines/front_ms 2564, pipelines/wait_ms 250, render/thread/ms 118, render/busy/ms 117, stage/render/prepare_assets 95.89, pipelines/driver_ms 69.64, main/busy/ms 56.97, stage/main/update 40.58, pipelines/lock_ms 28.62, stage/render/render 10.86, render/graph/ms 9.02 |
| warm-up | 4.41 | 73.1 | render/thread/ms 107, render/busy/ms 105, main/busy/ms 75.63, pipelines/compile_ms 72.00, pipelines/front_ms 68.31, stage/main/update 41.58, stage/render/prepare_write_phase_buffers 36.46, stage/render/prepare_assets 28.02, stage/render/prepare_flush 19.09, stage/main/last 14.34, stage/render/render 13.20, render/graph/ms 11.37 |
| warm-up | 4.45 | 36.0 | render/thread/ms 98.65, render/busy/ms 97.19, main/busy/ms 79.55, pipelines/compile_ms 64.54, pipelines/front_ms 61.10, stage/main/update 43.01, stage/render/prepare_write_phase_buffers 38.82, stage/render/prepare_flush 19.40, stage/render/render 15.33, stage/main/last 14.26, stage/main/post_update 12.52, stage/render/prepare_assets 11.96 |
| warm-up | 4.53 | 80.4 | pipelines/compile_ms 1449, pipelines/front_ms 1406, pipelines/wait_ms 434, render/thread/ms 103, render/busy/ms 101, main/busy/ms 68.81, stage/render/prepare_assets 46.44, pipelines/driver_ms 42.47, stage/main/post_update 35.56, stage/main/update 26.14, stage/render/prepare_flush 17.03, stage/render/render 14.33 |
| warm-up | 4.58 | 44.6 | pipelines/compile_ms 2773, pipelines/front_ms 2692, pipelines/wait_ms 575, render/thread/ms 114, render/busy/ms 112, main/busy/ms 105, pipelines/driver_ms 80.82, stage/main/update 64.34, stage/render/prepare_assets 54.41, stage/main/post_update 32.00, stage/render/prepare_flush 21.67, stream/drive/ms 21.54 |
| warm-up | 4.71 | 137.4 | pipelines/compile_ms 2390, pipelines/front_ms 2319, pipelines/wait_ms 274, render/thread/ms 128, render/busy/ms 126, stage/render/prepare_assets 91.32, main/busy/ms 86.94, pipelines/driver_ms 71.26, stage/main/update 60.85, stage/render/render 18.49, render/graph/ms 15.71, stage/main/post_update 13.76 |
| warm-up | 4.75 | 35.6 | pipelines/compile_ms 1364, pipelines/front_ms 1321, pipelines/wait_ms 138, render/thread/ms 118, render/busy/ms 117, stage/render/prepare_assets 83.75, main/busy/ms 49.36, pipelines/driver_ms 43.72, stage/main/update 21.80, stage/render/render 16.53, render/graph/ms 14.75, stage/main/post_update 13.74 |
| warm-up | 4.80 | 47.0 | pipelines/compile_ms 351, pipelines/front_ms 337, render/thread/ms 64.30, render/busy/ms 62.92, main/busy/ms 55.13, stage/main/update 29.49, stage/render/prepare_assets 22.71, stage/render/render 15.94, stage/main/post_update 14.76, stream/drive/ms 14.36, render/graph/ms 14.13, pipelines/driver_ms 13.81 |
| warm-up | 4.82 | 25.3 | main/busy/ms 92.54, pipelines/compile_ms 83.87, pipelines/front_ms 79.43, stage/main/update 69.68, render/thread/ms 54.74, render/busy/ms 53.32, stream/drive/ms 39.19, stage/render/prepare_assets 19.16, stage/render/render 15.36, stage/main/post_update 14.93, render/graph/ms 13.43, stage/render/graph_submit 9.92 |
| warm-up | 4.90 | 83.5 | pipelines/compile_ms 1070, pipelines/front_ms 1042, main/busy/ms 101, stage/main/update 84.52, stream/drive/ms 55.57, render/thread/ms 51.16, render/busy/ms 49.67, pipelines/driver_ms 28.70, stage/render/prepare_assets 20.38, stage/render/render 15.99, render/graph/ms 14.03, stage/render/graph_submit 10.83 |
| warm-up | 4.94 | 32.1 | pipelines/compile_ms 1124, pipelines/front_ms 1094, main/busy/ms 61.31, render/thread/ms 47.67, render/busy/ms 46.29, stage/main/update 45.11, pipelines/driver_ms 30.37, stream/drive/ms 29.60, stage/render/prepare_assets 21.09, stage/render/render 12.95, render/graph/ms 11.04, stage/render/graph_submit 7.98 |
| warm-up | 4.96 | 27.5 | pipelines/compile_ms 253, pipelines/front_ms 240, render/thread/ms 39.81, render/busy/ms 38.47, main/busy/ms 37.75, stage/main/update 19.65, pipelines/driver_ms 13.06, stage/render/render 10.80, stage/main/post_update 9.77, render/graph/ms 9.09, stage/render/prepare_assets 8.40, encode//sum 6.67 |
| warm-up | 5.00 | 34.9 | pipelines/compile_ms 167, pipelines/front_ms 157, render/thread/ms 54.87, render/busy/ms 53.33, main/busy/ms 44.33, stage/main/update 26.35, stage/render/render 13.19, render/graph/ms 11.43, stage/main/post_update 11.28, pipelines/driver_ms 9.97, stage/render/prepare_assets 9.45, stage/render/prepare_resources 8.88 |
| warm-up | 5.03 | 30.8 | render/thread/ms 54.28, render/busy/ms 52.81, main/busy/ms 41.98, stage/main/update 24.38, stage/render/prepare_assets 15.58, stage/render/render 13.79, render/graph/ms 11.99, stage/main/post_update 10.73, stage/render/graph_submit 8.41, stage/render/prepare_write_phase_buffers 8.04, encode//sum 7.79, encode/submit 6.20 |
| warm-up | 5.23 | 28.1 | main/busy/ms 39.11, render/thread/ms 36.71, render/busy/ms 35.36, stage/main/update 21.09, stream/drive/ms 13.96, stage/render/render 11.77, render/graph/ms 10.00, stage/main/post_update 9.97, stage/render/prepare_assets 9.28, encode//sum 6.79, stage/render/graph_submit 6.53, stage/main/fixed_main_loop 4.46 |
| warm-up | 5.28 | 28.8 | pipelines/compile_ms 531, pipelines/front_ms 519, render/thread/ms 38.77, render/busy/ms 37.39, main/busy/ms 28.90, stage/render/render 14.58, stage/main/update 14.31, encode//sum 12.71, render/graph/ms 12.56, pipelines/driver_ms 11.93, stage/render/prepare_assets 10.64, stage/main/post_update 8.65 |
| warm-up | 5.47 | 28.5 | pipelines/compile_ms 676, pipelines/front_ms 658, main/busy/ms 37.73, render/thread/ms 29.27, render/busy/ms 27.92, stage/main/update 23.15, pipelines/driver_ms 17.71, stream/drive/ms 15.16, stage/render/render 12.00, render/graph/ms 10.07, stage/main/post_update 8.47, encode//sum 7.75 |
| warm-up | 5.72 | 28.3 | main/busy/ms 32.81, render/thread/ms 23.36, render/busy/ms 22.47, stage/main/update 21.72, stream/drive/ms 13.34, stage/render/render 9.51, render/graph/ms 8.15, encode//sum 7.39, stage/main/post_update 7.31, stage/render/prepare_assets 6.01, stage/render/graph_submit 5.29, encode/submit 2.94 |
| warm-up | 5.85 | 29.8 | pipelines/compile_ms 271, pipelines/front_ms 260, render/thread/ms 43.00, render/busy/ms 42.04, main/busy/ms 31.71, stage/render/prepare_assets 18.76, stage/main/update 15.00, pipelines/driver_ms 11.84, stage/render/render 11.09, stage/main/post_update 10.47, render/graph/ms 9.60, encode//sum 8.53 |
| warm-up | 6.03 | 28.3 | render/thread/ms 37.99, render/busy/ms 37.03, main/busy/ms 35.79, stage/main/update 21.81, stage/render/render 12.62, render/graph/ms 11.18, stage/render/prepare_assets 10.90, stage/main/post_update 9.73, encode//sum 8.94, stage/render/graph_submit 7.65, stage/render/prepare_meshes 4.99, encode/submit 4.59 |
| warm-up | 6.08 | 28.3 | main/busy/ms 38.97, render/thread/ms 38.40, render/busy/ms 36.83, stage/main/update 21.62, stage/render/render 18.14, render/graph/ms 15.79, stage/main/post_update 12.76, stage/render/graph_submit 11.49, encode//sum 8.71, encode/submit 8.34, stage/render/camera_driver 4.12, stream/drive/ms 4.01 |
| warm-up | 6.11 | 30.9 | pipelines/compile_ms 139, pipelines/front_ms 136, render/thread/ms 48.69, render/busy/ms 47.25, main/busy/ms 39.24, stage/main/update 20.91, stage/render/render 19.63, render/graph/ms 17.56, stage/render/graph_submit 13.46, stage/main/post_update 13.12, encode//sum 9.00, stage/render/prepare_assets 8.50 |
| warm-up | 6.14 | 34.9 | pipelines/compile_ms 139, pipelines/front_ms 136, render/thread/ms 51.75, render/busy/ms 50.65, main/busy/ms 39.51, stage/main/update 19.80, stage/render/prepare_assets 16.03, stage/render/render 15.25, render/graph/ms 13.57, stage/main/post_update 11.58, stage/render/graph_submit 10.80, encode/finish_wall 7.54 |
| warm-up | 6.18 | 42.3 | render/thread/ms 66.70, render/busy/ms 65.41, main/busy/ms 43.15, stage/main/update 21.12, stage/render/prepare_assets 20.20, stage/render/render 16.15, render/graph/ms 14.14, stage/main/post_update 13.65, stage/render/graph_submit 10.57, stage/render/prepare_meshes 8.53, encode//sum 7.30, encode/finish_wall 6.97 |
| warm-up | 6.24 | 37.2 | render/thread/ms 45.51, render/busy/ms 44.54, main/busy/ms 33.27, stage/main/update 17.94, stage/render/render 15.71, stage/render/prepare_assets 15.58, render/graph/ms 14.29, stage/render/graph_submit 11.58, stage/main/post_update 9.41, encode/finish_wall 7.52, encode/pass:point_shadow_atlas_live_indirect/sum 7.30, encode//sum 6.88 |
| warm-up | 6.39 | 33.9 | main/busy/ms 47.53, render/thread/ms 38.03, render/busy/ms 36.41, stage/main/update 31.98, stage/render/render 16.58, render/graph/ms 14.17, stream/drive/ms 13.74, stage/main/post_update 10.38, stage/render/graph_submit 10.28, encode//sum 8.76, stage/render/prepare_assets 5.85, encode/finish_wall 5.58 |
| warm-up | 6.43 | 36.4 | render/thread/ms 53.19, render/busy/ms 51.57, main/busy/ms 50.22, stage/main/update 29.30, stage/render/render 22.00, render/graph/ms 19.43, stage/main/post_update 14.34, stage/render/graph_submit 14.03, stage/render/prepare_assets 12.87, encode//sum 11.16, encode/finish_wall 8.45, encode/pass:point_shadow_atlas_live_indirect/sum 8.10 |
| measure | 1.56 | 25.9 | render/thread/ms 29.52, render/busy/ms 28.01, main/busy/ms 26.16, stage/main/update 13.30, stage/render/prepare_assets 11.59, stage/render/render 10.89, render/graph/ms 8.99, stage/main/post_update 8.17, stage/render/graph_submit 6.41, encode//sum 6.15, encode/submit 4.68, stage/render/camera_driver 2.38 |
| measure | 18.33 | 31.0 | main/busy/ms 38.07, stage/main/update 25.36, render/thread/ms 21.80, render/busy/ms 20.83, stage/render/render 10.78, render/graph/ms 9.47, stage/main/post_update 8.39, stage/render/graph_submit 6.51, encode//sum 6.30, encode/submit 4.08, stage/render/prepare_assets 4.00, stage/render/camera_driver 2.82 |
| measure | 59.08 | 34.2 | render/thread/ms 32.94, render/busy/ms 31.56, main/busy/ms 24.57, stage/render/render 14.21, stage/main/post_update 12.76, render/graph/ms 12.49, stage/render/prepare_assets 10.54, encode//sum 10.32, stage/render/graph_submit 8.87, encode/submit 5.91, stage/main/update 5.25, stage/render/camera_driver 3.43 |
| measure | 59.11 | 35.9 | render/thread/ms 38.42, render/busy/ms 37.42, main/busy/ms 21.01, stage/render/prepare_assets 20.87, stage/render/render 10.54, render/graph/ms 9.16, stage/main/post_update 9.04, encode//sum 8.15, stage/render/graph_submit 6.65, stage/main/update 4.53, encode/submit 4.17, stage/main/fixed_main_loop 3.87 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 3% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **85%** (min 61%) · 2622 MHz (min 2610) · memory 8101 MHz · 84.8 W · 79 °C (max 85) · VRAM 5640 MiB · throttled 100% of samples

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
