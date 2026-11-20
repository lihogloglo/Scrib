# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=8 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1775 frames over 11.4 s · **median 6.32 ms (158 fps)** · mean 6.44 · p95 7.64 · p99 8.24 (1% low 121 fps) · worst 19.68 ms · 4 spikes over 1.5× median · 10.5 stutters/min (work 5.2/min)
- warm-up: 120 frames over 5.8 s, worst 2489.6 ms
- hitches (> 25.0 ms): **0** measured, 36 in warm-up
- timed GPU passes: 5.40 ms of a 6.44 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 158.3, target 144
- pass worst frame 19.68 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.052 | 2.161 | 2.492 | 0.95 | 0.167 |
| main_opaque_pass_3d | 2.022 | 2.129 | 3.155 | 0.95 | 0.039 |
| bloom | 0.324 | 0.341 | 0.492 | 0.95 | 0.062 |
| ssao | 0.200 | 0.210 | 0.532 | 0.95 | 0.007 |
| upscaling | 0.159 | 0.167 | 0.187 | 0.95 | 0.004 |
| tlas_build | 0.113 | 0.119 | 0.118 | 0.95 | 0.075 |
| tonemapping | 0.102 | 0.108 | 0.116 | 0.95 | 0.004 |
| gi/probe_update | 0.094 | 0.099 | 0.100 | 0.95 | 0.002 |
| gpu_instance_materialization | 0.087 | 0.092 | 0.098 | 0.95 | 0.004 |
| early prepass | 0.082 | 0.086 | 0.093 | 0.95 | 0.053 |
| clustering | 0.073 | 0.077 | 0.085 | 0.95 | 0.019 |
| point_shadow_atlas | 0.068 | 0.101 | 0.125 | 0.67 | 0.030 |
| uniform_allocation | 0.036 | 0.038 | 0.044 | 0.95 | 0.010 |
| bin_unpacking | 0.031 | 0.032 | 0.037 | 0.95 | 0.008 |
| early_mesh_preprocessing | 0.018 | 0.019 | 0.023 | 0.95 | 0.003 |
| range_unpacking | 0.018 | 0.018 | 0.018 | 0.95 | 0.002 |
| main_transparent_pass_3d | 0.008 | 0.008 | 0.012 | 0.92 | 0.024 |
| setup_tlas_instances | 0.007 | 0.008 | 0.008 | 0.95 | 0.005 |
| point_shadow_atlas/uniform_allocation | 0.006 | 0.008 | 0.008 | 0.67 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.005 | 0.007 | 0.008 | 0.67 | 0.002 |
| point_shadow_atlas/early_mesh_preprocessing | 0.005 | 0.007 | 0.008 | 0.67 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.005 | 0.007 | 0.008 | 0.67 | 0.002 |
| sparse buffer updates | 0.004 | 0.006 | 0.007 | 0.61 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.95 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.95 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.529 | 6.670 | 18.51 | 9813.5 |
| main/busy/ms | 5.064 | 5.874 | 8.22 | 8988.6 |
| render/busy/ms | 4.743 | 5.646 | 9.44 | 8418.5 |
| stage/render/render | 3.275 | 4.081 | 16.19 | 5812.5 |
| render/graph/ms | 2.722 | 3.438 | 6.31 | 4831.0 |
| stage/main/post_update | 2.611 | 3.115 | 5.42 | 4634.0 |
| encode//sum | 2.241 | 3.197 | 12.81 | 3978.1 |
| stage/render/graph_submit | 1.811 | 2.407 | 5.19 | 3215.2 |
| stage/main/update | 1.273 | 1.551 | 2.21 | 2259.9 |
| encode/submit | 1.211 | 1.689 | 2.97 | 2149.9 |
| stage/render/camera_driver | 0.768 | 0.923 | 1.28 | 1363.2 |
| encode/finish_wall | 0.644 | 0.892 | 3.69 | 1142.7 |
| encode/pass:early prepass/sum | 0.450 | 0.626 | 3.01 | 798.9 |
| stage/render/prepare_views | 0.441 | 1.316 | 2.14 | 782.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.427 | 0.617 | 1.06 | 757.3 |
| stage/render/prepare_resources | 0.401 | 0.485 | 0.72 | 711.4 |
| render/acquire/ms | 0.396 | 1.282 | 2.10 | 702.3 |
| render/present/ms | 0.390 | 0.812 | 11.99 | 692.6 |
| stage/main/pre_update | 0.354 | 0.442 | 0.69 | 628.8 |
| stage/main/fixed_main_loop | 0.257 | 0.692 | 1.10 | 455.9 |
| stage/main/first | 0.248 | 0.351 | 1.63 | 439.4 |
| stage/render/prepare_write_phase_buffers | 0.247 | 0.350 | 0.72 | 438.9 |
| bigspace/total/ms | 0.240 | 0.587 | 2.25 | 426.4 |
| stage/render/prepare_bind_groups | 0.219 | 0.297 | 0.62 | 388.4 |
| stage/render/prepare_assets | 0.216 | 0.289 | 0.57 | 382.8 |
| stage/render/create_views | 0.177 | 0.258 | 0.51 | 314.5 |
| stage/render/prepare_flush | 0.174 | 0.289 | 4.09 | 308.7 |
| stage/render/prepare_meshes | 0.171 | 0.233 | 0.52 | 303.5 |
| stage/main/last | 0.152 | 0.195 | 0.42 | 269.1 |
| stage/render/extract_commands | 0.139 | 0.167 | 0.30 | 246.7 |
| stream/drive/ms | 0.132 | 0.192 | 0.42 | 234.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.126 | 0.203 | 0.71 | 223.9 |
| stream/tick/ms | 0.104 | 0.158 | 0.37 | 184.6 |
| bigspace/low_precision/ms | 0.103 | 0.284 | 2.10 | 183.7 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.101 | 0.227 | 0.76 | 179.9 |
| physics/step/ms | 0.101 | 0.315 | 0.61 | 178.6 |
| controller/feed/ms | 0.085 | 0.112 | 0.39 | 150.8 |
| stage/render/prepare_batch_phases | 0.084 | 0.137 | 0.34 | 148.7 |
| anim/evaluate/ms | 0.074 | 0.189 | 2.35 | 131.9 |
| bigspace/high_precision/ms | 0.056 | 0.240 | 0.77 | 99.5 |
| stage/render/phase_sort | 0.051 | 0.085 | 0.33 | 89.7 |
| stage/render/specialize | 0.050 | 0.074 | 0.52 | 88.7 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.074 | 0.30 | 81.8 |
| stage/render/queue | 0.042 | 0.072 | 0.45 | 74.7 |
| encode/resolve_diagnostics/sum | 0.038 | 0.068 | 0.36 | 67.9 |
| anim/evaluate_cpu/ms | 0.037 | 0.056 | 0.21 | 65.1 |
| weather/map/ms | 0.026 | 0.036 | 0.20 | 45.7 |
| controller/write/ms | 0.025 | 0.033 | 0.10 | 43.6 |
| stage/render/cleanup | 0.021 | 0.033 | 0.26 | 37.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.031 | 0.11 | 27.0 |
| encode/camera_driver_span_end/sum | 0.014 | 0.028 | 0.25 | 25.7 |
| weather/eval/ms | 0.012 | 0.016 | 0.12 | 20.6 |
| physics/insert/ms | 0.009 | 0.000 | 0.63 | 15.6 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.008 | 0.000 | 1.54 | 13.7 |
| script/update/ms | 0.007 | 0.010 | 0.15 | 12.7 |
| anim/apply/ms | 0.005 | 0.009 | 0.12 | 8.9 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.6 |
| far/select/ms | 0.004 | 0.005 | 0.05 | 7.0 |
| controller/move/ms | 0.003 | 0.010 | 0.08 | 5.9 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 5.9 |
| controller/avoid/ms | 0.002 | 0.005 | 0.05 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.33 | 2.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.4 |
| weather/apply/ms | 0.001 | 0.001 | 0.03 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.2 |
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
| crowd/joints_extracted/count | count | 46 | 1357 | 0 |
| crowd/joints_uploaded/count | count | 94 | 94 | 0 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.43 | 3.54 | 0.40 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.11 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.25 | 0.01 |
| encode/finish_critical | ms | 0.50 | 3.54 | 0.40 |
| encode/pass:early prepass/max | ms | 0.44 | 3.01 | 0.33 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.42 | 1.06 | 0.34 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 0.71 | 0.08 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.13 | 0.76 | 0.13 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.16 | 1.54 | 1.00 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.36 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.86 | 9.44 | 5.20 |
| gpu/timed/ms | ms | 5.84 | 7.41 | 5.20 |
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
| phases/opaque3d/bins | count | 159 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 261 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 6 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 159 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 261 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 85 | 18 |
| phases/shadow/bins | count | 0 | 1004 | 225 |
| phases/shadow/instance_batches | count | 0 | 12 | 0 |
| phases/shadow/instance_draws | count | 0 | 1062 | 0 |
| phases/shadow/instance_slots | count | 0 | 49.2 k | 0 |
| phases/shadow/items | count | 0 | 1455 | 301 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 0 | 80 | 18 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 90 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 298 | 298 | 298 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 4 | 106 | 4 |
| preprocess/bind_groups_created | count | 0 | 102 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 10 | 4 |
| preprocess/bind_groups_reused | count | 47 | 59 | 35 |
| process/cpu_usage | % | 30.77 | 42.86 | 26.44 |
| process/mem_usage | GiB | 3.69 | 3.72 | 3.72 |
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
| shadow_atlas/alloc_failures | count | 3834 | 3834 | 3834 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 8 | 9 | 8 |
| shadow_atlas/live_occ_pct | % | 37.50 | 100 | 0 |
| shadow_atlas/live_tiles | count | 6 | 16 | 0 |
| shadow_atlas/mover_draws | count | 6 | 12 | 0 |
| shadow_atlas/movers | count | 200 | 459 | 56 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 5 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 6 | 0 |
| shadow_atlas/render/lamps | count | 8 | 9 | 8 |
| shadow_atlas/render/live_direct | count | 5 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 8 | 31 | 0 |
| shadow_atlas/resizes | count | 0 | 1 | 1 |
| shadow_atlas/static_occ_pct | % | 75 | 84.38 | 60.94 |
| shadow_atlas/static_tiles | count | 48 | 54 | 48 |
| shadow_atlas/views | count | 8 | 31 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 1 | 23 | 0 |
| skin/blocks_skipped/count | count | 1 | 2 | 0 |
| skin/blocks_walked/count | count | 1 | 2 | 0 |
| skin/joint_bytes_written/bytes | bytes | 3072 | 7680 | 0 |
| skin/joints_written/count | count | 48 | 120 | 0 |
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
| stream/latency/s | s | 0.06 | 0.06 | 0.06 |
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
| system/cpu_usage | % | 37.67 | 46.21 | 32.25 |
| system/mem_usage | % | 50.68 | 50.76 | 50.76 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 23 | 114 | 17 |
| views/all/bins | count | 334 | 1538 | 58 |
| views/all/count | count | 9 | 32 | 1 |
| views/all/instance_batches | count | 8 | 20 | 8 |
| views/all/instance_draws | count | 3220 | 4282 | 3220 |
| views/all/instance_slots | count | 98.3 k | 147.5 k | 98.3 k |
| views/all/items | count | 588 | 2465 | 86 |
| views/all/pipelines | count | 25 | 107 | 16 |
| views/all/pipelines_distinct | count | 25 | 34 | 16 |
| views/all/work_items | count | 98.9 k | 148.3 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 85 | 0 |
| views/lamp_face/bins | count | 0 | 1004 | 0 |
| views/lamp_face/count | count | 8 | 31 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 12 | 0 |
| views/lamp_face/instance_draws | count | 0 | 1062 | 0 |
| views/lamp_face/instance_slots | count | 0 | 49.2 k | 0 |
| views/lamp_face/items | count | 0 | 1455 | 0 |
| views/lamp_face/pipelines | count | 0 | 80 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 49.6 k | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 328 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 544 | 1062 | 86 |
| views/main/pipelines | count | 21 | 27 | 16 |
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
| anim/bones_written/count | count | 9.09 | 30 | 16.1 k |
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
| ecs/entities_delta | count | 0 | 0 | 0 |
| encode//count | count | 19.64 | 20 | 34.9 k |
| encode/bind_walk/init_textures | count | 4185 | 12.6 k | 7.43 M |
| encode/bind_walk/init_textures_skipped | count | 6981 | 24.0 k | 12.39 M |
| encode/bind_walk/inits | count | 157 | 277 | 278.5 k |
| encode/bind_walk/inits_skipped | count | 14.99 | 32 | 26.6 k |
| encode/bind_walk/merge_textures | count | 2942 | 11.4 k | 5.22 M |
| encode/bind_walk/merge_textures_skipped | count | 67.89 | 12.7 k | 120.5 k |
| encode/bind_walk/merges | count | 42.84 | 73 | 76.0 k |
| encode/bind_walk/merges_skipped | count | 10.17 | 39 | 18.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1775 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1775 |
| encode/pass:early prepass/count | count | 1 | 1 | 1775 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1775 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1724 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.69 | 1 | 1232 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.01 | 1 | 13 |
| encode/queue/command_buffers | count | 29.31 | 30 | 52.0 k |
| encode/queue/submits | count | 2 | 2 | 3550 |
| encode/queue/write_buffer/bytes | bytes | 533.5 k | 679.9 k | 946.95 M |
| encode/queue/write_buffer/calls | count | 99.73 | 201 | 177.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 118.7 k | 118.7 k | 210.66 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 10.7 k |
| encode/queue/write_texture/bytes | bytes | 591 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1775 |
| physics/inserts | count | 0.05 | 11 | 91 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.39 | 2 | 686 |
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

