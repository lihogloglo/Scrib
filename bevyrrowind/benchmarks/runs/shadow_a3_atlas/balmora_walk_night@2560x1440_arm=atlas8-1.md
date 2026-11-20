# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=8 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6633 frames over 65.6 s · **median 9.76 ms (102 fps)** · mean 9.90 · p95 11.60 · p99 13.40 (1% low 75 fps) · worst 34.30 ms · 31 spikes over 1.5× median · 32.0 stutters/min (work 24.7/min)
- warm-up: 355 frames over 9.3 s, worst 2482.6 ms
- hitches (> 25.0 ms): **2** measured, 48 in warm-up
- timed GPU passes: 6.41 ms of a 9.90 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.76 ms, limit 11 ms
- **FAIL** frame_ms: 35 stutters (> 1.5x moving median of 60) in 1.09 min = 31.99/min, limit 1/min
- **FAIL** worst frame 34.30 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.961 | 2.175 | 2.568 | 0.90 | 0.174 |
| main_opaque_pass_3d | 1.227 | 1.361 | 1.955 | 0.90 | 0.066 |
| early prepass | 0.479 | 0.531 | 0.769 | 0.90 | 0.091 |
| atmosphere_luts | 0.306 | 0.339 | 0.635 | 0.90 | 0.003 |
| bloom | 0.305 | 0.339 | 0.492 | 0.90 | 0.066 |
| water_velocity | 0.218 | 0.242 | 0.376 | 0.90 | 0.002 |
| water_sspr | 0.199 | 0.221 | 0.353 | 0.90 | 0.002 |
| ssao | 0.177 | 0.197 | 0.205 | 0.90 | 0.008 |
| upscaling | 0.124 | 0.138 | 0.156 | 0.90 | 0.004 |
| atmos_froxel_inject | 0.123 | 0.137 | 0.130 | 0.90 | 0.001 |
| point_shadow_atlas | 0.121 | 0.224 | 0.431 | 0.54 | 0.138 |
| water_fft | 0.115 | 0.127 | 0.126 | 0.90 | 0.003 |
| uniform_allocation | 0.111 | 0.123 | 0.141 | 0.90 | 0.022 |
| clouds_composite | 0.098 | 0.109 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.090 | 0.099 | 0.115 | 0.90 | 0.017 |
| gpu_instance_materialization | 0.086 | 0.096 | 0.099 | 0.90 | 0.004 |
| tonemapping | 0.084 | 0.093 | 0.094 | 0.90 | 0.004 |
| main_transmissive_pass_3d | 0.068 | 0.076 | 0.140 | 0.90 | 0.009 |
| clustering | 0.061 | 0.067 | 0.068 | 0.90 | 0.020 |
| early_mesh_preprocessing | 0.054 | 0.060 | 0.073 | 0.90 | 0.004 |
| clouds_trace | 0.050 | 0.056 | 0.079 | 0.90 | 0.001 |
| render_sky | 0.050 | 0.055 | 0.055 | 0.90 | 0.002 |
| clouds_resolve | 0.045 | 0.050 | 0.051 | 0.90 | 0.001 |
| surface_trail | 0.035 | 0.059 | 0.057 | 0.59 | 0.002 |
| lightprobe_irradiance_map | 0.033 | 0.464 | 0.769 | 0.07 | 0.001 |
| atmos_froxel_integrate | 0.028 | 0.031 | 0.032 | 0.90 | 0.001 |
| water_snapshot | 0.024 | 0.026 | 0.034 | 0.90 | 0.002 |
| range_unpacking | 0.020 | 0.023 | 0.028 | 0.90 | 0.003 |
| water_mips | 0.020 | 0.023 | 0.023 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.017 | 0.019 | 0.027 | 0.90 | 0.005 |
| point_shadow_atlas/early_mesh_preprocessing | 0.016 | 0.030 | 0.058 | 0.54 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.029 | 0.90 | 0.187 |
| lightprobe_radiance_map | 0.016 | 0.220 | 0.230 | 0.07 | 0.004 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.90 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.013 | 0.024 | 0.053 | 0.54 | 0.007 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.90 | 0.003 |
| point_shadow_atlas/bin_unpacking | 0.009 | 0.017 | 0.034 | 0.54 | 0.004 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.90 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.54 | 0.002 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.90 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.700 | 10.050 | 21.52 | 57707.1 |
| render/busy/ms | 8.157 | 9.431 | 20.49 | 54103.0 |
| main/busy/ms | 8.087 | 9.159 | 30.29 | 53643.1 |
| stage/render/render | 5.047 | 6.104 | 16.76 | 33480.0 |
| stage/main/post_update | 4.550 | 5.289 | 12.36 | 30179.6 |
| render/graph/ms | 4.353 | 5.211 | 12.74 | 28874.2 |
| encode//sum | 3.283 | 4.034 | 9.64 | 21775.6 |
| stage/render/graph_submit | 3.080 | 3.880 | 9.92 | 20432.3 |
| encode/submit | 1.986 | 2.870 | 5.93 | 13170.0 |
| stage/main/update | 1.862 | 2.323 | 24.41 | 12348.0 |
| stage/render/camera_driver | 1.209 | 1.434 | 2.70 | 8016.2 |
| encode/finish_wall | 1.183 | 1.514 | 4.18 | 7849.5 |
| encode/pass:early prepass/sum | 0.795 | 1.042 | 1.90 | 5270.2 |
| encode/pass:shadow_directional_light/sum | 0.794 | 1.200 | 3.51 | 5264.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.787 | 1.048 | 1.84 | 5220.0 |
| stage/render/prepare_write_phase_buffers | 0.581 | 0.861 | 2.43 | 3851.1 |
| bigspace/total/ms | 0.557 | 1.284 | 2.27 | 3692.4 |
| stage/render/prepare_assets | 0.528 | 0.665 | 12.90 | 3505.2 |
| stage/main/fixed_main_loop | 0.508 | 0.938 | 2.48 | 3368.9 |
| stage/render/prepare_meshes | 0.452 | 0.576 | 3.77 | 2998.5 |
| render/present/ms | 0.450 | 1.245 | 12.42 | 2988.1 |
| stage/render/prepare_resources | 0.423 | 0.524 | 4.09 | 2807.9 |
| stage/main/pre_update | 0.376 | 0.504 | 3.12 | 2493.9 |
| stage/main/first | 0.354 | 0.454 | 5.98 | 2347.6 |
| controller/feed/ms | 0.329 | 0.430 | 0.78 | 2181.4 |
| stage/render/prepare_bind_groups | 0.300 | 0.486 | 1.70 | 1990.8 |
| stage/render/prepare_flush | 0.298 | 0.522 | 1.65 | 1977.7 |
| stage/render/prepare_batch_phases | 0.276 | 0.424 | 0.94 | 1828.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.269 | 0.432 | 1.16 | 1781.2 |
| stage/main/last | 0.255 | 0.307 | 2.14 | 1688.3 |
| bigspace/low_precision/ms | 0.248 | 0.780 | 1.66 | 1641.9 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.233 | 0.922 | 2.07 | 1542.8 |
| stream/drive/ms | 0.209 | 0.290 | 10.93 | 1383.3 |
| stage/render/create_views | 0.207 | 0.285 | 0.60 | 1371.1 |
| stage/render/extract_commands | 0.187 | 0.224 | 0.58 | 1239.1 |
| anim/evaluate/ms | 0.169 | 0.306 | 1.50 | 1123.3 |
| bigspace/high_precision/ms | 0.161 | 0.677 | 1.52 | 1067.2 |
| anim/evaluate_cpu/ms | 0.160 | 0.200 | 0.52 | 1061.5 |
| stream/tick/ms | 0.154 | 0.222 | 0.54 | 1023.8 |
| stage/render/specialize | 0.153 | 0.245 | 1.68 | 1011.7 |
| stage/render/prepare_views | 0.136 | 0.180 | 1.12 | 904.6 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.104 | 0.877 | 1.41 | 689.7 |
| stage/render/queue | 0.096 | 0.165 | 1.53 | 634.5 |
| render/acquire/ms | 0.093 | 0.124 | 1.09 | 615.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.092 | 0.148 | 0.51 | 609.7 |
| stage/render/phase_sort | 0.083 | 0.114 | 0.38 | 548.1 |
| controller/move/ms | 0.075 | 0.220 | 0.45 | 497.2 |
| controller/write/ms | 0.073 | 0.099 | 0.37 | 485.1 |
| terrain/update/ms | 0.058 | 0.287 | 2.10 | 386.8 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.38 | 309.8 |
| physics/step/ms | 0.039 | 0.080 | 0.62 | 260.9 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.037 | 0.000 | 2.46 | 247.8 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.33 | 214.1 |
| pipelines/compile_ms | 0.026 | 0.000 | 83.03 | 172.5 |
| pipelines/front_ms | 0.025 | 0.000 | 80.36 | 166.9 |
| physics/insert/ms | 0.025 | 0.277 | 0.88 | 164.3 |
| stage/render/cleanup | 0.023 | 0.034 | 0.29 | 154.7 |
| terrain/select/ms | 0.021 | 0.254 | 0.66 | 140.8 |
| weather/map/ms | 0.017 | 0.024 | 0.26 | 114.8 |
| far/select/ms | 0.015 | 0.167 | 0.54 | 99.1 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.25 | 83.3 |
| anim/apply/ms | 0.012 | 0.022 | 0.30 | 80.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.07 | 71.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.29 | 69.9 |
| script/update/ms | 0.008 | 0.012 | 0.21 | 54.3 |
| controller/avoid/ms | 0.006 | 0.012 | 0.22 | 41.2 |
| stream/despawn/ms | 0.006 | 0.000 | 1.05 | 36.7 |
| script/input/ms | 0.005 | 0.007 | 0.19 | 35.0 |
| script/ui/ms | 0.004 | 0.005 | 0.05 | 24.7 |
| stream/spawn/ms | 0.003 | 0.000 | 1.27 | 22.9 |
| stream/upload/ms | 0.003 | 0.000 | 1.15 | 19.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 13.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 7.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.68 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 2 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1306 | 2262 | 641 |
| crowd/c0/meshes/count | count | 355 | 646 | 174 |
| crowd/c1/count | count | 145 | 155 | 147 |
| crowd/c1/joints/count | count | 5480 | 5902 | 5555 |
| crowd/c1/meshes/count | count | 1157 | 1267 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 206 |
| crowd/joints_extracted/count | count | 736 | 6965 | 750 |
| crowd/joints_uploaded/count | count | 3610 | 6220 | 5807 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.40 | 2.26 | 0.35 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.25 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.01 |
| encode/finish_critical | ms | 0.94 | 3.51 | 0.93 |
| encode/pass:early prepass/max | ms | 0.80 | 1.90 | 0.68 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.84 | 0.93 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.51 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.16 | 0.47 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.68 | 2.07 | 1.02 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.23 | 1.41 | 0.12 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.01 | 2.46 | 0.41 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 3.51 | 0.85 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.33 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.48 | 30.29 | 8.90 |
| gpu/timed/ms | ms | 7.93 | 9.97 | 8.44 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 153 | 386 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 153 | 386 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 767 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2488 | 3527 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 767 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2488 | 3527 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 28 | 196 | 27 |
| phases/shadow/bins | count | 745 | 1066 | 671 |
| phases/shadow/instance_batches | count | 4 | 14 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 98.3 k |
| phases/shadow/items | count | 2178 | 2756 | 2035 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 32 | 85 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2707 | 3182 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 182 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 266 | 43 |
| preprocess/bind_groups_created | count | 9 | 255 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 6 |
| preprocess/bind_groups_reused | count | 128 | 205 | 123 |
| process/cpu_usage | % | 25.49 | 44.98 | 24.43 |
| process/mem_usage | GiB | 5.15 | 5.41 | 5.41 |
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
| shadow_atlas/live_occ_pct | % | 37.50 | 99.22 | 0 |
| shadow_atlas/live_tiles | count | 16 | 37 | 0 |
| shadow_atlas/mover_draws | count | 8 | 12 | 0 |
| shadow_atlas/movers | count | 134 | 473 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 3 | 0 |
| shadow_atlas/render/blits | count | 8 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 17 | 0 |
| shadow_atlas/render/lamps | count | 8 | 9 | 0 |
| shadow_atlas/render/live_direct | count | 8 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 14 | 39 | 0 |
| shadow_atlas/resizes | count | 26 | 35 | 35 |
| shadow_atlas/static_occ_pct | % | 13.48 | 32.81 | 0 |
| shadow_atlas/static_tiles | count | 36 | 54 | 0 |
| shadow_atlas/views | count | 13 | 39 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 15 | 120 | 16 |
| skin/blocks_skipped/count | count | 144 | 204 | 190 |
| skin/blocks_walked/count | count | 15 | 27 | 16 |
| skin/joint_bytes_written/bytes | bytes | 51.7 k | 811.6 k | 53.2 k |
| skin/joints_written/count | count | 808 | 12.7 k | 832 |
| skin/upload_writes/count | count | 8 | 29 | 11 |
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
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.05 |
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
| system/cpu_usage | % | 31.66 | 41.56 | 35.33 |
| system/mem_usage | % | 55.17 | 56.13 | 56.13 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 85 | 251 | 85 |
| views/all/bins | count | 2868 | 3108 | 2961 |
| views/all/count | count | 17 | 42 | 3 |
| views/all/instance_batches | count | 12 | 22 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 196.6 k |
| views/all/items | count | 7996 | 10.1 k | 7977 |
| views/all/pipelines | count | 78 | 132 | 66 |
| views/all/pipelines_distinct | count | 63 | 71 | 61 |
| views/all/work_items | count | 154.1 k | 229.7 k | 205.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 27 |
| views/cascade/bins | count | 724 | 881 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2134 | 2434 | 2035 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 8 | 182 | 0 |
| views/lamp_face/bins | count | 24 | 337 | 0 |
| views/lamp_face/count | count | 14 | 39 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 55 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 31 | 483 | 0 |
| views/lamp_face/pipelines | count | 5 | 61 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 20 | 24.8 k | 0 |
| views/main/batch_sets | count | 57 | 61 | 58 |
| views/main/bins | count | 1992 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7492 | 5942 |
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
| anim/bones_written/count | count | 49.21 | 269 | 326.4 k |
| anim/events/count | count | 0.08 | 3 | 520 |
| assets/Image/added | count | 0.09 | 43 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 241 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.13 | 10 | 7473 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1154 | 1489 |
| encode//count | count | 27.91 | 30 | 185.1 k |
| encode/bind_walk/init_textures | count | 8864 | 21.5 k | 58.80 M |
| encode/bind_walk/init_textures_skipped | count | 50.0 k | 75.8 k | 331.73 M |
| encode/bind_walk/inits | count | 555 | 768 | 3.68 M |
| encode/bind_walk/inits_skipped | count | 145 | 232 | 962.6 k |
| encode/bind_walk/merge_textures | count | 8801 | 21.4 k | 58.38 M |
| encode/bind_walk/merge_textures_skipped | count | 10.2 k | 29.0 k | 67.75 M |
| encode/bind_walk/merges | count | 104 | 179 | 688.6 k |
| encode/bind_walk/merges_skipped | count | 132 | 226 | 878.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6633 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6633 |
| encode/pass:early prepass/count | count | 1 | 1 | 6633 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6633 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6633 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6633 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.32 | 1 | 2145 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.22 | 1 | 1441 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 239 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6633 |
| encode/queue/command_buffers | count | 38.50 | 42 | 255.3 k |
| encode/queue/submits | count | 2.01 | 4 | 13.3 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.41 M | 15.42 G |
| encode/queue/write_buffer/calls | count | 264 | 433 | 1.75 M |
| encode/queue/write_buffer_with/bytes | bytes | 126.8 k | 1.25 M | 840.91 M |
| encode/queue/write_buffer_with/calls | count | 10.29 | 732 | 68.2 k |
| encode/queue/write_texture/bytes | bytes | 1920 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6633 |
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
| stream/deactivate/items | count | 0.03 | 28 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5104 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1119 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 50, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2482.6 | main/busy/ms 2451, stage/main/post_update 2410, stage/main/update 31.53, stage/main/pre_update 6.49, stage/main/last 1.03, surface/advance_cpu_ms 0.73, stage/main/first 0.48, stream/drive/ms 0.44, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.30, bigspace/total/ms 0.10, stream/tick/ms 0.09 |
| warm-up | 3.27 | 785.4 | main/busy/ms 2600, stage/main/post_update 2436, render/thread/ms 749, render/busy/ms 586, stage/render/prepare_assets 535, stage/render/prepare_views 160, render/acquire/ms 160, stage/main/update 148, stage/render/create_views 29.75, stage/render/render 12.16, render/graph/ms 8.47, stage/main/pre_update 8.25 |
| warm-up | 3.33 | 62.6 | render/thread/ms 795, render/busy/ms 631, stage/render/prepare_assets 565, main/busy/ms 175, stage/render/prepare_views 161, render/acquire/ms 160, stage/main/update 123, pipelines/compile_ms 40.77, stage/main/post_update 39.45, pipelines/front_ms 39.05, stage/render/create_views 30.16, terrain/update/ms 19.34 |
| warm-up | 3.40 | 71.3 | pipelines/wait_ms 654, pipelines/compile_ms 507, pipelines/front_ms 473, render/thread/ms 116, render/busy/ms 112, main/busy/ms 92.40, stage/render/render 69.49, render/graph/ms 64.68, stage/main/post_update 56.49, stage/render/camera_driver 51.92, pipelines/driver_ms 33.74, stage/render/prepare_assets 31.17 |
| warm-up | 3.50 | 96.8 | pipelines/wait_ms 3930, pipelines/compile_ms 1201, pipelines/front_ms 1120, render/thread/ms 165, render/busy/ms 162, main/busy/ms 98.88, stage/render/render 88.47, render/graph/ms 84.96, pipelines/driver_ms 80.29, stage/main/post_update 59.18, stage/render/camera_driver 51.94, stage/render/prepare_resources 33.21 |
| warm-up | 3.58 | 77.6 | pipelines/wait_ms 5884, pipelines/compile_ms 1315, pipelines/front_ms 1227, render/thread/ms 171, render/busy/ms 170, main/busy/ms 99.76, pipelines/driver_ms 88.47, stage/render/prepare_assets 45.14, stage/render/render 42.87, render/graph/ms 40.79, stage/render/prepare_resources 38.94, stage/render/graph_submit 37.80 |
| warm-up | 3.62 | 42.4 | pipelines/wait_ms 2609, pipelines/compile_ms 938, pipelines/front_ms 879, render/thread/ms 112, render/busy/ms 110, main/busy/ms 84.33, stage/render/prepare_assets 62.19, pipelines/driver_ms 58.84, stage/main/update 30.66, stage/main/post_update 26.99, stage/render/render 23.37, render/graph/ms 21.33 |
| warm-up | 3.65 | 30.2 | pipelines/compile_ms 696, pipelines/front_ms 651, main/busy/ms 84.43, stage/main/update 62.73, render/thread/ms 59.45, render/busy/ms 57.49, pipelines/driver_ms 44.93, stage/render/prepare_assets 38.14, stage/main/post_update 15.31, stage/render/render 11.31, terrain/update/ms 10.25, render/graph/ms 9.10 |
| warm-up | 3.75 | 100.7 | pipelines/compile_ms 1718, pipelines/front_ms 1630, pipelines/wait_ms 183, pipelines/driver_ms 88.15, main/busy/ms 81.54, render/thread/ms 79.08, render/busy/ms 77.06, stage/main/update 62.77, stage/render/prepare_assets 57.35, stage/render/render 13.76, stage/main/post_update 13.38, render/graph/ms 11.46 |
| warm-up | 3.92 | 95.5 | pipelines/compile_ms 399, pipelines/front_ms 379, main/busy/ms 78.46, render/thread/ms 71.20, render/busy/ms 69.46, stage/main/update 60.34, stage/render/prepare_assets 43.93, pipelines/driver_ms 20.69, stage/render/render 16.09, render/graph/ms 13.65, stage/render/graph_submit 10.39, stage/main/post_update 9.44 |
| warm-up | 3.97 | 54.2 | pipelines/compile_ms 1083, pipelines/front_ms 1037, render/thread/ms 82.49, render/busy/ms 80.95, stage/render/prepare_assets 55.26, pipelines/driver_ms 46.66, main/busy/ms 41.14, stage/main/update 27.72, stage/render/render 17.88, render/graph/ms 15.91, stage/render/graph_submit 12.94, encode//sum 7.96 |
| warm-up | 4.01 | 34.2 | pipelines/compile_ms 1171, pipelines/front_ms 1131, render/thread/ms 54.10, render/busy/ms 52.88, pipelines/driver_ms 39.97, stage/render/prepare_assets 35.19, main/busy/ms 19.99, stage/render/render 13.45, render/graph/ms 11.98, stage/render/graph_submit 9.69, stage/main/update 9.58, pipelines/wait_ms 5.77 |
| warm-up | 4.06 | 46.7 | pipelines/compile_ms 276, pipelines/front_ms 265, render/thread/ms 52.16, render/busy/ms 26.84, stage/render/prepare_views 24.07, render/acquire/ms 23.95, main/busy/ms 21.24, stage/render/render 18.64, encode//sum 17.12, render/graph/ms 16.52, stage/render/graph_submit 13.34, stage/main/update 12.19 |
| warm-up | 4.12 | 33.9 | pipelines/compile_ms 245, pipelines/front_ms 236, main/busy/ms 45.13, render/thread/ms 40.21, render/busy/ms 37.85, stage/main/post_update 20.81, stage/main/update 19.44, stage/render/render 15.91, render/graph/ms 13.23, stage/render/prepare_assets 10.41, stream/drive/ms 9.91, pipelines/driver_ms 9.24 |
| warm-up | 4.18 | 52.7 | pipelines/compile_ms 890, pipelines/front_ms 857, main/busy/ms 128, stage/main/update 110, render/thread/ms 50.63, render/busy/ms 48.59, pipelines/driver_ms 33.64, stage/render/prepare_assets 24.75, stream/drive/ms 14.60, stage/render/render 13.81, render/graph/ms 11.09, stage/main/post_update 9.40 |
| warm-up | 4.38 | 204.4 | pipelines/compile_ms 1976, pipelines/front_ms 1918, main/busy/ms 150, stage/main/update 132, render/thread/ms 115, render/busy/ms 111, stage/render/prepare_assets 65.44, pipelines/driver_ms 58.17, stage/render/create_views 25.36, stream/drive/ms 15.78, stage/render/render 15.27, render/graph/ms 11.12 |
| warm-up | 4.48 | 99.0 | pipelines/compile_ms 1450, pipelines/front_ms 1404, render/thread/ms 166, render/busy/ms 163, main/busy/ms 113, stage/render/prepare_assets 105, stage/main/update 67.14, pipelines/driver_ms 45.88, pipelines/wait_ms 32.41, stage/main/post_update 25.81, stage/render/create_views 25.14, stage/render/render 17.43 |
| warm-up | 4.59 | 113.4 | pipelines/compile_ms 2484, pipelines/front_ms 2410, pipelines/wait_ms 525, render/thread/ms 182, render/busy/ms 180, main/busy/ms 123, stage/render/prepare_assets 87.74, pipelines/driver_ms 73.51, stage/main/update 46.02, stage/main/post_update 43.69, stage/render/prepare_flush 30.41, stage/main/pre_update 20.84 |
| warm-up | 4.66 | 65.9 | pipelines/compile_ms 2794, pipelines/front_ms 2719, pipelines/wait_ms 841, render/thread/ms 162, render/busy/ms 160, main/busy/ms 101, pipelines/driver_ms 74.49, stage/render/prepare_flush 56.33, stage/main/update 36.80, stage/render/prepare_assets 31.93, stage/main/post_update 31.37, stage/render/prepare_resources 22.19 |
| warm-up | 4.78 | 127.2 | pipelines/compile_ms 1742, pipelines/front_ms 1692, pipelines/wait_ms 936, render/thread/ms 179, render/busy/ms 177, main/busy/ms 109, pipelines/driver_ms 50.27, stage/render/prepare_resources 42.63, stage/main/post_update 41.35, stage/main/update 37.51, stage/render/prepare_flush 29.26, stage/main/fixed_main_loop 26.86 |
| warm-up | 4.82 | 37.7 | pipelines/compile_ms 2446, pipelines/front_ms 2373, pipelines/wait_ms 860, render/thread/ms 147, render/busy/ms 145, main/busy/ms 76.41, pipelines/driver_ms 72.96, stage/main/post_update 39.01, stage/render/prepare_assets 36.14, stage/render/prepare_resources 30.54, stage/render/prepare_write_phase_buffers 25.85, stage/render/prepare_meshes 23.92 |
| warm-up | 4.90 | 73.7 | pipelines/compile_ms 1607, pipelines/front_ms 1558, pipelines/wait_ms 271, render/thread/ms 81.52, render/busy/ms 79.86, main/busy/ms 58.79, pipelines/driver_ms 49.65, stage/render/prepare_assets 49.28, stage/main/update 27.01, stage/main/post_update 19.11, stage/render/render 15.65, render/graph/ms 12.96 |
| warm-up | 4.93 | 36.7 | pipelines/compile_ms 1205, pipelines/front_ms 1168, pipelines/wait_ms 81.09, render/thread/ms 77.08, render/busy/ms 75.50, main/busy/ms 53.65, stage/render/prepare_assets 44.09, pipelines/driver_ms 37.24, stage/main/update 25.53, stage/render/render 16.68, stage/main/post_update 16.24, render/graph/ms 14.23 |
| warm-up | 4.98 | 44.5 | pipelines/compile_ms 810, pipelines/front_ms 782, pipelines/wait_ms 82.83, main/busy/ms 79.43, stage/main/update 59.21, render/thread/ms 57.51, render/busy/ms 55.40, pipelines/driver_ms 27.82, stage/render/prepare_assets 24.58, stage/render/render 19.30, render/graph/ms 16.69, stream/drive/ms 14.63 |
| warm-up | 5.07 | 96.0 | pipelines/compile_ms 749, pipelines/front_ms 727, main/busy/ms 132, stage/main/update 112, render/thread/ms 81.04, render/busy/ms 78.82, stream/drive/ms 66.15, stage/render/prepare_assets 33.36, stage/render/render 25.46, pipelines/driver_ms 22.82, render/graph/ms 22.71, stage/render/graph_submit 16.97 |
| warm-up | 5.12 | 49.1 | pipelines/compile_ms 1246, pipelines/front_ms 1212, main/busy/ms 90.17, render/thread/ms 73.68, render/busy/ms 72.01, stage/main/update 71.48, stream/drive/ms 56.27, pipelines/driver_ms 34.15, stage/render/prepare_assets 26.20, stage/render/render 19.85, render/graph/ms 17.71, stage/render/graph_submit 13.67 |
| warm-up | 5.16 | 39.2 | pipelines/compile_ms 627, pipelines/front_ms 609, main/busy/ms 79.80, render/thread/ms 58.07, render/busy/ms 53.45, stage/main/update 35.82, pipelines/driver_ms 17.99, stage/render/render 17.00, render/graph/ms 15.12, stage/main/post_update 13.82, stage/render/prepare_assets 12.89, stage/render/graph_submit 11.46 |
| warm-up | 5.23 | 68.4 | main/busy/ms 85.83, render/thread/ms 66.13, render/busy/ms 61.71, pipelines/compile_ms 50.71, pipelines/front_ms 48.51, stage/main/update 38.38, stage/render/prepare_assets 20.39, stage/main/post_update 18.51, stage/render/render 17.64, render/graph/ms 15.71, stage/render/graph_submit 11.94, stage/main/first 11.20 |
| warm-up | 5.27 | 40.2 | pipelines/compile_ms 583, pipelines/front_ms 563, render/thread/ms 68.92, render/busy/ms 67.70, main/busy/ms 58.02, stage/main/update 33.16, pipelines/driver_ms 19.84, stage/render/prepare_assets 16.79, stage/main/post_update 16.54, stage/render/prepare_resources 15.13, stage/render/render 12.98, render/graph/ms 11.18 |
| warm-up | 5.31 | 40.4 | pipelines/compile_ms 1007, pipelines/front_ms 978, render/thread/ms 70.08, render/busy/ms 68.69, main/busy/ms 57.01, pipelines/driver_ms 29.22, stage/main/update 28.59, stage/main/post_update 19.42, stage/render/prepare_assets 17.24, stage/render/prepare_resources 15.73, stage/render/render 12.81, render/graph/ms 10.97 |
| warm-up | 5.34 | 30.7 | pipelines/compile_ms 1703, pipelines/front_ms 1662, render/thread/ms 52.34, render/busy/ms 50.92, main/busy/ms 42.73, pipelines/driver_ms 41.26, stage/render/prepare_assets 20.00, stage/main/update 18.02, stage/main/post_update 16.68, stage/render/render 11.30, render/graph/ms 9.36, encode//sum 6.68 |
| warm-up | 5.37 | 28.0 | pipelines/compile_ms 1874, pipelines/front_ms 1829, pipelines/driver_ms 44.88, render/thread/ms 41.34, render/busy/ms 38.62, main/busy/ms 33.27, pipelines/wait_ms 29.51, stage/main/update 16.67, stage/render/prepare_assets 15.23, stage/render/render 12.95, render/graph/ms 9.75, stage/main/post_update 9.44 |
| warm-up | 5.46 | 27.1 | pipelines/compile_ms 109, pipelines/front_ms 103, render/thread/ms 40.60, render/busy/ms 39.18, main/busy/ms 36.65, stage/main/update 20.75, stage/render/render 14.39, stream/drive/ms 12.40, render/graph/ms 12.30, stage/main/post_update 9.08, stage/render/graph_submit 8.97, stage/render/prepare_write_phase_buffers 8.95 |
| warm-up | 5.69 | 27.3 | pipelines/compile_ms 341, pipelines/front_ms 328, render/thread/ms 34.21, render/busy/ms 32.74, main/busy/ms 32.13, stage/main/update 16.67, stage/render/render 15.00, pipelines/driver_ms 13.60, render/graph/ms 13.00, stage/main/post_update 10.46, encode//sum 9.30, stage/render/graph_submit 9.08 |
| warm-up | 5.77 | 27.7 | pipelines/compile_ms 59.72, pipelines/front_ms 56.20, main/busy/ms 37.91, render/thread/ms 28.38, render/busy/ms 27.14, stage/main/update 22.70, stream/drive/ms 15.82, stage/render/render 12.10, render/graph/ms 10.33, stage/main/post_update 10.08, encode//sum 7.76, stage/render/graph_submit 6.94 |
| warm-up | 5.91 | 26.1 | pipelines/compile_ms 167, pipelines/front_ms 160, render/thread/ms 37.27, render/busy/ms 35.95, main/busy/ms 29.97, stage/main/post_update 14.59, stage/render/prepare_assets 13.16, stage/render/render 12.39, render/graph/ms 10.46, stage/main/update 10.20, encode//sum 7.98, pipelines/driver_ms 7.29 |
| warm-up | 6.06 | 32.9 | pipelines/compile_ms 196, pipelines/front_ms 191, main/busy/ms 39.65, stage/main/update 26.01, render/thread/ms 25.78, render/busy/ms 24.58, stream/drive/ms 16.70, stage/render/render 11.49, render/graph/ms 9.68, stage/main/post_update 8.38, encode//sum 8.33, stage/render/graph_submit 6.76 |
| warm-up | 6.18 | 26.9 | pipelines/compile_ms 160, pipelines/front_ms 155, render/thread/ms 43.13, render/busy/ms 41.47, main/busy/ms 39.79, stage/main/update 18.19, stage/render/render 17.90, render/graph/ms 15.65, stage/main/post_update 15.41, stage/render/prepare_assets 11.12, stage/render/graph_submit 11.01, encode//sum 10.32 |
| warm-up | 6.21 | 33.1 | render/thread/ms 47.45, render/busy/ms 46.21, main/busy/ms 37.63, stage/render/prepare_assets 19.25, stage/main/update 18.01, stage/render/render 13.29, stage/main/post_update 12.68, render/graph/ms 11.27, encode//sum 10.45, stage/render/graph_submit 7.15, stream/drive/ms 5.50, stage/render/camera_driver 3.94 |
| warm-up | 6.32 | 25.5 | render/thread/ms 33.06, main/busy/ms 32.16, render/busy/ms 32.03, stage/main/update 19.97, stage/render/render 10.34, stage/render/prepare_assets 9.46, render/graph/ms 8.95, stage/main/post_update 7.93, encode//sum 6.65, stage/render/graph_submit 6.10, encode/submit 3.50, stage/render/prepare_meshes 3.44 |
| warm-up | 6.41 | 27.3 | render/thread/ms 39.84, main/busy/ms 38.35, render/busy/ms 37.60, stage/main/update 23.88, stage/render/render 14.43, render/graph/ms 11.61, stage/main/post_update 10.05, stage/render/prepare_assets 9.78, stage/render/graph_submit 7.50, encode//sum 7.13, stage/render/prepare_meshes 4.92, encode/submit 4.89 |
| warm-up | 6.43 | 26.6 | main/busy/ms 43.69, render/thread/ms 43.41, render/busy/ms 42.06, stage/main/update 22.92, stage/render/render 17.05, render/graph/ms 15.08, stage/main/post_update 13.72, stage/render/prepare_assets 11.35, stage/render/graph_submit 10.68, encode//sum 8.49, encode/submit 7.45, stage/render/prepare_meshes 4.69 |
| warm-up | 6.46 | 31.9 | main/busy/ms 44.56, render/thread/ms 44.32, render/busy/ms 43.02, stage/main/update 24.37, stage/render/render 16.85, render/graph/ms 14.96, stage/main/post_update 12.98, stage/render/prepare_assets 10.69, stage/render/graph_submit 10.47, encode//sum 8.92, encode/submit 7.13, stream/drive/ms 5.05 |
| warm-up | 6.50 | 30.4 | render/thread/ms 45.37, render/busy/ms 44.37, main/busy/ms 39.94, stage/main/update 21.02, stage/render/prepare_assets 15.39, stage/main/post_update 11.11, stage/render/render 9.58, render/graph/ms 7.98, stage/render/prepare_meshes 6.86, encode//sum 6.60, stage/render/graph_submit 4.49, stream/drive/ms 4.24 |
| warm-up | 6.53 | 34.8 | pipelines/compile_ms 155, pipelines/front_ms 149, render/thread/ms 55.94, render/busy/ms 54.26, main/busy/ms 47.72, stage/main/update 22.27, stage/main/post_update 17.14, stage/render/prepare_assets 16.07, stage/render/render 13.75, render/graph/ms 11.27, encode//sum 9.51, stage/render/prepare_meshes 8.52 |
| warm-up | 6.58 | 31.5 | render/thread/ms 38.27, render/busy/ms 37.25, main/busy/ms 32.37, stage/main/update 17.04, stage/render/prepare_assets 13.39, stage/main/post_update 9.31, stage/render/render 9.00, render/graph/ms 7.40, encode//sum 7.15, stage/render/prepare_meshes 4.72, stage/render/graph_submit 4.58, stream/drive/ms 3.70 |
| warm-up | 6.73 | 47.9 | pipelines/compile_ms 136, pipelines/front_ms 132, main/busy/ms 58.87, stage/main/update 42.58, render/thread/ms 41.64, render/busy/ms 40.06, stage/render/render 16.16, stream/drive/ms 15.86, render/graph/ms 13.87, stage/main/post_update 11.56, stage/render/graph_submit 10.29, stage/render/prepare_assets 9.68 |
| warm-up | 6.77 | 35.1 | render/thread/ms 49.36, render/busy/ms 48.05, main/busy/ms 46.65, stage/main/update 27.01, stage/render/prepare_assets 17.41, stage/render/render 14.42, render/graph/ms 12.41, stage/main/post_update 11.77, encode//sum 9.13, stage/render/graph_submit 8.91, stage/render/prepare_meshes 6.85, stream/drive/ms 6.62 |
| measure | 18.29 | 32.8 | main/busy/ms 40.02, stage/main/update 26.45, render/thread/ms 21.10, render/busy/ms 19.90, stage/render/render 10.23, stage/main/post_update 8.96, render/graph/ms 8.69, stage/render/graph_submit 6.11, encode//sum 5.75, encode/submit 3.79, stage/render/prepare_assets 3.73, stage/render/camera_driver 2.44 |
| measure | 59.13 | 34.3 | render/thread/ms 38.73, render/busy/ms 37.62, main/busy/ms 22.30, stage/render/prepare_assets 17.66, stage/render/render 10.40, stage/main/post_update 9.65, render/graph/ms 9.14, encode//sum 8.19, stage/render/graph_submit 6.05, stage/main/fixed_main_loop 4.03, encode/submit 3.77, stage/main/update 3.58 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=8 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=atlas8-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1512 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **87%** (min 66%) · 2625 MHz (min 2625) · memory 8101 MHz · 83.4 W · 75 °C (max 79) · VRAM 6474 MiB · throttled 100% of samples

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
