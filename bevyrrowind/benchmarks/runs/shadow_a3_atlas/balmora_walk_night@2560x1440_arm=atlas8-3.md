# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=8 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6617 frames over 65.7 s · **median 9.82 ms (102 fps)** · mean 9.92 · p95 11.51 · p99 13.52 (1% low 74 fps) · worst 37.19 ms · 39 spikes over 1.5× median · 36.6 stutters/min (work 28.3/min)
- warm-up: 354 frames over 9.3 s, worst 2561.8 ms
- hitches (> 25.0 ms): **4** measured, 45 in warm-up
- timed GPU passes: 6.40 ms of a 9.92 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.82 ms, limit 11 ms
- **FAIL** frame_ms: 40 stutters (> 1.5x moving median of 60) in 1.09 min = 36.55/min, limit 1/min
- **FAIL** worst frame 37.19 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.960 | 2.191 | 2.607 | 0.89 | 0.176 |
| main_opaque_pass_3d | 1.234 | 1.379 | 1.926 | 0.89 | 0.065 |
| early prepass | 0.478 | 0.534 | 0.788 | 0.89 | 0.091 |
| bloom | 0.306 | 0.342 | 0.572 | 0.89 | 0.065 |
| atmosphere_luts | 0.306 | 0.342 | 0.642 | 0.89 | 0.003 |
| water_velocity | 0.216 | 0.241 | 0.364 | 0.89 | 0.002 |
| water_sspr | 0.195 | 0.218 | 0.220 | 0.89 | 0.002 |
| ssao | 0.176 | 0.196 | 0.204 | 0.89 | 0.007 |
| upscaling | 0.126 | 0.141 | 0.169 | 0.89 | 0.004 |
| atmos_froxel_inject | 0.123 | 0.137 | 0.131 | 0.89 | 0.002 |
| uniform_allocation | 0.120 | 0.134 | 0.144 | 0.89 | 0.022 |
| water_fft | 0.115 | 0.128 | 0.127 | 0.89 | 0.003 |
| point_shadow_atlas | 0.110 | 0.208 | 0.326 | 0.53 | 0.139 |
| clouds_composite | 0.097 | 0.109 | 0.109 | 0.89 | 0.002 |
| bin_unpacking | 0.091 | 0.101 | 0.116 | 0.89 | 0.017 |
| gpu_instance_materialization | 0.086 | 0.096 | 0.100 | 0.89 | 0.004 |
| tonemapping | 0.082 | 0.092 | 0.093 | 0.89 | 0.004 |
| main_transmissive_pass_3d | 0.067 | 0.075 | 0.140 | 0.89 | 0.008 |
| clustering | 0.060 | 0.067 | 0.068 | 0.89 | 0.020 |
| early_mesh_preprocessing | 0.054 | 0.061 | 0.073 | 0.89 | 0.004 |
| clouds_trace | 0.050 | 0.055 | 0.078 | 0.89 | 0.001 |
| render_sky | 0.050 | 0.055 | 0.054 | 0.89 | 0.002 |
| clouds_resolve | 0.045 | 0.050 | 0.051 | 0.89 | 0.001 |
| surface_trail | 0.034 | 0.058 | 0.057 | 0.59 | 0.002 |
| lightprobe_irradiance_map | 0.034 | 0.468 | 0.769 | 0.07 | 0.001 |
| atmos_froxel_integrate | 0.028 | 0.032 | 0.033 | 0.89 | 0.001 |
| water_snapshot | 0.023 | 0.026 | 0.035 | 0.89 | 0.002 |
| range_unpacking | 0.021 | 0.024 | 0.028 | 0.89 | 0.003 |
| water_mips | 0.021 | 0.023 | 0.023 | 0.89 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.019 | 0.027 | 0.89 | 0.005 |
| lightprobe_radiance_map | 0.016 | 0.220 | 0.230 | 0.07 | 0.004 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.89 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.016 | 0.029 | 0.89 | 0.190 |
| point_shadow_atlas/early_mesh_preprocessing | 0.014 | 0.027 | 0.044 | 0.53 | 0.005 |
| point_shadow_atlas/uniform_allocation | 0.012 | 0.024 | 0.048 | 0.53 | 0.006 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.89 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.89 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.009 | 0.017 | 0.034 | 0.53 | 0.004 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.89 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.53 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.89 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.89 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.730 | 9.998 | 36.10 | 57768.6 |
| render/busy/ms | 8.168 | 9.438 | 19.18 | 54048.5 |
| main/busy/ms | 8.081 | 9.190 | 30.89 | 53474.6 |
| stage/render/render | 5.092 | 6.069 | 31.84 | 33696.2 |
| stage/main/post_update | 4.562 | 5.303 | 9.88 | 30188.5 |
| render/graph/ms | 4.378 | 5.187 | 10.54 | 28969.0 |
| encode//sum | 3.376 | 4.145 | 12.12 | 22339.5 |
| stage/render/graph_submit | 3.101 | 3.858 | 7.53 | 20517.3 |
| encode/submit | 1.982 | 2.837 | 5.16 | 13114.5 |
| stage/main/update | 1.860 | 2.355 | 24.78 | 12308.6 |
| encode/finish_wall | 1.223 | 1.562 | 4.57 | 8091.8 |
| stage/render/camera_driver | 1.214 | 1.427 | 2.91 | 8031.0 |
| encode/pass:early prepass/sum | 0.830 | 1.085 | 2.16 | 5491.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.826 | 1.096 | 1.95 | 5467.5 |
| encode/pass:shadow_directional_light/sum | 0.824 | 1.244 | 2.59 | 5451.7 |
| stage/render/prepare_write_phase_buffers | 0.593 | 0.895 | 4.79 | 3922.0 |
| bigspace/total/ms | 0.555 | 1.319 | 2.52 | 3672.4 |
| stage/render/prepare_assets | 0.522 | 0.648 | 10.96 | 3454.5 |
| stage/main/fixed_main_loop | 0.490 | 0.888 | 3.86 | 3242.0 |
| render/present/ms | 0.471 | 1.266 | 25.23 | 3115.9 |
| stage/render/prepare_resources | 0.427 | 0.526 | 1.40 | 2824.7 |
| stage/render/prepare_meshes | 0.424 | 0.540 | 3.56 | 2805.0 |
| stage/main/pre_update | 0.375 | 0.505 | 2.87 | 2482.3 |
| stage/main/first | 0.352 | 0.451 | 6.15 | 2329.3 |
| controller/feed/ms | 0.336 | 0.438 | 0.79 | 2226.5 |
| stage/render/prepare_flush | 0.300 | 0.523 | 4.00 | 1985.0 |
| stage/render/prepare_bind_groups | 0.299 | 0.483 | 1.69 | 1981.1 |
| stage/render/prepare_batch_phases | 0.276 | 0.423 | 0.81 | 1828.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.268 | 0.426 | 0.78 | 1771.9 |
| stage/main/last | 0.261 | 0.313 | 1.60 | 1726.1 |
| bigspace/low_precision/ms | 0.237 | 0.729 | 1.93 | 1569.2 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.208 | 0.811 | 1.61 | 1378.2 |
| stage/render/create_views | 0.208 | 0.285 | 0.61 | 1374.0 |
| stream/drive/ms | 0.206 | 0.296 | 10.30 | 1364.2 |
| stage/render/extract_commands | 0.187 | 0.223 | 0.55 | 1234.5 |
| anim/evaluate/ms | 0.169 | 0.299 | 1.83 | 1118.4 |
| bigspace/high_precision/ms | 0.167 | 0.693 | 2.11 | 1101.9 |
| anim/evaluate_cpu/ms | 0.160 | 0.201 | 0.48 | 1056.8 |
| stream/tick/ms | 0.152 | 0.226 | 0.52 | 1006.5 |
| stage/render/specialize | 0.152 | 0.241 | 1.81 | 1005.0 |
| stage/render/prepare_views | 0.135 | 0.183 | 0.58 | 892.0 |
| pipelines/compile_ms | 0.131 | 0.000 | 133.16 | 867.9 |
| pipelines/front_ms | 0.127 | 0.000 | 129.70 | 842.7 |
| stage/render/queue | 0.096 | 0.164 | 1.38 | 632.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.092 | 0.151 | 0.60 | 607.9 |
| render/acquire/ms | 0.091 | 0.125 | 0.47 | 604.3 |
| stage/render/phase_sort | 0.087 | 0.120 | 0.36 | 575.5 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.078 | 0.378 | 1.36 | 519.2 |
| controller/write/ms | 0.075 | 0.099 | 0.34 | 495.9 |
| controller/move/ms | 0.064 | 0.205 | 0.61 | 426.2 |
| terrain/update/ms | 0.058 | 0.287 | 1.94 | 384.5 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.073 | 0.39 | 303.8 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.037 | 0.000 | 2.53 | 247.8 |
| physics/step/ms | 0.037 | 0.077 | 0.66 | 245.4 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.39 | 210.5 |
| physics/insert/ms | 0.026 | 0.270 | 0.97 | 170.4 |
| stage/render/cleanup | 0.024 | 0.036 | 0.31 | 157.9 |
| terrain/select/ms | 0.021 | 0.255 | 0.58 | 142.1 |
| weather/map/ms | 0.017 | 0.023 | 0.40 | 113.9 |
| far/select/ms | 0.015 | 0.168 | 0.56 | 99.3 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.021 | 0.19 | 81.2 |
| anim/apply/ms | 0.012 | 0.022 | 0.21 | 78.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.05 | 70.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.40 | 69.9 |
| script/update/ms | 0.008 | 0.012 | 0.31 | 53.5 |
| controller/avoid/ms | 0.006 | 0.012 | 0.18 | 40.6 |
| stream/despawn/ms | 0.006 | 0.000 | 1.16 | 36.4 |
| script/input/ms | 0.005 | 0.006 | 0.13 | 32.9 |
| pipelines/driver_ms | 0.004 | 0.000 | 3.55 | 25.1 |
| script/ui/ms | 0.004 | 0.005 | 0.30 | 24.5 |
| stream/spawn/ms | 0.003 | 0.000 | 1.17 | 22.1 |
| stream/upload/ms | 0.003 | 0.000 | 1.33 | 19.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.69 | 14.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.11 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 34 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 535 |
| crowd/c0/meshes/count | count | 355 | 646 | 144 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5539 | 5995 | 5661 |
| crowd/c1/meshes/count | count | 1168 | 1297 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 204 |
| crowd/joints_extracted/count | count | 666 | 6934 | 731 |
| crowd/joints_uploaded/count | count | 3604 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.41 | 4.53 | 0.57 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.19 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.40 | 0.01 |
| encode/finish_critical | ms | 0.97 | 4.53 | 0.90 |
| encode/pass:early prepass/max | ms | 0.83 | 2.16 | 0.90 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.84 | 1.95 | 0.66 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.60 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.78 | 0.44 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.65 | 1.61 | 0.35 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.20 | 1.36 | 0.90 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 0.98 | 2.53 | 0.99 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.59 | 0.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.39 | 0.02 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.49 | 30.89 | 11.18 |
| gpu/timed/ms | ms | 8.03 | 9.86 | 8.38 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 318 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2487 | 3515 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2487 | 3515 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 28 | 247 | 8 |
| phases/shadow/bins | count | 769 | 1106 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2218 | 2744 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 30 | 90 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2708 | 3262 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 275 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 14 | 274 | 31 |
| preprocess/bind_groups_created | count | 9 | 263 | 26 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 131 | 209 | 103 |
| process/cpu_usage | % | 24.66 | 37.02 | 29.33 |
| process/mem_usage | GiB | 5.12 | 5.37 | 5.37 |
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
| shadow_atlas/empty_faces | count | 0 | 1 | 0 |
| shadow_atlas/lamps | count | 8 | 9 | 0 |
| shadow_atlas/live_occ_pct | % | 29.30 | 83.98 | 0 |
| shadow_atlas/live_tiles | count | 12 | 37 | 0 |
| shadow_atlas/mover_draws | count | 8 | 12 | 0 |
| shadow_atlas/movers | count | 178 | 410 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 2 | 0 |
| shadow_atlas/render/blits | count | 8 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 16 | 0 |
| shadow_atlas/render/lamps | count | 8 | 9 | 0 |
| shadow_atlas/render/live_direct | count | 8 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 10 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 14 | 39 | 0 |
| shadow_atlas/resizes | count | 28 | 38 | 38 |
| shadow_atlas/static_occ_pct | % | 14.26 | 33.01 | 0 |
| shadow_atlas/static_tiles | count | 41 | 54 | 0 |
| shadow_atlas/views | count | 14 | 39 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 121 | 14 |
| skin/blocks_skipped/count | count | 146 | 207 | 189 |
| skin/blocks_walked/count | count | 14 | 26 | 15 |
| skin/joint_bytes_written/bytes | bytes | 47.1 k | 809.1 k | 44.8 k |
| skin/joints_written/count | count | 736 | 12.6 k | 700 |
| skin/upload_writes/count | count | 9 | 28 | 12 |
| started_load_count |  | 6439 | 6564 | 6564 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.74 | 46.71 | 31.39 |
| system/mem_usage | % | 55.30 | 56.12 | 56.09 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 85 | 304 | 66 |
| views/all/bins | count | 2907 | 3256 | 2948 |
| views/all/count | count | 17 | 42 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 7988 | 10.0 k | 7956 |
| views/all/pipelines | count | 76 | 139 | 66 |
| views/all/pipelines_distinct | count | 63 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 230.0 k | 154.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 8 |
| views/cascade/bins | count | 725 | 858 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2127 | 2396 | 2014 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 6 | 234 | 0 |
| views/lamp_face/bins | count | 12 | 340 | 0 |
| views/lamp_face/count | count | 14 | 39 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 55 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 20 | 479 | 0 |
| views/lamp_face/pipelines | count | 4 | 66 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 14 | 25.0 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2012 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7450 | 5942 |
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
| world/meshes | count | 21.3 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 49.16 | 207 | 325.3 k |
| anim/events/count | count | 0.08 | 3 | 560 |
| assets/Image/added | count | 0.09 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 12 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 310 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.13 | 10 | 7478 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1136 | 1489 |
| encode//count | count | 27.91 | 30 | 184.7 k |
| encode/bind_walk/init_textures | count | 8742 | 21.5 k | 57.85 M |
| encode/bind_walk/init_textures_skipped | count | 50.4 k | 70.7 k | 333.37 M |
| encode/bind_walk/inits | count | 555 | 773 | 3.67 M |
| encode/bind_walk/inits_skipped | count | 146 | 235 | 964.1 k |
| encode/bind_walk/merge_textures | count | 8679 | 21.4 k | 57.43 M |
| encode/bind_walk/merge_textures_skipped | count | 8870 | 18.7 k | 58.69 M |
| encode/bind_walk/merges | count | 103 | 175 | 678.7 k |
| encode/bind_walk/merges_skipped | count | 133 | 227 | 879.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6617 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6617 |
| encode/pass:early prepass/count | count | 1 | 1 | 6617 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6617 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6617 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6617 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.31 | 1 | 2027 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.23 | 1 | 1523 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 258 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6617 |
| encode/queue/command_buffers | count | 38.50 | 42 | 254.7 k |
| encode/queue/submits | count | 2.01 | 4 | 13.3 k |
| encode/queue/write_buffer/bytes | bytes | 2.31 M | 3.46 M | 15.26 G |
| encode/queue/write_buffer/calls | count | 266 | 435 | 1.76 M |
| encode/queue/write_buffer_with/bytes | bytes | 127.1 k | 1.25 M | 841.13 M |
| encode/queue/write_buffer_with/calls | count | 10.29 | 939 | 68.1 k |
| encode/queue/write_texture/bytes | bytes | 2083 | 1.05 M | 13.78 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 558 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6617 |
| physics/inserts | count | 0.34 | 109 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 176 | 2335 |
| physics/steps | count | 0.60 | 3 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 9 |
| pipelines/created | count | 0.00 | 1 | 10 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 36 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5116 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1122 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2561.8 | main/busy/ms 2529, stage/main/post_update 2486, stage/main/update 31.13, stage/main/pre_update 8.44, stage/main/last 1.11, surface/advance_cpu_ms 0.72, stream/drive/ms 0.54, stage/main/first 0.54, stage/main/fixed_main_loop 0.41, surface/cell_upload_ms 0.30, stream/tick/ms 0.15, bigspace/total/ms 0.07 |
| warm-up | 3.40 | 838.3 | main/busy/ms 2687, stage/main/post_update 2515, render/thread/ms 799, render/busy/ms 628, stage/render/prepare_assets 575, stage/render/prepare_views 169, render/acquire/ms 169, stage/main/update 152, stage/render/create_views 30.74, stage/render/render 12.56, stage/main/pre_update 10.27, render/graph/ms 8.83 |
| warm-up | 3.46 | 62.8 | render/thread/ms 845, render/busy/ms 673, stage/render/prepare_assets 605, main/busy/ms 182, stage/render/prepare_views 169, render/acquire/ms 169, stage/main/update 128, stage/main/post_update 40.74, stage/render/create_views 31.16, pipelines/compile_ms 24.79, pipelines/front_ms 22.84, terrain/update/ms 22.16 |
| warm-up | 3.51 | 48.4 | main/busy/ms 77.44, render/thread/ms 75.79, render/busy/ms 66.48, stage/main/post_update 58.44, pipelines/compile_ms 43.64, pipelines/front_ms 38.74, stage/render/prepare_assets 31.53, pipelines/wait_ms 30.43, stage/render/render 28.82, terrain/update/ms 28.01, render/graph/ms 12.86, stage/main/update 10.68 |
| warm-up | 3.62 | 109.8 | pipelines/wait_ms 3017, pipelines/compile_ms 1148, pipelines/front_ms 1030, render/thread/ms 130, render/busy/ms 121, pipelines/driver_ms 117, main/busy/ms 116, stage/main/post_update 84.26, stage/render/prepare_resources 37.08, stage/render/prepare_flush 34.68, stage/render/render 29.05, stage/render/prepare_assets 19.10 |
| warm-up | 3.69 | 71.9 | pipelines/wait_ms 7410, pipelines/compile_ms 1711, pipelines/front_ms 1555, pipelines/driver_ms 155, render/thread/ms 148, render/busy/ms 145, main/busy/ms 137, stage/main/update 63.05, stage/render/prepare_resources 50.64, stage/main/post_update 50.23, stage/render/prepare_flush 34.67, stage/render/prepare_assets 30.64 |
| warm-up | 3.81 | 113.1 | pipelines/wait_ms 4810, pipelines/compile_ms 2385, pipelines/front_ms 2248, pipelines/driver_ms 137, render/thread/ms 130, render/busy/ms 127, main/busy/ms 104, stage/render/prepare_assets 74.30, stage/main/update 66.95, stage/main/post_update 29.29, stage/render/render 28.97, render/graph/ms 26.56 |
| warm-up | 3.87 | 38.9 | pipelines/compile_ms 185, pipelines/front_ms 175, render/thread/ms 57.59, render/busy/ms 56.09, main/busy/ms 52.86, stage/render/prepare_assets 35.18, stage/main/post_update 24.45, pipelines/wait_ms 20.04, stage/main/update 15.58, stage/render/render 10.72, pipelines/driver_ms 10.49, terrain/update/ms 9.80 |
| warm-up | 3.90 | 30.6 | pipelines/compile_ms 354, pipelines/front_ms 328, render/thread/ms 64.31, render/busy/ms 62.28, main/busy/ms 52.44, stage/render/prepare_assets 26.61, pipelines/driver_ms 25.62, stage/main/post_update 24.93, pipelines/wait_ms 18.45, encode//sum 18.44, stage/main/update 15.30, stage/render/render 12.17 |
| warm-up | 3.95 | 47.3 | pipelines/compile_ms 890, pipelines/front_ms 837, pipelines/wait_ms 89.25, main/busy/ms 78.38, render/thread/ms 75.46, render/busy/ms 73.30, stage/main/update 53.06, pipelines/driver_ms 52.86, stage/render/prepare_resources 24.41, encode//sum 23.53, stage/main/post_update 19.56, stage/render/render 18.07 |
| warm-up | 4.01 | 59.8 | pipelines/compile_ms 1566, pipelines/front_ms 1501, pipelines/wait_ms 93.60, render/thread/ms 79.63, render/busy/ms 77.86, main/busy/ms 69.04, pipelines/driver_ms 65.73, stage/main/update 54.28, stage/render/prepare_assets 26.41, stage/render/prepare_resources 19.47, stage/render/render 17.07, render/graph/ms 14.72 |
| warm-up | 4.08 | 51.8 | pipelines/compile_ms 968, pipelines/front_ms 916, render/thread/ms 56.26, render/busy/ms 54.59, pipelines/driver_ms 52.08, main/busy/ms 36.00, stage/render/prepare_assets 31.61, stage/main/update 21.50, stage/render/render 16.08, render/graph/ms 14.08, pipelines/wait_ms 13.23, stage/render/graph_submit 11.06 |
| warm-up | 4.11 | 30.8 | pipelines/compile_ms 839, pipelines/front_ms 802, render/thread/ms 55.72, render/busy/ms 54.45, pipelines/driver_ms 37.28, stage/render/prepare_assets 37.21, main/busy/ms 23.46, pipelines/wait_ms 15.00, stage/render/render 12.41, stage/main/update 11.16, render/graph/ms 10.86, stage/render/graph_submit 8.81 |
| warm-up | 4.23 | 48.4 | pipelines/compile_ms 407, pipelines/front_ms 394, render/thread/ms 55.44, render/busy/ms 53.66, main/busy/ms 30.44, stage/render/prepare_assets 25.61, stage/render/render 20.25, render/graph/ms 17.74, pipelines/driver_ms 13.46, stage/main/post_update 12.97, stage/main/update 11.59, stage/render/graph_submit 9.70 |
| warm-up | 4.26 | 28.5 | pipelines/compile_ms 474, pipelines/front_ms 457, render/thread/ms 60.45, render/busy/ms 58.33, main/busy/ms 42.06, stage/render/prepare_assets 24.00, stage/render/render 22.12, render/graph/ms 19.47, stage/main/post_update 19.30, pipelines/driver_ms 17.02, stage/main/update 16.14, stage/render/graph_submit 10.97 |
| warm-up | 4.32 | 58.2 | pipelines/compile_ms 1311, pipelines/front_ms 1269, main/busy/ms 141, stage/main/update 119, render/thread/ms 55.84, render/busy/ms 54.24, pipelines/driver_ms 41.20, stage/render/prepare_assets 30.78, stage/main/post_update 13.33, stage/render/render 12.97, render/graph/ms 11.13, stage/render/graph_submit 8.59 |
| warm-up | 4.47 | 157.7 | pipelines/compile_ms 1758, pipelines/front_ms 1709, main/busy/ms 155, stage/main/update 139, render/thread/ms 82.39, render/busy/ms 80.72, stage/render/prepare_assets 58.74, pipelines/driver_ms 48.76, stage/render/render 15.48, render/graph/ms 13.60, pipelines/wait_ms 9.35, stage/render/graph_submit 9.32 |
| warm-up | 4.53 | 58.2 | pipelines/compile_ms 1923, pipelines/front_ms 1853, render/thread/ms 94.71, render/busy/ms 92.83, pipelines/driver_ms 70.41, main/busy/ms 69.81, pipelines/wait_ms 61.99, stage/render/prepare_assets 57.33, stage/main/update 49.03, stage/render/render 16.47, render/graph/ms 14.02, stream/drive/ms 13.00 |
| warm-up | 4.57 | 38.1 | pipelines/compile_ms 1507, pipelines/front_ms 1442, pipelines/wait_ms 102, render/thread/ms 78.06, render/busy/ms 76.23, pipelines/driver_ms 64.97, main/busy/ms 52.45, stage/render/prepare_assets 39.54, stage/main/update 25.15, pipelines/lock_ms 17.27, stage/main/post_update 16.38, stage/render/render 13.97 |
| warm-up | 4.60 | 32.8 | pipelines/compile_ms 125, pipelines/front_ms 118, pipelines/wait_ms 73.33, render/thread/ms 61.54, render/busy/ms 59.42, main/busy/ms 55.83, stage/main/update 27.51, stage/render/prepare_assets 21.96, stream/drive/ms 17.70, stage/main/post_update 16.56, stage/render/render 15.29, pipelines/lock_ms 13.62 |
| warm-up | 4.71 | 104.9 | pipelines/compile_ms 1198, pipelines/front_ms 1167, render/thread/ms 120, render/busy/ms 99.05, main/busy/ms 73.22, pipelines/wait_ms 57.38, stage/main/update 38.35, stage/render/prepare_resources 33.81, pipelines/driver_ms 31.24, stage/render/prepare_assets 23.60, stage/render/prepare_views 19.63, render/acquire/ms 19.50 |
| warm-up | 4.78 | 72.4 | pipelines/compile_ms 2315, pipelines/front_ms 2250, pipelines/wait_ms 460, render/thread/ms 155, render/busy/ms 134, main/busy/ms 91.61, pipelines/driver_ms 65.00, stage/main/update 51.29, stage/render/prepare_resources 45.99, stage/render/prepare_assets 33.55, stage/render/prepare_views 19.65, stage/main/post_update 19.50 |
| warm-up | 4.89 | 109.6 | pipelines/compile_ms 2360, pipelines/front_ms 2294, pipelines/wait_ms 712, render/thread/ms 130, render/busy/ms 128, main/busy/ms 100, pipelines/driver_ms 65.90, stage/main/update 63.92, stage/render/prepare_assets 58.87, stage/render/render 27.90, render/graph/ms 25.28, stage/render/prepare_resources 19.88 |
| warm-up | 4.97 | 84.3 | pipelines/compile_ms 1921, pipelines/front_ms 1872, pipelines/wait_ms 594, render/thread/ms 119, render/busy/ms 117, stage/render/prepare_assets 73.89, main/busy/ms 69.20, pipelines/driver_ms 48.36, stage/main/update 40.35, stage/render/render 24.23, render/graph/ms 21.88, stage/render/graph_submit 13.12 |
| warm-up | 5.04 | 70.1 | pipelines/compile_ms 808, pipelines/front_ms 785, pipelines/wait_ms 341, render/thread/ms 115, render/busy/ms 114, main/busy/ms 75.33, stage/render/prepare_assets 60.22, stage/main/update 47.75, pipelines/driver_ms 23.16, stage/render/render 17.64, stage/main/post_update 15.79, render/graph/ms 15.11 |
| warm-up | 5.09 | 42.5 | pipelines/compile_ms 1717, pipelines/front_ms 1673, main/busy/ms 115, render/thread/ms 92.19, stage/main/update 91.04, render/busy/ms 90.69, stream/drive/ms 58.25, pipelines/driver_ms 43.64, stage/render/prepare_assets 42.94, pipelines/wait_ms 34.48, stage/render/render 17.95, render/graph/ms 15.80 |
| warm-up | 5.15 | 63.8 | pipelines/compile_ms 1666, pipelines/front_ms 1625, main/busy/ms 88.91, stage/main/update 69.02, render/thread/ms 57.48, render/busy/ms 55.15, stream/drive/ms 50.80, pipelines/driver_ms 40.88, stage/render/prepare_assets 18.26, stage/render/render 15.34, render/graph/ms 13.22, stage/main/post_update 12.11 |
| warm-up | 5.18 | 33.5 | pipelines/compile_ms 478, pipelines/front_ms 461, render/thread/ms 52.50, render/busy/ms 50.21, main/busy/ms 46.05, stage/main/update 26.32, pipelines/driver_ms 16.80, stage/render/prepare_assets 16.02, stage/render/render 13.27, stage/main/post_update 11.64, render/graph/ms 11.18, encode//sum 7.47 |
| warm-up | 5.21 | 29.7 | pipelines/compile_ms 615, pipelines/front_ms 596, render/thread/ms 49.57, render/busy/ms 47.84, main/busy/ms 36.92, stage/main/update 21.26, pipelines/driver_ms 19.71, stage/render/prepare_assets 16.30, stage/render/render 14.36, render/graph/ms 12.03, encode//sum 9.25, stage/render/graph_submit 8.91 |
| warm-up | 5.38 | 31.2 | pipelines/compile_ms 458, pipelines/front_ms 435, render/thread/ms 48.89, render/busy/ms 47.29, main/busy/ms 36.60, pipelines/driver_ms 23.13, stage/main/update 16.41, stage/render/render 12.66, render/graph/ms 10.55, stage/render/prepare_assets 9.90, stage/main/post_update 9.83, stage/render/graph_submit 7.32 |
| warm-up | 5.41 | 31.0 | pipelines/compile_ms 275, pipelines/front_ms 258, render/thread/ms 56.66, render/busy/ms 55.45, main/busy/ms 46.05, stage/main/update 24.06, pipelines/driver_ms 17.33, stream/drive/ms 11.82, stage/render/render 11.12, stage/main/post_update 10.39, stage/render/prepare_assets 9.78, stage/render/prepare_resources 9.38 |
| warm-up | 5.44 | 29.9 | render/thread/ms 52.34, render/busy/ms 50.92, main/busy/ms 42.22, stage/main/update 23.70, stage/render/render 12.22, stream/drive/ms 11.26, stage/main/post_update 11.21, render/graph/ms 10.45, stage/render/prepare_resources 10.32, stage/render/prepare_assets 8.61, stage/render/graph_submit 7.04, encode//sum 6.61 |
| warm-up | 5.48 | 38.1 | pipelines/compile_ms 794, pipelines/front_ms 773, render/thread/ms 59.49, render/busy/ms 57.63, main/busy/ms 53.17, stage/main/post_update 29.54, pipelines/driver_ms 20.40, stage/render/render 19.33, render/graph/ms 16.71, stage/main/update 16.47, stage/render/graph_submit 12.27, stage/render/prepare_assets 10.49 |
| warm-up | 5.51 | 28.5 | pipelines/compile_ms 1412, pipelines/front_ms 1372, render/thread/ms 53.57, render/busy/ms 51.99, main/busy/ms 47.74, pipelines/driver_ms 40.88, stage/main/post_update 26.91, stage/render/render 18.62, render/graph/ms 16.17, stage/main/update 12.79, stage/render/prepare_assets 12.62, stage/render/graph_submit 11.91 |
| warm-up | 5.54 | 31.5 | pipelines/compile_ms 1189, pipelines/front_ms 1154, render/thread/ms 37.38, render/busy/ms 36.44, pipelines/driver_ms 35.38, main/busy/ms 26.93, stage/render/prepare_assets 21.81, stage/main/update 9.60, stage/main/post_update 9.50, stage/render/render 8.83, render/graph/ms 7.47, encode//sum 6.92 |
| warm-up | 5.71 | 28.6 | main/busy/ms 32.90, render/thread/ms 23.05, render/busy/ms 22.12, stage/main/update 20.49, stream/drive/ms 13.56, stage/render/render 9.11, stage/main/post_update 8.70, render/graph/ms 7.73, encode//sum 6.76, stage/render/graph_submit 5.48, stage/render/prepare_assets 4.22, encode/submit 3.29 |
| warm-up | 5.93 | 25.8 | main/busy/ms 32.73, render/thread/ms 20.09, stage/main/update 19.99, render/busy/ms 19.24, stream/drive/ms 12.76, stage/render/render 8.72, render/graph/ms 7.51, stage/main/post_update 7.12, encode//sum 6.36, stage/render/graph_submit 4.81, stage/render/prepare_assets 4.38, encode/submit 2.78 |
| warm-up | 6.26 | 31.2 | render/thread/ms 42.62, main/busy/ms 41.57, render/busy/ms 41.21, stage/main/update 21.06, stage/render/render 17.25, render/graph/ms 15.14, stage/main/post_update 13.72, stage/render/graph_submit 10.89, encode//sum 9.43, stage/render/prepare_assets 9.39, encode/submit 7.41, stage/render/prepare_meshes 4.87 |
| warm-up | 6.29 | 31.4 | render/thread/ms 45.63, render/busy/ms 44.57, main/busy/ms 36.91, stage/main/update 19.11, stage/render/prepare_assets 16.85, stage/main/post_update 9.85, stage/render/render 9.73, render/graph/ms 8.19, encode//sum 7.95, stage/render/prepare_meshes 6.60, stage/render/graph_submit 4.75, stream/drive/ms 4.15 |
| warm-up | 6.32 | 34.3 | render/thread/ms 55.73, render/busy/ms 54.19, main/busy/ms 47.97, stage/main/update 24.77, stage/render/prepare_assets 17.50, stage/main/post_update 13.84, stage/render/render 13.63, render/graph/ms 11.38, encode//sum 10.09, stage/render/graph_submit 6.67, stage/render/prepare_meshes 6.57, stage/render/camera_driver 4.43 |
| warm-up | 6.35 | 28.4 | pipelines/compile_ms 151, pipelines/front_ms 147, render/thread/ms 48.89, render/busy/ms 47.34, main/busy/ms 42.60, stage/main/update 19.58, stage/main/post_update 14.30, stage/render/render 13.78, stage/render/prepare_assets 13.21, render/graph/ms 11.62, encode//sum 8.87, stage/render/graph_submit 7.51 |
| warm-up | 6.38 | 27.7 | pipelines/compile_ms 151, pipelines/front_ms 147, render/thread/ms 37.86, render/busy/ms 36.77, main/busy/ms 32.65, stage/main/update 15.83, stage/render/prepare_assets 15.52, stage/main/post_update 9.51, stage/render/render 9.49, render/graph/ms 7.97, encode//sum 6.78, stage/render/graph_submit 5.35 |
| warm-up | 6.52 | 38.3 | main/busy/ms 49.43, render/thread/ms 35.06, render/busy/ms 33.94, stage/main/update 31.32, stream/drive/ms 14.62, stage/render/render 13.61, stage/main/post_update 12.84, render/graph/ms 11.98, encode//sum 9.39, stage/render/graph_submit 7.75, stage/render/prepare_assets 6.28, stage/render/prepare_meshes 4.39 |
| warm-up | 6.55 | 29.2 | main/busy/ms 50.97, render/thread/ms 47.02, render/busy/ms 45.62, stage/main/update 29.57, stage/render/render 16.06, stage/main/post_update 14.30, render/graph/ms 13.91, stage/render/prepare_assets 13.49, encode//sum 9.64, stage/render/graph_submit 8.71, stage/render/prepare_meshes 5.95, stream/drive/ms 5.30 |
| warm-up | 6.58 | 28.8 | pipelines/compile_ms 142, pipelines/front_ms 136, main/busy/ms 43.23, render/thread/ms 41.02, render/busy/ms 39.77, stage/main/update 26.09, stage/render/prepare_assets 15.73, stage/render/render 12.12, stage/main/post_update 11.13, render/graph/ms 10.26, encode//sum 7.48, stage/render/graph_submit 6.57 |
| measure | 18.31 | 33.7 | main/busy/ms 39.29, stage/main/update 26.80, render/thread/ms 20.70, render/busy/ms 19.85, stage/render/render 10.05, stage/main/post_update 8.88, render/graph/ms 8.82, encode//sum 6.02, stage/render/graph_submit 5.78, stage/render/prepare_assets 4.09, encode/submit 3.49, stage/render/camera_driver 2.90 |
| measure | 35.48 | 37.2 | render/thread/ms 45.33, stage/render/render 37.05, render/present/ms 25.65, render/busy/ms 19.44, main/busy/ms 19.35, stage/main/post_update 11.25, render/graph/ms 10.81, encode//sum 8.58, stage/render/graph_submit 7.77, encode/submit 4.88, stage/main/update 4.49, encode/finish_wall 2.93 |
| measure | 59.12 | 26.6 | render/thread/ms 39.35, render/busy/ms 36.80, main/busy/ms 34.31, stage/render/render 19.18, stage/main/post_update 18.11, render/graph/ms 15.77, encode//sum 12.38, stage/render/graph_submit 10.00, stage/main/update 8.50, stage/render/prepare_assets 7.25, encode/submit 6.23, stage/render/camera_driver 5.57 |
| measure | 59.15 | 29.6 | render/thread/ms 39.88, render/busy/ms 37.79, main/busy/ms 27.47, stage/render/prepare_assets 16.65, stage/main/post_update 13.21, stage/render/render 12.13, render/graph/ms 9.45, encode//sum 8.12, stage/main/update 5.36, stage/render/graph_submit 5.28, stage/main/fixed_main_loop 4.97, stage/render/camera_driver 4.00 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=8 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=atlas8-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1512 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **87%** (min 69%) · 2625 MHz (min 2625) · memory 8101 MHz · 84.1 W · 75 °C (max 79) · VRAM 6474 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.enabled=on (command line: --set)`
- `shadow.lamp.atlas.max_lamps=8 (command line: --set)`
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
