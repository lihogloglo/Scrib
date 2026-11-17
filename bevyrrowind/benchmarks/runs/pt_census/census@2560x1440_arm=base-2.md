# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1600 frames over 11.4 s · **median 6.70 ms (149 fps)** · mean 7.14 · p95 9.42 · p99 10.15 (1% low 99 fps) · worst 14.82 ms · 22 spikes over 1.5× median · 57.7 stutters/min (work 21.0/min)
- warm-up: 120 frames over 6.1 s, worst 2569.2 ms
- hitches (> 25.0 ms): **0** measured, 41 in warm-up
- timed GPU passes: 5.11 ms of a 7.14 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 149.3, target 144
- pass worst frame 14.82 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.954 | 2.028 | 2.067 | 0.96 | 0.163 |
| main_opaque_pass_3d | 1.788 | 1.855 | 2.545 | 0.96 | 0.032 |
| bloom | 0.310 | 0.322 | 0.327 | 0.96 | 0.062 |
| auxiliary_views | 0.285 | 0.296 | 0.934 | 0.96 | 0.283 |
| ssao | 0.182 | 0.189 | 0.193 | 0.96 | 0.006 |
| upscaling | 0.146 | 0.151 | 0.172 | 0.96 | 0.004 |
| tlas_build | 0.109 | 0.113 | 0.118 | 0.96 | 0.071 |
| tonemapping | 0.096 | 0.100 | 0.105 | 0.96 | 0.004 |
| gi/probe_update | 0.089 | 0.092 | 0.098 | 0.96 | 0.002 |
| early prepass | 0.074 | 0.077 | 0.090 | 0.96 | 0.034 |
| clustering | 0.070 | 0.072 | 0.075 | 0.96 | 0.018 |
| uniform_allocation | 0.034 | 0.035 | 0.044 | 0.96 | 0.010 |
| bin_unpacking | 0.030 | 0.031 | 0.039 | 0.96 | 0.008 |
| early_mesh_preprocessing | 0.013 | 0.013 | 0.014 | 0.96 | 0.004 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.010 | 0.94 | 0.023 |
| setup_tlas_instances | 0.007 | 0.007 | 0.007 | 0.96 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.28 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.96 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.994 | 8.394 | 13.85 | 9589.7 |
| render/busy/ms | 5.548 | 7.925 | 13.44 | 8877.6 |
| main/busy/ms | 5.311 | 6.091 | 7.26 | 8497.6 |
| stage/render/render | 3.790 | 5.460 | 11.00 | 6064.7 |
| render/graph/ms | 3.196 | 4.797 | 10.44 | 5112.9 |
| stage/main/post_update | 2.925 | 3.518 | 4.86 | 4679.8 |
| encode//sum | 2.378 | 3.670 | 9.53 | 3804.5 |
| stage/render/graph_submit | 2.137 | 3.168 | 9.15 | 3419.0 |
| encode/submit | 1.429 | 2.173 | 2.95 | 2285.8 |
| stage/main/update | 1.238 | 1.468 | 1.88 | 1981.5 |
| encode/pass:shadow_point_light/sum | 1.010 | 3.081 | 4.26 | 1615.3 |
| stage/render/camera_driver | 0.933 | 1.543 | 2.10 | 1492.0 |
| encode/finish_wall | 0.729 | 1.091 | 7.12 | 1167.2 |
| stage/render/prepare_write_phase_buffers | 0.416 | 0.793 | 1.27 | 665.8 |
| encode/pass:early prepass/sum | 0.404 | 0.546 | 5.84 | 645.8 |
| stage/render/prepare_resources | 0.398 | 0.476 | 0.80 | 636.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.391 | 0.543 | 1.03 | 625.5 |
| render/present/ms | 0.364 | 0.516 | 1.85 | 582.8 |
| stage/main/first | 0.337 | 0.425 | 1.36 | 539.9 |
| stage/main/pre_update | 0.325 | 0.400 | 0.66 | 519.8 |
| stage/render/prepare_bind_groups | 0.287 | 0.594 | 0.94 | 458.8 |
| bigspace/total/ms | 0.242 | 0.572 | 1.25 | 386.6 |
| stage/render/prepare_assets | 0.207 | 0.258 | 1.37 | 331.3 |
| stage/render/prepare_meshes | 0.204 | 0.274 | 0.50 | 326.5 |
| stage/main/fixed_main_loop | 0.201 | 0.521 | 0.99 | 322.2 |
| stage/render/extract_commands | 0.148 | 0.175 | 0.39 | 236.1 |
| stage/render/create_views | 0.147 | 0.183 | 0.50 | 234.8 |
| stream/drive/ms | 0.142 | 0.184 | 0.35 | 226.8 |
| stage/render/prepare_flush | 0.135 | 0.252 | 0.60 | 216.3 |
| stage/render/prepare_views | 0.125 | 0.167 | 0.55 | 200.6 |
| stream/tick/ms | 0.114 | 0.154 | 0.32 | 183.0 |
| stage/main/last | 0.111 | 0.145 | 1.26 | 177.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.110 | 0.157 | 0.66 | 176.4 |
| bigspace/low_precision/ms | 0.092 | 0.250 | 1.14 | 146.7 |
| render/acquire/ms | 0.081 | 0.108 | 0.44 | 129.4 |
| stage/render/specialize | 0.079 | 0.102 | 0.56 | 125.7 |
| anim/evaluate/ms | 0.069 | 0.184 | 0.74 | 110.2 |
| controller/feed/ms | 0.068 | 0.084 | 0.59 | 108.3 |
| stage/render/prepare_batch_phases | 0.066 | 0.110 | 0.40 | 105.3 |
| physics/step/ms | 0.061 | 0.215 | 0.48 | 98.0 |
| bigspace/high_precision/ms | 0.054 | 0.215 | 0.82 | 87.0 |
| stage/render/phase_sort | 0.054 | 0.086 | 0.28 | 86.5 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.070 | 0.27 | 76.1 |
| stage/render/queue | 0.040 | 0.071 | 0.38 | 64.8 |
| anim/evaluate_cpu/ms | 0.035 | 0.051 | 0.41 | 55.4 |
| encode/resolve_diagnostics/sum | 0.031 | 0.040 | 0.62 | 49.1 |
| weather/map/ms | 0.026 | 0.037 | 0.28 | 41.0 |
| stage/render/cleanup | 0.022 | 0.034 | 0.27 | 35.3 |
| far/select/ms | 0.016 | 0.019 | 0.20 | 24.9 |
| weather/eval/ms | 0.012 | 0.018 | 0.06 | 19.2 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.018 | 0.48 | 18.7 |
| physics/insert/ms | 0.010 | 0.000 | 0.64 | 16.7 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.23 | 16.2 |
| script/update/ms | 0.008 | 0.014 | 0.23 | 13.1 |
| controller/move/ms | 0.006 | 0.008 | 0.03 | 9.6 |
| script/input/ms | 0.005 | 0.006 | 0.19 | 8.4 |
| anim/apply/ms | 0.005 | 0.010 | 0.02 | 7.6 |
| controller/avoid/ms | 0.003 | 0.004 | 0.02 | 5.4 |
| script/ui/ms | 0.003 | 0.004 | 0.21 | 4.6 |
| controller/write/ms | 0.001 | 0.001 | 0.01 | 1.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.05 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.4 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stage/render/prepare | 0.001 | 0.002 | 0.03 | 0.9 |
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
| encode//max | ms | 0.39 | 7.04 | 0.43 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.48 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.23 | 0.02 |
| encode/finish_critical | ms | 0.49 | 7.04 | 0.45 |
| encode/pass:early prepass/max | ms | 0.41 | 5.84 | 0.45 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.40 | 1.03 | 0.42 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.64 | 0.09 |
| encode/pass:shadow_point_light/max | ms | 0.49 | 1.15 | 0.47 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.62 | 0.02 |
| far/entities | count | 162 | 162 | 162 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.6 k | 41.6 k | 41.6 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.26 M | 86.26 M | 86.26 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.56 | 13.44 | 4.64 |
| gpu/timed/ms | ms | 5.17 | 7.07 | 4.64 |
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
| physics/pending | count | 0 | 93 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 286 | 286 | 286 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 2 | 182 | 0 |
| preprocess/bind_groups_created | count | 2 | 182 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 47 | 103 | 35 |
| process/cpu_usage | % | 23.08 | 34.62 | 23.08 |
| process/mem_usage | GiB | 3.79 | 3.85 | 3.85 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
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
| system/cpu_usage | % | 29.87 | 38.84 | 30.58 |
| system/mem_usage | % | 57.18 | 57.38 | 57.38 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2038 | 2038 | 2038 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 18 | 104 | 9 |
| views/all/bins | count | 2970 | 3305 | 2223 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 4157 | 4588 | 3183 |
| views/all/pipelines | count | 247 | 261 | 198 |
| views/all/pipelines_distinct | count | 22 | 28 | 16 |
| views/all/work_items | count | 578 | 2391 | 78 |
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
| anim/bones_written/count | count | 8.40 | 30 | 13.4 k |
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
| encode//count | count | 31.01 | 74 | 49.6 k |
| encode/bind_walk/init_textures | count | 8606 | 23.8 k | 13.77 M |
| encode/bind_walk/init_textures_skipped | count | 17.9 k | 60.6 k | 28.65 M |
| encode/bind_walk/inits | count | 195 | 420 | 312.3 k |
| encode/bind_walk/inits_skipped | count | 20.68 | 65 | 33.1 k |
| encode/bind_walk/merge_textures | count | 7363 | 22.6 k | 11.78 M |
| encode/bind_walk/merge_textures_skipped | count | 32.44 | 77 | 51.9 k |
| encode/bind_walk/merges | count | 59.92 | 120 | 95.9 k |
| encode/bind_walk/merges_skipped | count | 7.98 | 23 | 12.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1600 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1600 |
| encode/pass:early prepass/count | count | 1 | 2 | 1600 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1600 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1554 |
| encode/pass:shadow_point_light/count | count | 3.18 | 14 | 5086 |
| encode/queue/command_buffers | count | 43.16 | 97 | 69.0 k |
| encode/queue/submits | count | 2 | 4 | 3200 |
| encode/queue/write_buffer/bytes | bytes | 474.1 k | 885.1 k | 758.53 M |
| encode/queue/write_buffer/calls | count | 150 | 359 | 240.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 158.0 k | 315.9 k | 252.72 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9600 |
| encode/queue/write_texture/bytes | bytes | 11.1 k | 8.39 M | 17.83 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1600 |
| physics/inserts | count | 0.06 | 10 | 94 |
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

