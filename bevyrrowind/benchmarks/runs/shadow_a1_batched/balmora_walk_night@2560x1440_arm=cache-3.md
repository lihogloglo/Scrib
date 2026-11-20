# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5697 frames over 65.6 s · **median 11.41 ms (88 fps)** · mean 11.52 · p95 14.94 · p99 17.07 (1% low 59 fps) · worst 33.25 ms · 57 spikes over 1.5× median · 21.9 stutters/min (work 12.8/min)
- warm-up: 354 frames over 9.3 s, worst 2493.7 ms
- hitches (> 28.5 ms): **2** measured, 38 in warm-up
- timed GPU passes: 7.12 ms of a 11.52 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.41 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- **FAIL** worst frame 33.25 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.017 | 2.169 | 2.524 | 0.93 | 0.181 |
| main_opaque_pass_3d | 1.275 | 1.365 | 1.864 | 0.93 | 0.068 |
| auxiliary_views | 0.599 | 0.641 | 1.793 | 0.93 | 0.506 |
| early prepass | 0.503 | 0.538 | 0.772 | 0.93 | 0.094 |
| bloom | 0.317 | 0.341 | 0.570 | 0.93 | 0.069 |
| atmosphere_luts | 0.311 | 0.333 | 0.622 | 0.93 | 0.003 |
| water_velocity | 0.222 | 0.238 | 0.249 | 0.93 | 0.002 |
| water_sspr | 0.208 | 0.223 | 0.224 | 0.93 | 0.002 |
| ssao | 0.185 | 0.198 | 0.205 | 0.93 | 0.007 |
| atmos_froxel_inject | 0.129 | 0.138 | 0.134 | 0.93 | 0.001 |
| upscaling | 0.126 | 0.135 | 0.158 | 0.93 | 0.004 |
| water_fft | 0.121 | 0.129 | 0.128 | 0.93 | 0.003 |
| uniform_allocation | 0.115 | 0.123 | 0.148 | 0.93 | 0.024 |
| clouds_composite | 0.101 | 0.108 | 0.109 | 0.93 | 0.002 |
| bin_unpacking | 0.094 | 0.100 | 0.123 | 0.93 | 0.018 |
| gpu_instance_materialization | 0.090 | 0.097 | 0.100 | 0.93 | 0.004 |
| tonemapping | 0.087 | 0.093 | 0.099 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.071 | 0.076 | 0.141 | 0.93 | 0.008 |
| clustering | 0.061 | 0.066 | 0.068 | 0.93 | 0.019 |
| early_mesh_preprocessing | 0.056 | 0.060 | 0.075 | 0.93 | 0.004 |
| clouds_trace | 0.053 | 0.056 | 0.079 | 0.93 | 0.001 |
| render_sky | 0.051 | 0.054 | 0.054 | 0.93 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.451 | 0.757 | 0.10 | 0.001 |
| surface_trail | 0.040 | 0.058 | 0.057 | 0.69 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.033 | 0.033 | 0.93 | 0.001 |
| water_snapshot | 0.024 | 0.026 | 0.035 | 0.93 | 0.002 |
| range_unpacking | 0.023 | 0.025 | 0.031 | 0.93 | 0.003 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.220 | 0.231 | 0.10 | 0.004 |
| per_view_shadow_pass | 0.018 | 0.019 | 0.027 | 0.93 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.018 | 0.031 | 0.93 | 0.205 |
| night_sky | 0.016 | 0.017 | 0.042 | 0.93 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.93 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.010 | 0.93 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.93 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.171 | 13.497 | 29.01 | 57942.7 |
| render/busy/ms | 9.616 | 12.950 | 20.09 | 54780.4 |
| main/busy/ms | 8.602 | 10.345 | 31.02 | 49007.4 |
| stage/render/render | 6.487 | 9.087 | 25.05 | 36954.9 |
| render/graph/ms | 5.775 | 8.359 | 11.77 | 32901.2 |
| encode//sum | 5.291 | 9.133 | 19.10 | 30143.4 |
| stage/main/post_update | 4.903 | 6.111 | 9.93 | 27933.6 |
| stage/render/graph_submit | 4.079 | 5.964 | 8.54 | 23238.0 |
| encode/submit | 2.728 | 4.115 | 5.97 | 15540.2 |
| stage/main/update | 1.897 | 2.464 | 24.43 | 10806.6 |
| stage/render/camera_driver | 1.638 | 2.488 | 3.50 | 9333.3 |
| encode/finish_wall | 1.429 | 1.997 | 4.28 | 8140.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.813 | 1.128 | 2.29 | 4630.5 |
| encode/pass:shadow_directional_light/sum | 0.797 | 1.251 | 2.32 | 4543.2 |
| encode/pass:early prepass/sum | 0.783 | 1.096 | 2.26 | 4458.7 |
| stage/render/prepare_write_phase_buffers | 0.654 | 1.049 | 2.10 | 3723.1 |
| encode/pass:shadow_point_light/sum | 0.626 | 2.649 | 4.32 | 3567.9 |
| stage/main/fixed_main_loop | 0.593 | 1.036 | 2.53 | 3378.7 |
| stage/render/prepare_assets | 0.576 | 0.708 | 9.69 | 3282.8 |
| bigspace/total/ms | 0.524 | 1.249 | 2.80 | 2987.6 |
| render/present/ms | 0.464 | 1.160 | 17.59 | 2645.6 |
| stage/render/prepare_meshes | 0.457 | 0.590 | 4.04 | 2604.7 |
| stage/render/prepare_resources | 0.438 | 0.552 | 1.02 | 2495.6 |
| stage/main/pre_update | 0.392 | 0.514 | 2.41 | 2234.4 |
| stage/main/first | 0.360 | 0.468 | 5.53 | 2052.7 |
| stage/render/prepare_bind_groups | 0.339 | 0.598 | 1.56 | 1931.3 |
| controller/feed/ms | 0.337 | 0.434 | 0.81 | 1922.1 |
| stage/render/prepare_flush | 0.300 | 0.524 | 1.58 | 1710.4 |
| stage/render/prepare_batch_phases | 0.291 | 0.443 | 0.85 | 1656.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.283 | 0.443 | 1.02 | 1610.1 |
| stage/main/last | 0.267 | 0.326 | 2.22 | 1519.3 |
| bigspace/low_precision/ms | 0.221 | 0.593 | 2.46 | 1258.7 |
| stream/drive/ms | 0.209 | 0.311 | 11.24 | 1188.4 |
| stage/render/create_views | 0.198 | 0.269 | 0.85 | 1130.6 |
| stage/render/extract_commands | 0.174 | 0.210 | 0.42 | 989.3 |
| anim/evaluate/ms | 0.161 | 0.255 | 2.17 | 918.7 |
| anim/evaluate_cpu/ms | 0.157 | 0.198 | 0.43 | 896.5 |
| stage/render/queue | 0.152 | 0.234 | 1.77 | 866.7 |
| stream/tick/ms | 0.151 | 0.227 | 0.83 | 857.6 |
| bigspace/high_precision/ms | 0.148 | 0.600 | 1.79 | 845.4 |
| stage/render/specialize | 0.144 | 0.234 | 1.94 | 822.7 |
| stage/render/prepare_views | 0.137 | 0.182 | 1.15 | 780.3 |
| pipelines/compile_ms | 0.131 | 0.000 | 134.36 | 745.7 |
| pipelines/front_ms | 0.127 | 0.000 | 131.19 | 725.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.097 | 0.159 | 0.53 | 553.3 |
| controller/move/ms | 0.091 | 0.282 | 0.81 | 519.8 |
| render/acquire/ms | 0.091 | 0.120 | 0.45 | 516.7 |
| controller/write/ms | 0.076 | 0.103 | 0.35 | 430.4 |
| terrain/update/ms | 0.063 | 0.303 | 1.55 | 358.3 |
| physics/step/ms | 0.049 | 0.096 | 0.49 | 280.1 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.075 | 0.24 | 270.1 |
| stage/render/phase_sort | 0.044 | 0.073 | 0.33 | 250.2 |
| encode/point_shadow_cache_copies/sum | 0.034 | 0.100 | 0.49 | 196.3 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.58 | 180.1 |
| physics/insert/ms | 0.032 | 0.367 | 0.84 | 179.9 |
| stage/render/cleanup | 0.026 | 0.040 | 0.87 | 148.2 |
| terrain/select/ms | 0.026 | 0.268 | 0.79 | 146.3 |
| weather/map/ms | 0.017 | 0.023 | 0.56 | 96.4 |
| far/select/ms | 0.017 | 0.174 | 0.38 | 94.8 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.16 | 72.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.36 | 63.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.30 | 61.4 |
| weather/eval/ms | 0.011 | 0.014 | 0.33 | 60.4 |
| script/update/ms | 0.008 | 0.012 | 0.21 | 46.2 |
| controller/avoid/ms | 0.007 | 0.012 | 0.13 | 39.0 |
| stream/despawn/ms | 0.006 | 0.000 | 1.14 | 36.8 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 27.9 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 27.5 |
| stream/spawn/ms | 0.004 | 0.000 | 1.14 | 22.7 |
| pipelines/driver_ms | 0.004 | 0.000 | 3.54 | 20.7 |
| stream/upload/ms | 0.004 | 0.000 | 1.48 | 20.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 13.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.45 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1291 | 2262 | 537 |
| crowd/c0/meshes/count | count | 353 | 646 | 146 |
| crowd/c1/count | count | 148 | 156 | 148 |
| crowd/c1/joints/count | count | 5638 | 5982 | 5627 |
| crowd/c1/meshes/count | count | 1182 | 1284 | 1187 |
| crowd/c2/count | count | 399 | 418 | 418 |
| crowd/c2/joints/count | count | 6406 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1822 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 162 | 216 | 205 |
| crowd/joints_extracted/count | count | 728 | 6955 | 597 |
| crowd/joints_uploaded/count | count | 3670 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.1 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.45 | 1.26 | 0.52 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.16 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.30 | 0.01 |
| encode/finish_critical | ms | 0.97 | 2.32 | 1.18 |
| encode/pass:early prepass/max | ms | 0.79 | 2.26 | 1.18 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 2.29 | 0.65 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.53 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 1.02 | 0.39 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.32 | 0.92 |
| encode/pass:shadow_point_light/max | ms | 0.12 | 1.44 | 0.05 |
| encode/point_shadow_cache_copies/max | ms | 0.06 | 0.49 | 0.03 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.58 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.40 | 31.02 | 8.16 |
| gpu/timed/ms | ms | 8.19 | 10.36 | 8.16 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 9 | 28 | 0 |
| lamp_cache/copies | count | 0 | 16 | 0 |
| lamp_cache/deferred_faces | count | 0 | 16 | 0 |
| lamp_cache/lamps | count | 6 | 8 | 0 |
| lamp_cache/mover_faces | count | 0 | 12 | 0 |
| lamp_cache/movers | count | 111 | 269 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 6 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 16 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 7 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 12 | 36 | 0 |
| lamp_cache/stock_lamps | count | 0 | 2 | 0 |
| lamp_cache/store_starved | count | 0 | 8 | 0 |
| lamp_cache/store_used | count | 7 | 24 | 0 |
| lamp_cache/views | count | 7 | 36 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2607 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 168 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 681 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 389 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 168 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 681 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 389 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 762 | 971 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5843 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2469 | 3538 | 2448 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 762 | 971 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5843 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2469 | 3538 | 2448 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 92 | 8 |
| phases/shadow/bins | count | 758 | 1357 | 671 |
| phases/shadow/instance_batches | count | 4 | 10 | 4 |
| phases/shadow/instance_draws | count | 6657 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 106.5 k | 49.2 k |
| phases/shadow/items | count | 2236 | 3091 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 41 | 113 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 333 | 334 | 334 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 25 | 304 | 11 |
| preprocess/bind_groups_created | count | 20 | 298 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 8 | 5 |
| preprocess/bind_groups_reused | count | 128 | 263 | 123 |
| process/cpu_usage | % | 24.52 | 36.61 | 29.03 |
| process/mem_usage | GiB | 5.42 | 5.68 | 5.68 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 3092 | 3166 | 3166 |
| scene/batch/masked/live | count | 3092 | 3166 | 3166 |
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
| scene/batch/opaque/high_water | count | 19.2 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 19.2 k | 19.9 k | 19.9 k |
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
| scene/cells/resident | count | 120 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 22.3 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 22.3 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 22.3 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 22.3 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 15 | 121 | 13 |
| skin/blocks_skipped/count | count | 147 | 204 | 192 |
| skin/blocks_walked/count | count | 15 | 28 | 13 |
| skin/joint_bytes_written/bytes | bytes | 51.2 k | 811.8 k | 42.5 k |
| skin/joints_written/count | count | 800 | 12.7 k | 664 |
| skin/upload_writes/count | count | 8 | 28 | 11 |
| started_load_count |  | 6465 | 6565 | 6565 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 229 | 333 | 180 |
| statics/materials_bound | count | 229 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 277 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.31 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.67 | 53.20 | 45.83 |
| system/mem_usage | % | 56.69 | 57.45 | 57.45 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 85 | 147 | 66 |
| views/all/bins | count | 2923 | 3569 | 2961 |
| views/all/count | count | 12 | 41 | 3 |
| views/all/instance_batches | count | 12 | 18 | 12 |
| views/all/instance_draws | count | 20.0 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 204.8 k | 147.5 k |
| views/all/items | count | 8006 | 10.3 k | 7953 |
| views/all/pipelines | count | 87 | 156 | 66 |
| views/all/pipelines_distinct | count | 65 | 74 | 61 |
| views/all/work_items | count | 154.0 k | 214.3 k | 153.9 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 716 | 877 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6657 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2095 | 2444 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 7 | 61 | 0 |
| views/lamp_face/bins | count | 57 | 524 | 0 |
| views/lamp_face/count | count | 9 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 10 | 0 |
| views/lamp_face/drawn | count | 2 | 25 | 0 |
| views/lamp_face/instance_batches | count | 0 | 2 | 0 |
| views/lamp_face/instance_draws | count | 0 | 24 | 0 |
| views/lamp_face/instance_slots | count | 0 | 8192 | 0 |
| views/lamp_face/items | count | 78 | 710 | 0 |
| views/lamp_face/pipelines | count | 15 | 87 | 0 |
| views/lamp_face/revealed | count | 0 | 2 | 0 |
| views/lamp_face/work_items | count | 25 | 8440 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2028 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 13.0 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5890 | 7496 | 5926 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.9 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 42.08 | 223 | 239.8 k |
| anim/events/count | count | 0.09 | 3 | 500 |
| assets/Image/added | count | 0.10 | 38 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.50 | 305 | 2874 |
| assets/Mesh/removed | count | 0.85 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.31 | 10 | 7481 |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.26 | 1154 | 1489 |
| encode//count | count | 53.61 | 128 | 305.4 k |
| encode/bind_walk/init_textures | count | 9767 | 21.4 k | 55.64 M |
| encode/bind_walk/init_textures_skipped | count | 53.6 k | 78.2 k | 305.42 M |
| encode/bind_walk/inits | count | 606 | 776 | 3.45 M |
| encode/bind_walk/inits_skipped | count | 142 | 234 | 810.2 k |
| encode/bind_walk/merge_textures | count | 9704 | 21.3 k | 55.28 M |
| encode/bind_walk/merge_textures_skipped | count | 9698 | 21.1 k | 55.25 M |
| encode/bind_walk/merges | count | 120 | 205 | 684.3 k |
| encode/bind_walk/merges_skipped | count | 131 | 228 | 748.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5697 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5697 |
| encode/pass:early prepass/count | count | 1 | 1 | 5697 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5697 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5697 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5697 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5697 |
| encode/pass:shadow_point_light/count | count | 6.40 | 25 | 36.5 k |
| encode/point_shadow_cache_copies/count | count | 0.50 | 1 | 2846 |
| encode/queue/command_buffers | count | 70.53 | 164 | 401.8 k |
| encode/queue/submits | count | 2.01 | 4 | 11.5 k |
| encode/queue/write_buffer/bytes | bytes | 2.35 M | 3.43 M | 13.40 G |
| encode/queue/write_buffer/calls | count | 287 | 480 | 1.64 M |
| encode/queue/write_buffer_with/bytes | bytes | 122.7 k | 1.25 M | 699.01 M |
| encode/queue/write_buffer_with/calls | count | 10.50 | 924 | 59.8 k |
| encode/queue/write_texture/bytes | bytes | 2235 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5697 |
| physics/inserts | count | 0.39 | 89 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 175 | 2335 |
| physics/steps | count | 0.69 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 1 | 9 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 35 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5942 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1303 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.5 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2493.7 | main/busy/ms 2465, stage/main/post_update 2424, stage/main/update 31.51, stage/main/pre_update 6.24, stage/main/last 0.96, surface/advance_cpu_ms 0.82, stage/main/first 0.55, stream/drive/ms 0.44, stage/main/fixed_main_loop 0.38, surface/cell_upload_ms 0.30, stream/tick/ms 0.11, bigspace/total/ms 0.07 |
| warm-up | 3.01 | 511.5 | main/busy/ms 2590, stage/main/post_update 2449, render/thread/ms 477, render/busy/ms 391, stage/render/prepare_assets 359, stage/main/update 124, stage/render/prepare_views 84.19, render/acquire/ms 84.00, stage/render/create_views 14.83, stage/render/render 10.77, stage/main/pre_update 7.95, render/graph/ms 7.57 |
| warm-up | 3.06 | 59.1 | render/thread/ms 517, render/busy/ms 430, stage/render/prepare_assets 386, main/busy/ms 146, stage/main/update 98.04, stage/render/prepare_views 84.67, render/acquire/ms 84.36, pipelines/compile_ms 56.66, pipelines/front_ms 52.21, stage/main/post_update 36.40, terrain/update/ms 18.41, stage/render/render 16.78 |
| warm-up | 3.11 | 49.2 | pipelines/compile_ms 231, pipelines/front_ms 212, pipelines/wait_ms 169, render/thread/ms 87.83, render/busy/ms 82.30, main/busy/ms 73.70, stage/main/post_update 54.46, stage/render/render 47.40, render/graph/ms 41.38, stage/render/camera_driver 29.17, stage/render/prepare_assets 28.41, terrain/update/ms 24.92 |
| warm-up | 3.24 | 128.8 | pipelines/wait_ms 5860, pipelines/compile_ms 1189, pipelines/front_ms 1075, render/thread/ms 168, render/busy/ms 163, pipelines/driver_ms 114, main/busy/ms 104, stage/main/post_update 86.60, stage/render/render 78.00, render/graph/ms 72.12, stage/render/prepare_resources 48.66, stage/render/graph_submit 40.00 |
| warm-up | 3.28 | 37.8 | pipelines/wait_ms 6369, pipelines/compile_ms 1377, pipelines/front_ms 1243, render/thread/ms 157, main/busy/ms 155, render/busy/ms 154, pipelines/driver_ms 133, stage/main/post_update 75.91, encode//sum 53.94, stage/main/update 52.21, stage/render/prepare_resources 51.50, stage/render/render 51.21 |
| warm-up | 3.43 | 148.6 | pipelines/compile_ms 2099, pipelines/front_ms 1993, pipelines/wait_ms 1115, main/busy/ms 121, render/thread/ms 112, render/busy/ms 110, pipelines/driver_ms 107, stage/render/prepare_assets 66.63, stage/main/update 56.13, stage/main/post_update 40.23, stage/render/render 27.35, encode//sum 25.17 |
| warm-up | 3.48 | 28.9 | pipelines/compile_ms 1512, pipelines/front_ms 1450, pipelines/wait_ms 152, pipelines/driver_ms 61.96, render/thread/ms 50.69, render/busy/ms 48.58, main/busy/ms 39.76, stage/render/prepare_assets 23.86, stage/main/post_update 15.15, stage/render/render 11.04, stage/main/pre_update 9.89, stage/main/update 9.86 |
| warm-up | 3.51 | 28.6 | pipelines/compile_ms 1516, pipelines/front_ms 1451, pipelines/wait_ms 157, pipelines/driver_ms 64.90, render/thread/ms 54.04, render/busy/ms 51.71, main/busy/ms 41.49, stage/render/prepare_resources 20.12, stage/main/post_update 19.80, stage/render/render 13.14, stage/render/prepare_assets 11.08, render/graph/ms 10.38 |
| warm-up | 3.55 | 39.0 | pipelines/compile_ms 144, pipelines/front_ms 134, main/busy/ms 76.31, render/thread/ms 64.39, render/busy/ms 61.87, stage/main/update 50.96, stage/render/render 29.14, render/graph/ms 26.11, stage/main/post_update 19.85, pipelines/wait_ms 18.93, stage/render/prepare_resources 16.51, stage/render/camera_driver 12.92 |
| warm-up | 3.64 | 93.2 | pipelines/compile_ms 662, pipelines/front_ms 618, render/thread/ms 96.13, render/busy/ms 93.41, main/busy/ms 77.72, stage/main/update 60.28, pipelines/driver_ms 43.90, stage/render/prepare_assets 40.58, stage/render/render 37.63, render/graph/ms 34.46, stage/render/camera_driver 16.97, stage/render/graph_submit 15.49 |
| warm-up | 3.68 | 39.4 | pipelines/compile_ms 1572, pipelines/front_ms 1500, render/thread/ms 86.54, render/busy/ms 84.50, pipelines/driver_ms 72.41, stage/render/prepare_assets 57.48, main/busy/ms 35.33, stage/render/render 20.19, stage/main/update 19.63, pipelines/wait_ms 19.28, render/graph/ms 17.75, stage/render/graph_submit 9.59 |
| warm-up | 3.73 | 43.4 | pipelines/compile_ms 1222, pipelines/front_ms 1177, render/thread/ms 54.49, render/busy/ms 53.17, pipelines/driver_ms 44.27, stage/render/prepare_assets 38.86, pipelines/wait_ms 23.88, main/busy/ms 22.37, stage/render/render 10.34, stage/main/update 10.12, render/graph/ms 8.75, stage/render/graph_submit 6.43 |
| warm-up | 3.87 | 56.2 | pipelines/compile_ms 414, pipelines/front_ms 387, render/thread/ms 64.50, render/busy/ms 61.83, main/busy/ms 43.53, stage/render/prepare_assets 36.37, pipelines/driver_ms 27.28, stage/main/update 18.18, stage/render/render 14.13, stage/main/post_update 13.00, render/graph/ms 10.74, encode//sum 8.24 |
| warm-up | 3.92 | 51.1 | pipelines/compile_ms 700, pipelines/front_ms 660, main/busy/ms 135, stage/main/update 108, render/thread/ms 70.69, render/busy/ms 68.48, stage/render/prepare_assets 50.11, pipelines/driver_ms 40.05, stage/main/post_update 12.60, stage/render/render 10.59, render/graph/ms 7.93, encode//sum 6.82 |
| warm-up | 4.06 | 140.0 | pipelines/compile_ms 1531, pipelines/front_ms 1487, main/busy/ms 150, stage/main/update 129, render/thread/ms 71.78, render/busy/ms 70.16, stage/render/prepare_assets 51.17, pipelines/driver_ms 44.18, stage/main/post_update 13.84, stage/render/render 12.43, render/graph/ms 10.40, encode//sum 10.31 |
| warm-up | 4.11 | 54.1 | pipelines/compile_ms 1612, pipelines/front_ms 1567, render/thread/ms 79.72, render/busy/ms 77.22, pipelines/wait_ms 74.09, main/busy/ms 63.67, stage/render/prepare_assets 51.98, pipelines/driver_ms 45.12, stage/main/update 41.56, stage/render/render 15.18, stage/main/post_update 13.41, render/graph/ms 12.24 |
| warm-up | 4.19 | 75.8 | pipelines/compile_ms 375, pipelines/front_ms 361, render/thread/ms 108, render/busy/ms 106, main/busy/ms 76.57, pipelines/wait_ms 71.47, stage/main/update 59.99, stage/render/prepare_assets 25.80, stage/render/prepare_resources 23.48, stage/render/render 14.19, pipelines/driver_ms 13.94, stage/render/prepare_batch_phases 13.01 |
| warm-up | 4.23 | 40.8 | render/thread/ms 102, render/busy/ms 101, main/busy/ms 72.21, stage/main/update 56.13, stage/render/prepare_resources 23.67, stage/render/prepare_assets 20.98, stage/render/prepare_batch_phases 13.03, stage/render/render 12.65, render/graph/ms 10.79, stream/drive/ms 10.31, stage/main/post_update 10.19, encode//sum 9.75 |
| warm-up | 4.31 | 77.1 | pipelines/compile_ms 1730, pipelines/front_ms 1664, pipelines/wait_ms 194, render/thread/ms 104, render/busy/ms 96.69, main/busy/ms 68.51, pipelines/driver_ms 66.33, stage/main/post_update 36.79, stage/render/prepare_resources 31.25, stage/render/render 26.09, stage/main/update 24.31, render/graph/ms 23.53 |
| warm-up | 4.40 | 92.0 | pipelines/compile_ms 3635, pipelines/front_ms 3501, pipelines/wait_ms 1476, render/thread/ms 156, render/busy/ms 149, pipelines/driver_ms 134, main/busy/ms 120, stage/render/prepare_resources 66.39, stage/main/post_update 61.29, stage/main/update 41.16, stage/render/render 29.52, render/graph/ms 26.70 |
| warm-up | 4.48 | 79.4 | pipelines/compile_ms 2164, pipelines/front_ms 2086, pipelines/wait_ms 1424, render/thread/ms 130, render/busy/ms 129, main/busy/ms 116, pipelines/driver_ms 78.20, stage/main/update 61.57, stage/render/prepare_assets 46.82, stage/render/prepare_resources 37.20, stage/main/post_update 36.01, stage/render/render 15.81 |
| warm-up | 4.56 | 78.4 | pipelines/compile_ms 1660, pipelines/front_ms 1614, pipelines/wait_ms 1101, render/thread/ms 94.89, render/busy/ms 93.53, main/busy/ms 75.32, stage/render/prepare_assets 72.05, stage/main/update 51.45, pipelines/driver_ms 46.19, stage/render/render 12.02, stage/main/post_update 10.84, render/graph/ms 10.03 |
| warm-up | 4.59 | 32.4 | pipelines/compile_ms 1644, pipelines/front_ms 1601, pipelines/wait_ms 1154, render/thread/ms 77.04, render/busy/ms 74.53, main/busy/ms 67.98, pipelines/driver_ms 43.12, stage/render/prepare_assets 40.47, stage/main/update 39.07, stage/render/render 20.85, render/graph/ms 17.87, stream/drive/ms 16.57 |
| warm-up | 4.62 | 32.0 | pipelines/compile_ms 971, pipelines/front_ms 945, pipelines/wait_ms 398, main/busy/ms 104, stage/main/update 81.29, render/thread/ms 47.41, render/busy/ms 44.51, stream/drive/ms 43.02, pipelines/driver_ms 25.21, stage/render/render 21.69, render/graph/ms 18.27, stage/main/post_update 13.87 |
| warm-up | 4.72 | 98.6 | pipelines/compile_ms 906, pipelines/front_ms 883, pipelines/wait_ms 204, main/busy/ms 106, stage/main/update 90.85, render/thread/ms 59.70, render/busy/ms 57.96, stream/drive/ms 55.05, pipelines/driver_ms 22.81, stage/render/prepare_assets 17.42, stage/render/render 16.96, render/graph/ms 14.52 |
| warm-up | 4.76 | 37.8 | pipelines/compile_ms 643, pipelines/front_ms 621, render/thread/ms 71.01, render/busy/ms 69.58, main/busy/ms 65.82, stage/main/update 47.88, stream/drive/ms 28.78, stage/render/prepare_assets 23.38, pipelines/driver_ms 22.38, stage/render/render 18.01, render/graph/ms 15.89, encode//sum 12.09 |
| warm-up | 4.84 | 30.0 | pipelines/compile_ms 203, pipelines/front_ms 197, render/thread/ms 49.26, render/busy/ms 47.52, main/busy/ms 42.34, stage/render/render 22.32, render/graph/ms 19.97, stage/main/update 19.29, stage/main/post_update 16.90, encode//sum 15.66, stage/render/graph_submit 13.63, stage/render/prepare_assets 7.64 |
| warm-up | 4.99 | 34.4 | pipelines/compile_ms 412, pipelines/front_ms 399, render/thread/ms 54.76, render/busy/ms 53.35, main/busy/ms 49.60, stage/main/update 30.77, stage/render/render 27.99, render/graph/ms 26.12, encode//sum 21.77, stage/render/graph_submit 19.73, pipelines/driver_ms 12.73, stream/drive/ms 11.54 |
| warm-up | 5.02 | 32.4 | render/thread/ms 57.76, render/busy/ms 56.52, main/busy/ms 44.27, stage/render/render 28.74, render/graph/ms 26.89, stage/main/update 24.78, encode//sum 21.02, stage/render/graph_submit 20.76, stage/main/post_update 12.98, encode/finish_wall 11.65, stream/drive/ms 10.66, stage/render/prepare_assets 10.02 |
| warm-up | 5.68 | 31.1 | render/thread/ms 39.19, render/busy/ms 38.32, main/busy/ms 27.88, stage/render/prepare_assets 20.62, stage/main/update 12.57, stage/render/render 9.65, encode//sum 9.40, render/graph/ms 8.31, stage/main/post_update 7.57, stage/render/graph_submit 5.25, stage/main/fixed_main_loop 3.99, stream/drive/ms 3.25 |
| warm-up | 5.91 | 37.1 | render/thread/ms 46.40, render/busy/ms 45.26, main/busy/ms 41.57, stage/main/update 22.87, stage/render/render 16.60, render/graph/ms 14.67, encode//sum 13.56, stage/main/post_update 13.28, stage/render/prepare_assets 11.40, stage/render/graph_submit 9.46, encode/submit 5.53, stage/render/prepare_meshes 5.33 |
| warm-up | 5.96 | 34.0 | render/thread/ms 46.38, render/busy/ms 45.32, main/busy/ms 36.78, stage/main/update 16.21, stage/render/prepare_assets 14.13, stage/render/render 12.65, stage/main/post_update 12.09, render/graph/ms 11.19, encode//sum 10.76, stage/render/graph_submit 7.29, stage/render/prepare_meshes 6.22, encode/pass:shadow_point_light/sum 4.76 |
| warm-up | 6.01 | 42.0 | pipelines/compile_ms 146, pipelines/front_ms 139, render/thread/ms 67.78, render/busy/ms 66.04, encode//sum 64.69, main/busy/ms 41.03, stage/render/render 30.61, render/graph/ms 28.41, stage/render/graph_submit 20.95, encode/pass:shadow_point_light/sum 20.33, stage/main/update 18.95, stage/render/prepare_assets 14.77 |
| warm-up | 6.07 | 35.2 | render/thread/ms 44.63, render/busy/ms 43.60, main/busy/ms 33.09, stage/main/update 17.08, stage/render/prepare_assets 16.69, stage/render/render 13.71, encode//sum 13.17, render/graph/ms 12.00, pipelines/compile_ms 9.46, stage/main/post_update 9.11, pipelines/front_ms 8.75, encode/pass:shadow_point_light/sum 7.94 |
| warm-up | 6.22 | 33.3 | pipelines/compile_ms 129, pipelines/front_ms 125, main/busy/ms 45.83, render/thread/ms 43.24, render/busy/ms 41.80, stage/main/update 30.22, stage/render/render 20.54, render/graph/ms 18.47, stream/drive/ms 13.66, encode//sum 13.34, stage/render/graph_submit 13.18, encode/submit 9.75 |
| warm-up | 6.26 | 38.2 | render/thread/ms 57.61, render/busy/ms 56.41, main/busy/ms 48.97, stage/main/update 26.68, stage/render/render 21.16, encode//sum 21.06, render/graph/ms 19.15, encode/pass:shadow_point_light/sum 15.19, stage/render/prepare_assets 14.35, stage/main/post_update 12.91, stage/render/graph_submit 11.53, pipelines/compile_ms 11.28 |
| warm-up | 6.29 | 28.9 | render/thread/ms 55.84, render/busy/ms 54.67, main/busy/ms 44.68, stage/main/update 23.96, stage/render/render 21.75, encode//sum 21.21, render/graph/ms 19.98, stage/render/prepare_assets 17.90, encode/pass:shadow_point_light/sum 14.85, stage/render/graph_submit 13.06, stage/main/post_update 12.41, pipelines/compile_ms 11.28 |
| measure | 15.00 | 30.5 | render/thread/ms 43.00, stage/render/render 33.92, render/busy/ms 24.62, main/busy/ms 21.06, render/present/ms 18.16, render/graph/ms 15.10, encode//sum 14.28, stage/main/post_update 11.93, stage/render/graph_submit 10.65, encode/submit 6.67, stage/main/update 4.37, stage/render/camera_driver 4.32 |
| measure | 18.41 | 33.2 | main/busy/ms 41.34, render/thread/ms 29.23, render/busy/ms 28.11, stage/main/update 26.78, encode//sum 18.93, stage/render/render 17.54, render/graph/ms 16.06, stage/render/graph_submit 11.80, stage/main/post_update 9.33, encode/submit 7.78, stage/render/prepare_assets 4.31, stage/render/camera_driver 4.12 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=cache-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 16%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1449 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **79%** (min 57%) · 2625 MHz (min 2625) · memory 8101 MHz · 75.4 W · 73 °C (max 79) · VRAM 6276 MiB · throttled 100% of samples

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
- `shadow.lamp.cache.enabled=on (command line: --set)`
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
