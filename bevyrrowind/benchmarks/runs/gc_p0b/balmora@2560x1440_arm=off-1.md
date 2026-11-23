# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1285 frames over 15.0 s · **median 11.69 ms (86 fps)** · mean 11.68 · p95 13.05 · p99 13.64 (1% low 73 fps) · worst 76.30 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 8.0/min)
- warm-up: 315 frames over 8.5 s, worst 2534.5 ms
- hitches (> 29.2 ms): **2** measured, 28 in warm-up
- timed GPU passes: 10.20 ms of a 11.68 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.69 ms, limit 11 ms
- **FAIL** worst frame 76.30 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.240 | 3.247 | 3.349 | 1.00 | 0.066 |
| dlss_super_resolution | 2.018 | 2.023 | 2.046 | 1.00 | 0.182 |
| sun_visibility | 0.779 | 0.781 | 0.808 | 1.00 | 0.006 |
| early prepass | 0.648 | 0.650 | 0.657 | 1.00 | 0.094 |
| main_transmissive_pass_3d | 0.401 | 0.401 | 0.412 | 1.00 | 0.008 |
| atmosphere_luts | 0.372 | 0.373 | 0.380 | 1.00 | 0.003 |
| bloom | 0.313 | 0.314 | 0.331 | 1.00 | 0.059 |
| atmos_froxel_inject | 0.307 | 0.308 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.305 | 0.306 | 0.310 | 1.00 | 0.001 |
| water_sspr | 0.213 | 0.213 | 0.210 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.196 | 1.00 | 0.007 |
| clouds_trace | 0.151 | 0.151 | 0.170 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.134 | 0.136 | 1.00 | 0.002 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| upscaling | 0.114 | 0.114 | 0.121 | 1.00 | 0.004 |
| uniform_allocation | 0.112 | 0.113 | 0.144 | 1.00 | 0.018 |
| bin_unpacking | 0.088 | 0.088 | 0.112 | 1.00 | 0.013 |
| tonemapping | 0.085 | 0.086 | 0.089 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.073 | 0.073 | 0.094 | 1.00 | 0.003 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.057 | 0.057 | 0.083 | 1.00 | 0.003 |
| lightprobe_irradiance_map | 0.051 | 0.409 | 0.411 | 0.13 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.231 | 0.13 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.029 | 1.00 | 0.003 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.037 | 1.00 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.025 | 1.00 | 0.187 |
| surface_exposure_far | 0.012 | 0.039 | 0.041 | 0.30 | 0.002 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| surface_exposure | 0.011 | 0.039 | 0.040 | 0.28 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.13 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.538 | 11.913 | 75.22 | 13541.9 |
| render/busy/ms | 7.085 | 8.142 | 72.58 | 9104.5 |
| main/busy/ms | 6.563 | 7.898 | 9.38 | 8433.7 |
| stage/render/render | 4.768 | 5.536 | 70.44 | 6126.3 |
| encode//sum | 4.701 | 5.953 | 69.59 | 6041.2 |
| render/graph/ms | 4.180 | 4.888 | 69.78 | 5371.0 |
| pipelines/compile_ms | 3.967 | 9.924 | 121.07 | 5098.1 |
| pipelines/front_ms | 3.855 | 9.261 | 118.08 | 4953.7 |
| stage/main/post_update | 3.667 | 4.640 | 6.50 | 4712.4 |
| stage/render/prepare_views | 3.146 | 4.714 | 5.66 | 4042.6 |
| render/acquire/ms | 3.096 | 4.654 | 5.56 | 3978.5 |
| stage/render/graph_submit | 3.029 | 3.690 | 68.67 | 3891.7 |
| encode/submit | 1.822 | 2.230 | 4.93 | 2340.8 |
| stage/main/update | 1.335 | 1.722 | 2.24 | 1715.8 |
| encode/finish_wall | 1.223 | 1.607 | 65.42 | 1572.2 |
| stage/render/camera_driver | 1.080 | 1.257 | 1.59 | 1387.5 |
| encode/pass:early prepass/sum | 0.794 | 1.012 | 64.83 | 1020.9 |
| encode/pass:shadow_directional_light/sum | 0.734 | 1.300 | 1.86 | 943.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.683 | 0.982 | 1.36 | 877.7 |
| stage/render/prepare_write_phase_buffers | 0.473 | 0.696 | 0.99 | 607.9 |
| stage/render/prepare_resources | 0.426 | 0.500 | 0.78 | 547.2 |
| stage/main/fixed_main_loop | 0.425 | 0.650 | 1.87 | 545.8 |
| stage/main/pre_update | 0.395 | 0.463 | 0.68 | 507.5 |
| stage/render/prepare_assets | 0.390 | 0.469 | 1.50 | 501.7 |
| render/present/ms | 0.357 | 0.418 | 2.50 | 458.8 |
| bigspace/total/ms | 0.342 | 0.633 | 1.47 | 439.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.327 | 0.464 | 0.68 | 420.5 |
| stage/main/first | 0.309 | 0.370 | 1.66 | 397.2 |
| stage/render/prepare_flush | 0.265 | 0.373 | 0.69 | 340.7 |
| stage/render/prepare_batch_phases | 0.261 | 0.362 | 0.53 | 334.8 |
| stage/main/last | 0.258 | 0.368 | 0.91 | 332.1 |
| stage/render/prepare_bind_groups | 0.257 | 0.356 | 0.51 | 329.8 |
| stage/render/prepare_meshes | 0.205 | 0.252 | 1.21 | 262.9 |
| controller/feed/ms | 0.186 | 0.237 | 0.67 | 239.5 |
| stage/render/extract_commands | 0.177 | 0.210 | 0.34 | 227.8 |
| stage/render/create_views | 0.154 | 0.187 | 0.39 | 197.6 |
| stream/drive/ms | 0.133 | 0.157 | 0.54 | 171.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.123 | 0.183 | 0.32 | 157.8 |
| bigspace/high_precision/ms | 0.120 | 0.305 | 1.18 | 154.5 |
| pipelines/driver_ms | 0.112 | 0.520 | 4.41 | 144.4 |
| stream/tick/ms | 0.107 | 0.126 | 0.52 | 138.0 |
| stage/render/queue | 0.103 | 0.136 | 2.38 | 132.6 |
| stage/render/specialize | 0.097 | 0.172 | 3.46 | 124.5 |
| bigspace/low_precision/ms | 0.093 | 0.233 | 0.83 | 119.6 |
| controller/move/ms | 0.076 | 0.159 | 0.34 | 97.2 |
| anim/evaluate_cpu/ms | 0.067 | 0.082 | 0.48 | 86.2 |
| encode/resolve_diagnostics/sum | 0.059 | 0.089 | 0.18 | 76.0 |
| anim/evaluate/ms | 0.058 | 0.087 | 0.68 | 74.6 |
| controller/write/ms | 0.051 | 0.066 | 0.13 | 65.7 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.086 | 0.19 | 60.7 |
| terrain/update/ms | 0.036 | 0.047 | 0.55 | 45.7 |
| physics/step/ms | 0.031 | 0.052 | 0.12 | 39.6 |
| stage/render/cleanup | 0.025 | 0.033 | 0.20 | 32.7 |
| stage/render/phase_sort | 0.025 | 0.044 | 0.11 | 32.3 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.038 | 0.09 | 32.0 |
| weather/map/ms | 0.023 | 0.031 | 0.13 | 29.4 |
| encode/camera_driver_span_end/sum | 0.022 | 0.036 | 0.11 | 28.1 |
| weather/eval/ms | 0.010 | 0.013 | 0.08 | 13.0 |
| script/update/ms | 0.006 | 0.007 | 0.12 | 7.9 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 6.3 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 5.8 |
| controller/avoid/ms | 0.004 | 0.006 | 0.03 | 5.0 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 5.0 |
| script/ui/ms | 0.003 | 0.005 | 0.19 | 3.9 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.01 | 2.2 |
| character/finish/ms | 0.001 | 0.002 | 0.04 | 1.7 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| stream/promote/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| pipelines/wait_ms | 0.001 | 0.014 | 0.03 | 1.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 1.2 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| terrain/select/ms | 0.001 | 0.001 | 0.01 | 0.7 |
| weather/apply/ms | 0.000 | 0.000 | 0.00 | 0.3 |
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
| crowd/joints_extracted/count | count | 146 | 181 | 136 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.57 | 65.35 | 0.46 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.09 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.11 | 0.01 |
| encode/finish_critical | ms | 0.86 | 65.35 | 0.79 |
| encode/pass:early prepass/max | ms | 0.73 | 64.83 | 0.79 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.69 | 1.36 | 0.50 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.32 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.33 | 0.68 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.69 | 1.86 | 0.48 |
| encode/resolve_diagnostics/max | ms | 0.06 | 0.18 | 0.05 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.82 | 72.58 | 10.76 |
| gpu/timed/ms | ms | 10.82 | 12.46 | 10.76 |
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
| phases/opaque3d/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d/bins | count | 875 | 875 | 875 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 18 | 18 |
| phases/opaque3d_prepass/bins | count | 875 | 875 | 875 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2784 | 2784 | 2784 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 15 | 15 |
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
| precache/throttled | count | 0 | 33 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 121 | 121 | 121 |
| process/cpu_usage | % | 23.04 | 44.20 | 20.54 |
| process/mem_usage | GiB | 2.98 | 3.03 | 3.03 |
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
| skin/blocks_changed/count | count | 5 | 7 | 6 |
| skin/blocks_skipped/count | count | 176 | 178 | 175 |
| skin/blocks_walked/count | count | 5 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 9984 | 12.5 k | 9472 |
| skin/joints_written/count | count | 156 | 196 | 148 |
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
| system/cpu_usage | % | 30.58 | 40.93 | 23.52 |
| system/mem_usage | % | 50.87 | 51.03 | 51.03 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2545 | 2545 | 2545 |
| textures/images | count | 1998 | 1998 | 1998 |
| views/all/batch_sets | count | 62 | 99 | 62 |
| views/all/bins | count | 3142 | 3755 | 3142 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 17.6 k | 29.3 k | 17.6 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 9408 | 14.4 k | 9408 |
| views/all/pipelines | count | 73 | 80 | 73 |
| views/all/pipelines_distinct | count | 61 | 61 | 61 |
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
| views/main/batch_sets | count | 58 | 58 | 58 |
| views/main/bins | count | 2246 | 2246 | 2246 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6206 | 6206 | 6206 |
| views/main/pipelines | count | 46 | 46 | 46 |
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
| anim/events/count | count | 0.12 | 4 | 160 |
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
| encode//count | count | 28.55 | 31 | 36.7 k |
| encode/bind_walk/init_textures | count | 7187 | 12.1 k | 9.23 M |
| encode/bind_walk/init_textures_skipped | count | 43.0 k | 48.8 k | 55.29 M |
| encode/bind_walk/inits | count | 500 | 604 | 642.3 k |
| encode/bind_walk/inits_skipped | count | 129 | 137 | 165.4 k |
| encode/bind_walk/merge_textures | count | 7122 | 12.0 k | 9.15 M |
| encode/bind_walk/merge_textures_skipped | count | 6434 | 12.2 k | 8.27 M |
| encode/bind_walk/merges | count | 91.26 | 111 | 117.3 k |
| encode/bind_walk/merges_skipped | count | 113 | 128 | 144.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1285 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1285 |
| encode/pass:early prepass/count | count | 1 | 1 | 1285 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1285 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1285 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1285 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1285 |
| encode/queue/command_buffers | count | 38.55 | 41 | 49.5 k |
| encode/queue/submits | count | 2 | 2 | 2570 |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 3.36 M | 2.41 G |
| encode/queue/write_buffer/calls | count | 227 | 298 | 292.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 41.94 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.5 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1285 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 3 | 899 |
| physics/steps_dropped | count | 0.00 | 2 | 2 |
| pipelines/after_reveal | count | 0.06 | 1 | 75 |
| pipelines/created | count | 0.06 | 1 | 78 |
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