## Hitches (> 25.0 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2569.2 | main/busy/ms 2540, stage/main/post_update 2418, stage/main/update 113, surface/advance_cpu_ms 80.36, stage/main/pre_update 4.74, stage/main/last 1.15, stage/main/fixed_main_loop 0.55, stage/main/first 0.49, stream/drive/ms 0.41, weather/map/ms 0.24, bigspace/total/ms 0.09, stream/tick/ms 0.07 |
| warm-up | 3.33 | 757.1 | main/busy/ms 2660, stage/main/post_update 2424, render/thread/ms 724, render/busy/ms 546, stage/render/prepare_assets 525, stage/main/update 222, stage/render/prepare_views 176, render/acquire/ms 175, surface/advance_cpu_ms 80.36, stage/render/render 10.80, stage/render/prepare_resources 8.06, render/graph/ms 7.13 |
| warm-up | 3.41 | 88.3 | render/thread/ms 796, render/busy/ms 571, stage/render/prepare_assets 544, stage/render/prepare_views 176, render/acquire/ms 175, main/busy/ms 130, stage/main/update 113, stage/render/render 61.05, render/present/ms 50.13, render/graph/ms 10.07, stage/render/prepare_resources 8.61, stage/main/post_update 7.65 |
| warm-up | 3.45 | 32.5 | pipelines/wait_ms 153, pipelines/compile_ms 112, pipelines/front_ms 101, render/thread/ms 96.94, stage/render/render 71.09, render/present/ms 54.05, main/busy/ms 45.17, render/busy/ms 42.29, stage/main/update 25.96, stage/render/prepare_assets 20.85, render/graph/ms 16.55, stage/main/post_update 12.11 |
| warm-up | 3.47 | 26.6 | pipelines/wait_ms 1207, pipelines/compile_ms 509, pipelines/front_ms 474, main/busy/ms 52.53, render/thread/ms 44.65, render/busy/ms 36.68, pipelines/driver_ms 35.11, stage/main/update 27.74, stage/render/render 24.32, pipelines/lock_ms 17.84, render/graph/ms 16.28, stage/main/post_update 13.27 |
| warm-up | 3.50 | 28.7 | pipelines/wait_ms 1055, pipelines/compile_ms 475, pipelines/front_ms 446, render/thread/ms 47.76, render/busy/ms 45.87, main/busy/ms 30.96, pipelines/driver_ms 28.76, stage/render/prepare_bind_groups 18.21, stage/main/update 15.15, stage/render/prepare_assets 13.62, pipelines/lock_ms 9.00, stage/render/render 8.75 |
| warm-up | 3.53 | 25.1 | pipelines/compile_ms 110, pipelines/front_ms 105, render/thread/ms 51.42, render/busy/ms 46.78, main/busy/ms 20.26, stage/render/prepare_bind_groups 18.14, stage/render/prepare_assets 14.31, stage/render/render 13.39, stage/main/update 11.34, render/graph/ms 8.83, pipelines/wait_ms 8.44, encode//sum 8.29 |
| warm-up | 3.55 | 26.2 | pipelines/compile_ms 311, pipelines/front_ms 299, main/busy/ms 70.40, stage/main/update 58.55, pipelines/wait_ms 54.48, render/thread/ms 43.42, render/busy/ms 38.92, stage/render/prepare_assets 14.69, stage/render/render 12.31, pipelines/driver_ms 11.71, stage/render/prepare_resources 8.87, render/graph/ms 7.90 |
| warm-up | 3.67 | 117.6 | pipelines/compile_ms 1411, pipelines/front_ms 1351, pipelines/wait_ms 163, main/busy/ms 76.61, render/thread/ms 72.46, render/busy/ms 67.62, stage/main/update 61.20, pipelines/driver_ms 59.45, stage/render/prepare_assets 39.23, stage/render/render 17.38, render/graph/ms 12.57, stage/main/post_update 10.08 |
| warm-up | 3.72 | 31.5 | pipelines/compile_ms 295, pipelines/front_ms 283, render/thread/ms 43.26, render/busy/ms 41.76, stage/render/prepare_assets 32.74, main/busy/ms 17.49, pipelines/driver_ms 12.41, stage/main/update 6.86, stage/render/render 6.77, pipelines/wait_ms 5.24, render/graph/ms 5.00, stage/main/post_update 4.72 |
| warm-up | 3.79 | 45.9 | pipelines/compile_ms 593, pipelines/front_ms 571, main/busy/ms 43.71, render/thread/ms 41.57, render/busy/ms 39.26, stage/main/update 25.26, pipelines/driver_ms 22.50, stage/render/render 21.29, render/graph/ms 18.80, stage/render/prepare_assets 15.80, stream/drive/ms 15.49, stage/render/graph_submit 14.85 |
| warm-up | 3.82 | 33.1 | pipelines/compile_ms 1003, pipelines/front_ms 959, render/thread/ms 46.73, render/busy/ms 44.82, pipelines/driver_ms 43.74, stage/render/prepare_assets 30.20, main/busy/ms 29.18, stage/main/post_update 14.14, pipelines/wait_ms 12.71, stage/render/render 10.99, stage/main/update 9.33, render/graph/ms 8.97 |
| warm-up | 3.88 | 61.2 | pipelines/compile_ms 854, pipelines/front_ms 818, render/thread/ms 82.22, render/busy/ms 80.64, main/busy/ms 44.48, stage/render/prepare_assets 41.51, pipelines/driver_ms 35.29, stage/render/prepare_resources 19.87, stage/main/post_update 17.04, stage/main/update 15.05, pipelines/wait_ms 12.37, stage/render/prepare_flush 8.04 |
| warm-up | 3.91 | 28.3 | pipelines/compile_ms 957, pipelines/front_ms 927, render/thread/ms 74.19, render/busy/ms 72.92, main/busy/ms 65.19, stage/main/update 44.81, stream/drive/ms 35.79, stage/render/prepare_assets 35.05, pipelines/driver_ms 29.59, stage/render/prepare_resources 19.51, stage/main/post_update 8.85, stage/render/prepare_flush 8.01 |
| warm-up | 3.96 | 50.4 | pipelines/compile_ms 1325, pipelines/front_ms 1284, pipelines/wait_ms 57.76, main/busy/ms 49.17, pipelines/driver_ms 41.11, stage/main/update 39.71, render/thread/ms 38.68, render/busy/ms 37.39, stream/drive/ms 35.80, stage/render/prepare_assets 26.04, stage/render/render 8.53, render/graph/ms 7.08 |
| warm-up | 4.02 | 33.7 | pipelines/compile_ms 581, pipelines/front_ms 537, pipelines/wait_ms 115, pipelines/driver_ms 43.87, main/busy/ms 38.86, render/thread/ms 31.32, render/busy/ms 24.56, stage/main/update 18.74, stage/render/prepare_assets 13.83, stage/render/render 12.61, stream/drive/ms 11.31, stage/main/post_update 7.77 |
| warm-up | 4.04 | 28.4 | pipelines/compile_ms 791, pipelines/front_ms 742, pipelines/wait_ms 101, pipelines/driver_ms 49.23, render/thread/ms 35.41, render/busy/ms 33.12, main/busy/ms 22.75, stage/render/prepare_assets 21.84, stage/main/post_update 13.08, stage/render/render 8.48, render/graph/ms 6.11, stage/main/update 4.84 |
| warm-up | 4.07 | 27.9 | pipelines/compile_ms 579, pipelines/front_ms 558, render/thread/ms 43.97, render/busy/ms 42.44, pipelines/wait_ms 33.31, main/busy/ms 27.75, pipelines/driver_ms 20.80, stage/render/prepare_assets 19.32, stage/main/post_update 18.62, stage/render/prepare_meshes 6.72, stage/render/render 6.62, stage/render/prepare_resources 5.78 |
| warm-up | 4.13 | 32.1 | pipelines/compile_ms 767, pipelines/front_ms 745, render/thread/ms 46.42, render/busy/ms 45.17, main/busy/ms 25.94, stage/render/prepare_assets 23.19, pipelines/driver_ms 22.48, stage/main/update 13.99, stage/render/prepare_flush 10.28, stage/main/post_update 7.25, stage/render/render 5.76, pipelines/wait_ms 5.37 |
| warm-up | 4.40 | 78.4 | pipelines/compile_ms 1153, pipelines/front_ms 1121, main/busy/ms 88.49, stage/main/update 51.27, stream/drive/ms 45.00, pipelines/wait_ms 39.81, render/thread/ms 38.60, render/busy/ms 36.40, pipelines/driver_ms 32.13, stage/main/post_update 27.55, stage/render/render 24.26, render/graph/ms 21.95 |
| warm-up | 4.43 | 35.1 | pipelines/compile_ms 1120, pipelines/front_ms 1087, main/busy/ms 114, stage/main/update 74.06, render/thread/ms 51.00, render/busy/ms 49.07, stream/drive/ms 43.68, pipelines/wait_ms 41.22, pipelines/driver_ms 33.33, stage/main/post_update 30.98, stage/render/render 27.86, render/graph/ms 25.38 |
| warm-up | 4.54 | 109.5 | pipelines/compile_ms 1117, pipelines/front_ms 1073, main/busy/ms 91.33, stage/main/update 74.65, render/thread/ms 56.88, render/busy/ms 55.12, pipelines/driver_ms 43.85, stream/drive/ms 42.88, stage/render/prepare_assets 28.37, pipelines/wait_ms 17.10, pipelines/lock_ms 15.17, stage/render/render 14.48 |
| warm-up | 4.58 | 35.0 | pipelines/compile_ms 884, pipelines/front_ms 848, render/thread/ms 65.29, render/busy/ms 63.44, pipelines/driver_ms 35.85, stage/render/prepare_assets 27.44, main/busy/ms 24.63, encode//sum 21.43, stage/render/render 19.64, render/graph/ms 17.23, pipelines/wait_ms 15.42, pipelines/lock_ms 13.54 |
| warm-up | 4.61 | 35.5 | pipelines/compile_ms 94.44, pipelines/front_ms 88.64, render/thread/ms 63.56, render/busy/ms 61.38, encode//sum 59.47, stage/render/render 32.20, render/graph/ms 29.45, main/busy/ms 26.42, stage/render/graph_submit 18.79, encode/pass:shadow_point_light/sum 12.67, stage/render/prepare_assets 10.84, stage/main/update 10.32 |
| warm-up | 4.66 | 52.0 | pipelines/compile_ms 289, pipelines/front_ms 273, render/thread/ms 71.83, render/busy/ms 69.81, encode//sum 51.55, main/busy/ms 46.42, stage/render/render 28.48, render/graph/ms 25.77, stage/main/update 24.74, stage/render/prepare_assets 21.11, stage/render/graph_submit 17.37, pipelines/driver_ms 15.81 |
| warm-up | 4.71 | 44.5 | pipelines/compile_ms 522, pipelines/front_ms 502, render/thread/ms 73.74, render/busy/ms 72.40, main/busy/ms 45.48, stage/render/prepare_assets 28.77, stage/main/update 26.89, pipelines/driver_ms 20.73, stage/render/render 17.67, render/graph/ms 15.48, encode//sum 15.34, stage/main/post_update 12.45 |
| warm-up | 4.77 | 57.6 | pipelines/compile_ms 309, pipelines/front_ms 299, render/thread/ms 85.67, render/busy/ms 77.21, main/busy/ms 73.14, stage/main/update 59.65, stream/drive/ms 44.04, stage/render/render 40.54, render/graph/ms 31.46, stage/render/prepare_assets 19.27, stage/render/graph_submit 18.04, encode//sum 17.68 |
| warm-up | 4.85 | 80.8 | pipelines/compile_ms 392, pipelines/front_ms 377, render/thread/ms 106, encode//sum 99.27, render/busy/ms 96.66, main/busy/ms 77.50, stage/render/render 62.30, stage/main/update 57.24, render/graph/ms 52.76, stream/drive/ms 44.67, stage/render/graph_submit 36.24, encode/pass:shadow_point_light/sum 33.44 |
| warm-up | 4.89 | 44.9 | pipelines/compile_ms 1145, pipelines/front_ms 1107, encode//sum 99.83, render/thread/ms 93.40, render/busy/ms 91.74, main/busy/ms 51.88, stage/render/render 45.55, render/graph/ms 43.19, pipelines/driver_ms 38.26, encode/pass:shadow_point_light/sum 35.56, stage/main/update 29.92, stage/render/graph_submit 29.80 |
| warm-up | 4.94 | 51.0 | pipelines/compile_ms 814, pipelines/front_ms 787, render/thread/ms 82.31, render/busy/ms 80.50, main/busy/ms 44.75, stage/render/render 43.07, render/graph/ms 40.49, stage/main/update 29.11, pipelines/driver_ms 27.18, stage/render/graph_submit 23.53, stream/drive/ms 22.10, stage/render/prepare_assets 17.80 |
| warm-up | 4.97 | 29.7 | pipelines/compile_ms 92.05, pipelines/front_ms 86.96, render/thread/ms 70.58, render/busy/ms 68.13, main/busy/ms 55.66, stage/render/render 42.27, render/graph/ms 39.18, stage/main/post_update 25.41, stage/main/update 24.19, stage/render/graph_submit 23.81, encode/submit 15.13, stage/render/camera_driver 14.57 |
| warm-up | 5.09 | 118.6 | pipelines/compile_ms 1180, pipelines/front_ms 1137, render/thread/ms 85.92, render/busy/ms 83.94, main/busy/ms 63.55, pipelines/driver_ms 43.27, stage/render/prepare_assets 37.12, stage/render/render 34.85, render/graph/ms 32.32, stage/main/post_update 28.51, stage/main/update 28.02, stage/render/graph_submit 24.21 |
| warm-up | 5.15 | 63.3 | pipelines/compile_ms 1498, pipelines/front_ms 1446, render/thread/ms 95.66, render/busy/ms 94.02, stage/render/prepare_assets 54.26, pipelines/driver_ms 51.35, main/busy/ms 34.55, stage/render/render 31.35, render/graph/ms 29.13, stage/render/graph_submit 24.22, stage/main/update 12.93, stage/main/post_update 12.50 |
| warm-up | 5.19 | 34.0 | pipelines/compile_ms 872, pipelines/front_ms 838, render/thread/ms 68.89, render/busy/ms 67.03, main/busy/ms 44.57, stage/render/render 34.32, pipelines/driver_ms 33.62, render/graph/ms 31.98, stage/render/graph_submit 25.55, stage/main/update 23.57, stage/render/prepare_assets 22.10, encode/submit 19.08 |
| warm-up | 5.25 | 27.2 | main/busy/ms 38.56, render/thread/ms 34.80, render/busy/ms 33.40, stage/main/update 25.69, stream/drive/ms 18.71, stage/render/render 14.73, encode//sum 14.02, render/graph/ms 12.62, stage/render/graph_submit 7.90, stage/main/post_update 7.86, encode/pass:shadow_point_light/sum 7.44, stage/render/prepare_assets 7.08 |
| warm-up | 5.34 | 30.6 | pipelines/compile_ms 192, pipelines/front_ms 187, render/thread/ms 40.05, render/busy/ms 38.97, main/busy/ms 19.64, stage/render/prepare_assets 17.72, stage/render/render 13.62, render/graph/ms 12.13, encode//sum 10.58, stage/main/post_update 7.91, stage/main/update 7.86, stage/render/graph_submit 7.73 |
| warm-up | 5.41 | 26.5 | render/thread/ms 38.88, render/busy/ms 37.73, main/busy/ms 24.39, stage/render/prepare_assets 16.83, stage/render/render 12.45, render/graph/ms 10.92, stage/main/update 10.62, encode//sum 9.62, stage/main/post_update 9.49, stage/render/graph_submit 7.12, encode/pass:shadow_point_light/sum 5.50, encode/submit 4.76 |
| warm-up | 5.44 | 25.6 | render/thread/ms 42.60, render/busy/ms 41.38, main/busy/ms 24.07, stage/render/prepare_assets 22.21, stage/render/render 12.74, stage/main/update 12.66, render/graph/ms 11.10, encode//sum 9.10, stage/render/graph_submit 7.42, stage/main/post_update 7.24, encode/pass:shadow_point_light/sum 5.99, encode/submit 4.99 |
| warm-up | 5.54 | 31.5 | render/thread/ms 40.98, render/busy/ms 39.07, main/busy/ms 34.27, stage/render/render 22.94, render/graph/ms 20.32, stage/main/post_update 16.52, stage/render/graph_submit 14.21, encode//sum 13.06, stage/main/update 10.72, encode/submit 10.51, encode/pass:shadow_point_light/sum 9.43, stage/render/prepare_assets 7.52 |
| warm-up | 5.57 | 35.7 | render/thread/ms 47.04, render/busy/ms 45.91, main/busy/ms 37.48, stage/render/prepare_assets 20.18, stage/render/render 17.12, render/graph/ms 15.47, stage/main/update 15.45, stage/main/post_update 14.46, encode//sum 11.42, stage/render/graph_submit 11.02, encode/submit 7.60, encode/pass:shadow_point_light/sum 7.34 |
| warm-up | 5.60 | 25.4 | render/thread/ms 42.15, render/busy/ms 41.03, main/busy/ms 26.42, stage/render/prepare_assets 22.38, stage/main/update 14.51, stage/render/render 11.39, encode//sum 10.20, render/graph/ms 9.84, stage/main/post_update 7.38, stage/render/graph_submit 6.65, encode/pass:shadow_point_light/sum 6.34, encode/submit 4.25 |

## Run

- `target/profiling/perftests.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\pt_census\census@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 15%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (240 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **81%** (min 68%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.2 W · 73 °C (max 78) · VRAM 5427 MiB · throttled 100% of samples

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
