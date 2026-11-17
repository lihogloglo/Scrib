# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1562 frames over 11.4 s · **median 6.66 ms (150 fps)** · mean 7.32 · p95 10.14 · p99 10.92 (1% low 92 fps) · worst 12.64 ms · 109 spikes over 1.5× median · 57.7 stutters/min (work 15.7/min)
- warm-up: 120 frames over 6.7 s, worst 2631.6 ms
- hitches (> 25.0 ms): **0** measured, 45 in warm-up
- timed GPU passes: 5.13 ms of a 7.32 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 150.1, target 144
- pass worst frame 12.64 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.963 | 2.029 | 2.070 | 0.97 | 0.165 |
| main_opaque_pass_3d | 1.793 | 1.854 | 2.549 | 0.97 | 0.036 |
| bloom | 0.312 | 0.322 | 0.328 | 0.97 | 0.063 |
| auxiliary_views | 0.279 | 0.288 | 0.934 | 0.97 | 0.301 |
| ssao | 0.184 | 0.190 | 0.194 | 0.97 | 0.007 |
| upscaling | 0.146 | 0.151 | 0.172 | 0.97 | 0.004 |
| tlas_build | 0.112 | 0.116 | 0.120 | 0.97 | 0.072 |
| tonemapping | 0.097 | 0.100 | 0.106 | 0.97 | 0.004 |
| gi/probe_update | 0.093 | 0.096 | 0.103 | 0.97 | 0.002 |
| early prepass | 0.078 | 0.081 | 0.100 | 0.97 | 0.035 |
| clustering | 0.070 | 0.072 | 0.075 | 0.97 | 0.020 |
| uniform_allocation | 0.035 | 0.036 | 0.045 | 0.97 | 0.011 |
| bin_unpacking | 0.031 | 0.032 | 0.039 | 0.97 | 0.009 |
| early_mesh_preprocessing | 0.013 | 0.013 | 0.014 | 0.97 | 0.004 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.010 | 0.94 | 0.023 |
| setup_tlas_instances | 0.007 | 0.007 | 0.007 | 0.97 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.31 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.97 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.182 | 9.034 | 11.01 | 9656.8 |
| render/busy/ms | 5.725 | 8.525 | 10.47 | 8942.3 |
| main/busy/ms | 5.384 | 6.374 | 7.40 | 8410.5 |
| stage/render/render | 3.903 | 5.890 | 7.34 | 6096.7 |
| render/graph/ms | 3.294 | 5.177 | 6.68 | 5144.9 |
| stage/main/post_update | 2.968 | 3.631 | 5.17 | 4635.6 |
| encode//sum | 2.558 | 4.147 | 5.61 | 3994.9 |
| stage/render/graph_submit | 2.195 | 3.358 | 4.63 | 3429.0 |
| pipelines/compile_ms | 1.657 | 0.000 | 1100.04 | 2588.2 |
| encode/submit | 1.473 | 2.322 | 3.36 | 2300.4 |
| stage/main/update | 1.264 | 1.513 | 2.37 | 1973.6 |
| pipelines/driver_ms | 1.071 | 0.000 | 1028.44 | 1672.5 |
| encode/pass:shadow_point_light/sum | 0.995 | 3.196 | 4.22 | 1553.4 |
| stage/render/camera_driver | 0.970 | 1.745 | 2.28 | 1514.8 |
| encode/finish_wall | 0.747 | 1.121 | 1.67 | 1167.0 |
| pipelines/front_ms | 0.586 | 0.000 | 76.37 | 915.7 |
| stage/render/prepare_write_phase_buffers | 0.430 | 0.836 | 2.35 | 671.8 |
| encode/pass:early prepass/sum | 0.413 | 0.574 | 1.24 | 644.9 |
| stage/render/prepare_resources | 0.411 | 0.496 | 0.80 | 641.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.394 | 0.563 | 1.02 | 616.1 |
| render/present/ms | 0.376 | 0.546 | 2.20 | 587.3 |
| stage/main/first | 0.337 | 0.425 | 1.36 | 527.0 |
| stage/main/pre_update | 0.323 | 0.404 | 0.85 | 504.1 |
| stage/render/prepare_bind_groups | 0.300 | 0.643 | 1.17 | 469.0 |
| bigspace/total/ms | 0.247 | 0.598 | 1.07 | 385.8 |
| stage/main/fixed_main_loop | 0.214 | 0.549 | 0.91 | 333.8 |
| stage/render/prepare_meshes | 0.211 | 0.282 | 0.46 | 329.9 |
| stage/render/prepare_assets | 0.208 | 0.262 | 1.27 | 325.3 |
| stage/render/extract_commands | 0.153 | 0.182 | 0.33 | 239.5 |
| stage/render/create_views | 0.147 | 0.185 | 0.57 | 230.4 |
| stream/drive/ms | 0.142 | 0.189 | 0.58 | 221.5 |
| stage/render/prepare_flush | 0.136 | 0.260 | 0.49 | 213.1 |
| stage/render/prepare_views | 0.128 | 0.174 | 0.34 | 199.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.119 | 0.172 | 0.75 | 186.1 |
| stream/tick/ms | 0.114 | 0.157 | 0.55 | 177.4 |
| stage/main/last | 0.108 | 0.143 | 0.89 | 168.7 |
| bigspace/low_precision/ms | 0.098 | 0.278 | 0.88 | 152.5 |
| stage/render/specialize | 0.088 | 0.131 | 0.65 | 137.4 |
| render/acquire/ms | 0.081 | 0.108 | 0.29 | 127.1 |
| physics/step/ms | 0.073 | 0.239 | 0.55 | 113.3 |
| anim/evaluate/ms | 0.070 | 0.178 | 0.92 | 109.1 |
| stage/render/prepare_batch_phases | 0.066 | 0.110 | 0.35 | 103.6 |
| controller/feed/ms | 0.065 | 0.083 | 0.52 | 101.2 |
| stage/render/phase_sort | 0.054 | 0.087 | 0.28 | 84.6 |
| bigspace/high_precision/ms | 0.052 | 0.194 | 0.62 | 81.1 |
| stage/render/prepare_collect_phase_buffers | 0.051 | 0.075 | 0.26 | 79.0 |
| stage/render/queue | 0.044 | 0.072 | 0.39 | 69.0 |
| anim/evaluate_cpu/ms | 0.035 | 0.052 | 0.13 | 54.6 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.43 | 49.2 |
| weather/map/ms | 0.026 | 0.037 | 0.17 | 40.3 |
| stage/render/cleanup | 0.022 | 0.034 | 0.38 | 35.0 |
| far/select/ms | 0.015 | 0.018 | 0.30 | 24.1 |
| physics/insert/ms | 0.013 | 0.000 | 0.59 | 19.9 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.019 | 0.46 | 19.8 |
| weather/eval/ms | 0.012 | 0.017 | 0.05 | 18.6 |
| encode/camera_driver_span_end/sum | 0.011 | 0.018 | 0.04 | 16.7 |
| script/update/ms | 0.009 | 0.014 | 0.07 | 13.3 |
| controller/move/ms | 0.006 | 0.008 | 0.03 | 9.6 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 7.5 |
| anim/apply/ms | 0.004 | 0.009 | 0.02 | 7.0 |
| controller/avoid/ms | 0.003 | 0.004 | 0.03 | 5.3 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 4.9 |
| controller/write/ms | 0.001 | 0.002 | 0.01 | 1.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| weather/apply/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 0.9 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.3 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| encode//max | ms | 0.40 | 1.36 | 0.40 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.46 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.04 | 0.01 |
| encode/finish_critical | ms | 0.50 | 1.36 | 0.40 |
| encode/pass:early prepass/max | ms | 0.42 | 1.15 | 0.34 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.40 | 1.02 | 0.24 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 0.75 | 0.55 |
| encode/pass:shadow_point_light/max | ms | 0.50 | 0.99 | 0.74 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.43 | 0.02 |
| far/entities | count | 162 | 162 | 162 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.6 k | 41.6 k | 41.6 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.26 M | 86.26 M | 86.26 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.55 | 10.47 | 4.88 |
| gpu/timed/ms | ms | 5.18 | 7.07 | 4.65 |
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
| phases/alpha_mask3d/items | count | 9 | 22 | 1 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 2 | 2 | 1 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 1 |
| phases/alpha_mask3d_prepass/bins | count | 3 | 7 | 1 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 9 | 22 | 1 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 2 | 2 | 1 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 7 | 10 | 4 |
| phases/opaque3d/bins | count | 140 | 280 | 24 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 248 | 505 | 38 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 7 | 10 | 4 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 7 | 3 |
| phases/opaque3d_prepass/bins | count | 140 | 280 | 24 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 248 | 505 | 38 |
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
| pipelines/total | count | 284 | 284 | 284 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 17 | 0 |
| preprocess/bind_groups | count | 2 | 182 | 0 |
| preprocess/bind_groups_created | count | 2 | 182 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 47 | 103 | 35 |
| process/cpu_usage | % | 25.96 | 35.10 | 28.85 |
| process/mem_usage | GiB | 3.87 | 3.93 | 3.93 |
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
| stream/latency/s | s | 0.07 | 0.07 | 0.07 |
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
| system/cpu_usage | % | 30.73 | 43.93 | 31.19 |
| system/mem_usage | % | 57.48 | 57.71 | 57.71 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2038 | 2038 | 2038 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 18 | 102 | 9 |
| views/all/bins | count | 2970 | 3303 | 2223 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 4157 | 4589 | 3183 |
| views/all/pipelines | count | 247 | 261 | 198 |
| views/all/pipelines_distinct | count | 22 | 28 | 16 |
| views/all/work_items | count | 563 | 2411 | 78 |
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
| views/main/bins | count | 286 | 574 | 50 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 516 | 1054 | 78 |
| views/main/pipelines | count | 15 | 21 | 9 |
| views/main/work_items | count | 535 | 1095 | 78 |
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
| anim/bones_written/count | count | 8.18 | 30 | 12.8 k |
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
| encode//count | count | 30.68 | 71 | 47.9 k |
| encode/bind_walk/init_textures | count | 8481 | 22.4 k | 13.25 M |
| encode/bind_walk/init_textures_skipped | count | 17.5 k | 60.6 k | 27.28 M |
| encode/bind_walk/inits | count | 193 | 410 | 301.1 k |
| encode/bind_walk/inits_skipped | count | 20.18 | 65 | 31.5 k |
| encode/bind_walk/merge_textures | count | 7238 | 21.2 k | 11.31 M |
| encode/bind_walk/merge_textures_skipped | count | 31.93 | 84 | 49.9 k |
| encode/bind_walk/merges | count | 59.31 | 117 | 92.6 k |
| encode/bind_walk/merges_skipped | count | 7.79 | 22 | 12.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1562 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1562 |
| encode/pass:early prepass/count | count | 1 | 2 | 1562 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1562 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1514 |
| encode/pass:shadow_point_light/count | count | 3.09 | 13 | 4827 |
| encode/queue/command_buffers | count | 42.74 | 93 | 66.8 k |
| encode/queue/submits | count | 2 | 4 | 3124 |
| encode/queue/write_buffer/bytes | bytes | 472.9 k | 934.5 k | 738.67 M |
| encode/queue/write_buffer/calls | count | 148 | 354 | 230.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 158.0 k | 315.9 k | 246.72 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9372 |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 17.83 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1562 |
| physics/inserts | count | 0.07 | 12 | 114 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.44 | 1 | 686 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.01 | 1 | 21 |
| pipelines/created | count | 0.01 | 1 | 21 |
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

