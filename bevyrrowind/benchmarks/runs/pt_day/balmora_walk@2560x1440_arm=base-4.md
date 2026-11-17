# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5976 frames over 65.6 s · **median 10.75 ms (93 fps)** · mean 10.99 · p95 13.55 · p99 17.21 (1% low 58 fps) · worst 43.20 ms · 78 spikes over 1.5× median · 65.8 stutters/min (work 52.1/min)
- warm-up: 325 frames over 9.1 s, worst 2552.8 ms
- hitches (> 26.9 ms): **34** measured, 47 in warm-up
- timed GPU passes: 6.53 ms of a 10.99 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.75 ms, limit 11 ms
- **FAIL** frame_ms: 72 stutters (> 1.5x moving median of 60) in 1.09 min = 65.81/min, limit 1/min
- **FAIL** worst frame 43.20 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.844 | 2.034 | 2.068 | 0.91 | 0.179 |
| main_opaque_pass_3d | 1.476 | 1.628 | 2.047 | 0.91 | 0.068 |
| early prepass | 0.451 | 0.498 | 0.735 | 0.91 | 0.086 |
| bloom | 0.285 | 0.314 | 0.319 | 0.91 | 0.062 |
| atmosphere_luts | 0.275 | 0.303 | 0.366 | 0.91 | 0.003 |
| water_velocity | 0.254 | 0.280 | 0.348 | 0.91 | 0.002 |
| water_sspr | 0.183 | 0.202 | 0.203 | 0.91 | 0.002 |
| ssao | 0.171 | 0.188 | 0.193 | 0.91 | 0.007 |
| auxiliary_views | 0.149 | 0.164 | 0.606 | 0.91 | 0.166 |
| atmos_froxel_inject | 0.132 | 0.146 | 0.148 | 0.91 | 0.001 |
| clouds_trace | 0.127 | 0.140 | 0.233 | 0.91 | 0.001 |
| water_fft | 0.119 | 0.131 | 0.138 | 0.91 | 0.003 |
| uniform_allocation | 0.117 | 0.129 | 0.182 | 0.91 | 0.026 |
| upscaling | 0.107 | 0.118 | 0.125 | 0.91 | 0.004 |
| clouds_composite | 0.096 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.094 | 0.104 | 0.141 | 0.91 | 0.019 |
| main_transmissive_pass_3d | 0.078 | 0.086 | 0.180 | 0.91 | 0.008 |
| tonemapping | 0.077 | 0.085 | 0.088 | 0.91 | 0.004 |
| clustering | 0.058 | 0.064 | 0.068 | 0.91 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.91 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.64 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.81 | 0.002 |
| lightprobe_irradiance_map | 0.030 | 0.412 | 0.416 | 0.07 | 0.001 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.046 | 0.91 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.031 | 0.91 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.91 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.016 | 0.221 | 0.232 | 0.07 | 0.004 |
| night_sky | 0.012 | 0.014 | 0.034 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.91 | 0.123 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.07 | 0.001 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.07 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.497 | 11.919 | 36.84 | 56755.1 |
| main/busy/ms | 9.060 | 10.885 | 25.68 | 54145.0 |
| render/busy/ms | 8.938 | 11.401 | 36.31 | 53415.6 |
| stage/main/post_update | 5.630 | 7.084 | 10.06 | 33645.8 |
| stage/render/render | 5.531 | 7.156 | 33.13 | 33054.6 |
| render/graph/ms | 4.778 | 6.398 | 32.47 | 28551.8 |
| encode//sum | 3.856 | 5.135 | 40.58 | 23043.1 |
| stage/render/graph_submit | 3.574 | 4.836 | 13.15 | 21355.8 |
| encode/submit | 2.284 | 3.284 | 6.88 | 13652.0 |
| stage/main/update | 2.157 | 2.570 | 19.24 | 12889.7 |
| encode/finish_wall | 1.370 | 1.989 | 6.29 | 8184.5 |
| stage/render/camera_driver | 1.151 | 1.615 | 28.32 | 6878.1 |
| encode/pass:shadow_point_light/sum | 0.949 | 3.606 | 17.91 | 5670.2 |
| encode/pass:shadow_directional_light/sum | 0.866 | 1.577 | 4.93 | 5176.8 |
| stage/render/prepare_write_phase_buffers | 0.777 | 1.133 | 2.36 | 4641.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.774 | 1.096 | 1.78 | 4624.3 |
| encode/pass:early prepass/sum | 0.748 | 1.073 | 1.94 | 4472.8 |
| stage/render/prepare_assets | 0.575 | 0.753 | 11.65 | 3436.5 |
| bigspace/total/ms | 0.534 | 1.298 | 2.41 | 3192.5 |
| render/present/ms | 0.465 | 1.204 | 2.10 | 2777.0 |
| stage/render/prepare_meshes | 0.457 | 0.587 | 6.76 | 2730.7 |
| stage/render/prepare_resources | 0.452 | 0.537 | 0.90 | 2699.5 |
| stage/main/first | 0.399 | 0.523 | 5.17 | 2385.7 |
| stage/main/pre_update | 0.361 | 0.517 | 1.19 | 2157.8 |
| stage/render/specialize | 0.327 | 0.423 | 11.85 | 1954.6 |
| terrain/update/ms | 0.309 | 0.396 | 2.54 | 1846.0 |
| stage/render/queue | 0.295 | 0.435 | 7.64 | 1765.3 |
| stage/render/prepare_bind_groups | 0.287 | 0.518 | 2.63 | 1712.8 |
| terrain/select/ms | 0.275 | 0.355 | 0.96 | 1644.9 |
| controller/feed/ms | 0.262 | 0.329 | 0.69 | 1564.5 |
| stream/drive/ms | 0.252 | 0.332 | 17.28 | 1508.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.236 | 0.349 | 0.97 | 1409.6 |
| bigspace/low_precision/ms | 0.222 | 0.698 | 2.09 | 1326.2 |
| stage/main/fixed_main_loop | 0.220 | 0.377 | 1.09 | 1315.1 |
| stage/render/create_views | 0.200 | 0.260 | 0.97 | 1195.7 |
| far/select/ms | 0.200 | 0.273 | 0.57 | 1194.5 |
| stream/tick/ms | 0.196 | 0.267 | 0.61 | 1172.3 |
| stage/render/prepare_batch_phases | 0.190 | 0.273 | 0.61 | 1138.2 |
| stage/render/extract_commands | 0.175 | 0.204 | 0.54 | 1044.4 |
| anim/evaluate/ms | 0.165 | 0.261 | 1.61 | 987.4 |
| stage/render/prepare_flush | 0.164 | 0.315 | 1.70 | 982.9 |
| anim/evaluate_cpu/ms | 0.157 | 0.198 | 0.76 | 938.3 |
| bigspace/high_precision/ms | 0.151 | 0.625 | 1.63 | 901.0 |
| stage/render/prepare_views | 0.138 | 0.171 | 0.42 | 823.9 |
| controller/move/ms | 0.130 | 0.290 | 0.54 | 774.5 |
| stage/main/last | 0.116 | 0.156 | 2.70 | 690.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.161 | 0.70 | 598.4 |
| render/acquire/ms | 0.094 | 0.119 | 0.32 | 562.5 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.127 | 0.64 | 357.2 |
| pipelines/compile_ms | 0.049 | 0.000 | 121.88 | 292.4 |
| physics/step/ms | 0.048 | 0.087 | 0.73 | 285.4 |
| pipelines/front_ms | 0.047 | 0.000 | 118.14 | 280.8 |
| stage/render/cleanup | 0.032 | 0.097 | 0.90 | 190.8 |
| physics/insert/ms | 0.032 | 0.349 | 0.97 | 189.0 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.69 | 183.4 |
| stage/render/phase_sort | 0.030 | 0.051 | 0.25 | 178.0 |
| weather/map/ms | 0.025 | 0.033 | 0.23 | 149.7 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.48 | 89.1 |
| anim/apply/ms | 0.012 | 0.020 | 0.43 | 69.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.26 | 67.0 |
| controller/avoid/ms | 0.011 | 0.015 | 0.14 | 64.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.20 | 60.0 |
| script/update/ms | 0.007 | 0.009 | 0.24 | 44.0 |
| script/input/ms | 0.005 | 0.007 | 0.18 | 32.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.17 | 30.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 28.1 |
| script/ui/ms | 0.005 | 0.006 | 0.10 | 26.9 |
| controller/write/ms | 0.004 | 0.005 | 0.33 | 21.7 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.74 | 11.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.22 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.11 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 19 |
| controller/movers/count | count | 20 | 23 | 19 |
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1272 | 2218 | 641 |
| crowd/c0/meshes/count | count | 353 | 633 | 174 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5555 |
| crowd/c1/meshes/count | count | 1171 | 1254 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4609 | 10.3 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.40 | 2.19 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.48 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.26 | 0.01 |
| encode/finish_critical | ms | 0.97 | 4.93 | 0.94 |
| encode/pass:early prepass/max | ms | 0.76 | 1.94 | 0.75 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.79 | 1.78 | 0.94 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.70 | 0.13 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.97 | 0.23 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 4.93 | 0.81 |
| encode/pass:shadow_point_light/max | ms | 0.82 | 1.59 | 0.83 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.69 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.31 | 36.31 | 8.28 |
| gpu/timed/ms | ms | 7.93 | 11.38 | 8.28 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 810 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 810 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3399 | 5109 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3399 | 5109 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 202 | 4 |
| phases/shadow/bins | count | 5032 | 8907 | 2431 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6384 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 257 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2348 |
| physics/items | count | 222 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 561 | 6 |
| preprocess/bind_groups_created | count | 13 | 561 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 244 | 123 |
| process/cpu_usage | % | 26.44 | 40.10 | 28.37 |
| process/mem_usage | GiB | 5.47 | 5.64 | 5.64 |
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
| skin/blocks_changed/count | count | 14 | 131 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.9 k | 808.4 k | 496.1 k |
| skin/joints_written/count | count | 5780 | 12.6 k | 7752 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5011 | 5011 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.76 | 52.45 | 29.94 |
| system/mem_usage | % | 54.49 | 55.45 | 55.41 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 68 | 249 | 54 |
| views/all/bins | count | 7652 | 11.8 k | 5929 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 251 | 304 | 76 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 33.9 k | 8119 |
| views/cascade/batch_sets | count | 9 | 48 | 4 |
| views/cascade/bins | count | 2806 | 6174 | 2431 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8908 | 23.5 k | 6384 |
| views/cascade/pipelines | count | 36 | 54 | 30 |
| views/cascade/work_items | count | 412 | 22.6 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2159 | 2763 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2418 | 3058 | 0 |
| views/lamp_face/pipelines | count | 168 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2932 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2888 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8412 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8577 | 13.2 k | 7918 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.07 | 246 | 335.1 k |
| anim/events/count | count | 0.09 | 2 | 530 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.62 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1260 | 4675 |
| encode//count | count | 33.95 | 220 | 202.9 k |
| encode/bind_walk/init_textures | count | 12.9 k | 114.7 k | 76.89 M |
| encode/bind_walk/init_textures_skipped | count | 69.5 k | 164.6 k | 415.38 M |
| encode/bind_walk/inits | count | 461 | 1127 | 2.75 M |
| encode/bind_walk/inits_skipped | count | 111 | 300 | 662.1 k |
| encode/bind_walk/merge_textures | count | 12.8 k | 114.6 k | 76.46 M |
| encode/bind_walk/merge_textures_skipped | count | 5367 | 12.0 k | 32.08 M |
| encode/bind_walk/merges | count | 99.38 | 336 | 593.9 k |
| encode/bind_walk/merges_skipped | count | 84.96 | 255 | 507.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 5976 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 5976 |
| encode/pass:early prepass/count | count | 1 | 2 | 5976 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 5976 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 5976 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 5976 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 5976 |
| encode/pass:shadow_point_light/count | count | 1.50 | 48 | 8951 |
| encode/queue/command_buffers | count | 45.46 | 278 | 271.7 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.89 M | 11.28 G |
| encode/queue/write_buffer/calls | count | 260 | 837 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.6 k | 3.52 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 11.02 | 465 | 65.9 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 5976 |
| physics/inserts | count | 0.38 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5665 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1242 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.9 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2552.8 | main/busy/ms 2522, stage/main/post_update 2416, stage/main/update 97.01, surface/advance_cpu_ms 67.28, stage/main/pre_update 5.06, stage/main/last 1.07, surface/cell_upload_ms 0.80, stream/drive/ms 0.65, stage/main/first 0.61, stage/main/fixed_main_loop 0.35, weather/map/ms 0.23, stream/tick/ms 0.14 |
| warm-up | 3.39 | 832.3 | main/busy/ms 2694, stage/main/post_update 2441, render/thread/ms 797, render/busy/ms 626, stage/render/prepare_assets 576, stage/main/update 238, stage/render/prepare_views 168, render/acquire/ms 168, surface/advance_cpu_ms 67.28, stage/render/create_views 28.50, stage/render/render 13.27, render/graph/ms 8.63 |
| warm-up | 3.50 | 114.4 | render/thread/ms 894, render/busy/ms 668, stage/render/prepare_assets 601, main/busy/ms 191, stage/render/prepare_views 168, render/acquire/ms 168, stage/main/update 146, pipelines/compile_ms 105, stage/render/render 73.68, pipelines/front_ms 63.40, render/present/ms 58.06, pipelines/driver_ms 41.30 |
| warm-up | 3.55 | 51.7 | pipelines/compile_ms 449, pipelines/front_ms 372, render/thread/ms 134, pipelines/wait_ms 106, stage/render/render 89.57, pipelines/driver_ms 77.59, render/present/ms 70.32, main/busy/ms 68.92, render/busy/ms 62.59, stage/main/post_update 51.57, stage/render/prepare_assets 26.29, terrain/update/ms 23.44 |
| warm-up | 3.59 | 34.5 | pipelines/wait_ms 844, pipelines/compile_ms 599, pipelines/front_ms 545, main/busy/ms 95.68, render/thread/ms 68.54, pipelines/driver_ms 54.33, stage/main/post_update 52.67, render/busy/ms 51.64, stage/render/render 33.27, stage/main/pre_update 30.80, pipelines/lock_ms 22.02, stage/render/prepare_assets 21.64 |
| warm-up | 3.70 | 118.9 | pipelines/wait_ms 4471, pipelines/compile_ms 898, pipelines/front_ms 819, render/thread/ms 135, main/busy/ms 134, render/busy/ms 133, pipelines/driver_ms 79.30, stage/main/update 70.47, stage/render/prepare_resources 48.88, stage/main/pre_update 38.73, stage/render/prepare_assets 32.78, stage/main/post_update 20.57 |
| warm-up | 3.78 | 76.2 | pipelines/wait_ms 5326, pipelines/compile_ms 1989, pipelines/front_ms 1864, render/thread/ms 172, render/busy/ms 170, main/busy/ms 130, pipelines/driver_ms 125, stage/main/update 72.21, stage/render/prepare_assets 64.84, stage/render/prepare_resources 48.20, stage/main/post_update 23.53, stage/render/render 19.03 |
| warm-up | 3.82 | 37.9 | pipelines/compile_ms 2267, pipelines/front_ms 2152, pipelines/wait_ms 1912, pipelines/driver_ms 115, render/thread/ms 104, render/busy/ms 102, stage/render/prepare_assets 77.79, main/busy/ms 58.02, stage/main/post_update 23.58, stage/main/fixed_main_loop 19.74, stage/render/render 17.17, render/graph/ms 14.85 |
| warm-up | 3.88 | 33.9 | pipelines/compile_ms 731, pipelines/front_ms 699, render/thread/ms 55.44, render/busy/ms 52.74, encode//sum 32.81, pipelines/driver_ms 32.60, main/busy/ms 29.51, pipelines/wait_ms 28.70, stage/main/post_update 17.94, stage/render/render 16.87, stage/render/prepare_assets 14.26, render/graph/ms 13.67 |
| warm-up | 3.93 | 37.0 | pipelines/compile_ms 290, pipelines/front_ms 283, pipelines/wait_ms 55.79, render/thread/ms 46.10, render/busy/ms 43.60, stage/render/prepare_assets 25.70, main/busy/ms 22.86, stage/render/render 13.16, stage/main/update 10.84, render/graph/ms 10.47, terrain/update/ms 9.14, stage/render/graph_submit 8.55 |
| warm-up | 4.03 | 93.5 | pipelines/compile_ms 1371, pipelines/front_ms 1321, main/busy/ms 61.59, render/thread/ms 50.64, pipelines/driver_ms 50.43, render/busy/ms 49.38, stage/main/update 46.41, stream/drive/ms 40.93, stage/render/prepare_assets 34.56, pipelines/wait_ms 22.23, stage/main/post_update 9.21, stage/render/render 9.18 |
| warm-up | 4.13 | 44.1 | pipelines/compile_ms 330, pipelines/front_ms 313, render/thread/ms 60.46, render/busy/ms 58.69, main/busy/ms 50.65, stage/render/prepare_assets 39.52, stage/main/update 32.07, stream/drive/ms 28.27, pipelines/driver_ms 16.97, pipelines/wait_ms 15.81, pipelines/lock_ms 15.33, stage/render/render 14.63 |
| warm-up | 4.19 | 61.8 | pipelines/compile_ms 312, pipelines/front_ms 295, render/thread/ms 87.42, render/busy/ms 66.37, main/busy/ms 61.55, stage/render/prepare_assets 41.70, stage/main/update 37.31, stage/render/render 35.16, stream/drive/ms 30.96, render/present/ms 20.65, stage/main/post_update 17.42, pipelines/driver_ms 16.75 |
| warm-up | 4.24 | 47.6 | pipelines/compile_ms 719, pipelines/front_ms 697, render/thread/ms 79.94, render/busy/ms 59.18, stage/render/prepare_assets 37.55, main/busy/ms 36.35, stage/render/render 29.60, pipelines/driver_ms 21.67, render/present/ms 20.43, stage/main/update 13.88, stage/main/post_update 13.67, render/graph/ms 7.95 |
| warm-up | 4.30 | 65.8 | pipelines/compile_ms 1524, pipelines/front_ms 1474, render/thread/ms 80.68, render/busy/ms 78.65, main/busy/ms 66.75, pipelines/driver_ms 50.51, stage/main/update 45.83, stage/render/prepare_assets 44.89, stage/render/render 25.04, render/graph/ms 22.66, stage/render/graph_submit 19.62, stage/main/post_update 13.02 |
| warm-up | 4.37 | 70.8 | pipelines/compile_ms 1113, pipelines/front_ms 1075, render/thread/ms 99.69, render/busy/ms 97.73, main/busy/ms 74.62, stage/render/prepare_assets 60.87, stage/main/update 48.33, pipelines/driver_ms 37.96, stage/render/render 25.04, render/graph/ms 22.78, stage/render/graph_submit 19.68, stage/main/post_update 19.14 |
| warm-up | 4.42 | 41.3 | pipelines/compile_ms 785, pipelines/front_ms 762, render/thread/ms 88.68, render/busy/ms 86.55, stage/render/prepare_assets 50.01, main/busy/ms 49.88, stage/main/update 23.41, pipelines/driver_ms 22.49, stage/main/post_update 18.33, pipelines/wait_ms 17.54, encode//sum 14.84, stage/render/render 14.67 |
| warm-up | 4.52 | 106.5 | pipelines/compile_ms 2117, pipelines/front_ms 2057, pipelines/wait_ms 515, render/thread/ms 130, render/busy/ms 128, main/busy/ms 90.91, pipelines/driver_ms 59.47, stage/main/post_update 55.50, stage/render/prepare_assets 55.08, pipelines/lock_ms 39.15, stage/main/update 22.75, encode//sum 19.48 |
| warm-up | 4.59 | 67.7 | pipelines/compile_ms 1900, pipelines/front_ms 1845, pipelines/wait_ms 1037, render/thread/ms 155, render/busy/ms 153, main/busy/ms 106, stage/main/post_update 69.52, encode//sum 56.12, pipelines/driver_ms 54.38, stage/render/prepare_assets 45.46, pipelines/lock_ms 37.83, stage/render/render 28.64 |
| warm-up | 4.66 | 72.7 | pipelines/compile_ms 1774, pipelines/front_ms 1715, pipelines/wait_ms 1212, render/thread/ms 124, render/busy/ms 122, main/busy/ms 64.97, encode//sum 59.38, pipelines/driver_ms 59.35, stage/render/prepare_assets 31.99, stage/render/render 31.78, stage/main/post_update 31.28, render/graph/ms 28.53 |
| warm-up | 4.74 | 78.7 | pipelines/compile_ms 2599, pipelines/front_ms 2517, pipelines/wait_ms 1113, render/thread/ms 110, render/busy/ms 108, main/busy/ms 95.56, pipelines/driver_ms 82.39, stage/render/prepare_assets 64.86, stage/main/update 63.95, stream/drive/ms 53.11, stage/main/post_update 18.92, stage/render/render 17.15 |
| warm-up | 4.81 | 65.0 | pipelines/compile_ms 1972, pipelines/front_ms 1908, pipelines/wait_ms 966, main/busy/ms 113, render/thread/ms 86.71, render/busy/ms 85.20, stage/main/update 83.14, pipelines/driver_ms 63.73, stream/drive/ms 53.85, stage/render/prepare_assets 43.18, stage/main/post_update 15.66, stage/render/prepare_resources 12.70 |
| warm-up | 4.88 | 73.0 | pipelines/compile_ms 1658, pipelines/front_ms 1606, pipelines/wait_ms 1141, main/busy/ms 131, stage/main/update 105, render/thread/ms 84.48, render/busy/ms 82.56, pipelines/driver_ms 51.77, stream/drive/ms 46.72, stage/render/prepare_assets 31.57, stage/main/post_update 15.20, stage/render/render 13.50 |
| warm-up | 4.97 | 92.6 | pipelines/compile_ms 1716, pipelines/front_ms 1676, pipelines/wait_ms 658, main/busy/ms 115, stage/main/update 92.52, render/thread/ms 88.40, render/busy/ms 86.54, stage/render/prepare_assets 48.96, stream/drive/ms 46.15, pipelines/driver_ms 39.62, stage/main/post_update 14.88, stage/render/render 14.47 |
| warm-up | 5.01 | 41.5 | pipelines/compile_ms 1475, pipelines/front_ms 1434, main/busy/ms 79.36, render/thread/ms 74.55, render/busy/ms 72.97, stage/main/update 61.11, pipelines/wait_ms 47.36, stream/drive/ms 41.27, pipelines/driver_ms 40.39, stage/render/prepare_assets 39.93, stage/render/render 14.11, render/graph/ms 12.10 |
| warm-up | 5.07 | 55.1 | pipelines/compile_ms 985, pipelines/front_ms 948, main/busy/ms 70.88, render/thread/ms 56.90, stage/main/update 56.67, render/busy/ms 55.41, stream/drive/ms 40.42, pipelines/driver_ms 36.98, stage/render/prepare_assets 29.52, stage/render/render 10.41, encode//sum 8.63, render/graph/ms 8.39 |
| warm-up | 5.13 | 37.2 | main/busy/ms 49.99, pipelines/compile_ms 49.82, pipelines/front_ms 47.44, render/thread/ms 35.92, stage/main/update 35.78, render/busy/ms 34.65, stream/drive/ms 29.75, stage/render/render 12.82, render/graph/ms 11.13, stage/render/prepare_assets 10.02, stage/main/post_update 9.06, stage/render/graph_submit 8.35 |
| warm-up | 5.21 | 29.5 | pipelines/compile_ms 108, pipelines/front_ms 104, render/thread/ms 46.85, render/busy/ms 45.50, main/busy/ms 35.04, stage/main/post_update 15.60, stage/main/update 13.97, stage/render/render 12.05, render/graph/ms 10.20, stage/render/prepare_resources 9.54, stage/render/prepare_assets 8.43, encode//sum 7.46 |
| warm-up | 5.27 | 58.1 | pipelines/compile_ms 198, pipelines/front_ms 192, render/thread/ms 83.36, render/busy/ms 81.92, main/busy/ms 59.06, stage/main/post_update 29.85, stage/render/prepare_resources 27.52, stage/main/update 23.95, stage/render/prepare_assets 18.79, stage/render/render 12.43, render/graph/ms 10.52, stage/render/prepare_write_phase_buffers 9.65 |
| warm-up | 5.30 | 34.6 | pipelines/compile_ms 941, pipelines/front_ms 915, render/thread/ms 81.27, render/busy/ms 79.83, main/busy/ms 58.70, stage/render/prepare_assets 27.88, stage/main/post_update 26.68, pipelines/driver_ms 25.84, stage/main/update 23.92, stage/render/prepare_resources 19.43, stage/render/render 11.67, encode//sum 9.76 |
| warm-up | 5.35 | 41.1 | pipelines/compile_ms 1729, pipelines/front_ms 1681, render/thread/ms 58.08, render/busy/ms 56.74, pipelines/driver_ms 48.39, main/busy/ms 42.82, stage/render/prepare_assets 32.08, stage/main/update 21.96, stage/render/render 12.57, stage/main/post_update 12.55, render/graph/ms 10.61, stage/render/graph_submit 7.81 |
| warm-up | 5.44 | 30.5 | pipelines/compile_ms 188, pipelines/front_ms 181, render/thread/ms 46.42, render/busy/ms 45.30, main/busy/ms 33.39, stage/render/prepare_assets 23.11, stage/main/update 16.19, stage/main/post_update 11.26, stage/render/render 9.66, stream/drive/ms 7.96, render/graph/ms 7.79, encode//sum 6.95 |
| warm-up | 5.51 | 32.3 | pipelines/compile_ms 375, pipelines/front_ms 367, main/busy/ms 39.88, render/thread/ms 39.34, render/busy/ms 38.18, stage/main/update 21.07, stage/render/prepare_assets 16.53, stage/main/post_update 14.17, stage/render/render 12.25, render/graph/ms 10.32, stream/drive/ms 9.96, encode//sum 8.52 |
| warm-up | 5.70 | 29.6 | pipelines/compile_ms 234, pipelines/front_ms 224, render/thread/ms 49.17, render/busy/ms 47.89, stage/render/prepare_assets 32.14, main/busy/ms 23.49, stage/render/render 10.55, pipelines/driver_ms 10.40, stage/main/post_update 9.87, render/graph/ms 8.62, stage/main/update 8.34, encode//sum 7.85 |
| warm-up | 6.18 | 30.8 | render/thread/ms 38.64, render/busy/ms 37.63, main/busy/ms 27.15, stage/render/prepare_assets 16.29, stage/main/update 11.00, stage/main/post_update 10.42, stage/render/render 9.02, render/graph/ms 7.50, encode//sum 7.08, stage/render/graph_submit 5.39, stream/drive/ms 4.08, stage/render/specialize 3.44 |
| warm-up | 6.50 | 35.9 | main/busy/ms 42.40, render/thread/ms 28.61, render/busy/ms 27.46, stage/main/update 23.97, stream/drive/ms 14.31, stage/main/post_update 12.95, stage/render/render 11.78, render/graph/ms 9.92, encode//sum 8.70, stage/render/graph_submit 6.74, stage/render/prepare_assets 6.38, encode/submit 4.37 |
| warm-up | 6.99 | 32.7 | render/thread/ms 43.67, render/busy/ms 42.60, main/busy/ms 21.75, pipelines/compile_ms 20.55, pipelines/front_ms 19.56, stage/render/render 11.84, stage/main/post_update 10.46, render/graph/ms 10.28, stage/render/specialize 9.85, encode//sum 8.02, stage/render/graph_submit 7.88, stage/main/update 7.72 |
| warm-up | 8.27 | 32.2 | pipelines/compile_ms 134, pipelines/front_ms 131, render/thread/ms 37.39, render/busy/ms 36.30, main/busy/ms 18.23, stage/render/render 11.17, stage/main/post_update 10.18, render/graph/ms 9.67, stage/render/specialize 9.33, stage/render/graph_submit 7.52, encode//sum 7.14, stage/render/queue 6.81 |
| measure | 0.18 | 39.5 | render/thread/ms 43.89, render/busy/ms 42.93, stage/render/render 37.22, render/graph/ms 35.89, stage/render/camera_driver 29.58, main/busy/ms 16.64, stage/main/post_update 10.44, encode//sum 8.65, stage/render/graph_submit 6.22, encode/pass:shadow_point_light/sum 4.04, stage/main/update 4.00, encode/submit 3.93 |
| measure | 0.43 | 30.0 | render/thread/ms 41.43, render/busy/ms 40.15, main/busy/ms 19.60, encode//sum 18.58, stage/render/render 15.36, render/graph/ms 13.63, stage/main/post_update 12.73, stage/render/graph_submit 11.07, stage/render/specialize 9.89, encode/submit 6.67, stage/render/queue 6.63, encode/finish_wall 4.43 |
| measure | 1.71 | 31.7 | render/thread/ms 38.82, render/busy/ms 37.73, main/busy/ms 18.19, stage/render/render 13.07, render/graph/ms 11.56, stage/main/post_update 11.49, encode//sum 9.14, stage/render/specialize 8.99, stage/render/graph_submit 8.43, encode/pass:shadow_point_light/sum 6.25, stage/render/queue 5.94, encode/submit 5.14 |
| measure | 3.17 | 34.9 | render/thread/ms 50.38, render/busy/ms 49.36, main/busy/ms 22.36, stage/main/post_update 15.13, stage/render/render 13.95, encode//sum 13.94, render/graph/ms 12.33, stage/render/specialize 10.57, stage/render/graph_submit 8.75, stage/render/prepare_assets 8.36, encode/pass:shadow_point_light/sum 7.96, stage/render/queue 6.61 |
| measure | 4.62 | 32.5 | render/thread/ms 38.05, render/busy/ms 36.81, main/busy/ms 21.77, stage/main/post_update 13.74, stage/render/render 11.30, render/graph/ms 9.54, stage/render/specialize 9.41, encode//sum 7.72, stage/render/graph_submit 7.36, stage/render/queue 7.04, encode/submit 4.48, stage/main/update 4.42 |
| measure | 5.97 | 30.2 | render/thread/ms 35.42, render/busy/ms 34.43, main/busy/ms 19.76, stage/main/post_update 12.80, stage/render/render 10.36, stage/render/specialize 9.77, render/graph/ms 8.94, encode//sum 7.66, stage/render/graph_submit 6.85, stage/render/queue 6.69, stage/main/update 3.80, encode/submit 3.66 |
| measure | 9.94 | 30.3 | render/thread/ms 35.78, render/busy/ms 34.68, main/busy/ms 19.02, stage/main/post_update 11.63, stage/render/render 11.10, stage/render/specialize 9.57, render/graph/ms 9.44, encode//sum 8.38, stage/render/graph_submit 7.13, stage/render/queue 6.28, stage/main/update 4.36, encode/submit 4.18 |
| measure | 11.24 | 29.8 | render/thread/ms 35.89, render/busy/ms 34.76, main/busy/ms 18.18, stage/render/render 11.32, stage/main/post_update 11.28, render/graph/ms 9.81, stage/render/specialize 9.25, encode//sum 8.34, stage/render/graph_submit 7.41, stage/render/queue 6.09, encode/submit 4.36, stage/main/update 4.08 |
| measure | 15.69 | 30.5 | render/thread/ms 38.56, render/busy/ms 37.46, main/busy/ms 18.00, stage/render/render 11.81, stage/main/post_update 11.19, stage/render/specialize 10.46, render/graph/ms 10.32, encode//sum 8.40, stage/render/graph_submit 7.75, stage/render/queue 6.96, encode/pass:shadow_point_light/sum 4.74, encode/submit 4.62 |
| measure | 19.37 | 30.4 | render/thread/ms 33.68, render/busy/ms 32.71, main/busy/ms 19.62, stage/main/post_update 11.75, stage/render/render 9.85, stage/render/specialize 9.36, render/graph/ms 8.44, encode//sum 6.80, stage/render/graph_submit 6.27, stage/render/queue 6.12, stage/main/update 4.96, encode/submit 3.36 |
| measure | 20.74 | 31.2 | render/thread/ms 35.60, render/busy/ms 34.36, main/busy/ms 19.06, stage/main/post_update 11.75, stage/render/render 10.56, stage/render/specialize 10.08, render/graph/ms 8.92, encode//sum 7.35, stage/render/queue 6.51, stage/render/graph_submit 6.35, stage/main/update 3.98, encode/submit 3.91 |
| measure | 23.03 | 33.7 | render/thread/ms 43.24, render/busy/ms 42.14, main/busy/ms 22.92, stage/render/render 12.80, stage/main/post_update 12.54, stage/render/specialize 11.99, render/graph/ms 11.25, encode//sum 9.95, stage/render/graph_submit 8.72, stage/render/queue 7.99, stage/main/update 6.25, encode/submit 5.31 |
| measure | 27.29 | 35.8 | render/thread/ms 47.19, render/busy/ms 46.10, main/busy/ms 25.25, stage/render/render 16.53, render/graph/ms 14.90, encode//sum 13.40, stage/main/post_update 13.12, stage/render/specialize 12.24, stage/render/graph_submit 11.40, encode/pass:shadow_point_light/sum 10.60, stage/render/queue 7.70, encode/submit 7.42 |
| measure | 28.87 | 30.9 | render/thread/ms 42.33, render/busy/ms 41.17, main/busy/ms 20.51, stage/render/render 13.81, stage/main/post_update 13.21, render/graph/ms 12.22, stage/render/specialize 9.96, encode//sum 9.92, stage/render/graph_submit 8.83, encode/pass:shadow_point_light/sum 7.75, stage/render/queue 6.98, encode/submit 5.63 |
| measure | 30.46 | 33.5 | render/thread/ms 40.06, render/busy/ms 38.96, main/busy/ms 21.14, stage/main/post_update 12.49, stage/render/render 11.73, stage/render/specialize 11.13, render/graph/ms 10.17, stage/render/graph_submit 7.97, encode//sum 7.94, stage/render/queue 7.45, stage/main/update 5.61, encode/submit 4.73 |
| measure | 31.95 | 31.3 | render/thread/ms 36.61, render/busy/ms 35.59, main/busy/ms 19.10, stage/render/render 11.60, stage/main/post_update 11.38, encode//sum 10.96, render/graph/ms 10.14, stage/render/specialize 9.77, stage/render/graph_submit 7.74, stage/render/queue 6.17, stage/main/update 5.19, encode/submit 4.50 |
| measure | 35.36 | 30.8 | render/thread/ms 39.37, render/busy/ms 38.36, main/busy/ms 21.40, stage/main/post_update 12.72, stage/render/render 10.99, stage/render/specialize 10.69, render/graph/ms 9.57, encode//sum 7.81, stage/render/queue 7.67, stage/render/graph_submit 7.36, stage/main/update 5.09, encode/submit 4.30 |
| measure | 36.75 | 32.7 | render/thread/ms 39.37, render/busy/ms 38.04, main/busy/ms 20.83, stage/main/post_update 12.99, stage/render/render 11.22, stage/render/specialize 10.86, render/graph/ms 9.68, encode//sum 8.72, stage/render/graph_submit 7.29, stage/render/queue 6.78, stage/main/update 4.82, encode/submit 4.43 |
| measure | 37.48 | 31.6 | render/thread/ms 39.57, render/busy/ms 38.40, main/busy/ms 20.74, stage/render/render 12.57, stage/main/post_update 12.46, render/graph/ms 11.01, stage/render/specialize 10.28, encode//sum 8.73, stage/render/graph_submit 8.18, stage/render/queue 6.91, encode/submit 4.92, stage/main/update 4.79 |
| measure | 40.79 | 31.8 | render/thread/ms 38.93, render/busy/ms 37.85, main/busy/ms 18.68, stage/render/render 11.26, stage/main/post_update 10.55, render/graph/ms 9.70, stage/render/specialize 9.53, encode//sum 8.16, stage/render/graph_submit 7.53, stage/render/queue 6.47, stage/main/update 4.81, encode/submit 4.36 |
| measure | 43.21 | 30.6 | render/thread/ms 37.35, render/busy/ms 36.26, main/busy/ms 16.98, stage/render/render 10.62, stage/render/specialize 10.17, stage/main/post_update 9.66, render/graph/ms 9.10, stage/render/graph_submit 7.28, encode//sum 6.97, stage/render/queue 6.48, stage/main/update 4.25, encode/submit 3.97 |
| measure | 45.66 | 34.6 | render/thread/ms 38.05, render/busy/ms 37.07, main/busy/ms 17.12, stage/render/specialize 10.45, stage/render/render 10.19, stage/main/post_update 9.32, render/graph/ms 8.88, encode//sum 8.10, stage/render/graph_submit 6.83, stage/render/queue 6.62, stage/main/update 4.72, stage/render/prepare_meshes 4.19 |
| measure | 56.44 | 30.0 | render/thread/ms 36.55, render/busy/ms 35.42, main/busy/ms 15.38, stage/render/render 11.11, stage/render/specialize 9.98, render/graph/ms 9.72, stage/main/post_update 8.64, stage/render/graph_submit 7.72, encode//sum 6.83, stage/render/queue 6.29, encode/submit 4.90, stage/main/update 3.97 |
| measure | 59.04 | 43.2 | render/thread/ms 55.66, render/busy/ms 54.26, main/busy/ms 29.82, stage/render/render 19.21, render/graph/ms 17.30, stage/main/post_update 15.95, stage/render/graph_submit 14.07, encode//sum 11.93, stage/render/specialize 9.69, stage/main/update 9.17, encode/finish_wall 7.90, stage/render/prepare_meshes 7.27 |
| measure | 59.07 | 32.6 | render/thread/ms 57.27, render/busy/ms 56.20, main/busy/ms 24.87, stage/render/prepare_assets 16.45, stage/main/post_update 12.02, stage/render/render 10.05, stage/render/specialize 9.94, render/graph/ms 8.73, stage/render/prepare_meshes 7.95, encode//sum 7.68, stage/main/update 7.12, stage/render/graph_submit 6.76 |
| measure | 62.58 | 31.4 | render/thread/ms 39.29, render/busy/ms 38.31, main/busy/ms 18.39, stage/render/render 11.24, stage/render/specialize 10.38, render/graph/ms 10.00, stage/main/post_update 9.53, stage/render/graph_submit 7.70, encode//sum 7.65, stage/render/queue 6.73, stage/main/update 6.33, stage/render/prepare_meshes 4.14 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=base-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 47%) · 2622 MHz (min 2610) · memory 8101 MHz · 79.7 W · 76 °C (max 82) · VRAM 6800 MiB · throttled 100% of samples

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
