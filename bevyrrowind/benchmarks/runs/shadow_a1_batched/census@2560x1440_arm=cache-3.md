# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.cache.enabled=on (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1612 frames over 11.4 s · **median 6.85 ms (146 fps)** · mean 7.09 · p95 8.97 · p99 9.82 (1% low 102 fps) · worst 20.05 ms · 7 spikes over 1.5× median · 10.5 stutters/min (work 5.2/min)
- warm-up: 120 frames over 5.8 s, worst 2552.3 ms
- hitches (> 25.0 ms): **0** measured, 34 in warm-up
- timed GPU passes: 5.52 ms of a 7.09 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 145.9, target 144
- pass worst frame 20.05 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.000 | 2.157 | 2.440 | 0.93 | 0.168 |
| main_opaque_pass_3d | 1.831 | 1.974 | 2.862 | 0.93 | 0.036 |
| auxiliary_views | 0.470 | 0.507 | 1.323 | 0.93 | 0.413 |
| bloom | 0.317 | 0.342 | 0.493 | 0.93 | 0.064 |
| ssao | 0.188 | 0.203 | 0.206 | 0.93 | 0.007 |
| upscaling | 0.151 | 0.163 | 0.201 | 0.93 | 0.004 |
| tlas_build | 0.109 | 0.117 | 0.120 | 0.93 | 0.075 |
| tonemapping | 0.099 | 0.107 | 0.119 | 0.93 | 0.004 |
| gi/probe_update | 0.093 | 0.100 | 0.100 | 0.93 | 0.002 |
| gpu_instance_materialization | 0.090 | 0.097 | 0.098 | 0.93 | 0.004 |
| early prepass | 0.076 | 0.082 | 0.087 | 0.93 | 0.048 |
| clustering | 0.069 | 0.075 | 0.077 | 0.93 | 0.019 |
| uniform_allocation | 0.034 | 0.037 | 0.045 | 0.93 | 0.009 |
| bin_unpacking | 0.031 | 0.034 | 0.040 | 0.93 | 0.007 |
| range_unpacking | 0.018 | 0.019 | 0.020 | 0.93 | 0.002 |
| early_mesh_preprocessing | 0.017 | 0.019 | 0.026 | 0.93 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.90 | 0.023 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.93 | 0.005 |
| sparse buffer updates | 0.003 | 0.006 | 0.007 | 0.56 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.93 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.269 | 8.034 | 18.77 | 10105.4 |
| render/busy/ms | 5.664 | 7.358 | 11.97 | 9130.5 |
| main/busy/ms | 5.061 | 5.780 | 6.92 | 8158.5 |
| stage/render/render | 4.018 | 5.569 | 16.60 | 6477.1 |
| render/graph/ms | 3.410 | 4.804 | 6.66 | 5497.2 |
| encode//sum | 2.775 | 4.044 | 10.40 | 4472.9 |
| stage/main/post_update | 2.638 | 3.120 | 3.83 | 4252.0 |
| stage/render/graph_submit | 2.140 | 3.034 | 4.94 | 3449.0 |
| encode/submit | 1.559 | 2.364 | 3.52 | 2513.4 |
| stage/main/update | 1.239 | 1.518 | 2.11 | 1997.2 |
| stage/render/camera_driver | 1.133 | 1.719 | 2.26 | 1827.0 |
| encode/finish_wall | 0.667 | 0.896 | 1.75 | 1075.0 |
| render/present/ms | 0.446 | 1.223 | 13.87 | 719.6 |
| encode/pass:early prepass/sum | 0.415 | 0.582 | 0.83 | 668.4 |
| stage/render/prepare_resources | 0.389 | 0.486 | 3.33 | 627.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.388 | 0.575 | 0.88 | 625.1 |
| stage/main/pre_update | 0.359 | 0.455 | 0.64 | 579.4 |
| stage/render/prepare_write_phase_buffers | 0.346 | 0.559 | 1.02 | 557.6 |
| encode/pass:shadow_point_light/sum | 0.260 | 0.565 | 2.35 | 419.1 |
| stage/main/fixed_main_loop | 0.251 | 0.618 | 0.85 | 405.0 |
| stage/main/first | 0.242 | 0.335 | 2.16 | 390.1 |
| stage/render/prepare_bind_groups | 0.236 | 0.357 | 0.98 | 381.2 |
| stage/render/prepare_assets | 0.231 | 0.328 | 0.59 | 372.8 |
| bigspace/total/ms | 0.217 | 0.407 | 0.95 | 349.3 |
| stage/render/prepare_views | 0.205 | 0.777 | 1.28 | 331.0 |
| stage/render/prepare_flush | 0.182 | 0.340 | 0.59 | 293.3 |
| stage/render/prepare_meshes | 0.170 | 0.233 | 0.50 | 273.4 |
| stage/main/last | 0.160 | 0.222 | 0.77 | 258.4 |
| stage/render/extract_commands | 0.159 | 0.198 | 0.37 | 255.9 |
| render/acquire/ms | 0.158 | 0.733 | 1.24 | 255.3 |
| stage/render/create_views | 0.137 | 0.184 | 0.36 | 221.5 |
| stream/drive/ms | 0.121 | 0.175 | 0.35 | 195.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.105 | 0.153 | 0.52 | 169.8 |
| stream/tick/ms | 0.093 | 0.143 | 0.33 | 150.2 |
| bigspace/low_precision/ms | 0.093 | 0.168 | 0.63 | 149.2 |
| stage/render/prepare_batch_phases | 0.084 | 0.141 | 0.39 | 135.3 |
| controller/feed/ms | 0.079 | 0.102 | 0.25 | 127.8 |
| physics/step/ms | 0.079 | 0.255 | 0.55 | 126.7 |
| stage/render/queue | 0.078 | 0.127 | 0.40 | 125.5 |
| stage/render/specialize | 0.061 | 0.089 | 0.70 | 98.0 |
| anim/evaluate/ms | 0.059 | 0.099 | 0.64 | 95.5 |
| bigspace/high_precision/ms | 0.046 | 0.102 | 0.62 | 74.8 |
| stage/render/prepare_collect_phase_buffers | 0.044 | 0.067 | 0.19 | 70.3 |
| anim/evaluate_cpu/ms | 0.034 | 0.050 | 0.31 | 55.4 |
| encode/point_shadow_cache_copies/sum | 0.034 | 0.069 | 0.23 | 54.8 |
| encode/resolve_diagnostics/sum | 0.032 | 0.046 | 0.41 | 51.6 |
| controller/write/ms | 0.026 | 0.036 | 0.26 | 42.1 |
| weather/map/ms | 0.026 | 0.035 | 0.19 | 41.5 |
| stage/render/cleanup | 0.024 | 0.039 | 0.21 | 38.4 |
| stage/render/phase_sort | 0.012 | 0.033 | 0.25 | 19.8 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.021 | 0.29 | 18.6 |
| weather/eval/ms | 0.011 | 0.016 | 0.05 | 17.9 |
| encode/camera_driver_span_end/sum | 0.010 | 0.020 | 0.09 | 16.8 |
| physics/insert/ms | 0.008 | 0.000 | 0.51 | 12.9 |
| script/update/ms | 0.008 | 0.011 | 0.21 | 12.3 |
| anim/apply/ms | 0.005 | 0.010 | 0.12 | 8.1 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.7 |
| far/select/ms | 0.004 | 0.005 | 0.03 | 6.4 |
| controller/move/ms | 0.004 | 0.010 | 0.07 | 6.1 |
| script/ui/ms | 0.003 | 0.005 | 0.06 | 5.4 |
| controller/avoid/ms | 0.002 | 0.005 | 0.04 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.12 | 1.1 |
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
| encode//max | ms | 0.40 | 1.19 | 0.48 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.29 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.09 | 0.01 |
| encode/finish_critical | ms | 0.49 | 1.19 | 0.49 |
| encode/pass:early prepass/max | ms | 0.42 | 0.83 | 0.49 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.38 | 0.88 | 0.36 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.10 | 0.52 | 0.07 |
| encode/pass:shadow_point_light/max | ms | 0.07 | 0.82 | 0.56 |
| encode/point_shadow_cache_copies/max | ms | 0.05 | 0.23 | 0.02 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.41 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.81 | 11.97 | 5.11 |
| gpu/timed/ms | ms | 5.73 | 8.30 | 5.11 |
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
| phases/alpha_mask3d/items | count | 11.50 | 24 | 3 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 4 | 4 | 3 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 4 | 4 | 3 |
| phases/alpha_mask3d_prepass/bins | count | 5 | 9 | 3 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 177 | 177 | 177 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 11.50 | 24 | 3 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 4 | 4 | 3 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 9 | 12 | 6 |
| phases/opaque3d/bins | count | 148 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 254 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 6 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 148 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 254 | 507 | 40 |
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
| physics/pending | count | 0 | 98 | 0 |
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
| preprocess/bind_groups_reused | count | 71 | 131 | 35 |
| process/cpu_usage | % | 23.95 | 38.46 | 25.48 |
| process/mem_usage | GiB | 3.79 | 3.94 | 3.94 |
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
| system/cpu_usage | % | 30.44 | 39.62 | 33.64 |
| system/mem_usage | % | 55.52 | 56.05 | 56.05 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 35 | 90 | 17 |
| views/all/bins | count | 388 | 1358 | 58 |
| views/all/count | count | 7 | 28 | 1 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 622 | 1891 | 86 |
| views/all/pipelines | count | 36 | 109 | 16 |
| views/all/pipelines_distinct | count | 24 | 34 | 16 |
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
| views/main/bins | count | 307 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 530 | 1062 | 86 |
| views/main/pipelines | count | 21 | 27 | 16 |
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
| anim/bones_written/count | count | 8.68 | 30 | 14.0 k |
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
| encode//count | count | 41.84 | 91 | 67.4 k |
| encode/bind_walk/init_textures | count | 4201 | 12.6 k | 6.77 M |
| encode/bind_walk/init_textures_skipped | count | 7016 | 18.4 k | 11.31 M |
| encode/bind_walk/inits | count | 217 | 416 | 350.0 k |
| encode/bind_walk/inits_skipped | count | 10.62 | 28 | 17.1 k |
| encode/bind_walk/merge_textures | count | 2958 | 11.3 k | 4.77 M |
| encode/bind_walk/merge_textures_skipped | count | 76.16 | 7105 | 122.8 k |
| encode/bind_walk/merges | count | 66.29 | 126 | 106.9 k |
| encode/bind_walk/merges_skipped | count | 11.32 | 47 | 18.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1612 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1612 |
| encode/pass:early prepass/count | count | 1 | 1 | 1612 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1612 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1564 |
| encode/pass:shadow_point_light/count | count | 5.56 | 18 | 8960 |
| encode/point_shadow_cache_copies/count | count | 0.67 | 1 | 1080 |
| encode/queue/command_buffers | count | 57.03 | 119 | 91.9 k |
| encode/queue/submits | count | 2 | 2 | 3224 |
| encode/queue/write_buffer/bytes | bytes | 544.0 k | 661.3 k | 877.00 M |
| encode/queue/write_buffer/calls | count | 145 | 310 | 233.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 87.8 k | 88.2 k | 141.51 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 9672 |
| encode/queue/write_texture/bytes | bytes | 650 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1612 |
| physics/inserts | count | 0.06 | 15 | 99 |
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

