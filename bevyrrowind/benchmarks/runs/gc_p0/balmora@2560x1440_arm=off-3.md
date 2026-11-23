# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1293 frames over 15.0 s · **median 11.68 ms (86 fps)** · mean 11.61 · p95 12.99 · p99 13.45 (1% low 74 fps) · worst 17.86 ms · 2 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 313 frames over 8.8 s, worst 2583.0 ms
- hitches (> 29.2 ms): **0** measured, 29 in warm-up
- timed GPU passes: 10.18 ms of a 11.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.68 ms, limit 11 ms
- pass worst frame 17.86 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.217 | 3.224 | 3.305 | 1.00 | 0.069 |
| dlss_super_resolution | 2.020 | 2.025 | 2.047 | 1.00 | 0.185 |
| sun_visibility | 0.777 | 0.779 | 0.807 | 1.00 | 0.006 |
| early prepass | 0.635 | 0.637 | 0.646 | 1.00 | 0.099 |
| main_transmissive_pass_3d | 0.395 | 0.396 | 0.408 | 1.00 | 0.008 |
| atmosphere_luts | 0.369 | 0.370 | 0.379 | 1.00 | 0.003 |
| bloom | 0.315 | 0.316 | 0.331 | 1.00 | 0.061 |
| atmos_froxel_inject | 0.308 | 0.309 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.305 | 0.306 | 0.310 | 1.00 | 0.001 |
| water_sspr | 0.212 | 0.213 | 0.213 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.196 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.152 | 0.171 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.134 | 0.136 | 1.00 | 0.002 |
| uniform_allocation | 0.116 | 0.116 | 0.147 | 1.00 | 0.019 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.113 | 0.113 | 0.119 | 1.00 | 0.004 |
| bin_unpacking | 0.090 | 0.090 | 0.114 | 1.00 | 0.014 |
| gpu_instance_materialization | 0.087 | 0.087 | 0.098 | 1.00 | 0.004 |
| tonemapping | 0.086 | 0.086 | 0.089 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.021 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.060 | 0.060 | 0.086 | 1.00 | 0.003 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.13 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.220 | 0.230 | 0.13 | 0.004 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.030 | 1.00 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.036 | 1.00 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.025 | 1.00 | 0.194 |
| surface_exposure_far | 0.012 | 0.040 | 0.041 | 0.31 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| surface_exposure | 0.011 | 0.040 | 0.041 | 0.28 | 0.001 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.476 | 11.872 | 16.73 | 13545.7 |
| render/busy/ms | 7.172 | 8.319 | 15.43 | 9272.9 |
| main/busy/ms | 6.684 | 8.014 | 12.18 | 8642.7 |
| stage/render/render | 4.825 | 5.698 | 11.34 | 6238.3 |
| encode//sum | 4.647 | 6.106 | 11.36 | 6008.9 |
| render/graph/ms | 4.234 | 5.073 | 9.91 | 5474.4 |
| stage/main/post_update | 3.735 | 4.661 | 9.66 | 4829.2 |
| stage/render/graph_submit | 3.054 | 3.845 | 8.62 | 3948.5 |
| stage/render/prepare_views | 2.997 | 4.650 | 5.74 | 3875.7 |
| render/acquire/ms | 2.947 | 4.604 | 5.68 | 3810.0 |
| encode/submit | 1.892 | 2.351 | 3.15 | 2446.5 |
| stage/main/update | 1.356 | 1.800 | 2.26 | 1753.9 |
| encode/finish_wall | 1.178 | 1.620 | 6.36 | 1523.5 |
| stage/render/camera_driver | 1.108 | 1.285 | 1.63 | 1432.1 |
| pipelines/compile_ms | 0.954 | 0.000 | 122.20 | 1233.8 |
| pipelines/front_ms | 0.928 | 0.000 | 119.45 | 1200.4 |
| encode/pass:early prepass/sum | 0.746 | 1.041 | 5.36 | 964.1 |
| encode/pass:shadow_directional_light/sum | 0.739 | 1.264 | 1.73 | 955.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.695 | 1.008 | 1.34 | 898.3 |
| stage/render/prepare_write_phase_buffers | 0.485 | 0.711 | 0.98 | 626.6 |
| stage/main/fixed_main_loop | 0.442 | 0.689 | 1.30 | 571.7 |
| stage/render/prepare_resources | 0.428 | 0.501 | 0.67 | 552.9 |
| stage/main/pre_update | 0.399 | 0.473 | 0.88 | 516.3 |
| stage/render/prepare_assets | 0.395 | 0.486 | 1.61 | 510.8 |
| render/present/ms | 0.358 | 0.420 | 1.19 | 462.8 |
| bigspace/total/ms | 0.355 | 0.635 | 1.84 | 459.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.325 | 0.454 | 0.78 | 420.2 |
| stage/main/first | 0.314 | 0.374 | 1.83 | 406.6 |
| stage/render/prepare_flush | 0.274 | 0.386 | 0.66 | 354.8 |
| stage/render/prepare_batch_phases | 0.266 | 0.369 | 0.51 | 344.4 |
| stage/main/last | 0.259 | 0.364 | 4.94 | 334.3 |
| stage/render/prepare_bind_groups | 0.254 | 0.357 | 0.51 | 328.6 |
| stage/render/prepare_meshes | 0.215 | 0.267 | 0.99 | 277.8 |
| controller/feed/ms | 0.179 | 0.232 | 0.68 | 231.1 |
| stage/render/extract_commands | 0.175 | 0.207 | 0.31 | 225.9 |
| stage/render/create_views | 0.159 | 0.191 | 0.29 | 205.1 |
| stream/drive/ms | 0.135 | 0.158 | 0.54 | 174.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.124 | 0.187 | 0.41 | 160.5 |
| bigspace/high_precision/ms | 0.123 | 0.304 | 1.60 | 158.6 |
| stream/tick/ms | 0.108 | 0.128 | 0.52 | 140.0 |
| stage/render/queue | 0.103 | 0.139 | 2.07 | 133.3 |
| bigspace/low_precision/ms | 0.101 | 0.245 | 1.15 | 130.3 |
| controller/move/ms | 0.083 | 0.169 | 0.49 | 107.8 |
| stage/render/specialize | 0.077 | 0.100 | 4.40 | 100.2 |
| anim/evaluate/ms | 0.076 | 0.150 | 0.87 | 98.2 |
| anim/evaluate_cpu/ms | 0.060 | 0.073 | 0.47 | 77.8 |
| encode/resolve_diagnostics/sum | 0.057 | 0.085 | 0.25 | 74.3 |
| controller/write/ms | 0.053 | 0.069 | 0.45 | 69.1 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.087 | 0.23 | 62.2 |
| terrain/update/ms | 0.037 | 0.048 | 0.48 | 47.8 |
| physics/step/ms | 0.032 | 0.053 | 0.47 | 41.2 |
| stage/render/phase_sort | 0.027 | 0.045 | 0.13 | 34.6 |
| stage/render/cleanup | 0.026 | 0.034 | 0.32 | 33.4 |
| pipelines/driver_ms | 0.026 | 0.000 | 4.10 | 33.4 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.037 | 0.07 | 31.6 |
| weather/map/ms | 0.022 | 0.030 | 0.08 | 28.9 |
| encode/camera_driver_span_end/sum | 0.022 | 0.035 | 0.08 | 27.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 13.2 |
| script/update/ms | 0.006 | 0.008 | 0.03 | 7.8 |
| anim/apply/ms | 0.005 | 0.006 | 0.14 | 6.0 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 5.9 |
| controller/avoid/ms | 0.004 | 0.007 | 0.02 | 5.5 |
| far/select/ms | 0.004 | 0.004 | 0.03 | 4.9 |
| script/ui/ms | 0.003 | 0.005 | 0.43 | 4.5 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.08 | 2.6 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.02 | 1.7 |
| character/finish/ms | 0.001 | 0.002 | 0.05 | 1.7 |
| stream/promote/ms | 0.001 | 0.001 | 0.02 | 1.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.05 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| terrain/select/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.3 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| assets/cutout_decode/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 181 | 181 | 181 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.57 | 6.21 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.07 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.08 | 0.03 |
| encode/finish_critical | ms | 0.88 | 6.21 | 1.16 |
| encode/pass:early prepass/max | ms | 0.74 | 5.36 | 0.72 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.69 | 1.34 | 0.56 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.12 | 0.41 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.32 | 0.78 | 0.37 |
| encode/pass:shadow_directional_light/max | ms | 0.70 | 1.73 | 1.16 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.25 | 0.04 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.79 | 15.43 | 10.69 |
| gpu/timed/ms | ms | 10.79 | 12.08 | 10.69 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1929 | 1929 | 1929 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 214.7 k | 214.7 k | 214.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.3 k | 11.3 k | 11.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 321.65 M | 321.65 M | 321.65 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 248 | 248 | 248 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 319 | 319 | 319 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 248 | 248 | 248 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 563 | 563 | 563 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 319 | 319 | 319 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 23 | 23 | 23 |
| phases/opaque3d/bins | count | 876 | 876 | 876 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 876 | 876 | 876 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 41 | 34 |
| phases/shadow/bins | count | 896 | 1509 | 896 |
| phases/shadow/instance_batches | count | 4 | 12 | 12 |
| phases/shadow/instance_draws | count | 5861 | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 3202 | 8161 | 3202 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 27 | 34 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 332 | 332 | 332 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 53 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 7 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 25.22 | 40.38 | 24.09 |
| process/mem_usage | GiB | 3.03 | 3.12 | 3.10 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2386 | 2386 | 2386 |
| scene/batch/masked/live | count | 2386 | 2386 | 2386 |
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
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1639 | 1639 | 1639 |
| scene/chains/live | count | 1639 | 1639 | 1639 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 0 | 0 | 0 |
| shadow_atlas/live_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/live_tiles | count | 0 | 0 | 0 |
| shadow_atlas/mover_draws | count | 0 | 0 | 0 |
| shadow_atlas/movers | count | 0 | 0 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 0 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/live_direct | count | 0 | 0 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 0 | 0 | 0 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/static_tiles | count | 0 | 0 | 0 |
| shadow_atlas/views | count | 0 | 0 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 7 | 7 | 7 |
| skin/blocks_skipped/count | count | 174 | 177 | 174 |
| skin/blocks_walked/count | count | 7 | 7 | 7 |
| skin/joint_bytes_written/bytes | bytes | 12.5 k | 12.5 k | 12.5 k |
| skin/joints_written/count | count | 196 | 196 | 196 |
| skin/upload_writes/count | count | 5 | 5 | 5 |
| started_load_count |  | 5288 | 5290 | 5290 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.77 | 45.25 | 28.23 |
| system/mem_usage | % | 54.67 | 54.84 | 54.79 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2545 | 2545 | 2545 |
| textures/images | count | 1998 | 1998 | 1998 |
| views/all/batch_sets | count | 64 | 101 | 94 |
| views/all/bins | count | 3144 | 3757 | 3144 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 20 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 245.8 k |
| views/all/items | count | 9408 | 14.4 k | 9408 |
| views/all/pipelines | count | 75 | 82 | 75 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.9 k | 260.4 k | 255.4 k |
| views/cascade/batch_sets | count | 4 | 41 | 34 |
| views/cascade/bins | count | 896 | 1509 | 896 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 12 |
| views/cascade/instance_draws | count | 5861 | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 3202 | 8161 | 3202 |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 49.2 k | 155.6 k | 150.6 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 60 | 60 | 60 |
| views/main/bins | count | 2248 | 2248 | 2248 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6206 | 6206 | 6206 |
| views/main/pipelines | count | 48 | 48 | 48 |
| views/main/work_items | count | 104.8 k | 104.8 k | 104.8 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.5 k | 15.5 k | 15.5 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 3 | 132 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 0.00 | 4 | 4 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| assets/cutout_decode/count | count | 0 | 0 | 0 |
| character/finish/count | count | 0 | 0 | 0 |
| character/finish/uploads | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 28.55 | 31 | 36.9 k |
| encode/bind_walk/init_textures | count | 7190 | 12.1 k | 9.30 M |
| encode/bind_walk/init_textures_skipped | count | 41.6 k | 48.8 k | 53.78 M |
| encode/bind_walk/inits | count | 504 | 608 | 651.6 k |
| encode/bind_walk/inits_skipped | count | 127 | 137 | 164.5 k |
| encode/bind_walk/merge_textures | count | 7125 | 12.0 k | 9.21 M |
| encode/bind_walk/merge_textures_skipped | count | 6439 | 12.2 k | 8.33 M |
| encode/bind_walk/merges | count | 91.27 | 111 | 118.0 k |
| encode/bind_walk/merges_skipped | count | 112 | 128 | 144.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1293 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1293 |
| encode/pass:early prepass/count | count | 1 | 1 | 1293 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1293 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1293 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1293 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1293 |
| encode/queue/command_buffers | count | 38.55 | 41 | 49.9 k |
| encode/queue/submits | count | 2 | 2 | 2586 |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 3.36 M | 2.43 G |
| encode/queue/write_buffer/calls | count | 234 | 305 | 302.2 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 42.20 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.4 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1293 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 21 |
| pipelines/created | count | 0.02 | 1 | 20 |
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
| surface/shelter_far_redraws | count | 0.00 | 1 | 1 |
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.2 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2583.0 | main/busy/ms 2550, stage/main/post_update 2438, stage/main/update 98.75, surface/advance_cpu_ms 68.84, stage/main/pre_update 9.61, stage/main/last 1.16, surface/cell_upload_ms 0.80, stream/drive/ms 0.47, stage/main/first 0.46, stage/main/fixed_main_loop 0.38, weather/map/ms 0.18, stream/tick/ms 0.14 |
| warm-up | 3.38 | 794.4 | main/busy/ms 2598, stage/main/post_update 2473, render/thread/ms 731, render/busy/ms 589, stage/render/prepare_assets 528, stage/render/prepare_views 138, render/acquire/ms 138, stage/main/update 106, surface/advance_cpu_ms 68.84, stage/render/create_views 29.16, stage/render/render 21.59, render/graph/ms 16.15 |
| warm-up | 3.49 | 112.4 | render/thread/ms 840, render/busy/ms 631, stage/render/prepare_assets 535, stage/render/prepare_views 139, render/acquire/ms 138, stage/render/render 112, main/busy/ms 90.96, render/present/ms 71.27, stage/main/post_update 64.37, render/graph/ms 38.90, stage/render/create_views 29.47, terrain/update/ms 27.65 |
| warm-up | 3.53 | 43.4 | pipelines/compile_ms 337, pipelines/front_ms 233, render/thread/ms 151, pipelines/driver_ms 104, stage/render/render 95.85, render/busy/ms 82.04, main/busy/ms 69.35, render/present/ms 68.42, stage/main/post_update 44.88, stage/render/prepare_assets 34.73, terrain/update/ms 33.51, render/graph/ms 25.72 |
| warm-up | 3.63 | 101.6 | pipelines/wait_ms 1838, pipelines/compile_ms 931, pipelines/front_ms 789, pipelines/driver_ms 142, render/thread/ms 136, render/busy/ms 133, main/busy/ms 123, stage/main/update 69.02, stage/render/prepare_assets 48.66, stage/render/prepare_resources 41.58, stage/main/post_update 28.73, stage/render/prepare_write_phase_buffers 21.67 |
| warm-up | 3.77 | 139.6 | pipelines/wait_ms 6329, pipelines/compile_ms 2157, pipelines/front_ms 2027, render/thread/ms 222, render/busy/ms 220, main/busy/ms 189, pipelines/driver_ms 130, stage/main/post_update 95.33, stage/render/prepare_assets 88.66, stage/main/update 71.21, stage/render/prepare_resources 58.27, stage/render/prepare_write_phase_buffers 35.73 |
| warm-up | 3.81 | 34.9 | pipelines/wait_ms 4793, pipelines/compile_ms 2447, pipelines/front_ms 2324, render/thread/ms 161, render/busy/ms 159, pipelines/driver_ms 123, main/busy/ms 119, stage/main/post_update 94.38, stage/render/prepare_assets 76.90, stage/render/prepare_resources 24.64, encode//sum 21.26, terrain/update/ms 17.63 |
| warm-up | 3.86 | 48.5 | pipelines/compile_ms 1689, pipelines/front_ms 1613, pipelines/wait_ms 372, render/thread/ms 78.49, render/busy/ms 76.36, pipelines/driver_ms 76.08, main/busy/ms 49.07, stage/render/prepare_assets 37.45, stage/main/post_update 24.68, encode//sum 22.46, stage/render/render 19.37, render/graph/ms 16.93 |
| warm-up | 3.98 | 48.3 | pipelines/compile_ms 1005, pipelines/front_ms 944, pipelines/wait_ms 183, render/thread/ms 72.05, render/busy/ms 68.59, pipelines/driver_ms 61.18, main/busy/ms 40.95, stage/render/prepare_assets 34.93, stage/render/render 19.73, stage/main/post_update 17.87, render/graph/ms 15.86, stage/main/update 14.02 |
| warm-up | 4.02 | 44.9 | pipelines/compile_ms 1249, pipelines/front_ms 1185, pipelines/wait_ms 119, render/thread/ms 87.22, render/busy/ms 84.38, pipelines/driver_ms 63.52, stage/render/prepare_assets 51.74, main/busy/ms 34.77, stage/render/render 24.63, render/graph/ms 21.10, stage/main/update 16.66, stage/render/graph_submit 12.88 |
| warm-up | 4.07 | 44.4 | pipelines/compile_ms 834, pipelines/front_ms 808, render/thread/ms 83.58, render/busy/ms 81.00, stage/render/prepare_assets 52.02, main/busy/ms 33.56, pipelines/wait_ms 32.20, pipelines/driver_ms 26.09, stage/render/render 21.59, render/graph/ms 18.50, stage/main/update 10.92, stage/render/graph_submit 10.24 |
| warm-up | 4.11 | 46.5 | pipelines/compile_ms 866, pipelines/front_ms 835, render/thread/ms 87.19, render/busy/ms 85.15, stage/render/prepare_assets 63.46, main/busy/ms 40.61, pipelines/driver_ms 31.23, stage/main/update 18.30, stage/render/render 15.57, render/graph/ms 13.30, stage/main/post_update 10.10, stage/render/graph_submit 9.62 |
| warm-up | 4.15 | 34.6 | pipelines/compile_ms 507, pipelines/front_ms 488, render/thread/ms 77.75, render/busy/ms 76.34, stage/render/prepare_assets 62.06, main/busy/ms 31.90, pipelines/driver_ms 18.83, stage/main/update 18.29, stage/render/render 10.25, render/graph/ms 8.60, stage/main/post_update 8.15, encode//sum 7.06 |
| warm-up | 4.30 | 33.6 | pipelines/compile_ms 844, pipelines/front_ms 820, render/thread/ms 50.71, render/busy/ms 49.14, main/busy/ms 41.87, stage/render/prepare_assets 31.52, pipelines/driver_ms 24.65, stage/main/update 23.36, stage/render/render 11.77, stage/main/post_update 11.08, render/graph/ms 9.30, encode//sum 8.46 |
| warm-up | 4.34 | 41.8 | pipelines/compile_ms 864, pipelines/front_ms 839, main/busy/ms 87.44, render/thread/ms 70.56, render/busy/ms 68.54, stage/main/update 63.34, stage/render/prepare_assets 34.17, pipelines/driver_ms 25.43, stage/render/render 23.66, render/graph/ms 21.03, stage/main/post_update 13.59, encode//sum 11.30 |
| warm-up | 4.45 | 104.7 | pipelines/compile_ms 159, pipelines/front_ms 153, render/thread/ms 120, render/busy/ms 118, main/busy/ms 79.15, stage/main/update 58.01, stage/render/prepare_assets 47.47, stage/render/prepare_flush 33.65, stage/render/render 22.46, render/graph/ms 20.17, pipelines/wait_ms 17, stage/main/post_update 11.67 |
| warm-up | 4.53 | 79.1 | pipelines/compile_ms 1731, pipelines/front_ms 1662, pipelines/wait_ms 213, render/thread/ms 159, render/busy/ms 157, pipelines/driver_ms 69.34, stage/render/prepare_assets 54.35, stage/render/prepare_flush 34.73, main/busy/ms 34.02, stage/render/prepare_batch_phases 28.27, stage/render/render 18.35, stage/main/update 17.18 |
| warm-up | 4.61 | 41.0 | main/busy/ms 60.02, render/thread/ms 55.43, render/busy/ms 46.58, pipelines/compile_ms 36.67, pipelines/front_ms 35.28, stage/main/update 34.65, stage/main/post_update 16.42, stage/render/render 12.59, render/graph/ms 10.70, stage/render/prepare_flush 10.64, stage/render/prepare_assets 10.51, encode//sum 9.85 |
| warm-up | 4.68 | 66.9 | pipelines/compile_ms 862, pipelines/front_ms 836, main/busy/ms 101, render/thread/ms 95.88, render/busy/ms 84.06, stage/main/update 65.33, stage/render/prepare_assets 26.71, pipelines/driver_ms 26.25, stage/main/post_update 21.27, stage/render/render 19.83, render/graph/ms 14.88, encode//sum 11.24 |
| warm-up | 4.73 | 54.2 | pipelines/compile_ms 1480, pipelines/front_ms 1431, main/busy/ms 111, render/thread/ms 107, render/busy/ms 102, pipelines/wait_ms 94.20, stage/main/update 72.36, stage/render/prepare_assets 50.18, pipelines/driver_ms 48.82, stage/render/render 21.13, stage/main/post_update 20.75, render/graph/ms 15.69 |
| warm-up | 4.84 | 101.4 | pipelines/compile_ms 1567, pipelines/front_ms 1517, render/thread/ms 142, render/busy/ms 141, main/busy/ms 97.88, pipelines/wait_ms 94.52, stage/render/prepare_assets 77.44, stage/main/update 56.25, pipelines/driver_ms 49.15, stage/render/prepare_bind_groups 33.94, stage/main/post_update 16.41, stage/render/render 16.20 |
| warm-up | 4.87 | 36.5 | pipelines/compile_ms 1178, pipelines/front_ms 1140, render/thread/ms 127, render/busy/ms 125, main/busy/ms 69.43, stage/render/prepare_assets 55.65, pipelines/driver_ms 37.54, stage/main/update 37.16, stage/render/prepare_bind_groups 33.32, stage/render/render 18.24, render/graph/ms 16.07, stage/render/graph_submit 13.02 |
| warm-up | 4.91 | 39.5 | pipelines/compile_ms 496, pipelines/front_ms 475, render/thread/ms 63.31, render/busy/ms 61.78, main/busy/ms 45.72, stage/main/update 25.94, pipelines/wait_ms 24.65, stage/render/prepare_assets 21.31, pipelines/driver_ms 21.05, stage/render/render 17.40, render/graph/ms 15.20, stage/main/post_update 12.63 |
| warm-up | 5.00 | 30.4 | pipelines/compile_ms 82.85, pipelines/front_ms 78.80, render/thread/ms 53.93, render/busy/ms 52.55, main/busy/ms 41.34, stage/main/update 23.92, stage/render/prepare_assets 18.23, stage/render/render 12.13, stage/main/post_update 11.03, render/graph/ms 10.33, stage/render/prepare_write_phase_buffers 7.98, encode//sum 7.44 |
| warm-up | 5.19 | 35.3 | pipelines/compile_ms 256, pipelines/front_ms 246, render/thread/ms 54.90, render/busy/ms 52.22, main/busy/ms 44.92, stage/main/post_update 23.51, stage/render/render 19.21, render/graph/ms 16.09, stage/render/prepare_resources 15.26, stage/main/update 12.88, pipelines/driver_ms 10.20, stage/render/graph_submit 9.87 |
| warm-up | 5.23 | 37.8 | pipelines/compile_ms 448, pipelines/front_ms 437, render/thread/ms 69.11, render/busy/ms 67.23, main/busy/ms 39.06, stage/render/prepare_resources 23.70, stage/render/prepare_assets 18.15, stage/main/post_update 14.36, stage/main/update 13.84, stage/render/render 11.78, pipelines/driver_ms 10.81, render/graph/ms 9.42 |
| warm-up | 5.29 | 59.5 | pipelines/compile_ms 806, pipelines/front_ms 784, render/thread/ms 94.16, render/busy/ms 92.17, main/busy/ms 70.24, stage/main/update 43.36, stage/render/prepare_assets 27.32, pipelines/driver_ms 22.00, stage/render/prepare_flush 16.31, stage/main/post_update 13.51, stage/render/prepare_write_phase_buffers 13.02, stage/render/render 12.66 |
| warm-up | 5.33 | 34.7 | pipelines/compile_ms 860, pipelines/front_ms 836, render/thread/ms 90.53, render/busy/ms 89.11, main/busy/ms 71.19, stage/main/update 45.22, stage/render/prepare_assets 31.74, pipelines/driver_ms 24.05, stage/render/prepare_flush 15.69, stage/main/post_update 13.38, stage/render/render 12.72, stage/render/prepare_write_phase_buffers 12.00 |
| warm-up | 5.54 | 31.3 | render/thread/ms 46.96, render/busy/ms 45.48, main/busy/ms 36.95, stage/render/prepare_assets 19.91, stage/main/update 16.80, stage/render/render 15.02, render/graph/ms 12.85, stage/main/post_update 12.36, encode//sum 9.42, stage/render/graph_submit 9.13, encode/submit 6.64, stage/render/camera_driver 3.49 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=off-3 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (300 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 93.5 W · 79 °C (max 81) · VRAM 5144 MiB · throttled 100% of samples

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
