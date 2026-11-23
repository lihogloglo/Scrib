# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1292 frames over 15.0 s · **median 11.69 ms (86 fps)** · mean 11.62 · p95 13.12 · p99 13.68 (1% low 73 fps) · worst 19.76 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 312 frames over 8.5 s, worst 2531.1 ms
- hitches (> 29.2 ms): **0** measured, 21 in warm-up
- timed GPU passes: 10.19 ms of a 11.62 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.69 ms, limit 11 ms
- pass worst frame 19.76 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.229 | 3.237 | 3.316 | 1.00 | 0.082 |
| dlss_super_resolution | 2.018 | 2.023 | 2.046 | 1.00 | 0.184 |
| sun_visibility | 0.778 | 0.780 | 0.807 | 1.00 | 0.007 |
| early prepass | 0.647 | 0.648 | 0.658 | 1.00 | 0.099 |
| main_transmissive_pass_3d | 0.399 | 0.400 | 0.411 | 1.00 | 0.008 |
| atmosphere_luts | 0.365 | 0.366 | 0.374 | 1.00 | 0.003 |
| bloom | 0.314 | 0.315 | 0.331 | 1.00 | 0.060 |
| atmos_froxel_inject | 0.310 | 0.311 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.305 | 0.306 | 0.310 | 1.00 | 0.002 |
| water_sspr | 0.215 | 0.216 | 0.213 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.196 | 1.00 | 0.008 |
| clouds_trace | 0.151 | 0.151 | 0.170 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.135 | 0.136 | 1.00 | 0.003 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| uniform_allocation | 0.115 | 0.116 | 0.146 | 1.00 | 0.023 |
| upscaling | 0.111 | 0.111 | 0.116 | 1.00 | 0.004 |
| bin_unpacking | 0.090 | 0.090 | 0.114 | 1.00 | 0.016 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.074 | 0.074 | 0.096 | 1.00 | 0.004 |
| clustering | 0.064 | 0.064 | 0.075 | 1.00 | 0.021 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.060 | 0.060 | 0.085 | 1.00 | 0.005 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.414 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.232 | 0.12 | 0.004 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.029 | 1.00 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.019 | 0.019 | 0.035 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.026 | 1.00 | 0.197 |
| surface_exposure_far | 0.012 | 0.041 | 0.044 | 0.30 | 0.002 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| surface_exposure | 0.011 | 0.040 | 0.041 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.004 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.476 | 12.016 | 19.39 | 13535.6 |
| render/busy/ms | 7.432 | 8.619 | 16.53 | 9602.2 |
| main/busy/ms | 6.728 | 8.036 | 14.11 | 8692.4 |
| stage/render/render | 5.034 | 5.880 | 11.22 | 6503.7 |
| encode//sum | 5.033 | 6.448 | 11.53 | 6503.0 |
| pipelines/compile_ms | 4.471 | 12.084 | 140.49 | 5776.5 |
| render/graph/ms | 4.437 | 5.233 | 9.67 | 5733.2 |
| pipelines/front_ms | 4.345 | 11.782 | 137.19 | 5614.2 |
| stage/main/post_update | 3.788 | 4.766 | 6.88 | 4894.7 |
| stage/render/graph_submit | 3.228 | 3.956 | 8.56 | 4171.0 |
| stage/render/prepare_views | 2.726 | 4.441 | 5.57 | 3522.5 |
| render/acquire/ms | 2.679 | 4.386 | 5.53 | 3461.7 |
| encode/submit | 1.972 | 2.349 | 3.89 | 2547.8 |
| stage/main/update | 1.385 | 1.875 | 2.41 | 1789.2 |
| encode/finish_wall | 1.286 | 1.769 | 5.76 | 1661.4 |
| stage/render/camera_driver | 1.140 | 1.317 | 2.36 | 1472.7 |
| encode/pass:shadow_directional_light/sum | 0.815 | 1.402 | 2.22 | 1052.7 |
| encode/pass:early prepass/sum | 0.801 | 1.083 | 5.07 | 1034.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.776 | 1.081 | 1.45 | 1002.8 |
| stage/render/prepare_write_phase_buffers | 0.506 | 0.756 | 1.22 | 654.1 |
| stage/render/prepare_resources | 0.425 | 0.502 | 0.85 | 549.0 |
| stage/main/fixed_main_loop | 0.409 | 0.626 | 1.44 | 529.0 |
| stage/render/prepare_assets | 0.406 | 0.497 | 1.78 | 524.2 |
| stage/main/pre_update | 0.405 | 0.466 | 0.89 | 523.9 |
| bigspace/total/ms | 0.367 | 0.665 | 1.84 | 474.6 |
| render/present/ms | 0.365 | 0.431 | 2.77 | 471.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.352 | 0.498 | 0.89 | 454.2 |
| stage/main/first | 0.310 | 0.371 | 1.55 | 401.0 |
| stage/render/prepare_flush | 0.272 | 0.381 | 0.84 | 351.2 |
| stage/render/prepare_batch_phases | 0.270 | 0.375 | 0.58 | 348.3 |
| stage/render/prepare_bind_groups | 0.264 | 0.375 | 0.58 | 341.0 |
| stage/main/last | 0.252 | 0.348 | 8.20 | 326.2 |
| stage/render/prepare_meshes | 0.211 | 0.261 | 1.01 | 272.0 |
| controller/feed/ms | 0.192 | 0.251 | 0.63 | 247.6 |
| stage/render/extract_commands | 0.187 | 0.219 | 0.42 | 241.4 |
| stage/render/create_views | 0.159 | 0.190 | 0.51 | 205.1 |
| stream/drive/ms | 0.143 | 0.176 | 0.58 | 185.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.139 | 0.205 | 0.57 | 179.7 |
| bigspace/high_precision/ms | 0.127 | 0.311 | 1.21 | 164.1 |
| pipelines/driver_ms | 0.126 | 0.670 | 4.67 | 162.3 |
| stream/tick/ms | 0.115 | 0.143 | 0.56 | 148.1 |
| stage/render/specialize | 0.105 | 0.188 | 3.24 | 135.7 |
| bigspace/low_precision/ms | 0.101 | 0.276 | 1.50 | 131.0 |
| stage/render/queue | 0.101 | 0.136 | 2.70 | 130.8 |
| anim/evaluate/ms | 0.079 | 0.165 | 1.00 | 102.0 |
| controller/move/ms | 0.064 | 0.143 | 0.22 | 82.8 |
| anim/evaluate_cpu/ms | 0.062 | 0.075 | 0.46 | 80.1 |
| encode/resolve_diagnostics/sum | 0.060 | 0.089 | 0.39 | 77.6 |
| controller/write/ms | 0.054 | 0.068 | 0.21 | 69.3 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.082 | 0.20 | 62.4 |
| terrain/update/ms | 0.036 | 0.048 | 0.40 | 46.0 |
| physics/step/ms | 0.028 | 0.047 | 0.09 | 36.1 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.041 | 0.39 | 35.3 |
| stage/render/cleanup | 0.025 | 0.035 | 0.28 | 32.4 |
| weather/map/ms | 0.024 | 0.033 | 0.48 | 30.9 |
| stage/render/phase_sort | 0.024 | 0.043 | 0.14 | 30.4 |
| encode/camera_driver_span_end/sum | 0.023 | 0.036 | 0.36 | 30.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 13.9 |
| script/update/ms | 0.007 | 0.008 | 0.03 | 8.6 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 6.8 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 5.9 |
| far/select/ms | 0.004 | 0.005 | 0.03 | 5.1 |
| controller/avoid/ms | 0.004 | 0.006 | 0.03 | 5.0 |
| script/ui/ms | 0.004 | 0.005 | 0.40 | 4.9 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.01 | 2.5 |
| character/finish/ms | 0.001 | 0.002 | 0.02 | 1.8 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.02 | 1.7 |
| stream/promote/ms | 0.001 | 0.002 | 0.02 | 1.5 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.39 | 1.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.05 | 1.2 |
| pipelines/wait_ms | 0.001 | 0.013 | 0.04 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| terrain/select/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| crowd/joints_extracted/count | count | 134 | 181 | 115 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.61 | 5.46 | 0.65 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.39 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.36 | 0.02 |
| encode/finish_critical | ms | 0.94 | 5.46 | 0.81 |
| encode/pass:early prepass/max | ms | 0.79 | 5.07 | 0.67 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.45 | 0.81 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.13 | 0.57 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.35 | 0.68 | 0.37 |
| encode/pass:shadow_directional_light/max | ms | 0.75 | 2.22 | 0.33 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.39 | 0.05 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.82 | 16.53 | 10.71 |
| gpu/timed/ms | ms | 10.82 | 12.27 | 10.71 |
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
| phases/opaque3d/bins | count | 875 | 875 | 875 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 19 | 19 |
| phases/opaque3d_prepass/bins | count | 875 | 875 | 875 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 4 | 41 | 4 |
| phases/shadow/bins | count | 896 | 1509 | 896 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 5861 | 17.6 k | 5861 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
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
| precache/throttled | count | 0 | 40 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 24.94 | 40.19 | 23.08 |
| process/mem_usage | GiB | 2.99 | 3.04 | 3.04 |
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
| skin/blocks_changed/count | count | 5 | 7 | 5 |
| skin/blocks_skipped/count | count | 176 | 180 | 176 |
| skin/blocks_walked/count | count | 5 | 7 | 5 |
| skin/joint_bytes_written/bytes | bytes | 10.2 k | 12.5 k | 10.5 k |
| skin/joints_written/count | count | 160 | 196 | 164 |
| skin/upload_writes/count | count | 4 | 6 | 4 |
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
| system/cpu_usage | % | 31.01 | 54.73 | 26.03 |
| system/mem_usage | % | 55.61 | 55.75 | 55.74 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2545 | 2545 | 2545 |
| textures/images | count | 1998 | 1998 | 1998 |
| views/all/batch_sets | count | 64 | 101 | 64 |
| views/all/bins | count | 3142 | 3755 | 3142 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 17.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 9408 | 14.4 k | 9408 |
| views/all/pipelines | count | 75 | 82 | 75 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 153.9 k | 260.4 k | 153.9 k |
| views/cascade/batch_sets | count | 4 | 41 | 4 |
| views/cascade/bins | count | 896 | 1509 | 896 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 5861 | 17.6 k | 5861 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 3202 | 8161 | 3202 |
| views/cascade/pipelines | count | 27 | 34 | 27 |
| views/cascade/work_items | count | 49.2 k | 155.6 k | 49.2 k |
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
| views/main/bins | count | 2246 | 2246 | 2246 |
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
| anim/events/count | count | 0.11 | 2 | 146 |
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
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 28.55 | 56 | 36.9 k |
| encode/bind_walk/init_textures | count | 7188 | 16.3 k | 9.29 M |
| encode/bind_walk/init_textures_skipped | count | 41.1 k | 86.1 k | 53.11 M |
| encode/bind_walk/inits | count | 504 | 1047 | 651.0 k |
| encode/bind_walk/inits_skipped | count | 127 | 259 | 163.8 k |
| encode/bind_walk/merge_textures | count | 7123 | 16.2 k | 9.20 M |
| encode/bind_walk/merge_textures_skipped | count | 6435 | 16.7 k | 8.31 M |
| encode/bind_walk/merges | count | 91.26 | 191 | 117.9 k |
| encode/bind_walk/merges_skipped | count | 112 | 233 | 144.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1292 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1292 |
| encode/pass:early prepass/count | count | 1 | 2 | 1292 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1292 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 1292 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 1292 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 1292 |
| encode/queue/command_buffers | count | 38.55 | 76 | 49.8 k |
| encode/queue/submits | count | 2 | 4 | 2584 |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 4.01 M | 2.42 G |
| encode/queue/write_buffer/calls | count | 233 | 502 | 301.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 65.3 k | 42.17 M |
| encode/queue/write_buffer_with/calls | count | 9 | 18 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.4 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1292 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.06 | 1 | 73 |
| pipelines/created | count | 0.06 | 1 | 83 |
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

