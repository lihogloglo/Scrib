# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=48 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1746 frames over 11.4 s · **median 6.52 ms (153 fps)** · mean 6.55 · p95 7.73 · p99 8.23 (1% low 121 fps) · worst 25.67 ms · 3 spikes over 1.5× median · 10.5 stutters/min (work 5.2/min)
- warm-up: 120 frames over 6.2 s, worst 2597.7 ms
- hitches (> 25.0 ms): **1** measured, 36 in warm-up
- timed GPU passes: 5.28 ms of a 6.55 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 153.3, target 144
- pass worst frame 25.67 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.012 | 2.196 | 3.252 | 0.92 | 0.040 |
| dlss_super_resolution | 1.983 | 2.163 | 2.508 | 0.92 | 0.168 |
| bloom | 0.315 | 0.344 | 0.509 | 0.92 | 0.063 |
| ssao | 0.196 | 0.213 | 0.538 | 0.92 | 0.007 |
| upscaling | 0.153 | 0.167 | 0.210 | 0.92 | 0.004 |
| tlas_build | 0.111 | 0.121 | 0.119 | 0.92 | 0.075 |
| tonemapping | 0.098 | 0.107 | 0.123 | 0.92 | 0.004 |
| gi/probe_update | 0.089 | 0.097 | 0.099 | 0.92 | 0.002 |
| gpu_instance_materialization | 0.081 | 0.089 | 0.089 | 0.92 | 0.004 |
| early prepass | 0.075 | 0.082 | 0.089 | 0.92 | 0.053 |
| clustering | 0.072 | 0.079 | 0.084 | 0.92 | 0.019 |
| point_shadow_atlas | 0.063 | 0.081 | 0.102 | 0.77 | 0.030 |
| uniform_allocation | 0.036 | 0.039 | 0.044 | 0.92 | 0.010 |
| bin_unpacking | 0.030 | 0.033 | 0.037 | 0.92 | 0.008 |
| range_unpacking | 0.017 | 0.019 | 0.018 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.017 | 0.018 | 0.023 | 0.92 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.89 | 0.024 |
| setup_tlas_instances | 0.007 | 0.007 | 0.008 | 0.92 | 0.005 |
| point_shadow_atlas/uniform_allocation | 0.006 | 0.007 | 0.008 | 0.77 | 0.002 |
| point_shadow_atlas/early_mesh_preprocessing | 0.006 | 0.007 | 0.007 | 0.77 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.005 | 0.007 | 0.008 | 0.77 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.005 | 0.007 | 0.008 | 0.77 | 0.002 |
| sparse buffer updates | 0.003 | 0.007 | 0.007 | 0.45 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.001 | 0.92 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.92 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.543 | 6.715 | 24.46 | 9677.9 |
| main/busy/ms | 5.406 | 6.244 | 8.96 | 9439.2 |
| render/busy/ms | 4.781 | 5.766 | 10.18 | 8347.5 |
| stage/render/render | 3.288 | 4.052 | 21.69 | 5740.1 |
| stage/main/post_update | 2.956 | 3.491 | 5.93 | 5161.1 |
| render/graph/ms | 2.722 | 3.461 | 4.61 | 4752.7 |
| encode//sum | 2.283 | 3.253 | 5.39 | 3986.8 |
| stage/render/graph_submit | 1.810 | 2.446 | 3.74 | 3161.1 |
| pipelines/compile_ms | 1.353 | 0.000 | 1664.39 | 2361.7 |
| stage/main/update | 1.288 | 1.616 | 2.12 | 2249.5 |
| pipelines/driver_ms | 1.233 | 0.000 | 1566.38 | 2153.2 |
| encode/submit | 1.225 | 1.715 | 2.33 | 2139.6 |
| stage/render/camera_driver | 0.772 | 0.937 | 1.26 | 1348.7 |
| encode/finish_wall | 0.636 | 0.894 | 1.91 | 1111.0 |
| pipelines/wait_ms | 0.570 | 0.000 | 950.10 | 995.2 |
| encode/pass:early prepass/sum | 0.443 | 0.628 | 1.01 | 773.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.422 | 0.625 | 1.08 | 737.0 |
| stage/render/prepare_views | 0.406 | 1.203 | 2.05 | 709.2 |
| stage/render/prepare_resources | 0.403 | 0.498 | 0.86 | 703.9 |
| render/present/ms | 0.401 | 0.824 | 18.33 | 700.4 |
| render/acquire/ms | 0.361 | 1.157 | 2.00 | 630.0 |
| stage/main/pre_update | 0.351 | 0.444 | 0.73 | 613.1 |
| stage/main/first | 0.264 | 0.385 | 1.77 | 460.8 |
| stage/render/prepare_write_phase_buffers | 0.247 | 0.350 | 0.70 | 431.2 |
| bigspace/total/ms | 0.234 | 0.557 | 1.18 | 408.1 |
| stage/main/fixed_main_loop | 0.225 | 0.594 | 2.12 | 393.4 |
| stage/render/prepare_bind_groups | 0.216 | 0.287 | 4.70 | 376.4 |
| stage/render/prepare_assets | 0.215 | 0.283 | 0.58 | 375.1 |
| stage/render/create_views | 0.195 | 0.284 | 0.52 | 340.2 |
| stage/render/prepare_meshes | 0.187 | 0.264 | 0.51 | 326.8 |
| stage/render/prepare_flush | 0.175 | 0.333 | 0.53 | 304.8 |
| stage/main/last | 0.153 | 0.201 | 0.49 | 267.4 |
| stage/render/extract_commands | 0.140 | 0.170 | 0.32 | 244.8 |
| stream/drive/ms | 0.140 | 0.198 | 0.48 | 243.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.127 | 0.211 | 0.69 | 220.9 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.126 | 0.238 | 0.74 | 220.2 |
| pipelines/front_ms | 0.119 | 0.000 | 110.46 | 208.5 |
| stream/tick/ms | 0.112 | 0.164 | 0.44 | 194.8 |
| bigspace/low_precision/ms | 0.101 | 0.280 | 0.84 | 176.1 |
| stage/render/prepare_batch_phases | 0.086 | 0.143 | 0.34 | 150.4 |
| controller/feed/ms | 0.082 | 0.106 | 0.26 | 143.5 |
| physics/step/ms | 0.071 | 0.252 | 1.85 | 124.0 |
| anim/evaluate/ms | 0.069 | 0.151 | 0.62 | 119.7 |
| stage/render/specialize | 0.054 | 0.084 | 0.30 | 94.4 |
| bigspace/high_precision/ms | 0.054 | 0.182 | 1.00 | 93.8 |
| stage/render/phase_sort | 0.051 | 0.083 | 0.27 | 88.4 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.31 | 80.0 |
| stage/render/queue | 0.042 | 0.075 | 0.35 | 73.1 |
| encode/resolve_diagnostics/sum | 0.039 | 0.070 | 0.38 | 68.3 |
| anim/evaluate_cpu/ms | 0.037 | 0.058 | 0.26 | 64.7 |
| weather/map/ms | 0.026 | 0.036 | 0.27 | 45.0 |
| controller/write/ms | 0.024 | 0.032 | 0.08 | 42.1 |
| stage/render/cleanup | 0.021 | 0.032 | 0.32 | 37.4 |
| physics/insert/ms | 0.015 | 0.000 | 0.67 | 27.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.029 | 0.22 | 25.7 |
| encode/camera_driver_span_end/sum | 0.014 | 0.029 | 0.36 | 24.2 |
| weather/eval/ms | 0.011 | 0.016 | 0.03 | 19.7 |
| script/update/ms | 0.007 | 0.011 | 0.16 | 13.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.8 |
| anim/apply/ms | 0.005 | 0.010 | 0.03 | 8.8 |
| far/select/ms | 0.004 | 0.005 | 0.15 | 7.4 |
| script/ui/ms | 0.004 | 0.005 | 0.07 | 6.1 |
| controller/move/ms | 0.003 | 0.009 | 0.03 | 5.5 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.002 | 0.000 | 1.75 | 3.2 |
| controller/avoid/ms | 0.002 | 0.005 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 2.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 1.1 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.1 |
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
| crowd/joint_blocks/count | count | 3 | 3 | 1 |
| crowd/joints_extracted/count | count | 48 | 153 | 0 |
| crowd/joints_uploaded/count | count | 153 | 153 | 59 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.44 | 1.50 | 0.45 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.22 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.36 | 0.01 |
| encode/finish_critical | ms | 0.50 | 1.75 | 0.45 |
| encode/pass:early prepass/max | ms | 0.43 | 1.01 | 0.36 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.41 | 1.08 | 0.45 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 0.69 | 0.07 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.14 | 0.74 | 0.09 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 0.50 | 1.75 | 1.75 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.38 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.83 | 10.18 | 5.47 |
| gpu/timed/ms | ms | 5.74 | 7.32 | 5.47 |
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
| phases/opaque3d/bins | count | 162 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 265 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 7 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 162 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 265 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 6 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 69 | 0 |
| phases/shadow/bins | count | 0 | 2939 | 0 |
| phases/shadow/instance_batches | count | 0 | 12 | 0 |
| phases/shadow/instance_draws | count | 0 | 1062 | 0 |
| phases/shadow/instance_slots | count | 0 | 49.2 k | 0 |
| phases/shadow/items | count | 0 | 3988 | 0 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 0 | 318 | 0 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 131 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 296 | 296 | 296 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 4 | 72 | 4 |
| preprocess/bind_groups_created | count | 0 | 62 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 10 | 4 |
| preprocess/bind_groups_reused | count | 47 | 59 | 35 |
| process/cpu_usage | % | 30.66 | 41.35 | 29.64 |
| process/mem_usage | GiB | 3.73 | 3.77 | 3.77 |
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
| shadow_atlas/alloc_failures | count | 3499 | 3544 | 3544 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 24 | 24 | 24 |
| shadow_atlas/live_occ_pct | % | 16.80 | 88.28 | 6.25 |
| shadow_atlas/live_tiles | count | 8 | 34 | 4 |
| shadow_atlas/mover_draws | count | 6 | 12 | 3 |
| shadow_atlas/movers | count | 397 | 400 | 397 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 6 | 12 | 1 |
| shadow_atlas/render/clears | count | 0 | 6 | 0 |
| shadow_atlas/render/lamps | count | 24 | 24 | 24 |
| shadow_atlas/render/live_direct | count | 6 | 12 | 1 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 10 | 109 | 9 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 94.92 | 94.92 | 94.92 |
| shadow_atlas/static_tiles | count | 144 | 144 | 144 |
| shadow_atlas/views | count | 10 | 109 | 9 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 1 | 3 | 0 |
| skin/blocks_skipped/count | count | 1 | 3 | 1 |
| skin/blocks_walked/count | count | 1 | 3 | 0 |
| skin/joint_bytes_written/bytes | bytes | 3072 | 9920 | 0 |
| skin/joints_written/count | count | 48 | 155 | 0 |
| skin/upload_writes/count | count | 1 | 1 | 0 |
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
| system/cpu_usage | % | 38.23 | 48.15 | 31.94 |
| system/mem_usage | % | 50.93 | 51.11 | 50.94 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 23 | 95 | 17 |
| views/all/bins | count | 334 | 3521 | 58 |
| views/all/count | count | 11 | 110 | 10 |
| views/all/instance_batches | count | 8 | 20 | 8 |
| views/all/instance_draws | count | 3220 | 4282 | 3220 |
| views/all/instance_slots | count | 98.3 k | 147.5 k | 98.3 k |
| views/all/items | count | 552 | 5050 | 86 |
| views/all/pipelines | count | 21 | 345 | 15 |
| views/all/pipelines_distinct | count | 21 | 34 | 15 |
| views/all/work_items | count | 98.9 k | 148.9 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 69 | 0 |
| views/lamp_face/bins | count | 0 | 2939 | 0 |
| views/lamp_face/count | count | 10 | 109 | 9 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 12 | 0 |
| views/lamp_face/instance_draws | count | 0 | 1062 | 0 |
| views/lamp_face/instance_slots | count | 0 | 49.2 k | 0 |
| views/lamp_face/items | count | 0 | 3988 | 0 |
| views/lamp_face/pipelines | count | 0 | 318 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 49.6 k | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 334 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 552 | 1062 | 86 |
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
| anim/bones_written/count | count | 9.15 | 30 | 16.0 k |
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
| encode//count | count | 19.49 | 20 | 34.0 k |
| encode/bind_walk/init_textures | count | 4147 | 12.6 k | 7.24 M |
| encode/bind_walk/init_textures_skipped | count | 6993 | 26.8 k | 12.21 M |
| encode/bind_walk/inits | count | 156 | 289 | 273.1 k |
| encode/bind_walk/inits_skipped | count | 16.56 | 41 | 28.9 k |
| encode/bind_walk/merge_textures | count | 2904 | 11.4 k | 5.07 M |
| encode/bind_walk/merge_textures_skipped | count | 50.82 | 15.6 k | 88.7 k |
| encode/bind_walk/merges | count | 42.89 | 71 | 74.9 k |
| encode/bind_walk/merges_skipped | count | 11.65 | 50 | 20.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1746 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1746 |
| encode/pass:early prepass/count | count | 1 | 1 | 1746 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1746 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1697 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.85 | 1 | 1488 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.00 | 1 | 4 |
| encode/queue/command_buffers | count | 29.31 | 30 | 51.2 k |
| encode/queue/submits | count | 2 | 2 | 3492 |
| encode/queue/write_buffer/bytes | bytes | 580.3 k | 725.1 k | 1.01 G |
| encode/queue/write_buffer/calls | count | 99.29 | 207 | 173.4 k |
| encode/queue/write_buffer_with/bytes | bytes | 283.1 k | 283.1 k | 494.24 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 10.5 k |
| encode/queue/write_texture/bytes | bytes | 601 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1746 |
| physics/inserts | count | 0.08 | 12 | 138 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.39 | 1 | 686 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0.00 | 4 | 6 |
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

