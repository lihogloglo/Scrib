# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1416 frames over 15.0 s · **median 10.49 ms (95 fps)** · mean 10.60 · p95 12.44 · p99 13.41 (1% low 75 fps) · worst 17.89 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 308 frames over 9.1 s, worst 2524.2 ms
- hitches (> 26.2 ms): **0** measured, 48 in warm-up
- timed GPU passes: 8.14 ms of a 10.60 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.49 ms, limit 11 ms
- pass worst frame 17.89 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 1.919 | 2.062 | 2.098 | 0.93 | 0.065 |
| dlss_super_resolution | 1.914 | 2.057 | 2.081 | 0.93 | 0.181 |
| early prepass | 0.700 | 0.752 | 0.766 | 0.93 | 0.095 |
| sun_visibility | 0.653 | 0.701 | 0.714 | 0.93 | 0.007 |
| atmosphere_luts | 0.333 | 0.358 | 0.362 | 0.93 | 0.003 |
| water_velocity | 0.307 | 0.330 | 0.333 | 0.93 | 0.002 |
| bloom | 0.296 | 0.318 | 0.324 | 0.93 | 0.067 |
| atmos_froxel_inject | 0.284 | 0.306 | 0.309 | 0.93 | 0.001 |
| water_sspr | 0.195 | 0.210 | 0.211 | 0.93 | 0.002 |
| ssao | 0.175 | 0.188 | 0.190 | 0.93 | 0.008 |
| uniform_allocation | 0.130 | 0.139 | 0.184 | 0.93 | 0.023 |
| water_fft | 0.125 | 0.134 | 0.136 | 0.93 | 0.003 |
| upscaling | 0.112 | 0.121 | 0.125 | 0.93 | 0.004 |
| clouds_composite | 0.101 | 0.109 | 0.111 | 0.93 | 0.002 |
| bin_unpacking | 0.094 | 0.101 | 0.132 | 0.93 | 0.016 |
| tonemapping | 0.079 | 0.085 | 0.089 | 0.93 | 0.004 |
| gpu_instance_materialization | 0.079 | 0.085 | 0.092 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.073 | 0.079 | 0.090 | 0.93 | 0.008 |
| clustering | 0.064 | 0.069 | 0.078 | 0.93 | 0.019 |
| render_sky | 0.059 | 0.063 | 0.065 | 0.93 | 0.002 |
| early_mesh_preprocessing | 0.059 | 0.063 | 0.095 | 0.93 | 0.004 |
| clouds_trace | 0.054 | 0.058 | 0.068 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.411 | 0.415 | 0.12 | 0.001 |
| clouds_resolve | 0.045 | 0.048 | 0.050 | 0.93 | 0.001 |
| point_shadow_atlas | 0.035 | 0.072 | 0.078 | 0.48 | 0.138 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.93 | 0.001 |
| lightprobe_radiance_map | 0.025 | 0.219 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.022 | 0.024 | 0.025 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.93 | 0.003 |
| clouds_shadow | 0.020 | 0.021 | 0.024 | 0.93 | 0.003 |
| per_view_shadow_pass | 0.020 | 0.021 | 0.038 | 0.93 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.026 | 0.93 | 0.240 |
| clouds_fog_light | 0.010 | 0.011 | 0.012 | 0.93 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.015 | 0.016 | 0.48 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 0.93 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.48 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.48 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.93 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.262 | 11.014 | 18.82 | 13114.7 |
| render/busy/ms | 8.387 | 9.994 | 17.87 | 11875.7 |
| main/busy/ms | 8.158 | 9.185 | 12.37 | 11551.1 |
| stage/render/render | 5.324 | 7.024 | 10.93 | 7538.8 |
| render/graph/ms | 4.669 | 5.841 | 9.53 | 6611.3 |
| stage/main/post_update | 4.626 | 5.502 | 7.87 | 6550.6 |
| encode//sum | 3.989 | 5.479 | 11.38 | 5647.9 |
| stage/render/graph_submit | 3.339 | 4.454 | 7.18 | 4727.8 |
| encode/submit | 2.083 | 3.138 | 5.55 | 2949.9 |
| stage/main/update | 1.827 | 2.144 | 2.60 | 2587.2 |
| encode/finish_wall | 1.323 | 1.883 | 3.29 | 1873.0 |
| stage/render/camera_driver | 1.266 | 1.475 | 2.56 | 1792.8 |
| encode/pass:shadow_directional_light/sum | 0.884 | 1.537 | 2.61 | 1251.7 |
| encode/pass:early prepass/sum | 0.844 | 1.134 | 1.69 | 1195.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.818 | 1.133 | 2.20 | 1157.8 |
| stage/render/prepare_assets | 0.577 | 0.712 | 1.60 | 816.7 |
| bigspace/total/ms | 0.551 | 1.233 | 2.25 | 780.5 |
| stage/render/prepare_views | 0.524 | 1.957 | 3.26 | 742.1 |
| stage/render/prepare_meshes | 0.516 | 0.754 | 1.18 | 730.1 |
| stage/render/prepare_write_phase_buffers | 0.508 | 0.756 | 1.51 | 718.8 |
| render/acquire/ms | 0.477 | 1.908 | 3.22 | 676.0 |
| stage/main/fixed_main_loop | 0.466 | 0.777 | 1.50 | 659.9 |
| stage/render/prepare_resources | 0.426 | 0.495 | 0.94 | 603.8 |
| stage/main/pre_update | 0.411 | 0.557 | 0.96 | 581.8 |
| render/present/ms | 0.398 | 1.161 | 1.71 | 563.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.393 | 0.600 | 2.07 | 557.0 |
| stage/main/first | 0.364 | 0.455 | 1.88 | 515.9 |
| controller/feed/ms | 0.329 | 0.428 | 0.79 | 466.2 |
| stage/render/prepare_batch_phases | 0.321 | 0.442 | 0.67 | 454.9 |
| stage/render/prepare_flush | 0.319 | 0.536 | 2.41 | 452.1 |
| stage/render/prepare_bind_groups | 0.277 | 0.420 | 0.68 | 392.0 |
| stage/main/last | 0.275 | 0.334 | 0.72 | 389.2 |
| bigspace/low_precision/ms | 0.249 | 0.688 | 1.72 | 352.0 |
| stage/render/create_views | 0.192 | 0.238 | 0.66 | 271.4 |
| stream/drive/ms | 0.191 | 0.264 | 0.59 | 270.5 |
| stage/render/extract_commands | 0.177 | 0.210 | 0.41 | 250.8 |
| pipelines/compile_ms | 0.174 | 0.000 | 125.21 | 245.8 |
| pipelines/front_ms | 0.170 | 0.000 | 122.56 | 240.8 |
| anim/evaluate_cpu/ms | 0.169 | 0.198 | 0.42 | 239.8 |
| stage/render/specialize | 0.159 | 0.226 | 0.59 | 225.8 |
| stream/tick/ms | 0.159 | 0.227 | 0.56 | 225.4 |
| anim/evaluate/ms | 0.153 | 0.288 | 1.36 | 216.5 |
| bigspace/high_precision/ms | 0.145 | 0.569 | 1.43 | 204.9 |
| stage/render/queue | 0.134 | 0.172 | 0.32 | 189.4 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.110 | 0.305 | 1.59 | 156.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.105 | 0.173 | 0.39 | 149.0 |
| controller/write/ms | 0.065 | 0.082 | 0.24 | 91.4 |
| controller/move/ms | 0.049 | 0.096 | 0.16 | 70.0 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.074 | 0.15 | 65.8 |
| encode/resolve_diagnostics/sum | 0.043 | 0.073 | 0.45 | 60.6 |
| physics/step/ms | 0.036 | 0.066 | 0.51 | 51.2 |
| terrain/update/ms | 0.035 | 0.049 | 0.10 | 50.0 |
| stage/render/phase_sort | 0.034 | 0.051 | 0.26 | 48.3 |
| stage/render/cleanup | 0.025 | 0.033 | 0.43 | 35.2 |
| encode/camera_driver_span_begin/sum | 0.018 | 0.034 | 0.13 | 24.9 |
| weather/map/ms | 0.017 | 0.022 | 0.40 | 23.7 |
| encode/camera_driver_span_end/sum | 0.016 | 0.034 | 0.35 | 22.7 |
| anim/apply/ms | 0.011 | 0.013 | 0.14 | 15.2 |
| weather/eval/ms | 0.010 | 0.012 | 0.43 | 13.7 |
| controller/avoid/ms | 0.010 | 0.018 | 0.38 | 13.6 |
| script/update/ms | 0.008 | 0.013 | 0.03 | 11.3 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.1 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 7.1 |
| far/select/ms | 0.004 | 0.005 | 0.04 | 5.8 |
| pipelines/driver_ms | 0.004 | 0.000 | 2.64 | 5.1 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.01 | 2.9 |
| stream/assets_ready/ms | 0.002 | 0.002 | 0.01 | 2.1 |
| stream/promote/ms | 0.001 | 0.002 | 0.20 | 1.9 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.0 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.03 | 1.0 |
| terrain/select/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.000 | 0.000 | 0.00 | 0.0 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/chains/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/plan/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| scene/load/wanted/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 3 | 3 | 3 |
| anim/a3/count | count | 447 | 447 | 447 |
| anim/characters/count | count | 450 | 450 | 450 |
| anim/evaluated/count | count | 26.50 | 27 | 26 |
| controller/movers/count | count | 28 | 28 | 28 |
| crowd/c0/count | count | 30 | 30 | 30 |
| crowd/c0/joints/count | count | 1249 | 1249 | 1249 |
| crowd/c0/meshes/count | count | 398 | 398 | 398 |
| crowd/c1/count | count | 116 | 116 | 116 |
| crowd/c1/joints/count | count | 5172 | 5172 | 5172 |
| crowd/c1/meshes/count | count | 1293 | 1293 | 1293 |
| crowd/c2/count | count | 304 | 304 | 304 |
| crowd/c2/joints/count | count | 5254 | 5254 | 5254 |
| crowd/c2/meshes/count | count | 1802 | 1802 | 1802 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 210 | 210 | 210 |
| crowd/joints_extracted/count | count | 736 | 1211 | 497 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.7 k | 43.7 k | 43.7 k |
| encode//max | ms | 0.50 | 2.99 | 0.48 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.13 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.35 | 0.02 |
| encode/finish_critical | ms | 0.99 | 2.99 | 1.02 |
| encode/pass:early prepass/max | ms | 0.85 | 1.69 | 0.98 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 2.20 | 0.74 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.39 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.37 | 2.07 | 0.26 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.21 | 1.59 | 0.17 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 2.61 | 1.02 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.45 | 0.03 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 9.38 | 17.87 | 9.32 |
| gpu/timed/ms | ms | 9.36 | 10.13 | 9.32 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2431 | 2431 | 2431 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 301.1 k | 301.1 k | 301.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.6 k | 12.6 k | 12.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 343.66 M | 343.66 M | 343.66 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 354 | 354 | 354 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 559 | 559 | 559 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 559 | 559 | 559 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 354 | 354 | 354 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 559 | 559 | 559 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 559 | 559 | 559 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 1347 | 1347 | 1347 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 6753 | 6753 | 6753 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2829 | 2829 | 2829 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 20 | 20 | 20 |
| phases/opaque3d_prepass/bins | count | 1347 | 1347 | 1347 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 6753 | 6753 | 6753 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2829 | 2829 | 2829 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 9 | 106 | 9 |
| phases/shadow/bins | count | 2177 | 2180 | 2177 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 7312 | 21.9 k | 7312 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 5349 | 5359 | 5350 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 47 | 48 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 342 | 342 | 342 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 5 | 67 | 5 |
| preprocess/bind_groups_created | count | 0 | 60 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 29.33 | 44.71 | 24.55 |
| process/mem_usage | GiB | 3.13 | 3.18 | 3.17 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 1720 | 1720 | 1720 |
| scene/batch/masked/live | count | 1720 | 1720 | 1720 |
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
| scene/batch/opaque/high_water | count | 19.7 k | 19.7 k | 19.7 k |
| scene/batch/opaque/live | count | 19.7 k | 19.7 k | 19.7 k |
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
| scene/cells/resident | count | 118 | 118 | 118 |
| scene/chains/high_water | count | 1663 | 1663 | 1663 |
| scene/chains/live | count | 1663 | 1663 | 1663 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/scene_bump | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/scene_live | count | 21.5 k | 21.5 k | 21.5 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 4 | 4 | 4 |
| shadow_atlas/live_occ_pct | % | 0.59 | 0.59 | 0.59 |
| shadow_atlas/live_tiles | count | 6 | 6 | 6 |
| shadow_atlas/mover_draws | count | 1 | 4 | 4 |
| shadow_atlas/movers | count | 100 | 100 | 100 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 4 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 0 | 4 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 6 | 6 | 6 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 5.86 | 5.86 | 5.86 |
| shadow_atlas/static_tiles | count | 24 | 24 | 24 |
| shadow_atlas/views | count | 6 | 6 | 6 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 18 | 27 | 14 |
| skin/blocks_skipped/count | count | 192 | 201 | 196 |
| skin/blocks_walked/count | count | 18 | 28 | 14 |
| skin/joint_bytes_written/bytes | bytes | 59.4 k | 80.6 k | 52.0 k |
| skin/joints_written/count | count | 928 | 1260 | 812 |
| skin/upload_writes/count | count | 9 | 13 | 10 |
| started_load_count |  | 5819 | 5820 | 5820 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 247 | 247 | 247 |
| statics/materials_bound | count | 247 | 247 | 247 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 221 | 221 | 221 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 17.40 M | 17.40 M | 17.40 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 899 | 899 | 899 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.98 | 43.92 | 28.50 |
| system/mem_usage | % | 58.94 | 59.05 | 59.03 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2950 | 2950 | 2950 |
| textures/images | count | 2500 | 2500 | 2500 |
| views/all/batch_sets | count | 70 | 167 | 70 |
| views/all/bins | count | 5579 | 5582 | 5579 |
| views/all/count | count | 10 | 10 | 10 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 21.9 k | 36.6 k | 21.9 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 12.1 k | 12.1 k | 12.1 k |
| views/all/pipelines | count | 97 | 98 | 97 |
| views/all/pipelines_distinct | count | 69 | 69 | 69 |
| views/all/work_items | count | 154.8 k | 258.2 k | 154.8 k |
| views/cascade/batch_sets | count | 9 | 45 | 9 |
| views/cascade/bins | count | 2028 | 2028 | 2028 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 7312 | 21.9 k | 7312 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 5200 | 5207 | 5201 |
| views/cascade/pipelines | count | 37 | 37 | 37 |
| views/cascade/work_items | count | 49.3 k | 152.7 k | 49.3 k |
| views/lamp_face/batch_sets | count | 0 | 91 | 0 |
| views/lamp_face/bins | count | 149 | 152 | 149 |
| views/lamp_face/count | count | 6 | 6 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 149 | 152 | 149 |
| views/lamp_face/pipelines | count | 10 | 11 | 10 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 91 | 0 |
| views/main/batch_sets | count | 61 | 61 | 61 |
| views/main/bins | count | 3402 | 3402 | 3402 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 14.6 k | 14.6 k | 14.6 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6776 | 6776 | 6776 |
| views/main/pipelines | count | 50 | 50 | 50 |
| views/main/work_items | count | 105.5 k | 105.5 k | 105.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 18.7 k | 18.7 k | 18.7 k |
| world/point_lights | count | 355 | 355 | 355 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 40.05 | 125 | 56.7 k |
| anim/events/count | count | 0.03 | 2 | 48 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.54 | 8 | 3600 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 58 | 40.0 k |
| encode/bind_walk/init_textures | count | 9082 | 20.7 k | 12.86 M |
| encode/bind_walk/init_textures_skipped | count | 52.4 k | 110.9 k | 74.25 M |
| encode/bind_walk/inits | count | 653 | 1365 | 925.0 k |
| encode/bind_walk/inits_skipped | count | 169 | 343 | 239.3 k |
| encode/bind_walk/merge_textures | count | 9020 | 20.5 k | 12.77 M |
| encode/bind_walk/merge_textures_skipped | count | 8730 | 23.5 k | 12.36 M |
| encode/bind_walk/merges | count | 115 | 240 | 163.2 k |
| encode/bind_walk/merges_skipped | count | 155 | 321 | 219.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1416 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1416 |
| encode/pass:early prepass/count | count | 1 | 2 | 1416 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1416 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1416 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1416 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 708 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1416 |
| encode/queue/command_buffers | count | 38.75 | 79 | 54.9 k |
| encode/queue/submits | count | 2 | 4 | 2832 |
| encode/queue/write_buffer/bytes | bytes | 2.73 M | 5.88 M | 3.86 G |
| encode/queue/write_buffer/calls | count | 263 | 572 | 372.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 55.6 k | 111.1 k | 78.66 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 12.7 k |
| encode/queue/write_texture/bytes | bytes | 18.6 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1416 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 1 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
| scene/load/deferred | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.2 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2524.2 | main/busy/ms 2493, stage/main/post_update 2451, stage/main/update 29.88, stage/main/pre_update 8.58, stage/main/last 1.33, surface/advance_cpu_ms 0.75, stage/main/first 0.49, stream/drive/ms 0.44, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.29, stream/tick/ms 0.12, bigspace/total/ms 0.08 |
| warm-up | 3.26 | 734.9 | main/busy/ms 2539, stage/main/post_update 2484, render/thread/ms 673, render/busy/ms 525, stage/render/prepare_assets 474, stage/render/prepare_views 146, render/acquire/ms 145, stage/main/update 36.48, stage/render/create_views 30.81, stage/render/render 12.54, stage/main/pre_update 10.70, render/graph/ms 8.68 |
| warm-up | 3.33 | 75.2 | render/thread/ms 745, render/busy/ms 542, stage/render/prepare_assets 478, stage/render/prepare_views 146, render/acquire/ms 146, pipelines/compile_ms 104, main/busy/ms 76.79, stage/render/render 73.91, pipelines/front_ms 70.77, render/present/ms 56.63, stage/main/post_update 54.85, pipelines/driver_ms 33.02 |
| warm-up | 3.37 | 38.4 | pipelines/compile_ms 229, pipelines/front_ms 152, render/thread/ms 108, pipelines/driver_ms 77.12, stage/render/render 66.51, main/busy/ms 55.16, render/present/ms 55.00, render/busy/ms 51.92, stage/main/post_update 39.31, terrain/update/ms 30.13, stage/render/prepare_assets 26.40, render/graph/ms 9.87 |
| warm-up | 3.51 | 137.4 | pipelines/wait_ms 3705, pipelines/compile_ms 1122, pipelines/front_ms 1004, render/thread/ms 172, render/busy/ms 163, pipelines/driver_ms 118, main/busy/ms 87.56, stage/render/prepare_assets 49.24, encode//sum 46.41, stage/render/prepare_resources 41.44, stage/main/post_update 36.14, stage/main/pre_update 31.95 |
| warm-up | 3.59 | 78.4 | pipelines/wait_ms 5956, pipelines/compile_ms 2437, pipelines/front_ms 2300, render/thread/ms 212, render/busy/ms 203, main/busy/ms 167, pipelines/driver_ms 137, stage/main/update 50.18, encode//sum 49.59, stage/render/prepare_assets 43.87, stage/render/render 40.87, stage/render/prepare_resources 40.83 |
| warm-up | 3.68 | 93.7 | pipelines/wait_ms 2401, pipelines/compile_ms 2282, pipelines/front_ms 2189, render/thread/ms 137, render/busy/ms 134, main/busy/ms 124, pipelines/driver_ms 92.23, stage/render/prepare_assets 66.66, stage/main/update 53.40, stage/main/fixed_main_loop 28.68, stage/render/render 22.92, terrain/update/ms 22.80 |
| warm-up | 3.74 | 59.6 | pipelines/compile_ms 1006, pipelines/front_ms 967, pipelines/wait_ms 195, render/thread/ms 117, render/busy/ms 114, stage/render/prepare_assets 93.82, main/busy/ms 84.64, stage/main/pre_update 45.84, pipelines/driver_ms 38.69, stage/main/post_update 19.28, stage/main/update 14.46, stage/render/render 13.49 |
| warm-up | 3.77 | 29.9 | pipelines/compile_ms 500, pipelines/front_ms 460, pipelines/wait_ms 148, main/busy/ms 81.33, render/thread/ms 76.33, render/busy/ms 74.79, stage/render/prepare_assets 53.66, stage/main/pre_update 45.59, pipelines/driver_ms 40.58, stage/main/post_update 17.61, stage/main/update 12.87, stage/render/render 11.63 |
| warm-up | 3.80 | 27.4 | pipelines/compile_ms 1146, pipelines/front_ms 1082, pipelines/wait_ms 169, pipelines/driver_ms 63.51, render/thread/ms 46.52, main/busy/ms 45.56, render/busy/ms 44.74, stage/main/post_update 24.88, encode//sum 16.94, stage/render/render 15.89, render/graph/ms 10.99, stage/render/prepare_assets 10.47 |
| warm-up | 3.83 | 26.9 | pipelines/compile_ms 944, pipelines/front_ms 909, pipelines/wait_ms 67.51, main/busy/ms 58.21, render/thread/ms 45.37, render/busy/ms 43.49, pipelines/driver_ms 35.44, stage/main/post_update 29.07, encode//sum 19.04, stage/render/render 18.07, stage/main/update 14.99, render/graph/ms 12.96 |
| warm-up | 3.88 | 54.1 | pipelines/compile_ms 384, pipelines/front_ms 356, render/thread/ms 63.54, render/busy/ms 61.19, main/busy/ms 44.48, stage/render/prepare_assets 35.13, pipelines/driver_ms 27.98, stage/main/post_update 16.56, stage/render/render 16.17, stage/main/update 14.95, encode//sum 14.11, render/graph/ms 13.39 |
| warm-up | 3.91 | 34.7 | pipelines/compile_ms 1315, pipelines/front_ms 1238, pipelines/wait_ms 78.26, pipelines/driver_ms 76.88, render/thread/ms 76.55, render/busy/ms 74.20, stage/render/prepare_assets 48.25, main/busy/ms 30.78, stage/render/render 17.84, render/graph/ms 15.03, encode//sum 12.08, stage/render/graph_submit 11.54 |
| warm-up | 4.00 | 85.2 | pipelines/compile_ms 1481, pipelines/front_ms 1335, pipelines/driver_ms 146, render/thread/ms 116, render/busy/ms 114, stage/render/prepare_assets 94.77, pipelines/wait_ms 80.55, main/busy/ms 25.93, stage/render/render 13.78, render/graph/ms 11.71, stage/main/update 9.83, stage/render/graph_submit 8.54 |
| warm-up | 4.03 | 34.1 | pipelines/compile_ms 713, pipelines/front_ms 605, render/thread/ms 116, render/busy/ms 115, pipelines/driver_ms 108, stage/render/prepare_assets 95.57, main/busy/ms 19.23, stage/render/render 15.95, render/graph/ms 14.20, stage/render/graph_submit 11.75, stage/main/update 7.73, encode/submit 7.64 |
| warm-up | 4.10 | 66.0 | pipelines/compile_ms 1125, pipelines/front_ms 1087, render/thread/ms 97.64, render/busy/ms 96.20, stage/render/prepare_assets 46.41, stage/render/render 45.94, render/graph/ms 44.24, stage/render/graph_submit 41.58, pipelines/driver_ms 37.80, encode//sum 36.42, encode/finish_wall 34.47, encode/pass:main_transparent_pass_3d/sum 31.08 |
| warm-up | 4.26 | 51.9 | pipelines/compile_ms 280, pipelines/front_ms 271, render/thread/ms 68.82, render/busy/ms 66.69, main/busy/ms 53.51, stage/main/update 29.44, stage/render/prepare_assets 26.71, stage/main/post_update 17.04, stage/render/render 11.24, stage/render/prepare_write_phase_buffers 10.35, pipelines/driver_ms 9.92, render/graph/ms 9.43 |
| warm-up | 4.31 | 55.6 | pipelines/compile_ms 459, pipelines/front_ms 444, render/thread/ms 105, render/busy/ms 103, main/busy/ms 71.08, stage/render/prepare_assets 38.54, stage/main/update 28.36, stage/main/post_update 28.19, stage/render/prepare_write_phase_buffers 21.94, stage/render/prepare_resources 21.39, pipelines/driver_ms 15.36, stage/render/render 9.65 |
| warm-up | 4.39 | 81.4 | pipelines/compile_ms 1153, pipelines/front_ms 1120, render/thread/ms 134, render/busy/ms 132, main/busy/ms 114, stage/main/post_update 48.46, stage/render/prepare_assets 46.39, stage/main/update 40.02, pipelines/driver_ms 33.08, stage/render/prepare_flush 27.28, stage/render/prepare_resources 21.91, stage/render/prepare_write_phase_buffers 15.87 |
| warm-up | 4.47 | 72.7 | pipelines/compile_ms 2258, pipelines/front_ms 2197, main/busy/ms 152, render/thread/ms 144, render/busy/ms 141, stage/main/update 77.09, pipelines/driver_ms 61.53, stage/main/post_update 53.76, stage/render/prepare_assets 50.26, stage/render/prepare_flush 34.17, stage/render/render 18.75, stage/render/prepare_resources 18.53 |
| warm-up | 4.61 | 139.6 | pipelines/compile_ms 3062, pipelines/front_ms 2985, pipelines/wait_ms 497, render/thread/ms 196, render/busy/ms 193, main/busy/ms 150, stage/render/prepare_assets 82.19, stage/main/update 81.16, pipelines/driver_ms 77.82, stage/main/post_update 56.26, stage/render/prepare_resources 32.15, stage/render/render 29.22 |
| warm-up | 4.68 | 76.7 | pipelines/compile_ms 3460, pipelines/front_ms 3370, pipelines/wait_ms 727, render/thread/ms 206, render/busy/ms 204, stage/render/prepare_assets 117, main/busy/ms 111, pipelines/driver_ms 90.00, stage/main/update 50.02, stage/main/post_update 49.48, stage/render/render 23.21, stage/render/prepare_resources 23.15 |
| warm-up | 4.80 | 113.3 | pipelines/compile_ms 2550, pipelines/front_ms 2456, pipelines/wait_ms 401, render/thread/ms 186, render/busy/ms 184, stage/render/prepare_assets 122, pipelines/driver_ms 94.45, main/busy/ms 60.64, stage/main/update 33.04, stage/render/prepare_flush 28.87, stage/main/post_update 18.09, stage/render/render 18.09 |
| warm-up | 4.83 | 33.5 | pipelines/compile_ms 911, pipelines/front_ms 858, pipelines/wait_ms 159, render/thread/ms 144, render/busy/ms 142, stage/render/prepare_assets 77.34, main/busy/ms 54.41, pipelines/driver_ms 52.87, stage/main/update 34.09, stage/render/prepare_flush 28.79, stage/render/render 23.25, render/graph/ms 21.55 |
| warm-up | 4.97 | 35.7 | pipelines/compile_ms 472, pipelines/front_ms 460, main/busy/ms 52.28, render/thread/ms 35.57, render/busy/ms 34.08, stage/main/update 30.46, stage/render/render 16.55, render/graph/ms 14.03, stage/main/post_update 13.67, pipelines/driver_ms 12.67, stage/render/graph_submit 9.64, character/finish/ms 9.40 |
| warm-up | 5.04 | 29.7 | pipelines/compile_ms 646, pipelines/front_ms 629, render/thread/ms 42.40, render/busy/ms 39.31, main/busy/ms 37.29, stage/main/update 18.84, pipelines/driver_ms 17.03, stage/render/render 14.06, stage/main/post_update 12.89, render/graph/ms 12.30, stage/render/graph_submit 8.92, stage/render/prepare_bind_groups 8.06 |
| warm-up | 5.14 | 40.1 | pipelines/compile_ms 209, pipelines/front_ms 203, render/thread/ms 54.63, render/busy/ms 52.82, main/busy/ms 42.62, stage/main/update 22.08, stage/render/render 14.48, render/graph/ms 12.11, stage/main/post_update 12.00, stage/render/phase_sort 11.65, stage/render/graph_submit 8.84, encode//sum 8.59 |
| warm-up | 5.17 | 33.9 | pipelines/compile_ms 229, pipelines/front_ms 223, render/thread/ms 66.93, render/busy/ms 65.22, main/busy/ms 37.32, stage/render/prepare_assets 21.70, stage/main/update 18.59, stage/render/render 11.95, stage/render/phase_sort 11.63, stage/main/post_update 11.11, render/graph/ms 9.66, encode//sum 8.87 |
| warm-up | 5.23 | 33.1 | pipelines/compile_ms 623, pipelines/front_ms 596, render/thread/ms 51.90, render/busy/ms 50.54, main/busy/ms 39.82, pipelines/driver_ms 26.75, stage/main/post_update 17.10, stage/main/update 14.94, stage/render/render 14.54, stage/render/prepare_resources 14.22, render/graph/ms 12.70, encode//sum 8.59 |
| warm-up | 5.28 | 39.6 | pipelines/compile_ms 1482, pipelines/front_ms 1449, main/busy/ms 51.66, render/thread/ms 38.32, render/busy/ms 36.97, pipelines/driver_ms 32.76, stage/main/post_update 23.21, stage/main/update 19.48, stage/render/render 13.40, render/graph/ms 11.50, encode//sum 8.31, stage/render/graph_submit 8.29 |
| warm-up | 5.35 | 29.0 | pipelines/compile_ms 217, pipelines/front_ms 214, render/thread/ms 49.90, render/busy/ms 48.53, main/busy/ms 42.93, stage/main/update 24.83, stage/render/render 12.28, stage/render/prepare_meshes 11.91, stage/main/post_update 11.80, render/graph/ms 10.20, stage/render/prepare_assets 9.77, encode//sum 7.97 |
| warm-up | 5.38 | 27.9 | render/thread/ms 54.09, render/busy/ms 52.36, main/busy/ms 40.47, stage/main/update 21.48, stage/render/render 15.85, render/graph/ms 13.71, stage/main/post_update 12.92, stage/render/prepare_assets 12.87, stage/render/prepare_meshes 11.31, stage/render/graph_submit 9.86, encode//sum 9.24, encode/submit 7.73 |
| warm-up | 5.44 | 37.1 | pipelines/compile_ms 745, pipelines/front_ms 721, render/thread/ms 50.76, render/busy/ms 48.97, main/busy/ms 35.25, pipelines/driver_ms 24.87, stage/render/prepare_assets 20.16, stage/render/render 17.91, render/graph/ms 15.06, stage/main/update 14.70, stage/main/post_update 12.98, encode//sum 10.69 |
| warm-up | 5.46 | 27.5 | pipelines/compile_ms 737, pipelines/front_ms 713, render/thread/ms 60.30, render/busy/ms 58.15, main/busy/ms 39.62, stage/render/prepare_assets 28.46, pipelines/driver_ms 23.81, stage/render/render 19.51, render/graph/ms 16.01, stage/main/update 14.72, stage/main/post_update 14.62, encode//sum 12.54 |
| warm-up | 5.49 | 26.8 | pipelines/compile_ms 191, pipelines/front_ms 187, render/thread/ms 49.32, render/busy/ms 47.90, main/busy/ms 33.15, stage/render/prepare_assets 28.95, stage/render/render 11.81, pipelines/wait_ms 11.65, stage/main/update 11.32, stage/main/post_update 10.77, encode//sum 9.69, render/graph/ms 9.59 |
| warm-up | 5.67 | 37.8 | render/thread/ms 49.25, render/busy/ms 47.52, main/busy/ms 43.28, stage/main/update 24.76, stage/render/prepare_assets 19.87, stage/render/render 14.18, render/graph/ms 11.78, stage/main/post_update 11.17, pipelines/compile_ms 9.51, pipelines/front_ms 8.83, encode//sum 8.74, stage/render/graph_submit 8.42 |
| warm-up | 5.69 | 26.5 | render/thread/ms 53.90, render/busy/ms 52.30, main/busy/ms 34.54, stage/render/prepare_assets 22.78, stage/render/render 16.47, pipelines/compile_ms 16.36, pipelines/front_ms 14.96, render/graph/ms 14.15, stage/main/update 13.58, stage/main/post_update 11.15, stage/render/graph_submit 10.37, encode//sum 8.38 |
| warm-up | 5.72 | 28.7 | render/thread/ms 51.82, render/busy/ms 50.34, main/busy/ms 39.44, stage/render/prepare_assets 23.68, stage/render/render 16.97, stage/main/update 16.37, render/graph/ms 14.72, stage/main/post_update 11.55, stage/render/graph_submit 10.86, encode//sum 8.81, encode/submit 7.64, pipelines/compile_ms 6.85 |
| warm-up | 5.75 | 29.2 | render/thread/ms 54.49, render/busy/ms 53.43, main/busy/ms 48.75, stage/render/prepare_assets 32.75, stage/main/update 22.39, stage/main/post_update 15.67, stage/render/render 9.99, render/graph/ms 8.48, encode//sum 7.46, stage/render/graph_submit 6.13, stream/drive/ms 5.93, encode/submit 3.82 |
| warm-up | 5.79 | 41.0 | render/thread/ms 66.68, render/busy/ms 64.91, main/busy/ms 49.86, stage/render/prepare_assets 28.34, stage/main/update 25.23, pipelines/compile_ms 18.03, stage/main/post_update 16.97, pipelines/front_ms 16.76, stage/render/render 15.03, render/graph/ms 12.58, encode//sum 10.25, stage/render/graph_submit 8.58 |
| warm-up | 5.85 | 33.5 | render/thread/ms 51.78, render/busy/ms 50.66, main/busy/ms 44.05, pipelines/compile_ms 25.94, pipelines/front_ms 25.09, stage/main/update 23.01, stage/render/prepare_assets 22.16, stage/main/post_update 12.87, stage/render/render 12.24, render/graph/ms 10.67, encode//sum 9.75, stage/render/graph_submit 7.56 |
| warm-up | 5.94 | 32.6 | pipelines/compile_ms 171, pipelines/front_ms 165, main/busy/ms 42.78, render/thread/ms 39.08, render/busy/ms 37.79, stage/main/update 26.94, stage/render/render 12.54, stage/main/post_update 10.64, render/graph/ms 10.63, stage/render/prepare_assets 9.76, encode//sum 8.07, stage/render/graph_submit 7.13 |
| warm-up | 6.01 | 31.4 | render/thread/ms 46.00, main/busy/ms 44.60, render/busy/ms 44.32, stage/main/update 19.33, stage/main/post_update 18.45, stage/render/render 17.81, render/graph/ms 15.39, pipelines/compile_ms 12.67, pipelines/front_ms 11.59, stage/render/graph_submit 10.67, stage/render/prepare_assets 9.39, encode//sum 9.39 |
| warm-up | 6.04 | 30.7 | render/thread/ms 56.27, render/busy/ms 54.67, main/busy/ms 50.07, stage/main/update 21.44, stage/render/prepare_assets 17.75, stage/main/post_update 17.23, stage/render/render 17.01, render/graph/ms 14.62, stage/render/graph_submit 9.88, encode//sum 9.32, stage/render/prepare_meshes 9.04, encode/submit 6.49 |
| warm-up | 6.08 | 33.0 | pipelines/compile_ms 264, pipelines/front_ms 254, render/thread/ms 58.92, render/busy/ms 57.67, main/busy/ms 40.40, stage/render/prepare_assets 29.56, stage/main/update 18.60, stage/render/render 11.46, stage/main/post_update 10.59, pipelines/driver_ms 9.92, render/graph/ms 9.77, encode//sum 8.95 |
| warm-up | 6.13 | 29.1 | pipelines/compile_ms 183, pipelines/front_ms 176, render/thread/ms 45.05, main/busy/ms 43.85, render/busy/ms 43.41, stage/main/update 25.98, stage/render/render 17.63, render/graph/ms 15.35, encode//sum 11.70, stage/render/graph_submit 11.59, stage/render/prepare_assets 11.39, stage/main/post_update 10.67 |
| warm-up | 6.22 | 26.6 | pipelines/compile_ms 104, pipelines/front_ms 102, render/thread/ms 44.89, render/busy/ms 43.17, main/busy/ms 35.98, stage/render/render 17.92, stage/main/post_update 15.54, render/graph/ms 15.50, stage/main/update 14.76, stage/render/prepare_assets 11.31, stage/render/graph_submit 10.76, encode//sum 10.39 |
| warm-up | 6.70 | 27.3 | main/busy/ms 39.16, render/thread/ms 32.70, render/busy/ms 31.75, pipelines/compile_ms 21.38, pipelines/front_ms 20.36, stage/main/update 19.76, stage/render/render 11.48, stage/main/post_update 11.27, render/graph/ms 9.92, encode//sum 9.43, stage/render/prepare_assets 6.81, stage/render/prepare_meshes 6.49 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --resolution 2560x1440 --bench-out target/perf\gc_p0\gc_ascadian@2560x1440_arm=off-1 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **97%** (min 93%) · 2625 MHz (min 2625) · memory 8101 MHz · 87.5 W · 75 °C (max 78) · VRAM 5988 MiB · throttled 100% of samples

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
