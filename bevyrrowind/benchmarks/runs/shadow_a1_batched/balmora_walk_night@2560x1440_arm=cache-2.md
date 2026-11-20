# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5812 frames over 65.6 s · **median 11.21 ms (89 fps)** · mean 11.30 · p95 14.64 · p99 16.68 (1% low 60 fps) · worst 28.41 ms · 53 spikes over 1.5× median · 20.1 stutters/min (work 13.7/min)
- warm-up: 357 frames over 9.5 s, worst 2517.9 ms
- hitches (> 28.0 ms): **1** measured, 39 in warm-up
- timed GPU passes: 6.93 ms of a 11.30 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.21 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- pass worst frame 28.41 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.980 | 2.166 | 2.531 | 0.91 | 0.180 |
| main_opaque_pass_3d | 1.233 | 1.347 | 1.847 | 0.92 | 0.067 |
| auxiliary_views | 0.570 | 0.623 | 1.697 | 0.92 | 0.492 |
| early prepass | 0.489 | 0.534 | 0.754 | 0.92 | 0.092 |
| bloom | 0.312 | 0.341 | 0.648 | 0.91 | 0.068 |
| atmosphere_luts | 0.303 | 0.331 | 0.509 | 0.92 | 0.003 |
| water_velocity | 0.218 | 0.239 | 0.248 | 0.91 | 0.002 |
| water_sspr | 0.197 | 0.215 | 0.210 | 0.92 | 0.002 |
| ssao | 0.182 | 0.199 | 0.206 | 0.92 | 0.008 |
| atmos_froxel_inject | 0.125 | 0.136 | 0.130 | 0.92 | 0.002 |
| upscaling | 0.123 | 0.134 | 0.151 | 0.91 | 0.004 |
| water_fft | 0.118 | 0.129 | 0.126 | 0.92 | 0.003 |
| uniform_allocation | 0.112 | 0.122 | 0.146 | 0.92 | 0.023 |
| clouds_composite | 0.098 | 0.107 | 0.109 | 0.92 | 0.002 |
| bin_unpacking | 0.091 | 0.100 | 0.122 | 0.92 | 0.018 |
| tonemapping | 0.085 | 0.093 | 0.097 | 0.91 | 0.004 |
| gpu_instance_materialization | 0.084 | 0.091 | 0.094 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.072 | 0.078 | 0.145 | 0.92 | 0.008 |
| clustering | 0.061 | 0.066 | 0.068 | 0.92 | 0.019 |
| early_mesh_preprocessing | 0.054 | 0.059 | 0.076 | 0.92 | 0.004 |
| clouds_trace | 0.051 | 0.056 | 0.077 | 0.92 | 0.001 |
| render_sky | 0.050 | 0.055 | 0.054 | 0.92 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.044 | 0.466 | 0.762 | 0.09 | 0.001 |
| surface_trail | 0.039 | 0.059 | 0.057 | 0.67 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.032 | 0.032 | 0.92 | 0.001 |
| water_snapshot | 0.024 | 0.026 | 0.033 | 0.92 | 0.002 |
| range_unpacking | 0.023 | 0.025 | 0.031 | 0.92 | 0.003 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.220 | 0.230 | 0.09 | 0.004 |
| per_view_shadow_pass | 0.017 | 0.018 | 0.027 | 0.92 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.018 | 0.031 | 0.92 | 0.200 |
| night_sky | 0.015 | 0.017 | 0.042 | 0.92 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.92 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.92 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.91 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.944 | 13.227 | 28.80 | 57792.7 |
| render/busy/ms | 9.389 | 12.680 | 21.79 | 54568.1 |
| main/busy/ms | 8.424 | 10.075 | 26.18 | 48958.8 |
| stage/render/render | 6.326 | 8.884 | 14.33 | 36768.2 |
| render/graph/ms | 5.614 | 8.170 | 13.29 | 32627.8 |
| stage/main/post_update | 4.792 | 5.872 | 11.39 | 27851.3 |
| encode//sum | 4.724 | 7.494 | 14.91 | 27458.6 |
| stage/render/graph_submit | 3.951 | 5.729 | 9.22 | 22963.4 |
| encode/submit | 2.674 | 4.055 | 5.61 | 15538.9 |
| stage/main/update | 1.881 | 2.432 | 21.07 | 10932.5 |
| stage/render/camera_driver | 1.605 | 2.443 | 3.99 | 9327.8 |
| encode/finish_wall | 1.362 | 1.819 | 3.64 | 7915.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.794 | 1.091 | 2.10 | 4615.8 |
| encode/pass:shadow_directional_light/sum | 0.788 | 1.245 | 1.79 | 4577.8 |
| encode/pass:early prepass/sum | 0.770 | 1.084 | 2.00 | 4473.5 |
| stage/render/prepare_write_phase_buffers | 0.629 | 1.020 | 2.77 | 3655.2 |
| encode/pass:shadow_point_light/sum | 0.599 | 2.617 | 4.33 | 3484.1 |
| stage/render/prepare_assets | 0.568 | 0.697 | 10.05 | 3303.7 |
| stage/main/fixed_main_loop | 0.559 | 0.953 | 1.97 | 3246.1 |
| bigspace/total/ms | 0.501 | 1.176 | 3.12 | 2914.2 |
| render/present/ms | 0.466 | 1.184 | 8.83 | 2706.4 |
| stage/render/prepare_meshes | 0.434 | 0.554 | 8.33 | 2524.7 |
| stage/render/prepare_resources | 0.430 | 0.536 | 1.36 | 2501.0 |
| stage/main/pre_update | 0.389 | 0.519 | 2.88 | 2263.1 |
| stage/main/first | 0.355 | 0.464 | 5.63 | 2061.4 |
| stage/render/prepare_bind_groups | 0.338 | 0.587 | 2.02 | 1962.1 |
| controller/feed/ms | 0.319 | 0.410 | 0.93 | 1852.5 |
| stage/render/prepare_flush | 0.303 | 0.543 | 2.75 | 1760.1 |
| stage/render/prepare_batch_phases | 0.293 | 0.441 | 0.85 | 1703.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.276 | 0.436 | 0.89 | 1606.3 |
| stage/main/last | 0.262 | 0.319 | 1.41 | 1523.6 |
| bigspace/low_precision/ms | 0.209 | 0.498 | 1.58 | 1212.5 |
| stream/drive/ms | 0.206 | 0.303 | 11.24 | 1197.9 |
| stage/render/create_views | 0.200 | 0.266 | 1.00 | 1161.2 |
| stage/render/extract_commands | 0.173 | 0.209 | 0.63 | 1004.6 |
| stage/render/queue | 0.151 | 0.232 | 1.74 | 875.4 |
| anim/evaluate/ms | 0.150 | 0.244 | 2.27 | 873.1 |
| anim/evaluate_cpu/ms | 0.150 | 0.190 | 0.55 | 871.8 |
| stream/tick/ms | 0.149 | 0.225 | 0.56 | 863.7 |
| bigspace/high_precision/ms | 0.143 | 0.553 | 2.68 | 829.1 |
| stage/render/specialize | 0.141 | 0.227 | 1.71 | 822.3 |
| stage/render/prepare_views | 0.136 | 0.184 | 0.46 | 792.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.093 | 0.148 | 0.67 | 538.5 |
| render/acquire/ms | 0.089 | 0.120 | 0.40 | 518.2 |
| controller/move/ms | 0.083 | 0.271 | 0.70 | 484.0 |
| controller/write/ms | 0.074 | 0.099 | 0.38 | 428.8 |
| terrain/update/ms | 0.062 | 0.297 | 1.94 | 358.8 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.074 | 0.35 | 275.0 |
| physics/step/ms | 0.043 | 0.080 | 0.75 | 250.3 |
| stage/render/phase_sort | 0.040 | 0.066 | 0.32 | 232.3 |
| encode/point_shadow_cache_copies/sum | 0.032 | 0.087 | 0.41 | 186.9 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.58 | 180.8 |
| physics/insert/ms | 0.031 | 0.354 | 0.76 | 179.4 |
| pipelines/compile_ms | 0.030 | 0.000 | 83.13 | 175.9 |
| pipelines/front_ms | 0.029 | 0.000 | 80.64 | 170.3 |
| stage/render/cleanup | 0.025 | 0.039 | 0.78 | 147.6 |
| terrain/select/ms | 0.024 | 0.262 | 0.54 | 142.0 |
| weather/map/ms | 0.018 | 0.024 | 0.22 | 102.8 |
| far/select/ms | 0.016 | 0.170 | 0.68 | 95.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.17 | 74.7 |
| anim/apply/ms | 0.011 | 0.020 | 0.12 | 63.8 |
| weather/eval/ms | 0.011 | 0.014 | 0.10 | 62.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.34 | 62.3 |
| script/update/ms | 0.008 | 0.013 | 0.35 | 49.4 |
| controller/avoid/ms | 0.007 | 0.012 | 0.07 | 38.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.20 | 35.8 |
| script/ui/ms | 0.005 | 0.007 | 0.27 | 29.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 28.4 |
| stream/spawn/ms | 0.004 | 0.000 | 1.14 | 22.7 |
| stream/upload/ms | 0.003 | 0.000 | 1.35 | 20.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 13.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.12 | 6.0 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.85 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.20 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.04 | 2.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 34 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1277 | 2262 | 528 |
| crowd/c0/meshes/count | count | 352 | 646 | 135 |
| crowd/c1/count | count | 148 | 156 | 149 |
| crowd/c1/joints/count | count | 5647 | 5991 | 5668 |
| crowd/c1/meshes/count | count | 1203 | 1295 | 1206 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 205 |
| crowd/joints_extracted/count | count | 665 | 6956 | 419 |
| crowd/joints_uploaded/count | count | 3635 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.1 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.44 | 1.56 | 0.65 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.17 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.34 | 0.02 |
| encode/finish_critical | ms | 0.95 | 2.10 | 0.96 |
| encode/pass:early prepass/max | ms | 0.78 | 2.00 | 0.96 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 2.10 | 0.81 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.67 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.89 | 0.38 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 1.79 | 0.92 |
| encode/pass:shadow_point_light/max | ms | 0.11 | 1.48 | 0.06 |
| encode/point_shadow_cache_copies/max | ms | 0.06 | 0.41 | 0.05 |
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
| frame/work/ms | ms | 9.25 | 26.18 | 7.85 |
| gpu/timed/ms | ms | 8.15 | 10.58 | 7.84 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 10 | 32 | 0 |
| lamp_cache/copies | count | 2 | 16 | 0 |
| lamp_cache/deferred_faces | count | 0 | 16 | 0 |
| lamp_cache/lamps | count | 6 | 8 | 0 |
| lamp_cache/mover_faces | count | 2 | 12 | 0 |
| lamp_cache/movers | count | 113 | 312 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 6 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 16 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 7 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 12 | 31 | 0 |
| lamp_cache/stock_lamps | count | 0 | 2 | 0 |
| lamp_cache/store_starved | count | 0 | 6 | 0 |
| lamp_cache/store_used | count | 7 | 24 | 0 |
| lamp_cache/views | count | 8 | 31 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2605 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 167 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 681 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 381 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 167 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 681 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 381 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 763 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5843 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2470 | 3552 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 763 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5843 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2470 | 3552 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 26 | 102 | 8 |
| phases/shadow/bins | count | 760 | 1371 | 671 |
| phases/shadow/instance_batches | count | 4 | 10 | 4 |
| phases/shadow/instance_draws | count | 6657 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 106.5 k | 49.2 k |
| phases/shadow/items | count | 2231 | 3071 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 42 | 124 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 184 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 26 | 332 | 11 |
| preprocess/bind_groups_created | count | 21 | 326 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 8 | 5 |
| preprocess/bind_groups_reused | count | 128 | 263 | 123 |
| process/cpu_usage | % | 23.92 | 39.81 | 28.71 |
| process/mem_usage | GiB | 5.38 | 5.66 | 5.66 |
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
| skin/blocks_changed/count | count | 14 | 121 | 9 |
| skin/blocks_skipped/count | count | 147 | 206 | 196 |
| skin/blocks_walked/count | count | 14 | 29 | 9 |
| skin/joint_bytes_written/bytes | bytes | 47.4 k | 808.1 k | 32.0 k |
| skin/joints_written/count | count | 740 | 12.6 k | 500 |
| skin/upload_writes/count | count | 8 | 24 | 10 |
| started_load_count |  | 6463 | 6564 | 6564 |
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
| stream/latency/s | s | 0.04 | 0.10 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 277 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.49 | 40.61 | 34.00 |
| system/mem_usage | % | 56.56 | 57.34 | 57.33 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 83 | 161 | 66 |
| views/all/bins | count | 2924 | 3414 | 2961 |
| views/all/count | count | 13 | 40 | 3 |
| views/all/instance_batches | count | 12 | 18 | 12 |
| views/all/instance_draws | count | 20.0 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 204.8 k | 147.5 k |
| views/all/items | count | 8001 | 10.3 k | 7969 |
| views/all/pipelines | count | 91 | 166 | 66 |
| views/all/pipelines_distinct | count | 64 | 74 | 61 |
| views/all/work_items | count | 154.0 k | 214.6 k | 154.0 k |
| views/cascade/batch_sets | count | 11 | 34 | 8 |
| views/cascade/bins | count | 721 | 858 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6657 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2095 | 2454 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.8 k | 49.2 k |
| views/lamp_face/batch_sets | count | 6 | 70 | 0 |
| views/lamp_face/bins | count | 64 | 622 | 0 |
| views/lamp_face/count | count | 10 | 37 | 0 |
| views/lamp_face/culled | count | 0 | 12 | 0 |
| views/lamp_face/drawn | count | 3 | 26 | 0 |
| views/lamp_face/instance_batches | count | 0 | 2 | 0 |
| views/lamp_face/instance_draws | count | 0 | 26 | 0 |
| views/lamp_face/instance_slots | count | 0 | 8192 | 0 |
| views/lamp_face/items | count | 93 | 752 | 0 |
| views/lamp_face/pipelines | count | 17 | 97 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 25 | 8460 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2024 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 13.0 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5882 | 7524 | 5942 |
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
| world/meshes | count | 20.9 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 39.37 | 207 | 228.8 k |
| anim/events/count | count | 0.09 | 4 | 527 |
| assets/Image/added | count | 0.10 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.49 | 304 | 2874 |
| assets/Mesh/removed | count | 0.84 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.29 | 10 | 7476 |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.26 | 1154 | 1490 |
| encode//count | count | 52.44 | 134 | 304.8 k |
| encode/bind_walk/init_textures | count | 9797 | 24.0 k | 56.94 M |
| encode/bind_walk/init_textures_skipped | count | 43.5 k | 63.2 k | 252.76 M |
| encode/bind_walk/inits | count | 602 | 807 | 3.50 M |
| encode/bind_walk/inits_skipped | count | 132 | 228 | 768.5 k |
| encode/bind_walk/merge_textures | count | 9734 | 23.9 k | 56.58 M |
| encode/bind_walk/merge_textures_skipped | count | 11.9 k | 22.0 k | 69.34 M |
| encode/bind_walk/merges | count | 119 | 220 | 691.3 k |
| encode/bind_walk/merges_skipped | count | 127 | 226 | 736.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5812 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5812 |
| encode/pass:early prepass/count | count | 1 | 1 | 5812 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5812 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5812 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5812 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5812 |
| encode/pass:shadow_point_light/count | count | 6.12 | 26 | 35.5 k |
| encode/point_shadow_cache_copies/count | count | 0.50 | 1 | 2933 |
| encode/queue/command_buffers | count | 69.08 | 171 | 401.5 k |
| encode/queue/submits | count | 2.01 | 4 | 11.7 k |
| encode/queue/write_buffer/bytes | bytes | 2.34 M | 3.32 M | 13.62 G |
| encode/queue/write_buffer/calls | count | 283 | 526 | 1.65 M |
| encode/queue/write_buffer_with/bytes | bytes | 121.6 k | 1.25 M | 706.96 M |
| encode/queue/write_buffer_with/calls | count | 10.47 | 921 | 60.8 k |
| encode/queue/write_texture/bytes | bytes | 2191 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5812 |
| physics/inserts | count | 0.39 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 177 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
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
| stream/deactivate/items | count | 0.04 | 38 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5825 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1277 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.0 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2517.9 | main/busy/ms 2487, stage/main/post_update 2446, stage/main/update 30.74, stage/main/pre_update 6.29, stage/main/last 1.12, surface/advance_cpu_ms 0.71, stage/main/first 0.48, stage/main/fixed_main_loop 0.46, stream/drive/ms 0.40, surface/cell_upload_ms 0.30, stream/tick/ms 0.10, bigspace/total/ms 0.08 |
| warm-up | 3.04 | 520.9 | main/busy/ms 2618, stage/main/post_update 2477, render/thread/ms 478, render/busy/ms 395, stage/render/prepare_assets 364, stage/main/update 124, stage/render/prepare_views 80.85, render/acquire/ms 80.67, stage/render/create_views 15.09, stage/render/render 11.08, render/graph/ms 8.01, stage/main/pre_update 7.92 |
| warm-up | 3.10 | 62.4 | render/thread/ms 523, render/busy/ms 439, stage/render/prepare_assets 396, main/busy/ms 155, stage/main/update 99.47, stage/render/prepare_views 81.49, render/acquire/ms 81.19, pipelines/compile_ms 58.42, pipelines/front_ms 53.95, stage/main/post_update 42.92, terrain/update/ms 22.47, stage/render/render 15.97 |
| warm-up | 3.13 | 32.0 | pipelines/compile_ms 133, pipelines/front_ms 122, render/thread/ms 70.84, main/busy/ms 62.20, render/busy/ms 61.53, pipelines/wait_ms 60.94, stage/main/post_update 43.62, stage/render/prepare_assets 33.65, terrain/update/ms 29.68, stage/render/render 24.29, render/graph/ms 14.88, pipelines/driver_ms 11.80 |
| warm-up | 3.19 | 53.0 | pipelines/wait_ms 980, pipelines/compile_ms 557, pipelines/front_ms 513, render/thread/ms 66.18, main/busy/ms 64.48, render/busy/ms 56.18, stage/main/post_update 49.37, pipelines/driver_ms 44.78, stage/render/render 26.06, stage/render/prepare_assets 24.28, terrain/update/ms 20.41, render/graph/ms 14.90 |
| warm-up | 3.23 | 39.4 | pipelines/wait_ms 2783, pipelines/compile_ms 1059, pipelines/front_ms 993, main/busy/ms 95.73, render/thread/ms 77.38, render/busy/ms 73.49, pipelines/driver_ms 66.06, stage/main/post_update 63.30, stage/render/prepare_assets 45.86, stage/main/update 23.91, terrain/update/ms 19.49, stage/render/render 15.60 |
| warm-up | 3.32 | 98.6 | pipelines/wait_ms 3521, pipelines/compile_ms 994, pipelines/front_ms 947, main/busy/ms 118, render/thread/ms 99.79, render/busy/ms 97.04, stage/main/post_update 86.45, stage/render/prepare_assets 50.24, pipelines/driver_ms 46.98, stage/main/update 25.02, stage/render/render 21.34, render/graph/ms 18.03 |
| warm-up | 3.39 | 67.3 | pipelines/wait_ms 1943, pipelines/compile_ms 1250, pipelines/front_ms 1190, render/thread/ms 128, main/busy/ms 128, render/busy/ms 126, pipelines/driver_ms 60.54, stage/main/update 57.38, stage/main/post_update 55.77, stage/render/prepare_assets 39.71, stage/render/prepare_resources 30.90, stage/render/render 24.82 |
| warm-up | 3.49 | 95.0 | pipelines/compile_ms 1966, pipelines/front_ms 1872, pipelines/wait_ms 508, render/thread/ms 141, render/busy/ms 137, main/busy/ms 105, pipelines/driver_ms 93.28, stage/render/prepare_assets 73.15, stage/main/update 66.70, stage/main/post_update 23.27, stage/render/render 23.20, stage/render/prepare_resources 21.71 |
| warm-up | 3.66 | 95.9 | pipelines/compile_ms 834, pipelines/front_ms 783, pipelines/wait_ms 89.55, main/busy/ms 83.27, render/thread/ms 73.29, render/busy/ms 71.09, stage/main/update 64.59, pipelines/driver_ms 50.84, stage/render/prepare_assets 45.59, stage/render/render 18.16, render/graph/ms 15.31, stage/render/graph_submit 11.05 |
| warm-up | 3.71 | 49.1 | pipelines/compile_ms 1460, pipelines/front_ms 1370, pipelines/wait_ms 101, pipelines/driver_ms 90.06, render/thread/ms 83.86, render/busy/ms 82.00, stage/render/prepare_assets 59.55, main/busy/ms 53.75, stage/main/update 38.41, stage/render/render 16.23, render/graph/ms 13.69, stage/render/graph_submit 9.85 |
| warm-up | 3.76 | 44.5 | pipelines/compile_ms 942, pipelines/front_ms 893, render/thread/ms 55.40, render/busy/ms 54.02, pipelines/driver_ms 49.18, stage/render/prepare_assets 42.00, main/busy/ms 27.11, pipelines/wait_ms 13.93, stage/main/update 11.02, stage/render/render 8.78, render/graph/ms 7.28, stage/main/post_update 6.68 |
| warm-up | 3.82 | 56.8 | pipelines/compile_ms 507, pipelines/front_ms 487, render/thread/ms 60.01, stage/render/prepare_views 39.45, render/acquire/ms 39.33, main/busy/ms 20.23, pipelines/driver_ms 20.14, render/busy/ms 19.39, encode//sum 18.32, stage/render/render 14.41, render/graph/ms 12.55, stage/main/update 12.04 |
| warm-up | 3.88 | 38.2 | pipelines/compile_ms 224, pipelines/front_ms 215, main/busy/ms 42.57, render/thread/ms 39.03, render/busy/ms 37.04, stage/main/update 23.13, stage/render/render 15.30, stage/render/prepare_assets 14.10, stage/main/post_update 13.28, render/graph/ms 12.84, stream/drive/ms 9.42, pipelines/driver_ms 8.89 |
| warm-up | 3.93 | 51.5 | pipelines/compile_ms 829, pipelines/front_ms 796, render/thread/ms 61.75, render/busy/ms 59.77, main/busy/ms 35.30, pipelines/driver_ms 32.90, stage/render/prepare_assets 32.84, stage/render/render 17.24, stage/main/update 15.95, render/graph/ms 14.54, stage/main/post_update 11.30, stage/render/graph_submit 11.11 |
| warm-up | 3.99 | 58.8 | pipelines/compile_ms 1329, pipelines/front_ms 1275, main/busy/ms 133, stage/main/update 117, render/thread/ms 78.40, render/busy/ms 76.73, pipelines/driver_ms 54.22, stage/render/prepare_assets 42.39, stage/render/render 23.73, render/graph/ms 14.26, stage/render/graph_submit 10.65, stage/main/post_update 7.07 |
| warm-up | 4.16 | 169.6 | pipelines/compile_ms 2142, pipelines/front_ms 2081, main/busy/ms 128, stage/main/update 114, render/thread/ms 103, render/busy/ms 101, stage/render/prepare_assets 75.51, pipelines/driver_ms 61.33, stage/render/render 18.44, pipelines/wait_ms 10.51, render/graph/ms 9.33, pipelines/lock_ms 8.14 |
| warm-up | 4.20 | 37.7 | pipelines/compile_ms 1732, pipelines/front_ms 1688, render/thread/ms 97.68, render/busy/ms 96.09, stage/render/prepare_assets 60.67, main/busy/ms 46.03, pipelines/driver_ms 44.25, stage/render/render 17.13, stage/main/update 15.16, render/graph/ms 14.92, pipelines/wait_ms 11.09, stage/render/graph_submit 10.18 |
| warm-up | 4.28 | 83.2 | pipelines/compile_ms 133, pipelines/front_ms 127, render/thread/ms 116, render/busy/ms 114, main/busy/ms 76.58, stage/main/update 43.43, stage/render/prepare_write_phase_buffers 32.72, stage/render/phase_sort 21.95, stage/render/render 18.08, render/graph/ms 15.83, stage/main/post_update 11.45, stage/render/graph_submit 9.75 |
| warm-up | 4.35 | 70.8 | render/thread/ms 136, render/busy/ms 134, main/busy/ms 82.28, stage/main/update 39.80, stage/render/prepare_write_phase_buffers 33.28, stage/main/post_update 32.81, stage/render/prepare_assets 30.89, pipelines/compile_ms 28.25, pipelines/front_ms 26.91, stage/render/phase_sort 21.97, stage/render/render 11.89, stage/render/prepare_resources 10.93 |
| warm-up | 4.42 | 72.8 | pipelines/compile_ms 2929, pipelines/front_ms 2854, pipelines/wait_ms 971, render/thread/ms 112, render/busy/ms 110, main/busy/ms 85.18, pipelines/driver_ms 74.58, stage/render/prepare_assets 61.32, stage/main/post_update 37.66, stage/main/update 36.03, stream/drive/ms 19.76, stage/render/prepare_resources 14.85 |
| warm-up | 4.51 | 81.4 | pipelines/compile_ms 3567, pipelines/front_ms 3474, pipelines/wait_ms 1033, render/thread/ms 116, render/busy/ms 114, pipelines/driver_ms 93.25, main/busy/ms 87.21, stage/render/prepare_assets 70.13, stage/main/update 48.61, stage/main/post_update 23.02, stream/drive/ms 20.47, stage/render/render 17.77 |
| warm-up | 4.57 | 59.9 | pipelines/compile_ms 699, pipelines/front_ms 678, render/thread/ms 106, render/busy/ms 104, main/busy/ms 91.55, pipelines/wait_ms 89.15, stage/render/prepare_assets 55.00, stage/main/update 51.85, pipelines/driver_ms 21.40, stage/main/post_update 17.66, stage/render/render 15.65, stage/render/prepare_resources 15.46 |
| warm-up | 4.65 | 89.0 | pipelines/compile_ms 2111, pipelines/front_ms 2051, pipelines/wait_ms 689, render/thread/ms 106, render/busy/ms 104, main/busy/ms 76.01, pipelines/driver_ms 60.21, stage/render/prepare_assets 56.21, stage/main/update 46.48, stage/render/render 14.86, stage/render/prepare_resources 14.59, render/graph/ms 13.00 |
| warm-up | 4.77 | 90.1 | pipelines/compile_ms 951, pipelines/front_ms 930, main/busy/ms 109, stage/main/update 86.89, pipelines/wait_ms 66.50, stream/drive/ms 59.74, render/thread/ms 51.34, render/busy/ms 49.12, pipelines/driver_ms 21.67, stage/render/render 20.31, render/graph/ms 17.57, stage/render/prepare_assets 15.41 |
| warm-up | 4.80 | 32.4 | pipelines/compile_ms 689, pipelines/front_ms 675, main/busy/ms 70.83, stage/main/update 52.65, render/thread/ms 49.67, render/busy/ms 46.93, pipelines/wait_ms 43.15, stream/drive/ms 21.80, stage/render/render 20.07, render/graph/ms 16.74, stage/render/prepare_assets 14.75, pipelines/driver_ms 14.30 |
| warm-up | 4.86 | 53.4 | pipelines/compile_ms 970, pipelines/front_ms 938, main/busy/ms 56.10, render/thread/ms 53.28, render/busy/ms 50.92, stage/main/update 35.39, pipelines/driver_ms 32.66, stage/render/render 18.97, stage/render/prepare_assets 16.90, render/graph/ms 16.24, encode//sum 14.86, stage/main/post_update 12.09 |
| warm-up | 4.94 | 34.6 | pipelines/compile_ms 179, pipelines/front_ms 176, render/thread/ms 53.92, render/busy/ms 52.53, main/busy/ms 44.13, stage/main/update 21.13, stage/render/render 20.22, render/graph/ms 18.37, stage/main/post_update 16.81, encode//sum 13.83, stage/render/graph_submit 12.18, encode/submit 8.48 |
| warm-up | 5.16 | 30.7 | pipelines/compile_ms 536, pipelines/front_ms 521, main/busy/ms 36.34, render/thread/ms 30.17, render/busy/ms 28.77, stage/main/update 19.31, pipelines/driver_ms 14.85, stage/render/render 13.81, render/graph/ms 12.07, stream/drive/ms 11.27, encode//sum 11.23, stage/main/post_update 10.81 |
| warm-up | 5.42 | 29.2 | pipelines/compile_ms 608, pipelines/front_ms 592, main/busy/ms 39.25, render/thread/ms 27.96, render/busy/ms 26.58, stage/main/update 24.89, stream/drive/ms 17.31, pipelines/driver_ms 16.27, stage/render/render 14.49, render/graph/ms 12.52, stage/main/post_update 9.33, encode//sum 9.31 |
| warm-up | 5.82 | 28.2 | render/thread/ms 38.11, render/busy/ms 37.05, main/busy/ms 25.23, stage/render/prepare_assets 21.36, stage/main/update 11.99, stage/render/render 9.70, encode//sum 9.32, render/graph/ms 8.20, stage/main/post_update 7.31, stage/render/graph_submit 5.19, stream/drive/ms 3.53, encode/submit 3.17 |
| warm-up | 6.05 | 33.5 | render/thread/ms 48.52, render/busy/ms 47.11, main/busy/ms 41.06, encode//sum 25.10, stage/render/render 22.86, render/graph/ms 21.02, stage/main/update 19.64, stage/main/post_update 16.49, stage/render/graph_submit 14.04, encode/submit 8.26, encode/pass:shadow_point_light/sum 6.96, stage/render/camera_driver 6.83 |
| warm-up | 6.11 | 36.2 | render/thread/ms 49.16, render/busy/ms 48.11, main/busy/ms 39.59, stage/main/update 20.78, stage/render/prepare_assets 20.59, stage/render/render 13.19, encode//sum 12.82, render/graph/ms 11.67, stage/main/post_update 11.44, encode/pass:shadow_point_light/sum 8.45, stage/render/graph_submit 7.28, stage/render/prepare_meshes 5.27 |
| warm-up | 6.15 | 42.4 | render/thread/ms 67.66, render/busy/ms 66.42, main/busy/ms 47.08, stage/render/render 21.70, stage/render/prepare_assets 21.60, stage/main/update 21.47, render/graph/ms 19.50, encode//sum 19.32, stage/main/post_update 16.74, stage/render/graph_submit 13.04, encode/pass:shadow_point_light/sum 11.50, encode/submit 8.48 |
| warm-up | 6.21 | 32.5 | pipelines/compile_ms 148, pipelines/front_ms 144, render/thread/ms 46.28, render/busy/ms 45.04, main/busy/ms 31.46, encode//sum 26.47, stage/render/render 19.29, render/graph/ms 17.64, stage/main/update 15.88, stage/render/prepare_assets 13.61, stage/render/graph_submit 12.66, encode/pass:shadow_point_light/sum 10.56 |
| warm-up | 6.38 | 34.5 | main/busy/ms 52.97, render/thread/ms 47.31, render/busy/ms 45.90, stage/main/update 35.62, pipelines/compile_ms 32.25, pipelines/front_ms 31.27, stage/render/render 25.21, render/graph/ms 22.98, encode//sum 18.09, stage/render/graph_submit 16.45, stream/drive/ms 14.42, encode/submit 11.77 |
| warm-up | 6.42 | 40.8 | render/thread/ms 61.72, render/busy/ms 60.42, main/busy/ms 56.53, stage/main/update 33.65, stage/render/render 22.79, render/graph/ms 19.87, stage/render/prepare_assets 18.93, encode//sum 18.44, pipelines/compile_ms 18.33, pipelines/front_ms 17.88, stage/main/post_update 15.83, encode/pass:shadow_point_light/sum 13.58 |
| warm-up | 6.45 | 33.2 | render/thread/ms 61.34, render/busy/ms 60.03, main/busy/ms 47.16, stage/main/update 27.14, stage/render/prepare_assets 23.59, stage/render/render 20.62, render/graph/ms 18.04, encode//sum 15.63, stage/main/post_update 13.59, encode/pass:shadow_point_light/sum 11.36, stage/render/graph_submit 10.30, stage/render/camera_driver 7.57 |
| warm-up | 7.42 | 46.8 | render/thread/ms 55.23, render/busy/ms 54.12, pipelines/compile_ms 49.66, stage/render/render 48.18, render/graph/ms 46.49, stage/render/graph_submit 43.34, encode/submit 40.94, pipelines/driver_ms 35.95, main/busy/ms 18.55, pipelines/front_ms 13.71, encode//sum 9.84, stage/main/post_update 9.26 |
| measure | 18.39 | 28.4 | main/busy/ms 35.94, render/thread/ms 24.65, render/busy/ms 23.65, stage/main/update 22.83, stage/render/render 14.40, render/graph/ms 12.91, encode//sum 11.81, stage/render/graph_submit 9.30, stage/main/post_update 8.87, encode/submit 6.28, stage/render/prepare_assets 4.01, stage/render/camera_driver 3.48 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=cache-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1449 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **80%** (min 52%) · 2625 MHz (min 2625) · memory 8101 MHz · 75.8 W · 74 °C (max 79) · VRAM 6248 MiB · throttled 100% of samples

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
