# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.max_lamps=16 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6666 frames over 65.6 s · **median 9.75 ms (103 fps)** · mean 9.85 · p95 11.46 · p99 13.11 (1% low 76 fps) · worst 32.60 ms · 27 spikes over 1.5× median · 25.6 stutters/min (work 22.8/min)
- warm-up: 355 frames over 8.9 s, worst 2389.2 ms
- hitches (> 25.0 ms): **2** measured, 44 in warm-up
- timed GPU passes: 6.50 ms of a 9.85 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.75 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 32.60 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.866 | 2.035 | 2.068 | 0.92 | 0.176 |
| main_opaque_pass_3d | 1.506 | 1.642 | 2.376 | 0.92 | 0.064 |
| early prepass | 0.472 | 0.515 | 0.754 | 0.92 | 0.087 |
| bloom | 0.291 | 0.318 | 0.325 | 0.92 | 0.063 |
| atmosphere_luts | 0.287 | 0.313 | 0.377 | 0.92 | 0.003 |
| water_velocity | 0.206 | 0.225 | 0.247 | 0.92 | 0.002 |
| water_sspr | 0.185 | 0.202 | 0.203 | 0.92 | 0.002 |
| ssao | 0.173 | 0.188 | 0.193 | 0.92 | 0.007 |
| point_shadow_atlas | 0.134 | 0.245 | 0.470 | 0.55 | 0.232 |
| upscaling | 0.119 | 0.130 | 0.139 | 0.92 | 0.004 |
| atmos_froxel_inject | 0.117 | 0.128 | 0.129 | 0.92 | 0.002 |
| water_fft | 0.115 | 0.125 | 0.128 | 0.92 | 0.003 |
| uniform_allocation | 0.108 | 0.118 | 0.142 | 0.92 | 0.021 |
| clouds_composite | 0.096 | 0.105 | 0.109 | 0.92 | 0.002 |
| bin_unpacking | 0.085 | 0.093 | 0.113 | 0.92 | 0.016 |
| gpu_instance_materialization | 0.082 | 0.090 | 0.094 | 0.92 | 0.004 |
| tonemapping | 0.080 | 0.087 | 0.092 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.072 | 0.144 | 0.92 | 0.008 |
| clustering | 0.059 | 0.064 | 0.067 | 0.92 | 0.019 |
| clouds_trace | 0.051 | 0.055 | 0.078 | 0.92 | 0.001 |
| early_mesh_preprocessing | 0.050 | 0.055 | 0.071 | 0.92 | 0.004 |
| render_sky | 0.049 | 0.053 | 0.054 | 0.92 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.036 | 0.414 | 0.417 | 0.09 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.60 | 0.002 |
| atmos_froxel_integrate | 0.026 | 0.029 | 0.030 | 0.92 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.026 | 0.047 | 0.066 | 0.55 | 0.010 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.92 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.92 | 0.003 |
| lightprobe_radiance_map | 0.019 | 0.220 | 0.230 | 0.09 | 0.004 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.92 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.016 | 0.031 | 0.92 | 0.181 |
| point_shadow_atlas/uniform_allocation | 0.014 | 0.025 | 0.059 | 0.55 | 0.007 |
| point_shadow_atlas/bin_unpacking | 0.010 | 0.018 | 0.038 | 0.55 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.92 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.92 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.55 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.653 | 9.966 | 19.00 | 57681.6 |
| render/busy/ms | 8.134 | 9.444 | 18.49 | 54221.7 |
| main/busy/ms | 8.014 | 9.217 | 29.51 | 53424.2 |
| stage/render/render | 5.057 | 5.979 | 10.90 | 33710.7 |
| stage/main/post_update | 4.520 | 5.365 | 8.58 | 30131.3 |
| render/graph/ms | 4.386 | 5.192 | 9.74 | 29237.4 |
| encode//sum | 3.204 | 3.955 | 12.10 | 21356.9 |
| stage/render/graph_submit | 3.091 | 3.805 | 7.00 | 20603.1 |
| encode/submit | 1.939 | 2.744 | 4.64 | 12925.7 |
| stage/main/update | 1.815 | 2.277 | 23.45 | 12095.7 |
| encode/finish_wall | 1.235 | 1.603 | 2.88 | 8229.8 |
| stage/render/camera_driver | 1.234 | 1.490 | 2.74 | 8227.1 |
| encode/pass:early prepass/sum | 0.840 | 1.085 | 1.74 | 5596.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.832 | 1.085 | 1.91 | 5546.7 |
| encode/pass:shadow_directional_light/sum | 0.829 | 1.241 | 2.02 | 5526.0 |
| stage/render/prepare_write_phase_buffers | 0.583 | 0.889 | 1.76 | 3883.7 |
| bigspace/total/ms | 0.544 | 1.320 | 2.15 | 3627.9 |
| stage/main/fixed_main_loop | 0.506 | 0.950 | 2.50 | 3375.7 |
| stage/render/prepare_assets | 0.504 | 0.645 | 10.12 | 3358.6 |
| stage/render/prepare_meshes | 0.430 | 0.538 | 3.21 | 2865.0 |
| render/present/ms | 0.424 | 1.185 | 1.97 | 2828.2 |
| stage/render/prepare_resources | 0.413 | 0.493 | 1.53 | 2753.6 |
| stage/main/pre_update | 0.382 | 0.532 | 3.20 | 2545.7 |
| stage/main/first | 0.362 | 0.484 | 5.29 | 2415.1 |
| stage/render/prepare_flush | 0.320 | 0.523 | 1.08 | 2132.6 |
| controller/feed/ms | 0.317 | 0.397 | 0.80 | 2113.4 |
| stage/render/prepare_bind_groups | 0.303 | 0.486 | 1.41 | 2016.7 |
| stage/render/prepare_batch_phases | 0.271 | 0.410 | 1.00 | 1803.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.264 | 0.431 | 0.94 | 1762.6 |
| stage/main/last | 0.253 | 0.301 | 0.83 | 1686.9 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.235 | 1.074 | 2.06 | 1568.8 |
| bigspace/low_precision/ms | 0.229 | 0.750 | 1.71 | 1529.7 |
| stream/drive/ms | 0.198 | 0.278 | 10.03 | 1319.7 |
| stage/render/create_views | 0.196 | 0.267 | 0.60 | 1309.3 |
| stage/render/extract_commands | 0.186 | 0.217 | 0.74 | 1238.0 |
| bigspace/high_precision/ms | 0.170 | 0.712 | 1.63 | 1134.8 |
| anim/evaluate/ms | 0.158 | 0.284 | 1.63 | 1052.0 |
| stage/render/specialize | 0.152 | 0.245 | 1.69 | 1013.6 |
| anim/evaluate_cpu/ms | 0.150 | 0.189 | 0.79 | 1001.1 |
| stream/tick/ms | 0.145 | 0.213 | 0.43 | 966.8 |
| stage/render/prepare_views | 0.136 | 0.174 | 1.45 | 907.5 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.121 | 0.876 | 1.56 | 804.6 |
| stage/render/queue | 0.096 | 0.166 | 1.56 | 636.7 |
| render/acquire/ms | 0.095 | 0.122 | 1.41 | 631.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.089 | 0.141 | 0.57 | 594.6 |
| controller/move/ms | 0.079 | 0.245 | 0.76 | 529.2 |
| stage/render/phase_sort | 0.079 | 0.108 | 0.29 | 528.4 |
| controller/write/ms | 0.073 | 0.100 | 0.51 | 486.7 |
| terrain/update/ms | 0.058 | 0.288 | 1.42 | 389.9 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.053 | 0.000 | 2.74 | 355.1 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.31 | 307.7 |
| physics/step/ms | 0.041 | 0.091 | 0.50 | 274.6 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.57 | 212.3 |
| stage/render/cleanup | 0.024 | 0.032 | 0.51 | 161.8 |
| pipelines/compile_ms | 0.024 | 0.000 | 76.43 | 161.4 |
| physics/insert/ms | 0.024 | 0.262 | 0.88 | 156.9 |
| pipelines/front_ms | 0.023 | 0.000 | 73.90 | 155.7 |
| terrain/select/ms | 0.021 | 0.254 | 0.73 | 142.4 |
| weather/map/ms | 0.018 | 0.025 | 0.38 | 121.3 |
| far/select/ms | 0.014 | 0.166 | 0.45 | 94.7 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.44 | 85.0 |
| anim/apply/ms | 0.011 | 0.021 | 0.43 | 76.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.57 | 72.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.14 | 69.9 |
| script/update/ms | 0.008 | 0.012 | 0.05 | 52.6 |
| controller/avoid/ms | 0.006 | 0.012 | 0.40 | 38.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.00 | 36.2 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 33.0 |
| script/ui/ms | 0.003 | 0.005 | 0.06 | 22.2 |
| stream/spawn/ms | 0.003 | 0.000 | 1.10 | 21.8 |
| stream/upload/ms | 0.003 | 0.000 | 1.22 | 19.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 13.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.0 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.90 | 5.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.15 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
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
| controller/movers/count | count | 19 | 22 | 17 |
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
| crowd/joint_blocks/count | count | 160 | 216 | 204 |
| crowd/joints_extracted/count | count | 661 | 6981 | 430 |
| crowd/joints_uploaded/count | count | 3607 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.4 k | 47.7 k |
| encode//max | ms | 0.39 | 1.43 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.44 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.57 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.74 | 1.37 |
| encode/pass:early prepass/max | ms | 0.84 | 1.74 | 1.13 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.84 | 1.91 | 0.59 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.57 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.94 | 0.51 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.81 | 2.06 | 0.92 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.32 | 1.56 | 0.25 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.08 | 2.74 | 1.11 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 2.02 | 1.37 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.57 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.53 | 29.51 | 8.86 |
| gpu/timed/ms | ms | 7.86 | 9.42 | 8.67 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
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
| phases/alpha_mask3d_prepass/items | count | 318 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2479 | 3554 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2479 | 3554 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 74 | 427 | 27 |
| phases/shadow/bins | count | 839 | 1551 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 98.3 k |
| phases/shadow/items | count | 2312 | 3299 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 49 | 148 | 20 |
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
| preprocess/bind_groups | count | 15 | 267 | 43 |
| preprocess/bind_groups_created | count | 10 | 257 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 6 |
| preprocess/bind_groups_reused | count | 137 | 227 | 123 |
| process/cpu_usage | % | 25.96 | 40.72 | 36.68 |
| process/mem_usage | GiB | 5.17 | 5.42 | 5.42 |
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
| shadow_atlas/lamps | count | 8 | 17 | 0 |
| shadow_atlas/live_occ_pct | % | 4.88 | 29.49 | 0 |
| shadow_atlas/live_tiles | count | 17 | 60 | 0 |
| shadow_atlas/mover_draws | count | 10 | 37 | 0 |
| shadow_atlas/movers | count | 162 | 462 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 2 | 0 |
| shadow_atlas/render/blits | count | 10 | 37 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 17 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 37 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 57 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 105 | 0 |
| shadow_atlas/views | count | 17 | 57 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 122 | 9 |
| skin/blocks_skipped/count | count | 146 | 207 | 195 |
| skin/blocks_walked/count | count | 14 | 29 | 9 |
| skin/joint_bytes_written/bytes | bytes | 46.8 k | 808.6 k | 28.4 k |
| skin/joints_written/count | count | 732 | 12.6 k | 444 |
| skin/upload_writes/count | count | 7 | 26 | 9 |
| started_load_count |  | 6441 | 6565 | 6565 |
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
| stream/latency/s | s | 0.04 | 0.07 | 0.05 |
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
| system/cpu_usage | % | 29.37 | 42.02 | 28.69 |
| system/mem_usage | % | 46.48 | 47.19 | 47.19 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 133 | 482 | 85 |
| views/all/bins | count | 2934 | 3564 | 2948 |
| views/all/count | count | 20 | 60 | 3 |
| views/all/instance_batches | count | 12 | 22 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 196.6 k |
| views/all/items | count | 8012 | 10.6 k | 7956 |
| views/all/pipelines | count | 97 | 195 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 230.2 k | 205.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 27 |
| views/cascade/bins | count | 714 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2123 | 2427 | 2014 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 61 | 414 | 0 |
| views/lamp_face/bins | count | 98 | 714 | 0 |
| views/lamp_face/count | count | 17 | 57 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 138 | 878 | 0 |
| views/lamp_face/pipelines | count | 25 | 122 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 109 | 25.1 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5909 | 7528 | 5942 |
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
| anim/bones_written/count | count | 48.48 | 230 | 323.1 k |
| anim/events/count | count | 0.07 | 3 | 492 |
| assets/Image/added | count | 0.09 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 295 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7482 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1151 | 1489 |
| encode//count | count | 27.90 | 30 | 186.0 k |
| encode/bind_walk/init_textures | count | 9048 | 23.0 k | 60.32 M |
| encode/bind_walk/init_textures_skipped | count | 52.7 k | 70.7 k | 351.42 M |
| encode/bind_walk/inits | count | 562 | 771 | 3.75 M |
| encode/bind_walk/inits_skipped | count | 172 | 247 | 1.15 M |
| encode/bind_walk/merge_textures | count | 8986 | 22.9 k | 59.90 M |
| encode/bind_walk/merge_textures_skipped | count | 8756 | 18.6 k | 58.37 M |
| encode/bind_walk/merges | count | 104 | 178 | 692.0 k |
| encode/bind_walk/merges_skipped | count | 151 | 224 | 1.01 M |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6666 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6666 |
| encode/pass:early prepass/count | count | 1 | 1 | 6666 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6666 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6666 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6666 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.27 | 1 | 1830 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.25 | 1 | 1663 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.05 | 1 | 310 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6666 |
| encode/queue/command_buffers | count | 38.49 | 42 | 256.5 k |
| encode/queue/submits | count | 2.01 | 4 | 13.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.56 M | 15.46 G |
| encode/queue/write_buffer/calls | count | 271 | 448 | 1.81 M |
| encode/queue/write_buffer_with/bytes | bytes | 162.3 k | 1.29 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.28 | 894 | 68.5 k |
| encode/queue/write_texture/bytes | bytes | 1979 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6666 |
| physics/inserts | count | 0.34 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 175 | 2335 |
| physics/steps | count | 0.59 | 2 | 3939 |
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
| stream/deactivate/items | count | 0.03 | 31 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5078 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1113 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 46, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.39 | 2389.2 | main/busy/ms 2361, stage/main/post_update 2318, stage/main/update 31.67, stage/main/pre_update 7.04, stage/main/last 1.01, surface/advance_cpu_ms 0.83, stream/drive/ms 0.51, stage/main/first 0.49, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.35, stream/tick/ms 0.14, bigspace/total/ms 0.10 |
| warm-up | 3.11 | 722.7 | main/busy/ms 2497, stage/main/post_update 2343, render/thread/ms 690, render/busy/ms 551, stage/render/prepare_assets 503, stage/main/update 137, stage/render/prepare_views 136, render/acquire/ms 136, stage/render/create_views 28.83, stage/render/render 10.26, stage/main/pre_update 9.27, render/graph/ms 6.79 |
| warm-up | 3.21 | 102.2 | render/thread/ms 776, render/busy/ms 587, stage/render/prepare_assets 526, main/busy/ms 157, stage/render/prepare_views 137, render/acquire/ms 136, pipelines/compile_ms 123, stage/main/update 111, pipelines/front_ms 83.83, stage/render/render 65.62, render/present/ms 52.26, pipelines/driver_ms 39.53 |
| warm-up | 3.40 | 166.5 | pipelines/wait_ms 5202, pipelines/compile_ms 1751, pipelines/front_ms 1592, render/thread/ms 164, pipelines/driver_ms 159, render/busy/ms 159, encode//sum 81.80, main/busy/ms 74.70, stage/main/post_update 50.83, stage/render/render 39.55, stage/render/prepare_assets 38.89, stage/render/prepare_flush 35.63 |
| warm-up | 3.43 | 28.0 | pipelines/wait_ms 5423, pipelines/compile_ms 1834, pipelines/front_ms 1679, render/thread/ms 172, render/busy/ms 170, pipelines/driver_ms 155, main/busy/ms 123, encode//sum 85.11, stage/main/update 57.50, stage/main/post_update 47.13, stage/render/prepare_assets 44.55, stage/render/prepare_resources 36.25 |
| warm-up | 3.59 | 162.8 | pipelines/compile_ms 2398, pipelines/front_ms 2260, pipelines/wait_ms 538, pipelines/driver_ms 138, render/thread/ms 137, render/busy/ms 136, stage/render/prepare_assets 87.74, main/busy/ms 81.18, stage/main/update 56.99, stage/render/prepare_flush 25.07, terrain/update/ms 15.73, stage/main/post_update 15.45 |
| warm-up | 3.61 | 25.0 | pipelines/compile_ms 2351, pipelines/front_ms 2212, pipelines/wait_ms 320, pipelines/driver_ms 139, render/thread/ms 137, render/busy/ms 135, stage/render/prepare_assets 96.27, main/busy/ms 26.60, stage/render/prepare_flush 24.88, stage/main/post_update 12.16, stage/render/render 10.22, terrain/update/ms 9.94 |
| warm-up | 3.68 | 25.8 | pipelines/compile_ms 74.08, pipelines/front_ms 70.33, main/busy/ms 37.05, render/thread/ms 36.50, render/busy/ms 34.25, stage/main/post_update 20.02, stage/render/render 15.69, stage/main/update 13.19, render/graph/ms 12.94, stage/render/prepare_assets 11.56, stage/render/graph_submit 8.85, terrain/update/ms 8.50 |
| warm-up | 3.72 | 39.8 | pipelines/compile_ms 76.95, main/busy/ms 74.57, pipelines/front_ms 72.38, render/thread/ms 58.08, render/busy/ms 56.44, stage/main/update 55.77, stage/render/prepare_assets 23.56, stage/main/post_update 14.05, stage/render/render 10.82, render/graph/ms 8.81, terrain/update/ms 8.80, stage/render/prepare_resources 8.54 |
| warm-up | 3.81 | 88.9 | pipelines/compile_ms 1375, pipelines/front_ms 1319, render/thread/ms 78.05, render/busy/ms 76.40, main/busy/ms 72.31, stage/main/update 57.49, pipelines/driver_ms 56.40, stage/render/prepare_assets 40.53, stage/render/render 15.20, render/graph/ms 13.13, stage/render/graph_submit 10.49, encode//sum 10.03 |
| warm-up | 3.85 | 39.0 | pipelines/compile_ms 1798, pipelines/front_ms 1731, pipelines/driver_ms 66.64, render/thread/ms 64.31, render/busy/ms 62.87, stage/render/prepare_assets 44.21, main/busy/ms 23.29, stage/render/render 13.90, render/graph/ms 12.17, stage/render/graph_submit 9.71, encode//sum 9.37, stage/main/update 9.13 |
| warm-up | 3.92 | 34.6 | pipelines/compile_ms 260, pipelines/front_ms 250, main/busy/ms 36.75, render/thread/ms 36.72, render/busy/ms 34.97, stage/render/render 19.31, stage/main/update 18.47, render/graph/ms 16.84, stage/main/post_update 13.66, stage/render/graph_submit 11.90, pipelines/driver_ms 9.81, encode//sum 8.60 |
| warm-up | 3.97 | 50.5 | pipelines/compile_ms 245, pipelines/front_ms 236, render/thread/ms 67.93, render/busy/ms 66.08, main/busy/ms 40.00, stage/render/prepare_assets 28.09, stage/render/render 18.58, stage/main/update 16.18, render/graph/ms 15.90, stage/main/post_update 14.91, stage/render/graph_submit 11.42, pipelines/driver_ms 9.07 |
| warm-up | 4.02 | 46.6 | render/thread/ms 68.89, render/busy/ms 67.46, stage/render/prepare_assets 42.97, main/busy/ms 37.70, stage/main/update 16.24, stage/main/post_update 11.75, stage/render/render 10.52, render/graph/ms 8.46, stage/render/graph_submit 5.80, encode//sum 5.61, stage/render/prepare_bind_groups 5.03, stream/drive/ms 4.66 |
| warm-up | 4.08 | 56.2 | pipelines/compile_ms 1253, pipelines/front_ms 1215, main/busy/ms 144, stage/main/update 126, render/thread/ms 65.19, render/busy/ms 62.75, stage/render/prepare_assets 39.47, pipelines/driver_ms 38.65, stage/render/render 16.18, render/graph/ms 13.18, stage/main/post_update 10.34, stage/render/graph_submit 8.63 |
| warm-up | 4.24 | 166.3 | pipelines/compile_ms 2596, pipelines/front_ms 2519, main/busy/ms 159, stage/main/update 143, pipelines/wait_ms 98.97, render/thread/ms 93.80, render/busy/ms 91.00, pipelines/driver_ms 76.47, stage/render/prepare_assets 59.56, stage/render/render 20.19, render/graph/ms 16.81, stage/render/graph_submit 11.96 |
| warm-up | 4.28 | 38.3 | pipelines/compile_ms 1975, pipelines/front_ms 1915, pipelines/wait_ms 174, main/busy/ms 83.82, render/thread/ms 83.71, render/busy/ms 81.77, pipelines/driver_ms 59.65, stage/render/prepare_assets 52.62, stage/main/update 45.21, stage/main/post_update 20.08, stage/render/render 14.07, render/graph/ms 11.57 |
| warm-up | 4.36 | 78.3 | pipelines/compile_ms 708, pipelines/front_ms 683, pipelines/wait_ms 132, main/busy/ms 97.48, render/thread/ms 82.01, render/busy/ms 80.40, stage/main/update 34.25, stage/main/post_update 25.73, pipelines/driver_ms 24.87, stage/render/prepare_assets 20.28, stage/render/prepare_write_phase_buffers 17.44, stage/main/fixed_main_loop 14.29 |
| warm-up | 4.46 | 103.5 | pipelines/compile_ms 1941, pipelines/front_ms 1892, render/thread/ms 150, render/busy/ms 149, main/busy/ms 96.21, pipelines/wait_ms 80.79, stage/render/prepare_resources 56.76, pipelines/driver_ms 49.28, stage/main/update 41.13, stage/render/prepare_write_phase_buffers 40.48, stage/main/post_update 26.19, stage/render/prepare_assets 21.28 |
| warm-up | 4.51 | 42.4 | pipelines/compile_ms 2747, pipelines/front_ms 2677, pipelines/wait_ms 136, render/thread/ms 129, render/busy/ms 127, main/busy/ms 71.03, pipelines/driver_ms 69.96, stage/render/prepare_resources 50.68, stage/main/update 36.45, stage/render/prepare_write_phase_buffers 26.91, stage/render/prepare_assets 25.02, stage/main/post_update 22.48 |
| warm-up | 4.58 | 71.9 | pipelines/compile_ms 1394, pipelines/front_ms 1340, pipelines/wait_ms 337, main/busy/ms 84.52, render/thread/ms 81.00, render/busy/ms 78.74, stage/main/update 60.11, pipelines/driver_ms 53.26, stage/render/prepare_assets 41.52, stage/render/render 18.92, render/graph/ms 15.56, stage/main/post_update 14.55 |
| warm-up | 4.66 | 81.8 | pipelines/compile_ms 917, pipelines/front_ms 874, pipelines/wait_ms 418, render/thread/ms 93.73, render/busy/ms 91.73, main/busy/ms 86.56, stage/render/prepare_assets 62.41, stage/main/update 60.25, pipelines/driver_ms 43.10, stage/render/render 16.45, stage/main/post_update 14.53, render/graph/ms 13.62 |
| warm-up | 4.72 | 55.9 | pipelines/compile_ms 1124, pipelines/front_ms 1092, pipelines/wait_ms 194, render/thread/ms 87.47, render/busy/ms 85.77, main/busy/ms 50.84, stage/render/prepare_assets 49.88, pipelines/driver_ms 32.28, stage/main/update 22.00, stage/render/render 15.89, render/graph/ms 13.61, stage/main/post_update 10.44 |
| warm-up | 4.74 | 27.7 | pipelines/compile_ms 816, pipelines/front_ms 793, main/busy/ms 87.92, render/thread/ms 68.07, render/busy/ms 66.17, stage/main/update 60.73, stream/drive/ms 33.55, pipelines/driver_ms 23.21, stage/render/prepare_assets 21.95, stage/render/render 18.27, render/graph/ms 15.62, stage/render/graph_submit 11.50 |
| warm-up | 4.81 | 71.0 | pipelines/compile_ms 859, pipelines/front_ms 834, main/busy/ms 96.42, stage/main/update 76.51, stream/drive/ms 53.44, render/thread/ms 50.33, render/busy/ms 48.58, pipelines/driver_ms 24.91, stage/render/prepare_assets 16.10, stage/render/render 15.51, render/graph/ms 12.96, stage/main/post_update 11.48 |
| warm-up | 4.85 | 36.3 | pipelines/compile_ms 1310, pipelines/front_ms 1276, main/busy/ms 72.75, stage/main/update 56.54, render/thread/ms 45.28, render/busy/ms 43.78, pipelines/driver_ms 34.18, stream/drive/ms 31.14, stage/render/prepare_assets 15.19, stage/render/render 15.19, render/graph/ms 13.13, encode//sum 9.96 |
| warm-up | 4.90 | 48.6 | pipelines/compile_ms 935, pipelines/front_ms 909, main/busy/ms 52.20, render/thread/ms 44.08, render/busy/ms 42.67, stage/main/update 36.81, pipelines/driver_ms 25.50, pipelines/wait_ms 17.26, stage/render/prepare_assets 16.19, stage/render/render 14.73, render/graph/ms 12.77, stream/drive/ms 11.46 |
| warm-up | 4.95 | 32.8 | render/thread/ms 46.15, render/busy/ms 44.80, main/busy/ms 44.62, pipelines/compile_ms 41.03, pipelines/front_ms 39.65, stage/main/update 24.79, stage/render/render 14.36, stage/main/post_update 13.00, render/graph/ms 12.43, stage/render/prepare_write_phase_buffers 9.00, stage/render/graph_submit 8.64, encode//sum 6.94 |
| warm-up | 4.98 | 30.1 | render/thread/ms 51.56, render/busy/ms 50.31, main/busy/ms 41.13, stage/main/update 21.70, stage/render/prepare_assets 15.88, stage/main/post_update 13.46, stage/render/render 11.75, render/graph/ms 9.99, stage/render/prepare_write_phase_buffers 8.91, encode//sum 6.86, stage/render/graph_submit 6.77, encode/submit 4.42 |
| warm-up | 5.01 | 27.0 | pipelines/compile_ms 334, pipelines/front_ms 322, render/thread/ms 43.85, render/busy/ms 42.53, main/busy/ms 35.15, stage/render/prepare_assets 19.98, stage/main/update 17.24, stage/render/render 11.98, pipelines/driver_ms 11.55, stage/main/post_update 10.57, render/graph/ms 10.20, encode//sum 7.00 |
| warm-up | 5.09 | 42.0 | pipelines/compile_ms 795, pipelines/front_ms 770, render/thread/ms 51.98, render/busy/ms 49.67, main/busy/ms 44.79, pipelines/driver_ms 25.05, stage/main/update 15.12, stage/render/render 14.87, stage/main/post_update 14.03, render/graph/ms 11.98, stage/render/prepare_assets 10.69, stage/render/graph_submit 8.16 |
| warm-up | 5.20 | 29.7 | pipelines/compile_ms 216, pipelines/front_ms 212, main/busy/ms 40.03, render/thread/ms 30.35, render/busy/ms 29.15, stage/main/update 21.75, stream/drive/ms 12.35, stage/render/render 12.33, stage/main/post_update 11.99, render/graph/ms 10.46, encode//sum 7.70, stage/render/graph_submit 6.44 |
| warm-up | 5.23 | 26.3 | pipelines/compile_ms 750, pipelines/front_ms 730, render/thread/ms 39.85, render/busy/ms 38.36, main/busy/ms 38.31, pipelines/driver_ms 20.22, stage/main/update 17.70, stage/render/render 16.30, stage/main/post_update 14.38, render/graph/ms 14.07, stage/render/graph_submit 9.14, encode//sum 8.39 |
| warm-up | 5.49 | 30.0 | pipelines/compile_ms 596, pipelines/front_ms 581, main/busy/ms 40.38, render/thread/ms 26.72, render/busy/ms 25.55, stage/main/update 25.30, stream/drive/ms 18.59, pipelines/driver_ms 15.35, stage/render/render 12.35, render/graph/ms 10.84, stage/main/post_update 8.71, stage/render/graph_submit 7.81 |
| warm-up | 5.71 | 28.6 | main/busy/ms 35.04, stage/main/update 22.29, render/thread/ms 22.10, render/busy/ms 21.02, stream/drive/ms 14.95, stage/render/render 9.72, render/graph/ms 8.12, stage/main/post_update 7.83, encode//sum 6.71, stage/render/graph_submit 5.19, stage/render/prepare_assets 4.15, encode/submit 3.02 |
| warm-up | 5.87 | 28.1 | render/thread/ms 34.03, render/busy/ms 33.11, main/busy/ms 25.02, stage/render/prepare_assets 15.62, stage/main/update 11.10, pipelines/compile_ms 9.68, pipelines/front_ms 9.02, stage/render/render 8.46, stage/main/post_update 7.11, render/graph/ms 6.91, encode//sum 6.72, stage/render/graph_submit 4.52 |
| warm-up | 5.96 | 26.9 | main/busy/ms 32.92, render/thread/ms 32.33, render/busy/ms 31.31, stage/main/update 20.33, stage/render/render 9.25, stage/render/prepare_assets 8.79, render/graph/ms 7.79, stage/main/post_update 6.94, encode//sum 6.01, stage/render/graph_submit 5.28, stage/render/prepare_meshes 4.17, encode/submit 3.35 |
| warm-up | 6.09 | 27.0 | main/busy/ms 38.61, render/thread/ms 38.58, render/busy/ms 37.19, stage/main/update 21.51, stage/render/render 13.64, render/graph/ms 11.75, stage/main/post_update 11.39, stage/render/prepare_assets 9.41, encode//sum 8.50, stage/render/graph_submit 7.98, encode/submit 5.31, stream/drive/ms 4.17 |
| warm-up | 6.13 | 38.5 | render/thread/ms 49.70, render/busy/ms 48.75, main/busy/ms 39.13, stage/main/update 18.08, stage/render/prepare_assets 17.19, stage/render/render 13.67, stage/main/post_update 13.17, render/graph/ms 12.21, stage/render/graph_submit 9.37, encode/finish_wall 6.65, encode/pass:point_shadow_atlas_live_indirect/sum 6.05, stage/render/prepare_meshes 5.93 |
| warm-up | 6.16 | 37.3 | pipelines/compile_ms 140, pipelines/front_ms 132, render/thread/ms 65.46, render/busy/ms 63.90, main/busy/ms 49.06, stage/render/render 23.47, render/graph/ms 21.06, stage/main/post_update 20.37, stage/main/update 20.21, stage/render/prepare_assets 17.40, stage/render/graph_submit 16.15, encode/finish_wall 11.26 |
| warm-up | 6.22 | 34.6 | render/thread/ms 45.91, render/busy/ms 44.81, main/busy/ms 31.76, stage/render/prepare_assets 16.19, stage/main/update 15.35, stage/render/render 14.98, render/graph/ms 13.27, pipelines/compile_ms 12.22, pipelines/front_ms 11.60, stage/render/graph_submit 10.37, stage/main/post_update 10.13, encode/finish_wall 7.47 |
| warm-up | 6.37 | 35.3 | pipelines/compile_ms 128, pipelines/front_ms 124, main/busy/ms 45.02, render/thread/ms 35.73, render/busy/ms 34.46, stage/main/update 31.34, stream/drive/ms 13.87, stage/render/render 12.93, render/graph/ms 11.02, stage/render/prepare_assets 9.00, stage/main/post_update 8.43, stage/render/graph_submit 8.32 |
| warm-up | 6.41 | 36.7 | pipelines/compile_ms 128, pipelines/front_ms 124, render/thread/ms 52.06, render/busy/ms 51.01, main/busy/ms 48.95, stage/main/update 27.42, stage/render/render 17.18, stage/render/prepare_assets 16.35, stage/main/post_update 15.49, render/graph/ms 15.18, stage/render/graph_submit 10.53, encode//sum 10.19 |
| warm-up | 6.44 | 26.0 | render/thread/ms 51.30, render/busy/ms 50.26, main/busy/ms 46.97, stage/main/update 24.76, stage/render/render 18.01, stage/render/prepare_assets 17.61, stage/main/post_update 16.24, render/graph/ms 16.06, stage/render/graph_submit 11.41, encode//sum 9.38, encode/finish_wall 7.61, encode/pass:point_shadow_atlas_live_indirect/sum 7.04 |
| measure | 18.36 | 32.6 | main/busy/ms 38.41, stage/main/update 25.32, render/thread/ms 20.96, render/busy/ms 20.14, stage/render/render 10.50, render/graph/ms 9.32, stage/main/post_update 8.26, stage/render/graph_submit 5.95, encode//sum 5.81, stage/render/prepare_assets 3.83, encode/submit 3.60, stage/render/camera_driver 3.22 |
| measure | 59.11 | 28.5 | render/thread/ms 37.37, render/busy/ms 36.34, main/busy/ms 26.16, stage/render/prepare_assets 15.77, stage/main/post_update 13.33, stage/render/render 11.85, render/graph/ms 10.41, encode//sum 7.62, stage/render/graph_submit 6.46, stage/main/update 4.92, stage/render/camera_driver 3.76, encode/submit 3.64 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.max_lamps=16 --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=lamps16-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 3% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (256 samples): GPU load **86%** (min 59%) · 2619 MHz (min 2610) · memory 8101 MHz · 85.5 W · 80 °C (max 85) · VRAM 5521 MiB · throttled 100% of samples

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
