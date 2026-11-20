# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1569 frames over 11.4 s · **median 7.05 ms (142 fps)** · mean 7.29 · p95 9.15 · p99 9.84 (1% low 102 fps) · worst 24.88 ms · 4 spikes over 1.5× median · 5.2 stutters/min (work 5.2/min)
- warm-up: 120 frames over 5.6 s, worst 2450.4 ms
- hitches (> 25.0 ms): **0** measured, 33 in warm-up
- timed GPU passes: 5.59 ms of a 7.29 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median fps 141.9, target 144
- pass worst frame 24.88 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.017 | 2.144 | 2.430 | 0.94 | 0.165 |
| main_opaque_pass_3d | 1.848 | 1.965 | 2.815 | 0.94 | 0.043 |
| auxiliary_views | 0.481 | 0.511 | 1.332 | 0.94 | 0.437 |
| bloom | 0.323 | 0.343 | 0.662 | 0.94 | 0.064 |
| ssao | 0.196 | 0.209 | 0.405 | 0.94 | 0.007 |
| upscaling | 0.153 | 0.163 | 0.203 | 0.94 | 0.004 |
| tlas_build | 0.109 | 0.116 | 0.119 | 0.94 | 0.072 |
| tonemapping | 0.101 | 0.107 | 0.123 | 0.94 | 0.004 |
| gi/probe_update | 0.093 | 0.099 | 0.101 | 0.94 | 0.002 |
| gpu_instance_materialization | 0.092 | 0.098 | 0.100 | 0.94 | 0.004 |
| early prepass | 0.076 | 0.080 | 0.086 | 0.94 | 0.048 |
| clustering | 0.071 | 0.075 | 0.077 | 0.94 | 0.018 |
| uniform_allocation | 0.034 | 0.036 | 0.045 | 0.94 | 0.010 |
| bin_unpacking | 0.030 | 0.032 | 0.040 | 0.94 | 0.008 |
| range_unpacking | 0.018 | 0.020 | 0.020 | 0.94 | 0.003 |
| early_mesh_preprocessing | 0.018 | 0.019 | 0.026 | 0.94 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.92 | 0.023 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.94 | 0.005 |
| sparse buffer updates | 0.004 | 0.006 | 0.007 | 0.57 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.94 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.463 | 8.298 | 23.73 | 10140.0 |
| render/busy/ms | 5.888 | 7.737 | 11.29 | 9238.8 |
| main/busy/ms | 5.118 | 5.867 | 8.81 | 8029.7 |
| stage/render/render | 4.139 | 5.714 | 21.13 | 6494.3 |
| render/graph/ms | 3.554 | 5.084 | 6.75 | 5576.5 |
| encode//sum | 3.009 | 4.535 | 6.16 | 4720.8 |
| stage/main/post_update | 2.674 | 3.201 | 3.78 | 4194.7 |
| stage/render/graph_submit | 2.269 | 3.232 | 4.91 | 3559.4 |
| encode/submit | 1.607 | 2.354 | 3.96 | 2520.7 |
| stage/main/update | 1.264 | 1.506 | 1.99 | 1983.9 |
| stage/render/camera_driver | 1.152 | 1.756 | 2.20 | 1806.8 |
| encode/finish_wall | 0.721 | 1.004 | 1.41 | 1131.7 |
| encode/pass:early prepass/sum | 0.452 | 0.624 | 1.01 | 709.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.422 | 0.628 | 0.89 | 661.8 |
| render/present/ms | 0.421 | 0.840 | 15.17 | 660.6 |
| stage/render/prepare_resources | 0.410 | 0.517 | 0.72 | 642.9 |
| stage/main/pre_update | 0.362 | 0.461 | 0.67 | 568.5 |
| stage/render/prepare_write_phase_buffers | 0.354 | 0.567 | 1.16 | 555.5 |
| encode/pass:shadow_point_light/sum | 0.286 | 0.648 | 2.70 | 449.3 |
| stage/render/prepare_bind_groups | 0.253 | 0.384 | 1.30 | 396.4 |
| stage/main/first | 0.244 | 0.344 | 1.54 | 382.4 |
| stage/render/prepare_assets | 0.238 | 0.330 | 0.52 | 373.5 |
| stage/main/fixed_main_loop | 0.236 | 0.567 | 0.96 | 370.3 |
| bigspace/total/ms | 0.213 | 0.410 | 0.90 | 334.3 |
| stage/render/prepare_views | 0.201 | 0.723 | 1.41 | 315.6 |
| stage/render/prepare_flush | 0.181 | 0.299 | 0.54 | 283.3 |
| stage/render/prepare_meshes | 0.180 | 0.251 | 0.42 | 282.8 |
| stage/render/extract_commands | 0.168 | 0.207 | 0.44 | 264.2 |
| stage/main/last | 0.166 | 0.240 | 3.31 | 259.8 |
| render/acquire/ms | 0.153 | 0.680 | 1.37 | 240.6 |
| stage/render/create_views | 0.137 | 0.186 | 0.34 | 214.9 |
| stream/drive/ms | 0.123 | 0.178 | 0.34 | 193.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.114 | 0.164 | 0.51 | 179.0 |
| stream/tick/ms | 0.095 | 0.144 | 0.31 | 148.4 |
| bigspace/low_precision/ms | 0.089 | 0.172 | 0.76 | 140.2 |
| stage/render/prepare_batch_phases | 0.084 | 0.142 | 0.26 | 131.4 |
| controller/feed/ms | 0.083 | 0.105 | 0.34 | 130.4 |
| stage/render/queue | 0.080 | 0.123 | 0.42 | 125.0 |
| physics/step/ms | 0.064 | 0.222 | 0.53 | 100.7 |
| stage/render/specialize | 0.062 | 0.088 | 0.51 | 97.2 |
| anim/evaluate/ms | 0.060 | 0.107 | 0.66 | 94.5 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.071 | 0.17 | 73.0 |
| bigspace/high_precision/ms | 0.045 | 0.099 | 0.59 | 70.9 |
| anim/evaluate_cpu/ms | 0.035 | 0.051 | 0.27 | 55.6 |
| encode/point_shadow_cache_copies/sum | 0.035 | 0.073 | 0.42 | 55.0 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.31 | 50.7 |
| controller/write/ms | 0.026 | 0.038 | 0.28 | 40.8 |
| weather/map/ms | 0.025 | 0.036 | 0.20 | 39.8 |
| stage/render/cleanup | 0.024 | 0.037 | 0.21 | 36.9 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.020 | 0.18 | 19.2 |
| stage/render/phase_sort | 0.012 | 0.029 | 0.18 | 18.4 |
| weather/eval/ms | 0.012 | 0.016 | 0.16 | 18.3 |
| encode/camera_driver_span_end/sum | 0.011 | 0.018 | 0.28 | 16.9 |
| physics/insert/ms | 0.009 | 0.000 | 0.60 | 14.2 |
| script/update/ms | 0.008 | 0.012 | 0.10 | 13.0 |
| anim/apply/ms | 0.005 | 0.010 | 0.03 | 7.7 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 7.5 |
| far/select/ms | 0.004 | 0.005 | 0.15 | 6.3 |
| controller/move/ms | 0.004 | 0.010 | 0.03 | 5.7 |
| script/ui/ms | 0.003 | 0.004 | 0.11 | 5.1 |
| controller/avoid/ms | 0.002 | 0.005 | 0.01 | 2.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| crowd/joints_extracted/count | count | 0 | 1357 | 0 |
| crowd/joints_uploaded/count | count | 94 | 94 | 0 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.43 | 1.21 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.18 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.02 |
| encode/finish_critical | ms | 0.53 | 1.21 | 0.44 |
| encode/pass:early prepass/max | ms | 0.45 | 1.01 | 0.36 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.41 | 0.89 | 0.31 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.51 | 0.09 |
| encode/pass:shadow_point_light/max | ms | 0.08 | 0.68 | 0.26 |
| encode/point_shadow_cache_copies/max | ms | 0.05 | 0.42 | 0.04 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.31 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.86 | 11.29 | 4.91 |
| gpu/timed/ms | ms | 5.70 | 8.44 | 4.77 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 42 | 48 | 48 |
| lamp_cache/copies | count | 5 | 13 | 0 |
| lamp_cache/deferred_faces | count | 0 | 13 | 0 |
| lamp_cache/lamps | count | 8 | 8 | 8 |
| lamp_cache/mover_faces | count | 5 | 12 | 0 |
| lamp_cache/movers | count | 200 | 269 | 56 |
| lamp_cache/queued | count | 0 | 6 | 0 |
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
| phases/alpha_mask3d/items | count | 11 | 24 | 3 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 4 | 4 | 3 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 4 | 4 | 3 |
| phases/alpha_mask3d_prepass/bins | count | 5 | 9 | 3 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 177 | 177 | 177 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 11 | 24 | 3 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 4 | 4 | 3 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 9 | 12 | 6 |
| phases/opaque3d/bins | count | 144 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 251 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 6 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 144 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 251 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 63 | 7 |
| phases/shadow/bins | count | 308 | 828 | 120 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 308 | 935 | 168 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 32 | 82 | 7 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 94 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 291 | 291 | 291 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 6 | 202 | 4 |
| preprocess/bind_groups_created | count | 2 | 198 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 68 | 131 | 35 |
| process/cpu_usage | % | 23.92 | 34.13 | 21.53 |
| process/mem_usage | GiB | 3.73 | 3.85 | 3.85 |
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
| skin/blocks_changed/count | count | 0 | 23 | 0 |
| skin/blocks_skipped/count | count | 1 | 2 | 0 |
| skin/blocks_walked/count | count | 0 | 2 | 0 |
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
| stream/latency/s | s | 0.09 | 0.09 | 0.09 |
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
| system/cpu_usage | % | 30.48 | 39.28 | 32.98 |
| system/mem_usage | % | 52.64 | 53.04 | 53.04 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 35 | 92 | 17 |
| views/all/bins | count | 387 | 1358 | 58 |
| views/all/count | count | 7 | 28 | 1 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 613 | 1891 | 86 |
| views/all/pipelines | count | 36 | 109 | 16 |
| views/all/pipelines_distinct | count | 24 | 34 | 16 |
| views/all/work_items | count | 98.9 k | 100.0 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 12 | 63 | 0 |
| views/lamp_face/bins | count | 127 | 828 | 0 |
| views/lamp_face/count | count | 6 | 27 | 0 |
| views/lamp_face/culled | count | 0 | 4 | 0 |
| views/lamp_face/drawn | count | 5 | 18 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 127 | 935 | 0 |
| views/lamp_face/pipelines | count | 15 | 82 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 107 | 781 | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 298 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 526 | 1062 | 86 |
| views/main/pipelines | count | 21 | 27 | 16 |
| views/main/work_items | count | 98.8 k | 99.4 k | 98.4 k |
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
| anim/bones_written/count | count | 8.58 | 30 | 13.5 k |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 8 | 8 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 0 | 0 |
| encode//count | count | 41.22 | 92 | 64.7 k |
| encode/bind_walk/init_textures | count | 4203 | 12.6 k | 6.59 M |
| encode/bind_walk/init_textures_skipped | count | 6842 | 18.4 k | 10.73 M |
| encode/bind_walk/inits | count | 215 | 425 | 337.5 k |
| encode/bind_walk/inits_skipped | count | 10.31 | 28 | 16.2 k |
| encode/bind_walk/merge_textures | count | 2960 | 11.3 k | 4.64 M |
| encode/bind_walk/merge_textures_skipped | count | 76.86 | 7105 | 120.6 k |
| encode/bind_walk/merges | count | 65.57 | 131 | 102.9 k |
| encode/bind_walk/merges_skipped | count | 10.98 | 47 | 17.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1569 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1569 |
| encode/pass:early prepass/count | count | 1 | 1 | 1569 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1569 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1519 |
| encode/pass:shadow_point_light/count | count | 5.40 | 18 | 8475 |
| encode/point_shadow_cache_copies/count | count | 0.66 | 1 | 1037 |
| encode/queue/command_buffers | count | 56.25 | 120 | 88.3 k |
| encode/queue/submits | count | 2 | 2 | 3138 |
| encode/queue/write_buffer/bytes | bytes | 541.5 k | 663.6 k | 849.63 M |
| encode/queue/write_buffer/calls | count | 143 | 316 | 224.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 87.9 k | 88.2 k | 137.96 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 9414 |
| encode/queue/write_texture/bytes | bytes | 668 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1569 |
| physics/inserts | count | 0.06 | 12 | 95 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.44 | 2 | 687 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0.00 | 4 | 4 |
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