## Hitches (> 29.2 ms; slowest 21, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2531.1 | main/busy/ms 2500, stage/main/post_update 2387, stage/main/update 99.72, surface/advance_cpu_ms 69.23, stage/main/pre_update 9.29, stage/main/last 1.20, surface/cell_upload_ms 0.74, stream/drive/ms 0.67, stage/main/first 0.54, stage/main/fixed_main_loop 0.39, weather/map/ms 0.14, stream/tick/ms 0.13 |
| warm-up | 3.36 | 827.1 | main/busy/ms 2542, stage/main/post_update 2416, render/thread/ms 763, render/busy/ms 643, stage/render/prepare_assets 590, stage/render/prepare_views 117, render/acquire/ms 117, stage/main/update 106, surface/advance_cpu_ms 69.23, stage/render/create_views 29.85, stage/render/render 13.57, stage/main/pre_update 11.12 |
| warm-up | 3.44 | 78.1 | render/thread/ms 839, render/busy/ms 661, stage/render/prepare_assets 594, stage/render/prepare_views 118, render/acquire/ms 117, main/busy/ms 116, stage/render/render 78.59, render/present/ms 59.80, stage/main/post_update 53.87, pipelines/compile_ms 46.82, pipelines/front_ms 43.29, stage/main/last 39.86 |
| warm-up | 3.49 | 49.0 | pipelines/compile_ms 191, pipelines/front_ms 137, render/thread/ms 116, main/busy/ms 96.71, stage/render/render 71.87, render/present/ms 58.20, render/busy/ms 57.42, pipelines/driver_ms 54.04, stage/main/last 40.61, stage/main/post_update 39.68, terrain/update/ms 27.06, stage/render/prepare_assets 26.30 |
| warm-up | 3.67 | 185.9 | pipelines/wait_ms 6146, pipelines/compile_ms 1885, pipelines/front_ms 1714, render/thread/ms 204, render/busy/ms 201, pipelines/driver_ms 170, main/busy/ms 161, stage/main/update 96.78, stage/render/prepare_write_phase_buffers 54.17, stage/render/prepare_assets 46.90, stage/render/prepare_resources 43.86, stage/main/post_update 30.47 |
| warm-up | 3.74 | 72.1 | pipelines/wait_ms 6315, pipelines/compile_ms 2994, pipelines/front_ms 2810, render/thread/ms 231, render/busy/ms 228, pipelines/driver_ms 184, main/busy/ms 172, stage/main/update 103, stage/render/prepare_assets 79.67, stage/render/prepare_write_phase_buffers 53.48, stage/render/prepare_resources 42.44, stage/main/post_update 27.97 |
| warm-up | 3.78 | 35.6 | pipelines/compile_ms 1347, pipelines/front_ms 1278, pipelines/wait_ms 185, render/thread/ms 97.18, render/busy/ms 95.18, stage/render/prepare_assets 74.43, pipelines/driver_ms 69.65, main/busy/ms 48.13, stage/main/post_update 17.70, terrain/update/ms 15.20, stage/main/update 14.34, stage/render/render 12.52 |
| warm-up | 3.88 | 48.0 | pipelines/compile_ms 671, pipelines/front_ms 622, render/thread/ms 67.81, render/busy/ms 65.69, pipelines/driver_ms 49.12, main/busy/ms 47.04, stage/render/render 29.08, stage/main/post_update 27.70, render/graph/ms 26.70, stage/render/prepare_resources 18.40, stage/render/camera_driver 17.43, encode//sum 12.30 |
| warm-up | 3.93 | 58.0 | pipelines/compile_ms 1238, pipelines/front_ms 1155, render/thread/ms 102, render/busy/ms 99.78, pipelines/driver_ms 82.96, pipelines/wait_ms 60.86, main/busy/ms 52.84, stage/render/prepare_assets 42.89, stage/render/render 35.24, render/graph/ms 32.10, stage/main/post_update 28.52, stage/render/camera_driver 17.93 |
| warm-up | 3.96 | 30.2 | pipelines/compile_ms 1055, pipelines/front_ms 1004, render/thread/ms 81.64, render/busy/ms 78.96, pipelines/wait_ms 61.77, pipelines/driver_ms 51.79, stage/render/prepare_assets 47.98, main/busy/ms 34.35, stage/render/render 21.91, render/graph/ms 18.52, stage/main/update 15.26, stage/render/graph_submit 14.77 |
| warm-up | 4.03 | 60.6 | pipelines/compile_ms 1159, pipelines/front_ms 1113, render/thread/ms 80.10, render/busy/ms 77.32, pipelines/driver_ms 45.71, stage/render/prepare_assets 43.96, main/busy/ms 26.45, stage/render/render 26.15, render/graph/ms 23.08, stage/render/graph_submit 13.92, pipelines/wait_ms 9.94, stage/main/post_update 9.52 |
| warm-up | 4.07 | 44.0 | pipelines/compile_ms 1060, pipelines/front_ms 1014, render/thread/ms 89.56, render/busy/ms 87.47, stage/render/prepare_assets 59.39, pipelines/driver_ms 45.52, main/busy/ms 25.50, stage/render/render 22.96, pipelines/wait_ms 21.23, render/graph/ms 20.51, pipelines/lock_ms 20.28, stage/render/graph_submit 11.58 |
| warm-up | 4.11 | 43.0 | pipelines/compile_ms 360, pipelines/front_ms 346, render/thread/ms 77.11, render/busy/ms 75.85, stage/render/prepare_assets 59.42, main/busy/ms 23.21, pipelines/wait_ms 13.99, pipelines/lock_ms 13.62, pipelines/driver_ms 13.41, stage/render/render 12.13, render/graph/ms 10.42, stage/main/update 9.99 |
| warm-up | 4.34 | 37.9 | render/thread/ms 53.35, render/busy/ms 51.26, main/busy/ms 36.23, stage/render/prepare_assets 30.18, pipelines/compile_ms 21.55, pipelines/front_ms 20.86, stage/main/update 17.79, stage/render/render 15.92, render/graph/ms 13.09, stage/render/graph_submit 9.35, encode//sum 9.00, stage/main/post_update 8.79 |
| warm-up | 4.41 | 56.1 | main/busy/ms 78.28, render/thread/ms 68.07, render/busy/ms 66.27, stage/main/update 56.58, stage/render/prepare_assets 34.56, stage/render/render 19.69, render/graph/ms 17.12, stage/render/graph_submit 11.74, stage/main/post_update 10.56, encode//sum 9.67, character/finish/ms 8.40, encode/submit 7.88 |
| warm-up | 4.52 | 103.2 | render/thread/ms 146, render/busy/ms 144, main/busy/ms 123, stage/render/prepare_assets 101, stage/main/update 97.17, pipelines/compile_ms 90.75, pipelines/front_ms 85.32, pipelines/wait_ms 76.38, stage/render/render 22.29, render/graph/ms 19.69, character/finish/ms 18.41, stage/render/graph_submit 15.65 |
| warm-up | 4.56 | 43.9 | pipelines/compile_ms 310, pipelines/front_ms 299, pipelines/wait_ms 201, render/thread/ms 138, render/busy/ms 136, stage/render/prepare_assets 98.54, main/busy/ms 87.70, stage/main/update 60.84, stage/render/render 16.04, stage/main/post_update 15.88, render/graph/ms 13.96, stage/render/graph_submit 11.53 |
| warm-up | 4.59 | 31.9 | pipelines/compile_ms 403, pipelines/front_ms 392, pipelines/wait_ms 128, render/thread/ms 72.36, render/busy/ms 70.98, stage/render/prepare_assets 42.11, main/busy/ms 37.21, stage/main/update 18.44, stage/render/render 11.63, pipelines/driver_ms 11.21, stage/main/post_update 11.05, render/graph/ms 9.88 |
| warm-up | 5.07 | 41.9 | pipelines/wait_ms 342, pipelines/compile_ms 172, pipelines/front_ms 165, render/thread/ms 61.42, render/busy/ms 59.03, main/busy/ms 32.35, stage/render/prepare_assets 21.53, stage/render/render 21.47, render/graph/ms 18.14, stage/main/update 16.05, stage/render/graph_submit 13.39, encode//sum 12.82 |
| warm-up | 5.11 | 36.2 | pipelines/wait_ms 1397, pipelines/compile_ms 205, pipelines/front_ms 197, render/thread/ms 71.19, render/busy/ms 69.41, stage/render/prepare_assets 39.57, main/busy/ms 34.25, stage/main/update 15.75, stage/render/render 15.38, render/graph/ms 12.78, encode//sum 11.65, stage/main/post_update 10.94 |
| warm-up | 5.96 | 33.0 | render/thread/ms 43.64, render/busy/ms 42.23, main/busy/ms 40.18, stage/main/update 16.89, stage/render/render 16.71, stage/main/post_update 15.92, render/graph/ms 14.26, stage/render/prepare_assets 11.42, encode//sum 11.18, stage/render/graph_submit 9.95, encode/submit 5.91, stage/render/prepare_meshes 5.83 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0\balmora@2560x1440_arm=off-2 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 93.1 W · 77 °C (max 80) · VRAM 5429 MiB · throttled 100% of samples

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
