# `exterior_gc_ascadian`

- bench_ground: false · build: release · camera: [292.0, 11.1, 760.0] -> [330.0, 10.1, 730.0] · cell: Bevyrrowind - 2,-7 (2, -7) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); groundcover.density_scale=0 (command line: --set); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1305 frames over 15.0 s · **median 11.52 ms (87 fps)** · mean 11.50 · p95 13.14 · p99 13.90 (1% low 72 fps) · worst 16.96 ms · 0 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 308 frames over 9.4 s, worst 2540.9 ms
- hitches (> 28.8 ms): **0** measured, 37 in warm-up
- timed GPU passes: 9.52 ms of a 11.50 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.52 ms, limit 11 ms
- pass worst frame 16.96 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.033 | 2.061 | 2.084 | 0.99 | 0.184 |
| main_opaque_pass_3d | 2.032 | 2.060 | 2.091 | 0.99 | 0.092 |
| early prepass | 0.733 | 0.744 | 0.756 | 0.99 | 0.126 |
| gpu_instance_materialization | 0.708 | 0.717 | 0.904 | 0.99 | 0.009 |
| sun_visibility | 0.677 | 0.687 | 0.703 | 0.99 | 0.007 |
| atmosphere_luts | 0.359 | 0.364 | 0.427 | 0.99 | 0.003 |
| water_velocity | 0.325 | 0.330 | 0.333 | 0.99 | 0.002 |
| bloom | 0.316 | 0.321 | 0.337 | 0.99 | 0.063 |
| atmos_froxel_inject | 0.308 | 0.312 | 0.312 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.252 | 0.256 | 0.289 | 0.99 | 0.004 |
| water_sspr | 0.203 | 0.206 | 0.205 | 0.99 | 0.002 |
| ssao | 0.186 | 0.189 | 0.197 | 0.99 | 0.008 |
| uniform_allocation | 0.144 | 0.146 | 0.189 | 0.99 | 0.024 |
| water_fft | 0.134 | 0.136 | 0.139 | 0.99 | 0.002 |
| upscaling | 0.121 | 0.123 | 0.128 | 0.99 | 0.004 |
| bin_unpacking | 0.108 | 0.109 | 0.141 | 0.99 | 0.016 |
| clouds_composite | 0.107 | 0.109 | 0.111 | 0.99 | 0.002 |
| range_unpacking | 0.086 | 0.087 | 0.099 | 0.99 | 0.003 |
| tonemapping | 0.084 | 0.085 | 0.089 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.079 | 0.080 | 0.092 | 0.99 | 0.008 |
| clustering | 0.068 | 0.069 | 0.080 | 0.99 | 0.020 |
| render_sky | 0.062 | 0.063 | 0.065 | 0.99 | 0.002 |
| clouds_trace | 0.056 | 0.057 | 0.062 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.047 | 0.048 | 0.049 | 0.99 | 0.001 |
| point_shadow_atlas | 0.037 | 0.074 | 0.081 | 0.50 | 0.131 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 0.99 | 0.001 |
| per_view_shadow_pass | 0.028 | 0.029 | 0.055 | 0.99 | 0.007 |
| lightprobe_radiance_map | 0.027 | 0.221 | 0.233 | 0.12 | 0.004 |
| water_snapshot | 0.023 | 0.023 | 0.025 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| clouds_shadow | 0.021 | 0.021 | 0.023 | 0.99 | 0.003 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.028 | 0.99 | 0.231 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.99 | 0.003 |
| point_shadow_atlas/early_mesh_preprocessing | 0.008 | 0.015 | 0.017 | 0.50 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.007 | 0.99 | 0.001 |
| point_shadow_atlas/bin_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| point_shadow_atlas/range_unpacking | 0.003 | 0.007 | 0.007 | 0.50 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.297 | 11.929 | 13.66 | 13437.1 |
| render/busy/ms | 8.928 | 10.391 | 12.58 | 11651.1 |
| main/busy/ms | 8.385 | 9.499 | 11.79 | 10943.0 |
| stage/render/render | 5.618 | 6.746 | 9.48 | 7331.9 |
| render/graph/ms | 5.002 | 6.063 | 8.08 | 6527.9 |
| encode//sum | 4.905 | 6.833 | 10.09 | 6400.6 |
| stage/main/post_update | 4.860 | 5.779 | 7.15 | 6342.4 |
| stage/render/graph_submit | 3.617 | 4.650 | 6.57 | 4720.4 |
| encode/submit | 2.179 | 2.796 | 4.88 | 2843.3 |
| stage/main/update | 1.768 | 2.166 | 2.79 | 2307.6 |
| encode/finish_wall | 1.471 | 2.035 | 3.04 | 1920.1 |
| stage/render/camera_driver | 1.308 | 1.522 | 1.78 | 1706.5 |
| pipelines/compile_ms | 1.140 | 0.000 | 131.00 | 1487.8 |
| pipelines/front_ms | 1.108 | 0.000 | 127.91 | 1445.6 |
| stage/render/prepare_views | 1.053 | 2.932 | 4.34 | 1374.2 |
| render/acquire/ms | 1.008 | 2.873 | 4.29 | 1315.2 |
| encode/pass:early prepass/sum | 0.933 | 1.264 | 1.75 | 1217.6 |
| encode/pass:shadow_directional_light/sum | 0.931 | 1.612 | 2.31 | 1215.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.898 | 1.253 | 1.73 | 1171.6 |
| stage/render/prepare_assets | 0.582 | 0.711 | 1.64 | 759.0 |
| bigspace/total/ms | 0.544 | 1.138 | 2.56 | 710.4 |
| stage/render/prepare_write_phase_buffers | 0.530 | 0.768 | 0.99 | 691.5 |
| stage/main/fixed_main_loop | 0.524 | 0.880 | 2.14 | 683.3 |
| stage/render/prepare_meshes | 0.519 | 0.729 | 1.16 | 676.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.431 | 0.635 | 1.12 | 562.3 |
| stage/render/prepare_resources | 0.427 | 0.501 | 0.68 | 557.2 |
| stage/main/pre_update | 0.397 | 0.514 | 1.02 | 517.7 |
| stage/main/first | 0.374 | 0.448 | 2.43 | 488.0 |
| render/present/ms | 0.361 | 0.425 | 1.68 | 470.8 |
| controller/feed/ms | 0.325 | 0.412 | 0.70 | 423.6 |
| stage/render/prepare_flush | 0.318 | 0.477 | 0.99 | 415.5 |
| stage/render/prepare_batch_phases | 0.317 | 0.446 | 0.65 | 414.0 |
| stage/render/prepare_bind_groups | 0.283 | 0.457 | 0.72 | 369.2 |
| stage/main/last | 0.270 | 0.337 | 1.03 | 352.7 |
| bigspace/low_precision/ms | 0.247 | 0.520 | 2.29 | 322.3 |
| stage/render/extract_commands | 0.197 | 0.228 | 0.48 | 257.0 |
| stage/render/create_views | 0.187 | 0.227 | 0.63 | 243.8 |
| stream/drive/ms | 0.179 | 0.255 | 0.67 | 233.5 |
| anim/evaluate_cpu/ms | 0.174 | 0.212 | 0.59 | 227.7 |
| stage/render/specialize | 0.168 | 0.228 | 0.69 | 218.7 |
| anim/evaluate/ms | 0.165 | 0.313 | 2.25 | 215.4 |
| stream/tick/ms | 0.149 | 0.217 | 0.65 | 194.6 |
| stage/render/queue | 0.137 | 0.178 | 0.31 | 178.2 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.134 | 0.356 | 1.36 | 174.4 |
| bigspace/high_precision/ms | 0.132 | 0.329 | 1.85 | 172.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.120 | 0.190 | 0.43 | 156.7 |
| controller/move/ms | 0.082 | 0.218 | 0.85 | 106.7 |
| controller/write/ms | 0.063 | 0.080 | 0.20 | 82.5 |
| encode/resolve_diagnostics/sum | 0.054 | 0.087 | 0.18 | 70.8 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.071 | 0.24 | 58.8 |
| physics/step/ms | 0.042 | 0.071 | 0.47 | 54.5 |
| terrain/update/ms | 0.036 | 0.049 | 0.24 | 46.6 |
| stage/render/phase_sort | 0.035 | 0.054 | 0.10 | 45.9 |
| pipelines/driver_ms | 0.032 | 0.000 | 4.49 | 42.2 |
| stage/render/cleanup | 0.024 | 0.033 | 0.27 | 31.1 |
| encode/camera_driver_span_begin/sum | 0.022 | 0.039 | 0.18 | 29.3 |
| encode/camera_driver_span_end/sum | 0.020 | 0.036 | 0.20 | 26.4 |
| weather/map/ms | 0.015 | 0.021 | 0.04 | 20.1 |
| anim/apply/ms | 0.010 | 0.013 | 0.03 | 13.6 |
| controller/avoid/ms | 0.010 | 0.018 | 0.03 | 12.7 |
| weather/eval/ms | 0.009 | 0.011 | 0.04 | 11.2 |
| script/update/ms | 0.008 | 0.011 | 0.38 | 9.9 |
| script/input/ms | 0.005 | 0.007 | 0.01 | 6.5 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 5.5 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 4.9 |
| stream/poll_models/ms | 0.002 | 0.003 | 0.04 | 2.8 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.00 | 1.7 |
| stream/promote/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.9 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.03 | 0.4 |
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
| anim/a3/count | count | 447 | 448 | 447 |
| anim/characters/count | count | 450 | 450 | 450 |
| anim/evaluated/count | count | 27 | 27 | 26 |
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
| crowd/joints_extracted/count | count | 771 | 1167 | 719 |
| crowd/joints_uploaded/count | count | 6368 | 6368 | 6368 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 43.8 k | 43.8 k | 43.8 k |
| encode//max | ms | 0.63 | 2.09 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.18 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.20 | 0.02 |
| encode/finish_critical | ms | 1.07 | 2.31 | 1.12 |
| encode/pass:early prepass/max | ms | 0.93 | 1.75 | 0.89 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.90 | 1.73 | 1.12 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.43 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.41 | 1.12 | 0.34 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.25 | 1.36 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.87 | 2.31 | 0.54 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.18 | 0.03 |
| far/entities | count | 2625 | 2625 | 2625 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 55.6 k | 55.6 k | 55.6 k |
| far/lights | count | 6627 | 6627 | 6627 |
| far/nodes/l1 | count | 879 | 879 | 879 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 175.14 M | 175.14 M | 175.14 M |
| far/triangles | count | 4.29 M | 4.29 M | 4.29 M |
| frame/work/ms | ms | 10.33 | 12.58 | 10.29 |
| gpu/timed/ms | ms | 10.28 | 11.74 | 10.29 |
| groundcover/batches/count | count | 18 | 18 | 18 |
| groundcover/jobs/pending | count | 0 | 0 | 0 |
| groundcover/scatter/us | us | 798 | 798 | 798 |
| groundcover/slots/capacity | count | 658.0 k | 658.0 k | 658.0 k |
| groundcover/slots/capacity_bytes | bytes | 42.11 M | 42.11 M | 42.11 M |
| groundcover/slots/live | count | 0 | 0 | 0 |
| groundcover/tiles/capped | count | 0 | 0 | 0 |
| groundcover/tiles/casting | count | 5 | 5 | 5 |
| groundcover/tiles/resident | count | 63 | 63 | 63 |
| groundcover/tiles/scattered | count | 63 | 63 | 63 |
| groundcover/tiles/unplaced | count | 0 | 0 | 0 |
| groundcover/upload/bytes | bytes | 0 | 0 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2433 | 2433 | 2433 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 303.9 k | 303.9 k | 303.9 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.6 k | 12.6 k | 12.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 343.66 M | 343.66 M | 343.66 M |
| phases/alpha_mask3d/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d/bins | count | 364 | 364 | 364 |
| phases/alpha_mask3d/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d/instance_draws | count | 569 | 569 | 569 |
| phases/alpha_mask3d/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d/items | count | 569 | 569 | 569 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/alpha_mask3d_prepass/bins | count | 364 | 364 | 364 |
| phases/alpha_mask3d_prepass/instance_batches | count | 12 | 12 | 12 |
| phases/alpha_mask3d_prepass/instance_draws | count | 569 | 569 | 569 |
| phases/alpha_mask3d_prepass/instance_slots | count | 117.5 k | 117.5 k | 117.5 k |
| phases/alpha_mask3d_prepass/items | count | 569 | 569 | 569 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 31 | 31 | 31 |
| phases/opaque3d/bins | count | 1369 | 1369 | 1369 |
| phases/opaque3d/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d/items | count | 2851 | 2851 | 2851 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 23 | 23 | 23 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 28 | 28 |
| phases/opaque3d_prepass/bins | count | 1369 | 1369 | 1369 |
| phases/opaque3d_prepass/instance_batches | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/instance_draws | count | 6761 | 6761 | 6761 |
| phases/opaque3d_prepass/instance_slots | count | 589.6 k | 589.6 k | 589.6 k |
| phases/opaque3d_prepass/items | count | 2851 | 2851 | 2851 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 90 | 121 | 105 |
| phases/shadow/bins | count | 2178 | 2195 | 2161 |
| phases/shadow/instance_batches | count | 9 | 27 | 9 |
| phases/shadow/instance_draws | count | 7317 | 22.0 k | 7317 |
| phases/shadow/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| phases/shadow/items | count | 5357 | 5374 | 5340 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 54 | 54 | 52 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2374 | 2374 | 2374 |
| physics/items | count | 79 | 79 | 79 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 56 | 56 | 56 |
| pipelines/total | count | 358 | 358 | 358 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 9 | 149 | 9 |
| preprocess/bind_groups_created | count | 4 | 142 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 137 | 137 | 137 |
| process/cpu_usage | % | 27.23 | 46.89 | 25.96 |
| process/mem_usage | GiB | 3.13 | 3.20 | 3.20 |
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
| shadow_atlas/mover_draws | count | 2 | 4 | 2 |
| shadow_atlas/movers | count | 100 | 100 | 100 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 2 | 4 | 4 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 4 | 4 | 4 |
| shadow_atlas/render/live_direct | count | 2 | 4 | 4 |
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
| skin/blocks_changed/count | count | 19 | 26 | 18 |
| skin/blocks_skipped/count | count | 191 | 201 | 192 |
| skin/blocks_walked/count | count | 19 | 27 | 18 |
| skin/joint_bytes_written/bytes | bytes | 60.2 k | 80.6 k | 66.8 k |
| skin/joints_written/count | count | 940 | 1260 | 1044 |
| skin/upload_writes/count | count | 8 | 12 | 8 |
| started_load_count |  | 5837 | 5838 | 5838 |
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
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
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
| system/cpu_usage | % | 33.22 | 43.32 | 29.12 |
| system/mem_usage | % | 56.31 | 56.52 | 56.52 |
| terrain/patches | count | 493 | 493 | 493 |
| textures/gpu_mib | MiB | 2953 | 2953 | 2953 |
| textures/images | count | 2502 | 2502 | 2502 |
| views/all/batch_sets | count | 187 | 218 | 202 |
| views/all/bins | count | 5642 | 5661 | 5627 |
| views/all/count | count | 10 | 10 | 10 |
| views/all/instance_batches | count | 53 | 71 | 53 |
| views/all/instance_draws | count | 22.0 k | 36.6 k | 22.0 k |
| views/all/instance_slots | count | 1.50 M | 1.66 M | 1.50 M |
| views/all/items | count | 12.2 k | 12.2 k | 12.2 k |
| views/all/pipelines | count | 112 | 112 | 110 |
| views/all/pipelines_distinct | count | 79 | 79 | 79 |
| views/all/work_items | count | 1.50 M | 1.68 M | 1.50 M |
| views/cascade/batch_sets | count | 14 | 60 | 14 |
| views/cascade/bins | count | 2027 | 2043 | 2027 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 9 | 27 | 9 |
| views/cascade/instance_draws | count | 7317 | 22.0 k | 7317 |
| views/cascade/instance_slots | count | 82.9 k | 248.6 k | 82.9 k |
| views/cascade/items | count | 5206 | 5222 | 5206 |
| views/cascade/pipelines | count | 43 | 43 | 43 |
| views/cascade/work_items | count | 83.0 k | 253.8 k | 83.0 k |
| views/lamp_face/batch_sets | count | 43 | 91 | 91 |
| views/lamp_face/bins | count | 151 | 152 | 134 |
| views/lamp_face/count | count | 6 | 6 | 6 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 151 | 152 | 134 |
| views/lamp_face/pipelines | count | 11 | 11 | 9 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 43 | 91 | 91 |
| views/main/batch_sets | count | 97 | 97 | 97 |
| views/main/bins | count | 3466 | 3466 | 3466 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 44 | 44 | 44 |
| views/main/instance_draws | count | 14.7 k | 14.7 k | 14.7 k |
| views/main/instance_slots | count | 1.41 M | 1.41 M | 1.41 M |
| views/main/items | count | 6840 | 6840 | 6840 |
| views/main/pipelines | count | 58 | 58 | 58 |
| views/main/work_items | count | 1.42 M | 1.42 M | 1.42 M |
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
| anim/bones_written/count | count | 38.33 | 104 | 50.0 k |
| anim/events/count | count | 0.04 | 2 | 48 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.76 | 8 | 3600 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| encode//count | count | 28.25 | 30 | 36.9 k |
| encode/bind_walk/init_textures | count | 9349 | 13.1 k | 12.20 M |
| encode/bind_walk/init_textures_skipped | count | 62.8 k | 73.7 k | 81.96 M |
| encode/bind_walk/inits | count | 802 | 913 | 1.05 M |
| encode/bind_walk/inits_skipped | count | 173 | 187 | 226.2 k |
| encode/bind_walk/merge_textures | count | 9287 | 13.0 k | 12.12 M |
| encode/bind_walk/merge_textures_skipped | count | 19.1 k | 30.0 k | 24.87 M |
| encode/bind_walk/merges | count | 159 | 183 | 207.1 k |
| encode/bind_walk/merges_skipped | count | 159 | 181 | 207.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1305 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1305 |
| encode/pass:early prepass/count | count | 1 | 1 | 1305 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1305 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1305 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1305 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.50 | 1 | 653 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0 | 0 | 0 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1305 |
| encode/queue/command_buffers | count | 38.75 | 41 | 50.6 k |
| encode/queue/submits | count | 2 | 2 | 2610 |
| encode/queue/write_buffer/bytes | bytes | 2.76 M | 3.50 M | 3.60 G |
| encode/queue/write_buffer/calls | count | 280 | 352 | 364.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 53.9 k | 53.9 k | 70.32 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.7 k |
| encode/queue/write_texture/bytes | bytes | 20.2 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1305 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.69 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 22 |
| pipelines/created | count | 0.02 | 1 | 21 |
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

