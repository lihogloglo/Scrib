# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=8 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1763 frames over 11.4 s · **median 6.32 ms (158 fps)** · mean 6.48 · p95 7.81 · p99 9.33 (1% low 107 fps) · worst 13.90 ms · 16 spikes over 1.5× median · 10.5 stutters/min (work 5.2/min)
- warm-up: 120 frames over 5.9 s, worst 2587.5 ms
- hitches (> 25.0 ms): **0** measured, 30 in warm-up
- timed GPU passes: 5.38 ms of a 6.48 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 158.2, target 144
- pass worst frame 13.90 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.038 | 2.158 | 2.465 | 0.94 | 0.168 |
| main_opaque_pass_3d | 2.021 | 2.139 | 3.186 | 0.94 | 0.039 |
| bloom | 0.323 | 0.342 | 0.492 | 0.94 | 0.064 |
| ssao | 0.196 | 0.208 | 0.349 | 0.94 | 0.007 |
| upscaling | 0.159 | 0.168 | 0.219 | 0.94 | 0.005 |
| tlas_build | 0.110 | 0.117 | 0.119 | 0.94 | 0.075 |
| tonemapping | 0.102 | 0.108 | 0.122 | 0.94 | 0.004 |
| gi/probe_update | 0.095 | 0.101 | 0.100 | 0.94 | 0.002 |
| gpu_instance_materialization | 0.095 | 0.101 | 0.100 | 0.94 | 0.004 |
| early prepass | 0.077 | 0.082 | 0.091 | 0.94 | 0.050 |
| clustering | 0.072 | 0.077 | 0.083 | 0.94 | 0.020 |
| point_shadow_atlas | 0.066 | 0.100 | 0.126 | 0.65 | 0.056 |
| uniform_allocation | 0.035 | 0.037 | 0.044 | 0.94 | 0.010 |
| bin_unpacking | 0.031 | 0.033 | 0.037 | 0.94 | 0.008 |
| early_mesh_preprocessing | 0.018 | 0.019 | 0.022 | 0.94 | 0.003 |
| range_unpacking | 0.017 | 0.018 | 0.018 | 0.94 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.92 | 0.024 |
| setup_tlas_instances | 0.007 | 0.007 | 0.008 | 0.94 | 0.005 |
| point_shadow_atlas/early_mesh_preprocessing | 0.006 | 0.009 | 0.012 | 0.65 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.005 | 0.007 | 0.008 | 0.65 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.005 | 0.007 | 0.008 | 0.65 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.005 | 0.007 | 0.007 | 0.65 | 0.002 |
| sparse buffer updates | 0.004 | 0.006 | 0.007 | 0.59 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.001 | 0.94 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.94 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 6.720 | 0.000 | 5256.81 | 11846.6 |
| render/thread/ms | 5.537 | 6.797 | 11.23 | 9762.1 |
| pipelines/driver_ms | 5.296 | 0.000 | 4919.95 | 9337.1 |
| main/busy/ms | 5.118 | 5.995 | 9.50 | 9023.7 |
| render/busy/ms | 4.766 | 5.867 | 10.04 | 8402.6 |
| stage/render/render | 3.268 | 4.190 | 8.79 | 5761.9 |
| render/graph/ms | 2.721 | 3.582 | 8.10 | 4796.8 |
| stage/main/post_update | 2.628 | 3.191 | 7.34 | 4632.9 |
| encode//sum | 2.182 | 3.024 | 7.42 | 3846.8 |
| stage/render/graph_submit | 1.787 | 2.477 | 6.77 | 3151.2 |
| pipelines/wait_ms | 1.648 | 0.000 | 1014.99 | 2904.7 |
| pipelines/front_ms | 1.423 | 0.000 | 336.87 | 2509.5 |
| stage/main/update | 1.296 | 1.614 | 3.02 | 2285.3 |
| encode/submit | 1.197 | 1.729 | 2.37 | 2109.7 |
| stage/render/camera_driver | 0.791 | 1.015 | 1.66 | 1395.2 |
| encode/finish_wall | 0.629 | 0.887 | 4.42 | 1109.8 |
| encode/pass:early prepass/sum | 0.442 | 0.620 | 1.29 | 779.1 |
| stage/render/prepare_views | 0.434 | 1.408 | 2.12 | 765.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.422 | 0.616 | 1.14 | 744.6 |
| stage/render/prepare_resources | 0.405 | 0.523 | 0.77 | 714.8 |
| render/acquire/ms | 0.388 | 1.365 | 2.07 | 683.9 |
| render/present/ms | 0.383 | 0.771 | 2.97 | 675.6 |
| stage/main/pre_update | 0.361 | 0.467 | 2.23 | 636.8 |
| bigspace/total/ms | 0.252 | 0.585 | 1.39 | 444.7 |
| stage/main/fixed_main_loop | 0.252 | 0.666 | 1.98 | 444.2 |
| stage/main/first | 0.251 | 0.358 | 1.60 | 442.6 |
| stage/render/prepare_write_phase_buffers | 0.251 | 0.358 | 0.74 | 442.3 |
| stage/render/prepare_bind_groups | 0.220 | 0.302 | 0.65 | 387.3 |
| stage/render/prepare_assets | 0.217 | 0.304 | 0.55 | 383.3 |
| stage/render/create_views | 0.178 | 0.265 | 0.59 | 313.5 |
| stage/render/prepare_flush | 0.177 | 0.334 | 1.03 | 312.4 |
| stage/render/prepare_meshes | 0.170 | 0.235 | 0.52 | 298.8 |
| stage/main/last | 0.156 | 0.209 | 0.48 | 275.0 |
| stage/render/extract_commands | 0.140 | 0.174 | 0.30 | 247.4 |
| stream/drive/ms | 0.132 | 0.190 | 0.44 | 233.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.124 | 0.201 | 1.15 | 218.3 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.110 | 0.267 | 0.82 | 194.3 |
| bigspace/low_precision/ms | 0.110 | 0.296 | 0.91 | 193.5 |
| stream/tick/ms | 0.104 | 0.156 | 0.41 | 183.8 |
| physics/step/ms | 0.092 | 0.284 | 1.48 | 162.2 |
| stage/render/prepare_batch_phases | 0.086 | 0.144 | 0.37 | 152.0 |
| controller/feed/ms | 0.085 | 0.113 | 0.31 | 149.8 |
| anim/evaluate/ms | 0.075 | 0.203 | 0.83 | 131.4 |
| bigspace/high_precision/ms | 0.060 | 0.264 | 1.19 | 106.3 |
| stage/render/specialize | 0.052 | 0.083 | 0.45 | 91.7 |
| stage/render/phase_sort | 0.051 | 0.082 | 0.31 | 89.2 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.078 | 0.19 | 84.4 |
| stage/render/queue | 0.043 | 0.080 | 0.37 | 76.5 |
| anim/evaluate_cpu/ms | 0.038 | 0.062 | 0.22 | 66.8 |
| encode/resolve_diagnostics/sum | 0.038 | 0.068 | 0.52 | 66.2 |
| controller/write/ms | 0.025 | 0.034 | 0.28 | 44.6 |
| weather/map/ms | 0.025 | 0.036 | 0.14 | 44.3 |
| stage/render/cleanup | 0.022 | 0.035 | 0.21 | 38.9 |
| physics/insert/ms | 0.018 | 0.000 | 0.76 | 32.3 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.028 | 0.10 | 24.8 |
| encode/camera_driver_span_end/sum | 0.013 | 0.025 | 0.30 | 23.4 |
| weather/eval/ms | 0.012 | 0.016 | 0.34 | 20.5 |
| script/update/ms | 0.007 | 0.011 | 0.05 | 13.0 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.007 | 0.000 | 1.71 | 11.8 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 9.0 |
| anim/apply/ms | 0.005 | 0.009 | 0.03 | 8.3 |
| far/select/ms | 0.004 | 0.005 | 0.06 | 7.2 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 6.3 |
| controller/move/ms | 0.003 | 0.010 | 0.03 | 5.8 |
| controller/avoid/ms | 0.002 | 0.005 | 0.04 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.8 |
| pipelines/lock_ms | 0.001 | 0.000 | 0.33 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.03 | 1.1 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.1 |
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
| encode//max | ms | 0.42 | 1.63 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.10 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.30 | 0.01 |
| encode/finish_critical | ms | 0.49 | 1.71 | 0.41 |
| encode/pass:early prepass/max | ms | 0.43 | 1.29 | 0.41 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.41 | 1.14 | 0.40 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 1.15 | 0.12 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.14 | 0.82 | 0.12 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.07 | 1.71 | 0.88 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.52 | 0.02 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.86 | 10.04 | 5.53 |
| gpu/timed/ms | ms | 5.84 | 7.65 | 5.53 |
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
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 159 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 261 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 61 | 18 |
| phases/shadow/bins | count | 36 | 815 | 225 |
| phases/shadow/instance_batches | count | 0 | 12 | 0 |
| phases/shadow/instance_draws | count | 0 | 1062 | 0 |
| phases/shadow/instance_slots | count | 0 | 49.2 k | 0 |
| phases/shadow/items | count | 36 | 1114 | 301 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 3 | 92 | 18 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 134 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 293 | 293 | 293 |
| pipelines/waiting | count | 0 | 13 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 0 | 7 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 4 | 64 | 4 |
| preprocess/bind_groups_created | count | 0 | 60 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 10 | 4 |
| preprocess/bind_groups_reused | count | 48 | 61 | 35 |
| process/cpu_usage | % | 29.81 | 76.68 | 25.60 |
| process/mem_usage | GiB | 3.70 | 3.75 | 3.75 |
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
| shadow_atlas/alloc_failures | count | 4145 | 4145 | 4145 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 8 | 9 | 8 |
| shadow_atlas/live_occ_pct | % | 31.25 | 100 | 0 |
| shadow_atlas/live_tiles | count | 5 | 16 | 0 |
| shadow_atlas/mover_draws | count | 5 | 12 | 0 |
| shadow_atlas/movers | count | 200 | 459 | 56 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 5 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 6 | 0 |
| shadow_atlas/render/lamps | count | 8 | 9 | 8 |
| shadow_atlas/render/live_direct | count | 5 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 8 | 37 | 0 |
| shadow_atlas/resizes | count | 0 | 1 | 1 |
| shadow_atlas/static_occ_pct | % | 75 | 84.38 | 60.94 |
| shadow_atlas/static_tiles | count | 48 | 54 | 48 |
| shadow_atlas/views | count | 8 | 37 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 1 | 23 | 0 |
| skin/blocks_skipped/count | count | 1 | 2 | 0 |
| skin/blocks_walked/count | count | 1 | 2 | 0 |
| skin/joint_bytes_written/bytes | bytes | 3072 | 7680 | 0 |
| skin/joints_written/count | count | 48 | 120 | 0 |
| skin/upload_writes/count | count | 1 | 2 | 0 |
| started_load_count |  | 2750 | 2750 | 2750 |
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
| system/cpu_usage | % | 36.84 | 89.69 | 31.33 |
| system/mem_usage | % | 50.81 | 50.96 | 50.96 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 26 | 88 | 17 |
| views/all/bins | count | 334 | 1397 | 58 |
| views/all/count | count | 9 | 38 | 1 |
| views/all/instance_batches | count | 8 | 20 | 8 |
| views/all/instance_draws | count | 3220 | 4282 | 3220 |
| views/all/instance_slots | count | 98.3 k | 147.5 k | 98.3 k |
| views/all/items | count | 598 | 2176 | 86 |
| views/all/pipelines | count | 24 | 119 | 15 |
| views/all/pipelines_distinct | count | 24 | 34 | 15 |
| views/all/work_items | count | 98.9 k | 148.3 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 61 | 0 |
| views/lamp_face/bins | count | 36 | 815 | 0 |
| views/lamp_face/count | count | 8 | 37 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 12 | 0 |
| views/lamp_face/instance_draws | count | 0 | 1062 | 0 |
| views/lamp_face/instance_slots | count | 0 | 49.2 k | 0 |
| views/lamp_face/items | count | 36 | 1114 | 0 |
| views/lamp_face/pipelines | count | 3 | 92 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 49.6 k | 0 |
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
| anim/bones_written/count | count | 8.96 | 30 | 15.8 k |
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
| encode//count | count | 19.62 | 20 | 34.6 k |
| encode/bind_walk/init_textures | count | 4180 | 12.6 k | 7.37 M |
| encode/bind_walk/init_textures_skipped | count | 6967 | 18.4 k | 12.28 M |
| encode/bind_walk/inits | count | 158 | 265 | 278.5 k |
| encode/bind_walk/inits_skipped | count | 15.04 | 34 | 26.5 k |
| encode/bind_walk/merge_textures | count | 2937 | 11.4 k | 5.18 M |
| encode/bind_walk/merge_textures_skipped | count | 64.80 | 7099 | 114.2 k |
| encode/bind_walk/merges | count | 44.10 | 74 | 77.7 k |
| encode/bind_walk/merges_skipped | count | 10.49 | 39 | 18.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1763 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1763 |
| encode/pass:early prepass/count | count | 1 | 1 | 1763 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1763 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1712 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.69 | 1 | 1222 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.01 | 1 | 11 |
| encode/queue/command_buffers | count | 29.29 | 30 | 51.6 k |
| encode/queue/submits | count | 2 | 2 | 3526 |
| encode/queue/write_buffer/bytes | bytes | 533.4 k | 649.2 k | 940.41 M |
| encode/queue/write_buffer/calls | count | 99.90 | 199 | 176.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 117.3 k | 117.3 k | 206.82 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 10.6 k |
| encode/queue/write_texture/bytes | bytes | 595 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1763 |
| physics/inserts | count | 0.08 | 7 | 135 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.39 | 1 | 686 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 3 | 20 |
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

