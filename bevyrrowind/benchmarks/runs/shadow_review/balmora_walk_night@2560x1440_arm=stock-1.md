# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=off (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6272 frames over 65.6 s · **median 10.32 ms (97 fps)** · mean 10.47 · p95 12.42 · p99 14.48 (1% low 69 fps) · worst 38.56 ms · 38 spikes over 1.5× median · 32.0 stutters/min (work 34.7/min)
- warm-up: 354 frames over 9.6 s, worst 2551.9 ms
- hitches (> 25.8 ms): **3** measured, 44 in warm-up
- timed GPU passes: 6.63 ms of a 10.47 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.32 ms, limit 11 ms
- **FAIL** frame_ms: 35 stutters (> 1.5x moving median of 60) in 1.09 min = 31.99/min, limit 1/min
- **FAIL** worst frame 38.56 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.893 | 2.036 | 2.070 | 0.93 | 0.177 |
| main_opaque_pass_3d | 1.532 | 1.647 | 2.367 | 0.93 | 0.076 |
| early prepass | 0.474 | 0.510 | 0.737 | 0.93 | 0.091 |
| bloom | 0.297 | 0.319 | 0.326 | 0.93 | 0.064 |
| atmosphere_luts | 0.292 | 0.314 | 0.379 | 0.93 | 0.004 |
| water_velocity | 0.210 | 0.225 | 0.247 | 0.93 | 0.002 |
| water_sspr | 0.188 | 0.202 | 0.203 | 0.93 | 0.002 |
| ssao | 0.175 | 0.188 | 0.191 | 0.93 | 0.008 |
| auxiliary_views | 0.169 | 0.182 | 0.685 | 0.93 | 0.178 |
| upscaling | 0.120 | 0.129 | 0.136 | 0.93 | 0.005 |
| atmos_froxel_inject | 0.119 | 0.128 | 0.130 | 0.93 | 0.002 |
| water_fft | 0.116 | 0.124 | 0.128 | 0.93 | 0.003 |
| uniform_allocation | 0.108 | 0.116 | 0.140 | 0.93 | 0.025 |
| clouds_composite | 0.097 | 0.105 | 0.109 | 0.93 | 0.002 |
| gpu_instance_materialization | 0.088 | 0.095 | 0.099 | 0.93 | 0.004 |
| bin_unpacking | 0.085 | 0.091 | 0.112 | 0.93 | 0.018 |
| tonemapping | 0.081 | 0.087 | 0.092 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.066 | 0.071 | 0.135 | 0.93 | 0.009 |
| clustering | 0.059 | 0.064 | 0.067 | 0.93 | 0.019 |
| clouds_trace | 0.052 | 0.056 | 0.078 | 0.93 | 0.001 |
| early_mesh_preprocessing | 0.050 | 0.054 | 0.071 | 0.93 | 0.004 |
| render_sky | 0.049 | 0.052 | 0.053 | 0.93 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.039 | 0.414 | 0.417 | 0.09 | 0.001 |
| surface_trail | 0.036 | 0.057 | 0.057 | 0.63 | 0.002 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.028 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.219 | 0.230 | 0.09 | 0.004 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.93 | 0.003 |
| night_sky | 0.016 | 0.017 | 0.044 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.93 | 0.005 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.031 | 0.93 | 0.188 |
| clouds_fog_light | 0.009 | 0.010 | 0.010 | 0.93 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.226 | 11.087 | 24.57 | 57865.5 |
| render/busy/ms | 8.717 | 10.595 | 23.82 | 54672.0 |
| main/busy/ms | 8.561 | 10.132 | 29.53 | 53697.2 |
| stage/render/render | 5.518 | 6.813 | 13.18 | 34609.4 |
| stage/main/post_update | 4.960 | 6.040 | 10.23 | 31111.9 |
| render/graph/ms | 4.831 | 6.145 | 12.12 | 30302.0 |
| encode//sum | 3.950 | 5.204 | 29.35 | 24773.4 |
| stage/render/graph_submit | 3.462 | 4.400 | 9.11 | 21716.5 |
| encode/submit | 2.222 | 3.015 | 4.70 | 13934.5 |
| stage/main/update | 1.847 | 2.367 | 23.74 | 11585.3 |
| encode/finish_wall | 1.316 | 1.669 | 4.62 | 8253.9 |
| stage/render/camera_driver | 1.307 | 1.773 | 4.26 | 8198.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.852 | 1.131 | 1.84 | 5344.1 |
| encode/pass:early prepass/sum | 0.848 | 1.128 | 1.84 | 5319.4 |
| encode/pass:shadow_directional_light/sum | 0.839 | 1.267 | 1.98 | 5263.0 |
| stage/render/prepare_write_phase_buffers | 0.624 | 0.935 | 1.75 | 3915.6 |
| bigspace/total/ms | 0.561 | 1.368 | 2.47 | 3515.8 |
| stage/main/fixed_main_loop | 0.535 | 0.957 | 2.68 | 3356.7 |
| stage/render/prepare_assets | 0.519 | 0.650 | 14.00 | 3256.9 |
| stage/render/prepare_meshes | 0.448 | 0.553 | 4.47 | 2809.6 |
| stage/render/prepare_resources | 0.434 | 0.523 | 1.08 | 2724.8 |
| render/present/ms | 0.419 | 1.167 | 2.18 | 2625.5 |
| stage/main/pre_update | 0.398 | 0.577 | 3.07 | 2495.5 |
| stage/main/first | 0.395 | 0.516 | 4.79 | 2480.5 |
| encode/pass:shadow_point_light/sum | 0.349 | 1.755 | 4.71 | 2191.9 |
| controller/feed/ms | 0.331 | 0.415 | 0.98 | 2076.6 |
| stage/render/prepare_bind_groups | 0.314 | 0.520 | 2.05 | 1972.3 |
| stage/render/prepare_flush | 0.292 | 0.496 | 1.40 | 1832.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.284 | 0.443 | 1.12 | 1778.1 |
| stage/render/prepare_batch_phases | 0.278 | 0.423 | 0.78 | 1742.6 |
| stage/main/last | 0.248 | 0.295 | 1.31 | 1554.4 |
| bigspace/low_precision/ms | 0.238 | 0.764 | 1.92 | 1489.8 |
| stream/drive/ms | 0.210 | 0.296 | 10.32 | 1319.6 |
| stage/render/extract_commands | 0.189 | 0.222 | 0.52 | 1186.9 |
| stage/render/create_views | 0.186 | 0.240 | 0.89 | 1166.8 |
| stage/render/specialize | 0.183 | 0.266 | 2.70 | 1145.0 |
| bigspace/high_precision/ms | 0.174 | 0.747 | 1.96 | 1091.2 |
| anim/evaluate/ms | 0.158 | 0.266 | 2.46 | 992.6 |
| stream/tick/ms | 0.154 | 0.226 | 0.49 | 968.6 |
| anim/evaluate_cpu/ms | 0.148 | 0.185 | 0.73 | 930.6 |
| stage/render/prepare_views | 0.133 | 0.173 | 0.40 | 833.7 |
| pipelines/compile_ms | 0.117 | 0.000 | 133.18 | 731.4 |
| stage/render/queue | 0.115 | 0.183 | 2.32 | 722.8 |
| pipelines/front_ms | 0.114 | 0.000 | 130.24 | 712.4 |
| encode/pass:main_transmissive_pass_3d/sum | 0.099 | 0.154 | 0.83 | 618.5 |
| render/acquire/ms | 0.091 | 0.120 | 0.35 | 568.0 |
| stage/render/phase_sort | 0.085 | 0.116 | 0.29 | 533.8 |
| controller/move/ms | 0.080 | 0.237 | 0.57 | 502.2 |
| controller/write/ms | 0.074 | 0.103 | 0.47 | 466.4 |
| terrain/update/ms | 0.062 | 0.295 | 2.01 | 387.6 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.074 | 0.45 | 302.1 |
| physics/step/ms | 0.042 | 0.087 | 0.60 | 265.6 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.70 | 199.1 |
| physics/insert/ms | 0.027 | 0.321 | 0.80 | 167.8 |
| stage/render/cleanup | 0.024 | 0.033 | 0.52 | 149.5 |
| terrain/select/ms | 0.023 | 0.260 | 0.83 | 147.4 |
| weather/map/ms | 0.018 | 0.024 | 0.24 | 111.4 |
| far/select/ms | 0.015 | 0.166 | 0.40 | 92.8 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.43 | 88.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.021 | 0.48 | 71.8 |
| anim/apply/ms | 0.011 | 0.018 | 0.38 | 68.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.07 | 68.1 |
| script/update/ms | 0.008 | 0.012 | 0.23 | 51.2 |
| controller/avoid/ms | 0.006 | 0.012 | 0.14 | 38.1 |
| stream/despawn/ms | 0.006 | 0.000 | 1.02 | 37.5 |
| script/input/ms | 0.005 | 0.007 | 0.19 | 33.7 |
| stream/spawn/ms | 0.003 | 0.000 | 1.11 | 21.9 |
| script/ui/ms | 0.003 | 0.004 | 0.10 | 20.7 |
| stream/upload/ms | 0.003 | 0.000 | 1.14 | 19.8 |
| pipelines/driver_ms | 0.003 | 0.000 | 2.97 | 19.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.67 | 13.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.35 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 30 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
| crowd/c0/count | count | 27 | 42 | 12 |
| crowd/c0/joints/count | count | 1301 | 2204 | 535 |
| crowd/c0/meshes/count | count | 355 | 634 | 144 |
| crowd/c1/count | count | 146 | 158 | 149 |
| crowd/c1/joints/count | count | 5570 | 6092 | 5661 |
| crowd/c1/meshes/count | count | 1178 | 1338 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 162 | 216 | 205 |
| crowd/joints_extracted/count | count | 668 | 6922 | 346 |
| crowd/joints_uploaded/count | count | 3617 | 6207 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.44 | 1.89 | 0.63 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.43 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.48 | 0.01 |
| encode/finish_critical | ms | 0.99 | 1.98 | 1.28 |
| encode/pass:early prepass/max | ms | 0.85 | 1.84 | 0.81 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.86 | 1.84 | 0.83 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.83 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 1.12 | 0.31 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 1.98 | 1.28 |
| encode/pass:shadow_point_light/max | ms | 0.35 | 1.40 | 0.79 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.70 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.88 | 29.53 | 8.88 |
| gpu/timed/ms | ms | 7.83 | 9.21 | 8.41 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.4 k | 302.4 k | 302.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 336 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 336 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 764 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2474 | 3522 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 764 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2474 | 3522 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 148 | 27 |
| phases/shadow/bins | count | 1223 | 1558 | 671 |
| phases/shadow/instance_batches | count | 4 | 8 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| phases/shadow/items | count | 2846 | 3330 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 158 | 192 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3182 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 228 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 334 | 335 | 335 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 16 | 454 | 43 |
| preprocess/bind_groups_created | count | 11 | 449 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 6 |
| preprocess/bind_groups_reused | count | 129 | 230 | 123 |
| process/cpu_usage | % | 26.92 | 41.43 | 23.08 |
| process/mem_usage | GiB | 5.45 | 5.66 | 5.65 |
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
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 120 | 7 |
| skin/blocks_skipped/count | count | 147 | 208 | 198 |
| skin/blocks_walked/count | count | 14 | 28 | 7 |
| skin/joint_bytes_written/bytes | bytes | 47.1 k | 811.6 k | 25.9 k |
| skin/joints_written/count | count | 736 | 12.7 k | 404 |
| skin/upload_writes/count | count | 6 | 27 | 5 |
| started_load_count |  | 6442 | 6566 | 6566 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.50 | 50.56 | 30.28 |
| system/mem_usage | % | 55.52 | 56.07 | 55.95 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2887 | 3003 | 3003 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 78 | 207 | 85 |
| views/all/bins | count | 3089 | 3746 | 2961 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 196.6 k |
| views/all/items | count | 8391 | 10.6 k | 7969 |
| views/all/pipelines | count | 206 | 235 | 66 |
| views/all/pipelines_distinct | count | 70 | 73 | 61 |
| views/all/work_items | count | 154.0 k | 206.6 k | 205.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 27 |
| views/cascade/bins | count | 714 | 860 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2115 | 2397 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 0 | 138 | 0 |
| views/lamp_face/bins | count | 508 | 760 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 24 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 741 | 1020 | 0 |
| views/lamp_face/pipelines | count | 132 | 166 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 694 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 1992 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5907 | 7464 | 5942 |
| views/main/pipelines | count | 46 | 49 | 46 |
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
| world/meshes | count | 21.1 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 40.09 | 223 | 251.4 k |
| anim/events/count | count | 0.08 | 3 | 523 |
| assets/Image/added | count | 0.09 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.46 | 273 | 2874 |
| assets/Mesh/removed | count | 0.77 | 1242 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.19 | 10 | 7474 |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.24 | 1151 | 1490 |
| encode//count | count | 34.93 | 204 | 219.1 k |
| encode/bind_walk/init_textures | count | 9359 | 26.5 k | 58.70 M |
| encode/bind_walk/init_textures_skipped | count | 59.3 k | 78.9 k | 372.02 M |
| encode/bind_walk/inits | count | 555 | 993 | 3.48 M |
| encode/bind_walk/inits_skipped | count | 143 | 243 | 894.9 k |
| encode/bind_walk/merge_textures | count | 9296 | 26.4 k | 58.30 M |
| encode/bind_walk/merge_textures_skipped | count | 9057 | 17.2 k | 56.81 M |
| encode/bind_walk/merges | count | 100 | 281 | 628.4 k |
| encode/bind_walk/merges_skipped | count | 128 | 231 | 802.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6272 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6272 |
| encode/pass:early prepass/count | count | 1 | 1 | 6272 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6272 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6272 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6272 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6272 |
| encode/pass:shadow_point_light/count | count | 1.75 | 44 | 11.0 k |
| encode/queue/command_buffers | count | 46.70 | 258 | 292.9 k |
| encode/queue/submits | count | 2.01 | 4 | 12.6 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.42 M | 14.56 G |
| encode/queue/write_buffer/calls | count | 263 | 691 | 1.65 M |
| encode/queue/write_buffer_with/bytes | bytes | 160.8 k | 1.28 M | 1.01 G |
| encode/queue/write_buffer_with/calls | count | 10.36 | 828 | 65.0 k |
| encode/queue/write_texture/bytes | bytes | 2031 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6272 |
| physics/inserts | count | 0.36 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 175 | 2335 |
| physics/steps | count | 0.63 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 7 |
| pipelines/created | count | 0.00 | 1 | 8 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 30 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5397 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1183 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.8 ms; slowest 47, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2551.9 | main/busy/ms 2521, stage/main/post_update 2481, stage/main/update 30.64, stage/main/pre_update 6.60, stage/main/last 1.04, surface/advance_cpu_ms 0.80, stage/main/first 0.47, stage/main/fixed_main_loop 0.37, stream/drive/ms 0.34, surface/cell_upload_ms 0.31, bigspace/total/ms 0.10, stream/tick/ms 0.10 |
| warm-up | 3.28 | 729.9 | main/busy/ms 2654, stage/main/post_update 2505, render/thread/ms 699, render/busy/ms 565, stage/render/prepare_assets 520, stage/main/update 133, stage/render/prepare_views 132, render/acquire/ms 132, stage/render/create_views 26.77, stage/render/render 10.89, stage/main/pre_update 8.14, render/graph/ms 6.95 |
| warm-up | 3.39 | 109.3 | render/thread/ms 792, render/busy/ms 605, stage/render/prepare_assets 544, main/busy/ms 155, pipelines/compile_ms 134, stage/render/prepare_views 133, render/acquire/ms 133, stage/main/update 109, pipelines/front_ms 92.53, stage/render/render 70.15, render/present/ms 54.91, pipelines/driver_ms 41.58 |
| warm-up | 3.55 | 140.3 | pipelines/wait_ms 4259, pipelines/compile_ms 1394, pipelines/front_ms 1279, render/thread/ms 129, render/busy/ms 123, pipelines/driver_ms 115, main/busy/ms 87.41, stage/render/prepare_write_phase_buffers 43.29, stage/main/update 38.81, stage/main/post_update 35.17, stage/render/prepare_resources 29.80, stage/render/render 24.69 |
| warm-up | 3.61 | 52.5 | pipelines/wait_ms 4438, pipelines/compile_ms 1658, pipelines/front_ms 1546, render/thread/ms 161, render/busy/ms 158, pipelines/driver_ms 112, main/busy/ms 109, stage/main/update 73.04, stage/render/prepare_write_phase_buffers 43.30, stage/render/render 37.88, stage/render/prepare_resources 35.56, render/graph/ms 34.96 |
| warm-up | 3.73 | 122.6 | pipelines/compile_ms 1744, pipelines/front_ms 1562, pipelines/wait_ms 579, encode//sum 286, pipelines/driver_ms 182, render/thread/ms 148, render/busy/ms 146, main/busy/ms 89.85, stage/render/render 68.80, render/graph/ms 66.09, stage/main/update 60.32, stage/render/prepare_assets 56.79 |
| warm-up | 3.78 | 28.1 | pipelines/compile_ms 1194, pipelines/front_ms 1072, pipelines/wait_ms 131, pipelines/driver_ms 122, render/thread/ms 48.51, render/busy/ms 46.75, main/busy/ms 39.17, stage/render/prepare_assets 21.21, stage/main/post_update 18.71, stage/render/render 14.75, render/graph/ms 12.62, encode//sum 10.53 |
| warm-up | 3.86 | 31.3 | main/busy/ms 77.46, render/thread/ms 52.17, render/busy/ms 50.41, stage/main/update 47.80, pipelines/compile_ms 44.28, pipelines/front_ms 42.60, stage/main/post_update 22.50, stage/render/render 18.96, render/graph/ms 16.87, stage/render/prepare_flush 14.34, stage/render/graph_submit 13.78, terrain/update/ms 8.91 |
| warm-up | 3.96 | 104.0 | pipelines/compile_ms 964, pipelines/front_ms 932, main/busy/ms 84.84, render/thread/ms 84.09, render/busy/ms 82.50, stage/main/update 65.71, stage/render/prepare_assets 52.20, pipelines/driver_ms 31.37, stage/render/prepare_flush 13.34, stage/main/post_update 11.72, stage/render/render 10.74, render/graph/ms 8.58 |
| warm-up | 4.00 | 42.1 | pipelines/compile_ms 1416, pipelines/front_ms 1365, render/thread/ms 79.86, render/busy/ms 78.44, stage/render/prepare_assets 62.10, pipelines/driver_ms 51.58, main/busy/ms 39.16, stage/main/update 27.93, pipelines/wait_ms 26.62, stage/render/render 10.72, render/graph/ms 8.84, stage/render/graph_submit 6.43 |
| warm-up | 4.03 | 32.3 | pipelines/compile_ms 699, pipelines/front_ms 662, render/thread/ms 44.98, render/busy/ms 43.63, pipelines/driver_ms 37.27, stage/render/prepare_assets 29.13, pipelines/wait_ms 26.18, main/busy/ms 19.85, stage/render/render 10.42, stage/main/update 9.05, render/graph/ms 8.86, stage/render/graph_submit 6.35 |
| warm-up | 4.08 | 39.2 | pipelines/compile_ms 63.61, pipelines/front_ms 61.02, render/thread/ms 41.74, stage/render/prepare_views 23.94, render/acquire/ms 23.79, main/busy/ms 20.13, render/busy/ms 16.38, encode//sum 14.94, stage/render/render 11.82, stage/main/update 11.74, render/graph/ms 9.73, stage/render/graph_submit 6.86 |
| warm-up | 4.14 | 42.0 | pipelines/compile_ms 323, pipelines/front_ms 311, main/busy/ms 47.96, render/thread/ms 39.13, render/busy/ms 37.22, stage/main/update 32.15, stream/drive/ms 19.67, stage/render/render 16.54, render/graph/ms 13.91, pipelines/driver_ms 12.31, stage/render/prepare_assets 10.83, stage/main/post_update 10.19 |
| warm-up | 4.19 | 53.2 | pipelines/compile_ms 511, pipelines/front_ms 489, render/thread/ms 63.92, render/busy/ms 62.07, main/busy/ms 50.90, stage/render/prepare_assets 30.25, stage/main/update 24.50, pipelines/driver_ms 21.47, stage/main/post_update 16.97, stage/render/render 16.90, render/graph/ms 14.34, stage/render/graph_submit 10.23 |
| warm-up | 4.26 | 67.8 | pipelines/compile_ms 532, pipelines/front_ms 508, main/busy/ms 186, stage/main/update 152, render/thread/ms 85.92, pipelines/wait_ms 84.66, render/busy/ms 84.54, stage/render/prepare_assets 55.24, pipelines/driver_ms 23.24, stage/render/render 14.42, stage/main/post_update 14.37, render/graph/ms 12.37 |
| warm-up | 4.48 | 215.1 | pipelines/compile_ms 2141, pipelines/front_ms 2070, main/busy/ms 172, stage/main/update 146, render/thread/ms 122, render/busy/ms 120, pipelines/wait_ms 96.72, pipelines/driver_ms 71.90, stage/render/prepare_assets 70.46, stage/render/render 40.53, render/graph/ms 38.46, encode//sum 35.65 |
| warm-up | 4.59 | 86.5 | render/thread/ms 82.83, render/busy/ms 80.99, main/busy/ms 75.78, stage/main/post_update 38.92, stage/render/prepare_assets 27.43, stage/main/update 26.56, stage/render/render 19.25, render/graph/ms 16.79, stage/render/graph_submit 13.26, stage/render/prepare_resources 12.70, stage/render/prepare_meshes 12.22, pipelines/compile_ms 12.10 |
| warm-up | 4.67 | 87.0 | pipelines/compile_ms 1367, pipelines/front_ms 1319, pipelines/wait_ms 284, render/thread/ms 145, render/busy/ms 143, main/busy/ms 82.13, stage/render/prepare_assets 47.66, pipelines/driver_ms 47.35, stage/main/update 39.00, stage/main/post_update 34.89, stage/render/prepare_resources 24.42, stage/render/prepare_write_phase_buffers 19.43 |
| warm-up | 4.73 | 54.8 | pipelines/compile_ms 2085, pipelines/front_ms 2013, pipelines/wait_ms 378, render/thread/ms 122, render/busy/ms 121, pipelines/driver_ms 72.05, main/busy/ms 61.31, stage/render/prepare_assets 45.34, stage/main/update 40.94, stage/render/prepare_write_phase_buffers 19.97, stage/render/prepare_flush 17.18, stage/render/render 16.09 |
| warm-up | 4.86 | 132.8 | pipelines/compile_ms 1811, pipelines/front_ms 1752, pipelines/wait_ms 513, render/thread/ms 153, render/busy/ms 151, main/busy/ms 110, stage/main/post_update 81.20, stage/render/prepare_assets 77.60, pipelines/driver_ms 58.51, stage/render/prepare_resources 24.90, encode//sum 22.78, stage/render/render 20.17 |
| warm-up | 4.92 | 56.7 | pipelines/compile_ms 1383, pipelines/front_ms 1341, pipelines/wait_ms 715, render/thread/ms 163, render/busy/ms 161, main/busy/ms 127, stage/main/post_update 82.70, stage/render/prepare_assets 70.56, pipelines/driver_ms 42.79, stage/main/update 30.87, encode//sum 30.60, stage/render/render 28.00 |
| warm-up | 5.00 | 88.0 | pipelines/compile_ms 1408, pipelines/front_ms 1378, pipelines/wait_ms 311, render/thread/ms 112, render/busy/ms 110, main/busy/ms 74.83, stage/main/update 49.43, stage/render/prepare_assets 46.98, stage/render/render 32.75, encode//sum 32.09, pipelines/driver_ms 29.70, render/graph/ms 29.61 |
| warm-up | 5.05 | 48.6 | pipelines/compile_ms 1117, pipelines/front_ms 1096, main/busy/ms 119, render/thread/ms 101, render/busy/ms 99.22, stage/main/update 93.09, stream/drive/ms 59.07, stage/render/prepare_assets 46.52, stage/render/render 24.32, encode//sum 23.08, render/graph/ms 21.17, pipelines/driver_ms 20.73 |
| warm-up | 5.13 | 80.6 | pipelines/compile_ms 285, pipelines/front_ms 275, main/busy/ms 109, stage/main/update 84.84, render/thread/ms 73.35, render/busy/ms 71.60, stream/drive/ms 66.63, pipelines/wait_ms 62.11, stage/render/prepare_assets 23.49, stage/render/render 14.31, stage/main/post_update 13.08, render/graph/ms 11.79 |
| warm-up | 5.19 | 54.3 | pipelines/compile_ms 1268, pipelines/front_ms 1230, render/thread/ms 82.19, render/busy/ms 80.53, main/busy/ms 75.02, pipelines/wait_ms 63.91, stage/main/update 49.47, pipelines/driver_ms 37.92, stage/render/prepare_assets 18.64, stage/render/render 16.07, stage/main/post_update 15.36, stream/drive/ms 14.19 |
| warm-up | 5.24 | 51.3 | pipelines/compile_ms 1234, pipelines/front_ms 1199, render/thread/ms 83.24, render/busy/ms 81.46, main/busy/ms 65.34, stage/main/update 40.52, pipelines/driver_ms 35.37, stage/render/prepare_assets 27.41, stage/render/render 18.57, render/graph/ms 16.10, stage/main/post_update 14.59, stage/render/prepare_resources 12.84 |
| warm-up | 5.28 | 36.0 | pipelines/compile_ms 950, pipelines/front_ms 923, render/thread/ms 67.96, render/busy/ms 66.40, main/busy/ms 49.42, pipelines/driver_ms 27.40, stage/main/update 25.86, stage/render/prepare_assets 22.83, pipelines/wait_ms 20.33, stage/render/render 18.08, render/graph/ms 15.81, encode//sum 12.21 |
| warm-up | 5.31 | 38.6 | pipelines/compile_ms 708, pipelines/front_ms 687, render/thread/ms 70.11, render/busy/ms 68.50, main/busy/ms 51.53, stage/main/update 26.71, pipelines/driver_ms 21.07, pipelines/wait_ms 19.79, stage/render/render 19.43, render/graph/ms 17.17, encode//sum 12.51, stage/render/graph_submit 12.23 |
| warm-up | 5.34 | 27.2 | pipelines/compile_ms 219, pipelines/front_ms 214, render/thread/ms 57.67, render/busy/ms 56.24, main/busy/ms 38.86, stage/main/update 19.81, stage/render/render 16.29, render/graph/ms 14.34, stage/render/prepare_assets 12.54, stage/main/post_update 11.30, encode//sum 11.08, stage/render/graph_submit 9.40 |
| warm-up | 5.37 | 26.7 | pipelines/compile_ms 535, pipelines/front_ms 519, render/thread/ms 42.27, render/busy/ms 41.16, main/busy/ms 31.69, stage/render/prepare_assets 16.58, pipelines/driver_ms 15.67, stage/main/update 13.82, stage/render/render 13.19, render/graph/ms 11.57, encode//sum 10.44, stage/main/post_update 10.35 |
| warm-up | 5.60 | 29.9 | pipelines/compile_ms 682, pipelines/front_ms 664, render/thread/ms 41.84, render/busy/ms 40.47, main/busy/ms 26.84, pipelines/driver_ms 18.30, stage/render/prepare_assets 15.97, stage/render/render 14.86, render/graph/ms 12.94, stage/main/post_update 11.16, encode//sum 9.61, stage/main/update 9.40 |
| warm-up | 5.77 | 26.0 | pipelines/compile_ms 530, pipelines/front_ms 514, render/thread/ms 40.01, main/busy/ms 39.94, render/busy/ms 37.81, stage/main/post_update 22.34, stage/render/render 21.45, render/graph/ms 19.21, pipelines/driver_ms 15.50, stage/render/graph_submit 12.45, stage/main/update 11.32, encode//sum 9.10 |
| warm-up | 5.84 | 29.4 | main/busy/ms 37.85, pipelines/compile_ms 32.26, pipelines/front_ms 30.40, render/thread/ms 26.23, render/busy/ms 25.12, stage/main/update 24.96, stream/drive/ms 19.50, stage/render/render 12.79, render/graph/ms 11.28, encode//sum 10.97, stage/main/post_update 7.93, stage/render/graph_submit 7.53 |
| warm-up | 6.07 | 28.4 | main/busy/ms 34.88, render/thread/ms 27.19, render/busy/ms 26.18, stage/main/update 22.72, stream/drive/ms 14.89, stage/render/render 12.24, render/graph/ms 10.60, encode//sum 9.49, stage/main/post_update 8.06, stage/render/graph_submit 7.21, stage/render/prepare_assets 5.08, encode/submit 4.55 |
| warm-up | 6.21 | 31.4 | render/thread/ms 43.18, render/busy/ms 41.94, main/busy/ms 29.99, stage/render/prepare_assets 20.96, stage/main/update 14.15, pipelines/compile_ms 12.04, pipelines/front_ms 11.14, stage/render/render 10.88, encode//sum 9.83, stage/main/post_update 9.34, render/graph/ms 8.99, stage/render/graph_submit 5.50 |
| warm-up | 6.35 | 28.6 | pipelines/compile_ms 137, pipelines/front_ms 133, main/busy/ms 35.53, render/thread/ms 34.95, render/busy/ms 33.87, stage/main/update 19.84, stage/render/prepare_assets 11.20, stage/render/render 11.18, encode//sum 10.00, stage/main/post_update 9.70, render/graph/ms 9.54, stage/render/graph_submit 5.99 |
| warm-up | 6.40 | 29.3 | render/thread/ms 40.92, render/busy/ms 39.66, main/busy/ms 37.60, stage/main/update 21.27, stage/render/render 16.85, pipelines/compile_ms 15.33, render/graph/ms 14.93, pipelines/front_ms 14.51, encode//sum 12.96, stage/main/post_update 10.98, stage/render/graph_submit 9.43, stage/render/prepare_assets 8.11 |
| warm-up | 6.47 | 29.2 | main/busy/ms 43.66, render/thread/ms 42.84, render/busy/ms 40.97, stage/main/update 25.31, stage/render/render 20.73, render/graph/ms 18.31, encode//sum 14.20, stage/main/post_update 13.34, stage/render/graph_submit 13.06, encode/submit 9.36, stage/render/prepare_assets 5.68, stage/render/camera_driver 5.08 |
| warm-up | 6.50 | 32.4 | render/thread/ms 50.05, render/busy/ms 48.18, main/busy/ms 45.59, stage/main/update 25.71, stage/render/render 19.38, render/graph/ms 16.81, encode//sum 14.26, stage/main/post_update 13.65, stage/render/graph_submit 11.69, stage/render/prepare_assets 11.46, encode/submit 8.00, stage/render/prepare_meshes 5.60 |
| warm-up | 6.53 | 31.2 | render/thread/ms 49.53, render/busy/ms 48.53, main/busy/ms 43.04, stage/main/update 20.24, stage/render/prepare_assets 16.45, stage/main/post_update 13.68, stage/render/render 13.52, encode//sum 12.41, render/graph/ms 11.82, stage/render/graph_submit 7.19, stage/render/prepare_meshes 5.64, stream/drive/ms 5.57 |
| warm-up | 6.56 | 33.9 | pipelines/compile_ms 143, pipelines/front_ms 140, render/thread/ms 57.88, render/busy/ms 56.25, main/busy/ms 48.28, stage/main/update 22.91, stage/render/render 19.05, render/graph/ms 16.63, stage/main/post_update 16.60, encode//sum 15.14, stage/render/prepare_assets 15.13, stage/render/graph_submit 10.05 |
| warm-up | 6.62 | 34.5 | render/thread/ms 43.33, render/busy/ms 42.44, main/busy/ms 35.53, stage/render/prepare_assets 19.01, stage/main/update 18.69, pipelines/compile_ms 14.41, pipelines/front_ms 13.71, stage/main/post_update 10.86, stage/render/render 10.57, encode//sum 9.87, render/graph/ms 9.09, stage/render/graph_submit 5.94 |
| warm-up | 6.78 | 45.6 | pipelines/compile_ms 139, pipelines/front_ms 134, main/busy/ms 57.67, render/thread/ms 46.13, render/busy/ms 44.51, stage/main/update 38.93, stage/render/render 18.12, render/graph/ms 15.76, stream/drive/ms 13.94, stage/main/post_update 13.61, encode//sum 13.23, stage/render/prepare_assets 12.29 |
| warm-up | 6.81 | 32.8 | render/thread/ms 49.40, render/busy/ms 48.03, main/busy/ms 43.12, pipelines/compile_ms 28.91, pipelines/front_ms 27.43, stage/main/update 24.22, stage/render/prepare_assets 19.56, stage/render/render 14.28, encode//sum 13.03, stage/main/post_update 12.55, render/graph/ms 12.23, stage/render/graph_submit 7.32 |
| measure | 1.69 | 29.8 | render/thread/ms 37.33, render/busy/ms 35.83, encode//sum 24.87, main/busy/ms 22.67, stage/render/render 18.73, render/graph/ms 16.57, stage/render/graph_submit 10.91, stage/main/update 9.44, stage/main/post_update 8.68, stage/render/prepare_assets 6.47, encode/submit 5.66, stage/render/camera_driver 5.53 |
| measure | 18.39 | 31.7 | main/busy/ms 39.01, stage/main/update 25.63, render/thread/ms 21.57, render/busy/ms 20.58, stage/render/render 10.97, render/graph/ms 9.50, stage/main/post_update 9.19, encode//sum 7.31, stage/render/graph_submit 6.80, encode/submit 4.38, stage/render/prepare_assets 4.07, stage/render/camera_driver 2.56 |
| measure | 59.11 | 38.6 | render/thread/ms 38.87, render/busy/ms 37.78, main/busy/ms 22.78, stage/render/prepare_assets 18.81, stage/main/post_update 10.40, stage/render/render 10.02, render/graph/ms 8.77, encode//sum 7.34, stage/render/graph_submit 5.80, stage/main/update 4.56, stage/main/fixed_main_loop 3.66, encode/submit 3.54 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=off --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=stock-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **80%** (min 45%) · 2619 MHz (min 2610) · memory 8101 MHz · 81.3 W · 79 °C (max 85) · VRAM 5689 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.enabled=off (command line: --set)`
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
