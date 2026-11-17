# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); stream.interior.disable_exterior=on (command line: --set); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1637 frames over 11.4 s · **median 6.53 ms (153 fps)** · mean 6.98 · p95 9.29 · p99 10.24 (1% low 98 fps) · worst 13.20 ms · 33 spikes over 1.5× median · 52.5 stutters/min (work 21.0/min)
- warm-up: 120 frames over 6.3 s, worst 2532.0 ms
- hitches (> 25.0 ms): **0** measured, 38 in warm-up
- timed GPU passes: 4.93 ms of a 6.98 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 153.2, target 144
- pass worst frame 13.20 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.883 | 2.028 | 2.068 | 0.93 | 0.162 |
| main_opaque_pass_3d | 1.726 | 1.859 | 2.549 | 0.93 | 0.030 |
| bloom | 0.299 | 0.322 | 0.327 | 0.93 | 0.061 |
| auxiliary_views | 0.274 | 0.295 | 0.939 | 0.93 | 0.275 |
| ssao | 0.176 | 0.190 | 0.194 | 0.93 | 0.007 |
| upscaling | 0.141 | 0.152 | 0.173 | 0.93 | 0.004 |
| tlas_build | 0.107 | 0.115 | 0.120 | 0.93 | 0.073 |
| tonemapping | 0.093 | 0.100 | 0.106 | 0.93 | 0.004 |
| gi/probe_update | 0.091 | 0.098 | 0.105 | 0.93 | 0.002 |
| early prepass | 0.073 | 0.078 | 0.087 | 0.93 | 0.034 |
| clustering | 0.068 | 0.073 | 0.076 | 0.93 | 0.018 |
| uniform_allocation | 0.034 | 0.037 | 0.045 | 0.93 | 0.009 |
| bin_unpacking | 0.029 | 0.032 | 0.039 | 0.93 | 0.008 |
| early_mesh_preprocessing | 0.012 | 0.013 | 0.014 | 0.93 | 0.004 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.010 | 0.90 | 0.022 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.93 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.28 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.93 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.803 | 8.263 | 12.07 | 9499.3 |
| render/busy/ms | 5.353 | 7.811 | 11.59 | 8762.8 |
| main/busy/ms | 5.240 | 6.019 | 7.11 | 8578.2 |
| stage/render/render | 3.686 | 5.395 | 9.10 | 6033.4 |
| render/graph/ms | 3.085 | 4.755 | 8.48 | 5050.4 |
| stage/main/post_update | 2.893 | 3.455 | 4.82 | 4736.0 |
| encode//sum | 2.273 | 3.601 | 7.86 | 3720.3 |
| stage/render/graph_submit | 2.050 | 3.136 | 7.02 | 3356.6 |
| encode/submit | 1.377 | 2.127 | 3.17 | 2253.5 |
| stage/main/update | 1.219 | 1.457 | 1.85 | 1995.4 |
| encode/pass:shadow_point_light/sum | 0.966 | 3.020 | 4.07 | 1580.7 |
| stage/render/camera_driver | 0.908 | 1.525 | 2.19 | 1486.8 |
| encode/finish_wall | 0.702 | 1.069 | 5.15 | 1148.5 |
| stage/render/prepare_write_phase_buffers | 0.405 | 0.799 | 1.22 | 663.0 |
| encode/pass:early prepass/sum | 0.396 | 0.549 | 4.00 | 648.9 |
| stage/render/prepare_resources | 0.385 | 0.467 | 0.81 | 630.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.384 | 0.532 | 1.16 | 628.7 |
| render/present/ms | 0.371 | 0.677 | 1.50 | 607.5 |
| stage/main/first | 0.332 | 0.414 | 1.76 | 544.2 |
| stage/main/pre_update | 0.319 | 0.397 | 0.65 | 522.3 |
| stage/render/prepare_bind_groups | 0.276 | 0.577 | 0.99 | 451.6 |
| bigspace/total/ms | 0.243 | 0.590 | 1.04 | 397.2 |
| stage/render/prepare_assets | 0.199 | 0.250 | 1.41 | 326.4 |
| stage/main/fixed_main_loop | 0.197 | 0.521 | 1.07 | 322.2 |
| stage/render/prepare_meshes | 0.183 | 0.247 | 0.44 | 299.7 |
| stage/render/create_views | 0.146 | 0.180 | 0.48 | 238.7 |
| stage/render/extract_commands | 0.140 | 0.167 | 0.30 | 229.9 |
| stage/render/prepare_flush | 0.135 | 0.222 | 0.45 | 221.7 |
| stream/drive/ms | 0.133 | 0.175 | 0.30 | 217.3 |
| stage/render/prepare_views | 0.120 | 0.161 | 0.35 | 197.2 |
| stage/main/last | 0.107 | 0.141 | 1.00 | 175.7 |
| stream/tick/ms | 0.106 | 0.147 | 0.27 | 174.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.103 | 0.149 | 0.60 | 169.1 |
| bigspace/low_precision/ms | 0.094 | 0.237 | 0.86 | 153.4 |
| render/acquire/ms | 0.079 | 0.107 | 0.30 | 129.0 |
| stage/render/specialize | 0.076 | 0.099 | 0.63 | 124.2 |
| anim/evaluate/ms | 0.073 | 0.211 | 0.90 | 119.2 |
| stage/render/prepare_batch_phases | 0.063 | 0.105 | 0.29 | 103.2 |
| controller/feed/ms | 0.063 | 0.078 | 0.52 | 103.2 |
| physics/step/ms | 0.062 | 0.218 | 0.42 | 100.9 |
| bigspace/high_precision/ms | 0.053 | 0.222 | 0.88 | 87.6 |
| stage/render/phase_sort | 0.053 | 0.087 | 0.31 | 86.9 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.069 | 0.17 | 76.1 |
| stage/render/queue | 0.039 | 0.068 | 0.35 | 63.2 |
| anim/evaluate_cpu/ms | 0.035 | 0.051 | 0.44 | 57.7 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.49 | 49.4 |
| weather/map/ms | 0.026 | 0.038 | 0.07 | 42.8 |
| stage/render/cleanup | 0.023 | 0.033 | 0.42 | 37.2 |
| far/select/ms | 0.015 | 0.018 | 0.15 | 24.6 |
| weather/eval/ms | 0.012 | 0.018 | 0.18 | 19.9 |
| physics/insert/ms | 0.012 | 0.000 | 0.63 | 19.0 |
| encode/camera_driver_span_begin/sum | 0.011 | 0.019 | 0.06 | 18.5 |
| encode/camera_driver_span_end/sum | 0.010 | 0.017 | 0.09 | 16.2 |
| script/update/ms | 0.008 | 0.013 | 0.03 | 12.5 |
| controller/move/ms | 0.006 | 0.007 | 0.02 | 9.7 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 7.8 |
| anim/apply/ms | 0.005 | 0.009 | 0.20 | 7.6 |
| controller/avoid/ms | 0.003 | 0.004 | 0.02 | 5.6 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 4.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.7 |
| controller/write/ms | 0.001 | 0.001 | 0.02 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 0.9 |
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
| encode//max | ms | 0.38 | 5.09 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.06 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.09 | 0.01 |
| encode/finish_critical | ms | 0.48 | 5.09 | 0.48 |
| encode/pass:early prepass/max | ms | 0.40 | 4.00 | 0.47 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.39 | 0.90 | 0.48 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.10 | 0.60 | 0.10 |
| encode/pass:shadow_point_light/max | ms | 0.48 | 0.99 | 0.38 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.49 | 0.05 |
| far/entities | count | 162 | 162 | 162 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.6 k | 41.6 k | 41.6 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.26 M | 86.26 M | 86.26 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.41 | 11.59 | 5.04 |
| gpu/timed/ms | ms | 5.18 | 7.07 | 4.61 |
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
| physics/pending | count | 0 | 113 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 285 | 285 | 285 |
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
| preprocess/bind_groups_reused | count | 47 | 91 | 35 |
| process/cpu_usage | % | 25.54 | 37.02 | 28.23 |
| process/mem_usage | GiB | 3.80 | 3.84 | 3.84 |
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
| system/cpu_usage | % | 30.41 | 34.58 | 34.58 |
| system/mem_usage | % | 57.09 | 57.24 | 57.24 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2038 | 2038 | 2038 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 18 | 104 | 9 |
| views/all/bins | count | 2970 | 3305 | 2223 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 4160 | 4588 | 3183 |
| views/all/pipelines | count | 247 | 261 | 198 |
| views/all/pipelines_distinct | count | 22 | 28 | 16 |
| views/all/work_items | count | 567 | 2411 | 78 |
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
| anim/bones_written/count | count | 8.33 | 30 | 13.6 k |
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
| encode//count | count | 30.63 | 71 | 50.1 k |
| encode/bind_walk/init_textures | count | 8474 | 22.4 k | 13.87 M |
| encode/bind_walk/init_textures_skipped | count | 17.8 k | 60.6 k | 29.14 M |
| encode/bind_walk/inits | count | 194 | 421 | 316.9 k |
| encode/bind_walk/inits_skipped | count | 20.57 | 65 | 33.7 k |
| encode/bind_walk/merge_textures | count | 7231 | 21.2 k | 11.84 M |
| encode/bind_walk/merge_textures_skipped | count | 32.41 | 77 | 53.1 k |
| encode/bind_walk/merges | count | 59.49 | 120 | 97.4 k |
| encode/bind_walk/merges_skipped | count | 7.94 | 22 | 13.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1637 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1637 |
| encode/pass:early prepass/count | count | 1 | 2 | 1637 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1637 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1589 |
| encode/pass:shadow_point_light/count | count | 3.09 | 13 | 5051 |
| encode/queue/command_buffers | count | 42.69 | 93 | 69.9 k |
| encode/queue/submits | count | 2 | 4 | 3274 |
| encode/queue/write_buffer/bytes | bytes | 472.6 k | 933.0 k | 773.72 M |
| encode/queue/write_buffer/calls | count | 148 | 361 | 243.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 158.0 k | 315.9 k | 258.57 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9822 |
| encode/queue/write_texture/bytes | bytes | 10.9 k | 8.39 M | 17.83 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1637 |
| physics/inserts | count | 0.07 | 11 | 114 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.42 | 1 | 686 |
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

