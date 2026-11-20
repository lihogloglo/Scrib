# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.batched_pass=on (command line: --set); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1676 frames over 11.4 s · **median 6.71 ms (149 fps)** · mean 6.82 · p95 8.20 · p99 8.99 (1% low 111 fps) · worst 14.34 ms · 5 spikes over 1.5× median · 15.7 stutters/min (work 10.5/min)
- warm-up: 120 frames over 5.7 s, worst 2572.9 ms
- hitches (> 25.0 ms): **0** measured, 31 in warm-up
- timed GPU passes: 5.44 ms of a 6.82 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 149.0, target 144
- pass worst frame 14.34 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.081 | 2.205 | 2.729 | 0.94 | 0.167 |
| main_opaque_pass_3d | 1.913 | 2.027 | 2.847 | 0.94 | 0.046 |
| bloom | 0.331 | 0.350 | 0.496 | 0.94 | 0.064 |
| ssao | 0.203 | 0.215 | 0.209 | 0.94 | 0.008 |
| batched_point_shadows | 0.167 | 0.255 | 0.331 | 0.66 | 0.111 |
| upscaling | 0.155 | 0.164 | 0.198 | 0.94 | 0.005 |
| tlas_build | 0.114 | 0.120 | 0.119 | 0.94 | 0.072 |
| tonemapping | 0.103 | 0.109 | 0.127 | 0.94 | 0.004 |
| gi/probe_update | 0.095 | 0.100 | 0.099 | 0.94 | 0.002 |
| gpu_instance_materialization | 0.095 | 0.100 | 0.099 | 0.94 | 0.004 |
| early prepass | 0.079 | 0.084 | 0.089 | 0.94 | 0.051 |
| clustering | 0.074 | 0.078 | 0.081 | 0.94 | 0.020 |
| uniform_allocation | 0.038 | 0.041 | 0.045 | 0.94 | 0.011 |
| batched_point_shadows/uniform_allocation | 0.033 | 0.051 | 0.068 | 0.66 | 0.012 |
| bin_unpacking | 0.032 | 0.034 | 0.040 | 0.94 | 0.008 |
| batched_point_shadows/early_mesh_preprocessing | 0.024 | 0.036 | 0.045 | 0.66 | 0.006 |
| range_unpacking | 0.021 | 0.022 | 0.020 | 0.94 | 0.003 |
| batched_point_shadows/bin_unpacking | 0.020 | 0.031 | 0.043 | 0.66 | 0.007 |
| early_mesh_preprocessing | 0.020 | 0.021 | 0.025 | 0.94 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.92 | 0.024 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.94 | 0.005 |
| batched_point_shadows/range_unpacking | 0.004 | 0.007 | 0.007 | 0.66 | 0.002 |
| sparse buffer updates | 0.004 | 0.006 | 0.007 | 0.57 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.94 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.94 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.017 | 7.298 | 13.57 | 10084.0 |
| render/busy/ms | 5.408 | 6.678 | 12.68 | 9063.3 |
| main/busy/ms | 5.203 | 5.989 | 10.34 | 8719.7 |
| stage/render/render | 3.602 | 4.553 | 10.70 | 6036.4 |
| render/graph/ms | 3.030 | 3.931 | 10.11 | 5077.9 |
| stage/main/post_update | 2.714 | 3.241 | 7.95 | 4548.8 |
| encode//sum | 2.187 | 2.837 | 41.93 | 3665.2 |
| stage/render/graph_submit | 2.052 | 2.821 | 9.13 | 3439.2 |
| encode/submit | 1.314 | 1.830 | 2.77 | 2201.6 |
| stage/main/update | 1.294 | 1.538 | 1.95 | 2168.3 |
| stage/render/camera_driver | 0.843 | 1.047 | 1.51 | 1413.5 |
| encode/finish_wall | 0.800 | 1.295 | 6.42 | 1340.1 |
| encode/pass:shadow_point_light/sum | 0.488 | 1.129 | 2.07 | 817.7 |
| encode/pass:early prepass/sum | 0.456 | 0.611 | 0.94 | 763.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.428 | 0.598 | 0.93 | 717.8 |
| stage/render/prepare_resources | 0.415 | 0.510 | 0.77 | 695.7 |
| render/present/ms | 0.404 | 0.938 | 2.70 | 677.6 |
| stage/render/prepare_write_phase_buffers | 0.376 | 0.597 | 1.04 | 630.7 |
| stage/main/pre_update | 0.361 | 0.458 | 0.71 | 604.7 |
| stage/main/fixed_main_loop | 0.259 | 0.684 | 1.06 | 434.3 |
| stage/render/prepare_bind_groups | 0.256 | 0.389 | 0.99 | 428.4 |
| stage/render/prepare_views | 0.255 | 0.894 | 1.50 | 428.0 |
| stage/main/first | 0.251 | 0.346 | 1.78 | 421.4 |
| bigspace/total/ms | 0.242 | 0.538 | 3.40 | 405.9 |
| stage/render/prepare_assets | 0.216 | 0.295 | 0.54 | 362.7 |
| render/acquire/ms | 0.205 | 0.844 | 1.44 | 343.1 |
| stage/render/prepare_flush | 0.192 | 0.355 | 0.77 | 322.1 |
| stage/render/prepare_meshes | 0.184 | 0.257 | 0.61 | 308.1 |
| stage/render/extract_commands | 0.168 | 0.205 | 0.46 | 281.9 |
| stage/main/last | 0.152 | 0.205 | 0.40 | 254.3 |
| stage/render/create_views | 0.144 | 0.197 | 0.46 | 240.6 |
| stream/drive/ms | 0.130 | 0.193 | 0.42 | 218.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.128 | 0.187 | 4.89 | 215.3 |
| bigspace/low_precision/ms | 0.106 | 0.291 | 3.10 | 177.6 |
| stream/tick/ms | 0.101 | 0.159 | 0.39 | 169.1 |
| physics/step/ms | 0.094 | 0.324 | 0.70 | 157.0 |
| controller/feed/ms | 0.087 | 0.112 | 0.37 | 145.6 |
| stage/render/prepare_batch_phases | 0.085 | 0.140 | 0.38 | 143.1 |
| stage/render/queue | 0.083 | 0.132 | 0.62 | 139.6 |
| anim/evaluate/ms | 0.065 | 0.120 | 0.99 | 109.5 |
| stage/render/specialize | 0.064 | 0.092 | 0.58 | 106.7 |
| pipelines/compile_ms | 0.054 | 0.000 | 90.51 | 90.5 |
| bigspace/high_precision/ms | 0.053 | 0.168 | 1.13 | 88.9 |
| pipelines/front_ms | 0.052 | 0.000 | 87.38 | 87.4 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.072 | 0.21 | 79.9 |
| encode/point_shadow_cache_copies/sum | 0.040 | 0.081 | 0.41 | 67.8 |
| anim/evaluate_cpu/ms | 0.037 | 0.057 | 0.31 | 62.7 |
| encode/resolve_diagnostics/sum | 0.033 | 0.049 | 0.30 | 55.5 |
| weather/map/ms | 0.027 | 0.037 | 0.34 | 44.4 |
| controller/write/ms | 0.025 | 0.035 | 0.32 | 42.2 |
| stage/render/cleanup | 0.022 | 0.034 | 0.38 | 37.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.19 | 22.6 |
| encode/camera_driver_span_end/sum | 0.012 | 0.022 | 0.12 | 20.8 |
| stage/render/phase_sort | 0.012 | 0.030 | 0.15 | 20.2 |
| weather/eval/ms | 0.011 | 0.016 | 0.15 | 19.1 |
| physics/insert/ms | 0.010 | 0.000 | 0.58 | 16.6 |
| script/update/ms | 0.008 | 0.011 | 0.04 | 13.4 |
| anim/apply/ms | 0.005 | 0.010 | 0.03 | 8.7 |
| script/input/ms | 0.005 | 0.006 | 0.09 | 8.3 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 6.4 |
| controller/move/ms | 0.003 | 0.010 | 0.07 | 5.8 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 5.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.12 | 3.1 |
| controller/avoid/ms | 0.002 | 0.005 | 0.02 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 1.2 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
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
| crowd/joint_blocks/count | count | 2 | 2 | 0 |
| crowd/joints_extracted/count | count | 46 | 1357 | 0 |
| crowd/joints_uploaded/count | count | 94 | 94 | 0 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.43 | 5.66 | 0.36 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.19 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.12 | 0.01 |
| encode/finish_critical | ms | 0.61 | 5.66 | 0.48 |
| encode/pass:early prepass/max | ms | 0.45 | 0.94 | 0.48 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.42 | 0.93 | 0.42 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 4.89 | 0.10 |
| encode/pass:shadow_point_light/max | ms | 0.71 | 2.07 | 0.56 |
| encode/point_shadow_cache_copies/max | ms | 0.05 | 0.41 | 0.04 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.30 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 6.10 | 12.68 | 5.08 |
| gpu/timed/ms | ms | 6.06 | 7.61 | 5.08 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 41 | 48 | 48 |
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
| lamp_cache/render/views | count | 7 | 26 | 0 |
| lamp_cache/stock_lamps | count | 0 | 1 | 0 |
| lamp_cache/store_starved | count | 0 | 0 | 0 |
| lamp_cache/store_used | count | 19 | 24 | 4 |
| lamp_cache/views | count | 7 | 26 | 0 |
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
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 7 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 159 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 261 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 6 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 24 | 61 | 7 |
| phases/shadow/bins | count | 324 | 828 | 120 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 324 | 935 | 168 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 34 | 82 | 7 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 97 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 294 | 295 | 295 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 6 | 198 | 4 |
| preprocess/bind_groups_created | count | 2 | 194 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 71 | 131 | 35 |
| process/cpu_usage | % | 25.24 | 40.38 | 31.73 |
| process/mem_usage | GiB | 3.68 | 3.71 | 3.71 |
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
| skin/joint_bytes_written/bytes | bytes | 3072 | 7680 | 0 |
| skin/joints_written/count | count | 48 | 120 | 0 |
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
| system/cpu_usage | % | 32.76 | 42.89 | 36.26 |
| system/mem_usage | % | 54.43 | 54.55 | 54.54 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 38 | 90 | 17 |
| views/all/bins | count | 458 | 1358 | 58 |
| views/all/count | count | 8 | 28 | 1 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 695 | 1891 | 86 |
| views/all/pipelines | count | 40 | 109 | 15 |
| views/all/pipelines_distinct | count | 24 | 34 | 15 |
| views/all/work_items | count | 99.0 k | 100.0 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 15 | 61 | 0 |
| views/lamp_face/bins | count | 189 | 828 | 0 |
| views/lamp_face/count | count | 7 | 27 | 0 |
| views/lamp_face/culled | count | 0 | 4 | 0 |
| views/lamp_face/drawn | count | 6 | 18 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 199 | 935 | 0 |
| views/lamp_face/pipelines | count | 18 | 82 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 127 | 768 | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 328 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 542 | 1062 | 86 |
| views/main/pipelines | count | 21 | 27 | 15 |
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
| anim/bones_written/count | count | 9.13 | 30 | 15.3 k |
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
| encode//count | count | 19.61 | 20 | 32.9 k |
| encode/bind_walk/init_textures | count | 4199 | 12.6 k | 7.04 M |
| encode/bind_walk/init_textures_skipped | count | 7144 | 18.4 k | 11.97 M |
| encode/bind_walk/inits | count | 222 | 416 | 372.3 k |
| encode/bind_walk/inits_skipped | count | 10.96 | 28 | 18.4 k |
| encode/bind_walk/merge_textures | count | 2956 | 11.3 k | 4.95 M |
| encode/bind_walk/merge_textures_skipped | count | 76.07 | 7105 | 127.5 k |
| encode/bind_walk/merges | count | 68.04 | 126 | 114.0 k |
| encode/bind_walk/merges_skipped | count | 11.92 | 47 | 20.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1676 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1676 |
| encode/pass:early prepass/count | count | 1 | 1 | 1676 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1676 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1628 |
| encode/pass:shadow_point_light/count | count | 0.70 | 1 | 1167 |
| encode/point_shadow_cache_copies/count | count | 0.69 | 1 | 1164 |
| encode/queue/command_buffers | count | 29.97 | 31 | 50.2 k |
| encode/queue/submits | count | 2 | 2 | 3352 |
| encode/queue/write_buffer/bytes | bytes | 547.5 k | 658.9 k | 917.68 M |
| encode/queue/write_buffer/calls | count | 148 | 310 | 247.9 k |
| encode/queue/write_buffer_with/bytes | bytes | 87.7 k | 88.2 k | 147.07 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 10.1 k |
| encode/queue/write_texture/bytes | bytes | 626 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1676 |
| physics/inserts | count | 0.06 | 8 | 98 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.41 | 1 | 686 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 1 | 1 |
| precache/hit | count | 0.00 | 3 | 3 |
| precache/missed | count | 0.00 | 1 | 1 |
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

