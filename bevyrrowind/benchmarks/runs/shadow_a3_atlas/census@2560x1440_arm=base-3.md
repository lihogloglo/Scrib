# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1580 frames over 11.4 s · **median 6.85 ms (146 fps)** · mean 7.24 · p95 10.11 · p99 10.89 (1% low 92 fps) · worst 13.14 ms · 55 spikes over 1.5× median · 47.2 stutters/min (work 5.2/min)
- warm-up: 120 frames over 6.3 s, worst 2502.6 ms
- hitches (> 25.0 ms): **0** measured, 29 in warm-up
- timed GPU passes: 5.13 ms of a 7.24 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 146.1, target 144
- pass worst frame 13.14 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.909 | 2.155 | 2.521 | 0.89 | 0.167 |
| main_opaque_pass_3d | 1.737 | 1.961 | 2.848 | 0.89 | 0.038 |
| auxiliary_views | 0.312 | 0.352 | 1.129 | 0.89 | 0.293 |
| bloom | 0.302 | 0.341 | 0.493 | 0.89 | 0.063 |
| ssao | 0.193 | 0.217 | 0.550 | 0.89 | 0.007 |
| upscaling | 0.146 | 0.165 | 0.191 | 0.89 | 0.004 |
| tlas_build | 0.102 | 0.115 | 0.119 | 0.89 | 0.075 |
| tonemapping | 0.096 | 0.109 | 0.119 | 0.89 | 0.004 |
| gpu_instance_materialization | 0.085 | 0.096 | 0.099 | 0.89 | 0.004 |
| gi/probe_update | 0.085 | 0.096 | 0.097 | 0.89 | 0.002 |
| early prepass | 0.070 | 0.079 | 0.086 | 0.89 | 0.045 |
| clustering | 0.065 | 0.074 | 0.077 | 0.89 | 0.019 |
| uniform_allocation | 0.035 | 0.039 | 0.044 | 0.89 | 0.009 |
| bin_unpacking | 0.028 | 0.031 | 0.039 | 0.89 | 0.008 |
| range_unpacking | 0.017 | 0.019 | 0.019 | 0.89 | 0.002 |
| early_mesh_preprocessing | 0.016 | 0.018 | 0.025 | 0.89 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.86 | 0.023 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.89 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.28 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.89 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 6.482 | 0.000 | 2293.29 | 10242.3 |
| render/thread/ms | 6.093 | 8.919 | 11.82 | 9627.3 |
| render/busy/ms | 5.608 | 8.453 | 11.23 | 8860.1 |
| pipelines/driver_ms | 5.406 | 0.000 | 2160.72 | 8542.2 |
| main/busy/ms | 5.393 | 6.354 | 8.18 | 8520.6 |
| stage/render/render | 3.864 | 5.880 | 7.99 | 6105.2 |
| render/graph/ms | 3.238 | 5.218 | 6.89 | 5116.5 |
| stage/main/post_update | 2.887 | 3.550 | 5.19 | 4561.4 |
| encode//sum | 2.549 | 4.166 | 6.90 | 4027.6 |
| stage/render/graph_submit | 2.102 | 3.425 | 4.96 | 3320.7 |
| encode/submit | 1.460 | 2.409 | 3.90 | 2307.5 |
| stage/main/update | 1.242 | 1.524 | 2.13 | 1962.3 |
| pipelines/front_ms | 1.076 | 0.000 | 132.57 | 1700.2 |
| stage/render/camera_driver | 1.000 | 1.664 | 2.26 | 1579.3 |
| encode/pass:shadow_point_light/sum | 0.877 | 2.863 | 5.21 | 1385.7 |
| encode/finish_wall | 0.692 | 1.079 | 2.04 | 1092.9 |
| stage/render/prepare_resources | 0.421 | 0.507 | 2.27 | 664.9 |
| stage/render/prepare_write_phase_buffers | 0.407 | 0.800 | 2.21 | 642.8 |
| encode/pass:early prepass/sum | 0.393 | 0.555 | 1.53 | 621.5 |
| render/present/ms | 0.388 | 0.785 | 2.22 | 613.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.368 | 0.534 | 1.57 | 581.6 |
| stage/main/pre_update | 0.354 | 0.442 | 0.72 | 559.1 |
| stage/main/first | 0.340 | 0.431 | 1.48 | 537.4 |
| stage/render/prepare_bind_groups | 0.323 | 0.666 | 1.21 | 509.9 |
| stage/main/fixed_main_loop | 0.243 | 0.584 | 0.92 | 384.7 |
| bigspace/total/ms | 0.218 | 0.457 | 0.93 | 344.6 |
| stage/render/prepare_assets | 0.214 | 0.275 | 1.03 | 338.8 |
| stage/render/prepare_meshes | 0.210 | 0.285 | 0.48 | 331.4 |
| stage/render/prepare_flush | 0.183 | 0.349 | 4.69 | 288.6 |
| stage/render/create_views | 0.171 | 0.215 | 0.52 | 270.3 |
| stage/main/last | 0.150 | 0.185 | 1.10 | 237.1 |
| stage/render/prepare_views | 0.139 | 0.248 | 0.94 | 219.1 |
| stage/render/extract_commands | 0.138 | 0.168 | 0.40 | 217.7 |
| stream/drive/ms | 0.124 | 0.175 | 0.33 | 196.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.110 | 0.163 | 1.92 | 173.2 |
| stream/tick/ms | 0.098 | 0.146 | 0.31 | 154.5 |
| render/acquire/ms | 0.097 | 0.190 | 0.90 | 153.9 |
| bigspace/low_precision/ms | 0.094 | 0.228 | 0.84 | 148.2 |
| stage/render/prepare_batch_phases | 0.089 | 0.140 | 0.41 | 140.7 |
| controller/feed/ms | 0.080 | 0.101 | 0.37 | 126.6 |
| stage/render/specialize | 0.070 | 0.112 | 0.51 | 110.5 |
| anim/evaluate/ms | 0.065 | 0.123 | 0.66 | 102.7 |
| physics/step/ms | 0.063 | 0.216 | 0.46 | 100.2 |
| stage/render/phase_sort | 0.055 | 0.088 | 0.32 | 86.6 |
| bigspace/high_precision/ms | 0.051 | 0.210 | 0.71 | 80.2 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.069 | 0.17 | 72.1 |
| stage/render/queue | 0.040 | 0.069 | 0.41 | 63.4 |
| anim/evaluate_cpu/ms | 0.035 | 0.053 | 0.31 | 55.2 |
| encode/resolve_diagnostics/sum | 0.032 | 0.045 | 0.21 | 50.7 |
| weather/map/ms | 0.027 | 0.039 | 0.28 | 42.8 |
| controller/write/ms | 0.027 | 0.040 | 0.18 | 42.0 |
| stage/render/cleanup | 0.022 | 0.033 | 0.21 | 35.4 |
| physics/insert/ms | 0.015 | 0.000 | 0.63 | 24.0 |
| weather/eval/ms | 0.013 | 0.020 | 0.19 | 20.5 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.020 | 0.10 | 19.2 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.17 | 16.4 |
| script/update/ms | 0.008 | 0.013 | 0.06 | 12.6 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 8.2 |
| anim/apply/ms | 0.005 | 0.010 | 0.02 | 7.4 |
| pipelines/wait_ms | 0.004 | 0.000 | 4.57 | 6.3 |
| controller/move/ms | 0.004 | 0.011 | 0.06 | 6.2 |
| far/select/ms | 0.004 | 0.004 | 0.02 | 5.9 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 5.1 |
| controller/avoid/ms | 0.002 | 0.005 | 0.02 | 3.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.9 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stage/render/prepare | 0.001 | 0.002 | 0.11 | 1.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.21 | 0.2 |
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
| crowd/joints_extracted/count | count | 46 | 1403 | 0 |
| crowd/joints_uploaded/count | count | 94 | 153 | 94 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.39 | 1.18 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.10 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.17 | 0.01 |
| encode/finish_critical | ms | 0.48 | 1.92 | 0.47 |
| encode/pass:early prepass/max | ms | 0.39 | 1.53 | 0.47 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.36 | 1.57 | 0.33 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 1.92 | 0.11 |
| encode/pass:shadow_point_light/max | ms | 0.46 | 0.98 | 0.47 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.21 | 0.05 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.70 | 11.23 | 5.46 |
| gpu/timed/ms | ms | 5.56 | 7.97 | 5.09 |
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
| phases/opaque3d/bins | count | 142 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 250 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 142 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 250 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 83 | 0 |
| phases/shadow/bins | count | 2638 | 2866 | 2173 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3600 | 3839 | 3105 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 227 | 240 | 189 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 130 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 291 | 291 | 291 |
| pipelines/waiting | count | 0 | 4 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 0 | 3 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 26 | 0 |
| preprocess/bind_groups | count | 7 | 192 | 4 |
| preprocess/bind_groups_created | count | 3 | 188 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 47 | 99 | 35 |
| process/cpu_usage | % | 27.43 | 77.40 | 30.29 |
| process/mem_usage | GiB | 3.96 | 4.04 | 4.03 |
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
| skin/blocks_changed/count | count | 1 | 24 | 0 |
| skin/blocks_skipped/count | count | 2 | 3 | 2 |
| skin/blocks_walked/count | count | 1 | 2 | 0 |
| skin/joint_bytes_written/bytes | bytes | 2944 | 19.8 k | 0 |
| skin/joints_written/count | count | 46 | 310 | 0 |
| skin/upload_writes/count | count | 1 | 2 | 0 |
| started_load_count |  | 2750 | 2750 | 2750 |
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
| system/cpu_usage | % | 33.21 | 54.24 | 33.96 |
| system/mem_usage | % | 51.57 | 51.82 | 51.82 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 26 | 112 | 17 |
| views/all/bins | count | 2981 | 3313 | 2231 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 4165 | 4586 | 3191 |
| views/all/pipelines | count | 253 | 267 | 204 |
| views/all/pipelines_distinct | count | 28 | 34 | 22 |
| views/all/work_items | count | 98.9 k | 100.7 k | 98.4 k |
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
| views/lamp_face/bins | count | 2638 | 2866 | 2173 |
| views/lamp_face/count | count | 48 | 48 | 48 |
| views/lamp_face/culled | count | 25 | 36 | 35 |
| views/lamp_face/drawn | count | 0 | 12 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3600 | 3839 | 3105 |
| views/lamp_face/pipelines | count | 227 | 240 | 189 |
| views/lamp_face/revealed | count | 0 | 1 | 0 |
| views/lamp_face/work_items | count | 0 | 1316 | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 295 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 524 | 1062 | 86 |
| views/main/pipelines | count | 21 | 27 | 15 |
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
| anim/bones_written/count | count | 8.18 | 30 | 12.9 k |
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
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 31.07 | 67 | 49.1 k |
| encode/bind_walk/init_textures | count | 8175 | 21.0 k | 12.92 M |
| encode/bind_walk/init_textures_skipped | count | 10.9 k | 29.6 k | 17.26 M |
| encode/bind_walk/inits | count | 214 | 438 | 338.4 k |
| encode/bind_walk/inits_skipped | count | 13.52 | 33 | 21.4 k |
| encode/bind_walk/merge_textures | count | 6932 | 19.8 k | 10.95 M |
| encode/bind_walk/merge_textures_skipped | count | 3801 | 16.9 k | 6.01 M |
| encode/bind_walk/merges | count | 58.52 | 118 | 92.5 k |
| encode/bind_walk/merges_skipped | count | 16.72 | 59 | 26.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1580 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1580 |
| encode/pass:early prepass/count | count | 1 | 1 | 1580 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1580 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1531 |
| encode/pass:shadow_point_light/count | count | 2.94 | 12 | 4644 |
| encode/queue/command_buffers | count | 42.98 | 88 | 67.9 k |
| encode/queue/submits | count | 2 | 2 | 3160 |
| encode/queue/write_buffer/bytes | bytes | 598.7 k | 686.8 k | 945.89 M |
| encode/queue/write_buffer/calls | count | 157 | 370 | 248.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 149.6 k | 149.6 k | 236.42 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 9480 |
| encode/queue/write_texture/bytes | bytes | 664 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1580 |
| physics/inserts | count | 0.09 | 11 | 136 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.43 | 1 | 687 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 28 |
| pipelines/created | count | 0.02 | 1 | 28 |
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

