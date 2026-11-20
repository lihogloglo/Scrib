# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=8 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1767 frames over 11.4 s · **median 6.35 ms (158 fps)** · mean 6.47 · p95 7.65 · p99 8.46 (1% low 118 fps) · worst 13.23 ms · 7 spikes over 1.5× median · 36.7 stutters/min (work 5.2/min)
- warm-up: 120 frames over 5.8 s, worst 2468.3 ms
- hitches (> 25.0 ms): **0** measured, 32 in warm-up
- timed GPU passes: 5.32 ms of a 6.47 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 157.5, target 144
- pass worst frame 13.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.015 | 2.148 | 2.438 | 0.94 | 0.168 |
| main_opaque_pass_3d | 1.979 | 2.111 | 3.161 | 0.94 | 0.046 |
| bloom | 0.320 | 0.341 | 0.495 | 0.94 | 0.063 |
| ssao | 0.204 | 0.218 | 0.540 | 0.94 | 0.007 |
| upscaling | 0.157 | 0.168 | 0.188 | 0.94 | 0.004 |
| tlas_build | 0.111 | 0.119 | 0.118 | 0.94 | 0.072 |
| tonemapping | 0.101 | 0.108 | 0.121 | 0.94 | 0.004 |
| gpu_instance_materialization | 0.093 | 0.099 | 0.099 | 0.94 | 0.004 |
| gi/probe_update | 0.092 | 0.098 | 0.099 | 0.94 | 0.002 |
| early prepass | 0.077 | 0.082 | 0.090 | 0.94 | 0.050 |
| clustering | 0.073 | 0.077 | 0.081 | 0.94 | 0.019 |
| point_shadow_atlas | 0.069 | 0.105 | 0.127 | 0.65 | 0.053 |
| uniform_allocation | 0.035 | 0.037 | 0.044 | 0.94 | 0.011 |
| bin_unpacking | 0.031 | 0.033 | 0.037 | 0.94 | 0.009 |
| early_mesh_preprocessing | 0.018 | 0.019 | 0.022 | 0.94 | 0.004 |
| range_unpacking | 0.017 | 0.018 | 0.018 | 0.94 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.91 | 0.025 |
| setup_tlas_instances | 0.007 | 0.007 | 0.008 | 0.94 | 0.005 |
| point_shadow_atlas/early_mesh_preprocessing | 0.006 | 0.009 | 0.011 | 0.65 | 0.002 |
| point_shadow_atlas/bin_unpacking | 0.005 | 0.008 | 0.008 | 0.65 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.005 | 0.008 | 0.007 | 0.65 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.005 | 0.007 | 0.007 | 0.65 | 0.002 |
| sparse buffer updates | 0.004 | 0.007 | 0.007 | 0.57 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.001 | 0.94 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.94 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.536 | 6.679 | 12.30 | 9782.9 |
| main/busy/ms | 5.042 | 5.742 | 8.46 | 8908.9 |
| render/busy/ms | 4.798 | 5.610 | 9.23 | 8478.1 |
| stage/render/render | 3.298 | 4.005 | 9.75 | 5827.1 |
| render/graph/ms | 2.743 | 3.384 | 5.27 | 4847.5 |
| stage/main/post_update | 2.613 | 3.092 | 4.19 | 4616.3 |
| encode//sum | 2.281 | 3.290 | 5.27 | 4030.4 |
| stage/render/graph_submit | 1.805 | 2.347 | 4.15 | 3189.1 |
| stage/main/update | 1.280 | 1.530 | 3.86 | 2261.7 |
| encode/submit | 1.211 | 1.641 | 5.36 | 2140.5 |
| stage/render/camera_driver | 0.800 | 0.965 | 1.52 | 1413.4 |
| encode/finish_wall | 0.647 | 0.894 | 1.61 | 1142.7 |
| encode/pass:early prepass/sum | 0.463 | 0.629 | 0.94 | 817.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.438 | 0.631 | 1.05 | 773.2 |
| stage/render/prepare_resources | 0.412 | 0.505 | 2.99 | 727.9 |
| render/present/ms | 0.392 | 0.830 | 4.28 | 693.2 |
| stage/render/prepare_views | 0.392 | 1.148 | 1.83 | 692.5 |
| stage/main/pre_update | 0.356 | 0.449 | 0.69 | 629.8 |
| render/acquire/ms | 0.346 | 1.109 | 1.76 | 611.5 |
| stage/render/prepare_write_phase_buffers | 0.256 | 0.360 | 0.83 | 453.0 |
| stage/main/first | 0.249 | 0.352 | 1.71 | 440.6 |
| bigspace/total/ms | 0.232 | 0.517 | 1.01 | 410.6 |
| stage/main/fixed_main_loop | 0.226 | 0.615 | 0.87 | 398.6 |
| stage/render/prepare_bind_groups | 0.224 | 0.298 | 0.67 | 395.7 |
| stage/render/prepare_assets | 0.214 | 0.282 | 1.87 | 378.2 |
| stage/render/prepare_meshes | 0.175 | 0.244 | 0.47 | 309.4 |
| stage/render/create_views | 0.175 | 0.249 | 0.59 | 308.6 |
| stage/render/prepare_flush | 0.175 | 0.286 | 0.60 | 308.5 |
| stage/main/last | 0.150 | 0.196 | 0.47 | 265.8 |
| stage/render/extract_commands | 0.148 | 0.180 | 0.32 | 262.3 |
| stream/drive/ms | 0.133 | 0.188 | 0.54 | 235.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.132 | 0.214 | 0.55 | 232.4 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.117 | 0.257 | 0.60 | 207.2 |
| stream/tick/ms | 0.105 | 0.157 | 0.49 | 186.4 |
| bigspace/low_precision/ms | 0.103 | 0.288 | 0.77 | 181.5 |
| controller/feed/ms | 0.087 | 0.111 | 0.43 | 154.1 |
| stage/render/prepare_batch_phases | 0.085 | 0.140 | 0.35 | 150.0 |
| physics/step/ms | 0.069 | 0.252 | 0.41 | 121.6 |
| anim/evaluate/ms | 0.066 | 0.135 | 0.82 | 116.4 |
| stage/render/specialize | 0.052 | 0.076 | 0.42 | 91.3 |
| stage/render/phase_sort | 0.051 | 0.084 | 0.34 | 90.7 |
| bigspace/high_precision/ms | 0.051 | 0.174 | 0.82 | 89.9 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.072 | 0.23 | 83.0 |
| stage/render/queue | 0.043 | 0.078 | 0.38 | 76.0 |
| anim/evaluate_cpu/ms | 0.037 | 0.059 | 0.42 | 66.1 |
| encode/resolve_diagnostics/sum | 0.037 | 0.069 | 0.33 | 65.6 |
| weather/map/ms | 0.026 | 0.036 | 0.21 | 45.8 |
| controller/write/ms | 0.025 | 0.034 | 0.09 | 44.2 |
| stage/render/cleanup | 0.021 | 0.031 | 0.20 | 37.7 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.028 | 0.20 | 26.3 |
| encode/camera_driver_span_end/sum | 0.014 | 0.025 | 0.29 | 23.9 |
| weather/eval/ms | 0.011 | 0.016 | 0.03 | 20.1 |
| physics/insert/ms | 0.010 | 0.000 | 0.59 | 18.3 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.008 | 0.000 | 1.59 | 14.6 |
| script/update/ms | 0.008 | 0.011 | 0.14 | 14.0 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 8.8 |
| anim/apply/ms | 0.005 | 0.009 | 0.03 | 8.6 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 7.5 |
| controller/move/ms | 0.003 | 0.009 | 0.27 | 6.0 |
| script/ui/ms | 0.003 | 0.004 | 0.14 | 5.7 |
| controller/avoid/ms | 0.002 | 0.005 | 0.06 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.4 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.1 |
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
| encode//max | ms | 0.44 | 1.20 | 0.34 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.20 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.01 |
| encode/finish_critical | ms | 0.51 | 1.59 | 0.50 |
| encode/pass:early prepass/max | ms | 0.46 | 0.94 | 0.40 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.42 | 1.05 | 0.50 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.13 | 0.55 | 0.10 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.16 | 0.60 | 0.13 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 1.22 | 1.59 | 0.80 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.33 | 0.02 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.83 | 9.23 | 5.75 |
| gpu/timed/ms | ms | 5.82 | 7.37 | 5.75 |
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
| phases/opaque3d/items | count | 262 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 7 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 162 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 262 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 6 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 17 | 56 | 18 |
| phases/shadow/bins | count | 17 | 817 | 225 |
| phases/shadow/instance_batches | count | 0 | 6 | 0 |
| phases/shadow/instance_draws | count | 0 | 42 | 0 |
| phases/shadow/instance_slots | count | 0 | 24.6 k | 0 |
| phases/shadow/items | count | 17 | 1086 | 301 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 2 | 79 | 18 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 86 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 297 | 297 | 297 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 4 | 106 | 4 |
| preprocess/bind_groups_created | count | 0 | 102 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 10 | 4 |
| preprocess/bind_groups_reused | count | 48 | 60 | 35 |
| process/cpu_usage | % | 25.53 | 34.62 | 28.23 |
| process/mem_usage | GiB | 3.60 | 3.63 | 3.63 |
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
| shadow_atlas/alloc_failures | count | 4242 | 4242 | 4242 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 8 | 9 | 8 |
| shadow_atlas/live_occ_pct | % | 37.50 | 100 | 0 |
| shadow_atlas/live_tiles | count | 6 | 16 | 0 |
| shadow_atlas/mover_draws | count | 6 | 12 | 0 |
| shadow_atlas/movers | count | 200 | 458 | 56 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 5 | 12 | 0 |
| shadow_atlas/render/clears | count | 0 | 6 | 0 |
| shadow_atlas/render/lamps | count | 8 | 9 | 8 |
| shadow_atlas/render/live_direct | count | 5 | 12 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 8 | 33 | 0 |
| shadow_atlas/resizes | count | 0 | 1 | 1 |
| shadow_atlas/static_occ_pct | % | 75 | 84.38 | 60.94 |
| shadow_atlas/static_tiles | count | 48 | 54 | 48 |
| shadow_atlas/views | count | 8 | 33 | 0 |
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
| system/cpu_usage | % | 33.16 | 44.58 | 32.88 |
| system/mem_usage | % | 50.51 | 50.60 | 50.60 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 29 | 85 | 17 |
| views/all/bins | count | 337 | 1347 | 58 |
| views/all/count | count | 9 | 34 | 1 |
| views/all/instance_batches | count | 8 | 14 | 8 |
| views/all/instance_draws | count | 3220 | 3262 | 3220 |
| views/all/instance_slots | count | 98.3 k | 122.9 k | 98.3 k |
| views/all/items | count | 605 | 2064 | 86 |
| views/all/pipelines | count | 24 | 106 | 15 |
| views/all/pipelines_distinct | count | 24 | 34 | 15 |
| views/all/work_items | count | 98.9 k | 124.3 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 56 | 0 |
| views/lamp_face/bins | count | 17 | 817 | 0 |
| views/lamp_face/count | count | 8 | 33 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 6 | 0 |
| views/lamp_face/instance_draws | count | 0 | 42 | 0 |
| views/lamp_face/instance_slots | count | 0 | 24.6 k | 0 |
| views/lamp_face/items | count | 17 | 1086 | 0 |
| views/lamp_face/pipelines | count | 2 | 79 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 25.0 k | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 334 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 548 | 1062 | 86 |
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
| anim/bones_written/count | count | 9.13 | 30 | 16.1 k |
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
| encode//count | count | 19.62 | 40 | 34.7 k |
| encode/bind_walk/init_textures | count | 4187 | 12.6 k | 7.40 M |
| encode/bind_walk/init_textures_skipped | count | 7018 | 22.6 k | 12.40 M |
| encode/bind_walk/inits | count | 159 | 312 | 280.2 k |
| encode/bind_walk/inits_skipped | count | 15.11 | 32 | 26.7 k |
| encode/bind_walk/merge_textures | count | 2944 | 11.4 k | 5.20 M |
| encode/bind_walk/merge_textures_skipped | count | 73.77 | 11.3 k | 130.4 k |
| encode/bind_walk/merges | count | 44.46 | 90 | 78.6 k |
| encode/bind_walk/merges_skipped | count | 10.66 | 38 | 18.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1767 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1767 |
| encode/pass:early prepass/count | count | 1 | 2 | 1767 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1767 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1717 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.69 | 2 | 1226 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.01 | 1 | 13 |
| encode/queue/command_buffers | count | 29.29 | 60 | 51.8 k |
| encode/queue/submits | count | 2 | 4 | 3534 |
| encode/queue/write_buffer/bytes | bytes | 535.7 k | 942.8 k | 946.55 M |
| encode/queue/write_buffer/calls | count | 100 | 198 | 177.0 k |
| encode/queue/write_buffer_with/bytes | bytes | 117.1 k | 234.1 k | 206.84 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 10.6 k |
| encode/queue/write_texture/bytes | bytes | 593 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1767 |
| physics/inserts | count | 0.05 | 10 | 87 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.39 | 1 | 686 |
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

