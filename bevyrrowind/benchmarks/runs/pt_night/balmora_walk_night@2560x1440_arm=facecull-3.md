# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.face_cull=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6067 frames over 65.6 s · **median 10.36 ms (97 fps)** · mean 10.82 · p95 13.77 · p99 15.83 (1% low 63 fps) · worst 29.78 ms · 71 spikes over 1.5× median · 22.8 stutters/min (work 18.3/min)
- warm-up: 323 frames over 9.3 s, worst 2380.8 ms
- hitches (> 25.9 ms): **5** measured, 54 in warm-up
- timed GPU passes: 6.71 ms of a 10.82 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.36 ms, limit 11 ms
- **FAIL** frame_ms: 25 stutters (> 1.5x moving median of 60) in 1.09 min = 22.85/min, limit 1/min
- pass worst frame 29.78 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.022 | 2.030 | 2.063 | 1.00 | 0.174 |
| main_opaque_pass_3d | 1.314 | 1.319 | 1.669 | 1.00 | 0.072 |
| early prepass | 0.496 | 0.498 | 0.718 | 1.00 | 0.088 |
| bloom | 0.315 | 0.316 | 0.322 | 1.00 | 0.062 |
| atmosphere_luts | 0.307 | 0.309 | 0.372 | 1.00 | 0.003 |
| water_velocity | 0.262 | 0.263 | 0.350 | 1.00 | 0.002 |
| auxiliary_views | 0.215 | 0.216 | 0.942 | 1.00 | 0.204 |
| water_sspr | 0.200 | 0.201 | 0.203 | 1.00 | 0.002 |
| ssao | 0.187 | 0.188 | 0.193 | 1.00 | 0.007 |
| upscaling | 0.129 | 0.130 | 0.136 | 1.00 | 0.004 |
| water_fft | 0.128 | 0.129 | 0.137 | 1.00 | 0.003 |
| atmos_froxel_inject | 0.127 | 0.127 | 0.128 | 1.00 | 0.001 |
| uniform_allocation | 0.125 | 0.125 | 0.153 | 1.00 | 0.027 |
| clouds_composite | 0.104 | 0.105 | 0.108 | 1.00 | 0.002 |
| bin_unpacking | 0.096 | 0.097 | 0.119 | 1.00 | 0.019 |
| tonemapping | 0.086 | 0.087 | 0.091 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.079 | 0.079 | 0.156 | 1.00 | 0.009 |
| clustering | 0.064 | 0.064 | 0.068 | 1.00 | 0.020 |
| clouds_trace | 0.056 | 0.056 | 0.073 | 1.00 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.410 | 0.415 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.028 | 0.028 | 0.037 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.230 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| night_sky | 0.016 | 0.017 | 0.040 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 1.00 | 0.126 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 1.00 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.012 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.459 | 12.410 | 25.26 | 57388.7 |
| main/busy/ms | 9.064 | 11.032 | 28.30 | 54992.1 |
| render/busy/ms | 8.949 | 11.867 | 24.42 | 54296.2 |
| stage/render/render | 5.664 | 7.959 | 16.96 | 34362.0 |
| stage/main/post_update | 5.632 | 7.262 | 10.12 | 34167.6 |
| render/graph/ms | 4.968 | 7.198 | 15.62 | 30141.6 |
| encode//sum | 4.216 | 6.180 | 28.93 | 25580.1 |
| stage/render/graph_submit | 3.710 | 5.270 | 10.92 | 22506.2 |
| encode/submit | 2.364 | 3.568 | 6.14 | 14339.8 |
| stage/main/update | 2.169 | 2.650 | 21.83 | 13162.1 |
| encode/finish_wall | 1.378 | 1.845 | 5.13 | 8358.6 |
| stage/render/camera_driver | 1.206 | 1.893 | 4.61 | 7317.4 |
| encode/pass:shadow_point_light/sum | 1.145 | 4.656 | 14.36 | 6948.4 |
| encode/pass:shadow_directional_light/sum | 0.791 | 1.264 | 2.34 | 4800.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.775 | 1.107 | 1.87 | 4699.4 |
| encode/pass:early prepass/sum | 0.761 | 1.092 | 2.03 | 4618.5 |
| stage/render/prepare_write_phase_buffers | 0.750 | 1.056 | 2.62 | 4549.0 |
| stage/render/prepare_assets | 0.586 | 0.738 | 9.95 | 3555.7 |
| bigspace/total/ms | 0.557 | 1.364 | 2.23 | 3379.1 |
| stage/render/prepare_resources | 0.459 | 0.551 | 1.34 | 2787.7 |
| stage/render/prepare_meshes | 0.428 | 0.546 | 2.20 | 2596.1 |
| render/present/ms | 0.415 | 0.510 | 2.14 | 2515.9 |
| stage/main/first | 0.399 | 0.516 | 5.78 | 2418.6 |
| stage/main/pre_update | 0.356 | 0.496 | 1.15 | 2157.7 |
| terrain/update/ms | 0.317 | 0.412 | 2.53 | 1920.6 |
| terrain/select/ms | 0.282 | 0.370 | 1.02 | 1711.9 |
| controller/feed/ms | 0.270 | 0.338 | 0.87 | 1638.8 |
| stage/render/prepare_bind_groups | 0.261 | 0.437 | 2.20 | 1585.2 |
| stage/render/specialize | 0.261 | 0.396 | 2.17 | 1582.7 |
| stream/drive/ms | 0.250 | 0.336 | 19.92 | 1519.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.246 | 0.364 | 1.58 | 1491.8 |
| stage/render/queue | 0.245 | 0.380 | 2.73 | 1485.0 |
| bigspace/low_precision/ms | 0.229 | 0.697 | 1.89 | 1392.2 |
| stage/main/fixed_main_loop | 0.217 | 0.373 | 1.08 | 1314.5 |
| far/select/ms | 0.201 | 0.271 | 0.61 | 1217.5 |
| stage/render/create_views | 0.196 | 0.253 | 0.86 | 1186.8 |
| stream/tick/ms | 0.193 | 0.260 | 0.72 | 1172.8 |
| stage/render/prepare_batch_phases | 0.189 | 0.267 | 0.67 | 1146.5 |
| stage/render/extract_commands | 0.176 | 0.205 | 0.46 | 1065.3 |
| anim/evaluate/ms | 0.170 | 0.286 | 1.96 | 1031.4 |
| stage/render/prepare_flush | 0.166 | 0.335 | 0.85 | 1009.6 |
| bigspace/high_precision/ms | 0.161 | 0.676 | 1.90 | 979.0 |
| anim/evaluate_cpu/ms | 0.160 | 0.203 | 0.72 | 972.8 |
| stage/render/prepare_views | 0.138 | 0.173 | 0.51 | 840.2 |
| controller/move/ms | 0.126 | 0.239 | 0.53 | 767.3 |
| stage/main/last | 0.115 | 0.156 | 1.85 | 694.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.103 | 0.169 | 0.70 | 627.3 |
| render/acquire/ms | 0.095 | 0.123 | 0.40 | 576.7 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.131 | 0.47 | 370.7 |
| physics/step/ms | 0.047 | 0.087 | 0.63 | 285.7 |
| encode/resolve_diagnostics/sum | 0.038 | 0.082 | 0.72 | 230.3 |
| physics/insert/ms | 0.035 | 0.374 | 0.78 | 210.2 |
| stage/render/cleanup | 0.032 | 0.102 | 0.76 | 195.9 |
| stage/render/phase_sort | 0.032 | 0.054 | 0.25 | 194.7 |
| weather/map/ms | 0.016 | 0.021 | 0.15 | 98.3 |
| pipelines/compile_ms | 0.015 | 0.000 | 79.58 | 91.5 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.40 | 90.6 |
| pipelines/front_ms | 0.015 | 0.000 | 76.71 | 88.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.48 | 69.5 |
| anim/apply/ms | 0.011 | 0.022 | 0.40 | 68.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.19 | 63.7 |
| weather/eval/ms | 0.010 | 0.012 | 0.14 | 58.1 |
| script/update/ms | 0.007 | 0.009 | 0.08 | 44.5 |
| script/input/ms | 0.005 | 0.007 | 0.22 | 33.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.00 | 30.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 29.1 |
| script/ui/ms | 0.004 | 0.006 | 0.04 | 25.7 |
| controller/write/ms | 0.004 | 0.004 | 0.16 | 21.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 14.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.87 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 31 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1291 | 2262 | 649 |
| crowd/c0/meshes/count | count | 353 | 646 | 177 |
| crowd/c1/count | count | 145 | 157 | 147 |
| crowd/c1/joints/count | count | 5531 | 5999 | 5547 |
| crowd/c1/meshes/count | count | 1166 | 1297 | 1164 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 204 |
| crowd/joints_extracted/count | count | 3700 | 8335 | 5731 |
| crowd/joints_uploaded/count | count | 3651 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.42 | 2.01 | 0.36 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.40 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.48 | 0.02 |
| encode/finish_critical | ms | 0.97 | 2.34 | 1.21 |
| encode/pass:early prepass/max | ms | 0.80 | 2.03 | 0.78 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.87 | 1.21 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.70 | 0.12 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.58 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.34 | 0.96 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.49 | 0.84 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.72 | 0.05 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.14 | 28.30 | 8.38 |
| gpu/timed/ms | ms | 7.45 | 10.04 | 7.80 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1285 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3364 | 5101 | 3193 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1285 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3364 | 5101 | 3193 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 14 | 195 | 4 |
| phases/shadow/bins | count | 3843 | 4441 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6196 | 7374 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 193 | 227 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 552 | 9 |
| preprocess/bind_groups_created | count | 11 | 552 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 235 | 124 |
| process/cpu_usage | % | 26.92 | 40.19 | 28.37 |
| process/mem_usage | GiB | 5.40 | 5.56 | 5.56 |
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
| skin/blocks_changed/count | count | 13 | 111 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 281.3 k | 809.7 k | 453.1 k |
| skin/joints_written/count | count | 4396 | 12.7 k | 7080 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.91 | 48.03 | 29.68 |
| system/mem_usage | % | 54.27 | 54.78 | 54.72 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 63 | 246 | 56 |
| views/all/bins | count | 6326 | 7235 | 4989 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 20.2 k | 10.6 k |
| views/all/pipelines | count | 239 | 274 | 67 |
| views/all/pipelines_distinct | count | 71 | 75 | 63 |
| views/all/work_items | count | 10.6 k | 17.5 k | 8152 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1445 | 1521 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3405 | 4446 | 3025 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 330 | 4443 | 222 |
| views/lamp_face/batch_sets | count | 0 | 172 | 0 |
| views/lamp_face/bins | count | 2530 | 3020 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 24 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2789 | 3288 | 0 |
| views/lamp_face/pipelines | count | 169 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 7 | 0 |
| views/lamp_face/work_items | count | 0 | 2666 | 0 |
| views/main/batch_sets | count | 51 | 53 | 52 |
| views/main/bins | count | 2878 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8328 | 13.0 k | 7622 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8498 | 13.2 k | 7930 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 45.43 | 312 | 275.6 k |
| anim/events/count | count | 0.08 | 2 | 495 |
| assets/Image/added | count | 0.10 | 41 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.57 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1298 | 4675 |
| encode//count | count | 35.99 | 205 | 218.4 k |
| encode/bind_walk/init_textures | count | 13.8 k | 106.4 k | 83.83 M |
| encode/bind_walk/init_textures_skipped | count | 59.1 k | 124.3 k | 358.26 M |
| encode/bind_walk/inits | count | 461 | 1116 | 2.80 M |
| encode/bind_walk/inits_skipped | count | 103 | 144 | 626.5 k |
| encode/bind_walk/merge_textures | count | 13.8 k | 106.3 k | 83.44 M |
| encode/bind_walk/merge_textures_skipped | count | 5385 | 6012 | 32.67 M |
| encode/bind_walk/merges | count | 102 | 327 | 616.5 k |
| encode/bind_walk/merges_skipped | count | 81.52 | 124 | 494.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6067 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6067 |
| encode/pass:early prepass/count | count | 1 | 1 | 6067 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6067 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6067 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6067 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6067 |
| encode/pass:shadow_point_light/count | count | 2.26 | 44 | 13.7 k |
| encode/queue/command_buffers | count | 48.27 | 259 | 292.8 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.81 M | 2.99 M | 11.01 G |
| encode/queue/write_buffer/calls | count | 257 | 825 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 167.4 k | 3.53 M | 1.02 G |
| encode/queue/write_buffer_with/calls | count | 10.99 | 465 | 66.7 k |
| encode/queue/write_texture/bytes | bytes | 10.4 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6067 |
| physics/inserts | count | 0.37 | 94 | 2274 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
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
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5580 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1223 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.9 ms; slowest 59, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.38 | 2380.8 | main/busy/ms 2346, stage/main/post_update 2308, stage/main/update 30.37, stage/main/pre_update 4.69, stage/main/last 0.95, surface/advance_cpu_ms 0.79, stage/main/first 0.47, stage/main/fixed_main_loop 0.39, stream/drive/ms 0.39, surface/cell_upload_ms 0.31, stream/tick/ms 0.11, bigspace/total/ms 0.07 |
| warm-up | 3.13 | 747.1 | main/busy/ms 2486, stage/main/post_update 2335, render/thread/ms 704, render/busy/ms 536, stage/render/prepare_assets 486, stage/render/prepare_views 165, render/acquire/ms 165, stage/main/update 137, stage/render/create_views 28.36, stage/render/render 12.21, render/graph/ms 7.90, stage/main/pre_update 6.74 |
| warm-up | 3.24 | 109.2 | render/thread/ms 797, render/busy/ms 575, stage/render/prepare_assets 513, stage/render/prepare_views 166, render/acquire/ms 165, main/busy/ms 158, stage/main/update 111, stage/render/render 70.49, pipelines/compile_ms 59.20, render/present/ms 56.51, pipelines/driver_ms 44.49, stage/main/post_update 37.71 |
| warm-up | 3.42 | 172.2 | pipelines/wait_ms 4547, pipelines/compile_ms 1619, pipelines/front_ms 1485, render/thread/ms 148, render/busy/ms 144, main/busy/ms 142, pipelines/driver_ms 133, stage/main/update 70.92, stage/render/prepare_write_phase_buffers 33.45, stage/main/pre_update 33.38, stage/main/post_update 29.62, stage/render/prepare_resources 25.36 |
| warm-up | 3.49 | 65.5 | pipelines/wait_ms 4628, pipelines/compile_ms 1835, pipelines/front_ms 1709, render/thread/ms 197, render/busy/ms 195, main/busy/ms 164, pipelines/driver_ms 125, stage/main/pre_update 72.19, stage/main/update 61.64, stage/render/prepare_assets 59.23, stage/render/prepare_write_phase_buffers 33.31, stage/render/render 32.58 |
| warm-up | 3.53 | 39.6 | pipelines/compile_ms 812, pipelines/front_ms 771, pipelines/wait_ms 98.48, render/thread/ms 94.49, render/busy/ms 92.68, main/busy/ms 73.20, stage/render/prepare_assets 68.86, pipelines/driver_ms 40.70, stage/main/pre_update 40.50, stage/main/post_update 18.98, stage/render/render 17.98, terrain/update/ms 17.96 |
| warm-up | 3.56 | 35.0 | pipelines/compile_ms 1138, pipelines/front_ms 1092, pipelines/wait_ms 226, render/thread/ms 67.05, render/busy/ms 65.01, pipelines/driver_ms 46.44, stage/render/prepare_assets 30.68, main/busy/ms 30.10, stage/render/render 18.94, encode//sum 18.07, render/graph/ms 16.86, stage/main/post_update 13.89 |
| warm-up | 3.66 | 28.7 | main/busy/ms 61.35, pipelines/compile_ms 56.59, pipelines/front_ms 53.12, render/thread/ms 47.17, render/busy/ms 45.34, stage/main/update 44.78, stream/drive/ms 40.74, stage/render/prepare_assets 20.02, stage/main/post_update 11.10, stage/render/render 10.70, stage/render/prepare_resources 10.31, terrain/update/ms 9.63 |
| warm-up | 3.72 | 61.7 | pipelines/compile_ms 1371, pipelines/front_ms 1322, main/busy/ms 59.95, render/thread/ms 59.15, render/busy/ms 57.82, pipelines/driver_ms 49.44, stage/main/update 45.95, stream/drive/ms 41.01, stage/render/prepare_assets 36.85, stage/render/render 9.08, stage/render/prepare_resources 7.84, stage/main/post_update 7.76 |
| warm-up | 3.75 | 33.4 | pipelines/compile_ms 1388, pipelines/front_ms 1335, render/thread/ms 63.53, render/busy/ms 61.75, main/busy/ms 54.10, pipelines/driver_ms 53.08, stage/main/update 41.45, stage/render/prepare_assets 35.19, stream/drive/ms 33.71, stage/render/render 21.71, render/graph/ms 19.57, stage/render/graph_submit 11.86 |
| warm-up | 3.81 | 57.2 | pipelines/compile_ms 535, pipelines/front_ms 511, render/thread/ms 69.41, render/busy/ms 67.15, main/busy/ms 55.47, stage/main/update 43.13, stage/render/prepare_assets 37.45, stream/drive/ms 35.37, stage/render/render 23.88, pipelines/driver_ms 23.72, render/graph/ms 21.17, pipelines/wait_ms 18.31 |
| warm-up | 3.85 | 35.1 | pipelines/compile_ms 513, pipelines/front_ms 493, render/thread/ms 69.02, render/busy/ms 64.50, main/busy/ms 47.96, stage/render/prepare_assets 28.98, stage/main/update 20.30, pipelines/driver_ms 19.62, pipelines/wait_ms 17.43, stage/render/render 17.22, stream/drive/ms 14.33, stage/main/fixed_main_loop 13.19 |
| warm-up | 3.94 | 96.9 | pipelines/compile_ms 294, pipelines/front_ms 285, render/thread/ms 89.15, render/busy/ms 85.02, main/busy/ms 56.71, stage/render/prepare_assets 49.73, stage/main/update 20.60, stage/render/render 16.74, stage/main/post_update 16.11, stream/drive/ms 13.76, stage/main/fixed_main_loop 13.64, render/graph/ms 11.84 |
| warm-up | 3.99 | 46.9 | pipelines/compile_ms 1150, pipelines/front_ms 1123, render/thread/ms 88.13, render/busy/ms 86.76, stage/render/prepare_assets 68.12, main/busy/ms 55.89, stage/main/update 31.83, pipelines/driver_ms 27.08, stage/main/post_update 16.01, stage/render/render 11.70, pipelines/wait_ms 10.15, pipelines/lock_ms 9.67 |
| warm-up | 4.04 | 48.4 | pipelines/compile_ms 1830, pipelines/front_ms 1784, render/thread/ms 56.92, render/busy/ms 55.52, main/busy/ms 54.13, pipelines/driver_ms 46.16, stage/render/prepare_assets 38.54, stage/main/update 34.60, pipelines/wait_ms 17.18, pipelines/lock_ms 16.04, stage/main/post_update 10.40, stage/render/render 10.21 |
| warm-up | 4.07 | 37.3 | pipelines/compile_ms 1018, pipelines/front_ms 988, main/busy/ms 62.67, render/thread/ms 60.07, render/busy/ms 58.21, pipelines/driver_ms 30.62, stage/main/update 25.68, stage/render/prepare_assets 24.02, stage/main/post_update 21.23, stage/render/render 13.08, render/graph/ms 10.82, pipelines/wait_ms 9.44 |
| warm-up | 4.13 | 57.3 | pipelines/compile_ms 120, pipelines/front_ms 115, render/thread/ms 69.56, main/busy/ms 68.53, render/busy/ms 67.65, stage/main/update 29.70, stage/main/post_update 23.68, stage/render/prepare_assets 19.92, stage/render/render 13.56, render/graph/ms 11.20, stage/render/prepare_resources 9.33, stage/render/graph_submit 8.24 |
| warm-up | 4.19 | 59.7 | pipelines/compile_ms 447, pipelines/front_ms 435, render/thread/ms 84.49, render/busy/ms 82.92, main/busy/ms 58.29, stage/render/prepare_assets 29.52, stage/main/post_update 26.79, stage/main/update 18.75, stage/render/prepare_resources 18.64, pipelines/driver_ms 12.46, stage/render/render 11.97, render/graph/ms 9.71 |
| warm-up | 4.25 | 61.3 | pipelines/compile_ms 2059, pipelines/front_ms 1997, pipelines/wait_ms 283, render/thread/ms 100, render/busy/ms 97.97, main/busy/ms 68.09, pipelines/driver_ms 62.40, stage/main/post_update 44.59, encode//sum 43.11, stage/render/prepare_assets 28.13, stage/render/render 21.88, stage/render/prepare_resources 21.25 |
| warm-up | 4.34 | 91.5 | pipelines/compile_ms 2559, pipelines/front_ms 2482, pipelines/wait_ms 695, render/thread/ms 138, render/busy/ms 135, main/busy/ms 79.46, pipelines/driver_ms 76.94, encode//sum 73.31, stage/main/post_update 51.48, stage/render/prepare_write_phase_buffers 39.54, stage/render/render 32.60, render/graph/ms 29.11 |
| warm-up | 4.46 | 117.8 | pipelines/compile_ms 2762, pipelines/front_ms 2670, pipelines/wait_ms 1770, render/thread/ms 194, render/busy/ms 192, main/busy/ms 151, stage/main/update 99.70, pipelines/driver_ms 91.72, encode//sum 60.71, stage/render/prepare_write_phase_buffers 53.66, stream/drive/ms 53.17, stage/main/post_update 38.07 |
| warm-up | 4.55 | 88.9 | pipelines/compile_ms 2954, pipelines/front_ms 2849, pipelines/wait_ms 2096, render/thread/ms 185, render/busy/ms 183, main/busy/ms 131, pipelines/driver_ms 105, stage/main/update 102, stream/drive/ms 54.49, stage/render/prepare_assets 47.94, stage/render/render 45.22, render/graph/ms 42.24 |
| warm-up | 4.58 | 32.3 | pipelines/compile_ms 1415, pipelines/front_ms 1366, pipelines/wait_ms 753, render/thread/ms 104, render/busy/ms 102, main/busy/ms 89.62, stage/main/update 65.37, stream/drive/ms 50.67, pipelines/driver_ms 48.85, stage/render/render 35.25, render/graph/ms 32.58, stage/render/prepare_assets 31.02 |
| warm-up | 4.71 | 124.1 | pipelines/compile_ms 1175, pipelines/front_ms 1144, pipelines/wait_ms 268, main/busy/ms 90.94, render/thread/ms 78.63, render/busy/ms 77.13, stage/main/update 66.21, stage/render/prepare_assets 50.15, stream/drive/ms 50.13, pipelines/driver_ms 30.71, stage/main/post_update 16.02, stage/render/render 14.75 |
| warm-up | 4.77 | 57.2 | pipelines/compile_ms 1966, pipelines/front_ms 1812, pipelines/wait_ms 258, pipelines/driver_ms 154, render/thread/ms 101, render/busy/ms 99.31, main/busy/ms 81.99, stage/render/prepare_assets 81.57, stage/main/update 57.21, stream/drive/ms 42.37, stage/main/post_update 16.85, stage/render/render 9.89 |
| warm-up | 4.82 | 54.6 | pipelines/compile_ms 1178, pipelines/front_ms 1029, pipelines/driver_ms 149, render/thread/ms 88.96, render/busy/ms 87.59, main/busy/ms 74.29, stage/render/prepare_assets 54.56, stage/main/update 53.77, stream/drive/ms 41.84, stage/render/render 22.38, render/graph/ms 20.53, stage/render/graph_submit 18.06 |
| warm-up | 4.90 | 54.2 | pipelines/compile_ms 359, pipelines/front_ms 348, main/busy/ms 60.19, stage/main/update 47.06, render/thread/ms 36.00, render/busy/ms 34.70, stream/drive/ms 31.65, stage/render/prepare_assets 14.00, pipelines/driver_ms 10.58, stage/render/render 8.85, render/graph/ms 6.96, stage/main/post_update 6.94 |
| warm-up | 4.97 | 35.9 | pipelines/compile_ms 348, pipelines/front_ms 336, render/thread/ms 45.00, render/busy/ms 43.79, main/busy/ms 31.91, stage/render/prepare_assets 20.05, stage/main/update 13.54, stage/main/post_update 12.69, pipelines/driver_ms 11.72, stage/render/render 11.65, render/graph/ms 9.72, encode//sum 8.97 |
| warm-up | 5.01 | 35.4 | pipelines/compile_ms 185, pipelines/front_ms 181, render/thread/ms 61.28, render/busy/ms 59.20, main/busy/ms 38.52, stage/render/prepare_assets 20.63, stage/main/post_update 17.16, stage/render/render 15.92, stage/main/update 14.21, render/graph/ms 13.10, encode//sum 10.32, stage/render/graph_submit 9.12 |
| warm-up | 5.04 | 35.3 | pipelines/compile_ms 198, pipelines/front_ms 194, render/thread/ms 61.18, render/busy/ms 58.94, main/busy/ms 41.86, stage/render/render 17.41, stage/main/update 15.75, stage/main/post_update 15.66, render/graph/ms 14.52, stage/render/prepare_assets 13.76, encode//sum 10.50, stage/render/prepare_bind_groups 9.60 |
| warm-up | 5.09 | 45.5 | pipelines/compile_ms 1121, pipelines/front_ms 1092, render/thread/ms 63.67, render/busy/ms 62.22, main/busy/ms 41.76, stage/render/prepare_assets 31.95, pipelines/driver_ms 29.13, stage/main/update 21.98, stage/render/render 15.12, render/graph/ms 13.12, stream/drive/ms 10.96, stage/main/post_update 10.09 |
| warm-up | 5.12 | 31.6 | pipelines/compile_ms 1166, pipelines/front_ms 1133, render/thread/ms 64.01, render/busy/ms 61.88, main/busy/ms 47.25, pipelines/driver_ms 33.08, stage/render/prepare_assets 26.32, stage/main/update 25.22, stage/render/render 16.77, render/graph/ms 14.80, stream/drive/ms 11.22, stage/render/graph_submit 9.73 |
| warm-up | 5.16 | 42.3 | pipelines/compile_ms 732, pipelines/front_ms 709, render/thread/ms 68.65, render/busy/ms 66.19, main/busy/ms 57.90, stage/main/post_update 26.89, pipelines/driver_ms 22.99, stage/render/render 20.34, stage/main/update 19.07, render/graph/ms 17.98, stage/render/graph_submit 11.96, encode//sum 11.92 |
| warm-up | 5.23 | 42.1 | pipelines/compile_ms 393, pipelines/front_ms 382, render/thread/ms 51.86, render/busy/ms 50.65, main/busy/ms 35.61, stage/render/prepare_assets 22.86, stage/main/update 18.22, stage/render/render 14.19, render/graph/ms 12.22, stage/main/post_update 11.41, encode//sum 10.95, pipelines/driver_ms 10.67 |
| warm-up | 5.25 | 26.7 | pipelines/compile_ms 393, pipelines/front_ms 382, render/thread/ms 56.65, render/busy/ms 55.34, main/busy/ms 31.52, stage/render/prepare_assets 30.01, stage/render/render 14.45, stage/main/update 14.20, render/graph/ms 12.44, stage/main/post_update 11.94, encode//sum 10.85, pipelines/driver_ms 10.67 |
| warm-up | 5.33 | 26.3 | pipelines/compile_ms 198, pipelines/front_ms 195, render/thread/ms 45.63, render/busy/ms 44.53, main/busy/ms 32.75, stage/render/prepare_assets 19.32, stage/render/render 15.01, stage/main/update 14.71, encode//sum 13.88, render/graph/ms 13.18, stage/main/post_update 12.34, stream/drive/ms 8.43 |
| warm-up | 5.36 | 26.6 | render/thread/ms 45.35, render/busy/ms 44.12, main/busy/ms 28.24, stage/render/prepare_assets 19.24, stage/render/render 12.77, encode//sum 11.87, stage/main/post_update 11.48, stage/main/update 10.88, render/graph/ms 10.85, stage/render/graph_submit 6.53, encode/pass:shadow_point_light/sum 4.92, stage/render/camera_driver 4.20 |
| warm-up | 5.39 | 37.9 | render/thread/ms 56.17, render/busy/ms 54.72, stage/render/prepare_assets 29.02, main/busy/ms 27.89, stage/render/render 15.75, pipelines/compile_ms 15.16, pipelines/front_ms 14.22, render/graph/ms 13.65, stage/main/post_update 11.69, stage/main/update 11.13, encode//sum 10.39, stage/render/graph_submit 8.88 |
| warm-up | 5.42 | 27.7 | render/thread/ms 57.48, render/busy/ms 55.97, pipelines/compile_ms 50.76, pipelines/front_ms 47.34, stage/render/prepare_assets 27.02, main/busy/ms 23.15, stage/render/render 19.13, render/graph/ms 17.06, stage/render/graph_submit 12.41, encode//sum 10.20, stage/main/post_update 10.08, stage/main/update 7.95 |
| warm-up | 5.45 | 26.7 | pipelines/compile_ms 330, pipelines/front_ms 310, render/thread/ms 47.83, render/busy/ms 46.25, main/busy/ms 23.99, pipelines/driver_ms 20.02, stage/render/render 18.36, stage/render/prepare_assets 18.24, render/graph/ms 15.59, encode//sum 12.16, stage/render/graph_submit 11.03, stage/main/post_update 10.57 |
| warm-up | 5.48 | 30.0 | pipelines/compile_ms 486, pipelines/front_ms 460, render/thread/ms 51.54, render/busy/ms 50.15, stage/render/prepare_assets 30.58, pipelines/driver_ms 25.88, main/busy/ms 25.84, stage/render/render 12.99, stage/main/post_update 11.37, encode//sum 11.15, render/graph/ms 10.39, stage/main/update 8.90 |
| warm-up | 5.67 | 31.6 | pipelines/compile_ms 193, pipelines/front_ms 189, main/busy/ms 39.56, render/thread/ms 30.39, render/busy/ms 29.14, stage/main/update 24.08, stream/drive/ms 18.23, stage/render/render 15.41, render/graph/ms 13.74, encode//sum 10.81, stage/main/post_update 10.18, stage/render/graph_submit 9.37 |
| warm-up | 5.76 | 26.3 | pipelines/compile_ms 710, pipelines/front_ms 686, render/thread/ms 37.86, render/busy/ms 36.48, main/busy/ms 32.06, pipelines/driver_ms 23.64, stage/render/render 20.59, render/graph/ms 18.72, stage/main/post_update 18.19, stage/render/graph_submit 13.13, encode//sum 12.13, stage/main/update 9.78 |
| warm-up | 6.05 | 30.9 | render/thread/ms 39.15, render/busy/ms 38.14, main/busy/ms 23.95, stage/render/prepare_assets 17.28, stage/render/render 9.71, stage/main/update 9.49, stage/main/post_update 8.78, render/graph/ms 8.33, encode//sum 7.79, stage/render/graph_submit 5.69, stream/drive/ms 4.29, encode/pass:shadow_point_light/sum 4.02 |
| warm-up | 6.15 | 28.5 | pipelines/compile_ms 159, pipelines/front_ms 155, render/thread/ms 40.53, render/busy/ms 39.24, main/busy/ms 30.95, stage/main/update 17.98, stage/render/render 14.94, encode//sum 14.06, render/graph/ms 13.19, stage/render/prepare_assets 12.07, stage/main/post_update 9.28, stage/render/graph_submit 8.52 |
| warm-up | 6.28 | 27.8 | render/thread/ms 43.88, render/busy/ms 42.45, main/busy/ms 41.72, stage/render/render 17.76, stage/main/update 17.65, stage/main/post_update 16.86, render/graph/ms 15.73, encode//sum 12.35, stage/render/graph_submit 10.88, stage/render/prepare_assets 8.55, encode/submit 7.94, encode/pass:shadow_point_light/sum 6.59 |
| warm-up | 6.30 | 26.9 | pipelines/compile_ms 150, pipelines/front_ms 144, render/thread/ms 45.87, render/busy/ms 44.47, main/busy/ms 41.19, stage/main/update 18.21, stage/render/render 17.39, stage/main/post_update 15.95, render/graph/ms 15.34, stage/render/prepare_assets 13.93, encode//sum 11.98, stage/render/graph_submit 10.73 |
| warm-up | 6.33 | 28.0 | pipelines/compile_ms 150, pipelines/front_ms 144, render/thread/ms 41.71, render/busy/ms 40.64, main/busy/ms 32.90, stage/render/prepare_assets 18.31, stage/main/update 13.76, stage/main/post_update 12.93, stage/render/render 11.42, render/graph/ms 9.84, encode//sum 9.61, stage/render/graph_submit 6.58 |
| warm-up | 6.41 | 31.4 | render/thread/ms 43.82, main/busy/ms 43.46, render/busy/ms 42.53, stage/main/update 23.61, stage/render/prepare_assets 15.57, stage/main/post_update 13.53, stage/render/render 13.27, render/graph/ms 11.37, encode//sum 9.72, pipelines/compile_ms 8.29, pipelines/front_ms 7.96, stage/render/graph_submit 7.11 |
| warm-up | 6.44 | 28.1 | render/thread/ms 45.51, render/busy/ms 44.35, main/busy/ms 38.44, stage/main/update 21.38, stage/render/prepare_assets 19.00, stage/render/render 11.63, stage/main/post_update 11.23, render/graph/ms 9.81, encode//sum 9.46, pipelines/compile_ms 8.29, pipelines/front_ms 7.96, stage/render/graph_submit 6.34 |
| warm-up | 6.54 | 25.9 | render/thread/ms 42.22, render/busy/ms 40.72, main/busy/ms 28.59, stage/main/post_update 15.69, stage/render/prepare_assets 15.57, stage/render/render 14.76, encode//sum 13.21, render/graph/ms 12.73, stage/render/graph_submit 9.08, stage/main/update 8.71, encode/pass:shadow_point_light/sum 7.55, encode/submit 5.86 |
| warm-up | 6.61 | 33.0 | render/thread/ms 48.19, render/busy/ms 46.90, main/busy/ms 38.31, stage/render/prepare_assets 26.34, stage/main/update 20.85, stream/drive/ms 14.82, stage/main/post_update 12.99, stage/render/render 12.38, encode//sum 11.96, pipelines/compile_ms 11.46, render/graph/ms 10.40, pipelines/front_ms 10.31 |
| warm-up | 6.73 | 30.7 | pipelines/compile_ms 134, pipelines/front_ms 130, render/thread/ms 41.76, render/busy/ms 40.70, stage/render/render 31.53, render/graph/ms 29.99, main/busy/ms 27.09, stage/render/camera_driver 21.58, stage/main/update 13.44, encode//sum 11.29, stage/main/post_update 10.30, stage/render/graph_submit 8.26 |
| warm-up | 6.84 | 34.2 | main/busy/ms 39.11, render/thread/ms 28.03, render/busy/ms 26.89, stage/main/update 20.09, pipelines/compile_ms 15.39, pipelines/front_ms 14.89, stage/render/render 13.82, stage/main/post_update 13.54, stream/drive/ms 13.50, render/graph/ms 12.33, stream/spawn/ms 12.16, encode//sum 10.22 |
| measure | 0.78 | 27.9 | main/busy/ms 34.96, render/thread/ms 28.93, render/busy/ms 27.66, stage/main/update 18.79, stage/render/render 14.53, stream/drive/ms 13.81, stage/main/post_update 13.34, render/graph/ms 12.91, encode//sum 12.09, stage/render/graph_submit 9.46, encode/pass:shadow_point_light/sum 7.86, encode/submit 6.29 |
| measure | 2.17 | 26.3 | render/thread/ms 36.12, render/busy/ms 34.82, encode//sum 34.21, stage/render/render 24.36, render/graph/ms 22.34, main/busy/ms 22.02, encode/pass:shadow_point_light/sum 19.14, stage/render/graph_submit 15.72, stage/main/post_update 15.20, pipelines/compile_ms 11.90, pipelines/front_ms 11.66, encode/submit 9.12 |
| measure | 4.89 | 28.0 | main/busy/ms 34.03, render/thread/ms 23.57, render/busy/ms 22.52, stage/main/update 16.40, stage/main/post_update 14.99, stage/render/render 12.22, stream/drive/ms 11.40, render/graph/ms 10.57, encode//sum 9.64, encode/pass:shadow_point_light/sum 7.58, stage/render/graph_submit 7.16, encode/submit 4.47 |
| measure | 18.17 | 29.8 | main/busy/ms 37.24, stage/main/update 23.89, stream/drive/ms 20.19, render/thread/ms 17.57, render/busy/ms 16.65, stage/main/post_update 10.48, stage/render/render 9.26, render/graph/ms 7.82, encode//sum 7.64, stage/render/graph_submit 5.82, encode/submit 3.68, encode/finish_wall 2.21 |
| measure | 59.04 | 27.3 | render/thread/ms 36.58, render/busy/ms 35.48, main/busy/ms 26.98, stage/render/prepare_assets 14.69, stage/main/post_update 14.69, encode//sum 12.88, stage/render/render 12.44, render/graph/ms 10.99, stage/render/graph_submit 7.74, stage/main/update 7.61, encode/submit 4.45, encode/finish_wall 3.34 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.face_cull=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=facecull-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **72%** (min 47%) · 2625 MHz (min 2625) · memory 8101 MHz · 76.2 W · 75 °C (max 81) · VRAM 6752 MiB · throttled 100% of samples

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
- `shadow.lamp.face_cull=on (command line: --set)`
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
