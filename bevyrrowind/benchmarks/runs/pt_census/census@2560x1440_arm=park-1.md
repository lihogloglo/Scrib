# `census_office`

- bench_ground: false · build: release · camera: [9.6, 19.8, 12.5] -> [9.6, 3.8, -7.1] · cell: Bevyrrowind - Seyda Neen, Census and Excise Office · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); stream.interior.disable_exterior=on (command line: --set); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1600 frames over 11.4 s · **median 6.70 ms (149 fps)** · mean 7.15 · p95 9.63 · p99 11.53 (1% low 87 fps) · worst 19.91 ms · 48 spikes over 1.5× median · 42.0 stutters/min (work 21.0/min)
- warm-up: 120 frames over 6.6 s, worst 2537.7 ms
- hitches (> 25.0 ms): **0** measured, 45 in warm-up
- timed GPU passes: 5.07 ms of a 7.15 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median fps 149.2, target 144
- pass worst frame 19.91 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.937 | 2.028 | 2.068 | 0.95 | 0.162 |
| main_opaque_pass_3d | 1.779 | 1.863 | 2.603 | 0.95 | 0.033 |
| bloom | 0.307 | 0.322 | 0.327 | 0.95 | 0.061 |
| auxiliary_views | 0.278 | 0.291 | 0.928 | 0.95 | 0.283 |
| ssao | 0.181 | 0.190 | 0.194 | 0.95 | 0.007 |
| upscaling | 0.145 | 0.151 | 0.172 | 0.95 | 0.004 |
| tlas_build | 0.110 | 0.116 | 0.120 | 0.95 | 0.071 |
| tonemapping | 0.095 | 0.100 | 0.106 | 0.95 | 0.004 |
| gi/probe_update | 0.092 | 0.096 | 0.103 | 0.95 | 0.002 |
| early prepass | 0.077 | 0.080 | 0.099 | 0.95 | 0.034 |
| clustering | 0.069 | 0.073 | 0.076 | 0.95 | 0.019 |
| uniform_allocation | 0.034 | 0.035 | 0.044 | 0.95 | 0.010 |
| bin_unpacking | 0.031 | 0.032 | 0.039 | 0.95 | 0.008 |
| early_mesh_preprocessing | 0.013 | 0.013 | 0.014 | 0.95 | 0.004 |
| main_transparent_pass_3d | 0.007 | 0.008 | 0.010 | 0.93 | 0.023 |
| setup_tlas_instances | 0.007 | 0.007 | 0.007 | 0.95 | 0.006 |
| sparse buffer updates | 0.002 | 0.006 | 0.007 | 0.31 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.95 | 0.001 |
| blas_build | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/compile_ms | 10.386 | 0.000 | 9938.00 | 16616.8 |
| pipelines/driver_ms | 9.657 | 0.000 | 9519.28 | 15451.6 |
| render/thread/ms | 5.978 | 8.562 | 12.75 | 9565.2 |
| render/busy/ms | 5.532 | 8.097 | 11.45 | 8851.0 |
| main/busy/ms | 5.340 | 6.304 | 9.67 | 8543.9 |
| pipelines/wait_ms | 3.945 | 0.000 | 1725.22 | 6312.5 |
| stage/render/render | 3.788 | 5.593 | 8.73 | 6060.9 |
| render/graph/ms | 3.189 | 4.936 | 7.21 | 5102.5 |
| stage/main/post_update | 2.953 | 3.609 | 5.23 | 4724.6 |
| encode//sum | 2.370 | 3.769 | 5.96 | 3791.6 |
| stage/render/graph_submit | 2.129 | 3.246 | 4.89 | 3405.8 |
| encode/submit | 1.437 | 2.256 | 3.71 | 2298.9 |
| stage/main/update | 1.251 | 1.557 | 2.85 | 2001.8 |
| encode/pass:shadow_point_light/sum | 0.981 | 3.083 | 4.47 | 1569.7 |
| stage/render/camera_driver | 0.933 | 1.624 | 2.58 | 1493.0 |
| pipelines/front_ms | 0.728 | 0.000 | 418.71 | 1165.2 |
| encode/finish_wall | 0.714 | 1.096 | 1.67 | 1143.1 |
| stage/render/prepare_write_phase_buffers | 0.417 | 0.802 | 1.21 | 667.5 |
| encode/pass:early prepass/sum | 0.402 | 0.560 | 1.27 | 642.7 |
| stage/render/prepare_resources | 0.400 | 0.492 | 0.82 | 640.0 |
| encode/pass:main_opaque_pass_3d/sum | 0.388 | 0.546 | 1.06 | 621.3 |
| render/present/ms | 0.367 | 0.553 | 2.67 | 587.5 |
| stage/main/first | 0.335 | 0.428 | 1.38 | 536.7 |
| stage/main/pre_update | 0.321 | 0.402 | 0.72 | 514.1 |
| stage/render/prepare_bind_groups | 0.289 | 0.610 | 1.36 | 462.9 |
| bigspace/total/ms | 0.243 | 0.578 | 1.22 | 388.9 |
| stage/render/prepare_assets | 0.206 | 0.257 | 1.62 | 328.9 |
| stage/main/fixed_main_loop | 0.201 | 0.531 | 1.39 | 321.3 |
| stage/render/prepare_meshes | 0.192 | 0.259 | 0.47 | 307.7 |
| stage/render/create_views | 0.148 | 0.188 | 0.63 | 236.1 |
| stage/render/extract_commands | 0.147 | 0.175 | 0.37 | 235.8 |
| stage/render/prepare_flush | 0.134 | 0.223 | 0.60 | 213.9 |
| stream/drive/ms | 0.133 | 0.182 | 0.42 | 213.4 |
| stage/render/prepare_views | 0.122 | 0.165 | 0.31 | 195.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.108 | 0.158 | 0.72 | 173.6 |
| stage/main/last | 0.108 | 0.140 | 0.87 | 172.9 |
| stream/tick/ms | 0.107 | 0.152 | 0.39 | 170.7 |
| bigspace/low_precision/ms | 0.091 | 0.248 | 0.79 | 145.9 |
| render/acquire/ms | 0.079 | 0.106 | 0.26 | 126.7 |
| stage/render/specialize | 0.079 | 0.104 | 0.54 | 125.7 |
| anim/evaluate/ms | 0.076 | 0.263 | 0.81 | 121.2 |
| stage/render/prepare_batch_phases | 0.065 | 0.110 | 0.27 | 104.8 |
| controller/feed/ms | 0.062 | 0.079 | 0.24 | 99.2 |
| physics/step/ms | 0.062 | 0.223 | 0.58 | 99.2 |
| stage/render/phase_sort | 0.054 | 0.088 | 0.31 | 86.1 |
| bigspace/high_precision/ms | 0.052 | 0.178 | 0.74 | 83.8 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.071 | 0.28 | 75.6 |
| stage/render/queue | 0.041 | 0.071 | 0.36 | 65.1 |
| anim/evaluate_cpu/ms | 0.035 | 0.052 | 0.44 | 55.8 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.11 | 47.6 |
| weather/map/ms | 0.026 | 0.038 | 0.29 | 41.6 |
| physics/insert/ms | 0.021 | 0.168 | 0.88 | 34.2 |
| stage/render/cleanup | 0.021 | 0.032 | 0.28 | 34.1 |
| far/select/ms | 0.015 | 0.019 | 0.09 | 24.7 |
| weather/eval/ms | 0.012 | 0.017 | 0.29 | 19.3 |
| encode/camera_driver_span_begin/sum | 0.012 | 0.019 | 0.42 | 19.3 |
| encode/camera_driver_span_end/sum | 0.010 | 0.018 | 0.07 | 16.1 |
| script/update/ms | 0.008 | 0.014 | 0.16 | 13.1 |
| controller/move/ms | 0.006 | 0.007 | 0.02 | 9.6 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 8.2 |
| anim/apply/ms | 0.005 | 0.010 | 0.20 | 7.9 |
| controller/avoid/ms | 0.004 | 0.004 | 0.03 | 5.7 |
| script/ui/ms | 0.003 | 0.004 | 0.02 | 4.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| controller/write/ms | 0.001 | 0.001 | 0.01 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.3 |
| weather/apply/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.02 | 0.9 |
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
| crowd/joint_blocks/count | count | 2 | 3 | 2 |
| crowd/joints_extracted/count | count | 94 | 1451 | 94 |
| crowd/joints_uploaded/count | count | 94 | 153 | 94 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 15.7 k | 15.7 k | 15.7 k |
| encode//max | ms | 0.38 | 1.09 | 0.42 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.42 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.07 | 0.01 |
| encode/finish_critical | ms | 0.49 | 1.27 | 0.42 |
| encode/pass:early prepass/max | ms | 0.41 | 1.27 | 0.26 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.40 | 1.06 | 0.37 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.11 | 0.72 | 0.12 |
| encode/pass:shadow_point_light/max | ms | 0.48 | 1.04 | 0.41 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.11 | 0.02 |
| far/entities | count | 162 | 162 | 162 |
| far/forced | count | 4 | 4 | 4 |
| far/impostors | count | 41.6 k | 41.6 k | 41.6 k |
| far/lights | count | 4955 | 4955 | 4955 |
| far/nodes/l1 | count | 29 | 29 | 29 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 86.26 M | 86.26 M | 86.26 M |
| far/triangles | count | 1.97 M | 1.97 M | 1.97 M |
| frame/work/ms | ms | 5.49 | 11.45 | 5.86 |
| gpu/timed/ms | ms | 5.18 | 7.41 | 4.65 |
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
| phases/shadow/batch_sets | count | 0 | 83 | 25 |
| phases/shadow/bins | count | 2638 | 2867 | 2173 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3603 | 3869 | 3105 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 227 | 240 | 189 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2117 | 2117 | 2117 |
| physics/items | count | 153 | 153 | 153 |
| physics/pending | count | 0 | 481 | 0 |
| physics/terrain/chunks | count | 0 | 0 | 0 |
| pipelines/total | count | 286 | 286 | 286 |
| pipelines/waiting | count | 0 | 11 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 14 | 1 |
| precache/outstanding | count | 0 | 4 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 3 | 188 | 56 |
| preprocess/bind_groups_created | count | 3 | 188 | 56 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 47 | 103 | 35 |
| process/cpu_usage | % | 25.69 | 70.09 | 25.96 |
| process/mem_usage | GiB | 3.93 | 3.96 | 3.96 |
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
| stream/latency/s | s | 0.06 | 0.06 | 0.06 |
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
| system/cpu_usage | % | 30.66 | 80.92 | 30.08 |
| system/mem_usage | % | 57.73 | 57.82 | 57.78 |
| terrain/patches | count | 0 | 0 | 0 |
| textures/gpu_mib | MiB | 2038 | 2038 | 2038 |
| textures/images | count | 1476 | 1476 | 1476 |
| views/all/batch_sets | count | 18 | 102 | 34 |
| views/all/bins | count | 2970 | 3303 | 2223 |
| views/all/count | count | 49 | 49 | 49 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 4157 | 4589 | 3183 |
| views/all/pipelines | count | 247 | 261 | 198 |
| views/all/pipelines_distinct | count | 22 | 28 | 16 |
| views/all/work_items | count | 582 | 2411 | 502 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
| views/lamp_face/batch_sets | count | 0 | 83 | 25 |
| views/lamp_face/bins | count | 2638 | 2867 | 2173 |
| views/lamp_face/count | count | 48 | 48 | 48 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 13 | 6 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3603 | 3869 | 3105 |
| views/lamp_face/pipelines | count | 227 | 240 | 189 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 1347 | 424 |
| views/main/batch_sets | count | 15 | 21 | 9 |
| views/main/bins | count | 290 | 574 | 50 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 518 | 1054 | 78 |
| views/main/pipelines | count | 15 | 21 | 9 |
| views/main/work_items | count | 538 | 1095 | 78 |
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
| anim/bones_written/count | count | 8.27 | 30 | 13.2 k |
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
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| encode//count | count | 30.77 | 70 | 49.2 k |
| encode/bind_walk/init_textures | count | 8511 | 22.4 k | 13.62 M |
| encode/bind_walk/init_textures_skipped | count | 17.7 k | 62.0 k | 28.36 M |
| encode/bind_walk/inits | count | 194 | 409 | 310.0 k |
| encode/bind_walk/inits_skipped | count | 20.53 | 67 | 32.9 k |
| encode/bind_walk/merge_textures | count | 7268 | 21.2 k | 11.63 M |
| encode/bind_walk/merge_textures_skipped | count | 32.25 | 77 | 51.6 k |
| encode/bind_walk/merges | count | 59.51 | 117 | 95.2 k |
| encode/bind_walk/merges_skipped | count | 7.95 | 23 | 12.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 1600 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 1600 |
| encode/pass:early prepass/count | count | 1 | 2 | 1600 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 1600 |
| encode/pass:main_transparent_pass_3d/count | count | 0.97 | 2 | 1554 |
| encode/pass:shadow_point_light/count | count | 3.11 | 13 | 4979 |
| encode/queue/command_buffers | count | 42.85 | 92 | 68.6 k |
| encode/queue/submits | count | 2 | 4 | 3200 |
| encode/queue/write_buffer/bytes | bytes | 468.1 k | 894.0 k | 748.97 M |
| encode/queue/write_buffer/calls | count | 149 | 354 | 238.2 k |
| encode/queue/write_buffer_with/bytes | bytes | 158.0 k | 315.9 k | 252.72 M |
| encode/queue/write_buffer_with/calls | count | 6 | 12 | 9600 |
| encode/queue/write_texture/bytes | bytes | 11.1 k | 8.39 M | 17.83 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 3 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 1600 |
| physics/inserts | count | 0.27 | 45 | 436 |
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