## Hitches (> 29.2 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2534.5 | main/busy/ms 2504, stage/main/post_update 2391, stage/main/update 101, surface/advance_cpu_ms 70.14, stage/main/pre_update 9.32, stage/main/last 1.11, surface/cell_upload_ms 0.74, stream/drive/ms 0.59, stage/main/first 0.48, stage/main/fixed_main_loop 0.39, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.19 |
| warm-up | 3.27 | 736.7 | main/busy/ms 2551, stage/main/post_update 2424, render/thread/ms 676, render/busy/ms 533, stage/render/prepare_assets 473, stage/render/prepare_views 140, render/acquire/ms 140, stage/main/update 108, surface/advance_cpu_ms 70.14, stage/render/create_views 30.52, stage/render/render 22.17, render/graph/ms 17.17 |
| warm-up | 3.40 | 125.3 | render/thread/ms 799, render/busy/ms 569, stage/render/prepare_assets 481, main/busy/ms 168, stage/render/prepare_views 140, render/acquire/ms 140, stage/render/render 126, render/present/ms 89.92, stage/main/last 76.29, stage/main/post_update 61.39, render/graph/ms 34.18, stage/render/create_views 30.81 |
| warm-up | 3.44 | 47.3 | pipelines/compile_ms 387, pipelines/front_ms 223, render/thread/ms 168, pipelines/driver_ms 164, main/busy/ms 146, stage/render/render 115, render/present/ms 87.55, render/busy/ms 79.89, stage/main/last 76.71, stage/main/post_update 40.79, stage/render/prepare_assets 34.72, terrain/update/ms 30.64 |
| warm-up | 3.55 | 109.1 | pipelines/wait_ms 2420, pipelines/compile_ms 1024, pipelines/front_ms 818, pipelines/driver_ms 207, render/thread/ms 140, render/busy/ms 137, main/busy/ms 108, stage/main/update 56.79, stage/render/prepare_assets 43.32, stage/main/post_update 31.15, stage/render/prepare_resources 30.54, stage/render/prepare_bind_groups 26.62 |
| warm-up | 3.65 | 98.1 | pipelines/wait_ms 4930, pipelines/compile_ms 2030, pipelines/front_ms 1908, render/thread/ms 189, render/busy/ms 187, main/busy/ms 127, pipelines/driver_ms 122, stage/render/prepare_assets 91.84, stage/main/update 67.13, stage/render/prepare_resources 35.75, stage/main/post_update 32.09, stage/render/prepare_bind_groups 26.83 |
| warm-up | 3.69 | 38.2 | pipelines/wait_ms 2908, pipelines/compile_ms 2281, pipelines/front_ms 2164, render/thread/ms 130, render/busy/ms 129, pipelines/driver_ms 116, stage/render/prepare_assets 86.84, main/busy/ms 61.66, stage/main/update 25.83, stage/main/post_update 19.58, stage/render/prepare_resources 14.96, terrain/update/ms 14.34 |
| warm-up | 3.73 | 42.6 | pipelines/compile_ms 1437, pipelines/front_ms 1370, pipelines/wait_ms 456, render/thread/ms 75.20, render/busy/ms 72.59, pipelines/driver_ms 67.23, main/busy/ms 37.75, stage/render/prepare_assets 35.00, stage/main/post_update 16.98, stage/render/render 15.64, stage/main/update 14.95, encode//sum 13.53 |
| warm-up | 3.85 | 39.8 | pipelines/compile_ms 255, pipelines/front_ms 239, pipelines/wait_ms 75.50, render/thread/ms 62.64, render/busy/ms 60.48, main/busy/ms 44.06, stage/render/prepare_assets 25.92, stage/main/post_update 16.07, stage/render/render 15.94, stage/main/update 15.84, pipelines/driver_ms 15.56, render/graph/ms 13.15 |
| warm-up | 3.90 | 54.4 | pipelines/compile_ms 1651, pipelines/front_ms 1566, pipelines/wait_ms 266, render/thread/ms 88.72, render/busy/ms 85.97, pipelines/driver_ms 84.32, stage/render/prepare_assets 47.74, main/busy/ms 45.19, stage/render/render 26.52, render/graph/ms 23.11, stage/main/update 20.06, stage/render/graph_submit 19.38 |
| warm-up | 3.96 | 57.0 | pipelines/compile_ms 1725, pipelines/front_ms 1633, pipelines/wait_ms 193, render/thread/ms 103, render/busy/ms 98.88, pipelines/driver_ms 91.78, stage/render/prepare_assets 56.66, main/busy/ms 45.75, stage/render/render 35.34, render/graph/ms 30.33, stage/render/graph_submit 22.65, stage/main/update 19.50 |
| warm-up | 4.06 | 97.7 | pipelines/compile_ms 926, pipelines/front_ms 860, render/thread/ms 147, render/busy/ms 144, stage/render/prepare_assets 114, pipelines/driver_ms 65.50, main/busy/ms 47.69, stage/render/render 24.41, render/graph/ms 20.39, stage/main/update 15.29, stage/main/post_update 13.21, stage/render/graph_submit 13.19 |
| warm-up | 4.09 | 29.6 | pipelines/compile_ms 794, pipelines/front_ms 739, render/thread/ms 123, render/busy/ms 121, stage/render/prepare_assets 107, pipelines/driver_ms 54.78, main/busy/ms 38.00, stage/main/update 15.58, stage/main/post_update 12.70, stage/render/render 11.07, render/graph/ms 9.46, encode//sum 7.54 |
| warm-up | 4.21 | 34.6 | render/thread/ms 44.13, render/busy/ms 42.33, main/busy/ms 29.50, stage/render/prepare_assets 26.63, stage/main/update 16.12, stage/render/render 12.03, encode//sum 10.42, render/graph/ms 9.64, stage/main/post_update 7.83, stage/render/graph_submit 6.18, encode/submit 3.45, stage/render/camera_driver 3.16 |
| warm-up | 4.26 | 31.9 | render/thread/ms 49.95, render/busy/ms 48.27, main/busy/ms 36.50, stage/render/prepare_assets 31.65, stage/main/update 16.19, stage/render/render 12.96, stage/main/post_update 11.25, render/graph/ms 10.87, stage/render/graph_submit 7.95, encode//sum 7.29, encode/submit 6.21, stage/render/camera_driver 2.71 |
| warm-up | 4.32 | 32.1 | main/busy/ms 55.92, render/thread/ms 53.14, render/busy/ms 51.52, stage/main/update 37.34, pipelines/compile_ms 33.15, pipelines/front_ms 29.40, stage/render/prepare_assets 24.43, pipelines/wait_ms 20.35, stage/render/render 14.86, render/graph/ms 12.90, stage/render/graph_submit 9.00, encode//sum 8.77 |
| warm-up | 4.35 | 30.5 | pipelines/compile_ms 54.09, pipelines/wait_ms 53.52, render/thread/ms 53.20, main/busy/ms 52.09, render/busy/ms 51.77, pipelines/front_ms 49.19, stage/main/update 33.35, stage/render/prepare_assets 28.39, stage/render/render 12.22, render/graph/ms 10.44, encode//sum 8.44, stage/render/graph_submit 6.99 |
| warm-up | 4.42 | 31.0 | pipelines/compile_ms 178, pipelines/front_ms 172, render/thread/ms 45.56, render/busy/ms 43.68, main/busy/ms 42.27, stage/main/update 25.66, stage/render/render 17.01, stage/render/prepare_assets 14.72, render/graph/ms 14.66, stage/render/graph_submit 10.24, encode//sum 8.94, scene/load/chains/ms 8.89 |
| warm-up | 4.45 | 30.1 | pipelines/compile_ms 178, pipelines/front_ms 172, render/thread/ms 56.96, render/busy/ms 55.07, main/busy/ms 53.19, stage/main/update 29.74, stage/render/prepare_assets 19.39, stage/render/render 19.05, render/graph/ms 16.48, stage/main/post_update 16.06, stage/render/graph_submit 10.89, encode//sum 9.78 |
| warm-up | 4.48 | 30.9 | pipelines/compile_ms 368, pipelines/front_ms 356, render/thread/ms 57.21, render/busy/ms 55.92, main/busy/ms 42.39, stage/render/prepare_assets 29.15, stage/main/update 20.12, stage/main/post_update 15.80, stage/render/render 12.94, pipelines/driver_ms 12.18, render/graph/ms 11.17, encode//sum 8.83 |
| warm-up | 4.54 | 31.6 | pipelines/wait_ms 593, pipelines/compile_ms 219, pipelines/front_ms 210, render/thread/ms 56.63, render/busy/ms 55.13, main/busy/ms 48.63, stage/main/update 32.73, stage/render/prepare_assets 23.49, stage/render/render 13.29, render/graph/ms 11.10, stage/main/post_update 9.78, encode//sum 9.42 |
| warm-up | 4.60 | 36.9 | pipelines/wait_ms 222, main/busy/ms 60.43, render/thread/ms 48.63, render/busy/ms 46.96, stage/main/update 43.63, stage/render/prepare_assets 15.79, stage/render/render 12.35, stage/main/post_update 10.91, render/graph/ms 10.24, encode//sum 7.82, character/finish/ms 6.97, stage/render/graph_submit 6.43 |
| warm-up | 5.07 | 35.6 | pipelines/wait_ms 973, pipelines/compile_ms 310, pipelines/front_ms 298, render/thread/ms 58.53, render/busy/ms 56.45, main/busy/ms 44.27, stage/main/update 22.64, stage/render/render 20.32, stage/render/prepare_assets 19.57, render/graph/ms 17.09, stage/main/post_update 15.37, stage/render/graph_submit 12.00 |
| warm-up | 5.16 | 62.8 | pipelines/wait_ms 2476, pipelines/compile_ms 250, pipelines/front_ms 241, render/thread/ms 87.43, render/busy/ms 86.08, main/busy/ms 46.31, stage/render/prepare_assets 39.71, stage/main/update 30.64, stage/render/prepare_bind_groups 27.67, stage/render/render 11.31, render/graph/ms 9.50, pipelines/driver_ms 9.16 |
| warm-up | 5.21 | 44.5 | pipelines/wait_ms 2118, pipelines/compile_ms 278, pipelines/front_ms 257, render/thread/ms 104, render/busy/ms 102, main/busy/ms 64.35, stage/main/update 49.26, stage/render/prepare_assets 30.08, stage/render/prepare_bind_groups 28.08, pipelines/driver_ms 20.71, stage/render/prepare_flush 18.16, stage/render/render 14.32 |
| warm-up | 5.37 | 43.4 | pipelines/wait_ms 932, main/busy/ms 57.81, render/thread/ms 38.55, render/busy/ms 36.78, stage/main/update 32.24, pipelines/compile_ms 24.86, pipelines/front_ms 23.52, stage/render/render 15.60, render/graph/ms 13.07, stage/render/prepare_assets 10.01, encode//sum 9.38, stage/render/graph_submit 9.30 |
| warm-up | 5.51 | 41.0 | pipelines/wait_ms 956, pipelines/compile_ms 146, pipelines/front_ms 142, main/busy/ms 43.52, render/thread/ms 34.87, render/busy/ms 33.63, stage/main/update 23.30, stage/main/post_update 13.22, stage/render/prepare_assets 11.83, stage/render/render 9.49, render/graph/ms 7.93, encode//sum 7.92 |
| warm-up | 5.99 | 29.5 | render/thread/ms 51.55, render/busy/ms 49.88, main/busy/ms 44.01, stage/main/update 22.26, stage/render/render 15.72, stage/main/post_update 14.36, render/graph/ms 13.47, stage/render/prepare_assets 12.65, encode//sum 10.04, stage/render/prepare_meshes 9.86, stage/render/graph_submit 9.15, encode/submit 6.43 |
| measure | 6.42 | 34.1 | render/thread/ms 45.58, render/busy/ms 37.54, encode//sum 34.24, stage/render/render 33.20, render/graph/ms 31.78, stage/render/graph_submit 29.44, encode/finish_wall 24.32, encode/pass:early prepass/sum 23.35, main/busy/ms 11.95, stage/render/prepare_views 7.16, render/acquire/ms 7.05, stage/main/post_update 6.48 |
| measure | 8.86 | 76.3 | render/thread/ms 86.08, render/busy/ms 79.99, stage/render/render 75.33, encode//sum 74.00, render/graph/ms 74.00, stage/render/graph_submit 71.77, encode/finish_wall 66.76, encode/pass:early prepass/sum 65.23, main/busy/ms 13.07, stage/main/post_update 7.33, stage/render/prepare_views 5.39, render/acquire/ms 5.26 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=off-1 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 96%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.8 W · 72 °C (max 75) · VRAM 5346 MiB · throttled 100% of samples

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
