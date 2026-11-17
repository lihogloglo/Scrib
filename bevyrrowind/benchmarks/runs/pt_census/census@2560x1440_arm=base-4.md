# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1597 frames over 11.4 s · **median 6.79 ms (147 fps)** · mean 7.16 · p95 9.50 · p99 10.23 (1% low 98 fps) · worst 16.38 ms · 20 spikes over 1.5× median · 68.2 stutters/min (work 21.0/min)
- warm-up: 120 frames over 6.2 s, worst 2567.6 ms
- hitches (> 25.0 ms): **0** measured, 39 in warm-up
- timed GPU passes: 5.12 ms of a 7.16 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 147.3, target 144
- pass worst frame 16.38 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.955 | 2.027 | 2.066 | 0.96 | 0.168 |
| main_opaque_pass_3d | 1.794 | 1.861 | 2.547 | 0.96 | 0.032 |
| bloom | 0.311 | 0.322 | 0.327 | 0.96 | 0.064 |
| auxiliary_views | 0.288 | 0.298 | 0.930 | 0.96 | 0.291 |
| ssao | 0.183 | 0.189 | 0.193 | 0.96 | 0.007 |
| upscaling | 0.146 | 0.152 | 0.173 | 0.96 | 0.005 |
| tlas_build | 0.110 | 0.114 | 0.119 | 0.96 | 0.073 |
| tonemapping | 0.096 | 0.100 | 0.106 | 0.96 | 0.004 |
| gi/probe_update | 0.091 | 0.095 | 0.101 | 0.96 | 0.002 |
| early prepass | 0.076 | 0.079 | 0.094 | 0.96 | 0.034 |
| clustering | 0.069 | 0.072 | 0.075 | 0.96 | 0.018 |
| uniform_allocation | 0.034 | 0.035 | 0.044 | 0.96 | 0.010 |
| bin_unpacking | 0.030 | 0.031 | 0.039 | 0.96 | 0.009 |
| early_mesh_preprocessing | 0.013 | 0.013 | 0.014 | 0.96 | 0.004 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.94 | 0.024 |
| setup_tlas_instances | 0.007 | 0.007 | 0.007 | 0.96 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.27 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.96 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.997 | 8.394 | 15.37 | 9576.5 |
| render/busy/ms | 5.552 | 7.925 | 14.90 | 8866.9 |
| main/busy/ms | 5.304 | 6.065 | 7.11 | 8470.6 |
| stage/render/render | 3.802 | 5.439 | 12.37 | 6071.9 |
| render/graph/ms | 3.205 | 4.805 | 11.71 | 5119.2 |
| stage/main/post_update | 2.919 | 3.464 | 4.60 | 4661.4 |
| encode//sum | 2.348 | 3.653 | 10.50 | 3750.1 |
| stage/render/graph_submit | 2.129 | 3.149 | 9.89 | 3399.7 |
| encode/submit | 1.435 | 2.167 | 3.24 | 2291.7 |
| stage/main/update | 1.237 | 1.475 | 2.18 | 1975.7 |
| encode/pass:shadow_point_light/sum | 0.979 | 2.935 | 4.36 | 1563.1 |
| stage/render/camera_driver | 0.948 | 1.585 | 2.44 | 1513.9 |
| encode/finish_wall | 0.714 | 1.098 | 7.02 | 1140.6 |
| stage/render/prepare_write_phase_buffers | 0.416 | 0.785 | 1.11 | 664.7 |
| stage/render/prepare_resources | 0.395 | 0.479 | 0.76 | 630.2 |
| encode/pass:early prepass/sum | 0.390 | 0.536 | 0.92 | 622.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.379 | 0.530 | 1.06 | 604.7 |
| render/present/ms | 0.364 | 0.516 | 1.47 | 582.0 |
| stage/main/first | 0.337 | 0.417 | 1.54 | 538.3 |
| stage/main/pre_update | 0.323 | 0.398 | 0.77 | 516.1 |
| stage/render/prepare_bind_groups | 0.283 | 0.592 | 0.91 | 451.2 |
| bigspace/total/ms | 0.249 | 0.582 | 1.25 | 397.1 |
| stage/main/fixed_main_loop | 0.212 | 0.550 | 1.25 | 338.5 |
| stage/render/prepare_meshes | 0.207 | 0.279 | 0.42 | 329.8 |
| stage/render/prepare_assets | 0.206 | 0.252 | 1.30 | 328.6 |
| stage/render/create_views | 0.149 | 0.187 | 0.45 | 237.3 |
| stage/render/extract_commands | 0.146 | 0.174 | 0.35 | 232.4 |
| stream/drive/ms | 0.138 | 0.184 | 0.38 | 220.4 |
| stage/render/prepare_flush | 0.136 | 0.236 | 0.52 | 217.8 |
| stage/render/prepare_views | 0.124 | 0.166 | 0.34 | 198.3 |
| stream/tick/ms | 0.111 | 0.154 | 0.35 | 177.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.109 | 0.154 | 0.61 | 173.4 |
| stage/main/last | 0.107 | 0.140 | 0.83 | 171.1 |
| bigspace/low_precision/ms | 0.096 | 0.269 | 0.77 | 153.4 |
| render/acquire/ms | 0.080 | 0.108 | 0.27 | 127.7 |
| stage/render/specialize | 0.077 | 0.101 | 0.50 | 123.3 |
| physics/step/ms | 0.076 | 0.243 | 0.83 | 121.3 |
| anim/evaluate/ms | 0.073 | 0.224 | 0.79 | 117.2 |
| stage/render/prepare_batch_phases | 0.065 | 0.111 | 0.25 | 104.3 |
| controller/feed/ms | 0.064 | 0.080 | 0.51 | 102.7 |
| bigspace/high_precision/ms | 0.054 | 0.216 | 0.69 | 87.0 |
| stage/render/phase_sort | 0.054 | 0.086 | 0.26 | 86.9 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.070 | 0.28 | 75.5 |
| stage/render/queue | 0.041 | 0.071 | 0.35 | 65.1 |
| anim/evaluate_cpu/ms | 0.035 | 0.051 | 0.45 | 56.3 |
| encode/resolve_diagnostics/sum | 0.031 | 0.040 | 0.60 | 49.0 |
| weather/map/ms | 0.027 | 0.038 | 0.08 | 42.6 |
| stage/render/cleanup | 0.022 | 0.033 | 0.20 | 34.7 |
| far/select/ms | 0.015 | 0.018 | 0.04 | 24.0 |
| weather/eval/ms | 0.012 | 0.017 | 0.04 | 19.3 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.019 | 0.51 | 19.3 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.48 | 16.7 |
| physics/insert/ms | 0.008 | 0.000 | 0.61 | 13.6 |
| script/update/ms | 0.008 | 0.013 | 0.04 | 13.2 |
| controller/move/ms | 0.006 | 0.008 | 0.03 | 9.8 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.0 |
| anim/apply/ms | 0.005 | 0.009 | 0.04 | 7.4 |
| controller/avoid/ms | 0.004 | 0.004 | 0.02 | 5.7 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 4.6 |
| controller/write/ms | 0.001 | 0.001 | 0.01 | 1.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.3 |
| weather/apply/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 1.0 |
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
| crowd/joint_blocks/count | count | 2 | 3 | 2 |
| crowd/joints_extracted/count | count | 94 | 1451 | 94 |
| crowd/joints_uploaded/count | count | 94 | 153 | 94 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 15.7 k | 15.7 k | 15.7 k |
| encode//max | ms | 0.38 | 6.79 | 0.34 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.51 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.48 | 0.02 |
| encode/finish_critical | ms | 0.48 | 6.79 | 0.41 |
| encode/pass:early prepass/max | ms | 0.40 | 0.92 | 0.41 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.38 | 1.06 | 0.39 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.61 | 0.08 |
| encode/pass:shadow_point_light/max | ms | 0.48 | 1.04 | 0.42 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.60 | 0.03 |
| far/entities | count | 162 | 162 | 162 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.6 k | 41.6 k | 41.6 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.26 M | 86.26 M | 86.26 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.55 | 14.90 | 4.90 |
| gpu/timed/ms | ms | 5.18 | 7.21 | 4.90 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1410 | 1410 | 1410 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 145.7 k | 145.7 k | 145.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 5850 | 5850 | 5850 |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 215.44 M | 215.44 M | 215.44 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 1 |
| phases/alpha_mask3d/bins | count | 3 | 7 | 1 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 10 | 22 | 1 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 2 | 2 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 1 |
| phases/alpha_mask3d_prepass/bins | count | 3 | 7 | 1 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 10 | 22 | 1 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 2 | 2 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 7 | 10 | 4 |
| phases/opaque3d/bins | count | 142 | 280 | 24 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 251 | 505 | 38 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 7 | 10 | 4 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 7 | 3 |
| phases/opaque3d_prepass/bins | count | 142 | 280 | 24 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 251 | 505 | 38 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 4 | 7 | 3 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 83 | 0 |
| phases/shadow/bins | count | 2638 | 2867 | 2173 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3600 | 3869 | 3105 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 227 | 240 | 189 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 99 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 285 | 285 | 285 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 3 | 188 | 0 |
| preprocess/bind_groups_created | count | 3 | 188 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 47 | 102 | 35 |
| process/cpu_usage | % | 24.52 | 35.41 | 31.73 |
| process/mem_usage | GiB | 3.75 | 3.80 | 3.80 |
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
| skin/blocks_changed/count | count | 0 | 23 | 0 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 2 | 3 | 2 |
| skin/joint_bytes_written/bytes | bytes | 6016 | 19.8 k | 9920 |
| skin/joints_written/count | count | 94 | 310 | 155 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 2108 | 2108 | 2108 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 306 | 306 | 306 |
| statics/materials_bound | count | 306 | 306 | 306 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 195 | 195 | 195 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 6.62 M | 6.62 M | 6.62 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 30 | 30 | 30 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.10 | 40.60 | 35.12 |
| system/mem_usage | % | 56.94 | 57.14 | 57.14 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2038 | 2038 | 2038 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 18 | 102 | 9 |
| views/all/bins | count | 2970 | 3305 | 2223 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 4157 | 4588 | 3183 |
| views/all/pipelines | count | 247 | 261 | 198 |
| views/all/pipelines_distinct | count | 22 | 28 | 16 |
| views/all/work_items | count | 582 | 2411 | 78 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 83 | 0 |
| views/lamp_face/bins | count | 2638 | 2867 | 2173 |
| views/lamp_face/count | count | 48 | 48 | 48 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 14 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3600 | 3869 | 3105 |
| views/lamp_face/pipelines | count | 227 | 240 | 189 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 1347 | 0 |
| views/main/batch_sets | count | 15 | 21 | 9 |
| views/main/bins | count | 290 | 574 | 50 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 522 | 1054 | 78 |
| views/main/pipelines | count | 15 | 21 | 9 |
| views/main/work_items | count | 539 | 1095 | 78 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 8725 | 8725 | 8725 |
| world/point_lights | count | 156 | 156 | 156 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 8.36 | 30 | 13.3 k |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 8 | 10 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 0 | 0 |
| encode//count | count | 31.12 | 75 | 49.7 k |
| encode/bind_walk/init_textures | count | 8648 | 23.8 k | 13.81 M |
| encode/bind_walk/init_textures_skipped | count | 17.9 k | 60.6 k | 28.53 M |
| encode/bind_walk/inits | count | 195 | 409 | 312.1 k |
| encode/bind_walk/inits_skipped | count | 20.63 | 65 | 32.9 k |
| encode/bind_walk/merge_textures | count | 7405 | 22.6 k | 11.83 M |
| encode/bind_walk/merge_textures_skipped | count | 32.41 | 77 | 51.8 k |
| encode/bind_walk/merges | count | 60.02 | 117 | 95.8 k |
| encode/bind_walk/merges_skipped | count | 7.96 | 22 | 12.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1597 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1597 |
| encode/pass:early prepass/count | count | 1 | 2 | 1597 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1597 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1550 |
| encode/pass:shadow_point_light/count | count | 3.21 | 14 | 5125 |
| encode/queue/command_buffers | count | 43.30 | 98 | 69.1 k |
| encode/queue/submits | count | 2 | 4 | 3194 |
| encode/queue/write_buffer/bytes | bytes | 476.2 k | 936.2 k | 760.45 M |
| encode/queue/write_buffer/calls | count | 150 | 352 | 239.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 158.0 k | 315.9 k | 252.25 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9582 |
| encode/queue/write_texture/bytes | bytes | 11.2 k | 8.39 M | 17.83 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1597 |
| physics/inserts | count | 0.06 | 12 | 100 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.43 | 1 | 686 |
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

