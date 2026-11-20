# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1592 frames over 11.4 s · **median 6.82 ms (147 fps)** · mean 7.18 · p95 9.89 · p99 12.17 (1% low 82 fps) · worst 19.75 ms · 58 spikes over 1.5× median · 57.7 stutters/min (work 10.5/min)
- warm-up: 120 frames over 6.4 s, worst 2579.3 ms
- hitches (> 25.0 ms): **0** measured, 35 in warm-up
- timed GPU passes: 5.13 ms of a 7.18 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 146.7, target 144
- pass worst frame 19.75 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.916 | 2.160 | 2.509 | 0.89 | 0.164 |
| main_opaque_pass_3d | 1.750 | 1.973 | 2.891 | 0.89 | 0.036 |
| bloom | 0.305 | 0.344 | 0.503 | 0.89 | 0.064 |
| auxiliary_views | 0.301 | 0.340 | 1.147 | 0.89 | 0.282 |
| ssao | 0.190 | 0.214 | 0.543 | 0.89 | 0.007 |
| upscaling | 0.146 | 0.165 | 0.184 | 0.89 | 0.004 |
| tlas_build | 0.103 | 0.116 | 0.118 | 0.89 | 0.076 |
| tonemapping | 0.093 | 0.105 | 0.113 | 0.89 | 0.004 |
| gpu_instance_materialization | 0.084 | 0.095 | 0.098 | 0.89 | 0.004 |
| gi/probe_update | 0.084 | 0.094 | 0.097 | 0.89 | 0.002 |
| early prepass | 0.067 | 0.076 | 0.085 | 0.89 | 0.045 |
| clustering | 0.066 | 0.075 | 0.077 | 0.89 | 0.019 |
| uniform_allocation | 0.035 | 0.040 | 0.044 | 0.89 | 0.009 |
| bin_unpacking | 0.028 | 0.031 | 0.039 | 0.89 | 0.008 |
| range_unpacking | 0.017 | 0.020 | 0.020 | 0.89 | 0.002 |
| early_mesh_preprocessing | 0.016 | 0.018 | 0.025 | 0.89 | 0.003 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.012 | 0.86 | 0.023 |
| setup_tlas_instances | 0.006 | 0.007 | 0.007 | 0.89 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.28 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.001 | 0.89 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 7.766 | 0.000 | 6920.09 | 12363.2 |
| pipelines/driver_ms | 7.309 | 0.000 | 6602.76 | 11635.9 |
| render/thread/ms | 6.063 | 8.746 | 18.66 | 9652.6 |
| render/busy/ms | 5.542 | 8.285 | 16.40 | 8823.4 |
| main/busy/ms | 5.347 | 6.211 | 9.85 | 8513.0 |
| stage/render/render | 3.864 | 5.810 | 16.25 | 6151.2 |
| render/graph/ms | 3.216 | 5.070 | 13.88 | 5120.4 |
| stage/main/post_update | 2.845 | 3.385 | 5.00 | 4529.5 |
| encode//sum | 2.506 | 4.098 | 8.24 | 3989.2 |
| pipelines/wait_ms | 2.447 | 0.000 | 1330.70 | 3896.0 |
| stage/render/graph_submit | 2.087 | 3.339 | 12.50 | 3322.1 |
| encode/submit | 1.477 | 2.499 | 11.42 | 2350.7 |
| stage/main/update | 1.232 | 1.521 | 5.40 | 1961.9 |
| stage/render/camera_driver | 0.992 | 1.632 | 2.45 | 1579.2 |
| encode/pass:shadow_point_light/sum | 0.873 | 2.806 | 4.71 | 1389.6 |
| encode/finish_wall | 0.673 | 1.033 | 1.70 | 1072.0 |
| pipelines/front_ms | 0.457 | 0.000 | 317.33 | 727.4 |
| render/present/ms | 0.409 | 0.837 | 13.02 | 650.4 |
| stage/render/prepare_resources | 0.407 | 0.508 | 0.97 | 647.4 |
| stage/render/prepare_write_phase_buffers | 0.404 | 0.826 | 1.42 | 642.6 |
| encode/pass:early prepass/sum | 0.371 | 0.527 | 0.81 | 589.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.347 | 0.514 | 1.05 | 553.1 |
| stage/main/pre_update | 0.347 | 0.436 | 0.86 | 552.9 |
| stage/main/first | 0.342 | 0.433 | 1.17 | 543.9 |
| stage/render/prepare_bind_groups | 0.318 | 0.653 | 1.37 | 506.4 |
| stage/main/fixed_main_loop | 0.253 | 0.615 | 0.90 | 402.1 |
| bigspace/total/ms | 0.219 | 0.485 | 0.99 | 348.3 |
| stage/render/prepare_assets | 0.211 | 0.280 | 0.52 | 336.1 |
| stage/render/prepare_meshes | 0.201 | 0.281 | 0.53 | 319.7 |
| stage/render/prepare_flush | 0.179 | 0.345 | 3.63 | 284.6 |
| stage/render/create_views | 0.169 | 0.218 | 0.66 | 269.5 |
| stage/main/last | 0.154 | 0.205 | 1.08 | 245.5 |
| stage/render/prepare_views | 0.153 | 0.473 | 1.09 | 243.7 |
| stage/render/extract_commands | 0.136 | 0.172 | 0.48 | 216.6 |
| stream/drive/ms | 0.130 | 0.182 | 0.39 | 207.0 |
| render/acquire/ms | 0.112 | 0.423 | 1.06 | 178.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.108 | 0.158 | 0.64 | 172.6 |
| stream/tick/ms | 0.103 | 0.150 | 0.34 | 164.6 |
| bigspace/low_precision/ms | 0.094 | 0.253 | 0.74 | 148.9 |
| stage/render/prepare_batch_phases | 0.089 | 0.144 | 0.38 | 141.0 |
| controller/feed/ms | 0.080 | 0.102 | 0.29 | 127.0 |
| physics/step/ms | 0.073 | 0.235 | 0.52 | 116.8 |
| anim/evaluate/ms | 0.067 | 0.154 | 1.18 | 107.0 |
| stage/render/specialize | 0.067 | 0.089 | 0.50 | 106.8 |
| stage/render/phase_sort | 0.054 | 0.087 | 0.36 | 86.3 |
| bigspace/high_precision/ms | 0.050 | 0.131 | 0.64 | 78.8 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.069 | 0.15 | 72.8 |
| stage/render/queue | 0.041 | 0.074 | 0.49 | 65.1 |
| anim/evaluate_cpu/ms | 0.034 | 0.052 | 0.35 | 54.9 |
| encode/resolve_diagnostics/sum | 0.030 | 0.043 | 0.45 | 48.3 |
| controller/write/ms | 0.027 | 0.039 | 0.09 | 42.4 |
| weather/map/ms | 0.026 | 0.038 | 0.18 | 40.7 |
| stage/render/cleanup | 0.023 | 0.035 | 0.22 | 36.1 |
| physics/insert/ms | 0.015 | 0.000 | 0.62 | 23.5 |
| weather/eval/ms | 0.012 | 0.019 | 0.27 | 19.6 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.020 | 0.20 | 19.5 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.37 | 16.4 |
| script/update/ms | 0.008 | 0.012 | 0.04 | 12.3 |
| script/input/ms | 0.005 | 0.006 | 0.05 | 7.9 |
| anim/apply/ms | 0.005 | 0.010 | 0.16 | 7.5 |
| controller/move/ms | 0.004 | 0.012 | 0.03 | 6.4 |
| far/select/ms | 0.004 | 0.005 | 0.04 | 6.4 |
| script/ui/ms | 0.003 | 0.005 | 0.03 | 5.2 |
| controller/avoid/ms | 0.002 | 0.005 | 0.01 | 3.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.3 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 1.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 1.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.45 | 0.7 |
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
| encode//max | ms | 0.38 | 1.26 | 0.38 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.20 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.37 | 0.01 |
| encode/finish_critical | ms | 0.46 | 1.26 | 0.41 |
| encode/pass:early prepass/max | ms | 0.37 | 0.81 | 0.29 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.34 | 1.05 | 0.41 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.64 | 0.13 |
| encode/pass:shadow_point_light/max | ms | 0.44 | 0.85 | 0.39 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.45 | 0.03 |
| far/entities | count | 228 | 228 | 228 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.3 k | 41.3 k | 41.3 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.47 M | 86.47 M | 86.47 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.74 | 16.40 | 4.87 |
| gpu/timed/ms | ms | 5.60 | 8.23 | 4.87 |
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
| phases/opaque3d/bins | count | 144 | 282 | 26 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 251 | 507 | 40 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 8 | 11 | 5 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 6 | 9 | 5 |
| phases/opaque3d_prepass/bins | count | 144 | 282 | 26 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1433 | 1433 | 1433 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 251 | 507 | 40 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 5 | 8 | 4 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 2 | 90 | 0 |
| phases/shadow/bins | count | 2638 | 2866 | 2173 |
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
| physics/pending | count | 0 | 125 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 293 | 293 | 293 |
| pipelines/waiting | count | 0 | 7 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 0 | 2 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 8 | 196 | 4 |
| preprocess/bind_groups_created | count | 4 | 192 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 4 | 4 | 4 |
| preprocess/bind_groups_reused | count | 47 | 93 | 35 |
| process/cpu_usage | % | 25.48 | 47.12 | 29.47 |
| process/mem_usage | GiB | 4.01 | 4.09 | 4.09 |
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
| stream/latency/s | s | 0.09 | 0.09 | 0.09 |
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
| system/cpu_usage | % | 32.37 | 62.79 | 32.55 |
| system/mem_usage | % | 51.72 | 52.09 | 52.09 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2031 | 2031 | 2031 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 26.50 | 119 | 17 |
| views/all/bins | count | 2981 | 3313 | 2231 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 8 | 8 | 8 |
| views/all/instance_draws | count | 3220 | 3220 | 3220 |
| views/all/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/all/items | count | 4168 | 4586 | 3191 |
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
| views/lamp_face/batch_sets | count | 2 | 90 | 0 |
| views/lamp_face/bins | count | 2638 | 2866 | 2173 |
| views/lamp_face/count | count | 48 | 48 | 48 |
| views/lamp_face/culled | count | 25 | 36 | 35 |
| views/lamp_face/drawn | count | 0.50 | 14 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3603 | 3839 | 3105 |
| views/lamp_face/pipelines | count | 227 | 240 | 189 |
| views/lamp_face/revealed | count | 0 | 1 | 0 |
| views/lamp_face/work_items | count | 37.50 | 1370 | 0 |
| views/main/batch_sets | count | 23 | 29 | 17 |
| views/main/bins | count | 298 | 582 | 58 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 3220 | 3220 | 3220 |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 526 | 1062 | 86 |
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
| anim/bones_written/count | count | 8.25 | 30 | 13.1 k |
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
| encode//count | count | 31.43 | 75 | 50.0 k |
| encode/bind_walk/init_textures | count | 8293 | 23.8 k | 13.20 M |
| encode/bind_walk/init_textures_skipped | count | 10.6 k | 29.6 k | 16.81 M |
| encode/bind_walk/inits | count | 216 | 466 | 343.9 k |
| encode/bind_walk/inits_skipped | count | 15.55 | 44 | 24.7 k |
| encode/bind_walk/merge_textures | count | 7050 | 22.6 k | 11.22 M |
| encode/bind_walk/merge_textures_skipped | count | 3894 | 18.4 k | 6.20 M |
| encode/bind_walk/merges | count | 59.01 | 126 | 93.9 k |
| encode/bind_walk/merges_skipped | count | 19.40 | 75 | 30.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1592 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1592 |
| encode/pass:early prepass/count | count | 1 | 1 | 1592 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1592 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 1 | 1543 |
| encode/pass:shadow_point_light/count | count | 3.03 | 14 | 4826 |
| encode/queue/command_buffers | count | 43.43 | 98 | 69.1 k |
| encode/queue/submits | count | 2 | 2 | 3184 |
| encode/queue/write_buffer/bytes | bytes | 600.3 k | 692.6 k | 955.62 M |
| encode/queue/write_buffer/calls | count | 159 | 393 | 252.7 k |
| encode/queue/write_buffer_with/bytes | bytes | 149.6 k | 149.6 k | 238.21 M |
| encode/queue/write_buffer_with/calls | count | 6 | 6 | 9552 |
| encode/queue/write_texture/bytes | bytes | 659 | 1.05 M | 1.05 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 1 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1592 |
| physics/inserts | count | 0.08 | 10 | 126 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.43 | 2 | 686 |
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
| warm-up | 2.58 | 2579.3 | main/busy/ms 2548, stage/main/post_update 2440, stage/main/update 96.80, surface/advance_cpu_ms 69.41, stage/main/pre_update 7.17, stage/main/last 1.26, stream/drive/ms 0.59, stage/main/first 0.55, stage/main/fixed_main_loop 0.37, weather/map/ms 0.17, stream/tick/ms 0.12, bigspace/total/ms 0.08 |
| warm-up | 3.31 | 733.8 | main/busy/ms 2666, stage/main/post_update 2446, render/thread/ms 691, render/busy/ms 558, stage/render/prepare_assets 536, stage/main/update 203, stage/render/prepare_views 132, render/acquire/ms 131, surface/advance_cpu_ms 69.41, stage/render/prepare_resources 11.48, stage/main/pre_update 8.62, stage/render/render 8.24 |
| warm-up | 3.36 | 49.5 | render/thread/ms 724, render/busy/ms 591, stage/render/prepare_assets 564, stage/render/prepare_views 132, main/busy/ms 132, render/acquire/ms 131, stage/main/update 112, stage/render/render 12.39, stage/render/prepare_resources 12.09, stage/main/post_update 9.18, render/graph/ms 9.00, stage/main/fixed_main_loop 7.30 |
| warm-up | 3.43 | 45.5 | pipelines/wait_ms 1138, pipelines/compile_ms 468, pipelines/front_ms 435, main/busy/ms 74.97, stage/main/update 40.13, pipelines/driver_ms 33.57, render/thread/ms 30.28, render/busy/ms 24.25, stage/main/post_update 20.21, stage/render/render 14.44, stage/main/pre_update 10.93, stage/render/prepare_assets 10.57 |
| warm-up | 3.46 | 33.7 | pipelines/wait_ms 1167, pipelines/compile_ms 551, pipelines/front_ms 517, main/busy/ms 70.17, render/thread/ms 35.62, pipelines/driver_ms 34.29, render/busy/ms 33.57, stage/main/update 23.01, stage/render/prepare_assets 19.00, stage/main/pre_update 17.68, stage/main/post_update 12.99, stage/render/render 10.75 |
| warm-up | 3.49 | 31.6 | pipelines/compile_ms 152, pipelines/front_ms 144, main/busy/ms 47.72, pipelines/wait_ms 42.29, render/thread/ms 27.07, render/busy/ms 25.76, stage/main/pre_update 14.53, stage/main/post_update 13.09, stage/render/prepare_assets 12.93, stage/main/last 10.17, pipelines/driver_ms 8.34, stage/render/render 6.86 |
| warm-up | 3.52 | 26.3 | pipelines/compile_ms 328, pipelines/front_ms 313, pipelines/wait_ms 36.62, render/thread/ms 29.40, render/busy/ms 27.93, main/busy/ms 23.89, pipelines/driver_ms 15.31, stage/render/prepare_assets 13.47, stage/main/post_update 10.13, stage/main/pre_update 7.71, stage/render/render 6.38, stage/render/prepare_resources 5.71 |
| warm-up | 3.56 | 41.1 | pipelines/compile_ms 1145, pipelines/front_ms 1089, pipelines/wait_ms 137, main/busy/ms 73.73, pipelines/driver_ms 55.14, render/thread/ms 53.97, stage/main/update 53.86, render/busy/ms 51.91, stage/render/render 22.24, render/graph/ms 20.00, stage/render/graph_submit 17.71, encode/submit 14.94 |
| warm-up | 3.64 | 85.9 | pipelines/compile_ms 1166, pipelines/front_ms 1110, pipelines/wait_ms 103, render/thread/ms 91.00, render/busy/ms 88.72, main/busy/ms 79.36, stage/main/update 61.22, pipelines/driver_ms 56.33, stage/render/prepare_assets 44.07, stage/render/render 27.97, render/graph/ms 25.64, stage/render/graph_submit 22.74 |
| warm-up | 3.70 | 28.1 | pipelines/compile_ms 745, pipelines/front_ms 712, render/thread/ms 45.26, render/busy/ms 43.46, pipelines/driver_ms 33.12, main/busy/ms 31.78, stage/render/prepare_assets 29.35, stage/main/update 9.02, stage/main/post_update 8.39, stage/render/render 7.12, stage/main/first 6.54, render/graph/ms 5.42 |
| warm-up | 3.72 | 26.5 | pipelines/compile_ms 722, pipelines/front_ms 689, render/thread/ms 49.86, render/busy/ms 47.15, main/busy/ms 39.04, pipelines/driver_ms 32.36, stage/render/prepare_assets 28.09, stage/main/update 26.64, stream/drive/ms 18.21, stage/render/render 15.56, render/graph/ms 12.86, stage/render/graph_submit 9.38 |
| warm-up | 3.77 | 52.2 | pipelines/compile_ms 637, pipelines/front_ms 615, render/thread/ms 62.38, render/busy/ms 59.87, main/busy/ms 51.67, stage/main/update 36.95, stage/render/prepare_assets 33.51, stage/render/render 22.42, pipelines/driver_ms 21.91, render/graph/ms 19.42, stream/drive/ms 16.79, stage/render/graph_submit 15.51 |
| warm-up | 3.84 | 61.6 | pipelines/compile_ms 748, pipelines/front_ms 717, main/busy/ms 148, stage/main/update 132, render/thread/ms 89.43, render/busy/ms 72.68, stage/render/render 40.72, stage/render/prepare_assets 40.72, pipelines/driver_ms 31.32, render/graph/ms 25.08, render/present/ms 14.89, stage/render/graph_submit 14.00 |
| warm-up | 4.03 | 197.9 | pipelines/compile_ms 2116, pipelines/front_ms 2047, main/busy/ms 157, stage/main/update 143, render/thread/ms 137, render/busy/ms 120, pipelines/wait_ms 118, stage/render/prepare_assets 96.62, pipelines/driver_ms 68.85, stage/render/render 33.19, render/graph/ms 17.88, render/present/ms 14.91 |
| warm-up | 4.08 | 42.1 | pipelines/compile_ms 2020, pipelines/front_ms 1959, pipelines/wait_ms 119, render/thread/ms 114, render/busy/ms 112, stage/render/prepare_assets 97.56, main/busy/ms 77.59, stage/main/update 62.04, pipelines/driver_ms 61.25, stream/drive/ms 33.73, stage/render/render 13.10, render/graph/ms 11.45 |
| warm-up | 4.13 | 53.0 | pipelines/compile_ms 198, pipelines/front_ms 189, main/busy/ms 66.31, render/thread/ms 55.79, render/busy/ms 54.12, stage/main/update 39.48, stage/render/prepare_assets 34.89, stream/drive/ms 33.87, pipelines/wait_ms 29.58, stage/main/post_update 20.84, stage/render/render 14.34, render/graph/ms 12.62 |
| warm-up | 4.19 | 34.7 | pipelines/compile_ms 1181, pipelines/front_ms 1126, pipelines/driver_ms 54.71, render/thread/ms 42.37, render/busy/ms 40.51, main/busy/ms 35.62, stage/render/prepare_assets 25.59, stage/main/update 18.38, stage/main/post_update 12.41, stream/drive/ms 11.58, stage/render/render 10.00, pipelines/wait_ms 9.51 |
| warm-up | 4.21 | 28.0 | pipelines/compile_ms 742, pipelines/front_ms 718, render/thread/ms 44.28, render/busy/ms 43.00, main/busy/ms 32.85, stage/render/prepare_assets 28.71, pipelines/wait_ms 25.63, pipelines/driver_ms 24.01, stage/main/post_update 15.06, stage/main/first 7.61, stage/main/update 7.02, stage/render/render 6.87 |
| warm-up | 4.37 | 35.3 | pipelines/compile_ms 316, pipelines/front_ms 304, main/busy/ms 49.25, render/thread/ms 41.35, render/busy/ms 39.99, stage/main/post_update 14.43, pipelines/driver_ms 12.79, stage/render/prepare_assets 12.42, stage/main/pre_update 11.81, stage/render/render 11.61, render/graph/ms 8.11, stage/main/update 7.58 |
| warm-up | 4.54 | 33.4 | main/busy/ms 37.27, render/thread/ms 35.72, render/busy/ms 34.30, stage/main/post_update 14.51, stage/main/update 14.12, stage/render/render 13.03, stage/render/prepare_assets 10.92, render/graph/ms 10.85, stage/render/graph_submit 7.24, encode/submit 5.68, encode//sum 3.79, stage/render/camera_driver 3.39 |
| warm-up | 4.58 | 37.1 | pipelines/compile_ms 246, pipelines/front_ms 235, render/thread/ms 55.39, main/busy/ms 54.73, render/busy/ms 53.41, stage/main/update 38.12, stage/render/render 19.71, render/graph/ms 16.73, stage/main/post_update 11.60, pipelines/driver_ms 11.14, stage/render/prepare_assets 9.72, stage/render/graph_submit 9.65 |
| warm-up | 4.64 | 59.4 | pipelines/compile_ms 385, pipelines/front_ms 369, main/busy/ms 93.71, stage/main/update 77.96, render/thread/ms 76.40, render/busy/ms 74.30, pipelines/wait_ms 59.63, stream/drive/ms 45.40, stage/render/prepare_assets 29.02, stage/render/render 21.32, encode//sum 21.09, render/graph/ms 18.61 |
| warm-up | 4.73 | 90.0 | pipelines/compile_ms 254, pipelines/front_ms 236, pipelines/wait_ms 112, render/thread/ms 106, render/busy/ms 104, main/busy/ms 101, stage/main/update 70.01, encode//sum 56.00, stream/drive/ms 45.78, stage/render/prepare_assets 36.74, stage/render/render 30.12, render/graph/ms 27.32 |
| warm-up | 4.79 | 63.0 | pipelines/compile_ms 114, render/thread/ms 114, render/busy/ms 111, pipelines/front_ms 102, main/busy/ms 84.32, pipelines/wait_ms 58.17, encode//sum 50.18, stage/main/update 49.20, stage/render/render 34.70, stage/render/prepare_assets 31.66, render/graph/ms 31.01, stage/main/post_update 22.81 |
| warm-up | 4.92 | 130.4 | pipelines/wait_ms 442, pipelines/compile_ms 150, render/thread/ms 143, render/busy/ms 141, pipelines/front_ms 140, stage/render/prepare_assets 63.05, main/busy/ms 61.17, stage/main/update 37.63, stage/render/create_views 31.33, stage/render/render 24.78, render/graph/ms 21.81, encode//sum 17.24 |
| warm-up | 4.99 | 66.2 | pipelines/wait_ms 1030, pipelines/compile_ms 233, pipelines/front_ms 219, render/thread/ms 137, render/busy/ms 136, stage/render/prepare_assets 65.96, main/busy/ms 43.44, stage/render/create_views 30.75, stage/main/update 23.03, stage/render/render 21.50, render/graph/ms 19.42, encode//sum 18.62 |
| warm-up | 5.05 | 61.0 | pipelines/wait_ms 899, pipelines/compile_ms 182, pipelines/front_ms 172, render/thread/ms 104, render/busy/ms 102, encode//sum 77.58, main/busy/ms 72.24, stage/main/update 41.91, stage/render/render 39.63, render/graph/ms 37.04, encode/pass:shadow_point_light/sum 31.93, stage/render/graph_submit 26.36 |
| warm-up | 5.09 | 42.2 | pipelines/wait_ms 311, pipelines/compile_ms 98.70, render/thread/ms 94.79, pipelines/front_ms 93.08, render/busy/ms 92.99, encode//sum 80.17, main/busy/ms 63.15, stage/render/render 41.63, render/graph/ms 39.02, stage/main/update 34.30, encode/pass:shadow_point_light/sum 34.04, stage/render/graph_submit 27.75 |
| warm-up | 5.12 | 25.8 | render/thread/ms 60.85, render/busy/ms 59.44, main/busy/ms 31.84, stage/render/render 23.62, render/graph/ms 21.50, encode//sum 18.44, stage/main/update 16.60, stage/render/graph_submit 13.81, encode/pass:shadow_point_light/sum 11.65, stage/render/prepare_assets 10.86, encode/submit 10.00, stage/main/post_update 8.97 |
| warm-up | 5.14 | 26.4 | render/thread/ms 46.17, render/busy/ms 44.71, main/busy/ms 30.94, stage/render/render 17.83, stage/main/update 17.25, render/graph/ms 15.85, encode//sum 11.64, stage/render/graph_submit 10.67, stage/main/post_update 8.64, encode/submit 8.18, stage/render/prepare_assets 7.04, encode/pass:shadow_point_light/sum 6.68 |
| warm-up | 5.19 | 27.9 | pipelines/compile_ms 117, pipelines/front_ms 112, pipelines/wait_ms 62.43, render/thread/ms 43.28, render/busy/ms 41.75, main/busy/ms 25.10, stage/render/render 15.24, render/graph/ms 13.06, stage/render/prepare_assets 12.69, stage/main/post_update 11.21, encode//sum 10.80, stage/main/update 9.86 |
| warm-up | 5.24 | 26.5 | render/thread/ms 36.23, render/busy/ms 34.81, main/busy/ms 21.43, stage/render/render 16.62, render/graph/ms 14.63, encode//sum 11.23, stage/render/prepare_assets 10.31, stage/render/graph_submit 9.94, stage/main/post_update 8.88, stage/main/update 8.06, encode/submit 7.69, encode/pass:shadow_point_light/sum 6.20 |
| warm-up | 5.35 | 48.2 | pipelines/wait_ms 262, pipelines/compile_ms 106, pipelines/front_ms 100, main/busy/ms 59.84, render/thread/ms 52.29, render/busy/ms 50.31, stage/main/update 37.36, stage/render/render 20.89, render/graph/ms 18.18, stage/main/post_update 16.44, encode//sum 13.80, stage/render/graph_submit 11.70 |
| warm-up | 5.40 | 48.8 | pipelines/wait_ms 262, pipelines/compile_ms 106, pipelines/front_ms 100, render/thread/ms 69.54, render/busy/ms 67.64, main/busy/ms 53.04, stage/main/update 29.97, stage/render/prepare_assets 28.47, stage/render/render 19.36, render/graph/ms 16.71, stage/main/post_update 14.27, encode//sum 13.60 |
| warm-up | 5.45 | 48.7 | render/thread/ms 65.27, render/busy/ms 63.83, stage/render/prepare_assets 38.19, main/busy/ms 33.29, stage/main/update 14.92, stage/render/render 14.09, render/graph/ms 11.90, encode//sum 10.94, stage/main/post_update 10.15, stage/render/graph_submit 7.11, encode/pass:shadow_point_light/sum 6.22, encode/submit 4.93 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\census@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 7% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (264 pmon samples)
- focus: clean
- nvidia-smi over the window (44 samples): GPU load **88%** (min 46%) · 2620 MHz (min 2355) · memory 8101 MHz · 88.1 W · 70 °C (max 76) · VRAM 5194 MiB · throttled 98% of samples

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
