# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); stream.interior.disable_exterior=on (command line: --set); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1627 frames over 11.4 s · **median 6.74 ms (148 fps)** · mean 7.02 · p95 9.30 · p99 10.00 (1% low 100 fps) · worst 18.79 ms · 15 spikes over 1.5× median · 47.2 stutters/min (work 21.0/min)
- warm-up: 120 frames over 6.3 s, worst 2573.7 ms
- hitches (> 25.0 ms): **0** measured, 44 in warm-up
- timed GPU passes: 4.99 ms of a 7.02 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 148.5, target 144
- pass worst frame 18.79 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.904 | 2.027 | 2.067 | 0.94 | 0.164 |
| main_opaque_pass_3d | 1.743 | 1.856 | 2.552 | 0.94 | 0.031 |
| bloom | 0.302 | 0.322 | 0.327 | 0.94 | 0.062 |
| auxiliary_views | 0.287 | 0.305 | 0.933 | 0.94 | 0.285 |
| ssao | 0.178 | 0.189 | 0.193 | 0.94 | 0.006 |
| upscaling | 0.142 | 0.152 | 0.172 | 0.94 | 0.004 |
| tlas_build | 0.107 | 0.114 | 0.119 | 0.94 | 0.077 |
| tonemapping | 0.093 | 0.099 | 0.106 | 0.94 | 0.004 |
| gi/probe_update | 0.086 | 0.092 | 0.099 | 0.94 | 0.002 |
| early prepass | 0.073 | 0.078 | 0.098 | 0.94 | 0.035 |
| clustering | 0.068 | 0.073 | 0.076 | 0.94 | 0.019 |
| uniform_allocation | 0.034 | 0.036 | 0.045 | 0.94 | 0.010 |
| bin_unpacking | 0.030 | 0.032 | 0.039 | 0.94 | 0.008 |
| early_mesh_preprocessing | 0.013 | 0.013 | 0.014 | 0.94 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.010 | 0.91 | 0.022 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.94 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.26 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.94 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.852 | 8.188 | 17.79 | 9521.2 |
| render/busy/ms | 5.409 | 7.727 | 17.36 | 8800.7 |
| main/busy/ms | 5.280 | 6.098 | 9.08 | 8590.9 |
| stage/render/render | 3.730 | 5.390 | 14.81 | 6068.3 |
| render/graph/ms | 3.134 | 4.733 | 14.26 | 5098.6 |
| stage/main/post_update | 2.892 | 3.435 | 4.89 | 4705.5 |
| encode//sum | 2.275 | 3.507 | 12.46 | 3702.1 |
| stage/render/graph_submit | 2.085 | 3.119 | 12.50 | 3392.7 |
| encode/submit | 1.396 | 2.138 | 3.63 | 2271.5 |
| stage/main/update | 1.216 | 1.471 | 2.18 | 1978.0 |
| encode/pass:shadow_point_light/sum | 1.022 | 3.069 | 4.36 | 1662.2 |
| stage/render/camera_driver | 0.917 | 1.532 | 1.99 | 1492.4 |
| encode/finish_wall | 0.714 | 1.058 | 9.69 | 1162.4 |
| stage/render/prepare_write_phase_buffers | 0.405 | 0.775 | 1.03 | 659.4 |
| encode/pass:early prepass/sum | 0.397 | 0.538 | 1.04 | 645.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.386 | 0.537 | 1.05 | 627.6 |
| stage/render/prepare_resources | 0.381 | 0.465 | 0.73 | 620.1 |
| render/present/ms | 0.364 | 0.594 | 1.89 | 591.7 |
| stage/main/first | 0.334 | 0.422 | 1.37 | 542.6 |
| stage/main/pre_update | 0.319 | 0.404 | 0.97 | 519.6 |
| stage/render/prepare_bind_groups | 0.279 | 0.581 | 0.91 | 453.2 |
| bigspace/total/ms | 0.244 | 0.573 | 1.14 | 396.2 |
| stage/main/fixed_main_loop | 0.232 | 0.628 | 1.10 | 378.3 |
| stage/render/prepare_assets | 0.200 | 0.247 | 1.44 | 325.6 |
| stage/render/prepare_meshes | 0.186 | 0.250 | 0.47 | 303.3 |
| stage/render/create_views | 0.146 | 0.186 | 0.46 | 237.4 |
| stage/render/extract_commands | 0.143 | 0.168 | 0.36 | 232.9 |
| stream/drive/ms | 0.134 | 0.180 | 0.49 | 217.6 |
| stage/render/prepare_flush | 0.132 | 0.241 | 0.47 | 214.2 |
| stage/render/prepare_views | 0.122 | 0.162 | 0.38 | 198.0 |
| stage/main/last | 0.113 | 0.144 | 3.10 | 183.4 |
| stream/tick/ms | 0.107 | 0.151 | 0.34 | 173.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.103 | 0.147 | 0.61 | 166.8 |
| physics/step/ms | 0.099 | 0.337 | 0.50 | 160.8 |
| bigspace/low_precision/ms | 0.095 | 0.295 | 1.06 | 154.5 |
| render/acquire/ms | 0.079 | 0.105 | 0.27 | 128.9 |
| stage/render/specialize | 0.077 | 0.102 | 0.56 | 125.5 |
| anim/evaluate/ms | 0.069 | 0.174 | 0.59 | 112.1 |
| stage/render/prepare_batch_phases | 0.066 | 0.110 | 0.30 | 107.7 |
| controller/feed/ms | 0.064 | 0.080 | 0.27 | 103.4 |
| stage/render/phase_sort | 0.053 | 0.086 | 0.27 | 86.3 |
| bigspace/high_precision/ms | 0.048 | 0.131 | 0.71 | 78.9 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.068 | 0.22 | 75.3 |
| stage/render/queue | 0.038 | 0.063 | 0.36 | 61.4 |
| anim/evaluate_cpu/ms | 0.035 | 0.051 | 0.43 | 56.3 |
| encode/resolve_diagnostics/sum | 0.030 | 0.039 | 0.47 | 48.9 |
| weather/map/ms | 0.025 | 0.038 | 0.08 | 41.3 |
| stage/render/cleanup | 0.021 | 0.034 | 0.33 | 34.9 |
| far/select/ms | 0.016 | 0.018 | 0.13 | 25.5 |
| physics/insert/ms | 0.013 | 0.000 | 0.61 | 20.5 |
| weather/eval/ms | 0.012 | 0.018 | 0.06 | 19.6 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.018 | 0.45 | 18.8 |
| encode/camera_driver_span_end/sum | 0.010 | 0.017 | 0.06 | 15.8 |
| script/update/ms | 0.007 | 0.012 | 0.03 | 12.0 |
| controller/move/ms | 0.006 | 0.007 | 0.03 | 9.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.2 |
| anim/apply/ms | 0.005 | 0.009 | 0.34 | 7.8 |
| controller/avoid/ms | 0.003 | 0.004 | 0.03 | 5.6 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 4.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.8 |
| controller/write/ms | 0.001 | 0.001 | 0.00 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| weather/apply/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.002 | 0.02 | 0.9 |
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
| encode//max | ms | 0.38 | 8.87 | 0.41 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.45 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.06 | 0.01 |
| encode/finish_critical | ms | 0.48 | 8.87 | 0.41 |
| encode/pass:early prepass/max | ms | 0.40 | 1.04 | 0.27 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.40 | 1.05 | 0.33 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.10 | 0.61 | 0.10 |
| encode/pass:shadow_point_light/max | ms | 0.49 | 1.07 | 0.52 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.47 | 0.04 |
| far/entities | count | 162 | 162 | 162 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.6 k | 41.6 k | 41.6 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.26 M | 86.26 M | 86.26 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.56 | 17.36 | 5.62 |
| gpu/timed/ms | ms | 5.18 | 7.01 | 4.58 |
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
| phases/opaque3d/items | count | 250 | 505 | 38 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 7 | 10 | 4 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 7 | 3 |
| phases/opaque3d_prepass/bins | count | 142 | 280 | 24 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 250 | 505 | 38 |
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
| physics/pending | count | 0 | 118 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 289 | 289 | 289 |
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
| preprocess/bind_groups_reused | count | 47 | 103 | 35 |
| process/cpu_usage | % | 24.94 | 33.65 | 30.57 |
| process/mem_usage | GiB | 3.83 | 3.88 | 3.88 |
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
| stream/latency/s | s | 0.06 | 0.06 | 0.06 |
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
| system/cpu_usage | % | 30.40 | 42.60 | 32.40 |
| system/mem_usage | % | 57.28 | 57.45 | 57.45 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2038 | 2038 | 2038 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 18 | 102 | 9 |
| views/all/bins | count | 2970 | 3305 | 2223 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 4158 | 4588 | 3183 |
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
| views/lamp_face/drawn | count | 0 | 13 | 0 |
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
| views/main/items | count | 520 | 1054 | 78 |
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
| anim/bones_written/count | count | 8.38 | 30 | 13.6 k |
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
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 31.16 | 70 | 50.7 k |
| encode/bind_walk/init_textures | count | 8662 | 22.4 k | 14.09 M |
| encode/bind_walk/init_textures_skipped | count | 18.0 k | 60.6 k | 29.34 M |
| encode/bind_walk/inits | count | 196 | 409 | 318.3 k |
| encode/bind_walk/inits_skipped | count | 20.87 | 65 | 34.0 k |
| encode/bind_walk/merge_textures | count | 7419 | 21.2 k | 12.07 M |
| encode/bind_walk/merge_textures_skipped | count | 32.47 | 77 | 52.8 k |
| encode/bind_walk/merges | count | 60.07 | 117 | 97.7 k |
| encode/bind_walk/merges_skipped | count | 8.08 | 22 | 13.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1627 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1627 |
| encode/pass:early prepass/count | count | 1 | 2 | 1627 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1627 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1580 |
| encode/pass:shadow_point_light/count | count | 3.22 | 13 | 5237 |
| encode/queue/command_buffers | count | 43.35 | 92 | 70.5 k |
| encode/queue/submits | count | 2 | 4 | 3254 |
| encode/queue/write_buffer/bytes | bytes | 475.7 k | 895.0 k | 773.99 M |
| encode/queue/write_buffer/calls | count | 150 | 352 | 244.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 158.0 k | 315.9 k | 256.99 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9762 |
| encode/queue/write_texture/bytes | bytes | 11.0 k | 8.39 M | 17.83 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1627 |
| physics/inserts | count | 0.07 | 10 | 122 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.42 | 1 | 687 |
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

