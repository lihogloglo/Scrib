# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: contactshadows · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=custom (preset: derived from its members); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=off (command line: --off contactshadows); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=off (command line: --off contactshadows); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6574 frames over 65.6 s · **median 9.89 ms (101 fps)** · mean 9.99 · p95 11.55 · p99 13.47 (1% low 74 fps) · worst 32.38 ms · 31 spikes over 1.5× median · 27.4 stutters/min (work 23.8/min)
- warm-up: 353 frames over 8.9 s, worst 2325.3 ms
- hitches (> 25.0 ms): **3** measured, 38 in warm-up
- timed GPU passes: 6.63 ms of a 9.99 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.89 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 32.38 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.916 | 2.035 | 2.068 | 0.94 | 0.178 |
| main_opaque_pass_3d | 1.470 | 1.560 | 2.352 | 0.94 | 0.063 |
| early prepass | 0.489 | 0.519 | 0.754 | 0.94 | 0.087 |
| bloom | 0.299 | 0.317 | 0.325 | 0.94 | 0.064 |
| atmosphere_luts | 0.295 | 0.313 | 0.377 | 0.94 | 0.003 |
| water_velocity | 0.212 | 0.225 | 0.247 | 0.94 | 0.002 |
| water_sspr | 0.190 | 0.202 | 0.203 | 0.94 | 0.002 |
| ssao | 0.177 | 0.188 | 0.191 | 0.94 | 0.007 |
| point_shadow_atlas | 0.161 | 0.290 | 0.536 | 0.56 | 0.230 |
| upscaling | 0.121 | 0.128 | 0.135 | 0.94 | 0.004 |
| atmos_froxel_inject | 0.121 | 0.128 | 0.130 | 0.94 | 0.001 |
| water_fft | 0.116 | 0.123 | 0.126 | 0.94 | 0.003 |
| uniform_allocation | 0.111 | 0.118 | 0.143 | 0.94 | 0.021 |
| clouds_composite | 0.099 | 0.105 | 0.109 | 0.94 | 0.002 |
| bin_unpacking | 0.088 | 0.093 | 0.114 | 0.94 | 0.016 |
| gpu_instance_materialization | 0.087 | 0.092 | 0.096 | 0.94 | 0.004 |
| tonemapping | 0.082 | 0.087 | 0.091 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.065 | 0.069 | 0.128 | 0.94 | 0.008 |
| clustering | 0.060 | 0.064 | 0.068 | 0.94 | 0.019 |
| clouds_trace | 0.053 | 0.056 | 0.078 | 0.94 | 0.001 |
| early_mesh_preprocessing | 0.052 | 0.055 | 0.071 | 0.94 | 0.003 |
| render_sky | 0.050 | 0.053 | 0.054 | 0.94 | 0.002 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.040 | 0.413 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.035 | 0.057 | 0.057 | 0.62 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.031 | 0.94 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.028 | 0.050 | 0.076 | 0.56 | 0.009 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.94 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.219 | 0.230 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.94 | 0.002 |
| range_unpacking | 0.021 | 0.022 | 0.027 | 0.94 | 0.003 |
| point_shadow_atlas/uniform_allocation | 0.019 | 0.034 | 0.076 | 0.56 | 0.010 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.94 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.043 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.94 | 0.183 |
| point_shadow_atlas/bin_unpacking | 0.013 | 0.024 | 0.046 | 0.56 | 0.007 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.94 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.56 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.94 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.817 | 10.188 | 20.02 | 57963.5 |
| render/busy/ms | 8.310 | 9.699 | 19.53 | 54632.3 |
| main/busy/ms | 8.117 | 9.444 | 29.91 | 53361.0 |
| stage/render/render | 5.127 | 6.031 | 12.08 | 33706.4 |
| stage/main/post_update | 4.580 | 5.483 | 9.54 | 30105.7 |
| render/graph/ms | 4.472 | 5.307 | 10.68 | 29396.9 |
| encode//sum | 3.196 | 3.929 | 7.88 | 21008.9 |
| stage/render/graph_submit | 3.172 | 3.877 | 7.62 | 20852.3 |
| encode/submit | 1.974 | 2.663 | 5.40 | 12978.3 |
| stage/main/update | 1.850 | 2.341 | 23.85 | 12163.5 |
| encode/finish_wall | 1.259 | 1.674 | 3.35 | 8277.2 |
| stage/render/camera_driver | 1.238 | 1.500 | 2.96 | 8135.4 |
| encode/pass:early prepass/sum | 0.833 | 1.082 | 2.15 | 5477.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.823 | 1.080 | 1.77 | 5413.3 |
| encode/pass:shadow_directional_light/sum | 0.819 | 1.234 | 2.11 | 5385.8 |
| stage/render/prepare_write_phase_buffers | 0.639 | 0.957 | 1.83 | 4200.0 |
| bigspace/total/ms | 0.552 | 1.325 | 2.25 | 3630.9 |
| stage/render/prepare_assets | 0.510 | 0.669 | 11.48 | 3355.7 |
| stage/main/fixed_main_loop | 0.499 | 0.900 | 2.08 | 3278.0 |
| stage/render/prepare_meshes | 0.432 | 0.541 | 3.80 | 2842.4 |
| stage/render/prepare_resources | 0.423 | 0.500 | 1.56 | 2782.1 |
| render/present/ms | 0.406 | 1.092 | 1.91 | 2671.5 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.393 | 1.354 | 2.26 | 2583.0 |
| stage/main/pre_update | 0.389 | 0.564 | 2.91 | 2555.4 |
| stage/main/first | 0.366 | 0.486 | 5.67 | 2403.6 |
| stage/render/prepare_flush | 0.318 | 0.524 | 1.32 | 2091.2 |
| controller/feed/ms | 0.318 | 0.401 | 1.12 | 2090.6 |
| stage/render/prepare_bind_groups | 0.307 | 0.509 | 1.81 | 2017.2 |
| stage/render/prepare_batch_phases | 0.272 | 0.417 | 0.72 | 1787.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.264 | 0.435 | 0.95 | 1736.7 |
| stage/main/last | 0.255 | 0.303 | 0.79 | 1678.6 |
| bigspace/low_precision/ms | 0.245 | 0.745 | 2.04 | 1610.4 |
| stage/render/create_views | 0.201 | 0.274 | 0.60 | 1319.3 |
| stream/drive/ms | 0.200 | 0.280 | 10.74 | 1313.5 |
| stage/render/extract_commands | 0.185 | 0.218 | 0.59 | 1218.2 |
| bigspace/high_precision/ms | 0.160 | 0.650 | 1.62 | 1051.3 |
| anim/evaluate/ms | 0.156 | 0.277 | 1.27 | 1026.8 |
| stage/render/specialize | 0.154 | 0.247 | 1.71 | 1015.1 |
| anim/evaluate_cpu/ms | 0.149 | 0.190 | 0.64 | 981.7 |
| stream/tick/ms | 0.148 | 0.214 | 0.48 | 971.5 |
| stage/render/prepare_views | 0.142 | 0.184 | 1.91 | 931.7 |
| render/acquire/ms | 0.100 | 0.130 | 1.85 | 659.7 |
| stage/render/queue | 0.097 | 0.170 | 1.53 | 639.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.088 | 0.140 | 0.63 | 580.5 |
| stage/render/phase_sort | 0.080 | 0.112 | 0.31 | 527.8 |
| controller/write/ms | 0.073 | 0.100 | 0.49 | 481.9 |
| controller/move/ms | 0.067 | 0.190 | 0.64 | 442.1 |
| terrain/update/ms | 0.059 | 0.288 | 1.82 | 384.9 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.057 | 0.000 | 3.15 | 374.4 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.053 | 0.289 | 1.48 | 347.2 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.074 | 0.39 | 303.5 |
| pipelines/compile_ms | 0.043 | 0.000 | 114.85 | 280.0 |
| pipelines/front_ms | 0.041 | 0.000 | 112.03 | 271.8 |
| physics/step/ms | 0.039 | 0.081 | 0.49 | 253.1 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.53 | 206.8 |
| physics/insert/ms | 0.024 | 0.280 | 0.86 | 158.7 |
| stage/render/cleanup | 0.023 | 0.031 | 0.36 | 150.5 |
| terrain/select/ms | 0.022 | 0.255 | 0.72 | 142.4 |
| weather/map/ms | 0.018 | 0.024 | 0.26 | 115.5 |
| far/select/ms | 0.015 | 0.168 | 0.36 | 96.1 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.54 | 84.2 |
| anim/apply/ms | 0.011 | 0.020 | 0.37 | 74.6 |
| weather/eval/ms | 0.011 | 0.014 | 0.23 | 70.0 |
| encode/camera_driver_span_end/sum | 0.010 | 0.020 | 0.27 | 68.9 |
| script/update/ms | 0.008 | 0.012 | 0.07 | 50.9 |
| controller/avoid/ms | 0.006 | 0.012 | 0.06 | 38.2 |
| stream/despawn/ms | 0.005 | 0.000 | 1.30 | 35.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 31.0 |
| script/ui/ms | 0.003 | 0.005 | 0.15 | 22.1 |
| stream/spawn/ms | 0.003 | 0.000 | 1.12 | 22.1 |
| stream/upload/ms | 0.003 | 0.000 | 1.09 | 18.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 12.8 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.82 | 8.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 7.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.10 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 18 | 32 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 14 |
| crowd/c0/joints/count | count | 1306 | 2262 | 659 |
| crowd/c0/meshes/count | count | 355 | 646 | 171 |
| crowd/c1/count | count | 145 | 156 | 147 |
| crowd/c1/joints/count | count | 5507 | 5982 | 5537 |
| crowd/c1/meshes/count | count | 1167 | 1284 | 1170 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 215 | 206 |
| crowd/joints_extracted/count | count | 709 | 6962 | 650 |
| crowd/joints_uploaded/count | count | 3611 | 6186 | 5807 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.40 | 1.28 | 0.45 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.54 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.27 | 0.01 |
| encode/finish_critical | ms | 1.01 | 3.15 | 0.91 |
| encode/pass:early prepass/max | ms | 0.83 | 2.15 | 0.76 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 1.77 | 0.72 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.63 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 0.95 | 0.25 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 1.03 | 2.26 | 0.75 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.25 | 1.48 | 0.12 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.29 | 3.15 | 1.35 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.11 | 0.91 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.60 | 29.91 | 8.82 |
| gpu/timed/ms | ms | 7.70 | 9.36 | 8.82 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 768 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2479 | 3522 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 768 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2479 | 3522 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 62 | 354 | 8 |
| phases/shadow/bins | count | 814 | 1392 | 671 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2294 | 3147 | 2035 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 48 | 135 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2679 | 3182 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 15 | 284 | 11 |
| preprocess/bind_groups_created | count | 10 | 274 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 140 | 250 | 123 |
| process/cpu_usage | % | 25 | 38.46 | 23.08 |
| process/mem_usage | GiB | 5.18 | 5.39 | 5.39 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 1 | 0 |
| shadow_atlas/lamps | count | 8 | 21 | 0 |
| shadow_atlas/live_occ_pct | % | 11.82 | 39.36 | 0 |
| shadow_atlas/live_tiles | count | 18 | 63 | 0 |
| shadow_atlas/mover_draws | count | 8 | 38 | 0 |
| shadow_atlas/movers | count | 151 | 389 | 0 |
| shadow_atlas/moving_lamps | count | 1 | 3 | 0 |
| shadow_atlas/render/blits | count | 8 | 38 | 0 |
| shadow_atlas/render/clears | count | 0 | 22 | 0 |
| shadow_atlas/render/lamps | count | 8 | 21 | 0 |
| shadow_atlas/render/live_direct | count | 8 | 38 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 16 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 16 | 57 | 0 |
| shadow_atlas/resizes | count | 30 | 39 | 39 |
| shadow_atlas/static_occ_pct | % | 16.41 | 36.33 | 0 |
| shadow_atlas/static_tiles | count | 42 | 108 | 0 |
| shadow_atlas/views | count | 16 | 57 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 121 | 14 |
| skin/blocks_skipped/count | count | 146 | 203 | 192 |
| skin/blocks_walked/count | count | 14 | 28 | 14 |
| skin/joint_bytes_written/bytes | bytes | 49.4 k | 809.7 k | 46.8 k |
| skin/joints_written/count | count | 772 | 12.7 k | 732 |
| skin/upload_writes/count | count | 6 | 23 | 9 |
| started_load_count |  | 6445 | 6569 | 6569 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.42 | 53.76 | 29.32 |
| system/mem_usage | % | 47.17 | 47.84 | 47.83 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 123 | 413 | 66 |
| views/all/bins | count | 2935 | 3406 | 2961 |
| views/all/count | count | 19 | 60 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8039 | 10.3 k | 7977 |
| views/all/pipelines | count | 96 | 177 | 66 |
| views/all/pipelines_distinct | count | 67 | 75 | 61 |
| views/all/work_items | count | 154.1 k | 230.8 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 716 | 885 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2112 | 2439 | 2035 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 45 | 324 | 0 |
| views/lamp_face/bins | count | 110 | 628 | 0 |
| views/lamp_face/count | count | 16 | 57 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 160 | 798 | 0 |
| views/lamp_face/pipelines | count | 23 | 110 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 92 | 25.2 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2000 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7464 | 5942 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 104.6 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.3 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 44.55 | 214 | 292.9 k |
| anim/events/count | count | 0.08 | 3 | 542 |
| assets/Image/added | count | 0.09 | 36 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.44 | 280 | 2874 |
| assets/Mesh/removed | count | 0.74 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.14 | 10 | 7477 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.23 | 1260 | 1489 |
| encode//count | count | 27.91 | 30 | 183.5 k |
| encode/bind_walk/init_textures | count | 9545 | 25.3 k | 62.75 M |
| encode/bind_walk/init_textures_skipped | count | 58.6 k | 84.2 k | 385.12 M |
| encode/bind_walk/inits | count | 575 | 773 | 3.78 M |
| encode/bind_walk/inits_skipped | count | 174 | 259 | 1.14 M |
| encode/bind_walk/merge_textures | count | 9482 | 25.2 k | 62.33 M |
| encode/bind_walk/merge_textures_skipped | count | 11.9 k | 26.4 k | 77.95 M |
| encode/bind_walk/merges | count | 108 | 198 | 709.1 k |
| encode/bind_walk/merges_skipped | count | 153 | 231 | 1.01 M |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6574 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6574 |
| encode/pass:early prepass/count | count | 1 | 1 | 6574 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6574 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6574 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6574 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.39 | 1 | 2552 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.14 | 1 | 921 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 280 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6574 |
| encode/queue/command_buffers | count | 38.49 | 42 | 253.1 k |
| encode/queue/submits | count | 2.01 | 4 | 13.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.25 M | 15.29 G |
| encode/queue/write_buffer/calls | count | 281 | 484 | 1.85 M |
| encode/queue/write_buffer_with/bytes | bytes | 166.2 k | 1.30 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 9.30 | 848 | 61.1 k |
| encode/queue/write_texture/bytes | bytes | 1848 | 589.8 k | 12.15 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 556 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6574 |
| physics/inserts | count | 0.34 | 101 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 175 | 2335 |
| physics/steps | count | 0.60 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 4 |
| pipelines/created | count | 0.00 | 1 | 5 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 31 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5149 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1129 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.33 | 2325.3 | main/busy/ms 2298, stage/main/post_update 2256, stage/main/update 30.86, stage/main/pre_update 7.03, stage/main/last 1.05, surface/advance_cpu_ms 0.80, stage/main/first 0.55, stage/main/fixed_main_loop 0.44, surface/cell_upload_ms 0.35, stream/drive/ms 0.33, bigspace/total/ms 0.09, stream/tick/ms 0.07 |
| warm-up | 3.09 | 760.3 | main/busy/ms 2430, stage/main/post_update 2277, render/thread/ms 728, render/busy/ms 609, stage/render/prepare_assets 556, stage/main/update 136, stage/render/prepare_views 116, render/acquire/ms 116, stage/render/create_views 30.04, stage/render/render 14.75, render/graph/ms 11.25, stage/render/camera_driver 9.22 |
| warm-up | 3.20 | 110.2 | render/thread/ms 822, render/busy/ms 648, stage/render/prepare_assets 579, main/busy/ms 153, stage/render/prepare_views 117, render/acquire/ms 117, stage/main/update 110, stage/render/render 75.91, pipelines/compile_ms 62.01, render/present/ms 58.01, pipelines/driver_ms 45.84, stage/main/post_update 31.98 |
| warm-up | 3.37 | 155.1 | pipelines/wait_ms 4602, pipelines/compile_ms 1646, pipelines/front_ms 1536, render/thread/ms 138, render/busy/ms 134, pipelines/driver_ms 111, encode//sum 73.30, main/busy/ms 62.44, stage/main/post_update 35.26, stage/render/prepare_write_phase_buffers 34.18, stage/render/prepare_flush 30.66, stage/render/render 29.09 |
| warm-up | 3.41 | 44.9 | pipelines/wait_ms 4795, pipelines/compile_ms 1921, pipelines/front_ms 1807, render/thread/ms 156, render/busy/ms 154, pipelines/driver_ms 114, main/busy/ms 95.80, encode//sum 76.67, stage/main/update 52.48, stage/render/prepare_assets 36.88, stage/render/prepare_write_phase_buffers 35.55, stage/render/prepare_flush 30.77 |
| warm-up | 3.53 | 121.1 | pipelines/compile_ms 1628, pipelines/front_ms 1548, pipelines/wait_ms 400, render/thread/ms 106, render/busy/ms 104, main/busy/ms 88.51, stage/render/prepare_assets 82.92, pipelines/driver_ms 80.34, stage/main/update 55.24, terrain/update/ms 18.23, stage/main/post_update 18.07, stage/render/render 11.50 |
| warm-up | 3.56 | 31.7 | pipelines/compile_ms 2096, pipelines/front_ms 1981, pipelines/wait_ms 241, pipelines/driver_ms 116, render/thread/ms 105, render/busy/ms 103, stage/render/prepare_assets 81.80, main/busy/ms 36.53, stage/main/post_update 16.70, stage/render/render 14.99, encode//sum 13.24, render/graph/ms 12.73 |
| warm-up | 3.61 | 27.5 | pipelines/compile_ms 145, pipelines/front_ms 137, render/thread/ms 42.35, render/busy/ms 36.13, main/busy/ms 31.34, encode//sum 25.83, stage/render/render 20.50, render/graph/ms 18.13, stage/main/post_update 17.23, stage/render/graph_submit 15.77, encode/finish_wall 10.19, stage/main/update 9.74 |
| warm-up | 3.75 | 118.9 | pipelines/compile_ms 898, pipelines/front_ms 859, main/busy/ms 77.06, render/thread/ms 68.23, render/busy/ms 66.23, stage/main/update 59.72, stage/render/prepare_assets 39.64, pipelines/driver_ms 38.91, stage/render/render 19.70, render/graph/ms 17.29, stage/render/graph_submit 14.45, stage/main/post_update 10.63 |
| warm-up | 3.79 | 43.8 | pipelines/compile_ms 1501, pipelines/front_ms 1444, render/thread/ms 72.89, render/busy/ms 71.18, pipelines/driver_ms 56.69, stage/render/prepare_assets 51.89, main/busy/ms 21.42, stage/render/render 14.56, render/graph/ms 12.69, stage/render/graph_submit 10.21, stage/main/update 10.01, pipelines/wait_ms 8.81 |
| warm-up | 3.89 | 29.1 | pipelines/compile_ms 77.45, pipelines/front_ms 65.40, render/thread/ms 37.22, render/busy/ms 35.80, main/busy/ms 35.58, stage/main/update 22.52, stage/render/render 13.88, stage/render/prepare_assets 13.45, pipelines/driver_ms 12.05, render/graph/ms 12.03, stream/drive/ms 11.23, stage/render/graph_submit 8.90 |
| warm-up | 3.92 | 31.7 | pipelines/compile_ms 573, pipelines/front_ms 543, main/busy/ms 47.05, render/thread/ms 44.24, render/busy/ms 42.18, pipelines/driver_ms 30.48, stage/main/update 29.24, stage/render/prepare_assets 23.25, stream/drive/ms 18.78, stage/render/render 12.85, stage/main/post_update 11.07, render/graph/ms 10.23 |
| warm-up | 3.97 | 52.7 | pipelines/compile_ms 755, pipelines/front_ms 720, render/thread/ms 59.45, main/busy/ms 58.35, render/busy/ms 56.70, pipelines/driver_ms 34.98, stage/main/update 34.87, stage/render/prepare_assets 32.40, stream/drive/ms 20.83, stage/main/post_update 16.99, stage/render/render 15.12, render/graph/ms 11.98 |
| warm-up | 4.03 | 61.2 | pipelines/compile_ms 1076, pipelines/front_ms 1031, render/thread/ms 70.76, render/busy/ms 68.69, main/busy/ms 45.48, pipelines/driver_ms 44.37, stage/render/prepare_assets 43.83, stage/main/update 20.22, stage/main/post_update 16.12, stage/render/render 13.39, render/graph/ms 10.91, encode//sum 6.82 |
| warm-up | 4.09 | 61.8 | pipelines/compile_ms 1061, pipelines/front_ms 1026, main/busy/ms 169, stage/main/update 150, render/thread/ms 77.14, render/busy/ms 75.53, stage/render/prepare_assets 52.97, pipelines/driver_ms 34.61, stage/render/render 11.92, stage/main/post_update 10.56, render/graph/ms 9.69, stage/render/graph_submit 6.18 |
| warm-up | 4.30 | 207.3 | pipelines/compile_ms 2559, pipelines/front_ms 2486, main/busy/ms 165, stage/main/update 149, render/thread/ms 114, render/busy/ms 113, pipelines/driver_ms 72.87, stage/render/prepare_assets 62.76, stage/render/render 41.75, render/graph/ms 39.82, stage/render/graph_submit 37.38, encode/finish_wall 33.26 |
| warm-up | 4.41 | 84.5 | main/busy/ms 99.70, render/thread/ms 88.16, render/busy/ms 85.57, pipelines/compile_ms 79.88, pipelines/front_ms 76.54, stage/render/render 48.89, stage/main/update 46.12, render/graph/ms 45.96, stage/main/post_update 42.45, stage/render/graph_submit 35.28, stage/render/prepare_assets 11.89, encode/submit 10.90 |
| warm-up | 4.46 | 45.5 | pipelines/compile_ms 1033, pipelines/front_ms 1003, pipelines/wait_ms 117, main/busy/ms 105, render/thread/ms 105, render/busy/ms 103, stage/main/post_update 41.41, stage/render/render 39.12, render/graph/ms 36.71, stage/main/update 35.70, stage/render/prepare_assets 30.89, pipelines/driver_ms 29.89 |
| warm-up | 4.52 | 66.1 | pipelines/compile_ms 1976, pipelines/front_ms 1916, pipelines/wait_ms 313, render/thread/ms 85.02, render/busy/ms 83.56, main/busy/ms 63.22, pipelines/driver_ms 59.50, stage/render/prepare_assets 55.82, stage/main/pre_update 20.88, stage/main/update 15.84, stage/main/post_update 13.94, stage/render/render 11.41 |
| warm-up | 4.60 | 79.5 | pipelines/compile_ms 1529, pipelines/front_ms 1475, pipelines/wait_ms 218, render/thread/ms 122, render/busy/ms 121, main/busy/ms 110, stage/main/update 80.19, stage/render/prepare_assets 57.31, pipelines/driver_ms 54.05, stage/render/prepare_batch_phases 31.87, stage/main/post_update 13.97, stage/render/render 13.18 |
| warm-up | 4.70 | 102.6 | pipelines/compile_ms 2256, pipelines/front_ms 2186, render/thread/ms 134, render/busy/ms 133, main/busy/ms 132, pipelines/wait_ms 112, stage/main/update 101, pipelines/driver_ms 70.15, stage/render/prepare_assets 69.25, stage/render/prepare_batch_phases 31.85, stream/drive/ms 23.61, stage/main/post_update 15.17 |
| warm-up | 4.75 | 51.5 | pipelines/compile_ms 2165, pipelines/front_ms 2079, render/thread/ms 105, render/busy/ms 103, main/busy/ms 101, pipelines/wait_ms 91.07, pipelines/driver_ms 85.78, stage/main/update 73.17, stage/render/prepare_assets 68.89, stream/drive/ms 56.83, stage/render/render 18.83, render/graph/ms 16.85 |
| warm-up | 4.79 | 38.4 | pipelines/compile_ms 483, pipelines/front_ms 443, main/busy/ms 85.85, render/thread/ms 74.43, render/busy/ms 72.65, stage/main/update 67.20, pipelines/driver_ms 40.33, stream/drive/ms 38.10, stage/render/prepare_assets 27.44, stage/render/render 24.34, render/graph/ms 21.96, stage/render/graph_submit 14.18 |
| warm-up | 4.83 | 41.0 | pipelines/compile_ms 135, pipelines/front_ms 129, render/thread/ms 53.83, main/busy/ms 53.01, render/busy/ms 52.22, stage/main/update 35.54, stage/render/render 17.49, render/graph/ms 15.21, stage/render/prepare_assets 15.16, stream/drive/ms 10.76, stage/main/post_update 9.98, encode//sum 9.26 |
| warm-up | 4.88 | 26.6 | pipelines/compile_ms 148, pipelines/front_ms 142, render/thread/ms 42.36, render/busy/ms 41.11, main/busy/ms 33.81, stage/main/update 16.78, stage/render/render 13.20, render/graph/ms 11.30, stage/main/post_update 10.61, stage/render/prepare_write_phase_buffers 8.57, encode//sum 7.92, stage/render/graph_submit 7.92 |
| warm-up | 4.92 | 33.7 | pipelines/compile_ms 495, pipelines/front_ms 479, render/thread/ms 53.62, render/busy/ms 52.11, main/busy/ms 46.14, stage/main/update 28.93, pipelines/driver_ms 16.39, stage/render/render 13.80, render/graph/ms 11.81, stage/main/post_update 10.63, stage/render/prepare_assets 9.58, stage/render/graph_submit 8.30 |
| warm-up | 4.95 | 32.6 | pipelines/compile_ms 795, pipelines/front_ms 771, render/thread/ms 55.21, render/busy/ms 53.81, main/busy/ms 43.20, stage/main/update 27.81, pipelines/driver_ms 24.24, stage/render/prepare_assets 17.07, stage/render/render 12.05, render/graph/ms 10.29, stage/main/post_update 8.76, stage/render/graph_submit 6.77 |
| warm-up | 5.01 | 25.6 | pipelines/compile_ms 466, pipelines/front_ms 450, main/busy/ms 40.08, render/thread/ms 33.76, render/busy/ms 31.90, stage/render/render 16.01, pipelines/driver_ms 15.42, stage/main/update 15.13, render/graph/ms 13.71, stage/main/post_update 12.01, stage/render/graph_submit 9.19, stage/main/pre_update 6.91 |
| warm-up | 5.65 | 28.5 | render/thread/ms 38.69, render/busy/ms 37.13, main/busy/ms 32.25, stage/main/update 14.78, stage/render/render 14.34, stage/main/post_update 12.99, render/graph/ms 12.07, stage/render/prepare_assets 11.89, encode//sum 9.55, stage/render/graph_submit 7.65, encode/submit 4.70, stage/render/camera_driver 4.20 |
| warm-up | 5.71 | 38.0 | pipelines/compile_ms 133, pipelines/front_ms 131, render/thread/ms 42.87, render/busy/ms 41.75, main/busy/ms 29.11, stage/render/prepare_assets 21.51, stage/main/update 12.83, stage/render/render 9.96, stage/main/post_update 9.58, render/graph/ms 8.28, encode//sum 7.17, stage/render/graph_submit 5.48 |
| warm-up | 5.89 | 31.1 | pipelines/compile_ms 131, pipelines/front_ms 128, render/thread/ms 40.36, render/busy/ms 39.14, main/busy/ms 34.55, stage/main/update 20.57, stage/render/render 14.46, render/graph/ms 12.74, stage/render/prepare_assets 9.77, stage/main/post_update 9.07, encode//sum 8.83, stage/render/graph_submit 8.65 |
| warm-up | 5.96 | 32.2 | main/busy/ms 42.97, render/thread/ms 40.31, render/busy/ms 38.86, stage/main/update 21.62, stage/main/post_update 15.34, stage/render/render 13.78, render/graph/ms 11.68, pipelines/compile_ms 10.35, pipelines/front_ms 9.69, stage/render/prepare_assets 9.65, stage/render/graph_submit 7.75, encode//sum 7.60 |
| warm-up | 5.99 | 37.3 | render/thread/ms 50.87, render/busy/ms 49.82, main/busy/ms 40.01, stage/main/update 18.84, stage/render/prepare_assets 16.08, stage/render/render 13.15, stage/main/post_update 12.43, render/graph/ms 11.33, stage/render/graph_submit 8.33, encode//sum 6.33, stage/render/prepare_meshes 5.81, encode/finish_wall 5.41 |
| warm-up | 6.03 | 39.0 | render/thread/ms 66.18, render/busy/ms 65.09, main/busy/ms 44.08, stage/main/update 20.35, stage/render/render 20.01, stage/render/prepare_assets 18.63, render/graph/ms 17.91, stage/main/post_update 15.06, stage/render/graph_submit 13.58, encode/finish_wall 9.25, encode/pass:point_shadow_atlas_live_indirect/sum 8.78, encode//sum 8.11 |
| warm-up | 6.10 | 38.0 | pipelines/compile_ms 144, pipelines/front_ms 140, render/thread/ms 47.48, render/busy/ms 46.49, main/busy/ms 36.70, stage/main/update 20.85, stage/render/prepare_assets 17.15, stage/render/render 15.78, render/graph/ms 14.19, stage/render/graph_submit 11.03, stage/main/post_update 9.45, encode/finish_wall 7.24 |
| warm-up | 6.25 | 42.3 | main/busy/ms 58.89, render/thread/ms 47.10, render/busy/ms 45.62, stage/main/update 42.88, stage/render/render 22.35, render/graph/ms 19.99, stage/render/graph_submit 16.24, stream/drive/ms 15.44, stage/main/post_update 10.73, encode/finish_wall 9.35, stage/render/prepare_assets 9.25, encode/pass:point_shadow_atlas_live_indirect/sum 8.94 |
| warm-up | 6.29 | 36.6 | pipelines/compile_ms 134, pipelines/front_ms 131, render/thread/ms 52.20, render/busy/ms 50.80, main/busy/ms 47.76, stage/main/update 29.66, stage/render/prepare_assets 18.42, stage/render/render 17.62, render/graph/ms 15.45, stage/render/graph_submit 11.84, stage/main/post_update 10.99, encode//sum 7.65 |
| warm-up | 6.32 | 26.3 | pipelines/compile_ms 155, pipelines/front_ms 150, render/thread/ms 43.76, render/busy/ms 42.75, main/busy/ms 33.91, stage/render/prepare_assets 18.27, stage/main/update 17.52, stage/render/render 14.59, render/graph/ms 13.07, stage/render/graph_submit 10.34, stage/main/post_update 10.02, encode/finish_wall 6.98 |
| measure | 18.28 | 32.4 | main/busy/ms 37.64, stage/main/update 25.72, render/thread/ms 20.20, render/busy/ms 19.16, stage/render/render 9.63, render/graph/ms 8.28, stage/main/post_update 7.84, encode//sum 6.47, stage/render/graph_submit 5.58, stage/render/prepare_assets 3.74, encode/submit 3.53, stage/render/camera_driver 2.53 |
| measure | 59.12 | 25.6 | render/thread/ms 37.67, render/busy/ms 35.93, main/busy/ms 33.45, stage/render/render 18.57, stage/main/post_update 18.31, render/graph/ms 16.23, encode//sum 11.45, stage/render/graph_submit 10.56, stage/main/update 8.61, stage/render/prepare_assets 7.34, encode/submit 7.14, stage/render/camera_driver 5.44 |
| measure | 59.15 | 31.0 | render/thread/ms 39.49, render/busy/ms 38.42, main/busy/ms 27.13, stage/render/prepare_assets 17.19, stage/main/post_update 14.44, stage/render/render 11.54, render/graph/ms 10.00, encode//sum 8.08, stage/render/graph_submit 6.23, stage/main/update 5.69, encode/submit 3.60, stage/render/camera_driver 3.55 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=nocontact-3 --bench-baseline none --off contactshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **84%** (min 62%) · 2618 MHz (min 2610) · memory 8101 MHz · 84.9 W · 79 °C (max 84) · VRAM 5614 MiB · throttled 100% of samples

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
- `sg.post=high (preset: level requested)`
- `sg.shadows=custom (preset: derived from its members)`
- `sg.sky=high (preset: level requested)`
- `sg.textures=high (preset: level requested)`
- `sg.view_distance=high (preset: level requested)`
- `sg.water=high (preset: level requested)`
- `shadow.contact=off (command line: --off contactshadows)`
- `shadow.contact.length_m=0.3 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.steps=16 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.thickness_m=0.1 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.filter=auto (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.lamp.contact=off (command line: --off contactshadows)`
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