## Hitches (> 28.8 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2540.9 | main/busy/ms 2508, stage/main/post_update 2461, stage/main/update 33.83, stage/main/pre_update 9.50, stage/main/last 1.09, surface/advance_cpu_ms 0.78, stage/main/first 0.56, stage/main/fixed_main_loop 0.40, surface/cell_upload_ms 0.30, stream/drive/ms 0.29, bigspace/total/ms 0.09, stream/tick/ms 0.07 |
| warm-up | 3.31 | 771.6 | main/busy/ms 2556, stage/main/post_update 2495, render/thread/ms 704, render/busy/ms 536, stage/render/prepare_assets 500, stage/render/prepare_views 165, render/acquire/ms 165, stage/main/update 40.59, stage/render/render 21.88, render/graph/ms 18.00, stage/render/camera_driver 16.25, stage/main/pre_update 11.38 |
| warm-up | 3.47 | 161.4 | render/thread/ms 863, render/busy/ms 596, stage/render/prepare_assets 510, stage/render/prepare_views 166, render/acquire/ms 165, stage/render/render 157, render/present/ms 101, pipelines/compile_ms 75.36, pipelines/front_ms 71.96, main/busy/ms 71.95, render/graph/ms 54.05, stage/main/post_update 46.88 |
| warm-up | 3.60 | 130.6 | pipelines/compile_ms 1748, pipelines/front_ms 1082, pipelines/driver_ms 665, render/thread/ms 288, render/busy/ms 187, stage/render/render 140, pipelines/wait_ms 121, render/present/ms 99.95, stage/render/prepare_flush 75.20, main/busy/ms 59.37, pipelines/lock_ms 51.85, stage/render/prepare_assets 51.50 |
| warm-up | 3.71 | 101.6 | pipelines/compile_ms 3161, pipelines/front_ms 2396, pipelines/wait_ms 2370, pipelines/driver_ms 765, render/thread/ms 226, render/busy/ms 223, main/busy/ms 134, stage/render/prepare_assets 95.95, stage/render/prepare_flush 91.44, stage/main/update 66.23, pipelines/lock_ms 43.11, stage/main/post_update 33.70 |
| warm-up | 3.81 | 99.7 | pipelines/wait_ms 4804, pipelines/compile_ms 2668, pipelines/front_ms 2504, render/thread/ms 190, render/busy/ms 188, pipelines/driver_ms 164, stage/render/prepare_assets 136, main/busy/ms 125, stage/main/update 61.98, stage/main/post_update 27.62, stage/main/fixed_main_loop 20.31, terrain/update/ms 18.20 |
| warm-up | 3.90 | 97.8 | pipelines/wait_ms 3084, pipelines/compile_ms 1986, pipelines/front_ms 1834, render/thread/ms 189, render/busy/ms 187, pipelines/driver_ms 152, stage/render/prepare_assets 89.52, encode//sum 60.83, main/busy/ms 47.29, stage/render/render 43.60, render/graph/ms 40.96, stage/render/camera_driver 20.84 |
| warm-up | 3.97 | 43.0 | pipelines/compile_ms 491, pipelines/front_ms 467, pipelines/wait_ms 148, render/thread/ms 49.67, render/busy/ms 46.18, main/busy/ms 41.60, pipelines/driver_ms 23.56, stage/main/post_update 22.77, stage/render/render 18.71, render/graph/ms 15.18, stage/main/update 12.74, stage/render/graph_submit 12.44 |
| warm-up | 4.02 | 53.2 | pipelines/compile_ms 1758, pipelines/front_ms 1622, pipelines/wait_ms 300, pipelines/driver_ms 136, render/thread/ms 81.97, render/busy/ms 78.06, main/busy/ms 45.49, stage/render/render 39.47, render/graph/ms 34.84, stage/render/graph_submit 31.45, encode/submit 26.88, stage/main/post_update 19.70 |
| warm-up | 4.08 | 58.5 | pipelines/compile_ms 1864, pipelines/front_ms 1720, pipelines/wait_ms 339, pipelines/driver_ms 144, render/thread/ms 105, render/busy/ms 102, main/busy/ms 46.29, stage/render/render 40.30, stage/render/prepare_assets 39.32, render/graph/ms 36.41, stage/render/graph_submit 32.52, encode/submit 27.63 |
| warm-up | 4.13 | 56.5 | pipelines/compile_ms 1070, pipelines/front_ms 1023, pipelines/wait_ms 185, render/thread/ms 109, render/busy/ms 106, stage/render/prepare_assets 66.34, pipelines/driver_ms 47.05, main/busy/ms 35.64, stage/render/render 25.89, render/graph/ms 22.23, stage/render/graph_submit 18.10, stage/main/update 13.56 |
| warm-up | 4.22 | 85.9 | pipelines/compile_ms 871, pipelines/front_ms 844, render/thread/ms 134, render/busy/ms 131, stage/render/prepare_assets 99.23, pipelines/wait_ms 40.29, main/busy/ms 34.01, pipelines/lock_ms 29.62, pipelines/driver_ms 27.39, stage/render/render 22.13, render/graph/ms 19.42, stage/render/graph_submit 15.89 |
| warm-up | 4.26 | 41.6 | pipelines/compile_ms 620, pipelines/front_ms 601, render/thread/ms 112, render/busy/ms 110, stage/render/prepare_assets 90.48, pipelines/wait_ms 30.42, pipelines/lock_ms 29.62, main/busy/ms 28.19, pipelines/driver_ms 19.24, stage/render/render 15.00, render/graph/ms 13.34, stage/render/graph_submit 10.22 |
| warm-up | 4.42 | 31.1 | pipelines/compile_ms 922, pipelines/front_ms 885, render/thread/ms 48.46, render/busy/ms 46.83, pipelines/driver_ms 37.04, main/busy/ms 36.75, stage/render/prepare_assets 26.41, stage/main/update 20.09, stage/render/render 11.76, stage/main/post_update 10.12, render/graph/ms 9.84, pipelines/wait_ms 9.38 |
| warm-up | 4.45 | 33.8 | pipelines/compile_ms 925, pipelines/front_ms 888, render/thread/ms 62.06, render/busy/ms 60.43, main/busy/ms 44.48, pipelines/driver_ms 37.47, stage/render/prepare_assets 37.04, stage/main/update 22.69, stage/main/post_update 10.92, stage/render/render 10.10, encode//sum 9.23, pipelines/wait_ms 8.90 |
| warm-up | 4.57 | 116.3 | pipelines/compile_ms 637, pipelines/front_ms 620, render/thread/ms 135, render/busy/ms 133, main/busy/ms 99.22, stage/main/update 53.65, stage/render/prepare_resources 27.94, stage/render/prepare_write_phase_buffers 27.22, stage/render/prepare_assets 26.09, stage/main/post_update 20.54, stage/render/prepare_bind_groups 17.97, pipelines/driver_ms 16.52 |
| warm-up | 4.68 | 115.6 | pipelines/compile_ms 2793, pipelines/front_ms 2680, render/thread/ms 216, render/busy/ms 213, main/busy/ms 168, pipelines/driver_ms 114, stage/main/update 105, pipelines/wait_ms 104, stage/render/prepare_write_phase_buffers 47.69, stage/render/prepare_resources 46.80, stage/render/prepare_assets 46.65, stage/main/post_update 34.32 |
| warm-up | 4.76 | 76.5 | pipelines/compile_ms 2433, pipelines/front_ms 2317, render/thread/ms 183, render/busy/ms 181, pipelines/wait_ms 151, main/busy/ms 117, pipelines/driver_ms 116, stage/render/prepare_assets 88.18, stage/main/update 72.34, stage/main/post_update 27.48, stage/render/render 24.91, stage/render/prepare_write_phase_buffers 23.81 |
| warm-up | 4.85 | 89.7 | pipelines/compile_ms 1788, pipelines/front_ms 1730, pipelines/wait_ms 274, render/thread/ms 158, render/busy/ms 156, stage/render/prepare_assets 114, main/busy/ms 60.98, pipelines/driver_ms 58.03, stage/main/update 25.93, stage/render/render 18.66, render/graph/ms 16.34, stage/main/post_update 14.68 |
| warm-up | 4.91 | 58.7 | pipelines/compile_ms 2080, pipelines/front_ms 2016, pipelines/wait_ms 227, render/thread/ms 146, render/busy/ms 144, stage/render/prepare_assets 101, pipelines/driver_ms 64.40, main/busy/ms 62.52, stage/main/update 31.64, stage/render/render 22.48, render/graph/ms 19.88, stage/render/graph_submit 15.34 |
| warm-up | 4.95 | 39.1 | pipelines/compile_ms 589, pipelines/front_ms 564, render/thread/ms 94.48, render/busy/ms 92.51, main/busy/ms 65.70, stage/render/prepare_assets 46.34, stage/main/update 35.73, pipelines/driver_ms 25.38, stage/render/render 20.99, stage/main/post_update 19.54, render/graph/ms 18.37, stage/render/graph_submit 13.39 |
| warm-up | 4.98 | 36.1 | pipelines/compile_ms 863, pipelines/front_ms 839, render/thread/ms 70.29, render/busy/ms 68.50, main/busy/ms 57.10, stage/main/update 27.41, pipelines/driver_ms 24.37, stage/render/render 20.65, stage/main/post_update 19.25, stage/render/prepare_assets 19.18, render/graph/ms 18.50, pipelines/wait_ms 14.39 |
| warm-up | 5.01 | 31.5 | pipelines/compile_ms 906, pipelines/front_ms 880, render/thread/ms 63.08, render/busy/ms 61.35, main/busy/ms 48.85, stage/main/update 28.70, pipelines/driver_ms 26.45, stage/render/render 19.06, render/graph/ms 17.12, pipelines/wait_ms 15.55, stage/render/prepare_assets 13.39, stage/render/graph_submit 12.62 |
| warm-up | 5.09 | 31.0 | pipelines/compile_ms 164, pipelines/front_ms 153, main/busy/ms 45.97, render/thread/ms 33.40, render/busy/ms 31.42, stage/main/update 22.12, stage/render/render 20.88, render/graph/ms 18.33, stage/main/post_update 16.10, stage/render/graph_submit 14.17, encode//sum 11.00, pipelines/driver_ms 10.21 |
| warm-up | 5.25 | 36.6 | pipelines/compile_ms 455, pipelines/front_ms 441, render/thread/ms 53.39, main/busy/ms 43.62, render/busy/ms 40.31, stage/render/render 13.85, pipelines/driver_ms 13.57, stage/main/post_update 13.13, stage/render/prepare_views 12.04, render/graph/ms 11.96, render/acquire/ms 11.88, stage/main/update 11.49 |
| warm-up | 5.39 | 28.9 | pipelines/compile_ms 1157, pipelines/front_ms 1131, main/busy/ms 49.92, render/thread/ms 46.04, render/busy/ms 44.53, stage/main/update 34.48, pipelines/driver_ms 26.89, stage/render/render 18.70, render/graph/ms 16.48, stream/drive/ms 14.54, encode//sum 11.51, stage/render/graph_submit 11.18 |
| warm-up | 5.70 | 36.4 | render/thread/ms 59.46, render/busy/ms 57.54, main/busy/ms 37.30, stage/render/prepare_assets 24.54, stage/render/render 18.45, render/graph/ms 15.59, stage/main/update 14.24, encode//sum 13.32, stage/main/post_update 12.92, stage/render/graph_submit 10.13, encode/submit 5.48, stage/render/camera_driver 5.10 |
| warm-up | 5.73 | 33.8 | render/thread/ms 66.51, render/busy/ms 64.83, main/busy/ms 34.79, stage/render/prepare_assets 32.92, stage/render/render 16.91, stage/main/update 15.39, render/graph/ms 14.42, encode//sum 13.64, stage/main/post_update 10.68, stage/render/graph_submit 10.23, stage/render/prepare_meshes 5.43, encode/submit 5.15 |
| warm-up | 5.82 | 81.2 | render/thread/ms 107, render/busy/ms 105, stage/render/prepare_assets 51.85, main/busy/ms 42.20, stage/render/prepare_flush 25.69, stage/main/update 18.65, stage/render/render 14.78, stage/main/post_update 14.25, render/graph/ms 12.73, stage/render/graph_submit 9.73, encode//sum 9.61, encode/submit 6.67 |
| warm-up | 5.95 | 47.4 | render/thread/ms 60.70, render/busy/ms 58.83, main/busy/ms 53.15, stage/main/update 29.59, stage/render/render 19.58, stage/main/post_update 17.13, render/graph/ms 16.85, stage/render/prepare_assets 13.62, encode//sum 12.62, stage/render/graph_submit 11.15, character/finish/ms 8.66, stage/render/prepare_meshes 8.26 |
| warm-up | 5.99 | 39.5 | render/thread/ms 79.26, render/busy/ms 77.28, main/busy/ms 53.26, pipelines/compile_ms 46.01, pipelines/front_ms 44.32, stage/main/update 30.74, stage/render/prepare_assets 30.51, pipelines/wait_ms 24.23, stage/render/render 18.22, render/graph/ms 15.24, stage/main/post_update 13.79, encode//sum 12.06 |
| warm-up | 6.03 | 34.5 | pipelines/compile_ms 377, pipelines/front_ms 368, render/thread/ms 68.87, render/busy/ms 67.57, main/busy/ms 43.93, stage/render/prepare_assets 37.91, pipelines/wait_ms 24.37, stage/main/update 19.83, stage/main/post_update 13.43, stage/render/render 11.72, render/graph/ms 9.79, pipelines/driver_ms 9.77 |
| warm-up | 6.09 | 42.6 | render/thread/ms 53.38, render/busy/ms 51.62, main/busy/ms 45.23, stage/main/update 29.12, pipelines/compile_ms 23.96, pipelines/front_ms 22.89, stage/render/render 18.95, render/graph/ms 16.37, stage/render/prepare_assets 16.08, encode//sum 13.49, stage/render/graph_submit 11.46, stage/main/post_update 9.66 |
| warm-up | 6.19 | 43.9 | render/thread/ms 53.40, render/busy/ms 51.88, main/busy/ms 46.15, stage/render/render 21.03, stage/main/update 20.62, render/graph/ms 18.61, stage/main/post_update 17.84, stage/render/prepare_assets 15.91, stage/render/graph_submit 12.93, encode//sum 10.95, encode/submit 7.53, encode/finish_wall 5.41 |
| warm-up | 6.25 | 33.5 | pipelines/compile_ms 638, pipelines/front_ms 615, pipelines/wait_ms 137, render/thread/ms 56.36, render/busy/ms 55.07, main/busy/ms 53.51, stage/main/update 28.09, stage/render/prepare_assets 27.29, pipelines/driver_ms 23.48, stage/main/post_update 14.13, stage/render/render 12.78, render/graph/ms 11.14 |
| warm-up | 6.31 | 29.9 | render/thread/ms 52.43, render/busy/ms 51.15, main/busy/ms 42.12, stage/main/update 21.45, stage/render/render 18.38, render/graph/ms 16.27, stage/render/prepare_assets 14.28, stage/main/post_update 14.16, encode//sum 12.09, stage/render/graph_submit 11.22, encode/submit 6.35, stage/render/prepare_meshes 5.82 |
| warm-up | 6.55 | 31.4 | render/thread/ms 44.83, render/busy/ms 43.53, main/busy/ms 37.10, stage/main/update 18.22, stage/render/render 16.22, render/graph/ms 14.41, stage/main/post_update 13.53, encode//sum 11.77, stage/render/graph_submit 10.13, stage/render/prepare_meshes 7.48, stage/render/prepare_assets 6.41, encode/submit 6.07 |

## Run

- `target/profiling/bench-gc.exe` view 2,-7 --camera 292,+1.7,760,330,+0.6,730 --hour 8 --weather clear --bench benchmarks/exterior_gc_ascadian.json --set groundcover.density_scale=0 --resolution 2560x1440 --bench-out target/perf\gc_p0\gc_ascadian@2560x1440_arm=empty-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 87.2 W · 75 °C (max 78) · VRAM 7578 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `groundcover.density_scale=0 (command line: --set)`
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
