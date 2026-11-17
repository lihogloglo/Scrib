# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1589 frames over 11.4 s · **median 6.69 ms (149 fps)** · mean 7.20 · p95 9.59 · p99 10.17 (1% low 98 fps) · worst 14.15 ms · 26 spikes over 1.5× median · 42.0 stutters/min (work 26.2/min)
- warm-up: 120 frames over 6.4 s, worst 2552.7 ms
- hitches (> 25.0 ms): **0** measured, 40 in warm-up
- timed GPU passes: 5.13 ms of a 7.20 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 149.5, target 144
- pass worst frame 14.15 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.959 | 2.028 | 2.070 | 0.97 | 0.166 |
| main_opaque_pass_3d | 1.794 | 1.857 | 2.550 | 0.97 | 0.033 |
| bloom | 0.311 | 0.322 | 0.327 | 0.97 | 0.062 |
| auxiliary_views | 0.289 | 0.299 | 0.929 | 0.97 | 0.295 |
| ssao | 0.183 | 0.189 | 0.194 | 0.97 | 0.007 |
| upscaling | 0.147 | 0.152 | 0.172 | 0.97 | 0.005 |
| tlas_build | 0.111 | 0.115 | 0.120 | 0.97 | 0.071 |
| tonemapping | 0.096 | 0.100 | 0.106 | 0.97 | 0.004 |
| gi/probe_update | 0.092 | 0.095 | 0.101 | 0.97 | 0.002 |
| early prepass | 0.077 | 0.080 | 0.100 | 0.97 | 0.035 |
| clustering | 0.069 | 0.072 | 0.075 | 0.97 | 0.019 |
| uniform_allocation | 0.035 | 0.036 | 0.044 | 0.97 | 0.010 |
| bin_unpacking | 0.031 | 0.032 | 0.039 | 0.97 | 0.009 |
| early_mesh_preprocessing | 0.013 | 0.013 | 0.014 | 0.97 | 0.004 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.94 | 0.022 |
| setup_tlas_instances | 0.007 | 0.007 | 0.007 | 0.97 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.29 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.97 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.036 | 8.539 | 13.59 | 9590.6 |
| render/busy/ms | 5.591 | 8.078 | 12.31 | 8883.6 |
| main/busy/ms | 5.338 | 6.115 | 7.17 | 8481.6 |
| stage/render/render | 3.816 | 5.552 | 8.79 | 6064.3 |
| render/graph/ms | 3.222 | 4.926 | 7.24 | 5119.7 |
| stage/main/post_update | 2.926 | 3.501 | 4.57 | 4649.3 |
| encode//sum | 2.417 | 3.772 | 5.63 | 3841.2 |
| stage/render/graph_submit | 2.145 | 3.208 | 4.84 | 3408.5 |
| encode/submit | 1.442 | 2.183 | 3.31 | 2291.2 |
| stage/main/update | 1.233 | 1.459 | 1.96 | 1959.4 |
| encode/pass:shadow_point_light/sum | 1.008 | 2.997 | 4.86 | 1601.1 |
| stage/render/camera_driver | 0.951 | 1.622 | 2.17 | 1510.4 |
| encode/finish_wall | 0.729 | 1.096 | 1.54 | 1157.9 |
| stage/render/prepare_write_phase_buffers | 0.426 | 0.808 | 1.20 | 676.4 |
| encode/pass:early prepass/sum | 0.400 | 0.554 | 1.04 | 635.7 |
| stage/render/prepare_resources | 0.397 | 0.475 | 0.80 | 630.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.390 | 0.542 | 0.93 | 619.6 |
| render/present/ms | 0.365 | 0.533 | 1.49 | 580.6 |
| stage/main/first | 0.339 | 0.421 | 1.41 | 538.1 |
| stage/main/pre_update | 0.321 | 0.401 | 0.59 | 510.6 |
| stage/render/prepare_bind_groups | 0.291 | 0.618 | 0.88 | 461.7 |
| bigspace/total/ms | 0.253 | 0.592 | 1.47 | 401.7 |
| stage/main/fixed_main_loop | 0.238 | 0.591 | 1.25 | 378.4 |
| stage/render/prepare_assets | 0.207 | 0.255 | 2.99 | 329.7 |
| stage/render/prepare_meshes | 0.203 | 0.273 | 0.42 | 322.3 |
| stage/render/extract_commands | 0.152 | 0.181 | 0.58 | 241.4 |
| stage/render/create_views | 0.146 | 0.184 | 0.59 | 232.7 |
| stream/drive/ms | 0.144 | 0.190 | 0.35 | 228.4 |
| stage/render/prepare_flush | 0.138 | 0.272 | 0.42 | 219.0 |
| stage/render/prepare_views | 0.123 | 0.166 | 0.35 | 195.2 |
| stream/tick/ms | 0.116 | 0.160 | 0.33 | 184.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.110 | 0.161 | 0.70 | 174.8 |
| stage/main/last | 0.109 | 0.140 | 0.86 | 173.1 |
| physics/step/ms | 0.097 | 0.261 | 0.62 | 154.7 |
| bigspace/low_precision/ms | 0.096 | 0.251 | 1.01 | 152.3 |
| render/acquire/ms | 0.080 | 0.107 | 0.31 | 126.4 |
| stage/render/specialize | 0.078 | 0.101 | 0.64 | 124.1 |
| anim/evaluate/ms | 0.075 | 0.229 | 0.64 | 119.8 |
| stage/render/prepare_batch_phases | 0.065 | 0.106 | 0.28 | 102.9 |
| controller/feed/ms | 0.065 | 0.081 | 0.55 | 102.8 |
| stage/render/phase_sort | 0.054 | 0.089 | 0.25 | 85.6 |
| bigspace/high_precision/ms | 0.052 | 0.170 | 0.80 | 83.0 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.071 | 0.15 | 76.3 |
| stage/render/queue | 0.041 | 0.072 | 0.38 | 65.3 |
| anim/evaluate_cpu/ms | 0.035 | 0.052 | 0.44 | 56.4 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.62 | 48.2 |
| weather/map/ms | 0.025 | 0.036 | 0.07 | 40.2 |
| stage/render/cleanup | 0.021 | 0.033 | 0.41 | 34.1 |
| far/select/ms | 0.015 | 0.019 | 0.04 | 24.4 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.019 | 0.12 | 19.3 |
| weather/eval/ms | 0.012 | 0.017 | 0.04 | 19.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.018 | 0.53 | 16.8 |
| physics/insert/ms | 0.009 | 0.000 | 0.58 | 13.7 |
| script/update/ms | 0.008 | 0.013 | 0.23 | 13.4 |
| controller/move/ms | 0.006 | 0.008 | 0.03 | 9.8 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.2 |
| anim/apply/ms | 0.005 | 0.009 | 0.04 | 7.6 |
| controller/avoid/ms | 0.003 | 0.004 | 0.03 | 5.5 |
| script/ui/ms | 0.003 | 0.004 | 0.01 | 4.6 |
| controller/write/ms | 0.001 | 0.002 | 0.01 | 1.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.3 |
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
| crowd/joint_blocks/count | count | 2 | 3 | 2 |
| crowd/joints_extracted/count | count | 94 | 1451 | 94 |
| crowd/joints_uploaded/count | count | 94 | 153 | 94 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 15.7 k | 15.7 k | 15.7 k |
| encode//max | ms | 0.39 | 1.34 | 0.34 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.12 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.53 | 0.01 |
| encode/finish_critical | ms | 0.49 | 1.34 | 0.39 |
| encode/pass:early prepass/max | ms | 0.41 | 1.04 | 0.39 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.40 | 0.93 | 0.39 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.70 | 0.06 |
| encode/pass:shadow_point_light/max | ms | 0.50 | 1.11 | 0.51 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.62 | 0.04 |
| far/entities | count | 162 | 162 | 162 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.6 k | 41.6 k | 41.6 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.26 M | 86.26 M | 86.26 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.55 | 12.31 | 5.34 |
| gpu/timed/ms | ms | 5.18 | 7.20 | 4.57 |
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
| phases/opaque3d/items | count | 249 | 505 | 38 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 7 | 10 | 4 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 7 | 3 |
| phases/opaque3d_prepass/bins | count | 142 | 280 | 24 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 249 | 505 | 38 |
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
| physics/pending | count | 0 | 85 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 289 | 289 | 289 |
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
| preprocess/bind_groups_reused | count | 47 | 107 | 35 |
| process/cpu_usage | % | 22.33 | 33.65 | 26.80 |
| process/mem_usage | GiB | 3.84 | 3.88 | 3.88 |
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
| system/cpu_usage | % | 30.16 | 33.57 | 31.32 |
| system/mem_usage | % | 57.36 | 57.49 | 57.49 |
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
| views/all/work_items | count | 578 | 2411 | 78 |
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
| views/main/items | count | 518 | 1054 | 78 |
| views/main/pipelines | count | 15 | 21 | 9 |
| views/main/work_items | count | 538 | 1095 | 78 |
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
| anim/bones_written/count | count | 8.33 | 30 | 13.2 k |
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
| encode//count | count | 31.01 | 84 | 49.3 k |
| encode/bind_walk/init_textures | count | 8604 | 25.2 k | 13.67 M |
| encode/bind_walk/init_textures_skipped | count | 17.9 k | 70.5 k | 28.49 M |
| encode/bind_walk/inits | count | 195 | 556 | 309.5 k |
| encode/bind_walk/inits_skipped | count | 20.75 | 82 | 33.0 k |
| encode/bind_walk/merge_textures | count | 7361 | 22.7 k | 11.70 M |
| encode/bind_walk/merge_textures_skipped | count | 32.29 | 126 | 51.3 k |
| encode/bind_walk/merges | count | 59.84 | 162 | 95.1 k |
| encode/bind_walk/merges_skipped | count | 8.03 | 32 | 12.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1589 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1589 |
| encode/pass:early prepass/count | count | 1 | 2 | 1589 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1589 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1542 |
| encode/pass:shadow_point_light/count | count | 3.18 | 13 | 5049 |
| encode/queue/command_buffers | count | 43.15 | 114 | 68.6 k |
| encode/queue/submits | count | 2 | 4 | 3178 |
| encode/queue/write_buffer/bytes | bytes | 474.2 k | 1.02 M | 753.47 M |
| encode/queue/write_buffer/calls | count | 150 | 443 | 237.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 158.0 k | 315.9 k | 250.99 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9534 |
| encode/queue/write_texture/bytes | bytes | 11.2 k | 8.39 M | 17.83 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1589 |
| physics/inserts | count | 0.05 | 11 | 86 |
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

