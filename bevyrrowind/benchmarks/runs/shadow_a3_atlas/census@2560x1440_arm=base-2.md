# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1538 frames over 11.4 s · **median 6.90 ms (145 fps)** · mean 7.43 · p95 10.47 · p99 11.79 (1% low 85 fps) · worst 13.65 ms · 93 spikes over 1.5× median · 31.5 stutters/min (work 0.0/min)
- warm-up: 120 frames over 6.7 s, worst 2566.5 ms
- hitches (> 25.0 ms): **0** measured, 48 in warm-up
- timed GPU passes: 5.30 ms of a 7.43 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 144.9, target 144
- pass worst frame 13.65 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.979 | 2.159 | 2.534 | 0.92 | 0.169 |
| main_opaque_pass_3d | 1.784 | 1.946 | 2.845 | 0.92 | 0.039 |
| auxiliary_views | 0.318 | 0.347 | 1.108 | 0.92 | 0.293 |
| bloom | 0.317 | 0.346 | 0.504 | 0.92 | 0.065 |
| ssao | 0.201 | 0.219 | 0.548 | 0.92 | 0.007 |
| upscaling | 0.153 | 0.167 | 0.188 | 0.92 | 0.004 |
| tlas_build | 0.104 | 0.114 | 0.118 | 0.92 | 0.078 |
| tonemapping | 0.099 | 0.108 | 0.116 | 0.92 | 0.004 |
| gpu_instance_materialization | 0.088 | 0.096 | 0.098 | 0.92 | 0.004 |
| gi/probe_update | 0.088 | 0.096 | 0.098 | 0.92 | 0.002 |
| early prepass | 0.072 | 0.079 | 0.086 | 0.92 | 0.047 |
| clustering | 0.067 | 0.073 | 0.076 | 0.92 | 0.019 |
| uniform_allocation | 0.034 | 0.037 | 0.044 | 0.92 | 0.010 |
| bin_unpacking | 0.028 | 0.031 | 0.040 | 0.92 | 0.008 |
| range_unpacking | 0.018 | 0.019 | 0.019 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.016 | 0.018 | 0.025 | 0.92 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.89 | 0.024 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.92 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.29 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.92 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 11.621 | 0.000 | 2649.27 | 17872.9 |
| pipelines/driver_ms | 9.623 | 0.000 | 2270.60 | 14799.9 |
| render/thread/ms | 6.293 | 9.262 | 12.20 | 9678.6 |
| render/busy/ms | 5.819 | 8.756 | 11.60 | 8949.3 |
| main/busy/ms | 5.564 | 6.653 | 8.97 | 8557.5 |
| stage/render/render | 3.990 | 6.044 | 8.10 | 6135.9 |
| render/graph/ms | 3.354 | 5.380 | 7.28 | 5158.7 |
| stage/main/post_update | 2.945 | 3.689 | 4.76 | 4529.2 |
| encode//sum | 2.636 | 4.358 | 6.42 | 4053.9 |
| pipelines/wait_ms | 2.337 | 0.000 | 1850.65 | 3594.2 |
| stage/render/graph_submit | 2.187 | 3.584 | 4.87 | 3363.4 |
| pipelines/front_ms | 1.998 | 0.000 | 378.67 | 3073.0 |
| encode/submit | 1.509 | 2.478 | 3.65 | 2320.5 |
| stage/main/update | 1.315 | 1.658 | 3.25 | 2022.1 |
| stage/render/camera_driver | 1.025 | 1.700 | 2.35 | 1576.6 |
| encode/pass:shadow_point_light/sum | 0.918 | 2.975 | 4.53 | 1411.6 |
| encode/finish_wall | 0.717 | 1.119 | 1.66 | 1102.5 |
| stage/render/prepare_resources | 0.425 | 0.535 | 0.76 | 654.1 |
| stage/render/prepare_write_phase_buffers | 0.418 | 0.842 | 2.45 | 643.0 |
| encode/pass:early prepass/sum | 0.405 | 0.578 | 0.97 | 622.3 |
| render/present/ms | 0.390 | 0.774 | 1.78 | 599.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.379 | 0.556 | 0.86 | 582.5 |
| stage/main/pre_update | 0.354 | 0.447 | 0.92 | 545.2 |
| stage/main/first | 0.348 | 0.440 | 1.58 | 535.2 |
| stage/render/prepare_bind_groups | 0.336 | 0.713 | 1.33 | 517.4 |
| stage/main/fixed_main_loop | 0.270 | 0.631 | 1.01 | 414.7 |
| bigspace/total/ms | 0.231 | 0.562 | 1.11 | 355.9 |
| stage/render/prepare_assets | 0.217 | 0.286 | 0.51 | 334.1 |
| stage/render/prepare_meshes | 0.210 | 0.287 | 0.55 | 323.1 |
| stage/render/prepare_flush | 0.181 | 0.341 | 0.73 | 277.6 |
| stage/render/create_views | 0.174 | 0.222 | 0.57 | 267.6 |
| stage/main/last | 0.158 | 0.221 | 1.19 | 242.7 |
| stage/render/extract_commands | 0.141 | 0.175 | 0.46 | 216.6 |
| stage/render/prepare_views | 0.129 | 0.183 | 0.75 | 199.0 |
| stream/drive/ms | 0.129 | 0.183 | 0.47 | 198.4 |
| stage/render/specialize | 0.114 | 0.167 | 0.72 | 174.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.111 | 0.164 | 0.33 | 171.4 |
| stream/tick/ms | 0.102 | 0.152 | 0.43 | 156.9 |
| bigspace/low_precision/ms | 0.096 | 0.261 | 0.62 | 148.0 |
| stage/render/prepare_batch_phases | 0.090 | 0.148 | 0.35 | 139.1 |
| physics/step/ms | 0.084 | 0.262 | 0.57 | 129.9 |
| render/acquire/ms | 0.084 | 0.119 | 0.72 | 129.8 |
| controller/feed/ms | 0.084 | 0.108 | 0.41 | 129.2 |
| anim/evaluate/ms | 0.068 | 0.159 | 0.69 | 104.3 |
| stage/render/phase_sort | 0.058 | 0.092 | 0.35 | 88.5 |
| bigspace/high_precision/ms | 0.056 | 0.230 | 0.65 | 85.4 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.074 | 0.34 | 74.4 |
| stage/render/queue | 0.043 | 0.078 | 0.41 | 66.4 |
| anim/evaluate_cpu/ms | 0.036 | 0.055 | 0.34 | 55.3 |
| physics/insert/ms | 0.034 | 0.374 | 1.50 | 52.6 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.26 | 49.7 |
| controller/write/ms | 0.027 | 0.038 | 0.16 | 41.2 |
| weather/map/ms | 0.027 | 0.037 | 0.23 | 40.8 |
| stage/render/cleanup | 0.022 | 0.034 | 0.18 | 34.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.08 | 19.3 |
| weather/eval/ms | 0.012 | 0.018 | 0.05 | 19.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.018 | 0.28 | 16.2 |
| script/update/ms | 0.008 | 0.012 | 0.25 | 12.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.9 |
| anim/apply/ms | 0.005 | 0.010 | 0.03 | 7.4 |
| controller/move/ms | 0.004 | 0.011 | 0.06 | 6.2 |
| far/select/ms | 0.004 | 0.005 | 0.03 | 6.1 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 5.1 |
| controller/avoid/ms | 0.002 | 0.005 | 0.04 | 3.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.07 | 1.9 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.9 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.30 | 0.6 |
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
| crowd/joints_extracted/count | count | 46 | 1357 | 0 |
| crowd/joints_uploaded/count | count | 94 | 153 | 94 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.41 | 1.26 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.08 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.01 |
| encode/finish_critical | ms | 0.50 | 1.26 | 0.44 |
| encode/pass:early prepass/max | ms | 0.40 | 0.97 | 0.39 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.37 | 0.86 | 0.21 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.33 | 0.09 |
| encode/pass:shadow_point_light/max | ms | 0.47 | 0.94 | 0.41 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.26 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.79 | 11.60 | 5.23 |
| gpu/timed/ms | ms | 5.59 | 7.99 | 4.70 |
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
| phases/shadow/bins | count | 2638 | 2864 | 2173 |
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
| physics/pending | count | 0 | 2254 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 272 | 292 | 292 |
| pipelines/waiting | count | 1 | 7 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 1 | 6 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 14 | 31 | 0 |
| preprocess/bind_groups | count | 7 | 200 | 4 |
| preprocess/bind_groups_created | count | 3 | 196 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 47 | 94 | 35 |
| process/cpu_usage | % | 30.14 | 45.41 | 28.23 |
| process/mem_usage | GiB | 3.98 | 4.07 | 4.07 |
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
| stream/latency/s | s | 0.14 | 0.14 | 0.14 |
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
| system/cpu_usage | % | 38.40 | 58.28 | 36.65 |
| system/mem_usage | % | 51.50 | 51.83 | 51.83 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 26 | 112 | 17 |
| views/all/bins | count | 2981 | 3313 | 2231 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 4165 | 4584 | 3191 |
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
| views/lamp_face/bins | count | 2638 | 2864 | 2173 |
| views/lamp_face/count | count | 48 | 48 | 48 |
| views/lamp_face/culled | count | 25 | 36 | 35 |
| views/lamp_face/drawn | count | 0 | 12 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3600 | 3839 | 3105 |
| views/lamp_face/pipelines | count | 227 | 240 | 189 |
| views/lamp_face/revealed | count | 0 | 1 | 0 |
| views/lamp_face/work_items | count | 0 | 1299 | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 294 | 582 | 58 |
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
| anim/bones_written/count | count | 8.18 | 30 | 12.6 k |
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
| encode//count | count | 31.08 | 68 | 47.8 k |
| encode/bind_walk/init_textures | count | 8189 | 21.0 k | 12.59 M |
| encode/bind_walk/init_textures_skipped | count | 10.5 k | 28.2 k | 16.10 M |
| encode/bind_walk/inits | count | 214 | 439 | 329.8 k |
| encode/bind_walk/inits_skipped | count | 15.40 | 42 | 23.7 k |
| encode/bind_walk/merge_textures | count | 6946 | 19.8 k | 10.68 M |
| encode/bind_walk/merge_textures_skipped | count | 3837 | 16.9 k | 5.90 M |
| encode/bind_walk/merges | count | 58.58 | 118 | 90.1 k |
| encode/bind_walk/merges_skipped | count | 16.43 | 58 | 25.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1538 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1538 |
| encode/pass:early prepass/count | count | 1 | 1 | 1538 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1538 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1490 |
| encode/pass:shadow_point_light/count | count | 2.94 | 12 | 4523 |
| encode/queue/command_buffers | count | 42.99 | 89 | 66.1 k |
| encode/queue/submits | count | 2 | 2 | 3076 |
| encode/queue/write_buffer/bytes | bytes | 596.7 k | 685.4 k | 917.75 M |
| encode/queue/write_buffer/calls | count | 157 | 372 | 242.2 k |
| encode/queue/write_buffer_with/bytes | bytes | 149.6 k | 149.6 k | 230.13 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 9228 |
| encode/queue/write_texture/bytes | bytes | 682 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1538 |
| physics/inserts | count | 0.89 | 76 | 1373 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.45 | 1 | 687 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.03 | 1 | 48 |
| pipelines/created | count | 0.03 | 1 | 53 |
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

