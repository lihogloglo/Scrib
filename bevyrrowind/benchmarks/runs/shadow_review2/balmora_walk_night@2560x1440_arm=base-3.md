# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6676 frames over 65.6 s · **median 9.73 ms (103 fps)** · mean 9.83 · p95 11.49 · p99 13.09 (1% low 76 fps) · worst 31.95 ms · 27 spikes over 1.5× median · 22.8 stutters/min (work 19.2/min)
- warm-up: 353 frames over 8.8 s, worst 2330.9 ms
- hitches (> 25.0 ms): **2** measured, 39 in warm-up
- timed GPU passes: 6.57 ms of a 9.83 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.73 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- pass worst frame 31.95 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.875 | 2.035 | 2.067 | 0.92 | 0.179 |
| main_opaque_pass_3d | 1.526 | 1.657 | 2.382 | 0.92 | 0.061 |
| early prepass | 0.476 | 0.517 | 0.752 | 0.92 | 0.088 |
| bloom | 0.293 | 0.318 | 0.325 | 0.92 | 0.064 |
| atmosphere_luts | 0.289 | 0.313 | 0.377 | 0.92 | 0.003 |
| water_velocity | 0.207 | 0.225 | 0.246 | 0.92 | 0.002 |
| water_sspr | 0.186 | 0.202 | 0.203 | 0.92 | 0.002 |
| ssao | 0.173 | 0.188 | 0.191 | 0.92 | 0.007 |
| point_shadow_atlas | 0.152 | 0.279 | 0.474 | 0.54 | 0.240 |
| upscaling | 0.118 | 0.128 | 0.135 | 0.92 | 0.004 |
| atmos_froxel_inject | 0.118 | 0.128 | 0.129 | 0.92 | 0.002 |
| water_fft | 0.113 | 0.123 | 0.126 | 0.92 | 0.003 |
| uniform_allocation | 0.107 | 0.117 | 0.141 | 0.92 | 0.020 |
| clouds_composite | 0.097 | 0.105 | 0.109 | 0.92 | 0.002 |
| gpu_instance_materialization | 0.089 | 0.096 | 0.100 | 0.92 | 0.004 |
| bin_unpacking | 0.085 | 0.093 | 0.113 | 0.92 | 0.015 |
| tonemapping | 0.080 | 0.087 | 0.092 | 0.92 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.071 | 0.148 | 0.92 | 0.008 |
| clustering | 0.059 | 0.064 | 0.068 | 0.92 | 0.019 |
| clouds_trace | 0.051 | 0.056 | 0.079 | 0.92 | 0.001 |
| early_mesh_preprocessing | 0.050 | 0.054 | 0.070 | 0.92 | 0.003 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.92 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.413 | 0.416 | 0.10 | 0.001 |
| surface_trail | 0.034 | 0.057 | 0.057 | 0.60 | 0.002 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.92 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.027 | 0.050 | 0.073 | 0.54 | 0.009 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.220 | 0.231 | 0.10 | 0.004 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.92 | 0.003 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.018 | 0.032 | 0.92 | 0.180 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.92 | 0.005 |
| point_shadow_atlas/uniform_allocation | 0.016 | 0.029 | 0.058 | 0.54 | 0.008 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.92 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.011 | 0.020 | 0.037 | 0.54 | 0.005 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.92 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.92 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.54 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.662 | 10.077 | 20.40 | 57824.7 |
| render/busy/ms | 8.124 | 9.537 | 19.93 | 54232.7 |
| main/busy/ms | 8.007 | 9.221 | 29.13 | 53456.5 |
| stage/render/render | 5.079 | 6.047 | 11.53 | 33907.3 |
| stage/main/post_update | 4.522 | 5.350 | 10.31 | 30187.1 |
| render/graph/ms | 4.396 | 5.277 | 10.11 | 29347.8 |
| encode//sum | 3.142 | 3.981 | 19.56 | 20975.3 |
| stage/render/graph_submit | 3.102 | 3.861 | 7.26 | 20708.9 |
| encode/submit | 1.972 | 2.774 | 5.78 | 13162.6 |
| stage/main/update | 1.811 | 2.284 | 23.33 | 12087.6 |
| stage/render/camera_driver | 1.233 | 1.507 | 3.07 | 8232.8 |
| encode/finish_wall | 1.219 | 1.604 | 4.28 | 8138.5 |
| encode/pass:early prepass/sum | 0.814 | 1.068 | 3.23 | 5437.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.811 | 1.064 | 3.34 | 5415.5 |
| encode/pass:shadow_directional_light/sum | 0.811 | 1.230 | 3.28 | 5415.1 |
| stage/render/prepare_write_phase_buffers | 0.580 | 0.873 | 2.02 | 3874.0 |
| bigspace/total/ms | 0.542 | 1.293 | 4.59 | 3617.6 |
| stage/main/fixed_main_loop | 0.500 | 0.935 | 2.05 | 3335.7 |
| stage/render/prepare_assets | 0.497 | 0.642 | 12.30 | 3315.1 |
| render/present/ms | 0.436 | 1.224 | 4.31 | 2913.1 |
| stage/render/prepare_meshes | 0.423 | 0.522 | 3.89 | 2821.0 |
| stage/render/prepare_resources | 0.412 | 0.492 | 1.51 | 2750.9 |
| stage/main/pre_update | 0.381 | 0.549 | 1.72 | 2542.7 |
| stage/main/first | 0.366 | 0.483 | 5.79 | 2442.0 |
| controller/feed/ms | 0.317 | 0.402 | 1.01 | 2115.8 |
| stage/render/prepare_flush | 0.315 | 0.520 | 1.24 | 2105.9 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.306 | 1.120 | 1.90 | 2043.8 |
| stage/render/prepare_bind_groups | 0.297 | 0.488 | 1.66 | 1984.4 |
| stage/render/prepare_batch_phases | 0.269 | 0.412 | 0.83 | 1796.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.262 | 0.433 | 1.04 | 1751.6 |
| stage/main/last | 0.252 | 0.295 | 0.87 | 1682.0 |
| bigspace/low_precision/ms | 0.225 | 0.691 | 1.66 | 1504.5 |
| stage/render/create_views | 0.199 | 0.271 | 0.83 | 1331.7 |
| stream/drive/ms | 0.197 | 0.282 | 10.92 | 1311.9 |
| stage/render/extract_commands | 0.183 | 0.217 | 0.55 | 1223.8 |
| bigspace/high_precision/ms | 0.170 | 0.695 | 4.25 | 1136.7 |
| anim/evaluate/ms | 0.159 | 0.285 | 1.45 | 1059.9 |
| stage/render/specialize | 0.159 | 0.269 | 2.23 | 1059.3 |
| anim/evaluate_cpu/ms | 0.151 | 0.187 | 0.74 | 1010.2 |
| pipelines/compile_ms | 0.148 | 0.000 | 129.06 | 989.5 |
| stream/tick/ms | 0.144 | 0.211 | 0.56 | 962.6 |
| pipelines/front_ms | 0.144 | 0.000 | 126.07 | 961.8 |
| stage/render/prepare_views | 0.143 | 0.184 | 1.51 | 951.8 |
| render/acquire/ms | 0.102 | 0.131 | 1.46 | 678.9 |
| stage/render/queue | 0.096 | 0.169 | 1.69 | 643.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.089 | 0.140 | 2.36 | 593.6 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.087 | 0.822 | 1.40 | 580.2 |
| stage/render/phase_sort | 0.080 | 0.110 | 0.36 | 532.5 |
| controller/write/ms | 0.073 | 0.102 | 0.42 | 489.9 |
| controller/move/ms | 0.069 | 0.195 | 0.58 | 458.9 |
| terrain/update/ms | 0.059 | 0.288 | 1.87 | 391.9 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.050 | 0.000 | 2.96 | 335.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.43 | 304.5 |
| physics/step/ms | 0.043 | 0.107 | 0.68 | 288.3 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.73 | 212.9 |
| stage/render/cleanup | 0.024 | 0.033 | 0.41 | 156.9 |
| physics/insert/ms | 0.023 | 0.262 | 0.84 | 151.6 |
| terrain/select/ms | 0.021 | 0.255 | 0.68 | 143.4 |
| weather/map/ms | 0.017 | 0.025 | 0.34 | 116.0 |
| far/select/ms | 0.015 | 0.163 | 0.51 | 98.0 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.68 | 84.8 |
| anim/apply/ms | 0.011 | 0.020 | 0.08 | 75.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.60 | 71.6 |
| weather/eval/ms | 0.011 | 0.014 | 0.12 | 70.6 |
| script/update/ms | 0.008 | 0.012 | 0.04 | 52.5 |
| controller/avoid/ms | 0.006 | 0.012 | 0.12 | 39.1 |
| stream/despawn/ms | 0.005 | 0.000 | 0.97 | 35.6 |
| script/input/ms | 0.005 | 0.006 | 0.13 | 32.3 |
| pipelines/driver_ms | 0.004 | 0.000 | 3.94 | 27.6 |
| script/ui/ms | 0.004 | 0.005 | 0.09 | 23.4 |
| stream/spawn/ms | 0.003 | 0.000 | 1.06 | 21.9 |
| stream/upload/ms | 0.003 | 0.000 | 0.93 | 17.6 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 7.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.6 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 34 | 17 |
| controller/movers/count | count | 19 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1313 | 2262 | 641 |
| crowd/c0/meshes/count | count | 357 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5531 | 5997 | 5555 |
| crowd/c1/meshes/count | count | 1181 | 1299 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 205 |
| crowd/joints_extracted/count | count | 679 | 6948 | 405 |
| crowd/joints_uploaded/count | count | 3611 | 6193 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.39 | 2.65 | 0.45 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.68 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.60 | 0.02 |
| encode/finish_critical | ms | 0.96 | 3.34 | 1.01 |
| encode/pass:early prepass/max | ms | 0.82 | 3.23 | 1.01 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 3.34 | 0.59 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 2.36 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.04 | 0.25 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.79 | 1.90 | 0.83 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.36 | 1.40 | 0.14 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.16 | 2.96 | 0.89 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 3.28 | 0.93 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.73 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.51 | 29.13 | 8.71 |
| gpu/timed/ms | ms | 7.95 | 9.62 | 8.71 |
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
| phases/alpha_mask3d/items | count | 319 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 319 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2482 | 3551 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2482 | 3551 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 70 | 485 | 8 |
| phases/shadow/bins | count | 878 | 1556 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2348 | 3326 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 55 | 151 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2707 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 15 | 274 | 7 |
| preprocess/bind_groups_created | count | 10 | 268 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 137 | 219 | 127 |
| process/cpu_usage | % | 25.48 | 37.02 | 31.80 |
| process/mem_usage | GiB | 5.12 | 5.42 | 5.42 |
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
| shadow_atlas/live_occ_pct | % | 11.72 | 37.60 | 0 |
| shadow_atlas/live_tiles | count | 18 | 70 | 0 |
| shadow_atlas/mover_draws | count | 10 | 38 | 0 |
| shadow_atlas/movers | count | 147 | 476 | 0 |
| shadow_atlas/moving_lamps | count | 1 | 3 | 0 |
| shadow_atlas/render/blits | count | 10 | 38 | 0 |
| shadow_atlas/render/clears | count | 0 | 18 | 0 |
| shadow_atlas/render/lamps | count | 8 | 19 | 0 |
| shadow_atlas/render/live_direct | count | 10 | 38 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 12 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 61 | 0 |
| shadow_atlas/resizes | count | 32 | 41 | 41 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 107 | 0 |
| shadow_atlas/views | count | 17 | 61 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 120 | 9 |
| skin/blocks_skipped/count | count | 145 | 203 | 196 |
| skin/blocks_walked/count | count | 14 | 28 | 9 |
| skin/joint_bytes_written/bytes | bytes | 48.1 k | 808.1 k | 35.1 k |
| skin/joints_written/count | count | 752 | 12.6 k | 548 |
| skin/upload_writes/count | count | 8 | 26 | 9 |
| started_load_count |  | 6439 | 6563 | 6563 |
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
| stream/latency/s | s | 0.03 | 0.07 | 0.06 |
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
| system/cpu_usage | % | 29.24 | 53.35 | 34.55 |
| system/mem_usage | % | 47.04 | 47.97 | 47.93 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 130 | 540 | 66 |
| views/all/bins | count | 2937 | 3569 | 2948 |
| views/all/count | count | 20 | 64 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8062 | 10.6 k | 7964 |
| views/all/pipelines | count | 101 | 199 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 231.1 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 717 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2123 | 2454 | 2022 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 59.50 | 453 | 0 |
| views/lamp_face/bins | count | 115 | 718 | 0 |
| views/lamp_face/count | count | 17 | 61 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 175 | 879 | 0 |
| views/lamp_face/pipelines | count | 29 | 125 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 122 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7522 | 5942 |
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
| anim/bones_written/count | count | 50.20 | 221 | 335.2 k |
| anim/events/count | count | 0.08 | 3 | 529 |
| assets/Image/added | count | 0.09 | 36 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 11 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.43 | 227 | 2874 |
| assets/Mesh/removed | count | 0.73 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.12 | 10 | 7482 |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.22 | 1154 | 1490 |
| encode//count | count | 27.90 | 30 | 186.3 k |
| encode/bind_walk/init_textures | count | 9103 | 23.1 k | 60.77 M |
| encode/bind_walk/init_textures_skipped | count | 50.2 k | 73.2 k | 334.96 M |
| encode/bind_walk/inits | count | 568 | 768 | 3.79 M |
| encode/bind_walk/inits_skipped | count | 169 | 251 | 1.13 M |
| encode/bind_walk/merge_textures | count | 9040 | 23.0 k | 60.35 M |
| encode/bind_walk/merge_textures_skipped | count | 9373 | 21.2 k | 62.57 M |
| encode/bind_walk/merges | count | 106 | 179 | 710.6 k |
| encode/bind_walk/merges_skipped | count | 151 | 225 | 1.01 M |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6676 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6676 |
| encode/pass:early prepass/count | count | 1 | 1 | 6676 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6676 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6676 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6676 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.37 | 1 | 2446 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.16 | 1 | 1090 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 271 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6676 |
| encode/queue/command_buffers | count | 38.49 | 42 | 256.9 k |
| encode/queue/submits | count | 2.01 | 4 | 13.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.32 M | 15.51 G |
| encode/queue/write_buffer/calls | count | 276 | 452 | 1.84 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.9 k | 1.29 M | 1.14 G |
| encode/queue/write_buffer_with/calls | count | 10.28 | 690 | 68.6 k |
| encode/queue/write_texture/bytes | bytes | 1976 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6676 |
| physics/inserts | count | 0.34 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.35 | 181 | 2335 |
| physics/steps | count | 0.59 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 12 |
| pipelines/created | count | 0.00 | 1 | 13 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 39 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5071 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1112 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.33 | 2330.9 | main/busy/ms 2300, stage/main/post_update 2251, stage/main/update 38.06, stage/main/pre_update 7.34, stage/main/last 1.07, surface/advance_cpu_ms 0.85, stage/main/fixed_main_loop 0.51, stage/main/first 0.49, stream/drive/ms 0.40, surface/cell_upload_ms 0.35, bigspace/total/ms 0.10, stream/tick/ms 0.09 |
| warm-up | 3.08 | 748.3 | main/busy/ms 2445, stage/main/post_update 2279, render/thread/ms 717, render/busy/ms 575, stage/render/prepare_assets 513, stage/main/update 148, stage/render/prepare_views 139, render/acquire/ms 139, stage/render/create_views 29.46, stage/render/render 24.39, render/graph/ms 19.45, stage/render/camera_driver 17.54 |
| warm-up | 3.21 | 133.0 | render/thread/ms 834, render/busy/ms 630, stage/render/prepare_assets 557, main/busy/ms 173, stage/render/prepare_views 140, render/acquire/ms 139, stage/main/update 117, stage/render/render 89.65, pipelines/compile_ms 65.73, render/present/ms 64.93, pipelines/driver_ms 51.26, stage/main/post_update 41.58 |
| warm-up | 3.25 | 34.8 | pipelines/compile_ms 405, pipelines/front_ms 301, render/thread/ms 143, pipelines/driver_ms 104, stage/render/render 84.67, render/busy/ms 78.05, main/busy/ms 76.32, render/present/ms 63.99, stage/main/post_update 53.64, pipelines/wait_ms 51.00, stage/render/prepare_assets 44.37, terrain/update/ms 27.97 |
| warm-up | 3.36 | 109.2 | pipelines/wait_ms 2555, pipelines/compile_ms 1453, pipelines/front_ms 1304, pipelines/driver_ms 149, main/busy/ms 136, render/thread/ms 113, render/busy/ms 108, stage/main/post_update 61.05, stage/main/update 51.19, encode//sum 43.83, stage/render/render 37.52, render/graph/ms 32.94 |
| warm-up | 3.44 | 81.8 | pipelines/wait_ms 4512, pipelines/compile_ms 1982, pipelines/front_ms 1768, pipelines/driver_ms 215, render/thread/ms 163, render/busy/ms 160, main/busy/ms 123, stage/render/prepare_assets 83.00, stage/main/update 54.43, encode//sum 47.39, stage/main/post_update 44.07, stage/render/prepare_resources 27.73 |
| warm-up | 3.47 | 29.5 | pipelines/wait_ms 2026, pipelines/compile_ms 941, pipelines/front_ms 818, pipelines/driver_ms 123, render/thread/ms 104, render/busy/ms 102, stage/render/prepare_assets 72.32, main/busy/ms 69.11, stage/main/post_update 29.26, terrain/update/ms 17.47, stage/main/pre_update 14.72, stage/main/update 14.58 |
| warm-up | 3.54 | 67.8 | pipelines/compile_ms 769, pipelines/front_ms 739, pipelines/wait_ms 206, render/thread/ms 86.10, render/busy/ms 84.38, main/busy/ms 84.09, stage/render/prepare_assets 54.76, stage/main/post_update 31.60, pipelines/driver_ms 29.85, stage/main/fixed_main_loop 24.59, stage/main/pre_update 14.38, stage/main/update 12.13 |
| warm-up | 3.58 | 27.3 | pipelines/compile_ms 678, pipelines/front_ms 655, pipelines/wait_ms 168, main/busy/ms 54.67, render/thread/ms 46.47, render/busy/ms 44.38, stage/main/post_update 37.00, pipelines/driver_ms 23.10, encode//sum 17.89, stage/render/render 14.04, stage/main/update 11.96, render/graph/ms 11.56 |
| warm-up | 3.63 | 42.9 | pipelines/compile_ms 968, pipelines/front_ms 924, pipelines/wait_ms 247, main/busy/ms 105, stage/main/update 65.43, render/thread/ms 59.30, render/busy/ms 57.26, pipelines/driver_ms 43.62, stage/main/post_update 31.12, stage/render/render 13.26, stage/render/prepare_resources 13.26, render/graph/ms 10.56 |
| warm-up | 3.75 | 120.9 | pipelines/compile_ms 2091, pipelines/front_ms 1976, pipelines/wait_ms 266, pipelines/driver_ms 115, render/thread/ms 82.74, main/busy/ms 82.58, render/busy/ms 80.80, stage/main/update 63.69, stage/render/prepare_assets 42.06, stage/render/render 16.73, render/graph/ms 14.25, stage/render/prepare_resources 9.30 |
| warm-up | 3.80 | 48.2 | pipelines/compile_ms 2070, pipelines/front_ms 1969, pipelines/wait_ms 181, pipelines/driver_ms 100, render/thread/ms 77.62, render/busy/ms 75.98, stage/render/prepare_assets 56.43, main/busy/ms 20.64, stage/render/render 15.22, render/graph/ms 13.40, stage/main/update 8.07, pipelines/lock_ms 7.53 |
| warm-up | 3.88 | 33.2 | render/thread/ms 45.88, render/busy/ms 44.52, pipelines/compile_ms 42.97, pipelines/front_ms 40.73, main/busy/ms 37.06, stage/main/update 24.33, stage/render/render 12.57, stage/render/prepare_assets 12.44, render/graph/ms 10.66, stage/render/prepare_write_phase_buffers 8.42, stage/main/post_update 7.38, encode//sum 7.28 |
| warm-up | 3.95 | 63.2 | pipelines/compile_ms 764, pipelines/front_ms 736, render/thread/ms 74.92, render/busy/ms 72.63, main/busy/ms 39.72, stage/render/prepare_assets 37.03, pipelines/driver_ms 27.70, stage/main/update 24.74, stage/render/render 16.64, render/graph/ms 13.66, stage/render/graph_submit 10.10, stage/main/post_update 8.23 |
| warm-up | 4.15 | 184.8 | pipelines/compile_ms 1468, pipelines/front_ms 1427, main/busy/ms 118, stage/main/update 105, render/thread/ms 98.98, render/busy/ms 97.39, stage/render/prepare_assets 46.40, stage/render/render 43.27, render/graph/ms 41.25, pipelines/driver_ms 41.13, stage/render/graph_submit 38.30, pipelines/wait_ms 34.85 |
| warm-up | 4.21 | 53.1 | pipelines/compile_ms 1460, pipelines/front_ms 1420, render/thread/ms 124, render/busy/ms 123, main/busy/ms 78.07, pipelines/wait_ms 67.66, stage/main/update 62.69, stage/render/render 55.02, stage/render/prepare_assets 54.97, render/graph/ms 52.93, stage/render/graph_submit 48.21, pipelines/driver_ms 40.66 |
| warm-up | 4.30 | 91.6 | pipelines/compile_ms 558, pipelines/front_ms 539, render/thread/ms 117, render/busy/ms 115, main/busy/ms 89.23, stage/render/prepare_assets 50.88, stage/main/update 50.47, pipelines/wait_ms 41.01, stage/main/post_update 30.86, stage/render/render 23.45, render/graph/ms 21.24, pipelines/driver_ms 19.16 |
| warm-up | 4.37 | 72.5 | pipelines/compile_ms 1591, pipelines/front_ms 1539, pipelines/wait_ms 191, render/thread/ms 125, render/busy/ms 123, main/busy/ms 87.58, stage/render/prepare_assets 70.44, pipelines/driver_ms 52.15, stage/main/update 46.49, stage/main/post_update 34.30, stage/render/prepare_resources 18.85, stage/render/render 11.90 |
| warm-up | 4.45 | 77.5 | pipelines/compile_ms 2397, pipelines/front_ms 2319, pipelines/wait_ms 287, render/thread/ms 106, render/busy/ms 104, main/busy/ms 86.46, pipelines/driver_ms 78.28, stage/render/prepare_assets 60.27, stage/main/update 56.27, stage/main/post_update 21.85, stream/drive/ms 18.83, stage/render/render 18.49 |
| warm-up | 4.50 | 51.9 | pipelines/compile_ms 1538, pipelines/front_ms 1488, pipelines/wait_ms 115, main/busy/ms 112, render/thread/ms 97.03, render/busy/ms 94.60, stage/main/update 82.87, pipelines/driver_ms 50.40, stage/render/prepare_assets 45.62, stage/render/render 22.56, stage/main/post_update 20.01, stream/drive/ms 19.84 |
| warm-up | 4.61 | 110.0 | pipelines/compile_ms 2315, pipelines/front_ms 2257, pipelines/wait_ms 548, main/busy/ms 103, render/thread/ms 92.32, render/busy/ms 90.84, stage/main/update 77.80, stage/render/prepare_assets 59.85, pipelines/driver_ms 58.20, stage/render/render 15.12, render/graph/ms 11.91, stage/main/post_update 11.77 |
| warm-up | 4.65 | 41.9 | pipelines/compile_ms 2565, pipelines/front_ms 2502, pipelines/wait_ms 562, render/thread/ms 86.79, render/busy/ms 85.35, main/busy/ms 68.41, pipelines/driver_ms 62.82, stage/render/prepare_assets 51.12, stage/main/update 36.76, stage/main/post_update 16.91, stage/render/render 15.60, render/graph/ms 13.78 |
| warm-up | 4.68 | 27.3 | pipelines/compile_ms 729, pipelines/front_ms 709, main/busy/ms 83.00, render/thread/ms 57.09, render/busy/ms 55.64, stage/main/update 53.04, pipelines/wait_ms 35.44, stream/drive/ms 31.41, stage/main/post_update 20.01, pipelines/driver_ms 19.72, stage/render/prepare_assets 17.84, stage/render/render 16.30 |
| warm-up | 4.74 | 64.5 | pipelines/compile_ms 316, pipelines/front_ms 305, main/busy/ms 81.23, stage/main/update 58.58, render/thread/ms 53.56, render/busy/ms 51.69, stream/drive/ms 39.82, stage/render/render 19.88, render/graph/ms 15.59, stage/render/prepare_assets 15.34, stage/main/post_update 14.48, pipelines/driver_ms 10.65 |
| warm-up | 4.79 | 41.4 | pipelines/compile_ms 439, pipelines/front_ms 425, main/busy/ms 79.70, render/thread/ms 62.10, stage/main/update 60.83, render/busy/ms 60.15, stream/drive/ms 36.68, stage/render/prepare_assets 19.83, stage/render/render 19.52, render/graph/ms 15.09, pipelines/driver_ms 13.43, stage/main/post_update 10.54 |
| warm-up | 4.83 | 45.8 | pipelines/compile_ms 1187, pipelines/front_ms 1154, main/busy/ms 63.17, render/thread/ms 51.78, render/busy/ms 50.34, stage/main/update 46.30, pipelines/driver_ms 32.66, stream/drive/ms 28.73, stage/render/prepare_assets 21.32, stage/render/render 11.11, render/graph/ms 9.07, stage/main/post_update 8.38 |
| warm-up | 5.02 | 28.4 | pipelines/compile_ms 572, pipelines/front_ms 555, main/busy/ms 43.31, render/thread/ms 42.30, render/busy/ms 40.79, stage/main/update 18.79, stage/main/post_update 18.72, pipelines/driver_ms 17.49, stage/render/render 16.06, render/graph/ms 14.04, stage/render/graph_submit 10.07, stage/render/prepare_resources 7.59 |
| warm-up | 5.05 | 29.8 | pipelines/compile_ms 426, pipelines/front_ms 412, render/thread/ms 51.31, render/busy/ms 49.83, main/busy/ms 43.20, stage/main/update 20.67, stage/render/render 16.33, render/graph/ms 14.15, pipelines/driver_ms 13.98, stage/main/post_update 11.71, stage/render/prepare_resources 10.86, stage/render/graph_submit 10.10 |
| warm-up | 5.54 | 28.4 | main/busy/ms 34.51, stage/main/update 23.33, render/thread/ms 20.67, render/busy/ms 19.69, stream/drive/ms 16.78, stage/render/render 9.01, render/graph/ms 7.61, stage/main/post_update 7.02, encode//sum 6.18, stage/render/graph_submit 4.55, stage/render/prepare_assets 4.36, stage/render/camera_driver 2.91 |
| warm-up | 5.65 | 28.2 | render/thread/ms 38.06, render/busy/ms 36.70, main/busy/ms 27.37, pipelines/compile_ms 22.96, pipelines/front_ms 21.80, stage/render/render 13.61, stage/render/prepare_assets 12.40, stage/main/update 12.36, stage/main/post_update 11.76, render/graph/ms 11.57, encode//sum 8.11, stage/render/graph_submit 7.78 |
| warm-up | 5.70 | 30.0 | render/thread/ms 33.57, render/busy/ms 32.69, main/busy/ms 24.69, stage/render/prepare_assets 16.63, stage/main/update 10.04, stage/main/post_update 9.11, stage/render/render 8.32, render/graph/ms 6.94, encode//sum 5.93, stage/render/graph_submit 4.73, encode/submit 2.88, stage/main/fixed_main_loop 2.24 |
| warm-up | 5.79 | 25.3 | pipelines/compile_ms 129, pipelines/front_ms 125, render/thread/ms 39.73, render/busy/ms 38.64, main/busy/ms 36.95, stage/main/update 23.14, stage/render/render 14.03, render/graph/ms 12.38, stage/render/prepare_assets 10.47, encode//sum 8.32, stage/render/graph_submit 8.16, stage/main/post_update 7.56 |
| warm-up | 5.89 | 27.2 | render/thread/ms 39.92, render/busy/ms 38.47, main/busy/ms 37.70, stage/render/render 18.15, stage/main/update 17.33, render/graph/ms 16.27, stage/main/post_update 14.68, stage/render/graph_submit 11.33, encode//sum 9.10, encode/submit 8.21, pipelines/compile_ms 5.90, stage/render/prepare_assets 5.70 |
| warm-up | 5.96 | 41.9 | render/thread/ms 52.30, render/busy/ms 51.20, main/busy/ms 40.59, stage/main/update 23.53, stage/render/prepare_assets 18.82, stage/render/render 13.84, render/graph/ms 12.45, stage/main/post_update 10.23, stage/render/graph_submit 9.65, encode//sum 6.77, stage/render/prepare_meshes 6.12, encode/finish_wall 5.58 |
| warm-up | 6.00 | 40.3 | render/thread/ms 68.31, render/busy/ms 67.11, main/busy/ms 56.17, stage/main/post_update 24.89, stage/render/render 23.98, stage/main/update 22.70, render/graph/ms 22.23, stage/render/prepare_assets 20.36, stage/render/graph_submit 18.28, encode/finish_wall 12.33, encode/pass:point_shadow_atlas_live_indirect/sum 12.08, encode//sum 9.03 |
| warm-up | 6.06 | 33.8 | pipelines/compile_ms 152, pipelines/front_ms 149, render/thread/ms 43.70, render/busy/ms 42.83, main/busy/ms 29.62, stage/render/prepare_assets 17.48, stage/render/render 14.97, stage/main/update 14.67, render/graph/ms 13.43, stage/render/graph_submit 10.74, stage/main/post_update 8.74, encode/finish_wall 7.76 |
| warm-up | 6.21 | 38.8 | pipelines/compile_ms 118, pipelines/front_ms 115, main/busy/ms 45.28, render/thread/ms 38.08, render/busy/ms 36.77, stage/main/update 30.45, stage/render/render 16.80, render/graph/ms 15.08, stream/drive/ms 13.85, stage/render/graph_submit 12.02, stage/render/prepare_assets 9.48, stage/main/post_update 9.48 |
| warm-up | 6.25 | 36.2 | render/thread/ms 52.83, render/busy/ms 51.52, main/busy/ms 44.37, stage/main/update 24.68, stage/render/render 23.43, render/graph/ms 21.56, stage/render/graph_submit 17.38, stage/render/prepare_assets 14.53, encode/finish_wall 12.62, encode/pass:point_shadow_atlas_live_indirect/sum 12.37, stage/main/post_update 12.32, pipelines/compile_ms 9.30 |
| warm-up | 6.28 | 26.4 | render/thread/ms 50.88, render/busy/ms 49.56, main/busy/ms 43.24, stage/main/update 24.06, stage/render/render 22.86, render/graph/ms 20.83, stage/render/graph_submit 16.93, stage/render/prepare_assets 14.24, stage/main/post_update 12.61, encode/finish_wall 12.25, encode/pass:point_shadow_atlas_live_indirect/sum 12.06, pipelines/compile_ms 9.30 |
| measure | 18.30 | 31.7 | main/busy/ms 38.47, stage/main/update 25.47, render/thread/ms 20.49, render/busy/ms 19.56, stage/render/render 9.98, render/graph/ms 8.75, stage/main/post_update 8.36, stage/render/graph_submit 5.92, encode//sum 5.63, stage/render/prepare_assets 3.89, encode/submit 3.84, stage/render/camera_driver 2.68 |
| measure | 59.20 | 32.0 | render/thread/ms 37.81, render/busy/ms 36.68, main/busy/ms 25.95, stage/render/prepare_assets 17.43, stage/main/post_update 13.09, stage/render/render 10.42, render/graph/ms 8.95, encode//sum 7.85, stage/render/graph_submit 5.89, stage/main/update 4.79, stage/main/fixed_main_loop 3.82, encode/submit 3.79 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (254 samples): GPU load **86%** (min 69%) · 2622 MHz (min 2610) · memory 8101 MHz · 85.4 W · 80 °C (max 84) · VRAM 5585 MiB · throttled 100% of samples

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