## Hitches (> 25.0 ms; slowest 40, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2552.7 | main/busy/ms 2523, stage/main/post_update 2421, stage/main/update 93.87, surface/advance_cpu_ms 65.69, stage/main/pre_update 4.55, stage/main/last 1.02, stage/main/first 0.73, stream/drive/ms 0.62, stage/main/fixed_main_loop 0.46, weather/map/ms 0.12, stream/tick/ms 0.09, bigspace/total/ms 0.09 |
| warm-up | 3.33 | 779.5 | main/busy/ms 2642, stage/main/post_update 2426, render/thread/ms 748, render/busy/ms 566, stage/render/prepare_assets 533, stage/main/update 202, stage/render/prepare_views 178, render/acquire/ms 177, surface/advance_cpu_ms 65.69, stage/render/prepare_resources 21.21, stage/render/render 12.38, render/graph/ms 7.07 |
| warm-up | 3.48 | 146.4 | render/thread/ms 877, render/busy/ms 614, stage/render/prepare_assets 575, stage/render/prepare_views 178, render/acquire/ms 177, main/busy/ms 135, stage/main/update 114, stage/render/render 96.91, render/present/ms 85.85, stage/render/prepare_resources 21.86, render/graph/ms 10.28, stage/main/post_update 9.04 |
| warm-up | 3.62 | 124.1 | pipelines/wait_ms 1083, pipelines/compile_ms 649, pipelines/front_ms 608, main/busy/ms 143, stage/main/update 104, render/thread/ms 98.72, render/busy/ms 88.19, stage/render/prepare_assets 57.12, pipelines/driver_ms 41.14, stage/render/render 36.44, render/graph/ms 25.71, stage/main/post_update 19.28 |
| warm-up | 3.72 | 103.7 | pipelines/compile_ms 1640, pipelines/front_ms 1572, pipelines/wait_ms 1090, render/thread/ms 170, render/busy/ms 165, stage/render/prepare_assets 131, main/busy/ms 109, stage/main/update 84.65, pipelines/driver_ms 68.23, stage/render/render 32.47, render/graph/ms 27.84, stage/main/post_update 15.87 |
| warm-up | 3.75 | 26.2 | pipelines/compile_ms 1547, pipelines/front_ms 1492, render/thread/ms 113, render/busy/ms 109, stage/render/prepare_assets 90.96, pipelines/wait_ms 88.12, pipelines/driver_ms 54.72, main/busy/ms 39.16, stage/render/render 16.62, stage/main/post_update 15.58, stage/main/update 15.18, render/graph/ms 12.35 |
| warm-up | 3.81 | 55.8 | pipelines/compile_ms 670, pipelines/front_ms 642, render/thread/ms 78.03, render/busy/ms 76.29, stage/render/prepare_assets 63.43, main/busy/ms 39.20, pipelines/driver_ms 28.12, stage/main/update 16.61, stage/main/post_update 15.96, pipelines/wait_ms 12.14, stage/render/render 9.91, render/graph/ms 7.90 |
| warm-up | 3.87 | 39.0 | pipelines/compile_ms 520, pipelines/front_ms 503, main/busy/ms 58.87, render/thread/ms 45.74, stage/main/update 38.26, render/busy/ms 37.29, stage/render/render 23.39, stream/drive/ms 17.27, pipelines/driver_ms 16.61, stage/render/prepare_assets 15.02, render/graph/ms 14.22, stage/main/post_update 11.02 |
| warm-up | 3.92 | 51.3 | pipelines/compile_ms 876, pipelines/front_ms 845, render/thread/ms 64.64, render/busy/ms 62.60, stage/render/prepare_assets 45.94, main/busy/ms 45.80, pipelines/driver_ms 31.42, stage/main/update 23.43, stream/drive/ms 17.43, pipelines/wait_ms 17.00, stage/render/render 11.69, render/graph/ms 9.41 |
| warm-up | 3.95 | 35.8 | pipelines/compile_ms 995, pipelines/front_ms 954, render/thread/ms 65.49, render/busy/ms 63.74, stage/render/prepare_assets 45.98, pipelines/driver_ms 40.36, main/busy/ms 28.40, stage/render/render 14.38, render/graph/ms 12.55, pipelines/wait_ms 11.43, pipelines/lock_ms 10.02, stage/render/graph_submit 8.88 |
| warm-up | 3.99 | 32.2 | pipelines/compile_ms 654, pipelines/front_ms 627, render/thread/ms 58.07, render/busy/ms 55.97, stage/render/prepare_assets 34.52, pipelines/driver_ms 26.65, main/busy/ms 22.95, stage/render/render 18.04, render/graph/ms 15.99, stage/render/graph_submit 12.23, pipelines/wait_ms 9.84, pipelines/lock_ms 8.61 |
| warm-up | 4.02 | 33.6 | pipelines/compile_ms 206, pipelines/front_ms 198, main/busy/ms 70.85, render/thread/ms 55.76, render/busy/ms 54.06, stage/render/prepare_assets 39.03, stage/main/update 37.93, stream/drive/ms 34.41, stage/render/render 10.26, stage/main/pre_update 9.44, render/graph/ms 8.47, stage/main/fixed_main_loop 8.04 |
| warm-up | 4.08 | 59.1 | pipelines/compile_ms 1297, pipelines/front_ms 1259, main/busy/ms 67.02, render/thread/ms 45.46, render/busy/ms 44.18, pipelines/driver_ms 38.43, stage/main/update 36.98, stream/drive/ms 34.33, pipelines/wait_ms 32.74, stage/render/prepare_assets 29.81, pipelines/lock_ms 10.72, stage/main/pre_update 8.99 |
| warm-up | 4.12 | 37.4 | pipelines/compile_ms 1420, pipelines/front_ms 1371, pipelines/wait_ms 60.35, render/thread/ms 53.65, render/busy/ms 52.21, pipelines/driver_ms 48.85, stage/render/prepare_assets 39.28, main/busy/ms 33.15, stage/main/post_update 14.81, stage/main/update 14.56, stream/drive/ms 11.58, pipelines/lock_ms 9.59 |
| warm-up | 4.15 | 30.6 | pipelines/compile_ms 601, pipelines/front_ms 574, pipelines/wait_ms 74.75, render/thread/ms 56.78, render/busy/ms 55.32, main/busy/ms 48.75, stage/render/prepare_assets 37.69, pipelines/driver_ms 27.21, stage/main/post_update 26.85, stage/main/update 16.31, stream/drive/ms 11.73, bigspace/total/ms 7.75 |
| warm-up | 4.18 | 37.1 | pipelines/compile_ms 830, pipelines/front_ms 799, pipelines/wait_ms 214, render/thread/ms 52.69, render/busy/ms 50.98, stage/render/prepare_assets 32.75, main/busy/ms 32.65, pipelines/driver_ms 30.51, stage/main/post_update 16.21, stage/main/update 9.74, stage/render/render 8.33, render/graph/ms 6.56 |
| warm-up | 4.24 | 52.7 | pipelines/compile_ms 1297, pipelines/front_ms 1246, pipelines/wait_ms 279, render/thread/ms 71.52, render/busy/ms 69.55, pipelines/driver_ms 51.43, main/busy/ms 50.45, stage/main/post_update 32.32, stage/render/prepare_assets 29.71, stage/render/prepare_flush 16.68, stage/main/update 13.05, stage/render/render 12.08 |
| warm-up | 4.29 | 46.1 | pipelines/compile_ms 1031, pipelines/front_ms 1001, pipelines/wait_ms 105, main/busy/ms 41.32, pipelines/driver_ms 29.84, render/thread/ms 24.62, render/busy/ms 23.10, stage/main/post_update 19.98, stage/main/update 18.04, stage/render/render 10.18, render/graph/ms 8.62, stage/render/prepare_assets 8.36 |
| warm-up | 4.37 | 30.1 | pipelines/compile_ms 247, pipelines/front_ms 238, main/busy/ms 28.12, render/thread/ms 19.71, render/busy/ms 18.50, stage/main/post_update 15.71, pipelines/driver_ms 9.12, stage/render/prepare_assets 9.06, pipelines/wait_ms 7.04, stage/main/update 5.68, stage/render/render 5.31, render/graph/ms 3.97 |
| warm-up | 4.40 | 29.3 | pipelines/compile_ms 753, pipelines/front_ms 728, pipelines/wait_ms 187, render/thread/ms 36.95, render/busy/ms 35.64, main/busy/ms 32.24, pipelines/driver_ms 25.30, stage/main/update 19.49, stage/render/prepare_resources 11.43, stage/render/prepare_bind_groups 7.18, stage/render/render 6.41, stage/main/post_update 5.90 |
| warm-up | 4.61 | 83.5 | pipelines/compile_ms 974, pipelines/front_ms 943, main/busy/ms 85.87, stage/main/update 48.35, stream/drive/ms 43.42, pipelines/driver_ms 30.91, stage/main/post_update 25.99, render/thread/ms 22.25, render/busy/ms 19.95, stage/render/render 13.92, render/graph/ms 11.53, stage/render/graph_submit 9.78 |
| warm-up | 4.66 | 46.9 | pipelines/compile_ms 1094, pipelines/front_ms 1059, main/busy/ms 138, stage/main/update 97.81, stream/drive/ms 60.36, render/thread/ms 47.65, render/busy/ms 44.66, pipelines/driver_ms 35.46, stage/main/post_update 28.27, stage/render/render 24.85, pipelines/wait_ms 22.11, render/graph/ms 21.58 |
| warm-up | 4.80 | 139.1 | pipelines/compile_ms 1599, pipelines/front_ms 1548, main/busy/ms 115, stage/main/update 99.52, render/thread/ms 64.01, render/busy/ms 62.13, stream/drive/ms 59.82, pipelines/driver_ms 51.14, pipelines/wait_ms 41.10, pipelines/lock_ms 39.97, stage/render/prepare_assets 33.25, stage/render/render 18.86 |
| warm-up | 4.83 | 31.9 | pipelines/compile_ms 1501, pipelines/front_ms 1452, render/thread/ms 68.71, render/busy/ms 66.63, encode//sum 60.85, pipelines/driver_ms 48.37, stage/render/render 27.90, stage/render/prepare_assets 26.14, render/graph/ms 25.39, main/busy/ms 25.10, pipelines/wait_ms 22.47, pipelines/lock_ms 21.62 |
| warm-up | 4.85 | 26.1 | encode//sum 82.66, render/thread/ms 54.06, render/busy/ms 52.34, stage/render/render 33.43, render/graph/ms 31.25, main/busy/ms 24.71, stage/render/graph_submit 22.62, pipelines/compile_ms 21.89, pipelines/front_ms 20.11, encode/pass:shadow_point_light/sum 17.96, encode/finish_wall 11.68, encode/submit 10.93 |
| warm-up | 4.90 | 43.0 | pipelines/compile_ms 168, pipelines/front_ms 160, render/thread/ms 57.49, render/busy/ms 55.83, main/busy/ms 37.99, encode//sum 34.12, stage/render/render 21.86, stage/main/update 19.81, render/graph/ms 19.72, stage/render/prepare_assets 15.59, stage/render/graph_submit 12.41, stage/main/post_update 11.53 |
| warm-up | 4.93 | 35.5 | pipelines/compile_ms 168, pipelines/front_ms 160, render/thread/ms 61.21, render/busy/ms 59.75, main/busy/ms 40.34, stage/main/update 24.92, stage/render/prepare_assets 24.86, stage/render/render 17.06, encode//sum 15.28, render/graph/ms 15.01, stage/main/post_update 9.57, encode/pass:shadow_point_light/sum 9.26 |
| warm-up | 5.01 | 78.5 | render/thread/ms 98.36, render/busy/ms 97.04, main/busy/ms 83.32, stage/main/update 58.69, stage/render/render 48.88, render/graph/ms 46.92, stream/drive/ms 43.61, stage/render/camera_driver 23.46, stage/render/graph_submit 23.14, encode//sum 20.85, stage/render/prepare_assets 20.54, encode/pass:shadow_point_light/sum 14.24 |
| warm-up | 5.09 | 82.1 | pipelines/compile_ms 994, pipelines/front_ms 962, render/thread/ms 129, render/busy/ms 128, encode//sum 99.52, main/busy/ms 86.11, stage/render/render 69.48, render/graph/ms 67.16, stage/main/update 54.25, stream/drive/ms 44.13, stage/render/graph_submit 40.94, encode/pass:shadow_point_light/sum 33.83 |
| warm-up | 5.13 | 38.1 | pipelines/compile_ms 1126, pipelines/front_ms 1090, encode//sum 119, render/thread/ms 94.94, render/busy/ms 92.98, stage/render/render 51.78, render/graph/ms 49.28, main/busy/ms 47.96, encode/pass:shadow_point_light/sum 39.07, stage/render/graph_submit 37.60, pipelines/driver_ms 35.66, stage/main/update 29.98 |
| warm-up | 5.18 | 45.4 | pipelines/compile_ms 144, pipelines/front_ms 140, render/thread/ms 74.35, render/busy/ms 71.78, stage/render/render 42.09, main/busy/ms 41.95, render/graph/ms 38.94, encode//sum 38.87, stage/main/update 30.33, stage/render/graph_submit 29.13, encode/submit 20.63, stream/drive/ms 20.44 |
| warm-up | 5.21 | 37.4 | render/thread/ms 72.32, render/busy/ms 69.77, main/busy/ms 59.78, stage/render/render 38.00, pipelines/compile_ms 37.58, pipelines/front_ms 36.43, render/graph/ms 34.83, stage/main/post_update 30.29, stage/render/graph_submit 24.38, stage/main/update 24.07, stage/render/prepare_assets 16.51, encode/submit 14.08 |
| warm-up | 5.33 | 113.1 | pipelines/compile_ms 1217, pipelines/front_ms 1177, render/thread/ms 90.59, render/busy/ms 88.60, main/busy/ms 65.85, stage/render/prepare_assets 40.36, pipelines/driver_ms 39.50, stage/render/render 37.42, render/graph/ms 34.96, stage/main/post_update 31.75, stage/main/update 27.31, stage/render/graph_submit 17.76 |
| warm-up | 5.39 | 60.8 | pipelines/compile_ms 1383, pipelines/front_ms 1339, render/thread/ms 92.79, render/busy/ms 91.16, stage/render/prepare_assets 55.87, pipelines/driver_ms 44.14, main/busy/ms 34.35, stage/render/render 26.73, render/graph/ms 24.60, stage/main/update 13.54, stage/render/camera_driver 12.32, stage/render/graph_submit 11.86 |
| warm-up | 5.42 | 29.4 | pipelines/compile_ms 514, pipelines/front_ms 501, render/thread/ms 64.38, render/busy/ms 61.51, main/busy/ms 39.34, stage/render/render 29.57, render/graph/ms 26.15, stage/render/prepare_assets 23.77, stage/render/graph_submit 21.55, stage/main/update 20.97, encode/submit 20.36, pipelines/driver_ms 13.42 |
| warm-up | 5.55 | 30.3 | render/thread/ms 39.76, render/busy/ms 38.52, main/busy/ms 19.07, stage/render/prepare_assets 16.44, stage/render/render 13.16, render/graph/ms 11.41, encode//sum 10.32, stage/main/update 8.58, stage/render/graph_submit 7.09, stage/main/post_update 6.84, encode/pass:shadow_point_light/sum 5.39, encode/submit 4.77 |
| warm-up | 5.65 | 34.4 | render/thread/ms 48.17, render/busy/ms 46.47, main/busy/ms 31.24, stage/render/prepare_assets 24.32, stage/main/update 16.17, stage/render/render 12.73, render/graph/ms 10.65, stage/main/post_update 10.37, encode//sum 10.02, stage/render/graph_submit 6.86, encode/pass:shadow_point_light/sum 5.60, encode/submit 4.56 |
| warm-up | 5.76 | 25.5 | render/thread/ms 40.58, render/busy/ms 38.79, main/busy/ms 33.31, stage/render/render 23.22, render/graph/ms 20.89, stage/main/post_update 16.39, encode//sum 15.88, stage/render/graph_submit 14.17, stage/main/update 10.60, encode/submit 10.17, encode/pass:shadow_point_light/sum 9.21, stage/render/camera_driver 6.13 |
| warm-up | 5.80 | 38.5 | render/thread/ms 45.97, render/busy/ms 44.63, main/busy/ms 34.78, stage/render/prepare_assets 19.72, stage/render/render 16.01, render/graph/ms 14.26, stage/main/update 14.09, stage/main/post_update 13.85, encode//sum 11.75, stage/render/graph_submit 9.70, encode/submit 6.83, encode/pass:shadow_point_light/sum 6.67 |
| warm-up | 5.83 | 26.0 | render/thread/ms 44.30, render/busy/ms 43.23, stage/render/prepare_assets 25.55, main/busy/ms 24.71, stage/main/update 12.21, stage/render/render 11.09, render/graph/ms 9.57, encode//sum 8.77, stage/main/post_update 8.11, stage/render/graph_submit 6.35, encode/pass:shadow_point_light/sum 5.12, encode/submit 4.37 |

## Run

- `target/profiling/perftests.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\pt_census\census@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (220 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **80%** (min 67%) · 2625 MHz (min 2625) · memory 8101 MHz · 90.0 W · 73 °C (max 78) · VRAM 5434 MiB · throttled 100% of samples

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
