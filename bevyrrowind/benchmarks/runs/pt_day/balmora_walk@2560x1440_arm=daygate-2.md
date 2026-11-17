# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6197 frames over 65.6 s · **median 10.33 ms (97 fps)** · mean 10.59 · p95 13.14 · p99 16.27 (1% low 61 fps) · worst 41.19 ms · 78 spikes over 1.5× median · 56.7 stutters/min (work 52.1/min)
- warm-up: 322 frames over 9.1 s, worst 2580.5 ms
- hitches (> 25.8 ms): **33** measured, 41 in warm-up
- timed GPU passes: 6.43 ms of a 10.59 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.33 ms, limit 11 ms
- **FAIL** frame_ms: 62 stutters (> 1.5x moving median of 60) in 1.09 min = 56.67/min, limit 1/min
- **FAIL** worst frame 41.19 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.817 | 2.035 | 2.068 | 0.89 | 0.174 |
| main_opaque_pass_3d | 1.491 | 1.670 | 2.065 | 0.89 | 0.057 |
| early prepass | 0.445 | 0.498 | 0.737 | 0.89 | 0.087 |
| bloom | 0.280 | 0.314 | 0.321 | 0.89 | 0.061 |
| atmosphere_luts | 0.272 | 0.305 | 0.366 | 0.89 | 0.003 |
| water_velocity | 0.250 | 0.280 | 0.347 | 0.89 | 0.002 |
| water_sspr | 0.180 | 0.202 | 0.203 | 0.89 | 0.002 |
| ssao | 0.168 | 0.188 | 0.193 | 0.89 | 0.007 |
| atmos_froxel_inject | 0.130 | 0.146 | 0.148 | 0.89 | 0.001 |
| clouds_trace | 0.124 | 0.139 | 0.231 | 0.89 | 0.001 |
| water_fft | 0.119 | 0.133 | 0.140 | 0.89 | 0.003 |
| uniform_allocation | 0.114 | 0.127 | 0.179 | 0.89 | 0.024 |
| upscaling | 0.104 | 0.117 | 0.125 | 0.89 | 0.004 |
| auxiliary_views | 0.103 | 0.116 | 0.485 | 0.89 | 0.114 |
| clouds_composite | 0.094 | 0.106 | 0.109 | 0.89 | 0.002 |
| bin_unpacking | 0.092 | 0.103 | 0.141 | 0.89 | 0.018 |
| main_transmissive_pass_3d | 0.076 | 0.085 | 0.178 | 0.89 | 0.008 |
| tonemapping | 0.076 | 0.085 | 0.089 | 0.89 | 0.003 |
| clustering | 0.057 | 0.064 | 0.067 | 0.89 | 0.018 |
| render_sky | 0.046 | 0.052 | 0.053 | 0.89 | 0.002 |
| clouds_resolve | 0.043 | 0.048 | 0.049 | 0.89 | 0.001 |
| lightprobe_irradiance_map | 0.040 | 0.412 | 0.416 | 0.10 | 0.001 |
| surface_exposure | 0.039 | 0.044 | 0.045 | 0.89 | 0.001 |
| surface_trail | 0.035 | 0.056 | 0.057 | 0.61 | 0.002 |
| surface_exposure_far | 0.033 | 0.042 | 0.045 | 0.79 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.89 | 0.003 |
| early_mesh_preprocessing | 0.027 | 0.030 | 0.046 | 0.89 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.029 | 0.031 | 0.89 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.89 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.220 | 0.231 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.89 | 0.002 |
| night_sky | 0.012 | 0.014 | 0.033 | 0.89 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.89 | 0.122 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.89 | 0.002 |
| per_view_shadow_pass | 0.009 | 0.011 | 0.022 | 0.89 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.89 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.89 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.031 | 10.993 | 35.79 | 55962.5 |
| main/busy/ms | 8.866 | 10.487 | 28.75 | 54944.3 |
| render/busy/ms | 8.470 | 10.496 | 35.30 | 52489.5 |
| stage/main/post_update | 5.494 | 6.818 | 10.82 | 34046.7 |
| stage/render/render | 5.256 | 6.623 | 14.58 | 32573.4 |
| render/graph/ms | 4.509 | 5.901 | 13.67 | 27941.3 |
| encode//sum | 3.705 | 4.794 | 19.23 | 22959.9 |
| stage/render/graph_submit | 3.383 | 4.511 | 10.09 | 20962.1 |
| encode/submit | 2.164 | 3.130 | 8.33 | 13409.1 |
| stage/main/update | 2.126 | 2.541 | 21.78 | 13175.2 |
| encode/finish_wall | 1.304 | 1.914 | 4.78 | 8083.6 |
| stage/render/camera_driver | 1.074 | 1.445 | 3.95 | 6656.6 |
| encode/pass:shadow_directional_light/sum | 0.845 | 1.526 | 4.58 | 5237.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.753 | 1.046 | 1.58 | 4663.3 |
| encode/pass:early prepass/sum | 0.731 | 1.025 | 3.72 | 4527.0 |
| stage/render/prepare_write_phase_buffers | 0.716 | 1.010 | 2.59 | 4436.1 |
| encode/pass:shadow_point_light/sum | 0.671 | 2.738 | 15.87 | 4160.0 |
| bigspace/total/ms | 0.567 | 1.329 | 2.37 | 3511.2 |
| stage/render/prepare_assets | 0.549 | 0.706 | 12.21 | 3399.2 |
| render/present/ms | 0.466 | 1.190 | 2.64 | 2887.5 |
| stage/render/prepare_resources | 0.445 | 0.525 | 3.70 | 2760.6 |
| stage/render/prepare_meshes | 0.434 | 0.542 | 6.96 | 2689.8 |
| stage/main/first | 0.392 | 0.514 | 6.29 | 2428.7 |
| stage/main/pre_update | 0.354 | 0.523 | 1.57 | 2190.8 |
| stage/render/specialize | 0.307 | 0.397 | 11.64 | 1904.2 |
| terrain/update/ms | 0.304 | 0.390 | 1.74 | 1884.5 |
| stage/render/queue | 0.277 | 0.400 | 7.58 | 1719.1 |
| terrain/select/ms | 0.272 | 0.349 | 0.89 | 1682.5 |
| stage/render/prepare_bind_groups | 0.267 | 0.460 | 2.03 | 1654.8 |
| controller/feed/ms | 0.264 | 0.330 | 0.78 | 1636.8 |
| stream/drive/ms | 0.249 | 0.332 | 19.68 | 1540.5 |
| bigspace/low_precision/ms | 0.238 | 0.849 | 1.96 | 1475.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.232 | 0.343 | 1.03 | 1437.0 |
| stage/main/fixed_main_loop | 0.211 | 0.367 | 1.25 | 1308.1 |
| far/select/ms | 0.196 | 0.261 | 0.79 | 1212.7 |
| stream/tick/ms | 0.193 | 0.263 | 0.63 | 1194.4 |
| stage/render/create_views | 0.193 | 0.247 | 1.04 | 1194.0 |
| stage/render/prepare_batch_phases | 0.187 | 0.268 | 0.69 | 1156.3 |
| bigspace/high_precision/ms | 0.168 | 0.739 | 1.97 | 1041.2 |
| stage/render/extract_commands | 0.167 | 0.195 | 0.41 | 1034.6 |
| anim/evaluate/ms | 0.167 | 0.314 | 1.47 | 1033.7 |
| pipelines/compile_ms | 0.159 | 0.000 | 130.17 | 988.1 |
| stage/render/prepare_flush | 0.158 | 0.318 | 1.07 | 981.9 |
| pipelines/front_ms | 0.155 | 0.000 | 127.25 | 959.3 |
| anim/evaluate_cpu/ms | 0.153 | 0.187 | 0.76 | 946.0 |
| stage/render/prepare_views | 0.137 | 0.171 | 0.59 | 847.8 |
| controller/move/ms | 0.127 | 0.250 | 0.61 | 786.2 |
| stage/main/last | 0.115 | 0.159 | 1.57 | 714.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.096 | 0.159 | 0.73 | 594.9 |
| render/acquire/ms | 0.094 | 0.121 | 0.32 | 585.4 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.131 | 0.54 | 390.2 |
| physics/step/ms | 0.047 | 0.088 | 0.53 | 288.2 |
| stage/render/phase_sort | 0.033 | 0.057 | 0.25 | 204.8 |
| stage/render/cleanup | 0.032 | 0.093 | 0.61 | 196.2 |
| physics/insert/ms | 0.031 | 0.336 | 0.87 | 193.8 |
| encode/resolve_diagnostics/sum | 0.029 | 0.039 | 0.60 | 182.7 |
| weather/map/ms | 0.025 | 0.033 | 0.35 | 155.8 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.29 | 87.9 |
| anim/apply/ms | 0.011 | 0.021 | 0.45 | 70.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.55 | 70.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.20 | 65.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.20 | 63.0 |
| script/update/ms | 0.007 | 0.009 | 0.05 | 42.6 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 32.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 30.3 |
| pipelines/driver_ms | 0.005 | 0.000 | 4.17 | 28.8 |
| script/ui/ms | 0.005 | 0.006 | 0.12 | 28.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.15 | 28.3 |
| controller/write/ms | 0.004 | 0.005 | 0.10 | 22.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.57 | 14.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.21 | 6.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.19 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 15 |
| crowd/c0/joints/count | count | 1306 | 2262 | 707 |
| crowd/c0/meshes/count | count | 359 | 646 | 189 |
| crowd/c1/count | count | 145 | 156 | 146 |
| crowd/c1/joints/count | count | 5531 | 6023 | 5489 |
| crowd/c1/meshes/count | count | 1167 | 1297 | 1152 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4613 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.39 | 3.49 | 0.56 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.29 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.55 | 0.02 |
| encode/finish_critical | ms | 0.94 | 4.58 | 1.35 |
| encode/pass:early prepass/max | ms | 0.75 | 3.72 | 0.84 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.58 | 0.94 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.73 | 0.16 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 1.03 | 0.28 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 4.58 | 1.35 |
| encode/pass:shadow_point_light/max | ms | 0.76 | 1.28 | 0.81 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.60 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.05 | 35.30 | 8.89 |
| gpu/timed/ms | ms | 7.92 | 9.10 | 8.23 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 817 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 817 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1281 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3415 | 5141 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1281 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3415 | 5141 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 198 | 29 |
| phases/shadow/bins | count | 5018 | 8840 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.3 k | 6380 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 200 | 251 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 184 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 11 | 549 | 58 |
| preprocess/bind_groups_created | count | 11 | 549 | 58 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 228 | 123 |
| process/cpu_usage | % | 26.54 | 40.99 | 25.94 |
| process/mem_usage | GiB | 5.51 | 5.68 | 5.68 |
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
| skin/blocks_changed/count | count | 14 | 131 | 15 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 365.6 k | 808.2 k | 498.7 k |
| skin/joints_written/count | count | 5712 | 12.6 k | 7792 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5010 | 5010 |
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
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.78 | 42.72 | 32.24 |
| system/mem_usage | % | 54.60 | 55.25 | 55.20 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 64 | 245 | 79 |
| views/all/bins | count | 7604 | 11.7 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.2 k | 14.0 k |
| views/all/pipelines | count | 245 | 298 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.8 k | 33.6 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 54 | 29 |
| views/cascade/bins | count | 2803 | 6199 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8904 | 23.5 k | 6380 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 407 | 22.9 k | 6380 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2141 | 2675 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2405 | 2996 | 0 |
| views/lamp_face/pipelines | count | 165 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2888 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2858 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8450 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8610 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 54.97 | 284 | 340.7 k |
| anim/events/count | count | 0.09 | 3 | 554 |
| assets/Image/added | count | 0.10 | 41 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.78 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.50 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.75 | 1296 | 4675 |
| encode//count | count | 32.32 | 219 | 200.3 k |
| encode/bind_walk/init_textures | count | 11.8 k | 114.7 k | 73.38 M |
| encode/bind_walk/init_textures_skipped | count | 58.1 k | 143.9 k | 359.92 M |
| encode/bind_walk/inits | count | 451 | 1115 | 2.80 M |
| encode/bind_walk/inits_skipped | count | 102 | 155 | 629.0 k |
| encode/bind_walk/merge_textures | count | 11.8 k | 114.6 k | 72.93 M |
| encode/bind_walk/merge_textures_skipped | count | 5367 | 6012 | 33.26 M |
| encode/bind_walk/merges | count | 96.72 | 333 | 599.4 k |
| encode/bind_walk/merges_skipped | count | 80.27 | 129 | 497.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6197 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6197 |
| encode/pass:early prepass/count | count | 1 | 1 | 6197 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6197 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6197 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6197 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6197 |
| encode/pass:shadow_point_light/count | count | 1.10 | 48 | 6791 |
| encode/queue/command_buffers | count | 43.43 | 277 | 269.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.5 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.89 M | 11.64 G |
| encode/queue/write_buffer/calls | count | 252 | 826 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.7 k | 3.52 M | 1.10 G |
| encode/queue/write_buffer_with/calls | count | 10.95 | 465 | 67.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 71.91 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6197 |
| physics/inserts | count | 0.36 | 110 | 2255 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 10 |
| pipelines/created | count | 0.00 | 2 | 14 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 19 | 378 |
| stream/upload/bytes | bytes | 5463 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1198 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.8 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2580.5 | main/busy/ms 2550, stage/main/post_update 2448, stage/main/update 92.82, surface/advance_cpu_ms 65.40, stage/main/pre_update 5.49, stage/main/last 1.07, surface/cell_upload_ms 0.78, stage/main/first 0.52, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.34, weather/map/ms 0.11, stream/tick/ms 0.11 |
| warm-up | 3.34 | 762.3 | main/busy/ms 2694, stage/main/post_update 2472, render/thread/ms 730, render/busy/ms 556, stage/render/prepare_assets 490, stage/main/update 206, stage/render/prepare_views 168, render/acquire/ms 168, surface/advance_cpu_ms 65.40, stage/render/create_views 28.48, stage/render/prepare_resources 24.86, stage/render/render 14.06 |
| warm-up | 3.52 | 175.4 | render/thread/ms 887, render/busy/ms 625, stage/render/prepare_assets 541, main/busy/ms 175, stage/render/prepare_views 169, render/acquire/ms 168, stage/main/update 121, stage/render/render 109, pipelines/compile_ms 96.01, render/present/ms 93.71, pipelines/driver_ms 74.24, stage/main/post_update 38.97 |
| warm-up | 3.55 | 28.6 | pipelines/compile_ms 446, pipelines/front_ms 240, pipelines/driver_ms 206, render/thread/ms 184, stage/render/render 113, render/present/ms 98.75, render/busy/ms 84.34, main/busy/ms 79.93, stage/main/post_update 54.13, stage/render/prepare_assets 53.29, terrain/update/ms 25.37, pipelines/wait_ms 19.71 |
| warm-up | 3.62 | 77.5 | pipelines/wait_ms 2339, pipelines/compile_ms 1252, pipelines/front_ms 1066, pipelines/driver_ms 186, main/busy/ms 123, render/thread/ms 77.34, render/busy/ms 66.07, stage/main/update 64.76, stage/main/post_update 48.68, stage/render/prepare_assets 43.26, terrain/update/ms 21.76, stage/render/render 21.37 |
| warm-up | 3.75 | 123.1 | pipelines/wait_ms 4711, pipelines/compile_ms 2239, pipelines/front_ms 2119, render/thread/ms 121, pipelines/driver_ms 120, render/busy/ms 119, main/busy/ms 114, stage/render/prepare_assets 98.91, stage/main/update 66.78, stage/main/pre_update 21.25, stage/main/post_update 20.72, terrain/update/ms 20.72 |
| warm-up | 3.79 | 37.7 | pipelines/wait_ms 2394, pipelines/compile_ms 1402, pipelines/front_ms 1331, render/thread/ms 106, render/busy/ms 104, stage/render/prepare_assets 83.13, pipelines/driver_ms 70.80, main/busy/ms 58.52, stage/main/pre_update 26.61, stage/main/post_update 16.92, terrain/update/ms 14.44, stage/render/render 12.51 |
| warm-up | 3.83 | 42.7 | pipelines/compile_ms 692, pipelines/front_ms 656, render/thread/ms 76.20, render/busy/ms 74.30, main/busy/ms 56.17, stage/render/prepare_assets 44.88, pipelines/wait_ms 42.59, pipelines/driver_ms 35.51, stage/main/update 26.27, stage/main/post_update 19.02, terrain/update/ms 10.74, stage/render/render 9.17 |
| warm-up | 3.98 | 82.5 | pipelines/compile_ms 1603, pipelines/front_ms 1540, render/thread/ms 72.43, render/busy/ms 70.32, pipelines/wait_ms 66.42, pipelines/driver_ms 62.18, main/busy/ms 61.71, stage/main/update 47.63, stage/render/prepare_assets 47.40, stream/drive/ms 41.92, stage/render/render 13.72, render/graph/ms 11.18 |
| warm-up | 4.14 | 104.1 | pipelines/compile_ms 463, pipelines/front_ms 448, render/thread/ms 65.91, render/busy/ms 64.31, main/busy/ms 50.41, stage/render/prepare_assets 44.75, stage/main/update 30.57, stream/drive/ms 25.87, pipelines/driver_ms 15.00, stage/render/render 12.70, stage/main/post_update 11.90, render/graph/ms 10.59 |
| warm-up | 4.17 | 38.1 | pipelines/compile_ms 492, pipelines/front_ms 476, render/thread/ms 81.25, render/busy/ms 79.47, stage/render/prepare_assets 59.85, main/busy/ms 31.91, pipelines/driver_ms 16.81, stage/render/render 13.18, stage/main/update 12.70, render/graph/ms 10.95, stage/main/post_update 10.57, stage/render/graph_submit 8.26 |
| warm-up | 4.26 | 85.2 | pipelines/compile_ms 1351, pipelines/front_ms 1306, render/thread/ms 73.70, render/busy/ms 72.33, stage/render/prepare_assets 58.96, main/busy/ms 58.11, pipelines/driver_ms 44.49, stage/main/update 41.59, stage/main/post_update 8.99, stage/render/render 8.47, render/graph/ms 6.77, encode//sum 5.42 |
| warm-up | 4.34 | 85.0 | pipelines/compile_ms 2015, pipelines/front_ms 1945, render/thread/ms 123, render/busy/ms 120, stage/render/prepare_assets 85.04, pipelines/driver_ms 69.58, main/busy/ms 63.87, stage/main/update 45.42, stage/render/render 27.06, render/graph/ms 23.70, stage/render/graph_submit 21.23, encode/submit 17.65 |
| warm-up | 4.39 | 44.2 | pipelines/compile_ms 887, pipelines/front_ms 852, pipelines/wait_ms 148, render/thread/ms 107, render/busy/ms 104, stage/render/prepare_assets 53.39, main/busy/ms 46.66, pipelines/driver_ms 35.08, stage/render/render 29.33, render/graph/ms 25.77, stage/render/graph_submit 21.63, stage/main/update 20.89 |
| warm-up | 4.47 | 82.1 | pipelines/compile_ms 651, pipelines/front_ms 628, pipelines/wait_ms 305, render/thread/ms 110, render/busy/ms 108, main/busy/ms 79.47, stage/main/post_update 56.55, stage/render/prepare_assets 52.70, encode//sum 36.37, bigspace/total/ms 24.40, pipelines/driver_ms 22.60, stage/render/render 21.12 |
| warm-up | 4.55 | 83.2 | pipelines/compile_ms 2171, pipelines/front_ms 2095, pipelines/wait_ms 583, render/thread/ms 152, render/busy/ms 150, pipelines/driver_ms 75.56, main/busy/ms 75.50, encode//sum 58.36, stage/main/post_update 55.06, stage/render/prepare_assets 47.61, stage/render/render 32.17, render/graph/ms 29.30 |
| warm-up | 4.61 | 55.9 | pipelines/compile_ms 2407, pipelines/front_ms 2328, pipelines/wait_ms 920, render/thread/ms 125, render/busy/ms 124, pipelines/driver_ms 79.42, main/busy/ms 55.96, encode//sum 38.64, stage/render/render 29.10, stage/render/prepare_assets 27.17, render/graph/ms 26.64, stage/main/post_update 24.83 |
| warm-up | 4.65 | 38.6 | pipelines/compile_ms 965, pipelines/front_ms 937, pipelines/wait_ms 755, main/busy/ms 110, render/thread/ms 76.73, render/busy/ms 74.88, stage/main/update 70.19, stream/drive/ms 57.34, stage/render/prepare_assets 35.93, pipelines/driver_ms 27.94, stage/main/post_update 24.72, stage/render/render 20.41 |
| warm-up | 4.73 | 78.0 | pipelines/compile_ms 1084, pipelines/front_ms 1051, pipelines/wait_ms 585, main/busy/ms 111, stage/main/update 79.93, render/thread/ms 58.84, stream/drive/ms 57.35, render/busy/ms 55.71, pipelines/driver_ms 33.47, stage/render/render 26.95, render/graph/ms 22.03, stage/render/prepare_assets 19.26 |
| warm-up | 4.79 | 67.6 | pipelines/compile_ms 2027, pipelines/front_ms 1972, pipelines/wait_ms 589, main/busy/ms 123, stage/main/update 95.58, render/thread/ms 72.59, render/busy/ms 69.75, pipelines/driver_ms 54.67, stream/drive/ms 49.44, stage/render/render 27.75, render/graph/ms 22.67, stage/render/prepare_assets 21.39 |
| warm-up | 4.90 | 109.2 | pipelines/compile_ms 1646, pipelines/front_ms 1601, pipelines/wait_ms 492, main/busy/ms 101, render/thread/ms 83.10, stage/main/update 81.89, render/busy/ms 81.51, stream/drive/ms 50.23, pipelines/driver_ms 45.07, stage/render/prepare_assets 42.13, stage/render/render 16.98, render/graph/ms 14.28 |
| warm-up | 4.95 | 43.6 | pipelines/compile_ms 1168, pipelines/front_ms 1128, pipelines/wait_ms 245, main/busy/ms 87.82, render/thread/ms 76.64, render/busy/ms 75.02, stage/main/update 63.62, stream/drive/ms 48.77, pipelines/driver_ms 39.96, stage/render/prepare_assets 38.77, stage/render/render 16.91, stage/main/post_update 15.30 |
| warm-up | 5.03 | 85.2 | pipelines/compile_ms 1525, pipelines/front_ms 1477, main/busy/ms 91.09, render/thread/ms 68.92, render/busy/ms 67.40, stage/main/update 66.33, pipelines/driver_ms 47.87, stream/drive/ms 47.64, stage/render/prepare_assets 38.99, stage/main/post_update 16.45, stage/render/render 13.61, pipelines/wait_ms 12.21 |
| warm-up | 5.06 | 32.5 | pipelines/compile_ms 852, pipelines/front_ms 827, main/busy/ms 70.44, render/thread/ms 58.58, render/busy/ms 57.36, stage/main/update 53.90, stage/render/prepare_assets 34.61, stream/drive/ms 33.28, pipelines/driver_ms 24.98, stage/render/render 11.88, render/graph/ms 10.17, stage/main/post_update 9.23 |
| warm-up | 5.11 | 50.1 | pipelines/compile_ms 812, pipelines/front_ms 787, main/busy/ms 63.95, stage/main/update 50.06, render/thread/ms 43.25, render/busy/ms 42.03, stream/drive/ms 34.12, pipelines/driver_ms 24.74, stage/render/prepare_assets 17.00, stage/render/render 11.77, render/graph/ms 10.07, encode//sum 7.54 |
| warm-up | 5.19 | 33.9 | pipelines/compile_ms 136, pipelines/front_ms 130, render/thread/ms 40.81, render/busy/ms 39.49, main/busy/ms 27.94, stage/render/prepare_assets 12.95, stage/main/post_update 11.79, stage/render/render 11.20, stage/main/update 10.93, render/graph/ms 9.33, encode//sum 8.66, stage/render/graph_submit 6.56 |
| warm-up | 5.31 | 76.8 | pipelines/compile_ms 289, pipelines/front_ms 279, render/thread/ms 65.98, render/busy/ms 63.30, main/busy/ms 54.79, stage/main/update 36.65, stage/render/prepare_assets 33.61, stage/render/render 22.82, render/graph/ms 19.49, stage/render/graph_submit 15.79, stage/main/post_update 11.73, stream/drive/ms 10.45 |
| warm-up | 5.34 | 28.3 | pipelines/compile_ms 527, pipelines/front_ms 509, render/thread/ms 68.19, render/busy/ms 66.03, main/busy/ms 54.14, stage/render/prepare_assets 31.18, stage/main/update 21.05, stage/render/render 20.76, pipelines/driver_ms 18.14, render/graph/ms 18.07, stage/render/graph_submit 13.93, stream/drive/ms 13.61 |
| warm-up | 5.40 | 56.8 | pipelines/compile_ms 282, pipelines/front_ms 272, render/thread/ms 60.83, render/busy/ms 59.23, main/busy/ms 51.85, pipelines/wait_ms 28.25, stage/render/prepare_assets 22.71, stage/main/update 18.74, stage/render/render 14.09, stream/drive/ms 12.39, render/graph/ms 11.94, stage/main/post_update 10.69 |
| warm-up | 5.68 | 30.1 | render/thread/ms 35.26, main/busy/ms 34.86, render/busy/ms 33.64, stage/main/update 20.19, stage/render/prepare_assets 13.36, stage/render/render 12.77, render/graph/ms 10.94, stage/main/post_update 9.82, encode//sum 8.41, stream/drive/ms 7.75, stage/render/graph_submit 7.71, encode/submit 5.72 |
| warm-up | 6.09 | 30.2 | render/thread/ms 38.98, render/busy/ms 37.97, main/busy/ms 26.17, stage/render/prepare_assets 15.25, stage/main/update 10.40, stage/render/render 9.01, stage/main/post_update 8.92, render/graph/ms 7.60, encode//sum 6.97, stage/render/graph_submit 5.60, stage/render/specialize 4.69, stage/main/pre_update 3.75 |
| warm-up | 6.35 | 32.3 | pipelines/compile_ms 224, pipelines/front_ms 217, render/thread/ms 40.51, render/busy/ms 39.59, main/busy/ms 31.97, stage/render/prepare_assets 20.06, stage/main/update 14.01, stage/main/post_update 10.93, stage/render/render 8.47, render/graph/ms 7.00, pipelines/driver_ms 6.93, encode//sum 6.17 |
| warm-up | 6.42 | 28.6 | render/thread/ms 38.21, main/busy/ms 37.45, render/busy/ms 37.25, stage/main/update 17.85, stage/render/prepare_assets 16.21, stage/main/post_update 12.55, stage/render/render 8.96, render/graph/ms 7.45, encode//sum 6.74, stage/render/graph_submit 4.89, stream/drive/ms 3.93, stage/main/pre_update 3.49 |
| warm-up | 6.59 | 31.1 | render/thread/ms 44.34, render/busy/ms 43.01, main/busy/ms 40.43, stage/main/update 22.76, stage/render/prepare_assets 22.12, stream/drive/ms 16.76, stage/main/post_update 12.01, stage/render/render 10.89, encode//sum 8.98, render/graph/ms 8.88, stage/render/graph_submit 6.26, encode/submit 3.57 |
| warm-up | 6.62 | 28.2 | render/thread/ms 47.62, render/busy/ms 46.47, stage/render/prepare_assets 27.90, main/busy/ms 25.33, stage/main/post_update 11.55, stage/render/render 10.21, stage/main/update 8.71, render/graph/ms 8.48, encode//sum 8.24, stage/render/graph_submit 6.23, encode/submit 3.73, encode/finish_wall 2.52 |
| warm-up | 6.95 | 33.4 | render/thread/ms 41.25, render/busy/ms 40.08, main/busy/ms 18.63, stage/render/render 13.31, render/graph/ms 11.62, stage/main/post_update 10.31, stage/render/specialize 10.25, stage/render/graph_submit 9.14, encode//sum 7.77, stage/render/queue 6.76, stage/main/update 5.70, encode/submit 5.48 |
| warm-up | 8.25 | 30.9 | render/thread/ms 36.65, render/busy/ms 35.54, main/busy/ms 19.21, encode//sum 11.58, stage/main/post_update 11.36, stage/render/render 11.25, render/graph/ms 9.69, stage/render/specialize 9.51, stage/render/graph_submit 7.41, stage/render/queue 6.20, stage/main/update 5.56, encode/submit 3.82 |
| measure | 0.45 | 32.6 | pipelines/compile_ms 126, pipelines/front_ms 123, render/thread/ms 37.67, render/busy/ms 36.59, main/busy/ms 19.58, stage/render/render 12.44, stage/main/post_update 12.09, render/graph/ms 10.95, encode//sum 9.57, stage/render/specialize 9.11, stage/render/graph_submit 8.13, stage/render/queue 5.94 |
| measure | 1.77 | 32.8 | render/thread/ms 39.65, render/busy/ms 38.53, main/busy/ms 17.37, stage/render/render 11.85, stage/main/post_update 10.92, render/graph/ms 10.17, stage/render/specialize 10.11, encode//sum 8.43, stage/render/graph_submit 7.58, stage/render/queue 6.62, stage/render/prepare_meshes 4.60, encode/submit 4.56 |
| measure | 3.14 | 29.4 | render/thread/ms 37.24, render/busy/ms 36.23, main/busy/ms 19.76, stage/main/post_update 13.05, stage/render/render 11.23, stage/render/specialize 9.94, render/graph/ms 9.76, encode//sum 8.50, stage/render/graph_submit 7.36, stage/render/queue 6.43, encode/submit 4.40, stage/main/update 4.04 |
| measure | 4.50 | 30.0 | render/thread/ms 36.04, render/busy/ms 35.10, main/busy/ms 19.27, stage/main/post_update 12.87, stage/render/render 10.61, stage/render/specialize 9.62, render/graph/ms 9.24, encode//sum 7.76, stage/render/graph_submit 6.80, stage/render/queue 6.46, encode/submit 3.98, stage/main/update 3.89 |
| measure | 5.83 | 29.1 | render/thread/ms 34.51, render/busy/ms 33.43, main/busy/ms 18.26, stage/main/post_update 11.86, stage/render/render 10.11, stage/render/specialize 9.37, render/graph/ms 8.61, encode//sum 6.93, stage/render/queue 6.50, stage/render/graph_submit 6.43, stage/main/update 3.90, encode/submit 3.81 |
| measure | 7.14 | 30.4 | render/thread/ms 41.76, render/busy/ms 40.63, main/busy/ms 18.34, stage/render/render 13.74, render/graph/ms 12.12, stage/main/post_update 11.38, encode//sum 10.82, stage/render/specialize 9.29, stage/render/graph_submit 8.93, encode/pass:shadow_point_light/sum 7.33, stage/render/queue 6.52, encode/submit 5.36 |
| measure | 8.45 | 29.2 | render/thread/ms 36.91, render/busy/ms 35.96, main/busy/ms 18.34, stage/render/render 12.78, stage/main/post_update 12.12, render/graph/ms 11.29, encode//sum 9.71, stage/render/specialize 9.12, stage/render/graph_submit 8.42, stage/render/queue 6.13, encode/pass:shadow_point_light/sum 5.92, encode/submit 5.14 |
| measure | 9.74 | 28.7 | render/thread/ms 40.16, render/busy/ms 39.14, main/busy/ms 19.65, stage/main/post_update 12.68, stage/render/render 10.97, stage/render/specialize 9.70, render/graph/ms 9.41, encode//sum 8.02, stage/render/graph_submit 7.03, stage/render/queue 6.23, stage/render/prepare_assets 5.09, encode/submit 4.28 |
| measure | 11.01 | 29.3 | render/thread/ms 34.79, render/busy/ms 33.75, main/busy/ms 17.75, stage/main/post_update 10.94, stage/render/render 10.76, stage/render/specialize 9.22, render/graph/ms 9.18, encode//sum 7.76, stage/render/graph_submit 7.09, stage/render/queue 5.80, encode/submit 4.21, stage/main/update 4.21 |
| measure | 15.73 | 30.5 | render/thread/ms 38.01, render/busy/ms 35.49, main/busy/ms 19.39, stage/main/post_update 12.17, stage/render/render 11.30, stage/render/specialize 10.25, render/graph/ms 8.35, encode//sum 7.65, stage/render/queue 7.22, stage/render/graph_submit 5.80, encode/submit 4.38, stage/main/update 3.91 |
| measure | 17.06 | 31.6 | render/thread/ms 35.52, render/busy/ms 34.52, main/busy/ms 19.28, stage/main/post_update 10.93, stage/render/render 10.37, stage/render/specialize 9.88, render/graph/ms 8.88, encode//sum 8.27, stage/render/queue 6.89, stage/render/graph_submit 6.35, stage/main/update 5.92, encode/submit 3.95 |
| measure | 18.01 | 28.5 | render/thread/ms 34.17, render/busy/ms 33.21, main/busy/ms 19.68, stage/main/post_update 12.05, stage/render/render 9.43, stage/render/specialize 9.39, render/graph/ms 8.02, encode//sum 7.52, stage/render/queue 6.48, stage/render/graph_submit 6.12, stage/main/update 4.29, encode/submit 3.61 |
| measure | 18.16 | 32.9 | main/busy/ms 38.31, stage/main/update 23.93, stream/drive/ms 19.93, render/thread/ms 18.31, render/busy/ms 17.35, stage/main/post_update 11.29, stage/render/render 10.18, render/graph/ms 8.75, encode//sum 8.44, stage/render/graph_submit 6.43, encode/submit 3.86, encode/finish_wall 2.65 |
| measure | 19.38 | 29.2 | render/thread/ms 35.14, render/busy/ms 34.07, main/busy/ms 18.49, stage/main/post_update 10.92, stage/render/render 10.41, stage/render/specialize 9.21, encode//sum 9.13, render/graph/ms 8.96, stage/render/queue 6.91, stage/render/graph_submit 6.75, stage/main/update 4.16, encode/submit 4.13 |
| measure | 20.72 | 28.3 | render/thread/ms 34.93, render/busy/ms 33.86, main/busy/ms 19.00, stage/main/post_update 11.36, stage/render/render 9.69, stage/render/specialize 9.60, render/graph/ms 8.32, stage/render/queue 6.91, encode//sum 6.68, stage/render/graph_submit 6.04, stage/main/update 4.47, encode/submit 3.60 |
| measure | 23.00 | 33.4 | render/thread/ms 41.79, render/busy/ms 40.74, main/busy/ms 24.62, stage/main/post_update 12.65, stage/render/render 12.00, stage/render/specialize 11.58, render/graph/ms 10.52, encode//sum 8.76, stage/render/graph_submit 8.29, stage/render/queue 7.87, stage/main/update 6.28, encode/submit 4.90 |
| measure | 24.46 | 28.7 | render/thread/ms 37.77, render/busy/ms 36.77, main/busy/ms 19.03, stage/main/post_update 12.23, stage/render/render 12.06, render/graph/ms 10.70, stage/render/specialize 9.73, encode//sum 8.90, stage/render/graph_submit 8.42, stage/render/queue 6.72, encode/submit 4.59, stage/main/update 4.31 |
| measure | 27.29 | 35.4 | render/thread/ms 46.42, render/busy/ms 45.37, main/busy/ms 25.81, stage/render/render 17.31, render/graph/ms 15.71, encode//sum 15.62, stage/main/post_update 13.44, stage/render/graph_submit 11.98, stage/render/specialize 11.85, encode/pass:shadow_point_light/sum 11.42, encode/submit 7.84, stage/render/queue 7.34 |
| measure | 30.47 | 34.9 | render/thread/ms 39.67, render/busy/ms 38.62, main/busy/ms 21.13, stage/main/post_update 12.09, stage/render/render 11.67, stage/render/specialize 10.49, render/graph/ms 10.17, stage/render/graph_submit 8.03, encode//sum 7.74, stage/render/queue 7.00, stage/main/update 6.01, encode/submit 4.42 |
| measure | 31.88 | 29.7 | render/thread/ms 37.09, render/busy/ms 36.11, main/busy/ms 18.60, stage/main/post_update 11.91, stage/render/render 10.84, stage/render/specialize 10.51, render/graph/ms 9.44, encode//sum 7.59, stage/render/graph_submit 7.55, stage/render/queue 6.66, stage/main/update 4.26, encode/submit 4.18 |
| measure | 35.34 | 29.7 | render/thread/ms 37.59, render/busy/ms 36.61, main/busy/ms 18.78, stage/main/post_update 11.51, stage/render/render 10.93, stage/render/specialize 10.62, render/graph/ms 9.55, stage/render/graph_submit 7.35, encode//sum 6.82, stage/render/queue 6.45, stage/main/update 4.27, encode/submit 3.73 |
| measure | 40.79 | 33.4 | render/thread/ms 35.13, render/busy/ms 33.82, main/busy/ms 17.85, stage/main/post_update 10.24, stage/render/render 9.45, stage/render/specialize 9.24, render/graph/ms 7.89, encode//sum 7.10, stage/render/graph_submit 6.17, stage/render/queue 5.86, stage/main/update 4.30, encode/submit 3.71 |
| measure | 43.21 | 29.9 | render/thread/ms 38.06, render/busy/ms 37.09, main/busy/ms 17.14, stage/render/render 10.85, stage/render/specialize 10.35, stage/main/post_update 10.04, render/graph/ms 9.55, stage/render/graph_submit 7.79, stage/render/queue 6.75, encode//sum 6.70, stage/main/update 4.11, encode/finish_wall 4.08 |
| measure | 45.65 | 33.2 | render/thread/ms 39.71, render/busy/ms 38.51, main/busy/ms 18.48, stage/render/render 11.17, stage/render/specialize 10.39, stage/main/post_update 10.13, render/graph/ms 9.71, encode//sum 7.86, stage/render/graph_submit 7.75, stage/render/queue 6.52, stage/main/update 4.87, stage/render/prepare_meshes 4.27 |
| measure | 59.03 | 41.2 | render/thread/ms 53.79, render/busy/ms 52.32, main/busy/ms 31.46, stage/render/render 18.75, stage/main/post_update 18.48, render/graph/ms 16.83, stage/render/graph_submit 13.60, stage/render/specialize 11.03, encode//sum 10.76, stage/main/update 8.08, stage/render/prepare_meshes 7.30, encode/submit 7.16 |
| measure | 59.07 | 37.9 | render/thread/ms 57.15, render/busy/ms 56.12, main/busy/ms 24.84, stage/render/prepare_assets 15.70, stage/main/post_update 12.86, stage/render/specialize 11.14, stage/render/render 9.61, stage/render/prepare_meshes 8.31, render/graph/ms 8.30, encode//sum 8.00, stage/main/update 6.31, stage/render/graph_submit 6.30 |
| measure | 62.48 | 30.2 | render/thread/ms 37.70, render/busy/ms 35.00, main/busy/ms 18.73, stage/render/specialize 10.54, stage/render/render 10.52, stage/main/post_update 9.52, encode//sum 7.66, render/graph/ms 7.60, stage/render/queue 6.40, stage/main/update 5.80, stage/render/graph_submit 5.69, stage/render/prepare_meshes 4.34 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=daygate-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **80%** (min 58%) · 2623 MHz (min 2610) · memory 8101 MHz · 82.3 W · 78 °C (max 83) · VRAM 7060 MiB · throttled 100% of samples

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
- `shadow.lamp.day_gate=on (command line: --set)`
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
