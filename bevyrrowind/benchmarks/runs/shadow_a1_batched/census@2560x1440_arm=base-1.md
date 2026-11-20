# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1490 frames over 11.4 s · **median 7.22 ms (139 fps)** · mean 7.67 · p95 10.66 · p99 14.94 (1% low 67 fps) · worst 17.18 ms · 72 spikes over 1.5× median · 26.2 stutters/min (work 15.7/min)
- warm-up: 120 frames over 6.6 s, worst 2557.5 ms
- hitches (> 25.0 ms): **0** measured, 41 in warm-up
- timed GPU passes: 5.37 ms of a 7.67 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median fps 138.5, target 144
- pass worst frame 17.18 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.019 | 2.185 | 2.541 | 0.92 | 0.181 |
| main_opaque_pass_3d | 1.804 | 1.952 | 2.866 | 0.92 | 0.041 |
| bloom | 0.322 | 0.348 | 0.672 | 0.92 | 0.068 |
| auxiliary_views | 0.318 | 0.345 | 1.129 | 0.92 | 0.307 |
| ssao | 0.202 | 0.218 | 0.544 | 0.92 | 0.008 |
| upscaling | 0.154 | 0.167 | 0.212 | 0.92 | 0.005 |
| tonemapping | 0.106 | 0.115 | 0.139 | 0.92 | 0.004 |
| tlas_build | 0.106 | 0.115 | 0.119 | 0.92 | 0.079 |
| gpu_instance_materialization | 0.089 | 0.096 | 0.099 | 0.92 | 0.004 |
| gi/probe_update | 0.089 | 0.096 | 0.100 | 0.92 | 0.002 |
| early prepass | 0.070 | 0.076 | 0.085 | 0.92 | 0.049 |
| clustering | 0.068 | 0.074 | 0.077 | 0.92 | 0.020 |
| uniform_allocation | 0.035 | 0.038 | 0.044 | 0.92 | 0.010 |
| bin_unpacking | 0.028 | 0.031 | 0.039 | 0.92 | 0.008 |
| range_unpacking | 0.018 | 0.019 | 0.020 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.017 | 0.018 | 0.025 | 0.92 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.012 | 0.90 | 0.025 |
| setup_tlas_instances | 0.006 | 0.007 | 0.008 | 0.92 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.30 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.92 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 19.873 | 0.000 | 5995.12 | 29610.7 |
| pipelines/driver_ms | 17.402 | 0.000 | 5391.87 | 25929.5 |
| render/thread/ms | 6.629 | 9.687 | 15.99 | 9877.9 |
| render/busy/ms | 6.130 | 9.122 | 14.44 | 9133.9 |
| pipelines/wait_ms | 5.902 | 0.000 | 1831.51 | 8794.1 |
| main/busy/ms | 5.780 | 6.994 | 11.70 | 8612.2 |
| stage/render/render | 4.116 | 6.176 | 12.47 | 6132.6 |
| render/graph/ms | 3.456 | 5.478 | 9.71 | 5149.2 |
| stage/main/post_update | 3.055 | 3.820 | 6.33 | 4551.8 |
| encode//sum | 2.685 | 4.422 | 7.32 | 4000.9 |
| pipelines/front_ms | 2.471 | 0.000 | 603.25 | 3681.2 |
| stage/render/graph_submit | 2.249 | 3.623 | 6.69 | 3351.3 |
| encode/submit | 1.565 | 2.659 | 4.71 | 2332.5 |
| stage/main/update | 1.372 | 1.771 | 3.26 | 2044.3 |
| stage/render/camera_driver | 1.066 | 1.779 | 2.89 | 1588.6 |
| encode/pass:shadow_point_light/sum | 0.913 | 2.945 | 5.26 | 1360.9 |
| encode/finish_wall | 0.723 | 1.132 | 2.03 | 1077.2 |
| stage/render/prepare_resources | 0.428 | 0.577 | 0.84 | 638.5 |
| stage/render/prepare_write_phase_buffers | 0.424 | 0.853 | 1.69 | 632.5 |
| render/present/ms | 0.408 | 0.770 | 7.12 | 608.0 |
| encode/pass:early prepass/sum | 0.402 | 0.582 | 0.93 | 599.4 |
| encode/pass:main_opaque_pass_3d/sum | 0.381 | 0.558 | 1.42 | 567.0 |
| stage/main/pre_update | 0.369 | 0.500 | 0.89 | 549.2 |
| stage/main/first | 0.361 | 0.490 | 1.23 | 538.0 |
| stage/render/prepare_bind_groups | 0.340 | 0.725 | 1.57 | 506.6 |
| stage/main/fixed_main_loop | 0.281 | 0.652 | 1.12 | 418.5 |
| bigspace/total/ms | 0.237 | 0.523 | 2.94 | 353.1 |
| stage/render/prepare_assets | 0.227 | 0.346 | 0.59 | 338.2 |
| stage/render/prepare_meshes | 0.221 | 0.312 | 0.76 | 329.6 |
| stage/render/prepare_flush | 0.187 | 0.359 | 0.67 | 278.8 |
| stage/render/create_views | 0.165 | 0.228 | 0.68 | 245.4 |
| stage/render/extract_commands | 0.163 | 0.209 | 0.37 | 242.8 |
| stage/main/last | 0.160 | 0.223 | 1.39 | 238.4 |
| stage/render/prepare_views | 0.142 | 0.201 | 1.08 | 211.5 |
| stream/drive/ms | 0.136 | 0.198 | 0.49 | 202.6 |
| stage/render/specialize | 0.134 | 0.194 | 0.79 | 200.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.116 | 0.174 | 0.62 | 172.2 |
| stream/tick/ms | 0.107 | 0.164 | 0.45 | 159.6 |
| bigspace/low_precision/ms | 0.096 | 0.233 | 2.78 | 143.5 |
| render/acquire/ms | 0.091 | 0.135 | 1.03 | 136.0 |
| stage/render/queue | 0.088 | 0.130 | 0.54 | 131.6 |
| stage/render/prepare_batch_phases | 0.088 | 0.151 | 0.52 | 130.6 |
| physics/step/ms | 0.087 | 0.266 | 0.66 | 129.5 |
| controller/feed/ms | 0.086 | 0.114 | 0.36 | 127.6 |
| anim/evaluate/ms | 0.069 | 0.145 | 0.96 | 102.8 |
| bigspace/high_precision/ms | 0.055 | 0.211 | 0.66 | 82.4 |
| stage/render/prepare_collect_phase_buffers | 0.051 | 0.078 | 0.27 | 76.5 |
| anim/evaluate_cpu/ms | 0.037 | 0.061 | 0.22 | 54.8 |
| encode/resolve_diagnostics/sum | 0.032 | 0.046 | 0.08 | 48.3 |
| physics/insert/ms | 0.029 | 0.351 | 0.74 | 42.6 |
| weather/map/ms | 0.028 | 0.038 | 0.42 | 42.0 |
| controller/write/ms | 0.028 | 0.041 | 0.11 | 41.4 |
| stage/render/cleanup | 0.023 | 0.036 | 0.21 | 34.9 |
| stage/render/phase_sort | 0.017 | 0.042 | 0.33 | 25.9 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.35 | 19.9 |
| weather/eval/ms | 0.013 | 0.017 | 0.29 | 19.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.21 | 16.8 |
| script/update/ms | 0.008 | 0.013 | 0.05 | 12.6 |
| anim/apply/ms | 0.005 | 0.010 | 0.03 | 7.5 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.2 |
| controller/move/ms | 0.004 | 0.011 | 0.31 | 6.5 |
| far/select/ms | 0.004 | 0.005 | 0.02 | 6.1 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 5.9 |
| controller/avoid/ms | 0.002 | 0.005 | 0.05 | 3.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.14 | 1.3 |
| weather/apply/ms | 0.001 | 0.001 | 0.03 | 1.1 |
| pipelines/lock_ms | 0.001 | 0.000 | 0.52 | 1.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 0.9 |
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
| crowd/joints_extracted/count | count | 46 | 1403 | 0 |
| crowd/joints_uploaded/count | count | 94 | 153 | 94 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.41 | 1.22 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.35 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.21 | 0.01 |
| encode/finish_critical | ms | 0.49 | 1.42 | 0.44 |
| encode/pass:early prepass/max | ms | 0.40 | 0.93 | 0.31 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.38 | 1.42 | 0.44 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.62 | 0.08 |
| encode/pass:shadow_point_light/max | ms | 0.46 | 0.98 | 0.43 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.08 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.99 | 14.44 | 5.42 |
| gpu/timed/ms | ms | 5.65 | 8.19 | 5.16 |
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
| phases/opaque3d/bins | count | 142 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 250 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 142 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 250 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 84 | 0 |
| phases/shadow/bins | count | 2638 | 2864 | 2173 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3603 | 3839 | 3105 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 227 | 240 | 189 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 630 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 268 | 285 | 285 |
| pipelines/waiting | count | 1 | 18 | 1 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 1 | 13 | 1 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 20 | 31 | 5 |
| preprocess/bind_groups | count | 7 | 206 | 4 |
| preprocess/bind_groups_created | count | 3 | 202 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 47 | 109 | 35 |
| process/cpu_usage | % | 31.25 | 72.60 | 32.21 |
| process/mem_usage | GiB | 3.98 | 4.06 | 4.06 |
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
| skin/blocks_changed/count | count | 1 | 24 | 0 |
| skin/blocks_skipped/count | count | 2 | 3 | 2 |
| skin/blocks_walked/count | count | 1 | 2 | 0 |
| skin/joint_bytes_written/bytes | bytes | 2944 | 19.8 k | 0 |
| skin/joints_written/count | count | 46 | 310 | 0 |
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
| stream/latency/s | s | 0.10 | 0.10 | 0.10 |
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
| system/cpu_usage | % | 39.03 | 79.47 | 44.13 |
| system/mem_usage | % | 52.68 | 52.87 | 52.75 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 26 | 113 | 17 |
| views/all/bins | count | 2981 | 3313 | 2231 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 4165 | 4584 | 3191 |
| views/all/pipelines | count | 253 | 267 | 204 |
| views/all/pipelines_distinct | count | 28 | 34 | 22 |
| views/all/work_items | count | 98.9 k | 100.8 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 84 | 0 |
| views/lamp_face/bins | count | 2638 | 2864 | 2173 |
| views/lamp_face/count | count | 48 | 48 | 48 |
| views/lamp_face/culled | count | 25 | 36 | 35 |
| views/lamp_face/drawn | count | 0 | 12 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3603 | 3839 | 3105 |
| views/lamp_face/pipelines | count | 227 | 240 | 189 |
| views/lamp_face/revealed | count | 0 | 1 | 0 |
| views/lamp_face/work_items | count | 0 | 1370 | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 294 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 524 | 1062 | 86 |
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
| anim/bones_written/count | count | 8.10 | 30 | 12.1 k |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.01 | 8 | 8 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 31.26 | 68 | 46.6 k |
| encode/bind_walk/init_textures | count | 8223 | 21.0 k | 12.25 M |
| encode/bind_walk/init_textures_skipped | count | 10.5 k | 28.2 k | 15.62 M |
| encode/bind_walk/inits | count | 215 | 449 | 320.0 k |
| encode/bind_walk/inits_skipped | count | 15.37 | 43 | 22.9 k |
| encode/bind_walk/merge_textures | count | 6980 | 19.8 k | 10.40 M |
| encode/bind_walk/merge_textures_skipped | count | 3827 | 16.9 k | 5.70 M |
| encode/bind_walk/merges | count | 58.65 | 119 | 87.4 k |
| encode/bind_walk/merges_skipped | count | 16.43 | 59 | 24.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1490 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1490 |
| encode/pass:early prepass/count | count | 1 | 1 | 1490 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1490 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1445 |
| encode/pass:shadow_point_light/count | count | 2.98 | 12 | 4445 |
| encode/queue/command_buffers | count | 43.21 | 89 | 64.4 k |
| encode/queue/submits | count | 2 | 2 | 2980 |
| encode/queue/write_buffer/bytes | bytes | 593.9 k | 679.0 k | 884.86 M |
| encode/queue/write_buffer/calls | count | 158 | 381 | 235.1 k |
| encode/queue/write_buffer_with/bytes | bytes | 149.6 k | 149.6 k | 222.95 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 8940 |
| encode/queue/write_texture/bytes | bytes | 704 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1490 |
| physics/inserts | count | 0.32 | 31 | 482 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.46 | 1 | 686 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.03 | 1 | 42 |
| pipelines/created | count | 0.03 | 1 | 45 |
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

