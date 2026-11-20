# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1566 frames over 11.4 s · **median 6.98 ms (143 fps)** · mean 7.30 · p95 9.76 · p99 10.91 (1% low 92 fps) · worst 57.64 ms · 25 spikes over 1.5× median · 105.0 stutters/min (work 57.7/min)
- warm-up: 120 frames over 5.9 s, worst 2531.4 ms
- hitches (> 25.0 ms): **2** measured, 33 in warm-up
- timed GPU passes: 5.13 ms of a 7.30 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median fps 143.4, target 144
- **FAIL** worst frame 57.64 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.913 | 2.138 | 2.625 | 0.89 | 0.171 |
| main_opaque_pass_3d | 1.745 | 1.950 | 2.845 | 0.89 | 0.037 |
| auxiliary_views | 0.319 | 0.357 | 1.118 | 0.89 | 0.299 |
| bloom | 0.305 | 0.341 | 0.338 | 0.89 | 0.064 |
| ssao | 0.184 | 0.205 | 0.196 | 0.89 | 0.007 |
| upscaling | 0.148 | 0.165 | 0.200 | 0.89 | 0.004 |
| tlas_build | 0.102 | 0.114 | 0.119 | 0.89 | 0.077 |
| tonemapping | 0.096 | 0.107 | 0.117 | 0.89 | 0.004 |
| gi/probe_update | 0.081 | 0.091 | 0.096 | 0.89 | 0.002 |
| gpu_instance_materialization | 0.076 | 0.085 | 0.089 | 0.89 | 0.004 |
| early prepass | 0.068 | 0.076 | 0.085 | 0.89 | 0.046 |
| clustering | 0.065 | 0.073 | 0.076 | 0.89 | 0.019 |
| uniform_allocation | 0.032 | 0.036 | 0.044 | 0.89 | 0.009 |
| bin_unpacking | 0.031 | 0.034 | 0.039 | 0.89 | 0.008 |
| range_unpacking | 0.017 | 0.019 | 0.020 | 0.89 | 0.003 |
| early_mesh_preprocessing | 0.016 | 0.018 | 0.025 | 0.89 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.87 | 0.023 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.89 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.26 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.89 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.262 | 8.667 | 54.77 | 9805.6 |
| render/busy/ms | 5.735 | 8.138 | 20.33 | 8980.8 |
| main/busy/ms | 5.433 | 6.346 | 13.66 | 8508.3 |
| stage/render/render | 3.904 | 5.628 | 31.83 | 6113.1 |
| render/graph/ms | 3.248 | 4.890 | 12.19 | 5085.9 |
| stage/main/post_update | 2.903 | 3.508 | 7.25 | 4546.3 |
| encode//sum | 2.543 | 4.061 | 7.95 | 3983.0 |
| stage/render/graph_submit | 2.109 | 3.233 | 8.06 | 3302.5 |
| encode/submit | 1.465 | 2.273 | 6.13 | 2294.6 |
| stage/main/update | 1.232 | 1.534 | 2.99 | 1929.7 |
| stage/render/camera_driver | 1.005 | 1.624 | 3.36 | 1574.5 |
| encode/pass:shadow_point_light/sum | 0.901 | 2.784 | 4.14 | 1410.8 |
| encode/finish_wall | 0.691 | 1.039 | 3.15 | 1082.2 |
| render/present/ms | 0.417 | 0.787 | 26.68 | 652.8 |
| stage/render/prepare_write_phase_buffers | 0.413 | 0.799 | 1.16 | 646.3 |
| stage/render/prepare_resources | 0.403 | 0.514 | 1.11 | 630.6 |
| encode/pass:early prepass/sum | 0.386 | 0.553 | 1.09 | 604.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.363 | 0.537 | 1.21 | 568.0 |
| stage/main/pre_update | 0.353 | 0.452 | 0.99 | 552.6 |
| stage/main/first | 0.345 | 0.441 | 1.88 | 540.4 |
| stage/render/prepare_bind_groups | 0.327 | 0.637 | 1.07 | 511.6 |
| stage/main/fixed_main_loop | 0.266 | 0.633 | 2.83 | 416.7 |
| bigspace/total/ms | 0.225 | 0.466 | 0.86 | 353.0 |
| stage/render/prepare_meshes | 0.213 | 0.296 | 0.57 | 334.3 |
| stage/render/prepare_assets | 0.210 | 0.280 | 0.73 | 328.6 |
| stage/render/prepare_flush | 0.178 | 0.329 | 0.54 | 279.2 |
| stage/main/last | 0.157 | 0.221 | 3.12 | 246.2 |
| stage/render/create_views | 0.155 | 0.201 | 0.69 | 243.4 |
| stage/render/prepare_views | 0.155 | 0.181 | 45.42 | 242.7 |
| stage/render/extract_commands | 0.155 | 0.194 | 0.47 | 242.1 |
| stream/drive/ms | 0.121 | 0.179 | 0.37 | 189.5 |
| render/acquire/ms | 0.110 | 0.115 | 45.37 | 172.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.109 | 0.167 | 0.49 | 170.2 |
| stream/tick/ms | 0.093 | 0.147 | 0.34 | 146.2 |
| bigspace/low_precision/ms | 0.091 | 0.202 | 0.54 | 141.8 |
| stage/render/prepare_batch_phases | 0.082 | 0.136 | 0.26 | 128.1 |
| physics/step/ms | 0.081 | 0.258 | 0.68 | 126.6 |
| stage/render/queue | 0.080 | 0.120 | 0.58 | 125.0 |
| stage/render/specialize | 0.079 | 0.105 | 0.77 | 123.5 |
| controller/feed/ms | 0.078 | 0.102 | 0.35 | 121.6 |
| anim/evaluate/ms | 0.066 | 0.133 | 0.65 | 103.6 |
| bigspace/high_precision/ms | 0.054 | 0.200 | 0.67 | 85.0 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.073 | 0.21 | 76.0 |
| anim/evaluate_cpu/ms | 0.035 | 0.052 | 0.23 | 55.5 |
| encode/resolve_diagnostics/sum | 0.032 | 0.045 | 0.36 | 49.8 |
| controller/write/ms | 0.027 | 0.039 | 0.17 | 42.6 |
| weather/map/ms | 0.027 | 0.039 | 0.19 | 41.7 |
| stage/render/cleanup | 0.022 | 0.034 | 0.67 | 35.0 |
| stage/render/phase_sort | 0.018 | 0.043 | 0.18 | 27.6 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.020 | 0.11 | 19.0 |
| weather/eval/ms | 0.012 | 0.017 | 0.05 | 18.8 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.10 | 16.2 |
| physics/insert/ms | 0.008 | 0.000 | 0.74 | 11.9 |
| script/update/ms | 0.007 | 0.012 | 0.06 | 11.7 |
| anim/apply/ms | 0.005 | 0.010 | 0.16 | 8.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.2 |
| controller/move/ms | 0.004 | 0.013 | 0.18 | 6.8 |
| far/select/ms | 0.004 | 0.005 | 0.15 | 6.4 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 4.9 |
| controller/avoid/ms | 0.002 | 0.005 | 0.05 | 3.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.27 | 1.5 |
| weather/apply/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 0.9 |
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
| crowd/joints_extracted/count | count | 46 | 1403 | 0 |
| crowd/joints_uploaded/count | count | 94 | 153 | 94 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.38 | 1.46 | 0.37 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.11 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.10 | 0.01 |
| encode/finish_critical | ms | 0.47 | 1.46 | 0.37 |
| encode/pass:early prepass/max | ms | 0.38 | 1.09 | 0.35 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.36 | 1.00 | 0.26 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.49 | 0.13 |
| encode/pass:shadow_point_light/max | ms | 0.45 | 0.84 | 0.36 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.36 | 0.02 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.82 | 20.33 | 5.25 |
| gpu/timed/ms | ms | 5.52 | 7.90 | 4.72 |
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
| phases/shadow/items | count | 3603 | 3839 | 3105 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 227 | 240 | 189 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 79 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 291 | 291 | 291 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 8 | 200 | 4 |
| preprocess/bind_groups_created | count | 4 | 196 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 47 | 101 | 35 |
| process/cpu_usage | % | 23.94 | 35.10 | 26.92 |
| process/mem_usage | GiB | 3.83 | 3.94 | 3.93 |
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
| system/cpu_usage | % | 32.75 | 78.44 | 35.23 |
| system/mem_usage | % | 53.76 | 54.15 | 54.10 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 26 | 112 | 17 |
| views/all/bins | count | 2993 | 3313 | 2231 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 4180 | 4586 | 3191 |
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
| views/lamp_face/items | count | 3603 | 3839 | 3105 |
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
| anim/bones_written/count | count | 8.53 | 30 | 13.4 k |
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
| encode//count | count | 31.66 | 67 | 49.6 k |
| encode/bind_walk/init_textures | count | 8383 | 21.0 k | 13.13 M |
| encode/bind_walk/init_textures_skipped | count | 10.7 k | 28.2 k | 16.81 M |
| encode/bind_walk/inits | count | 217 | 438 | 340.2 k |
| encode/bind_walk/inits_skipped | count | 15.74 | 42 | 24.6 k |
| encode/bind_walk/merge_textures | count | 7140 | 19.8 k | 11.18 M |
| encode/bind_walk/merge_textures_skipped | count | 3978 | 16.9 k | 6.23 M |
| encode/bind_walk/merges | count | 59.37 | 118 | 93.0 k |
| encode/bind_walk/merges_skipped | count | 19.71 | 69 | 30.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1566 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1566 |
| encode/pass:early prepass/count | count | 1 | 2 | 1566 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1566 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1518 |
| encode/pass:shadow_point_light/count | count | 3.09 | 12 | 4842 |
| encode/queue/command_buffers | count | 43.72 | 88 | 68.5 k |
| encode/queue/submits | count | 2 | 4 | 3132 |
| encode/queue/write_buffer/bytes | bytes | 602.9 k | 1.21 M | 944.15 M |
| encode/queue/write_buffer/calls | count | 160 | 370 | 250.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 149.6 k | 299.3 k | 234.32 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9396 |
| encode/queue/write_texture/bytes | bytes | 670 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1566 |
| physics/inserts | count | 0.05 | 9 | 80 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.44 | 3 | 685 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
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