## Hitches (> 25.0 ms; slowest 45, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2537.7 | main/busy/ms 2509, stage/main/post_update 2406, stage/main/update 93.80, surface/advance_cpu_ms 65.42, stage/main/pre_update 4.55, stage/main/last 1.09, stage/main/first 0.72, stream/drive/ms 0.33, stage/main/fixed_main_loop 0.33, weather/map/ms 0.11, stream/tick/ms 0.09, bigspace/total/ms 0.07 |
| warm-up | 3.37 | 827.8 | main/busy/ms 2626, stage/main/post_update 2411, render/thread/ms 792, render/busy/ms 611, stage/render/prepare_assets 576, stage/main/update 201, stage/render/prepare_views 177, render/acquire/ms 176, surface/advance_cpu_ms 65.42, stage/render/prepare_resources 21.91, stage/render/render 12.75, render/graph/ms 7.01 |
| warm-up | 3.46 | 95.8 | render/thread/ms 873, render/busy/ms 651, stage/render/prepare_assets 611, stage/render/prepare_views 178, render/acquire/ms 177, main/busy/ms 137, stage/main/update 115, stage/render/render 55.92, render/present/ms 44.82, stage/render/prepare_resources 22.44, render/graph/ms 10.38, stage/main/fixed_main_loop 9.58 |
| warm-up | 3.49 | 28.4 | pipelines/compile_ms 134, pipelines/front_ms 123, render/thread/ms 101, main/busy/ms 61.59, stage/render/render 60.57, render/busy/ms 59.85, pipelines/wait_ms 41.69, render/present/ms 40.94, stage/render/prepare_assets 36.37, stage/main/post_update 25.91, stage/main/update 22.88, render/graph/ms 19.09 |
| warm-up | 3.53 | 36.1 | pipelines/wait_ms 1065, pipelines/compile_ms 464, pipelines/front_ms 430, main/busy/ms 61.58, render/thread/ms 38.96, render/busy/ms 36.31, pipelines/driver_ms 33.61, stage/main/update 27.06, stage/main/post_update 25.75, stage/render/render 21.16, render/graph/ms 18.47, stage/render/prepare_assets 11.41 |
| warm-up | 3.56 | 36.0 | pipelines/wait_ms 1146, pipelines/compile_ms 503, pipelines/front_ms 473, render/thread/ms 42.78, render/busy/ms 40.66, pipelines/driver_ms 30.15, main/busy/ms 29.44, stage/render/prepare_assets 20.06, stage/main/update 13.85, stage/main/post_update 9.80, stage/render/render 7.62, render/graph/ms 5.54 |
| warm-up | 3.69 | 102.8 | pipelines/compile_ms 1355, pipelines/front_ms 1304, pipelines/wait_ms 141, render/thread/ms 78.49, main/busy/ms 76.61, render/busy/ms 76.05, stage/main/update 55.69, pipelines/driver_ms 51.47, stage/render/prepare_assets 41.17, stage/render/render 21.46, render/graph/ms 19.04, stage/render/graph_submit 15.79 |
| warm-up | 3.71 | 26.1 | pipelines/compile_ms 1445, pipelines/front_ms 1389, pipelines/wait_ms 140, render/thread/ms 81.27, render/busy/ms 74.87, pipelines/driver_ms 55.84, stage/render/prepare_assets 54.17, main/busy/ms 20.72, stage/render/render 18.55, render/graph/ms 15.08, pipelines/lock_ms 13.43, stage/render/graph_submit 12.88 |
| warm-up | 3.75 | 33.1 | pipelines/compile_ms 318, pipelines/front_ms 305, render/thread/ms 50.90, render/busy/ms 44.94, stage/render/prepare_assets 35.10, main/busy/ms 16.07, pipelines/driver_ms 12.37, stage/render/render 8.19, pipelines/wait_ms 7.27, pipelines/lock_ms 6.61, stage/main/update 6.26, render/graph/ms 5.19 |
| warm-up | 3.83 | 56.5 | pipelines/compile_ms 877, pipelines/front_ms 847, render/thread/ms 43.40, render/busy/ms 41.16, main/busy/ms 33.10, stage/render/prepare_assets 29.37, pipelines/driver_ms 29.11, stage/main/update 21.56, stream/drive/ms 16.79, stage/render/render 9.09, render/graph/ms 6.72, stage/main/post_update 6.51 |
| warm-up | 3.87 | 39.7 | pipelines/compile_ms 1029, pipelines/front_ms 996, render/thread/ms 61.40, render/busy/ms 59.50, stage/render/prepare_assets 46.79, pipelines/driver_ms 33.69, main/busy/ms 27.69, stage/main/update 13.28, stage/render/render 10.09, stage/main/post_update 8.82, render/graph/ms 7.94, pipelines/wait_ms 6.04 |
| warm-up | 3.94 | 44.4 | pipelines/compile_ms 401, pipelines/front_ms 387, main/busy/ms 65.06, render/thread/ms 44.62, render/busy/ms 43.29, stage/main/update 40.66, stream/drive/ms 37.33, stage/render/prepare_assets 28.69, pipelines/driver_ms 14.13, stage/main/first 13.47, stage/main/post_update 7.11, stage/render/render 7.04 |
| warm-up | 4.00 | 60.8 | pipelines/compile_ms 1576, pipelines/front_ms 1527, main/busy/ms 80.49, pipelines/wait_ms 75.11, render/thread/ms 53.43, pipelines/driver_ms 49.18, render/busy/ms 48.52, stage/main/update 40.67, stream/drive/ms 36.99, stage/render/prepare_assets 21.21, stage/render/render 19.82, render/graph/ms 18.31 |
| warm-up | 4.03 | 27.1 | pipelines/compile_ms 1623, pipelines/front_ms 1569, pipelines/wait_ms 96.12, main/busy/ms 54.93, pipelines/driver_ms 54.20, render/thread/ms 51.50, render/busy/ms 45.70, stage/render/render 31.85, render/graph/ms 29.45, stage/main/update 25.81, stage/render/camera_driver 20.26, stage/main/last 13.29 |
| warm-up | 4.07 | 36.9 | pipelines/compile_ms 570, pipelines/front_ms 548, pipelines/wait_ms 113, main/busy/ms 46.16, render/thread/ms 43.83, render/busy/ms 41.44, stage/main/update 25.67, pipelines/driver_ms 22.64, stage/render/prepare_assets 19.03, stage/render/render 18.87, stage/main/post_update 16.53, render/graph/ms 16.46 |
| warm-up | 4.10 | 32.3 | pipelines/compile_ms 761, pipelines/front_ms 734, pipelines/wait_ms 139, render/thread/ms 47.22, render/busy/ms 45.78, main/busy/ms 29.27, pipelines/driver_ms 26.36, stage/render/prepare_assets 20.21, stage/render/prepare_resources 15.13, stage/main/post_update 14.61, stage/main/update 10.65, stage/render/render 6.70 |
| warm-up | 4.13 | 27.2 | pipelines/compile_ms 470, pipelines/front_ms 455, pipelines/wait_ms 56.55, render/thread/ms 53.21, render/busy/ms 51.86, main/busy/ms 47.06, stage/main/post_update 29.07, stage/render/prepare_resources 16.29, pipelines/driver_ms 14.70, bigspace/total/ms 13.80, stage/render/prepare_bind_groups 11.96, stage/render/prepare_assets 11.04 |
| warm-up | 4.16 | 35.4 | pipelines/compile_ms 859, pipelines/front_ms 835, pipelines/wait_ms 102, main/busy/ms 49.00, render/thread/ms 48.18, render/busy/ms 46.84, stage/main/post_update 35.15, pipelines/driver_ms 23.42, stage/render/prepare_assets 20.45, bigspace/total/ms 13.80, stage/render/prepare_bind_groups 12.02, stage/render/render 8.63 |
| warm-up | 4.30 | 26.1 | pipelines/compile_ms 342, pipelines/front_ms 329, render/thread/ms 31.61, render/busy/ms 30.09, main/busy/ms 19.33, stage/render/prepare_assets 13.46, pipelines/driver_ms 12.41, stage/main/post_update 8.24, stage/render/render 6.00, stage/main/pre_update 4.79, stage/main/update 4.43, stage/render/prepare_resources 4.39 |
| warm-up | 4.33 | 25.1 | pipelines/compile_ms 679, pipelines/front_ms 661, render/thread/ms 39.99, render/busy/ms 38.62, pipelines/driver_ms 18.86, main/busy/ms 16.52, stage/render/prepare_assets 13.35, stage/render/prepare_flush 8.60, stage/render/prepare_resources 8.29, stage/render/render 6.23, stage/main/post_update 5.61, render/graph/ms 4.62 |
| warm-up | 4.45 | 60.6 | main/busy/ms 68.71, stage/main/update 51.33, stream/drive/ms 47.17, pipelines/compile_ms 37.50, pipelines/front_ms 36.27, pipelines/wait_ms 23.78, render/thread/ms 23.67, render/busy/ms 21.98, stage/main/post_update 9.35, stage/render/render 8.18, stage/render/prepare_assets 7.94, render/graph/ms 6.15 |
| warm-up | 4.60 | 126.8 | pipelines/compile_ms 217, pipelines/front_ms 210, pipelines/wait_ms 103, main/busy/ms 97.46, stage/main/update 77.89, render/thread/ms 47.98, stream/drive/ms 47.59, render/busy/ms 46.10, stage/render/prepare_assets 28.04, stage/main/post_update 12.49, stage/render/render 10.26, render/graph/ms 7.86 |
| warm-up | 4.65 | 49.8 | pipelines/compile_ms 117, pipelines/front_ms 113, pipelines/wait_ms 101, render/thread/ms 72.80, render/busy/ms 70.84, stage/render/prepare_assets 40.15, main/busy/ms 34.98, stage/main/update 15.89, stage/render/render 14.57, stage/main/post_update 11.98, render/graph/ms 11.94, encode//sum 7.30 |
| warm-up | 4.69 | 38.1 | render/thread/ms 72.61, render/busy/ms 68.53, main/busy/ms 39.21, encode//sum 38.64, stage/render/render 25.49, stage/render/prepare_assets 24.45, render/graph/ms 21.05, stage/main/post_update 15.60, stage/main/update 14.90, stage/render/graph_submit 13.20, encode/pass:shadow_point_light/sum 8.16, encode/submit 7.21 |
| warm-up | 4.74 | 50.7 | render/thread/ms 74.15, render/busy/ms 69.96, main/busy/ms 50.37, encode//sum 43.98, stage/render/render 29.72, stage/main/update 25.66, render/graph/ms 25.07, stage/render/prepare_assets 18.85, stage/main/post_update 16.22, stage/render/graph_submit 15.95, encode/pass:shadow_point_light/sum 11.98, encode/submit 8.86 |
| warm-up | 4.78 | 45.1 | render/thread/ms 72.77, render/busy/ms 70.62, main/busy/ms 48.72, stage/main/update 28.77, stage/render/prepare_assets 28.54, stage/render/render 22.33, render/graph/ms 19.41, encode//sum 17.98, stage/main/post_update 12.70, stage/render/graph_submit 10.97, encode/pass:shadow_point_light/sum 9.97, stage/render/camera_driver 7.94 |
| warm-up | 4.85 | 67.6 | render/thread/ms 94.43, main/busy/ms 91.62, render/busy/ms 91.39, encode//sum 76.38, stage/main/update 73.90, stream/drive/ms 61.60, stage/render/render 41.54, render/graph/ms 37.70, stage/render/graph_submit 27.96, encode/pass:shadow_point_light/sum 23.28, stage/render/prepare_assets 22.63, encode/submit 14.93 |
| warm-up | 4.92 | 70.9 | render/thread/ms 103, render/busy/ms 99.94, encode//sum 96.11, main/busy/ms 92.12, stage/main/update 70.80, stream/drive/ms 61.90, stage/render/render 50.06, render/graph/ms 46.22, stage/render/graph_submit 34.63, encode/pass:shadow_point_light/sum 27.34, encode/submit 18.98, stage/render/prepare_assets 17.92 |
| warm-up | 4.97 | 53.2 | render/thread/ms 85.62, render/busy/ms 83.38, main/busy/ms 58.83, encode//sum 42.52, stage/main/update 38.62, stage/render/render 36.61, render/graph/ms 33.79, stream/drive/ms 27.33, stage/render/prepare_assets 22.68, stage/render/graph_submit 22.04, encode/pass:shadow_point_light/sum 19.10, encode/submit 14.07 |
| warm-up | 5.05 | 74.4 | render/thread/ms 110, render/busy/ms 108, encode//sum 73.19, main/busy/ms 57.16, stage/render/render 56.06, render/graph/ms 52.99, stage/main/update 39.38, stage/render/graph_submit 34.94, encode/pass:shadow_point_light/sum 30.79, stream/drive/ms 27.59, encode/submit 22.59, stage/render/prepare_assets 20.76 |
| warm-up | 5.09 | 44.6 | render/thread/ms 105, render/busy/ms 103, encode//sum 65.60, stage/render/render 50.15, main/busy/ms 45.98, render/graph/ms 45.57, stage/render/graph_submit 30.28, stage/render/prepare_assets 25.74, encode/pass:shadow_point_light/sum 24.09, stage/main/update 22.33, encode/submit 19.39, stage/main/post_update 17.36 |
| warm-up | 5.19 | 98.8 | render/thread/ms 96.88, render/busy/ms 95.00, stage/render/prepare_assets 58.95, main/busy/ms 47.46, stage/main/update 22.32, stage/render/render 20.89, render/graph/ms 16.86, stage/main/post_update 16.20, encode//sum 13.72, stream/drive/ms 12.57, stage/render/graph_submit 10.10, encode/pass:shadow_point_light/sum 6.93 |
| warm-up | 5.27 | 78.7 | render/thread/ms 112, render/busy/ms 110, stage/render/prepare_assets 81.98, main/busy/ms 32.28, stage/render/render 17.66, render/graph/ms 15.26, stage/main/update 12.53, encode//sum 11.14, stage/render/graph_submit 10.01, stage/main/post_update 9.64, encode/submit 7.32, encode/pass:shadow_point_light/sum 6.96 |
| warm-up | 5.30 | 30.1 | render/thread/ms 82.64, render/busy/ms 79.66, stage/render/prepare_assets 40.76, main/busy/ms 40.05, stage/render/render 30.93, render/graph/ms 27.36, stage/main/update 23.13, stage/render/graph_submit 22.60, encode/submit 20.50, stream/drive/ms 12.16, encode//sum 10.80, stage/main/post_update 9.02 |
| warm-up | 5.37 | 28.7 | main/busy/ms 41.37, render/thread/ms 36.96, render/busy/ms 35.19, stage/main/update 27.00, stream/drive/ms 18.95, stage/render/render 15.44, render/graph/ms 13.01, encode//sum 10.44, stage/main/post_update 8.91, stage/render/graph_submit 7.90, stage/render/prepare_assets 7.48, encode/submit 5.58 |
| warm-up | 5.46 | 26.4 | render/thread/ms 43.22, render/busy/ms 41.42, main/busy/ms 25.16, stage/render/render 19.12, render/graph/ms 16.88, encode//sum 13.53, stage/render/graph_submit 11.68, stage/main/update 11.36, stage/render/prepare_assets 9.23, stage/main/post_update 9.18, encode/submit 8.31, encode/pass:shadow_point_light/sum 6.98 |
| warm-up | 5.50 | 29.5 | render/thread/ms 40.85, render/busy/ms 39.44, main/busy/ms 21.50, stage/render/prepare_assets 16.91, stage/render/render 15.26, render/graph/ms 12.94, encode//sum 10.99, stage/main/post_update 10.10, stage/render/graph_submit 8.48, stage/main/update 7.29, encode/pass:shadow_point_light/sum 6.17, encode/submit 5.92 |
| warm-up | 5.53 | 26.0 | render/thread/ms 52.19, render/busy/ms 50.24, main/busy/ms 24.16, stage/render/render 22.27, render/graph/ms 19.52, stage/render/prepare_assets 19.48, encode//sum 16.33, stage/render/graph_submit 12.84, stage/main/post_update 10.74, encode/submit 9.50, stage/main/update 9.37, encode/pass:shadow_point_light/sum 8.34 |
| warm-up | 5.56 | 29.8 | render/thread/ms 51.18, render/busy/ms 48.95, main/busy/ms 34.34, stage/render/render 22.34, render/graph/ms 19.23, encode//sum 15.16, stage/main/update 14.38, stage/render/prepare_assets 12.32, stage/render/graph_submit 12.28, stage/main/post_update 11.26, encode/submit 9.43, encode/pass:shadow_point_light/sum 7.80 |
| warm-up | 5.60 | 34.9 | render/thread/ms 52.34, render/busy/ms 50.53, main/busy/ms 35.00, stage/render/prepare_assets 19.82, stage/render/render 16.85, stage/main/update 14.59, render/graph/ms 14.35, stage/main/post_update 11.17, encode//sum 10.71, stage/render/graph_submit 9.37, encode/submit 7.15, encode/pass:shadow_point_light/sum 6.57 |
| warm-up | 5.63 | 31.2 | render/thread/ms 54.53, render/busy/ms 53.03, main/busy/ms 29.90, stage/render/prepare_assets 26.31, stage/render/render 16.64, render/graph/ms 14.50, stage/main/update 14.00, encode//sum 11.85, stage/main/post_update 10.50, stage/render/graph_submit 9.35, encode/pass:shadow_point_light/sum 7.05, encode/submit 6.76 |
| warm-up | 5.68 | 26.2 | render/thread/ms 39.66, main/busy/ms 38.98, render/busy/ms 38.14, stage/main/update 23.50, stage/render/render 16.84, render/graph/ms 14.51, encode//sum 11.45, stream/drive/ms 10.79, stage/main/post_update 9.46, stage/render/graph_submit 9.29, stage/render/prepare_assets 7.78, encode/submit 6.50 |
| warm-up | 5.79 | 43.2 | pipelines/wait_ms 5865, pipelines/compile_ms 1608, pipelines/driver_ms 1374, pipelines/front_ms 235, main/busy/ms 56.32, render/thread/ms 51.62, render/busy/ms 49.32, stage/main/post_update 27.79, stage/render/render 24.46, render/graph/ms 21.58, stage/main/update 16.51, stage/render/graph_submit 13.93 |
| warm-up | 5.84 | 54.6 | pipelines/wait_ms 7001, pipelines/compile_ms 4705, pipelines/driver_ms 4013, pipelines/front_ms 692, render/thread/ms 61.12, render/busy/ms 59.53, main/busy/ms 45.54, stage/render/prepare_assets 26.67, stage/main/update 17.45, stage/render/render 16.96, stage/main/post_update 15.03, render/graph/ms 14.77 |
| warm-up | 5.87 | 26.8 | pipelines/compile_ms 4776, pipelines/wait_ms 4484, pipelines/driver_ms 4064, pipelines/front_ms 712, render/thread/ms 54.58, render/busy/ms 53.13, main/busy/ms 30.96, stage/render/prepare_assets 24.36, stage/render/render 16.51, render/graph/ms 14.34, stage/main/update 13.92, stage/main/post_update 10.89 |

## Run

- `target/profiling/perftests.exe` view "Seyda Neen, Census and Excise Office" --bench benchmarks/census_office.json --set stream.interior.disable_exterior=on --resolution 2560x1440 --bench-out target/perf\pt_census\census@2560x1440_arm=park-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 1%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (220 pmon samples)
- focus: clean
- nvidia-smi over the window (43 samples): GPU load **81%** (min 51%) · 2626 MHz (min 2625) · memory 8101 MHz · 88.1 W · 73 °C (max 78) · VRAM 5432 MiB · throttled 100% of samples

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
