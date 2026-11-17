# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); stream.interior.disable_exterior=on (command line: --set); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1566 frames over 11.4 s · **median 6.72 ms (149 fps)** · mean 7.30 · p95 9.84 · p99 10.82 (1% low 92 fps) · worst 15.23 ms · 47 spikes over 1.5× median · 63.0 stutters/min (work 63.0/min)
- warm-up: 120 frames over 6.4 s, worst 2580.8 ms
- hitches (> 25.0 ms): **0** measured, 39 in warm-up
- timed GPU passes: 5.18 ms of a 7.30 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 148.8, target 144
- pass worst frame 15.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.981 | 2.028 | 2.067 | 0.98 | 0.168 |
| main_opaque_pass_3d | 1.815 | 1.858 | 2.549 | 0.98 | 0.034 |
| bloom | 0.314 | 0.322 | 0.327 | 0.98 | 0.063 |
| auxiliary_views | 0.285 | 0.292 | 0.936 | 0.98 | 0.298 |
| ssao | 0.185 | 0.190 | 0.194 | 0.98 | 0.007 |
| upscaling | 0.148 | 0.152 | 0.173 | 0.98 | 0.004 |
| tlas_build | 0.111 | 0.114 | 0.118 | 0.98 | 0.070 |
| tonemapping | 0.097 | 0.100 | 0.105 | 0.98 | 0.004 |
| gi/probe_update | 0.090 | 0.092 | 0.098 | 0.98 | 0.002 |
| early prepass | 0.076 | 0.078 | 0.096 | 0.98 | 0.035 |
| clustering | 0.070 | 0.072 | 0.075 | 0.98 | 0.019 |
| uniform_allocation | 0.035 | 0.035 | 0.044 | 0.98 | 0.011 |
| bin_unpacking | 0.031 | 0.031 | 0.039 | 0.98 | 0.009 |
| early_mesh_preprocessing | 0.013 | 0.013 | 0.014 | 0.98 | 0.004 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.010 | 0.95 | 0.023 |
| setup_tlas_instances | 0.007 | 0.007 | 0.007 | 0.98 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.28 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.98 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.143 | 8.786 | 17.21 | 9619.5 |
| render/busy/ms | 5.700 | 8.292 | 16.02 | 8926.5 |
| main/busy/ms | 5.367 | 6.241 | 7.46 | 8405.4 |
| stage/render/render | 3.890 | 5.686 | 11.60 | 6092.5 |
| render/graph/ms | 3.295 | 5.044 | 9.98 | 5160.7 |
| stage/main/post_update | 2.945 | 3.548 | 5.01 | 4611.8 |
| encode//sum | 2.493 | 4.020 | 8.27 | 3904.4 |
| stage/render/graph_submit | 2.205 | 3.323 | 6.89 | 3453.3 |
| encode/submit | 1.482 | 2.287 | 4.90 | 2321.4 |
| stage/main/update | 1.249 | 1.502 | 2.55 | 1956.0 |
| encode/pass:shadow_point_light/sum | 1.012 | 3.161 | 5.69 | 1585.0 |
| stage/render/camera_driver | 0.965 | 1.667 | 2.82 | 1511.8 |
| encode/finish_wall | 0.744 | 1.094 | 5.12 | 1164.6 |
| stage/render/prepare_write_phase_buffers | 0.429 | 0.823 | 1.48 | 672.3 |
| encode/pass:early prepass/sum | 0.413 | 0.571 | 1.10 | 647.1 |
| stage/render/prepare_resources | 0.406 | 0.492 | 0.92 | 635.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.405 | 0.578 | 1.36 | 635.0 |
| render/present/ms | 0.363 | 0.475 | 1.91 | 568.1 |
| stage/main/first | 0.341 | 0.426 | 1.38 | 533.9 |
| stage/main/pre_update | 0.323 | 0.403 | 0.67 | 505.4 |
| stage/render/prepare_bind_groups | 0.296 | 0.629 | 1.09 | 464.3 |
| bigspace/total/ms | 0.257 | 0.611 | 1.00 | 403.1 |
| stage/main/fixed_main_loop | 0.232 | 0.573 | 0.96 | 363.5 |
| stage/render/prepare_assets | 0.207 | 0.257 | 1.33 | 324.2 |
| stage/render/prepare_meshes | 0.206 | 0.272 | 0.46 | 323.1 |
| stage/render/extract_commands | 0.154 | 0.186 | 0.32 | 241.7 |
| stage/render/create_views | 0.147 | 0.184 | 0.54 | 230.3 |
| stream/drive/ms | 0.144 | 0.190 | 0.38 | 224.8 |
| stage/render/prepare_flush | 0.140 | 0.291 | 0.51 | 218.5 |
| stage/render/prepare_views | 0.125 | 0.170 | 0.37 | 195.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.116 | 0.165 | 0.77 | 181.0 |
| stream/tick/ms | 0.115 | 0.160 | 0.36 | 180.7 |
| stage/main/last | 0.106 | 0.138 | 0.98 | 166.7 |
| bigspace/low_precision/ms | 0.100 | 0.290 | 0.91 | 156.2 |
| physics/step/ms | 0.092 | 0.247 | 0.64 | 143.6 |
| render/acquire/ms | 0.080 | 0.109 | 0.33 | 124.9 |
| stage/render/specialize | 0.080 | 0.106 | 0.53 | 124.7 |
| anim/evaluate/ms | 0.073 | 0.232 | 0.76 | 115.1 |
| controller/feed/ms | 0.068 | 0.084 | 0.56 | 107.1 |
| stage/render/prepare_batch_phases | 0.066 | 0.115 | 0.26 | 104.1 |
| stage/render/phase_sort | 0.053 | 0.088 | 0.17 | 83.6 |
| bigspace/high_precision/ms | 0.052 | 0.153 | 0.71 | 80.7 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.071 | 0.26 | 76.9 |
| stage/render/queue | 0.042 | 0.071 | 0.40 | 66.2 |
| anim/evaluate_cpu/ms | 0.035 | 0.051 | 0.43 | 55.1 |
| encode/resolve_diagnostics/sum | 0.033 | 0.044 | 0.50 | 52.0 |
| weather/map/ms | 0.025 | 0.035 | 0.05 | 39.9 |
| stage/render/cleanup | 0.021 | 0.032 | 0.28 | 33.3 |
| far/select/ms | 0.015 | 0.018 | 0.06 | 23.4 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.019 | 0.51 | 20.0 |
| weather/eval/ms | 0.012 | 0.017 | 0.04 | 18.9 |
| physics/insert/ms | 0.011 | 0.000 | 0.61 | 17.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.018 | 0.04 | 16.6 |
| script/update/ms | 0.009 | 0.014 | 0.05 | 13.4 |
| controller/move/ms | 0.006 | 0.008 | 0.20 | 9.9 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.8 |
| anim/apply/ms | 0.005 | 0.010 | 0.04 | 7.5 |
| controller/avoid/ms | 0.003 | 0.004 | 0.18 | 5.5 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 4.9 |
| controller/write/ms | 0.001 | 0.001 | 0.00 | 1.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 1.0 |
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
| encode//max | ms | 0.40 | 5.08 | 0.38 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.51 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.03 | 0.01 |
| encode/finish_critical | ms | 0.51 | 5.08 | 0.40 |
| encode/pass:early prepass/max | ms | 0.42 | 1.00 | 0.40 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.41 | 1.11 | 0.40 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.77 | 0.10 |
| encode/pass:shadow_point_light/max | ms | 0.51 | 1.02 | 0.51 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.50 | 0.02 |
| far/entities | count | 162 | 162 | 162 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.6 k | 41.6 k | 41.6 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.26 M | 86.26 M | 86.26 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.62 | 16.02 | 5.26 |
| gpu/timed/ms | ms | 5.17 | 7.18 | 4.64 |
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
| phases/shadow/batch_sets | count | 0 | 87 | 0 |
| phases/shadow/bins | count | 2638 | 2868 | 2173 |
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
| physics/pending | count | 0 | 107 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 289 | 289 | 289 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 3 | 198 | 0 |
| preprocess/bind_groups_created | count | 3 | 198 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 47 | 103 | 35 |
| process/cpu_usage | % | 23.80 | 37.26 | 24.52 |
| process/mem_usage | GiB | 3.78 | 3.83 | 3.83 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
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
| system/cpu_usage | % | 30.29 | 39.53 | 30.44 |
| system/mem_usage | % | 57.05 | 57.22 | 57.21 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2038 | 2038 | 2038 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 18 | 102 | 9 |
| views/all/bins | count | 2970 | 3305 | 2223 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 4157 | 4589 | 3183 |
| views/all/pipelines | count | 247 | 261 | 198 |
| views/all/pipelines_distinct | count | 22 | 28 | 16 |
| views/all/work_items | count | 568 | 2411 | 78 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 87 | 0 |
| views/lamp_face/bins | count | 2638 | 2868 | 2173 |
| views/lamp_face/count | count | 48 | 48 | 48 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 15 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3600 | 3869 | 3105 |
| views/lamp_face/pipelines | count | 227 | 240 | 189 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 1400 | 0 |
| views/main/batch_sets | count | 15 | 21 | 9 |
| views/main/bins | count | 290 | 574 | 50 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 518 | 1054 | 78 |
| views/main/pipelines | count | 15 | 21 | 9 |
| views/main/work_items | count | 539 | 1095 | 78 |
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
| anim/bones_written/count | count | 8.32 | 30 | 13.0 k |
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
| encode//count | count | 30.92 | 113 | 48.4 k |
| encode/bind_walk/init_textures | count | 8574 | 35.0 k | 13.43 M |
| encode/bind_walk/init_textures_skipped | count | 17.9 k | 101.4 k | 27.97 M |
| encode/bind_walk/inits | count | 194 | 717 | 304.4 k |
| encode/bind_walk/inits_skipped | count | 20.68 | 112 | 32.4 k |
| encode/bind_walk/merge_textures | count | 7331 | 32.5 k | 11.48 M |
| encode/bind_walk/merge_textures_skipped | count | 32.26 | 126 | 50.5 k |
| encode/bind_walk/merges | count | 59.73 | 206 | 93.5 k |
| encode/bind_walk/merges_skipped | count | 8.01 | 40 | 12.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1566 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1566 |
| encode/pass:early prepass/count | count | 1 | 2 | 1566 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1566 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1521 |
| encode/pass:shadow_point_light/count | count | 3.16 | 19 | 4943 |
| encode/queue/command_buffers | count | 43.05 | 150 | 67.4 k |
| encode/queue/submits | count | 2 | 4 | 3132 |
| encode/queue/write_buffer/bytes | bytes | 474.2 k | 1.04 M | 742.64 M |
| encode/queue/write_buffer/calls | count | 149 | 599 | 233.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 158.0 k | 315.9 k | 247.35 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9396 |
| encode/queue/write_texture/bytes | bytes | 11.4 k | 8.39 M | 17.83 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1566 |
| physics/inserts | count | 0.07 | 10 | 110 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.44 | 1 | 686 |
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

