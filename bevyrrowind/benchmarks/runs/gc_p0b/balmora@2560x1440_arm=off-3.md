# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: groundcover · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1292 frames over 15.0 s · **median 11.69 ms (86 fps)** · mean 11.61 · p95 12.95 · p99 13.42 (1% low 75 fps) · worst 18.50 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 0.0/min)
- warm-up: 313 frames over 8.7 s, worst 2576.5 ms
- hitches (> 29.2 ms): **0** measured, 25 in warm-up
- timed GPU passes: 10.19 ms of a 11.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.69 ms, limit 11 ms
- pass worst frame 18.50 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.232 | 3.237 | 3.324 | 1.00 | 0.065 |
| dlss_super_resolution | 2.019 | 2.022 | 2.045 | 1.00 | 0.182 |
| sun_visibility | 0.779 | 0.780 | 0.808 | 1.00 | 0.006 |
| early prepass | 0.647 | 0.648 | 0.656 | 1.00 | 0.096 |
| main_transmissive_pass_3d | 0.397 | 0.397 | 0.409 | 1.00 | 0.008 |
| atmosphere_luts | 0.372 | 0.372 | 0.380 | 1.00 | 0.003 |
| bloom | 0.313 | 0.314 | 0.331 | 1.00 | 0.059 |
| atmos_froxel_inject | 0.307 | 0.308 | 0.309 | 1.00 | 0.001 |
| water_velocity | 0.306 | 0.306 | 0.310 | 1.00 | 0.001 |
| water_sspr | 0.208 | 0.208 | 0.210 | 1.00 | 0.002 |
| ssao | 0.188 | 0.188 | 0.196 | 1.00 | 0.007 |
| clouds_trace | 0.152 | 0.153 | 0.173 | 1.00 | 0.001 |
| water_fft | 0.134 | 0.134 | 0.135 | 1.00 | 0.002 |
| clouds_composite | 0.116 | 0.116 | 0.118 | 1.00 | 0.002 |
| uniform_allocation | 0.116 | 0.116 | 0.147 | 1.00 | 0.019 |
| upscaling | 0.115 | 0.115 | 0.121 | 1.00 | 0.004 |
| bin_unpacking | 0.091 | 0.091 | 0.115 | 1.00 | 0.014 |
| tonemapping | 0.085 | 0.086 | 0.089 | 1.00 | 0.003 |
| gpu_instance_materialization | 0.074 | 0.074 | 0.097 | 1.00 | 0.003 |
| clustering | 0.063 | 0.063 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.062 | 0.062 | 0.062 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.060 | 0.060 | 0.086 | 1.00 | 0.003 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.412 | 0.13 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.221 | 0.232 | 0.13 | 0.003 |
| water_snapshot | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| range_unpacking | 0.022 | 0.022 | 0.028 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.019 | 0.019 | 0.036 | 1.00 | 0.005 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.026 | 1.00 | 0.186 |
| surface_exposure_far | 0.012 | 0.040 | 0.042 | 0.30 | 0.002 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.001 |
| surface_exposure | 0.011 | 0.039 | 0.040 | 0.28 | 0.001 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.001 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.13 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.13 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.488 | 11.834 | 17.38 | 13550.8 |
| render/busy/ms | 6.939 | 7.913 | 15.42 | 8965.3 |
| main/busy/ms | 6.492 | 7.744 | 9.37 | 8387.7 |
| stage/render/render | 4.669 | 5.433 | 11.10 | 6032.5 |
| encode//sum | 4.492 | 5.793 | 10.21 | 5804.1 |
| render/graph/ms | 4.081 | 4.812 | 10.35 | 5272.2 |
| stage/main/post_update | 3.633 | 4.563 | 6.08 | 4693.7 |
| stage/render/prepare_views | 3.242 | 4.852 | 6.14 | 4188.3 |
| render/acquire/ms | 3.193 | 4.804 | 6.09 | 4125.6 |
| stage/render/graph_submit | 2.938 | 3.623 | 9.08 | 3796.0 |
| encode/submit | 1.810 | 2.203 | 3.46 | 2338.5 |
| stage/main/update | 1.308 | 1.669 | 2.26 | 1690.4 |
| encode/finish_wall | 1.145 | 1.554 | 6.08 | 1479.3 |
| pipelines/compile_ms | 1.126 | 0.000 | 123.73 | 1455.2 |
| pipelines/front_ms | 1.097 | 0.000 | 121.09 | 1416.9 |
| stage/render/camera_driver | 1.072 | 1.244 | 1.63 | 1384.7 |
| encode/pass:shadow_directional_light/sum | 0.730 | 1.277 | 5.90 | 942.6 |
| encode/pass:early prepass/sum | 0.723 | 1.012 | 1.34 | 933.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.691 | 0.973 | 1.39 | 892.9 |
| stage/render/prepare_write_phase_buffers | 0.476 | 0.677 | 0.87 | 615.1 |
| stage/main/fixed_main_loop | 0.424 | 0.643 | 1.19 | 547.8 |
| stage/render/prepare_resources | 0.421 | 0.488 | 1.82 | 543.5 |
| stage/main/pre_update | 0.396 | 0.465 | 0.67 | 511.7 |
| stage/render/prepare_assets | 0.384 | 0.455 | 1.54 | 495.6 |
| render/present/ms | 0.356 | 0.420 | 2.46 | 459.9 |
| bigspace/total/ms | 0.330 | 0.631 | 1.36 | 426.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.318 | 0.447 | 0.63 | 411.3 |
| stage/main/first | 0.316 | 0.375 | 1.80 | 408.6 |
| stage/render/prepare_flush | 0.270 | 0.411 | 0.67 | 348.8 |
| stage/render/prepare_batch_phases | 0.259 | 0.355 | 0.50 | 334.5 |
| stage/render/prepare_bind_groups | 0.245 | 0.336 | 0.45 | 316.4 |
| stage/main/last | 0.242 | 0.343 | 0.81 | 312.9 |
| stage/render/prepare_meshes | 0.202 | 0.253 | 1.19 | 261.3 |
| controller/feed/ms | 0.179 | 0.222 | 0.70 | 231.8 |
| stage/render/extract_commands | 0.173 | 0.207 | 0.30 | 223.0 |
| stage/render/create_views | 0.155 | 0.191 | 0.28 | 200.3 |
| stream/drive/ms | 0.130 | 0.150 | 0.48 | 168.3 |
| bigspace/high_precision/ms | 0.121 | 0.309 | 1.02 | 156.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.118 | 0.177 | 0.57 | 152.6 |
| stream/tick/ms | 0.104 | 0.120 | 0.46 | 133.9 |
| stage/render/queue | 0.101 | 0.133 | 2.56 | 130.2 |
| bigspace/low_precision/ms | 0.083 | 0.209 | 0.94 | 107.8 |
| controller/move/ms | 0.080 | 0.160 | 0.25 | 103.9 |
| anim/evaluate/ms | 0.076 | 0.141 | 0.89 | 97.6 |
| stage/render/specialize | 0.073 | 0.095 | 3.32 | 93.9 |
| anim/evaluate_cpu/ms | 0.061 | 0.073 | 0.54 | 79.3 |
| encode/resolve_diagnostics/sum | 0.057 | 0.084 | 0.44 | 73.0 |
| controller/write/ms | 0.051 | 0.065 | 0.10 | 65.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.082 | 0.23 | 59.1 |
| terrain/update/ms | 0.037 | 0.047 | 0.49 | 47.2 |
| pipelines/driver_ms | 0.030 | 0.000 | 4.21 | 38.3 |
| stage/render/phase_sort | 0.029 | 0.049 | 0.13 | 37.6 |
| physics/step/ms | 0.028 | 0.047 | 0.08 | 36.1 |
| stage/render/cleanup | 0.026 | 0.035 | 0.39 | 33.1 |
| encode/camera_driver_span_begin/sum | 0.024 | 0.037 | 0.31 | 30.7 |
| weather/map/ms | 0.023 | 0.031 | 0.18 | 29.4 |
| encode/camera_driver_span_end/sum | 0.021 | 0.034 | 0.13 | 26.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.05 | 13.3 |
| script/update/ms | 0.006 | 0.007 | 0.07 | 7.7 |
| anim/apply/ms | 0.005 | 0.006 | 0.37 | 6.2 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 6.0 |
| controller/avoid/ms | 0.004 | 0.007 | 0.07 | 5.2 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 4.7 |
| script/ui/ms | 0.003 | 0.005 | 0.04 | 3.6 |
| stream/poll_models/ms | 0.002 | 0.002 | 0.02 | 2.1 |
| character/finish/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| stream/assets_ready/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| stream/promote/ms | 0.001 | 0.001 | 0.02 | 1.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.1 |
| stream/poll_fetches/ms | 0.001 | 0.001 | 0.01 | 0.8 |
| terrain/select/ms | 0.001 | 0.001 | 0.02 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.7 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| crowd/joints_extracted/count | count | 115 | 181 | 180 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 37.9 k | 37.9 k | 37.9 k |
| encode//max | ms | 0.56 | 1.87 | 0.46 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.31 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.13 | 0.02 |
| encode/finish_critical | ms | 0.86 | 5.90 | 0.70 |
| encode/pass:early prepass/max | ms | 0.73 | 1.34 | 0.63 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.70 | 1.39 | 0.70 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.57 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.31 | 0.63 | 0.38 |
| encode/pass:shadow_directional_light/max | ms | 0.69 | 5.90 | 0.50 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.44 | 0.05 |
| far/entities | count | 2800 | 2800 | 2800 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 57.9 k | 57.9 k | 57.9 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 188.03 M | 188.03 M | 188.03 M |
| far/triangles | count | 4.63 M | 4.63 M | 4.63 M |
| frame/work/ms | ms | 10.81 | 15.42 | 10.91 |
| gpu/timed/ms | ms | 10.81 | 12.12 | 10.91 |
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
| precache/throttled | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 5 | 69 | 5 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 125 | 125 |
| process/cpu_usage | % | 22.42 | 41.35 | 17.79 |
| process/mem_usage | GiB | 3.00 | 3.05 | 3.05 |
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
| skin/blocks_skipped/count | count | 176 | 179 | 175 |
| skin/blocks_walked/count | count | 5 | 7 | 6 |
| skin/joint_bytes_written/bytes | bytes | 10.2 k | 12.5 k | 12.5 k |
| skin/joints_written/count | count | 160 | 196 | 196 |
| skin/upload_writes/count | count | 4 | 6 | 6 |
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
| system/cpu_usage | % | 29.56 | 40.75 | 23.68 |
| system/mem_usage | % | 49.96 | 50.14 | 50.14 |
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
| anim/events/count | count | 0.11 | 2 | 137 |
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
| encode/bind_walk/init_textures | count | 7188 | 12.1 k | 9.29 M |
| encode/bind_walk/init_textures_skipped | count | 41.6 k | 48.8 k | 53.73 M |
| encode/bind_walk/inits | count | 504 | 608 | 651.0 k |
| encode/bind_walk/inits_skipped | count | 127 | 137 | 164.4 k |
| encode/bind_walk/merge_textures | count | 7123 | 12.0 k | 9.20 M |
| encode/bind_walk/merge_textures_skipped | count | 6435 | 12.2 k | 8.31 M |
| encode/bind_walk/merges | count | 91.26 | 111 | 117.9 k |
| encode/bind_walk/merges_skipped | count | 112 | 128 | 144.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1292 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1292 |
| encode/pass:early prepass/count | count | 1 | 1 | 1292 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1292 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1292 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1292 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1292 |
| encode/queue/command_buffers | count | 38.55 | 41 | 49.8 k |
| encode/queue/submits | count | 2 | 2 | 2584 |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 3.36 M | 2.42 G |
| encode/queue/write_buffer/calls | count | 233 | 308 | 301.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 42.17 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 11.6 k |
| encode/queue/write_texture/bytes | bytes | 20.4 k | 8.39 M | 26.35 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 5 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1292 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.70 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 23 |
| pipelines/created | count | 0.02 | 1 | 23 |
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