## Hitches (> 25.0 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2557.5 | main/busy/ms 2528, stage/main/post_update 2425, stage/main/update 92.38, surface/advance_cpu_ms 66.08, stage/main/pre_update 6.16, stage/main/last 1.14, stage/main/first 0.55, stream/drive/ms 0.42, stage/main/fixed_main_loop 0.38, weather/map/ms 0.10, bigspace/total/ms 0.08, stream/tick/ms 0.06 |
| warm-up | 3.27 | 715.6 | main/busy/ms 2644, stage/main/post_update 2430, render/thread/ms 680, render/busy/ms 544, stage/render/prepare_assets 524, stage/main/update 198, stage/render/prepare_views 135, render/acquire/ms 134, surface/advance_cpu_ms 66.08, stage/render/render 8.28, stage/render/prepare_resources 8.10, stage/main/pre_update 7.37 |
| warm-up | 3.32 | 46.5 | render/thread/ms 712, render/busy/ms 575, stage/render/prepare_assets 550, stage/render/prepare_views 136, render/acquire/ms 134, main/busy/ms 131, stage/main/update 111, stage/render/render 11.89, stage/main/post_update 9.19, stage/render/prepare_resources 8.64, render/graph/ms 8.29, stage/main/fixed_main_loop 7.65 |
| warm-up | 3.36 | 25.6 | pipelines/wait_ms 364, pipelines/compile_ms 236, pipelines/front_ms 215, main/busy/ms 62.70, stage/main/post_update 33.77, render/thread/ms 26.55, pipelines/driver_ms 21.39, render/busy/ms 20.74, stage/main/update 13.12, pipelines/lock_ms 11.58, stage/render/render 11.47, stage/main/last 6.77 |
| warm-up | 3.40 | 41.1 | pipelines/wait_ms 1345, pipelines/compile_ms 510, pipelines/front_ms 473, main/busy/ms 57.34, pipelines/driver_ms 36.99, render/thread/ms 29.76, stage/main/post_update 27.30, render/busy/ms 26.30, stage/main/update 16.80, stage/render/render 11.89, pipelines/lock_ms 10.45, stage/render/prepare_assets 8.28 |
| warm-up | 3.44 | 34.3 | pipelines/wait_ms 1008, pipelines/compile_ms 434, pipelines/front_ms 410, main/busy/ms 48.36, render/thread/ms 45.31, render/busy/ms 36.18, stage/render/render 24.64, pipelines/driver_ms 23.79, render/graph/ms 15.45, stage/main/post_update 14.06, stage/main/fixed_main_loop 12.12, stage/main/update 11.86 |
| warm-up | 3.57 | 91.3 | pipelines/compile_ms 717, pipelines/front_ms 690, pipelines/wait_ms 84.65, main/busy/ms 74.65, render/thread/ms 71.26, render/busy/ms 66.39, stage/main/update 54.04, stage/render/prepare_assets 45.97, pipelines/driver_ms 26.72, stage/render/render 17.13, render/graph/ms 12.18, stage/main/post_update 9.68 |
| warm-up | 3.62 | 33.4 | pipelines/compile_ms 678, pipelines/front_ms 656, render/thread/ms 37.95, render/busy/ms 34.95, main/busy/ms 31.11, pipelines/driver_ms 21.78, stage/render/prepare_assets 21.54, stage/main/post_update 13.62, stage/render/render 10.17, stage/main/update 8.50, render/graph/ms 6.49, stage/render/graph_submit 3.74 |
| warm-up | 3.67 | 32.5 | pipelines/compile_ms 427, pipelines/front_ms 405, render/thread/ms 47.95, render/busy/ms 46.03, main/busy/ms 38.13, stage/main/update 22.75, pipelines/driver_ms 21.75, stage/render/prepare_assets 20.59, stream/drive/ms 15.25, stage/main/post_update 11.45, stage/render/prepare_meshes 9.63, stage/render/render 6.94 |
| warm-up | 3.72 | 46.1 | pipelines/compile_ms 236, pipelines/front_ms 226, render/thread/ms 63.33, render/busy/ms 61.51, main/busy/ms 49.25, stage/render/prepare_assets 35.27, stage/main/update 31.97, stream/drive/ms 15.28, stage/main/post_update 13.14, pipelines/driver_ms 9.99, stage/render/prepare_meshes 9.64, stage/render/render 7.94 |
| warm-up | 3.81 | 90.3 | pipelines/compile_ms 463, pipelines/front_ms 447, main/busy/ms 151, stage/main/update 124, render/thread/ms 116, render/busy/ms 115, stage/render/prepare_assets 41.30, stage/render/prepare_write_phase_buffers 30.53, stage/render/prepare_resources 17.00, pipelines/driver_ms 16.37, stage/render/prepare_bind_groups 13.94, stage/main/pre_update 10.20 |
| warm-up | 3.93 | 122.3 | pipelines/compile_ms 548, pipelines/front_ms 531, main/busy/ms 144, render/thread/ms 127, render/busy/ms 126, stage/main/update 115, stage/render/prepare_assets 51.64, stage/render/prepare_write_phase_buffers 30.49, pipelines/driver_ms 17.25, stage/render/prepare_resources 16.91, stage/render/prepare_bind_groups 14.10, stage/main/pre_update 9.92 |
| warm-up | 3.99 | 63.0 | pipelines/compile_ms 369, pipelines/front_ms 343, render/thread/ms 104, render/busy/ms 101, stage/render/prepare_assets 70.69, main/busy/ms 65.05, stage/main/update 46.66, pipelines/wait_ms 39.72, stream/drive/ms 31.65, pipelines/driver_ms 26.68, stage/render/render 23.13, render/graph/ms 20.10 |
| warm-up | 4.04 | 50.2 | pipelines/compile_ms 156, pipelines/front_ms 135, render/thread/ms 97.08, render/busy/ms 94.09, main/busy/ms 78.76, pipelines/wait_ms 77.90, stage/render/prepare_assets 63.17, stage/main/update 46.96, stream/drive/ms 32.05, stage/render/render 23.88, pipelines/driver_ms 21.00, render/graph/ms 20.66 |
| warm-up | 4.09 | 47.1 | render/thread/ms 67.20, render/busy/ms 65.40, stage/render/prepare_assets 51.67, main/busy/ms 46.56, pipelines/wait_ms 39.08, pipelines/compile_ms 34.13, pipelines/front_ms 32.03, stage/main/post_update 17.82, stage/main/update 17.67, stream/drive/ms 12.03, stage/render/render 8.60, render/graph/ms 6.50 |
| warm-up | 4.12 | 32.1 | render/thread/ms 52.63, render/busy/ms 50.99, stage/render/prepare_assets 38.11, main/busy/ms 31.32, stage/main/update 19.25, stream/drive/ms 11.65, stage/render/render 7.50, stage/main/post_update 6.89, render/graph/ms 5.53, stage/render/graph_submit 3.51, encode//sum 3.48, stage/render/camera_driver 1.83 |
| warm-up | 4.17 | 48.5 | render/thread/ms 61.39, render/busy/ms 59.82, main/busy/ms 48.20, stage/render/prepare_assets 44.89, stage/main/update 36.79, stage/render/render 7.13, stage/main/post_update 6.35, render/graph/ms 5.37, stage/render/prepare_resources 4.83, stage/render/graph_submit 3.48, encode//sum 2.89, encode/submit 2.19 |
| warm-up | 4.21 | 40.0 | render/thread/ms 63.23, render/busy/ms 61.63, main/busy/ms 48.43, stage/render/prepare_assets 46.26, stage/main/update 37.07, stage/render/render 7.81, stage/main/post_update 6.52, render/graph/ms 5.99, stage/render/prepare_resources 4.83, stage/render/graph_submit 3.89, encode//sum 3.38, encode/submit 2.52 |
| warm-up | 4.48 | 40.3 | render/thread/ms 40.22, render/busy/ms 38.76, main/busy/ms 35.69, stage/render/prepare_assets 15.47, stage/main/post_update 11.92, stage/main/update 10.97, stage/render/render 8.54, render/graph/ms 6.72, stage/render/prepare_flush 6.32, stage/render/prepare_resources 6.07, stage/main/pre_update 5.15, stream/drive/ms 4.66 |
| warm-up | 4.52 | 39.1 | render/thread/ms 64.61, render/busy/ms 62.88, main/busy/ms 57.26, stage/main/update 35.39, stage/render/prepare_assets 15.38, stage/render/render 13.31, stage/main/post_update 11.67, render/graph/ms 10.87, stage/render/prepare_bind_groups 9.94, stage/render/prepare_resources 9.27, stage/render/prepare_flush 7.92, stage/render/graph_submit 6.87 |
| warm-up | 4.58 | 57.0 | main/busy/ms 110, stage/main/update 77.69, render/thread/ms 75.53, render/busy/ms 73.30, stream/drive/ms 44.70, stage/render/render 20.04, stage/render/prepare_assets 19.01, render/graph/ms 16.94, stage/main/post_update 14.35, stage/main/pre_update 11.96, encode//sum 10.61, stage/render/graph_submit 10.53 |
| warm-up | 4.69 | 116.7 | main/busy/ms 125, render/thread/ms 111, render/busy/ms 108, stage/main/update 76.63, encode//sum 50.73, stream/drive/ms 46.98, stage/render/prepare_assets 33.70, stage/render/render 31.82, stage/main/post_update 30.76, render/graph/ms 28.12, stage/render/graph_submit 19.31, stage/render/prepare_resources 17.10 |
| warm-up | 4.79 | 96.8 | render/thread/ms 142, render/busy/ms 138, main/busy/ms 100, stage/render/prepare_assets 55.80, stage/main/update 52.91, encode//sum 51.41, stage/render/render 35.82, stage/main/post_update 34.48, render/graph/ms 32.19, stage/render/graph_submit 22.84, stage/render/prepare_resources 16.97, encode/submit 15.62 |
| warm-up | 4.85 | 61.8 | render/thread/ms 112, render/busy/ms 110, main/busy/ms 71.15, stage/render/prepare_assets 59.16, stage/main/update 35.13, stage/render/render 25.27, render/graph/ms 22.60, stage/main/post_update 20.13, encode//sum 18.11, stream/drive/ms 15.15, stage/render/graph_submit 14.62, encode/submit 11.41 |
| warm-up | 4.92 | 69.1 | render/thread/ms 87.88, render/busy/ms 86.17, main/busy/ms 47.99, stage/render/prepare_assets 44.93, stage/main/update 23.52, stage/render/render 23.18, encode//sum 21.96, render/graph/ms 20.84, stage/main/post_update 13.21, stage/render/graph_submit 12.74, encode/submit 8.93, encode/pass:shadow_point_light/sum 8.91 |
| warm-up | 4.97 | 49.2 | render/thread/ms 92.38, render/busy/ms 90.12, encode//sum 76.21, main/busy/ms 66.29, stage/main/update 47.78, stage/render/render 40.74, render/graph/ms 37.89, stream/drive/ms 27.31, stage/render/graph_submit 27.08, stage/render/prepare_assets 26.37, encode/pass:shadow_point_light/sum 22.19, encode/submit 15.02 |
| warm-up | 5.02 | 47.1 | render/thread/ms 89.20, render/busy/ms 87.17, encode//sum 76.98, main/busy/ms 57.92, stage/main/update 41.61, stage/render/render 39.06, render/graph/ms 36.44, stream/drive/ms 27.00, stage/render/graph_submit 25.95, encode/pass:shadow_point_light/sum 22.78, encode/submit 13.89, stage/render/create_views 12.79 |
| warm-up | 5.06 | 47.2 | render/thread/ms 87.63, render/busy/ms 85.55, encode//sum 64.55, stage/render/render 37.05, render/graph/ms 34.26, main/busy/ms 29.80, stage/render/graph_submit 23.72, encode/pass:shadow_point_light/sum 20.25, encode/submit 13.76, stage/main/update 13.03, stage/render/create_views 12.05, stage/render/prepare_assets 11.93 |
| warm-up | 5.09 | 30.7 | render/thread/ms 73.50, render/busy/ms 71.31, encode//sum 56.71, stage/render/render 37.51, render/graph/ms 34.67, main/busy/ms 34.07, stage/render/graph_submit 25.59, stage/main/update 17.65, encode/pass:shadow_point_light/sum 17.53, encode/submit 16.44, stage/main/post_update 9.52, encode/finish_wall 9.24 |
| warm-up | 5.13 | 33.3 | render/thread/ms 55.66, render/busy/ms 54.07, main/busy/ms 32.41, stage/render/render 20.02, render/graph/ms 17.66, stage/main/update 16.84, encode//sum 12.39, stage/render/graph_submit 11.90, stage/render/prepare_assets 11.86, stage/main/post_update 9.55, encode/submit 9.22, stage/render/prepare_flush 6.74 |
| warm-up | 5.18 | 35.2 | render/thread/ms 52.61, render/busy/ms 50.98, main/busy/ms 37.85, stage/main/post_update 23.32, stage/render/render 18.34, render/graph/ms 16.16, encode//sum 11.95, stage/render/graph_submit 10.23, stage/main/update 10.09, stage/render/prepare_write_phase_buffers 9.19, stage/render/prepare_resources 8.47, encode/submit 7.98 |
| warm-up | 5.24 | 33.0 | render/thread/ms 39.40, render/busy/ms 37.91, main/busy/ms 23.40, stage/render/render 18.51, render/graph/ms 16.46, encode//sum 11.56, stage/render/graph_submit 11.51, stage/render/prepare_assets 10.09, stage/main/post_update 9.85, stage/main/update 8.70, encode/submit 7.95, encode/pass:shadow_point_light/sum 5.83 |
| warm-up | 5.39 | 81.1 | pipelines/compile_ms 3385, pipelines/driver_ms 3098, pipelines/front_ms 287, render/thread/ms 90.15, main/busy/ms 89.59, render/busy/ms 87.97, stage/main/update 51.14, stage/main/post_update 32.44, stage/render/prepare_write_phase_buffers 18.46, stage/render/render 18.15, render/graph/ms 15.92, stage/render/prepare_assets 15.63 |
| warm-up | 5.46 | 68.5 | pipelines/compile_ms 10.8 k, pipelines/driver_ms 9998, pipelines/wait_ms 1407, pipelines/front_ms 847, render/thread/ms 114, render/busy/ms 112, main/busy/ms 82.44, stage/main/update 48.36, stage/render/prepare_assets 44.45, stage/main/post_update 26.02, stage/render/prepare_write_phase_buffers 16.05, stage/render/render 15.54 |
| warm-up | 5.48 | 25.6 | pipelines/compile_ms 7461, pipelines/driver_ms 6901, pipelines/wait_ms 1406, pipelines/front_ms 560, render/thread/ms 65.01, render/busy/ms 63.83, main/busy/ms 37.93, stage/render/prepare_assets 35.96, stage/main/update 18.75, stage/render/render 14.16, render/graph/ms 12.37, stage/main/post_update 11.52 |
| warm-up | 5.64 | 27.7 | pipelines/compile_ms 5451, pipelines/driver_ms 4916, pipelines/wait_ms 4824, pipelines/front_ms 535, render/thread/ms 41.60, render/busy/ms 40.01, main/busy/ms 31.66, stage/render/render 18.34, stage/main/update 17.26, render/graph/ms 16.15, encode//sum 12.04, stage/render/graph_submit 10.33 |
| warm-up | 5.76 | 33.0 | pipelines/wait_ms 5132, pipelines/compile_ms 705, pipelines/front_ms 678, render/thread/ms 48.18, render/busy/ms 46.79, main/busy/ms 36.92, pipelines/driver_ms 26.90, stage/render/render 18.93, render/graph/ms 16.91, stage/main/post_update 15.77, stage/render/graph_submit 11.31, encode//sum 10.97 |
| warm-up | 6.02 | 25.3 | pipelines/compile_ms 211, pipelines/front_ms 202, pipelines/wait_ms 61.59, render/thread/ms 45.16, render/busy/ms 43.54, main/busy/ms 27.47, stage/render/render 21.64, render/graph/ms 19.58, stage/main/post_update 14.30, encode//sum 12.86, stage/render/graph_submit 11.75, pipelines/driver_ms 9.47 |
| warm-up | 6.13 | 28.9 | pipelines/compile_ms 744, pipelines/front_ms 721, render/thread/ms 39.16, pipelines/wait_ms 39.06, render/busy/ms 37.55, main/busy/ms 27.74, stage/render/render 23.44, pipelines/driver_ms 22.51, render/graph/ms 21.41, stage/main/post_update 18.15, stage/render/graph_submit 16.48, encode//sum 10.41 |
| warm-up | 6.19 | 59.5 | pipelines/compile_ms 410, pipelines/front_ms 396, render/thread/ms 79.46, render/busy/ms 65.60, main/busy/ms 42.68, stage/render/render 23.87, render/graph/ms 21.57, stage/render/graph_submit 16.54, stage/render/prepare_write_phase_buffers 15.01, stage/main/post_update 14.42, pipelines/driver_ms 14.28, stage/render/prepare_views 12.51 |
| warm-up | 6.22 | 34.8 | pipelines/compile_ms 210, pipelines/front_ms 201, render/thread/ms 89.08, render/busy/ms 75.31, main/busy/ms 51.97, stage/render/prepare_resources 20.27, stage/render/render 18.45, stage/main/post_update 16.87, stage/render/prepare_write_phase_buffers 16.75, render/graph/ms 16.16, stage/render/prepare_views 12.49, render/acquire/ms 12.35 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 7%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (253 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **84%** (min 51%) · 2625 MHz (min 2625) · memory 8101 MHz · 87.4 W · 72 °C (max 77) · VRAM 4933 MiB · throttled 100% of samples

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
