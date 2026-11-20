# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.batched_pass=on (command line: --set); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1677 frames over 11.4 s · **median 6.61 ms (151 fps)** · mean 6.82 · p95 8.38 · p99 10.90 (1% low 92 fps) · worst 17.14 ms · 26 spikes over 1.5× median · 10.5 stutters/min (work 10.5/min)
- warm-up: 120 frames over 5.8 s, worst 2536.1 ms
- hitches (> 25.0 ms): **0** measured, 29 in warm-up
- timed GPU passes: 5.16 ms of a 6.82 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 151.3, target 144
- pass worst frame 17.14 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.958 | 2.148 | 2.464 | 0.91 | 0.171 |
| main_opaque_pass_3d | 1.817 | 1.993 | 2.940 | 0.91 | 0.047 |
| bloom | 0.326 | 0.358 | 0.689 | 0.91 | 0.065 |
| ssao | 0.188 | 0.206 | 0.295 | 0.91 | 0.008 |
| batched_point_shadows | 0.157 | 0.251 | 0.328 | 0.63 | 0.115 |
| upscaling | 0.147 | 0.161 | 0.195 | 0.91 | 0.005 |
| tlas_build | 0.106 | 0.117 | 0.119 | 0.91 | 0.074 |
| tonemapping | 0.098 | 0.107 | 0.126 | 0.91 | 0.004 |
| gpu_instance_materialization | 0.091 | 0.100 | 0.101 | 0.91 | 0.005 |
| gi/probe_update | 0.086 | 0.095 | 0.099 | 0.91 | 0.002 |
| early prepass | 0.073 | 0.080 | 0.087 | 0.91 | 0.051 |
| clustering | 0.071 | 0.078 | 0.076 | 0.91 | 0.020 |
| uniform_allocation | 0.039 | 0.043 | 0.045 | 0.91 | 0.011 |
| batched_point_shadows/uniform_allocation | 0.030 | 0.048 | 0.067 | 0.63 | 0.012 |
| bin_unpacking | 0.030 | 0.032 | 0.040 | 0.91 | 0.009 |
| batched_point_shadows/early_mesh_preprocessing | 0.022 | 0.035 | 0.045 | 0.63 | 0.006 |
| batched_point_shadows/bin_unpacking | 0.019 | 0.031 | 0.043 | 0.63 | 0.008 |
| range_unpacking | 0.018 | 0.020 | 0.020 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.017 | 0.019 | 0.025 | 0.91 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.89 | 0.024 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.91 | 0.005 |
| batched_point_shadows/range_unpacking | 0.004 | 0.007 | 0.007 | 0.63 | 0.003 |
| sparse buffer updates | 0.004 | 0.006 | 0.007 | 0.56 | 0.004 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.91 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.91 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 8.501 | 0.000 | 5730.26 | 14256.7 |
| render/thread/ms | 5.979 | 7.521 | 15.74 | 10027.4 |
| pipelines/driver_ms | 5.755 | 0.000 | 5446.18 | 9650.8 |
| render/busy/ms | 5.433 | 6.890 | 15.05 | 9111.9 |
| main/busy/ms | 5.272 | 6.199 | 10.66 | 8841.9 |
| stage/render/render | 3.610 | 4.747 | 7.80 | 6053.3 |
| render/graph/ms | 3.018 | 4.000 | 6.94 | 5061.4 |
| pipelines/front_ms | 2.746 | 0.000 | 373.34 | 4605.9 |
| stage/main/post_update | 2.742 | 3.349 | 6.57 | 4598.6 |
| encode//sum | 2.162 | 2.663 | 6.07 | 3625.6 |
| pipelines/wait_ms | 2.031 | 0.000 | 2348.85 | 3406.1 |
| stage/render/graph_submit | 2.028 | 2.875 | 5.35 | 3400.7 |
| stage/main/update | 1.316 | 1.650 | 2.72 | 2207.4 |
| encode/submit | 1.299 | 1.902 | 2.71 | 2178.8 |
| stage/render/camera_driver | 0.854 | 1.076 | 1.51 | 1432.1 |
| encode/finish_wall | 0.803 | 1.338 | 3.10 | 1347.4 |
| encode/pass:shadow_point_light/sum | 0.492 | 1.181 | 2.77 | 825.3 |
| encode/pass:early prepass/sum | 0.450 | 0.599 | 0.97 | 754.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.431 | 0.598 | 0.92 | 722.5 |
| render/present/ms | 0.421 | 0.945 | 2.28 | 705.8 |
| stage/render/prepare_resources | 0.419 | 0.546 | 1.01 | 702.8 |
| stage/render/prepare_write_phase_buffers | 0.381 | 0.639 | 2.89 | 638.5 |
| stage/main/pre_update | 0.367 | 0.476 | 3.50 | 615.7 |
| stage/render/prepare_bind_groups | 0.267 | 0.433 | 1.80 | 448.5 |
| stage/main/fixed_main_loop | 0.265 | 0.704 | 2.16 | 445.0 |
| stage/main/first | 0.252 | 0.360 | 1.61 | 423.0 |
| bigspace/total/ms | 0.237 | 0.540 | 1.53 | 397.4 |
| stage/render/prepare_assets | 0.217 | 0.310 | 0.88 | 364.2 |
| stage/render/prepare_flush | 0.192 | 0.356 | 1.00 | 321.3 |
| stage/render/prepare_meshes | 0.182 | 0.252 | 0.53 | 304.7 |
| stage/render/prepare_views | 0.174 | 0.508 | 1.27 | 291.7 |
| stage/render/extract_commands | 0.169 | 0.212 | 0.34 | 283.4 |
| stage/main/last | 0.153 | 0.200 | 0.59 | 257.2 |
| stage/render/create_views | 0.147 | 0.202 | 3.14 | 245.9 |
| stream/drive/ms | 0.132 | 0.190 | 0.33 | 221.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.126 | 0.185 | 1.03 | 210.6 |
| render/acquire/ms | 0.125 | 0.451 | 1.23 | 209.8 |
| stream/tick/ms | 0.103 | 0.155 | 0.28 | 173.0 |
| bigspace/low_precision/ms | 0.103 | 0.314 | 1.01 | 172.3 |
| physics/step/ms | 0.092 | 0.323 | 0.76 | 153.6 |
| stage/render/prepare_batch_phases | 0.087 | 0.148 | 0.39 | 145.3 |
| stage/render/queue | 0.084 | 0.131 | 0.79 | 141.5 |
| controller/feed/ms | 0.083 | 0.108 | 0.38 | 139.1 |
| stage/render/specialize | 0.076 | 0.158 | 0.62 | 126.8 |
| anim/evaluate/ms | 0.068 | 0.147 | 0.67 | 114.0 |
| bigspace/high_precision/ms | 0.052 | 0.135 | 0.93 | 87.3 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.076 | 0.26 | 81.8 |
| encode/point_shadow_cache_copies/sum | 0.040 | 0.083 | 0.34 | 67.8 |
| anim/evaluate_cpu/ms | 0.037 | 0.058 | 0.20 | 62.2 |
| encode/resolve_diagnostics/sum | 0.033 | 0.045 | 0.29 | 55.4 |
| weather/map/ms | 0.027 | 0.037 | 0.36 | 44.6 |
| controller/write/ms | 0.026 | 0.037 | 0.12 | 44.3 |
| stage/render/cleanup | 0.022 | 0.034 | 0.21 | 37.2 |
| physics/insert/ms | 0.019 | 0.155 | 0.65 | 31.5 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.28 | 22.2 |
| stage/render/phase_sort | 0.013 | 0.031 | 0.23 | 21.1 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.39 | 19.9 |
| weather/eval/ms | 0.012 | 0.016 | 0.04 | 19.5 |
| script/update/ms | 0.008 | 0.011 | 0.04 | 13.9 |
| anim/apply/ms | 0.005 | 0.010 | 0.05 | 8.8 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 8.2 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 6.3 |
| controller/move/ms | 0.004 | 0.010 | 0.18 | 6.1 |
| pipelines/lock_ms | 0.003 | 0.000 | 1.60 | 5.8 |
| script/ui/ms | 0.003 | 0.005 | 0.08 | 5.8 |
| controller/avoid/ms | 0.002 | 0.005 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.0 |
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
| encode//max | ms | 0.43 | 1.09 | 0.43 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.28 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.39 | 0.01 |
| encode/finish_critical | ms | 0.58 | 2.77 | 0.43 |
| encode/pass:early prepass/max | ms | 0.45 | 0.97 | 0.38 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.42 | 0.92 | 0.30 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 1.03 | 0.09 |
| encode/pass:shadow_point_light/max | ms | 0.71 | 2.77 | 0.47 |
| encode/point_shadow_cache_copies/max | ms | 0.06 | 0.34 | 0.03 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.29 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.80 | 15.05 | 5.26 |
| gpu/timed/ms | ms | 5.74 | 8.04 | 5.26 |
| lamp_cache/captures | count | 0 | 2 | 0 |
| lamp_cache/clean_faces | count | 41 | 48 | 48 |
| lamp_cache/copies | count | 5 | 13 | 0 |
| lamp_cache/deferred_faces | count | 0 | 13 | 0 |
| lamp_cache/lamps | count | 8 | 8 | 8 |
| lamp_cache/mover_faces | count | 5 | 12 | 0 |
| lamp_cache/movers | count | 200 | 269 | 56 |
| lamp_cache/queued | count | 0 | 18 | 0 |
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
| phases/opaque3d/bins | count | 155 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 261 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 155 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 261 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 24 | 63 | 7 |
| phases/shadow/bins | count | 324 | 828 | 120 |
| phases/shadow/instance_batches | count | 0 | 4 | 0 |
| phases/shadow/instance_draws | count | 0 | 354 | 0 |
| phases/shadow/instance_slots | count | 0 | 16.4 k | 0 |
| phases/shadow/items | count | 324 | 935 | 168 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 34 | 82 | 7 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 148 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 289 | 289 | 289 |
| pipelines/waiting | count | 0 | 16 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 0 | 14 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 34 | 0 |
| preprocess/bind_groups | count | 6 | 202 | 4 |
| preprocess/bind_groups_created | count | 2 | 198 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 6 | 4 |
| preprocess/bind_groups_reused | count | 71 | 131 | 35 |
| process/cpu_usage | % | 26.02 | 84.38 | 32.06 |
| process/mem_usage | GiB | 3.71 | 3.75 | 3.74 |
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
| stream/latency/s | s | 0.07 | 0.07 | 0.07 |
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
| system/cpu_usage | % | 35.70 | 90.32 | 35.03 |
| system/mem_usage | % | 51.49 | 51.59 | 51.51 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 35 | 92 | 17 |
| views/all/bins | count | 423 | 1358 | 58 |
| views/all/count | count | 7 | 28 | 1 |
| views/all/instance_batches | count | 8 | 12 | 8 |
| views/all/instance_draws | count | 3220 | 3574 | 3220 |
| views/all/instance_slots | count | 98.3 k | 114.7 k | 98.3 k |
| views/all/items | count | 675 | 1942 | 86 |
| views/all/pipelines | count | 39 | 109 | 15 |
| views/all/pipelines_distinct | count | 24 | 34 | 15 |
| views/all/work_items | count | 98.9 k | 116.3 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 12 | 63 | 0 |
| views/lamp_face/bins | count | 163 | 828 | 0 |
| views/lamp_face/count | count | 6 | 27 | 0 |
| views/lamp_face/culled | count | 0 | 4 | 0 |
| views/lamp_face/drawn | count | 5 | 18 | 0 |
| views/lamp_face/instance_batches | count | 0 | 4 | 0 |
| views/lamp_face/instance_draws | count | 0 | 354 | 0 |
| views/lamp_face/instance_slots | count | 0 | 16.4 k | 0 |
| views/lamp_face/items | count | 163 | 935 | 0 |
| views/lamp_face/pipelines | count | 18 | 82 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 107 | 16.9 k | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 320 | 582 | 58 |
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
| anim/bones_written/count | count | 8.81 | 30 | 14.8 k |
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
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 19.61 | 20 | 32.9 k |
| encode/bind_walk/init_textures | count | 4215 | 12.6 k | 7.07 M |
| encode/bind_walk/init_textures_skipped | count | 7491 | 19.8 k | 12.56 M |
| encode/bind_walk/inits | count | 220 | 425 | 369.4 k |
| encode/bind_walk/inits_skipped | count | 11.38 | 27 | 19.1 k |
| encode/bind_walk/merge_textures | count | 2972 | 11.3 k | 4.98 M |
| encode/bind_walk/merge_textures_skipped | count | 87.23 | 7105 | 146.3 k |
| encode/bind_walk/merges | count | 67.32 | 131 | 112.9 k |
| encode/bind_walk/merges_skipped | count | 11.93 | 44 | 20.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1677 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1677 |
| encode/pass:early prepass/count | count | 1 | 1 | 1677 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1677 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1628 |
| encode/pass:shadow_point_light/count | count | 0.69 | 1 | 1155 |
| encode/point_shadow_cache_copies/count | count | 0.69 | 1 | 1152 |
| encode/queue/command_buffers | count | 29.96 | 31 | 50.2 k |
| encode/queue/submits | count | 2 | 2 | 3354 |
| encode/queue/write_buffer/bytes | bytes | 542.0 k | 664.0 k | 908.99 M |
| encode/queue/write_buffer/calls | count | 147 | 316 | 246.2 k |
| encode/queue/write_buffer_with/bytes | bytes | 92.4 k | 92.7 k | 154.99 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 10.1 k |
| encode/queue/write_texture/bytes | bytes | 625 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1677 |
| physics/inserts | count | 0.09 | 8 | 149 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.41 | 1 | 686 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.02 | 1 | 36 |
| pipelines/created | count | 0.04 | 8 | 67 |
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
| warm-up | 2.54 | 2536.1 | main/busy/ms 2504, stage/main/post_update 2398, stage/main/update 95.88, surface/advance_cpu_ms 68.12, stage/main/pre_update 6.87, stage/main/last 1.32, stage/main/first 0.45, stream/drive/ms 0.40, stage/main/fixed_main_loop 0.37, weather/map/ms 0.15, stream/tick/ms 0.06, bigspace/total/ms 0.06 |
| warm-up | 3.28 | 743.8 | main/busy/ms 2628, stage/main/post_update 2404, render/thread/ms 699, render/busy/ms 594, stage/render/prepare_assets 548, stage/main/update 208, stage/render/prepare_views 104, render/acquire/ms 103, surface/advance_cpu_ms 68.12, stage/render/create_views 32.11, stage/render/render 9.35, stage/main/pre_update 8.03 |
| warm-up | 3.33 | 48.5 | render/thread/ms 732, render/busy/ms 626, stage/render/prepare_assets 575, main/busy/ms 137, stage/main/update 117, stage/render/prepare_views 104, render/acquire/ms 104, stage/render/create_views 32.27, stage/render/render 13.76, render/graph/ms 9.84, stage/main/post_update 8.40, stage/main/fixed_main_loop 8.00 |
| warm-up | 3.41 | 54.9 | pipelines/wait_ms 1523, pipelines/compile_ms 484, pipelines/front_ms 450, main/busy/ms 73.27, stage/main/post_update 38.87, pipelines/driver_ms 33.86, render/thread/ms 26.85, stage/main/pre_update 21.74, render/busy/ms 20.52, stage/render/render 15.70, render/graph/ms 9.73, stage/main/update 9.35 |
| warm-up | 3.45 | 42.2 | pipelines/wait_ms 1552, pipelines/compile_ms 591, pipelines/front_ms 555, main/busy/ms 72.56, render/thread/ms 56.81, render/busy/ms 54.62, stage/main/pre_update 38.42, pipelines/driver_ms 35.98, stage/render/prepare_assets 18.85, stage/render/render 17.17, render/graph/ms 13.79, stage/main/post_update 13.05 |
| warm-up | 3.48 | 28.2 | pipelines/compile_ms 650, pipelines/front_ms 626, render/thread/ms 66.10, pipelines/wait_ms 64.77, render/busy/ms 64.19, main/busy/ms 50.97, stage/render/prepare_assets 30.67, pipelines/driver_ms 24.34, stage/main/pre_update 20.31, stage/render/render 15.99, render/graph/ms 12.58, stage/main/fixed_main_loop 10.78 |
| warm-up | 3.60 | 101.2 | pipelines/compile_ms 713, pipelines/front_ms 689, main/busy/ms 78.18, render/thread/ms 71.46, render/busy/ms 66.89, stage/main/update 60.15, stage/render/prepare_assets 47.49, pipelines/wait_ms 30.86, pipelines/driver_ms 23.88, stage/render/render 13.37, stage/main/post_update 11.56, render/graph/ms 8.97 |
| warm-up | 3.65 | 35.5 | pipelines/compile_ms 1014, pipelines/front_ms 941, pipelines/driver_ms 73.07, render/thread/ms 48.78, render/busy/ms 46.87, stage/render/prepare_assets 36.81, main/busy/ms 23.40, stage/main/update 7.72, stage/render/render 6.92, stage/main/post_update 6.22, render/graph/ms 5.01, stage/main/first 4.59 |
| warm-up | 3.70 | 32.5 | pipelines/compile_ms 106, pipelines/front_ms 101, render/thread/ms 43.76, render/busy/ms 42.17, main/busy/ms 37.95, stage/main/update 27.33, stage/render/prepare_assets 18.72, stream/drive/ms 17.68, stage/render/prepare_resources 8.82, stage/main/post_update 7.27, stage/render/render 7.19, render/graph/ms 5.37 |
| warm-up | 3.74 | 38.2 | pipelines/compile_ms 627, pipelines/front_ms 603, render/thread/ms 56.47, render/busy/ms 54.66, main/busy/ms 46.59, stage/render/prepare_assets 28.40, stage/main/update 26.96, pipelines/driver_ms 24.51, stream/drive/ms 18.64, stage/render/prepare_resources 9.05, stage/main/post_update 8.87, stage/render/render 8.63 |
| warm-up | 3.79 | 51.4 | pipelines/compile_ms 880, pipelines/front_ms 844, main/busy/ms 164, stage/main/update 142, render/thread/ms 67.51, render/busy/ms 66.08, stage/render/prepare_assets 54.34, pipelines/driver_ms 35.87, stage/main/pre_update 9.78, stage/render/render 7.58, stage/main/post_update 7.49, render/graph/ms 5.99 |
| warm-up | 4.00 | 211.7 | pipelines/compile_ms 2145, pipelines/front_ms 2076, main/busy/ms 150, stage/main/update 137, render/thread/ms 106, render/busy/ms 104, stage/render/prepare_assets 70.30, pipelines/driver_ms 69.03, pipelines/wait_ms 66.19, stage/render/prepare_resources 24.18, stage/render/render 7.96, pipelines/lock_ms 7.73 |
| warm-up | 4.05 | 52.2 | pipelines/compile_ms 1915, pipelines/front_ms 1856, render/thread/ms 97.33, render/busy/ms 95.97, pipelines/wait_ms 67.54, stage/render/prepare_assets 61.96, pipelines/driver_ms 59.51, main/busy/ms 57.29, stage/main/update 37.32, stream/drive/ms 32.66, stage/render/prepare_resources 24.19, pipelines/lock_ms 9.18 |
| warm-up | 4.10 | 45.0 | pipelines/compile_ms 262, pipelines/front_ms 250, main/busy/ms 88.73, render/thread/ms 54.87, render/busy/ms 53.26, stage/main/update 39.08, stage/render/prepare_assets 34.04, stream/drive/ms 32.80, stage/main/post_update 31.97, pipelines/wait_ms 23.86, pipelines/driver_ms 11.62, stage/render/render 9.68 |
| warm-up | 4.13 | 36.3 | pipelines/compile_ms 183, pipelines/front_ms 176, main/busy/ms 59.87, render/thread/ms 37.69, render/busy/ms 35.73, stage/main/post_update 30.51, pipelines/wait_ms 21.90, stage/main/update 18.65, stream/drive/ms 12.98, stage/render/prepare_assets 12.02, stage/render/render 11.08, render/graph/ms 8.74 |
| warm-up | 4.17 | 31.7 | pipelines/compile_ms 383, pipelines/front_ms 367, main/busy/ms 32.41, render/thread/ms 31.83, render/busy/ms 29.84, stage/main/update 18.70, pipelines/driver_ms 15.33, stage/render/prepare_assets 15.26, stream/drive/ms 12.98, stage/render/render 9.03, stage/main/post_update 7.90, render/graph/ms 6.81 |
| warm-up | 4.20 | 37.6 | pipelines/compile_ms 522, pipelines/front_ms 498, pipelines/wait_ms 58.42, render/thread/ms 45.14, render/busy/ms 43.51, stage/render/prepare_assets 33.31, pipelines/driver_ms 24.74, main/busy/ms 24.31, stage/main/post_update 9.94, stage/main/update 8.23, stage/render/render 7.22, render/graph/ms 5.41 |
| warm-up | 4.42 | 26.4 | main/busy/ms 32.00, render/thread/ms 31.46, render/busy/ms 29.84, stage/main/update 13.40, stage/render/render 11.19, stage/main/post_update 10.93, stage/render/prepare_assets 9.53, render/graph/ms 9.26, stage/render/graph_submit 5.99, encode/submit 4.38, encode//sum 4.02, stage/render/camera_driver 3.10 |
| warm-up | 4.45 | 28.0 | main/busy/ms 45.35, render/thread/ms 43.01, render/busy/ms 41.29, stage/main/update 33.16, stage/render/render 13.35, render/graph/ms 11.03, stage/render/prepare_assets 9.19, stage/main/post_update 7.96, stage/render/graph_submit 6.04, encode//sum 5.91, stage/render/camera_driver 4.45, encode/submit 4.15 |
| warm-up | 4.49 | 43.9 | main/busy/ms 80.28, stage/main/update 63.04, render/thread/ms 52.14, render/busy/ms 50.20, stream/drive/ms 34.96, stage/render/prepare_assets 20.55, stage/render/render 12.33, stage/main/post_update 11.27, render/graph/ms 9.95, encode//sum 6.52, stage/render/graph_submit 6.05, encode/submit 4.12 |
| warm-up | 4.56 | 69.0 | main/busy/ms 84.85, render/thread/ms 65.64, render/busy/ms 63.47, stage/main/update 57.29, stream/drive/ms 36.17, stage/render/prepare_assets 27.55, stage/main/post_update 19.99, stage/render/render 13.40, render/graph/ms 10.78, stage/render/graph_submit 7.10, encode//sum 6.40, encode/submit 5.40 |
| warm-up | 4.62 | 58.6 | render/thread/ms 79.41, main/busy/ms 77.28, render/busy/ms 77.14, stage/main/update 44.37, stage/render/prepare_assets 35.09, stage/main/post_update 22.24, stage/render/render 16.66, stream/drive/ms 15.28, render/graph/ms 13.59, stage/render/graph_submit 8.50, encode//sum 7.67, encode/submit 5.59 |
| warm-up | 4.73 | 107.7 | render/thread/ms 110, render/busy/ms 108, main/busy/ms 64.60, stage/render/prepare_assets 55.31, stage/main/update 37.59, stage/render/create_views 28.90, stage/main/post_update 16.13, stream/drive/ms 15.88, stage/render/render 13.00, render/graph/ms 10.62, stage/render/graph_submit 6.83, encode//sum 6.45 |
| warm-up | 4.78 | 52.1 | render/thread/ms 102, render/busy/ms 101, stage/render/prepare_assets 53.27, main/busy/ms 42.46, stage/render/create_views 27.70, stage/main/update 22.45, stage/main/post_update 11.23, stage/render/render 9.81, render/graph/ms 8.06, stage/render/graph_submit 5.76, stream/drive/ms 5.58, encode//sum 5.06 |
| warm-up | 4.83 | 29.7 | main/busy/ms 46.23, render/thread/ms 37.93, render/busy/ms 36.45, stage/main/update 31.23, stream/drive/ms 21.26, stage/render/create_views 10.59, stage/render/render 9.95, encode//sum 9.77, stage/main/post_update 9.01, render/graph/ms 8.06, stage/render/prepare_assets 7.12, stage/render/graph_submit 5.08 |
| warm-up | 5.02 | 37.9 | main/busy/ms 46.38, render/thread/ms 32.22, render/busy/ms 30.60, stage/main/update 29.09, stage/render/render 13.85, render/graph/ms 11.72, stage/main/post_update 11.69, encode//sum 8.58, stage/render/graph_submit 7.84, stage/render/prepare_assets 6.99, encode/submit 4.68, stream/drive/ms 3.55 |
| warm-up | 5.06 | 43.1 | pipelines/compile_ms 2279, pipelines/driver_ms 2046, pipelines/front_ms 233, render/thread/ms 54.82, main/busy/ms 54.19, render/busy/ms 53.14, pipelines/wait_ms 38.80, stage/main/update 32.63, stage/render/prepare_assets 27.79, stage/render/render 13.44, stage/main/post_update 12.55, render/graph/ms 11.27 |
| warm-up | 5.11 | 46.3 | pipelines/compile_ms 2279, pipelines/driver_ms 2046, pipelines/front_ms 233, render/thread/ms 59.61, render/busy/ms 58.39, main/busy/ms 42.33, stage/render/prepare_assets 40.78, pipelines/wait_ms 38.80, stage/main/update 23.53, stage/main/post_update 11.30, stage/render/render 8.57, render/graph/ms 6.91 |
| warm-up | 5.70 | 29.9 | pipelines/wait_ms 1984, pipelines/compile_ms 169, pipelines/front_ms 162, render/thread/ms 40.44, render/busy/ms 39.03, main/busy/ms 34.70, stage/main/update 22.41, stage/render/prepare_flush 17.88, stage/render/render 12.51, render/graph/ms 10.76, stage/main/post_update 9.06, stage/render/graph_submit 7.71 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.cache.enabled=on --set shadow.lamp.batched_pass=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=batched-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 10% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (253 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **92%** (min 42%) · 2625 MHz (min 2625) · memory 8101 MHz · 94.7 W · 70 °C (max 75) · VRAM 4898 MiB · throttled 100% of samples

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