## Hitches (> 25.0 ms; slowest 44, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2573.7 | main/busy/ms 2544, stage/main/post_update 2438, stage/main/update 97.89, surface/advance_cpu_ms 66.91, stage/main/pre_update 4.30, stage/main/last 1.44, stream/drive/ms 0.64, stage/main/first 0.51, stage/main/fixed_main_loop 0.39, stream/tick/ms 0.15, weather/map/ms 0.12, bigspace/total/ms 0.08 |
| warm-up | 3.29 | 715.4 | main/busy/ms 2664, stage/main/post_update 2443, render/thread/ms 678, render/busy/ms 561, stage/render/prepare_assets 540, stage/main/update 208, stage/render/prepare_views 114, render/acquire/ms 113, surface/advance_cpu_ms 66.91, stage/render/render 11.83, stage/render/prepare_resources 8.16, render/graph/ms 7.68 |
| warm-up | 3.37 | 84.3 | render/thread/ms 746, render/busy/ms 587, stage/render/prepare_assets 561, main/busy/ms 131, stage/render/prepare_views 115, stage/main/update 114, render/acquire/ms 114, stage/render/render 56.79, render/present/ms 45.40, render/graph/ms 10.55, stage/render/prepare_resources 8.64, encode//sum 8.33 |
| warm-up | 3.43 | 52.1 | pipelines/wait_ms 774, pipelines/compile_ms 429, pipelines/front_ms 395, main/busy/ms 54.94, pipelines/driver_ms 33.87, render/thread/ms 27.43, render/busy/ms 24.61, stage/main/update 23.31, stage/main/post_update 18.48, stage/render/prepare_assets 11.73, stage/render/render 8.53, stage/main/pre_update 6.36 |
| warm-up | 3.47 | 38.8 | pipelines/wait_ms 860, pipelines/compile_ms 562, pipelines/front_ms 526, pipelines/driver_ms 36.38, render/thread/ms 33.63, render/busy/ms 31.78, main/busy/ms 26.69, stage/render/prepare_assets 16.36, stage/main/update 13.93, stage/render/cleanup 8.61, stage/render/render 7.75, stage/main/pre_update 6.33 |
| warm-up | 3.50 | 30.6 | pipelines/compile_ms 415, pipelines/front_ms 394, pipelines/wait_ms 123, render/thread/ms 43.60, render/busy/ms 34.05, main/busy/ms 28.64, pipelines/driver_ms 20.97, stage/render/prepare_assets 15.61, stage/render/render 14.43, stage/main/fixed_main_loop 14.27, render/present/ms 9.18, stage/render/cleanup 8.61 |
| warm-up | 3.54 | 32.5 | pipelines/compile_ms 672, pipelines/front_ms 643, pipelines/wait_ms 89.70, main/busy/ms 81.81, render/thread/ms 60.19, stage/main/update 59.07, render/busy/ms 49.73, pipelines/driver_ms 29.07, stage/render/render 23.43, stage/render/prepare_assets 21.52, stage/main/fixed_main_loop 13.91, render/graph/ms 12.77 |
| warm-up | 3.61 | 76.7 | pipelines/compile_ms 948, pipelines/front_ms 910, main/busy/ms 76.25, pipelines/wait_ms 75.96, render/thread/ms 72.88, render/busy/ms 69.79, stage/main/update 63.12, stage/render/prepare_assets 46.53, pipelines/driver_ms 37.59, stage/render/render 17.45, render/graph/ms 14.22, stage/main/post_update 6.98 |
| warm-up | 3.71 | 35.6 | pipelines/compile_ms 247, pipelines/front_ms 237, main/busy/ms 46.12, render/thread/ms 44.18, render/busy/ms 40.89, stage/main/update 24.26, stage/render/render 21.41, stage/main/post_update 17.95, render/graph/ms 17.94, stage/render/prepare_assets 17.43, stream/drive/ms 16.96, pipelines/driver_ms 10.34 |
| warm-up | 3.76 | 50.5 | pipelines/compile_ms 619, pipelines/front_ms 598, render/thread/ms 72.82, render/busy/ms 69.29, main/busy/ms 42.19, stage/render/prepare_assets 38.42, stage/render/render 29.27, render/graph/ms 25.41, encode//sum 23.89, pipelines/driver_ms 21.26, stage/main/update 20.88, stage/main/post_update 17.64 |
| warm-up | 3.80 | 40.8 | pipelines/compile_ms 835, pipelines/front_ms 807, render/thread/ms 73.02, render/busy/ms 71.01, main/busy/ms 37.94, stage/render/prepare_assets 30.52, stage/render/render 29.49, pipelines/driver_ms 28.26, render/graph/ms 27.30, stage/render/graph_submit 25.30, encode//sum 23.95, stage/main/post_update 22.78 |
| warm-up | 3.83 | 26.1 | pipelines/compile_ms 855, pipelines/front_ms 827, render/thread/ms 54.68, render/busy/ms 53.17, main/busy/ms 44.40, pipelines/driver_ms 27.91, stage/render/prepare_assets 24.28, stage/main/post_update 22.17, pipelines/wait_ms 18.42, stage/render/render 17.52, render/graph/ms 15.98, stage/render/graph_submit 14.23 |
| warm-up | 3.87 | 42.0 | pipelines/compile_ms 1100, pipelines/front_ms 1064, pipelines/wait_ms 75.87, main/busy/ms 75.04, render/thread/ms 53.78, render/busy/ms 52.32, stage/main/update 43.89, stage/render/prepare_assets 39.97, pipelines/driver_ms 36.22, stream/drive/ms 33.12, stage/main/pre_update 23.06, stage/render/render 8.71 |
| warm-up | 3.91 | 41.9 | pipelines/compile_ms 1303, pipelines/front_ms 1260, pipelines/wait_ms 118, main/busy/ms 68.51, stage/main/update 43.59, render/thread/ms 43.48, pipelines/driver_ms 42.75, render/busy/ms 41.87, stream/drive/ms 33.23, stage/render/prepare_assets 29.04, stage/main/pre_update 16.81, stage/render/render 9.37 |
| warm-up | 3.96 | 53.2 | pipelines/compile_ms 1100, pipelines/front_ms 1065, pipelines/wait_ms 108, render/thread/ms 45.14, render/busy/ms 43.61, pipelines/driver_ms 35.51, main/busy/ms 29.20, stage/main/update 16.72, stage/render/prepare_assets 16.57, stage/render/prepare_resources 16.22, stream/drive/ms 12.44, stage/main/post_update 8.51 |
| warm-up | 4.04 | 55.0 | pipelines/compile_ms 960, pipelines/front_ms 927, render/thread/ms 58.07, render/busy/ms 56.68, pipelines/wait_ms 47.87, main/busy/ms 34.11, stage/render/prepare_assets 32.88, pipelines/driver_ms 32.62, stage/main/update 16.84, stage/render/phase_sort 9.72, stage/main/post_update 9.15, stage/render/render 6.65 |
| warm-up | 4.21 | 28.6 | pipelines/compile_ms 577, pipelines/front_ms 559, render/thread/ms 44.95, render/busy/ms 43.62, main/busy/ms 35.97, stage/render/prepare_assets 17.94, pipelines/driver_ms 17.30, stage/main/update 16.53, stage/main/post_update 13.22, bigspace/total/ms 7.93, stage/render/prepare_resources 7.78, bigspace/low_precision/ms 7.78 |
| warm-up | 4.27 | 25.7 | pipelines/compile_ms 493, pipelines/front_ms 477, render/thread/ms 26.30, render/busy/ms 24.96, main/busy/ms 20.33, pipelines/driver_ms 15.79, stage/render/prepare_assets 14.31, stage/main/post_update 12.36, pipelines/wait_ms 10.53, stage/render/render 6.76, render/graph/ms 5.19, stage/main/update 4.73 |
| warm-up | 4.35 | 44.4 | pipelines/compile_ms 498, pipelines/front_ms 477, main/busy/ms 74.58, stage/main/update 57.38, stream/drive/ms 53.17, pipelines/driver_ms 21.18, render/thread/ms 19.46, render/busy/ms 18.75, pipelines/wait_ms 15.24, stage/render/render 10.11, render/graph/ms 9.29, stage/render/graph_submit 8.47 |
| warm-up | 4.42 | 70.8 | pipelines/compile_ms 1214, pipelines/front_ms 1171, main/busy/ms 95.63, stage/main/update 59.31, pipelines/wait_ms 55.34, stream/drive/ms 53.64, pipelines/driver_ms 43.69, render/thread/ms 41.31, render/busy/ms 38.31, stage/main/post_update 25.90, stage/render/render 20.85, render/graph/ms 17.95 |
| warm-up | 4.47 | 48.4 | pipelines/compile_ms 940, pipelines/front_ms 907, main/busy/ms 127, pipelines/wait_ms 99.56, stage/main/update 85.70, stream/drive/ms 55.61, render/thread/ms 52.11, render/busy/ms 49.04, pipelines/driver_ms 32.77, stage/main/post_update 30.78, stage/render/prepare_assets 18.98, stage/render/render 16.90 |
| warm-up | 4.60 | 126.9 | pipelines/compile_ms 573, pipelines/front_ms 548, main/busy/ms 105, stage/main/update 86.98, pipelines/wait_ms 80.60, render/thread/ms 74.38, render/busy/ms 72.57, stream/drive/ms 54.57, stage/render/prepare_assets 40.30, pipelines/driver_ms 25.14, stage/render/render 14.42, stage/main/post_update 12.22 |
| warm-up | 4.64 | 40.8 | pipelines/compile_ms 357, pipelines/front_ms 342, render/thread/ms 80.76, render/busy/ms 78.36, main/busy/ms 39.80, stage/render/prepare_assets 39.65, stage/main/update 23.44, pipelines/wait_ms 21.35, encode//sum 19.18, stage/render/render 18.78, render/graph/ms 15.88, pipelines/driver_ms 15.35 |
| warm-up | 4.67 | 37.9 | pipelines/compile_ms 101, pipelines/front_ms 94.56, render/thread/ms 66.85, render/busy/ms 64.60, main/busy/ms 39.73, encode//sum 29.30, stage/render/render 22.86, stage/render/prepare_assets 21.40, stage/main/update 21.11, render/graph/ms 19.97, stage/main/post_update 11.79, stage/render/graph_submit 11.59 |
| warm-up | 4.72 | 46.2 | pipelines/compile_ms 93.02, pipelines/front_ms 87.36, render/thread/ms 70.63, render/busy/ms 67.56, main/busy/ms 42.50, encode//sum 31.67, stage/render/render 26.79, render/graph/ms 24.27, stage/main/update 22.26, stage/render/prepare_assets 17.79, stage/render/graph_submit 13.84, stage/main/post_update 11.63 |
| warm-up | 4.77 | 44.3 | pipelines/compile_ms 105, pipelines/front_ms 102, render/thread/ms 73.94, render/busy/ms 71.00, main/busy/ms 47.03, encode//sum 35.72, stage/main/update 27.64, stage/render/render 25.71, render/graph/ms 23.34, stage/render/prepare_assets 22.81, stage/render/graph_submit 13.78, encode/pass:shadow_point_light/sum 12.41 |
| warm-up | 4.83 | 60.3 | pipelines/compile_ms 105, pipelines/front_ms 102, render/thread/ms 85.87, render/busy/ms 83.56, main/busy/ms 75.85, stage/main/update 49.86, encode//sum 43.61, stage/render/render 34.99, stream/drive/ms 34.28, render/graph/ms 31.83, stage/render/prepare_assets 23.42, stage/render/graph_submit 21.65 |
| warm-up | 4.89 | 62.1 | render/thread/ms 96.34, render/busy/ms 93.62, main/busy/ms 85.46, stage/main/update 54.83, stream/drive/ms 44.06, stage/render/render 41.77, encode//sum 41.49, render/graph/ms 38.27, stage/render/graph_submit 25.43, stage/render/prepare_assets 21.33, encode/pass:shadow_point_light/sum 17.95, encode/submit 16.30 |
| warm-up | 4.95 | 60.7 | pipelines/compile_ms 378, pipelines/front_ms 364, render/thread/ms 93.97, render/busy/ms 91.77, main/busy/ms 63.51, stage/main/update 42.76, encode//sum 39.54, stage/render/render 36.77, render/graph/ms 33.94, stream/drive/ms 32.95, stage/render/prepare_assets 24.73, stage/render/graph_submit 20.93 |
| warm-up | 5.00 | 50.6 | pipelines/compile_ms 378, pipelines/front_ms 364, render/thread/ms 92.64, render/busy/ms 89.88, encode//sum 51.80, main/busy/ms 46.87, stage/render/render 42.38, render/graph/ms 38.82, stage/main/update 31.31, stage/render/graph_submit 27.22, stream/drive/ms 23.20, stage/render/prepare_assets 21.59 |
| warm-up | 5.12 | 97.2 | pipelines/wait_ms 378, pipelines/compile_ms 368, pipelines/front_ms 347, render/thread/ms 65.76, render/busy/ms 63.73, main/busy/ms 44.12, stage/render/prepare_assets 34.91, stage/main/update 26.90, pipelines/driver_ms 21.31, stream/drive/ms 20.58, stage/render/render 18.48, render/graph/ms 15.80 |
| warm-up | 5.20 | 79.7 | pipelines/wait_ms 1125, pipelines/compile_ms 756, pipelines/front_ms 713, render/thread/ms 103, render/busy/ms 102, stage/render/prepare_assets 77.00, pipelines/driver_ms 43.56, main/busy/ms 34.74, stage/render/render 15.55, stage/main/update 13.62, render/graph/ms 13.42, encode//sum 11.42 |
| warm-up | 5.23 | 35.9 | pipelines/wait_ms 747, pipelines/compile_ms 388, pipelines/front_ms 366, render/thread/ms 90.27, render/busy/ms 86.94, stage/render/prepare_assets 48.64, main/busy/ms 36.22, stage/render/render 30.11, render/graph/ms 26.26, pipelines/driver_ms 22.25, stage/render/graph_submit 21.35, encode/submit 20.60 |
| warm-up | 5.28 | 27.0 | pipelines/wait_ms 367, pipelines/compile_ms 98.06, pipelines/front_ms 93.58, render/thread/ms 34.11, main/busy/ms 33.30, render/busy/ms 32.60, stage/main/update 20.67, stage/render/render 17.14, render/graph/ms 14.97, encode//sum 12.56, stream/drive/ms 9.77, stage/render/graph_submit 9.04 |
| warm-up | 5.33 | 29.9 | pipelines/wait_ms 768, pipelines/compile_ms 191, pipelines/front_ms 183, main/busy/ms 39.13, render/thread/ms 36.99, render/busy/ms 34.54, stage/main/update 26.47, stream/drive/ms 18.72, stage/render/render 16.02, render/graph/ms 13.15, encode//sum 11.05, stage/render/graph_submit 8.06 |
| warm-up | 5.42 | 36.7 | pipelines/wait_ms 465, pipelines/compile_ms 104, pipelines/front_ms 98.98, render/thread/ms 47.13, render/busy/ms 45.76, main/busy/ms 24.75, stage/render/prepare_assets 19.44, stage/render/render 14.87, render/graph/ms 12.92, encode//sum 10.89, stage/main/update 10.09, stage/main/post_update 9.13 |
| warm-up | 5.47 | 26.9 | render/thread/ms 43.81, render/busy/ms 42.08, main/busy/ms 38.55, stage/render/render 20.14, render/graph/ms 17.67, stage/main/post_update 14.06, stage/main/update 12.90, encode//sum 11.16, stage/render/graph_submit 11.03, stage/render/prepare_assets 9.62, stage/main/pre_update 8.46, encode/submit 8.29 |
| warm-up | 5.50 | 26.8 | render/thread/ms 42.91, render/busy/ms 41.58, main/busy/ms 34.59, stage/render/render 15.34, stage/render/prepare_assets 14.42, render/graph/ms 13.44, stage/main/update 12.15, encode//sum 10.67, stage/main/post_update 10.39, stage/main/pre_update 9.16, stage/render/graph_submit 8.03, encode/pass:shadow_point_light/sum 7.23 |
| warm-up | 5.53 | 29.9 | render/thread/ms 45.85, render/busy/ms 44.61, main/busy/ms 27.18, stage/render/prepare_assets 23.16, stage/main/update 13.48, stage/render/render 12.40, render/graph/ms 10.66, encode//sum 9.68, stage/main/post_update 8.37, stage/render/graph_submit 6.83, encode/pass:shadow_point_light/sum 6.41, encode/submit 4.59 |
| warm-up | 5.58 | 28.2 | render/thread/ms 39.28, main/busy/ms 38.29, render/busy/ms 37.83, stage/main/update 23.89, stage/render/render 16.88, render/graph/ms 14.87, stream/drive/ms 12.01, encode//sum 11.40, stage/render/graph_submit 9.37, stage/main/post_update 8.65, stage/render/prepare_assets 7.78, encode/pass:shadow_point_light/sum 6.97 |
| warm-up | 5.64 | 26.4 | pipelines/compile_ms 4931, pipelines/driver_ms 4427, pipelines/front_ms 504, render/thread/ms 40.28, render/busy/ms 38.25, pipelines/wait_ms 31.39, main/busy/ms 26.94, stage/render/render 26.69, render/graph/ms 24.10, stage/render/graph_submit 16.91, encode/pass:shadow_point_light/sum 14.25, stage/main/post_update 13.76 |
| warm-up | 5.68 | 38.1 | pipelines/compile_ms 8668, pipelines/driver_ms 7813, pipelines/front_ms 855, render/thread/ms 57.88, render/busy/ms 53.61, main/busy/ms 50.93, pipelines/wait_ms 42.75, stage/render/render 31.96, stage/main/post_update 27.90, render/graph/ms 27.23, stage/render/graph_submit 19.30, stage/main/update 13.57 |
| warm-up | 5.73 | 55.9 | pipelines/compile_ms 4422, pipelines/driver_ms 3405, pipelines/front_ms 1017, render/thread/ms 66.98, render/busy/ms 63.31, main/busy/ms 52.85, stage/render/prepare_assets 26.17, stage/main/post_update 24.00, stage/render/render 21.66, stage/main/update 17.63, render/graph/ms 17.58, pipelines/wait_ms 17.04 |
| warm-up | 5.76 | 26.9 | pipelines/compile_ms 935, pipelines/front_ms 909, render/thread/ms 55.49, render/busy/ms 54.10, pipelines/wait_ms 44.02, main/busy/ms 30.10, stage/render/prepare_assets 29.87, pipelines/driver_ms 25.08, stage/main/update 14.34, stage/render/render 13.42, render/graph/ms 11.44, encode//sum 10.67 |

## Run

- `target/profiling/perftests.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set stream.interior.disable_exterior=on --resolution 2560x1440 --bench-out target/perf\pt_census\census@2560x1440_arm=park-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (220 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **82%** (min 73%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.1 W · 73 °C (max 79) · VRAM 5423 MiB · throttled 100% of samples

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
- `stream.interior.disable_exterior=on (command line: --set)`
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
