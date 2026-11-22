# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6455 frames over 65.6 s · **median 10.02 ms (100 fps)** · mean 10.17 · p95 11.83 · p99 13.98 (1% low 72 fps) · worst 30.98 ms · 35 spikes over 1.5× median · 28.3 stutters/min (work 23.8/min)
- warm-up: 353 frames over 9.0 s, worst 2425.7 ms
- hitches (> 25.1 ms): **3** measured, 53 in warm-up
- timed GPU passes: 6.58 ms of a 10.17 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.02 ms, limit 11 ms
- **FAIL** frame_ms: 31 stutters (> 1.5x moving median of 60) in 1.09 min = 28.33/min, limit 1/min
- pass worst frame 30.98 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.891 | 2.036 | 2.071 | 0.93 | 0.176 |
| main_opaque_pass_3d | 1.511 | 1.627 | 2.360 | 0.93 | 0.079 |
| early prepass | 0.476 | 0.512 | 0.754 | 0.93 | 0.090 |
| bloom | 0.295 | 0.318 | 0.326 | 0.93 | 0.063 |
| atmosphere_luts | 0.292 | 0.314 | 0.378 | 0.93 | 0.003 |
| water_velocity | 0.209 | 0.225 | 0.247 | 0.93 | 0.002 |
| water_sspr | 0.188 | 0.202 | 0.203 | 0.93 | 0.002 |
| ssao | 0.175 | 0.188 | 0.191 | 0.93 | 0.008 |
| point_shadow_atlas | 0.148 | 0.265 | 0.402 | 0.56 | 0.210 |
| upscaling | 0.119 | 0.128 | 0.136 | 0.93 | 0.004 |
| atmos_froxel_inject | 0.119 | 0.128 | 0.130 | 0.93 | 0.002 |
| water_fft | 0.116 | 0.125 | 0.128 | 0.93 | 0.003 |
| uniform_allocation | 0.108 | 0.117 | 0.141 | 0.93 | 0.025 |
| clouds_composite | 0.098 | 0.105 | 0.109 | 0.93 | 0.002 |
| gpu_instance_materialization | 0.090 | 0.097 | 0.101 | 0.93 | 0.004 |
| bin_unpacking | 0.086 | 0.092 | 0.113 | 0.93 | 0.019 |
| tonemapping | 0.080 | 0.086 | 0.092 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.071 | 0.132 | 0.93 | 0.009 |
| clustering | 0.059 | 0.064 | 0.067 | 0.93 | 0.019 |
| clouds_trace | 0.052 | 0.056 | 0.078 | 0.93 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.054 | 0.071 | 0.93 | 0.004 |
| render_sky | 0.049 | 0.053 | 0.054 | 0.93 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.037 | 0.416 | 0.417 | 0.09 | 0.001 |
| surface_trail | 0.036 | 0.057 | 0.057 | 0.62 | 0.002 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.93 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.027 | 0.047 | 0.074 | 0.56 | 0.012 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| range_unpacking | 0.021 | 0.022 | 0.028 | 0.93 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.019 | 0.219 | 0.230 | 0.09 | 0.004 |
| point_shadow_atlas/uniform_allocation | 0.016 | 0.029 | 0.053 | 0.56 | 0.010 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.93 | 0.006 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.030 | 0.93 | 0.184 |
| point_shadow_atlas/bin_unpacking | 0.011 | 0.021 | 0.035 | 0.56 | 0.007 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.93 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.93 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.004 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.56 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.93 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.995 | 10.423 | 19.93 | 58059.7 |
| render/busy/ms | 8.489 | 9.959 | 19.42 | 54795.0 |
| main/busy/ms | 8.284 | 9.563 | 29.75 | 53471.9 |
| stage/render/render | 5.226 | 6.218 | 11.00 | 33733.7 |
| stage/main/post_update | 4.716 | 5.573 | 10.83 | 30440.3 |
| render/graph/ms | 4.562 | 5.489 | 9.96 | 29444.9 |
| encode//sum | 3.500 | 4.212 | 11.79 | 22590.5 |
| stage/render/graph_submit | 3.230 | 4.043 | 8.64 | 20849.1 |
| encode/submit | 1.991 | 2.775 | 7.31 | 12854.0 |
| stage/main/update | 1.864 | 2.367 | 23.45 | 12034.1 |
| encode/finish_wall | 1.312 | 1.772 | 3.05 | 8466.1 |
| stage/render/camera_driver | 1.268 | 1.515 | 2.99 | 8184.1 |
| encode/pass:shadow_directional_light/sum | 0.859 | 1.258 | 2.11 | 5545.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.852 | 1.119 | 1.99 | 5502.3 |
| encode/pass:early prepass/sum | 0.852 | 1.096 | 1.86 | 5497.8 |
| stage/render/prepare_write_phase_buffers | 0.625 | 0.930 | 2.22 | 4033.5 |
| bigspace/total/ms | 0.566 | 1.360 | 2.62 | 3654.9 |
| stage/render/prepare_assets | 0.527 | 0.664 | 9.63 | 3400.7 |
| stage/main/fixed_main_loop | 0.513 | 0.898 | 1.88 | 3313.5 |
| stage/render/prepare_meshes | 0.450 | 0.558 | 4.39 | 2903.0 |
| stage/render/prepare_resources | 0.436 | 0.516 | 1.17 | 2815.4 |
| render/present/ms | 0.416 | 1.176 | 1.96 | 2686.4 |
| stage/main/pre_update | 0.388 | 0.513 | 3.42 | 2505.2 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.374 | 1.450 | 2.92 | 2413.6 |
| stage/main/first | 0.361 | 0.476 | 5.49 | 2328.2 |
| controller/feed/ms | 0.339 | 0.423 | 0.93 | 2185.3 |
| stage/render/prepare_bind_groups | 0.323 | 0.538 | 1.85 | 2082.8 |
| stage/render/prepare_flush | 0.317 | 0.512 | 2.15 | 2047.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.283 | 0.451 | 0.92 | 1830.0 |
| stage/render/prepare_batch_phases | 0.283 | 0.426 | 1.15 | 1829.0 |
| stage/main/last | 0.264 | 0.311 | 1.73 | 1702.1 |
| bigspace/low_precision/ms | 0.250 | 0.791 | 2.19 | 1616.1 |
| stream/drive/ms | 0.214 | 0.294 | 10.37 | 1378.4 |
| stage/render/create_views | 0.200 | 0.272 | 0.52 | 1293.4 |
| stage/render/extract_commands | 0.198 | 0.228 | 0.49 | 1276.2 |
| anim/evaluate/ms | 0.166 | 0.305 | 1.57 | 1074.2 |
| bigspace/high_precision/ms | 0.166 | 0.716 | 1.99 | 1070.7 |
| stage/render/specialize | 0.163 | 0.256 | 1.73 | 1051.4 |
| stream/tick/ms | 0.159 | 0.230 | 0.52 | 1023.2 |
| anim/evaluate_cpu/ms | 0.156 | 0.194 | 0.77 | 1009.1 |
| stage/render/prepare_views | 0.133 | 0.173 | 0.52 | 857.0 |
| stage/render/queue | 0.105 | 0.178 | 1.40 | 679.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.156 | 0.60 | 648.0 |
| render/acquire/ms | 0.090 | 0.119 | 0.32 | 578.3 |
| stage/render/phase_sort | 0.080 | 0.110 | 0.28 | 517.5 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.077 | 0.879 | 1.36 | 496.5 |
| controller/write/ms | 0.073 | 0.098 | 0.35 | 472.2 |
| controller/move/ms | 0.072 | 0.179 | 0.31 | 462.4 |
| terrain/update/ms | 0.060 | 0.294 | 1.81 | 388.7 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.056 | 0.000 | 2.56 | 364.4 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.071 | 0.30 | 297.8 |
| physics/step/ms | 0.040 | 0.082 | 0.67 | 260.2 |
| encode/resolve_diagnostics/sum | 0.034 | 0.046 | 0.48 | 219.3 |
| pipelines/compile_ms | 0.026 | 0.000 | 81.15 | 169.3 |
| pipelines/front_ms | 0.025 | 0.000 | 78.63 | 164.0 |
| physics/insert/ms | 0.025 | 0.307 | 0.82 | 163.8 |
| stage/render/cleanup | 0.023 | 0.031 | 0.37 | 148.4 |
| terrain/select/ms | 0.023 | 0.260 | 0.79 | 148.4 |
| weather/map/ms | 0.017 | 0.024 | 0.24 | 112.9 |
| far/select/ms | 0.015 | 0.169 | 0.40 | 97.5 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.47 | 89.3 |
| anim/apply/ms | 0.012 | 0.021 | 0.09 | 79.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.45 | 73.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.17 | 67.5 |
| script/update/ms | 0.008 | 0.011 | 0.20 | 52.7 |
| controller/avoid/ms | 0.006 | 0.012 | 0.18 | 40.3 |
| stream/despawn/ms | 0.006 | 0.000 | 1.21 | 36.1 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 34.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.15 | 22.3 |
| script/ui/ms | 0.003 | 0.004 | 0.04 | 21.2 |
| stream/upload/ms | 0.003 | 0.000 | 1.30 | 19.9 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 12.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.07 | 6.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.58 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.13 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 19 |
| controller/movers/count | count | 19 | 21 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1292 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5531 | 6039 | 5661 |
| crowd/c1/meshes/count | count | 1181 | 1313 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 205 |
| crowd/joints_extracted/count | count | 702 | 5413 | 648 |
| crowd/joints_uploaded/count | count | 3580 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.43 | 1.50 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.47 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.45 | 0.01 |
| encode/finish_critical | ms | 1.01 | 2.92 | 1.30 |
| encode/pass:early prepass/max | ms | 0.84 | 1.86 | 0.85 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.86 | 1.99 | 0.70 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.60 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 0.92 | 0.30 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.79 | 2.92 | 0.99 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.81 | 1.36 | 0.17 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.38 | 2.56 | 1.10 |
| encode/pass:shadow_directional_light/max | ms | 0.83 | 2.11 | 1.30 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.48 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.70 | 29.75 | 9.08 |
| gpu/timed/ms | ms | 7.91 | 9.37 | 8.72 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 325 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 325 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2479 | 3527 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2479 | 3527 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 59 | 358 | 27 |
| phases/shadow/bins | count | 865 | 1294 | 658 |
| phases/shadow/instance_batches | count | 4 | 14 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 98.3 k |
| phases/shadow/items | count | 2340 | 3043 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 52 | 133 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2678 | 3183 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 175 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 15 | 293 | 43 |
| preprocess/bind_groups_created | count | 10 | 287 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 6 |
| preprocess/bind_groups_reused | count | 139 | 213 | 123 |
| process/cpu_usage | % | 25 | 39.42 | 25 |
| process/mem_usage | GiB | 5.08 | 5.35 | 5.34 |
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
| shadow_atlas/lamps | count | 8 | 20 | 0 |
| shadow_atlas/live_occ_pct | % | 10.25 | 42.09 | 0 |
| shadow_atlas/live_tiles | count | 16 | 68 | 0 |
| shadow_atlas/mover_draws | count | 9 | 38 | 0 |
| shadow_atlas/movers | count | 122 | 380 | 0 |
| shadow_atlas/moving_lamps | count | 1 | 4 | 0 |
| shadow_atlas/render/blits | count | 9 | 38 | 0 |
| shadow_atlas/render/clears | count | 0 | 15 | 0 |
| shadow_atlas/render/lamps | count | 8 | 20 | 0 |
| shadow_atlas/render/live_direct | count | 9 | 38 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 9 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 17 | 60 | 0 |
| shadow_atlas/resizes | count | 30 | 39 | 39 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.82 | 0 |
| shadow_atlas/static_tiles | count | 42 | 108 | 0 |
| shadow_atlas/views | count | 17 | 60 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 91 | 14 |
| skin/blocks_skipped/count | count | 145 | 206 | 191 |
| skin/blocks_walked/count | count | 14 | 26 | 14 |
| skin/joint_bytes_written/bytes | bytes | 49.4 k | 808.2 k | 47.4 k |
| skin/joints_written/count | count | 772 | 12.6 k | 740 |
| skin/upload_writes/count | count | 8 | 23 | 10 |
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
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.33 | 38.35 | 28.60 |
| system/mem_usage | % | 46.13 | 46.98 | 46.96 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 118 | 411 | 85 |
| views/all/bins | count | 2922 | 3284 | 2948 |
| views/all/count | count | 20 | 63 | 3 |
| views/all/instance_batches | count | 12 | 22 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 196.6 k |
| views/all/items | count | 8022 | 10.0 k | 7964 |
| views/all/pipelines | count | 99 | 177 | 66 |
| views/all/pipelines_distinct | count | 66 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 231.1 k | 205.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 27 |
| views/cascade/bins | count | 716 | 861 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2116 | 2424 | 2022 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 45 | 346 | 0 |
| views/lamp_face/bins | count | 115 | 538 | 0 |
| views/lamp_face/count | count | 17 | 60 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 168 | 671 | 0 |
| views/lamp_face/pipelines | count | 26 | 108 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 121 | 25.0 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 1998 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7492 | 5942 |
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
| anim/bones_written/count | count | 47.79 | 213 | 308.5 k |
| anim/events/count | count | 0.08 | 3 | 546 |
| assets/Image/added | count | 0.09 | 39 | 586 |
| assets/Image/modified | count | 0.00 | 9 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.45 | 242 | 2874 |
| assets/Mesh/removed | count | 0.75 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.16 | 10 | 7481 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.23 | 1154 | 1489 |
| encode//count | count | 27.92 | 30 | 180.2 k |
| encode/bind_walk/init_textures | count | 9411 | 23.9 k | 60.75 M |
| encode/bind_walk/init_textures_skipped | count | 49.8 k | 70.5 k | 321.23 M |
| encode/bind_walk/inits | count | 568 | 766 | 3.67 M |
| encode/bind_walk/inits_skipped | count | 166 | 264 | 1.07 M |
| encode/bind_walk/merge_textures | count | 9349 | 23.8 k | 60.34 M |
| encode/bind_walk/merge_textures_skipped | count | 9547 | 21.2 k | 61.63 M |
| encode/bind_walk/merges | count | 106 | 173 | 684.6 k |
| encode/bind_walk/merges_skipped | count | 145 | 219 | 937.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6455 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6455 |
| encode/pass:early prepass/count | count | 1 | 1 | 6455 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6455 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6455 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6455 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.41 | 1 | 2645 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.12 | 1 | 762 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 257 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6455 |
| encode/queue/command_buffers | count | 38.50 | 42 | 248.5 k |
| encode/queue/submits | count | 2.01 | 4 | 13.0 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.29 M | 15.03 G |
| encode/queue/write_buffer/calls | count | 275 | 461 | 1.78 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.3 k | 1.30 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 10.32 | 735 | 66.6 k |
| encode/queue/write_texture/bytes | bytes | 2044 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6455 |
| physics/inserts | count | 0.35 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 177 | 2335 |
| physics/steps | count | 0.61 | 2 | 3939 |
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
| stream/deactivate/items | count | 0.03 | 38 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5244 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1150 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.1 ms; slowest 56, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.43 | 2425.7 | main/busy/ms 2397, stage/main/post_update 2357, stage/main/update 30.43, stage/main/pre_update 6.66, stage/main/last 1.08, surface/advance_cpu_ms 0.77, stage/main/first 0.55, stream/drive/ms 0.37, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.30, bigspace/total/ms 0.10, stream/tick/ms 0.07 |
| warm-up | 3.12 | 690.8 | main/busy/ms 2530, stage/main/post_update 2382, render/thread/ms 653, render/busy/ms 546, stage/render/prepare_assets 497, stage/main/update 131, stage/render/prepare_views 104, render/acquire/ms 104, stage/render/create_views 29.59, stage/render/render 11.12, stage/main/pre_update 8.33, render/graph/ms 7.25 |
| warm-up | 3.22 | 103.5 | render/thread/ms 740, render/busy/ms 583, stage/render/prepare_assets 519, main/busy/ms 152, pipelines/compile_ms 122, stage/main/update 106, stage/render/prepare_views 105, render/acquire/ms 105, pipelines/front_ms 79.93, stage/render/render 67.77, render/present/ms 52.79, pipelines/driver_ms 42.46 |
| warm-up | 3.39 | 157.1 | pipelines/wait_ms 3772, pipelines/compile_ms 1131, pipelines/front_ms 1054, main/busy/ms 157, render/thread/ms 135, render/busy/ms 132, pipelines/driver_ms 76.80, stage/main/update 69.35, encode//sum 57.76, stage/render/render 51.00, stage/main/post_update 50.87, render/graph/ms 37.11 |
| warm-up | 3.47 | 80.1 | pipelines/wait_ms 4257, pipelines/compile_ms 2150, pipelines/front_ms 2042, render/thread/ms 185, render/busy/ms 183, main/busy/ms 143, pipelines/driver_ms 108, stage/main/update 70.60, stage/render/prepare_assets 65.36, encode//sum 60.62, stage/render/render 50.05, render/graph/ms 36.97 |
| warm-up | 3.51 | 35.1 | pipelines/compile_ms 1607, pipelines/front_ms 1540, pipelines/wait_ms 490, render/thread/ms 96.04, render/busy/ms 94.20, stage/render/prepare_assets 74.25, pipelines/driver_ms 66.42, main/busy/ms 50.18, stage/main/post_update 29.37, terrain/update/ms 16.08, stage/render/render 11.76, stage/main/update 11.63 |
| warm-up | 3.54 | 32.4 | pipelines/compile_ms 631, pipelines/front_ms 606, pipelines/wait_ms 74.52, render/thread/ms 62.54, render/busy/ms 60.56, stage/render/prepare_assets 41.25, main/busy/ms 35.62, pipelines/driver_ms 25.57, stage/main/post_update 15.92, stage/main/update 12.45, terrain/update/ms 11.68, stage/render/render 11.26 |
| warm-up | 3.58 | 42.3 | pipelines/compile_ms 830, pipelines/front_ms 796, pipelines/wait_ms 161, render/thread/ms 71.05, render/busy/ms 68.87, main/busy/ms 42.46, pipelines/driver_ms 34.44, stage/render/prepare_assets 22.15, stage/main/update 20.72, encode//sum 20.51, stage/render/render 18.11, render/graph/ms 15.72 |
| warm-up | 3.62 | 35.1 | pipelines/compile_ms 1078, pipelines/front_ms 1036, pipelines/wait_ms 264, main/busy/ms 86.99, render/thread/ms 74.65, render/busy/ms 71.92, stage/main/update 64.40, pipelines/driver_ms 41.99, stage/render/render 27.22, render/graph/ms 24.07, encode//sum 21.19, stage/main/post_update 16.07 |
| warm-up | 3.70 | 80.1 | pipelines/compile_ms 1799, pipelines/front_ms 1730, pipelines/wait_ms 213, main/busy/ms 79.20, render/thread/ms 69.96, pipelines/driver_ms 68.59, render/busy/ms 67.72, stage/main/update 56.30, stage/render/prepare_assets 40.55, stage/render/render 19.95, render/graph/ms 17.16, stage/main/post_update 16.90 |
| warm-up | 3.76 | 39.6 | pipelines/compile_ms 460, pipelines/front_ms 433, render/thread/ms 40.55, render/busy/ms 38.47, main/busy/ms 32.27, pipelines/driver_ms 27.32, stage/main/update 20.97, stage/render/prepare_assets 20.56, stage/render/render 11.86, render/graph/ms 9.59, stage/render/graph_submit 6.82, encode//sum 6.47 |
| warm-up | 3.79 | 33.8 | pipelines/compile_ms 182, pipelines/front_ms 170, render/thread/ms 42.36, render/busy/ms 40.96, stage/render/prepare_assets 26.79, main/busy/ms 22.74, stage/main/update 12.07, pipelines/driver_ms 12.07, stage/render/render 9.42, render/graph/ms 7.83, encode//sum 5.89, stage/render/graph_submit 5.54 |
| warm-up | 3.84 | 35.9 | pipelines/compile_ms 470, pipelines/front_ms 456, render/thread/ms 40.96, render/busy/ms 37.61, main/busy/ms 36.41, stage/main/update 17.17, stage/main/post_update 15.16, stage/render/render 13.86, pipelines/driver_ms 13.30, render/graph/ms 11.94, pipelines/wait_ms 10.88, pipelines/lock_ms 10.72 |
| warm-up | 3.87 | 29.4 | pipelines/compile_ms 470, pipelines/front_ms 456, render/thread/ms 55.74, render/busy/ms 51.95, main/busy/ms 40.13, stage/main/post_update 19.93, stage/render/render 15.29, stage/main/update 14.90, pipelines/driver_ms 13.30, render/graph/ms 13.10, stage/render/prepare_assets 11.62, pipelines/wait_ms 10.88 |
| warm-up | 3.92 | 52.1 | pipelines/compile_ms 402, pipelines/front_ms 391, render/thread/ms 60.74, render/busy/ms 59.00, main/busy/ms 43.43, stage/render/prepare_assets 34.33, stage/main/update 13.41, stage/render/render 12.68, stage/main/post_update 12.26, pipelines/driver_ms 11.21, render/graph/ms 10.30, stage/main/pre_update 9.59 |
| warm-up | 3.97 | 51.9 | pipelines/compile_ms 1266, pipelines/front_ms 1232, main/busy/ms 108, stage/main/update 80.13, render/thread/ms 63.19, render/busy/ms 61.94, stage/render/prepare_assets 46.95, pipelines/driver_ms 33.78, stage/main/pre_update 10.01, stage/render/render 9.05, stage/main/post_update 8.80, render/graph/ms 7.13 |
| warm-up | 4.08 | 108.6 | pipelines/compile_ms 1468, pipelines/front_ms 1430, main/busy/ms 147, stage/main/update 132, render/thread/ms 69.76, render/busy/ms 67.66, stage/render/prepare_assets 43.81, pipelines/driver_ms 37.80, stage/render/render 16.38, render/graph/ms 13.80, stage/render/graph_submit 9.95, stage/main/post_update 9.03 |
| warm-up | 4.17 | 90.6 | pipelines/compile_ms 912, pipelines/front_ms 885, render/thread/ms 87.14, main/busy/ms 85.34, render/busy/ms 84.97, stage/main/update 69.53, stage/render/prepare_assets 54.11, pipelines/wait_ms 42.95, pipelines/driver_ms 26.94, stage/render/render 18.69, render/graph/ms 15.84, stage/render/graph_submit 11.52 |
| warm-up | 4.22 | 44.2 | pipelines/compile_ms 327, pipelines/front_ms 315, render/thread/ms 85.18, render/busy/ms 83.36, main/busy/ms 57.88, pipelines/wait_ms 41.88, stage/render/prepare_assets 35.50, stage/main/update 31.60, stage/render/render 18.71, render/graph/ms 16.20, stage/main/post_update 15.61, stage/render/graph_submit 12.88 |
| warm-up | 4.29 | 70.4 | pipelines/compile_ms 1253, pipelines/front_ms 1210, pipelines/wait_ms 233, render/thread/ms 106, render/busy/ms 104, main/busy/ms 98.71, pipelines/driver_ms 43.62, stage/main/post_update 35.64, stage/main/pre_update 29.24, stage/render/render 28.58, stage/main/update 25.92, render/graph/ms 25.83 |
| warm-up | 4.33 | 38.1 | pipelines/compile_ms 1988, pipelines/front_ms 1907, pipelines/wait_ms 467, render/thread/ms 93.95, main/busy/ms 92.93, render/busy/ms 92.08, pipelines/driver_ms 81.01, stage/main/pre_update 30.65, stage/render/prepare_assets 30.35, stage/main/update 29.19, stage/main/post_update 27.80, stage/render/render 22.79 |
| warm-up | 4.39 | 64.1 | pipelines/compile_ms 1634, pipelines/front_ms 1568, pipelines/wait_ms 245, pipelines/driver_ms 65.95, render/thread/ms 63.12, render/busy/ms 61.70, main/busy/ms 57.56, stage/main/update 36.54, stage/render/prepare_assets 34.71, stream/drive/ms 21.12, stage/main/post_update 11.99, stage/render/render 11.97 |
| warm-up | 4.44 | 52.6 | pipelines/compile_ms 1168, pipelines/front_ms 1127, render/thread/ms 83.81, render/busy/ms 82.06, main/busy/ms 76.48, pipelines/driver_ms 41.52, stage/main/update 37.57, pipelines/wait_ms 34.96, stage/render/prepare_assets 34.74, stage/main/post_update 24.47, stage/render/prepare_resources 17.28, stage/render/render 16.93 |
| warm-up | 4.50 | 55.1 | pipelines/compile_ms 1142, pipelines/front_ms 1105, pipelines/wait_ms 382, main/busy/ms 90.10, render/thread/ms 83.54, render/busy/ms 81.83, stage/main/update 51.96, pipelines/driver_ms 36.75, stage/render/prepare_assets 29.46, stage/main/post_update 20.03, stage/render/render 18.26, stage/render/prepare_resources 17.45 |
| warm-up | 4.56 | 65.7 | pipelines/compile_ms 1280, pipelines/front_ms 1249, pipelines/wait_ms 358, render/thread/ms 78.84, render/busy/ms 77.54, main/busy/ms 58.53, stage/render/prepare_assets 42.68, stage/main/update 37.41, pipelines/driver_ms 31.33, stage/render/render 13.64, render/graph/ms 11.73, stream/drive/ms 9.03 |
| warm-up | 4.59 | 26.0 | pipelines/compile_ms 910, pipelines/front_ms 888, main/busy/ms 89.24, render/thread/ms 67.80, render/busy/ms 66.31, stage/main/update 66.13, stream/drive/ms 38.94, stage/render/prepare_assets 27.34, pipelines/driver_ms 22.16, stage/render/render 16.63, render/graph/ms 14.78, stage/main/post_update 12.95 |
| warm-up | 4.67 | 84.8 | pipelines/compile_ms 839, pipelines/front_ms 815, main/busy/ms 105, stage/main/update 83.14, stream/drive/ms 56.85, render/thread/ms 50.63, render/busy/ms 49.05, pipelines/driver_ms 24.61, stage/render/render 16.14, stage/render/prepare_assets 15.34, render/graph/ms 14.03, stage/main/post_update 13.34 |
| warm-up | 4.71 | 35.9 | pipelines/compile_ms 365, pipelines/front_ms 354, main/busy/ms 70.86, stage/main/update 52.47, render/thread/ms 52.12, render/busy/ms 50.44, stream/drive/ms 24.13, stage/render/render 19.08, render/graph/ms 16.56, stage/render/prepare_assets 14.86, stage/render/graph_submit 13.32, pipelines/driver_ms 11.57 |
| warm-up | 4.76 | 49.3 | pipelines/compile_ms 764, pipelines/front_ms 741, main/busy/ms 54.10, render/thread/ms 52.62, render/busy/ms 51.07, stage/main/update 35.69, pipelines/driver_ms 22.28, stage/render/render 17.56, stage/render/prepare_assets 16.74, render/graph/ms 15.32, stage/render/graph_submit 12.23, stage/main/post_update 8.89 |
| warm-up | 4.81 | 30.5 | pipelines/compile_ms 388, pipelines/front_ms 373, render/thread/ms 49.33, render/busy/ms 48.16, main/busy/ms 39.67, stage/main/update 18.67, pipelines/driver_ms 15.31, stage/main/post_update 13.64, stage/render/render 12.15, stage/render/prepare_assets 11.07, render/graph/ms 10.14, encode//sum 7.83 |
| warm-up | 4.86 | 28.6 | pipelines/compile_ms 763, pipelines/front_ms 743, render/thread/ms 37.23, render/busy/ms 35.86, main/busy/ms 29.77, pipelines/driver_ms 19.80, stage/main/update 14.71, stage/render/prepare_assets 13.78, stage/render/render 11.72, render/graph/ms 9.79, stage/main/post_update 8.67, encode//sum 7.41 |
| warm-up | 4.93 | 27.3 | pipelines/compile_ms 264, pipelines/front_ms 254, main/busy/ms 33.42, render/thread/ms 30.17, render/busy/ms 28.64, stage/render/render 16.14, stage/main/post_update 14.89, render/graph/ms 14.26, stage/main/update 13.62, pipelines/driver_ms 9.86, stage/render/graph_submit 7.77, encode//sum 7.39 |
| warm-up | 5.04 | 31.9 | pipelines/compile_ms 186, pipelines/front_ms 180, main/busy/ms 40.68, render/thread/ms 37.84, render/busy/ms 36.20, stage/main/update 22.03, stage/render/render 13.39, stage/main/post_update 12.32, render/graph/ms 11.37, stream/drive/ms 11.28, stage/render/graph_submit 7.95, encode//sum 7.63 |
| warm-up | 5.07 | 33.3 | pipelines/compile_ms 387, pipelines/front_ms 373, render/thread/ms 50.97, render/busy/ms 49.62, main/busy/ms 38.07, stage/main/update 17.99, pipelines/driver_ms 13.69, stage/main/post_update 13.48, stage/render/prepare_resources 12.86, stage/render/render 12.08, render/graph/ms 10.24, stage/render/prepare_assets 8.63 |
| warm-up | 5.10 | 25.1 | pipelines/compile_ms 584, pipelines/front_ms 565, render/thread/ms 48.80, render/busy/ms 47.51, main/busy/ms 33.08, pipelines/driver_ms 19.29, stage/main/update 13.82, stage/render/prepare_assets 13.66, stage/render/render 12.21, stage/main/post_update 11.83, render/graph/ms 10.33, stage/render/prepare_resources 9.06 |
| warm-up | 5.12 | 25.8 | pipelines/compile_ms 1572, pipelines/front_ms 1527, pipelines/driver_ms 45.22, render/thread/ms 40.97, render/busy/ms 39.63, main/busy/ms 28.03, stage/render/prepare_assets 15.27, stage/render/render 11.98, stage/main/post_update 10.81, render/graph/ms 10.16, stage/main/update 10.09, encode//sum 7.38 |
| warm-up | 5.16 | 25.5 | pipelines/compile_ms 93.22, pipelines/front_ms 88.51, render/thread/ms 39.86, render/busy/ms 38.60, main/busy/ms 34.43, stage/main/update 14.20, stage/render/render 13.27, render/graph/ms 11.55, stage/main/post_update 10.61, stage/render/prepare_write_phase_buffers 9.91, stage/render/graph_submit 8.21, encode//sum 7.21 |
| warm-up | 5.27 | 26.6 | main/busy/ms 31.03, render/thread/ms 28.49, render/busy/ms 27.14, pipelines/compile_ms 25.93, pipelines/front_ms 24.24, stage/render/render 14.60, stage/main/post_update 12.96, render/graph/ms 12.79, stage/main/update 11.24, stage/render/graph_submit 9.37, encode//sum 7.42, encode/submit 6.86 |
| warm-up | 5.31 | 28.6 | pipelines/compile_ms 1305, pipelines/front_ms 1274, main/busy/ms 37.74, pipelines/driver_ms 31.50, render/thread/ms 26.83, render/busy/ms 25.63, stage/main/update 23.19, stream/drive/ms 15.71, stage/render/render 10.15, stage/main/post_update 8.61, render/graph/ms 8.51, stage/render/prepare_assets 7.55 |
| warm-up | 5.45 | 28.8 | render/thread/ms 37.14, render/busy/ms 35.92, main/busy/ms 26.09, stage/render/prepare_assets 18.89, stage/main/post_update 11.59, stage/render/render 10.56, stage/main/update 9.77, render/graph/ms 8.91, encode//sum 7.32, stage/render/graph_submit 6.18, encode/submit 4.16, stage/render/camera_driver 2.53 |
| warm-up | 5.60 | 25.1 | main/busy/ms 31.73, render/thread/ms 20.38, stage/main/update 19.90, render/busy/ms 19.24, stream/drive/ms 12.44, stage/render/render 8.77, render/graph/ms 7.48, stage/main/post_update 6.95, encode//sum 6.41, stage/render/prepare_assets 5.08, stage/render/graph_submit 5.00, encode/submit 2.66 |
| warm-up | 5.67 | 29.3 | render/thread/ms 38.38, render/busy/ms 36.75, main/busy/ms 29.38, stage/render/render 14.84, stage/main/update 14.39, render/graph/ms 12.58, encode//sum 11.87, stage/render/prepare_assets 11.43, stage/main/post_update 10.49, stage/render/graph_submit 8.60, encode/submit 4.77, encode/finish_wall 3.90 |
| warm-up | 5.72 | 35.5 | render/thread/ms 41.89, render/busy/ms 41.00, main/busy/ms 29.05, stage/render/prepare_assets 21.59, stage/main/update 12.56, stage/render/render 9.15, stage/main/post_update 8.84, render/graph/ms 7.79, encode//sum 6.12, stage/render/graph_submit 4.91, stage/main/fixed_main_loop 3.72, stream/drive/ms 3.59 |
| warm-up | 5.83 | 25.7 | render/thread/ms 37.99, main/busy/ms 36.74, render/busy/ms 36.64, stage/main/update 22.13, stage/render/render 13.34, render/graph/ms 11.64, stage/render/prepare_assets 11.56, encode//sum 10.11, stage/main/post_update 9.59, stage/render/graph_submit 8.46, encode/submit 5.26, stream/drive/ms 4.56 |
| warm-up | 5.89 | 29.0 | pipelines/compile_ms 132, pipelines/front_ms 127, render/thread/ms 41.29, render/busy/ms 39.93, main/busy/ms 36.97, stage/main/update 23.02, stage/render/render 14.51, render/graph/ms 12.65, stage/render/prepare_assets 8.86, stage/main/post_update 8.86, encode//sum 8.73, stage/render/graph_submit 8.10 |
| warm-up | 5.94 | 29.2 | render/thread/ms 42.64, render/busy/ms 41.27, main/busy/ms 40.81, stage/main/update 23.07, stage/render/render 15.64, render/graph/ms 13.46, stage/main/post_update 13.11, encode//sum 8.99, stage/render/graph_submit 8.75, stage/render/prepare_assets 7.76, encode/submit 5.34, stage/render/prepare_meshes 5.08 |
| warm-up | 6.00 | 35.6 | render/thread/ms 47.11, render/busy/ms 46.15, main/busy/ms 36.13, stage/main/update 19.74, stage/render/prepare_assets 18.64, stage/render/render 12.33, render/graph/ms 10.89, stage/main/post_update 9.17, stage/render/graph_submit 8.29, encode//sum 6.85, stage/render/prepare_meshes 6.39, encode/finish_wall 5.26 |
| warm-up | 6.03 | 25.8 | render/thread/ms 50.90, render/busy/ms 49.90, main/busy/ms 39.82, stage/main/update 20.97, stage/render/prepare_assets 16.27, stage/render/render 16.00, render/graph/ms 14.48, stage/render/graph_submit 11.43, stage/main/post_update 10.23, encode/finish_wall 7.89, encode/pass:point_shadow_atlas_live_indirect/sum 7.71, encode//sum 6.58 |
| warm-up | 6.06 | 35.6 | pipelines/compile_ms 140, pipelines/front_ms 137, render/thread/ms 55.49, render/busy/ms 54.27, main/busy/ms 39.67, stage/render/render 21.24, render/graph/ms 19.35, stage/main/update 16.81, stage/main/post_update 15.69, stage/render/graph_submit 14.35, encode/finish_wall 10.23, encode//sum 10.12 |
| warm-up | 6.12 | 35.7 | render/thread/ms 45.11, render/busy/ms 44.09, main/busy/ms 35.77, stage/main/update 20.92, pipelines/compile_ms 20.48, pipelines/front_ms 19.42, stage/render/prepare_assets 16.69, stage/render/render 15.11, render/graph/ms 13.51, stage/render/graph_submit 10.62, stage/main/post_update 9.07, encode/finish_wall 7.80 |
| warm-up | 6.24 | 33.1 | pipelines/compile_ms 122, pipelines/front_ms 118, main/busy/ms 40.17, render/thread/ms 33.27, render/busy/ms 31.97, stage/main/update 25.81, stage/render/render 14.49, stream/drive/ms 14.22, render/graph/ms 12.66, stage/render/graph_submit 9.73, stage/main/post_update 9.43, encode//sum 7.69 |
| warm-up | 6.31 | 47.3 | main/busy/ms 49.29, render/thread/ms 41.43, render/busy/ms 40.21, stage/main/update 30.87, stage/render/render 16.65, render/graph/ms 14.81, stage/main/post_update 13.18, stage/render/prepare_assets 12.63, stage/render/graph_submit 11.45, encode//sum 7.92, encode/finish_wall 6.24, stream/drive/ms 6.20 |
| warm-up | 6.34 | 27.6 | render/thread/ms 40.70, render/busy/ms 39.75, main/busy/ms 36.45, stage/render/prepare_assets 16.88, stage/main/update 16.66, stage/render/render 11.67, stage/main/post_update 11.13, render/graph/ms 10.27, stage/render/graph_submit 7.57, encode//sum 7.03, encode/finish_wall 4.96, stream/drive/ms 4.91 |
| measure | 18.37 | 31.0 | main/busy/ms 38.72, stage/main/update 25.11, render/thread/ms 20.69, render/busy/ms 19.69, stage/render/render 10.32, render/graph/ms 8.99, stage/main/post_update 8.63, stage/render/graph_submit 6.44, encode//sum 5.85, encode/submit 4.04, stage/render/prepare_assets 3.93, encode/finish_wall 2.45 |
| measure | 59.11 | 25.3 | render/thread/ms 37.36, render/busy/ms 35.97, main/busy/ms 34.36, stage/main/post_update 19.58, stage/render/render 19.15, render/graph/ms 17.24, encode//sum 15.92, stage/render/graph_submit 11.16, stage/main/update 8.41, stage/render/prepare_assets 6.46, encode/submit 6.27, stage/render/camera_driver 5.89 |
| measure | 59.14 | 26.9 | render/thread/ms 37.32, render/busy/ms 36.34, main/busy/ms 27.86, stage/main/post_update 15.92, stage/render/prepare_assets 14.36, stage/render/render 12.82, render/graph/ms 11.49, encode//sum 11.45, stage/render/graph_submit 7.26, stage/main/update 5.24, stage/render/camera_driver 4.07, encode/submit 3.79 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.soft_pcf_radius=0 --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=pcf0-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **83%** (min 54%) · 2618 MHz (min 2610) · memory 8101 MHz · 83.1 W · 79 °C (max 84) · VRAM 5547 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.soft_pcf_radius=0 (command line: --set)`
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