## Hitches (> 25.0 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2502.6 | main/busy/ms 2474, stage/main/post_update 2365, stage/main/update 99.55, surface/advance_cpu_ms 70.56, stage/main/pre_update 6.47, stage/main/last 1.20, stage/main/first 0.56, stage/main/fixed_main_loop 0.38, stream/drive/ms 0.26, weather/map/ms 0.22, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06 |
| warm-up | 3.28 | 778.8 | main/busy/ms 2594, stage/main/post_update 2370, render/thread/ms 731, render/busy/ms 554, stage/render/prepare_assets 521, stage/main/update 208, stage/render/prepare_views 176, render/acquire/ms 174, surface/advance_cpu_ms 70.56, stage/render/prepare_resources 22.29, stage/render/render 9.10, stage/main/pre_update 7.66 |
| warm-up | 3.34 | 58.3 | render/thread/ms 773, render/busy/ms 596, stage/render/prepare_assets 557, stage/render/prepare_views 176, render/acquire/ms 175, main/busy/ms 135, stage/main/update 115, stage/render/prepare_resources 23.12, stage/render/render 12.95, stage/main/post_update 9.38, render/graph/ms 9.29, stage/main/fixed_main_loop 7.53 |
| warm-up | 3.40 | 37.1 | pipelines/wait_ms 1205, pipelines/compile_ms 511, pipelines/front_ms 469, main/busy/ms 42.98, pipelines/driver_ms 42.32, render/thread/ms 37.39, render/busy/ms 32.90, stage/main/post_update 21.81, stage/render/render 13.46, stage/render/prepare_resources 9.90, render/graph/ms 9.04, stage/main/update 8.14 |
| warm-up | 3.48 | 41.5 | pipelines/compile_ms 300, pipelines/front_ms 275, pipelines/wait_ms 59.98, main/busy/ms 51.55, render/thread/ms 45.07, render/busy/ms 43.18, stage/main/post_update 30.89, pipelines/driver_ms 24.84, stage/render/render 16.98, render/graph/ms 15.35, stage/render/prepare_assets 12.42, stage/render/camera_driver 10.83 |
| warm-up | 3.53 | 36.2 | pipelines/compile_ms 681, pipelines/front_ms 656, pipelines/wait_ms 110, main/busy/ms 80.13, stage/main/update 54.74, render/thread/ms 33.46, render/busy/ms 31.63, pipelines/driver_ms 25.38, stage/render/prepare_assets 13.77, stage/main/post_update 13.23, stage/render/render 10.55, render/graph/ms 8.56 |
| warm-up | 3.61 | 83.4 | pipelines/compile_ms 1060, pipelines/front_ms 1021, pipelines/wait_ms 96.55, main/busy/ms 74.91, render/thread/ms 68.53, render/busy/ms 66.94, stage/main/update 57.60, stage/render/prepare_assets 52.42, pipelines/driver_ms 39.00, stage/main/post_update 9.17, stage/render/render 7.97, render/graph/ms 6.20 |
| warm-up | 3.67 | 37.6 | pipelines/compile_ms 817, pipelines/front_ms 779, render/thread/ms 57.20, render/busy/ms 55.23, main/busy/ms 43.53, stage/render/prepare_assets 38.85, pipelines/driver_ms 37.67, stage/main/update 27.85, stream/drive/ms 18.42, stage/render/render 13.51, render/graph/ms 11.26, stage/render/graph_submit 8.87 |
| warm-up | 3.70 | 28.5 | pipelines/compile_ms 751, pipelines/front_ms 714, render/thread/ms 55.73, render/busy/ms 53.66, stage/render/prepare_assets 37.45, pipelines/driver_ms 37.26, main/busy/ms 37.18, stage/main/update 24.63, stream/drive/ms 18.05, stage/render/render 13.88, render/graph/ms 11.48, stage/render/graph_submit 8.96 |
| warm-up | 3.75 | 42.3 | pipelines/compile_ms 385, pipelines/front_ms 372, render/thread/ms 53.97, render/busy/ms 52.14, main/busy/ms 38.34, stage/render/prepare_assets 34.29, stage/main/post_update 16.30, pipelines/driver_ms 13.23, stage/main/update 11.22, stage/render/render 9.93, render/graph/ms 7.81, stage/main/fixed_main_loop 7.26 |
| warm-up | 3.80 | 57.5 | pipelines/compile_ms 634, pipelines/front_ms 614, main/busy/ms 186, stage/main/update 156, render/thread/ms 84.50, render/busy/ms 82.39, stage/render/render 32.68, stage/render/prepare_assets 29.55, render/graph/ms 26.94, pipelines/driver_ms 19.62, stage/main/post_update 15.36, stage/render/camera_driver 12.38 |
| warm-up | 4.03 | 229.6 | pipelines/compile_ms 2111, pipelines/front_ms 2051, main/busy/ms 196, stage/main/update 181, render/thread/ms 138, render/busy/ms 136, pipelines/wait_ms 93.68, stage/render/prepare_assets 92.83, pipelines/driver_ms 59.42, stream/drive/ms 33.61, stage/render/render 29.31, render/graph/ms 23.90 |
| warm-up | 4.06 | 25.5 | pipelines/compile_ms 1867, pipelines/front_ms 1812, render/thread/ms 105, render/busy/ms 103, pipelines/wait_ms 95.76, stage/render/prepare_assets 90.09, main/busy/ms 56.82, pipelines/driver_ms 54.55, stage/main/update 33.89, stream/drive/ms 25.57, stage/main/post_update 14.71, stage/render/render 11.43 |
| warm-up | 4.12 | 62.0 | pipelines/compile_ms 186, pipelines/front_ms 172, render/thread/ms 58.94, render/busy/ms 56.86, main/busy/ms 42.50, stage/render/prepare_assets 42.48, stage/main/update 18.16, stage/main/post_update 16.91, pipelines/driver_ms 13.60, stage/render/render 12.49, render/graph/ms 10.23, stream/drive/ms 9.10 |
| warm-up | 4.19 | 43.5 | pipelines/compile_ms 544, pipelines/front_ms 521, main/busy/ms 49.76, render/thread/ms 42.70, render/busy/ms 40.67, pipelines/wait_ms 23.42, pipelines/driver_ms 23.14, stage/render/prepare_assets 19.80, stage/main/post_update 18.54, stage/main/update 13.28, stage/main/pre_update 11.02, stage/render/render 7.71 |
| warm-up | 4.61 | 135.8 | main/busy/ms 120, stage/main/update 92.52, render/thread/ms 75.14, render/busy/ms 72.80, stream/drive/ms 40.18, stage/render/prepare_assets 36.56, encode//sum 24.43, stage/render/render 19.82, stage/main/post_update 18.71, render/graph/ms 17.11, stage/render/graph_submit 10.61, encode/submit 6.48 |
| warm-up | 4.67 | 64.6 | render/thread/ms 110, render/busy/ms 107, main/busy/ms 78.58, stage/render/prepare_assets 51.77, stage/main/update 47.59, encode//sum 30.12, stage/render/render 29.86, render/graph/ms 25.99, stage/main/post_update 20.50, stage/render/graph_submit 16.93, stream/drive/ms 14.31, encode/submit 11.81 |
| warm-up | 4.79 | 119.2 | render/thread/ms 138, render/busy/ms 135, main/busy/ms 51.90, stage/render/prepare_assets 47.60, stage/render/create_views 38.87, encode//sum 30.15, stage/render/render 28.31, stage/main/update 27.74, render/graph/ms 24.65, stage/render/graph_submit 15.78, stream/drive/ms 15.24, stage/main/post_update 13.67 |
| warm-up | 4.86 | 69.2 | render/thread/ms 133, render/busy/ms 132, stage/render/prepare_assets 55.46, main/busy/ms 38.90, stage/render/create_views 38.32, encode//sum 34.90, stage/render/render 22.24, render/graph/ms 19.82, stage/main/update 18.38, stage/render/graph_submit 13.08, stage/main/post_update 10.69, encode/pass:shadow_point_light/sum 9.99 |
| warm-up | 4.91 | 54.9 | render/thread/ms 99.50, render/busy/ms 97.63, encode//sum 79.40, main/busy/ms 68.87, stage/main/update 50.65, stage/render/render 37.70, render/graph/ms 35.17, stage/render/prepare_assets 30.55, stream/drive/ms 27.29, stage/render/graph_submit 24.46, encode/pass:shadow_point_light/sum 20.81, encode/finish_wall 12.34 |
| warm-up | 4.95 | 35.6 | render/thread/ms 82.16, render/busy/ms 80.48, encode//sum 72.53, main/busy/ms 63.72, stage/main/update 44.92, stage/render/render 37.17, render/graph/ms 34.77, stream/drive/ms 27.01, stage/render/graph_submit 23.44, encode/pass:shadow_point_light/sum 21.61, encode/submit 12.26, stage/main/post_update 12.00 |
| warm-up | 4.99 | 36.0 | render/thread/ms 64.37, render/busy/ms 62.80, encode//sum 44.77, main/busy/ms 32.57, stage/render/render 29.18, render/graph/ms 26.91, stage/render/graph_submit 18.11, encode/pass:shadow_point_light/sum 17.02, stage/main/update 16.44, encode/submit 10.93, stage/main/post_update 10.60, stage/render/camera_driver 8.36 |
| warm-up | 5.01 | 27.6 | render/thread/ms 54.75, render/busy/ms 53.11, encode//sum 39.76, main/busy/ms 27.67, stage/render/render 25.00, render/graph/ms 22.73, stage/render/graph_submit 15.38, stage/main/update 15.02, encode/pass:shadow_point_light/sum 14.13, encode/submit 9.02, stage/render/prepare_assets 8.33, stage/main/post_update 7.91 |
| warm-up | 5.06 | 26.5 | render/thread/ms 38.91, render/busy/ms 37.41, main/busy/ms 23.38, stage/render/render 15.78, render/graph/ms 13.54, encode//sum 12.90, stage/main/post_update 10.11, stage/render/prepare_assets 8.73, stage/main/update 8.71, stage/render/graph_submit 7.99, encode/pass:shadow_point_light/sum 5.73, encode/submit 5.16 |
| warm-up | 5.10 | 27.1 | render/thread/ms 33.90, render/busy/ms 32.70, main/busy/ms 20.90, stage/render/render 13.15, stage/render/prepare_assets 12.08, encode//sum 11.52, render/graph/ms 11.50, stage/main/post_update 8.90, stage/main/update 7.99, stage/render/graph_submit 7.69, encode/pass:shadow_point_light/sum 5.94, encode/submit 5.10 |
| warm-up | 5.21 | 49.0 | main/busy/ms 60.27, render/thread/ms 57.49, render/busy/ms 55.49, stage/main/update 37.89, stage/render/render 24.17, render/graph/ms 21.46, stage/main/post_update 17.05, encode//sum 14.96, stage/render/graph_submit 14.40, stage/render/prepare_assets 11.80, encode/submit 11.36, encode/pass:shadow_point_light/sum 6.39 |
| warm-up | 5.26 | 46.1 | render/thread/ms 69.79, render/busy/ms 67.97, main/busy/ms 52.06, stage/main/update 30.91, stage/render/prepare_assets 28.89, stage/render/render 19.88, render/graph/ms 17.28, encode//sum 14.49, stage/main/post_update 13.53, stage/render/graph_submit 10.65, encode/submit 7.88, encode/pass:shadow_point_light/sum 6.84 |
| warm-up | 5.31 | 49.9 | pipelines/compile_ms 2376, pipelines/driver_ms 2162, pipelines/front_ms 214, render/thread/ms 60.99, render/busy/ms 59.63, pipelines/wait_ms 41.62, stage/render/prepare_assets 36.87, main/busy/ms 30.25, stage/main/update 14.92, stage/render/render 13.38, render/graph/ms 11.52, encode//sum 11.04 |
| warm-up | 5.47 | 26.9 | pipelines/compile_ms 4102, pipelines/driver_ms 3537, pipelines/wait_ms 1477, pipelines/front_ms 565, render/thread/ms 41.47, render/busy/ms 39.89, main/busy/ms 29.17, stage/render/render 18.37, render/graph/ms 15.89, stage/main/update 14.30, encode//sum 10.95, stage/main/post_update 10.43 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **87%** (min 53%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.1 W · 72 °C (max 76) · VRAM 5198 MiB · throttled 100% of samples

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
