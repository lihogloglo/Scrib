# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5759 frames over 65.6 s · **median 10.96 ms (91 fps)** · mean 11.40 · p95 14.79 · p99 16.93 (1% low 59 fps) · worst 26.93 ms · 78 spikes over 1.5× median · 21.9 stutters/min (work 19.2/min)
- warm-up: 324 frames over 9.3 s, worst 2401.0 ms
- hitches (> 27.4 ms): **0** measured, 47 in warm-up
- timed GPU passes: 6.80 ms of a 11.40 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.96 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.94/min, limit 1/min
- pass worst frame 26.93 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.028 | 2.030 | 2.063 | 1.00 | 0.174 |
| main_opaque_pass_3d | 1.299 | 1.300 | 1.648 | 1.00 | 0.077 |
| early prepass | 0.491 | 0.491 | 0.699 | 1.00 | 0.086 |
| bloom | 0.316 | 0.316 | 0.323 | 1.00 | 0.063 |
| atmosphere_luts | 0.309 | 0.310 | 0.372 | 1.00 | 0.004 |
| auxiliary_views | 0.306 | 0.306 | 1.044 | 1.00 | 0.300 |
| water_velocity | 0.263 | 0.263 | 0.350 | 1.00 | 0.002 |
| water_sspr | 0.201 | 0.201 | 0.202 | 1.00 | 0.003 |
| ssao | 0.188 | 0.188 | 0.193 | 1.00 | 0.007 |
| water_fft | 0.131 | 0.131 | 0.139 | 1.00 | 0.003 |
| upscaling | 0.130 | 0.130 | 0.137 | 1.00 | 0.004 |
| atmos_froxel_inject | 0.127 | 0.127 | 0.128 | 1.00 | 0.001 |
| uniform_allocation | 0.126 | 0.126 | 0.153 | 1.00 | 0.029 |
| clouds_composite | 0.105 | 0.105 | 0.108 | 1.00 | 0.002 |
| bin_unpacking | 0.098 | 0.098 | 0.119 | 1.00 | 0.020 |
| tonemapping | 0.087 | 0.087 | 0.091 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.078 | 0.078 | 0.154 | 1.00 | 0.009 |
| clustering | 0.063 | 0.063 | 0.067 | 1.00 | 0.020 |
| clouds_trace | 0.056 | 0.056 | 0.075 | 1.00 | 0.001 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.415 | 0.12 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 1.00 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.041 | 0.056 | 0.057 | 0.73 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.028 | 0.028 | 0.036 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.028 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| night_sky | 0.016 | 0.017 | 0.039 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.025 | 1.00 | 0.127 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 1.00 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.012 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.089 | 13.320 | 24.46 | 58101.2 |
| render/busy/ms | 9.583 | 12.785 | 23.65 | 55190.5 |
| main/busy/ms | 9.341 | 11.323 | 25.47 | 53797.2 |
| stage/render/render | 6.115 | 8.584 | 15.82 | 35215.4 |
| stage/main/post_update | 5.878 | 7.465 | 10.56 | 33850.0 |
| render/graph/ms | 5.419 | 7.810 | 14.68 | 31208.0 |
| encode//sum | 4.932 | 7.451 | 25.73 | 28404.1 |
| stage/render/graph_submit | 4.044 | 5.753 | 10.35 | 23287.5 |
| encode/submit | 2.580 | 3.823 | 6.17 | 14858.9 |
| stage/main/update | 2.183 | 2.621 | 19.41 | 12571.9 |
| encode/pass:shadow_point_light/sum | 1.641 | 5.071 | 16.16 | 9447.7 |
| encode/finish_wall | 1.495 | 2.090 | 4.67 | 8611.5 |
| stage/render/camera_driver | 1.321 | 2.063 | 4.25 | 7605.5 |
| stage/render/prepare_write_phase_buffers | 0.814 | 1.153 | 2.58 | 4689.1 |
| encode/pass:shadow_directional_light/sum | 0.792 | 1.315 | 2.19 | 4563.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.754 | 1.120 | 1.90 | 4340.1 |
| encode/pass:early prepass/sum | 0.747 | 1.125 | 1.81 | 4300.5 |
| stage/render/prepare_assets | 0.607 | 0.755 | 9.03 | 3494.4 |
| bigspace/total/ms | 0.557 | 1.406 | 2.85 | 3209.1 |
| stage/render/prepare_resources | 0.470 | 0.563 | 1.25 | 2706.2 |
| stage/render/prepare_meshes | 0.442 | 0.568 | 2.25 | 2548.0 |
| render/present/ms | 0.412 | 0.497 | 2.03 | 2373.8 |
| stage/main/first | 0.393 | 0.521 | 5.77 | 2265.0 |
| stage/main/pre_update | 0.368 | 0.506 | 1.67 | 2121.6 |
| terrain/update/ms | 0.322 | 0.429 | 1.85 | 1855.9 |
| controller/feed/ms | 0.296 | 0.374 | 0.81 | 1705.0 |
| terrain/select/ms | 0.288 | 0.386 | 1.00 | 1658.2 |
| stage/render/prepare_bind_groups | 0.281 | 0.497 | 2.44 | 1620.5 |
| stage/render/specialize | 0.280 | 0.416 | 2.48 | 1612.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.260 | 0.386 | 0.97 | 1500.0 |
| stage/render/queue | 0.260 | 0.404 | 2.95 | 1495.3 |
| stream/drive/ms | 0.254 | 0.341 | 16.98 | 1465.6 |
| bigspace/low_precision/ms | 0.236 | 0.737 | 2.23 | 1360.5 |
| stage/main/fixed_main_loop | 0.228 | 0.373 | 1.02 | 1314.0 |
| stage/render/create_views | 0.201 | 0.259 | 0.88 | 1158.5 |
| far/select/ms | 0.198 | 0.272 | 0.56 | 1139.6 |
| stream/tick/ms | 0.196 | 0.268 | 0.56 | 1128.4 |
| stage/render/prepare_batch_phases | 0.189 | 0.269 | 1.07 | 1089.5 |
| stage/render/extract_commands | 0.183 | 0.213 | 0.46 | 1051.2 |
| anim/evaluate/ms | 0.169 | 0.264 | 1.82 | 976.0 |
| stage/render/prepare_flush | 0.168 | 0.319 | 1.29 | 965.0 |
| anim/evaluate_cpu/ms | 0.167 | 0.207 | 0.71 | 962.6 |
| bigspace/high_precision/ms | 0.154 | 0.666 | 1.84 | 887.9 |
| stage/render/prepare_views | 0.137 | 0.169 | 0.48 | 786.8 |
| controller/move/ms | 0.134 | 0.261 | 0.66 | 771.8 |
| stage/main/last | 0.115 | 0.159 | 1.60 | 661.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.112 | 0.185 | 0.73 | 645.3 |
| render/acquire/ms | 0.093 | 0.118 | 0.37 | 536.9 |
| stage/render/prepare_collect_phase_buffers | 0.065 | 0.139 | 0.52 | 373.0 |
| physics/step/ms | 0.051 | 0.089 | 0.59 | 292.5 |
| physics/insert/ms | 0.038 | 0.391 | 1.02 | 216.0 |
| stage/render/phase_sort | 0.033 | 0.055 | 0.29 | 189.1 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.55 | 179.8 |
| stage/render/cleanup | 0.030 | 0.089 | 0.92 | 173.9 |
| pipelines/compile_ms | 0.017 | 0.000 | 87.71 | 99.7 |
| pipelines/front_ms | 0.017 | 0.000 | 84.93 | 96.7 |
| weather/map/ms | 0.016 | 0.021 | 0.14 | 93.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.52 | 88.6 |
| anim/apply/ms | 0.012 | 0.021 | 0.44 | 71.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.57 | 65.9 |
| controller/avoid/ms | 0.011 | 0.015 | 0.09 | 62.6 |
| weather/eval/ms | 0.010 | 0.013 | 0.20 | 57.6 |
| script/update/ms | 0.008 | 0.009 | 0.13 | 44.4 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 32.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.17 | 31.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.13 | 28.9 |
| script/ui/ms | 0.004 | 0.005 | 0.12 | 22.8 |
| controller/write/ms | 0.004 | 0.005 | 0.03 | 21.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 14.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.43 | 4.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.78 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 20 |
| controller/movers/count | count | 20 | 23 | 20 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1269 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 146 | 155 | 149 |
| crowd/c1/joints/count | count | 5539 | 5936 | 5661 |
| crowd/c1/meshes/count | count | 1170 | 1268 | 1197 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 205 |
| crowd/joints_extracted/count | count | 3709 | 7863 | 5775 |
| crowd/joints_uploaded/count | count | 3690 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.45 | 1.79 | 0.41 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.52 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.57 | 0.01 |
| encode/finish_critical | ms | 0.99 | 2.19 | 1.35 |
| encode/pass:early prepass/max | ms | 0.78 | 1.81 | 0.96 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.90 | 0.56 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.73 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.97 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.19 | 1.35 |
| encode/pass:shadow_point_light/max | ms | 0.83 | 1.46 | 0.73 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.55 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.61 | 25.47 | 8.34 |
| gpu/timed/ms | ms | 7.48 | 10.82 | 7.72 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 198 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 794 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 198 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 794 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1296 | 1399 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3354 | 5134 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1296 | 1399 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3354 | 5134 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 194 | 19 |
| phases/shadow/bins | count | 3857 | 4588 | 1494 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6300 | 7713 | 3038 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 237 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 190 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 549 | 37 |
| preprocess/bind_groups_created | count | 13 | 549 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 277 | 127 |
| process/cpu_usage | % | 25.96 | 42.31 | 27.40 |
| process/mem_usage | GiB | 5.42 | 5.59 | 5.58 |
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
| skin/blocks_changed/count | count | 14 | 93 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 279.0 k | 809.2 k | 443.4 k |
| skin/joints_written/count | count | 4360 | 12.6 k | 6928 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4922 | 5012 | 5012 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 431 | 528 | 396 |
| statics/materials_bound | count | 431 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.90 | 46.47 | 31.52 |
| system/mem_usage | % | 54.23 | 54.64 | 54.61 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 71 | 243 | 71 |
| views/all/bins | count | 6325 | 7503 | 5002 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 20.6 k | 10.7 k |
| views/all/pipelines | count | 249 | 286 | 67 |
| views/all/pipelines_distinct | count | 70 | 75 | 63 |
| views/all/work_items | count | 10.7 k | 18.2 k | 11.0 k |
| views/cascade/batch_sets | count | 9 | 27 | 19 |
| views/cascade/bins | count | 1454 | 1549 | 1494 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3398 | 4468 | 3038 |
| views/cascade/pipelines | count | 23 | 27 | 19 |
| views/cascade/work_items | count | 324 | 4464 | 3038 |
| views/lamp_face/batch_sets | count | 0 | 186 | 0 |
| views/lamp_face/bins | count | 2547 | 3063 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2819 | 3389 | 0 |
| views/lamp_face/pipelines | count | 178 | 210 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2797 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2890 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8306 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8465 | 13.3 k | 7922 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 64.61 | 251 | 372.1 k |
| anim/events/count | count | 0.09 | 3 | 527 |
| assets/Image/added | count | 0.10 | 36 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.71 | 152 | 4061 |
| assets/Mesh/removed | count | 0.84 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.76 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.81 | 1296 | 4675 |
| encode//count | count | 39.65 | 219 | 228.3 k |
| encode/bind_walk/init_textures | count | 16.0 k | 112.9 k | 92.12 M |
| encode/bind_walk/init_textures_skipped | count | 58.9 k | 141.5 k | 339.40 M |
| encode/bind_walk/inits | count | 478 | 1109 | 2.75 M |
| encode/bind_walk/inits_skipped | count | 103 | 145 | 595.6 k |
| encode/bind_walk/merge_textures | count | 15.9 k | 112.9 k | 91.75 M |
| encode/bind_walk/merge_textures_skipped | count | 5392 | 6012 | 31.05 M |
| encode/bind_walk/merges | count | 107 | 336 | 618.0 k |
| encode/bind_walk/merges_skipped | count | 81.73 | 123 | 470.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5759 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5759 |
| encode/pass:early prepass/count | count | 1 | 1 | 5759 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5759 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5759 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5759 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5759 |
| encode/pass:shadow_point_light/count | count | 3.17 | 48 | 18.2 k |
| encode/queue/command_buffers | count | 52.83 | 277 | 304.3 k |
| encode/queue/submits | count | 2.02 | 4 | 11.6 k |
| encode/queue/write_buffer/bytes | bytes | 1.82 M | 2.73 M | 10.49 G |
| encode/queue/write_buffer/calls | count | 272 | 820 | 1.57 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.0 k | 3.53 M | 979.27 M |
| encode/queue/write_buffer_with/calls | count | 11.10 | 465 | 63.9 k |
| encode/queue/write_texture/bytes | bytes | 11.1 k | 8.39 M | 64.12 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5759 |
| physics/inserts | count | 0.39 | 105 | 2271 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.68 | 2 | 3939 |
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
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 24 | 378 |
| stream/upload/bytes | bytes | 5878 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1289 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.4 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.40 | 2401.0 | main/busy/ms 2372, stage/main/post_update 2333, stage/main/update 31.09, stage/main/pre_update 4.26, stage/main/last 1.08, surface/advance_cpu_ms 0.88, stream/drive/ms 0.51, stage/main/first 0.47, stage/main/fixed_main_loop 0.33, surface/cell_upload_ms 0.28, stream/tick/ms 0.11, bigspace/total/ms 0.10 |
| warm-up | 3.21 | 808.2 | main/busy/ms 2547, stage/main/post_update 2356, render/thread/ms 778, render/busy/ms 610, stage/render/prepare_assets 551, stage/main/update 177, stage/render/prepare_views 163, render/acquire/ms 163, stage/render/create_views 28.84, stage/render/render 26.67, render/graph/ms 19.88, stage/render/camera_driver 18.11 |
| warm-up | 3.31 | 103.7 | render/thread/ms 865, render/busy/ms 647, stage/render/prepare_assets 575, main/busy/ms 194, stage/render/prepare_views 163, render/acquire/ms 163, stage/main/update 150, stage/render/render 81.29, pipelines/compile_ms 55.57, render/present/ms 55.11, stage/main/post_update 34.12, pipelines/driver_ms 33.70 |
| warm-up | 3.35 | 41.9 | pipelines/compile_ms 342, pipelines/front_ms 255, render/thread/ms 118, pipelines/driver_ms 86.49, stage/render/render 78.78, pipelines/wait_ms 59.61, render/busy/ms 58.96, main/busy/ms 58.95, render/present/ms 58.52, stage/main/post_update 43.57, terrain/update/ms 25.13, stage/render/prepare_assets 24.88 |
| warm-up | 3.44 | 90.1 | pipelines/wait_ms 3947, pipelines/compile_ms 1332, pipelines/front_ms 1218, pipelines/driver_ms 114, render/thread/ms 112, render/busy/ms 101, main/busy/ms 75.25, stage/main/post_update 43.21, stage/render/render 37.87, render/graph/ms 27.18, stage/render/prepare_resources 22.86, stage/render/prepare_assets 21.78 |
| warm-up | 3.51 | 60.2 | pipelines/wait_ms 4405, pipelines/compile_ms 1385, pipelines/front_ms 1311, render/thread/ms 140, render/busy/ms 138, main/busy/ms 135, stage/main/update 76.00, pipelines/driver_ms 74.28, stage/render/prepare_resources 44.98, stage/render/prepare_assets 38.86, stage/main/pre_update 24.54, stage/main/post_update 20.73 |
| warm-up | 3.61 | 103.1 | pipelines/compile_ms 1335, pipelines/front_ms 1256, pipelines/wait_ms 606, main/busy/ms 119, render/thread/ms 115, render/busy/ms 112, pipelines/driver_ms 78.98, stage/main/update 67.22, stage/render/prepare_assets 62.88, stage/render/prepare_resources 24.32, stage/main/pre_update 19.60, stage/main/post_update 18.03 |
| warm-up | 3.65 | 40.7 | pipelines/compile_ms 1724, pipelines/front_ms 1592, pipelines/wait_ms 314, pipelines/driver_ms 132, render/thread/ms 95.18, render/busy/ms 92.36, stage/render/prepare_assets 74.20, main/busy/ms 34.13, stage/main/post_update 15.47, stage/render/render 13.76, stage/main/update 12.38, render/graph/ms 10.63 |
| warm-up | 3.68 | 30.9 | pipelines/compile_ms 1035, pipelines/front_ms 958, pipelines/wait_ms 249, pipelines/driver_ms 77.27, render/thread/ms 68.22, render/busy/ms 65.53, main/busy/ms 42.47, stage/render/prepare_assets 40.53, stage/render/render 17.40, stage/main/post_update 16.19, render/graph/ms 15.52, stage/render/graph_submit 13.39 |
| warm-up | 3.71 | 28.1 | pipelines/compile_ms 750, pipelines/front_ms 720, render/thread/ms 56.44, render/busy/ms 53.68, main/busy/ms 44.69, pipelines/wait_ms 38.93, pipelines/driver_ms 29.89, stage/render/render 20.20, stage/main/post_update 20.13, encode//sum 18.76, render/graph/ms 17.97, stage/render/graph_submit 13.52 |
| warm-up | 3.76 | 37.7 | main/busy/ms 70.69, stage/main/update 56.49, stream/drive/ms 52.69, render/thread/ms 51.41, render/busy/ms 49.33, pipelines/compile_ms 46.72, pipelines/front_ms 44.33, stage/render/prepare_assets 26.76, stage/render/render 14.64, render/graph/ms 12.40, stage/main/post_update 10.37, stage/render/graph_submit 9.88 |
| warm-up | 3.83 | 68.2 | pipelines/compile_ms 1360, pipelines/front_ms 1313, main/busy/ms 69.54, render/thread/ms 68.84, render/busy/ms 67.12, stage/main/update 57.00, stream/drive/ms 52.51, stage/render/prepare_assets 47.92, pipelines/driver_ms 46.56, stage/render/render 12.23, render/graph/ms 10.24, stage/main/post_update 8.25 |
| warm-up | 3.86 | 29.3 | pipelines/compile_ms 1568, pipelines/front_ms 1515, render/thread/ms 59.98, render/busy/ms 58.36, pipelines/driver_ms 53.17, main/busy/ms 36.62, stage/render/prepare_assets 29.34, stage/main/update 24.00, stage/render/render 23.11, render/graph/ms 21.19, stream/drive/ms 10.61, stage/render/graph_submit 8.08 |
| warm-up | 3.91 | 28.0 | pipelines/compile_ms 128, pipelines/front_ms 118, render/thread/ms 42.47, main/busy/ms 41.37, render/busy/ms 41.04, stage/main/update 29.49, stream/drive/ms 25.27, stage/render/prepare_assets 24.57, pipelines/wait_ms 14.69, pipelines/lock_ms 14.14, stage/render/render 11.45, render/graph/ms 9.72 |
| warm-up | 3.97 | 55.7 | pipelines/compile_ms 487, pipelines/front_ms 471, render/thread/ms 66.12, render/busy/ms 64.15, main/busy/ms 52.64, stage/main/update 32.26, stream/drive/ms 27.13, stage/render/prepare_assets 21.52, pipelines/driver_ms 15.73, stage/render/render 15.68, render/graph/ms 13.32, stage/main/post_update 13.19 |
| warm-up | 4.01 | 47.1 | pipelines/compile_ms 496, pipelines/front_ms 480, render/thread/ms 72.57, render/busy/ms 70.37, main/busy/ms 36.64, stage/render/prepare_assets 21.88, stage/render/render 20.24, render/graph/ms 17.40, pipelines/driver_ms 15.95, stage/main/post_update 15.13, stage/render/graph_submit 13.97, stage/main/update 12.13 |
| warm-up | 4.10 | 83.4 | pipelines/compile_ms 1003, pipelines/front_ms 953, render/thread/ms 89.00, render/busy/ms 87.09, stage/render/prepare_assets 61.43, main/busy/ms 60.92, pipelines/driver_ms 49.70, stage/main/update 40.93, stage/render/render 16.37, render/graph/ms 14.01, stage/main/post_update 12.52, stage/render/graph_submit 11.49 |
| warm-up | 4.18 | 76.8 | pipelines/compile_ms 1957, pipelines/front_ms 1868, render/thread/ms 108, render/busy/ms 106, pipelines/driver_ms 88.66, stage/render/prepare_assets 81.63, main/busy/ms 69.89, stage/main/update 47.60, pipelines/wait_ms 28.03, stage/main/post_update 13.09, stage/render/render 11.76, render/graph/ms 9.80 |
| warm-up | 4.23 | 54.6 | pipelines/compile_ms 1304, pipelines/front_ms 1253, pipelines/wait_ms 168, render/thread/ms 95.95, render/busy/ms 94.23, main/busy/ms 60.23, pipelines/driver_ms 51.48, stage/render/prepare_assets 45.62, encode//sum 43.57, stage/render/render 22.10, stage/main/update 21.53, render/graph/ms 19.06 |
| warm-up | 4.36 | 134.4 | pipelines/compile_ms 2165, pipelines/front_ms 2099, pipelines/wait_ms 898, render/thread/ms 175, render/busy/ms 174, main/busy/ms 87.47, pipelines/driver_ms 65.42, stage/main/post_update 53.98, encode//sum 48.48, stage/render/prepare_write_phase_buffers 44.35, stage/render/prepare_meshes 27.80, stage/render/prepare_resources 24.53 |
| warm-up | 4.47 | 103.8 | pipelines/compile_ms 3013, pipelines/front_ms 2893, pipelines/wait_ms 1397, render/thread/ms 229, render/busy/ms 223, pipelines/driver_ms 120, main/busy/ms 81.00, encode//sum 66.30, stage/render/prepare_write_phase_buffers 63.60, stage/main/post_update 48.30, stage/render/prepare_resources 46.56, stage/render/render 32.95 |
| warm-up | 4.52 | 53.9 | pipelines/compile_ms 2057, pipelines/front_ms 1964, pipelines/wait_ms 954, render/thread/ms 139, render/busy/ms 133, pipelines/driver_ms 92.79, encode//sum 68.74, main/busy/ms 46.38, stage/render/render 36.51, render/graph/ms 34.05, stage/main/update 27.15, stage/render/graph_submit 26.40 |
| warm-up | 4.58 | 59.5 | pipelines/compile_ms 1951, pipelines/front_ms 1891, pipelines/wait_ms 997, render/thread/ms 79.30, render/busy/ms 77.91, main/busy/ms 76.02, pipelines/driver_ms 59.28, stage/main/update 55.86, stage/render/prepare_assets 44.57, stream/drive/ms 44.20, stage/render/render 19.37, encode//sum 17.27 |
| warm-up | 4.64 | 62.6 | pipelines/compile_ms 1401, pipelines/front_ms 1356, pipelines/wait_ms 737, main/busy/ms 102, render/thread/ms 81.99, render/busy/ms 80.25, stage/main/update 59.92, stream/drive/ms 45.67, pipelines/driver_ms 45.42, stage/render/prepare_assets 33.40, stage/render/render 31.71, render/graph/ms 29.24 |
| warm-up | 4.70 | 51.9 | pipelines/compile_ms 465, pipelines/front_ms 445, main/busy/ms 154, stage/main/update 107, render/thread/ms 81.56, render/busy/ms 78.11, pipelines/wait_ms 61.02, stream/drive/ms 58.16, stage/render/render 40.89, render/graph/ms 36.66, stage/main/post_update 32.83, stage/render/graph_submit 21.38 |
| warm-up | 4.85 | 153.7 | pipelines/compile_ms 2407, pipelines/front_ms 2345, main/busy/ms 135, stage/main/update 108, render/thread/ms 99.10, render/busy/ms 95.91, pipelines/driver_ms 62.21, stream/drive/ms 57.49, stage/render/prepare_assets 47.96, stage/render/render 29.19, render/graph/ms 25.37, pipelines/wait_ms 21.92 |
| warm-up | 4.88 | 32.3 | pipelines/compile_ms 2571, pipelines/front_ms 2503, main/busy/ms 89.74, render/thread/ms 87.24, render/busy/ms 85.03, stage/main/update 68.53, pipelines/driver_ms 67.18, stream/drive/ms 46.04, stage/render/prepare_assets 44.64, stage/render/render 26.30, render/graph/ms 23.34, stage/render/graph_submit 17.54 |
| warm-up | 4.96 | 77.6 | pipelines/compile_ms 303, pipelines/front_ms 291, main/busy/ms 76.45, stage/main/update 60.23, render/thread/ms 57.43, render/busy/ms 55.50, stream/drive/ms 45.65, stage/render/prepare_assets 22.73, stage/render/render 18.70, render/graph/ms 15.86, pipelines/driver_ms 12.54, encode//sum 10.39 |
| warm-up | 5.04 | 56.7 | pipelines/compile_ms 871, pipelines/front_ms 841, main/busy/ms 61.39, render/thread/ms 46.61, stage/main/update 45.47, render/busy/ms 44.66, stream/drive/ms 33.87, pipelines/driver_ms 30.08, stage/render/render 18.97, render/graph/ms 16.46, stage/render/graph_submit 11.48, encode//sum 10.96 |
| warm-up | 5.10 | 30.5 | pipelines/compile_ms 473, pipelines/front_ms 457, render/thread/ms 52.92, render/busy/ms 51.29, main/busy/ms 35.62, stage/render/prepare_assets 17.86, stage/render/render 17.69, pipelines/driver_ms 16.52, render/graph/ms 15.42, encode//sum 13.69, stage/main/update 13.53, stage/main/post_update 13.47 |
| warm-up | 5.14 | 43.5 | pipelines/compile_ms 125, pipelines/front_ms 120, render/thread/ms 66.48, render/busy/ms 65.21, stage/render/prepare_assets 40.16, main/busy/ms 30.75, stage/render/render 13.61, stage/main/post_update 12.15, stage/main/update 11.89, render/graph/ms 11.58, encode//sum 10.95, stage/render/graph_submit 7.82 |
| warm-up | 5.17 | 28.4 | pipelines/compile_ms 193, pipelines/front_ms 189, render/thread/ms 63.07, render/busy/ms 60.94, main/busy/ms 30.62, stage/render/prepare_assets 30.48, stage/render/render 16.40, render/graph/ms 13.80, stage/main/post_update 12.97, stage/main/update 11.41, encode//sum 10.29, stage/render/graph_submit 8.95 |
| warm-up | 5.21 | 44.1 | pipelines/compile_ms 313, pipelines/front_ms 304, render/thread/ms 61.32, render/busy/ms 58.83, main/busy/ms 57.82, stage/main/post_update 26.26, stage/main/update 23.55, stage/render/render 23.37, render/graph/ms 20.53, stage/render/prepare_assets 15.75, stream/drive/ms 13.82, stage/render/graph_submit 13.20 |
| warm-up | 5.26 | 44.4 | pipelines/compile_ms 1470, pipelines/front_ms 1425, render/thread/ms 71.41, render/busy/ms 69.21, main/busy/ms 66.47, pipelines/driver_ms 44.78, stage/main/post_update 30.27, stage/main/update 28.27, stage/render/prepare_assets 24.50, stage/render/render 24.15, render/graph/ms 21.36, stream/drive/ms 16.96 |
| warm-up | 5.29 | 29.9 | pipelines/compile_ms 1404, pipelines/front_ms 1360, render/thread/ms 57.74, render/busy/ms 55.92, pipelines/driver_ms 43.90, main/busy/ms 39.07, stage/render/prepare_assets 26.74, stage/main/update 17.44, stage/render/render 15.57, stage/main/post_update 15.57, pipelines/wait_ms 14.46, render/graph/ms 13.18 |
| warm-up | 5.33 | 41.1 | pipelines/compile_ms 261, pipelines/front_ms 253, render/thread/ms 56.37, render/busy/ms 55.14, stage/render/prepare_assets 35.37, main/busy/ms 30.14, stage/main/update 13.07, stage/main/post_update 11.51, stage/render/render 10.31, encode//sum 9.86, render/graph/ms 8.68, pipelines/driver_ms 7.30 |
| warm-up | 5.39 | 40.7 | pipelines/compile_ms 568, pipelines/front_ms 556, render/thread/ms 51.54, render/busy/ms 50.28, main/busy/ms 32.98, stage/render/prepare_assets 26.58, stage/main/update 17.20, stage/render/render 13.93, render/graph/ms 12.00, pipelines/driver_ms 11.07, stage/main/post_update 10.63, encode//sum 9.92 |
| warm-up | 5.44 | 40.5 | pipelines/compile_ms 175, pipelines/front_ms 172, render/thread/ms 36.74, render/busy/ms 35.70, main/busy/ms 29.28, pipelines/wait_ms 25.44, stage/render/prepare_assets 17.48, stage/main/update 15.13, stage/render/render 10.06, encode//sum 9.89, stage/main/post_update 9.15, stream/drive/ms 8.85 |
| warm-up | 5.50 | 31.1 | render/thread/ms 48.25, render/busy/ms 46.78, main/busy/ms 28.85, stage/render/render 18.30, stage/render/prepare_assets 16.52, render/graph/ms 16.18, stage/main/update 12.73, stage/main/post_update 11.41, stage/render/graph_submit 11.36, encode//sum 11.26, encode/submit 7.89, encode/pass:shadow_point_light/sum 5.23 |
| warm-up | 5.74 | 28.5 | pipelines/compile_ms 192, pipelines/front_ms 189, main/busy/ms 35.47, render/thread/ms 28.48, render/busy/ms 27.19, stage/main/update 21.24, stream/drive/ms 15.38, stage/render/render 13.10, render/graph/ms 11.39, encode//sum 9.83, stage/main/post_update 9.23, stage/render/graph_submit 8.00 |
| warm-up | 6.11 | 29.9 | render/thread/ms 36.61, render/busy/ms 35.51, main/busy/ms 23.05, stage/render/prepare_assets 15.20, stage/render/render 10.17, stage/main/post_update 9.21, encode//sum 8.86, render/graph/ms 8.67, stage/main/update 8.24, stage/render/graph_submit 5.92, encode/pass:shadow_point_light/sum 5.07, encode/submit 3.73 |
| warm-up | 6.34 | 29.2 | pipelines/compile_ms 152, pipelines/front_ms 148, render/thread/ms 41.55, render/busy/ms 40.08, main/busy/ms 39.59, stage/render/render 19.30, stage/main/update 17.03, render/graph/ms 16.69, stage/main/post_update 15.98, encode//sum 15.57, stage/render/graph_submit 10.74, encode/pass:shadow_point_light/sum 9.29 |
| warm-up | 6.51 | 57.7 | render/thread/ms 72.81, render/busy/ms 71.73, stage/render/prepare_assets 54.26, main/busy/ms 22.18, stage/main/post_update 10.52, stage/render/render 10.22, encode//sum 9.12, render/graph/ms 8.60, stage/main/update 7.98, encode/pass:shadow_point_light/sum 6.22, stage/render/graph_submit 5.73, encode/submit 3.28 |
| warm-up | 6.54 | 27.6 | pipelines/compile_ms 135, pipelines/front_ms 132, render/thread/ms 82.18, render/busy/ms 80.88, stage/render/prepare_assets 58.81, main/busy/ms 31.53, stage/main/post_update 14.02, stage/main/update 12.96, stage/render/render 11.88, encode//sum 10.53, render/graph/ms 9.89, stage/render/graph_submit 6.83 |
| warm-up | 6.63 | 39.1 | main/busy/ms 60.31, render/thread/ms 54.83, render/busy/ms 53.19, stage/main/update 38.66, stage/render/prepare_assets 25.08, stream/drive/ms 23.61, stage/render/render 15.31, stage/main/post_update 15.05, render/graph/ms 12.87, encode//sum 12.25, stage/render/graph_submit 7.66, encode/pass:shadow_point_light/sum 7.45 |
| warm-up | 6.69 | 61.9 | render/thread/ms 69.99, render/busy/ms 68.58, main/busy/ms 59.23, stage/render/prepare_assets 45.93, stage/main/update 39.61, stream/drive/ms 23.74, stage/render/render 13.99, stage/main/post_update 13.87, render/graph/ms 12.01, encode//sum 10.18, stage/render/graph_submit 7.72, encode/pass:shadow_point_light/sum 6.72 |
| warm-up | 6.95 | 29.0 | main/busy/ms 39.66, render/thread/ms 28.86, render/busy/ms 27.80, stage/main/update 23.70, pipelines/compile_ms 20.38, pipelines/front_ms 19.59, stage/render/render 12.58, stage/main/post_update 11.02, render/graph/ms 10.99, encode//sum 9.42, stage/render/graph_submit 7.77, stage/render/prepare_assets 6.77 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 0%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **69%** (min 42%) · 2625 MHz (min 2625) · memory 8101 MHz · 73.4 W · 75 °C (max 81) · VRAM 6756 MiB · throttled 100% of samples

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
