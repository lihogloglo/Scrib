# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=48 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1730 frames over 11.4 s · **median 6.55 ms (153 fps)** · mean 6.61 · p95 7.70 · p99 8.30 (1% low 121 fps) · worst 13.68 ms · 2 spikes over 1.5× median · 10.5 stutters/min (work 5.2/min)
- warm-up: 120 frames over 6.0 s, worst 2603.2 ms
- hitches (> 25.0 ms): **0** measured, 34 in warm-up
- timed GPU passes: 5.57 ms of a 6.61 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 152.7, target 144
- pass worst frame 13.68 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.133 | 2.244 | 3.269 | 0.95 | 0.039 |
| dlss_super_resolution | 2.074 | 2.183 | 2.607 | 0.95 | 0.168 |
| bloom | 0.333 | 0.350 | 0.684 | 0.95 | 0.062 |
| ssao | 0.203 | 0.213 | 0.349 | 0.95 | 0.007 |
| upscaling | 0.164 | 0.172 | 0.188 | 0.95 | 0.005 |
| tlas_build | 0.116 | 0.122 | 0.119 | 0.95 | 0.076 |
| tonemapping | 0.102 | 0.108 | 0.114 | 0.95 | 0.004 |
| gpu_instance_materialization | 0.096 | 0.101 | 0.099 | 0.95 | 0.004 |
| gi/probe_update | 0.093 | 0.098 | 0.101 | 0.95 | 0.002 |
| early prepass | 0.078 | 0.082 | 0.091 | 0.95 | 0.051 |
| clustering | 0.075 | 0.079 | 0.084 | 0.95 | 0.019 |
| point_shadow_atlas | 0.071 | 0.084 | 0.104 | 0.84 | 0.037 |
| uniform_allocation | 0.037 | 0.039 | 0.044 | 0.95 | 0.010 |
| bin_unpacking | 0.031 | 0.033 | 0.037 | 0.95 | 0.007 |
| range_unpacking | 0.018 | 0.019 | 0.018 | 0.95 | 0.002 |
| early_mesh_preprocessing | 0.017 | 0.018 | 0.020 | 0.95 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.92 | 0.024 |
| setup_tlas_instances | 0.007 | 0.007 | 0.008 | 0.95 | 0.006 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.008 | 0.011 | 0.84 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.006 | 0.008 | 0.008 | 0.84 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.006 | 0.007 | 0.008 | 0.84 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.006 | 0.007 | 0.008 | 0.84 | 0.002 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.35 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.001 | 0.95 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.95 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.631 | 6.714 | 12.17 | 9742.0 |
| main/busy/ms | 5.415 | 6.130 | 9.62 | 9367.4 |
| render/busy/ms | 4.774 | 5.592 | 7.32 | 8259.6 |
| stage/render/render | 3.252 | 3.967 | 9.75 | 5626.8 |
| stage/main/post_update | 2.995 | 3.525 | 6.49 | 5180.7 |
| render/graph/ms | 2.701 | 3.339 | 5.31 | 4672.5 |
| encode//sum | 2.249 | 3.026 | 4.23 | 3890.0 |
| stage/render/graph_submit | 1.785 | 2.337 | 4.25 | 3087.3 |
| stage/main/update | 1.279 | 1.534 | 2.01 | 2213.4 |
| encode/submit | 1.203 | 1.672 | 6.19 | 2080.8 |
| stage/render/camera_driver | 0.777 | 0.941 | 1.28 | 1344.3 |
| encode/finish_wall | 0.622 | 0.833 | 1.52 | 1075.2 |
| stage/render/prepare_views | 0.517 | 1.396 | 1.93 | 895.1 |
| render/acquire/ms | 0.470 | 1.341 | 1.90 | 812.7 |
| encode/pass:early prepass/sum | 0.434 | 0.596 | 0.84 | 751.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.410 | 0.599 | 0.82 | 708.7 |
| stage/render/prepare_resources | 0.406 | 0.506 | 0.77 | 703.2 |
| render/present/ms | 0.387 | 0.782 | 6.70 | 669.6 |
| stage/main/pre_update | 0.351 | 0.450 | 0.67 | 606.8 |
| stage/main/first | 0.258 | 0.354 | 1.34 | 446.2 |
| stage/render/prepare_write_phase_buffers | 0.244 | 0.342 | 0.64 | 422.4 |
| bigspace/total/ms | 0.235 | 0.548 | 1.21 | 406.5 |
| stage/render/prepare_meshes | 0.217 | 0.286 | 0.59 | 376.2 |
| stage/render/prepare_assets | 0.215 | 0.284 | 0.56 | 372.3 |
| stage/render/prepare_bind_groups | 0.208 | 0.285 | 0.53 | 360.3 |
| stage/main/fixed_main_loop | 0.204 | 0.529 | 0.79 | 352.6 |
| stage/render/create_views | 0.187 | 0.261 | 0.58 | 323.5 |
| stage/render/prepare_flush | 0.176 | 0.347 | 0.54 | 304.1 |
| stage/main/last | 0.157 | 0.203 | 2.46 | 272.4 |
| stage/render/extract_commands | 0.141 | 0.171 | 0.50 | 243.1 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.135 | 0.267 | 0.54 | 234.1 |
| stream/drive/ms | 0.131 | 0.195 | 0.47 | 226.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.129 | 0.207 | 0.56 | 222.3 |
| stream/tick/ms | 0.103 | 0.159 | 0.44 | 178.3 |
| bigspace/low_precision/ms | 0.101 | 0.291 | 0.76 | 175.0 |
| controller/feed/ms | 0.086 | 0.111 | 0.35 | 148.5 |
| stage/render/prepare_batch_phases | 0.085 | 0.134 | 0.29 | 146.9 |
| anim/evaluate/ms | 0.067 | 0.146 | 0.69 | 116.5 |
| bigspace/high_precision/ms | 0.055 | 0.217 | 0.68 | 95.8 |
| physics/step/ms | 0.052 | 0.199 | 0.51 | 90.7 |
| stage/render/specialize | 0.052 | 0.075 | 0.34 | 89.1 |
| stage/render/phase_sort | 0.049 | 0.076 | 0.34 | 85.0 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.069 | 0.20 | 78.0 |
| stage/render/queue | 0.041 | 0.069 | 0.32 | 70.6 |
| encode/resolve_diagnostics/sum | 0.038 | 0.068 | 0.14 | 66.3 |
| anim/evaluate_cpu/ms | 0.037 | 0.059 | 0.19 | 64.0 |
| controller/write/ms | 0.025 | 0.034 | 0.20 | 43.1 |
| weather/map/ms | 0.025 | 0.034 | 0.19 | 43.0 |
| stage/render/cleanup | 0.021 | 0.031 | 0.18 | 36.9 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.029 | 0.09 | 25.3 |
| encode/camera_driver_span_end/sum | 0.014 | 0.029 | 0.10 | 23.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.22 | 20.5 |
| physics/insert/ms | 0.010 | 0.000 | 0.59 | 16.6 |
| script/update/ms | 0.007 | 0.011 | 0.11 | 12.8 |
| anim/apply/ms | 0.005 | 0.009 | 0.31 | 8.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 8.5 |
| far/select/ms | 0.004 | 0.005 | 0.07 | 6.9 |
| script/ui/ms | 0.004 | 0.005 | 0.03 | 6.1 |
| controller/move/ms | 0.003 | 0.009 | 0.07 | 5.3 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.002 | 0.000 | 1.47 | 2.9 |
| controller/avoid/ms | 0.002 | 0.005 | 0.02 | 2.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| weather/apply/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 1.1 |
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
| crowd/joint_blocks/count | count | 2 | 3 | 3 |
| crowd/joints_extracted/count | count | 48 | 1403 | 0 |
| crowd/joints_uploaded/count | count | 94 | 153 | 153 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.43 | 1.20 | 0.41 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.09 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.10 | 0.01 |
| encode/finish_critical | ms | 0.49 | 1.47 | 0.42 |
| encode/pass:early prepass/max | ms | 0.43 | 0.84 | 0.26 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.40 | 0.82 | 0.42 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 0.56 | 0.08 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.14 | 0.54 | 0.11 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.44 | 1.47 | 1.47 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.14 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.98 | 9.62 | 5.06 |
| gpu/timed/ms | ms | 5.94 | 7.54 | 5.05 |
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
| phases/opaque3d/items | count | 266 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 7 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 162 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 266 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 6 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 69 | 0 |
| phases/shadow/bins | count | 0 | 577 | 0 |
| phases/shadow/instance_batches | count | 0 | 12 | 0 |
| phases/shadow/instance_draws | count | 0 | 1062 | 0 |
| phases/shadow/instance_slots | count | 0 | 49.2 k | 0 |
| phases/shadow/items | count | 0 | 750 | 0 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 0 | 63 | 0 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 100 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 294 | 294 | 294 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 4 | 71 | 4 |
| preprocess/bind_groups_created | count | 0 | 61 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 10 | 4 |
| preprocess/bind_groups_reused | count | 47 | 60 | 35 |
| process/cpu_usage | % | 28.93 | 38.76 | 20.67 |
| process/mem_usage | GiB | 3.61 | 3.65 | 3.65 |
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
| shadow_atlas/alloc_failures | count | 710 | 751 | 751 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 24 | 24 | 24 |
| shadow_atlas/live_occ_pct | % | 15.62 | 72.66 | 9.38 |
| shadow_atlas/live_tiles | count | 7 | 27 | 6 |
| shadow_atlas/mover_draws | count | 6 | 12 | 1 |
| shadow_atlas/movers | count | 348 | 349 | 348 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 6 | 12 | 5 |
| shadow_atlas/render/clears | count | 0 | 6 | 0 |
| shadow_atlas/render/lamps | count | 24 | 24 | 24 |
| shadow_atlas/render/live_direct | count | 6 | 12 | 5 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 9 | 44 | 11 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 94.92 | 94.92 | 94.92 |
| shadow_atlas/static_tiles | count | 144 | 144 | 144 |
| shadow_atlas/views | count | 9 | 44 | 11 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 1 | 24 | 0 |
| skin/blocks_skipped/count | count | 1 | 3 | 3 |
| skin/blocks_walked/count | count | 1 | 2 | 0 |
| skin/joint_bytes_written/bytes | bytes | 3072 | 19.8 k | 0 |
| skin/joints_written/count | count | 48 | 310 | 0 |
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
| system/cpu_usage | % | 37.73 | 45.80 | 31.09 |
| system/mem_usage | % | 50.57 | 50.64 | 50.64 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 23 | 95 | 17 |
| views/all/bins | count | 334 | 1159 | 58 |
| views/all/count | count | 10 | 45 | 12 |
| views/all/instance_batches | count | 8 | 20 | 8 |
| views/all/instance_draws | count | 3220 | 4282 | 3220 |
| views/all/instance_slots | count | 98.3 k | 147.5 k | 98.3 k |
| views/all/items | count | 552 | 1812 | 86 |
| views/all/pipelines | count | 21 | 90 | 15 |
| views/all/pipelines_distinct | count | 21 | 34 | 15 |
| views/all/work_items | count | 98.9 k | 148.8 k | 98.4 k |
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
| views/lamp_face/bins | count | 0 | 577 | 0 |
| views/lamp_face/count | count | 9 | 44 | 11 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 12 | 0 |
| views/lamp_face/instance_draws | count | 0 | 1062 | 0 |
| views/lamp_face/instance_slots | count | 0 | 49.2 k | 0 |
| views/lamp_face/items | count | 0 | 750 | 0 |
| views/lamp_face/pipelines | count | 0 | 63 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 49.4 k | 0 |
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
| anim/bones_written/count | count | 9.20 | 30 | 15.9 k |
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
| encode//count | count | 19.36 | 20 | 33.5 k |
| encode/bind_walk/init_textures | count | 4149 | 12.6 k | 7.18 M |
| encode/bind_walk/init_textures_skipped | count | 7157 | 25.4 k | 12.38 M |
| encode/bind_walk/inits | count | 157 | 282 | 271.4 k |
| encode/bind_walk/inits_skipped | count | 16.39 | 40 | 28.4 k |
| encode/bind_walk/merge_textures | count | 2906 | 11.4 k | 5.03 M |
| encode/bind_walk/merge_textures_skipped | count | 52.27 | 14.1 k | 90.4 k |
| encode/bind_walk/merges | count | 43.63 | 71 | 75.5 k |
| encode/bind_walk/merges_skipped | count | 11.53 | 48 | 20.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1730 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1730 |
| encode/pass:early prepass/count | count | 1 | 1 | 1730 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1730 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1682 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.88 | 1 | 1529 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.00 | 1 | 2 |
| encode/queue/command_buffers | count | 29.22 | 30 | 50.5 k |
| encode/queue/submits | count | 2 | 2 | 3460 |
| encode/queue/write_buffer/bytes | bytes | 597.1 k | 668.9 k | 1.03 G |
| encode/queue/write_buffer/calls | count | 98.87 | 206 | 171.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 271.9 k | 271.9 k | 470.34 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 10.4 k |
| encode/queue/write_texture/bytes | bytes | 606 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1730 |
| physics/inserts | count | 0.06 | 10 | 101 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.40 | 1 | 686 |
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