## Hitches (> 25.0 ms; slowest 45, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.63 | 2631.6 | main/busy/ms 2601, stage/main/post_update 2497, stage/main/update 95.36, surface/advance_cpu_ms 67.19, stage/main/pre_update 5.22, stage/main/last 1.25, stream/drive/ms 0.62, stage/main/first 0.61, stage/main/fixed_main_loop 0.54, stream/tick/ms 0.16, weather/map/ms 0.13, bigspace/total/ms 0.10 |
| warm-up | 3.37 | 740.0 | main/busy/ms 2716, stage/main/post_update 2502, render/thread/ms 698, render/busy/ms 512, stage/render/prepare_assets 478, stage/main/update 199, stage/render/prepare_views 182, render/acquire/ms 180, surface/advance_cpu_ms 67.19, stage/render/prepare_resources 21.14, stage/render/render 12.89, render/graph/ms 6.86 |
| warm-up | 3.52 | 148.4 | render/thread/ms 830, render/busy/ms 560, stage/render/prepare_assets 520, stage/render/prepare_views 182, render/acquire/ms 180, main/busy/ms 134, stage/main/update 111, stage/render/render 101, render/present/ms 89.84, stage/render/prepare_resources 21.87, stage/main/post_update 10.10, render/graph/ms 10.04 |
| warm-up | 3.70 | 160.7 | pipelines/wait_ms 1393, pipelines/compile_ms 1044, pipelines/front_ms 988, render/thread/ms 135, render/busy/ms 122, stage/render/prepare_assets 103, main/busy/ms 84.80, stage/main/update 64.78, pipelines/driver_ms 56.50, stage/render/render 25.61, pipelines/lock_ms 19.19, render/present/ms 12.56 |
| warm-up | 3.73 | 27.1 | pipelines/wait_ms 1403, pipelines/compile_ms 1257, pipelines/front_ms 1195, render/thread/ms 142, render/busy/ms 139, stage/render/prepare_assets 118, pipelines/driver_ms 62.00, main/busy/ms 41.67, stage/main/update 20.93, stage/render/render 16.43, pipelines/lock_ms 13.39, render/graph/ms 12.78 |
| warm-up | 3.80 | 67.4 | pipelines/compile_ms 923, pipelines/front_ms 874, render/thread/ms 81.80, render/busy/ms 79.65, pipelines/wait_ms 73.66, stage/render/prepare_assets 60.65, pipelines/driver_ms 48.86, main/busy/ms 44.72, stage/main/update 22.21, stage/render/render 14.94, render/graph/ms 12.71, stage/main/post_update 11.93 |
| warm-up | 3.83 | 33.7 | pipelines/compile_ms 737, pipelines/front_ms 695, render/thread/ms 88.30, render/busy/ms 86.07, stage/render/prepare_assets 69.28, main/busy/ms 56.98, pipelines/wait_ms 49.55, pipelines/driver_ms 42.34, stage/main/update 29.48, stage/main/post_update 13.71, stage/render/render 13.68, render/graph/ms 11.63 |
| warm-up | 3.87 | 39.6 | pipelines/compile_ms 872, pipelines/front_ms 839, render/thread/ms 70.36, render/busy/ms 67.71, main/busy/ms 67.51, stage/main/update 35.01, stage/render/prepare_assets 32.78, pipelines/driver_ms 32.50, stage/main/post_update 21.09, stage/render/render 18.93, stream/drive/ms 17.94, stage/render/prepare_meshes 10.43 |
| warm-up | 3.91 | 35.3 | pipelines/compile_ms 995, pipelines/front_ms 961, render/thread/ms 65.42, render/busy/ms 62.96, main/busy/ms 49.95, pipelines/driver_ms 33.25, stage/render/prepare_assets 27.39, stage/main/post_update 23.13, stage/main/update 20.70, stage/render/render 19.83, stream/drive/ms 16.16, stage/render/prepare_meshes 10.41 |
| warm-up | 3.95 | 39.5 | pipelines/compile_ms 678, pipelines/front_ms 654, render/thread/ms 56.32, render/busy/ms 54.37, stage/render/prepare_assets 39.65, main/busy/ms 23.95, pipelines/driver_ms 23.29, stage/render/render 11.71, stage/main/post_update 10.68, render/graph/ms 9.60, stage/render/graph_submit 6.74, stage/main/update 5.46 |
| warm-up | 4.05 | 80.1 | pipelines/compile_ms 794, pipelines/front_ms 770, render/thread/ms 86.96, render/busy/ms 85.49, main/busy/ms 65.42, stage/main/update 52.38, stream/drive/ms 49.31, stage/render/prepare_assets 35.62, stage/render/prepare_meshes 27.42, pipelines/driver_ms 24.42, stage/render/render 7.05, stage/render/prepare_resources 6.76 |
| warm-up | 4.08 | 29.8 | pipelines/compile_ms 1149, pipelines/front_ms 1111, render/thread/ms 96.00, render/busy/ms 94.71, main/busy/ms 69.85, pipelines/wait_ms 64.39, stage/main/update 52.31, stream/drive/ms 49.21, stage/render/prepare_assets 42.95, pipelines/driver_ms 38.12, stage/render/prepare_meshes 27.42, stage/main/post_update 8.31 |
| warm-up | 4.10 | 26.2 | pipelines/compile_ms 1090, pipelines/front_ms 1055, pipelines/wait_ms 72.00, render/thread/ms 38.67, render/busy/ms 36.95, pipelines/driver_ms 34.86, main/busy/ms 29.55, stage/render/prepare_assets 22.27, stage/main/update 14.44, stage/main/post_update 10.91, stream/drive/ms 9.63, stage/render/render 7.84 |
| warm-up | 4.13 | 26.1 | pipelines/compile_ms 741, pipelines/front_ms 719, pipelines/wait_ms 39.25, render/thread/ms 31.05, render/busy/ms 29.12, main/busy/ms 28.23, pipelines/driver_ms 22.16, stage/main/update 16.41, stage/render/prepare_assets 13.44, stage/render/render 10.50, stream/drive/ms 9.65, render/graph/ms 8.48 |
| warm-up | 4.18 | 52.8 | pipelines/compile_ms 946, pipelines/front_ms 919, render/thread/ms 60.69, render/busy/ms 58.60, pipelines/wait_ms 57.70, main/busy/ms 47.00, stage/render/prepare_assets 30.45, stage/main/post_update 27.57, pipelines/driver_ms 26.65, stage/main/update 15.17, stage/render/render 9.93, render/graph/ms 8.07 |
| warm-up | 4.24 | 38.2 | pipelines/compile_ms 389, pipelines/front_ms 377, pipelines/wait_ms 75.72, render/thread/ms 52.40, render/busy/ms 50.94, main/busy/ms 34.31, stage/main/post_update 25.97, stage/render/prepare_assets 22.80, stage/render/prepare_resources 12.63, pipelines/driver_ms 12.10, stage/render/prepare_meshes 7.66, stage/render/render 6.69 |
| warm-up | 4.29 | 33.5 | render/thread/ms 41.34, render/busy/ms 39.87, main/busy/ms 27.67, stage/render/prepare_assets 19.74, stage/main/post_update 17.37, stage/render/render 7.37, stage/render/prepare_flush 6.69, bigspace/total/ms 6.64, render/graph/ms 5.66, bigspace/low_precision/ms 5.22, stage/render/prepare_resources 4.83, stage/main/update 4.57 |
| warm-up | 4.51 | 76.5 | main/busy/ms 77.74, stage/main/update 54.21, stream/drive/ms 48.67, render/thread/ms 29.42, render/busy/ms 27.94, stage/main/post_update 12.06, stage/render/prepare_assets 9.51, stage/render/render 8.02, render/graph/ms 6.20, stage/render/prepare_resources 5.00, stage/render/graph_submit 3.96, stage/render/prepare_flush 3.55 |
| warm-up | 4.54 | 32.5 | main/busy/ms 100, stage/main/update 75.49, render/thread/ms 49.65, render/busy/ms 47.66, stream/drive/ms 44.11, stage/render/prepare_assets 15.38, stage/main/post_update 13.33, stage/render/render 12.71, render/graph/ms 9.88, stage/render/graph_submit 6.66, stage/render/prepare_resources 5.62, encode/submit 4.67 |
| warm-up | 4.66 | 117.2 | main/busy/ms 91.93, stage/main/update 77.25, render/thread/ms 71.51, render/busy/ms 69.21, stream/drive/ms 43.21, stage/render/prepare_assets 36.00, stage/render/render 15.80, render/graph/ms 12.62, stage/main/post_update 8.54, encode//sum 7.97, stage/render/graph_submit 7.49, encode/submit 5.53 |
| warm-up | 4.70 | 40.5 | render/thread/ms 82.63, render/busy/ms 80.51, encode//sum 45.83, main/busy/ms 37.41, stage/render/prepare_assets 29.69, stage/render/render 25.02, render/graph/ms 22.30, stage/main/update 19.55, stage/render/graph_submit 12.63, stage/main/post_update 11.83, encode/pass:shadow_point_light/sum 10.33, stage/render/prepare_resources 9.01 |
| warm-up | 4.73 | 34.5 | render/thread/ms 68.61, render/busy/ms 66.78, encode//sum 51.31, main/busy/ms 41.89, stage/render/render 27.67, render/graph/ms 25.18, stage/main/update 18.25, stage/main/post_update 16.25, stage/render/graph_submit 14.94, encode/pass:shadow_point_light/sum 12.46, stage/render/prepare_assets 11.39, stage/render/camera_driver 8.73 |
| warm-up | 4.78 | 51.9 | render/thread/ms 71.16, render/busy/ms 68.98, main/busy/ms 50.24, stage/main/update 28.55, stage/render/prepare_assets 25.17, stage/render/render 22.95, render/graph/ms 20.15, encode//sum 19.17, stage/main/post_update 13.69, stage/render/graph_submit 12.07, encode/submit 8.74, encode/pass:shadow_point_light/sum 8.01 |
| warm-up | 4.85 | 65.1 | render/thread/ms 91.84, render/busy/ms 89.38, main/busy/ms 49.87, stage/render/prepare_assets 33.34, stage/main/update 32.06, stage/render/render 26.27, encode//sum 24.84, render/graph/ms 23.18, stage/render/graph_submit 14.68, stage/render/create_views 11.42, encode/submit 10.44, encode/pass:shadow_point_light/sum 10.16 |
| warm-up | 4.91 | 66.0 | render/thread/ms 112, render/busy/ms 109, encode//sum 106, main/busy/ms 79.04, stage/main/update 60.90, stage/render/render 52.19, stream/drive/ms 48.60, render/graph/ms 48.59, stage/render/graph_submit 37.25, encode/pass:shadow_point_light/sum 31.13, stage/render/prepare_assets 22.46, encode/submit 20.99 |
| warm-up | 4.97 | 60.2 | encode//sum 116, render/thread/ms 106, render/busy/ms 102, main/busy/ms 79.33, stage/render/render 58.44, stage/main/update 56.93, render/graph/ms 54.43, stream/drive/ms 48.84, stage/render/graph_submit 41.16, encode/pass:shadow_point_light/sum 36.22, encode/submit 23.56, encode/finish_wall 17.82 |
| warm-up | 5.03 | 56.0 | render/thread/ms 91.93, render/busy/ms 89.66, main/busy/ms 54.47, encode//sum 40.19, stage/render/render 39.77, render/graph/ms 36.52, stage/main/update 34.27, stage/render/prepare_assets 25.30, stream/drive/ms 24.90, stage/render/graph_submit 21.93, encode/pass:shadow_point_light/sum 20.58, encode/submit 15.21 |
| warm-up | 5.08 | 50.5 | render/thread/ms 90.13, render/busy/ms 87.67, main/busy/ms 50.56, stage/render/render 39.50, render/graph/ms 35.96, stage/main/update 33.10, stream/drive/ms 24.73, stage/render/graph_submit 23.06, stage/render/prepare_assets 23.05, encode//sum 21.38, encode/submit 18.37, encode/pass:shadow_point_light/sum 13.54 |
| warm-up | 5.11 | 33.9 | render/thread/ms 72.12, render/busy/ms 69.42, main/busy/ms 41.44, stage/render/render 31.20, render/graph/ms 27.55, stage/main/update 20.33, stage/render/prepare_assets 20.08, stage/render/graph_submit 19.23, encode/submit 15.38, encode//sum 15.19, stage/main/post_update 13.85, stream/drive/ms 12.79 |
| warm-up | 5.21 | 91.4 | render/thread/ms 77.09, render/busy/ms 75.34, stage/render/prepare_assets 43.23, main/busy/ms 41.78, stage/main/update 21.94, stage/render/render 19.11, render/graph/ms 16.42, stream/drive/ms 13.22, encode//sum 12.46, stage/main/post_update 12.04, stage/render/graph_submit 10.09, encode/pass:shadow_point_light/sum 7.09 |
| warm-up | 5.27 | 63.4 | render/thread/ms 88.19, render/busy/ms 86.59, stage/render/prepare_assets 61.27, main/busy/ms 34.31, stage/render/render 15.30, render/graph/ms 13.08, stage/main/update 11.55, encode//sum 11.13, stage/main/post_update 10.30, stage/render/graph_submit 8.59, encode/pass:shadow_point_light/sum 6.10, encode/submit 5.88 |
| warm-up | 5.30 | 30.4 | render/thread/ms 69.65, render/busy/ms 66.48, main/busy/ms 43.35, stage/render/render 30.09, stage/render/prepare_assets 28.34, render/graph/ms 26.38, stage/main/update 22.27, stage/render/graph_submit 21.51, encode/submit 20.24, stream/drive/ms 12.35, encode//sum 11.18, stage/main/post_update 10.04 |
| warm-up | 5.37 | 25.3 | main/busy/ms 39.57, render/thread/ms 37.55, render/busy/ms 35.71, stage/main/update 26.24, stream/drive/ms 18.44, stage/render/render 16.62, render/graph/ms 14.12, encode//sum 11.66, stage/render/graph_submit 9.14, stage/main/post_update 7.89, stage/render/prepare_assets 7.79, encode/submit 6.56 |
| warm-up | 5.43 | 26.1 | render/thread/ms 38.37, render/busy/ms 36.94, main/busy/ms 29.13, stage/render/render 18.17, stage/main/update 16.18, render/graph/ms 16.00, encode//sum 13.81, stage/render/graph_submit 9.60, stage/main/post_update 8.88, stage/render/prepare_assets 6.98, encode/submit 6.50, encode/pass:shadow_point_light/sum 5.81 |
| warm-up | 5.48 | 32.0 | render/thread/ms 43.36, render/busy/ms 42.00, main/busy/ms 19.57, stage/render/prepare_assets 18.35, stage/render/render 14.88, render/graph/ms 12.96, encode//sum 11.46, stage/main/post_update 8.60, stage/render/graph_submit 8.59, stage/main/update 7.04, encode/submit 6.15, encode/pass:shadow_point_light/sum 5.88 |
| warm-up | 5.53 | 30.8 | render/thread/ms 45.82, render/busy/ms 44.11, main/busy/ms 37.41, stage/render/render 19.85, stage/main/update 18.90, render/graph/ms 17.33, stage/main/post_update 13.06, encode//sum 12.91, stage/render/graph_submit 11.38, stage/render/prepare_assets 10.73, stream/drive/ms 9.91, encode/submit 8.62 |
| warm-up | 5.56 | 29.7 | render/thread/ms 49.44, render/busy/ms 47.85, main/busy/ms 36.34, stage/main/update 17.85, stage/render/prepare_assets 17.16, stage/render/render 16.92, render/graph/ms 14.71, stage/main/post_update 12.85, encode//sum 10.90, stage/render/graph_submit 9.42, stream/drive/ms 9.14, encode/pass:shadow_point_light/sum 7.14 |
| warm-up | 5.60 | 41.3 | pipelines/compile_ms 8544, pipelines/driver_ms 7782, pipelines/wait_ms 1358, pipelines/front_ms 762, render/thread/ms 58.38, render/busy/ms 56.75, main/busy/ms 33.48, stage/render/prepare_assets 28.93, stage/render/render 16.75, stage/main/update 15.06, render/graph/ms 14.52, stage/main/post_update 12.22 |
| warm-up | 5.62 | 26.8 | pipelines/compile_ms 8544, pipelines/driver_ms 7782, pipelines/wait_ms 1358, pipelines/front_ms 762, render/thread/ms 59.00, render/busy/ms 57.43, main/busy/ms 47.41, stage/main/update 30.14, stage/render/prepare_assets 27.08, stage/render/render 17.09, render/graph/ms 14.71, stream/drive/ms 12.79 |
| warm-up | 5.68 | 58.6 | pipelines/compile_ms 6262, pipelines/driver_ms 5467, pipelines/wait_ms 4827, pipelines/front_ms 796, render/thread/ms 71.48, render/busy/ms 69.96, main/busy/ms 50.03, stage/main/update 26.92, stage/render/render 22.86, stage/render/prepare_assets 22.43, render/graph/ms 20.49, stream/drive/ms 13.03 |
| warm-up | 5.77 | 66.5 | pipelines/wait_ms 13.0 k, pipelines/compile_ms 2364, pipelines/driver_ms 1420, pipelines/front_ms 944, render/thread/ms 85.78, render/busy/ms 84.34, main/busy/ms 60.41, stage/main/post_update 41.69, stage/render/prepare_resources 33.96, stage/render/render 15.78, stage/main/update 13.65, render/graph/ms 13.64 |
| warm-up | 5.81 | 33.7 | pipelines/wait_ms 12.7 k, pipelines/compile_ms 731, pipelines/front_ms 695, render/thread/ms 92.85, render/busy/ms 91.03, main/busy/ms 68.79, stage/main/post_update 45.24, stage/render/prepare_resources 37.63, pipelines/driver_ms 35.34, stage/render/render 20.51, render/graph/ms 18.06, stage/main/update 15.83 |
| warm-up | 5.87 | 59.2 | pipelines/wait_ms 9674, pipelines/compile_ms 894, pipelines/front_ms 864, render/thread/ms 64.63, render/busy/ms 62.80, main/busy/ms 44.03, pipelines/driver_ms 30.06, stage/render/prepare_assets 28.27, stage/render/render 19.50, stage/main/update 18.65, render/graph/ms 17.12, stage/main/post_update 17.07 |
| warm-up | 5.89 | 25.0 | pipelines/wait_ms 9503, pipelines/compile_ms 1004, pipelines/front_ms 969, render/thread/ms 57.50, render/busy/ms 55.94, pipelines/driver_ms 35.02, main/busy/ms 33.37, stage/render/prepare_assets 30.31, stage/main/update 16.95, stage/render/render 16.18, render/graph/ms 14.13, encode//sum 10.99 |
| warm-up | 6.06 | 25.5 | pipelines/compile_ms 391, pipelines/front_ms 377, render/thread/ms 39.19, render/busy/ms 37.28, main/busy/ms 27.37, stage/render/render 18.79, pipelines/wait_ms 17.33, render/graph/ms 16.45, pipelines/driver_ms 14.05, encode//sum 12.11, stage/render/graph_submit 11.20, stage/main/update 10.14 |

## Run

- `target/profiling/perftests.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\pt_census\census@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (220 pmon samples)
- focus: clean
- nvidia-smi over the window (43 samples): GPU load **78%** (min 68%) · 2625 MHz (min 2625) · memory 8101 MHz · 87.1 W · 72 °C (max 77) · VRAM 5427 MiB · throttled 100% of samples

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