## Hitches (> 25.0 ms; slowest 39, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2567.6 | main/busy/ms 2536, stage/main/post_update 2434, stage/main/update 94.69, surface/advance_cpu_ms 66.79, stage/main/pre_update 4.23, stage/main/last 1.14, stage/main/first 0.50, stage/main/fixed_main_loop 0.42, stream/drive/ms 0.32, weather/map/ms 0.15, bigspace/total/ms 0.12, bigspace/high_precision/ms 0.06 |
| warm-up | 3.33 | 762.3 | main/busy/ms 2656, stage/main/post_update 2439, render/thread/ms 723, render/busy/ms 528, stage/render/prepare_assets 493, stage/main/update 203, stage/render/prepare_views 191, render/acquire/ms 190, surface/advance_cpu_ms 66.79, stage/render/prepare_resources 21.43, stage/render/render 13.66, render/graph/ms 7.78 |
| warm-up | 3.48 | 152.3 | render/thread/ms 859, render/busy/ms 576, stage/render/prepare_assets 535, stage/render/prepare_views 191, render/acquire/ms 190, main/busy/ms 134, stage/main/update 113, stage/render/render 105, render/present/ms 93.25, stage/render/prepare_resources 22.12, render/graph/ms 10.93, stage/main/post_update 8.92 |
| warm-up | 3.58 | 77.4 | pipelines/wait_ms 1513, pipelines/compile_ms 674, pipelines/front_ms 583, pipelines/driver_ms 90.92, render/thread/ms 83.64, main/busy/ms 81.44, render/busy/ms 80.82, stage/render/prepare_assets 53.73, stage/main/update 35.58, stage/main/pre_update 20.49, stage/main/post_update 16.73, stage/render/render 16.60 |
| warm-up | 3.61 | 35.8 | pipelines/wait_ms 1565, pipelines/compile_ms 794, pipelines/front_ms 698, main/busy/ms 141, render/thread/ms 103, render/busy/ms 101, pipelines/driver_ms 95.41, stage/main/update 81.09, stage/render/prepare_assets 61.97, stage/main/pre_update 38.33, stage/main/post_update 14.79, stage/render/render 11.08 |
| warm-up | 3.72 | 107.0 | pipelines/compile_ms 1443, pipelines/front_ms 1391, pipelines/wait_ms 165, main/busy/ms 93.03, render/thread/ms 90.82, render/busy/ms 89.10, stage/main/update 61.36, stage/render/prepare_assets 59.58, pipelines/driver_ms 51.90, stage/main/pre_update 20.24, stage/render/render 9.55, render/graph/ms 7.62 |
| warm-up | 3.82 | 39.2 | pipelines/compile_ms 70.12, pipelines/front_ms 67.04, main/busy/ms 58.85, render/thread/ms 49.21, render/busy/ms 45.06, stage/main/update 41.15, stage/render/render 16.38, stream/drive/ms 15.45, stage/render/prepare_assets 14.57, render/graph/ms 11.74, stage/main/post_update 11.66, stage/render/prepare_flush 10.51 |
| warm-up | 3.86 | 43.2 | pipelines/compile_ms 937, pipelines/front_ms 907, render/thread/ms 57.72, render/busy/ms 53.40, main/busy/ms 38.34, stage/render/prepare_assets 36.76, pipelines/driver_ms 30.16, stage/main/update 22.00, stream/drive/ms 15.17, stage/render/render 12.44, stage/main/post_update 9.02, render/graph/ms 8.06 |
| warm-up | 3.89 | 31.1 | pipelines/compile_ms 1229, pipelines/front_ms 1188, render/thread/ms 55.36, render/busy/ms 52.79, pipelines/driver_ms 40.68, main/busy/ms 33.26, stage/render/prepare_assets 30.25, stage/main/update 12.61, stage/render/render 10.56, stage/render/prepare_resources 9.65, stage/main/post_update 8.29, render/graph/ms 8.19 |
| warm-up | 3.93 | 36.6 | pipelines/compile_ms 470, pipelines/front_ms 452, render/thread/ms 57.59, render/busy/ms 55.49, stage/render/prepare_assets 33.44, main/busy/ms 32.52, pipelines/driver_ms 17.20, stage/main/update 13.27, stage/render/prepare_resources 9.79, stage/render/render 9.46, stage/main/post_update 7.62, render/graph/ms 7.49 |
| warm-up | 3.98 | 48.7 | pipelines/compile_ms 723, pipelines/front_ms 700, render/thread/ms 73.93, render/busy/ms 72.40, main/busy/ms 58.89, stage/render/prepare_assets 47.79, stage/main/update 45.24, stream/drive/ms 41.16, pipelines/driver_ms 22.16, stage/main/post_update 10.05, stage/render/prepare_flush 9.46, stage/render/render 7.46 |
| warm-up | 4.01 | 33.4 | pipelines/compile_ms 1214, pipelines/front_ms 1174, main/busy/ms 59.03, render/thread/ms 58.60, render/busy/ms 57.05, pipelines/wait_ms 47.29, stage/main/update 44.25, stream/drive/ms 41.24, pipelines/driver_ms 40.39, stage/render/prepare_assets 31.12, stage/main/post_update 10.87, stage/render/prepare_flush 9.61 |
| warm-up | 4.04 | 30.9 | pipelines/compile_ms 759, pipelines/front_ms 730, pipelines/wait_ms 52.36, render/thread/ms 45.84, render/busy/ms 43.64, main/busy/ms 33.55, pipelines/driver_ms 28.81, stage/render/prepare_assets 19.28, stage/main/update 18.75, stage/render/render 18.05, render/graph/ms 15.66, stage/render/graph_submit 13.68 |
| warm-up | 4.10 | 39.6 | pipelines/compile_ms 1016, pipelines/front_ms 976, pipelines/wait_ms 61.95, render/thread/ms 45.26, render/busy/ms 43.64, pipelines/driver_ms 39.51, main/busy/ms 37.47, stage/render/prepare_assets 31.34, stage/main/update 23.37, stage/render/render 7.44, render/graph/ms 5.77, stage/main/post_update 5.42 |
| warm-up | 4.15 | 26.7 | pipelines/compile_ms 299, pipelines/front_ms 292, pipelines/wait_ms 67.03, render/thread/ms 38.90, main/busy/ms 37.15, render/busy/ms 36.24, stage/main/post_update 16.86, stage/render/render 15.67, stage/main/update 15.03, stage/render/prepare_bind_groups 10.65, render/graph/ms 9.76, stage/render/prepare_assets 8.97 |
| warm-up | 4.19 | 39.6 | pipelines/compile_ms 439, pipelines/front_ms 426, pipelines/wait_ms 51.56, main/busy/ms 48.15, render/thread/ms 47.25, render/busy/ms 44.48, stage/main/post_update 24.59, stage/main/update 15.37, pipelines/driver_ms 13.17, stage/render/prepare_assets 11.70, stage/render/render 11.31, stage/render/prepare_bind_groups 10.75 |
| warm-up | 4.33 | 26.6 | pipelines/compile_ms 589, pipelines/front_ms 571, pipelines/wait_ms 58.50, main/busy/ms 39.38, render/thread/ms 30.20, render/busy/ms 28.84, stage/main/post_update 20.97, pipelines/driver_ms 17.43, stage/main/update 11.12, stage/render/render 10.57, stage/render/prepare_assets 9.83, render/graph/ms 9.07 |
| warm-up | 4.36 | 30.0 | pipelines/compile_ms 731, pipelines/front_ms 707, render/thread/ms 40.40, render/busy/ms 39.10, main/busy/ms 36.18, pipelines/wait_ms 33.47, pipelines/driver_ms 23.82, stage/main/post_update 16.65, stage/main/update 16.28, stage/render/render 9.77, stage/render/prepare_resources 9.32, render/graph/ms 8.37 |
| warm-up | 4.49 | 84.5 | pipelines/compile_ms 1240, pipelines/front_ms 1202, main/busy/ms 89.09, stage/main/update 64.87, stream/drive/ms 56.95, pipelines/wait_ms 45.32, pipelines/driver_ms 38.42, render/thread/ms 28.11, render/busy/ms 25.78, stage/main/post_update 14.47, stage/render/prepare_assets 9.75, stage/render/render 9.52 |
| warm-up | 4.53 | 37.3 | pipelines/compile_ms 1563, pipelines/front_ms 1513, main/busy/ms 102, stage/main/update 77.07, pipelines/wait_ms 54.24, pipelines/driver_ms 50.27, render/thread/ms 48.89, render/busy/ms 46.61, stream/drive/ms 43.81, stage/render/prepare_assets 19.41, stage/main/post_update 16.11, stage/render/render 14.47 |
| warm-up | 4.64 | 111.4 | pipelines/compile_ms 1342, pipelines/front_ms 1295, main/busy/ms 90.46, stage/main/update 75.95, render/thread/ms 68.77, render/busy/ms 66.86, pipelines/driver_ms 47.36, stream/drive/ms 43.04, pipelines/wait_ms 38.00, stage/render/prepare_assets 36.84, stage/render/render 17.13, render/graph/ms 14.69 |
| warm-up | 4.68 | 32.3 | pipelines/compile_ms 940, pipelines/front_ms 905, render/thread/ms 71.45, render/busy/ms 69.52, encode//sum 51.93, pipelines/driver_ms 34.24, stage/render/render 28.87, main/busy/ms 27.52, stage/render/prepare_assets 27.38, render/graph/ms 26.42, pipelines/wait_ms 19.32, stage/render/graph_submit 17.00 |
| warm-up | 4.71 | 33.3 | encode//sum 71.52, render/thread/ms 60.92, render/busy/ms 58.78, pipelines/compile_ms 39.30, stage/render/render 36.83, pipelines/front_ms 36.73, render/graph/ms 34.12, main/busy/ms 29.01, stage/render/graph_submit 23.12, encode/pass:shadow_point_light/sum 16.90, stage/main/post_update 12.89, encode/finish_wall 12.20 |
| warm-up | 4.76 | 50.5 | pipelines/compile_ms 318, pipelines/front_ms 299, render/thread/ms 69.10, render/busy/ms 67.27, main/busy/ms 45.90, encode//sum 35.33, stage/render/render 28.81, render/graph/ms 26.25, stage/main/update 23.97, pipelines/driver_ms 19.39, stage/render/prepare_assets 15.91, stage/render/graph_submit 15.24 |
| warm-up | 4.80 | 38.7 | pipelines/compile_ms 524, pipelines/front_ms 500, render/thread/ms 68.52, render/busy/ms 67.09, main/busy/ms 44.54, stage/main/update 26.21, stage/render/prepare_assets 24.27, pipelines/driver_ms 23.42, stage/render/render 21.54, render/graph/ms 19.44, encode//sum 17.65, stage/main/post_update 11.17 |
| warm-up | 4.88 | 83.6 | pipelines/compile_ms 225, pipelines/front_ms 220, render/thread/ms 106, render/busy/ms 104, encode//sum 85.46, main/busy/ms 50.65, stage/render/render 50.45, render/graph/ms 47.78, stage/render/graph_submit 37.18, stage/main/update 36.03, encode/pass:shadow_point_light/sum 29.16, stream/drive/ms 26.49 |
| warm-up | 4.93 | 47.8 | render/thread/ms 116, render/busy/ms 114, encode//sum 89.69, pipelines/compile_ms 84.96, pipelines/front_ms 79.43, main/busy/ms 74.34, stage/render/render 57.47, render/graph/ms 54.67, stage/main/update 51.35, stream/drive/ms 44.13, stage/render/graph_submit 40.86, encode/pass:shadow_point_light/sum 32.32 |
| warm-up | 4.99 | 57.2 | pipelines/compile_ms 1186, pipelines/front_ms 1137, render/thread/ms 82.42, render/busy/ms 81.12, main/busy/ms 66.16, pipelines/driver_ms 48.69, stage/main/update 43.53, stream/drive/ms 34.50, stage/render/render 28.85, render/graph/ms 26.75, stage/render/prepare_assets 25.90, encode//sum 25.21 |
| warm-up | 5.04 | 50.2 | pipelines/compile_ms 1137, pipelines/front_ms 1093, render/thread/ms 87.67, render/busy/ms 86.00, main/busy/ms 44.67, pipelines/driver_ms 44.46, stage/render/render 42.98, render/graph/ms 40.57, stage/main/update 30.20, stage/render/graph_submit 22.34, stage/render/prepare_assets 22.30, stream/drive/ms 22.21 |
| warm-up | 5.08 | 39.1 | pipelines/compile_ms 93.65, pipelines/front_ms 89.69, render/thread/ms 80.30, render/busy/ms 77.60, main/busy/ms 58.67, stage/render/render 50.53, render/graph/ms 47.27, stage/main/post_update 31.71, stage/render/graph_submit 28.45, stage/main/update 21.54, stage/render/camera_driver 17.93, stream/drive/ms 16.41 |
| warm-up | 5.17 | 95.9 | pipelines/compile_ms 945, pipelines/front_ms 913, render/thread/ms 85.10, render/busy/ms 82.33, main/busy/ms 61.30, stage/render/render 35.82, stage/render/prepare_assets 35.40, stage/main/post_update 34.22, render/graph/ms 32.51, pipelines/driver_ms 32.16, stage/main/update 21.27, stage/render/graph_submit 17.38 |
| warm-up | 5.24 | 71.7 | pipelines/compile_ms 1513, pipelines/front_ms 1470, render/thread/ms 93.05, render/busy/ms 91.22, stage/render/prepare_assets 58.10, pipelines/driver_ms 43.59, main/busy/ms 37.11, stage/render/render 22.60, render/graph/ms 20.11, pipelines/wait_ms 16.52, stage/main/update 15.86, pipelines/lock_ms 14.90 |
| warm-up | 5.32 | 55.5 | pipelines/compile_ms 684, pipelines/front_ms 667, render/thread/ms 70.19, render/busy/ms 48.92, main/busy/ms 37.52, stage/render/prepare_assets 22.11, stage/main/update 21.04, stage/render/prepare_views 20.45, render/acquire/ms 20.31, pipelines/driver_ms 17.14, stage/render/render 14.53, render/graph/ms 12.83 |
| warm-up | 5.36 | 26.9 | pipelines/compile_ms 189, pipelines/front_ms 184, render/thread/ms 40.15, render/busy/ms 37.41, main/busy/ms 34.93, stage/render/render 25.59, render/graph/ms 22.22, stage/main/update 22.19, stage/render/graph_submit 17.82, encode/submit 16.53, stream/drive/ms 15.72, encode//sum 10.12 |
| warm-up | 5.46 | 26.1 | render/thread/ms 35.86, render/busy/ms 34.80, main/busy/ms 18.72, stage/render/render 13.73, stage/render/prepare_assets 13.13, render/graph/ms 12.20, encode//sum 10.35, stage/render/graph_submit 7.84, stage/main/post_update 7.62, stage/main/update 7.47, encode/pass:shadow_point_light/sum 5.98, encode/submit 5.27 |
| warm-up | 5.54 | 30.2 | render/thread/ms 41.66, render/busy/ms 40.42, main/busy/ms 26.73, stage/render/prepare_assets 20.41, stage/main/update 12.70, stage/render/render 11.95, render/graph/ms 10.25, stage/main/post_update 9.77, encode//sum 8.92, stage/render/graph_submit 6.44, encode/pass:shadow_point_light/sum 5.11, encode/submit 4.45 |
| warm-up | 5.59 | 25.9 | render/thread/ms 36.92, render/busy/ms 35.85, main/busy/ms 32.10, stage/main/update 18.69, stage/render/render 14.26, render/graph/ms 12.61, stage/render/prepare_assets 11.38, encode//sum 9.01, stream/drive/ms 8.38, stage/render/graph_submit 8.34, stage/main/post_update 8.16, encode/pass:shadow_point_light/sum 6.41 |
| warm-up | 5.67 | 25.7 | render/thread/ms 36.97, render/busy/ms 35.45, main/busy/ms 31.33, stage/render/render 22.49, render/graph/ms 20.27, stage/main/post_update 15.40, stage/render/graph_submit 13.97, encode//sum 12.36, encode/submit 10.49, stage/main/update 10.45, encode/pass:shadow_point_light/sum 8.95, stage/render/camera_driver 5.71 |
| warm-up | 5.70 | 32.2 | render/thread/ms 43.18, render/busy/ms 42.12, main/busy/ms 32.83, stage/render/prepare_assets 16.47, stage/render/render 16.29, render/graph/ms 14.87, stage/main/post_update 13.39, stage/main/update 12.59, encode//sum 10.68, stage/render/graph_submit 10.21, encode/submit 7.35, encode/pass:shadow_point_light/sum 6.97 |

## Run

- `target/profiling/perftests.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\pt_census\census@2560x1440_arm=base-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 19%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (220 pmon samples)
- focus: clean
- nvidia-smi over the window (43 samples): GPU load **82%** (min 71%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.7 W · 74 °C (max 79) · VRAM 5423 MiB · throttled 100% of samples

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