## Hitches (> 25.0 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2597.7 | main/busy/ms 2565, stage/main/post_update 2458, stage/main/update 96.71, surface/advance_cpu_ms 67.79, stage/main/pre_update 6.98, stage/main/last 1.13, stream/drive/ms 0.55, stage/main/first 0.55, stage/main/fixed_main_loop 0.37, stream/tick/ms 0.15, weather/map/ms 0.11, bigspace/total/ms 0.09 |
| warm-up | 3.34 | 741.9 | main/busy/ms 2688, stage/main/post_update 2464, render/thread/ms 698, render/busy/ms 592, stage/render/prepare_assets 548, stage/main/update 207, stage/render/prepare_views 104, render/acquire/ms 104, surface/advance_cpu_ms 67.79, stage/render/create_views 31.50, stage/render/render 9.30, stage/main/pre_update 8.67 |
| warm-up | 3.39 | 49.9 | render/thread/ms 732, render/busy/ms 625, stage/render/prepare_assets 576, main/busy/ms 134, stage/main/update 114, stage/render/prepare_views 104, render/acquire/ms 104, stage/render/create_views 31.73, stage/render/render 13.18, render/graph/ms 9.22, stage/main/post_update 8.93, stage/main/fixed_main_loop 7.48 |
| warm-up | 3.43 | 37.0 | pipelines/wait_ms 342, pipelines/compile_ms 237, pipelines/front_ms 216, render/thread/ms 69.65, render/busy/ms 61.32, main/busy/ms 57.95, stage/main/update 38.85, stage/render/render 37.66, render/graph/ms 29.27, stage/render/prepare_assets 28.10, pipelines/driver_ms 20.47, stage/render/graph_submit 14.24 |
| warm-up | 3.47 | 38.6 | pipelines/wait_ms 1631, pipelines/compile_ms 547, pipelines/front_ms 508, main/busy/ms 74.43, render/thread/ms 62.31, render/busy/ms 53.94, stage/render/render 40.47, pipelines/driver_ms 38.54, stage/main/update 37.01, render/graph/ms 32.14, stage/main/post_update 29.58, stage/render/graph_submit 17.48 |
| warm-up | 3.49 | 27.7 | pipelines/wait_ms 1290, pipelines/compile_ms 372, pipelines/front_ms 351, main/busy/ms 59.76, render/thread/ms 49.99, render/busy/ms 44.33, stage/main/post_update 43.41, pipelines/driver_ms 21.08, stage/render/render 15.38, stage/render/prepare_meshes 11.74, stage/render/prepare_assets 9.80, render/graph/ms 9.60 |
| warm-up | 3.56 | 26.7 | pipelines/compile_ms 346, pipelines/front_ms 336, render/thread/ms 37.30, render/busy/ms 34.65, main/busy/ms 23.85, pipelines/wait_ms 19.05, stage/render/render 16.99, render/graph/ms 14.09, pipelines/driver_ms 10.61, stage/main/update 10.03, stage/render/prepare_assets 8.82, stage/main/post_update 7.03 |
| warm-up | 3.61 | 49.5 | pipelines/compile_ms 616, pipelines/front_ms 598, main/busy/ms 76.08, render/thread/ms 69.89, render/busy/ms 67.13, pipelines/wait_ms 60.50, stage/main/update 58.04, stage/render/prepare_resources 24.52, pipelines/driver_ms 18.86, stage/render/render 17.37, stage/render/prepare_assets 15.43, render/graph/ms 14.38 |
| warm-up | 3.69 | 76.8 | pipelines/compile_ms 1319, pipelines/front_ms 1281, render/thread/ms 102, render/busy/ms 101, main/busy/ms 75.25, stage/render/prepare_assets 59.77, stage/main/update 58.80, pipelines/wait_ms 45.10, pipelines/driver_ms 38.81, stage/render/prepare_resources 21.84, stage/main/post_update 8.06, stage/render/render 7.90 |
| warm-up | 3.73 | 28.0 | pipelines/compile_ms 397, pipelines/front_ms 379, render/thread/ms 35.32, render/busy/ms 33.50, stage/render/prepare_assets 21.83, main/busy/ms 18.75, pipelines/driver_ms 17.52, stage/render/render 9.13, stage/main/update 7.95, render/graph/ms 7.25, encode//sum 6.73, stage/main/post_update 6.44 |
| warm-up | 3.76 | 26.8 | pipelines/compile_ms 191, pipelines/front_ms 184, main/busy/ms 47.01, stage/main/update 30.92, render/thread/ms 25.40, render/busy/ms 18.69, stream/drive/ms 18.05, stage/render/render 16.31, stage/main/post_update 10.38, render/graph/ms 9.46, render/present/ms 6.49, pipelines/driver_ms 6.26 |
| warm-up | 3.84 | 84.1 | pipelines/compile_ms 756, pipelines/front_ms 716, main/busy/ms 190, stage/main/update 160, render/thread/ms 74.32, render/busy/ms 57.40, stage/render/render 42.29, pipelines/driver_ms 40.14, render/graph/ms 25.29, stage/render/prepare_assets 25.27, stream/drive/ms 17.63, render/present/ms 16.49 |
| warm-up | 4.06 | 215.1 | pipelines/compile_ms 2157, pipelines/front_ms 2073, main/busy/ms 161, stage/main/update 137, render/thread/ms 127, render/busy/ms 116, stage/render/prepare_assets 89.44, pipelines/driver_ms 84.15, pipelines/wait_ms 39.80, stage/render/render 32.40, pipelines/lock_ms 31.31, render/graph/ms 20.78 |
| warm-up | 4.12 | 56.8 | pipelines/compile_ms 1557, pipelines/front_ms 1505, render/thread/ms 113, render/busy/ms 111, stage/render/prepare_assets 100, pipelines/driver_ms 52.30, pipelines/wait_ms 39.43, pipelines/lock_ms 31.08, main/busy/ms 20.58, stage/render/render 9.12, render/graph/ms 7.34, stage/main/update 6.72 |
| warm-up | 4.19 | 69.5 | pipelines/compile_ms 1029, pipelines/front_ms 966, render/thread/ms 102, render/busy/ms 96.90, main/busy/ms 78.56, stage/render/prepare_assets 72.69, pipelines/driver_ms 63.36, pipelines/wait_ms 59.17, stage/main/update 48.47, stream/drive/ms 36.90, stage/render/render 24.51, render/graph/ms 19.06 |
| warm-up | 4.26 | 55.3 | pipelines/compile_ms 681, pipelines/front_ms 629, render/thread/ms 66.75, render/busy/ms 65.43, pipelines/wait_ms 63.33, pipelines/driver_ms 51.78, stage/render/prepare_assets 48.85, main/busy/ms 43.69, stage/main/update 24.55, stage/main/post_update 11.53, stream/drive/ms 9.94, stage/render/prepare_resources 7.81 |
| warm-up | 4.31 | 43.0 | pipelines/compile_ms 1612, pipelines/front_ms 1532, pipelines/wait_ms 306, pipelines/driver_ms 80.30, render/thread/ms 76.10, render/busy/ms 74.69, stage/render/prepare_assets 60.74, main/busy/ms 48.82, stage/main/update 28.78, stage/main/post_update 12.35, stream/drive/ms 9.91, stage/render/render 6.92 |
| warm-up | 4.42 | 29.1 | pipelines/compile_ms 341, pipelines/front_ms 331, pipelines/wait_ms 52.33, main/busy/ms 43.07, render/thread/ms 30.48, render/busy/ms 29.36, stage/main/post_update 28.83, stage/render/render 14.25, render/graph/ms 12.79, pipelines/driver_ms 9.92, stage/render/camera_driver 9.64, stage/main/pre_update 7.75 |
| warm-up | 4.45 | 31.9 | pipelines/compile_ms 749, pipelines/front_ms 728, pipelines/wait_ms 74.98, main/busy/ms 53.59, render/thread/ms 39.24, stage/main/post_update 38.08, render/busy/ms 37.46, stage/render/render 22.96, pipelines/driver_ms 21.09, render/graph/ms 20.92, stage/render/camera_driver 9.70, stage/main/pre_update 8.12 |
| warm-up | 4.54 | 25.8 | pipelines/compile_ms 442, pipelines/front_ms 425, pipelines/wait_ms 52.22, render/thread/ms 35.54, render/busy/ms 34.34, main/busy/ms 30.55, stage/main/post_update 17.42, pipelines/driver_ms 17.12, stage/render/prepare_assets 9.71, stage/render/render 9.66, stage/render/prepare_resources 8.98, render/graph/ms 8.23 |
| warm-up | 4.59 | 29.5 | pipelines/compile_ms 692, pipelines/front_ms 664, render/thread/ms 36.43, main/busy/ms 28.61, pipelines/driver_ms 27.14, render/busy/ms 24.80, stage/render/render 21.12, stage/main/post_update 17.45, pipelines/wait_ms 11.76, render/present/ms 11.39, render/graph/ms 9.26, stage/render/prepare_assets 7.94 |
| warm-up | 4.66 | 49.4 | pipelines/compile_ms 262, pipelines/front_ms 253, main/busy/ms 55.49, render/thread/ms 35.08, render/busy/ms 33.48, stage/main/post_update 30.12, pipelines/wait_ms 18.16, stage/main/update 14.70, pipelines/driver_ms 8.97, stage/render/render 7.53, stage/main/pre_update 7.40, stage/render/prepare_flush 7.06 |
| warm-up | 4.73 | 51.3 | pipelines/compile_ms 187, pipelines/front_ms 179, main/busy/ms 46.83, render/thread/ms 44.44, render/busy/ms 43.16, pipelines/wait_ms 40.23, stage/main/post_update 18.00, stage/render/prepare_assets 17.79, stage/main/update 15.57, stage/render/prepare_flush 9.15, stage/render/prepare_resources 7.93, pipelines/driver_ms 7.89 |
| warm-up | 4.78 | 52.6 | pipelines/compile_ms 175, pipelines/front_ms 168, render/thread/ms 81.79, render/busy/ms 80.14, main/busy/ms 64.73, stage/main/update 48.16, pipelines/wait_ms 38.87, stage/render/prepare_flush 21.79, stage/render/prepare_assets 17.56, stage/render/render 14.18, render/graph/ms 12.09, stage/main/post_update 11.50 |
| warm-up | 4.83 | 52.8 | main/busy/ms 108, stage/main/update 87.68, render/thread/ms 85.72, render/busy/ms 83.61, stream/drive/ms 44.25, stage/render/render 22.18, stage/render/prepare_assets 20.53, render/graph/ms 19.32, stage/render/graph_submit 14.78, stage/render/prepare_flush 13.88, stage/main/post_update 13.58, encode//sum 12.19 |
| warm-up | 4.91 | 81.5 | main/busy/ms 101, render/thread/ms 79.68, render/busy/ms 77.67, stage/main/update 68.64, stream/drive/ms 46.46, stage/render/prepare_assets 30.77, stage/main/post_update 24.54, stage/render/render 21.21, render/graph/ms 18.30, stage/render/graph_submit 13.29, encode/finish_wall 7.19, stage/render/prepare_resources 7.18 |
| warm-up | 4.96 | 51.1 | render/thread/ms 83.04, render/busy/ms 81.10, main/busy/ms 80.46, stage/main/update 40.79, stage/render/prepare_assets 32.93, stage/main/post_update 27.41, stage/render/render 20.89, render/graph/ms 18.18, stream/drive/ms 14.09, stage/render/graph_submit 12.89, encode//sum 7.55, stage/render/prepare_flush 7.42 |
| warm-up | 5.08 | 115.4 | render/thread/ms 113, render/busy/ms 111, main/busy/ms 65.84, stage/render/prepare_assets 51.09, stage/render/render 45.76, render/graph/ms 43.27, stage/render/graph_submit 39.07, encode//sum 37.17, encode/finish_wall 33.75, stage/main/update 33.70, stage/main/post_update 19.75, stream/drive/ms 14.71 |
| warm-up | 5.14 | 56.9 | render/thread/ms 109, render/busy/ms 108, stage/render/prepare_assets 57.55, main/busy/ms 48.63, stage/render/render 40.94, render/graph/ms 38.93, stage/render/graph_submit 35.95, encode//sum 35.82, encode/finish_wall 32.11, stage/main/update 22.52, stage/main/post_update 15.75, stream/drive/ms 5.26 |
| warm-up | 5.17 | 34.8 | render/thread/ms 68.38, render/busy/ms 66.45, main/busy/ms 63.21, stage/main/update 42.52, stage/render/prepare_assets 26.92, stream/drive/ms 26.14, stage/render/render 17.45, render/graph/ms 15.02, encode//sum 14.34, stage/render/graph_submit 11.26, stage/main/post_update 10.98, stage/render/prepare_write_phase_buffers 7.02 |
| warm-up | 5.20 | 29.1 | main/busy/ms 52.71, render/thread/ms 51.95, render/busy/ms 50.17, stage/main/update 36.65, stream/drive/ms 26.40, stage/render/render 17.31, render/graph/ms 15.00, encode//sum 14.52, stage/render/graph_submit 10.99, stage/main/post_update 9.73, stage/render/prepare_assets 7.28, stage/render/prepare_write_phase_buffers 6.97 |
| warm-up | 5.26 | 26.2 | render/thread/ms 33.72, render/busy/ms 32.22, main/busy/ms 30.74, stage/main/update 17.63, stage/render/render 16.08, render/graph/ms 13.98, stage/render/graph_submit 10.51, encode//sum 9.82, stage/main/post_update 8.98, stage/render/prepare_assets 8.95, encode/finish_wall 5.40, encode/submit 5.11 |
| warm-up | 5.34 | 26.0 | render/thread/ms 31.07, render/busy/ms 29.86, main/busy/ms 21.56, stage/render/prepare_assets 11.45, stage/render/render 11.28, render/graph/ms 9.58, stage/main/update 8.71, stage/main/post_update 7.99, stage/render/graph_submit 6.93, encode//sum 5.67, encode/finish_wall 3.50, encode/submit 3.46 |
| warm-up | 5.43 | 42.0 | main/busy/ms 57.76, render/thread/ms 45.19, render/busy/ms 43.52, stage/main/update 37.74, stage/render/render 21.00, render/graph/ms 18.51, stage/main/post_update 14.81, stage/render/graph_submit 14.43, stage/render/prepare_assets 10.27, encode/finish_wall 8.64, encode/pass:point_shadow_atlas_static_capture/sum 8.09, encode//sum 6.50 |
| warm-up | 5.49 | 55.6 | render/thread/ms 64.41, render/busy/ms 62.86, main/busy/ms 55.93, stage/main/update 33.59, stage/render/prepare_assets 29.70, stage/render/render 20.32, render/graph/ms 17.95, stage/render/graph_submit 13.53, stage/main/post_update 12.57, encode/finish_wall 8.99, encode/pass:point_shadow_atlas_static_capture/sum 8.37, encode//sum 6.73 |
| warm-up | 5.53 | 46.4 | render/thread/ms 63.73, render/busy/ms 62.33, stage/render/prepare_assets 38.55, main/busy/ms 33.64, stage/main/update 14.90, stage/render/render 13.75, render/graph/ms 11.89, stage/main/post_update 9.65, stage/render/graph_submit 8.85, encode//sum 6.17, encode/finish_wall 4.59, encode/submit 4.27 |
| measure | 8.25 | 25.7 | render/thread/ms 30.29, stage/render/render 25.56, render/present/ms 18.79, main/busy/ms 12.49, render/busy/ms 11.30, stage/main/post_update 6.67, render/graph/ms 6.46, encode//sum 5.87, stage/render/graph_submit 4.26, stage/main/update 3.09, encode/submit 2.72, stage/render/camera_driver 1.90 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=atlas48-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 10%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **97%** (min 90%) · 2626 MHz (min 2625) · memory 8101 MHz · 98.5 W · 73 °C (max 76) · VRAM 5161 MiB · throttled 100% of samples

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
- `shadow.lamp.atlas.max_lamps=48 (command line: --set)`
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
