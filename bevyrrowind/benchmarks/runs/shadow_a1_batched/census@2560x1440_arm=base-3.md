# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1543 frames over 11.4 s · **median 6.95 ms (144 fps)** · mean 7.41 · p95 9.98 · p99 10.69 (1% low 94 fps) · worst 21.12 ms · 28 spikes over 1.5× median · 10.5 stutters/min (work 5.2/min)
- warm-up: 120 frames over 6.0 s, worst 2578.4 ms
- hitches (> 25.0 ms): **0** measured, 35 in warm-up
- timed GPU passes: 5.32 ms of a 7.41 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median fps 144.0, target 144
- pass worst frame 21.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.983 | 2.149 | 2.449 | 0.92 | 0.166 |
| main_opaque_pass_3d | 1.790 | 1.940 | 2.824 | 0.92 | 0.043 |
| auxiliary_views | 0.323 | 0.350 | 1.123 | 0.92 | 0.309 |
| bloom | 0.319 | 0.346 | 0.680 | 0.92 | 0.064 |
| ssao | 0.203 | 0.220 | 0.553 | 0.92 | 0.007 |
| upscaling | 0.153 | 0.166 | 0.209 | 0.92 | 0.004 |
| tlas_build | 0.106 | 0.115 | 0.119 | 0.92 | 0.075 |
| tonemapping | 0.102 | 0.110 | 0.125 | 0.92 | 0.004 |
| gpu_instance_materialization | 0.089 | 0.096 | 0.099 | 0.92 | 0.004 |
| gi/probe_update | 0.085 | 0.092 | 0.097 | 0.92 | 0.002 |
| early prepass | 0.071 | 0.077 | 0.086 | 0.92 | 0.046 |
| clustering | 0.068 | 0.073 | 0.075 | 0.92 | 0.019 |
| uniform_allocation | 0.035 | 0.038 | 0.044 | 0.92 | 0.011 |
| bin_unpacking | 0.029 | 0.032 | 0.037 | 0.92 | 0.008 |
| range_unpacking | 0.017 | 0.019 | 0.019 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.016 | 0.018 | 0.025 | 0.92 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.012 | 0.90 | 0.024 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.92 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.26 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.92 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.408 | 8.937 | 19.90 | 9887.1 |
| render/busy/ms | 5.930 | 8.468 | 11.77 | 9149.6 |
| main/busy/ms | 5.510 | 6.547 | 9.51 | 8502.5 |
| stage/render/render | 4.008 | 5.806 | 16.22 | 6184.6 |
| render/graph/ms | 3.373 | 5.165 | 7.15 | 5204.9 |
| stage/main/post_update | 2.929 | 3.580 | 6.97 | 4519.5 |
| encode//sum | 2.737 | 4.342 | 6.23 | 4222.5 |
| stage/render/graph_submit | 2.218 | 3.384 | 4.33 | 3422.4 |
| encode/submit | 1.531 | 2.361 | 3.23 | 2362.7 |
| stage/main/update | 1.241 | 1.509 | 2.33 | 1914.7 |
| stage/render/camera_driver | 1.024 | 1.669 | 3.43 | 1579.4 |
| encode/pass:shadow_point_light/sum | 0.911 | 2.844 | 3.86 | 1405.1 |
| encode/finish_wall | 0.731 | 1.082 | 1.61 | 1127.4 |
| stage/render/prepare_resources | 0.418 | 0.523 | 0.81 | 645.7 |
| stage/render/prepare_write_phase_buffers | 0.416 | 0.808 | 1.37 | 641.7 |
| encode/pass:early prepass/sum | 0.414 | 0.592 | 0.94 | 638.1 |
| render/present/ms | 0.395 | 0.786 | 10.65 | 609.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.392 | 0.569 | 0.93 | 604.4 |
| stage/main/pre_update | 0.357 | 0.458 | 0.73 | 551.0 |
| stage/main/first | 0.347 | 0.436 | 1.79 | 535.7 |
| stage/render/prepare_bind_groups | 0.334 | 0.655 | 1.91 | 515.0 |
| stage/main/fixed_main_loop | 0.304 | 0.765 | 1.03 | 468.9 |
| bigspace/total/ms | 0.245 | 0.591 | 0.97 | 377.6 |
| stage/render/prepare_meshes | 0.228 | 0.310 | 0.50 | 351.6 |
| stage/render/prepare_assets | 0.211 | 0.274 | 0.68 | 326.0 |
| stage/render/prepare_flush | 0.182 | 0.354 | 1.18 | 280.1 |
| stage/render/extract_commands | 0.165 | 0.199 | 0.37 | 254.0 |
| stage/main/last | 0.157 | 0.218 | 0.99 | 242.5 |
| stage/render/create_views | 0.155 | 0.198 | 0.45 | 238.7 |
| stage/render/prepare_views | 0.129 | 0.185 | 0.74 | 199.4 |
| stream/drive/ms | 0.128 | 0.185 | 0.41 | 197.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.122 | 0.179 | 0.68 | 188.2 |
| physics/step/ms | 0.121 | 0.396 | 0.64 | 187.3 |
| bigspace/low_precision/ms | 0.103 | 0.277 | 0.73 | 158.4 |
| stream/tick/ms | 0.098 | 0.150 | 0.38 | 150.9 |
| stage/render/specialize | 0.085 | 0.116 | 0.73 | 131.4 |
| stage/render/queue | 0.085 | 0.124 | 0.48 | 131.2 |
| stage/render/prepare_batch_phases | 0.084 | 0.133 | 0.38 | 129.4 |
| render/acquire/ms | 0.083 | 0.120 | 0.71 | 128.2 |
| controller/feed/ms | 0.082 | 0.105 | 0.25 | 126.3 |
| anim/evaluate/ms | 0.071 | 0.193 | 0.80 | 110.2 |
| bigspace/high_precision/ms | 0.050 | 0.130 | 0.62 | 76.9 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.072 | 0.20 | 72.9 |
| anim/evaluate_cpu/ms | 0.036 | 0.054 | 0.27 | 55.3 |
| encode/resolve_diagnostics/sum | 0.033 | 0.044 | 0.27 | 50.2 |
| controller/write/ms | 0.027 | 0.039 | 0.20 | 40.9 |
| weather/map/ms | 0.026 | 0.037 | 0.22 | 40.1 |
| stage/render/cleanup | 0.023 | 0.035 | 0.18 | 35.5 |
| stage/render/phase_sort | 0.018 | 0.043 | 0.22 | 27.2 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.12 | 19.7 |
| weather/eval/ms | 0.012 | 0.016 | 0.04 | 17.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.15 | 16.8 |
| script/update/ms | 0.009 | 0.012 | 0.17 | 13.1 |
| physics/insert/ms | 0.008 | 0.000 | 0.58 | 12.6 |
| anim/apply/ms | 0.005 | 0.010 | 0.03 | 7.6 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.6 |
| controller/move/ms | 0.004 | 0.011 | 0.21 | 6.4 |
| far/select/ms | 0.004 | 0.004 | 0.13 | 6.0 |
| script/ui/ms | 0.004 | 0.005 | 0.08 | 5.5 |
| controller/avoid/ms | 0.002 | 0.005 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.7 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.01 | 0.8 |
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
| crowd/joints_extracted/count | count | 46 | 1357 | 0 |
| crowd/joints_uploaded/count | count | 94 | 153 | 94 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 13.1 k | 13.1 k | 13.1 k |
| encode//max | ms | 0.42 | 1.31 | 0.37 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.12 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.15 | 0.01 |
| encode/finish_critical | ms | 0.51 | 1.31 | 0.67 |
| encode/pass:early prepass/max | ms | 0.42 | 0.94 | 0.38 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.39 | 0.93 | 0.67 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 0.68 | 0.12 |
| encode/pass:shadow_point_light/max | ms | 0.48 | 1.01 | 0.49 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.27 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.81 | 11.77 | 5.06 |
| gpu/timed/ms | ms | 5.55 | 7.85 | 5.06 |
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
| phases/shadow/batch_sets | count | 0 | 77 | 0 |
| phases/shadow/bins | count | 2635 | 2866 | 2173 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3597 | 3839 | 3105 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 227 | 240 | 189 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 85 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 291 | 291 | 291 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 7 | 202 | 4 |
| preprocess/bind_groups_created | count | 3 | 198 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 47 | 103 | 35 |
| process/cpu_usage | % | 25.30 | 37.98 | 34.62 |
| process/mem_usage | GiB | 3.81 | 3.93 | 3.92 |
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
| system/cpu_usage | % | 32.64 | 42.26 | 32.51 |
| system/mem_usage | % | 54.10 | 54.43 | 54.43 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 26 | 106 | 17 |
| views/all/bins | count | 2981 | 3313 | 2231 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 4165 | 4586 | 3191 |
| views/all/pipelines | count | 253 | 267 | 204 |
| views/all/pipelines_distinct | count | 28 | 34 | 22 |
| views/all/work_items | count | 98.9 k | 100.7 k | 98.4 k |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 77 | 0 |
| views/lamp_face/bins | count | 2635 | 2866 | 2173 |
| views/lamp_face/count | count | 48 | 48 | 48 |
| views/lamp_face/culled | count | 25 | 36 | 35 |
| views/lamp_face/drawn | count | 0 | 12 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3597 | 3839 | 3105 |
| views/lamp_face/pipelines | count | 227 | 240 | 189 |
| views/lamp_face/revealed | count | 0 | 1 | 0 |
| views/lamp_face/work_items | count | 0 | 1296 | 0 |
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
| anim/bones_written/count | count | 8.23 | 30 | 12.7 k |
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
| ecs/entities_delta | count | 0 | 0 | 0 |
| encode//count | count | 31.23 | 68 | 48.2 k |
| encode/bind_walk/init_textures | count | 8238 | 19.6 k | 12.71 M |
| encode/bind_walk/init_textures_skipped | count | 10.5 k | 26.8 k | 16.18 M |
| encode/bind_walk/inits | count | 215 | 425 | 331.6 k |
| encode/bind_walk/inits_skipped | count | 15.40 | 42 | 23.8 k |
| encode/bind_walk/merge_textures | count | 6995 | 18.4 k | 10.79 M |
| encode/bind_walk/merge_textures_skipped | count | 3844 | 15.5 k | 5.93 M |
| encode/bind_walk/merges | count | 58.74 | 113 | 90.6 k |
| encode/bind_walk/merges_skipped | count | 19.15 | 67 | 29.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1543 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1543 |
| encode/pass:early prepass/count | count | 1 | 1 | 1543 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1543 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1497 |
| encode/pass:shadow_point_light/count | count | 2.99 | 12 | 4609 |
| encode/queue/command_buffers | count | 43.19 | 89 | 66.6 k |
| encode/queue/submits | count | 2 | 2 | 3086 |
| encode/queue/write_buffer/bytes | bytes | 601.7 k | 685.2 k | 928.49 M |
| encode/queue/write_buffer/calls | count | 158 | 355 | 243.3 k |
| encode/queue/write_buffer_with/bytes | bytes | 149.6 k | 149.6 k | 230.88 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 9258 |
| encode/queue/write_texture/bytes | bytes | 680 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1543 |
| physics/inserts | count | 0.06 | 12 | 86 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.44 | 2 | 685 |
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