## Hitches (> 25.0 ms; slowest 34, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2603.2 | main/busy/ms 2571, stage/main/post_update 2448, stage/main/update 110, surface/advance_cpu_ms 81.55, stage/main/pre_update 7.06, stage/main/last 2.75, stage/main/first 0.59, stream/drive/ms 0.51, stage/main/fixed_main_loop 0.51, weather/map/ms 0.22, stream/tick/ms 0.09, bigspace/total/ms 0.08 |
| warm-up | 3.43 | 823.8 | main/busy/ms 2690, stage/main/post_update 2453, render/thread/ms 775, render/busy/ms 607, stage/render/prepare_assets 563, stage/main/update 218, stage/render/prepare_views 166, render/acquire/ms 166, surface/advance_cpu_ms 81.55, stage/render/create_views 31.55, stage/render/render 9.14, stage/main/pre_update 8.63 |
| warm-up | 3.48 | 48.6 | render/thread/ms 808, render/busy/ms 639, stage/render/prepare_assets 591, stage/render/prepare_views 166, render/acquire/ms 166, main/busy/ms 133, stage/main/update 112, stage/render/create_views 31.89, stage/render/render 12.52, stage/main/post_update 8.94, render/graph/ms 8.80, stage/main/fixed_main_loop 8.49 |
| warm-up | 3.51 | 34.5 | pipelines/wait_ms 497, pipelines/compile_ms 289, pipelines/front_ms 266, render/thread/ms 66.04, render/busy/ms 60.71, main/busy/ms 55.21, stage/main/update 39.24, stage/render/render 33.13, stage/render/prepare_assets 29.14, render/graph/ms 27.87, pipelines/driver_ms 22.56, stage/render/graph_submit 15.78 |
| warm-up | 3.56 | 48.8 | pipelines/wait_ms 1070, pipelines/compile_ms 559, pipelines/front_ms 517, render/thread/ms 68.60, render/busy/ms 59.08, main/busy/ms 56.25, stage/render/render 48.93, pipelines/driver_ms 42.24, stage/main/update 39.78, render/graph/ms 39.42, stage/render/graph_submit 22.00, encode/finish_wall 10.25 |
| warm-up | 3.63 | 30.9 | pipelines/compile_ms 426, pipelines/front_ms 406, main/busy/ms 72.56, pipelines/wait_ms 66.56, stage/main/update 60.13, render/thread/ms 38.48, render/busy/ms 36.74, pipelines/driver_ms 19.89, stage/render/prepare_assets 9.08, stage/render/render 8.96, stage/main/post_update 7.82, render/graph/ms 6.93 |
| warm-up | 3.74 | 105.3 | pipelines/compile_ms 1174, pipelines/front_ms 1128, pipelines/wait_ms 130, render/thread/ms 82.70, main/busy/ms 80.77, render/busy/ms 80.67, stage/main/update 65.77, pipelines/driver_ms 46.12, stage/render/prepare_assets 41.94, stage/render/render 25.10, render/graph/ms 22.78, stage/render/graph_submit 15.46 |
| warm-up | 3.76 | 25.5 | pipelines/compile_ms 1186, pipelines/front_ms 1138, render/thread/ms 84.83, render/busy/ms 82.75, pipelines/wait_ms 76.23, stage/render/prepare_assets 55.54, pipelines/driver_ms 48.11, main/busy/ms 30.72, stage/render/render 24.30, render/graph/ms 22.04, stage/render/graph_submit 15.10, stage/main/update 14.52 |
| warm-up | 3.80 | 31.6 | pipelines/compile_ms 671, pipelines/front_ms 633, render/thread/ms 50.68, render/busy/ms 49.16, stage/render/prepare_assets 38.54, pipelines/driver_ms 38.26, main/busy/ms 23.79, pipelines/wait_ms 13.98, pipelines/lock_ms 13.50, stage/main/update 9.54, stage/main/post_update 8.21, stage/render/render 7.52 |
| warm-up | 3.83 | 30.7 | pipelines/compile_ms 416, pipelines/front_ms 392, render/thread/ms 52.52, render/busy/ms 51.08, main/busy/ms 40.33, stage/render/prepare_assets 39.43, pipelines/driver_ms 24.67, stage/main/update 22.81, stream/drive/ms 16.88, stage/main/post_update 8.46, stage/render/render 7.07, render/graph/ms 5.41 |
| warm-up | 3.87 | 46.7 | pipelines/compile_ms 638, pipelines/front_ms 611, render/thread/ms 60.29, render/busy/ms 58.59, main/busy/ms 53.26, stage/render/prepare_assets 44.16, stage/main/update 33.07, pipelines/driver_ms 27.39, stream/drive/ms 15.23, stage/main/post_update 10.91, stage/render/render 7.91, render/graph/ms 5.93 |
| warm-up | 3.94 | 62.4 | pipelines/compile_ms 1037, pipelines/front_ms 996, render/thread/ms 87.38, render/busy/ms 85.88, main/busy/ms 62.61, stage/render/prepare_assets 52.50, pipelines/driver_ms 41.22, stage/main/update 25.93, stage/main/post_update 21.88, stage/render/render 17.16, render/graph/ms 15.34, stage/render/graph_submit 13.47 |
| warm-up | 3.97 | 34.0 | pipelines/compile_ms 722, pipelines/front_ms 696, main/busy/ms 187, stage/main/update 137, render/thread/ms 73.94, render/busy/ms 72.34, stage/render/prepare_assets 39.90, pipelines/driver_ms 25.64, stage/main/pre_update 24.45, stage/main/post_update 19.34, stage/render/render 17.97, render/graph/ms 15.92 |
| warm-up | 4.21 | 241.7 | pipelines/compile_ms 2534, pipelines/front_ms 2456, pipelines/wait_ms 287, main/busy/ms 179, stage/main/update 153, render/thread/ms 95.91, render/busy/ms 94.25, stage/render/prepare_assets 84.41, pipelines/driver_ms 77.70, stream/drive/ms 25.48, stage/main/pre_update 16.83, stage/render/render 7.55 |
| warm-up | 4.25 | 42.9 | pipelines/compile_ms 2357, pipelines/front_ms 2283, pipelines/wait_ms 285, render/thread/ms 100, render/busy/ms 98.48, stage/render/prepare_assets 86.82, pipelines/driver_ms 74.62, main/busy/ms 40.74, stage/main/update 30.08, stream/drive/ms 25.41, stage/render/render 10.12, pipelines/lock_ms 8.60 |
| warm-up | 4.35 | 77.0 | pipelines/compile_ms 839, pipelines/front_ms 805, main/busy/ms 54.21, render/thread/ms 47.73, render/busy/ms 44.76, pipelines/driver_ms 33.24, stage/main/update 27.52, stage/render/render 21.79, render/graph/ms 18.61, stage/render/prepare_assets 15.95, stage/render/graph_submit 14.11, pipelines/wait_ms 13.35 |
| warm-up | 4.64 | 26.7 | pipelines/compile_ms 459, pipelines/front_ms 445, render/thread/ms 40.10, render/busy/ms 38.75, main/busy/ms 36.43, stage/main/post_update 17.32, pipelines/driver_ms 13.90, stage/render/render 12.49, pipelines/wait_ms 12.49, stage/render/prepare_flush 11.86, stage/main/update 11.81, render/graph/ms 10.93 |
| warm-up | 4.69 | 44.1 | pipelines/compile_ms 904, pipelines/front_ms 871, render/thread/ms 61.93, render/busy/ms 54.13, main/busy/ms 48.85, pipelines/driver_ms 32.59, stage/render/prepare_assets 27.56, stage/main/post_update 21.06, stage/render/render 19.39, stage/main/update 14.52, render/graph/ms 11.52, pipelines/wait_ms 10.60 |
| warm-up | 4.73 | 40.1 | pipelines/compile_ms 963, pipelines/front_ms 926, render/thread/ms 74.52, render/busy/ms 66.65, main/busy/ms 62.95, stage/main/update 37.98, pipelines/driver_ms 36.46, stage/render/prepare_assets 28.07, stage/render/render 20.74, pipelines/wait_ms 17.07, stage/main/post_update 13.41, render/graph/ms 12.24 |
| warm-up | 4.78 | 52.9 | pipelines/compile_ms 872, pipelines/front_ms 844, main/busy/ms 106, stage/main/update 85.21, render/thread/ms 73.70, render/busy/ms 71.86, stream/drive/ms 40.13, pipelines/driver_ms 28.06, stage/render/render 23.24, render/graph/ms 20.56, stage/render/prepare_assets 18.56, stage/render/graph_submit 15.01 |
| warm-up | 4.86 | 76.2 | pipelines/compile_ms 1318, pipelines/front_ms 1270, main/busy/ms 97.25, stage/main/update 71.78, render/thread/ms 70.02, render/busy/ms 68.18, pipelines/driver_ms 47.90, stream/drive/ms 41.84, stage/render/prepare_assets 29.10, stage/render/render 20.90, pipelines/wait_ms 20.51, render/graph/ms 18.29 |
| warm-up | 4.90 | 45.0 | pipelines/compile_ms 1021, pipelines/front_ms 978, main/busy/ms 72.99, render/thread/ms 67.10, render/busy/ms 65.21, pipelines/driver_ms 42.41, stage/main/update 41.34, stage/render/prepare_assets 29.17, stage/main/post_update 21.11, stage/render/render 18.12, pipelines/wait_ms 17.87, render/graph/ms 15.45 |
| warm-up | 5.00 | 99.1 | pipelines/compile_ms 340, pipelines/front_ms 328, render/thread/ms 93.93, render/busy/ms 92.44, main/busy/ms 61.57, stage/render/prepare_assets 43.01, stage/main/update 33.48, stage/render/prepare_flush 24.28, stage/main/post_update 18.02, stream/drive/ms 14.27, stage/render/render 13.89, render/graph/ms 11.86 |
| warm-up | 5.04 | 42.7 | pipelines/compile_ms 108, pipelines/front_ms 106, render/thread/ms 87.69, render/busy/ms 86.57, stage/render/prepare_assets 47.52, main/busy/ms 36.16, stage/render/prepare_flush 24.09, stage/main/update 17.46, stage/main/post_update 11.01, stage/render/render 8.08, render/graph/ms 6.55, stage/render/graph_submit 4.13 |
| warm-up | 5.07 | 30.4 | main/busy/ms 59.98, render/thread/ms 56.27, render/busy/ms 53.53, stage/main/update 42.78, stream/drive/ms 23.70, stage/render/render 21.81, stage/render/prepare_assets 21.09, render/graph/ms 18.78, stage/render/graph_submit 11.82, stage/main/post_update 11.02, encode/submit 8.61, encode//sum 4.58 |
| warm-up | 5.12 | 42.2 | main/busy/ms 58.39, render/thread/ms 52.33, render/busy/ms 49.66, stage/main/update 40.45, stage/render/render 24.22, stream/drive/ms 24.05, render/graph/ms 21.17, stage/render/graph_submit 13.78, stage/main/post_update 13.18, stage/render/prepare_assets 10.63, encode/submit 9.09, encode/finish_wall 5.50 |
| warm-up | 5.19 | 25.4 | pipelines/compile_ms 1367, pipelines/front_ms 1324, pipelines/driver_ms 43.01, render/thread/ms 40.74, render/busy/ms 39.43, main/busy/ms 26.62, pipelines/wait_ms 20.32, stage/main/update 13.84, stage/render/render 13.00, render/graph/ms 11.17, stage/render/prepare_assets 9.40, pipelines/lock_ms 8.75 |
| warm-up | 5.23 | 33.7 | pipelines/compile_ms 91.81, pipelines/front_ms 86.36, render/thread/ms 44.10, render/busy/ms 42.18, main/busy/ms 36.22, stage/main/post_update 21.58, stage/render/render 14.47, render/graph/ms 12.67, stage/main/update 10.79, stage/render/graph_submit 9.42, stage/render/prepare_assets 6.24, encode//sum 5.53 |
| warm-up | 5.27 | 31.9 | pipelines/compile_ms 104, pipelines/front_ms 97.57, render/thread/ms 57.87, render/busy/ms 56.05, main/busy/ms 40.33, stage/main/post_update 20.24, stage/render/render 15.37, render/graph/ms 13.38, stage/main/update 10.28, stage/render/graph_submit 9.97, stage/render/prepare_assets 9.31, stage/render/prepare_resources 7.25 |
| warm-up | 5.31 | 41.1 | pipelines/compile_ms 273, pipelines/front_ms 260, render/thread/ms 61.21, render/busy/ms 59.64, main/busy/ms 45.02, stage/render/prepare_assets 16.10, stage/render/render 15.71, stage/main/pre_update 15.24, render/graph/ms 13.77, pipelines/driver_ms 12.67, stage/main/post_update 12.39, stage/main/update 11.17 |
| warm-up | 5.40 | 29.3 | main/busy/ms 33.98, render/thread/ms 32.43, render/busy/ms 31.19, stage/main/update 20.49, stage/render/render 11.59, render/graph/ms 9.77, stage/main/post_update 8.77, stage/render/prepare_assets 8.70, stage/render/graph_submit 6.98, encode//sum 4.24, stream/drive/ms 3.85, encode/finish_wall 3.65 |
| warm-up | 5.43 | 33.8 | pipelines/compile_ms 408, pipelines/front_ms 392, pipelines/wait_ms 97.70, render/thread/ms 50.70, render/busy/ms 49.20, main/busy/ms 40.97, stage/main/update 24.42, stage/render/prepare_assets 19.75, pipelines/driver_ms 15.93, stage/render/render 15.35, render/graph/ms 12.95, stage/main/post_update 10.59 |
| warm-up | 5.46 | 27.4 | pipelines/compile_ms 408, pipelines/front_ms 392, pipelines/wait_ms 97.70, render/thread/ms 46.91, render/busy/ms 45.54, main/busy/ms 39.01, stage/main/update 22.92, stage/render/prepare_assets 21.01, pipelines/driver_ms 15.93, stage/render/render 13.28, render/graph/ms 11.03, stage/main/post_update 9.64 |
| warm-up | 5.50 | 36.7 | render/thread/ms 40.14, render/busy/ms 39.21, main/busy/ms 23.77, stage/render/prepare_assets 22.54, stage/main/update 10.60, stage/render/render 9.16, render/graph/ms 7.72, stage/main/post_update 7.60, stage/render/graph_submit 5.48, encode//sum 4.36, encode/finish_wall 2.93, stream/drive/ms 2.92 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=atlas48-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 9% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **97%** (min 91%) · 2625 MHz (min 2625) · memory 8101 MHz · 100.9 W · 75 °C (max 78) · VRAM 5176 MiB · throttled 100% of samples

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
