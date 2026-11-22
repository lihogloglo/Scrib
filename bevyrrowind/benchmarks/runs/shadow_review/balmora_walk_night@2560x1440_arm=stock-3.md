# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=off (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6270 frames over 65.6 s · **median 10.38 ms (96 fps)** · mean 10.47 · p95 12.46 · p99 14.42 (1% low 69 fps) · worst 32.00 ms · 36 spikes over 1.5× median · 27.4 stutters/min (work 26.5/min)
- warm-up: 352 frames over 9.4 s, worst 2464.1 ms
- hitches (> 26.0 ms): **3** measured, 40 in warm-up
- timed GPU passes: 6.76 ms of a 10.47 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.38 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 32.00 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.902 | 2.038 | 2.073 | 0.93 | 0.177 |
| main_opaque_pass_3d | 1.562 | 1.674 | 2.390 | 0.93 | 0.063 |
| early prepass | 0.485 | 0.519 | 0.755 | 0.93 | 0.089 |
| bloom | 0.296 | 0.317 | 0.324 | 0.93 | 0.067 |
| atmosphere_luts | 0.291 | 0.312 | 0.376 | 0.93 | 0.003 |
| auxiliary_views | 0.242 | 0.259 | 0.880 | 0.93 | 0.227 |
| water_velocity | 0.210 | 0.225 | 0.247 | 0.93 | 0.002 |
| water_sspr | 0.189 | 0.202 | 0.204 | 0.93 | 0.002 |
| ssao | 0.176 | 0.188 | 0.193 | 0.93 | 0.007 |
| atmos_froxel_inject | 0.120 | 0.128 | 0.130 | 0.93 | 0.002 |
| upscaling | 0.120 | 0.128 | 0.135 | 0.93 | 0.004 |
| water_fft | 0.116 | 0.124 | 0.128 | 0.93 | 0.002 |
| uniform_allocation | 0.108 | 0.115 | 0.140 | 0.93 | 0.021 |
| clouds_composite | 0.098 | 0.105 | 0.109 | 0.93 | 0.002 |
| gpu_instance_materialization | 0.088 | 0.094 | 0.099 | 0.93 | 0.004 |
| bin_unpacking | 0.085 | 0.091 | 0.111 | 0.93 | 0.016 |
| tonemapping | 0.081 | 0.087 | 0.092 | 0.93 | 0.004 |
| main_transmissive_pass_3d | 0.069 | 0.074 | 0.146 | 0.93 | 0.008 |
| clustering | 0.060 | 0.064 | 0.067 | 0.93 | 0.019 |
| clouds_trace | 0.052 | 0.056 | 0.077 | 0.93 | 0.001 |
| early_mesh_preprocessing | 0.051 | 0.054 | 0.071 | 0.93 | 0.003 |
| render_sky | 0.049 | 0.053 | 0.054 | 0.93 | 0.002 |
| clouds_resolve | 0.046 | 0.049 | 0.050 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.042 | 0.412 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.036 | 0.057 | 0.057 | 0.64 | 0.002 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.029 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.231 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| range_unpacking | 0.020 | 0.022 | 0.027 | 0.93 | 0.003 |
| per_view_shadow_pass | 0.016 | 0.017 | 0.027 | 0.93 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.042 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.030 | 0.93 | 0.194 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.010 | 0.93 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.93 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.214 | 11.180 | 21.65 | 57774.6 |
| render/busy/ms | 8.685 | 10.686 | 21.22 | 54456.0 |
| main/busy/ms | 8.524 | 10.087 | 26.14 | 53443.2 |
| stage/render/render | 5.593 | 7.074 | 13.27 | 35066.0 |
| stage/main/post_update | 4.967 | 6.092 | 10.23 | 31141.0 |
| render/graph/ms | 4.891 | 6.380 | 12.28 | 30663.9 |
| encode//sum | 4.415 | 6.300 | 23.67 | 27680.6 |
| stage/render/graph_submit | 3.495 | 4.562 | 9.28 | 21911.6 |
| encode/submit | 2.288 | 3.123 | 7.10 | 14347.8 |
| stage/main/update | 1.820 | 2.297 | 20.47 | 11410.7 |
| stage/render/camera_driver | 1.334 | 1.845 | 3.68 | 8367.3 |
| encode/finish_wall | 1.291 | 1.694 | 4.06 | 8094.6 |
| encode/pass:shadow_directional_light/sum | 0.780 | 1.214 | 2.34 | 4893.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.778 | 1.066 | 1.90 | 4878.1 |
| encode/pass:early prepass/sum | 0.766 | 1.060 | 1.66 | 4801.8 |
| stage/render/prepare_write_phase_buffers | 0.620 | 0.931 | 2.98 | 3888.4 |
| bigspace/total/ms | 0.540 | 1.341 | 2.41 | 3387.0 |
| stage/main/fixed_main_loop | 0.530 | 0.952 | 2.08 | 3323.9 |
| stage/render/prepare_assets | 0.512 | 0.647 | 10.43 | 3211.2 |
| render/present/ms | 0.436 | 1.187 | 2.03 | 2732.9 |
| stage/render/prepare_resources | 0.430 | 0.511 | 0.90 | 2695.6 |
| stage/render/prepare_meshes | 0.419 | 0.519 | 3.89 | 2625.5 |
| stage/main/first | 0.399 | 0.514 | 5.14 | 2499.6 |
| stage/main/pre_update | 0.383 | 0.548 | 6.33 | 2400.3 |
| encode/pass:shadow_point_light/sum | 0.368 | 1.812 | 3.47 | 2308.2 |
| controller/feed/ms | 0.320 | 0.402 | 0.78 | 2004.2 |
| stage/render/prepare_bind_groups | 0.299 | 0.486 | 2.03 | 1877.2 |
| stage/render/prepare_flush | 0.285 | 0.478 | 1.27 | 1789.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.278 | 0.443 | 1.06 | 1743.1 |
| stage/render/prepare_batch_phases | 0.270 | 0.406 | 0.94 | 1691.9 |
| stage/main/last | 0.248 | 0.290 | 0.85 | 1557.6 |
| bigspace/low_precision/ms | 0.226 | 0.764 | 2.19 | 1414.2 |
| stream/drive/ms | 0.198 | 0.275 | 10.12 | 1243.2 |
| stage/render/create_views | 0.185 | 0.237 | 0.90 | 1161.7 |
| stage/render/extract_commands | 0.181 | 0.214 | 0.80 | 1134.0 |
| stage/render/specialize | 0.178 | 0.256 | 2.41 | 1115.7 |
| bigspace/high_precision/ms | 0.168 | 0.768 | 1.87 | 1051.1 |
| anim/evaluate/ms | 0.155 | 0.260 | 2.53 | 968.8 |
| anim/evaluate_cpu/ms | 0.148 | 0.190 | 0.58 | 929.3 |
| stream/tick/ms | 0.145 | 0.207 | 0.50 | 910.5 |
| stage/render/prepare_views | 0.135 | 0.173 | 1.20 | 844.3 |
| stage/render/queue | 0.107 | 0.174 | 2.29 | 669.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.097 | 0.162 | 0.80 | 609.5 |
| render/acquire/ms | 0.093 | 0.122 | 1.16 | 585.8 |
| stage/render/phase_sort | 0.091 | 0.126 | 0.38 | 572.8 |
| controller/move/ms | 0.077 | 0.187 | 0.88 | 484.7 |
| controller/write/ms | 0.074 | 0.101 | 0.45 | 463.7 |
| terrain/update/ms | 0.062 | 0.305 | 1.90 | 388.6 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.074 | 0.28 | 301.9 |
| physics/step/ms | 0.046 | 0.121 | 0.66 | 287.0 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.69 | 188.0 |
| physics/insert/ms | 0.027 | 0.323 | 0.95 | 170.6 |
| pipelines/compile_ms | 0.026 | 0.000 | 80.86 | 165.7 |
| pipelines/front_ms | 0.026 | 0.000 | 78.27 | 160.1 |
| terrain/select/ms | 0.024 | 0.271 | 0.76 | 152.9 |
| stage/render/cleanup | 0.024 | 0.033 | 0.81 | 151.5 |
| weather/map/ms | 0.018 | 0.024 | 0.26 | 111.0 |
| far/select/ms | 0.015 | 0.164 | 0.49 | 94.9 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.31 | 82.7 |
| anim/apply/ms | 0.011 | 0.021 | 0.49 | 71.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.68 | 67.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.16 | 65.4 |
| script/update/ms | 0.008 | 0.012 | 0.16 | 50.7 |
| controller/avoid/ms | 0.006 | 0.013 | 0.20 | 40.0 |
| stream/despawn/ms | 0.006 | 0.000 | 1.10 | 35.3 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 30.0 |
| script/ui/ms | 0.004 | 0.005 | 0.09 | 22.8 |
| stream/spawn/ms | 0.003 | 0.000 | 1.21 | 20.9 |
| stream/upload/ms | 0.003 | 0.000 | 1.20 | 19.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 13.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 7.1 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.67 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.22 | 4.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.09 | 2.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 31 | 16 |
| controller/movers/count | count | 19 | 21 | 16 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1291 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 147 | 156 | 148 |
| crowd/c1/joints/count | count | 5556 | 5942 | 5613 |
| crowd/c1/meshes/count | count | 1187 | 1272 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 205 |
| crowd/joints_extracted/count | count | 645 | 5581 | 487 |
| crowd/joints_uploaded/count | count | 3617 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.45 | 1.55 | 0.49 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.31 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.68 | 0.01 |
| encode/finish_critical | ms | 0.93 | 2.34 | 0.93 |
| encode/pass:early prepass/max | ms | 0.77 | 1.66 | 0.77 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.79 | 1.90 | 0.55 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.80 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 1.06 | 0.40 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.34 | 0.93 |
| encode/pass:shadow_point_light/max | ms | 0.12 | 1.54 | 0.74 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.69 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.86 | 26.14 | 8.45 |
| gpu/timed/ms | ms | 7.95 | 9.08 | 8.45 |
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
| phases/alpha_mask3d/items | count | 338 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 338 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2474 | 3554 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2474 | 3554 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 160 | 8 |
| phases/shadow/bins | count | 1258 | 1699 | 658 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2902 | 3552 | 2022 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 154 | 197 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2643 | 3182 | 2345 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 184 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 17 | 463 | 11 |
| preprocess/bind_groups_created | count | 12 | 457 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 128 | 213 | 123 |
| process/cpu_usage | % | 26.79 | 44.23 | 25.50 |
| process/mem_usage | GiB | 5.43 | 5.71 | 5.71 |
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
| skin/blocks_changed/count | count | 13 | 95 | 10 |
| skin/blocks_skipped/count | count | 147 | 204 | 195 |
| skin/blocks_walked/count | count | 13 | 26 | 10 |
| skin/joint_bytes_written/bytes | bytes | 45.8 k | 812.2 k | 35.8 k |
| skin/joints_written/count | count | 716 | 12.7 k | 560 |
| skin/upload_writes/count | count | 7 | 20 | 9 |
| started_load_count |  | 6442 | 6565 | 6565 |
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
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.13 | 44.85 | 29.22 |
| system/mem_usage | % | 47.38 | 48.08 | 48.07 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2888 | 3003 | 3003 |
| textures/images | count | 2668 | 2726 | 2726 |
| views/all/batch_sets | count | 82 | 215 | 66 |
| views/all/bins | count | 3143 | 3895 | 2948 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 8449 | 10.8 k | 7964 |
| views/all/pipelines | count | 201 | 244 | 66 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 154.0 k | 206.9 k | 154.0 k |
| views/cascade/batch_sets | count | 12 | 33 | 8 |
| views/cascade/bins | count | 712 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2115 | 2427 | 2022 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 131 | 0 |
| views/lamp_face/bins | count | 554 | 879 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 23 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 798 | 1192 | 0 |
| views/lamp_face/pipelines | count | 130 | 171 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 686 | 0 |
| views/main/batch_sets | count | 57 | 61 | 58 |
| views/main/bins | count | 2014 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5892 | 7528 | 5942 |
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
| anim/bones_written/count | count | 46.88 | 214 | 293.9 k |
| anim/events/count | count | 0.08 | 3 | 485 |
| assets/Image/added | count | 0.09 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 12 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.46 | 257 | 2874 |
| assets/Mesh/removed | count | 0.78 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.19 | 10 | 7480 |
| assets/StandardMaterial/removed | count | 0.06 | 52 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.24 | 1303 | 1489 |
| encode//count | count | 37.98 | 204 | 238.1 k |
| encode/bind_walk/init_textures | count | 9406 | 23.9 k | 58.97 M |
| encode/bind_walk/init_textures_skipped | count | 51.5 k | 78.2 k | 322.70 M |
| encode/bind_walk/inits | count | 566 | 1015 | 3.55 M |
| encode/bind_walk/inits_skipped | count | 136 | 235 | 849.8 k |
| encode/bind_walk/merge_textures | count | 9343 | 23.8 k | 58.58 M |
| encode/bind_walk/merge_textures_skipped | count | 11.9 k | 26.9 k | 74.82 M |
| encode/bind_walk/merges | count | 104 | 283 | 654.2 k |
| encode/bind_walk/merges_skipped | count | 127 | 231 | 795.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6270 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6270 |
| encode/pass:early prepass/count | count | 1 | 1 | 6270 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6270 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6270 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6270 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6270 |
| encode/pass:shadow_point_light/count | count | 2.51 | 44 | 15.7 k |
| encode/queue/command_buffers | count | 50.50 | 258 | 316.7 k |
| encode/queue/submits | count | 2.01 | 4 | 12.6 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.55 M | 14.57 G |
| encode/queue/write_buffer/calls | count | 271 | 705 | 1.70 M |
| encode/queue/write_buffer_with/bytes | bytes | 161.6 k | 1.28 M | 1.01 G |
| encode/queue/write_buffer_with/calls | count | 10.36 | 780 | 65.0 k |
| encode/queue/write_texture/bytes | bytes | 2198 | 1.05 M | 13.78 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 558 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6270 |
| physics/inserts | count | 0.36 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 175 | 2335 |
| physics/steps | count | 0.63 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 35 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5399 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1184 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.0 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2464.1 | main/busy/ms 2435, stage/main/post_update 2396, stage/main/update 29.34, stage/main/pre_update 6.95, stage/main/last 1.03, surface/advance_cpu_ms 0.78, stage/main/first 0.50, stage/main/fixed_main_loop 0.35, stream/drive/ms 0.31, surface/cell_upload_ms 0.30, bigspace/total/ms 0.09, weather/map/ms 0.08 |
| warm-up | 3.20 | 735.3 | main/busy/ms 2569, stage/main/post_update 2417, render/thread/ms 701, render/busy/ms 564, stage/render/prepare_assets 519, stage/main/update 136, stage/render/prepare_views 135, render/acquire/ms 135, stage/render/create_views 26.43, stage/render/render 10.44, stage/main/pre_update 8.66, render/graph/ms 6.92 |
| warm-up | 3.37 | 174.0 | render/thread/ms 859, render/busy/ms 627, stage/render/prepare_assets 568, main/busy/ms 165, pipelines/compile_ms 143, stage/render/prepare_views 135, render/acquire/ms 135, stage/main/update 114, stage/render/render 109, render/present/ms 96.70, pipelines/driver_ms 86.61, pipelines/front_ms 55.95 |
| warm-up | 3.41 | 39.9 | pipelines/compile_ms 588, pipelines/front_ms 374, pipelines/wait_ms 237, pipelines/driver_ms 215, render/thread/ms 196, main/busy/ms 155, stage/render/render 130, render/busy/ms 99.96, render/present/ms 95.76, stage/main/update 88.57, stage/main/post_update 51.60, stage/render/prepare_assets 49.94 |
| warm-up | 3.69 | 280.0 | pipelines/wait_ms 3859, pipelines/compile_ms 3450, pipelines/front_ms 2822, pipelines/driver_ms 627, encode//sum 501, render/thread/ms 228, render/busy/ms 225, main/busy/ms 154, stage/render/prepare_assets 128, stage/main/update 90.97, stage/render/render 87.71, render/graph/ms 84.29 |
| warm-up | 3.74 | 51.7 | pipelines/wait_ms 3686, pipelines/compile_ms 3333, pipelines/front_ms 2802, pipelines/driver_ms 531, encode//sum 514, render/thread/ms 239, render/busy/ms 237, stage/render/prepare_assets 166, main/busy/ms 66.48, stage/render/render 63.60, render/graph/ms 61.27, stage/render/graph_submit 58.53 |
| warm-up | 3.78 | 37.1 | pipelines/compile_ms 439, pipelines/front_ms 396, render/thread/ms 84.77, render/busy/ms 82.77, pipelines/wait_ms 77.12, main/busy/ms 67.23, stage/render/prepare_assets 64.65, pipelines/driver_ms 42.16, stage/main/update 22.28, stage/main/fixed_main_loop 22.00, stage/main/post_update 17.84, terrain/update/ms 17.77 |
| warm-up | 3.84 | 33.9 | pipelines/compile_ms 1017, pipelines/front_ms 981, pipelines/wait_ms 105, render/thread/ms 53.19, pipelines/driver_ms 36.43, main/busy/ms 29.28, stage/render/prepare_views 28.67, render/acquire/ms 28.50, render/busy/ms 23.41, encode//sum 18.97, stage/main/post_update 16.35, stage/render/render 13.05 |
| warm-up | 3.88 | 45.5 | pipelines/compile_ms 1154, pipelines/front_ms 1113, main/busy/ms 84.95, render/thread/ms 76.39, encode//sum 68.27, stage/main/update 59.37, pipelines/wait_ms 55.42, render/busy/ms 53.86, pipelines/driver_ms 41.51, stage/render/render 25.12, render/graph/ms 22.47, stage/render/prepare_views 21.47 |
| warm-up | 3.98 | 101.0 | pipelines/compile_ms 1452, pipelines/front_ms 1383, render/thread/ms 90.09, render/busy/ms 86.82, main/busy/ms 85.32, pipelines/wait_ms 70.23, pipelines/driver_ms 69.54, encode//sum 69.18, stage/main/update 59.01, stage/render/prepare_assets 42.61, stage/render/render 26.04, render/graph/ms 23.10 |
| warm-up | 4.04 | 58.9 | pipelines/compile_ms 1584, pipelines/front_ms 1511, render/thread/ms 86.66, render/busy/ms 84.94, pipelines/wait_ms 74.58, pipelines/driver_ms 72.71, stage/render/prepare_assets 66.82, main/busy/ms 26.95, stage/render/render 13.12, render/graph/ms 11.24, stage/main/update 9.54, stage/render/graph_submit 9.16 |
| warm-up | 4.13 | 32.0 | render/thread/ms 49.40, render/busy/ms 47.32, pipelines/compile_ms 42.98, pipelines/front_ms 40.88, main/busy/ms 35.95, stage/render/render 16.10, stage/main/update 14.44, stage/render/prepare_assets 14.32, stage/main/post_update 13.37, render/graph/ms 13.35, stage/render/graph_submit 9.40, encode//sum 8.36 |
| warm-up | 4.19 | 54.7 | pipelines/compile_ms 749, pipelines/front_ms 723, main/busy/ms 121, stage/main/update 97.91, render/thread/ms 68.25, render/busy/ms 66.26, stage/render/prepare_assets 39.17, pipelines/driver_ms 26.48, stage/render/render 14.59, render/graph/ms 11.50, stage/main/post_update 11.43, stage/render/graph_submit 8.13 |
| warm-up | 4.36 | 172.6 | pipelines/compile_ms 1977, pipelines/front_ms 1918, render/thread/ms 124, render/busy/ms 123, main/busy/ms 115, stage/main/update 99.44, stage/render/prepare_assets 70.78, pipelines/driver_ms 59.57, stage/render/prepare_resources 33.46, stage/render/render 12.28, render/graph/ms 9.93, stage/render/graph_submit 7.61 |
| warm-up | 4.45 | 87.7 | pipelines/compile_ms 1427, pipelines/front_ms 1325, render/thread/ms 169, render/busy/ms 163, pipelines/driver_ms 102, main/busy/ms 85.57, stage/main/update 65.55, stage/render/prepare_assets 47.03, stage/render/prepare_resources 43.28, stage/render/render 41.40, render/graph/ms 38.62, stage/render/graph_submit 35.61 |
| warm-up | 4.53 | 85.2 | pipelines/compile_ms 765, pipelines/front_ms 674, render/thread/ms 144, render/busy/ms 138, main/busy/ms 106, pipelines/driver_ms 91.20, pipelines/wait_ms 90.77, stage/main/update 70.58, stage/render/render 44.01, render/graph/ms 40.87, stage/render/graph_submit 36.16, stage/render/prepare_assets 35.68 |
| warm-up | 4.58 | 43.1 | pipelines/compile_ms 1806, pipelines/front_ms 1739, pipelines/wait_ms 151, render/thread/ms 100, render/busy/ms 98.25, main/busy/ms 96.02, pipelines/driver_ms 66.90, stage/main/post_update 51.73, stage/main/update 35.47, stage/render/prepare_assets 33.23, encode//sum 26.39, stage/render/render 25.57 |
| warm-up | 4.64 | 64.8 | pipelines/compile_ms 1984, pipelines/front_ms 1920, pipelines/wait_ms 316, render/thread/ms 81.36, render/busy/ms 79.75, main/busy/ms 79.29, pipelines/driver_ms 63.94, stage/main/post_update 37.79, stage/main/update 35.25, encode//sum 29.07, stage/render/render 26.53, render/graph/ms 24.33 |
| warm-up | 4.72 | 81.1 | pipelines/compile_ms 1219, pipelines/front_ms 1185, pipelines/wait_ms 517, render/thread/ms 114, render/busy/ms 113, main/busy/ms 87.58, encode//sum 63.34, stage/main/update 52.58, stage/render/render 42.51, render/graph/ms 40.35, pipelines/driver_ms 34.40, stage/render/prepare_resources 23.59 |
| warm-up | 4.83 | 102.9 | pipelines/compile_ms 2741, pipelines/front_ms 2621, pipelines/wait_ms 902, render/thread/ms 164, render/busy/ms 163, pipelines/driver_ms 121, main/busy/ms 106, encode//sum 67.48, stage/render/prepare_assets 64.67, stage/main/update 54.48, stage/render/render 45.68, render/graph/ms 43.13 |
| warm-up | 4.86 | 34.3 | pipelines/compile_ms 2831, pipelines/front_ms 2709, pipelines/wait_ms 815, pipelines/driver_ms 123, render/thread/ms 115, render/busy/ms 113, stage/render/prepare_assets 67.95, main/busy/ms 57.24, stage/main/update 23.41, stage/main/post_update 21.72, stage/render/render 18.58, encode//sum 16.71 |
| warm-up | 4.97 | 108.5 | pipelines/compile_ms 1013, pipelines/front_ms 990, pipelines/wait_ms 374, render/thread/ms 108, render/busy/ms 107, main/busy/ms 68.94, stage/render/prepare_assets 53.16, stage/main/update 48.43, stage/render/prepare_resources 23.76, pipelines/driver_ms 23.16, stage/render/render 15.00, render/graph/ms 12.47 |
| warm-up | 5.03 | 63.2 | pipelines/compile_ms 1078, pipelines/front_ms 1052, pipelines/wait_ms 200, render/thread/ms 127, render/busy/ms 125, main/busy/ms 70.48, stage/render/prepare_assets 68.12, stage/main/update 51.68, pipelines/driver_ms 26.42, stage/render/prepare_resources 23.18, stage/render/render 16.11, render/graph/ms 14.05 |
| warm-up | 5.06 | 31.6 | pipelines/compile_ms 724, pipelines/front_ms 700, main/busy/ms 84.39, render/thread/ms 72.63, render/busy/ms 71.23, stage/main/update 62.68, stream/drive/ms 35.54, stage/render/prepare_assets 29.40, pipelines/driver_ms 24.67, stage/render/render 20.46, render/graph/ms 18.48, stage/render/graph_submit 12.71 |
| warm-up | 5.14 | 74.1 | pipelines/compile_ms 654, pipelines/front_ms 632, main/busy/ms 106, stage/main/update 86.20, stream/drive/ms 63.42, render/thread/ms 58.15, render/busy/ms 56.77, pipelines/driver_ms 22.28, stage/render/render 20.42, render/graph/ms 18.43, stage/render/prepare_assets 16.04, stage/render/graph_submit 12.54 |
| warm-up | 5.18 | 43.7 | pipelines/compile_ms 592, pipelines/front_ms 573, main/busy/ms 73.05, render/thread/ms 55.65, stage/main/update 55.21, render/busy/ms 53.86, stream/drive/ms 33.97, pipelines/driver_ms 18.47, stage/render/render 17.96, stage/render/prepare_assets 17.16, render/graph/ms 15.78, encode//sum 10.29 |
| warm-up | 5.24 | 57.7 | pipelines/compile_ms 792, pipelines/front_ms 767, render/thread/ms 65.98, render/busy/ms 64.03, main/busy/ms 63.54, pipelines/wait_ms 30.72, stage/main/update 28.82, pipelines/driver_ms 25.01, stage/render/prepare_assets 21.92, stage/render/render 16.69, render/graph/ms 14.43, stage/main/pre_update 13.23 |
| warm-up | 5.30 | 41.1 | render/thread/ms 55.97, render/busy/ms 53.99, main/busy/ms 45.03, stage/render/prepare_resources 19.59, stage/main/update 19.43, stage/main/post_update 16.34, stage/render/render 15.57, render/graph/ms 12.72, encode//sum 10.58, stage/render/graph_submit 8.46, encode/submit 5.48, stage/render/prepare_assets 4.46 |
| warm-up | 5.33 | 33.6 | pipelines/compile_ms 198, pipelines/front_ms 191, render/thread/ms 64.42, render/busy/ms 63.25, main/busy/ms 41.99, stage/main/update 21.28, stage/render/prepare_assets 19.84, stage/render/prepare_resources 19.53, stage/render/render 12.90, encode//sum 11.16, stage/main/post_update 10.88, render/graph/ms 10.76 |
| warm-up | 5.36 | 28.2 | pipelines/compile_ms 779, pipelines/front_ms 756, render/thread/ms 48.32, render/busy/ms 47.13, main/busy/ms 32.38, stage/render/prepare_assets 24.05, pipelines/driver_ms 23.46, stage/main/update 14.94, stage/render/render 12.77, render/graph/ms 10.85, encode//sum 10.36, stage/main/post_update 8.52 |
| warm-up | 5.96 | 26.1 | pipelines/compile_ms 644, pipelines/front_ms 624, main/busy/ms 41.41, render/thread/ms 37.05, render/busy/ms 35.20, pipelines/driver_ms 19.70, stage/render/render 19.41, stage/main/update 18.76, render/graph/ms 17.18, stage/main/post_update 14.55, stage/render/graph_submit 9.61, encode//sum 9.22 |
| warm-up | 6.01 | 36.5 | pipelines/compile_ms 419, pipelines/front_ms 407, main/busy/ms 43.33, render/thread/ms 29.85, stage/main/update 29.24, render/busy/ms 28.64, stream/drive/ms 19.89, stage/render/render 13.61, pipelines/driver_ms 12.47, render/graph/ms 11.72, encode//sum 11.00, stage/main/post_update 7.90 |
| warm-up | 6.14 | 26.4 | pipelines/compile_ms 304, pipelines/front_ms 295, render/thread/ms 40.33, render/busy/ms 38.89, main/busy/ms 27.70, stage/render/render 14.36, stage/main/update 12.98, encode//sum 12.78, stage/render/prepare_assets 12.49, render/graph/ms 12.40, stage/main/post_update 10.59, stage/render/graph_submit 8.65 |
| warm-up | 6.19 | 29.9 | render/thread/ms 34.49, render/busy/ms 33.47, main/busy/ms 23.97, stage/render/prepare_assets 16.22, stage/main/update 10.85, stage/render/render 9.04, encode//sum 8.38, render/graph/ms 7.44, stage/main/post_update 6.58, stage/render/graph_submit 4.60, stream/drive/ms 3.18, stage/render/camera_driver 2.70 |
| warm-up | 6.40 | 28.3 | render/thread/ms 43.19, render/busy/ms 41.79, main/busy/ms 41.34, stage/main/update 19.77, stage/render/render 18.99, render/graph/ms 16.67, stage/main/post_update 15.07, stage/render/graph_submit 12.32, encode//sum 11.77, stage/render/prepare_assets 8.82, encode/submit 8.78, pipelines/compile_ms 5.31 |
| warm-up | 6.46 | 33.3 | render/thread/ms 46.31, render/busy/ms 45.16, main/busy/ms 38.14, stage/main/update 17.93, stage/render/prepare_assets 17.02, stage/main/post_update 11.88, stage/render/render 10.38, encode//sum 9.11, render/graph/ms 8.66, stage/render/prepare_meshes 5.86, stage/render/graph_submit 5.50, stream/drive/ms 3.56 |
| warm-up | 6.49 | 32.7 | render/thread/ms 55.52, render/busy/ms 53.98, main/busy/ms 43.90, stage/main/update 20.86, stage/render/prepare_assets 17.51, stage/render/render 15.44, stage/main/post_update 14.46, render/graph/ms 12.93, encode//sum 10.82, stage/render/graph_submit 8.41, stage/render/prepare_meshes 7.00, encode/submit 5.31 |
| warm-up | 6.54 | 30.6 | pipelines/compile_ms 142, pipelines/front_ms 139, render/thread/ms 39.12, render/busy/ms 38.23, main/busy/ms 31.20, stage/main/update 15.96, stage/render/prepare_assets 15.04, stage/render/render 10.02, encode//sum 9.25, render/graph/ms 8.64, stage/main/post_update 8.20, stage/render/graph_submit 5.19 |
| warm-up | 6.69 | 40.8 | main/busy/ms 49.46, render/thread/ms 36.70, render/busy/ms 35.38, stage/main/update 33.63, stream/drive/ms 16.84, stage/render/render 12.21, stage/render/prepare_assets 11.00, render/graph/ms 10.40, stage/main/post_update 10.35, encode//sum 9.63, stage/render/graph_submit 6.55, stage/render/prepare_meshes 4.82 |
| warm-up | 6.73 | 40.5 | pipelines/compile_ms 156, pipelines/front_ms 149, render/thread/ms 56.74, render/busy/ms 55.44, main/busy/ms 51.97, stage/main/update 30.45, stage/render/prepare_assets 20.70, stage/render/render 17.79, render/graph/ms 15.61, stage/main/post_update 14.44, encode//sum 12.63, stage/render/graph_submit 9.91 |
| measure | 1.69 | 27.2 | render/thread/ms 33.81, render/busy/ms 32.91, main/busy/ms 22.89, encode//sum 17.38, stage/render/render 15.29, render/graph/ms 13.76, stage/main/update 9.76, stage/main/post_update 8.49, stage/render/graph_submit 8.23, stage/render/prepare_assets 6.51, stage/render/camera_driver 5.38, encode/submit 5.11 |
| measure | 18.28 | 28.1 | main/busy/ms 35.38, stage/main/update 22.13, render/thread/ms 22.02, render/busy/ms 20.99, stage/render/render 11.72, render/graph/ms 10.35, stage/main/post_update 8.96, encode//sum 8.93, stage/render/graph_submit 7.54, encode/submit 5.20, stage/render/prepare_assets 3.85, stage/render/camera_driver 2.66 |
| measure | 59.18 | 32.0 | render/thread/ms 37.38, render/busy/ms 36.40, main/busy/ms 23.51, stage/render/prepare_assets 17.45, stage/main/post_update 11.12, stage/render/render 10.56, encode//sum 9.99, render/graph/ms 9.21, stage/render/graph_submit 6.11, stage/main/update 5.05, stage/main/fixed_main_loop 3.38, encode/finish_wall 3.07 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=off --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=stock-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1197 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **81%** (min 59%) · 2619 MHz (min 2610) · memory 8101 MHz · 81.9 W · 79 °C (max 85) · VRAM 5577 MiB · throttled 100% of samples

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