## Hitches (> 25.0 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2587.5 | main/busy/ms 2556, stage/main/post_update 2446, stage/main/update 99.73, surface/advance_cpu_ms 69.99, stage/main/pre_update 6.76, stage/main/last 1.17, stage/main/fixed_main_loop 0.70, stream/drive/ms 0.60, stage/main/first 0.46, weather/map/ms 0.24, stream/tick/ms 0.11, bigspace/total/ms 0.07 |
| warm-up | 3.13 | 545.0 | main/busy/ms 2674, stage/main/post_update 2452, render/thread/ms 503, render/busy/ms 391, stage/render/prepare_assets 361, stage/main/update 204, stage/render/prepare_views 109, render/acquire/ms 109, surface/advance_cpu_ms 69.99, stage/render/create_views 17.02, stage/render/render 9.86, stage/main/pre_update 8.36 |
| warm-up | 3.19 | 56.0 | render/thread/ms 541, render/busy/ms 428, stage/render/prepare_assets 392, main/busy/ms 131, stage/render/prepare_views 110, render/acquire/ms 110, stage/main/update 109, stage/render/create_views 17.30, stage/render/render 14.43, render/graph/ms 9.92, stage/main/post_update 9.52, stage/main/fixed_main_loop 7.78 |
| warm-up | 3.22 | 29.2 | pipelines/wait_ms 169, pipelines/compile_ms 160, pipelines/front_ms 145, render/thread/ms 62.50, render/busy/ms 54.69, stage/render/prepare_assets 32.40, stage/render/render 25.16, main/busy/ms 21.94, render/graph/ms 17.67, pipelines/driver_ms 14.87, stage/render/graph_submit 8.92, stage/render/camera_driver 8.60 |
| warm-up | 3.24 | 25.9 | pipelines/wait_ms 1114, pipelines/compile_ms 396, pipelines/front_ms 358, render/thread/ms 44.51, pipelines/driver_ms 38.60, render/busy/ms 37.36, main/busy/ms 23.23, stage/render/render 23.23, render/graph/ms 16.06, stage/main/update 9.13, stage/render/prepare_resources 8.41, stage/render/camera_driver 8.10 |
| warm-up | 3.29 | 32.6 | pipelines/compile_ms 215, pipelines/front_ms 205, pipelines/wait_ms 167, main/busy/ms 63.63, stage/main/post_update 42.40, render/thread/ms 41.38, render/busy/ms 39.39, stage/render/prepare_assets 10.63, pipelines/driver_ms 10.59, stage/render/prepare_flush 10.48, stage/render/render 8.91, stage/main/update 8.53 |
| warm-up | 3.33 | 35.8 | pipelines/compile_ms 359, pipelines/front_ms 343, pipelines/wait_ms 79.34, main/busy/ms 72.59, stage/main/post_update 53.55, render/thread/ms 37.41, render/busy/ms 35.62, pipelines/driver_ms 16.26, stage/render/prepare_flush 11.51, stage/render/prepare_assets 8.54, stage/main/update 6.80, stage/render/render 6.05 |
| warm-up | 3.37 | 41.2 | pipelines/compile_ms 910, pipelines/front_ms 872, main/busy/ms 82.75, stage/main/update 58.05, pipelines/wait_ms 49.66, pipelines/driver_ms 37.37, render/thread/ms 30.14, render/busy/ms 28.19, stage/main/post_update 20.21, stage/render/prepare_assets 12.44, stage/render/render 9.84, render/graph/ms 7.55 |
| warm-up | 3.47 | 96.0 | pipelines/compile_ms 1079, pipelines/front_ms 1036, pipelines/wait_ms 80.92, main/busy/ms 76.23, render/thread/ms 74.80, render/busy/ms 72.82, stage/main/update 62.81, stage/render/prepare_assets 57.38, pipelines/driver_ms 42.48, stage/render/render 12.06, pipelines/lock_ms 10.07, render/graph/ms 9.99 |
| warm-up | 3.52 | 42.2 | pipelines/compile_ms 692, pipelines/front_ms 663, render/thread/ms 40.17, render/busy/ms 37.91, main/busy/ms 34.23, pipelines/driver_ms 28.05, stage/render/prepare_assets 25.24, stage/main/pre_update 10.55, stage/main/post_update 9.38, stage/render/render 9.09, stage/main/update 7.77, render/graph/ms 6.97 |
| warm-up | 3.59 | 47.7 | pipelines/compile_ms 495, pipelines/front_ms 471, render/thread/ms 39.56, render/busy/ms 37.59, main/busy/ms 37.42, pipelines/driver_ms 23.44, stage/render/prepare_assets 22.98, stage/main/update 21.98, stream/drive/ms 17.13, stage/render/render 11.12, stage/main/post_update 9.25, render/graph/ms 7.92 |
| warm-up | 3.62 | 32.9 | pipelines/compile_ms 1092, pipelines/front_ms 1049, render/thread/ms 51.60, render/busy/ms 49.83, pipelines/driver_ms 43.33, stage/render/prepare_assets 32.55, main/busy/ms 27.13, stage/main/post_update 11.01, stage/render/render 10.39, stage/main/update 9.49, render/graph/ms 8.36, pipelines/wait_ms 4.96 |
| warm-up | 3.70 | 63.6 | pipelines/compile_ms 699, pipelines/front_ms 676, main/busy/ms 219, stage/main/update 179, render/thread/ms 49.70, render/busy/ms 47.27, stream/drive/ms 35.68, stage/render/prepare_assets 32.43, pipelines/driver_ms 22.94, stage/main/post_update 18.61, stage/main/pre_update 12.42, stage/render/render 11.46 |
| warm-up | 3.99 | 286.6 | pipelines/compile_ms 2748, pipelines/front_ms 2657, pipelines/wait_ms 196, main/busy/ms 189, stage/main/update 178, render/thread/ms 115, render/busy/ms 114, stage/render/prepare_assets 101, pipelines/driver_ms 91.18, stream/drive/ms 35.48, pipelines/lock_ms 11.82, stage/render/render 8.47 |
| warm-up | 4.03 | 47.3 | pipelines/compile_ms 2317, pipelines/front_ms 2234, pipelines/wait_ms 197, render/thread/ms 108, render/busy/ms 106, stage/render/prepare_assets 87.65, pipelines/driver_ms 83.02, main/busy/ms 43.72, stage/main/update 23.39, pipelines/lock_ms 13.15, stage/render/render 11.74, stage/main/post_update 11.63 |
| warm-up | 4.09 | 55.2 | pipelines/compile_ms 187, pipelines/front_ms 176, render/thread/ms 62.18, render/busy/ms 60.02, main/busy/ms 50.80, stage/render/prepare_assets 40.87, stage/main/update 24.58, pipelines/wait_ms 17.81, pipelines/lock_ms 17.33, stage/main/post_update 11.65, stage/render/render 11.44, stream/drive/ms 11.31 |
| warm-up | 4.30 | 37.6 | pipelines/compile_ms 177, pipelines/front_ms 137, render/thread/ms 49.37, render/busy/ms 48.19, stage/render/prepare_assets 39.93, pipelines/driver_ms 39.92, main/busy/ms 20.05, stage/main/update 11.86, stage/render/render 5.74, stage/main/post_update 4.49, render/graph/ms 4.35, stage/render/graph_submit 2.54 |
| warm-up | 4.43 | 49.9 | pipelines/compile_ms 196, pipelines/front_ms 189, pipelines/wait_ms 90.25, main/busy/ms 66.91, render/thread/ms 57.08, render/busy/ms 54.86, stage/main/update 38.36, stage/main/post_update 17.32, stage/render/render 12.64, render/graph/ms 10.05, stage/render/prepare_assets 9.39, stage/render/prepare_flush 9.22 |
| warm-up | 4.47 | 41.9 | render/thread/ms 67.67, render/busy/ms 65.88, main/busy/ms 51.40, pipelines/wait_ms 42.81, stage/main/update 35.76, stage/render/prepare_assets 21.11, pipelines/compile_ms 18.68, pipelines/front_ms 18.17, stage/render/render 13.67, render/graph/ms 11.42, stage/main/post_update 10.65, stage/render/prepare_flush 8.70 |
| warm-up | 4.58 | 86.9 | main/busy/ms 80.47, stage/main/update 58.21, render/thread/ms 57.95, render/busy/ms 55.90, stream/drive/ms 37.02, stage/render/prepare_assets 24.62, stage/main/post_update 16.72, stage/render/render 14.64, render/graph/ms 11.95, stage/render/graph_submit 7.52, encode//sum 6.67, encode/submit 4.47 |
| warm-up | 4.63 | 49.8 | render/thread/ms 79.50, render/busy/ms 77.37, main/busy/ms 76.48, stage/main/update 43.96, stage/render/prepare_assets 38.30, stage/main/post_update 20.71, stage/render/render 17.97, render/graph/ms 14.90, stream/drive/ms 13.26, stage/render/graph_submit 9.68, encode//sum 8.74, stage/render/prepare_write_phase_buffers 6.54 |
| warm-up | 4.71 | 80.5 | render/thread/ms 79.91, render/busy/ms 78.30, main/busy/ms 65.41, stage/render/prepare_assets 50.89, stage/main/update 37.28, stage/main/post_update 15.51, stream/drive/ms 13.90, stage/render/render 13.79, render/graph/ms 11.48, stage/render/graph_submit 7.51, encode//sum 7.29, stage/render/prepare_write_phase_buffers 5.69 |
| warm-up | 4.78 | 69.2 | render/thread/ms 97.22, render/busy/ms 95.74, stage/render/prepare_assets 73.51, main/busy/ms 56.38, stage/main/update 38.62, stream/drive/ms 23.58, stage/render/render 14.70, render/graph/ms 12.84, stage/main/post_update 10.53, stage/render/graph_submit 10.34, encode/submit 7.16, encode//sum 5.56 |
| warm-up | 4.85 | 25.5 | render/thread/ms 34.39, render/busy/ms 33.13, main/busy/ms 32.13, stage/main/update 20.25, stage/render/render 14.52, render/graph/ms 12.87, stage/render/prepare_assets 11.39, stage/render/graph_submit 9.64, stage/main/post_update 7.95, encode//sum 7.00, encode/finish_wall 5.31, encode/pass:point_shadow_atlas_static_capture/sum 4.94 |
| warm-up | 4.88 | 27.8 | render/thread/ms 42.91, render/busy/ms 41.21, main/busy/ms 35.65, stage/main/update 20.86, stage/render/prepare_assets 18.48, stage/render/render 12.21, render/graph/ms 10.19, stage/main/post_update 10.06, encode//sum 7.44, stage/render/graph_submit 6.88, stream/drive/ms 4.36, encode/finish_wall 3.67 |
| warm-up | 5.01 | 28.9 | main/busy/ms 40.41, render/thread/ms 35.58, render/busy/ms 34.22, stage/main/update 24.66, stage/render/render 12.89, render/graph/ms 11.10, stage/main/post_update 9.68, stage/render/prepare_assets 8.10, stage/render/graph_submit 7.89, encode//sum 6.00, encode/submit 4.01, encode/finish_wall 3.96 |
| warm-up | 5.05 | 41.3 | render/thread/ms 57.95, render/busy/ms 56.19, main/busy/ms 48.74, stage/main/update 29.64, stage/render/prepare_assets 23.36, stage/render/render 16.92, render/graph/ms 14.54, stage/main/post_update 12.01, stage/render/graph_submit 10.28, encode//sum 8.20, encode/submit 5.23, encode/finish_wall 5.10 |
| warm-up | 5.08 | 29.0 | render/thread/ms 53.76, render/busy/ms 51.89, main/busy/ms 39.18, stage/render/prepare_assets 24.78, stage/main/update 21.61, stage/render/render 17.37, render/graph/ms 14.94, stage/render/graph_submit 11.26, stage/main/post_update 10.66, encode//sum 8.38, encode/submit 6.01, encode/finish_wall 5.32 |
| warm-up | 5.12 | 42.2 | render/thread/ms 44.45, render/busy/ms 43.12, main/busy/ms 23.98, stage/render/prepare_assets 22.86, stage/render/render 11.69, render/graph/ms 9.96, stage/main/update 9.91, stage/render/graph_submit 7.51, stage/main/post_update 7.42, encode//sum 5.96, encode/submit 4.06, encode/finish_wall 3.50 |
| warm-up | 5.32 | 30.8 | pipelines/wait_ms 2338, pipelines/compile_ms 235, pipelines/front_ms 226, render/thread/ms 40.02, stage/render/render 32.09, render/busy/ms 20.45, main/busy/ms 19.27, render/present/ms 19.24, render/graph/ms 12.25, pipelines/driver_ms 9.62, stage/render/graph_submit 9.06, stage/main/post_update 8.97 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=8 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=atlas8-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **95%** (min 50%) · 2624 MHz (min 2565) · memory 8101 MHz · 99.1 W · 75 °C (max 78) · VRAM 5161 MiB · throttled 98% of samples

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
- `shadow.lamp.atlas.enabled=on (command line: --set)`
- `shadow.lamp.atlas.max_lamps=8 (command line: --set)`
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
