# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1622 frames over 11.4 s · **median 6.81 ms (147 fps)** · mean 7.05 · p95 8.95 · p99 9.82 (1% low 102 fps) · worst 16.07 ms · 7 spikes over 1.5× median · 5.2 stutters/min (work 5.2/min)
- warm-up: 120 frames over 5.8 s, worst 2550.1 ms
- hitches (> 25.0 ms): **0** measured, 29 in warm-up
- timed GPU passes: 5.53 ms of a 7.05 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 146.8, target 144
- pass worst frame 16.07 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.012 | 2.153 | 2.444 | 0.93 | 0.164 |
| main_opaque_pass_3d | 1.822 | 1.950 | 2.833 | 0.93 | 0.038 |
| auxiliary_views | 0.468 | 0.501 | 1.310 | 0.93 | 0.400 |
| bloom | 0.321 | 0.343 | 0.497 | 0.93 | 0.064 |
| ssao | 0.186 | 0.199 | 0.203 | 0.93 | 0.007 |
| upscaling | 0.152 | 0.163 | 0.203 | 0.93 | 0.004 |
| tlas_build | 0.109 | 0.117 | 0.119 | 0.93 | 0.074 |
| tonemapping | 0.100 | 0.107 | 0.119 | 0.93 | 0.004 |
| gpu_instance_materialization | 0.094 | 0.101 | 0.102 | 0.93 | 0.004 |
| gi/probe_update | 0.092 | 0.099 | 0.099 | 0.93 | 0.002 |
| early prepass | 0.078 | 0.084 | 0.086 | 0.93 | 0.046 |
| clustering | 0.070 | 0.075 | 0.077 | 0.93 | 0.018 |
| uniform_allocation | 0.036 | 0.038 | 0.045 | 0.93 | 0.009 |
| bin_unpacking | 0.031 | 0.034 | 0.040 | 0.93 | 0.007 |
| range_unpacking | 0.018 | 0.020 | 0.020 | 0.93 | 0.002 |
| early_mesh_preprocessing | 0.018 | 0.019 | 0.026 | 0.93 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.91 | 0.022 |
| setup_tlas_instances | 0.007 | 0.007 | 0.007 | 0.93 | 0.005 |
| sparse buffer updates | 0.004 | 0.006 | 0.007 | 0.62 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.93 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.242 | 8.069 | 15.27 | 10125.3 |
| render/busy/ms | 5.608 | 7.349 | 14.74 | 9095.8 |
| main/busy/ms | 5.020 | 5.784 | 14.63 | 8141.9 |
| stage/render/render | 3.968 | 5.587 | 12.53 | 6435.4 |
| render/graph/ms | 3.375 | 4.776 | 12.01 | 5473.4 |
| encode//sum | 2.751 | 4.092 | 42.60 | 4461.5 |
| stage/main/post_update | 2.605 | 3.075 | 11.58 | 4224.8 |
| stage/render/graph_submit | 2.127 | 3.040 | 10.68 | 3449.8 |
| encode/submit | 1.548 | 2.355 | 3.65 | 2510.4 |
| stage/main/update | 1.229 | 1.489 | 1.89 | 1994.1 |
| stage/render/camera_driver | 1.111 | 1.679 | 2.06 | 1802.6 |
| encode/finish_wall | 0.663 | 0.916 | 8.89 | 1075.4 |
| render/present/ms | 0.430 | 1.216 | 1.90 | 697.6 |
| encode/pass:early prepass/sum | 0.404 | 0.557 | 8.45 | 655.2 |
| stage/render/prepare_resources | 0.385 | 0.482 | 0.75 | 624.7 |
| encode/pass:main_opaque_pass_3d/sum | 0.378 | 0.546 | 8.71 | 613.4 |
| stage/main/pre_update | 0.353 | 0.443 | 3.15 | 571.9 |
| stage/render/prepare_write_phase_buffers | 0.347 | 0.556 | 1.14 | 562.1 |
| stage/main/fixed_main_loop | 0.262 | 0.632 | 0.95 | 425.6 |
| encode/pass:shadow_point_light/sum | 0.260 | 0.572 | 1.98 | 422.3 |
| stage/render/prepare_views | 0.251 | 0.932 | 1.45 | 406.8 |
| stage/main/first | 0.240 | 0.344 | 1.72 | 389.7 |
| stage/render/prepare_bind_groups | 0.236 | 0.358 | 1.01 | 383.3 |
| stage/render/prepare_assets | 0.226 | 0.328 | 0.56 | 366.4 |
| bigspace/total/ms | 0.208 | 0.390 | 1.20 | 337.3 |
| render/acquire/ms | 0.205 | 0.879 | 1.40 | 331.9 |
| stage/render/prepare_flush | 0.181 | 0.320 | 0.74 | 294.1 |
| stage/render/prepare_meshes | 0.166 | 0.227 | 0.45 | 269.8 |
| stage/main/last | 0.161 | 0.228 | 0.68 | 260.7 |
| stage/render/extract_commands | 0.155 | 0.189 | 0.34 | 252.2 |
| stage/render/create_views | 0.136 | 0.184 | 0.33 | 220.8 |
| stream/drive/ms | 0.122 | 0.174 | 0.44 | 198.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.107 | 0.147 | 8.12 | 172.8 |
| stream/tick/ms | 0.095 | 0.144 | 0.30 | 154.6 |
| physics/step/ms | 0.095 | 0.263 | 0.61 | 153.9 |
| bigspace/low_precision/ms | 0.091 | 0.183 | 1.11 | 146.8 |
| controller/feed/ms | 0.081 | 0.102 | 0.37 | 130.6 |
| stage/render/prepare_batch_phases | 0.080 | 0.138 | 0.38 | 130.5 |
| stage/render/queue | 0.077 | 0.122 | 0.46 | 125.1 |
| stage/render/specialize | 0.062 | 0.089 | 2.63 | 100.2 |
| anim/evaluate/ms | 0.061 | 0.107 | 0.64 | 98.3 |
| bigspace/high_precision/ms | 0.044 | 0.086 | 0.54 | 70.7 |
| stage/render/prepare_collect_phase_buffers | 0.043 | 0.067 | 0.18 | 69.9 |
| anim/evaluate_cpu/ms | 0.035 | 0.051 | 0.23 | 56.9 |
| encode/point_shadow_cache_copies/sum | 0.034 | 0.069 | 0.39 | 55.4 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.14 | 48.3 |
| controller/write/ms | 0.026 | 0.035 | 0.10 | 41.5 |
| weather/map/ms | 0.025 | 0.034 | 0.21 | 39.7 |
| stage/render/cleanup | 0.023 | 0.036 | 0.24 | 37.0 |
| physics/insert/ms | 0.015 | 0.000 | 0.60 | 25.0 |
| stage/render/phase_sort | 0.011 | 0.027 | 0.18 | 18.4 |
| weather/eval/ms | 0.011 | 0.015 | 0.14 | 18.1 |
| encode/camera_driver_span_begin/sum | 0.011 | 0.019 | 0.05 | 17.8 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.06 | 15.9 |
| script/update/ms | 0.007 | 0.010 | 0.15 | 11.9 |
| anim/apply/ms | 0.005 | 0.010 | 0.03 | 7.8 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.4 |
| far/select/ms | 0.004 | 0.005 | 0.10 | 6.7 |
| controller/move/ms | 0.003 | 0.009 | 0.03 | 5.6 |
| script/ui/ms | 0.003 | 0.005 | 0.10 | 5.3 |
| controller/avoid/ms | 0.002 | 0.005 | 0.02 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.7 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.002 | 0.14 | 1.0 |
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
| encode//max | ms | 0.38 | 8.39 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.05 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.06 | 0.01 |
| encode/finish_critical | ms | 0.47 | 8.71 | 0.39 |
| encode/pass:early prepass/max | ms | 0.40 | 8.45 | 0.37 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.37 | 8.71 | 0.39 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.10 | 8.12 | 0.11 |
| encode/pass:shadow_point_light/max | ms | 0.08 | 0.56 | 0.40 |
| encode/point_shadow_cache_copies/max | ms | 0.05 | 0.39 | 0.03 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.14 | 0.04 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.77 | 14.74 | 5.54 |
| gpu/timed/ms | ms | 5.68 | 8.28 | 5.04 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 42 | 48 | 48 |
| lamp_cache/copies | count | 5 | 13 | 0 |
| lamp_cache/deferred_faces | count | 0 | 12 | 0 |
| lamp_cache/lamps | count | 8 | 8 | 8 |
| lamp_cache/mover_faces | count | 5 | 12 | 0 |
| lamp_cache/movers | count | 200 | 269 | 56 |
| lamp_cache/queued | count | 0 | 6 | 0 |
| lamp_cache/ready | count | 8 | 8 | 8 |
| lamp_cache/render/copies | count | 5 | 13 | 0 |
| lamp_cache/render/drawn | count | 5 | 14 | 0 |
| lamp_cache/render/dropped_copies | count | 0 | 0 | 0 |
| lamp_cache/render/lamps | count | 8 | 8 | 8 |
| lamp_cache/render/loaded | count | 5 | 12 | 0 |
| lamp_cache/render/lost | count | 0 | 0 | 0 |
| lamp_cache/render/views | count | 6 | 26 | 0 |
| lamp_cache/stock_lamps | count | 0 | 1 | 0 |
| lamp_cache/store_starved | count | 0 | 0 | 0 |
| lamp_cache/store_used | count | 19 | 24 | 4 |
| lamp_cache/views | count | 6 | 26 | 0 |
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
| phases/opaque3d/bins | count | 146 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 253 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 146 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 253 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 24 | 61 | 7 |
| phases/shadow/bins | count | 308 | 828 | 120 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 308 | 935 | 168 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 32 | 82 | 7 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 141 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 291 | 291 | 291 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 6 | 198 | 4 |
| preprocess/bind_groups_created | count | 2 | 194 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 70 | 131 | 35 |
| process/cpu_usage | % | 22.12 | 35.89 | 27.23 |
| process/mem_usage | GiB | 3.77 | 3.84 | 3.84 |
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
| skin/blocks_skipped/count | count | 1 | 2 | 0 |
| skin/blocks_walked/count | count | 1 | 2 | 0 |
| skin/joint_bytes_written/bytes | bytes | 2944 | 7680 | 0 |
| skin/joints_written/count | count | 46 | 120 | 0 |
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
| system/cpu_usage | % | 30.03 | 50.84 | 32.20 |
| system/mem_usage | % | 56.02 | 56.26 | 56.26 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 35 | 90 | 17 |
| views/all/bins | count | 387 | 1358 | 58 |
| views/all/count | count | 7 | 28 | 1 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 620 | 1891 | 86 |
| views/all/pipelines | count | 36 | 109 | 15 |
| views/all/pipelines_distinct | count | 24 | 34 | 15 |
| views/all/work_items | count | 98.9 k | 100.0 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 12 | 61 | 0 |
| views/lamp_face/bins | count | 127 | 828 | 0 |
| views/lamp_face/count | count | 6 | 27 | 0 |
| views/lamp_face/culled | count | 0 | 4 | 0 |
| views/lamp_face/drawn | count | 5 | 18 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 127 | 935 | 0 |
| views/lamp_face/pipelines | count | 15 | 82 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 107 | 768 | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 302 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 530 | 1062 | 86 |
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
| anim/bones_written/count | count | 8.70 | 30 | 14.1 k |
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
| encode//count | count | 41.79 | 91 | 67.8 k |
| encode/bind_walk/init_textures | count | 4201 | 12.6 k | 6.81 M |
| encode/bind_walk/init_textures_skipped | count | 6455 | 17.0 k | 10.47 M |
| encode/bind_walk/inits | count | 217 | 416 | 351.8 k |
| encode/bind_walk/inits_skipped | count | 9.79 | 26 | 15.9 k |
| encode/bind_walk/merge_textures | count | 2958 | 11.3 k | 4.80 M |
| encode/bind_walk/merge_textures_skipped | count | 75.73 | 7105 | 122.8 k |
| encode/bind_walk/merges | count | 66.19 | 126 | 107.4 k |
| encode/bind_walk/merges_skipped | count | 10.87 | 46 | 17.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1622 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1622 |
| encode/pass:early prepass/count | count | 1 | 1 | 1622 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1622 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1572 |
| encode/pass:shadow_point_light/count | count | 5.53 | 18 | 8973 |
| encode/point_shadow_cache_copies/count | count | 0.67 | 1 | 1082 |
| encode/queue/command_buffers | count | 56.96 | 119 | 92.4 k |
| encode/queue/submits | count | 2 | 2 | 3244 |
| encode/queue/write_buffer/bytes | bytes | 537.6 k | 661.7 k | 872.06 M |
| encode/queue/write_buffer/calls | count | 145 | 310 | 234.6 k |
| encode/queue/write_buffer_with/bytes | bytes | 87.8 k | 88.2 k | 142.39 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 9732 |
| encode/queue/write_texture/bytes | bytes | 646 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1622 |
| physics/inserts | count | 0.09 | 9 | 142 |
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

