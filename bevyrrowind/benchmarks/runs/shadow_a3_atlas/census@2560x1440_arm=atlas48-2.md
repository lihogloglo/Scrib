# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=on (command line: --set); shadow.lamp.atlas.max_lamps=48 (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1727 frames over 11.4 s · **median 6.52 ms (153 fps)** · mean 6.62 · p95 7.77 · p99 8.67 (1% low 115 fps) · worst 79.99 ms · 8 spikes over 1.5× median · 36.7 stutters/min (work 10.5/min)
- warm-up: 120 frames over 6.0 s, worst 2568.5 ms
- hitches (> 25.0 ms): **1** measured, 36 in warm-up
- timed GPU passes: 5.22 ms of a 6.62 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median fps 153.3, target 144
- **FAIL** worst frame 79.99 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 1.992 | 2.195 | 3.229 | 0.91 | 0.045 |
| dlss_super_resolution | 1.959 | 2.159 | 2.500 | 0.91 | 0.164 |
| bloom | 0.315 | 0.347 | 0.683 | 0.91 | 0.064 |
| ssao | 0.193 | 0.213 | 0.548 | 0.91 | 0.007 |
| upscaling | 0.151 | 0.167 | 0.186 | 0.91 | 0.005 |
| tlas_build | 0.106 | 0.117 | 0.118 | 0.91 | 0.072 |
| tonemapping | 0.095 | 0.104 | 0.111 | 0.91 | 0.004 |
| gi/probe_update | 0.090 | 0.099 | 0.100 | 0.91 | 0.002 |
| gpu_instance_materialization | 0.089 | 0.098 | 0.099 | 0.91 | 0.004 |
| early prepass | 0.072 | 0.080 | 0.092 | 0.91 | 0.052 |
| clustering | 0.071 | 0.078 | 0.083 | 0.91 | 0.019 |
| point_shadow_atlas | 0.066 | 0.085 | 0.108 | 0.78 | 0.039 |
| uniform_allocation | 0.035 | 0.038 | 0.044 | 0.91 | 0.011 |
| bin_unpacking | 0.030 | 0.033 | 0.038 | 0.91 | 0.009 |
| range_unpacking | 0.017 | 0.018 | 0.018 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.016 | 0.018 | 0.020 | 0.91 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.011 | 0.89 | 0.024 |
| point_shadow_atlas/early_mesh_preprocessing | 0.007 | 0.008 | 0.013 | 0.78 | 0.002 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.91 | 0.006 |
| point_shadow_atlas/bin_unpacking | 0.006 | 0.007 | 0.007 | 0.78 | 0.002 |
| point_shadow_atlas/uniform_allocation | 0.005 | 0.007 | 0.007 | 0.78 | 0.002 |
| point_shadow_atlas/range_unpacking | 0.005 | 0.007 | 0.007 | 0.78 | 0.002 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.34 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.001 | 0.91 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.91 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 5.583 | 6.679 | 79.06 | 9641.5 |
| main/busy/ms | 5.413 | 6.241 | 11.57 | 9348.8 |
| render/busy/ms | 4.846 | 5.611 | 78.55 | 8369.2 |
| stage/render/render | 3.315 | 3.976 | 77.03 | 5725.3 |
| stage/main/post_update | 2.962 | 3.438 | 9.04 | 5115.9 |
| render/graph/ms | 2.752 | 3.370 | 76.42 | 4751.9 |
| encode//sum | 2.474 | 3.010 | 440.89 | 4272.0 |
| stage/render/graph_submit | 1.822 | 2.355 | 75.44 | 3146.3 |
| stage/main/update | 1.279 | 1.547 | 2.36 | 2208.2 |
| encode/submit | 1.196 | 1.699 | 2.94 | 2065.5 |
| stage/render/camera_driver | 0.793 | 0.970 | 1.17 | 1368.7 |
| encode/finish_wall | 0.681 | 0.831 | 73.66 | 1176.9 |
| encode/pass:early prepass/sum | 0.501 | 0.614 | 72.97 | 865.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.441 | 0.604 | 6.06 | 761.0 |
| stage/render/prepare_resources | 0.407 | 0.510 | 0.77 | 703.7 |
| render/present/ms | 0.400 | 0.835 | 11.72 | 690.0 |
| stage/render/prepare_views | 0.383 | 1.249 | 1.83 | 661.7 |
| stage/main/pre_update | 0.351 | 0.449 | 0.76 | 606.5 |
| render/acquire/ms | 0.337 | 1.208 | 1.78 | 582.3 |
| stage/main/first | 0.253 | 0.347 | 1.74 | 437.2 |
| stage/main/fixed_main_loop | 0.250 | 0.691 | 1.48 | 432.2 |
| stage/render/prepare_write_phase_buffers | 0.249 | 0.355 | 0.60 | 430.6 |
| bigspace/total/ms | 0.241 | 0.588 | 1.18 | 416.7 |
| stage/render/prepare_meshes | 0.224 | 0.293 | 0.53 | 386.1 |
| stage/render/prepare_assets | 0.216 | 0.290 | 0.62 | 372.6 |
| stage/render/prepare_bind_groups | 0.215 | 0.293 | 0.59 | 370.7 |
| stage/render/create_views | 0.186 | 0.263 | 0.64 | 320.8 |
| stage/render/prepare_flush | 0.177 | 0.364 | 0.56 | 304.8 |
| stage/main/last | 0.150 | 0.189 | 0.43 | 259.1 |
| stage/render/extract_commands | 0.147 | 0.178 | 0.30 | 253.4 |
| stream/drive/ms | 0.139 | 0.199 | 0.52 | 240.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.131 | 0.199 | 5.39 | 226.1 |
| encode/pass:point_shadow_atlas_live_movers/sum | 0.128 | 0.230 | 5.24 | 220.6 |
| stream/tick/ms | 0.110 | 0.165 | 0.50 | 190.5 |
| bigspace/low_precision/ms | 0.107 | 0.310 | 0.74 | 184.2 |
| physics/step/ms | 0.097 | 0.357 | 0.70 | 166.7 |
| controller/feed/ms | 0.086 | 0.111 | 0.24 | 148.9 |
| stage/render/prepare_batch_phases | 0.086 | 0.133 | 0.36 | 148.5 |
| anim/evaluate/ms | 0.070 | 0.161 | 0.71 | 120.3 |
| bigspace/high_precision/ms | 0.055 | 0.227 | 0.73 | 94.4 |
| stage/render/specialize | 0.053 | 0.081 | 0.32 | 91.7 |
| stage/render/phase_sort | 0.048 | 0.075 | 0.21 | 82.6 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.071 | 0.33 | 81.6 |
| stage/render/queue | 0.040 | 0.069 | 0.44 | 69.7 |
| encode/resolve_diagnostics/sum | 0.039 | 0.056 | 5.22 | 67.3 |
| anim/evaluate_cpu/ms | 0.038 | 0.058 | 0.20 | 64.8 |
| controller/write/ms | 0.026 | 0.035 | 0.33 | 45.0 |
| weather/map/ms | 0.025 | 0.034 | 0.06 | 43.1 |
| stage/render/cleanup | 0.021 | 0.031 | 0.19 | 36.0 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.46 | 23.2 |
| encode/camera_driver_span_end/sum | 0.012 | 0.021 | 0.24 | 20.3 |
| weather/eval/ms | 0.011 | 0.016 | 0.04 | 19.7 |
| physics/insert/ms | 0.011 | 0.000 | 0.74 | 18.8 |
| script/update/ms | 0.008 | 0.011 | 0.28 | 13.9 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 9.0 |
| anim/apply/ms | 0.005 | 0.010 | 0.13 | 8.8 |
| far/select/ms | 0.004 | 0.005 | 0.03 | 6.9 |
| script/ui/ms | 0.003 | 0.005 | 0.13 | 6.0 |
| controller/move/ms | 0.003 | 0.009 | 0.05 | 5.5 |
| encode/pass:point_shadow_atlas_static_capture/sum | 0.002 | 0.000 | 2.13 | 4.0 |
| controller/avoid/ms | 0.002 | 0.005 | 0.06 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.4 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 1.1 |
| weather/apply/ms | 0.001 | 0.001 | 0.03 | 1.1 |
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
| encode//max | ms | 0.43 | 73.39 | 0.37 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.46 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.24 | 0.01 |
| encode/finish_critical | ms | 0.51 | 73.39 | 0.37 |
| encode/pass:early prepass/max | ms | 0.46 | 72.97 | 0.35 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.43 | 6.06 | 0.34 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.12 | 5.39 | 0.11 |
| encode/pass:point_shadow_atlas_live_movers/max | ms | 0.13 | 5.24 | 0.11 |
| encode/pass:point_shadow_atlas_static_capture/max | ms | 2.02 | 2.13 | 1.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 5.22 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.82 | 78.55 | 5.31 |
| gpu/timed/ms | ms | 5.71 | 7.52 | 5.29 |
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
| phases/opaque3d/bins | count | 161 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 262 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 6 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 161 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 262 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 0 | 69 | 0 |
| phases/shadow/bins | count | 0 | 1034 | 0 |
| phases/shadow/instance_batches | count | 0 | 24 | 0 |
| phases/shadow/instance_draws | count | 0 | 9660 | 0 |
| phases/shadow/instance_slots | count | 0 | 294.9 k | 0 |
| phases/shadow/items | count | 0 | 1374 | 0 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 0 | 114 | 0 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 96 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 293 | 293 | 293 |
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
| preprocess/bind_groups_reused | count | 47 | 61 | 35 |
| process/cpu_usage | % | 26.92 | 36.06 | 25.48 |
| process/mem_usage | GiB | 3.62 | 3.64 | 3.64 |
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
| shadow_atlas/alloc_failures | count | 681 | 722 | 722 |
| shadow_atlas/captures | count | 0 | 6 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 24 | 24 | 24 |
| shadow_atlas/live_occ_pct | % | 17.19 | 83.98 | 9.38 |
| shadow_atlas/live_tiles | count | 7 | 29 | 6 |
| shadow_atlas/mover_draws | count | 6 | 12 | 3 |
| shadow_atlas/movers | count | 348 | 349 | 348 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 6 | 12 | 3 |
| shadow_atlas/render/clears | count | 0 | 6 | 0 |
| shadow_atlas/render/lamps | count | 24 | 24 | 24 |
| shadow_atlas/render/live_direct | count | 6 | 12 | 3 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 6 | 0 |
| shadow_atlas/render/views | count | 9 | 58 | 11 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 94.92 | 94.92 | 94.92 |
| shadow_atlas/static_tiles | count | 144 | 144 | 144 |
| shadow_atlas/views | count | 9 | 52 | 11 |
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
| system/cpu_usage | % | 34.82 | 55.47 | 31.51 |
| system/mem_usage | % | 50.52 | 50.59 | 50.58 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 23 | 95 | 17 |
| views/all/bins | count | 332 | 1606 | 58 |
| views/all/count | count | 10 | 59 | 12 |
| views/all/instance_batches | count | 8 | 32 | 8 |
| views/all/instance_draws | count | 3220 | 12.9 k | 3220 |
| views/all/instance_slots | count | 98.3 k | 393.2 k | 98.3 k |
| views/all/items | count | 548 | 2422 | 86 |
| views/all/pipelines | count | 21 | 141 | 16 |
| views/all/pipelines_distinct | count | 21 | 34 | 16 |
| views/all/work_items | count | 98.9 k | 394.7 k | 98.4 k |
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
| views/lamp_face/bins | count | 0 | 1034 | 0 |
| views/lamp_face/count | count | 9 | 58 | 11 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 24 | 0 |
| views/lamp_face/instance_draws | count | 0 | 9660 | 0 |
| views/lamp_face/instance_slots | count | 0 | 294.9 k | 0 |
| views/lamp_face/items | count | 0 | 1374 | 0 |
| views/lamp_face/pipelines | count | 0 | 114 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 295.3 k | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 332 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 548 | 1062 | 86 |
| views/main/pipelines | count | 21 | 27 | 16 |
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
| anim/bones_written/count | count | 9.13 | 30 | 15.8 k |
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
| encode//count | count | 19.38 | 20 | 33.5 k |
| encode/bind_walk/init_textures | count | 4149 | 12.6 k | 7.16 M |
| encode/bind_walk/init_textures_skipped | count | 6983 | 25.4 k | 12.06 M |
| encode/bind_walk/inits | count | 157 | 303 | 270.8 k |
| encode/bind_walk/inits_skipped | count | 16.12 | 40 | 27.8 k |
| encode/bind_walk/merge_textures | count | 2906 | 11.4 k | 5.02 M |
| encode/bind_walk/merge_textures_skipped | count | 51.80 | 14.1 k | 89.5 k |
| encode/bind_walk/merges | count | 43.64 | 71 | 75.4 k |
| encode/bind_walk/merges_skipped | count | 11.39 | 54 | 19.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1727 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1727 |
| encode/pass:early prepass/count | count | 1 | 1 | 1727 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1727 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1679 |
| encode/pass:point_shadow_atlas_live_movers/count | count | 0.87 | 1 | 1505 |
| encode/pass:point_shadow_atlas_static_capture/count | count | 0.00 | 1 | 2 |
| encode/queue/command_buffers | count | 29.22 | 30 | 50.5 k |
| encode/queue/submits | count | 2 | 2 | 3454 |
| encode/queue/write_buffer/bytes | bytes | 592.4 k | 1.01 M | 1.02 G |
| encode/queue/write_buffer/calls | count | 98.91 | 212 | 170.8 k |
| encode/queue/write_buffer_with/bytes | bytes | 271.9 k | 271.9 k | 469.52 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 10.4 k |
| encode/queue/write_texture/bytes | bytes | 607 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1727 |
| physics/inserts | count | 0.06 | 10 | 98 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.40 | 3 | 684 |
| physics/steps_dropped | count | 0.00 | 2 | 2 |
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
| warm-up | 2.57 | 2568.5 | main/busy/ms 2538, stage/main/post_update 2431, stage/main/update 95.61, surface/advance_cpu_ms 67.52, stage/main/pre_update 7.09, stage/main/last 1.25, stage/main/first 0.58, stream/drive/ms 0.48, stage/main/fixed_main_loop 0.39, stream/tick/ms 0.16, weather/map/ms 0.12, bigspace/total/ms 0.07 |
| warm-up | 3.39 | 822.3 | main/busy/ms 2666, stage/main/post_update 2437, render/thread/ms 783, render/busy/ms 575, stage/render/prepare_assets 531, stage/main/update 212, stage/render/prepare_views 206, render/acquire/ms 206, surface/advance_cpu_ms 67.52, stage/render/create_views 31.37, stage/main/pre_update 8.85, stage/render/render 8.42 |
| warm-up | 3.44 | 49.5 | render/thread/ms 817, render/busy/ms 608, stage/render/prepare_assets 559, stage/render/prepare_views 207, render/acquire/ms 206, main/busy/ms 140, stage/main/update 121, stage/render/create_views 31.64, stage/render/render 12.46, render/graph/ms 8.60, stage/main/post_update 8.51, stage/main/fixed_main_loop 6.63 |
| warm-up | 3.48 | 40.0 | pipelines/wait_ms 195, pipelines/compile_ms 157, pipelines/front_ms 143, render/thread/ms 63.56, main/busy/ms 43.90, render/busy/ms 40.13, stage/render/render 30.25, stage/render/prepare_assets 29.29, render/present/ms 22.78, stage/main/post_update 17.22, pipelines/driver_ms 13.95, stage/main/last 12.80 |
| warm-up | 3.51 | 31.7 | pipelines/wait_ms 1301, pipelines/compile_ms 492, pipelines/front_ms 457, main/busy/ms 73.13, render/thread/ms 41.70, pipelines/driver_ms 34.23, stage/render/render 29.38, render/present/ms 22.65, stage/main/post_update 21.86, stage/main/update 19.14, render/busy/ms 18.40, pipelines/lock_ms 16.70 |
| warm-up | 3.58 | 68.3 | pipelines/wait_ms 1240, pipelines/compile_ms 683, pipelines/front_ms 650, main/busy/ms 92.15, render/thread/ms 55.70, render/busy/ms 53.86, stage/main/post_update 41.22, pipelines/driver_ms 33.45, stage/main/update 19.77, stage/render/prepare_flush 19.05, stage/main/pre_update 17.33, stage/render/prepare_assets 14.30 |
| warm-up | 3.61 | 31.5 | pipelines/compile_ms 822, pipelines/front_ms 787, pipelines/wait_ms 197, main/busy/ms 119, render/thread/ms 60.26, render/busy/ms 58.64, stage/main/update 50.37, stage/main/post_update 37.68, pipelines/driver_ms 34.20, stage/main/pre_update 21.59, stage/render/prepare_flush 19.35, stage/render/prepare_assets 16.91 |
| warm-up | 3.73 | 120.3 | pipelines/compile_ms 1217, pipelines/front_ms 1174, pipelines/wait_ms 99.83, render/thread/ms 85.77, main/busy/ms 84.50, render/busy/ms 83.36, stage/render/prepare_assets 54.68, stage/main/update 54.59, pipelines/driver_ms 43.51, stage/render/render 20.23, render/graph/ms 17.61, stage/render/graph_submit 12.55 |
| warm-up | 3.80 | 44.1 | pipelines/compile_ms 732, pipelines/front_ms 699, render/thread/ms 59.55, render/busy/ms 58.15, stage/render/prepare_assets 48.39, main/busy/ms 46.52, pipelines/driver_ms 32.63, stage/main/update 24.50, stream/drive/ms 18.17, stage/main/post_update 8.83, stage/render/render 6.87, render/graph/ms 5.25 |
| warm-up | 3.84 | 41.1 | pipelines/compile_ms 529, pipelines/front_ms 508, render/thread/ms 71.87, render/busy/ms 70.30, stage/render/prepare_assets 61.40, main/busy/ms 47.67, stage/main/update 32.12, pipelines/driver_ms 21.37, stream/drive/ms 17.26, stage/main/post_update 10.33, stage/render/render 6.33, pipelines/wait_ms 4.79 |
| warm-up | 3.89 | 48.7 | pipelines/compile_ms 756, pipelines/front_ms 720, main/busy/ms 148, stage/main/update 134, render/thread/ms 74.94, render/busy/ms 72.14, stage/render/prepare_assets 44.48, pipelines/driver_ms 36.08, stage/render/render 25.83, render/graph/ms 22.83, stage/render/graph_submit 10.41, stage/main/post_update 8.63 |
| warm-up | 4.09 | 208.3 | pipelines/compile_ms 2173, pipelines/front_ms 2101, main/busy/ms 171, stage/main/update 158, render/thread/ms 120, render/busy/ms 117, pipelines/wait_ms 105, pipelines/driver_ms 72.69, stage/render/prepare_assets 65.70, stage/render/render 26.44, stage/render/prepare_resources 24.49, render/graph/ms 23.19 |
| warm-up | 4.16 | 63.1 | pipelines/compile_ms 1950, pipelines/front_ms 1893, render/thread/ms 118, render/busy/ms 116, pipelines/wait_ms 108, stage/render/prepare_assets 81.72, main/busy/ms 79.29, stage/main/update 66.08, pipelines/driver_ms 56.75, stream/drive/ms 27.37, stage/render/prepare_resources 24.27, pipelines/lock_ms 18.12 |
| warm-up | 4.19 | 29.4 | pipelines/compile_ms 268, pipelines/front_ms 257, render/thread/ms 61.55, render/busy/ms 59.79, main/busy/ms 56.97, stage/render/prepare_assets 43.42, stage/main/update 40.45, stream/drive/ms 27.41, pipelines/driver_ms 11.64, stage/render/render 11.62, render/graph/ms 9.38, stage/main/post_update 8.35 |
| warm-up | 4.22 | 36.1 | pipelines/compile_ms 655, pipelines/front_ms 630, render/thread/ms 52.37, render/busy/ms 48.91, main/busy/ms 47.03, stage/main/update 31.97, pipelines/wait_ms 25.01, pipelines/driver_ms 24.85, stage/render/prepare_assets 24.67, stream/drive/ms 11.27, stage/render/render 10.05, stage/main/post_update 9.11 |
| warm-up | 4.26 | 32.1 | pipelines/compile_ms 485, pipelines/front_ms 466, render/thread/ms 54.43, render/busy/ms 51.24, main/busy/ms 47.67, stage/render/prepare_assets 27.53, stage/main/update 25.85, pipelines/wait_ms 22.79, pipelines/driver_ms 18.85, stream/drive/ms 11.31, stage/main/pre_update 11.18, stage/render/render 8.77 |
| warm-up | 4.29 | 33.0 | pipelines/compile_ms 734, pipelines/front_ms 709, pipelines/wait_ms 64.79, main/busy/ms 51.93, render/thread/ms 50.68, render/busy/ms 49.31, stage/main/post_update 27.51, stage/render/prepare_assets 25.61, pipelines/driver_ms 25.02, stage/main/pre_update 12.99, stage/render/render 8.78, stage/main/update 8.54 |
| warm-up | 4.48 | 29.7 | pipelines/compile_ms 387, pipelines/front_ms 374, render/thread/ms 43.62, render/busy/ms 42.39, main/busy/ms 37.24, stage/main/update 21.13, stage/render/prepare_flush 14.31, stage/render/prepare_assets 13.65, pipelines/driver_ms 13.29, stage/main/post_update 8.49, stage/render/prepare_resources 6.46, stage/render/render 6.26 |
| warm-up | 4.61 | 42.4 | pipelines/compile_ms 457, pipelines/front_ms 442, main/busy/ms 58.94, render/thread/ms 53.39, render/busy/ms 51.69, stage/main/update 27.23, pipelines/wait_ms 21.31, stage/main/post_update 18.25, stage/render/prepare_assets 16.26, pipelines/driver_ms 14.74, stage/render/render 12.74, stage/render/prepare_bind_groups 11.71 |
| warm-up | 4.65 | 43.8 | pipelines/compile_ms 1127, pipelines/front_ms 1085, main/busy/ms 72.82, render/thread/ms 66.61, render/busy/ms 64.82, stage/main/update 51.50, pipelines/wait_ms 48.39, pipelines/driver_ms 42.19, stage/render/prepare_assets 26.45, stage/render/render 15.72, stage/main/post_update 14.38, render/graph/ms 13.51 |
| warm-up | 4.70 | 49.1 | pipelines/compile_ms 1246, pipelines/front_ms 1201, main/busy/ms 91.92, stage/main/update 71.82, render/thread/ms 64.55, render/busy/ms 62.98, pipelines/driver_ms 45.44, stream/drive/ms 33.68, pipelines/wait_ms 32.57, stage/render/prepare_assets 28.26, stage/render/render 15.61, render/graph/ms 13.34 |
| warm-up | 4.77 | 70.4 | pipelines/compile_ms 1036, pipelines/front_ms 994, main/busy/ms 100, render/thread/ms 69.97, render/busy/ms 67.14, stage/main/update 64.29, stream/drive/ms 42.72, pipelines/driver_ms 41.76, stage/render/render 27.71, stage/main/post_update 25.79, stage/render/prepare_assets 25.62, render/graph/ms 23.84 |
| warm-up | 4.84 | 65.7 | pipelines/compile_ms 982, pipelines/front_ms 936, main/busy/ms 94.41, render/thread/ms 84.49, render/busy/ms 81.26, stage/main/update 55.99, pipelines/driver_ms 45.72, stage/render/prepare_assets 31.65, stage/render/render 30.94, stage/main/post_update 27.98, render/graph/ms 26.51, stream/drive/ms 20.96 |
| warm-up | 4.93 | 97.5 | pipelines/compile_ms 612, pipelines/front_ms 590, render/thread/ms 108, render/busy/ms 106, main/busy/ms 71.26, stage/render/prepare_assets 50.09, stage/main/update 45.40, stage/render/prepare_flush 26.94, pipelines/driver_ms 21.59, stage/main/post_update 15.74, stage/render/render 14.27, stream/drive/ms 13.31 |
| warm-up | 4.98 | 45.2 | pipelines/compile_ms 242, pipelines/front_ms 238, render/thread/ms 93.04, render/busy/ms 92.13, main/busy/ms 55.92, stage/render/prepare_assets 48.15, stage/main/update 37.22, stage/render/prepare_flush 25.44, stream/drive/ms 21.58, stage/render/render 9.84, stage/main/post_update 9.72, render/graph/ms 8.29 |
| warm-up | 5.01 | 28.4 | render/thread/ms 55.83, render/busy/ms 52.58, main/busy/ms 52.18, stage/main/update 32.41, stage/render/prepare_assets 22.57, stream/drive/ms 21.23, stage/render/render 19.16, render/graph/ms 15.55, stage/main/post_update 13.35, encode//sum 13.20, stage/render/graph_submit 12.26, encode/finish_wall 7.71 |
| warm-up | 5.06 | 48.4 | render/thread/ms 66.74, render/busy/ms 63.20, main/busy/ms 55.09, stage/main/post_update 20.93, stage/main/update 19.97, stage/render/render 19.97, render/graph/ms 16.12, encode//sum 14.02, stage/render/graph_submit 12.47, stage/render/prepare_write_phase_buffers 8.84, encode/finish_wall 7.94, stage/render/prepare_assets 7.11 |
| warm-up | 5.09 | 32.1 | pipelines/compile_ms 163, pipelines/front_ms 151, render/thread/ms 71.48, render/busy/ms 67.13, main/busy/ms 55.55, stage/main/update 19.94, stage/main/post_update 19.38, stage/render/render 16.57, stage/render/prepare_resources 14.54, pipelines/driver_ms 11.77, render/graph/ms 11.61, stage/render/prepare_write_phase_buffers 8.92 |
| warm-up | 5.12 | 27.7 | pipelines/compile_ms 865, pipelines/front_ms 829, render/thread/ms 55.05, render/busy/ms 50.41, main/busy/ms 45.67, pipelines/driver_ms 36.42, stage/main/update 19.33, stage/render/render 19.23, stage/main/post_update 16.93, render/graph/ms 13.91, stage/render/prepare_resources 12.46, stage/render/graph_submit 8.53 |
| warm-up | 5.15 | 29.5 | pipelines/compile_ms 1370, pipelines/front_ms 1325, render/thread/ms 49.99, render/busy/ms 47.80, pipelines/driver_ms 44.60, main/busy/ms 35.62, stage/render/render 22.34, render/graph/ms 19.68, stage/main/update 16.54, pipelines/wait_ms 11.84, stage/main/post_update 11.75, stage/render/graph_submit 11.62 |
| warm-up | 5.21 | 38.2 | pipelines/compile_ms 146, pipelines/front_ms 139, render/thread/ms 57.84, render/busy/ms 56.28, main/busy/ms 35.85, stage/render/prepare_assets 21.85, stage/main/post_update 20.15, stage/render/render 18.58, render/graph/ms 16.47, stage/render/graph_submit 12.78, stage/main/update 10.35, encode/finish_wall 9.56 |
| warm-up | 5.25 | 45.3 | pipelines/compile_ms 532, pipelines/front_ms 515, render/thread/ms 76.86, render/busy/ms 75.53, main/busy/ms 38.47, stage/render/prepare_assets 29.57, stage/main/post_update 22.63, pipelines/wait_ms 21.09, stage/render/render 18.80, pipelines/driver_ms 16.94, render/graph/ms 16.84, stage/render/graph_submit 13.37 |
| warm-up | 5.28 | 29.2 | pipelines/compile_ms 1003, pipelines/front_ms 975, render/thread/ms 62.51, render/busy/ms 61.22, main/busy/ms 31.22, pipelines/driver_ms 28.30, pipelines/wait_ms 23.12, stage/render/prepare_assets 17.18, stage/render/render 15.09, stage/main/post_update 14.99, render/graph/ms 13.19, stage/render/graph_submit 9.59 |
| warm-up | 5.37 | 39.2 | pipelines/compile_ms 467, pipelines/front_ms 452, pipelines/wait_ms 57.74, main/busy/ms 53.02, render/thread/ms 44.56, render/busy/ms 43.02, stage/main/update 34.16, stage/render/render 18.88, render/graph/ms 16.50, pipelines/driver_ms 15.28, stage/main/post_update 14.02, stage/render/graph_submit 11.64 |
| warm-up | 5.42 | 43.9 | pipelines/compile_ms 594, pipelines/front_ms 576, render/thread/ms 58.40, pipelines/wait_ms 57.76, render/busy/ms 57.01, main/busy/ms 49.27, stage/main/update 31.89, stage/render/prepare_assets 25.35, pipelines/driver_ms 18.39, stage/render/render 16.47, render/graph/ms 14.28, stage/main/post_update 10.11 |
| warm-up | 5.45 | 37.1 | pipelines/compile_ms 127, pipelines/front_ms 124, render/thread/ms 52.63, render/busy/ms 51.64, stage/render/prepare_assets 32.20, main/busy/ms 27.26, stage/main/update 13.54, stage/render/render 9.88, render/graph/ms 8.42, stage/main/post_update 6.87, stage/render/graph_submit 5.71, encode//sum 4.53 |
| measure | 4.41 | 80.0 | encode//sum 444, render/thread/ms 85.88, render/busy/ms 84.92, stage/render/render 81.84, render/graph/ms 80.73, stage/render/graph_submit 78.62, encode/finish_wall 74.45, encode/pass:early prepass/sum 73.51, main/busy/ms 10.87, stage/main/post_update 6.44, encode/submit 4.22, stage/main/update 2.50 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set shadow.lamp.atlas.enabled=on --set shadow.lamp.atlas.max_lamps=48 --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=atlas48-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 17%
- during the run: other processes 8% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **96%** (min 92%) · 2625 MHz (min 2625) · memory 8101 MHz · 98.1 W · 74 °C (max 77) · VRAM 5418 MiB · throttled 98% of samples

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