## Hitches (> 25.0 ms; slowest 34, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2552.3 | main/busy/ms 2522, stage/main/post_update 2414, stage/main/update 98.00, surface/advance_cpu_ms 68.32, stage/main/pre_update 6.97, stage/main/last 1.14, stage/main/first 0.85, stage/main/fixed_main_loop 0.37, stream/drive/ms 0.33, weather/map/ms 0.22, bigspace/total/ms 0.07, stream/tick/ms 0.05 |
| warm-up | 3.28 | 730.3 | main/busy/ms 2641, stage/main/post_update 2419, render/thread/ms 691, render/busy/ms 578, stage/render/prepare_assets 533, stage/main/update 205, stage/render/prepare_views 111, render/acquire/ms 111, surface/advance_cpu_ms 68.32, stage/render/create_views 31.31, stage/render/render 8.71, stage/main/pre_update 8.04 |
| warm-up | 3.33 | 47.1 | render/thread/ms 723, render/busy/ms 609, stage/render/prepare_assets 559, main/busy/ms 133, stage/main/update 113, stage/render/prepare_views 112, render/acquire/ms 112, stage/render/create_views 31.53, stage/render/render 12.84, render/graph/ms 9.24, stage/main/post_update 8.34, stage/main/fixed_main_loop 8.20 |
| warm-up | 3.40 | 54.6 | pipelines/wait_ms 1270, pipelines/compile_ms 437, pipelines/front_ms 401, main/busy/ms 53.20, pipelines/driver_ms 36.24, render/thread/ms 35.82, render/busy/ms 33.20, stage/render/prepare_assets 22.74, stage/main/post_update 21.53, stage/main/pre_update 18.95, stage/render/render 8.37, stage/main/update 6.40 |
| warm-up | 3.43 | 33.7 | pipelines/wait_ms 1344, pipelines/compile_ms 598, pipelines/front_ms 558, main/busy/ms 59.11, render/thread/ms 52.68, render/busy/ms 50.79, pipelines/driver_ms 40.70, stage/render/prepare_assets 30.54, stage/main/pre_update 24.32, stage/main/update 11.92, stage/render/render 9.37, stage/main/post_update 8.96 |
| warm-up | 3.49 | 25.7 | pipelines/compile_ms 414, pipelines/front_ms 399, render/thread/ms 38.56, render/busy/ms 36.50, main/busy/ms 23.95, pipelines/wait_ms 16.65, stage/render/prepare_assets 15.14, pipelines/driver_ms 14.87, stage/render/render 9.49, stage/render/prepare_bind_groups 8.32, render/graph/ms 7.49, stage/main/first 7.43 |
| warm-up | 3.60 | 93.5 | pipelines/compile_ms 964, pipelines/front_ms 924, main/busy/ms 68.44, render/thread/ms 64.99, render/busy/ms 63.28, stage/main/update 56.88, stage/render/prepare_assets 47.50, pipelines/wait_ms 41.68, pipelines/driver_ms 39.45, stage/render/render 12.24, render/graph/ms 10.37, stage/main/post_update 7.23 |
| warm-up | 3.65 | 39.1 | pipelines/compile_ms 791, pipelines/front_ms 753, render/thread/ms 44.80, render/busy/ms 42.74, pipelines/driver_ms 38.04, stage/render/prepare_assets 29.40, main/busy/ms 22.47, stage/render/render 11.08, stage/main/update 10.66, render/graph/ms 8.87, stage/render/graph_submit 6.00, stage/main/post_update 5.77 |
| warm-up | 3.70 | 36.0 | pipelines/compile_ms 120, pipelines/front_ms 115, main/busy/ms 53.83, render/thread/ms 45.21, render/busy/ms 43.34, stage/main/update 37.85, stage/render/prepare_assets 18.89, stream/drive/ms 18.29, stage/render/render 16.50, render/graph/ms 14.96, stage/main/post_update 12.89, stage/render/camera_driver 9.46 |
| warm-up | 3.78 | 78.0 | pipelines/compile_ms 604, pipelines/front_ms 582, main/busy/ms 164, stage/main/update 141, render/thread/ms 70.55, render/busy/ms 69.00, stage/render/prepare_assets 37.82, pipelines/driver_ms 21.91, stage/render/render 18.26, stage/render/extract_commands 17.36, stream/drive/ms 16.60, render/graph/ms 16.43 |
| warm-up | 3.98 | 197.8 | pipelines/compile_ms 2309, pipelines/front_ms 2235, main/busy/ms 130, stage/main/update 112, render/thread/ms 106, render/busy/ms 104, pipelines/wait_ms 100, stage/render/prepare_assets 89.03, pipelines/driver_ms 73.77, stage/render/extract_commands 17.37, pipelines/lock_ms 11.63, stage/render/render 7.69 |
| warm-up | 4.05 | 78.5 | pipelines/compile_ms 2172, pipelines/front_ms 2086, render/thread/ms 129, render/busy/ms 128, stage/render/prepare_assets 118, pipelines/wait_ms 102, pipelines/driver_ms 86.63, main/busy/ms 19.05, pipelines/lock_ms 12.83, stage/render/render 7.21, stage/main/update 7.00, render/graph/ms 5.85 |
| warm-up | 4.10 | 42.3 | pipelines/compile_ms 642, pipelines/front_ms 601, render/thread/ms 95.83, render/busy/ms 94.35, stage/render/prepare_assets 81.72, main/busy/ms 64.60, pipelines/driver_ms 40.85, stage/main/update 39.90, stream/drive/ms 33.12, stage/main/pre_update 10.90, stage/render/render 9.84, pipelines/wait_ms 8.57 |
| warm-up | 4.14 | 40.2 | pipelines/compile_ms 274, pipelines/front_ms 264, main/busy/ms 65.29, render/thread/ms 52.24, render/busy/ms 50.75, stage/main/update 39.50, stage/render/prepare_assets 36.24, stream/drive/ms 33.06, stage/main/pre_update 10.89, pipelines/driver_ms 10.12, stage/render/render 8.51, stage/main/post_update 7.69 |
| warm-up | 4.20 | 43.5 | pipelines/compile_ms 1274, pipelines/front_ms 1227, pipelines/wait_ms 236, main/busy/ms 49.67, pipelines/driver_ms 47.08, render/thread/ms 27.98, stage/main/post_update 27.68, render/busy/ms 26.46, stage/main/update 18.69, stage/render/render 10.09, stream/drive/ms 9.95, render/graph/ms 8.37 |
| warm-up | 4.22 | 27.3 | pipelines/compile_ms 1870, pipelines/front_ms 1807, pipelines/wait_ms 258, pipelines/driver_ms 63.55, render/thread/ms 31.14, render/busy/ms 29.68, main/busy/ms 19.41, stage/render/prepare_assets 17.82, stage/main/update 8.39, stage/render/render 8.29, pipelines/lock_ms 6.69, render/graph/ms 6.64 |
| warm-up | 4.45 | 25.2 | pipelines/compile_ms 334, pipelines/front_ms 322, pipelines/wait_ms 28.86, main/busy/ms 28.25, render/thread/ms 25.88, render/busy/ms 24.45, stage/render/prepare_resources 14.12, stage/main/update 13.91, pipelines/driver_ms 12.79, stage/main/post_update 7.37, stage/render/render 5.94, render/graph/ms 4.41 |
| warm-up | 4.47 | 27.0 | pipelines/compile_ms 455, pipelines/front_ms 439, main/busy/ms 42.44, render/thread/ms 40.14, render/busy/ms 37.16, pipelines/wait_ms 28.89, stage/main/update 18.05, pipelines/driver_ms 15.84, stage/render/prepare_resources 14.33, stage/main/post_update 14.17, stage/render/render 10.24, stage/render/prepare_assets 8.69 |
| warm-up | 4.52 | 49.1 | pipelines/compile_ms 823, pipelines/front_ms 790, main/busy/ms 72.53, render/thread/ms 62.99, render/busy/ms 57.94, stage/main/update 46.37, pipelines/driver_ms 33.54, stage/render/render 18.08, stage/main/post_update 17.11, render/graph/ms 14.77, pipelines/wait_ms 13.24, stage/render/prepare_assets 11.84 |
| warm-up | 4.59 | 63.7 | pipelines/compile_ms 1404, pipelines/front_ms 1354, render/thread/ms 83.88, render/busy/ms 80.42, main/busy/ms 78.06, stage/main/update 56.61, pipelines/driver_ms 49.46, stage/render/prepare_assets 31.57, pipelines/wait_ms 29.15, stage/render/render 17.10, stage/main/post_update 15.16, render/graph/ms 15.16 |
| warm-up | 4.62 | 31.2 | pipelines/compile_ms 1135, pipelines/front_ms 1101, main/busy/ms 73.49, render/thread/ms 62.99, render/busy/ms 61.54, stage/main/update 52.35, stage/render/prepare_assets 39.53, pipelines/driver_ms 33.65, stream/drive/ms 31.03, pipelines/wait_ms 29.27, stage/main/post_update 14.08, stage/render/render 11.56 |
| warm-up | 4.68 | 58.9 | pipelines/compile_ms 730, pipelines/front_ms 698, main/busy/ms 71.84, render/thread/ms 51.30, stage/main/update 50.76, render/busy/ms 49.29, stream/drive/ms 39.78, pipelines/driver_ms 31.29, stage/render/prepare_assets 24.85, pipelines/wait_ms 14.10, stage/main/post_update 13.91, stage/render/render 12.31 |
| warm-up | 4.71 | 36.9 | pipelines/compile_ms 394, pipelines/front_ms 373, render/thread/ms 54.31, render/busy/ms 52.07, main/busy/ms 50.66, stage/main/update 27.39, stage/render/prepare_assets 21.62, pipelines/driver_ms 20.76, stage/main/post_update 15.23, stage/render/render 14.22, stream/drive/ms 12.61, render/graph/ms 11.70 |
| warm-up | 4.76 | 46.4 | pipelines/compile_ms 98.43, pipelines/front_ms 94.27, render/thread/ms 52.28, render/busy/ms 50.72, main/busy/ms 39.97, stage/render/prepare_assets 23.99, stage/main/update 19.76, stage/main/post_update 11.14, stage/render/render 10.26, render/graph/ms 8.35, encode//sum 7.86, stage/render/create_views 6.88 |
| warm-up | 4.84 | 82.6 | render/thread/ms 89.70, render/busy/ms 88.36, main/busy/ms 43.41, stage/render/prepare_assets 38.03, stage/render/create_views 34.91, stage/main/update 30.16, stream/drive/ms 20.35, stage/render/render 7.98, render/graph/ms 6.38, stage/main/post_update 6.24, encode//sum 5.87, stage/render/graph_submit 4.20 |
| warm-up | 4.88 | 37.1 | render/thread/ms 96.92, render/busy/ms 95.29, main/busy/ms 48.38, stage/render/create_views 43.28, stage/main/update 32.10, stage/render/prepare_assets 27.18, stream/drive/ms 20.52, stage/render/render 15.64, encode//sum 15.12, render/graph/ms 13.63, stage/render/graph_submit 10.81, stage/main/post_update 8.75 |
| warm-up | 4.97 | 27.0 | pipelines/compile_ms 900, pipelines/front_ms 867, render/thread/ms 41.77, render/busy/ms 40.06, pipelines/driver_ms 33.38, main/busy/ms 29.03, stage/main/update 15.76, stage/render/render 10.37, stage/render/prepare_write_phase_buffers 10.06, stage/render/prepare_assets 9.70, render/graph/ms 8.56, pipelines/wait_ms 8.02 |
| warm-up | 5.00 | 30.5 | pipelines/compile_ms 1466, pipelines/front_ms 1414, pipelines/driver_ms 51.74, render/thread/ms 49.73, render/busy/ms 47.68, pipelines/wait_ms 27.03, main/busy/ms 24.41, stage/render/render 20.68, render/graph/ms 18.42, stage/render/prepare_assets 13.33, encode//sum 11.18, stage/main/post_update 9.94 |
| warm-up | 5.02 | 26.6 | pipelines/compile_ms 675, pipelines/front_ms 652, render/thread/ms 52.79, render/busy/ms 50.63, pipelines/wait_ms 50.58, stage/render/render 24.34, pipelines/driver_ms 22.96, render/graph/ms 21.85, main/busy/ms 21.13, encode//sum 17.53, stage/render/graph_submit 10.75, stage/render/camera_driver 10.45 |
| warm-up | 5.05 | 26.8 | pipelines/compile_ms 348, pipelines/front_ms 336, render/thread/ms 42.00, render/busy/ms 40.37, pipelines/wait_ms 34.06, main/busy/ms 21.90, stage/render/render 14.45, encode//sum 13.57, render/graph/ms 12.44, pipelines/driver_ms 11.50, stage/render/prepare_assets 10.43, stage/main/post_update 9.52 |
| warm-up | 5.15 | 26.2 | pipelines/compile_ms 612, pipelines/front_ms 597, pipelines/wait_ms 42.34, render/thread/ms 35.07, main/busy/ms 34.95, render/busy/ms 33.60, stage/main/update 19.30, pipelines/driver_ms 15.10, stage/render/render 14.84, render/graph/ms 12.88, encode//sum 11.86, stage/main/post_update 10.57 |
| warm-up | 5.18 | 36.3 | pipelines/compile_ms 484, pipelines/front_ms 471, render/thread/ms 52.31, render/busy/ms 50.87, main/busy/ms 42.92, pipelines/wait_ms 37.46, stage/main/update 26.05, stage/render/prepare_assets 21.91, stage/render/render 16.65, render/graph/ms 14.64, pipelines/driver_ms 12.91, encode//sum 12.74 |
| warm-up | 5.22 | 32.7 | pipelines/compile_ms 604, pipelines/front_ms 589, pipelines/wait_ms 116, render/thread/ms 53.30, render/busy/ms 51.95, main/busy/ms 41.60, stage/main/update 25.74, stage/render/prepare_assets 25.15, pipelines/driver_ms 15.80, stage/render/render 15.72, render/graph/ms 13.86, encode//sum 11.46 |
| warm-up | 5.25 | 35.9 | pipelines/compile_ms 365, pipelines/front_ms 357, pipelines/wait_ms 109, render/thread/ms 42.57, render/busy/ms 41.38, stage/render/prepare_assets 25.07, main/busy/ms 20.22, stage/render/render 10.18, encode//sum 9.64, render/graph/ms 8.74, stage/main/update 8.65, pipelines/driver_ms 7.55 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.cache.enabled=on --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=cache-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 20%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (253 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **92%** (min 83%) · 2625 MHz (min 2625) · memory 8101 MHz · 93.0 W · 71 °C (max 76) · VRAM 4975 MiB · throttled 100% of samples

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