## Hitches (> 25.0 ms; slowest 32, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2468.3 | main/busy/ms 2440, stage/main/post_update 2334, stage/main/update 94.62, surface/advance_cpu_ms 67.01, stage/main/pre_update 7.36, stage/main/last 1.11, stage/main/first 0.58, stream/drive/ms 0.44, stage/main/fixed_main_loop 0.37, weather/map/ms 0.21, bigspace/total/ms 0.08, stream/tick/ms 0.08 |
| warm-up | 3.23 | 759.5 | main/busy/ms 2554, stage/main/post_update 2339, render/thread/ms 721, render/busy/ms 558, stage/render/prepare_assets 515, stage/main/update 198, stage/render/prepare_views 161, render/acquire/ms 161, surface/advance_cpu_ms 67.01, stage/render/create_views 30.29, stage/render/render 9.46, stage/main/pre_update 8.57 |
| warm-up | 3.29 | 65.0 | render/thread/ms 770, render/busy/ms 606, stage/render/prepare_assets 559, stage/render/prepare_views 161, render/acquire/ms 161, main/busy/ms 127, stage/main/update 108, stage/render/create_views 30.58, stage/render/render 12.96, render/graph/ms 9.28, stage/main/post_update 8.97, stage/main/fixed_main_loop 6.44 |
| warm-up | 3.33 | 38.7 | pipelines/compile_ms 144, pipelines/front_ms 131, pipelines/wait_ms 116, render/thread/ms 73.61, render/busy/ms 68.77, main/busy/ms 56.72, stage/render/prepare_assets 45.05, stage/main/update 38.92, stage/render/render 24.71, render/graph/ms 19.74, pipelines/lock_ms 15.66, pipelines/driver_ms 12.82 |
| warm-up | 3.37 | 41.4 | pipelines/wait_ms 1497, pipelines/compile_ms 548, pipelines/front_ms 492, main/busy/ms 74.07, pipelines/driver_ms 55.82, render/thread/ms 47.41, render/busy/ms 41.65, stage/main/update 39.02, stage/render/render 30.86, render/graph/ms 24.90, stage/main/pre_update 18.87, pipelines/lock_ms 17.10 |
| warm-up | 3.42 | 47.3 | pipelines/wait_ms 1456, pipelines/compile_ms 632, pipelines/front_ms 579, render/thread/ms 55.76, pipelines/driver_ms 52.97, render/busy/ms 52.95, main/busy/ms 44.40, stage/render/prepare_assets 20.29, stage/main/pre_update 18.56, stage/render/render 16.50, render/graph/ms 13.39, stage/render/prepare_flush 11.21 |
| warm-up | 3.45 | 26.4 | pipelines/compile_ms 427, pipelines/front_ms 408, pipelines/wait_ms 79.31, main/busy/ms 76.20, render/thread/ms 57.58, stage/main/update 55.77, render/busy/ms 52.12, stage/render/render 21.13, pipelines/driver_ms 18.82, render/graph/ms 15.60, stage/render/prepare_assets 12.58, stage/render/graph_submit 11.57 |
| warm-up | 3.55 | 106.0 | pipelines/compile_ms 1133, pipelines/front_ms 1095, pipelines/wait_ms 94.08, render/thread/ms 86.19, render/busy/ms 78.62, main/busy/ms 77.35, stage/main/update 58.38, stage/render/prepare_assets 45.97, pipelines/driver_ms 38.50, stage/render/render 27.67, render/graph/ms 20.20, stage/render/graph_submit 14.45 |
| warm-up | 3.60 | 32.9 | pipelines/compile_ms 165, pipelines/front_ms 157, render/thread/ms 46.12, render/busy/ms 44.47, pipelines/wait_ms 40.06, pipelines/lock_ms 39.32, stage/render/prepare_assets 34.87, main/busy/ms 24.84, pipelines/driver_ms 8.76, stage/main/update 8.26, stage/main/post_update 7.15, stage/render/render 6.60 |
| warm-up | 3.70 | 57.1 | pipelines/compile_ms 944, pipelines/front_ms 906, main/busy/ms 55.98, pipelines/driver_ms 38.10, stage/main/update 37.79, render/thread/ms 34.63, render/busy/ms 32.67, stage/render/prepare_assets 20.99, stream/drive/ms 20.91, stage/main/post_update 9.74, stage/render/render 9.46, render/graph/ms 7.18 |
| warm-up | 3.76 | 55.3 | pipelines/compile_ms 1256, pipelines/front_ms 1204, main/busy/ms 177, stage/main/update 153, render/thread/ms 71.53, render/busy/ms 69.68, stage/render/prepare_assets 53.36, pipelines/driver_ms 52.02, stage/main/post_update 11.31, stage/render/render 10.61, pipelines/wait_ms 9.11, pipelines/lock_ms 8.60 |
| warm-up | 3.99 | 237.4 | pipelines/compile_ms 2167, pipelines/front_ms 2084, main/busy/ms 159, stage/main/update 142, render/thread/ms 118, render/busy/ms 117, pipelines/driver_ms 83.62, stage/render/prepare_assets 76.04, pipelines/wait_ms 46.77, stage/render/render 35.24, pipelines/lock_ms 34.52, render/graph/ms 33.72 |
| warm-up | 4.06 | 71.9 | pipelines/compile_ms 2278, pipelines/front_ms 2191, render/thread/ms 120, render/busy/ms 118, pipelines/driver_ms 86.84, stage/render/prepare_assets 69.02, main/busy/ms 64.62, stage/main/update 53.44, stage/render/render 47.65, pipelines/wait_ms 46.65, render/graph/ms 44.96, stream/drive/ms 33.30 |
| warm-up | 4.11 | 29.4 | pipelines/compile_ms 311, pipelines/front_ms 299, main/busy/ms 70.23, render/thread/ms 38.37, stage/main/post_update 37.96, render/busy/ms 33.51, stage/main/update 24.58, pipelines/wait_ms 17.76, stage/render/render 14.00, stream/drive/ms 13.71, pipelines/driver_ms 12.18, stage/render/prepare_bind_groups 9.63 |
| warm-up | 4.20 | 82.1 | pipelines/compile_ms 1055, pipelines/front_ms 1017, pipelines/wait_ms 157, main/busy/ms 70.71, render/thread/ms 46.24, render/busy/ms 41.42, stage/main/post_update 38.80, pipelines/driver_ms 38.29, stage/main/update 24.27, stage/render/render 15.31, stage/render/prepare_assets 14.93, stream/drive/ms 13.48 |
| warm-up | 4.58 | 91.0 | pipelines/compile_ms 1315, pipelines/front_ms 1266, main/busy/ms 100, render/thread/ms 75.83, stage/main/post_update 69.76, render/busy/ms 66.87, stage/render/render 49.05, pipelines/driver_ms 49.02, render/graph/ms 39.79, stage/render/graph_submit 37.97, encode/finish_wall 30.79, pipelines/wait_ms 22.80 |
| warm-up | 4.64 | 65.7 | pipelines/compile_ms 1471, pipelines/front_ms 1413, main/busy/ms 127, render/thread/ms 119, render/busy/ms 101, stage/render/render 73.12, stage/main/post_update 59.26, stage/main/update 59.08, pipelines/driver_ms 57.75, pipelines/wait_ms 55.97, render/graph/ms 52.79, stage/render/graph_submit 46.31 |
| warm-up | 4.71 | 65.9 | pipelines/compile_ms 1559, pipelines/front_ms 1507, main/busy/ms 114, render/thread/ms 96.76, stage/main/update 89.54, render/busy/ms 85.11, pipelines/wait_ms 64.94, pipelines/driver_ms 51.49, stage/render/render 37.53, stream/drive/ms 34.92, stage/render/prepare_assets 34.15, render/graph/ms 24.03 |
| warm-up | 4.75 | 44.1 | pipelines/compile_ms 1356, pipelines/front_ms 1304, main/busy/ms 76.06, render/thread/ms 66.08, render/busy/ms 63.33, pipelines/driver_ms 51.64, stage/main/update 51.29, stream/drive/ms 37.18, stage/render/prepare_assets 32.92, pipelines/wait_ms 32.26, stage/render/render 20.02, render/graph/ms 16.88 |
| warm-up | 4.79 | 42.9 | pipelines/compile_ms 416, pipelines/front_ms 392, render/thread/ms 55.95, render/busy/ms 53.76, main/busy/ms 46.66, stage/main/update 24.53, pipelines/driver_ms 24.10, stage/render/render 21.11, stage/render/prepare_assets 19.97, render/graph/ms 18.20, stage/main/post_update 15.46, encode//sum 13.50 |
| warm-up | 4.85 | 58.8 | pipelines/compile_ms 109, pipelines/front_ms 105, render/thread/ms 60.94, render/busy/ms 59.51, main/busy/ms 44.93, stage/render/prepare_assets 34.57, stage/main/update 26.08, stage/render/render 13.69, stream/drive/ms 12.98, render/graph/ms 11.66, stage/main/post_update 11.05, stage/render/graph_submit 7.86 |
| warm-up | 4.90 | 43.5 | pipelines/compile_ms 109, pipelines/front_ms 105, render/thread/ms 58.03, render/busy/ms 56.91, main/busy/ms 48.98, stage/render/prepare_assets 39.68, stage/main/update 32.72, stream/drive/ms 21.01, stage/render/render 9.47, stage/main/post_update 8.17, render/graph/ms 7.94, stage/render/graph_submit 5.27 |
| warm-up | 4.93 | 31.4 | main/busy/ms 57.87, render/thread/ms 55.95, render/busy/ms 53.81, stage/main/update 29.23, stage/render/render 22.74, stream/drive/ms 20.52, render/graph/ms 20.23, stage/render/prepare_assets 20.06, stage/main/post_update 15.22, stage/render/graph_submit 13.51, encode/finish_wall 8.59, stage/main/last 7.43 |
| warm-up | 4.97 | 43.1 | render/thread/ms 67.36, render/busy/ms 65.18, main/busy/ms 59.59, stage/render/render 23.50, stage/main/post_update 22.91, render/graph/ms 20.96, stage/render/graph_submit 14.00, stage/main/update 12.37, stage/render/prepare_write_phase_buffers 11.98, stage/main/first 11.14, encode/finish_wall 8.85, stage/main/last 7.81 |
| warm-up | 5.00 | 27.8 | pipelines/compile_ms 159, pipelines/front_ms 149, render/thread/ms 66.00, render/busy/ms 64.81, main/busy/ms 53.12, stage/render/prepare_write_phase_buffers 22.42, stage/main/update 17.24, stage/main/post_update 16.75, stage/render/render 12.83, stage/main/first 11.41, render/graph/ms 11.10, pipelines/driver_ms 10.53 |
| warm-up | 5.03 | 27.3 | pipelines/compile_ms 494, pipelines/front_ms 468, render/thread/ms 47.70, render/busy/ms 46.50, main/busy/ms 32.06, pipelines/driver_ms 26.11, stage/main/update 15.84, stage/render/render 12.61, stage/render/prepare_write_phase_buffers 12.01, render/graph/ms 10.87, stage/main/post_update 9.57, stage/render/prepare_assets 9.07 |
| warm-up | 5.07 | 25.9 | pipelines/compile_ms 1038, pipelines/front_ms 991, pipelines/driver_ms 47.32, render/thread/ms 44.93, render/busy/ms 43.46, main/busy/ms 28.98, stage/render/render 17.99, stage/main/post_update 17.01, render/graph/ms 16.09, stage/render/graph_submit 12.37, stage/main/update 8.32, encode/finish_wall 8.15 |
| warm-up | 5.11 | 34.7 | pipelines/compile_ms 53.35, pipelines/front_ms 51.03, render/thread/ms 44.32, render/busy/ms 42.94, main/busy/ms 27.67, stage/main/post_update 15.39, stage/render/render 14.98, render/graph/ms 13.13, stage/render/prepare_assets 10.97, stage/render/graph_submit 9.35, stage/main/update 8.34, encode//sum 6.75 |
| warm-up | 5.15 | 42.4 | pipelines/compile_ms 425, pipelines/front_ms 409, render/thread/ms 51.67, render/busy/ms 50.47, main/busy/ms 36.95, stage/main/post_update 19.70, stage/render/prepare_assets 15.92, pipelines/driver_ms 15.90, stage/render/render 13.74, render/graph/ms 11.97, stage/main/update 10.29, stage/render/prepare_flush 9.52 |
| warm-up | 5.23 | 36.8 | pipelines/compile_ms 576, pipelines/front_ms 560, main/busy/ms 48.15, stage/main/update 32.63, render/thread/ms 31.67, render/busy/ms 30.05, pipelines/driver_ms 15.49, stage/render/render 12.59, stage/main/post_update 11.74, render/graph/ms 10.56, encode//sum 8.01, stage/render/prepare_assets 7.64 |
| warm-up | 5.27 | 39.9 | pipelines/compile_ms 342, pipelines/front_ms 331, render/thread/ms 45.58, main/busy/ms 45.26, render/busy/ms 44.16, stage/main/update 28.19, stage/render/prepare_assets 23.47, stage/render/render 11.06, pipelines/driver_ms 10.40, render/graph/ms 9.18, stage/main/post_update 8.74, encode//sum 6.97 |
| warm-up | 5.31 | 35.5 | render/thread/ms 44.92, render/busy/ms 44.15, stage/render/prepare_assets 31.22, main/busy/ms 29.78, stage/main/update 15.24, stage/render/render 6.74, stage/main/post_update 6.29, render/graph/ms 5.60, encode//sum 4.61, stage/render/graph_submit 3.50, stage/main/pre_update 3.30, stage/main/fixed_main_loop 3.19 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=8 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=atlas8-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 19%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **96%** (min 88%) · 2625 MHz (min 2625) · memory 8101 MHz · 101.3 W · 73 °C (max 77) · VRAM 5160 MiB · throttled 100% of samples

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