## Hitches (> 25.0 ms; slowest 33, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2450.4 | main/busy/ms 2423, stage/main/post_update 2322, stage/main/update 92.11, surface/advance_cpu_ms 65.76, stage/main/pre_update 6.00, stage/main/last 1.00, stage/main/first 0.52, stream/drive/ms 0.37, stage/main/fixed_main_loop 0.37, weather/map/ms 0.15, bigspace/total/ms 0.07, stream/tick/ms 0.06 |
| warm-up | 3.09 | 635.5 | main/busy/ms 2535, stage/main/post_update 2326, render/thread/ms 603, render/busy/ms 521, stage/render/prepare_assets 493, stage/main/update 193, stage/render/prepare_views 80.47, render/acquire/ms 80.25, surface/advance_cpu_ms 65.76, stage/render/create_views 13.29, stage/render/render 8.56, stage/main/pre_update 7.19 |
| warm-up | 3.13 | 47.2 | render/thread/ms 635, render/busy/ms 552, stage/render/prepare_assets 519, main/busy/ms 126, stage/main/update 107, stage/render/prepare_views 80.76, render/acquire/ms 80.46, stage/render/create_views 13.50, stage/render/render 13.03, render/graph/ms 9.49, stage/main/post_update 7.71, stage/main/fixed_main_loop 7.59 |
| warm-up | 3.17 | 38.6 | pipelines/compile_ms 126, pipelines/front_ms 116, pipelines/wait_ms 109, main/busy/ms 73.29, render/thread/ms 56.34, stage/main/post_update 55.99, render/busy/ms 36.95, stage/render/prepare_assets 27.48, stage/render/render 24.45, render/present/ms 18.88, pipelines/driver_ms 9.90, stage/main/update 7.87 |
| warm-up | 3.22 | 43.6 | pipelines/wait_ms 1598, pipelines/compile_ms 534, pipelines/front_ms 493, main/busy/ms 70.30, stage/main/post_update 57.08, render/thread/ms 43.78, pipelines/driver_ms 40.93, stage/render/render 33.08, render/busy/ms 22.55, render/present/ms 20.59, render/graph/ms 12.17, stage/main/update 6.68 |
| warm-up | 3.25 | 33.1 | pipelines/wait_ms 1491, pipelines/compile_ms 638, pipelines/front_ms 594, render/thread/ms 50.94, render/busy/ms 47.10, pipelines/driver_ms 43.49, main/busy/ms 40.45, stage/render/render 24.47, render/graph/ms 20.70, stage/main/post_update 16.70, stage/render/prepare_assets 15.72, stage/render/graph_submit 12.64 |
| warm-up | 3.29 | 28.6 | pipelines/compile_ms 484, pipelines/front_ms 462, render/thread/ms 38.33, render/busy/ms 36.47, main/busy/ms 33.62, pipelines/driver_ms 22.57, pipelines/wait_ms 18.90, stage/main/post_update 15.84, stage/render/prepare_assets 13.90, stage/render/render 10.32, render/graph/ms 8.39, encode//sum 6.57 |
| warm-up | 3.40 | 92.4 | pipelines/compile_ms 927, pipelines/front_ms 889, main/busy/ms 65.70, render/thread/ms 64.34, render/busy/ms 61.72, pipelines/wait_ms 53.69, stage/main/update 51.41, stage/render/prepare_assets 38.98, pipelines/driver_ms 37.70, stage/render/render 16.04, render/graph/ms 13.36, stage/main/post_update 9.09 |
| warm-up | 3.45 | 25.1 | pipelines/compile_ms 843, pipelines/front_ms 803, render/thread/ms 41.20, pipelines/driver_ms 40.43, render/busy/ms 39.52, stage/render/prepare_assets 28.79, main/busy/ms 28.37, stage/main/update 10.70, stage/render/render 7.45, stage/main/post_update 7.02, render/graph/ms 5.63, stage/main/fixed_main_loop 4.56 |
| warm-up | 3.49 | 27.1 | pipelines/compile_ms 119, pipelines/front_ms 113, main/busy/ms 44.17, render/thread/ms 35.96, render/busy/ms 33.30, stage/main/update 28.01, stage/render/prepare_assets 18.94, stream/drive/ms 18.46, stage/main/post_update 11.54, stage/render/render 10.46, render/graph/ms 7.75, pipelines/driver_ms 5.56 |
| warm-up | 3.54 | 49.3 | pipelines/compile_ms 672, pipelines/front_ms 646, main/busy/ms 59.72, render/thread/ms 50.51, render/busy/ms 48.02, stage/main/update 38.29, stage/render/prepare_assets 32.79, pipelines/driver_ms 26.02, stream/drive/ms 20.14, stage/main/post_update 13.75, stage/render/render 10.28, render/graph/ms 7.55 |
| warm-up | 3.60 | 65.0 | pipelines/compile_ms 738, pipelines/front_ms 711, main/busy/ms 159, stage/main/update 142, render/thread/ms 79.22, render/busy/ms 77.68, stage/render/prepare_assets 61.13, pipelines/driver_ms 27.28, stage/main/post_update 9.39, stage/render/render 8.59, render/graph/ms 6.74, stage/render/graph_submit 4.36 |
| warm-up | 3.80 | 195.7 | pipelines/compile_ms 2396, pipelines/front_ms 2318, main/busy/ms 140, stage/main/update 129, render/thread/ms 117, render/busy/ms 115, stage/render/prepare_assets 101, pipelines/driver_ms 77.87, pipelines/wait_ms 71.76, stage/render/render 8.18, pipelines/lock_ms 7.14, render/graph/ms 6.43 |
| warm-up | 3.85 | 51.0 | pipelines/compile_ms 2326, pipelines/front_ms 2249, render/thread/ms 95.40, render/busy/ms 93.96, stage/render/prepare_assets 83.86, pipelines/driver_ms 76.73, pipelines/wait_ms 76.11, main/busy/ms 46.61, stage/main/update 34.97, stream/drive/ms 30.00, pipelines/lock_ms 11.03, stage/render/render 7.80 |
| warm-up | 3.88 | 34.5 | pipelines/compile_ms 139, pipelines/front_ms 132, render/thread/ms 53.55, render/busy/ms 52.06, main/busy/ms 50.87, stage/main/update 37.99, stage/render/prepare_assets 33.94, stream/drive/ms 30.24, stage/render/render 15.17, render/graph/ms 13.47, stage/render/graph_submit 9.05, encode/submit 7.18 |
| warm-up | 3.91 | 32.2 | pipelines/compile_ms 751, pipelines/front_ms 688, pipelines/driver_ms 62.79, render/thread/ms 51.21, render/busy/ms 49.56, main/busy/ms 33.21, pipelines/wait_ms 25.09, stage/render/prepare_resources 20.81, stage/main/update 17.90, stage/render/render 14.81, render/graph/ms 12.87, stream/drive/ms 12.03 |
| warm-up | 3.96 | 29.0 | pipelines/compile_ms 135, pipelines/front_ms 130, render/thread/ms 34.45, render/busy/ms 33.31, main/busy/ms 27.53, stage/render/prepare_assets 23.95, stage/main/post_update 12.28, stage/main/update 11.79, stage/render/render 6.10, pipelines/driver_ms 5.57, render/graph/ms 4.73, stage/render/graph_submit 3.10 |
| warm-up | 4.21 | 29.3 | pipelines/compile_ms 590, pipelines/front_ms 575, main/busy/ms 40.86, pipelines/wait_ms 34.11, render/thread/ms 22.00, stage/main/post_update 20.28, render/busy/ms 20.12, pipelines/driver_ms 15.23, stage/main/update 10.10, stage/render/prepare_assets 8.91, stage/render/render 8.34, render/graph/ms 6.11 |
| warm-up | 4.25 | 31.1 | pipelines/compile_ms 753, pipelines/front_ms 730, main/busy/ms 42.57, pipelines/wait_ms 33.53, render/thread/ms 32.50, render/busy/ms 30.45, pipelines/driver_ms 22.30, stage/main/post_update 22.06, stage/render/prepare_assets 11.03, stage/main/update 10.95, stage/render/render 10.23, render/graph/ms 8.05 |
| warm-up | 4.27 | 26.7 | pipelines/compile_ms 847, pipelines/front_ms 819, pipelines/wait_ms 57.75, render/thread/ms 40.31, render/busy/ms 38.54, main/busy/ms 34.81, pipelines/driver_ms 27.28, stage/main/update 16.20, stage/render/render 14.39, stage/main/post_update 13.14, render/graph/ms 12.44, stage/render/prepare_assets 8.11 |
| warm-up | 4.41 | 110.8 | pipelines/compile_ms 989, pipelines/front_ms 955, main/busy/ms 110, stage/main/update 86.45, render/thread/ms 58.02, render/busy/ms 56.33, stream/drive/ms 38.33, pipelines/driver_ms 34.40, stage/render/prepare_assets 29.74, pipelines/wait_ms 28.55, stage/main/post_update 16.71, stage/render/render 12.06 |
| warm-up | 4.45 | 48.9 | pipelines/compile_ms 759, pipelines/front_ms 731, render/thread/ms 78.45, render/busy/ms 76.64, main/busy/ms 74.16, stage/main/update 44.69, stage/render/prepare_assets 43.00, pipelines/driver_ms 27.53, stage/main/post_update 19.35, pipelines/wait_ms 18.84, stage/render/render 16.34, stream/drive/ms 13.96 |
| warm-up | 4.55 | 97.2 | pipelines/compile_ms 431, pipelines/front_ms 413, render/thread/ms 99.51, render/busy/ms 97.94, main/busy/ms 48.78, stage/render/prepare_assets 41.78, stage/render/create_views 33.02, stage/main/update 26.32, pipelines/driver_ms 17.45, stream/drive/ms 14.29, stage/render/render 13.12, stage/main/post_update 13.02 |
| warm-up | 4.59 | 42.8 | pipelines/compile_ms 417, pipelines/front_ms 401, render/thread/ms 89.66, render/busy/ms 88.69, stage/render/prepare_assets 44.16, main/busy/ms 33.60, stage/render/create_views 32.25, stage/main/update 16.37, pipelines/driver_ms 16.11, stage/main/post_update 9.97, stage/render/render 6.32, render/graph/ms 5.16 |
| warm-up | 4.63 | 36.7 | pipelines/compile_ms 71.52, pipelines/front_ms 67.56, render/thread/ms 62.14, render/busy/ms 59.52, main/busy/ms 59.34, stage/main/update 42.40, stage/render/prepare_assets 21.57, stream/drive/ms 20.54, stage/render/render 19.23, render/graph/ms 16.42, stage/render/graph_submit 13.55, encode//sum 12.26 |
| warm-up | 4.67 | 42.6 | pipelines/compile_ms 71.52, pipelines/front_ms 67.56, render/thread/ms 67.20, main/busy/ms 66.30, render/busy/ms 62.69, stage/main/update 39.98, stage/render/render 21.37, stream/drive/ms 20.46, stage/main/post_update 19.71, render/graph/ms 18.37, stage/render/graph_submit 15.21, encode//sum 13.16 |
| warm-up | 4.72 | 29.8 | pipelines/compile_ms 183, pipelines/front_ms 172, main/busy/ms 32.25, render/thread/ms 31.03, render/busy/ms 29.01, stage/main/update 18.86, stage/render/render 12.28, pipelines/driver_ms 11.39, render/graph/ms 10.04, stage/render/prepare_assets 7.75, stage/main/post_update 7.30, stage/render/graph_submit 7.05 |
| warm-up | 4.77 | 30.7 | pipelines/compile_ms 1419, pipelines/front_ms 1358, pipelines/driver_ms 60.74, render/thread/ms 41.59, render/busy/ms 39.73, main/busy/ms 28.33, stage/render/render 20.12, render/graph/ms 17.92, stage/main/update 12.30, stage/main/post_update 12.06, stage/render/graph_submit 11.85, pipelines/wait_ms 11.54 |
| warm-up | 4.81 | 28.0 | pipelines/compile_ms 131, pipelines/front_ms 125, render/thread/ms 33.21, render/busy/ms 31.74, main/busy/ms 24.75, pipelines/wait_ms 13.51, stage/render/render 13.27, render/graph/ms 11.43, stage/main/post_update 9.99, stage/render/prepare_assets 9.43, encode//sum 9.27, stage/main/update 8.25 |
| warm-up | 4.89 | 32.0 | pipelines/compile_ms 646, pipelines/front_ms 623, main/busy/ms 50.78, render/thread/ms 47.58, render/busy/ms 45.80, stage/main/update 28.88, stage/render/render 24.90, encode//sum 23.70, pipelines/driver_ms 23.07, render/graph/ms 22.71, stage/main/post_update 18.48, pipelines/wait_ms 13.42 |
| warm-up | 4.94 | 49.2 | pipelines/compile_ms 1497, pipelines/front_ms 1447, pipelines/wait_ms 75.99, render/thread/ms 69.43, render/busy/ms 67.28, main/busy/ms 66.45, pipelines/driver_ms 50.08, stage/main/update 42.06, stage/render/render 28.46, render/graph/ms 25.82, stage/main/post_update 20.39, encode//sum 20.37 |
| warm-up | 4.98 | 38.7 | pipelines/compile_ms 1443, pipelines/front_ms 1394, pipelines/wait_ms 116, render/thread/ms 62.44, render/busy/ms 60.32, pipelines/driver_ms 48.16, main/busy/ms 45.95, stage/main/update 27.23, stage/render/prepare_assets 24.58, stage/render/render 19.82, render/graph/ms 17.27, encode//sum 14.80 |
| warm-up | 5.02 | 40.4 | pipelines/compile_ms 590, pipelines/front_ms 576, pipelines/wait_ms 113, render/thread/ms 48.94, render/busy/ms 47.46, main/busy/ms 30.05, stage/render/prepare_assets 24.63, stage/main/update 14.91, pipelines/driver_ms 14.22, stage/render/render 12.98, encode//sum 12.03, render/graph/ms 11.21 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=cache-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 16%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (253 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **89%** (min 79%) · 2634 MHz (min 2625) · memory 8101 MHz · 88.0 W · 68 °C (max 74) · VRAM 4942 MiB · throttled 100% of samples

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
