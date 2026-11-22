# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: contactshadows · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=custom (preset: derived from its members); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=off (command line: --off contactshadows); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=off (command line: --off contactshadows); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6566 frames over 65.7 s · **median 9.88 ms (101 fps)** · mean 10.00 · p95 11.56 · p99 13.54 (1% low 74 fps) · worst 29.00 ms · 33 spikes over 1.5× median · 24.7 stutters/min (work 23.8/min)
- warm-up: 352 frames over 9.0 s, worst 2341.5 ms
- hitches (> 25.0 ms): **2** measured, 40 in warm-up
- timed GPU passes: 6.68 ms of a 10.00 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.88 ms, limit 11 ms
- **FAIL** frame_ms: 27 stutters (> 1.5x moving median of 60) in 1.09 min = 24.68/min, limit 1/min
- pass worst frame 29.00 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.943 | 2.038 | 2.071 | 0.95 | 0.178 |
| main_opaque_pass_3d | 1.484 | 1.557 | 2.354 | 0.95 | 0.064 |
| early prepass | 0.492 | 0.516 | 0.756 | 0.95 | 0.091 |
| bloom | 0.303 | 0.318 | 0.325 | 0.95 | 0.066 |
| atmosphere_luts | 0.301 | 0.315 | 0.380 | 0.95 | 0.003 |
| water_velocity | 0.215 | 0.226 | 0.247 | 0.95 | 0.002 |
| water_sspr | 0.193 | 0.202 | 0.204 | 0.95 | 0.002 |
| ssao | 0.179 | 0.188 | 0.193 | 0.95 | 0.008 |
| point_shadow_atlas | 0.130 | 0.227 | 0.343 | 0.57 | 0.210 |
| atmos_froxel_inject | 0.122 | 0.128 | 0.130 | 0.95 | 0.002 |
| upscaling | 0.122 | 0.128 | 0.135 | 0.95 | 0.004 |
| water_fft | 0.117 | 0.123 | 0.126 | 0.95 | 0.003 |
| uniform_allocation | 0.112 | 0.118 | 0.142 | 0.95 | 0.021 |
| clouds_composite | 0.100 | 0.105 | 0.109 | 0.95 | 0.002 |
| bin_unpacking | 0.089 | 0.094 | 0.115 | 0.95 | 0.016 |
| gpu_instance_materialization | 0.086 | 0.090 | 0.094 | 0.95 | 0.004 |
| tonemapping | 0.082 | 0.086 | 0.091 | 0.95 | 0.004 |
| main_transmissive_pass_3d | 0.065 | 0.068 | 0.128 | 0.95 | 0.009 |
| clustering | 0.061 | 0.064 | 0.067 | 0.95 | 0.020 |
| clouds_trace | 0.053 | 0.056 | 0.078 | 0.95 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.054 | 0.070 | 0.95 | 0.003 |
| render_sky | 0.050 | 0.053 | 0.053 | 0.95 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.415 | 0.418 | 0.11 | 0.001 |
| surface_trail | 0.036 | 0.057 | 0.057 | 0.63 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.95 | 0.001 |
| point_shadow_atlas/early_mesh_preprocessing | 0.025 | 0.044 | 0.066 | 0.57 | 0.009 |
| lightprobe_radiance_map | 0.024 | 0.219 | 0.230 | 0.11 | 0.004 |
| water_snapshot | 0.023 | 0.024 | 0.027 | 0.95 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.95 | 0.002 |
| range_unpacking | 0.020 | 0.021 | 0.026 | 0.95 | 0.003 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.95 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.95 | 0.005 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 0.95 | 0.188 |
| point_shadow_atlas/uniform_allocation | 0.013 | 0.022 | 0.045 | 0.57 | 0.007 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.95 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.010 | 0.017 | 0.032 | 0.57 | 0.005 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.003 |
| point_shadow_atlas/range_unpacking | 0.004 | 0.007 | 0.007 | 0.57 | 0.002 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.95 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.95 | 0.003 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.819 | 10.209 | 19.79 | 57902.9 |
| render/busy/ms | 8.315 | 9.692 | 18.83 | 54595.0 |
| main/busy/ms | 8.242 | 9.412 | 26.96 | 54118.0 |
| stage/render/render | 5.131 | 6.014 | 13.50 | 33689.0 |
| stage/main/post_update | 4.656 | 5.425 | 9.21 | 30571.7 |
| render/graph/ms | 4.471 | 5.322 | 11.06 | 29357.2 |
| encode//sum | 3.329 | 4.403 | 12.03 | 21861.4 |
| stage/render/graph_submit | 3.152 | 3.869 | 8.17 | 20693.9 |
| encode/submit | 1.975 | 2.620 | 6.18 | 12966.5 |
| stage/main/update | 1.871 | 2.324 | 20.60 | 12286.9 |
| stage/render/camera_driver | 1.256 | 1.535 | 2.73 | 8245.8 |
| encode/finish_wall | 1.230 | 1.659 | 2.96 | 8074.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.815 | 1.093 | 2.37 | 5352.7 |
| encode/pass:early prepass/sum | 0.814 | 1.090 | 1.89 | 5345.5 |
| encode/pass:shadow_directional_light/sum | 0.812 | 1.254 | 2.65 | 5330.6 |
| stage/render/prepare_write_phase_buffers | 0.584 | 0.858 | 1.86 | 3836.0 |
| bigspace/total/ms | 0.582 | 1.330 | 2.23 | 3820.7 |
| stage/render/prepare_assets | 0.518 | 0.681 | 10.05 | 3401.4 |
| stage/main/fixed_main_loop | 0.517 | 0.978 | 2.27 | 3397.0 |
| stage/render/prepare_meshes | 0.448 | 0.553 | 3.95 | 2943.3 |
| stage/render/prepare_resources | 0.423 | 0.511 | 0.87 | 2774.3 |
| render/present/ms | 0.410 | 0.586 | 2.11 | 2690.7 |
| stage/main/pre_update | 0.392 | 0.554 | 1.83 | 2574.0 |
| stage/main/first | 0.364 | 0.473 | 4.98 | 2387.8 |
| controller/feed/ms | 0.329 | 0.425 | 1.10 | 2162.3 |
| stage/render/prepare_flush | 0.329 | 0.549 | 1.06 | 2157.1 |
| stage/render/prepare_bind_groups | 0.306 | 0.496 | 1.81 | 2012.4 |
| stage/render/prepare_batch_phases | 0.282 | 0.453 | 0.82 | 1853.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.275 | 0.455 | 1.25 | 1802.6 |
| stage/main/last | 0.258 | 0.304 | 0.82 | 1696.7 |
| bigspace/low_precision/ms | 0.253 | 0.804 | 1.82 | 1660.0 |
| encode/pass:point_shadow_atlas_live_indirect/sum | 0.237 | 0.977 | 2.08 | 1556.5 |
| stream/drive/ms | 0.206 | 0.287 | 10.59 | 1349.8 |
| stage/render/create_views | 0.203 | 0.270 | 0.53 | 1329.9 |
| stage/render/extract_commands | 0.187 | 0.222 | 0.51 | 1229.3 |
| bigspace/high_precision/ms | 0.178 | 0.763 | 1.69 | 1166.0 |
| anim/evaluate/ms | 0.162 | 0.284 | 1.77 | 1066.2 |
| stage/render/specialize | 0.157 | 0.261 | 1.90 | 1033.4 |
| anim/evaluate_cpu/ms | 0.157 | 0.195 | 0.95 | 1028.8 |
| stream/tick/ms | 0.152 | 0.219 | 0.51 | 996.1 |
| stage/render/prepare_views | 0.137 | 0.180 | 0.44 | 896.9 |
| pipelines/compile_ms | 0.136 | 0.000 | 113.45 | 889.7 |
| pipelines/front_ms | 0.132 | 0.000 | 110.75 | 864.9 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.110 | 0.893 | 1.58 | 722.5 |
| stage/render/queue | 0.100 | 0.179 | 1.58 | 653.9 |
| render/acquire/ms | 0.094 | 0.126 | 0.39 | 617.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.092 | 0.146 | 0.76 | 605.5 |
| stage/render/phase_sort | 0.083 | 0.116 | 0.38 | 544.3 |
| controller/move/ms | 0.079 | 0.228 | 0.75 | 518.5 |
| controller/write/ms | 0.073 | 0.099 | 0.44 | 479.9 |
| terrain/update/ms | 0.060 | 0.293 | 1.47 | 394.3 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.049 | 0.000 | 2.64 | 324.3 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.076 | 0.28 | 314.5 |
| physics/step/ms | 0.040 | 0.085 | 0.63 | 264.5 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.67 | 206.6 |
| physics/insert/ms | 0.024 | 0.277 | 0.79 | 160.7 |
| stage/render/cleanup | 0.023 | 0.032 | 0.41 | 153.3 |
| terrain/select/ms | 0.022 | 0.259 | 0.82 | 147.5 |
| weather/map/ms | 0.018 | 0.024 | 0.07 | 115.1 |
| far/select/ms | 0.015 | 0.165 | 0.40 | 96.9 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.42 | 84.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.39 | 74.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.30 | 72.5 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.66 | 72.1 |
| script/update/ms | 0.008 | 0.012 | 0.26 | 53.8 |
| controller/avoid/ms | 0.006 | 0.013 | 0.07 | 41.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.18 | 36.1 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 32.0 |
| pipelines/driver_ms | 0.004 | 0.000 | 3.24 | 24.9 |
| script/ui/ms | 0.004 | 0.005 | 0.17 | 23.3 |
| stream/spawn/ms | 0.003 | 0.000 | 1.24 | 21.9 |
| stream/upload/ms | 0.003 | 0.000 | 0.90 | 18.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 13.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 4.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.7 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 559 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 586 |
| crowd/c0/meshes/count | count | 355 | 646 | 147 |
| crowd/c1/count | count | 144 | 157 | 148 |
| crowd/c1/joints/count | count | 5507 | 6037 | 5610 |
| crowd/c1/meshes/count | count | 1165 | 1311 | 1194 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6390 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 159 | 216 | 204 |
| crowd/joints_extracted/count | count | 714 | 5361 | 512 |
| crowd/joints_uploaded/count | count | 3573 | 6220 | 5720 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.42 | 1.64 | 0.69 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.42 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.66 | 0.01 |
| encode/finish_critical | ms | 0.97 | 2.65 | 0.96 |
| encode/pass:early prepass/max | ms | 0.81 | 1.89 | 0.78 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.82 | 2.37 | 0.96 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.76 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.25 | 1.25 | 0.46 |
| encode/pass:point_shadow_atlas_live_indirect/max | ms | 0.64 | 2.08 | 0.57 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.33 | 1.58 | 0.90 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.05 | 2.64 | 0.80 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.65 | 0.88 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.67 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.56 | 26.96 | 9.15 |
| gpu/timed/ms | ms | 7.65 | 9.31 | 9.15 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 149 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 316 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 149 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 316 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 770 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2485 | 3554 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 770 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2485 | 3554 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 61 | 352 | 8 |
| phases/shadow/bins | count | 806 | 1428 | 671 |
| phases/shadow/instance_batches | count | 4 | 14 | 4 |
| phases/shadow/instance_draws | count | 6557 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 122.9 k | 49.2 k |
| phases/shadow/items | count | 2295 | 3149 | 2000 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 50 | 143 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2708 | 3182 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 334 | 335 | 335 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 5 | 0 |
| preprocess/bind_groups | count | 15 | 263 | 11 |
| preprocess/bind_groups_created | count | 10 | 251 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 12 | 5 |
| preprocess/bind_groups_reused | count | 142 | 204 | 123 |
| process/cpu_usage | % | 24.94 | 38.10 | 25.48 |
| process/mem_usage | GiB | 5.13 | 5.34 | 5.34 |
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
| shadow_atlas/lamps | count | 9 | 19 | 0 |
| shadow_atlas/live_occ_pct | % | 7.42 | 29.88 | 0 |
| shadow_atlas/live_tiles | count | 17 | 56 | 0 |
| shadow_atlas/mover_draws | count | 9 | 36 | 0 |
| shadow_atlas/movers | count | 163 | 427 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 2 | 0 |
| shadow_atlas/render/blits | count | 9 | 36 | 0 |
| shadow_atlas/render/clears | count | 0 | 12 | 0 |
| shadow_atlas/render/lamps | count | 9 | 19 | 0 |
| shadow_atlas/render/live_direct | count | 9 | 36 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 6 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 16 | 54 | 0 |
| shadow_atlas/resizes | count | 30 | 39 | 39 |
| shadow_atlas/static_occ_pct | % | 16.41 | 38.67 | 0 |
| shadow_atlas/static_tiles | count | 48 | 102 | 0 |
| shadow_atlas/views | count | 16 | 54 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 91 | 9 |
| skin/blocks_skipped/count | count | 145 | 203 | 193 |
| skin/blocks_walked/count | count | 14 | 25 | 11 |
| skin/joint_bytes_written/bytes | bytes | 50.4 k | 808.6 k | 32.8 k |
| skin/joints_written/count | count | 788 | 12.6 k | 512 |
| skin/upload_writes/count | count | 8 | 24 | 9 |
| started_load_count |  | 6442 | 6566 | 6566 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.13 | 67.54 | 56.14 |
| system/mem_usage | % | 46.55 | 47.92 | 47.86 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 121 | 402 | 66 |
| views/all/bins | count | 2920 | 3506 | 2961 |
| views/all/count | count | 19 | 57 | 3 |
| views/all/instance_batches | count | 12 | 22 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 221.2 k | 147.5 k |
| views/all/items | count | 8024 | 10.5 k | 7942 |
| views/all/pipelines | count | 98 | 185 | 66 |
| views/all/pipelines_distinct | count | 68 | 74 | 61 |
| views/all/work_items | count | 154.1 k | 230.2 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 726 | 858 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6557 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2117 | 2406 | 2000 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 42 | 324 | 0 |
| views/lamp_face/bins | count | 121 | 613 | 0 |
| views/lamp_face/count | count | 16 | 54 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 78 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 164 | 774 | 0 |
| views/lamp_face/pipelines | count | 26 | 117 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 106 | 25.0 k | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2004 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5908 | 7528 | 5942 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.5 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 46.24 | 188 | 303.6 k |
| anim/events/count | count | 0.08 | 4 | 557 |
| assets/Image/added | count | 0.09 | 33 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.44 | 270 | 2874 |
| assets/Mesh/removed | count | 0.74 | 1242 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.14 | 10 | 7475 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.23 | 1260 | 1489 |
| encode//count | count | 27.91 | 30 | 183.3 k |
| encode/bind_walk/init_textures | count | 9048 | 26.8 k | 59.41 M |
| encode/bind_walk/init_textures_skipped | count | 49.9 k | 81.0 k | 327.62 M |
| encode/bind_walk/inits | count | 556 | 770 | 3.65 M |
| encode/bind_walk/inits_skipped | count | 166 | 255 | 1.09 M |
| encode/bind_walk/merge_textures | count | 8985 | 26.7 k | 59.00 M |
| encode/bind_walk/merge_textures_skipped | count | 11.9 k | 34.2 k | 78.45 M |
| encode/bind_walk/merges | count | 103 | 168 | 675.1 k |
| encode/bind_walk/merges_skipped | count | 148 | 225 | 971.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6566 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6566 |
| encode/pass:early prepass/count | count | 1 | 1 | 6566 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6566 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6566 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6566 |
| encode/pass:point_shadow_atlas_live_indirect/count | count | 0.34 | 1 | 2228 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.20 | 1 | 1338 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.04 | 1 | 286 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6566 |
| encode/queue/command_buffers | count | 38.51 | 42 | 252.9 k |
| encode/queue/submits | count | 2.01 | 4 | 13.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.29 M | 15.26 G |
| encode/queue/write_buffer/calls | count | 271 | 446 | 1.78 M |
| encode/queue/write_buffer_with/bytes | bytes | 160.3 k | 1.29 M | 1.05 G |
| encode/queue/write_buffer_with/calls | count | 9.30 | 818 | 61.1 k |
| encode/queue/write_texture/bytes | bytes | 1850 | 589.8 k | 12.15 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 556 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6566 |
| physics/inserts | count | 0.34 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 176 | 2335 |
| physics/steps | count | 0.60 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 13 |
| pipelines/created | count | 0.00 | 1 | 13 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 36 | 223 |
| stream/despawn/items | count | 0.03 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 5156 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1130 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.34 | 2341.5 | main/busy/ms 2313, stage/main/post_update 2268, stage/main/update 34.28, stage/main/pre_update 6.93, stage/main/last 1.03, surface/advance_cpu_ms 0.79, stage/main/fixed_main_loop 0.51, stage/main/first 0.48, stream/drive/ms 0.47, surface/cell_upload_ms 0.39, stream/tick/ms 0.20, bigspace/total/ms 0.10 |
| warm-up | 3.08 | 741.2 | main/busy/ms 2446, stage/main/post_update 2291, render/thread/ms 708, render/busy/ms 566, stage/render/prepare_assets 506, stage/render/prepare_views 140, render/acquire/ms 140, stage/main/update 138, stage/render/create_views 28.73, stage/render/render 22.41, render/graph/ms 18.63, stage/render/camera_driver 16.93 |
| warm-up | 3.23 | 143.4 | render/thread/ms 836, render/busy/ms 624, stage/render/prepare_assets 550, main/busy/ms 165, stage/render/prepare_views 141, render/acquire/ms 140, stage/main/update 113, stage/render/render 95.06, pipelines/compile_ms 73.59, render/present/ms 70.96, pipelines/driver_ms 53.95, stage/main/post_update 37.20 |
| warm-up | 3.34 | 103.1 | pipelines/wait_ms 2667, pipelines/compile_ms 1168, pipelines/front_ms 1020, pipelines/driver_ms 148, main/busy/ms 124, render/thread/ms 64.50, stage/main/update 62.97, render/busy/ms 61.71, stage/main/post_update 55.25, encode//sum 31.91, stage/render/render 22.86, render/graph/ms 19.68 |
| warm-up | 3.44 | 98.6 | pipelines/wait_ms 4489, pipelines/compile_ms 2051, pipelines/front_ms 1916, pipelines/driver_ms 135, render/thread/ms 131, render/busy/ms 129, main/busy/ms 104, stage/render/prepare_assets 73.36, stage/main/update 67.47, encode//sum 34.05, stage/main/post_update 25.59, stage/render/render 25.18 |
| warm-up | 3.50 | 55.4 | pipelines/wait_ms 1913, pipelines/compile_ms 1638, pipelines/front_ms 1548, render/thread/ms 132, render/busy/ms 130, pipelines/driver_ms 89.75, stage/render/prepare_assets 61.87, main/busy/ms 48.77, stage/render/render 29.59, render/graph/ms 27.30, stage/render/prepare_resources 25.04, encode//sum 25.03 |
| warm-up | 3.53 | 34.3 | pipelines/compile_ms 867, pipelines/front_ms 780, pipelines/wait_ms 231, render/thread/ms 86.29, pipelines/driver_ms 86.13, render/busy/ms 84.61, main/busy/ms 45.17, encode//sum 26.47, stage/render/render 26.31, render/graph/ms 24.19, stage/render/prepare_assets 19.93, stage/render/prepare_resources 17.26 |
| warm-up | 3.56 | 31.1 | pipelines/compile_ms 815, pipelines/front_ms 731, pipelines/wait_ms 335, pipelines/driver_ms 84.47, render/thread/ms 58.27, render/busy/ms 56.89, main/busy/ms 44.07, stage/render/prepare_assets 32.37, stage/main/pre_update 16.37, stage/main/post_update 15.53, terrain/update/ms 9.67, stage/render/render 8.59 |
| warm-up | 3.65 | 49.1 | pipelines/compile_ms 1252, pipelines/front_ms 1186, pipelines/wait_ms 192, main/busy/ms 94.34, stage/main/update 71.23, pipelines/driver_ms 65.55, render/thread/ms 54.13, render/busy/ms 51.41, stage/render/prepare_assets 19.92, stage/render/render 17.53, stage/main/post_update 16.85, render/graph/ms 14.42 |
| warm-up | 3.76 | 109.0 | pipelines/compile_ms 1835, pipelines/front_ms 1749, pipelines/wait_ms 226, pipelines/driver_ms 86.18, render/thread/ms 83.57, render/busy/ms 81.05, main/busy/ms 77.40, stage/main/update 62.52, stage/render/prepare_assets 60.41, stage/render/render 13.14, render/graph/ms 10.16, stage/main/post_update 8.10 |
| warm-up | 3.83 | 65.7 | pipelines/compile_ms 1249, pipelines/front_ms 1209, pipelines/wait_ms 103, render/thread/ms 101, render/busy/ms 99.72, stage/render/prepare_assets 84.13, pipelines/driver_ms 39.87, main/busy/ms 24.56, stage/main/update 11.64, stage/render/render 9.88, render/graph/ms 8.00, stage/main/post_update 6.17 |
| warm-up | 3.90 | 32.9 | pipelines/compile_ms 299, pipelines/front_ms 287, main/busy/ms 33.28, render/thread/ms 30.04, render/busy/ms 28.32, stage/main/update 20.34, stage/render/render 14.22, render/graph/ms 11.89, pipelines/driver_ms 11.69, stream/drive/ms 9.32, encode//sum 8.37, stage/render/prepare_assets 8.07 |
| warm-up | 3.94 | 34.7 | pipelines/compile_ms 282, pipelines/front_ms 271, render/thread/ms 51.28, main/busy/ms 48.80, render/busy/ms 48.63, stage/main/update 31.39, stage/render/prepare_assets 20.03, stream/drive/ms 18.89, stage/render/render 18.78, render/graph/ms 15.51, pipelines/driver_ms 11.37, stage/render/graph_submit 10.61 |
| warm-up | 4.00 | 67.9 | pipelines/compile_ms 72.90, pipelines/front_ms 69.62, render/thread/ms 67.09, render/busy/ms 64.69, main/busy/ms 50.54, stage/render/prepare_assets 41.79, stage/main/update 32.24, stream/drive/ms 20.88, stage/render/render 13.70, render/graph/ms 11.03, stage/main/post_update 10.73, stage/render/graph_submit 7.36 |
| warm-up | 4.06 | 59.2 | pipelines/compile_ms 658, pipelines/front_ms 637, render/thread/ms 76.94, render/busy/ms 74.65, main/busy/ms 68.11, stage/main/update 49.94, stage/render/prepare_assets 46.79, pipelines/driver_ms 20.63, stage/render/render 16.78, render/graph/ms 14.01, stage/main/post_update 10.87, encode//sum 6.58 |
| warm-up | 4.15 | 84.0 | pipelines/compile_ms 887, pipelines/front_ms 860, main/busy/ms 184, stage/main/update 163, render/thread/ms 82.67, render/busy/ms 80.49, stage/render/prepare_assets 47.88, pipelines/driver_ms 26.72, stage/render/render 18.99, render/graph/ms 16.15, stage/main/post_update 10.81, pipelines/wait_ms 7.68 |
| warm-up | 4.32 | 167.8 | pipelines/compile_ms 1837, pipelines/front_ms 1777, main/busy/ms 155, stage/main/update 133, render/thread/ms 99.35, render/busy/ms 97.92, pipelines/driver_ms 59.57, stage/render/prepare_assets 52.88, stage/render/render 33.77, render/graph/ms 31.80, stage/render/graph_submit 27.50, pipelines/wait_ms 25.30 |
| warm-up | 4.36 | 43.3 | pipelines/compile_ms 1574, pipelines/front_ms 1522, render/thread/ms 93.06, render/busy/ms 91.59, main/busy/ms 62.04, pipelines/driver_ms 52.40, pipelines/wait_ms 48.66, stage/render/render 35.50, render/graph/ms 33.63, stage/render/prepare_assets 31.56, stage/main/update 31.35, stage/render/graph_submit 29.55 |
| warm-up | 4.44 | 84.4 | pipelines/compile_ms 112, pipelines/front_ms 107, render/thread/ms 91.64, render/busy/ms 89.96, main/busy/ms 68.81, pipelines/wait_ms 40.97, stage/main/update 35.40, stage/render/prepare_assets 26.99, stage/main/post_update 22.38, pipelines/lock_ms 21.66, stage/render/render 15.79, render/graph/ms 13.73 |
| warm-up | 4.48 | 38.0 | pipelines/compile_ms 1166, pipelines/front_ms 1125, render/thread/ms 91.79, render/busy/ms 89.77, main/busy/ms 52.48, pipelines/wait_ms 47.28, pipelines/driver_ms 40.89, stage/main/update 32.26, stage/render/prepare_write_phase_buffers 22.83, stage/render/prepare_assets 22.12, stage/render/render 13.83, stage/render/prepare_flush 12.51 |
| warm-up | 4.53 | 44.8 | pipelines/compile_ms 1618, pipelines/front_ms 1564, pipelines/wait_ms 203, render/thread/ms 72.18, main/busy/ms 70.31, render/busy/ms 70.13, pipelines/driver_ms 54.05, stage/main/post_update 31.75, stage/main/update 26.66, stage/render/prepare_write_phase_buffers 21.21, stage/render/prepare_assets 16.33, stage/render/render 14.08 |
| warm-up | 4.62 | 95.7 | pipelines/compile_ms 961, pipelines/front_ms 932, pipelines/wait_ms 222, main/busy/ms 117, render/thread/ms 115, render/busy/ms 114, stage/main/update 56.79, stage/render/prepare_assets 50.47, stage/main/post_update 46.98, pipelines/driver_ms 29.57, stage/render/prepare_resources 24.31, stage/render/render 12.83 |
| warm-up | 4.69 | 70.5 | pipelines/compile_ms 1862, pipelines/front_ms 1804, pipelines/wait_ms 219, main/busy/ms 146, render/thread/ms 124, render/busy/ms 122, stage/main/update 106, stage/render/prepare_assets 64.95, pipelines/driver_ms 58.45, stream/drive/ms 55.65, stage/main/post_update 26.63, stage/render/prepare_resources 24.52 |
| warm-up | 4.78 | 84.9 | pipelines/compile_ms 4577, pipelines/driver_ms 2347, pipelines/front_ms 2230, pipelines/wait_ms 199, main/busy/ms 107, render/thread/ms 82.68, stage/main/update 82.56, render/busy/ms 80.98, stream/drive/ms 63.53, stage/render/prepare_assets 48.65, stage/render/render 15.03, render/graph/ms 12.77 |
| warm-up | 4.82 | 41.4 | pipelines/compile_ms 3405, pipelines/driver_ms 2312, pipelines/front_ms 1093, render/thread/ms 77.80, render/busy/ms 73.72, main/busy/ms 72.57, pipelines/wait_ms 61.66, stage/main/update 52.17, stage/render/prepare_assets 25.63, stage/render/render 18.17, render/graph/ms 15.85, stream/drive/ms 12.46 |
| warm-up | 4.86 | 46.7 | pipelines/compile_ms 766, pipelines/front_ms 742, render/thread/ms 68.50, render/busy/ms 64.55, main/busy/ms 53.50, stage/main/update 34.40, pipelines/wait_ms 26.91, pipelines/driver_ms 23.51, stage/render/prepare_assets 21.63, stage/render/render 17.59, render/graph/ms 15.31, stage/render/graph_submit 11.24 |
| warm-up | 4.94 | 32.6 | pipelines/compile_ms 219, pipelines/front_ms 212, render/thread/ms 48.93, render/busy/ms 47.27, main/busy/ms 40.69, stage/main/update 17.43, stage/main/post_update 17.00, stage/render/render 12.65, render/graph/ms 10.55, stage/render/prepare_resources 9.58, stage/render/graph_submit 7.37, stage/render/prepare_write_phase_buffers 7.22 |
| warm-up | 5.07 | 27.3 | pipelines/compile_ms 512, pipelines/front_ms 499, main/busy/ms 34.33, render/thread/ms 34.08, render/busy/ms 32.70, stage/main/update 20.75, stage/render/render 15.29, pipelines/driver_ms 13.29, stream/drive/ms 13.27, render/graph/ms 13.10, stage/render/graph_submit 9.20, stage/render/prepare_assets 8.51 |
| warm-up | 5.31 | 29.3 | main/busy/ms 35.16, stage/main/update 22.85, render/thread/ms 22.54, render/busy/ms 21.55, stream/drive/ms 17.25, stage/render/render 8.88, render/graph/ms 7.66, stage/main/post_update 7.22, encode//sum 5.66, stage/render/graph_submit 5.37, stage/render/prepare_assets 4.59, encode/submit 3.29 |
| warm-up | 5.55 | 29.3 | pipelines/compile_ms 135, pipelines/front_ms 132, pipelines/wait_ms 49.62, main/busy/ms 35.42, stage/main/update 24.30, render/thread/ms 23.69, render/busy/ms 22.57, stream/drive/ms 16.72, stage/render/render 10.34, render/graph/ms 8.79, encode//sum 7.35, stage/main/post_update 6.77 |
| warm-up | 5.68 | 28.0 | render/thread/ms 41.21, render/busy/ms 40.21, pipelines/compile_ms 30.55, main/busy/ms 30.40, pipelines/front_ms 28.79, stage/render/prepare_assets 17.94, stage/main/update 14.89, stage/render/render 10.53, stage/main/post_update 9.19, render/graph/ms 8.87, encode//sum 6.97, stage/render/graph_submit 4.97 |
| warm-up | 5.79 | 28.3 | render/thread/ms 33.63, main/busy/ms 33.17, render/busy/ms 32.45, stage/main/update 20.82, stage/render/render 9.84, stage/render/prepare_assets 9.54, render/graph/ms 8.16, stage/main/post_update 7.31, encode//sum 6.07, stage/render/graph_submit 5.53, stream/drive/ms 4.90, stage/render/prepare_meshes 3.68 |
| warm-up | 5.93 | 31.9 | render/thread/ms 44.03, render/busy/ms 42.56, main/busy/ms 41.62, stage/main/update 22.76, stage/render/render 14.83, stage/main/post_update 13.08, render/graph/ms 12.77, stage/render/prepare_assets 10.97, encode//sum 9.85, stage/render/graph_submit 8.92, encode/submit 6.17, stage/render/prepare_meshes 4.87 |
| warm-up | 5.96 | 34.7 | render/thread/ms 50.65, render/busy/ms 49.56, main/busy/ms 39.48, stage/main/update 18.16, stage/render/prepare_assets 16.36, stage/render/render 13.18, stage/main/post_update 12.82, render/graph/ms 11.63, stage/render/graph_submit 8.48, encode//sum 7.01, stage/render/prepare_meshes 6.78, encode/finish_wall 5.57 |
| warm-up | 6.00 | 38.9 | render/thread/ms 63.72, render/busy/ms 62.34, main/busy/ms 45.85, stage/render/render 21.64, stage/main/update 21.06, render/graph/ms 19.60, stage/main/post_update 15.26, stage/render/graph_submit 15.11, stage/render/prepare_assets 14.67, encode/finish_wall 11.13, encode/pass:point_shadow_atlas_live_indirect/sum 10.73, encode//sum 9.01 |
| warm-up | 6.02 | 25.7 | pipelines/compile_ms 139, pipelines/front_ms 137, render/thread/ms 55.96, render/busy/ms 54.64, main/busy/ms 37.79, stage/render/render 20.60, render/graph/ms 18.66, stage/main/update 17.33, stage/render/graph_submit 14.60, stage/main/post_update 13.42, stage/render/prepare_assets 12.27, encode/finish_wall 11.03 |
| warm-up | 6.06 | 36.2 | pipelines/compile_ms 157, pipelines/front_ms 153, render/thread/ms 47.44, render/busy/ms 46.34, main/busy/ms 30.63, stage/render/render 16.42, stage/render/prepare_assets 16.26, stage/main/update 15.76, render/graph/ms 14.80, stage/render/graph_submit 12.00, encode/finish_wall 9.10, stage/main/post_update 9.03 |
| warm-up | 6.23 | 43.5 | pipelines/compile_ms 124, pipelines/front_ms 121, main/busy/ms 59.46, render/thread/ms 48.33, render/busy/ms 46.65, stage/main/update 42.35, stage/render/render 24.97, render/graph/ms 22.47, stage/render/graph_submit 17.92, stream/drive/ms 14.95, encode/finish_wall 12.03, encode/pass:point_shadow_atlas_live_indirect/sum 11.52 |
| warm-up | 6.26 | 33.8 | render/thread/ms 56.72, render/busy/ms 55.47, main/busy/ms 52.58, stage/main/update 33.18, pipelines/compile_ms 28.78, pipelines/front_ms 27.52, stage/render/render 22.01, render/graph/ms 19.97, stage/render/prepare_assets 16.67, stage/render/graph_submit 15.89, encode/finish_wall 11.49, stage/main/post_update 11.45 |
| warm-up | 6.30 | 36.5 | render/thread/ms 52.20, render/busy/ms 51.18, main/busy/ms 38.28, pipelines/compile_ms 28.78, pipelines/front_ms 27.52, stage/render/prepare_assets 24.17, stage/main/update 21.19, stage/render/render 13.94, render/graph/ms 12.45, stage/render/graph_submit 9.70, stage/main/post_update 9.03, stream/drive/ms 7.87 |
| measure | 18.29 | 29.0 | main/busy/ms 35.09, stage/main/update 22.24, render/thread/ms 20.80, render/busy/ms 19.87, stage/render/render 10.43, render/graph/ms 9.03, stage/main/post_update 8.91, encode//sum 6.60, stage/render/graph_submit 6.37, encode/submit 3.91, stage/render/prepare_assets 3.66, stage/render/camera_driver 2.52 |
| measure | 59.19 | 28.6 | render/thread/ms 37.82, render/busy/ms 36.68, main/busy/ms 26.12, stage/render/prepare_assets 15.58, stage/main/post_update 12.37, stage/render/render 11.42, encode//sum 9.78, render/graph/ms 9.74, stage/render/graph_submit 6.03, stage/main/update 5.27, stage/main/fixed_main_loop 4.47, stage/render/camera_driver 3.45 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=nocontact-1 --bench-baseline none --off contactshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 2%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **83%** (min 54%) · 2614 MHz (min 2610) · memory 8101 MHz · 83.7 W · 79 °C (max 85) · VRAM 5579 MiB · throttled 100% of samples

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