## Hitches (> 25.0 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2550.1 | main/busy/ms 2520, stage/main/post_update 2417, stage/main/update 93.25, surface/advance_cpu_ms 66.56, stage/main/pre_update 6.49, stage/main/last 1.15, stage/main/first 0.44, stage/main/fixed_main_loop 0.37, stream/drive/ms 0.26, weather/map/ms 0.12, bigspace/total/ms 0.07, stream/tick/ms 0.05 |
| warm-up | 3.27 | 720.5 | main/busy/ms 2642, stage/main/post_update 2422, render/thread/ms 683, render/busy/ms 570, stage/render/prepare_assets 527, stage/main/update 204, stage/render/prepare_views 111, render/acquire/ms 111, surface/advance_cpu_ms 66.56, stage/render/create_views 28.64, stage/render/render 8.42, stage/main/pre_update 7.61 |
| warm-up | 3.32 | 47.5 | render/thread/ms 715, render/busy/ms 601, stage/render/prepare_assets 553, main/busy/ms 135, stage/main/update 116, stage/render/prepare_views 112, render/acquire/ms 111, stage/render/create_views 28.87, stage/render/render 12.04, render/graph/ms 8.64, stage/main/post_update 8.24, stage/main/fixed_main_loop 7.92 |
| warm-up | 3.39 | 64.6 | pipelines/wait_ms 1715, pipelines/compile_ms 530, pipelines/front_ms 494, main/busy/ms 70.65, stage/main/post_update 46.33, render/thread/ms 35.47, pipelines/driver_ms 35.22, render/busy/ms 32.73, stage/render/prepare_assets 20.35, stage/main/pre_update 15.69, stage/render/render 9.80, render/graph/ms 7.09 |
| warm-up | 3.42 | 30.9 | pipelines/wait_ms 1713, pipelines/compile_ms 618, pipelines/front_ms 580, render/thread/ms 54.83, render/busy/ms 52.80, main/busy/ms 49.36, pipelines/driver_ms 38.26, stage/render/prepare_assets 37.72, stage/main/pre_update 16.87, stage/main/update 15.49, stage/render/render 10.36, stage/main/post_update 8.99 |
| warm-up | 3.45 | 29.6 | pipelines/compile_ms 386, pipelines/front_ms 373, render/thread/ms 50.98, render/busy/ms 48.86, main/busy/ms 31.84, stage/render/prepare_assets 25.47, stage/main/update 14.38, pipelines/driver_ms 13.40, stage/render/render 12.80, render/graph/ms 11.01, stage/render/graph_submit 9.10, encode//sum 8.20 |
| warm-up | 3.58 | 111.4 | pipelines/compile_ms 1001, pipelines/front_ms 962, render/thread/ms 75.13, pipelines/wait_ms 72.99, main/busy/ms 68.76, render/busy/ms 63.81, stage/main/update 52.70, stage/render/prepare_assets 48.35, pipelines/driver_ms 39.12, stage/render/render 21.40, render/present/ms 10.86, render/graph/ms 10.13 |
| warm-up | 3.65 | 58.3 | pipelines/compile_ms 742, pipelines/front_ms 706, render/thread/ms 69.35, render/busy/ms 67.20, main/busy/ms 52.72, stage/render/prepare_assets 43.31, pipelines/driver_ms 36.40, stage/main/update 23.91, stage/render/render 19.89, stream/drive/ms 17.92, render/graph/ms 17.72, stage/render/graph_submit 15.29 |
| warm-up | 3.73 | 49.4 | pipelines/compile_ms 629, pipelines/front_ms 605, main/busy/ms 161, stage/main/update 139, render/thread/ms 54.93, render/busy/ms 52.87, stage/render/prepare_assets 33.20, pipelines/driver_ms 24.44, stage/render/render 15.61, render/graph/ms 13.41, stage/render/graph_submit 10.32, stage/main/post_update 8.37 |
| warm-up | 3.95 | 226.8 | pipelines/compile_ms 2311, pipelines/front_ms 2227, main/busy/ms 156, stage/main/update 138, render/thread/ms 120, render/busy/ms 118, stage/render/prepare_assets 101, pipelines/driver_ms 84.66, pipelines/wait_ms 56.44, pipelines/lock_ms 17.58, stage/render/render 13.25, render/graph/ms 11.32 |
| warm-up | 3.99 | 36.2 | pipelines/compile_ms 1831, pipelines/front_ms 1763, render/thread/ms 111, render/busy/ms 110, stage/render/prepare_assets 89.03, pipelines/driver_ms 67.67, pipelines/wait_ms 55.30, main/busy/ms 54.17, stage/main/update 39.92, stream/drive/ms 31.97, stage/render/render 18.19, render/graph/ms 16.48 |
| warm-up | 4.06 | 76.3 | pipelines/compile_ms 695, pipelines/front_ms 668, render/thread/ms 66.85, render/busy/ms 64.98, main/busy/ms 51.57, stage/render/prepare_assets 41.75, stage/main/update 37.99, stream/drive/ms 32.24, pipelines/driver_ms 26.82, stage/render/render 19.95, render/graph/ms 17.90, stage/render/graph_submit 15.02 |
| warm-up | 4.12 | 52.6 | pipelines/compile_ms 1540, pipelines/front_ms 1425, pipelines/driver_ms 115, render/thread/ms 88.26, render/busy/ms 67.39, main/busy/ms 47.55, stage/render/render 37.01, stage/render/prepare_assets 33.72, stage/main/update 23.29, pipelines/wait_ms 21.80, render/present/ms 20.59, stream/drive/ms 12.06 |
| warm-up | 4.41 | 25.3 | pipelines/compile_ms 493, pipelines/front_ms 477, main/busy/ms 38.41, render/thread/ms 33.45, render/busy/ms 31.75, pipelines/wait_ms 21.89, stage/main/post_update 21.19, pipelines/driver_ms 16.13, stage/render/render 11.41, render/graph/ms 9.68, stage/main/update 8.32, stage/render/graph_submit 7.70 |
| warm-up | 4.44 | 30.1 | pipelines/compile_ms 862, pipelines/front_ms 836, main/busy/ms 50.43, render/thread/ms 41.51, render/busy/ms 39.78, pipelines/wait_ms 33.16, pipelines/driver_ms 25.61, stage/main/post_update 18.97, stage/main/update 15.72, stage/render/prepare_assets 13.36, stage/render/render 11.89, render/graph/ms 9.98 |
| warm-up | 4.50 | 56.8 | pipelines/compile_ms 1112, pipelines/front_ms 1060, main/busy/ms 80.53, render/thread/ms 65.80, render/busy/ms 63.15, pipelines/driver_ms 52.18, stage/main/update 48.59, pipelines/wait_ms 25.24, stage/render/render 20.89, render/graph/ms 17.88, stage/main/post_update 14.85, stage/render/prepare_assets 14.77 |
| warm-up | 4.55 | 59.0 | pipelines/compile_ms 1137, pipelines/front_ms 1081, main/busy/ms 121, stage/main/update 94.29, render/thread/ms 81.03, render/busy/ms 78.36, pipelines/driver_ms 56.01, stream/drive/ms 41.02, pipelines/wait_ms 33.43, stage/render/prepare_assets 28.94, stage/render/render 21.14, render/graph/ms 18.10 |
| warm-up | 4.64 | 86.3 | pipelines/compile_ms 1386, pipelines/front_ms 1340, main/busy/ms 87.27, render/thread/ms 75.45, render/busy/ms 73.85, stage/main/update 65.26, pipelines/driver_ms 46.61, stage/render/prepare_assets 45.63, pipelines/wait_ms 44.27, stream/drive/ms 42.50, stage/render/render 15.37, stage/main/post_update 14.70 |
| warm-up | 4.67 | 31.2 | pipelines/compile_ms 979, pipelines/front_ms 946, render/thread/ms 65.36, render/busy/ms 63.36, main/busy/ms 50.55, stage/render/prepare_assets 32.83, pipelines/driver_ms 32.38, stage/main/update 29.03, pipelines/wait_ms 25.06, stage/render/render 17.96, render/graph/ms 15.42, stage/main/post_update 14.63 |
| warm-up | 4.77 | 99.0 | pipelines/compile_ms 244, pipelines/front_ms 209, render/thread/ms 87.78, render/busy/ms 86.11, main/busy/ms 49.59, stage/render/prepare_assets 35.90, pipelines/driver_ms 34.48, stage/main/update 30.58, stage/render/create_views 29.04, stage/render/render 12.75, stream/drive/ms 12.74, stage/main/post_update 12.31 |
| warm-up | 4.81 | 37.9 | pipelines/compile_ms 231, pipelines/front_ms 197, render/thread/ms 86.29, render/busy/ms 85.31, main/busy/ms 46.45, stage/render/prepare_assets 43.77, pipelines/driver_ms 33.84, stage/main/update 30.93, stage/render/create_views 28.99, stream/drive/ms 20.66, stage/main/post_update 8.76, stage/render/render 6.97 |
| warm-up | 4.84 | 32.1 | render/thread/ms 52.41, render/busy/ms 50.58, main/busy/ms 41.70, stage/main/update 28.55, stream/drive/ms 19.40, stage/render/prepare_assets 18.77, stage/render/render 14.73, encode//sum 12.98, render/graph/ms 12.65, stage/render/create_views 10.36, stage/render/graph_submit 9.49, stage/main/post_update 7.38 |
| warm-up | 4.87 | 31.7 | render/thread/ms 56.20, render/busy/ms 54.22, main/busy/ms 42.08, stage/main/update 27.34, stage/render/render 15.74, encode//sum 15.04, render/graph/ms 13.56, stage/render/create_views 11.18, stage/render/graph_submit 10.39, stage/main/post_update 8.75, stage/render/prepare_assets 8.17, encode/finish_wall 6.35 |
| warm-up | 4.96 | 25.9 | pipelines/compile_ms 535, pipelines/front_ms 512, main/busy/ms 42.88, render/thread/ms 38.03, render/busy/ms 36.36, pipelines/driver_ms 23.12, stage/main/update 19.92, stage/render/render 15.65, stage/main/post_update 15.39, render/graph/ms 12.95, pipelines/wait_ms 12.82, encode//sum 7.97 |
| warm-up | 4.99 | 29.6 | pipelines/compile_ms 1462, pipelines/front_ms 1408, pipelines/driver_ms 54.60, pipelines/wait_ms 46.36, render/thread/ms 43.14, render/busy/ms 41.43, main/busy/ms 33.37, stage/main/update 16.78, stage/render/render 16.48, render/graph/ms 13.74, stage/render/prepare_assets 13.32, stage/main/post_update 9.32 |
| warm-up | 5.01 | 25.4 | pipelines/compile_ms 1078, pipelines/front_ms 1038, render/thread/ms 40.83, pipelines/driver_ms 40.20, pipelines/wait_ms 39.98, render/busy/ms 39.18, encode//sum 18.91, main/busy/ms 18.45, stage/render/render 16.77, render/graph/ms 14.91, stage/render/prepare_assets 11.78, stage/render/graph_submit 10.64 |
| warm-up | 5.16 | 66.9 | pipelines/compile_ms 1272, pipelines/front_ms 1230, main/busy/ms 73.19, render/thread/ms 67.04, render/busy/ms 64.80, pipelines/driver_ms 42.01, stage/main/update 38.39, pipelines/wait_ms 31.26, stage/main/post_update 28.75, stage/render/render 27.82, render/graph/ms 24.17, stage/render/prepare_assets 13.43 |
| warm-up | 5.20 | 42.8 | pipelines/compile_ms 1474, pipelines/front_ms 1429, render/thread/ms 77.75, render/busy/ms 76.14, main/busy/ms 53.80, pipelines/wait_ms 46.28, pipelines/driver_ms 45.83, stage/main/update 28.84, stage/render/render 25.79, stage/render/prepare_assets 25.46, render/graph/ms 22.65, stage/main/post_update 16.72 |
| warm-up | 5.23 | 36.0 | pipelines/compile_ms 592, pipelines/front_ms 579, render/thread/ms 50.83, render/busy/ms 49.60, stage/render/prepare_assets 27.74, main/busy/ms 21.84, pipelines/wait_ms 15.38, encode//sum 14.93, stage/render/render 13.71, pipelines/driver_ms 12.19, render/graph/ms 12.08, stage/render/graph_submit 8.20 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=cache-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (253 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **93%** (min 82%) · 2625 MHz (min 2625) · memory 8101 MHz · 94.3 W · 73 °C (max 78) · VRAM 4975 MiB · throttled 100% of samples

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
- `shadow.lamp.cache.enabled=on (command line: --set)`
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