## Hitches (> 25.0 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2578.4 | main/busy/ms 2550, stage/main/post_update 2446, stage/main/update 93.82, surface/advance_cpu_ms 66.49, stage/main/pre_update 5.87, stage/main/last 1.16, stream/drive/ms 0.48, stage/main/fixed_main_loop 0.46, stage/main/first 0.45, bigspace/total/ms 0.13, weather/map/ms 0.13, stream/tick/ms 0.11 |
| warm-up | 3.30 | 722.8 | main/busy/ms 2662, stage/main/post_update 2451, render/thread/ms 687, render/busy/ms 543, stage/render/prepare_assets 521, stage/main/update 196, stage/render/prepare_views 143, render/acquire/ms 142, surface/advance_cpu_ms 66.49, stage/render/prepare_resources 11.49, stage/render/render 8.14, stage/main/pre_update 6.96 |
| warm-up | 3.35 | 48.7 | render/thread/ms 721, render/busy/ms 576, stage/render/prepare_assets 548, stage/render/prepare_views 143, render/acquire/ms 142, main/busy/ms 126, stage/main/update 107, stage/render/render 12.37, stage/render/prepare_resources 11.97, render/graph/ms 8.87, stage/main/post_update 8.43, stage/main/fixed_main_loop 7.46 |
| warm-up | 3.43 | 63.4 | pipelines/wait_ms 1241, pipelines/compile_ms 504, pipelines/front_ms 460, main/busy/ms 53.25, pipelines/driver_ms 43.14, render/thread/ms 42.98, stage/main/post_update 39.28, render/busy/ms 39.01, stage/render/render 18.71, render/graph/ms 14.88, stage/render/prepare_resources 9.87, stage/render/prepare_assets 9.01 |
| warm-up | 3.49 | 28.8 | pipelines/compile_ms 391, pipelines/front_ms 379, render/thread/ms 40.80, render/busy/ms 38.90, main/busy/ms 22.52, stage/render/render 16.71, render/graph/ms 14.68, pipelines/driver_ms 12.66, encode//sum 12.35, stage/render/graph_submit 11.45, stage/render/prepare_assets 10.84, encode/finish_wall 9.59 |
| warm-up | 3.52 | 31.6 | pipelines/compile_ms 788, pipelines/front_ms 762, main/busy/ms 62.77, stage/main/update 51.51, render/thread/ms 50.67, pipelines/wait_ms 46.13, render/busy/ms 40.43, stage/render/render 33.58, pipelines/driver_ms 25.73, render/graph/ms 23.41, stage/render/graph_submit 17.91, encode//sum 11.95 |
| warm-up | 3.60 | 76.0 | pipelines/compile_ms 868, pipelines/front_ms 837, render/thread/ms 78.17, main/busy/ms 73.29, render/busy/ms 68.29, stage/main/update 57.42, stage/render/prepare_assets 48.72, pipelines/wait_ms 48.28, pipelines/driver_ms 30.85, stage/render/render 24.02, render/graph/ms 14.22, stage/main/post_update 10.35 |
| warm-up | 3.63 | 25.6 | pipelines/compile_ms 758, pipelines/front_ms 731, render/thread/ms 69.89, render/busy/ms 67.18, stage/render/prepare_assets 54.42, main/busy/ms 31.91, pipelines/driver_ms 26.67, stage/main/update 15.73, stage/render/render 9.92, stage/main/post_update 9.56, render/graph/ms 7.21, pipelines/wait_ms 6.35 |
| warm-up | 3.69 | 29.1 | pipelines/compile_ms 477, pipelines/front_ms 457, render/thread/ms 40.69, render/busy/ms 38.87, main/busy/ms 36.04, stage/render/prepare_assets 26.51, stage/main/update 24.62, pipelines/driver_ms 20.18, stream/drive/ms 16.84, stage/render/render 8.54, stage/main/post_update 7.70, render/graph/ms 6.61 |
| warm-up | 3.74 | 49.2 | pipelines/compile_ms 853, pipelines/front_ms 823, render/thread/ms 66.23, render/busy/ms 64.41, main/busy/ms 57.52, stage/render/prepare_assets 33.73, pipelines/driver_ms 30.22, stage/main/post_update 27.66, stage/main/update 23.47, stream/drive/ms 16.44, stage/render/prepare_resources 9.30, stage/render/render 8.84 |
| warm-up | 3.81 | 64.8 | pipelines/compile_ms 744, pipelines/front_ms 716, main/busy/ms 181, stage/main/update 144, render/thread/ms 92.69, render/busy/ms 90.33, stage/render/render 39.19, render/graph/ms 35.78, stage/render/graph_submit 33.90, pipelines/driver_ms 28.02, stage/render/prepare_assets 27.99, encode/submit 27.72 |
| warm-up | 4.03 | 221.6 | pipelines/compile_ms 2126, pipelines/front_ms 2062, main/busy/ms 155, stage/main/update 142, render/thread/ms 129, render/busy/ms 127, stage/render/prepare_assets 84.03, pipelines/wait_ms 83.50, pipelines/driver_ms 63.99, stage/render/render 38.17, render/graph/ms 34.96, stage/render/graph_submit 33.21 |
| warm-up | 4.10 | 71.1 | pipelines/compile_ms 1905, pipelines/front_ms 1852, render/thread/ms 136, render/busy/ms 134, stage/render/prepare_assets 119, pipelines/wait_ms 84.71, pipelines/driver_ms 53.34, main/busy/ms 37.48, stage/main/update 27.50, stream/drive/ms 23.76, stage/render/render 13.21, render/graph/ms 11.74 |
| warm-up | 4.13 | 32.6 | pipelines/compile_ms 622, pipelines/front_ms 591, render/thread/ms 86.96, render/busy/ms 83.64, main/busy/ms 53.86, stage/render/prepare_assets 52.81, stage/main/update 32.49, pipelines/driver_ms 30.65, stage/render/render 28.94, stream/drive/ms 23.93, render/graph/ms 18.90, stage/render/graph_submit 16.98 |
| warm-up | 4.17 | 33.9 | pipelines/compile_ms 563, pipelines/front_ms 536, main/busy/ms 59.70, render/thread/ms 59.23, render/busy/ms 55.18, stage/main/update 37.06, stage/render/render 36.72, pipelines/driver_ms 26.81, render/graph/ms 26.10, pipelines/wait_ms 23.27, stream/drive/ms 13.02, stage/render/graph_submit 10.87 |
| warm-up | 4.20 | 36.4 | pipelines/compile_ms 135, pipelines/front_ms 127, main/busy/ms 53.86, render/thread/ms 52.34, render/busy/ms 50.34, stage/main/update 34.44, stage/render/render 22.62, render/graph/ms 20.50, pipelines/wait_ms 19.28, stage/render/prepare_assets 15.93, stream/drive/ms 13.02, stage/main/post_update 11.91 |
| warm-up | 4.25 | 42.1 | pipelines/compile_ms 1446, pipelines/front_ms 1263, pipelines/driver_ms 183, render/thread/ms 60.73, render/busy/ms 59.54, stage/render/prepare_assets 47.92, pipelines/wait_ms 38.31, main/busy/ms 29.36, stage/main/post_update 14.15, stage/main/update 8.35, stage/render/render 7.68, render/graph/ms 6.30 |
| warm-up | 4.50 | 31.5 | pipelines/compile_ms 354, pipelines/front_ms 332, render/thread/ms 41.32, pipelines/wait_ms 40.40, render/busy/ms 39.79, main/busy/ms 25.17, pipelines/driver_ms 22.70, stage/render/prepare_resources 15.89, stage/render/render 11.00, stage/render/prepare_bind_groups 9.98, stage/main/update 9.71, stage/main/post_update 8.33 |
| warm-up | 4.53 | 35.8 | pipelines/compile_ms 913, pipelines/front_ms 878, pipelines/wait_ms 69.47, render/thread/ms 60.24, render/busy/ms 58.78, main/busy/ms 42.95, pipelines/driver_ms 35.21, stage/render/prepare_assets 22.52, stage/main/update 19.49, stage/render/prepare_resources 18.63, stage/main/post_update 11.72, stage/render/render 9.73 |
| warm-up | 4.57 | 36.2 | pipelines/compile_ms 1128, pipelines/front_ms 1093, main/busy/ms 68.98, render/thread/ms 64.03, render/busy/ms 62.02, pipelines/wait_ms 57.33, stage/main/update 47.33, pipelines/driver_ms 34.61, stage/render/prepare_assets 24.18, stage/render/render 19.74, render/graph/ms 17.54, stage/render/graph_submit 15.16 |
| warm-up | 4.62 | 53.0 | pipelines/compile_ms 1191, pipelines/front_ms 1153, pipelines/wait_ms 120, render/thread/ms 68.31, render/busy/ms 66.01, main/busy/ms 53.20, stage/main/update 40.38, pipelines/driver_ms 37.58, stage/render/render 26.29, render/graph/ms 23.60, stage/render/prepare_assets 19.37, stage/render/graph_submit 18.73 |
| warm-up | 4.66 | 36.5 | pipelines/compile_ms 1086, pipelines/front_ms 1049, pipelines/wait_ms 132, render/thread/ms 69.66, render/busy/ms 67.65, main/busy/ms 66.96, encode//sum 63.35, stage/main/update 52.36, stream/drive/ms 43.78, pipelines/driver_ms 37.60, stage/render/render 28.96, render/graph/ms 26.59 |
| warm-up | 4.73 | 76.1 | pipelines/compile_ms 938, pipelines/front_ms 893, main/busy/ms 84.97, render/thread/ms 79.40, render/busy/ms 77.43, encode//sum 70.63, stage/main/update 64.21, pipelines/driver_ms 45.30, stream/drive/ms 44.87, pipelines/wait_ms 41.85, stage/render/render 33.89, render/graph/ms 31.57 |
| warm-up | 4.79 | 54.1 | pipelines/compile_ms 597, pipelines/front_ms 571, render/thread/ms 89.27, render/busy/ms 87.44, main/busy/ms 64.03, stage/main/update 36.28, stage/render/prepare_assets 31.16, stage/render/render 28.30, pipelines/driver_ms 26.49, render/graph/ms 25.70, encode//sum 24.34, stage/main/post_update 19.25 |
| warm-up | 4.88 | 95.1 | render/thread/ms 113, render/busy/ms 112, pipelines/compile_ms 102, pipelines/front_ms 99.75, main/busy/ms 52.56, stage/render/prepare_assets 43.26, stage/render/create_views 29.03, stage/main/update 28.50, encode//sum 21.39, stage/render/render 21.29, render/graph/ms 19.01, stage/main/post_update 14.58 |
| warm-up | 4.95 | 64.5 | render/thread/ms 117, render/busy/ms 116, stage/render/prepare_assets 48.28, main/busy/ms 34.49, stage/render/create_views 28.23, stage/render/render 25.34, render/graph/ms 23.40, encode//sum 22.87, stage/main/update 18.48, stage/render/graph_submit 16.93, encode/finish_wall 9.40, encode/pass:shadow_point_light/sum 9.31 |
| warm-up | 5.02 | 68.9 | pipelines/compile_ms 167, pipelines/front_ms 159, render/thread/ms 115, render/busy/ms 113, encode//sum 80.60, main/busy/ms 72.62, stage/render/render 46.11, stage/main/update 45.44, render/graph/ms 43.55, stage/render/graph_submit 32.80, stream/drive/ms 32.41, encode/pass:shadow_point_light/sum 28.36 |
| warm-up | 5.06 | 38.3 | pipelines/compile_ms 1446, pipelines/front_ms 1401, render/thread/ms 99.70, render/busy/ms 97.98, encode//sum 80.18, main/busy/ms 71.61, pipelines/driver_ms 45.29, stage/main/update 43.03, stage/render/render 40.03, render/graph/ms 37.79, stream/drive/ms 32.95, encode/pass:shadow_point_light/sum 31.58 |
| warm-up | 5.13 | 29.1 | pipelines/compile_ms 114, pipelines/front_ms 108, render/thread/ms 46.07, render/busy/ms 44.05, main/busy/ms 30.33, stage/render/render 21.94, render/graph/ms 18.93, encode//sum 17.11, stage/main/update 16.10, stage/render/graph_submit 11.53, stage/main/post_update 10.13, stage/render/prepare_assets 9.96 |
| warm-up | 5.16 | 30.4 | pipelines/compile_ms 119, pipelines/front_ms 111, render/thread/ms 51.86, render/busy/ms 50.27, main/busy/ms 30.75, stage/render/render 18.44, render/graph/ms 16.45, stage/main/post_update 13.82, stage/render/prepare_assets 12.52, stage/main/update 11.84, encode//sum 10.35, stage/render/graph_submit 10.08 |
| warm-up | 5.19 | 31.6 | pipelines/compile_ms 541, pipelines/front_ms 510, render/thread/ms 56.99, render/busy/ms 55.50, main/busy/ms 39.14, pipelines/driver_ms 31.57, stage/main/post_update 22.74, stage/render/render 19.29, render/graph/ms 17.10, encode//sum 11.24, stage/main/update 10.75, stage/render/prepare_write_phase_buffers 10.02 |
| warm-up | 5.23 | 25.0 | pipelines/compile_ms 689, pipelines/front_ms 668, render/thread/ms 33.81, render/busy/ms 32.48, main/busy/ms 23.10, pipelines/driver_ms 20.81, stage/render/render 15.13, render/graph/ms 13.34, encode//sum 11.26, stage/main/post_update 9.90, stage/render/prepare_assets 8.80, stage/render/graph_submit 8.54 |
| warm-up | 5.32 | 41.4 | pipelines/compile_ms 551, pipelines/front_ms 533, main/busy/ms 55.34, render/thread/ms 50.72, render/busy/ms 48.76, stage/main/update 34.98, stage/render/render 22.39, render/graph/ms 19.80, pipelines/driver_ms 17.64, encode//sum 17.41, stage/main/post_update 14.98, stage/render/graph_submit 12.83 |
| warm-up | 5.37 | 42.1 | pipelines/compile_ms 795, pipelines/front_ms 771, render/thread/ms 61.19, render/busy/ms 59.91, main/busy/ms 50.44, stage/main/update 30.13, stage/render/prepare_assets 25.14, pipelines/driver_ms 24.06, stage/render/render 18.31, encode//sum 16.52, render/graph/ms 16.30, stage/main/post_update 12.05 |
| warm-up | 5.40 | 35.5 | pipelines/compile_ms 244, pipelines/front_ms 238, render/thread/ms 50.60, render/busy/ms 49.62, stage/render/prepare_assets 30.02, main/busy/ms 25.96, stage/main/update 12.09, stage/render/render 10.18, encode//sum 9.19, render/graph/ms 8.66, stage/main/post_update 7.25, pipelines/driver_ms 6.42 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\census@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (253 pmon samples)
- focus: clean
- nvidia-smi over the window (45 samples): GPU load **86%** (min 73%) · 2626 MHz (min 2625) · memory 8101 MHz · 89.7 W · 70 °C (max 76) · VRAM 4975 MiB · throttled 100% of samples

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
