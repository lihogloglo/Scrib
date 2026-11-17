# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5565 frames over 65.6 s · **median 10.95 ms (91 fps)** · mean 11.80 · p95 16.92 · p99 18.49 (1% low 54 fps) · worst 30.13 ms · 395 spikes over 1.5× median · 12.8 stutters/min (work 10.1/min)
- warm-up: 324 frames over 9.1 s, worst 2392.7 ms
- hitches (> 27.4 ms): **2** measured, 49 in warm-up
- timed GPU passes: 6.80 ms of a 11.80 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.95 ms, limit 11 ms
- **FAIL** frame_ms: 14 stutters (> 1.5x moving median of 60) in 1.09 min = 12.80/min, limit 1/min
- pass worst frame 30.13 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.952 | 2.030 | 2.062 | 0.96 | 0.174 |
| main_opaque_pass_3d | 1.274 | 1.325 | 1.661 | 0.96 | 0.058 |
| auxiliary_views | 0.538 | 0.559 | 1.477 | 0.96 | 0.491 |
| early prepass | 0.484 | 0.503 | 0.721 | 0.96 | 0.086 |
| bloom | 0.303 | 0.315 | 0.321 | 0.96 | 0.064 |
| atmosphere_luts | 0.296 | 0.308 | 0.369 | 0.96 | 0.003 |
| water_velocity | 0.252 | 0.262 | 0.350 | 0.96 | 0.002 |
| water_sspr | 0.194 | 0.201 | 0.203 | 0.96 | 0.002 |
| ssao | 0.181 | 0.188 | 0.193 | 0.96 | 0.007 |
| water_fft | 0.124 | 0.128 | 0.137 | 0.96 | 0.003 |
| atmos_froxel_inject | 0.123 | 0.127 | 0.128 | 0.96 | 0.001 |
| upscaling | 0.121 | 0.126 | 0.134 | 0.96 | 0.004 |
| uniform_allocation | 0.121 | 0.125 | 0.155 | 0.96 | 0.024 |
| clouds_composite | 0.101 | 0.105 | 0.108 | 0.96 | 0.002 |
| bin_unpacking | 0.093 | 0.097 | 0.119 | 0.96 | 0.017 |
| tonemapping | 0.084 | 0.087 | 0.091 | 0.96 | 0.003 |
| main_transmissive_pass_3d | 0.076 | 0.079 | 0.158 | 0.96 | 0.008 |
| clustering | 0.061 | 0.063 | 0.066 | 0.96 | 0.018 |
| clouds_trace | 0.054 | 0.056 | 0.074 | 0.96 | 0.001 |
| render_sky | 0.049 | 0.051 | 0.052 | 0.96 | 0.002 |
| clouds_resolve | 0.047 | 0.049 | 0.050 | 0.96 | 0.001 |
| surface_trail | 0.040 | 0.056 | 0.057 | 0.72 | 0.002 |
| lightprobe_irradiance_map | 0.037 | 0.413 | 0.416 | 0.09 | 0.001 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.96 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.96 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.029 | 0.96 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.020 | 0.219 | 0.229 | 0.09 | 0.004 |
| night_sky | 0.016 | 0.017 | 0.038 | 0.96 | 0.002 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.025 | 0.96 | 0.127 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.96 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.96 | 0.003 |
| per_view_shadow_pass | 0.009 | 0.009 | 0.015 | 0.96 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.96 | 0.003 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.549 | 15.668 | 23.65 | 58705.5 |
| encode//sum | 10.094 | 26.400 | 36.84 | 56174.0 |
| render/busy/ms | 10.009 | 15.097 | 23.16 | 55701.2 |
| main/busy/ms | 9.227 | 12.291 | 28.23 | 51350.9 |
| stage/render/render | 6.878 | 11.371 | 14.83 | 38275.3 |
| render/graph/ms | 6.185 | 10.593 | 14.02 | 34420.2 |
| stage/main/post_update | 5.882 | 8.642 | 12.45 | 32734.3 |
| stage/render/graph_submit | 4.679 | 8.269 | 10.09 | 26037.7 |
| encode/submit | 2.810 | 4.441 | 5.42 | 15639.3 |
| stage/main/update | 2.129 | 2.566 | 22.56 | 11848.1 |
| encode/finish_wall | 1.920 | 4.008 | 5.28 | 10684.4 |
| encode/pass:shadow_point_light/sum | 1.490 | 4.738 | 7.52 | 8290.9 |
| stage/render/camera_driver | 1.451 | 2.450 | 4.42 | 8076.8 |
| encode/pass:shadow_directional_light/sum | 0.738 | 1.287 | 2.10 | 4107.8 |
| stage/render/prepare_write_phase_buffers | 0.726 | 1.077 | 2.41 | 4038.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.654 | 1.020 | 1.71 | 3638.4 |
| encode/pass:early prepass/sum | 0.639 | 1.009 | 2.10 | 3556.8 |
| stage/render/prepare_assets | 0.605 | 0.752 | 9.21 | 3368.1 |
| bigspace/total/ms | 0.572 | 1.502 | 4.58 | 3180.5 |
| stage/render/prepare_resources | 0.450 | 0.541 | 1.11 | 2505.9 |
| render/present/ms | 0.447 | 0.550 | 2.33 | 2487.7 |
| stage/render/prepare_meshes | 0.404 | 0.510 | 2.15 | 2248.8 |
| stage/main/pre_update | 0.349 | 0.477 | 1.16 | 1944.3 |
| stage/main/first | 0.349 | 0.451 | 7.65 | 1942.0 |
| terrain/update/ms | 0.349 | 0.576 | 2.70 | 1941.6 |
| terrain/select/ms | 0.315 | 0.533 | 1.13 | 1753.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.279 | 0.457 | 0.95 | 1554.0 |
| stage/render/prepare_bind_groups | 0.268 | 0.468 | 2.29 | 1493.9 |
| controller/feed/ms | 0.259 | 0.324 | 0.74 | 1441.3 |
| stream/drive/ms | 0.247 | 0.333 | 20.22 | 1372.3 |
| bigspace/low_precision/ms | 0.241 | 1.032 | 4.37 | 1339.3 |
| stage/main/fixed_main_loop | 0.227 | 0.365 | 1.34 | 1263.9 |
| stage/render/specialize | 0.199 | 0.319 | 1.46 | 1105.8 |
| far/select/ms | 0.195 | 0.257 | 0.49 | 1083.7 |
| stage/render/create_views | 0.193 | 0.261 | 0.89 | 1075.3 |
| stage/render/queue | 0.188 | 0.316 | 1.74 | 1047.4 |
| stream/tick/ms | 0.188 | 0.252 | 0.52 | 1046.1 |
| stage/render/prepare_batch_phases | 0.186 | 0.262 | 0.53 | 1032.4 |
| anim/evaluate/ms | 0.175 | 0.279 | 1.64 | 971.2 |
| stage/render/extract_commands | 0.173 | 0.204 | 0.52 | 962.8 |
| bigspace/high_precision/ms | 0.168 | 0.983 | 3.04 | 933.3 |
| stage/render/prepare_flush | 0.165 | 0.324 | 0.80 | 919.5 |
| anim/evaluate_cpu/ms | 0.155 | 0.188 | 0.67 | 863.3 |
| controller/move/ms | 0.148 | 0.345 | 0.71 | 824.5 |
| stage/render/prepare_views | 0.135 | 0.166 | 0.67 | 749.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.133 | 0.257 | 0.99 | 742.8 |
| stage/main/last | 0.116 | 0.161 | 1.57 | 647.5 |
| render/acquire/ms | 0.093 | 0.118 | 0.64 | 516.6 |
| stage/render/prepare_collect_phase_buffers | 0.058 | 0.125 | 0.37 | 324.1 |
| physics/step/ms | 0.050 | 0.084 | 0.52 | 278.9 |
| encode/point_shadow_cache_copies/sum | 0.045 | 0.168 | 0.65 | 248.3 |
| physics/insert/ms | 0.042 | 0.402 | 2.37 | 231.5 |
| stage/render/phase_sort | 0.039 | 0.070 | 0.27 | 216.1 |
| stage/render/cleanup | 0.038 | 0.120 | 0.79 | 209.9 |
| encode/resolve_diagnostics/sum | 0.028 | 0.038 | 0.54 | 153.9 |
| pipelines/compile_ms | 0.016 | 0.000 | 80.32 | 90.1 |
| weather/map/ms | 0.016 | 0.021 | 0.06 | 88.5 |
| pipelines/front_ms | 0.016 | 0.000 | 77.21 | 86.7 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.020 | 0.34 | 70.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.21 | 60.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.24 | 60.1 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.12 | 55.2 |
| weather/eval/ms | 0.009 | 0.012 | 0.19 | 51.9 |
| script/update/ms | 0.007 | 0.009 | 0.24 | 37.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.01 | 30.6 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 28.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 27.7 |
| script/ui/ms | 0.005 | 0.006 | 0.42 | 26.2 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 18.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.60 | 14.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.06 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.8 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.11 | 3.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 2.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 7 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1230 | 2262 | 537 |
| crowd/c0/meshes/count | count | 343 | 646 | 146 |
| crowd/c1/count | count | 148 | 156 | 149 |
| crowd/c1/joints/count | count | 5596 | 5991 | 5659 |
| crowd/c1/meshes/count | count | 1175 | 1295 | 1195 |
| crowd/c2/count | count | 400 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 205 |
| crowd/joints_extracted/count | count | 3723 | 7862 | 5775 |
| crowd/joints_uploaded/count | count | 3661 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.64 | 2.13 | 0.68 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.34 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.12 | 0.01 |
| encode/finish_critical | ms | 0.92 | 2.13 | 0.90 |
| encode/pass:early prepass/max | ms | 0.59 | 2.10 | 0.90 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.62 | 1.71 | 0.85 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.99 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.95 | 0.37 |
| encode/pass:shadow_directional_light/max | ms | 0.70 | 2.10 | 0.87 |
| encode/pass:shadow_point_light/max | ms | 0.70 | 1.35 | 0.71 |
| encode/point_shadow_cache_copies/max | ms | 0.07 | 0.65 | 0.12 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.54 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.14 | 28.23 | 10.62 |
| gpu/timed/ms | ms | 7.63 | 8.95 | 7.66 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 9 | 31 | 0 |
| lamp_cache/copies | count | 0 | 15 | 0 |
| lamp_cache/deferred_faces | count | 0 | 15 | 0 |
| lamp_cache/lamps | count | 6 | 8 | 0 |
| lamp_cache/mover_faces | count | 0 | 12 | 0 |
| lamp_cache/movers | count | 114 | 230 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 6 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 15 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 7 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 12 | 32 | 0 |
| lamp_cache/stock_lamps | count | 0 | 2 | 0 |
| lamp_cache/store_starved | count | 0 | 8 | 0 |
| lamp_cache/store_used | count | 5 | 24 | 0 |
| lamp_cache/views | count | 2 | 32 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2621 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 221 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 762 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 221 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 762 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1294 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3319 | 5121 | 3192 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1294 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3319 | 5121 | 3192 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 21 | 84 | 4 |
| phases/shadow/bins | count | 1495 | 2482 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3693 | 5337 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 37 | 106 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2634 | 3268 | 2347 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 201 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 18 | 297 | 24 |
| preprocess/bind_groups_created | count | 18 | 297 | 24 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 259 | 109 |
| process/cpu_usage | % | 23.66 | 42.58 | 28.71 |
| process/mem_usage | GiB | 5.36 | 5.54 | 5.54 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 0 | 0 | 0 |
| scene/batch/masked/count | count | 0 | 0 | 0 |
| scene/batch/masked/high_water | count | 0 | 0 | 0 |
| scene/batch/masked/live | count | 0 | 0 | 0 |
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
| scene/batch/opaque/capacity | count | 0 | 0 | 0 |
| scene/batch/opaque/count | count | 0 | 0 | 0 |
| scene/batch/opaque/high_water | count | 0 | 0 | 0 |
| scene/batch/opaque/live | count | 0 | 0 | 0 |
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
| scene/batches/count | count | 0 | 0 | 0 |
| scene/cells/resident | count | 0 | 0 | 0 |
| scene/chains/high_water | count | 0 | 0 | 0 |
| scene/chains/live | count | 0 | 0 | 0 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 0 | 0 | 0 |
| scene/slots/batch_capacity | count | 0 | 0 | 0 |
| scene/slots/batch_capacity_bytes | bytes | 0 | 0 | 0 |
| scene/slots/batch_live | count | 0 | 0 | 0 |
| scene/slots/scene_bump | count | 0 | 0 | 0 |
| scene/slots/scene_live | count | 0 | 0 | 0 |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 13 | 90 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 282.4 k | 807.6 k | 444.9 k |
| skin/joints_written/count | count | 4412 | 12.6 k | 6952 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4940 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 429 | 528 | 396 |
| statics/materials_bound | count | 429 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.11 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 323 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.38 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1126 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.91 | 41.58 | 29.92 |
| system/mem_usage | % | 54.04 | 54.66 | 54.60 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2910 | 3016 | 3016 |
| textures/images | count | 2691 | 2740 | 2740 |
| views/all/batch_sets | count | 71 | 133 | 56 |
| views/all/bins | count | 4894 | 5340 | 5002 |
| views/all/count | count | 9 | 41 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.0 k | 17.9 k | 10.7 k |
| views/all/pipelines | count | 84 | 152 | 67 |
| views/all/pipelines_distinct | count | 63 | 73 | 63 |
| views/all/work_items | count | 9922 | 18.0 k | 8148 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1445 | 1527 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3315 | 4428 | 3038 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 316 | 4428 | 222 |
| views/lamp_face/batch_sets | count | 2 | 62 | 0 |
| views/lamp_face/bins | count | 52 | 1053 | 0 |
| views/lamp_face/count | count | 6 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 1 | 24 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 94 | 1162 | 0 |
| views/lamp_face/pipelines | count | 13 | 84 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 3 | 1093 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2890 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7942 | 13.0 k | 7618 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8245 | 13.2 k | 7926 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.1 k | 37.9 k | 30.8 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 45.93 | 217 | 255.6 k |
| anim/events/count | count | 0.10 | 3 | 543 |
| assets/Image/added | count | 0.11 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.73 | 152 | 4061 |
| assets/Mesh/removed | count | 0.87 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.89 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.84 | 1386 | 4675 |
| encode//count | count | 54.08 | 123 | 300.9 k |
| encode/bind_walk/init_textures | count | 15.7 k | 45.0 k | 87.48 M |
| encode/bind_walk/init_textures_skipped | count | 51.0 k | 94.3 k | 283.80 M |
| encode/bind_walk/inits | count | 498 | 727 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 97.71 | 140 | 543.7 k |
| encode/bind_walk/merge_textures | count | 15.7 k | 44.9 k | 87.13 M |
| encode/bind_walk/merge_textures_skipped | count | 5399 | 6012 | 30.04 M |
| encode/bind_walk/merges | count | 121 | 211 | 672.5 k |
| encode/bind_walk/merges_skipped | count | 79.30 | 123 | 441.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5565 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5565 |
| encode/pass:early prepass/count | count | 1 | 1 | 5565 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5565 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5565 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5565 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5565 |
| encode/pass:shadow_point_light/count | count | 6.77 | 24 | 37.7 k |
| encode/point_shadow_cache_copies/count | count | 0.48 | 1 | 2686 |
| encode/queue/command_buffers | count | 71.35 | 158 | 397.1 k |
| encode/queue/submits | count | 2.02 | 4 | 11.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.83 M | 3.00 M | 10.18 G |
| encode/queue/write_buffer/calls | count | 274 | 465 | 1.52 M |
| encode/queue/write_buffer_with/bytes | bytes | 128.0 k | 3.50 M | 712.13 M |
| encode/queue/write_buffer_with/calls | count | 11.17 | 465 | 62.2 k |
| encode/queue/write_texture/bytes | bytes | 11.3 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5565 |
| physics/inserts | count | 0.40 | 94 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.71 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 18 | 378 |
| stream/upload/bytes | bytes | 6083 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1334 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 51, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.39 | 2392.7 | main/busy/ms 2364, stage/main/post_update 2325, stage/main/update 30.60, stage/main/pre_update 4.71, stage/main/last 0.99, surface/advance_cpu_ms 0.73, stream/drive/ms 0.48, stage/main/first 0.47, stage/main/fixed_main_loop 0.34, surface/cell_upload_ms 0.28, bigspace/total/ms 0.11, stream/tick/ms 0.10 |
| warm-up | 3.12 | 724.8 | main/busy/ms 2501, stage/main/post_update 2346, render/thread/ms 695, render/busy/ms 530, stage/render/prepare_assets 476, stage/render/prepare_views 162, render/acquire/ms 162, stage/main/update 140, stage/render/create_views 36.23, stage/render/render 11.15, render/graph/ms 6.62, stage/main/pre_update 6.44 |
| warm-up | 3.23 | 108.1 | render/thread/ms 799, render/busy/ms 580, stage/render/prepare_assets 504, pipelines/compile_ms 306, pipelines/front_ms 245, stage/render/prepare_views 163, render/acquire/ms 163, main/busy/ms 155, stage/main/update 114, stage/render/render 73.04, pipelines/driver_ms 60.65, render/present/ms 56.08 |
| warm-up | 3.25 | 28.2 | pipelines/compile_ms 306, pipelines/front_ms 245, render/thread/ms 104, main/busy/ms 77.96, stage/render/render 61.88, pipelines/driver_ms 60.65, render/present/ms 52.55, render/busy/ms 50.43, stage/main/post_update 28.10, stage/render/prepare_assets 27.78, terrain/update/ms 20.87, stage/main/fixed_main_loop 15.70 |
| warm-up | 3.35 | 91.9 | pipelines/wait_ms 1350, pipelines/compile_ms 738, pipelines/front_ms 646, pipelines/driver_ms 92.20, main/busy/ms 74.83, render/thread/ms 36.79, render/busy/ms 35.92, encode//sum 32.00, stage/main/post_update 28, stage/render/prepare_assets 22.70, stage/main/fixed_main_loop 14.36, stage/main/pre_update 13.90 |
| warm-up | 3.39 | 47.9 | pipelines/wait_ms 4026, pipelines/compile_ms 1002, pipelines/front_ms 893, pipelines/driver_ms 109, main/busy/ms 90.08, render/thread/ms 83.77, render/busy/ms 81.83, stage/main/update 61.07, encode//sum 41.44, stage/render/prepare_bind_groups 28.73, stage/render/prepare_assets 23.44, stage/render/render 22.04 |
| warm-up | 3.48 | 89.6 | pipelines/wait_ms 4311, pipelines/compile_ms 1297, pipelines/front_ms 1194, render/thread/ms 105, render/busy/ms 103, pipelines/driver_ms 102, main/busy/ms 97.33, stage/main/update 63.56, stage/render/prepare_assets 42.11, stage/render/prepare_bind_groups 28.87, stage/render/render 25.78, stage/main/post_update 24.59 |
| warm-up | 3.54 | 31.9 | pipelines/compile_ms 785, pipelines/front_ms 741, render/thread/ms 54.45, render/busy/ms 53.05, main/busy/ms 46.76, pipelines/driver_ms 44.03, stage/render/prepare_assets 31.73, pipelines/wait_ms 20.78, stage/main/post_update 15.87, stage/main/pre_update 12.99, terrain/update/ms 10.14, stage/render/render 9.29 |
| warm-up | 3.57 | 29.5 | pipelines/compile_ms 668, pipelines/front_ms 624, pipelines/wait_ms 121, encode//sum 85.45, render/thread/ms 55.81, render/busy/ms 54.01, main/busy/ms 45.64, pipelines/driver_ms 43.89, stage/main/post_update 24.59, stage/render/render 24.05, render/graph/ms 21.86, stage/render/graph_submit 17.07 |
| warm-up | 3.72 | 85.9 | pipelines/compile_ms 1813, pipelines/front_ms 1745, pipelines/driver_ms 67.97, main/busy/ms 67.57, render/thread/ms 63.42, render/busy/ms 61.11, pipelines/wait_ms 55.27, stage/main/update 54.49, stream/drive/ms 50.31, stage/render/prepare_assets 43.57, stage/render/render 12.96, render/graph/ms 10.28 |
| warm-up | 3.75 | 29.9 | pipelines/compile_ms 1274, pipelines/front_ms 1220, render/thread/ms 68.45, render/busy/ms 66.73, pipelines/driver_ms 54.23, main/busy/ms 38.37, stage/render/prepare_assets 34.15, pipelines/wait_ms 22.73, stage/main/update 18.69, stage/render/render 17.06, render/graph/ms 15.00, stage/main/post_update 13.44 |
| warm-up | 3.81 | 32.8 | pipelines/compile_ms 416, pipelines/front_ms 406, main/busy/ms 63.37, render/thread/ms 46.26, render/busy/ms 43.81, stage/main/update 40.34, stage/render/prepare_assets 27.12, stream/drive/ms 23.09, stage/main/post_update 17.13, stage/render/render 11.89, pipelines/driver_ms 10.72, render/graph/ms 8.81 |
| warm-up | 3.88 | 74.4 | pipelines/compile_ms 259, pipelines/front_ms 250, main/busy/ms 66.70, render/thread/ms 65.07, render/busy/ms 62.45, stage/render/prepare_assets 44.65, stage/main/update 41.31, stream/drive/ms 24.68, stage/main/post_update 20.74, stage/render/render 12.70, render/graph/ms 9.46, pipelines/driver_ms 9.31 |
| warm-up | 3.94 | 54.9 | pipelines/compile_ms 916, pipelines/front_ms 883, render/thread/ms 69.87, render/busy/ms 68.49, stage/render/prepare_assets 52.05, pipelines/driver_ms 32.90, main/busy/ms 29.77, stage/main/post_update 14.06, stage/render/render 11.29, stage/main/update 9.50, render/graph/ms 9.38, encode//sum 6.11 |
| warm-up | 3.98 | 46.2 | pipelines/compile_ms 1353, pipelines/front_ms 1304, render/thread/ms 64.13, render/busy/ms 62.36, pipelines/driver_ms 48.98, stage/render/prepare_assets 43.85, main/busy/ms 31.64, stage/main/post_update 12.28, stage/render/render 12.14, stage/main/update 11.84, render/graph/ms 10.05, stage/render/graph_submit 6.47 |
| warm-up | 4.08 | 93.6 | pipelines/compile_ms 1215, pipelines/front_ms 1170, render/thread/ms 120, render/busy/ms 117, main/busy/ms 91.69, stage/main/update 49.60, pipelines/driver_ms 45.26, stage/render/render 32.34, pipelines/wait_ms 32.02, render/graph/ms 29.23, stage/render/prepare_resources 27.84, stage/render/graph_submit 26.76 |
| warm-up | 4.23 | 154.0 | pipelines/compile_ms 3577, pipelines/front_ms 3196, pipelines/wait_ms 411, pipelines/driver_ms 381, render/thread/ms 208, render/busy/ms 205, main/busy/ms 116, stage/main/update 67.43, stage/render/render 52.11, render/graph/ms 48.71, stage/render/create_views 44.86, stage/render/prepare_assets 44.21 |
| warm-up | 4.29 | 56.8 | pipelines/compile_ms 3144, pipelines/front_ms 2775, pipelines/wait_ms 447, pipelines/driver_ms 370, render/thread/ms 173, render/busy/ms 171, main/busy/ms 85.37, stage/render/prepare_assets 66.69, stage/main/update 49.90, stage/render/create_views 45.62, stage/render/render 31.95, render/graph/ms 29.47 |
| warm-up | 4.38 | 95.6 | pipelines/compile_ms 523, pipelines/front_ms 502, pipelines/wait_ms 226, render/thread/ms 145, render/busy/ms 143, main/busy/ms 82.30, stage/main/update 49.96, stage/render/create_views 36.99, stage/render/prepare_assets 34.06, stage/render/prepare_resources 27.70, stage/main/post_update 23.91, pipelines/driver_ms 20.98 |
| warm-up | 4.44 | 58.2 | pipelines/compile_ms 1797, pipelines/front_ms 1741, pipelines/wait_ms 1421, render/thread/ms 136, render/busy/ms 134, main/busy/ms 63.48, pipelines/driver_ms 56.07, stage/main/update 40.33, stage/render/create_views 36.91, stage/render/prepare_resources 35.01, stage/render/prepare_assets 22.80, stage/render/render 14.82 |
| warm-up | 4.51 | 66.4 | pipelines/compile_ms 3019, pipelines/front_ms 2929, pipelines/wait_ms 1313, main/busy/ms 94.19, pipelines/driver_ms 89.78, render/thread/ms 84.07, render/busy/ms 82.70, stage/main/update 74.17, stream/drive/ms 58.25, stage/render/prepare_assets 45.01, stage/render/prepare_resources 13.76, stage/render/render 11.49 |
| warm-up | 4.57 | 58.5 | pipelines/compile_ms 1605, pipelines/front_ms 1557, main/busy/ms 103, pipelines/wait_ms 92.37, stage/main/update 82.42, render/thread/ms 65.86, render/busy/ms 64.29, stream/drive/ms 57.29, pipelines/driver_ms 47.80, stage/render/prepare_assets 31.88, stage/render/render 21.93, render/graph/ms 19.32 |
| warm-up | 4.62 | 51.8 | pipelines/compile_ms 759, pipelines/front_ms 737, pipelines/wait_ms 444, main/busy/ms 120, stage/main/update 101, render/thread/ms 58.15, render/busy/ms 56.43, stream/drive/ms 54.00, stage/render/render 23.34, pipelines/driver_ms 22.78, render/graph/ms 20.67, stage/render/prepare_assets 17.87 |
| warm-up | 4.73 | 107.6 | pipelines/compile_ms 1394, pipelines/front_ms 1360, pipelines/wait_ms 411, main/busy/ms 116, stage/main/update 92.86, render/thread/ms 71.64, render/busy/ms 70.20, stream/drive/ms 54.97, stage/render/prepare_assets 36.83, pipelines/driver_ms 33.85, stage/main/post_update 14.33, stage/render/render 12.91 |
| warm-up | 4.77 | 38.4 | pipelines/compile_ms 1318, pipelines/front_ms 1279, main/busy/ms 84.62, render/thread/ms 70.27, render/busy/ms 68.26, stage/main/update 59.68, stream/drive/ms 42.04, pipelines/driver_ms 39.13, stage/render/prepare_assets 32.56, stage/render/render 17.64, stage/main/post_update 15.32, render/graph/ms 15.16 |
| warm-up | 4.83 | 61.9 | pipelines/compile_ms 955, pipelines/front_ms 921, main/busy/ms 79.75, stage/main/update 60.69, render/thread/ms 58.39, render/busy/ms 56.27, stream/drive/ms 41.66, pipelines/driver_ms 34.72, stage/render/prepare_assets 22.95, stage/render/render 18.16, encode//sum 16.40, render/graph/ms 15.73 |
| warm-up | 4.86 | 33.1 | pipelines/compile_ms 449, pipelines/front_ms 434, main/busy/ms 75.30, stage/main/update 56.49, render/thread/ms 52.55, render/busy/ms 51.05, stream/drive/ms 33.33, stage/render/prepare_assets 23.37, pipelines/driver_ms 15.52, stage/render/render 13.56, render/graph/ms 11.77, stage/main/post_update 11.11 |
| warm-up | 4.92 | 57.5 | pipelines/compile_ms 584, pipelines/front_ms 566, main/busy/ms 64.09, render/thread/ms 48.40, render/busy/ms 47.05, stage/main/update 46.34, stream/drive/ms 33.43, stage/render/prepare_assets 20.39, pipelines/driver_ms 18.01, stage/render/render 13.81, render/graph/ms 11.92, encode//sum 10.20 |
| warm-up | 4.96 | 40.1 | pipelines/compile_ms 671, pipelines/front_ms 650, render/thread/ms 62.30, render/busy/ms 60.67, main/busy/ms 43.11, stage/render/render 24.61, render/graph/ms 21.75, encode//sum 21.37, pipelines/driver_ms 20.79, stage/main/post_update 16.40, stage/main/update 14.19, stage/render/prepare_assets 13.00 |
| warm-up | 5.00 | 39.3 | pipelines/compile_ms 121, pipelines/front_ms 117, render/thread/ms 75.58, render/busy/ms 73.93, main/busy/ms 49.95, stage/render/render 26.13, render/graph/ms 23.19, encode//sum 22.88, stage/main/post_update 19.26, stage/main/update 19.17, stage/render/prepare_write_phase_buffers 16.33, stage/render/prepare_bind_groups 12.95 |
| warm-up | 5.04 | 44.9 | pipelines/compile_ms 1167, pipelines/front_ms 1137, render/thread/ms 74.60, render/busy/ms 73.30, main/busy/ms 36.55, pipelines/driver_ms 29.13, stage/render/prepare_assets 24.28, stage/main/update 17.75, stage/render/render 14.21, stage/main/post_update 12.37, render/graph/ms 12.22, stage/render/prepare_bind_groups 12.09 |
| warm-up | 5.08 | 37.8 | pipelines/compile_ms 1517, pipelines/front_ms 1476, render/thread/ms 72.07, render/busy/ms 70.37, pipelines/driver_ms 40.50, stage/render/prepare_assets 36.78, main/busy/ms 30.63, stage/render/render 16.90, render/graph/ms 14.38, stage/main/update 13.24, stage/main/post_update 10.43, encode//sum 9.84 |
| warm-up | 5.11 | 29.9 | pipelines/compile_ms 839, pipelines/front_ms 803, render/thread/ms 61.66, render/busy/ms 59.89, main/busy/ms 41.77, pipelines/driver_ms 36.55, stage/render/render 23.56, render/graph/ms 20.88, stage/main/update 18.75, stage/render/prepare_assets 18.12, stage/main/post_update 16.65, stage/render/graph_submit 13.97 |
| warm-up | 5.15 | 42.7 | pipelines/compile_ms 709, pipelines/front_ms 676, render/thread/ms 59.98, render/busy/ms 58.59, main/busy/ms 42.89, pipelines/driver_ms 32.92, stage/render/render 24.39, render/graph/ms 22.12, stage/main/update 19.78, stage/render/prepare_assets 17.23, stage/main/post_update 16.53, encode//sum 15.63 |
| warm-up | 5.21 | 37.7 | render/thread/ms 50.26, render/busy/ms 48.86, main/busy/ms 39.03, stage/main/update 22.42, stage/render/prepare_assets 21.58, encode//sum 16.56, stage/render/render 15.29, render/graph/ms 13.28, stream/drive/ms 12.96, stage/main/post_update 11.04, stage/render/graph_submit 8.31, encode/submit 5.27 |
| warm-up | 5.25 | 34.0 | pipelines/compile_ms 743, pipelines/front_ms 727, render/thread/ms 58.83, render/busy/ms 57.71, main/busy/ms 36.78, stage/render/prepare_assets 35.00, stage/main/update 20.09, pipelines/driver_ms 15.46, stream/drive/ms 12.44, stage/render/render 12.38, stage/main/post_update 11.09, encode//sum 10.78 |
| warm-up | 5.56 | 33.2 | pipelines/compile_ms 117, pipelines/front_ms 109, render/thread/ms 44.90, render/busy/ms 43.16, main/busy/ms 42.53, stage/main/update 26.46, stage/render/render 22.03, render/graph/ms 19.85, stage/render/graph_submit 15.22, stream/drive/ms 13.84, stage/render/prepare_assets 12.47, stage/main/post_update 11.60 |
| warm-up | 5.69 | 27.8 | pipelines/compile_ms 161, pipelines/front_ms 155, render/thread/ms 37.23, render/busy/ms 35.89, main/busy/ms 22.34, stage/render/render 16.81, render/graph/ms 14.92, stage/render/prepare_assets 11.87, stage/main/post_update 10.92, stage/render/graph_submit 10.90, encode//sum 9.34, stage/main/update 7.63 |
| warm-up | 5.86 | 30.5 | pipelines/compile_ms 676, pipelines/front_ms 660, main/busy/ms 35.50, render/thread/ms 25.87, render/busy/ms 24.71, stage/main/update 24.35, stream/drive/ms 16.64, pipelines/driver_ms 15.36, stage/render/render 12.42, encode//sum 11.02, render/graph/ms 10.80, stage/main/post_update 7.84 |
| warm-up | 5.94 | 30.5 | pipelines/compile_ms 302, pipelines/front_ms 293, render/thread/ms 46.52, render/busy/ms 44.74, main/busy/ms 31.72, stage/render/render 19.50, render/graph/ms 17.14, stage/main/post_update 15.44, encode//sum 14.62, stage/render/graph_submit 11.98, stage/main/update 11.69, pipelines/driver_ms 9.21 |
| warm-up | 5.99 | 29.1 | render/thread/ms 34.37, render/busy/ms 33.34, main/busy/ms 20.90, stage/render/prepare_assets 14.06, stage/render/render 11.89, render/graph/ms 10.32, encode//sum 9.08, stage/main/post_update 8.11, stage/main/update 8.06, stage/render/graph_submit 7.01, encode/pass:shadow_point_light/sum 5.68, encode/submit 4.78 |
| warm-up | 6.26 | 31.0 | render/thread/ms 42.68, render/busy/ms 41.52, main/busy/ms 31.15, stage/render/prepare_assets 18.55, stage/main/update 14.12, stage/render/render 13.08, stage/main/post_update 11.41, render/graph/ms 11.35, encode//sum 10.14, encode/pass:shadow_point_light/sum 8.40, stage/render/graph_submit 7.62, encode/submit 4.70 |
| warm-up | 6.35 | 33.4 | render/thread/ms 47.48, render/busy/ms 46.36, main/busy/ms 24.78, stage/render/prepare_assets 24.13, stage/render/render 13.78, stage/main/post_update 12.45, render/graph/ms 12.09, encode//sum 9.60, stage/render/graph_submit 8.21, stage/main/update 8.06, encode/pass:shadow_point_light/sum 6.85, encode/submit 5.55 |
| warm-up | 6.40 | 29.7 | render/thread/ms 50.83, render/busy/ms 49.41, main/busy/ms 32.96, stage/render/prepare_assets 27.62, stage/main/post_update 16.32, stage/render/render 13.26, stage/main/update 12.11, render/graph/ms 11.23, encode//sum 10.15, stage/render/graph_submit 7.66, encode/pass:shadow_point_light/sum 7.65, stream/drive/ms 4.99 |
| warm-up | 6.43 | 29.9 | render/thread/ms 51.52, render/busy/ms 49.79, main/busy/ms 34.36, stage/render/prepare_assets 22.51, stage/render/render 17.79, stage/main/post_update 16.61, render/graph/ms 15.07, encode//sum 13.11, stage/main/update 11.35, stage/render/graph_submit 9.86, encode/pass:shadow_point_light/sum 9.62, encode/submit 5.82 |
| warm-up | 6.46 | 31.6 | pipelines/compile_ms 166, pipelines/front_ms 161, render/thread/ms 49.45, render/busy/ms 48.05, main/busy/ms 31.54, stage/render/prepare_assets 22.96, stage/render/render 16.20, render/graph/ms 14.02, encode//sum 12.72, stage/main/update 12.70, stage/main/post_update 12.61, encode/pass:shadow_point_light/sum 10.63 |
| warm-up | 6.51 | 46.3 | pipelines/compile_ms 179, pipelines/front_ms 174, render/thread/ms 63.14, render/busy/ms 61.97, main/busy/ms 46.46, stage/render/prepare_assets 39.23, stage/main/update 28.14, stream/drive/ms 14.83, stage/render/render 14.55, stage/main/post_update 13.24, render/graph/ms 12.66, encode//sum 11.66 |
| warm-up | 6.55 | 38.8 | render/thread/ms 72.12, render/busy/ms 70.85, main/busy/ms 48.65, stage/render/prepare_assets 46.46, stage/main/update 28.89, pipelines/compile_ms 27.57, pipelines/front_ms 26.33, stage/render/render 15.51, stream/drive/ms 14.38, render/graph/ms 13.57, stage/main/post_update 13.55, encode//sum 12.98 |
| warm-up | 6.80 | 33.4 | main/busy/ms 40.01, render/thread/ms 30.10, render/busy/ms 29.10, stage/main/update 20.82, stage/render/render 11.94, stage/main/post_update 11.30, stage/render/prepare_assets 11.23, render/graph/ms 10.50, encode//sum 8.08, stage/render/graph_submit 7.58, encode/pass:shadow_point_light/sum 6.01, encode/submit 4.94 |
| measure | 18.19 | 29.5 | main/busy/ms 37.39, stage/main/update 24.40, render/thread/ms 22.59, render/busy/ms 21.57, stream/drive/ms 20.48, encode//sum 14.79, stage/render/render 13.89, render/graph/ms 12.46, stage/main/post_update 10.11, stage/render/graph_submit 9.11, encode/submit 6.07, encode/pass:shadow_point_light/sum 3.62 |
| measure | 23.16 | 30.1 | encode//sum 60.48, render/thread/ms 44.89, render/busy/ms 43.75, main/busy/ms 30.45, stage/render/render 23.96, render/graph/ms 22.23, stage/render/graph_submit 17.14, stage/main/update 14.95, encode/pass:shadow_point_light/sum 12.05, stage/main/post_update 11.36, stage/render/prepare_assets 9.60, encode/finish_wall 8.90 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=lampcache-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 13%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **69%** (min 40%) · 2625 MHz (min 2625) · memory 8101 MHz · 73.1 W · 75 °C (max 81) · VRAM 6752 MiB · throttled 100% of samples

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
- `sf.graphics_preset=custom (preset: derived from its members)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=custom (preset: derived from its members)`
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
