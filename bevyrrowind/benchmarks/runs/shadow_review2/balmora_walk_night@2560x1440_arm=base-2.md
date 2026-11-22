# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6134 frames over 65.6 s · **median 10.55 ms (95 fps)** · mean 10.70 · p95 12.40 · p99 14.68 (1% low 68 fps) · worst 32.29 ms · 32 spikes over 1.5× median · 27.4 stutters/min (work 27.4/min)
- warm-up: 354 frames over 9.3 s, worst 2411.6 ms
- hitches (> 26.4 ms): **3** measured, 40 in warm-up
- timed GPU passes: 6.99 ms of a 10.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.55 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 32.29 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.996 | 2.034 | 2.067 | 0.98 | 0.183 |
| main_opaque_pass_3d | 1.614 | 1.645 | 2.365 | 0.98 | 0.084 |
| early prepass | 0.510 | 0.520 | 0.754 | 0.98 | 0.095 |
| bloom | 0.311 | 0.317 | 0.325 | 0.98 | 0.067 |
| atmosphere_luts | 0.307 | 0.313 | 0.377 | 0.98 | 0.004 |
| water_velocity | 0.221 | 0.225 | 0.246 | 0.98 | 0.002 |
| water_sspr | 0.198 | 0.202 | 0.203 | 0.98 | 0.003 |
| ssao | 0.184 | 0.188 | 0.191 | 0.98 | 0.008 |
| point_shadow_atlas | 0.152 | 0.267 | 0.629 | 0.57 | 0.249 |
| upscaling | 0.126 | 0.128 | 0.135 | 0.98 | 0.005 |
| atmos_froxel_inject | 0.126 | 0.128 | 0.129 | 0.98 | 0.002 |
| water_fft | 0.121 | 0.123 | 0.126 | 0.98 | 0.003 |
| uniform_allocation | 0.115 | 0.117 | 0.141 | 0.98 | 0.026 |
| clouds_composite | 0.103 | 0.105 | 0.109 | 0.98 | 0.002 |
| gpu_instance_materialization | 0.095 | 0.097 | 0.101 | 0.98 | 0.004 |
| bin_unpacking | 0.091 | 0.093 | 0.114 | 0.98 | 0.019 |
| tonemapping | 0.085 | 0.086 | 0.091 | 0.98 | 0.004 |
| main_transmissive_pass_3d | 0.069 | 0.070 | 0.133 | 0.98 | 0.010 |
| clustering | 0.063 | 0.064 | 0.068 | 0.98 | 0.021 |
| clouds_trace | 0.055 | 0.056 | 0.078 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.053 | 0.054 | 0.070 | 0.98 | 0.005 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.98 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.049 | 0.414 | 0.417 | 0.12 | 0.001 |
| surface_trail | 0.039 | 0.057 | 0.057 | 0.69 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.98 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.028 | 0.048 | 0.081 | 0.57 | 0.013 |
| lightprobe_radiance_map | 0.026 | 0.219 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.027 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.98 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.027 | 0.98 | 0.004 |
| point_shadow_atlas/uniform_allocation | 0.018 | 0.031 | 0.092 | 0.57 | 0.010 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.027 | 0.98 | 0.006 |
| night_sky | 0.016 | 0.017 | 0.042 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 0.98 | 0.195 |
| point_shadow_atlas/bin_unpacking | 0.013 | 0.022 | 0.059 | 0.57 | 0.006 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.004 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.57 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.98 | 0.004 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.98 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.534 | 11.044 | 21.20 | 58478.6 |
| render/busy/ms | 9.048 | 10.546 | 20.72 | 55499.0 |
| main/busy/ms | 8.645 | 10.086 | 28.90 | 53028.2 |
| stage/render/render | 5.520 | 6.405 | 14.37 | 33859.5 |
| stage/main/post_update | 4.907 | 5.835 | 10.35 | 30100.7 |
| render/graph/ms | 4.869 | 5.719 | 13.53 | 29868.8 |
| encode//sum | 3.753 | 4.794 | 11.68 | 23023.9 |
| stage/render/graph_submit | 3.450 | 4.140 | 11.95 | 21161.1 |
| encode/submit | 2.085 | 2.560 | 5.95 | 12788.3 |
| stage/main/update | 1.948 | 2.471 | 23.30 | 11947.1 |
| encode/finish_wall | 1.400 | 1.890 | 9.93 | 8588.3 |
| stage/render/camera_driver | 1.354 | 1.668 | 2.99 | 8304.8 |
| encode/pass:early prepass/sum | 0.906 | 1.226 | 2.20 | 5556.1 |
| encode/pass:shadow_directional_light/sum | 0.905 | 1.363 | 2.81 | 5548.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.897 | 1.232 | 1.89 | 5499.9 |
| stage/render/prepare_write_phase_buffers | 0.684 | 1.049 | 2.70 | 4196.8 |
| bigspace/total/ms | 0.583 | 1.418 | 2.74 | 3576.5 |
| stage/main/fixed_main_loop | 0.557 | 0.981 | 2.28 | 3418.4 |
| stage/render/prepare_assets | 0.548 | 0.708 | 12.23 | 3363.4 |
| stage/render/prepare_meshes | 0.472 | 0.599 | 4.70 | 2892.6 |
| stage/render/prepare_resources | 0.468 | 0.567 | 1.28 | 2868.8 |
| stage/main/pre_update | 0.404 | 0.555 | 2.10 | 2480.6 |
| render/present/ms | 0.391 | 0.480 | 1.86 | 2400.6 |
| stage/main/first | 0.376 | 0.486 | 5.10 | 2304.0 |
| controller/feed/ms | 0.354 | 0.452 | 0.97 | 2169.4 |
| stage/render/prepare_bind_groups | 0.343 | 0.573 | 2.12 | 2103.8 |
| stage/render/prepare_flush | 0.339 | 0.539 | 5.26 | 2078.0 |
| stage/render/prepare_batch_phases | 0.309 | 0.484 | 0.91 | 1893.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.306 | 0.490 | 1.31 | 1877.6 |
| stage/main/last | 0.263 | 0.318 | 0.78 | 1615.7 |
| bigspace/low_precision/ms | 0.258 | 0.868 | 1.83 | 1584.7 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.255 | 1.382 | 9.64 | 1565.2 |
| stream/drive/ms | 0.220 | 0.309 | 10.62 | 1351.5 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.215 | 1.130 | 1.84 | 1316.9 |
| stage/render/create_views | 0.211 | 0.288 | 0.58 | 1296.2 |
| stage/render/extract_commands | 0.202 | 0.239 | 0.53 | 1237.2 |
| stage/render/specialize | 0.176 | 0.279 | 1.80 | 1077.5 |
| anim/evaluate/ms | 0.168 | 0.322 | 2.12 | 1032.3 |
| stream/tick/ms | 0.163 | 0.235 | 0.45 | 997.3 |
| bigspace/high_precision/ms | 0.162 | 0.705 | 2.32 | 993.0 |
| anim/evaluate_cpu/ms | 0.159 | 0.207 | 0.75 | 977.7 |
| stage/render/prepare_views | 0.139 | 0.184 | 0.49 | 851.9 |
| stage/render/queue | 0.113 | 0.189 | 1.53 | 692.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.107 | 0.168 | 0.77 | 657.0 |
| render/acquire/ms | 0.094 | 0.128 | 0.44 | 579.0 |
| stage/render/phase_sort | 0.084 | 0.119 | 0.36 | 515.2 |
| controller/move/ms | 0.082 | 0.214 | 0.59 | 505.5 |
| controller/write/ms | 0.073 | 0.099 | 0.30 | 450.4 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.069 | 0.405 | 3.21 | 423.3 |
| terrain/update/ms | 0.062 | 0.295 | 1.80 | 382.9 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.077 | 0.38 | 296.0 |
| physics/step/ms | 0.043 | 0.085 | 0.63 | 261.7 |
| encode/resolve_diagnostics/sum | 0.036 | 0.050 | 0.64 | 220.7 |
| pipelines/compile_ms | 0.029 | 0.000 | 83.15 | 177.4 |
| pipelines/front_ms | 0.028 | 0.000 | 80.39 | 171.7 |
| physics/insert/ms | 0.027 | 0.326 | 0.97 | 165.0 |
| terrain/select/ms | 0.024 | 0.260 | 0.67 | 147.0 |
| stage/render/cleanup | 0.024 | 0.031 | 0.44 | 145.2 |
| weather/map/ms | 0.018 | 0.025 | 0.25 | 112.2 |
| far/select/ms | 0.016 | 0.171 | 0.45 | 96.1 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.023 | 0.55 | 88.4 |
| anim/apply/ms | 0.012 | 0.023 | 0.45 | 76.1 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.49 | 74.8 |
| weather/eval/ms | 0.011 | 0.014 | 0.07 | 67.1 |
| script/update/ms | 0.009 | 0.013 | 0.06 | 52.8 |
| controller/avoid/ms | 0.006 | 0.012 | 0.06 | 39.3 |
| stream/despawn/ms | 0.006 | 0.000 | 1.15 | 37.3 |
| script/input/ms | 0.006 | 0.007 | 0.04 | 34.4 |
| script/ui/ms | 0.004 | 0.005 | 0.08 | 22.2 |
| stream/spawn/ms | 0.004 | 0.000 | 1.15 | 22.0 |
| stream/upload/ms | 0.004 | 0.000 | 1.35 | 21.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.67 | 14.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.0 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.76 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.5 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 42 | 12 |
| crowd/c0/joints/count | count | 1301 | 2207 | 535 |
| crowd/c0/meshes/count | count | 354 | 619 | 144 |
| crowd/c1/count | count | 145 | 156 | 149 |
| crowd/c1/joints/count | count | 5497 | 5946 | 5661 |
| crowd/c1/meshes/count | count | 1166 | 1273 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6390 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 205 |
| crowd/joints_extracted/count | count | 691 | 5533 | 631 |
| crowd/joints_uploaded/count | count | 3591 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.49 | 1.79 | 0.47 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.55 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.49 | 0.02 |
| encode/finish_critical | ms | 1.10 | 9.64 | 0.92 |
| encode/pass:early prepass/max | ms | 0.89 | 2.20 | 0.86 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.89 | 1.89 | 0.92 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.77 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.29 | 1.31 | 0.43 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.98 | 9.64 | 1.05 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.93 | 1.84 | 0.22 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.34 | 3.21 | 1.12 |
| encode/pass:shadow_directional_light/max | ms | 0.87 | 2.81 | 0.72 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.64 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.07 | 28.90 | 8.69 |
| gpu/timed/ms | ms | 7.85 | 9.35 | 8.69 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 152 | 386 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 152 | 386 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 764 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2479 | 3527 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 764 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2479 | 3527 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 74 | 464 | 8 |
| phases/shadow/bins | count | 800 | 1728 | 671 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2310 | 3523 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 53 | 179 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2679 | 3183 | 2347 |
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
| preprocess/bind_groups | count | 15 | 318 | 11 |
| preprocess/bind_groups_created | count | 10 | 307 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 141 | 271 | 123 |
| process/cpu_usage | % | 25.48 | 35.58 | 22.60 |
| process/mem_usage | GiB | 5.17 | 5.34 | 5.34 |
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
| shadow_atlas/lamps | count | 8 | 21 | 0 |
| shadow_atlas/live_occ_pct | % | 2.73 | 35.16 | 0 |
| shadow_atlas/live_tiles | count | 13 | 66 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 157 | 552 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 24 | 0 |
| shadow_atlas/render/lamps | count | 8 | 21 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 18 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 15 | 61 | 0 |
| shadow_atlas/resizes | count | 31 | 40 | 40 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 112 | 0 |
| shadow_atlas/views | count | 15 | 61 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 94 | 12 |
| skin/blocks_skipped/count | count | 145 | 206 | 192 |
| skin/blocks_walked/count | count | 14 | 30 | 13 |
| skin/joint_bytes_written/bytes | bytes | 49.2 k | 810.2 k | 41.7 k |
| skin/joints_written/count | count | 768 | 12.7 k | 652 |
| skin/upload_writes/count | count | 7 | 27 | 8 |
| started_load_count |  | 6442 | 6566 | 6566 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.69 | 61.93 | 43.78 |
| system/mem_usage | % | 47.46 | 48.20 | 47.95 |
| terrain/patches | count | 511 | 520 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 134 | 517 | 66 |
| views/all/bins | count | 2931 | 3682 | 2961 |
| views/all/count | count | 18 | 64 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8028 | 10.6 k | 7969 |
| views/all/pipelines | count | 101 | 226 | 66 |
| views/all/pipelines_distinct | count | 67 | 76 | 61 |
| views/all/work_items | count | 154.1 k | 230.9 k | 154.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 8 |
| views/cascade/bins | count | 723 | 842 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2126 | 2421 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 60 | 431 | 0 |
| views/lamp_face/bins | count | 111 | 926 | 0 |
| views/lamp_face/count | count | 15 | 61 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 149 | 1111 | 0 |
| views/lamp_face/pipelines | count | 27 | 154 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 112 | 25.3 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2012 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5899 | 7492 | 5942 |
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
| anim/bones_written/count | count | 47.20 | 313 | 289.5 k |
| anim/events/count | count | 0.09 | 3 | 551 |
| assets/Image/added | count | 0.10 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.47 | 255 | 2874 |
| assets/Mesh/removed | count | 0.79 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.22 | 10 | 7477 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.24 | 1154 | 1489 |
| encode//count | count | 27.95 | 30 | 171.5 k |
| encode/bind_walk/init_textures | count | 9264 | 23.1 k | 56.82 M |
| encode/bind_walk/init_textures_skipped | count | 59.7 k | 91.4 k | 366.47 M |
| encode/bind_walk/inits | count | 571 | 815 | 3.51 M |
| encode/bind_walk/inits_skipped | count | 179 | 272 | 1.10 M |
| encode/bind_walk/merge_textures | count | 9201 | 23.0 k | 56.44 M |
| encode/bind_walk/merge_textures_skipped | count | 11.0 k | 31.6 k | 67.45 M |
| encode/bind_walk/merges | count | 107 | 213 | 656.2 k |
| encode/bind_walk/merges_skipped | count | 157 | 230 | 961.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6134 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6134 |
| encode/pass:early prepass/count | count | 1 | 1 | 6134 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6134 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6134 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6134 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.24 | 1 | 1454 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.29 | 1 | 1771 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 312 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6134 |
| encode/queue/command_buffers | count | 38.54 | 42 | 236.4 k |
| encode/queue/submits | count | 2.01 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.61 M | 14.30 G |
| encode/queue/write_buffer/calls | count | 281 | 527 | 1.72 M |
| encode/queue/write_buffer_with/bytes | bytes | 172.2 k | 1.31 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 10.39 | 774 | 63.7 k |
| encode/queue/write_texture/bytes | bytes | 2076 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6134 |
| physics/inserts | count | 0.37 | 109 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 175 | 2335 |
| physics/steps | count | 0.64 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 32 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5519 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1210 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.41 | 2411.6 | main/busy/ms 2383, stage/main/post_update 2340, stage/main/update 32.01, stage/main/pre_update 7.47, stage/main/last 1.23, surface/advance_cpu_ms 0.78, stage/main/first 0.64, stream/drive/ms 0.58, stage/main/fixed_main_loop 0.39, surface/cell_upload_ms 0.34, stream/tick/ms 0.12, bigspace/total/ms 0.09 |
| warm-up | 3.34 | 932.7 | main/busy/ms 2519, stage/main/post_update 2362, render/thread/ms 897, render/busy/ms 744, stage/render/prepare_assets 693, stage/render/prepare_views 149, render/acquire/ms 149, stage/main/update 140, stage/render/create_views 30.08, stage/render/render 13.30, stage/main/pre_update 9.21, render/graph/ms 8.97 |
| warm-up | 3.46 | 114.5 | render/thread/ms 993, render/busy/ms 779, stage/render/prepare_assets 714, main/busy/ms 155, stage/render/prepare_views 150, render/acquire/ms 150, stage/main/update 114, pipelines/compile_ms 92.37, stage/render/render 80.26, render/present/ms 63.65, pipelines/driver_ms 48.35, pipelines/front_ms 44.02 |
| warm-up | 3.62 | 142.7 | pipelines/wait_ms 3455, pipelines/compile_ms 1597, pipelines/front_ms 1476, pipelines/driver_ms 121, render/thread/ms 119, render/busy/ms 116, main/busy/ms 114, stage/main/post_update 78.79, encode//sum 40.32, stage/render/prepare_resources 35.65, bigspace/total/ms 32.14, stage/main/update 28.16 |
| warm-up | 3.65 | 37.7 | pipelines/wait_ms 3602, pipelines/compile_ms 1595, pipelines/front_ms 1502, main/busy/ms 155, render/thread/ms 140, render/busy/ms 138, pipelines/driver_ms 92.72, stage/main/update 74.47, stage/main/post_update 52.60, stage/render/prepare_assets 47.78, encode//sum 44.01, stage/render/prepare_resources 36.61 |
| warm-up | 3.77 | 112.1 | pipelines/compile_ms 1036, pipelines/front_ms 989, pipelines/wait_ms 225, main/busy/ms 105, render/thread/ms 88.94, render/busy/ms 87.11, stage/render/prepare_assets 66.15, stage/main/update 59.72, pipelines/driver_ms 46.89, stage/main/fixed_main_loop 22.76, stage/main/post_update 19.10, terrain/update/ms 14.24 |
| warm-up | 3.80 | 36.2 | pipelines/compile_ms 1410, pipelines/front_ms 1334, pipelines/wait_ms 204, render/thread/ms 90.25, render/busy/ms 88.49, pipelines/driver_ms 75.22, stage/render/prepare_assets 69.62, main/busy/ms 43.53, stage/main/post_update 21.75, stage/main/update 16.42, terrain/update/ms 10.75, stage/render/render 10.47 |
| warm-up | 3.85 | 31.4 | pipelines/compile_ms 751, pipelines/front_ms 727, pipelines/wait_ms 169, render/thread/ms 47.74, render/busy/ms 45.63, main/busy/ms 33.27, pipelines/driver_ms 23.79, encode//sum 21.98, stage/main/post_update 18.42, stage/render/render 14.72, stage/render/prepare_resources 13.75, render/graph/ms 12.40 |
| warm-up | 3.89 | 37.0 | pipelines/compile_ms 271, pipelines/front_ms 261, main/busy/ms 85.81, render/thread/ms 65.84, render/busy/ms 63.79, stage/main/update 58.25, stage/main/post_update 20.30, stage/render/prepare_write_phase_buffers 13.79, stage/render/prepare_resources 12.61, stage/render/render 12.16, pipelines/driver_ms 9.67, render/graph/ms 9.67 |
| warm-up | 3.99 | 100.1 | pipelines/compile_ms 1346, pipelines/front_ms 1274, render/thread/ms 77.94, pipelines/wait_ms 77.49, main/busy/ms 77.12, render/busy/ms 76.17, pipelines/driver_ms 72.06, stage/main/update 58.25, stage/render/prepare_assets 39.02, stage/render/render 11.64, stage/main/post_update 10.62, stage/render/prepare_write_phase_buffers 10.21 |
| warm-up | 4.03 | 41.9 | pipelines/compile_ms 1981, pipelines/front_ms 1894, pipelines/wait_ms 138, pipelines/driver_ms 86.92, render/thread/ms 68.51, render/busy/ms 67.08, stage/render/prepare_assets 50.74, main/busy/ms 24.19, stage/main/update 11.95, stage/render/render 11.49, render/graph/ms 9.77, stage/render/graph_submit 7.44 |
| warm-up | 4.15 | 54.5 | render/thread/ms 58.34, render/busy/ms 56.08, main/busy/ms 49.91, pipelines/compile_ms 45.68, pipelines/front_ms 43.80, stage/main/update 31.73, stage/render/prepare_assets 22.00, stream/drive/ms 20.64, stage/render/render 17.71, render/graph/ms 14.18, stage/main/post_update 10.15, stage/render/graph_submit 8.91 |
| warm-up | 4.21 | 62.2 | pipelines/compile_ms 250, pipelines/front_ms 242, main/busy/ms 85.39, render/thread/ms 84.03, render/busy/ms 81.46, stage/main/update 64.11, stage/render/prepare_assets 42.60, stage/render/render 20.72, render/graph/ms 16.86, stream/drive/ms 13.81, stage/main/post_update 12.27, stage/render/graph_submit 11.26 |
| warm-up | 4.34 | 128.4 | pipelines/compile_ms 1798, pipelines/front_ms 1756, render/thread/ms 127, render/busy/ms 125, main/busy/ms 80.38, stage/render/prepare_assets 67.95, stage/main/update 62.42, pipelines/driver_ms 41.70, stage/render/prepare_bind_groups 27.48, stage/render/render 16.43, render/graph/ms 13.95, stage/main/post_update 10.72 |
| warm-up | 4.37 | 33.8 | pipelines/compile_ms 1867, pipelines/front_ms 1818, main/busy/ms 192, stage/main/update 156, render/thread/ms 114, render/busy/ms 112, pipelines/driver_ms 48.50, stage/render/prepare_assets 46.52, pipelines/wait_ms 37.85, stage/render/prepare_bind_groups 26.98, stage/main/fixed_main_loop 20.43, stage/render/render 18.97 |
| warm-up | 4.63 | 256.4 | pipelines/compile_ms 3004, pipelines/front_ms 2913, pipelines/wait_ms 1030, main/busy/ms 187, stage/main/update 152, render/thread/ms 111, render/busy/ms 109, pipelines/driver_ms 91.04, stage/render/prepare_assets 74.68, stage/main/fixed_main_loop 20.10, stage/render/render 18.66, render/graph/ms 16.75 |
| warm-up | 4.67 | 37.0 | pipelines/compile_ms 3078, pipelines/front_ms 2982, pipelines/wait_ms 1304, render/thread/ms 114, render/busy/ms 112, pipelines/driver_ms 96.52, stage/render/prepare_assets 76.95, main/busy/ms 45.19, stage/main/update 26.67, stage/render/render 16.58, render/graph/ms 14.47, stage/render/graph_submit 11.42 |
| warm-up | 4.72 | 55.1 | pipelines/compile_ms 418, pipelines/front_ms 397, pipelines/wait_ms 307, render/thread/ms 81.16, render/busy/ms 79.25, main/busy/ms 51.91, stage/render/render 32.03, stage/main/update 30.04, render/graph/ms 29.65, pipelines/driver_ms 20.84, stage/render/prepare_assets 20.72, stage/render/graph_submit 20.40 |
| warm-up | 4.80 | 50.6 | pipelines/compile_ms 285, pipelines/front_ms 271, render/thread/ms 61.22, render/busy/ms 59.93, main/busy/ms 51.27, stage/main/update 28.23, stage/render/render 17.56, render/graph/ms 15.59, pipelines/driver_ms 14.80, stage/render/prepare_assets 13.93, stage/main/post_update 12.45, stage/render/graph_submit 11.88 |
| warm-up | 4.84 | 46.7 | pipelines/compile_ms 924, pipelines/front_ms 893, render/thread/ms 82.11, render/busy/ms 80.46, main/busy/ms 67.17, stage/main/update 41.00, pipelines/driver_ms 30.48, stage/render/prepare_assets 27.59, stage/main/post_update 16.62, stage/render/render 15.53, stage/render/prepare_resources 14.77, render/graph/ms 12.88 |
| warm-up | 4.90 | 53.9 | pipelines/compile_ms 1661, pipelines/front_ms 1609, main/busy/ms 114, stage/main/update 93.00, render/thread/ms 75.82, render/busy/ms 74.06, stream/drive/ms 52.37, pipelines/driver_ms 52.10, stage/render/prepare_assets 34.79, stage/render/render 17.71, render/graph/ms 15.16, stage/main/post_update 11.66 |
| warm-up | 5.00 | 105.9 | pipelines/compile_ms 1807, pipelines/front_ms 1758, main/busy/ms 104, stage/main/update 81.34, render/thread/ms 71.55, render/busy/ms 70.31, stream/drive/ms 59.53, pipelines/driver_ms 49.42, stage/render/prepare_assets 48.38, pipelines/wait_ms 23.40, stage/render/render 13.61, render/graph/ms 11.89 |
| warm-up | 5.08 | 28.3 | pipelines/compile_ms 1066, pipelines/front_ms 1032, render/thread/ms 40.12, main/busy/ms 39.18, render/busy/ms 38.30, pipelines/driver_ms 34.65, stage/main/update 24.88, stage/render/prepare_assets 12.20, stage/render/render 11.30, render/graph/ms 9.28, stage/main/post_update 8.90, encode//sum 7.93 |
| warm-up | 5.15 | 27.8 | render/thread/ms 46.20, render/busy/ms 44.53, pipelines/compile_ms 39.63, pipelines/front_ms 38.08, main/busy/ms 37.41, stage/main/update 20.80, stage/render/render 14.68, render/graph/ms 12.43, stage/main/post_update 11.80, stage/render/graph_submit 8.57, encode//sum 7.93, stage/render/prepare_assets 7.65 |
| warm-up | 5.28 | 37.0 | pipelines/compile_ms 741, pipelines/front_ms 718, main/busy/ms 44.46, render/thread/ms 37.96, render/busy/ms 36.25, stage/main/update 26.59, pipelines/driver_ms 23.25, stage/render/render 16.86, render/graph/ms 14.64, stage/main/post_update 13.03, stream/drive/ms 11.82, stage/render/graph_submit 10.26 |
| warm-up | 5.32 | 33.3 | pipelines/compile_ms 656, pipelines/front_ms 628, main/busy/ms 41.26, render/thread/ms 40.24, render/busy/ms 38.72, pipelines/driver_ms 27.66, stage/main/update 25.77, stage/render/render 14.41, render/graph/ms 12.08, stream/drive/ms 11.28, stage/render/prepare_assets 10.78, stage/main/post_update 9.65 |
| warm-up | 5.58 | 30.9 | main/busy/ms 32.85, render/thread/ms 23.38, render/busy/ms 22.38, stage/main/update 19.91, stream/drive/ms 13.08, stage/main/post_update 9.30, stage/render/render 9.03, render/graph/ms 7.58, encode//sum 6.39, stage/render/prepare_assets 5.14, stage/render/graph_submit 4.88, encode/submit 2.86 |
| warm-up | 5.86 | 32.0 | pipelines/compile_ms 613, pipelines/front_ms 597, main/busy/ms 43.92, render/thread/ms 30.37, stage/main/update 29.94, render/busy/ms 29.05, stream/drive/ms 19.86, pipelines/driver_ms 16.40, stage/render/render 13.76, render/graph/ms 12.04, stage/render/graph_submit 8.35, stage/main/post_update 7.82 |
| warm-up | 5.98 | 26.5 | pipelines/compile_ms 637, pipelines/front_ms 614, render/thread/ms 40.35, render/busy/ms 38.73, main/busy/ms 32.73, pipelines/driver_ms 22.78, stage/main/post_update 14.25, stage/render/render 14.19, stage/main/update 13.31, stage/render/prepare_assets 12.72, render/graph/ms 12.05, encode//sum 8.49 |
| warm-up | 6.03 | 28.8 | render/thread/ms 36.41, render/busy/ms 35.29, main/busy/ms 26.64, stage/render/prepare_assets 17.38, stage/main/update 11.46, stage/render/render 8.79, stage/main/post_update 7.63, render/graph/ms 7.18, encode//sum 5.98, stage/render/graph_submit 4.50, stage/main/fixed_main_loop 3.35, stream/drive/ms 3.01 |
| warm-up | 6.10 | 26.5 | main/busy/ms 38.74, render/thread/ms 37.41, render/busy/ms 35.87, stage/main/update 22.15, stage/render/render 16.05, render/graph/ms 13.86, stage/main/post_update 11.60, stage/render/graph_submit 9.78, encode//sum 8.93, stage/render/prepare_assets 7.66, encode/submit 7.01, stage/render/camera_driver 3.91 |
| warm-up | 6.23 | 27.4 | render/thread/ms 42.50, render/busy/ms 41.36, main/busy/ms 39.72, stage/main/update 18.36, stage/render/render 16.78, stage/main/post_update 16.15, render/graph/ms 15.14, stage/render/graph_submit 11.03, encode//sum 8.80, encode/submit 7.77, stage/render/prepare_assets 7.34, stage/render/prepare_meshes 5.01 |
| warm-up | 6.28 | 27.9 | render/thread/ms 37.77, main/busy/ms 36.94, render/busy/ms 36.74, stage/main/update 20.78, stage/render/prepare_assets 13.70, stage/main/post_update 8.90, stage/render/render 8.14, render/graph/ms 6.78, encode//sum 5.82, stream/drive/ms 5.50, stage/render/prepare_meshes 5.09, stage/render/graph_submit 4.27 |
| warm-up | 6.31 | 27.2 | render/thread/ms 46.05, render/busy/ms 45.09, main/busy/ms 42.68, stage/main/update 21.76, stage/render/render 14.19, stage/main/post_update 14.07, render/graph/ms 12.78, stage/render/prepare_assets 11.37, stage/render/graph_submit 9.32, encode//sum 7.06, encode/finish_wall 5.36, stage/render/prepare_meshes 5.18 |
| warm-up | 6.35 | 36.9 | render/thread/ms 58.69, render/busy/ms 57.45, main/busy/ms 45.61, stage/render/render 22.28, render/graph/ms 20.36, stage/main/update 20.07, stage/main/post_update 19.22, stage/render/graph_submit 15.74, stage/render/prepare_assets 10.59, encode/finish_wall 10.27, encode/pass:point_shadow_atlas_live_indirect/sum 9.76, encode//sum 9.73 |
| warm-up | 6.41 | 37.3 | pipelines/compile_ms 145, pipelines/front_ms 141, render/thread/ms 46.27, render/busy/ms 45.22, main/busy/ms 34.50, stage/main/update 18.81, stage/render/prepare_assets 17.33, stage/render/render 15.47, render/graph/ms 13.87, stage/render/graph_submit 11.24, stage/main/post_update 9.31, encode/finish_wall 7.98 |
| warm-up | 6.53 | 32.9 | main/busy/ms 39.72, render/thread/ms 36.27, render/busy/ms 34.46, stage/main/update 25.16, stage/render/render 19.31, render/graph/ms 16.87, stream/drive/ms 14.69, stage/render/graph_submit 13.26, stage/main/post_update 9.09, encode/finish_wall 8.74, encode/pass:point_shadow_atlas_live_indirect/sum 8.40, encode//sum 7.41 |
| warm-up | 6.59 | 42.3 | pipelines/compile_ms 133, pipelines/front_ms 130, main/busy/ms 47.47, render/thread/ms 43.90, render/busy/ms 42.41, stage/main/update 30.41, stage/render/render 20.09, render/graph/ms 17.95, stage/render/graph_submit 14.50, stage/main/post_update 10.66, stage/render/prepare_assets 10.66, encode/finish_wall 8.65 |
| warm-up | 6.63 | 34.7 | pipelines/compile_ms 133, pipelines/front_ms 130, render/thread/ms 47.28, render/busy/ms 46.22, main/busy/ms 37.36, stage/main/update 19.48, stage/render/prepare_assets 19.27, stage/render/render 15.28, render/graph/ms 13.58, stage/render/graph_submit 10.81, stage/main/post_update 9.58, encode/finish_wall 7.71 |
| warm-up | 6.92 | 34.8 | render/thread/ms 44.43, stage/render/render 35.58, render/present/ms 25.15, main/busy/ms 19.39, render/busy/ms 19.11, render/graph/ms 9.58, stage/main/post_update 9.10, encode//sum 7.42, stage/render/graph_submit 6.98, stage/main/update 6.62, encode/submit 4.16, encode/finish_wall 2.84 |
| measure | 18.28 | 32.3 | main/busy/ms 38.27, stage/main/update 25.29, render/thread/ms 22.04, render/busy/ms 21.05, stage/render/render 11.27, render/graph/ms 9.83, stage/main/post_update 8.85, stage/render/graph_submit 6.96, encode//sum 6.81, encode/submit 4.24, stage/render/prepare_assets 3.83, stage/render/camera_driver 2.75 |
| measure | 59.13 | 27.1 | render/thread/ms 40.77, render/busy/ms 39.07, main/busy/ms 34.96, stage/render/render 18.60, stage/main/post_update 17.07, render/graph/ms 16.55, encode//sum 12.26, stage/render/graph_submit 11.93, stage/main/update 10.47, stage/render/prepare_assets 8.71, encode/submit 7.90, stage/render/camera_driver 4.44 |
| measure | 59.16 | 32.0 | render/thread/ms 40.84, render/busy/ms 39.71, main/busy/ms 26.97, stage/render/prepare_assets 18.75, stage/main/post_update 11.98, stage/render/render 10.79, encode//sum 9.69, render/graph/ms 9.45, stage/render/graph_submit 6.60, stage/main/update 6.02, stage/main/fixed_main_loop 4.42, encode/submit 4.00 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 11% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (254 samples): GPU load **79%** (min 40%) · 2621 MHz (min 2610) · memory 8101 MHz · 80.4 W · 78 °C (max 84) · VRAM 5582 MiB · throttled 100% of samples

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
