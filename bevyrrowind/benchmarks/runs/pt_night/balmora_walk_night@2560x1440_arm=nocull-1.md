# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6020 frames over 65.6 s · **median 10.36 ms (97 fps)** · mean 10.90 · p95 14.83 · p99 16.57 (1% low 60 fps) · worst 30.66 ms · 167 spikes over 1.5× median · 25.6 stutters/min (work 29.2/min)
- warm-up: 323 frames over 9.3 s, worst 2378.8 ms
- hitches (> 25.9 ms): **2** measured, 55 in warm-up
- timed GPU passes: 6.75 ms of a 10.90 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.36 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 30.66 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.018 | 2.032 | 2.064 | 0.99 | 0.173 |
| main_opaque_pass_3d | 1.296 | 1.305 | 1.652 | 0.99 | 0.063 |
| early prepass | 0.487 | 0.491 | 0.701 | 0.99 | 0.087 |
| bloom | 0.314 | 0.316 | 0.323 | 0.99 | 0.063 |
| atmosphere_luts | 0.308 | 0.310 | 0.372 | 0.99 | 0.003 |
| water_velocity | 0.260 | 0.261 | 0.350 | 0.99 | 0.002 |
| auxiliary_views | 0.257 | 0.259 | 1.119 | 0.99 | 0.236 |
| water_sspr | 0.200 | 0.201 | 0.203 | 0.99 | 0.002 |
| ssao | 0.187 | 0.188 | 0.193 | 0.99 | 0.007 |
| uniform_allocation | 0.146 | 0.147 | 0.177 | 0.99 | 0.024 |
| upscaling | 0.129 | 0.130 | 0.137 | 0.99 | 0.004 |
| water_fft | 0.128 | 0.129 | 0.137 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.126 | 0.127 | 0.128 | 0.99 | 0.002 |
| clouds_composite | 0.104 | 0.105 | 0.109 | 0.99 | 0.002 |
| bin_unpacking | 0.104 | 0.104 | 0.126 | 0.99 | 0.017 |
| tonemapping | 0.086 | 0.087 | 0.091 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.078 | 0.078 | 0.154 | 0.99 | 0.009 |
| clustering | 0.063 | 0.064 | 0.067 | 0.99 | 0.019 |
| clouds_trace | 0.056 | 0.056 | 0.073 | 0.99 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.412 | 0.415 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| early_mesh_preprocessing | 0.036 | 0.036 | 0.045 | 0.99 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.219 | 0.229 | 0.12 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.029 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.033 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.028 | 0.99 | 0.173 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_copy | 0.002 | 0.013 | 0.013 | 0.12 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.522 | 13.194 | 24.23 | 57322.3 |
| render/busy/ms | 9.026 | 12.634 | 23.50 | 54339.5 |
| main/busy/ms | 8.621 | 10.521 | 28.22 | 51900.1 |
| stage/render/render | 5.578 | 8.458 | 15.96 | 33578.7 |
| stage/main/post_update | 5.219 | 6.755 | 9.36 | 31416.2 |
| render/graph/ms | 4.872 | 7.675 | 14.89 | 29329.3 |
| encode//sum | 4.052 | 6.487 | 28.40 | 24394.1 |
| stage/render/graph_submit | 3.548 | 5.497 | 10.88 | 21359.5 |
| encode/submit | 2.287 | 3.711 | 5.90 | 13768.0 |
| stage/main/update | 2.143 | 2.564 | 21.88 | 12898.5 |
| encode/finish_wall | 1.289 | 1.819 | 5.06 | 7758.0 |
| stage/render/camera_driver | 1.271 | 2.128 | 3.97 | 7648.8 |
| encode/pass:shadow_point_light/sum | 1.255 | 4.984 | 15.86 | 7557.3 |
| stage/render/prepare_write_phase_buffers | 0.754 | 1.120 | 2.27 | 4537.7 |
| encode/pass:shadow_directional_light/sum | 0.707 | 1.171 | 3.86 | 4253.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.697 | 1.023 | 1.93 | 4198.7 |
| encode/pass:early prepass/sum | 0.684 | 1.011 | 1.86 | 4115.3 |
| stage/render/prepare_assets | 0.578 | 0.745 | 11.50 | 3480.3 |
| bigspace/total/ms | 0.501 | 1.116 | 2.35 | 3014.4 |
| stage/render/prepare_meshes | 0.483 | 0.669 | 1.83 | 2907.2 |
| stage/render/prepare_resources | 0.443 | 0.530 | 0.98 | 2669.8 |
| render/present/ms | 0.402 | 0.483 | 1.86 | 2417.4 |
| stage/main/first | 0.396 | 0.521 | 5.43 | 2382.0 |
| stage/main/pre_update | 0.355 | 0.515 | 1.30 | 2138.3 |
| terrain/update/ms | 0.307 | 0.391 | 2.27 | 1847.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.290 | 0.428 | 0.94 | 1747.8 |
| stage/render/specialize | 0.284 | 0.434 | 4.72 | 1711.9 |
| terrain/select/ms | 0.274 | 0.349 | 0.93 | 1646.8 |
| stage/render/prepare_batch_phases | 0.265 | 0.375 | 0.70 | 1594.5 |
| stage/render/queue | 0.265 | 0.418 | 3.08 | 1593.2 |
| stage/render/prepare_bind_groups | 0.265 | 0.464 | 2.32 | 1592.5 |
| controller/feed/ms | 0.252 | 0.316 | 0.83 | 1518.7 |
| stream/drive/ms | 0.240 | 0.319 | 20.01 | 1445.3 |
| stage/main/fixed_main_loop | 0.214 | 0.364 | 1.06 | 1289.1 |
| bigspace/low_precision/ms | 0.202 | 0.551 | 1.64 | 1218.5 |
| far/select/ms | 0.197 | 0.263 | 0.55 | 1184.1 |
| stage/render/create_views | 0.192 | 0.248 | 1.28 | 1153.3 |
| stream/tick/ms | 0.184 | 0.242 | 0.52 | 1109.3 |
| stage/render/prepare_flush | 0.170 | 0.311 | 0.85 | 1026.3 |
| stage/render/extract_commands | 0.167 | 0.195 | 0.43 | 1008.3 |
| anim/evaluate/ms | 0.154 | 0.246 | 1.90 | 924.9 |
| anim/evaluate_cpu/ms | 0.150 | 0.187 | 0.71 | 902.2 |
| stage/render/prepare_views | 0.137 | 0.169 | 0.52 | 824.3 |
| controller/move/ms | 0.136 | 0.289 | 0.56 | 817.9 |
| bigspace/high_precision/ms | 0.135 | 0.508 | 1.69 | 814.1 |
| stage/main/last | 0.116 | 0.157 | 1.86 | 695.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.095 | 0.154 | 0.57 | 573.4 |
| render/acquire/ms | 0.094 | 0.118 | 0.48 | 565.5 |
| stage/render/cleanup | 0.087 | 0.214 | 0.64 | 524.9 |
| stage/render/phase_sort | 0.061 | 0.087 | 0.34 | 367.2 |
| stage/render/prepare_collect_phase_buffers | 0.057 | 0.123 | 0.49 | 342.1 |
| physics/step/ms | 0.047 | 0.086 | 0.62 | 283.6 |
| physics/insert/ms | 0.033 | 0.352 | 1.28 | 201.0 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.59 | 179.0 |
| weather/map/ms | 0.016 | 0.021 | 0.10 | 97.5 |
| pipelines/compile_ms | 0.015 | 0.000 | 80.27 | 91.7 |
| pipelines/front_ms | 0.015 | 0.000 | 77.23 | 88.5 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.46 | 87.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.26 | 63.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.24 | 63.8 |
| anim/apply/ms | 0.011 | 0.020 | 0.09 | 63.3 |
| weather/eval/ms | 0.010 | 0.013 | 0.17 | 58.8 |
| script/update/ms | 0.007 | 0.008 | 0.04 | 40.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.23 | 30.6 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 29.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 29.1 |
| script/ui/ms | 0.005 | 0.006 | 0.47 | 27.9 |
| controller/write/ms | 0.003 | 0.004 | 0.21 | 20.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.76 | 13.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.20 | 3.4 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.04 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1258 | 2262 | 658 |
| crowd/c0/meshes/count | count | 343 | 646 | 188 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5538 |
| crowd/c1/meshes/count | count | 1154 | 1254 | 1153 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 216 | 204 |
| crowd/joints_extracted/count | count | 3670 | 8334 | 5731 |
| crowd/joints_uploaded/count | count | 3634 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.41 | 1.27 | 0.51 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.46 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.24 | 0.01 |
| encode/finish_critical | ms | 0.90 | 3.86 | 1.48 |
| encode/pass:early prepass/max | ms | 0.70 | 1.86 | 0.91 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.71 | 1.93 | 0.45 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.57 | 0.06 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 0.94 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.71 | 3.86 | 1.48 |
| encode/pass:shadow_point_light/max | ms | 0.75 | 1.32 | 0.76 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.82 | 28.22 | 8.53 |
| gpu/timed/ms | ms | 7.49 | 10.72 | 7.78 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 252 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3947 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 252 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3947 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 4118 | 4335 | 4335 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 4118 | 4335 | 4335 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 200 | 20 |
| phases/shadow/bins | count | 10.4 k | 11.7 k | 7980 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 27.2 k | 29.2 k | 25.7 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 236 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 200 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 16 | 565 | 37 |
| preprocess/bind_groups_created | count | 16 | 565 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 135 | 314 | 129 |
| process/cpu_usage | % | 25.48 | 44.44 | 28.85 |
| process/mem_usage | GiB | 5.49 | 5.66 | 5.66 |
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
| skin/blocks_changed/count | count | 13 | 109 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 282.6 k | 812.2 k | 448.0 k |
| skin/joints_written/count | count | 4416 | 12.7 k | 7000 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4920 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 432 | 528 | 396 |
| statics/materials_bound | count | 432 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.36 | 39.50 | 31.14 |
| system/mem_usage | % | 54.25 | 54.75 | 54.75 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 71 | 251 | 72 |
| views/all/bins | count | 19.0 k | 20.4 k | 17.6 k |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 55.0 k | 57.6 k | 55.2 k |
| views/all/pipelines | count | 250 | 284 | 68 |
| views/all/pipelines_distinct | count | 71 | 75 | 63 |
| views/all/work_items | count | 41.5 k | 56.1 k | 56.0 k |
| views/cascade/batch_sets | count | 9 | 28 | 20 |
| views/cascade/bins | count | 7670 | 7981 | 7980 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 24.5 k | 25.7 k | 25.7 k |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 11.7 k | 25.7 k | 25.7 k |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 3564 | 4400 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3854 | 4716 | 0 |
| views/lamp_face/pipelines | count | 183 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 4040 | 0 |
| views/main/batch_sets | count | 51 | 55 | 52 |
| views/main/bins | count | 8726 | 9612 | 9612 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.1 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 48 | 51 | 48 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 45.30 | 202 | 272.7 k |
| anim/events/count | count | 0.10 | 3 | 588 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.60 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1191 | 4675 |
| encode//count | count | 37.67 | 219 | 226.8 k |
| encode/bind_walk/init_textures | count | 14.8 k | 112.9 k | 89.00 M |
| encode/bind_walk/init_textures_skipped | count | 56.8 k | 118.8 k | 341.91 M |
| encode/bind_walk/inits | count | 531 | 1210 | 3.20 M |
| encode/bind_walk/inits_skipped | count | 127 | 199 | 762.6 k |
| encode/bind_walk/merge_textures | count | 14.7 k | 112.9 k | 88.62 M |
| encode/bind_walk/merge_textures_skipped | count | 5532 | 6026 | 33.30 M |
| encode/bind_walk/merges | count | 107 | 343 | 644.6 k |
| encode/bind_walk/merges_skipped | count | 106 | 150 | 637.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6020 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6020 |
| encode/pass:early prepass/count | count | 1 | 1 | 6020 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6020 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6020 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6020 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6020 |
| encode/pass:shadow_point_light/count | count | 2.68 | 48 | 16.1 k |
| encode/queue/command_buffers | count | 50.36 | 277 | 303.2 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 2.25 M | 4.60 M | 13.55 G |
| encode/queue/write_buffer/calls | count | 272 | 843 | 1.64 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.8 k | 3.53 M | 1.02 G |
| encode/queue/write_buffer_with/calls | count | 11.01 | 465 | 66.3 k |
| encode/queue/write_texture/bytes | bytes | 10.5 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6020 |
| physics/inserts | count | 0.38 | 91 | 2268 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.65 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5623 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1233 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.9 ms; slowest 57, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.38 | 2378.8 | main/busy/ms 2351, stage/main/post_update 2311, stage/main/update 30.58, stage/main/pre_update 5.38, stage/main/last 0.97, surface/advance_cpu_ms 0.73, stage/main/first 0.68, stage/main/fixed_main_loop 0.35, surface/cell_upload_ms 0.29, stream/drive/ms 0.28, bigspace/total/ms 0.12, stream/tick/ms 0.08 |
| warm-up | 3.06 | 678.2 | main/busy/ms 2483, stage/main/post_update 2333, render/thread/ms 648, render/busy/ms 521, stage/render/prepare_assets 476, stage/main/update 135, stage/render/prepare_views 123, render/acquire/ms 123, stage/render/create_views 28.56, stage/render/render 11.71, stage/main/pre_update 7.08, render/graph/ms 6.89 |
| warm-up | 3.16 | 105.8 | render/thread/ms 736, render/busy/ms 559, stage/render/prepare_assets 499, main/busy/ms 153, stage/render/prepare_views 124, render/acquire/ms 123, pipelines/compile_ms 117, stage/main/update 109, pipelines/front_ms 76.66, stage/render/render 67.45, render/present/ms 53.93, pipelines/driver_ms 40.78 |
| warm-up | 3.19 | 27.9 | pipelines/compile_ms 440, pipelines/front_ms 370, pipelines/wait_ms 157, render/thread/ms 115, stage/render/render 76.88, main/busy/ms 71.82, pipelines/driver_ms 70.28, render/busy/ms 62.14, stage/main/post_update 54.08, render/present/ms 52.23, stage/render/prepare_assets 24.35, render/graph/ms 23.64 |
| warm-up | 3.32 | 133.3 | pipelines/wait_ms 4041, pipelines/compile_ms 1338, pipelines/front_ms 1238, render/thread/ms 133, render/busy/ms 130, main/busy/ms 124, pipelines/driver_ms 99.33, stage/main/post_update 88.67, stage/render/render 31.62, bigspace/total/ms 31.40, bigspace/high_precision/ms 30.94, stage/main/update 28.70 |
| warm-up | 3.38 | 54.3 | pipelines/wait_ms 3996, pipelines/compile_ms 1732, pipelines/front_ms 1624, render/thread/ms 159, render/busy/ms 155, main/busy/ms 151, pipelines/driver_ms 108, stage/main/update 81.64, stage/main/post_update 55.88, stage/render/prepare_assets 38.56, stage/render/prepare_resources 34.58, bigspace/total/ms 31.02 |
| warm-up | 3.47 | 88.9 | pipelines/compile_ms 1441, pipelines/front_ms 1353, pipelines/wait_ms 236, render/thread/ms 112, render/busy/ms 108, main/busy/ms 95.24, pipelines/driver_ms 88.29, stage/main/update 63.00, stage/render/prepare_assets 60.57, stage/render/render 24.11, render/graph/ms 21.63, stage/render/graph_submit 19.26 |
| warm-up | 3.54 | 47.6 | pipelines/compile_ms 1065, pipelines/front_ms 1005, pipelines/wait_ms 233, encode//sum 66.62, render/thread/ms 66.14, render/busy/ms 64.33, pipelines/driver_ms 59.71, main/busy/ms 41.58, stage/render/render 27.48, render/graph/ms 25.54, stage/render/graph_submit 23.23, stage/main/post_update 19.95 |
| warm-up | 3.59 | 34.5 | pipelines/compile_ms 646, pipelines/front_ms 624, render/thread/ms 48.31, render/busy/ms 46.60, main/busy/ms 33.10, stage/render/prepare_assets 25.26, pipelines/driver_ms 21.16, stage/render/render 15.31, stage/main/post_update 15.04, stage/main/update 13.66, render/graph/ms 13.09, stage/render/graph_submit 11.07 |
| warm-up | 3.68 | 80.1 | pipelines/compile_ms 931, pipelines/front_ms 888, main/busy/ms 60.56, stage/main/update 44.41, pipelines/driver_ms 43.56, render/thread/ms 42.63, render/busy/ms 41.34, stream/drive/ms 39.50, stage/render/prepare_assets 26.14, stage/render/render 10.56, encode//sum 10.10, render/graph/ms 9.08 |
| warm-up | 3.82 | 66.7 | pipelines/compile_ms 243, pipelines/front_ms 234, render/thread/ms 64.60, render/busy/ms 61.76, main/busy/ms 44.17, stage/render/prepare_assets 40.88, stage/main/update 31.28, stream/drive/ms 25.66, stage/render/render 16.51, render/graph/ms 13.00, pipelines/driver_ms 9.05, stage/render/camera_driver 7.34 |
| warm-up | 3.92 | 79.4 | pipelines/compile_ms 627, pipelines/front_ms 611, render/thread/ms 66.80, render/busy/ms 65.22, stage/render/prepare_assets 44.74, main/busy/ms 43.16, stage/main/update 21.32, pipelines/driver_ms 15.82, stage/main/post_update 13.88, stage/render/render 10.53, render/graph/ms 8.38, stage/render/graph_submit 5.61 |
| warm-up | 4.01 | 85.5 | pipelines/compile_ms 1000, pipelines/front_ms 960, render/thread/ms 127, render/busy/ms 113, main/busy/ms 65.09, stage/render/prepare_assets 47.01, stage/main/update 43.19, pipelines/driver_ms 39.92, stage/render/render 34.33, stage/render/create_views 34.21, render/graph/ms 20.28, stage/render/graph_submit 17.87 |
| warm-up | 4.08 | 75.3 | pipelines/compile_ms 1100, pipelines/front_ms 1053, render/thread/ms 129, pipelines/wait_ms 127, render/busy/ms 114, main/busy/ms 58.67, stage/render/render 47.70, pipelines/driver_ms 46.51, stage/main/update 37.72, stage/render/create_views 34.35, render/graph/ms 33.06, stage/render/prepare_assets 31.20 |
| warm-up | 4.15 | 69.6 | pipelines/compile_ms 1381, pipelines/front_ms 1335, pipelines/wait_ms 151, render/thread/ms 109, render/busy/ms 107, stage/render/prepare_assets 46.89, pipelines/driver_ms 45.54, main/busy/ms 42.28, stage/render/render 39.65, render/graph/ms 36.70, stage/render/camera_driver 19.46, stage/main/update 17.93 |
| warm-up | 4.26 | 113.2 | pipelines/compile_ms 1598, pipelines/front_ms 1543, pipelines/wait_ms 254, render/thread/ms 169, render/busy/ms 167, encode//sum 64.60, stage/render/prepare_assets 62.14, main/busy/ms 59.46, pipelines/driver_ms 55.40, stage/render/render 41.61, render/graph/ms 38.61, stage/main/post_update 27.09 |
| warm-up | 4.38 | 113.6 | pipelines/compile_ms 3475, pipelines/front_ms 3376, pipelines/wait_ms 425, render/thread/ms 207, render/busy/ms 205, pipelines/driver_ms 99.64, stage/render/prepare_assets 61.58, encode//sum 60.97, main/busy/ms 57.91, stage/render/prepare_resources 43.16, stage/render/render 31.31, render/graph/ms 28.72 |
| warm-up | 4.46 | 84.5 | pipelines/compile_ms 3043, pipelines/front_ms 2960, pipelines/wait_ms 479, render/thread/ms 148, render/busy/ms 147, main/busy/ms 90.31, pipelines/driver_ms 82.74, stage/main/update 61.66, stream/drive/ms 51.57, stage/render/prepare_assets 49.12, stage/render/render 25.02, render/graph/ms 22.75 |
| warm-up | 4.52 | 53.8 | pipelines/compile_ms 1087, pipelines/front_ms 1048, pipelines/wait_ms 472, main/busy/ms 94.76, render/thread/ms 77.32, render/busy/ms 72.84, stage/main/update 63.61, stream/drive/ms 52.45, pipelines/driver_ms 38.96, stage/render/prepare_assets 35.34, stage/render/render 22.55, render/graph/ms 17.28 |
| warm-up | 4.58 | 60.7 | pipelines/compile_ms 1202, pipelines/front_ms 1159, pipelines/wait_ms 504, main/busy/ms 155, stage/main/update 119, render/thread/ms 73.49, stream/drive/ms 71.06, render/busy/ms 69.11, pipelines/driver_ms 43.80, stage/render/prepare_assets 25.39, stage/render/render 20.24, stage/main/post_update 19.08 |
| warm-up | 4.75 | 169.4 | pipelines/compile_ms 3057, pipelines/front_ms 2954, pipelines/wait_ms 344, main/busy/ms 153, stage/main/update 121, render/thread/ms 107, render/busy/ms 105, pipelines/driver_ms 103, stream/drive/ms 71.00, stage/render/prepare_assets 53.28, stage/render/render 27.86, render/graph/ms 25.38 |
| warm-up | 4.78 | 32.5 | pipelines/compile_ms 2696, pipelines/front_ms 2604, pipelines/driver_ms 92.70, render/thread/ms 87.15, render/busy/ms 85.46, main/busy/ms 85.15, stage/main/update 61.56, stage/render/prepare_assets 43.19, stream/drive/ms 42.28, pipelines/wait_ms 33.05, stage/render/render 26.52, render/graph/ms 23.76 |
| warm-up | 4.87 | 95.0 | pipelines/compile_ms 321, pipelines/front_ms 308, main/busy/ms 84.91, stage/main/update 66.90, render/thread/ms 65.19, render/busy/ms 63.67, stream/drive/ms 41.39, stage/render/prepare_assets 28.25, stage/render/render 16.62, render/graph/ms 14.06, pipelines/driver_ms 12.72, encode//sum 11.74 |
| warm-up | 4.91 | 36.9 | pipelines/compile_ms 281, pipelines/front_ms 269, render/thread/ms 69.79, main/busy/ms 68.27, render/busy/ms 68.12, stage/main/update 49.37, stream/drive/ms 31.37, stage/render/prepare_assets 28.74, stage/render/render 16.84, render/graph/ms 14.42, pipelines/driver_ms 11.99, encode//sum 10.51 |
| warm-up | 4.95 | 38.7 | pipelines/compile_ms 996, pipelines/front_ms 960, main/busy/ms 57.33, render/thread/ms 51.86, render/busy/ms 50.15, stage/main/update 38.80, pipelines/driver_ms 36.20, stream/drive/ms 32.00, stage/render/render 17.77, render/graph/ms 15.41, stage/render/prepare_assets 12.10, stage/render/graph_submit 11.26 |
| warm-up | 4.99 | 36.8 | pipelines/compile_ms 956, pipelines/front_ms 925, render/thread/ms 54.49, render/busy/ms 52.79, main/busy/ms 35.28, pipelines/driver_ms 31.52, stage/main/post_update 16.69, stage/render/render 16.51, render/graph/ms 14.41, stage/main/update 12.50, stage/render/graph_submit 10.37, stage/render/prepare_assets 9.35 |
| warm-up | 5.02 | 30.8 | pipelines/compile_ms 309, pipelines/front_ms 301, render/thread/ms 61.29, render/busy/ms 59.74, main/busy/ms 40.58, stage/main/post_update 21.30, stage/render/render 17.13, render/graph/ms 15.05, stage/main/update 12.77, encode//sum 11.17, stage/render/graph_submit 10.92, stage/render/prepare_assets 8.72 |
| warm-up | 5.05 | 37.8 | pipelines/compile_ms 122, pipelines/front_ms 118, render/thread/ms 58.92, render/busy/ms 57.57, main/busy/ms 33.47, stage/render/prepare_assets 18.18, stage/render/render 15.34, stage/main/post_update 14.60, render/graph/ms 13.24, stage/main/update 12.61, encode//sum 11.54, stage/render/graph_submit 9.01 |
| warm-up | 5.09 | 32.4 | pipelines/compile_ms 198, pipelines/front_ms 192, render/thread/ms 60.35, render/busy/ms 58.72, main/busy/ms 41.33, stage/main/post_update 22.34, stage/render/render 18.73, stage/render/prepare_assets 18.06, render/graph/ms 16.30, stage/main/update 12.62, stage/render/graph_submit 10.81, encode//sum 10.59 |
| warm-up | 5.12 | 28.7 | pipelines/compile_ms 727, pipelines/front_ms 704, render/thread/ms 56.45, render/busy/ms 54.59, main/busy/ms 53.03, stage/main/post_update 28.02, stage/render/render 24.57, pipelines/driver_ms 22.69, render/graph/ms 21.94, stage/render/graph_submit 15.16, stage/main/update 13.32, encode//sum 12.11 |
| warm-up | 5.17 | 50.2 | pipelines/compile_ms 1768, pipelines/front_ms 1725, render/thread/ms 66.93, render/busy/ms 65.38, pipelines/driver_ms 43.60, main/busy/ms 42.33, stage/render/prepare_assets 31.10, stage/render/render 18.64, render/graph/ms 16.48, stage/main/post_update 16.22, stage/main/update 14.58, encode//sum 11.59 |
| warm-up | 5.19 | 29.3 | pipelines/compile_ms 1296, pipelines/front_ms 1266, render/thread/ms 64.36, render/busy/ms 62.94, main/busy/ms 34.74, stage/render/prepare_assets 34.25, pipelines/driver_ms 30.27, stage/main/update 16.00, stage/render/render 14.52, render/graph/ms 12.63, stage/main/post_update 12.01, encode//sum 9.65 |
| warm-up | 5.24 | 41.2 | render/thread/ms 61.31, render/busy/ms 59.65, pipelines/compile_ms 56.62, pipelines/front_ms 53.54, main/busy/ms 45.95, stage/render/prepare_assets 26.93, stage/main/update 26.17, stage/render/render 15.46, stream/drive/ms 14.95, render/graph/ms 13.37, stage/main/post_update 13.13, encode//sum 9.94 |
| warm-up | 5.26 | 27.9 | render/thread/ms 62.22, render/busy/ms 60.73, pipelines/compile_ms 56.91, pipelines/front_ms 54.66, main/busy/ms 38.70, stage/render/prepare_assets 32.58, stage/main/update 21.70, stream/drive/ms 14.67, stage/render/render 14.35, render/graph/ms 12.21, stage/main/post_update 11.27, encode//sum 10.24 |
| warm-up | 5.30 | 31.6 | pipelines/compile_ms 365, pipelines/front_ms 350, render/thread/ms 51.47, render/busy/ms 50.10, main/busy/ms 26.50, stage/render/prepare_assets 23.15, stage/render/render 16.16, pipelines/driver_ms 14.72, render/graph/ms 13.67, stage/main/post_update 12.31, encode//sum 10.76, stage/render/graph_submit 8.87 |
| warm-up | 5.33 | 34.7 | pipelines/compile_ms 377, pipelines/front_ms 360, render/thread/ms 59.62, render/busy/ms 58.12, main/busy/ms 47.75, stage/render/render 34.36, stage/main/post_update 33.15, render/graph/ms 31.86, pipelines/driver_ms 16.84, stage/render/camera_driver 16.10, stage/render/graph_submit 15.60, stage/render/prepare_assets 13.97 |
| warm-up | 5.36 | 33.8 | pipelines/compile_ms 752, pipelines/front_ms 724, render/thread/ms 61.08, render/busy/ms 59.67, main/busy/ms 44.44, stage/render/render 34.54, render/graph/ms 32.49, stage/main/post_update 30.00, pipelines/driver_ms 27.89, pipelines/wait_ms 19.30, stage/render/prepare_assets 17.87, pipelines/lock_ms 16.46 |
| warm-up | 5.42 | 41.4 | pipelines/compile_ms 193, pipelines/front_ms 188, render/thread/ms 53.95, render/busy/ms 52.50, main/busy/ms 26.78, stage/render/prepare_assets 22.84, stage/render/render 15.97, render/graph/ms 13.75, stage/main/post_update 13.59, encode//sum 11.52, stage/render/graph_submit 9.18, stage/main/update 8.30 |
| warm-up | 5.48 | 42.1 | render/thread/ms 52.39, render/busy/ms 51.00, stage/render/prepare_assets 29.03, main/busy/ms 24.83, stage/render/render 15.56, render/graph/ms 13.57, encode//sum 12.44, stage/main/post_update 11.09, stage/render/graph_submit 9.56, stage/main/update 8.23, encode/submit 7.05, encode/pass:shadow_point_light/sum 4.25 |
| warm-up | 5.71 | 27.4 | main/busy/ms 33.50, render/thread/ms 29.19, render/busy/ms 27.78, pipelines/compile_ms 27.21, pipelines/front_ms 25.87, stage/main/update 19.77, stream/drive/ms 15.60, stage/render/render 14.02, render/graph/ms 12.03, stage/main/post_update 10.27, encode//sum 9.20, stage/render/graph_submit 8.11 |
| warm-up | 5.86 | 28.6 | pipelines/compile_ms 748, pipelines/front_ms 720, render/thread/ms 41.73, render/busy/ms 40.39, pipelines/driver_ms 28.06, main/busy/ms 27.60, stage/render/prepare_assets 17.70, stage/main/post_update 14.63, stage/render/render 13.58, render/graph/ms 11.28, encode//sum 10.53, stage/main/update 8.95 |
| warm-up | 6.09 | 31.9 | pipelines/compile_ms 264, pipelines/front_ms 255, render/thread/ms 47.87, render/busy/ms 46.20, main/busy/ms 35.62, stage/main/post_update 16.94, stage/render/render 14.60, stage/main/update 13.78, render/graph/ms 12.14, encode//sum 11.83, pipelines/driver_ms 8.51, stage/render/specialize 8.25 |
| warm-up | 6.14 | 30.2 | render/thread/ms 33.99, render/busy/ms 32.96, main/busy/ms 22.74, stage/render/prepare_assets 13.15, stage/render/render 9.82, stage/main/post_update 9.72, render/graph/ms 8.35, stage/main/update 8.30, encode//sum 7.52, stage/render/graph_submit 5.49, encode/pass:shadow_point_light/sum 4.13, encode/submit 3.58 |
| warm-up | 6.23 | 28.5 | render/thread/ms 39.55, render/busy/ms 38.51, main/busy/ms 30.33, stage/main/update 14.96, stage/main/post_update 11.98, stage/render/render 11.53, render/graph/ms 10.03, encode//sum 9.40, stage/render/prepare_assets 8.63, stage/render/graph_submit 6.97, stage/render/specialize 5.43, encode/pass:shadow_point_light/sum 5.08 |
| warm-up | 6.36 | 27.3 | pipelines/compile_ms 151, pipelines/front_ms 146, render/thread/ms 43.67, render/busy/ms 42.34, main/busy/ms 37.75, stage/main/update 20.04, stage/main/post_update 12.87, stage/render/render 10.82, render/graph/ms 9.39, stage/render/specialize 9.20, encode//sum 8.67, stage/render/prepare_assets 6.83 |
| warm-up | 6.39 | 30.5 | pipelines/compile_ms 151, pipelines/front_ms 146, render/thread/ms 50.37, render/busy/ms 49.13, main/busy/ms 41.40, stage/main/update 18.09, stage/main/post_update 15.64, stage/render/render 13.28, render/graph/ms 11.56, encode//sum 11.34, stage/render/prepare_assets 10.75, stage/render/specialize 8.85 |
| warm-up | 6.42 | 27.7 | render/thread/ms 48.62, render/busy/ms 47.37, main/busy/ms 38.83, stage/main/update 17.09, stage/main/post_update 14.97, stage/render/render 13.64, stage/render/prepare_assets 13.31, render/graph/ms 11.89, encode//sum 11.86, stage/render/graph_submit 7.33, encode/pass:shadow_point_light/sum 6.78, stage/render/specialize 5.37 |
| warm-up | 6.45 | 29.7 | render/thread/ms 43.78, render/busy/ms 42.70, main/busy/ms 29.32, stage/render/prepare_assets 17.91, stage/main/update 13.80, stage/render/render 11.05, stage/main/post_update 10.31, encode//sum 9.76, render/graph/ms 9.57, stage/render/graph_submit 6.64, encode/pass:shadow_point_light/sum 5.02, stream/drive/ms 4.23 |
| warm-up | 6.48 | 26.5 | render/thread/ms 46.25, render/busy/ms 45.02, main/busy/ms 33.47, stage/render/render 18.17, render/graph/ms 16.43, stage/main/post_update 14.11, stage/main/update 13.26, stage/render/prepare_assets 12.38, stage/render/graph_submit 11.96, encode//sum 11.89, encode/submit 8.70, encode/pass:shadow_point_light/sum 7.18 |
| warm-up | 6.53 | 32.9 | render/thread/ms 46.40, render/busy/ms 45.43, main/busy/ms 33.56, stage/main/update 15.23, stage/render/prepare_assets 14.22, stage/main/post_update 13.00, encode//sum 11.62, stage/render/render 10.49, render/graph/ms 9.03, stage/render/specialize 6.76, encode/pass:shadow_point_light/sum 6.54, stage/render/graph_submit 6.13 |
| warm-up | 6.62 | 26.9 | render/thread/ms 41.38, render/busy/ms 40.18, main/busy/ms 23.63, stage/render/prepare_assets 16.81, stage/render/render 13.55, render/graph/ms 11.75, stage/main/post_update 11.59, encode//sum 11.54, stage/main/update 8.05, stage/render/graph_submit 7.83, encode/pass:shadow_point_light/sum 6.92, encode/submit 4.85 |
| warm-up | 6.70 | 26.5 | pipelines/compile_ms 145, pipelines/front_ms 141, render/thread/ms 45.17, render/busy/ms 43.88, main/busy/ms 40.45, stage/main/update 23.23, stage/render/prepare_assets 18.74, stream/drive/ms 14.27, stage/main/post_update 13.23, stage/render/render 13.17, encode//sum 11.32, render/graph/ms 11.21 |
| warm-up | 6.74 | 47.5 | pipelines/compile_ms 145, pipelines/front_ms 141, render/thread/ms 65.79, render/busy/ms 64.51, main/busy/ms 44.64, stage/render/prepare_assets 33.55, stage/main/update 26.38, stream/drive/ms 16.67, stage/render/render 14.53, stage/main/post_update 13.09, render/graph/ms 12.39, encode//sum 11.82 |
| warm-up | 6.79 | 29.6 | render/thread/ms 45.54, render/busy/ms 44.49, main/busy/ms 24.16, stage/render/prepare_assets 18.02, stage/render/render 11.39, stage/main/post_update 10.69, render/graph/ms 9.89, encode//sum 9.83, stage/main/update 9.39, pipelines/compile_ms 8.81, pipelines/front_ms 8.44, stage/render/graph_submit 6.97 |
| warm-up | 7.02 | 35.9 | main/busy/ms 40.61, render/thread/ms 34.09, render/busy/ms 32.93, stage/main/update 22.56, stream/drive/ms 14.07, stage/render/render 12.61, stage/render/prepare_assets 11.61, stage/main/post_update 10.95, render/graph/ms 10.89, encode//sum 9.97, stage/render/graph_submit 7.33, encode/pass:shadow_point_light/sum 6.29 |
| measure | 18.18 | 29.9 | main/busy/ms 38.00, render/thread/ms 24.02, stage/main/update 23.67, render/busy/ms 23.10, stream/drive/ms 20.24, stage/render/render 14.33, render/graph/ms 12.85, stage/main/post_update 11.71, encode//sum 10.55, stage/render/graph_submit 9.12, encode/pass:shadow_point_light/sum 7.44, encode/submit 5.96 |
| measure | 59.04 | 30.7 | render/thread/ms 38.31, render/busy/ms 37.16, main/busy/ms 25.65, stage/render/prepare_assets 15.99, stage/main/post_update 12.81, stage/render/render 9.80, render/graph/ms 8.37, encode//sum 8.30, stage/main/update 7.31, stage/render/graph_submit 5.70, encode/submit 3.49, stage/render/camera_driver 2.54 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=nocull-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **73%** (min 47%) · 2623 MHz (min 2610) · memory 8101 MHz · 76.3 W · 76 °C (max 82) · VRAM 7018 MiB · throttled 100% of samples

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