## Hitches (> 29.2 ms; slowest 25, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2576.5 | main/busy/ms 2545, stage/main/post_update 2430, stage/main/update 102, surface/advance_cpu_ms 73.53, stage/main/pre_update 9.78, stage/main/last 1.05, surface/cell_upload_ms 0.73, stage/main/first 0.57, stream/drive/ms 0.47, stage/main/fixed_main_loop 0.38, weather/map/ms 0.15, stream/tick/ms 0.12 |
| warm-up | 3.33 | 752.7 | main/busy/ms 2591, stage/main/post_update 2462, render/thread/ms 688, render/busy/ms 544, stage/render/prepare_assets 483, stage/render/prepare_views 142, render/acquire/ms 142, stage/main/update 109, surface/advance_cpu_ms 73.53, stage/render/create_views 29.33, stage/render/render 19.85, render/graph/ms 16.24 |
| warm-up | 3.47 | 140.2 | render/thread/ms 826, render/busy/ms 581, stage/render/prepare_assets 492, stage/render/prepare_views 142, render/acquire/ms 142, stage/render/render 138, render/present/ms 103, main/busy/ms 74.11, stage/main/post_update 47.06, render/graph/ms 33.37, stage/render/create_views 29.57, terrain/update/ms 24.44 |
| warm-up | 3.54 | 74.5 | pipelines/compile_ms 635, pipelines/front_ms 436, pipelines/wait_ms 432, render/thread/ms 210, pipelines/driver_ms 198, stage/render/render 156, render/present/ms 106, render/busy/ms 104, main/busy/ms 51.81, render/graph/ms 48.76, stage/render/prepare_assets 36.44, terrain/update/ms 30.77 |
| warm-up | 3.67 | 130.0 | pipelines/wait_ms 4014, pipelines/compile_ms 2455, pipelines/front_ms 2131, pipelines/driver_ms 324, render/thread/ms 199, render/busy/ms 192, main/busy/ms 115, stage/main/update 69.29, stage/render/prepare_assets 55.16, stage/render/render 53.47, render/graph/ms 46.15, stage/render/prepare_flush 40.88 |
| warm-up | 3.82 | 150.6 | pipelines/wait_ms 4009, pipelines/compile_ms 2706, pipelines/front_ms 2513, render/thread/ms 273, render/busy/ms 270, pipelines/driver_ms 192, main/busy/ms 123, stage/render/prepare_assets 108, stage/render/prepare_flush 85.48, stage/main/update 75.55, stage/render/render 31.47, render/graph/ms 28.31 |
| warm-up | 3.85 | 29.4 | pipelines/compile_ms 1343, pipelines/front_ms 1258, pipelines/wait_ms 476, render/thread/ms 173, render/busy/ms 171, stage/render/prepare_assets 95.58, pipelines/driver_ms 85.22, main/busy/ms 52.67, stage/render/prepare_flush 45.20, stage/render/render 20.93, stage/main/post_update 18.49, render/graph/ms 18.08 |
| warm-up | 3.99 | 53.4 | pipelines/compile_ms 225, pipelines/front_ms 208, render/thread/ms 72.56, render/busy/ms 70.24, stage/render/prepare_assets 38.11, main/busy/ms 37.65, stage/render/render 19.79, render/graph/ms 16.95, stage/main/post_update 16.76, pipelines/driver_ms 16.50, stage/main/update 13.70, stage/render/graph_submit 12.40 |
| warm-up | 4.05 | 52.7 | pipelines/compile_ms 1241, pipelines/front_ms 1181, render/thread/ms 93.25, render/busy/ms 89.13, pipelines/driver_ms 59.94, stage/render/prepare_assets 47.66, main/busy/ms 34.00, stage/render/render 33.09, render/graph/ms 28.08, stage/render/graph_submit 21.27, stage/main/update 13.65, stage/main/post_update 12.63 |
| warm-up | 4.11 | 57.9 | pipelines/compile_ms 1617, pipelines/front_ms 1551, render/thread/ms 97.28, render/busy/ms 93.67, pipelines/driver_ms 66.08, stage/render/prepare_assets 60.72, main/busy/ms 28.04, stage/render/render 26.25, pipelines/wait_ms 24.16, render/graph/ms 22.02, pipelines/lock_ms 19.78, stage/render/graph_submit 16.24 |
| warm-up | 4.19 | 81.8 | pipelines/compile_ms 968, pipelines/front_ms 822, pipelines/driver_ms 146, render/thread/ms 135, render/busy/ms 134, stage/render/prepare_assets 116, main/busy/ms 31.32, pipelines/wait_ms 20.44, pipelines/lock_ms 16.28, stage/main/post_update 15.06, stage/render/render 11.89, render/graph/ms 9.91 |
| warm-up | 4.22 | 30.1 | pipelines/compile_ms 408, pipelines/front_ms 284, pipelines/driver_ms 124, render/thread/ms 108, render/busy/ms 107, stage/render/prepare_assets 90.70, main/busy/ms 28.10, stage/main/post_update 13.02, stage/render/render 12.41, render/graph/ms 10.82, stage/main/update 8.88, stage/render/graph_submit 8.58 |
| warm-up | 4.37 | 58.2 | render/thread/ms 79.19, render/busy/ms 77.17, pipelines/compile_ms 57.91, pipelines/front_ms 55.54, main/busy/ms 48.52, stage/render/render 34.51, stage/render/prepare_assets 32.50, render/graph/ms 31.95, stage/main/update 25.16, stage/render/camera_driver 19.76, stage/main/post_update 14.16, stage/render/graph_submit 11.89 |
| warm-up | 4.42 | 42.7 | pipelines/compile_ms 152, pipelines/front_ms 146, render/thread/ms 96.21, render/busy/ms 94.20, main/busy/ms 67.20, stage/render/prepare_assets 47.51, stage/main/update 45.59, stage/render/render 32.25, render/graph/ms 29.79, stage/render/camera_driver 18.81, pipelines/wait_ms 18.32, stage/main/post_update 12.37 |
| warm-up | 4.46 | 46.5 | pipelines/compile_ms 124, pipelines/front_ms 119, render/thread/ms 85.48, render/busy/ms 83.75, main/busy/ms 65.29, stage/render/prepare_assets 56.25, stage/main/update 42.01, pipelines/wait_ms 16.13, stage/main/post_update 14.88, stage/render/render 13.49, render/graph/ms 11.30, stage/render/graph_submit 8.18 |
| warm-up | 4.57 | 102.2 | pipelines/compile_ms 1579, pipelines/front_ms 1531, pipelines/wait_ms 532, render/thread/ms 145, render/busy/ms 144, stage/render/prepare_write_phase_buffers 59.27, pipelines/driver_ms 48.14, stage/render/prepare_assets 44.89, main/busy/ms 40.75, stage/render/render 17.57, stage/main/update 17.51, render/graph/ms 15.23 |
| warm-up | 4.60 | 31.6 | pipelines/compile_ms 1822, pipelines/front_ms 1766, pipelines/wait_ms 729, render/thread/ms 131, render/busy/ms 124, stage/render/prepare_write_phase_buffers 58.83, pipelines/driver_ms 56.33, main/busy/ms 37.34, stage/render/prepare_assets 22.92, stage/render/render 19.08, render/graph/ms 16.83, stage/main/update 14.62 |
| warm-up | 4.63 | 38.0 | pipelines/compile_ms 508, pipelines/front_ms 491, pipelines/wait_ms 197, render/thread/ms 66.00, main/busy/ms 65.18, render/busy/ms 58.61, stage/main/update 39.80, stage/render/prepare_bind_groups 17.65, stage/render/render 16.98, pipelines/driver_ms 16.94, stage/main/post_update 15.32, render/graph/ms 14.47 |
| warm-up | 4.71 | 74.8 | pipelines/compile_ms 1030, pipelines/front_ms 995, main/busy/ms 115, render/thread/ms 99.24, render/busy/ms 95.58, stage/main/update 78.33, stage/render/prepare_assets 41.17, pipelines/wait_ms 38.34, pipelines/driver_ms 35.40, stage/main/post_update 25.57, stage/render/render 21.61, stage/render/prepare_bind_groups 18.23 |
| warm-up | 4.78 | 70.0 | pipelines/compile_ms 1287, pipelines/front_ms 1249, pipelines/wait_ms 207, render/thread/ms 122, render/busy/ms 119, main/busy/ms 96.69, stage/render/prepare_assets 72.79, stage/main/update 57.40, pipelines/driver_ms 38.46, stage/main/post_update 25.89, stage/render/render 22.83, render/graph/ms 18.83 |
| warm-up | 4.84 | 60.2 | pipelines/compile_ms 1403, pipelines/front_ms 1354, pipelines/wait_ms 172, render/thread/ms 118, render/busy/ms 116, main/busy/ms 79.60, stage/main/update 54.58, stage/render/prepare_assets 50.81, pipelines/driver_ms 49.03, stage/render/prepare_resources 27.22, stage/render/render 16.62, stage/main/post_update 14.82 |
| warm-up | 4.92 | 29.5 | pipelines/compile_ms 419, pipelines/front_ms 407, render/thread/ms 53.10, render/busy/ms 51.23, main/busy/ms 36.63, stage/main/update 19.64, stage/render/render 14.95, render/graph/ms 12.81, pipelines/driver_ms 11.79, stage/main/post_update 11.22, stage/render/graph_submit 9.19, stage/render/prepare_resources 8.61 |
| warm-up | 5.42 | 45.4 | pipelines/compile_ms 203, pipelines/front_ms 194, main/busy/ms 53.94, render/thread/ms 53.01, render/busy/ms 37.53, stage/render/render 33.32, stage/main/last 19.40, stage/main/post_update 18.16, render/graph/ms 17.31, render/present/ms 15.26, stage/render/graph_submit 13.21, stage/main/update 13.09 |
| warm-up | 5.46 | 46.4 | pipelines/compile_ms 436, pipelines/front_ms 423, render/thread/ms 86.78, main/busy/ms 76.75, render/busy/ms 70.93, stage/render/render 37.16, pipelines/wait_ms 26.92, stage/main/update 25.17, stage/main/post_update 22.53, render/graph/ms 20.48, stage/main/last 19.53, render/present/ms 15.62 |
| warm-up | 5.51 | 46.4 | pipelines/compile_ms 234, pipelines/front_ms 228, render/thread/ms 82.68, render/busy/ms 81.30, main/busy/ms 52.47, stage/render/prepare_assets 36.01, stage/main/update 25.68, pipelines/wait_ms 24.25, stage/main/post_update 14.60, stage/render/render 13.76, stage/render/prepare_bind_groups 13.02, render/graph/ms 11.43 |

## Run

- `target/profiling/bench-gc.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\gc_p0b\balmora@2560x1440_arm=off-3 --bench-baseline none --off groundcover
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (266 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2625 MHz (min 2625) · memory 8101 MHz · 92.2 W · 75 °C (max 77) · VRAM 5347 MiB · throttled 100% of samples

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