## Hitches (> 25.0 ms; slowest 36, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2489.6 | main/busy/ms 2461, stage/main/post_update 2344, stage/main/update 107, surface/advance_cpu_ms 77.21, stage/main/pre_update 6.40, stage/main/last 1.10, stage/main/first 0.58, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.39, weather/map/ms 0.17, bigspace/total/ms 0.07, stream/tick/ms 0.07 |
| warm-up | 3.22 | 729.6 | main/busy/ms 2577, stage/main/post_update 2349, render/thread/ms 692, render/busy/ms 573, stage/render/prepare_assets 529, stage/main/update 213, stage/render/prepare_views 117, render/acquire/ms 116, surface/advance_cpu_ms 77.21, stage/render/create_views 32.17, stage/render/render 8.60, stage/main/pre_update 7.79 |
| warm-up | 3.27 | 50.2 | render/thread/ms 727, render/busy/ms 607, stage/render/prepare_assets 558, main/busy/ms 129, stage/render/prepare_views 117, render/acquire/ms 117, stage/main/update 110, stage/render/create_views 32.47, stage/render/render 12.56, render/graph/ms 8.88, stage/main/post_update 8.11, stage/main/fixed_main_loop 7.33 |
| warm-up | 3.30 | 34.3 | pipelines/wait_ms 222, pipelines/compile_ms 177, pipelines/front_ms 164, render/thread/ms 62.99, render/busy/ms 59.26, main/busy/ms 52.94, stage/main/post_update 38.49, stage/render/prepare_assets 29.82, stage/render/render 28.33, render/graph/ms 24.45, pipelines/driver_ms 12.85, stage/render/graph_submit 6.77 |
| warm-up | 3.35 | 44.2 | pipelines/wait_ms 1373, pipelines/compile_ms 529, pipelines/front_ms 483, render/thread/ms 59.99, render/busy/ms 56.08, main/busy/ms 47.59, pipelines/driver_ms 45.91, stage/main/post_update 37.46, stage/render/render 35.76, render/graph/ms 31.71, stage/render/prepare_assets 16.71, stage/render/graph_submit 8.74 |
| warm-up | 3.38 | 28.6 | pipelines/wait_ms 1155, pipelines/compile_ms 439, pipelines/front_ms 402, render/thread/ms 58.45, render/busy/ms 56.34, pipelines/driver_ms 36.82, main/busy/ms 33.52, stage/render/prepare_assets 24.85, stage/render/render 18.26, render/graph/ms 16.26, stage/main/post_update 10.75, stage/render/camera_driver 9.21 |
| warm-up | 3.41 | 30.3 | pipelines/compile_ms 269, pipelines/front_ms 259, render/thread/ms 54.21, render/busy/ms 51.99, main/busy/ms 41.92, stage/render/prepare_assets 23.08, stage/render/render 17.49, render/graph/ms 15.73, stage/main/post_update 14.78, stage/render/graph_submit 10.77, stage/main/update 10.59, pipelines/driver_ms 10.31 |
| warm-up | 3.46 | 35.5 | pipelines/compile_ms 524, pipelines/front_ms 508, pipelines/wait_ms 46.15, main/busy/ms 43.86, render/thread/ms 35.39, render/busy/ms 33.21, pipelines/driver_ms 15.98, stage/main/post_update 15.11, stage/render/render 14.90, stage/render/prepare_assets 13.41, render/graph/ms 13.04, stage/main/pre_update 12.93 |
| warm-up | 3.61 | 89.6 | pipelines/compile_ms 1364, pipelines/front_ms 1317, main/busy/ms 76.86, render/thread/ms 62.18, render/busy/ms 60.44, stage/main/update 57.22, pipelines/driver_ms 46.59, stage/render/prepare_assets 43.53, stage/render/render 9.63, stage/main/post_update 9.10, render/graph/ms 7.74, stage/main/pre_update 7.26 |
| warm-up | 3.77 | 107.4 | pipelines/compile_ms 1428, pipelines/front_ms 1362, main/busy/ms 200, pipelines/wait_ms 163, stage/main/update 157, pipelines/driver_ms 65.40, render/thread/ms 47.94, render/busy/ms 46.29, stage/render/prepare_assets 35.18, stage/main/post_update 32.07, stream/drive/ms 23.13, stage/main/pre_update 7.46 |
| warm-up | 3.99 | 224.5 | pipelines/compile_ms 2545, pipelines/front_ms 2446, pipelines/wait_ms 532, main/busy/ms 147, stage/main/update 133, render/thread/ms 110, render/busy/ms 108, pipelines/driver_ms 98.94, stage/render/prepare_assets 98.23, pipelines/lock_ms 7.54, stage/main/post_update 6.64, stage/render/render 6.41 |
| warm-up | 4.08 | 90.9 | pipelines/compile_ms 1490, pipelines/front_ms 1441, pipelines/wait_ms 375, render/thread/ms 140, render/busy/ms 138, stage/render/prepare_assets 93.15, pipelines/driver_ms 49.28, stage/render/render 44.00, render/graph/ms 41.88, stage/render/graph_submit 40.36, encode//sum 29.31, encode/finish_wall 27.66 |
| warm-up | 4.13 | 40.8 | pipelines/compile_ms 665, pipelines/front_ms 637, render/thread/ms 98.38, render/busy/ms 96.75, main/busy/ms 52.40, stage/render/prepare_assets 49.79, stage/render/render 45.25, render/graph/ms 43.41, stage/render/graph_submit 41.80, stage/main/update 37.87, stream/drive/ms 31.96, encode//sum 29.53 |
| warm-up | 4.16 | 32.9 | pipelines/compile_ms 1120, pipelines/front_ms 1077, main/busy/ms 55.93, render/thread/ms 44.20, pipelines/driver_ms 43.12, render/busy/ms 43.02, stage/main/update 37.80, stream/drive/ms 32.14, stage/render/prepare_assets 30.24, stage/main/post_update 10.23, stage/render/render 7.19, render/graph/ms 5.82 |
| warm-up | 4.20 | 35.3 | pipelines/compile_ms 315, pipelines/front_ms 303, pipelines/wait_ms 56.27, main/busy/ms 44.04, render/thread/ms 24.84, render/busy/ms 22.92, stage/main/post_update 20.33, stage/main/update 17.43, stage/render/render 14.94, render/graph/ms 12.92, pipelines/driver_ms 12.00, stream/drive/ms 10.92 |
| warm-up | 4.24 | 37.2 | pipelines/compile_ms 649, pipelines/front_ms 627, pipelines/wait_ms 71.60, render/thread/ms 39.02, render/busy/ms 37.39, main/busy/ms 30.77, pipelines/driver_ms 22.73, stage/render/prepare_assets 21.46, stage/main/post_update 18.20, stage/render/render 13.38, render/graph/ms 11.61, stage/main/update 8.50 |
| warm-up | 4.30 | 29.0 | pipelines/compile_ms 396, pipelines/front_ms 382, pipelines/wait_ms 40.73, render/thread/ms 37.82, render/busy/ms 36.66, main/busy/ms 21.98, stage/render/prepare_assets 15.82, pipelines/driver_ms 14.10, stage/main/update 10.95, stage/main/post_update 6.92, stage/render/render 6.74, render/graph/ms 5.38 |
| warm-up | 4.49 | 25.1 | pipelines/compile_ms 453, pipelines/front_ms 438, render/thread/ms 33.38, main/busy/ms 33.10, render/busy/ms 28.27, stage/main/post_update 16.53, pipelines/driver_ms 14.54, stage/render/render 12.88, stage/main/update 12.24, stage/render/prepare_resources 10.73, pipelines/wait_ms 8.59, render/graph/ms 7.37 |
| warm-up | 4.51 | 26.0 | pipelines/compile_ms 273, pipelines/front_ms 261, main/busy/ms 71.04, render/thread/ms 47.96, render/busy/ms 42.89, stage/main/update 39.42, pipelines/wait_ms 30.88, stage/main/post_update 23.54, stage/render/prepare_bind_groups 17.22, stage/render/render 16.66, pipelines/driver_ms 12.04, stage/render/prepare_resources 10.73 |
| warm-up | 4.56 | 48.3 | pipelines/compile_ms 858, pipelines/front_ms 823, pipelines/wait_ms 73.11, main/busy/ms 70.24, render/thread/ms 43.57, render/busy/ms 42.11, pipelines/driver_ms 34.37, stage/main/update 33.80, stage/main/post_update 21.79, stage/render/prepare_bind_groups 18.41, stage/render/render 11.54, stage/render/prepare_assets 7.97 |
| warm-up | 4.59 | 26.7 | pipelines/compile_ms 1049, pipelines/front_ms 1010, pipelines/wait_ms 67.32, main/busy/ms 49.29, render/thread/ms 39.57, pipelines/driver_ms 39.42, render/busy/ms 38.03, stage/main/post_update 16.31, stage/main/pre_update 11.64, stage/render/prepare_assets 10.58, stage/main/update 9.60, stage/render/render 9.32 |
| warm-up | 4.62 | 37.0 | pipelines/compile_ms 774, pipelines/front_ms 747, main/busy/ms 64.57, pipelines/wait_ms 52.24, render/thread/ms 50.75, render/busy/ms 49.29, stage/main/update 36.96, pipelines/driver_ms 27.51, stage/render/render 21.21, render/graph/ms 18.13, stage/main/pre_update 11.30, stage/main/post_update 11.06 |
| warm-up | 4.68 | 54.9 | pipelines/compile_ms 1013, pipelines/front_ms 983, main/busy/ms 90.17, stage/main/update 74.66, render/thread/ms 65.36, render/busy/ms 63.03, pipelines/wait_ms 55.69, stream/drive/ms 40.47, pipelines/driver_ms 30.03, stage/render/render 26.57, render/graph/ms 23.00, stage/render/prepare_assets 16.20 |
| warm-up | 4.74 | 62.9 | pipelines/compile_ms 751, pipelines/front_ms 724, main/busy/ms 85.48, render/thread/ms 69.51, render/busy/ms 66.93, stage/main/update 61.88, stream/drive/ms 41.45, stage/render/prepare_assets 27.78, pipelines/driver_ms 27.46, pipelines/wait_ms 24.49, stage/render/render 19.83, stage/main/post_update 17.34 |
| warm-up | 4.79 | 46.6 | pipelines/compile_ms 388, pipelines/front_ms 369, render/thread/ms 71.56, main/busy/ms 71.00, render/busy/ms 69.61, stage/main/update 40.62, stage/render/prepare_assets 32.19, stage/main/post_update 21.12, pipelines/driver_ms 19.11, stage/render/render 18.45, render/graph/ms 16.03, stream/drive/ms 12.81 |
| warm-up | 4.88 | 90.2 | pipelines/compile_ms 114, pipelines/front_ms 110, render/thread/ms 94.96, render/busy/ms 93.53, main/busy/ms 52.64, stage/render/prepare_assets 44.82, stage/render/render 36.44, render/graph/ms 34.42, stage/render/graph_submit 30.72, stage/main/update 30.17, encode//sum 29.73, encode/finish_wall 26.55 |
| warm-up | 4.92 | 44.0 | render/thread/ms 86.59, render/busy/ms 85.55, stage/render/prepare_assets 47.12, main/busy/ms 33.21, stage/render/render 31.27, render/graph/ms 29.84, encode//sum 28.03, stage/render/graph_submit 27.50, encode/finish_wall 25.40, encode/pass:main_transparent_pass_3d/sum 23.67, stage/main/update 17.09, stage/main/post_update 8.65 |
| warm-up | 4.95 | 29.5 | main/busy/ms 59.90, render/thread/ms 54.04, render/busy/ms 46.43, stage/main/update 43.88, stream/drive/ms 27.60, stage/render/prepare_assets 22.95, stage/render/render 20.89, render/graph/ms 13.00, stage/render/graph_submit 9.53, stage/main/post_update 9.13, render/present/ms 7.43, encode//sum 6.81 |
| warm-up | 5.02 | 63.6 | pipelines/compile_ms 82.98, pipelines/front_ms 77.66, render/thread/ms 71.70, render/busy/ms 64.01, main/busy/ms 63.63, stage/main/update 40.66, stream/drive/ms 28.49, stage/render/render 22.62, stage/main/post_update 18.69, render/graph/ms 14.57, stage/render/prepare_write_phase_buffers 13.89, stage/render/prepare_assets 10.85 |
| warm-up | 5.04 | 26.2 | pipelines/compile_ms 269, pipelines/front_ms 254, render/thread/ms 69.35, render/busy/ms 68.09, main/busy/ms 37.17, stage/main/post_update 17.83, pipelines/driver_ms 15.04, stage/render/prepare_write_phase_buffers 14.23, stage/render/render 13.74, stage/main/update 11.88, render/graph/ms 11.85, stage/render/prepare_assets 11.59 |
| warm-up | 5.10 | 29.0 | pipelines/compile_ms 1166, pipelines/front_ms 1126, render/thread/ms 46.67, render/busy/ms 45.19, pipelines/driver_ms 40.50, main/busy/ms 32.78, stage/render/render 23.79, render/graph/ms 21.57, stage/main/update 16.04, stage/render/graph_submit 15.81, stage/main/post_update 12.10, encode/finish_wall 11.77 |
| warm-up | 5.13 | 35.3 | pipelines/compile_ms 133, pipelines/front_ms 129, render/thread/ms 55.62, render/busy/ms 54.19, main/busy/ms 34.08, stage/main/update 18.66, stage/render/render 17.13, stage/render/prepare_assets 17.03, render/graph/ms 15.17, stage/render/graph_submit 12.04, stage/main/post_update 11.28, encode/finish_wall 8.29 |
| warm-up | 5.17 | 41.0 | pipelines/compile_ms 326, pipelines/front_ms 315, render/thread/ms 69.02, render/busy/ms 67.54, main/busy/ms 41.18, stage/render/prepare_assets 35.73, stage/main/update 18.88, stage/main/post_update 16.47, stage/render/render 13.51, pipelines/driver_ms 11.78, render/graph/ms 11.54, stage/render/graph_submit 8.36 |
| warm-up | 5.28 | 34.1 | render/thread/ms 42.68, render/busy/ms 41.05, main/busy/ms 40.21, stage/main/update 23.64, stage/render/prepare_assets 19.34, stage/render/render 12.10, stage/main/post_update 12.07, render/graph/ms 9.87, encode//sum 7.42, stage/render/graph_submit 6.28, encode/submit 4.42, stage/render/prepare_meshes 3.74 |
| warm-up | 5.31 | 30.0 | render/thread/ms 48.59, render/busy/ms 46.93, main/busy/ms 41.53, stage/main/update 25.71, stage/render/prepare_assets 24.46, stage/render/render 12.32, render/graph/ms 10.08, stage/main/post_update 9.79, encode//sum 7.93, stage/render/graph_submit 6.63, encode/submit 4.64, stage/render/prepare_meshes 3.75 |
| warm-up | 5.34 | 33.9 | render/thread/ms 38.10, render/busy/ms 37.13, stage/render/prepare_assets 24.35, main/busy/ms 22.63, stage/main/update 10.45, stage/render/render 6.62, stage/main/post_update 5.54, render/graph/ms 5.40, encode//sum 4.72, stage/render/graph_submit 3.52, stream/drive/ms 2.91, stage/main/pre_update 2.82 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=8 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=atlas8-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **97%** (min 90%) · 2625 MHz (min 2625) · memory 8101 MHz · 100.9 W · 75 °C (max 78) · VRAM 5160 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.enabled=on (command line: --set)`
- `shadow.lamp.atlas.max_lamps=8 (command line: --set)`
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
