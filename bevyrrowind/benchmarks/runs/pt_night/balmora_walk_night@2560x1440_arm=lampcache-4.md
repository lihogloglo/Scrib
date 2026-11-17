# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5442 frames over 65.6 s · **median 10.83 ms (92 fps)** · mean 12.06 · p95 17.38 · p99 19.15 (1% low 52 fps) · worst 32.11 ms · 449 spikes over 1.5× median · 14.6 stutters/min (work 13.7/min)
- warm-up: 323 frames over 9.3 s, worst 2490.3 ms
- hitches (> 27.1 ms): **4** measured, 46 in warm-up
- timed GPU passes: 7.00 ms of a 12.06 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.83 ms, limit 11 ms
- **FAIL** frame_ms: 16 stutters (> 1.5x moving median of 60) in 1.09 min = 14.62/min, limit 1/min
- pass worst frame 32.11 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.005 | 2.032 | 2.064 | 0.99 | 0.177 |
| main_opaque_pass_3d | 1.319 | 1.337 | 1.669 | 0.99 | 0.061 |
| auxiliary_views | 0.534 | 0.542 | 1.713 | 0.99 | 0.475 |
| early prepass | 0.501 | 0.508 | 0.718 | 0.99 | 0.087 |
| bloom | 0.311 | 0.315 | 0.321 | 0.99 | 0.063 |
| atmosphere_luts | 0.305 | 0.309 | 0.372 | 0.99 | 0.003 |
| water_velocity | 0.255 | 0.259 | 0.350 | 0.99 | 0.001 |
| water_sspr | 0.199 | 0.201 | 0.203 | 0.99 | 0.002 |
| ssao | 0.186 | 0.188 | 0.193 | 0.99 | 0.007 |
| water_fft | 0.128 | 0.130 | 0.139 | 0.99 | 0.003 |
| upscaling | 0.127 | 0.129 | 0.136 | 0.99 | 0.004 |
| uniform_allocation | 0.126 | 0.128 | 0.156 | 0.99 | 0.024 |
| atmos_froxel_inject | 0.126 | 0.128 | 0.129 | 0.99 | 0.001 |
| clouds_composite | 0.104 | 0.105 | 0.109 | 0.99 | 0.002 |
| bin_unpacking | 0.095 | 0.097 | 0.120 | 0.99 | 0.018 |
| tonemapping | 0.086 | 0.087 | 0.092 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.079 | 0.080 | 0.160 | 0.99 | 0.008 |
| clustering | 0.063 | 0.063 | 0.067 | 0.99 | 0.018 |
| clouds_trace | 0.055 | 0.056 | 0.073 | 0.99 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.99 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.414 | 0.416 | 0.11 | 0.001 |
| surface_trail | 0.042 | 0.056 | 0.057 | 0.75 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.99 | 0.004 |
| lightprobe_radiance_map | 0.025 | 0.219 | 0.229 | 0.11 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.036 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.024 | 0.99 | 0.126 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 11.897 | 25.417 | 40.39 | 64742.0 |
| render/thread/ms | 10.844 | 16.113 | 29.82 | 59012.5 |
| render/busy/ms | 10.308 | 15.536 | 29.33 | 56096.3 |
| main/busy/ms | 9.482 | 12.278 | 27.86 | 51602.3 |
| stage/render/render | 7.144 | 11.729 | 26.07 | 38878.1 |
| render/graph/ms | 6.460 | 10.930 | 25.44 | 35157.1 |
| stage/main/post_update | 6.131 | 8.519 | 11.82 | 33362.7 |
| stage/render/graph_submit | 4.954 | 8.376 | 23.02 | 26958.6 |
| encode/submit | 2.852 | 4.587 | 19.57 | 15520.3 |
| encode/finish_wall | 2.149 | 3.975 | 5.80 | 11696.3 |
| stage/main/update | 2.127 | 2.583 | 21.62 | 11573.3 |
| encode/pass:shadow_point_light/sum | 1.648 | 5.268 | 7.82 | 8968.6 |
| stage/render/camera_driver | 1.452 | 2.488 | 3.63 | 7901.7 |
| encode/pass:shadow_directional_light/sum | 0.767 | 1.303 | 2.30 | 4172.7 |
| stage/render/prepare_write_phase_buffers | 0.733 | 1.099 | 2.59 | 3991.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.671 | 1.051 | 1.69 | 3653.7 |
| encode/pass:early prepass/sum | 0.652 | 1.036 | 2.08 | 3548.1 |
| stage/render/prepare_assets | 0.604 | 0.753 | 9.10 | 3284.6 |
| bigspace/total/ms | 0.589 | 1.544 | 5.04 | 3205.4 |
| stage/render/prepare_resources | 0.449 | 0.540 | 1.49 | 2442.1 |
| render/present/ms | 0.443 | 0.549 | 2.21 | 2408.4 |
| stage/render/prepare_meshes | 0.417 | 0.533 | 2.42 | 2269.7 |
| terrain/update/ms | 0.353 | 0.582 | 2.88 | 1920.5 |
| stage/main/first | 0.350 | 0.460 | 7.50 | 1904.9 |
| stage/main/pre_update | 0.349 | 0.470 | 1.50 | 1900.7 |
| terrain/select/ms | 0.319 | 0.542 | 1.10 | 1734.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.291 | 0.470 | 1.11 | 1581.6 |
| stage/render/prepare_bind_groups | 0.274 | 0.487 | 1.72 | 1489.5 |
| controller/feed/ms | 0.264 | 0.325 | 0.76 | 1438.4 |
| stream/drive/ms | 0.247 | 0.333 | 19.64 | 1342.0 |
| bigspace/low_precision/ms | 0.239 | 0.975 | 3.54 | 1302.8 |
| stage/main/fixed_main_loop | 0.231 | 0.367 | 1.00 | 1255.9 |
| stage/render/specialize | 0.198 | 0.325 | 1.51 | 1077.0 |
| far/select/ms | 0.197 | 0.258 | 0.53 | 1074.1 |
| stage/render/create_views | 0.192 | 0.265 | 1.29 | 1047.1 |
| stage/render/queue | 0.190 | 0.329 | 2.01 | 1035.8 |
| stage/render/prepare_batch_phases | 0.189 | 0.265 | 0.89 | 1026.4 |
| stream/tick/ms | 0.188 | 0.254 | 0.62 | 1021.2 |
| bigspace/high_precision/ms | 0.187 | 1.046 | 4.64 | 1016.7 |
| stage/render/extract_commands | 0.171 | 0.201 | 0.43 | 931.3 |
| anim/evaluate/ms | 0.169 | 0.242 | 2.76 | 920.1 |
| stage/render/prepare_flush | 0.167 | 0.336 | 0.80 | 910.9 |
| anim/evaluate_cpu/ms | 0.156 | 0.191 | 0.78 | 849.0 |
| controller/move/ms | 0.146 | 0.297 | 0.49 | 793.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.143 | 0.258 | 0.85 | 775.9 |
| stage/render/prepare_views | 0.136 | 0.169 | 0.47 | 737.8 |
| pipelines/compile_ms | 0.121 | 0.000 | 131.87 | 660.7 |
| stage/main/last | 0.119 | 0.162 | 4.02 | 645.6 |
| pipelines/front_ms | 0.118 | 0.000 | 128.23 | 642.7 |
| render/acquire/ms | 0.093 | 0.118 | 0.35 | 507.7 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.119 | 0.34 | 304.5 |
| physics/step/ms | 0.051 | 0.085 | 0.57 | 276.2 |
| encode/point_shadow_cache_copies/sum | 0.050 | 0.191 | 0.67 | 273.0 |
| physics/insert/ms | 0.046 | 0.422 | 1.91 | 248.5 |
| stage/render/cleanup | 0.040 | 0.132 | 0.84 | 219.5 |
| stage/render/phase_sort | 0.040 | 0.070 | 0.29 | 216.1 |
| encode/resolve_diagnostics/sum | 0.027 | 0.037 | 0.50 | 145.1 |
| weather/map/ms | 0.016 | 0.020 | 0.21 | 85.0 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.020 | 0.34 | 69.2 |
| anim/apply/ms | 0.011 | 0.022 | 0.24 | 59.8 |
| controller/avoid/ms | 0.011 | 0.014 | 0.23 | 58.2 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.47 | 54.3 |
| weather/eval/ms | 0.009 | 0.012 | 0.24 | 51.6 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 37.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.11 | 31.1 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 28.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.16 | 27.4 |
| script/ui/ms | 0.005 | 0.006 | 0.41 | 25.3 |
| controller/write/ms | 0.003 | 0.004 | 0.09 | 18.8 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.64 | 18.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 13.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.03 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.5 |
| stage/render/prepare | 0.001 | 0.002 | 0.19 | 2.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.3 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 20 |
| controller/movers/count | count | 20 | 23 | 20 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1202 | 2262 | 528 |
| crowd/c0/meshes/count | count | 346 | 646 | 135 |
| crowd/c1/count | count | 149 | 156 | 149 |
| crowd/c1/joints/count | count | 5652 | 5992 | 5668 |
| crowd/c1/meshes/count | count | 1198 | 1308 | 1206 |
| crowd/c2/count | count | 400 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1824 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 165 | 216 | 205 |
| crowd/joints_extracted/count | count | 3742 | 8262 | 5775 |
| crowd/joints_uploaded/count | count | 3715 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.67 | 1.62 | 0.48 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.34 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.47 | 0.02 |
| encode/finish_critical | ms | 0.94 | 2.30 | 1.05 |
| encode/pass:early prepass/max | ms | 0.60 | 2.08 | 0.99 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.63 | 1.69 | 0.99 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.13 | 0.85 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 1.11 | 0.20 |
| encode/pass:shadow_directional_light/max | ms | 0.75 | 2.30 | 1.05 |
| encode/pass:shadow_point_light/max | ms | 0.72 | 1.34 | 0.34 |
| encode/point_shadow_cache_copies/max | ms | 0.09 | 0.67 | 0.18 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.50 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.12 | 29.33 | 7.93 |
| gpu/timed/ms | ms | 7.61 | 9.39 | 7.78 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 9 | 27 | 0 |
| lamp_cache/copies | count | 0 | 16 | 0 |
| lamp_cache/deferred_faces | count | 0 | 16 | 0 |
| lamp_cache/lamps | count | 5 | 8 | 0 |
| lamp_cache/mover_faces | count | 0 | 12 | 0 |
| lamp_cache/movers | count | 106 | 303 | 0 |
| lamp_cache/queued | count | 0 | 40 | 0 |
| lamp_cache/ready | count | 5 | 8 | 0 |
| lamp_cache/render/copies | count | 9 | 16 | 0 |
| lamp_cache/render/drawn | count | 9 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 7 | 8 | 0 |
| lamp_cache/render/loaded | count | 9 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 6 | 0 |
| lamp_cache/render/views | count | 12 | 32 | 0 |
| lamp_cache/stock_lamps | count | 0 | 3 | 0 |
| lamp_cache/store_starved | count | 0 | 9 | 0 |
| lamp_cache/store_used | count | 1 | 24 | 0 |
| lamp_cache/views | count | 0 | 32 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2621 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 236 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 754 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 236 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 754 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1306 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3314 | 5151 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1306 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3314 | 5151 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 90 | 19 |
| phases/shadow/bins | count | 1486 | 2902 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3349 | 5909 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 25 | 119 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 137 | 476 | 19 |
| physics/pending | count | 0 | 193 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 21 | 318 | 33 |
| preprocess/bind_groups_created | count | 21 | 318 | 33 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 278 | 131 |
| process/cpu_usage | % | 23.08 | 35.58 | 30.49 |
| process/mem_usage | GiB | 5.38 | 5.56 | 5.56 |
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
| skin/blocks_changed/count | count | 14 | 106 | 9 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 165 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 282.1 k | 811.6 k | 445.4 k |
| skin/joints_written/count | count | 4408 | 12.7 k | 6960 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4942 | 5012 | 5012 |
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
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 323 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1126 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.42 | 39.33 | 31.56 |
| system/mem_usage | % | 54.21 | 54.81 | 54.78 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 72 | 143 | 71 |
| views/all/bins | count | 4925 | 5747 | 5002 |
| views/all/count | count | 3 | 41 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.0 k | 18.7 k | 10.7 k |
| views/all/pipelines | count | 74 | 168 | 67 |
| views/all/pipelines_distinct | count | 63 | 74 | 63 |
| views/all/work_items | count | 10.2 k | 18.4 k | 11.0 k |
| views/cascade/batch_sets | count | 8 | 27 | 19 |
| views/cascade/bins | count | 1447 | 1521 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3293 | 4462 | 3038 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 332 | 4462 | 3038 |
| views/lamp_face/batch_sets | count | 0 | 67 | 0 |
| views/lamp_face/bins | count | 0 | 1401 | 0 |
| views/lamp_face/count | count | 0 | 38 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 24 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 1643 | 0 |
| views/lamp_face/pipelines | count | 0 | 96 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 1204 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2912 | 3590 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7940 | 13.0 k | 7614 |
| views/main/pipelines | count | 48 | 49 | 48 |
| views/main/work_items | count | 8237 | 13.2 k | 7922 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.3 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 49.65 | 258 | 270.2 k |
| anim/events/count | count | 0.10 | 3 | 528 |
| assets/Image/added | count | 0.11 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 7 | 22 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.75 | 152 | 4061 |
| assets/Mesh/removed | count | 0.89 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.98 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 60 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.86 | 1389 | 4675 |
| encode//count | count | 53.67 | 125 | 292.1 k |
| encode/bind_walk/init_textures | count | 16.9 k | 51.6 k | 91.93 M |
| encode/bind_walk/init_textures_skipped | count | 57.2 k | 96.9 k | 311.53 M |
| encode/bind_walk/inits | count | 503 | 782 | 2.74 M |
| encode/bind_walk/inits_skipped | count | 103 | 147 | 558.6 k |
| encode/bind_walk/merge_textures | count | 16.8 k | 51.5 k | 91.59 M |
| encode/bind_walk/merge_textures_skipped | count | 5448 | 9101 | 29.65 M |
| encode/bind_walk/merges | count | 122 | 221 | 663.3 k |
| encode/bind_walk/merges_skipped | count | 81.82 | 125 | 445.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5442 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5442 |
| encode/pass:early prepass/count | count | 1 | 1 | 5442 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5442 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5442 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5442 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5442 |
| encode/pass:shadow_point_light/count | count | 6.67 | 24 | 36.3 k |
| encode/point_shadow_cache_copies/count | count | 0.47 | 1 | 2559 |
| encode/queue/command_buffers | count | 70.82 | 160 | 385.4 k |
| encode/queue/submits | count | 2.02 | 4 | 11.0 k |
| encode/queue/write_buffer/bytes | bytes | 1.84 M | 2.77 M | 10.00 G |
| encode/queue/write_buffer/calls | count | 278 | 496 | 1.51 M |
| encode/queue/write_buffer_with/bytes | bytes | 128.8 k | 3.50 M | 700.83 M |
| encode/queue/write_buffer_with/calls | count | 11.22 | 465 | 61.1 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 64.12 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5442 |
| physics/inserts | count | 0.42 | 109 | 2265 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.43 | 145 | 2335 |
| physics/steps | count | 0.72 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 7 |
| pipelines/created | count | 0.00 | 1 | 8 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 20 | 378 |
| stream/upload/bytes | bytes | 6221 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1364 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 50, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2490.3 | main/busy/ms 2459, stage/main/post_update 2420, stage/main/update 30.57, stage/main/pre_update 4.89, stage/main/last 1.25, surface/advance_cpu_ms 0.73, stage/main/first 0.60, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.30, surface/cell_upload_ms 0.30, bigspace/total/ms 0.20, bigspace/high_precision/ms 0.18 |
| warm-up | 3.27 | 778.7 | main/busy/ms 2633, stage/main/post_update 2445, render/thread/ms 744, render/busy/ms 607, stage/render/prepare_assets 557, stage/main/update 173, stage/render/prepare_views 134, render/acquire/ms 133, stage/render/create_views 29.99, stage/render/render 12.64, render/graph/ms 7.65, stage/main/pre_update 6.60 |
| warm-up | 3.38 | 110.6 | render/thread/ms 833, render/busy/ms 646, stage/render/prepare_assets 585, main/busy/ms 193, stage/main/update 147, stage/render/prepare_views 134, render/acquire/ms 134, pipelines/compile_ms 93.31, stage/render/render 67.56, pipelines/front_ms 54.28, render/present/ms 54.03, pipelines/driver_ms 39.02 |
| warm-up | 3.43 | 47.8 | pipelines/compile_ms 530, pipelines/front_ms 442, pipelines/wait_ms 354, render/thread/ms 136, stage/render/render 95.47, pipelines/driver_ms 87.57, render/busy/ms 74.46, render/present/ms 61.05, main/busy/ms 60.15, stage/main/post_update 45.39, render/graph/ms 33.47, stage/render/prepare_assets 27.78 |
| warm-up | 3.46 | 31.7 | pipelines/wait_ms 1128, pipelines/compile_ms 656, pipelines/front_ms 569, main/busy/ms 117, pipelines/driver_ms 86.91, stage/main/post_update 81.16, render/thread/ms 75.31, render/busy/ms 63.27, stage/render/render 50.73, render/graph/ms 38.48, stage/render/graph_submit 22.04, stage/main/update 16.07 |
| warm-up | 3.60 | 138.8 | pipelines/wait_ms 3677, pipelines/compile_ms 1466, pipelines/front_ms 1349, main/busy/ms 159, render/thread/ms 119, pipelines/driver_ms 117, render/busy/ms 117, stage/main/update 66.79, stage/main/post_update 59.55, stage/render/render 29.13, stage/render/prepare_bind_groups 29.01, stage/render/prepare_resources 27.93 |
| warm-up | 3.66 | 65.5 | pipelines/wait_ms 3116, pipelines/compile_ms 1477, pipelines/front_ms 1388, render/thread/ms 152, render/busy/ms 150, main/busy/ms 102, pipelines/driver_ms 88.57, stage/main/update 59.05, stage/render/prepare_assets 58.04, stage/render/prepare_bind_groups 29.19, stage/render/prepare_resources 27.37, stage/render/render 26.64 |
| warm-up | 3.69 | 28.4 | pipelines/compile_ms 1113, pipelines/front_ms 1073, pipelines/wait_ms 316, render/thread/ms 88.04, render/busy/ms 85.09, stage/render/prepare_assets 65.50, pipelines/driver_ms 39.74, main/busy/ms 36.19, stage/main/post_update 17.41, terrain/update/ms 12.61, stage/render/render 12.01, stage/main/update 11.21 |
| warm-up | 3.72 | 31.7 | pipelines/compile_ms 1487, pipelines/front_ms 1427, pipelines/wait_ms 165, pipelines/driver_ms 60.05, render/thread/ms 55.25, render/busy/ms 53.35, main/busy/ms 29.17, stage/render/prepare_assets 28.43, stage/render/render 16.85, stage/main/post_update 15.45, render/graph/ms 14.78, encode//sum 13.86 |
| warm-up | 3.76 | 32.8 | pipelines/compile_ms 676, pipelines/front_ms 642, pipelines/wait_ms 71.75, render/thread/ms 59.84, render/busy/ms 57.83, pipelines/driver_ms 34.55, main/busy/ms 32.58, encode//sum 32.22, stage/render/render 22.08, stage/main/post_update 21.88, render/graph/ms 19.61, stage/render/graph_submit 14.76 |
| warm-up | 3.81 | 31.8 | pipelines/compile_ms 683, pipelines/front_ms 643, pipelines/wait_ms 137, main/busy/ms 60.54, render/thread/ms 46.77, stage/main/update 44.93, render/busy/ms 44.45, stream/drive/ms 41.18, pipelines/driver_ms 40.38, stage/render/prepare_assets 19.17, stage/render/render 18.21, render/graph/ms 15.48 |
| warm-up | 3.86 | 58.0 | pipelines/compile_ms 1254, pipelines/front_ms 1194, pipelines/driver_ms 60.53, render/thread/ms 59.99, main/busy/ms 59.64, render/busy/ms 58.30, stage/main/update 45.85, stream/drive/ms 41.78, stage/render/prepare_assets 36.66, stage/render/render 16.62, render/graph/ms 14.37, stage/main/post_update 8.53 |
| warm-up | 3.94 | 27.4 | pipelines/compile_ms 477, pipelines/front_ms 461, main/busy/ms 49.15, render/thread/ms 38.50, render/busy/ms 36.77, stage/main/update 32.32, stream/drive/ms 28.00, stage/render/prepare_assets 16.32, pipelines/driver_ms 16.32, stage/render/render 14.62, render/graph/ms 12.48, stage/main/post_update 11.59 |
| warm-up | 4.00 | 67.7 | pipelines/compile_ms 692, pipelines/front_ms 673, render/thread/ms 66.83, render/busy/ms 65.15, main/busy/ms 49.31, stage/render/prepare_assets 48.57, stage/main/update 35.32, stream/drive/ms 30.32, pipelines/driver_ms 18.92, stage/render/render 11.62, render/graph/ms 9.51, stage/main/post_update 7.77 |
| warm-up | 4.10 | 80.0 | pipelines/compile_ms 425, pipelines/front_ms 408, render/thread/ms 65.54, render/busy/ms 64.06, stage/render/prepare_assets 46.93, main/busy/ms 37.93, stage/main/update 18.92, pipelines/driver_ms 16.71, stage/main/post_update 12.09, stage/render/render 9.87, render/graph/ms 7.86, encode//sum 6.76 |
| warm-up | 4.17 | 63.6 | pipelines/compile_ms 997, pipelines/front_ms 927, render/thread/ms 108, render/busy/ms 106, main/busy/ms 85.09, pipelines/driver_ms 69.60, stage/main/update 63.19, stage/render/prepare_assets 52.98, stage/render/render 41.41, render/graph/ms 39.02, encode//sum 36.91, stage/render/graph_submit 34.96 |
| warm-up | 4.26 | 93.4 | pipelines/compile_ms 1373, pipelines/front_ms 1275, pipelines/wait_ms 124, render/thread/ms 121, render/busy/ms 118, pipelines/driver_ms 97.53, main/busy/ms 77.57, stage/main/update 56.24, stage/render/render 55.82, render/graph/ms 53.17, stage/render/graph_submit 48.21, stage/render/prepare_assets 45.81 |
| warm-up | 4.31 | 50.6 | pipelines/compile_ms 1180, pipelines/front_ms 1122, pipelines/wait_ms 225, render/thread/ms 110, render/busy/ms 108, main/busy/ms 62.72, pipelines/driver_ms 57.39, stage/render/prepare_assets 40.70, stage/render/render 34.23, stage/main/post_update 27.33, render/graph/ms 23.21, stage/main/update 19.78 |
| warm-up | 4.37 | 57.2 | pipelines/compile_ms 1466, pipelines/front_ms 1417, pipelines/wait_ms 244, render/thread/ms 93.29, render/busy/ms 91.51, main/busy/ms 86.63, pipelines/driver_ms 49.22, stage/main/update 36.18, stage/main/post_update 25.85, stage/render/render 21.09, stage/render/prepare_write_phase_buffers 19.56, stage/main/fixed_main_loop 16.01 |
| warm-up | 4.45 | 84.2 | pipelines/compile_ms 1510, pipelines/front_ms 1461, pipelines/wait_ms 181, render/thread/ms 115, render/busy/ms 113, main/busy/ms 78.85, pipelines/driver_ms 48.66, stage/render/prepare_assets 38.04, stage/main/update 30.64, stage/main/post_update 30.60, stage/render/prepare_write_phase_buffers 16.96, anim/evaluate/ms 16.85 |
| warm-up | 4.52 | 65.4 | pipelines/compile_ms 1337, pipelines/front_ms 1302, render/thread/ms 121, render/busy/ms 119, pipelines/wait_ms 116, main/busy/ms 105, stage/main/update 66.69, stream/drive/ms 57.04, stage/render/prepare_assets 55.79, pipelines/driver_ms 35.31, stage/main/post_update 31.51, stage/render/prepare_resources 19.91 |
| warm-up | 4.58 | 61.3 | pipelines/compile_ms 2378, pipelines/front_ms 2304, pipelines/wait_ms 358, main/busy/ms 86.84, render/thread/ms 81.76, render/busy/ms 80.04, pipelines/driver_ms 73.38, stage/main/update 66.46, stream/drive/ms 56.50, stage/render/prepare_assets 45.79, stage/render/render 11.71, stage/render/prepare_resources 11.70 |
| warm-up | 4.73 | 126.4 | pipelines/compile_ms 884, pipelines/front_ms 862, main/busy/ms 105, stage/main/update 88.18, render/thread/ms 59.33, render/busy/ms 57.55, stream/drive/ms 46.36, pipelines/wait_ms 44.33, stage/render/prepare_assets 32.22, pipelines/driver_ms 21.91, stage/render/render 17.31, render/graph/ms 15.09 |
| warm-up | 4.79 | 59.3 | pipelines/compile_ms 1088, pipelines/front_ms 1049, render/thread/ms 96.59, main/busy/ms 91.09, render/busy/ms 75.27, stage/main/update 63.25, stage/render/render 46.84, stream/drive/ms 44.63, encode//sum 40.74, pipelines/driver_ms 38.78, stage/render/prepare_assets 31.11, render/graph/ms 24.95 |
| warm-up | 4.85 | 67.2 | pipelines/compile_ms 1019, pipelines/front_ms 966, main/busy/ms 94.01, render/thread/ms 90.09, render/busy/ms 69.13, stage/main/update 65.08, pipelines/driver_ms 52.38, pipelines/wait_ms 51.35, stage/render/render 45.82, stream/drive/ms 44.55, encode//sum 42.07, render/graph/ms 24.14 |
| warm-up | 4.95 | 74.8 | pipelines/compile_ms 1743, pipelines/front_ms 1708, main/busy/ms 76.12, stage/main/update 57.86, render/thread/ms 46.64, render/busy/ms 45.06, pipelines/driver_ms 34.77, stream/drive/ms 33.51, stage/render/prepare_assets 18.22, stage/render/render 15.80, render/graph/ms 13.70, encode//sum 13.62 |
| warm-up | 4.99 | 33.7 | pipelines/compile_ms 2040, pipelines/front_ms 1994, render/thread/ms 55.28, render/busy/ms 53.47, pipelines/driver_ms 45.24, main/busy/ms 41.17, stage/render/prepare_assets 20.74, stage/main/update 19.98, stage/render/render 15.19, stage/main/post_update 14.34, render/graph/ms 13.07, encode//sum 12.19 |
| warm-up | 5.04 | 27.7 | pipelines/compile_ms 238, pipelines/front_ms 231, render/thread/ms 43.55, render/busy/ms 41.99, pipelines/wait_ms 39.02, main/busy/ms 27.73, stage/render/render 16.35, render/graph/ms 14.28, stage/main/update 13.63, encode//sum 10.58, stage/render/graph_submit 10.04, stage/main/post_update 8.72 |
| warm-up | 5.07 | 27.5 | pipelines/compile_ms 140, pipelines/front_ms 131, render/thread/ms 47.11, render/busy/ms 45.46, main/busy/ms 28.01, stage/render/render 19.86, render/graph/ms 17.79, stage/main/update 13.32, stage/render/graph_submit 12.88, encode//sum 12.36, encode/submit 9.40, stage/main/post_update 9.14 |
| warm-up | 5.15 | 52.6 | pipelines/compile_ms 514, pipelines/front_ms 498, render/thread/ms 67.43, render/busy/ms 65.31, main/busy/ms 40.36, stage/render/render 30.09, render/graph/ms 27.39, stage/render/prepare_assets 25.04, stage/main/update 21.34, stage/render/graph_submit 21.21, pipelines/driver_ms 16.50, encode//sum 15.48 |
| warm-up | 5.19 | 48.3 | pipelines/compile_ms 567, pipelines/front_ms 550, render/thread/ms 83.64, render/busy/ms 82.15, main/busy/ms 50.09, stage/render/prepare_assets 38.23, stage/main/update 27.94, stage/render/render 23.78, render/graph/ms 21.63, pipelines/driver_ms 16.32, stage/render/graph_submit 15.74, stream/drive/ms 14.76 |
| warm-up | 5.24 | 47.1 | pipelines/compile_ms 445, pipelines/front_ms 432, render/thread/ms 81.40, render/busy/ms 80.05, main/busy/ms 62.01, stage/render/prepare_assets 32.01, stage/main/update 31.51, stage/render/render 17.84, render/graph/ms 15.86, stage/main/post_update 14.13, stream/drive/ms 13.90, stage/render/prepare_write_phase_buffers 13.24 |
| warm-up | 5.27 | 30.9 | pipelines/compile_ms 237, pipelines/front_ms 231, render/thread/ms 68.92, render/busy/ms 67.59, main/busy/ms 46.01, stage/render/render 18.17, stage/main/post_update 16.89, render/graph/ms 16.32, stage/render/prepare_assets 15.85, stage/main/update 15.09, stage/render/prepare_write_phase_buffers 13.08, encode//sum 12.00 |
| warm-up | 5.32 | 49.6 | pipelines/compile_ms 409, pipelines/front_ms 397, render/thread/ms 74.72, render/busy/ms 73.26, main/busy/ms 44.07, stage/main/post_update 28.95, stage/render/prepare_assets 24.53, stage/render/render 18.37, render/graph/ms 16.13, stage/render/prepare_write_phase_buffers 15.25, encode//sum 12.88, pipelines/driver_ms 12.13 |
| warm-up | 5.35 | 28.1 | pipelines/compile_ms 1631, pipelines/front_ms 1588, render/thread/ms 72.56, render/busy/ms 70.73, main/busy/ms 47.03, pipelines/driver_ms 43.18, stage/render/prepare_assets 29.35, stage/main/post_update 28.18, stage/render/render 19.99, encode//sum 18.79, render/graph/ms 17.51, stage/main/update 12.48 |
| warm-up | 5.49 | 30.0 | render/thread/ms 38.62, render/busy/ms 37.62, main/busy/ms 24.83, stage/render/prepare_assets 20.43, stage/render/render 11.52, stage/main/update 10.33, render/graph/ms 10.16, stage/main/post_update 9.68, encode//sum 9.65, stage/render/graph_submit 6.89, stream/drive/ms 5.56, encode/pass:shadow_point_light/sum 4.76 |
| warm-up | 5.99 | 29.9 | render/thread/ms 41.87, render/busy/ms 39.86, main/busy/ms 29.98, stage/render/render 18.26, render/graph/ms 16.02, stage/main/update 13.94, encode//sum 13.30, stage/main/post_update 12.16, stage/render/graph_submit 10.87, encode/submit 7.83, encode/pass:shadow_point_light/sum 7.16, stage/render/prepare_assets 5.21 |
| warm-up | 6.04 | 28.8 | render/thread/ms 37.88, render/busy/ms 36.90, main/busy/ms 25.74, stage/render/prepare_assets 12.75, stage/render/render 12.25, render/graph/ms 10.79, encode//sum 10.10, stage/main/update 10.05, stage/main/post_update 9.78, stage/render/graph_submit 7.20, encode/pass:shadow_point_light/sum 6.53, encode/submit 4.61 |
| warm-up | 6.25 | 29.4 | pipelines/compile_ms 133, pipelines/front_ms 130, render/thread/ms 38.09, main/busy/ms 37.12, render/busy/ms 37.01, stage/main/update 19.43, stage/render/render 16.00, render/graph/ms 14.15, encode//sum 12.61, stage/main/post_update 12.31, stage/render/graph_submit 9.14, encode/pass:shadow_point_light/sum 9.12 |
| warm-up | 6.28 | 29.5 | pipelines/compile_ms 143, pipelines/front_ms 139, render/thread/ms 48.38, render/busy/ms 47.03, main/busy/ms 40.43, stage/render/render 19.94, stage/main/update 17.99, render/graph/ms 17.79, stage/main/post_update 14.56, stage/render/graph_submit 13.22, encode//sum 13.19, stage/render/prepare_assets 10.38 |
| warm-up | 6.31 | 27.1 | render/thread/ms 45.94, render/busy/ms 44.66, main/busy/ms 36.82, stage/render/render 17.96, stage/main/update 16.80, render/graph/ms 16.19, stage/render/prepare_assets 15.29, stage/main/post_update 13.51, stage/render/graph_submit 12.12, encode//sum 11.97, pipelines/compile_ms 9.80, pipelines/front_ms 9.06 |
| warm-up | 6.37 | 30.0 | render/thread/ms 47.22, render/busy/ms 46.00, main/busy/ms 32.61, stage/render/render 19.39, render/graph/ms 17.81, encode//sum 13.88, stage/main/post_update 13.67, stage/main/update 13.39, encode/pass:shadow_point_light/sum 11.56, stage/render/graph_submit 11.38, stage/render/prepare_assets 10.37, encode/submit 7.57 |
| warm-up | 6.42 | 31.7 | pipelines/compile_ms 144, pipelines/front_ms 140, render/thread/ms 40.43, render/busy/ms 39.19, main/busy/ms 33.74, stage/render/prepare_assets 18.24, stage/main/update 17.30, stage/main/post_update 11.18, stage/render/render 10.17, encode//sum 8.87, render/graph/ms 8.68, encode/pass:shadow_point_light/sum 5.84 |
| warm-up | 6.56 | 31.4 | render/thread/ms 47.76, render/busy/ms 46.55, main/busy/ms 44.81, stage/main/update 24.10, stage/render/render 17.28, stage/render/prepare_assets 17.19, stream/drive/ms 16.64, stage/main/post_update 16.53, render/graph/ms 15.36, encode//sum 12.27, encode/pass:shadow_point_light/sum 10.30, stage/render/graph_submit 9.98 |
| warm-up | 6.59 | 28.7 | render/thread/ms 54.82, render/busy/ms 53.72, main/busy/ms 41.44, stage/render/prepare_assets 27.50, stage/main/update 23.95, stream/drive/ms 16.16, stage/render/render 14.81, render/graph/ms 13.00, stage/main/post_update 12.52, encode//sum 11.79, encode/pass:shadow_point_light/sum 8.93, stage/render/graph_submit 8.29 |
| warm-up | 6.85 | 30.7 | pipelines/compile_ms 165, pipelines/front_ms 161, main/busy/ms 38.63, render/thread/ms 31.16, render/busy/ms 30.09, stage/main/update 20.28, stage/render/render 12.64, stage/render/prepare_assets 11.73, stage/main/post_update 11.69, render/graph/ms 11.20, encode//sum 9.63, stage/render/graph_submit 7.93 |
| measure | 15.48 | 31.1 | render/thread/ms 45.89, encode//sum 45.80, render/busy/ms 44.69, stage/render/render 38.07, render/graph/ms 36.61, stage/render/graph_submit 31.64, encode/submit 24.67, main/busy/ms 23.12, stage/main/post_update 16.20, encode/pass:shadow_point_light/sum 10.41, encode/finish_wall 7.05, stage/render/camera_driver 4.86 |
| measure | 18.19 | 32.1 | encode//sum 44.87, main/busy/ms 39.60, render/thread/ms 28.56, render/busy/ms 27.44, stage/main/update 23.75, stream/drive/ms 19.87, stage/render/render 19.60, render/graph/ms 18.19, stage/render/graph_submit 14.23, stage/main/post_update 13.09, encode/finish_wall 7.16, encode/submit 7.15 |
| measure | 18.29 | 29.0 | encode//sum 58.86, render/thread/ms 43.17, render/busy/ms 42.00, main/busy/ms 26.77, stage/render/render 22.24, render/graph/ms 20.38, stage/render/graph_submit 15.18, stage/main/post_update 14.53, stage/render/prepare_assets 9.43, encode/pass:shadow_point_light/sum 8.66, stage/main/update 8.44, encode/finish_wall 8.39 |
| measure | 23.11 | 27.4 | encode//sum 47.97, render/thread/ms 41.60, render/busy/ms 40.55, main/busy/ms 31.62, stage/render/render 20.24, render/graph/ms 18.45, stage/main/post_update 15.50, stage/render/graph_submit 13.07, stage/main/update 12.20, stage/render/prepare_assets 9.67, encode/pass:shadow_point_light/sum 7.92, encode/finish_wall 7.23 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=lampcache-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 8%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **68%** (min 47%) · 2624 MHz (min 2610) · memory 8101 MHz · 70.9 W · 74 °C (max 80) · VRAM 7046 MiB · throttled 100% of samples

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