## Hitches (> 25.0 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2572.9 | main/busy/ms 2546, stage/main/post_update 2442, stage/main/update 93.79, surface/advance_cpu_ms 66.99, stage/main/pre_update 6.45, stage/main/last 0.97, stage/main/first 0.57, stage/main/fixed_main_loop 0.41, stream/drive/ms 0.29, weather/map/ms 0.12, bigspace/total/ms 0.09, stream/tick/ms 0.06 |
| warm-up | 3.06 | 489.5 | main/busy/ms 2648, stage/main/post_update 2447, render/thread/ms 448, render/busy/ms 364, stage/render/prepare_assets 336, stage/main/update 186, stage/render/prepare_views 81.75, render/acquire/ms 81.54, surface/advance_cpu_ms 66.99, stage/render/create_views 14.90, stage/render/render 8.79, stage/main/pre_update 7.65 |
| warm-up | 3.11 | 48.7 | render/thread/ms 481, render/busy/ms 397, stage/render/prepare_assets 363, main/busy/ms 112, stage/main/update 95.95, stage/render/prepare_views 82.06, render/acquire/ms 81.76, stage/render/create_views 15.24, stage/render/render 12.40, render/graph/ms 9.11, stage/main/post_update 7.29, encode//sum 5.92 |
| warm-up | 3.14 | 29.9 | pipelines/wait_ms 147, pipelines/compile_ms 121, pipelines/front_ms 111, render/thread/ms 56.81, main/busy/ms 45.04, render/busy/ms 42.69, stage/main/update 31.32, stage/render/prepare_assets 28.11, stage/render/render 24.86, render/present/ms 13.73, render/graph/ms 10.83, pipelines/driver_ms 10.32 |
| warm-up | 3.19 | 45.8 | pipelines/wait_ms 1442, pipelines/compile_ms 424, pipelines/front_ms 393, main/busy/ms 63.64, render/thread/ms 61.75, render/busy/ms 47.78, pipelines/driver_ms 30.75, stage/main/update 30.35, stage/main/post_update 28.78, stage/render/render 26.09, stage/render/prepare_resources 25.69, render/present/ms 13.70 |
| warm-up | 3.22 | 29.6 | pipelines/wait_ms 1452, pipelines/compile_ms 532, pipelines/front_ms 477, encode//sum 104, render/thread/ms 66.73, render/busy/ms 64.91, main/busy/ms 58.51, pipelines/driver_ms 54.58, stage/main/post_update 45.25, stage/render/prepare_resources 26.23, stage/render/render 25.83, render/graph/ms 23.55 |
| warm-up | 3.37 | 121.0 | pipelines/compile_ms 852, pipelines/front_ms 817, main/busy/ms 76.47, render/thread/ms 76.17, render/busy/ms 69.24, stage/main/update 58.66, stage/render/prepare_assets 49.91, pipelines/driver_ms 35.55, stage/render/render 19.49, pipelines/wait_ms 15.80, render/graph/ms 12.26, stage/main/post_update 9.20 |
| warm-up | 3.43 | 48.3 | pipelines/compile_ms 348, pipelines/front_ms 318, render/thread/ms 54.72, render/busy/ms 53.11, encode//sum 42.39, pipelines/driver_ms 29.66, stage/render/prepare_assets 28.63, main/busy/ms 22.54, stage/render/render 20.82, render/graph/ms 19.07, stage/render/graph_submit 17.12, encode/finish_wall 15.34 |
| warm-up | 3.51 | 55.6 | pipelines/compile_ms 916, pipelines/front_ms 877, render/thread/ms 54.84, render/busy/ms 52.48, main/busy/ms 49.75, pipelines/driver_ms 38.62, stage/main/update 26.71, stage/render/render 24.60, stage/render/prepare_assets 24.36, render/graph/ms 21.98, pipelines/wait_ms 21.24, pipelines/lock_ms 20.63 |
| warm-up | 3.58 | 70.5 | pipelines/compile_ms 915, pipelines/front_ms 877, render/thread/ms 61.10, render/busy/ms 55.34, main/busy/ms 40.39, pipelines/driver_ms 37.46, stage/render/cleanup 33.92, stage/render/prepare_assets 30.23, pipelines/wait_ms 23.65, pipelines/lock_ms 23.00, stage/render/render 17.61, stage/main/update 17.06 |
| warm-up | 3.64 | 61.5 | pipelines/compile_ms 1400, pipelines/front_ms 1353, main/busy/ms 178, stage/main/update 141, render/thread/ms 76.34, render/busy/ms 70.17, pipelines/driver_ms 46.11, stage/render/prepare_assets 40.06, stage/render/cleanup 34.75, pipelines/wait_ms 31.06, stage/main/pre_update 15.22, stage/render/prepare_resources 14.35 |
| warm-up | 3.85 | 207.4 | pipelines/compile_ms 2826, pipelines/front_ms 2730, pipelines/wait_ms 319, main/busy/ms 188, stage/main/update 159, render/thread/ms 106, render/busy/ms 104, pipelines/driver_ms 95.65, stage/render/prepare_assets 83.10, stream/drive/ms 26.55, pipelines/lock_ms 14.96, stage/main/pre_update 12.97 |
| warm-up | 3.92 | 66.8 | pipelines/compile_ms 1890, pipelines/front_ms 1821, pipelines/wait_ms 296, render/thread/ms 101, render/busy/ms 99.52, stage/render/prepare_assets 88.34, pipelines/driver_ms 69.16, main/busy/ms 49.56, stage/main/update 34.64, stream/drive/ms 26.77, pipelines/lock_ms 12.84, stage/render/render 9.32 |
| warm-up | 3.96 | 46.7 | pipelines/compile_ms 390, pipelines/front_ms 344, render/thread/ms 86.99, render/busy/ms 85.15, stage/render/prepare_assets 73.26, main/busy/ms 56.39, pipelines/driver_ms 46.25, stage/main/update 36.29, stream/drive/ms 12.15, stage/render/render 9.60, stage/main/post_update 9.52, render/graph/ms 7.78 |
| warm-up | 4.01 | 48.1 | pipelines/compile_ms 793, pipelines/front_ms 736, render/thread/ms 78.86, render/busy/ms 77.14, pipelines/driver_ms 56.78, stage/render/prepare_assets 54.64, main/busy/ms 53.87, stage/main/update 35.61, stage/render/render 19.67, render/graph/ms 17.81, stage/render/graph_submit 15.93, stream/drive/ms 12.30 |
| warm-up | 4.23 | 26.1 | pipelines/compile_ms 785, pipelines/front_ms 757, render/thread/ms 33.42, render/busy/ms 32.12, main/busy/ms 31.28, pipelines/driver_ms 27.68, pipelines/wait_ms 27.17, stage/render/prepare_assets 17.99, stage/main/update 15.01, stage/main/post_update 7.90, stage/render/render 7.74, render/graph/ms 6.26 |
| warm-up | 4.36 | 37.2 | pipelines/compile_ms 506, pipelines/front_ms 489, main/busy/ms 59.85, render/thread/ms 43.56, render/busy/ms 41.35, stage/main/update 35.86, pipelines/driver_ms 16.55, stage/render/prepare_assets 16.24, stage/main/post_update 15.08, stage/render/render 8.58, render/graph/ms 6.78, stage/render/prepare_flush 5.49 |
| warm-up | 4.42 | 58.3 | pipelines/compile_ms 1047, pipelines/front_ms 1017, main/busy/ms 116, stage/main/update 95.49, render/thread/ms 67.76, render/busy/ms 65.06, stream/drive/ms 52.30, pipelines/driver_ms 30.28, stage/render/render 22.81, render/graph/ms 20.18, stage/render/prepare_assets 19.31, pipelines/wait_ms 16.81 |
| warm-up | 4.52 | 97.3 | pipelines/compile_ms 2073, pipelines/front_ms 1993, main/busy/ms 126, stage/main/update 88.26, pipelines/driver_ms 79.87, render/thread/ms 75.70, render/busy/ms 73.31, stream/drive/ms 52.72, pipelines/wait_ms 32.55, stage/render/render 28.78, render/graph/ms 25.52, stage/render/prepare_assets 25.12 |
| warm-up | 4.58 | 60.3 | pipelines/compile_ms 1790, pipelines/front_ms 1713, main/busy/ms 84.34, pipelines/driver_ms 76.17, render/thread/ms 73.36, render/busy/ms 71.27, pipelines/wait_ms 48.21, stage/main/update 40.08, stage/render/prepare_assets 33.92, stage/main/post_update 19.69, stage/render/render 19.53, stage/main/pre_update 17.56 |
| warm-up | 4.67 | 97.7 | pipelines/compile_ms 870, pipelines/front_ms 826, render/thread/ms 100, render/busy/ms 98.96, main/busy/ms 54.44, stage/render/prepare_assets 44.02, pipelines/driver_ms 43.24, stage/render/create_views 32.93, pipelines/wait_ms 31.49, stage/main/update 28.31, stage/main/post_update 16.40, stream/drive/ms 14.19 |
| warm-up | 4.71 | 40.8 | pipelines/compile_ms 335, pipelines/front_ms 310, render/thread/ms 88.13, render/busy/ms 87.19, stage/render/prepare_assets 43.69, main/busy/ms 36.21, stage/render/create_views 32.35, pipelines/driver_ms 24.52, stage/main/update 18.61, stage/main/post_update 9.72, stage/render/render 5.88, render/graph/ms 4.75 |
| warm-up | 4.78 | 53.5 | pipelines/compile_ms 124, pipelines/front_ms 119, render/thread/ms 43.17, main/busy/ms 41.23, render/busy/ms 40.51, stage/main/update 30.46, stream/drive/ms 21.60, stage/render/create_views 11.39, stage/render/render 11.22, stage/render/prepare_assets 9.98, render/graph/ms 8.49, stage/main/post_update 6.92 |
| warm-up | 4.81 | 27.0 | pipelines/compile_ms 124, pipelines/front_ms 119, render/thread/ms 54.93, render/busy/ms 51.79, main/busy/ms 42.63, stage/main/update 21.91, stage/render/render 18.70, render/graph/ms 14.61, stage/render/graph_submit 11.75, stage/render/create_views 11.70, stage/render/prepare_assets 10.82, stage/main/post_update 10.22 |
| warm-up | 4.84 | 32.4 | pipelines/compile_ms 290, pipelines/front_ms 279, main/busy/ms 57.19, render/thread/ms 45.40, render/busy/ms 42.71, stage/main/update 31.17, stage/render/render 17.53, render/graph/ms 13.74, stage/main/post_update 13.17, pipelines/driver_ms 11.06, stage/render/graph_submit 10.71, encode//sum 8.07 |
| warm-up | 4.87 | 28.0 | pipelines/compile_ms 374, pipelines/front_ms 358, render/thread/ms 40.86, render/busy/ms 39.32, main/busy/ms 35.96, stage/main/update 19.50, pipelines/driver_ms 16.43, stage/render/prepare_assets 11.92, stage/main/post_update 10.08, stage/render/render 9.15, render/graph/ms 7.25, encode//sum 5.60 |
| warm-up | 4.92 | 34.7 | pipelines/compile_ms 1202, pipelines/front_ms 1155, pipelines/driver_ms 47.39, render/thread/ms 45.93, render/busy/ms 43.88, main/busy/ms 34.22, stage/render/render 15.60, stage/render/prepare_flush 14.36, render/graph/ms 13.62, stage/main/post_update 12.71, stage/main/update 10.53, stage/render/graph_submit 9.46 |
| warm-up | 4.97 | 35.2 | pipelines/compile_ms 300, pipelines/front_ms 290, pipelines/wait_ms 84.37, render/thread/ms 39.54, render/busy/ms 38.22, main/busy/ms 33.18, stage/main/post_update 18.61, stage/render/prepare_assets 15.00, stage/render/render 11.66, render/graph/ms 9.98, pipelines/driver_ms 9.70, stage/main/update 9.68 |
| warm-up | 5.06 | 36.6 | pipelines/compile_ms 623, pipelines/front_ms 607, main/busy/ms 52.97, render/thread/ms 39.66, render/busy/ms 37.91, stage/main/update 36.69, pipelines/wait_ms 29.19, pipelines/driver_ms 16.92, stage/render/render 14.97, render/graph/ms 12.75, stage/main/post_update 11.39, stage/render/prepare_assets 9.23 |
| warm-up | 5.11 | 46.8 | pipelines/compile_ms 705, pipelines/front_ms 680, render/thread/ms 52.96, render/busy/ms 51.37, main/busy/ms 48.42, stage/main/update 30.49, stage/render/prepare_assets 26.15, pipelines/driver_ms 25.00, stage/render/render 13.71, render/graph/ms 11.64, stage/main/post_update 10.08, stage/render/graph_submit 8.04 |
| warm-up | 5.15 | 35.9 | pipelines/compile_ms 473, pipelines/front_ms 456, render/thread/ms 49.29, render/busy/ms 48.22, stage/render/prepare_assets 33.09, main/busy/ms 27.87, pipelines/driver_ms 17.30, stage/main/update 13.67, stage/render/render 8.54, render/graph/ms 7.12, stage/main/post_update 7.11, stage/render/graph_submit 5.09 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=batched-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (253 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **95%** (min 87%) · 2625 MHz (min 2625) · memory 8101 MHz · 99.3 W · 74 °C (max 77) · VRAM 4961 MiB · throttled 100% of samples

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
- `shadow.lamp.batched_pass=on (command line: --set)`
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