## Hitches (> 25.0 ms; slowest 48, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2566.5 | main/busy/ms 2527, stage/main/post_update 2423, stage/main/update 94.59, surface/advance_cpu_ms 66.15, stage/main/pre_update 6.12, stage/main/last 1.22, stage/main/first 0.48, stream/drive/ms 0.47, stage/main/fixed_main_loop 0.41, weather/map/ms 0.12, stream/tick/ms 0.08, bigspace/total/ms 0.07 |
| warm-up | 3.25 | 681.1 | main/busy/ms 2645, stage/main/post_update 2428, render/thread/ms 634, render/busy/ms 540, stage/render/prepare_assets 517, stage/main/update 200, stage/render/prepare_views 91.93, render/acquire/ms 90.93, surface/advance_cpu_ms 66.15, stage/render/prepare_resources 12.65, stage/render/render 10.22, stage/main/pre_update 7.52 |
| warm-up | 3.30 | 48.3 | render/thread/ms 666, render/busy/ms 572, stage/render/prepare_assets 543, main/busy/ms 131, stage/main/update 111, stage/render/prepare_views 92.21, render/acquire/ms 91.14, stage/render/render 14.03, stage/render/prepare_resources 13.20, render/graph/ms 9.57, stage/main/post_update 8.72, encode//sum 7.31 |
| warm-up | 3.33 | 33.4 | pipelines/wait_ms 225, pipelines/compile_ms 144, pipelines/front_ms 132, main/busy/ms 58.62, render/thread/ms 58.37, render/busy/ms 43.19, stage/main/post_update 36.44, stage/render/prepare_assets 28.62, stage/render/render 26.26, render/present/ms 14.85, pipelines/driver_ms 12.54, render/graph/ms 11.04 |
| warm-up | 3.36 | 26.0 | pipelines/wait_ms 1297, pipelines/compile_ms 483, pipelines/front_ms 449, main/busy/ms 63.34, stage/main/post_update 41.47, render/thread/ms 35.93, pipelines/driver_ms 34.00, stage/render/render 26.62, render/busy/ms 20.89, pipelines/lock_ms 16.08, render/present/ms 14.83, render/graph/ms 11.48 |
| warm-up | 3.38 | 26.8 | pipelines/wait_ms 1106, pipelines/compile_ms 452, pipelines/front_ms 425, main/busy/ms 33.96, pipelines/driver_ms 27.09, render/thread/ms 26.80, render/busy/ms 25.05, stage/main/post_update 18.84, stage/render/prepare_assets 10.38, stage/main/update 8.72, stage/render/render 8.44, pipelines/lock_ms 7.36 |
| warm-up | 3.44 | 33.5 | pipelines/compile_ms 227, pipelines/front_ms 218, render/thread/ms 46.76, main/busy/ms 46.38, render/busy/ms 44.87, stage/main/fixed_main_loop 27.06, stage/render/prepare_resources 21.13, stage/render/prepare_assets 11.05, pipelines/driver_ms 8.90, stage/render/render 8.30, stage/main/post_update 8.04, render/graph/ms 6.45 |
| warm-up | 3.51 | 31.3 | pipelines/compile_ms 930, pipelines/front_ms 901, pipelines/wait_ms 113, main/busy/ms 73.46, stage/main/update 63.10, render/thread/ms 46.70, render/busy/ms 41.46, pipelines/driver_ms 28.82, stage/render/prepare_assets 25.37, stage/render/render 9.66, render/graph/ms 7.60, stage/main/post_update 5.55 |
| warm-up | 3.61 | 98.8 | pipelines/compile_ms 1306, pipelines/front_ms 1260, render/thread/ms 83.78, render/busy/ms 81.79, main/busy/ms 76.37, stage/render/prepare_assets 66.02, stage/main/update 62.93, pipelines/driver_ms 46.05, stage/render/render 10.22, stage/main/post_update 8.58, render/graph/ms 8.01, stage/render/graph_submit 5.45 |
| warm-up | 3.64 | 27.2 | pipelines/compile_ms 1102, pipelines/front_ms 1063, render/thread/ms 83.80, render/busy/ms 81.29, stage/render/prepare_assets 62.82, main/busy/ms 44.86, pipelines/driver_ms 38.90, stage/main/update 29.91, stream/drive/ms 16.22, stage/render/render 15.56, render/graph/ms 12.83, stage/main/post_update 9.30 |
| warm-up | 3.69 | 48.1 | pipelines/compile_ms 266, pipelines/front_ms 256, render/thread/ms 55.61, render/busy/ms 53.52, main/busy/ms 44.49, stage/render/prepare_assets 38.51, stage/main/update 30.48, stream/drive/ms 17.42, stage/render/render 12.63, render/graph/ms 10.36, pipelines/driver_ms 9.84, stage/main/post_update 7.62 |
| warm-up | 3.72 | 34.4 | pipelines/compile_ms 142, pipelines/front_ms 135, render/thread/ms 60.61, render/busy/ms 54.96, main/busy/ms 49.52, stage/render/prepare_assets 38.26, stage/main/update 14.93, stage/render/render 12.69, stage/main/post_update 12.58, stage/main/pre_update 10.63, render/graph/ms 6.75, pipelines/driver_ms 6.41 |
| warm-up | 3.76 | 38.0 | pipelines/compile_ms 209, pipelines/front_ms 200, main/busy/ms 174, stage/main/update 127, render/thread/ms 52.31, render/busy/ms 46.13, stage/render/prepare_assets 26.86, stage/main/post_update 26.18, stage/render/render 14.50, stage/main/pre_update 10.46, pipelines/driver_ms 8.62, render/graph/ms 7.88 |
| warm-up | 3.94 | 178.9 | pipelines/compile_ms 285, pipelines/front_ms 264, main/busy/ms 207, stage/main/update 157, render/thread/ms 56.83, render/busy/ms 54.90, stage/main/post_update 43.77, stage/render/prepare_assets 43.47, stream/drive/ms 33.10, pipelines/driver_ms 20.79, stage/render/render 8.11, render/graph/ms 5.81 |
| warm-up | 4.03 | 95.3 | pipelines/compile_ms 138, pipelines/front_ms 122, main/busy/ms 81.12, render/thread/ms 70.45, render/busy/ms 68.97, stage/render/prepare_assets 57.91, stage/main/update 41.82, stream/drive/ms 33.10, stage/main/post_update 29.05, pipelines/driver_ms 15.10, stage/render/render 8.30, render/graph/ms 6.48 |
| warm-up | 4.10 | 68.9 | render/thread/ms 86.35, render/busy/ms 84.71, main/busy/ms 58.01, stage/render/prepare_assets 55.50, stage/main/post_update 25.05, stage/render/prepare_resources 18.47, stage/main/update 18.07, stream/drive/ms 12.33, stage/render/render 8.92, stage/main/pre_update 8.54, render/graph/ms 6.95, stage/render/graph_submit 4.86 |
| warm-up | 4.16 | 41.3 | render/thread/ms 41.91, render/busy/ms 39.97, stage/render/prepare_assets 23.18, main/busy/ms 22.16, stage/render/render 9.35, stage/main/post_update 9.14, stage/main/update 7.97, render/graph/ms 7.26, encode//sum 5.02, stage/render/graph_submit 4.85, stage/render/prepare_resources 3.62, encode/submit 2.76 |
| warm-up | 4.19 | 27.1 | render/thread/ms 54.22, render/busy/ms 52.24, main/busy/ms 31.62, stage/render/prepare_assets 22.76, stage/main/post_update 15.32, stage/render/prepare_resources 15.07, stage/main/update 11.49, stage/render/render 9.47, render/graph/ms 7.25, bigspace/total/ms 5.43, stage/render/graph_submit 4.85, encode//sum 3.86 |
| warm-up | 4.23 | 41.3 | render/thread/ms 58.35, main/busy/ms 57.79, render/busy/ms 56.68, stage/main/update 39.53, stage/render/prepare_assets 28.55, stage/render/prepare_resources 14.12, stage/main/post_update 12.50, stage/render/render 8.59, render/graph/ms 6.61, bigspace/total/ms 5.44, stage/render/graph_submit 4.51, encode//sum 3.44 |
| warm-up | 4.27 | 45.8 | render/thread/ms 58.10, render/busy/ms 56.65, main/busy/ms 45.62, stage/render/prepare_assets 44.35, stage/main/update 34.58, stage/render/render 7.60, render/graph/ms 5.86, stage/main/post_update 5.35, stage/render/graph_submit 3.88, encode//sum 3.37, encode/submit 2.40, stage/main/pre_update 1.89 |
| warm-up | 4.37 | 26.9 | render/thread/ms 31.79, render/busy/ms 30.44, main/busy/ms 20.28, stage/render/prepare_assets 14.03, stage/render/render 8.44, stage/main/update 7.55, render/graph/ms 6.77, stage/main/post_update 5.29, stage/main/pre_update 4.99, stage/render/prepare_resources 4.27, stage/render/graph_submit 4.10, stream/drive/ms 4.07 |
| warm-up | 4.51 | 26.1 | render/thread/ms 37.21, render/busy/ms 35.60, main/busy/ms 30.52, stage/main/update 13.33, stage/render/prepare_assets 9.85, stage/render/prepare_flush 9.76, stage/main/post_update 9.74, stage/render/render 8.68, render/graph/ms 6.76, stage/render/graph_submit 4.59, encode//sum 4.51, stream/drive/ms 4.15 |
| warm-up | 4.55 | 39.0 | render/thread/ms 57.73, render/busy/ms 56.10, main/busy/ms 39.16, stage/render/prepare_assets 22.88, stage/main/post_update 12.99, stage/main/update 11.36, stage/render/prepare_resources 10.48, stage/render/render 9.57, render/graph/ms 7.68, stage/render/prepare_flush 7.49, stage/main/pre_update 5.64, stage/render/graph_submit 4.59 |
| warm-up | 4.59 | 41.2 | render/thread/ms 75.05, render/busy/ms 70.98, main/busy/ms 63.10, stage/main/update 35.55, stage/render/prepare_resources 23.68, stage/render/prepare_assets 19.30, stage/main/post_update 13.91, stage/render/render 13.75, render/graph/ms 11.38, stage/render/prepare_flush 7.87, stage/render/graph_submit 7.72, encode/submit 5.89 |
| warm-up | 4.68 | 82.6 | main/busy/ms 131, render/thread/ms 106, render/busy/ms 101, stage/main/update 88.19, stream/drive/ms 49.40, stage/main/post_update 35.66, stage/render/prepare_assets 25.52, stage/render/render 25.42, render/graph/ms 21.13, stage/render/prepare_resources 16.26, stage/render/graph_submit 14.50, stage/render/prepare_bind_groups 9.56 |
| warm-up | 4.79 | 113.5 | main/busy/ms 153, render/thread/ms 138, render/busy/ms 135, stage/main/update 96.76, stream/drive/ms 50.56, stage/main/post_update 46.95, stage/render/prepare_assets 44.17, stage/render/render 26.45, render/graph/ms 22.02, bigspace/total/ms 18.70, encode//sum 16.93, stage/render/graph_submit 13.42 |
| warm-up | 4.90 | 111.3 | render/thread/ms 157, render/busy/ms 155, main/busy/ms 131, stage/main/update 66.85, stage/render/prepare_assets 55.95, stage/main/post_update 44.02, encode//sum 24.94, stage/render/render 22.03, bigspace/total/ms 21.50, stage/render/prepare_flush 20.23, render/graph/ms 18.83, stream/drive/ms 13.93 |
| warm-up | 5.00 | 94.9 | render/thread/ms 148, render/busy/ms 145, main/busy/ms 100, stage/render/prepare_assets 67.13, stage/main/update 44.43, stage/main/post_update 34.09, encode//sum 33.40, stage/render/render 26.58, render/graph/ms 23.23, stage/render/prepare_flush 19.25, stream/drive/ms 14.36, stage/render/graph_submit 13.67 |
| warm-up | 5.09 | 90.3 | render/thread/ms 138, render/busy/ms 136, main/busy/ms 51.93, stage/render/prepare_assets 49.86, encode//sum 41.51, stage/render/render 34.55, stage/render/create_views 33.43, render/graph/ms 31.41, stage/main/update 26.58, stage/render/graph_submit 21.28, encode/submit 14.77, stage/main/post_update 14.17 |
| warm-up | 5.14 | 49.1 | render/thread/ms 124, render/busy/ms 122, main/busy/ms 66.49, stage/main/update 44.43, encode//sum 37.02, stage/render/render 36.38, render/graph/ms 33.38, stage/render/create_views 33.30, stream/drive/ms 28.36, stage/render/prepare_assets 26.23, stage/render/graph_submit 22.40, encode/submit 15.62 |
| warm-up | 5.18 | 44.7 | render/thread/ms 86.31, render/busy/ms 84.07, main/busy/ms 59.26, stage/main/update 38.19, stage/render/render 30.71, stream/drive/ms 27.74, render/graph/ms 27.71, encode//sum 27.16, stage/render/prepare_assets 22.43, stage/render/graph_submit 16.23, encode/pass:shadow_point_light/sum 11.72, stage/main/post_update 11.01 |
| warm-up | 5.24 | 59.1 | render/thread/ms 97.15, encode//sum 95.42, render/busy/ms 95.11, stage/render/render 44.36, render/graph/ms 41.50, stage/render/graph_submit 29.51, main/busy/ms 27.48, encode/pass:shadow_point_light/sum 27.34, encode/submit 14.79, encode/finish_wall 14.75, stage/render/create_views 14.34, stage/render/prepare_assets 13.39 |
| warm-up | 5.27 | 29.3 | encode//sum 87.76, render/thread/ms 83.33, render/busy/ms 81.58, stage/render/render 41.53, render/graph/ms 39.03, main/busy/ms 30.22, stage/render/graph_submit 29.36, encode/pass:shadow_point_light/sum 24.36, encode/submit 15.58, stage/main/update 15.03, encode/finish_wall 13.90, stage/render/create_views 12.95 |
| warm-up | 5.30 | 29.3 | render/thread/ms 51.35, render/busy/ms 49.92, main/busy/ms 30.27, stage/render/render 18.76, render/graph/ms 16.61, stage/main/update 15.35, encode//sum 11.61, stage/render/prepare_assets 11.40, stage/render/graph_submit 11.02, stage/main/post_update 9.89, encode/submit 8.40, stage/render/prepare_flush 6.51 |
| warm-up | 5.35 | 33.1 | pipelines/compile_ms 1644, pipelines/driver_ms 1505, pipelines/front_ms 139, render/thread/ms 48.24, render/busy/ms 46.62, main/busy/ms 36.17, stage/main/update 21.98, stage/render/render 18.74, render/graph/ms 16.49, encode//sum 12.81, stage/render/prepare_flush 11.33, stage/render/graph_submit 11.01 |
| warm-up | 5.38 | 32.1 | pipelines/compile_ms 5062, pipelines/driver_ms 4699, pipelines/front_ms 364, render/thread/ms 54.80, render/busy/ms 53.14, main/busy/ms 37.28, stage/main/update 21.71, stage/render/render 18.57, render/graph/ms 16.16, encode//sum 11.99, stage/render/prepare_assets 11.22, stage/main/post_update 11.03 |
| warm-up | 5.44 | 32.0 | pipelines/compile_ms 3719, pipelines/driver_ms 3364, pipelines/wait_ms 1579, pipelines/front_ms 355, render/thread/ms 45.98, render/busy/ms 44.67, main/busy/ms 25.71, stage/render/render 17.36, render/graph/ms 15.41, stage/render/prepare_assets 13.10, encode//sum 12.81, stage/main/update 11.39 |
| warm-up | 5.47 | 34.1 | pipelines/compile_ms 5300, pipelines/driver_ms 4868, pipelines/wait_ms 1606, pipelines/front_ms 432, render/thread/ms 57.00, render/busy/ms 54.98, main/busy/ms 46.71, stage/main/update 29.49, stage/render/render 26.18, render/graph/ms 23.76, stage/render/camera_driver 13.91, stage/render/prepare_assets 13.49 |
| warm-up | 5.56 | 84.6 | pipelines/compile_ms 9546, pipelines/driver_ms 8439, pipelines/wait_ms 5464, pipelines/front_ms 1108, render/thread/ms 108, render/busy/ms 106, main/busy/ms 98.68, stage/main/update 45.64, stage/main/post_update 41.88, stage/render/prepare_resources 34.94, stage/render/render 27.25, stage/render/prepare_assets 25.52 |
| warm-up | 5.60 | 45.3 | pipelines/wait_ms 12.3 k, pipelines/compile_ms 6568, pipelines/driver_ms 5195, pipelines/front_ms 1373, render/thread/ms 109, render/busy/ms 108, main/busy/ms 83.28, stage/render/prepare_assets 39.76, stage/main/post_update 38.08, stage/render/prepare_resources 33.20, stage/main/update 32.02, stage/render/render 18.14 |
| warm-up | 5.66 | 61.2 | pipelines/wait_ms 11.4 k, pipelines/compile_ms 8616, pipelines/driver_ms 7188, pipelines/front_ms 1428, render/thread/ms 73.35, render/busy/ms 71.97, main/busy/ms 38.28, stage/render/prepare_assets 36.73, stage/main/update 21.61, stage/render/render 17.36, render/graph/ms 15.26, encode//sum 11.78 |
| warm-up | 5.69 | 26.4 | pipelines/compile_ms 8347, pipelines/driver_ms 7179, pipelines/wait_ms 6328, pipelines/front_ms 1168, render/thread/ms 61.82, render/busy/ms 60.34, main/busy/ms 37.91, stage/render/prepare_assets 21.06, stage/main/update 17.65, stage/render/render 15.87, render/graph/ms 13.82, stage/main/post_update 12.91 |
| warm-up | 5.73 | 34.7 | pipelines/wait_ms 9200, pipelines/compile_ms 926, pipelines/front_ms 887, render/thread/ms 54.89, render/busy/ms 53.39, main/busy/ms 52.74, pipelines/driver_ms 38.49, stage/main/post_update 28.29, stage/render/render 27.87, render/graph/ms 25.82, stage/render/graph_submit 16.15, stage/main/update 15.97 |
| warm-up | 5.81 | 28.6 | pipelines/wait_ms 2282, pipelines/compile_ms 314, pipelines/front_ms 301, render/thread/ms 41.21, render/busy/ms 37.80, main/busy/ms 29.96, stage/render/render 20.23, render/graph/ms 16.28, stage/main/post_update 13.47, pipelines/driver_ms 12.76, encode//sum 11.88, stage/render/graph_submit 11.03 |
| warm-up | 5.87 | 36.3 | pipelines/wait_ms 2879, pipelines/compile_ms 583, pipelines/front_ms 563, render/thread/ms 49.13, render/busy/ms 47.73, main/busy/ms 31.03, stage/render/render 29.61, render/graph/ms 27.59, pipelines/driver_ms 19.12, stage/main/update 16.65, stage/render/camera_driver 16.08, encode//sum 12.52 |
| warm-up | 6.14 | 32.3 | pipelines/compile_ms 253, pipelines/front_ms 244, render/thread/ms 49.05, render/busy/ms 47.38, main/busy/ms 34.62, stage/render/render 25.24, stage/main/post_update 24.22, render/graph/ms 23.08, stage/render/graph_submit 13.57, encode//sum 13.11, encode/submit 9.07, stage/render/camera_driver 9.05 |
| warm-up | 6.17 | 33.3 | pipelines/compile_ms 253, pipelines/front_ms 244, main/busy/ms 65.28, render/thread/ms 60.02, render/busy/ms 58.77, stage/main/post_update 23.26, stage/render/render 22.45, render/graph/ms 20.64, stage/main/update 19.64, stage/main/pre_update 19.56, stage/render/graph_submit 15.40, encode//sum 14.62 |
| warm-up | 6.21 | 35.3 | pipelines/compile_ms 748, pipelines/front_ms 726, main/busy/ms 62.16, render/thread/ms 48.59, render/busy/ms 47.18, stage/render/render 22.77, pipelines/driver_ms 22.14, stage/main/pre_update 21.11, render/graph/ms 20.86, stage/main/update 19.41, stage/main/post_update 18.53, stage/render/graph_submit 15.34 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 8%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (45 samples): GPU load **86%** (min 66%) · 2623 MHz (min 2535) · memory 8101 MHz · 90.3 W · 72 °C (max 77) · VRAM 5215 MiB · throttled 98% of samples

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