## Hitches (> 25.0 ms; slowest 38, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2532.0 | main/busy/ms 2502, stage/main/post_update 2398, stage/main/update 95.87, surface/advance_cpu_ms 66.17, stage/main/pre_update 4.52, stage/main/last 1.02, stream/drive/ms 0.68, stage/main/first 0.55, stage/main/fixed_main_loop 0.41, weather/map/ms 0.28, stream/tick/ms 0.14, bigspace/total/ms 0.08 |
| warm-up | 3.39 | 857.0 | main/busy/ms 2622, stage/main/post_update 2403, render/thread/ms 826, render/busy/ms 639, stage/render/prepare_assets 605, stage/main/update 205, stage/render/prepare_views 183, render/acquire/ms 182, surface/advance_cpu_ms 66.17, stage/render/prepare_resources 21.64, stage/render/render 13.00, render/graph/ms 7.18 |
| warm-up | 3.55 | 159.0 | render/thread/ms 969, render/busy/ms 685, stage/render/prepare_assets 646, stage/render/prepare_views 183, render/acquire/ms 182, main/busy/ms 138, stage/main/update 117, stage/render/render 113, render/present/ms 102, stage/render/prepare_resources 22.39, render/graph/ms 10.25, stage/main/fixed_main_loop 8.37 |
| warm-up | 3.65 | 87.8 | pipelines/wait_ms 1724, pipelines/compile_ms 589, pipelines/front_ms 541, main/busy/ms 113, render/thread/ms 72.65, render/busy/ms 69.86, stage/main/update 69.61, stage/render/prepare_assets 55.20, pipelines/driver_ms 47.92, stage/main/post_update 15.69, stage/main/last 11.10, stage/render/render 10.64 |
| warm-up | 3.74 | 91.8 | pipelines/wait_ms 1806, pipelines/compile_ms 1524, pipelines/front_ms 1433, render/thread/ms 127, render/busy/ms 125, main/busy/ms 109, stage/render/prepare_assets 103, pipelines/driver_ms 91.23, stage/main/update 73.05, stage/render/render 17.69, render/graph/ms 15.14, pipelines/lock_ms 12.76 |
| warm-up | 3.77 | 27.9 | pipelines/compile_ms 1181, pipelines/front_ms 1128, pipelines/wait_ms 97.20, render/thread/ms 89.68, render/busy/ms 86.99, stage/render/prepare_assets 67.86, pipelines/driver_ms 53.66, main/busy/ms 38.79, stage/render/render 16.44, stage/main/fixed_main_loop 15.11, pipelines/lock_ms 14.65, render/graph/ms 13.92 |
| warm-up | 3.84 | 43.4 | pipelines/compile_ms 85.39, pipelines/front_ms 79.43, main/busy/ms 44.63, render/thread/ms 37.27, render/busy/ms 35.01, stage/main/update 25.06, stream/drive/ms 15.73, stage/main/post_update 15.26, stage/render/render 11.64, stage/render/prepare_assets 10.98, render/graph/ms 9.25, encode//sum 9.25 |
| warm-up | 3.88 | 42.2 | pipelines/compile_ms 585, pipelines/front_ms 563, render/thread/ms 61.19, render/busy/ms 59.37, main/busy/ms 37.17, stage/render/prepare_assets 36.15, pipelines/driver_ms 22.25, stage/main/update 19.81, stream/drive/ms 15.62, stage/main/post_update 10.75, pipelines/wait_ms 10.65, stage/render/render 10.48 |
| warm-up | 3.91 | 29.6 | pipelines/compile_ms 1152, pipelines/front_ms 1114, render/thread/ms 53.29, render/busy/ms 51.23, pipelines/driver_ms 38.05, stage/render/prepare_assets 37.96, main/busy/ms 25.15, pipelines/wait_ms 14.02, stage/render/render 11.33, stage/main/post_update 11.04, pipelines/lock_ms 9.97, render/graph/ms 9.03 |
| warm-up | 3.94 | 26.1 | pipelines/compile_ms 708, pipelines/front_ms 684, render/thread/ms 44.02, render/busy/ms 42.01, stage/render/prepare_assets 29.73, pipelines/driver_ms 23.27, main/busy/ms 21.76, stage/render/render 9.86, render/graph/ms 7.73, stage/main/update 7.33, pipelines/wait_ms 6.50, stage/main/post_update 5.95 |
| warm-up | 4.00 | 66.2 | pipelines/compile_ms 231, pipelines/front_ms 221, render/thread/ms 81.53, render/busy/ms 79.98, main/busy/ms 67.28, stage/main/update 48.27, stream/drive/ms 42.91, stage/render/prepare_assets 40.90, stage/render/prepare_bind_groups 21.50, stage/main/post_update 9.62, pipelines/driver_ms 9.21, stage/render/render 8.91 |
| warm-up | 4.07 | 47.1 | pipelines/compile_ms 1417, pipelines/front_ms 1371, render/thread/ms 59.84, render/busy/ms 47.44, pipelines/driver_ms 46.78, main/busy/ms 41.79, stage/render/prepare_assets 29.83, pipelines/wait_ms 24.44, stage/main/post_update 18.57, stage/main/update 13.87, stage/render/prepare_views 11.41, render/acquire/ms 11.24 |
| warm-up | 4.13 | 46.2 | pipelines/compile_ms 622, pipelines/front_ms 591, pipelines/wait_ms 44.06, render/thread/ms 43.14, render/busy/ms 41.47, pipelines/driver_ms 31.22, stage/render/prepare_assets 24.44, main/busy/ms 18.15, stage/render/render 7.36, stage/main/update 7.05, stage/render/prepare_resources 6.78, stage/main/post_update 5.95 |
| warm-up | 4.18 | 26.4 | pipelines/compile_ms 543, pipelines/front_ms 524, pipelines/wait_ms 62.63, render/thread/ms 38.50, render/busy/ms 36.74, main/busy/ms 30.66, pipelines/driver_ms 19.06, stage/main/update 14.74, stage/render/prepare_assets 14.47, stage/main/post_update 9.62, stage/render/prepare_resources 8.18, stage/render/render 6.98 |
| warm-up | 4.21 | 35.2 | pipelines/compile_ms 317, pipelines/front_ms 306, render/thread/ms 41.28, render/busy/ms 39.84, pipelines/wait_ms 36.15, main/busy/ms 31.45, stage/main/update 13.66, stage/render/prepare_resources 11.11, stage/render/prepare_assets 10.86, pipelines/driver_ms 10.77, stage/main/pre_update 8.41, stage/render/render 7.78 |
| warm-up | 4.24 | 29.7 | pipelines/compile_ms 363, pipelines/front_ms 352, render/thread/ms 41.52, render/busy/ms 40.24, main/busy/ms 20.55, stage/render/prepare_assets 18.41, pipelines/driver_ms 11.58, stage/main/pre_update 8.55, stage/render/render 7.98, render/graph/ms 5.78, stage/render/prepare_resources 5.43, stage/main/post_update 5.30 |
| warm-up | 4.51 | 92.3 | pipelines/compile_ms 1490, pipelines/front_ms 1445, main/busy/ms 91.45, stage/main/update 65.40, stream/drive/ms 59.38, pipelines/driver_ms 45.00, render/thread/ms 31.03, render/busy/ms 27.81, stage/render/render 16.90, stage/main/post_update 16.47, pipelines/wait_ms 15.70, render/graph/ms 13.45 |
| warm-up | 4.54 | 30.4 | pipelines/compile_ms 1484, pipelines/front_ms 1437, main/busy/ms 111, stage/main/update 78.75, stream/drive/ms 48.38, render/thread/ms 46.58, pipelines/driver_ms 46.52, render/busy/ms 44.77, pipelines/wait_ms 30.13, stage/main/post_update 21.37, stage/render/render 19.43, render/graph/ms 17.13 |
| warm-up | 4.66 | 116.9 | pipelines/compile_ms 1248, pipelines/front_ms 1202, main/busy/ms 99.40, stage/main/update 79.70, render/thread/ms 57.95, render/busy/ms 56.07, stream/drive/ms 47.44, pipelines/driver_ms 46.68, stage/render/prepare_assets 28.48, pipelines/wait_ms 26.27, pipelines/lock_ms 18.47, stage/render/render 14.85 |
| warm-up | 4.70 | 42.7 | pipelines/compile_ms 892, pipelines/front_ms 858, render/thread/ms 71.52, render/busy/ms 69.32, encode//sum 42.22, pipelines/driver_ms 33.38, stage/render/prepare_assets 30.19, main/busy/ms 24.43, stage/render/render 23.53, render/graph/ms 21.01, stage/render/graph_submit 12.70, pipelines/wait_ms 11.79 |
| warm-up | 4.74 | 35.4 | pipelines/compile_ms 84.28, pipelines/front_ms 78.17, render/thread/ms 69.81, render/busy/ms 66.98, encode//sum 66.14, stage/render/render 32.95, main/busy/ms 30.39, render/graph/ms 30.34, stage/render/graph_submit 20.76, stage/render/prepare_assets 14.60, stage/main/post_update 14.18, encode/pass:shadow_point_light/sum 12.82 |
| warm-up | 4.79 | 47.9 | pipelines/compile_ms 168, pipelines/front_ms 160, render/thread/ms 69.14, render/busy/ms 66.72, main/busy/ms 47.67, encode//sum 37.76, stage/render/render 25.89, stage/main/update 24.64, render/graph/ms 23.48, stage/render/prepare_assets 19.77, stage/main/post_update 15.79, stage/render/graph_submit 15.24 |
| warm-up | 4.83 | 48.7 | pipelines/compile_ms 411, pipelines/front_ms 400, render/thread/ms 75.82, render/busy/ms 74.42, main/busy/ms 43.13, stage/render/prepare_assets 27.11, stage/main/update 26.25, stage/render/render 17.99, encode//sum 17.14, render/graph/ms 15.59, stage/render/create_views 12.29, pipelines/driver_ms 11.48 |
| warm-up | 4.90 | 60.9 | pipelines/compile_ms 316, pipelines/front_ms 307, render/thread/ms 93.94, render/busy/ms 91.76, encode//sum 88.51, main/busy/ms 87.79, stage/main/update 63.21, stream/drive/ms 51.94, stage/render/render 44.46, render/graph/ms 41.11, stage/render/graph_submit 31.32, encode/pass:shadow_point_light/sum 24.68 |
| warm-up | 4.97 | 75.9 | pipelines/compile_ms 181, pipelines/front_ms 172, encode//sum 106, render/thread/ms 98.52, render/busy/ms 95.94, main/busy/ms 92.08, stage/main/update 61.69, stage/render/render 57.63, render/graph/ms 54.29, stream/drive/ms 52.53, stage/render/graph_submit 41.33, encode/pass:shadow_point_light/sum 29.64 |
| warm-up | 5.02 | 46.6 | pipelines/compile_ms 1130, pipelines/front_ms 1092, render/thread/ms 81.97, render/busy/ms 80.21, main/busy/ms 52.04, encode//sum 38.65, pipelines/driver_ms 37.95, stage/render/render 37.18, render/graph/ms 34.85, stage/main/update 30.56, stream/drive/ms 22.19, stage/render/graph_submit 21.91 |
| warm-up | 5.07 | 48.1 | pipelines/compile_ms 982, pipelines/front_ms 953, render/thread/ms 80.63, render/busy/ms 78.95, main/busy/ms 46.58, stage/render/render 42.84, render/graph/ms 40.63, stage/main/update 29.66, pipelines/driver_ms 29.54, stream/drive/ms 22.65, stage/render/graph_submit 20.34, stage/render/camera_driver 19.76 |
| warm-up | 5.11 | 40.4 | pipelines/compile_ms 98.64, pipelines/front_ms 94.54, render/thread/ms 77.82, render/busy/ms 75.18, main/busy/ms 52.77, stage/render/render 40.77, render/graph/ms 37.53, stage/main/update 28.01, stream/drive/ms 20.78, stage/render/graph_submit 20.18, stage/main/post_update 18.63, stage/render/prepare_assets 18.43 |
| warm-up | 5.21 | 102.7 | pipelines/compile_ms 1081, pipelines/front_ms 1047, render/thread/ms 88.39, render/busy/ms 85.83, main/busy/ms 54.44, stage/render/prepare_assets 50.74, pipelines/driver_ms 33.87, stage/main/update 28.68, stage/render/render 21.84, stream/drive/ms 20.59, render/graph/ms 18.59, stage/main/post_update 18.58 |
| warm-up | 5.27 | 61.5 | pipelines/compile_ms 1509, pipelines/front_ms 1464, render/thread/ms 91.48, render/busy/ms 89.61, stage/render/prepare_assets 59.26, pipelines/driver_ms 45.41, main/busy/ms 32.14, stage/render/render 21.54, render/graph/ms 18.99, stage/render/graph_submit 13.68, pipelines/wait_ms 12.35, stage/main/update 12.05 |
| warm-up | 5.30 | 32.2 | pipelines/compile_ms 562, pipelines/front_ms 545, render/thread/ms 69.06, render/busy/ms 65.73, main/busy/ms 35.80, stage/render/render 34.50, render/graph/ms 30.53, stage/render/graph_submit 25.43, stage/render/prepare_assets 22.60, encode/submit 22.09, pipelines/driver_ms 17.37, stage/main/update 15.32 |
| warm-up | 5.39 | 29.3 | pipelines/compile_ms 337, pipelines/front_ms 329, main/busy/ms 37.04, render/thread/ms 35.31, render/busy/ms 33.78, stage/main/update 23.74, stream/drive/ms 17.17, stage/render/render 16.18, render/graph/ms 13.72, encode//sum 11.42, stage/main/post_update 8.96, pipelines/driver_ms 8.87 |
| warm-up | 5.53 | 27.0 | main/busy/ms 32.03, render/thread/ms 29.80, render/busy/ms 28.46, stage/main/update 19.61, stream/drive/ms 12.78, stage/render/render 12.54, encode//sum 10.91, render/graph/ms 10.63, stage/render/prepare_assets 9.37, stage/main/post_update 8.93, stage/render/graph_submit 6.79, encode/pass:shadow_point_light/sum 6.43 |
| warm-up | 5.56 | 31.9 | render/thread/ms 46.44, render/busy/ms 44.98, main/busy/ms 28.08, stage/render/prepare_assets 25.51, stage/main/update 11.91, stage/main/post_update 11.88, stage/render/render 10.37, encode//sum 9.49, render/graph/ms 8.72, encode/pass:shadow_point_light/sum 5.66, stage/render/graph_submit 5.31, stream/drive/ms 3.23 |
| warm-up | 5.61 | 26.9 | render/thread/ms 35.71, render/busy/ms 34.69, main/busy/ms 32.19, stage/main/update 18.40, stage/render/render 12.11, stage/render/prepare_assets 10.87, render/graph/ms 10.61, stream/drive/ms 8.91, encode//sum 8.53, stage/main/post_update 8.01, stage/render/graph_submit 6.80, encode/pass:shadow_point_light/sum 5.85 |
| warm-up | 5.70 | 29.1 | render/thread/ms 43.04, render/busy/ms 41.25, main/busy/ms 38.20, stage/render/render 26.22, render/graph/ms 23.66, stage/main/post_update 18.11, encode//sum 15.89, stage/render/graph_submit 14.98, stage/main/update 13.01, encode/pass:shadow_point_light/sum 11.65, encode/submit 10.77, stage/render/camera_driver 7.85 |
| warm-up | 5.73 | 36.3 | render/thread/ms 48.26, render/busy/ms 46.88, main/busy/ms 36.75, stage/render/render 18.41, stage/render/prepare_assets 17.61, render/graph/ms 16.55, stage/main/post_update 15.15, stage/main/update 14.12, encode//sum 12.26, stage/render/graph_submit 10.96, encode/pass:shadow_point_light/sum 9.56, encode/submit 7.72 |
| warm-up | 5.76 | 25.5 | render/thread/ms 43.01, render/busy/ms 41.80, main/busy/ms 23.16, stage/render/prepare_assets 22.73, stage/render/render 11.84, stage/main/update 11.28, render/graph/ms 10.08, encode//sum 9.55, stage/main/post_update 7.60, stage/render/graph_submit 6.50, encode/pass:shadow_point_light/sum 6.43, encode/submit 4.35 |

## Run

- `target/profiling/perftests.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set stream.interior.disable_exterior=on --resolution 2560x1440 --bench-out target/perf\pt_census\census@2560x1440_arm=park-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 1%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (220 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **83%** (min 70%) · 2625 MHz (min 2625) · memory 8101 MHz · 92.7 W · 74 °C (max 79) · VRAM 5427 MiB · throttled 100% of samples

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
