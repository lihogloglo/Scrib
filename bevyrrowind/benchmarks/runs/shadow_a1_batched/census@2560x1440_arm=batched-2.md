# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.batched_pass=on (command line: --set); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1668 frames over 11.4 s · **median 6.61 ms (151 fps)** · mean 6.85 · p95 8.46 · p99 11.91 (1% low 84 fps) · worst 15.23 ms · 36 spikes over 1.5× median · 10.5 stutters/min (work 0.0/min)
- warm-up: 120 frames over 5.7 s, worst 2420.4 ms
- hitches (> 25.0 ms): **0** measured, 26 in warm-up
- timed GPU passes: 5.34 ms of a 6.85 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 151.4, target 144
- pass worst frame 15.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.050 | 2.218 | 2.639 | 0.92 | 0.172 |
| main_opaque_pass_3d | 1.858 | 2.010 | 3.072 | 0.92 | 0.042 |
| bloom | 0.333 | 0.360 | 0.873 | 0.92 | 0.066 |
| ssao | 0.191 | 0.207 | 0.205 | 0.92 | 0.007 |
| batched_point_shadows | 0.168 | 0.266 | 0.339 | 0.63 | 0.107 |
| upscaling | 0.151 | 0.164 | 0.199 | 0.92 | 0.005 |
| tlas_build | 0.112 | 0.121 | 0.119 | 0.92 | 0.076 |
| tonemapping | 0.104 | 0.113 | 0.123 | 0.92 | 0.004 |
| gpu_instance_materialization | 0.094 | 0.102 | 0.099 | 0.92 | 0.004 |
| gi/probe_update | 0.092 | 0.099 | 0.099 | 0.92 | 0.002 |
| early prepass | 0.077 | 0.084 | 0.091 | 0.92 | 0.052 |
| clustering | 0.074 | 0.080 | 0.082 | 0.92 | 0.020 |
| uniform_allocation | 0.038 | 0.041 | 0.045 | 0.92 | 0.010 |
| batched_point_shadows/uniform_allocation | 0.032 | 0.050 | 0.068 | 0.63 | 0.010 |
| bin_unpacking | 0.032 | 0.034 | 0.040 | 0.92 | 0.008 |
| batched_point_shadows/early_mesh_preprocessing | 0.023 | 0.036 | 0.045 | 0.63 | 0.005 |
| batched_point_shadows/bin_unpacking | 0.022 | 0.034 | 0.043 | 0.63 | 0.007 |
| range_unpacking | 0.018 | 0.020 | 0.020 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.018 | 0.020 | 0.026 | 0.92 | 0.003 |
| main_transparent_pass_3d | 0.008 | 0.009 | 0.011 | 0.90 | 0.024 |
| setup_tlas_instances | 0.007 | 0.007 | 0.008 | 0.92 | 0.005 |
| batched_point_shadows/range_unpacking | 0.005 | 0.007 | 0.007 | 0.63 | 0.002 |
| sparse buffer updates | 0.004 | 0.006 | 0.007 | 0.59 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.92 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 11.165 | 0.000 | 2221.89 | 18623.7 |
| pipelines/compile_ms | 8.956 | 0.000 | 6695.98 | 14938.5 |
| pipelines/driver_ms | 6.302 | 0.000 | 6381.15 | 10511.7 |
| render/thread/ms | 6.010 | 7.556 | 14.02 | 10024.1 |
| render/busy/ms | 5.373 | 6.916 | 13.34 | 8962.7 |
| main/busy/ms | 5.246 | 6.234 | 9.50 | 8750.0 |
| stage/render/render | 3.583 | 4.763 | 7.57 | 5977.1 |
| render/graph/ms | 3.006 | 4.093 | 6.63 | 5013.2 |
| stage/main/post_update | 2.735 | 3.383 | 5.59 | 4562.3 |
| pipelines/front_ms | 2.654 | 0.000 | 328.95 | 4426.9 |
| encode//sum | 2.110 | 2.827 | 4.89 | 3520.2 |
| stage/render/graph_submit | 2.027 | 2.944 | 5.01 | 3380.7 |
| stage/main/update | 1.319 | 1.708 | 3.04 | 2200.1 |
| encode/submit | 1.295 | 1.874 | 3.25 | 2160.8 |
| stage/render/camera_driver | 0.841 | 1.097 | 1.78 | 1402.1 |
| encode/finish_wall | 0.791 | 1.364 | 2.53 | 1319.2 |
| encode/pass:shadow_point_light/sum | 0.478 | 1.173 | 2.15 | 796.6 |
| encode/pass:early prepass/sum | 0.444 | 0.604 | 0.94 | 740.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.423 | 0.597 | 0.89 | 706.2 |
| stage/render/prepare_resources | 0.407 | 0.532 | 0.86 | 679.2 |
| render/present/ms | 0.407 | 0.903 | 1.38 | 678.4 |
| stage/render/prepare_write_phase_buffers | 0.371 | 0.615 | 1.16 | 619.1 |
| stage/main/pre_update | 0.363 | 0.481 | 0.83 | 604.8 |
| stage/render/prepare_views | 0.279 | 0.913 | 1.88 | 464.6 |
| stage/render/prepare_bind_groups | 0.263 | 0.425 | 1.38 | 438.1 |
| stage/main/first | 0.253 | 0.373 | 1.68 | 422.1 |
| stage/main/fixed_main_loop | 0.245 | 0.638 | 0.99 | 409.2 |
| bigspace/total/ms | 0.237 | 0.512 | 1.08 | 394.9 |
| render/acquire/ms | 0.230 | 0.873 | 1.84 | 383.1 |
| stage/render/prepare_assets | 0.220 | 0.316 | 0.79 | 366.8 |
| stage/render/prepare_flush | 0.189 | 0.338 | 2.80 | 314.9 |
| stage/render/prepare_meshes | 0.175 | 0.251 | 0.53 | 291.5 |
| stage/render/extract_commands | 0.164 | 0.212 | 0.48 | 272.7 |
| stage/main/last | 0.156 | 0.208 | 0.44 | 260.1 |
| stage/render/create_views | 0.147 | 0.211 | 0.47 | 244.9 |
| stream/drive/ms | 0.126 | 0.194 | 0.44 | 209.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.121 | 0.186 | 0.68 | 201.7 |
| bigspace/low_precision/ms | 0.102 | 0.257 | 0.99 | 170.9 |
| stream/tick/ms | 0.097 | 0.159 | 0.39 | 161.7 |
| stage/render/queue | 0.086 | 0.138 | 0.66 | 143.0 |
| stage/render/prepare_batch_phases | 0.084 | 0.141 | 0.36 | 139.7 |
| controller/feed/ms | 0.084 | 0.113 | 0.55 | 139.7 |
| physics/step/ms | 0.077 | 0.259 | 0.54 | 127.8 |
| stage/render/specialize | 0.069 | 0.110 | 0.87 | 115.6 |
| anim/evaluate/ms | 0.069 | 0.145 | 0.88 | 115.0 |
| bigspace/high_precision/ms | 0.053 | 0.163 | 0.75 | 88.8 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.075 | 0.16 | 77.6 |
| encode/point_shadow_cache_copies/sum | 0.039 | 0.081 | 0.55 | 65.5 |
| anim/evaluate_cpu/ms | 0.036 | 0.058 | 0.27 | 60.5 |
| encode/resolve_diagnostics/sum | 0.035 | 0.056 | 0.33 | 57.9 |
| controller/write/ms | 0.027 | 0.039 | 0.10 | 44.2 |
| weather/map/ms | 0.026 | 0.037 | 0.16 | 43.9 |
| stage/render/cleanup | 0.023 | 0.037 | 0.27 | 38.0 |
| physics/insert/ms | 0.020 | 0.185 | 0.72 | 33.2 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.023 | 0.06 | 21.8 |
| stage/render/phase_sort | 0.013 | 0.034 | 0.21 | 21.5 |
| encode/camera_driver_span_end/sum | 0.012 | 0.022 | 0.26 | 20.8 |
| weather/eval/ms | 0.012 | 0.017 | 0.14 | 20.5 |
| script/update/ms | 0.008 | 0.011 | 0.08 | 12.5 |
| anim/apply/ms | 0.005 | 0.011 | 0.07 | 8.4 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 8.2 |
| far/select/ms | 0.004 | 0.005 | 0.19 | 7.5 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.2 |
| controller/move/ms | 0.003 | 0.010 | 0.07 | 5.8 |
| controller/avoid/ms | 0.002 | 0.005 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.21 | 2.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.11 | 1.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.14 | 1.3 |
| pipelines/lock_ms | 0.001 | 0.000 | 0.36 | 1.2 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| terrain/select/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| terrain/update/ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 1 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 143 | 143 | 143 |
| anim/characters/count | count | 143 | 143 | 143 |
| anim/evaluated/count | count | 3 | 3 | 3 |
| controller/movers/count | count | 3 | 3 | 3 |
| crowd/c0/count | count | 3 | 3 | 3 |
| crowd/c0/joints/count | count | 153 | 153 | 153 |
| crowd/c0/meshes/count | count | 56 | 56 | 56 |
| crowd/c1/count | count | 3 | 3 | 3 |
| crowd/c1/joints/count | count | 102 | 102 | 102 |
| crowd/c1/meshes/count | count | 18 | 18 | 18 |
| crowd/c2/count | count | 137 | 137 | 137 |
| crowd/c2/joints/count | count | 2505 | 2505 | 2505 |
| crowd/c2/meshes/count | count | 725 | 725 | 725 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 2 | 2 | 0 |
| crowd/joints_extracted/count | count | 46 | 1357 | 0 |
| crowd/joints_uploaded/count | count | 94 | 94 | 0 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.42 | 1.31 | 0.49 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.06 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.26 | 0.01 |
| encode/finish_critical | ms | 0.58 | 2.15 | 0.49 |
| encode/pass:early prepass/max | ms | 0.44 | 0.94 | 0.42 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.42 | 0.89 | 0.36 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 0.68 | 0.07 |
| encode/pass:shadow_point_light/max | ms | 0.68 | 2.15 | 0.61 |
| encode/point_shadow_cache_copies/max | ms | 0.05 | 0.55 | 0.04 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.33 | 0.04 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.95 | 13.34 | 5.47 |
| gpu/timed/ms | ms | 5.87 | 8.17 | 5.47 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 41 | 48 | 48 |
| lamp_cache/copies | count | 5 | 13 | 0 |
| lamp_cache/deferred_faces | count | 0 | 12 | 0 |
| lamp_cache/lamps | count | 8 | 8 | 8 |
| lamp_cache/mover_faces | count | 5 | 12 | 0 |
| lamp_cache/movers | count | 200 | 269 | 56 |
| lamp_cache/queued | count | 0 | 46 | 0 |
| lamp_cache/ready | count | 8 | 8 | 8 |
| lamp_cache/render/copies | count | 5 | 13 | 0 |
| lamp_cache/render/drawn | count | 5 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 8 | 8 | 8 |
| lamp_cache/render/loaded | count | 5 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 0 | 0 |
| lamp_cache/render/views | count | 6 | 26 | 0 |
| lamp_cache/stock_lamps | count | 0 | 1 | 0 |
| lamp_cache/store_starved | count | 0 | 0 | 0 |
| lamp_cache/store_used | count | 19 | 24 | 4 |
| lamp_cache/views | count | 6 | 26 | 0 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1410 | 1410 | 1410 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 152.1 k | 152.1 k | 152.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 5575 | 5575 | 5575 |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 172.04 M | 172.04 M | 172.04 M |
| phases/alpha_mask3d/batch_sets | count | 4 | 4 | 3 |
| phases/alpha_mask3d/bins | count | 5 | 9 | 3 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 177 | 177 | 177 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 12 | 24 | 3 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 4 | 4 | 3 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 4 | 4 | 3 |
| phases/alpha_mask3d_prepass/bins | count | 5 | 9 | 3 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 177 | 177 | 177 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 12 | 24 | 3 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 4 | 4 | 3 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 9 | 12 | 6 |
| phases/opaque3d/bins | count | 153 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 261 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 153 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 261 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 24 | 61 | 7 |
| phases/shadow/bins | count | 321 | 828 | 120 |
| phases/shadow/instance_batches | count | 0 | 4 | 0 |
| phases/shadow/instance_draws | count | 0 | 354 | 0 |
| phases/shadow/instance_slots | count | 0 | 16.4 k | 0 |
| phases/shadow/items | count | 324 | 935 | 168 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 32 | 82 | 7 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 143 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 291 | 291 | 291 |
| pipelines/waiting | count | 0 | 28 | 0 |
| precache/draw_waiting | count | 0 | 15 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 0 | 14 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 67 | 0 |
| preprocess/bind_groups | count | 6 | 198 | 4 |
| preprocess/bind_groups_created | count | 2 | 194 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 6 | 4 |
| preprocess/bind_groups_reused | count | 71 | 131 | 35 |
| process/cpu_usage | % | 27.71 | 84.38 | 28.50 |
| process/mem_usage | GiB | 3.81 | 3.85 | 3.85 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 677 | 677 | 677 |
| scene/batch/masked/live | count | 677 | 677 | 677 |
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
| scene/batch/opaque/high_water | count | 4611 | 4611 | 4611 |
| scene/batch/opaque/live | count | 4611 | 4611 | 4611 |
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
| scene/cells/resident | count | 29 | 29 | 29 |
| scene/chains/high_water | count | 674 | 674 | 674 |
| scene/chains/live | count | 674 | 674 | 674 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 5288 | 5288 | 5288 |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 5288 | 5288 | 5288 |
| scene/slots/scene_bump | count | 5288 | 5288 | 5288 |
| scene/slots/scene_live | count | 5288 | 5288 | 5288 |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 1 | 23 | 0 |
| skin/blocks_skipped/count | count | 1 | 2 | 0 |
| skin/blocks_walked/count | count | 1 | 2 | 0 |
| skin/joint_bytes_written/bytes | bytes | 2944 | 7680 | 0 |
| skin/joints_written/count | count | 46 | 120 | 0 |
| skin/upload_writes/count | count | 1 | 2 | 0 |
| started_load_count |  | 2749 | 2749 | 2749 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 201 | 201 | 201 |
| statics/materials_bound | count | 201 | 201 | 201 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.08 | 0.08 | 0.08 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 167 | 167 | 167 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 6.62 M | 6.62 M | 6.62 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 30 | 30 | 30 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 36.32 | 93.62 | 32.60 |
| system/mem_usage | % | 52.00 | 52.10 | 52.05 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 35 | 90 | 17 |
| views/all/bins | count | 423 | 1358 | 58 |
| views/all/count | count | 8 | 28 | 1 |
| views/all/instance_batches | count | 8 | 12 | 8 |
| views/all/instance_draws | count | 3220 | 3574 | 3220 |
| views/all/instance_slots | count | 98.3 k | 114.7 k | 98.3 k |
| views/all/items | count | 675 | 1926 | 86 |
| views/all/pipelines | count | 39 | 109 | 15 |
| views/all/pipelines_distinct | count | 24 | 34 | 15 |
| views/all/work_items | count | 98.9 k | 116.3 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 12 | 61 | 0 |
| views/lamp_face/bins | count | 163 | 828 | 0 |
| views/lamp_face/count | count | 7 | 27 | 0 |
| views/lamp_face/culled | count | 0 | 4 | 0 |
| views/lamp_face/drawn | count | 5 | 18 | 0 |
| views/lamp_face/instance_batches | count | 0 | 4 | 0 |
| views/lamp_face/instance_draws | count | 0 | 354 | 0 |
| views/lamp_face/instance_slots | count | 0 | 16.4 k | 0 |
| views/lamp_face/items | count | 163 | 935 | 0 |
| views/lamp_face/pipelines | count | 18 | 82 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 107 | 16.9 k | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 316 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 542 | 1062 | 86 |
| views/main/pipelines | count | 21 | 27 | 15 |
| views/main/work_items | count | 98.9 k | 99.4 k | 98.4 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 6100 | 6100 | 6100 |
| world/point_lights | count | 156 | 156 | 156 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 8.75 | 30 | 14.6 k |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 8 | 8 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 19.64 | 20 | 32.8 k |
| encode/bind_walk/init_textures | count | 4242 | 12.6 k | 7.08 M |
| encode/bind_walk/init_textures_skipped | count | 6943 | 18.4 k | 11.58 M |
| encode/bind_walk/inits | count | 220 | 417 | 367.1 k |
| encode/bind_walk/inits_skipped | count | 10.57 | 28 | 17.6 k |
| encode/bind_walk/merge_textures | count | 2999 | 11.3 k | 5.00 M |
| encode/bind_walk/merge_textures_skipped | count | 119 | 7105 | 197.7 k |
| encode/bind_walk/merges | count | 67.21 | 126 | 112.1 k |
| encode/bind_walk/merges_skipped | count | 11.57 | 47 | 19.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1668 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1668 |
| encode/pass:early prepass/count | count | 1 | 1 | 1668 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1668 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1619 |
| encode/pass:shadow_point_light/count | count | 0.69 | 1 | 1148 |
| encode/point_shadow_cache_copies/count | count | 0.68 | 1 | 1140 |
| encode/queue/command_buffers | count | 29.98 | 31 | 50.0 k |
| encode/queue/submits | count | 2 | 2 | 3336 |
| encode/queue/write_buffer/bytes | bytes | 541.0 k | 661.8 k | 902.39 M |
| encode/queue/write_buffer/calls | count | 147 | 312 | 244.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 87.7 k | 88.2 k | 146.35 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 10.0 k |
| encode/queue/write_texture/bytes | bytes | 629 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1668 |
| physics/inserts | count | 0.09 | 13 | 145 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.41 | 1 | 686 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.02 | 8 | 27 |
| precache/hit | count | 0.00 | 4 | 6 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 26, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.42 | 2420.4 | main/busy/ms 2393, stage/main/post_update 2289, stage/main/update 93.97, surface/advance_cpu_ms 66.74, stage/main/pre_update 6.12, stage/main/last 1.11, stage/main/first 0.54, stage/main/fixed_main_loop 0.37, stream/drive/ms 0.34, weather/map/ms 0.16, bigspace/total/ms 0.09, stream/tick/ms 0.06 |
| warm-up | 3.14 | 723.6 | main/busy/ms 2502, stage/main/post_update 2294, render/thread/ms 683, render/busy/ms 567, stage/render/prepare_assets 525, stage/main/update 192, stage/render/prepare_views 114, render/acquire/ms 113, surface/advance_cpu_ms 66.74, stage/render/create_views 29.42, stage/render/render 8.21, stage/main/pre_update 7.35 |
| warm-up | 3.19 | 48.9 | render/thread/ms 716, render/busy/ms 600, stage/render/prepare_assets 553, main/busy/ms 123, stage/render/prepare_views 114, render/acquire/ms 113, stage/main/update 103, stage/render/create_views 29.65, stage/render/render 11.96, stage/main/post_update 9.44, render/graph/ms 8.50, stage/main/fixed_main_loop 7.08 |
| warm-up | 3.26 | 53.4 | pipelines/wait_ms 1164, pipelines/compile_ms 402, pipelines/front_ms 375, main/busy/ms 73.21, stage/main/post_update 35.01, render/thread/ms 34.82, render/busy/ms 31.72, pipelines/driver_ms 27.51, stage/main/update 17.67, stage/main/fixed_main_loop 12.89, stage/render/render 11.87, stage/render/prepare_resources 9.32 |
| warm-up | 3.29 | 33.0 | pipelines/wait_ms 1366, pipelines/compile_ms 577, pipelines/front_ms 541, main/busy/ms 64.91, render/thread/ms 41.62, render/busy/ms 39.52, pipelines/driver_ms 36.84, stage/main/post_update 26.36, stage/main/fixed_main_loop 16.99, stage/main/update 15.26, stage/render/render 11.35, stage/render/prepare_assets 11.05 |
| warm-up | 3.32 | 26.5 | pipelines/compile_ms 459, pipelines/front_ms 437, pipelines/wait_ms 214, main/busy/ms 38.83, render/thread/ms 35.54, render/busy/ms 33.76, pipelines/driver_ms 21.85, stage/render/prepare_assets 21.72, stage/main/post_update 14.20, stage/main/pre_update 11.51, stage/render/render 8.69, render/graph/ms 6.84 |
| warm-up | 3.35 | 29.2 | pipelines/compile_ms 596, pipelines/front_ms 573, pipelines/wait_ms 52.01, render/thread/ms 40.15, render/busy/ms 38.43, main/busy/ms 31.45, stage/render/prepare_assets 26.12, pipelines/driver_ms 22.62, stage/main/pre_update 12.27, stage/render/render 8.74, render/graph/ms 6.75, stage/main/update 6.31 |
| warm-up | 3.46 | 90.5 | pipelines/compile_ms 878, pipelines/front_ms 849, render/thread/ms 70.11, main/busy/ms 69.80, render/busy/ms 67.87, stage/main/update 53.34, stage/render/prepare_assets 43.50, pipelines/driver_ms 28.77, stage/render/render 17.23, render/graph/ms 14.92, pipelines/wait_ms 12.34, stage/main/post_update 11.39 |
| warm-up | 3.53 | 60.0 | pipelines/compile_ms 800, pipelines/front_ms 762, render/thread/ms 68.50, render/busy/ms 66.89, stage/render/prepare_assets 57.08, main/busy/ms 42.13, pipelines/driver_ms 37.88, stage/main/update 22.20, stream/drive/ms 16.38, stage/main/first 9.68, stage/render/render 6.68, stage/main/post_update 6.09 |
| warm-up | 3.60 | 40.9 | pipelines/compile_ms 582, pipelines/front_ms 566, render/thread/ms 50.15, render/busy/ms 43.81, stage/render/prepare_assets 31.97, main/busy/ms 28.65, pipelines/driver_ms 16.44, stage/render/render 13.36, stage/main/update 12.88, stage/main/post_update 8.80, render/graph/ms 7.14, render/present/ms 5.79 |
| warm-up | 3.64 | 40.4 | pipelines/compile_ms 617, pipelines/front_ms 597, render/thread/ms 67.31, render/busy/ms 60.99, main/busy/ms 45.91, stage/render/prepare_assets 25.48, stage/render/prepare_resources 22.26, stage/main/update 21.97, pipelines/driver_ms 19.28, stage/render/render 14.96, stage/main/post_update 14.90, render/graph/ms 8.71 |
| warm-up | 3.73 | 87.4 | pipelines/compile_ms 1557, pipelines/front_ms 1500, main/busy/ms 181, stage/main/update 156, render/thread/ms 105, render/busy/ms 103, pipelines/wait_ms 98.90, stage/render/prepare_assets 62.59, pipelines/driver_ms 56.82, stream/drive/ms 33.01, stage/render/prepare_resources 22.21, stage/render/render 15.31 |
| warm-up | 3.89 | 161.8 | pipelines/compile_ms 2567, pipelines/front_ms 2474, pipelines/wait_ms 179, main/busy/ms 158, stage/main/update 144, render/thread/ms 103, render/busy/ms 99.01, pipelines/driver_ms 92.83, stage/render/prepare_assets 82.89, stream/drive/ms 32.92, pipelines/lock_ms 16.92, stage/render/render 15.59 |
| warm-up | 3.96 | 68.1 | pipelines/compile_ms 1195, pipelines/front_ms 1148, pipelines/wait_ms 85.53, render/thread/ms 82.01, render/busy/ms 76.66, stage/render/prepare_assets 61.68, pipelines/driver_ms 46.05, main/busy/ms 31.17, stage/main/update 18.10, stage/render/render 16.60, stream/drive/ms 12.24, render/graph/ms 11.25 |
| warm-up | 3.98 | 25.8 | pipelines/compile_ms 140, pipelines/front_ms 132, render/thread/ms 66.19, render/busy/ms 63.42, stage/render/prepare_assets 46.56, main/busy/ms 38.31, stage/main/update 19.23, stage/render/render 14.42, stream/drive/ms 12.15, render/graph/ms 11.55, stage/render/graph_submit 9.96, stage/main/post_update 9.81 |
| warm-up | 4.01 | 31.7 | pipelines/compile_ms 970, pipelines/front_ms 938, render/thread/ms 40.27, render/busy/ms 38.88, pipelines/driver_ms 32.11, stage/render/prepare_assets 26.80, main/busy/ms 22.60, stage/render/render 8.07, stage/main/post_update 8.03, stage/main/update 7.39, render/graph/ms 6.41, pipelines/wait_ms 5.43 |
| warm-up | 4.34 | 28.2 | pipelines/compile_ms 215, pipelines/front_ms 208, render/thread/ms 30.97, render/busy/ms 29.53, main/busy/ms 29.22, stage/main/update 13.27, stage/render/render 12.60, render/graph/ms 10.50, stage/main/post_update 9.75, pipelines/driver_ms 7.74, stage/render/prepare_assets 7.08, stage/render/graph_submit 6.49 |
| warm-up | 4.50 | 137.7 | main/busy/ms 122, pipelines/compile_ms 111, pipelines/front_ms 107, stage/main/update 95.11, render/thread/ms 70.61, render/busy/ms 68.33, pipelines/wait_ms 55.12, stream/drive/ms 42.33, stage/render/prepare_assets 34.72, stage/main/post_update 18.19, stage/render/render 14.42, render/graph/ms 11.15 |
| warm-up | 4.55 | 52.0 | pipelines/compile_ms 111, pipelines/front_ms 107, render/thread/ms 90.82, render/busy/ms 88.49, main/busy/ms 79.33, pipelines/wait_ms 55.12, stage/render/prepare_assets 51.18, stage/main/update 46.22, stage/main/post_update 21.19, stage/render/render 16.38, stream/drive/ms 14.13, render/graph/ms 13.32 |
| warm-up | 4.66 | 110.6 | render/thread/ms 113, render/busy/ms 111, main/busy/ms 66.02, stage/render/prepare_assets 50.46, stage/main/update 35.99, stage/render/create_views 35.46, stage/main/post_update 16.91, stream/drive/ms 14.95, stage/render/render 12.06, render/graph/ms 10.03, stage/render/graph_submit 6.46, encode//sum 5.98 |
| warm-up | 4.72 | 55.3 | render/thread/ms 107, render/busy/ms 106, stage/render/prepare_assets 55.75, main/busy/ms 49.02, stage/render/create_views 34.62, stage/main/update 25.70, stage/main/post_update 12.91, stage/render/render 7.81, render/graph/ms 6.34, stream/drive/ms 5.94, encode//sum 5.20, stage/render/graph_submit 3.53 |
| warm-up | 4.75 | 30.5 | render/thread/ms 63.16, render/busy/ms 61.33, main/busy/ms 57.01, stage/main/update 37.35, stage/render/prepare_assets 27.94, stream/drive/ms 24.69, stage/render/render 12.30, stage/render/create_views 10.56, encode//sum 10.43, render/graph/ms 9.99, stage/main/post_update 9.92, stage/render/graph_submit 6.37 |
| warm-up | 4.78 | 27.2 | main/busy/ms 50.72, render/thread/ms 46.04, render/busy/ms 44.01, stage/main/update 33.82, stream/drive/ms 24.53, stage/render/render 12.71, stage/render/create_views 11.00, encode//sum 10.77, stage/main/post_update 10.39, render/graph/ms 10.15, stage/render/prepare_assets 8.61, stage/render/graph_submit 6.62 |
| warm-up | 4.98 | 40.0 | main/busy/ms 56.93, render/thread/ms 40.09, render/busy/ms 38.04, stage/main/update 37.51, stage/render/render 14.03, stage/main/post_update 13.49, render/graph/ms 11.58, stage/render/prepare_assets 10.36, stage/render/graph_submit 7.94, encode//sum 7.70, encode/submit 5.40, stage/render/prepare_meshes 3.54 |
| warm-up | 5.04 | 53.6 | render/thread/ms 57.29, render/busy/ms 55.24, main/busy/ms 52.76, stage/main/update 33.05, stage/render/prepare_assets 28.89, stage/render/render 13.88, render/graph/ms 11.22, stage/main/post_update 10.72, stage/render/graph_submit 7.46, encode//sum 7.03, encode/submit 5.41, stage/main/pre_update 4.20 |
| warm-up | 5.08 | 44.1 | render/thread/ms 58.05, render/busy/ms 56.58, stage/render/prepare_assets 38.01, main/busy/ms 32.05, stage/main/update 16.25, stage/render/render 10.17, render/graph/ms 8.29, stage/main/post_update 7.99, encode//sum 6.28, stage/render/graph_submit 5.51, stream/drive/ms 3.62, encode/submit 3.59 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=batched-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 9%
- during the run: other processes 9% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (253 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **94%** (min 47%) · 2626 MHz (min 2625) · memory 8101 MHz · 97.2 W · 73 °C (max 78) · VRAM 4905 MiB · throttled 100% of samples

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
- `shadow.lamp.batched_pass=on (command line: --set)`
- `shadow.lamp.cache.enabled=on (command line: --set)`
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
