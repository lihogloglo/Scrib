# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6126 frames over 65.6 s · **median 10.61 ms (94 fps)** · mean 10.72 · p95 12.91 · p99 15.05 (1% low 66 fps) · worst 31.36 ms · 44 spikes over 1.5× median · 36.6 stutters/min (work 32.0/min)
- warm-up: 354 frames over 9.7 s, worst 2511.9 ms
- hitches (> 26.5 ms): **2** measured, 46 in warm-up
- timed GPU passes: 6.74 ms of a 10.72 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.61 ms, limit 11 ms
- **FAIL** frame_ms: 40 stutters (> 1.5x moving median of 60) in 1.09 min = 36.56/min, limit 1/min
- pass worst frame 31.36 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.043 | 2.190 | 2.630 | 0.93 | 0.181 |
| main_opaque_pass_3d | 1.262 | 1.352 | 1.919 | 0.93 | 0.070 |
| early prepass | 0.497 | 0.533 | 0.764 | 0.93 | 0.093 |
| bloom | 0.323 | 0.346 | 0.674 | 0.93 | 0.067 |
| atmosphere_luts | 0.310 | 0.332 | 0.477 | 0.93 | 0.004 |
| auxiliary_views | 0.228 | 0.245 | 0.987 | 0.93 | 0.216 |
| water_velocity | 0.224 | 0.240 | 0.248 | 0.93 | 0.002 |
| water_sspr | 0.201 | 0.215 | 0.207 | 0.93 | 0.002 |
| ssao | 0.189 | 0.203 | 0.205 | 0.93 | 0.008 |
| atmos_froxel_inject | 0.128 | 0.137 | 0.130 | 0.93 | 0.001 |
| upscaling | 0.125 | 0.134 | 0.150 | 0.93 | 0.004 |
| water_fft | 0.120 | 0.128 | 0.128 | 0.93 | 0.003 |
| uniform_allocation | 0.116 | 0.124 | 0.144 | 0.93 | 0.023 |
| clouds_composite | 0.101 | 0.108 | 0.109 | 0.93 | 0.002 |
| bin_unpacking | 0.090 | 0.097 | 0.116 | 0.93 | 0.018 |
| gpu_instance_materialization | 0.089 | 0.095 | 0.098 | 0.93 | 0.004 |
| tonemapping | 0.087 | 0.093 | 0.096 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.073 | 0.078 | 0.143 | 0.93 | 0.009 |
| clustering | 0.064 | 0.069 | 0.068 | 0.93 | 0.020 |
| early_mesh_preprocessing | 0.053 | 0.057 | 0.071 | 0.93 | 0.004 |
| clouds_trace | 0.052 | 0.056 | 0.078 | 0.93 | 0.001 |
| render_sky | 0.051 | 0.055 | 0.054 | 0.93 | 0.002 |
| clouds_resolve | 0.046 | 0.050 | 0.051 | 0.93 | 0.001 |
| surface_trail | 0.042 | 0.063 | 0.057 | 0.66 | 0.002 |
| lightprobe_irradiance_map | 0.036 | 0.464 | 0.766 | 0.08 | 0.001 |
| atmos_froxel_integrate | 0.029 | 0.031 | 0.031 | 0.93 | 0.001 |
| water_snapshot | 0.024 | 0.026 | 0.034 | 0.93 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.023 | 0.93 | 0.002 |
| range_unpacking | 0.021 | 0.023 | 0.028 | 0.93 | 0.003 |
| per_view_shadow_pass | 0.017 | 0.019 | 0.026 | 0.93 | 0.005 |
| lightprobe_radiance_map | 0.017 | 0.220 | 0.230 | 0.08 | 0.004 |
| night_sky | 0.016 | 0.017 | 0.042 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.030 | 0.93 | 0.199 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.93 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.010 | 0.93 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.473 | 11.535 | 23.24 | 58033.3 |
| render/busy/ms | 8.940 | 11.032 | 22.77 | 54763.7 |
| main/busy/ms | 8.663 | 10.161 | 28.25 | 53072.0 |
| stage/render/render | 5.719 | 7.270 | 18.26 | 35032.2 |
| stage/main/post_update | 5.013 | 6.098 | 10.47 | 30708.5 |
| render/graph/ms | 5.003 | 6.528 | 17.50 | 30645.5 |
| encode//sum | 4.362 | 5.956 | 26.77 | 26721.1 |
| stage/render/graph_submit | 3.572 | 4.723 | 15.14 | 21882.5 |
| encode/submit | 2.327 | 3.268 | 14.02 | 14253.5 |
| stage/main/update | 1.882 | 2.369 | 21.42 | 11529.9 |
| stage/render/camera_driver | 1.366 | 1.833 | 3.94 | 8367.6 |
| encode/finish_wall | 1.319 | 1.737 | 4.09 | 8080.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.819 | 1.120 | 2.11 | 5014.2 |
| encode/pass:shadow_directional_light/sum | 0.807 | 1.267 | 1.92 | 4944.5 |
| encode/pass:early prepass/sum | 0.800 | 1.101 | 1.87 | 4898.5 |
| stage/render/prepare_write_phase_buffers | 0.639 | 0.961 | 2.05 | 3916.1 |
| stage/main/fixed_main_loop | 0.550 | 0.974 | 1.79 | 3372.0 |
| bigspace/total/ms | 0.544 | 1.354 | 3.07 | 3333.7 |
| stage/render/prepare_assets | 0.537 | 0.690 | 11.22 | 3290.6 |
| stage/render/prepare_resources | 0.446 | 0.547 | 4.63 | 2729.9 |
| render/present/ms | 0.441 | 1.172 | 10.46 | 2699.9 |
| stage/render/prepare_meshes | 0.440 | 0.553 | 8.03 | 2693.7 |
| stage/main/first | 0.396 | 0.514 | 6.32 | 2428.1 |
| encode/pass:shadow_point_light/sum | 0.388 | 2.111 | 4.20 | 2378.6 |
| stage/main/pre_update | 0.383 | 0.535 | 3.33 | 2343.7 |
| controller/feed/ms | 0.328 | 0.429 | 0.93 | 2009.7 |
| stage/render/prepare_bind_groups | 0.314 | 0.520 | 2.18 | 1922.3 |
| stage/render/prepare_flush | 0.293 | 0.509 | 4.89 | 1795.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.285 | 0.460 | 1.11 | 1748.4 |
| stage/render/prepare_batch_phases | 0.284 | 0.435 | 0.88 | 1737.5 |
| stage/main/last | 0.258 | 0.310 | 1.04 | 1581.1 |
| bigspace/low_precision/ms | 0.240 | 0.812 | 2.72 | 1470.7 |
| stream/drive/ms | 0.211 | 0.309 | 10.86 | 1295.5 |
| stage/render/create_views | 0.187 | 0.239 | 0.89 | 1143.5 |
| stage/render/extract_commands | 0.186 | 0.221 | 0.55 | 1140.7 |
| stage/render/specialize | 0.183 | 0.270 | 3.00 | 1122.3 |
| pipelines/compile_ms | 0.172 | 0.000 | 136.47 | 1051.7 |
| pipelines/front_ms | 0.167 | 0.000 | 132.96 | 1022.1 |
| anim/evaluate/ms | 0.158 | 0.255 | 1.93 | 964.9 |
| stream/tick/ms | 0.155 | 0.229 | 0.54 | 952.3 |
| bigspace/high_precision/ms | 0.154 | 0.677 | 1.73 | 941.0 |
| anim/evaluate_cpu/ms | 0.153 | 0.194 | 0.55 | 934.8 |
| stage/render/prepare_views | 0.136 | 0.180 | 0.48 | 830.9 |
| stage/render/queue | 0.113 | 0.184 | 3.01 | 690.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.099 | 0.166 | 0.66 | 604.2 |
| stage/render/phase_sort | 0.094 | 0.132 | 0.43 | 576.3 |
| render/acquire/ms | 0.093 | 0.125 | 0.43 | 569.7 |
| controller/move/ms | 0.081 | 0.208 | 0.48 | 494.2 |
| controller/write/ms | 0.074 | 0.101 | 0.37 | 453.0 |
| terrain/update/ms | 0.062 | 0.299 | 2.28 | 379.6 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.076 | 0.39 | 301.0 |
| physics/step/ms | 0.043 | 0.085 | 0.50 | 261.7 |
| encode/resolve_diagnostics/sum | 0.032 | 0.045 | 0.50 | 197.3 |
| physics/insert/ms | 0.029 | 0.340 | 1.09 | 179.7 |
| terrain/select/ms | 0.024 | 0.264 | 0.67 | 147.7 |
| stage/render/cleanup | 0.023 | 0.035 | 0.97 | 142.6 |
| weather/map/ms | 0.018 | 0.024 | 0.22 | 107.5 |
| far/select/ms | 0.017 | 0.175 | 0.77 | 101.5 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.33 | 82.6 |
| anim/apply/ms | 0.012 | 0.021 | 0.30 | 71.6 |
| weather/eval/ms | 0.011 | 0.014 | 0.26 | 66.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.12 | 65.2 |
| script/update/ms | 0.008 | 0.012 | 0.26 | 50.7 |
| controller/avoid/ms | 0.007 | 0.013 | 0.17 | 41.3 |
| stream/despawn/ms | 0.006 | 0.000 | 1.27 | 36.8 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 31.1 |
| pipelines/driver_ms | 0.005 | 0.000 | 4.14 | 29.6 |
| stream/spawn/ms | 0.004 | 0.000 | 1.11 | 22.1 |
| script/ui/ms | 0.004 | 0.005 | 0.05 | 21.9 |
| stream/upload/ms | 0.003 | 0.000 | 1.33 | 20.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.71 | 14.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.33 | 7.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.4 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 561 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 583 |
| crowd/c0/meshes/count | count | 368 | 646 | 162 |
| crowd/c1/count | count | 145 | 156 | 148 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5613 |
| crowd/c1/meshes/count | count | 1165 | 1295 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 205 |
| crowd/joints_extracted/count | count | 686 | 5528 | 459 |
| crowd/joints_uploaded/count | count | 3596 | 6220 | 5775 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.45 | 1.51 | 0.51 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.33 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.12 | 0.01 |
| encode/finish_critical | ms | 0.97 | 2.11 | 0.90 |
| encode/pass:early prepass/max | ms | 0.80 | 1.87 | 0.74 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 2.11 | 0.90 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.66 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 1.11 | 0.26 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 1.92 | 0.80 |
| encode/pass:shadow_point_light/max | ms | 0.11 | 1.35 | 1.00 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.50 | 0.04 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.06 | 28.25 | 9.13 |
| gpu/timed/ms | ms | 7.92 | 9.48 | 8.09 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 156 | 386 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 336 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 156 | 386 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 336 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2474 | 3524 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2474 | 3524 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 25 | 159 | 8 |
| phases/shadow/bins | count | 1215 | 1668 | 671 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2860 | 3517 | 2027 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 157 | 197 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 334 | 335 | 335 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 17 | 469 | 11 |
| preprocess/bind_groups_created | count | 12 | 463 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 132 | 214 | 123 |
| process/cpu_usage | % | 26.01 | 40.29 | 24.52 |
| process/mem_usage | GiB | 5.44 | 5.67 | 5.67 |
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
| skin/blocks_changed/count | count | 14 | 93 | 10 |
| skin/blocks_skipped/count | count | 145 | 205 | 195 |
| skin/blocks_walked/count | count | 14 | 28 | 10 |
| skin/joint_bytes_written/bytes | bytes | 48.4 k | 808.1 k | 35.1 k |
| skin/joints_written/count | count | 756 | 12.6 k | 548 |
| skin/upload_writes/count | count | 7 | 23 | 6 |
| started_load_count |  | 6440 | 6564 | 6564 |
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
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.19 | 51.76 | 34.49 |
| system/mem_usage | % | 56.08 | 57.07 | 57.03 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2666 | 2724 | 2724 |
| views/all/batch_sets | count | 82 | 218 | 66 |
| views/all/bins | count | 3119 | 3778 | 2961 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 8379 | 10.6 k | 7969 |
| views/all/pipelines | count | 206 | 244 | 66 |
| views/all/pipelines_distinct | count | 70 | 74 | 61 |
| views/all/work_items | count | 154.0 k | 206.8 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 34 | 8 |
| views/cascade/bins | count | 716 | 854 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2116 | 2427 | 2027 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 130 | 0 |
| views/lamp_face/bins | count | 494 | 880 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 24 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 741 | 1177 | 0 |
| views/lamp_face/pipelines | count | 132 | 169 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 697 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2014 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5892 | 7486 | 5942 |
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
| anim/bones_written/count | count | 43.18 | 231 | 264.5 k |
| anim/events/count | count | 0.09 | 3 | 521 |
| assets/Image/added | count | 0.10 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.47 | 273 | 2874 |
| assets/Mesh/removed | count | 0.79 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.22 | 10 | 7473 |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.24 | 1186 | 1490 |
| encode//count | count | 37.01 | 204 | 226.7 k |
| encode/bind_walk/init_textures | count | 9523 | 23.9 k | 58.34 M |
| encode/bind_walk/init_textures_skipped | count | 61.8 k | 81.4 k | 378.82 M |
| encode/bind_walk/inits | count | 564 | 1019 | 3.45 M |
| encode/bind_walk/inits_skipped | count | 144 | 243 | 884.3 k |
| encode/bind_walk/merge_textures | count | 9460 | 23.8 k | 57.95 M |
| encode/bind_walk/merge_textures_skipped | count | 10.6 k | 19.9 k | 64.79 M |
| encode/bind_walk/merges | count | 103 | 283 | 631.1 k |
| encode/bind_walk/merges_skipped | count | 130 | 232 | 797.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6126 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6126 |
| encode/pass:early prepass/count | count | 1 | 1 | 6126 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6126 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6126 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6126 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6126 |
| encode/pass:shadow_point_light/count | count | 2.26 | 44 | 13.9 k |
| encode/queue/command_buffers | count | 49.29 | 258 | 301.9 k |
| encode/queue/submits | count | 2.01 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.62 M | 14.19 G |
| encode/queue/write_buffer/calls | count | 272 | 714 | 1.67 M |
| encode/queue/write_buffer_with/bytes | bytes | 161.7 k | 1.28 M | 990.34 M |
| encode/queue/write_buffer_with/calls | count | 10.39 | 828 | 63.7 k |
| encode/queue/write_texture/bytes | bytes | 2079 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6126 |
| physics/inserts | count | 0.37 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 175 | 2335 |
| physics/steps | count | 0.64 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 12 |
| pipelines/created | count | 0.00 | 1 | 13 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 30 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5526 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1212 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.5 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2511.9 | main/busy/ms 2480, stage/main/post_update 2439, stage/main/update 31.49, stage/main/pre_update 6.15, stage/main/last 1.30, surface/advance_cpu_ms 0.72, stage/main/first 0.48, stream/drive/ms 0.43, stage/main/fixed_main_loop 0.36, surface/cell_upload_ms 0.29, bigspace/total/ms 0.10, stream/tick/ms 0.10 |
| warm-up | 3.29 | 782.9 | main/busy/ms 2635, stage/main/post_update 2466, render/thread/ms 745, render/busy/ms 577, stage/render/prepare_assets 528, stage/render/prepare_views 166, render/acquire/ms 166, stage/main/update 152, stage/render/create_views 28.23, stage/render/render 11.48, render/graph/ms 8.24, stage/main/pre_update 8.02 |
| warm-up | 3.36 | 67.4 | render/thread/ms 796, render/busy/ms 626, stage/render/prepare_assets 563, main/busy/ms 179, stage/render/prepare_views 166, render/acquire/ms 166, stage/main/update 129, pipelines/compile_ms 41.91, pipelines/front_ms 39.28, stage/main/post_update 37.48, stage/render/create_views 28.63, terrain/update/ms 21.83 |
| warm-up | 3.40 | 41.8 | pipelines/compile_ms 138, pipelines/front_ms 130, render/thread/ms 84.07, pipelines/wait_ms 80.63, main/busy/ms 73.67, render/busy/ms 69.67, stage/main/post_update 49.51, stage/render/prepare_assets 35.70, stage/render/render 32.93, terrain/update/ms 26.72, render/graph/ms 18.05, pipelines/lock_ms 15.24 |
| warm-up | 3.52 | 115.5 | pipelines/wait_ms 3807, pipelines/compile_ms 1193, pipelines/front_ms 1100, render/thread/ms 139, render/busy/ms 125, pipelines/driver_ms 92.35, main/busy/ms 85.97, stage/main/post_update 49.71, stage/render/render 47.09, stage/render/prepare_flush 33.92, render/graph/ms 32.47, stage/render/prepare_bind_groups 27.74 |
| warm-up | 3.61 | 91.8 | pipelines/wait_ms 6201, pipelines/compile_ms 1564, pipelines/front_ms 1430, render/thread/ms 196, render/busy/ms 194, pipelines/driver_ms 134, main/busy/ms 127, stage/main/update 76.65, stage/render/prepare_flush 43.50, stage/render/prepare_assets 32.89, stage/render/render 30.75, stage/main/post_update 29.49 |
| warm-up | 3.69 | 78.8 | pipelines/wait_ms 2898, pipelines/compile_ms 1478, pipelines/front_ms 1383, render/thread/ms 165, render/busy/ms 163, main/busy/ms 114, pipelines/driver_ms 95.06, stage/render/prepare_assets 83.49, stage/main/update 61.42, stage/main/post_update 32.33, stage/render/prepare_resources 27.36, stage/render/prepare_write_phase_buffers 22.04 |
| warm-up | 3.72 | 29.0 | pipelines/compile_ms 1360, pipelines/front_ms 1288, pipelines/wait_ms 461, render/thread/ms 92.41, render/busy/ms 90.57, pipelines/driver_ms 72.70, stage/render/prepare_assets 69.79, main/busy/ms 41.94, stage/main/post_update 22.62, stage/render/render 14.48, stage/main/update 12.36, render/graph/ms 12.32 |
| warm-up | 3.76 | 43.0 | pipelines/compile_ms 1147, pipelines/front_ms 1046, pipelines/driver_ms 101, pipelines/wait_ms 77.73, render/thread/ms 56.34, render/busy/ms 54.60, main/busy/ms 52.80, stage/render/prepare_assets 29.66, stage/main/post_update 19.95, stage/main/pre_update 15.04, stage/render/render 12.46, render/graph/ms 10.42 |
| warm-up | 3.81 | 29.7 | pipelines/compile_ms 926, pipelines/front_ms 894, pipelines/wait_ms 222, render/thread/ms 46.70, render/busy/ms 44.27, main/busy/ms 41.46, pipelines/driver_ms 32.16, stage/main/post_update 25.21, encode//sum 17.47, stage/render/render 14.03, stage/render/prepare_resources 12.27, stage/main/update 11.98 |
| warm-up | 3.84 | 31.0 | pipelines/compile_ms 260, pipelines/front_ms 247, main/busy/ms 101, stage/main/update 64.40, render/thread/ms 55.33, render/busy/ms 53.06, stage/main/post_update 29.42, stage/render/prepare_write_phase_buffers 14.03, pipelines/driver_ms 12.93, stage/render/render 12.13, terrain/update/ms 10.19, render/graph/ms 9.56 |
| warm-up | 3.99 | 147.3 | pipelines/compile_ms 1349, pipelines/front_ms 1274, main/busy/ms 98.54, render/thread/ms 89.70, render/busy/ms 88.09, pipelines/driver_ms 75.25, stage/main/update 66.69, stage/render/prepare_assets 56.01, pipelines/wait_ms 46.45, stage/main/post_update 20.17, stage/render/render 12.42, render/graph/ms 10.31 |
| warm-up | 4.03 | 37.9 | pipelines/compile_ms 1593, pipelines/front_ms 1509, render/thread/ms 84.18, pipelines/driver_ms 84.04, render/busy/ms 82.74, stage/render/prepare_assets 65.87, pipelines/wait_ms 47.75, main/busy/ms 28.62, stage/main/update 12.57, stage/render/render 11.77, render/graph/ms 9.97, stage/render/graph_submit 7.56 |
| warm-up | 4.11 | 35.8 | main/busy/ms 51.67, pipelines/compile_ms 43.01, render/thread/ms 41.45, pipelines/front_ms 40.84, render/busy/ms 39.39, stage/main/update 35.73, stage/render/render 15.22, render/graph/ms 12.45, stage/main/post_update 11.73, stream/drive/ms 11.27, stage/render/graph_submit 8.33, encode//sum 6.82 |
| warm-up | 4.18 | 62.6 | main/busy/ms 74.48, render/thread/ms 70.28, render/busy/ms 68.37, stage/main/update 47.43, stage/render/prepare_assets 26.62, stream/drive/ms 20.76, stage/render/render 16.02, render/graph/ms 13.09, stage/main/post_update 12.93, stage/render/graph_submit 9.08, stage/render/prepare_bind_groups 6.91, encode//sum 6.82 |
| warm-up | 4.25 | 77.7 | pipelines/compile_ms 745, pipelines/front_ms 723, render/thread/ms 85.36, render/busy/ms 83.82, main/busy/ms 59.68, stage/render/prepare_assets 53.65, stage/main/update 33.80, stream/drive/ms 22.96, pipelines/driver_ms 21.38, stage/render/render 12.51, render/graph/ms 10.38, stage/main/post_update 10.05 |
| warm-up | 4.31 | 55.3 | pipelines/compile_ms 1210, pipelines/front_ms 1178, render/thread/ms 77.95, render/busy/ms 76.48, stage/render/prepare_assets 55.26, main/busy/ms 40.93, pipelines/driver_ms 32.30, stage/main/update 20.13, stage/render/render 13.76, render/graph/ms 11.81, stage/main/post_update 11.75, stage/render/graph_submit 8.42 |
| warm-up | 4.37 | 59.4 | pipelines/compile_ms 567, pipelines/front_ms 552, main/busy/ms 179, stage/main/update 150, render/thread/ms 84.27, render/busy/ms 82.77, stage/render/prepare_assets 37.17, stage/render/render 18.21, render/graph/ms 16.26, pipelines/driver_ms 15.54, stage/render/graph_submit 13.28, stage/render/prepare_bind_groups 12.63 |
| warm-up | 4.57 | 196.4 | pipelines/compile_ms 2107, pipelines/front_ms 2038, main/busy/ms 176, stage/main/update 149, render/thread/ms 111, render/busy/ms 109, pipelines/wait_ms 92.13, pipelines/driver_ms 68.69, stage/render/prepare_assets 49.46, stage/render/render 32.34, render/graph/ms 29.82, stage/render/graph_submit 27.08 |
| warm-up | 4.62 | 56.9 | pipelines/compile_ms 2361, pipelines/front_ms 2267, pipelines/wait_ms 185, render/thread/ms 114, render/busy/ms 112, main/busy/ms 105, pipelines/driver_ms 94.28, stage/main/update 52.87, stage/render/prepare_assets 43.99, stage/render/render 36.35, render/graph/ms 33.85, stage/main/post_update 28.16 |
| warm-up | 4.75 | 125.3 | pipelines/compile_ms 916, pipelines/front_ms 868, main/busy/ms 130, render/thread/ms 122, render/busy/ms 120, pipelines/wait_ms 98.10, stage/main/update 64.00, pipelines/driver_ms 48.30, stage/main/post_update 43.19, stage/render/prepare_resources 31.75, stage/render/prepare_assets 26.32, stage/render/prepare_flush 20.29 |
| warm-up | 4.80 | 56.4 | pipelines/compile_ms 2065, pipelines/front_ms 1998, pipelines/wait_ms 244, render/thread/ms 126, render/busy/ms 124, main/busy/ms 88.72, pipelines/driver_ms 67.31, stage/render/prepare_resources 46.44, stage/main/post_update 40.48, stage/main/update 36.57, bigspace/total/ms 24.81, stage/render/prepare_assets 22.50 |
| warm-up | 4.86 | 51.3 | pipelines/compile_ms 2220, pipelines/front_ms 2152, pipelines/wait_ms 273, render/thread/ms 95.83, render/busy/ms 93.92, main/busy/ms 74.31, pipelines/driver_ms 67.93, stage/main/update 30.32, stage/main/post_update 30.10, stage/render/render 24.07, encode//sum 22.79, stage/render/prepare_resources 22.62 |
| warm-up | 4.95 | 95.3 | pipelines/compile_ms 798, pipelines/front_ms 776, render/thread/ms 130, render/busy/ms 128, main/busy/ms 110, stage/main/update 65.54, stage/render/prepare_write_phase_buffers 35.63, pipelines/wait_ms 32.33, stage/render/prepare_assets 29.68, encode//sum 25.97, stage/render/render 22.85, pipelines/driver_ms 22.09 |
| warm-up | 5.04 | 91.3 | pipelines/compile_ms 1188, pipelines/front_ms 1153, main/busy/ms 163, render/thread/ms 151, render/busy/ms 149, stage/main/update 116, stream/drive/ms 56.81, stage/render/prepare_assets 53.48, pipelines/driver_ms 35.08, pipelines/wait_ms 34.16, encode//sum 33.46, stage/render/prepare_write_phase_buffers 31.94 |
| warm-up | 5.15 | 107.1 | pipelines/compile_ms 1817, pipelines/front_ms 1769, render/thread/ms 133, render/busy/ms 131, main/busy/ms 112, stage/main/update 83.43, stream/drive/ms 64.14, pipelines/driver_ms 48.31, stage/render/prepare_assets 48.24, stage/render/render 43.35, render/graph/ms 39.78, pipelines/wait_ms 37.13 |
| warm-up | 5.20 | 54.6 | pipelines/compile_ms 740, pipelines/front_ms 722, render/thread/ms 123, render/busy/ms 121, main/busy/ms 72.54, stage/render/render 66.84, render/graph/ms 62.52, encode//sum 43.20, stage/render/graph_submit 37.83, stage/main/update 31.45, stage/main/post_update 29.35, stage/render/camera_driver 24.43 |
| warm-up | 5.25 | 42.6 | pipelines/compile_ms 647, pipelines/front_ms 629, main/busy/ms 86.58, render/thread/ms 84.44, render/busy/ms 82.54, stage/render/render 44.52, render/graph/ms 40.81, stage/main/update 31.54, pipelines/wait_ms 31.12, encode//sum 31.08, stage/render/graph_submit 29.06, stage/main/post_update 28.35 |
| warm-up | 5.28 | 38.0 | pipelines/compile_ms 633, pipelines/front_ms 616, main/busy/ms 61.91, render/thread/ms 58.90, render/busy/ms 57.25, pipelines/wait_ms 30.36, stage/render/prepare_assets 29.36, stage/main/update 26.44, pipelines/driver_ms 17.65, stage/main/pre_update 17.14, stage/main/post_update 12.08, stage/render/render 11.71 |
| warm-up | 5.33 | 41.6 | pipelines/compile_ms 629, pipelines/front_ms 612, render/thread/ms 62.67, render/busy/ms 61.05, main/busy/ms 52.70, stage/main/update 24.72, stage/render/prepare_assets 20.19, stage/main/post_update 18.04, pipelines/driver_ms 17.03, stage/render/render 14.60, stage/render/prepare_resources 13.20, render/graph/ms 12.22 |
| warm-up | 5.39 | 38.8 | pipelines/compile_ms 1275, pipelines/front_ms 1244, render/thread/ms 58.11, render/busy/ms 56.61, main/busy/ms 45.49, pipelines/driver_ms 30.73, stage/main/update 23.66, stage/render/render 17.62, render/graph/ms 15.71, stage/render/prepare_write_phase_buffers 14.49, stage/main/post_update 14.39, stage/render/graph_submit 11.03 |
| warm-up | 5.42 | 31.5 | pipelines/compile_ms 854, pipelines/front_ms 833, render/thread/ms 59.24, render/busy/ms 57.83, main/busy/ms 44.17, stage/main/update 22.92, pipelines/driver_ms 21.49, stage/render/render 14.87, stage/render/prepare_write_phase_buffers 14.65, stage/render/prepare_assets 13.90, stage/main/post_update 13.63, render/graph/ms 12.97 |
| warm-up | 5.88 | 28.9 | pipelines/compile_ms 70.48, pipelines/front_ms 66.37, render/thread/ms 45.45, render/busy/ms 44.01, main/busy/ms 34.35, stage/render/render 17.28, render/graph/ms 15.47, stage/main/post_update 15.31, stage/main/update 13.90, stage/render/graph_submit 11.35, encode//sum 9.62, encode/submit 7.58 |
| warm-up | 6.13 | 35.8 | pipelines/compile_ms 622, pipelines/front_ms 607, main/busy/ms 45.75, render/thread/ms 29.66, render/busy/ms 28.48, stage/main/update 27.85, stream/drive/ms 18.14, pipelines/driver_ms 15.43, stage/render/render 11.37, stage/main/post_update 11.10, encode//sum 9.86, render/graph/ms 9.49 |
| warm-up | 6.25 | 28.3 | pipelines/compile_ms 173, pipelines/front_ms 169, render/thread/ms 45.48, render/busy/ms 43.46, main/busy/ms 34.19, stage/render/render 18.63, render/graph/ms 15.79, stage/main/update 15.19, encode//sum 13.53, stage/main/post_update 12.92, stage/render/prepare_assets 11.72, stage/render/graph_submit 10.23 |
| warm-up | 6.28 | 31.9 | render/thread/ms 49.23, render/busy/ms 47.57, main/busy/ms 33.78, stage/render/prepare_assets 21.20, stage/main/update 16.30, stage/render/render 12.82, encode//sum 11.39, stage/main/post_update 10.99, render/graph/ms 10.37, stage/render/graph_submit 5.85, stream/drive/ms 5.33, stage/render/camera_driver 4.33 |
| warm-up | 6.41 | 26.6 | pipelines/compile_ms 145, pipelines/front_ms 141, main/busy/ms 35.39, render/thread/ms 35.02, render/busy/ms 33.80, stage/main/update 22.70, stage/render/render 12.49, render/graph/ms 10.66, encode//sum 9.88, stage/render/prepare_assets 7.52, stage/main/post_update 7.37, stage/render/graph_submit 7.04 |
| warm-up | 6.52 | 31.1 | render/thread/ms 50.13, render/busy/ms 48.45, main/busy/ms 44.39, stage/main/update 22.27, stage/render/render 21.88, render/graph/ms 19.71, stage/main/post_update 16.16, stage/render/graph_submit 14.51, pipelines/compile_ms 14.33, encode//sum 14.08, pipelines/front_ms 13.37, encode/submit 11.23 |
| warm-up | 6.59 | 36.8 | render/thread/ms 51.01, render/busy/ms 49.77, main/busy/ms 43.74, stage/main/update 20.11, stage/render/prepare_assets 16.07, stage/main/post_update 14.99, stage/render/render 12.33, render/graph/ms 10.50, encode//sum 10.13, stage/render/prepare_meshes 6.88, stage/render/graph_submit 6.12, stream/drive/ms 4.69 |
| warm-up | 6.62 | 35.2 | render/thread/ms 63.42, render/busy/ms 61.45, main/busy/ms 44.52, stage/main/update 20.81, stage/render/render 19.60, stage/main/post_update 16.92, render/graph/ms 16.71, stage/render/prepare_assets 14.48, encode//sum 12.54, stage/render/graph_submit 10.70, stage/render/prepare_meshes 8.23, encode/submit 7.46 |
| warm-up | 6.68 | 32.6 | render/thread/ms 41.49, render/busy/ms 40.33, main/busy/ms 32.50, stage/render/prepare_assets 18.00, stage/main/update 16.86, stage/render/render 9.78, encode//sum 9.40, stage/main/post_update 9.00, render/graph/ms 8.45, stage/render/graph_submit 4.89, stage/render/prepare_meshes 4.07, stream/drive/ms 3.85 |
| warm-up | 6.82 | 39.6 | main/busy/ms 57.81, stage/main/update 42.60, render/thread/ms 36.53, render/busy/ms 35.12, stream/drive/ms 14.73, stage/render/render 13.80, encode//sum 12.22, render/graph/ms 11.94, stage/main/post_update 10.50, stage/render/graph_submit 7.69, stage/render/prepare_assets 7.06, stage/render/prepare_meshes 5.13 |
| warm-up | 6.86 | 40.1 | main/busy/ms 51.09, render/thread/ms 47.27, render/busy/ms 45.88, stage/main/update 33.50, stage/render/prepare_assets 17.27, stage/render/render 13.04, encode//sum 11.33, render/graph/ms 11.11, stage/main/post_update 10.69, stream/drive/ms 6.60, stage/render/graph_submit 6.41, stage/render/prepare_meshes 5.42 |
| warm-up | 6.90 | 31.2 | pipelines/compile_ms 144, pipelines/front_ms 141, render/thread/ms 46.37, render/busy/ms 45.42, main/busy/ms 33.09, stage/render/prepare_assets 22.32, stage/main/update 17.20, stage/render/render 10.41, encode//sum 9.66, stage/main/post_update 8.99, render/graph/ms 8.98, stream/drive/ms 5.84 |
| warm-up | 7.08 | 27.3 | pipelines/compile_ms 143, pipelines/front_ms 139, render/thread/ms 37.73, main/busy/ms 36.05, render/busy/ms 29.82, stage/render/render 20.87, stage/main/update 17.52, render/graph/ms 12.42, encode//sum 11.62, stage/main/post_update 9.93, stage/render/graph_submit 8.72, render/present/ms 7.69 |
| warm-up | 7.99 | 75.9 | pipelines/compile_ms 179, pipelines/front_ms 132, render/thread/ms 81.99, render/busy/ms 81.15, stage/render/render 74.06, render/graph/ms 72.65, encode//sum 71.19, stage/render/graph_submit 70.03, encode/finish_wall 65.95, encode/pass:early prepass/sum 64.94, pipelines/driver_ms 47.36, main/busy/ms 19.75 |
| measure | 18.28 | 29.6 | main/busy/ms 38.35, render/thread/ms 24.68, render/busy/ms 23.52, stage/main/update 23.28, stage/render/render 13.44, render/graph/ms 11.88, stage/main/post_update 10.01, encode//sum 9.80, stage/render/graph_submit 8.24, encode/submit 5.50, stage/render/prepare_assets 3.63, stage/render/camera_driver 3.45 |
| measure | 59.08 | 31.4 | render/thread/ms 39.90, render/busy/ms 38.75, main/busy/ms 25.77, stage/render/prepare_assets 17.51, stage/main/post_update 13.07, stage/render/render 10.45, render/graph/ms 8.93, encode//sum 8.74, stage/render/graph_submit 6.03, stage/main/update 5.28, stage/main/fixed_main_loop 3.51, encode/submit 3.41 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1512 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **82%** (min 55%) · 2625 MHz (min 2625) · memory 8101 MHz · 78.6 W · 74 °C (max 79) · VRAM 6522 MiB · throttled 100% of samples

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