## Hitches (> 25.0 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2531.4 | main/busy/ms 2504, stage/main/post_update 2400, stage/main/update 94.35, surface/advance_cpu_ms 67.16, stage/main/pre_update 6.17, stage/main/last 1.16, stream/drive/ms 0.58, stage/main/first 0.53, stage/main/fixed_main_loop 0.40, weather/map/ms 0.14, stream/tick/ms 0.08, bigspace/total/ms 0.07 |
| warm-up | 3.29 | 755.8 | main/busy/ms 2616, stage/main/post_update 2405, render/thread/ms 722, render/busy/ms 538, stage/render/prepare_assets 518, stage/main/update 196, stage/render/prepare_views 183, render/acquire/ms 182, surface/advance_cpu_ms 67.16, stage/render/prepare_resources 7.90, stage/render/render 7.88, stage/main/pre_update 7.45 |
| warm-up | 3.33 | 46.8 | render/thread/ms 753, render/busy/ms 569, stage/render/prepare_assets 544, stage/render/prepare_views 183, render/acquire/ms 182, main/busy/ms 125, stage/main/update 106, stage/render/render 11.43, stage/render/prepare_resources 8.46, render/graph/ms 7.75, stage/main/fixed_main_loop 7.54, stage/main/post_update 7.31 |
| warm-up | 3.38 | 41.1 | pipelines/wait_ms 437, pipelines/compile_ms 207, pipelines/front_ms 189, main/busy/ms 69.25, render/thread/ms 64.14, render/busy/ms 60.97, stage/main/update 42.52, stage/render/render 32.96, render/graph/ms 29.72, stage/render/prepare_assets 26.89, stage/render/graph_submit 22.74, pipelines/driver_ms 18.71 |
| warm-up | 3.41 | 36.6 | pipelines/wait_ms 1203, pipelines/compile_ms 499, pipelines/front_ms 461, main/busy/ms 85.75, render/thread/ms 50.93, stage/main/update 49.86, render/busy/ms 46.01, stage/render/render 39.39, pipelines/driver_ms 38.33, render/graph/ms 34.51, stage/main/post_update 24.54, stage/render/graph_submit 22.48 |
| warm-up | 3.44 | 29.1 | pipelines/wait_ms 818, pipelines/compile_ms 453, pipelines/front_ms 426, main/busy/ms 43.13, render/thread/ms 33.63, render/busy/ms 29.35, pipelines/driver_ms 26.61, stage/main/post_update 15.95, stage/main/update 15.20, stage/render/render 14.00, stage/render/prepare_assets 12.56, render/graph/ms 10.08 |
| warm-up | 3.49 | 30.1 | pipelines/compile_ms 585, pipelines/front_ms 563, main/busy/ms 67.82, stage/main/update 53.63, render/thread/ms 49.29, render/busy/ms 42.41, pipelines/wait_ms 42.38, pipelines/driver_ms 22.10, stage/render/render 20.69, render/graph/ms 14.25, stage/render/prepare_resources 11.96, stage/render/prepare_assets 10.72 |
| warm-up | 3.58 | 89.0 | pipelines/compile_ms 1230, pipelines/front_ms 1185, render/thread/ms 86.08, render/busy/ms 79.32, main/busy/ms 73.05, stage/main/update 57.79, pipelines/wait_ms 55.91, stage/render/prepare_assets 55.79, pipelines/driver_ms 44.32, stage/render/render 23.22, render/graph/ms 16.75, stage/render/graph_submit 9.97 |
| warm-up | 3.63 | 31.9 | pipelines/compile_ms 256, pipelines/front_ms 249, render/thread/ms 44.81, render/busy/ms 40.28, main/busy/ms 33.64, stage/render/prepare_assets 25.43, stage/main/post_update 17.65, bigspace/total/ms 12.24, bigspace/high_precision/ms 12.04, stage/render/render 9.16, pipelines/driver_ms 7.44, render/graph/ms 7.32 |
| warm-up | 3.71 | 57.9 | pipelines/compile_ms 868, pipelines/front_ms 819, render/thread/ms 51.78, render/busy/ms 49.32, pipelines/driver_ms 49.27, main/busy/ms 48.78, stage/render/prepare_assets 33.20, stage/main/update 31.39, stream/drive/ms 17.01, stage/render/render 9.57, stage/main/post_update 9.05, render/graph/ms 7.01 |
| warm-up | 3.77 | 55.1 | pipelines/compile_ms 1292, pipelines/front_ms 1232, main/busy/ms 184, stage/main/update 165, render/thread/ms 83.26, render/busy/ms 69.38, pipelines/driver_ms 60.13, stage/render/prepare_assets 52.36, stage/render/render 25.41, render/present/ms 13.51, render/graph/ms 11.37, stage/render/graph_submit 8.55 |
| warm-up | 4.01 | 243.1 | pipelines/compile_ms 2257, pipelines/front_ms 2182, main/busy/ms 171, stage/main/update 158, render/thread/ms 130, render/busy/ms 116, stage/render/prepare_assets 100, pipelines/driver_ms 75.77, pipelines/wait_ms 39.63, pipelines/lock_ms 35.87, stage/render/render 24.33, render/present/ms 13.43 |
| warm-up | 4.08 | 70.3 | pipelines/compile_ms 1844, pipelines/front_ms 1730, render/thread/ms 141, render/busy/ms 138, pipelines/driver_ms 114, stage/render/prepare_assets 109, main/busy/ms 58.53, stage/main/update 44.10, pipelines/wait_ms 39.65, pipelines/lock_ms 34.83, stream/drive/ms 34.28, stage/render/render 27.26 |
| warm-up | 4.10 | 25.4 | pipelines/compile_ms 581, pipelines/front_ms 506, render/thread/ms 78.72, render/busy/ms 76.27, pipelines/driver_ms 75.28, main/busy/ms 63.71, pipelines/wait_ms 48.93, stage/main/update 44.76, stage/render/prepare_assets 44.45, stream/drive/ms 34.26, stage/render/render 27.04, render/graph/ms 24.36 |
| warm-up | 4.15 | 30.4 | pipelines/compile_ms 631, pipelines/front_ms 607, main/busy/ms 35.95, render/thread/ms 29.72, render/busy/ms 27.82, pipelines/driver_ms 24.45, pipelines/wait_ms 23.04, stage/main/update 18.96, stage/render/prepare_assets 13.94, stage/main/post_update 13.61, stream/drive/ms 9.22, stage/render/render 8.79 |
| warm-up | 4.19 | 35.1 | pipelines/compile_ms 538, pipelines/front_ms 516, render/thread/ms 42.57, render/busy/ms 41.29, main/busy/ms 30.90, stage/render/prepare_assets 30.24, pipelines/driver_ms 21.63, pipelines/wait_ms 21.52, stage/main/post_update 14.72, stage/main/update 12.48, stage/render/render 6.85, render/graph/ms 5.37 |
| warm-up | 4.48 | 33.7 | pipelines/compile_ms 635, pipelines/front_ms 618, main/busy/ms 48.82, render/thread/ms 45.13, render/busy/ms 42.98, stage/main/post_update 24.52, stage/render/render 22.27, render/graph/ms 19.60, pipelines/wait_ms 18.41, pipelines/driver_ms 17.66, stage/render/prepare_assets 11.23, stage/main/update 11.15 |
| warm-up | 4.53 | 49.8 | pipelines/compile_ms 985, pipelines/front_ms 951, main/busy/ms 78.76, render/thread/ms 72.47, render/busy/ms 69.48, stage/main/update 38.73, pipelines/driver_ms 34.21, stage/main/post_update 32.78, stage/render/prepare_assets 29.34, pipelines/wait_ms 25.37, stage/render/render 23.56, render/graph/ms 20.30 |
| warm-up | 4.60 | 70.6 | pipelines/compile_ms 1210, pipelines/front_ms 1165, main/busy/ms 148, stage/main/update 114, render/thread/ms 89.09, render/busy/ms 86.08, stream/drive/ms 60.38, stage/render/prepare_assets 45.88, pipelines/driver_ms 45.06, stage/main/post_update 24.75, stage/render/render 17.62, render/graph/ms 14.63 |
| warm-up | 4.73 | 127.6 | pipelines/compile_ms 1889, pipelines/front_ms 1816, main/busy/ms 124, stage/main/update 95.64, render/thread/ms 94.06, render/busy/ms 91.89, pipelines/driver_ms 72.29, stream/drive/ms 61.06, stage/render/prepare_assets 54.83, pipelines/wait_ms 20.20, stage/main/post_update 18.44, stage/render/render 17.90 |
| warm-up | 4.77 | 45.8 | pipelines/compile_ms 1036, pipelines/front_ms 992, render/thread/ms 84.05, render/busy/ms 82.51, stage/render/prepare_assets 52.17, main/busy/ms 48.44, pipelines/driver_ms 44.19, stage/main/update 24.65, encode//sum 22.47, stage/render/render 15.62, pipelines/wait_ms 15.58, stage/main/post_update 15.15 |
| warm-up | 4.83 | 54.1 | pipelines/compile_ms 153, pipelines/front_ms 146, render/thread/ms 70.09, render/busy/ms 68.79, stage/render/prepare_assets 40.46, main/busy/ms 40.43, encode//sum 29.20, stage/main/update 21.34, stage/render/render 16.00, render/graph/ms 14.32, stage/main/post_update 10.28, stage/render/graph_submit 9.46 |
| warm-up | 4.86 | 33.5 | pipelines/compile_ms 153, pipelines/front_ms 146, encode//sum 74.55, render/thread/ms 66.07, render/busy/ms 64.62, main/busy/ms 45.72, stage/main/update 31.77, stage/render/render 27.35, render/graph/ms 25.52, stage/render/prepare_assets 23.69, encode/pass:shadow_point_light/sum 20.03, stage/render/graph_submit 19.10 |
| warm-up | 4.90 | 37.0 | encode//sum 78.95, render/thread/ms 64.86, render/busy/ms 62.95, stage/render/render 42.39, render/graph/ms 40.11, main/busy/ms 39.34, stage/render/graph_submit 31.96, stage/main/update 27.55, encode/pass:shadow_point_light/sum 22.49, stream/drive/ms 19.26, encode/finish_wall 19.22, encode/submit 12.78 |
| warm-up | 4.94 | 45.1 | render/thread/ms 77.26, render/busy/ms 75.20, stage/render/render 42.16, render/graph/ms 39.67, encode//sum 38.79, main/busy/ms 32.94, stage/render/graph_submit 30.10, encode/submit 17.15, encode/pass:shadow_point_light/sum 14.58, encode/finish_wall 13.00, stage/main/update 12.27, stage/main/post_update 12.24 |
| warm-up | 4.98 | 38.3 | pipelines/compile_ms 175, pipelines/front_ms 164, render/thread/ms 77.26, render/busy/ms 75.62, main/busy/ms 33.67, encode//sum 32.73, stage/render/render 32.68, render/graph/ms 30.42, stage/render/graph_submit 20.57, encode/pass:shadow_point_light/sum 16.68, encode/submit 14.83, stage/main/post_update 13.72 |
| warm-up | 5.01 | 26.9 | pipelines/compile_ms 1495, pipelines/front_ms 1431, pipelines/driver_ms 63.98, render/thread/ms 59.35, render/busy/ms 57.80, main/busy/ms 34.00, stage/render/render 25.48, render/graph/ms 23.15, encode//sum 18.84, stage/main/update 16.77, stage/render/graph_submit 15.51, encode/submit 11.49 |
| warm-up | 5.04 | 29.4 | pipelines/compile_ms 1340, pipelines/front_ms 1285, pipelines/driver_ms 54.71, render/thread/ms 48.69, render/busy/ms 47.05, main/busy/ms 30.44, stage/render/render 21.55, render/graph/ms 19.36, stage/main/update 16.07, stage/render/graph_submit 13.96, encode//sum 10.24, stage/render/prepare_assets 9.11 |
| warm-up | 5.10 | 35.8 | pipelines/compile_ms 156, pipelines/front_ms 141, render/thread/ms 56.18, render/busy/ms 54.72, main/busy/ms 37.49, stage/main/post_update 23.61, stage/render/render 20.29, render/graph/ms 18.28, pipelines/driver_ms 15.30, stage/render/prepare_resources 12.59, encode//sum 12.13, stage/render/camera_driver 9.82 |
| warm-up | 5.14 | 26.7 | pipelines/compile_ms 580, pipelines/front_ms 559, render/thread/ms 36.68, render/busy/ms 35.29, main/busy/ms 26.46, pipelines/driver_ms 21.63, stage/render/render 18.28, render/graph/ms 16.31, stage/main/post_update 13.89, stage/render/graph_submit 11.85, stage/render/prepare_assets 9.82, encode//sum 9.54 |
| warm-up | 5.26 | 45.2 | pipelines/compile_ms 601, pipelines/front_ms 585, main/busy/ms 60.54, render/thread/ms 50.05, render/busy/ms 48.01, stage/main/update 41.11, stage/render/render 20.89, render/graph/ms 18.15, pipelines/driver_ms 16.20, stage/main/post_update 14.23, encode//sum 13.47, stage/render/prepare_assets 12.38 |
| warm-up | 5.30 | 44.5 | pipelines/compile_ms 1251, pipelines/front_ms 1220, render/thread/ms 61.17, render/busy/ms 59.39, main/busy/ms 51.35, stage/main/update 31.97, pipelines/driver_ms 30.90, stage/render/prepare_assets 28.26, stage/render/render 17.55, render/graph/ms 15.09, encode//sum 12.64, stage/main/post_update 11.29 |
| warm-up | 5.34 | 35.7 | pipelines/compile_ms 650, pipelines/front_ms 635, render/thread/ms 50.96, render/busy/ms 50.00, stage/render/prepare_assets 30.47, main/busy/ms 26.53, pipelines/driver_ms 14.70, stage/main/update 11.67, stage/render/render 10.96, encode//sum 10.20, render/graph/ms 9.48, stage/main/post_update 7.92 |
| measure | 7.85 | 35.9 | render/thread/ms 41.35, stage/render/render 36.16, render/present/ms 27.51, render/busy/ms 13.63, main/busy/ms 13.50, render/graph/ms 7.98, encode//sum 7.47, stage/main/post_update 7.22, stage/render/graph_submit 4.91, encode/submit 3.05, stage/main/update 2.94, stage/render/camera_driver 2.71 |
| measure | 8.01 | 57.6 | render/thread/ms 61.13, stage/render/prepare_views 45.59, render/acquire/ms 45.45, main/busy/ms 14.81, render/busy/ms 13.35, stage/render/render 10.10, stage/main/post_update 7.08, render/graph/ms 6.85, encode//sum 5.84, stage/render/graph_submit 4.00, stage/main/update 3.81, encode/submit 3.17 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 9% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (250 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **87%** (min 80%) · 2625 MHz (min 2625) · memory 8101 MHz · 91.4 W · 72 °C (max 77) · VRAM 4960 MiB · throttled 100% of samples

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