## Hitches (> 25.0 ms; slowest 39, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2580.8 | main/busy/ms 2551, stage/main/post_update 2445, stage/main/update 97.57, surface/advance_cpu_ms 67.56, stage/main/pre_update 4.76, stage/main/last 1.04, stage/main/first 0.66, stream/drive/ms 0.49, stage/main/fixed_main_loop 0.38, weather/map/ms 0.17, bigspace/total/ms 0.09, stream/tick/ms 0.08 |
| warm-up | 3.55 | 971.5 | main/busy/ms 2671, stage/main/post_update 2449, render/thread/ms 939, render/busy/ms 753, stage/render/prepare_assets 719, stage/main/update 207, stage/render/prepare_views 183, render/acquire/ms 181, surface/advance_cpu_ms 67.56, stage/render/prepare_resources 20.91, stage/render/render 13.48, render/graph/ms 7.52 |
| warm-up | 3.64 | 86.8 | render/thread/ms 1010, render/busy/ms 777, stage/render/prepare_assets 738, stage/render/prepare_views 183, render/acquire/ms 182, main/busy/ms 130, stage/main/update 113, stage/render/render 63.39, render/present/ms 51.31, stage/render/prepare_resources 21.52, render/graph/ms 11.24, stage/main/post_update 7.67 |
| warm-up | 3.68 | 38.6 | pipelines/compile_ms 98.51, render/thread/ms 92.51, pipelines/front_ms 89.82, pipelines/wait_ms 79.19, stage/render/render 68.12, main/busy/ms 58.89, render/present/ms 53.39, stage/main/update 41.20, render/busy/ms 38.25, stage/render/prepare_assets 19.78, stage/render/cleanup 15.33, render/graph/ms 14.31 |
| warm-up | 3.72 | 38.9 | pipelines/wait_ms 1288, pipelines/compile_ms 518, pipelines/front_ms 485, main/busy/ms 60.81, render/thread/ms 46.23, stage/main/update 39.94, render/busy/ms 37.22, pipelines/driver_ms 32.67, pipelines/lock_ms 26.81, stage/render/render 21.88, stage/main/post_update 15.75, stage/render/cleanup 15.33 |
| warm-up | 3.75 | 28.6 | pipelines/wait_ms 1253, pipelines/compile_ms 523, pipelines/front_ms 493, render/thread/ms 52.32, render/busy/ms 49.84, pipelines/driver_ms 29.19, main/busy/ms 23.26, pipelines/lock_ms 22.55, stage/main/post_update 14.69, stage/render/prepare_resources 12.12, stage/render/prepare_bind_groups 11.93, stage/render/prepare_assets 11.28 |
| warm-up | 3.78 | 36.8 | pipelines/compile_ms 627, pipelines/front_ms 602, pipelines/wait_ms 81.28, main/busy/ms 71.75, render/thread/ms 61.68, render/busy/ms 59.34, stage/main/update 57.30, pipelines/driver_ms 24.49, stage/render/prepare_bind_groups 17.42, stage/render/render 16.77, render/graph/ms 14.21, stage/render/prepare_assets 12.57 |
| warm-up | 3.87 | 87.0 | pipelines/compile_ms 1218, pipelines/front_ms 1168, render/thread/ms 91.43, pipelines/wait_ms 89.60, render/busy/ms 87.36, main/busy/ms 73.91, stage/main/update 60.41, stage/render/prepare_assets 52.57, pipelines/driver_ms 49.94, stage/render/render 24.80, render/graph/ms 20.36, stage/render/graph_submit 11.17 |
| warm-up | 3.92 | 31.8 | pipelines/compile_ms 394, pipelines/front_ms 381, render/thread/ms 42.04, render/busy/ms 40.51, stage/render/prepare_assets 30.00, main/busy/ms 15.97, pipelines/driver_ms 13.29, pipelines/wait_ms 11.32, pipelines/lock_ms 9.18, stage/render/render 8.00, render/graph/ms 6.30, stage/main/update 6.09 |
| warm-up | 4.01 | 49.7 | pipelines/compile_ms 863, pipelines/front_ms 833, render/thread/ms 51.94, render/busy/ms 49.58, main/busy/ms 37.94, pipelines/driver_ms 30.02, stage/render/prepare_assets 27.73, stage/main/update 21.10, stage/render/render 16.71, stream/drive/ms 15.20, render/graph/ms 14.28, stage/main/post_update 8.82 |
| warm-up | 4.05 | 31.0 | pipelines/compile_ms 885, pipelines/front_ms 848, render/thread/ms 52.83, render/busy/ms 51.10, pipelines/driver_ms 36.36, stage/render/prepare_assets 33.21, main/busy/ms 21.72, stage/render/render 10.47, stage/main/post_update 8.85, stage/main/update 8.79, render/graph/ms 8.59, pipelines/wait_ms 5.91 |
| warm-up | 4.11 | 44.3 | pipelines/compile_ms 424, pipelines/front_ms 397, main/busy/ms 73.74, render/thread/ms 53.72, render/busy/ms 52.48, stage/main/update 49.66, stream/drive/ms 41.90, stage/render/prepare_assets 30.39, pipelines/driver_ms 26.66, stage/main/post_update 14.58, stage/render/prepare_bind_groups 7.18, stage/render/prepare_resources 7.03 |
| warm-up | 4.16 | 47.9 | pipelines/compile_ms 1373, pipelines/front_ms 1330, main/busy/ms 69.07, pipelines/wait_ms 51.18, render/thread/ms 49.65, render/busy/ms 48.37, stage/main/update 47.04, pipelines/driver_ms 43.24, stream/drive/ms 41.92, stage/render/prepare_assets 25.77, stage/main/post_update 13.87, stage/render/prepare_bind_groups 7.01 |
| warm-up | 4.22 | 42.0 | pipelines/compile_ms 529, pipelines/front_ms 502, main/busy/ms 49.98, pipelines/wait_ms 34.08, pipelines/driver_ms 27.01, stage/main/post_update 23.88, render/thread/ms 22.89, render/busy/ms 21.36, stage/main/update 19.58, stream/drive/ms 14.28, stage/render/prepare_assets 11.31, stage/render/render 7.27 |
| warm-up | 4.28 | 61.9 | pipelines/compile_ms 987, pipelines/front_ms 949, pipelines/wait_ms 239, render/thread/ms 56.22, render/busy/ms 54.88, main/busy/ms 45.60, stage/render/prepare_assets 40.30, pipelines/driver_ms 38.27, stage/main/post_update 23.35, stage/main/update 18.85, stage/render/render 7.33, render/graph/ms 5.82 |
| warm-up | 4.31 | 28.9 | pipelines/compile_ms 1007, pipelines/front_ms 978, pipelines/wait_ms 211, render/thread/ms 71.32, render/busy/ms 70.01, main/busy/ms 48.20, stage/render/prepare_assets 41.91, pipelines/driver_ms 29.32, stage/main/post_update 26.86, stage/main/update 17.46, stage/render/prepare_resources 9.51, stage/render/render 8.30 |
| warm-up | 4.35 | 38.1 | pipelines/compile_ms 1017, pipelines/front_ms 978, render/thread/ms 61.86, render/busy/ms 54.18, pipelines/wait_ms 52.20, main/busy/ms 50.38, pipelines/driver_ms 39.50, stage/render/prepare_resources 21.95, stage/render/render 17.75, stage/main/pre_update 16.07, stage/main/update 15.78, stage/main/post_update 15.23 |
| warm-up | 4.46 | 31.3 | pipelines/compile_ms 405, pipelines/front_ms 392, pipelines/wait_ms 49.36, render/thread/ms 45.84, render/busy/ms 44.43, main/busy/ms 32.09, stage/main/update 21.66, pipelines/driver_ms 12.94, stage/render/prepare_resources 12.14, stage/render/prepare_assets 11.01, stage/render/render 8.36, stage/main/post_update 7.98 |
| warm-up | 4.60 | 26.6 | pipelines/compile_ms 318, pipelines/front_ms 309, render/thread/ms 31.89, render/busy/ms 30.41, main/busy/ms 26.39, pipelines/wait_ms 19.23, stage/render/render 15.03, render/graph/ms 13.45, stage/main/post_update 10.03, stage/render/camera_driver 10.02, pipelines/driver_ms 8.51, stage/main/update 6.96 |
| warm-up | 4.68 | 70.8 | pipelines/compile_ms 1088, pipelines/front_ms 1048, main/busy/ms 86.90, stage/main/update 54.69, stream/drive/ms 49.01, pipelines/driver_ms 40.44, render/thread/ms 31.17, render/busy/ms 24.54, stage/main/post_update 22.60, stage/render/render 15.09, render/graph/ms 8.21, pipelines/wait_ms 7.89 |
| warm-up | 4.73 | 44.5 | pipelines/compile_ms 1175, pipelines/front_ms 1132, main/busy/ms 117, stage/main/update 82.14, stream/drive/ms 51.37, render/thread/ms 45.72, pipelines/driver_ms 42.71, render/busy/ms 38.72, stage/main/post_update 24.78, pipelines/wait_ms 19.18, stage/render/render 18.70, stage/render/prepare_assets 16.09 |
| warm-up | 4.86 | 130.5 | pipelines/compile_ms 1596, pipelines/front_ms 1544, main/busy/ms 101, stage/main/update 83.79, render/thread/ms 71.63, render/busy/ms 69.52, pipelines/driver_ms 52.67, stream/drive/ms 51.05, stage/render/prepare_assets 39.70, pipelines/wait_ms 34.03, pipelines/lock_ms 32.73, stage/render/render 17.72 |
| warm-up | 4.89 | 36.6 | pipelines/compile_ms 1382, pipelines/front_ms 1336, render/thread/ms 79.49, render/busy/ms 77.43, encode//sum 50.19, pipelines/driver_ms 46.74, stage/render/prepare_assets 32.00, main/busy/ms 29.77, stage/render/render 29.13, render/graph/ms 26.59, pipelines/wait_ms 22.80, pipelines/lock_ms 21.87 |
| warm-up | 4.92 | 28.8 | encode//sum 70.83, render/thread/ms 61.04, render/busy/ms 59.04, stage/render/render 34.36, render/graph/ms 32.00, main/busy/ms 27.51, stage/render/graph_submit 22.59, encode/pass:shadow_point_light/sum 15.87, pipelines/compile_ms 14.33, pipelines/front_ms 12.90, encode/submit 12.23, encode/finish_wall 10.38 |
| warm-up | 4.98 | 52.3 | pipelines/compile_ms 434, pipelines/front_ms 415, render/thread/ms 65.96, render/busy/ms 64.11, main/busy/ms 43.30, encode//sum 35.19, stage/render/render 23.73, stage/main/update 21.69, render/graph/ms 21.49, stage/render/prepare_assets 19.71, pipelines/driver_ms 18.92, stage/main/post_update 14.37 |
| warm-up | 5.03 | 51.5 | pipelines/compile_ms 434, pipelines/front_ms 415, render/thread/ms 80.73, render/busy/ms 79.18, main/busy/ms 47.69, stage/render/prepare_assets 31.33, stage/main/update 28.04, pipelines/driver_ms 18.92, stage/render/render 17.23, encode//sum 16.04, render/graph/ms 15.22, stage/main/post_update 13.04 |
| warm-up | 5.11 | 80.0 | encode//sum 167, render/thread/ms 115, render/busy/ms 112, main/busy/ms 88.34, stage/main/update 69.76, stage/render/render 60.15, render/graph/ms 56.83, stream/drive/ms 56.56, stage/render/graph_submit 44.37, encode/pass:shadow_point_light/sum 33.57, encode/finish_wall 27.35, stage/render/prepare_assets 21.32 |
| warm-up | 5.17 | 62.9 | encode//sum 185, pipelines/compile_ms 177, pipelines/front_ms 166, render/thread/ms 123, render/busy/ms 120, main/busy/ms 89.21, stage/render/render 74.70, render/graph/ms 71.04, stage/main/update 64.10, stream/drive/ms 56.72, stage/render/graph_submit 53.97, encode/pass:shadow_point_light/sum 38.94 |
| warm-up | 5.22 | 52.8 | pipelines/compile_ms 1218, pipelines/front_ms 1176, render/thread/ms 91.15, render/busy/ms 89.50, main/busy/ms 54.00, pipelines/driver_ms 41.91, encode//sum 39.57, stage/render/render 38.10, render/graph/ms 35.85, stage/main/update 30.53, stage/render/prepare_assets 24.22, stream/drive/ms 22.65 |
| warm-up | 5.27 | 43.4 | pipelines/compile_ms 1054, pipelines/front_ms 1022, render/thread/ms 77.84, render/busy/ms 75.76, main/busy/ms 50.12, stage/render/render 34.92, render/graph/ms 32.37, pipelines/driver_ms 31.43, stage/main/update 31.36, encode//sum 25.62, stream/drive/ms 23.01, stage/render/graph_submit 21.21 |
| warm-up | 5.32 | 54.5 | render/thread/ms 86.73, render/busy/ms 84.34, main/busy/ms 64.41, stage/render/render 47.50, render/graph/ms 44.55, pipelines/compile_ms 39.83, pipelines/front_ms 38.32, stage/main/post_update 35.33, stage/render/graph_submit 23.73, stage/main/update 22.50, stage/render/camera_driver 20.11, stage/render/prepare_assets 19.36 |
| warm-up | 5.40 | 83.3 | pipelines/compile_ms 951, pipelines/front_ms 913, render/thread/ms 97.10, render/busy/ms 95.22, main/busy/ms 64.89, stage/render/prepare_assets 44.00, pipelines/driver_ms 38.89, stage/render/render 36.81, render/graph/ms 34.27, stage/main/post_update 34.26, stage/main/update 22.39, stage/render/camera_driver 18.60 |
| warm-up | 5.47 | 65.0 | pipelines/compile_ms 1498, pipelines/front_ms 1444, render/thread/ms 84.71, render/busy/ms 82.58, stage/render/prepare_assets 57.34, pipelines/driver_ms 53.35, main/busy/ms 31.93, stage/render/render 17.11, render/graph/ms 14.40, stage/main/update 10.81, stage/main/post_update 10.50, encode//sum 9.95 |
| warm-up | 5.53 | 45.7 | pipelines/compile_ms 573, pipelines/front_ms 556, render/thread/ms 41.40, render/busy/ms 38.61, main/busy/ms 37.90, stage/render/render 24.39, stage/main/update 22.88, render/graph/ms 22.39, stage/render/graph_submit 17.97, pipelines/driver_ms 17.47, encode/submit 15.56, encode//sum 10.93 |
| warm-up | 5.58 | 26.5 | pipelines/compile_ms 302, pipelines/front_ms 294, main/busy/ms 36.98, render/thread/ms 32.65, render/busy/ms 31.24, stage/main/update 24.16, stream/drive/ms 16.85, stage/render/render 14.05, render/graph/ms 11.97, encode//sum 11.32, pipelines/driver_ms 8.62, stage/main/post_update 7.99 |
| warm-up | 5.65 | 27.2 | render/thread/ms 36.54, render/busy/ms 35.54, main/busy/ms 18.26, stage/render/prepare_assets 14.74, stage/render/render 13.70, render/graph/ms 12.30, encode//sum 9.43, stage/main/update 7.92, stage/render/graph_submit 7.60, stage/main/post_update 6.82, encode/pass:shadow_point_light/sum 6.70, encode/submit 5.03 |
| warm-up | 5.72 | 25.7 | render/thread/ms 35.48, render/busy/ms 34.43, main/busy/ms 29.15, stage/render/render 13.82, render/graph/ms 12.34, stage/render/prepare_assets 12.08, stage/main/post_update 10.65, encode//sum 10.51, stage/main/update 9.72, stage/render/graph_submit 7.94, stage/main/pre_update 6.31, encode/pass:shadow_point_light/sum 5.68 |
| warm-up | 5.85 | 27.5 | render/thread/ms 39.24, render/busy/ms 37.48, main/busy/ms 32.70, stage/render/render 24.34, render/graph/ms 21.91, stage/main/post_update 16.03, stage/render/graph_submit 14.68, encode//sum 14.43, encode/submit 10.79, stage/main/update 10.76, encode/pass:shadow_point_light/sum 10.22, stage/render/camera_driver 6.61 |
| warm-up | 5.88 | 31.5 | render/thread/ms 44.42, render/busy/ms 43.21, main/busy/ms 36.23, stage/render/render 17.23, stage/render/prepare_assets 16.62, render/graph/ms 15.69, stage/main/update 14.53, stage/main/post_update 13.81, encode//sum 11.32, stage/render/graph_submit 10.73, encode/submit 7.86, encode/pass:shadow_point_light/sum 6.98 |

## Run

- `target/profiling/perftests.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set stream.interior.disable_exterior=on --resolution 2560x1440 --bench-out target/perf\pt_census\census@2560x1440_arm=park-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 6%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (220 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **78%** (min 65%) · 2625 MHz (min 2625) · memory 8101 MHz · 89.9 W · 73 °C (max 77) · VRAM 5428 MiB · throttled 100% of samples

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
- `stream.interior.disable_exterior=on (command line: --set)`
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
