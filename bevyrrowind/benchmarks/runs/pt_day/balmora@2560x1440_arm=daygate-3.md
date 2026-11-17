# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 1554 frames over 15.0 s · **median 9.71 ms (103 fps)** · mean 9.66 · p95 11.07 · p99 11.60 (1% low 86 fps) · worst 27.17 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 280 frames over 8.5 s, worst 2577.3 ms
- hitches (> 25.0 ms): **1** measured, 54 in warm-up
- timed GPU passes: 8.52 ms of a 9.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.71 ms, limit 11 ms
- pass worst frame 27.17 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.893 | 2.903 | 2.981 | 1.00 | 0.060 |
| dlss_super_resolution | 2.017 | 2.024 | 2.044 | 1.00 | 0.183 |
| early prepass | 0.610 | 0.612 | 0.622 | 1.00 | 0.091 |
| atmosphere_luts | 0.323 | 0.324 | 0.340 | 1.00 | 0.003 |
| bloom | 0.314 | 0.316 | 0.331 | 1.00 | 0.061 |
| main_transmissive_pass_3d | 0.308 | 0.309 | 0.331 | 1.00 | 0.008 |
| water_velocity | 0.238 | 0.239 | 0.263 | 1.00 | 0.002 |
| water_sspr | 0.206 | 0.207 | 0.210 | 1.00 | 0.002 |
| ssao | 0.188 | 0.189 | 0.197 | 1.00 | 0.007 |
| clouds_trace | 0.149 | 0.149 | 0.170 | 1.00 | 0.001 |
| atmos_froxel_inject | 0.145 | 0.146 | 0.152 | 1.00 | 0.001 |
| water_fft | 0.130 | 0.131 | 0.134 | 1.00 | 0.003 |
| uniform_allocation | 0.116 | 0.116 | 0.158 | 1.00 | 0.019 |
| upscaling | 0.112 | 0.113 | 0.118 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.112 | 0.113 | 1.00 | 0.002 |
| bin_unpacking | 0.086 | 0.086 | 0.118 | 1.00 | 0.014 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| clustering | 0.064 | 0.064 | 0.074 | 1.00 | 0.020 |
| render_sky | 0.052 | 0.052 | 0.052 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.413 | 0.413 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.029 | 0.029 | 0.038 | 1.00 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.222 | 0.232 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.028 | 1.00 | 0.081 |
| water_snapshot | 0.024 | 0.024 | 0.025 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| surface_exposure_far | 0.014 | 0.045 | 0.047 | 0.32 | 0.002 |
| per_view_shadow_pass | 0.012 | 0.012 | 0.023 | 1.00 | 0.005 |
| surface_exposure | 0.012 | 0.042 | 0.043 | 0.28 | 0.001 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.732 | 10.165 | 25.98 | 13569.9 |
| main/busy/ms | 6.996 | 7.878 | 9.44 | 10871.9 |
| render/busy/ms | 6.798 | 7.872 | 25.38 | 10563.7 |
| stage/render/render | 4.544 | 5.415 | 10.94 | 7061.7 |
| stage/main/post_update | 4.182 | 4.948 | 6.18 | 6499.1 |
| render/graph/ms | 3.978 | 4.797 | 10.18 | 6182.4 |
| encode//sum | 3.708 | 4.928 | 9.71 | 5762.4 |
| stage/render/graph_submit | 3.009 | 3.777 | 9.20 | 4676.8 |
| encode/submit | 1.888 | 2.345 | 5.69 | 2934.1 |
| stage/main/update | 1.717 | 2.137 | 2.99 | 2668.4 |
| stage/render/prepare_views | 1.627 | 3.226 | 3.98 | 2528.6 |
| render/acquire/ms | 1.582 | 3.184 | 3.94 | 2458.5 |
| encode/finish_wall | 1.141 | 1.601 | 6.10 | 1773.4 |
| stage/render/camera_driver | 0.914 | 1.058 | 1.50 | 1419.9 |
| encode/pass:shadow_directional_light/sum | 0.733 | 1.267 | 6.01 | 1138.6 |
| encode/pass:early prepass/sum | 0.699 | 0.966 | 1.29 | 1086.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.689 | 0.962 | 1.37 | 1071.4 |
| stage/render/prepare_write_phase_buffers | 0.523 | 0.727 | 0.92 | 812.0 |
| stage/render/prepare_assets | 0.443 | 0.566 | 1.58 | 687.7 |
| stage/render/prepare_resources | 0.383 | 0.446 | 2.31 | 595.0 |
| bigspace/total/ms | 0.355 | 0.721 | 1.50 | 552.3 |
| render/present/ms | 0.352 | 0.413 | 1.65 | 547.7 |
| stage/main/pre_update | 0.332 | 0.419 | 0.86 | 515.5 |
| stage/main/first | 0.293 | 0.356 | 1.66 | 454.8 |
| terrain/update/ms | 0.232 | 0.296 | 0.45 | 361.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.228 | 0.325 | 0.75 | 355.0 |
| terrain/select/ms | 0.203 | 0.259 | 0.42 | 314.8 |
| stage/render/prepare_bind_groups | 0.200 | 0.272 | 0.40 | 311.3 |
| stage/render/prepare_meshes | 0.196 | 0.241 | 2.20 | 304.4 |
| stream/drive/ms | 0.180 | 0.231 | 0.69 | 279.9 |
| stage/main/fixed_main_loop | 0.176 | 0.325 | 0.93 | 272.8 |
| stage/render/prepare_batch_phases | 0.172 | 0.231 | 0.37 | 266.9 |
| stage/render/extract_commands | 0.168 | 0.199 | 0.34 | 260.5 |
| stage/render/create_views | 0.165 | 0.211 | 0.56 | 257.2 |
| far/select/ms | 0.159 | 0.196 | 0.65 | 246.7 |
| stream/tick/ms | 0.153 | 0.195 | 0.66 | 237.7 |
| controller/feed/ms | 0.153 | 0.197 | 0.49 | 237.1 |
| stage/render/prepare_flush | 0.148 | 0.275 | 0.60 | 230.5 |
| bigspace/high_precision/ms | 0.131 | 0.364 | 1.26 | 204.0 |
| stage/main/last | 0.117 | 0.158 | 0.77 | 182.5 |
| encode/pass:main_transmissive_pass_3d/sum | 0.113 | 0.172 | 0.61 | 175.9 |
| controller/move/ms | 0.109 | 0.162 | 0.58 | 169.5 |
| stage/render/queue | 0.106 | 0.135 | 6.01 | 164.4 |
| anim/evaluate/ms | 0.087 | 0.209 | 1.00 | 134.5 |
| bigspace/low_precision/ms | 0.084 | 0.181 | 1.17 | 130.0 |
| stage/render/specialize | 0.083 | 0.104 | 8.11 | 129.3 |
| anim/evaluate_cpu/ms | 0.062 | 0.076 | 0.46 | 96.3 |
| stage/render/prepare_collect_phase_buffers | 0.056 | 0.115 | 0.21 | 87.3 |
| encode/resolve_diagnostics/sum | 0.053 | 0.079 | 0.17 | 82.9 |
| physics/step/ms | 0.027 | 0.054 | 0.39 | 41.8 |
| encode/camera_driver_span_begin/sum | 0.025 | 0.037 | 0.13 | 39.6 |
| weather/map/ms | 0.024 | 0.032 | 0.45 | 37.7 |
| stage/render/cleanup | 0.023 | 0.032 | 0.30 | 35.5 |
| encode/camera_driver_span_end/sum | 0.022 | 0.036 | 0.10 | 34.7 |
| stage/render/phase_sort | 0.019 | 0.034 | 0.11 | 30.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.05 | 16.4 |
| script/update/ms | 0.006 | 0.008 | 0.06 | 10.0 |
| controller/avoid/ms | 0.005 | 0.007 | 0.02 | 8.5 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.9 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 7.0 |
| script/ui/ms | 0.004 | 0.006 | 0.05 | 6.2 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.1 |
| stage/render/prepare | 0.000 | 0.002 | 0.05 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 0.51 | 5.12 | 0.56 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.13 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.10 | 0.01 |
| encode/finish_critical | ms | 0.83 | 6.01 | 1.00 |
| encode/pass:early prepass/max | ms | 0.70 | 1.29 | 0.99 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.70 | 1.37 | 0.39 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.61 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.75 | 0.20 |
| encode/pass:shadow_directional_light/max | ms | 0.67 | 6.01 | 1.00 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.17 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.14 M | 187.14 M | 187.14 M |
| far/triangles | count | 4.61 M | 4.61 M | 4.61 M |
| frame/work/ms | ms | 9.17 | 25.38 | 9.21 |
| gpu/timed/ms | ms | 9.17 | 10.42 | 9.21 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 13.3 k | 13.3 k | 13.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 485.99 M | 485.99 M | 485.99 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 20 | 20 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 19 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 15 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 1 | 30 | 23 |
| phases/shadow/bins | count | 2152 | 4889 | 2152 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6053 | 17.7 k | 6053 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 30 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 320 | 320 | 320 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 0 | 62 | 46 |
| preprocess/bind_groups_created | count | 0 | 62 | 46 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 123 | 123 | 123 |
| process/cpu_usage | % | 25 | 39.62 | 26.54 |
| process/mem_usage | GiB | 4.68 | 4.74 | 4.74 |
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
| skin/blocks_changed/count | count | 4 | 7 | 4 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 181 | 181 | 181 |
| skin/joint_bytes_written/bytes | bytes | 309.5 k | 309.5 k | 309.5 k |
| skin/joints_written/count | count | 4836 | 4836 | 4836 |
| skin/upload_writes/count | count | 1 | 1 | 1 |
| started_load_count |  | 3704 | 3706 | 3706 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.91 | 37.80 | 32.75 |
| system/mem_usage | % | 57.99 | 58.24 | 58.24 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2558 | 2558 | 2558 |
| textures/images | count | 2012 | 2012 | 2012 |
| views/all/batch_sets | count | 51 | 80 | 73 |
| views/all/bins | count | 5322 | 8059 | 5322 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.3 k | 27.0 k | 15.3 k |
| views/all/pipelines | count | 69 | 76 | 69 |
| views/all/pipelines_distinct | count | 61 | 61 | 61 |
| views/all/work_items | count | 9496 | 27.2 k | 15.5 k |
| views/cascade/batch_sets | count | 1 | 30 | 23 |
| views/cascade/bins | count | 2152 | 4889 | 2152 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6053 | 17.7 k | 6053 |
| views/cascade/pipelines | count | 23 | 30 | 23 |
| views/cascade/work_items | count | 34 | 17.7 k | 6053 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 50 | 50 | 50 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 46 | 46 | 46 |
| views/main/work_items | count | 9462 | 9462 | 9462 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 3 | 155 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.19 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 26.57 | 29 | 41.3 k |
| encode/bind_walk/init_textures | count | 7167 | 14.0 k | 11.14 M |
| encode/bind_walk/init_textures_skipped | count | 35.6 k | 48.7 k | 55.34 M |
| encode/bind_walk/inits | count | 341 | 433 | 530.5 k |
| encode/bind_walk/inits_skipped | count | 67.01 | 79 | 104.1 k |
| encode/bind_walk/merge_textures | count | 7103 | 13.9 k | 11.04 M |
| encode/bind_walk/merge_textures_skipped | count | 4081 | 4081 | 6.34 M |
| encode/bind_walk/merges | count | 82.27 | 103 | 127.8 k |
| encode/bind_walk/merges_skipped | count | 50.75 | 56 | 78.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1554 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1554 |
| encode/pass:early prepass/count | count | 1 | 1 | 1554 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 1554 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 1554 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 1554 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 1554 |
| encode/queue/command_buffers | count | 36.57 | 39 | 56.8 k |
| encode/queue/submits | count | 2 | 2 | 3108 |
| encode/queue/write_buffer/bytes | bytes | 1.44 M | 3.65 M | 2.23 G |
| encode/queue/write_buffer/calls | count | 211 | 281 | 328.4 k |
| encode/queue/write_buffer_with/bytes | bytes | 32.6 k | 32.6 k | 50.72 M |
| encode/queue/write_buffer_with/calls | count | 9 | 9 | 14.0 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 17.96 M |
| encode/queue/write_texture/calls | count | 0.00 | 1 | 4 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1554 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0 | 0 | 0 |
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
| surface/shelter_far_redraws | count | 0.00 | 1 | 1 |
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 55, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2577.3 | main/busy/ms 2546, stage/main/post_update 2442, stage/main/update 94.79, surface/advance_cpu_ms 65.87, stage/main/pre_update 5.20, stage/main/last 1.17, surface/cell_upload_ms 0.74, stream/drive/ms 0.63, stage/main/first 0.46, stage/main/fixed_main_loop 0.36, stream/tick/ms 0.13, weather/map/ms 0.12 |
| warm-up | 3.34 | 764.9 | main/busy/ms 2695, stage/main/post_update 2469, render/thread/ms 732, render/busy/ms 563, stage/render/prepare_assets 516, stage/main/update 210, stage/render/prepare_views 166, render/acquire/ms 166, surface/advance_cpu_ms 65.87, stage/render/create_views 27.91, stage/render/render 12.13, render/graph/ms 7.77 |
| warm-up | 3.45 | 111.5 | render/thread/ms 826, render/busy/ms 605, stage/render/prepare_assets 544, main/busy/ms 168, stage/render/prepare_views 166, render/acquire/ms 166, stage/main/update 120, stage/render/render 69.45, render/present/ms 55.40, pipelines/compile_ms 43.82, pipelines/front_ms 40.99, stage/main/post_update 37.95 |
| warm-up | 3.50 | 45.7 | pipelines/compile_ms 359, pipelines/front_ms 327, render/thread/ms 134, pipelines/wait_ms 129, stage/render/render 89.96, render/present/ms 72.04, main/busy/ms 66.60, render/busy/ms 60.80, stage/main/post_update 51.15, pipelines/driver_ms 32.02, stage/render/prepare_assets 29.77, terrain/update/ms 26.61 |
| warm-up | 3.62 | 117.6 | pipelines/wait_ms 3883, pipelines/compile_ms 1245, pipelines/front_ms 1167, render/thread/ms 152, render/busy/ms 129, main/busy/ms 93.70, stage/main/post_update 78.89, pipelines/driver_ms 78.11, stage/render/render 36.80, stage/render/prepare_resources 36.31, stage/render/prepare_assets 32.06, render/present/ms 20.61 |
| warm-up | 3.69 | 70.5 | pipelines/wait_ms 5538, pipelines/compile_ms 1882, pipelines/front_ms 1789, render/thread/ms 182, render/busy/ms 178, main/busy/ms 125, pipelines/driver_ms 93.52, stage/render/prepare_assets 73.30, stage/main/update 56.74, stage/main/post_update 49.30, stage/render/prepare_resources 40.84, stage/render/render 17.10 |
| warm-up | 3.78 | 95.3 | pipelines/wait_ms 1865, pipelines/compile_ms 1833, pipelines/front_ms 1762, render/thread/ms 134, render/busy/ms 132, stage/render/prepare_assets 97.06, main/busy/ms 95.40, pipelines/driver_ms 70.39, stage/main/update 60.30, stage/main/post_update 17.99, stage/render/render 17.63, stage/render/cleanup 16.53 |
| warm-up | 3.84 | 57.8 | pipelines/compile_ms 1714, pipelines/front_ms 1638, pipelines/wait_ms 212, render/thread/ms 121, render/busy/ms 118, stage/render/prepare_assets 82.69, pipelines/driver_ms 75.98, main/busy/ms 35.13, stage/render/render 28.95, render/graph/ms 25.18, stage/render/graph_submit 21.91, encode//sum 19.52 |
| warm-up | 3.94 | 36.1 | render/thread/ms 42.47, render/busy/ms 39.40, pipelines/compile_ms 38.96, pipelines/front_ms 36.93, main/busy/ms 27.19, stage/render/prepare_assets 17.20, stage/render/render 16.71, render/graph/ms 13.07, stage/main/post_update 11.45, terrain/update/ms 10.69, encode//sum 10.55, stage/render/graph_submit 10.14 |
| warm-up | 3.97 | 34.3 | pipelines/compile_ms 1000, pipelines/front_ms 959, main/busy/ms 61.27, render/thread/ms 58.15, render/busy/ms 56.54, stage/main/update 48.97, stream/drive/ms 43.65, pipelines/driver_ms 41.72, stage/render/prepare_assets 38.28, pipelines/wait_ms 31.96, stage/render/render 12.72, render/graph/ms 10.68 |
| warm-up | 4.04 | 61.5 | pipelines/compile_ms 1464, pipelines/front_ms 1410, render/thread/ms 66.90, render/busy/ms 65.57, main/busy/ms 60.36, pipelines/driver_ms 53.91, stage/render/prepare_assets 49.27, stage/main/update 49.05, stream/drive/ms 43.69, pipelines/wait_ms 34.50, stage/render/render 11.44, render/graph/ms 9.64 |
| warm-up | 4.11 | 66.3 | pipelines/compile_ms 203, pipelines/front_ms 196, render/thread/ms 47.73, render/busy/ms 45.77, main/busy/ms 34.54, stage/render/prepare_assets 28.82, stage/main/update 22.35, stream/drive/ms 18.06, stage/render/render 12.28, render/graph/ms 9.98, encode//sum 7.89, stage/render/graph_submit 7.40 |
| warm-up | 4.27 | 132.7 | pipelines/compile_ms 761, pipelines/front_ms 740, main/busy/ms 89.97, render/thread/ms 72.26, stage/main/update 72.03, render/busy/ms 70.85, stage/render/prepare_assets 52.71, stream/drive/ms 45.12, pipelines/driver_ms 21.45, stage/render/render 10.87, stage/main/post_update 8.81, render/graph/ms 8.77 |
| warm-up | 4.38 | 119.1 | pipelines/compile_ms 856, pipelines/front_ms 830, render/thread/ms 141, render/busy/ms 139, stage/render/prepare_assets 81.76, main/busy/ms 81.64, stage/main/update 66.79, stage/render/prepare_bind_groups 36.84, stream/drive/ms 36.43, pipelines/driver_ms 25.61, stage/render/render 12.19, render/graph/ms 10.05 |
| warm-up | 4.43 | 40.2 | pipelines/compile_ms 552, pipelines/front_ms 510, render/thread/ms 123, render/busy/ms 120, main/busy/ms 107, pipelines/wait_ms 57.22, stage/main/update 51.62, stage/render/prepare_assets 44.04, pipelines/driver_ms 42.73, stage/render/prepare_bind_groups 37.12, stream/drive/ms 36.78, stage/main/post_update 31.50 |
| warm-up | 4.48 | 55.3 | pipelines/compile_ms 1155, pipelines/front_ms 1093, pipelines/wait_ms 321, main/busy/ms 80.64, pipelines/driver_ms 62.86, render/thread/ms 62.57, render/busy/ms 59.85, stage/main/post_update 39.42, stage/render/render 30.66, render/graph/ms 27.72, stage/render/graph_submit 22.10, stage/main/update 18.34 |
| warm-up | 4.59 | 113.6 | pipelines/compile_ms 1395, pipelines/front_ms 1346, pipelines/wait_ms 301, render/thread/ms 135, render/busy/ms 133, main/busy/ms 63.83, pipelines/driver_ms 48.22, stage/render/prepare_write_phase_buffers 34.78, stage/render/prepare_resources 28.85, stage/main/update 23.36, stage/render/prepare_assets 20.03, stage/main/post_update 18.50 |
| warm-up | 4.63 | 35.7 | pipelines/compile_ms 1373, pipelines/front_ms 1329, render/thread/ms 130, render/busy/ms 129, pipelines/wait_ms 80.23, main/busy/ms 58.21, pipelines/driver_ms 43.95, stage/render/prepare_write_phase_buffers 34.42, stage/render/prepare_resources 28.43, stage/render/prepare_assets 26.06, stage/main/update 23.55, stage/main/pre_update 16.41 |
| warm-up | 4.70 | 71.1 | pipelines/compile_ms 1990, pipelines/front_ms 1942, pipelines/wait_ms 155, render/thread/ms 69.09, render/busy/ms 66.97, pipelines/driver_ms 47.45, stage/render/prepare_assets 41.64, main/busy/ms 33.92, stage/render/render 17.93, render/graph/ms 15.36, stage/main/update 13.23, stage/render/graph_submit 13.05 |
| warm-up | 4.75 | 47.9 | pipelines/compile_ms 1351, pipelines/front_ms 1322, pipelines/wait_ms 111, render/thread/ms 91.81, render/busy/ms 89.42, main/busy/ms 54.97, stage/render/prepare_assets 33.49, stage/main/update 28.62, pipelines/driver_ms 28.60, stage/render/prepare_resources 23.84, stage/render/render 21.80, render/graph/ms 19.12 |
| warm-up | 4.82 | 71.6 | pipelines/compile_ms 222, pipelines/front_ms 214, render/thread/ms 107, render/busy/ms 106, main/busy/ms 70.77, stage/render/prepare_resources 32.82, stage/main/post_update 31.03, stage/main/update 29.53, stage/render/prepare_assets 21.94, stage/render/prepare_write_phase_buffers 15.97, stage/render/render 13.45, bigspace/total/ms 12.23 |
| warm-up | 4.87 | 50.1 | pipelines/compile_ms 562, pipelines/front_ms 541, main/busy/ms 130, render/thread/ms 98.43, render/busy/ms 96.87, stage/main/update 92.47, stream/drive/ms 84.38, stage/render/prepare_assets 34.22, stage/main/post_update 25.15, pipelines/driver_ms 20.09, stage/render/prepare_write_phase_buffers 14.22, stage/render/render 12.80 |
| warm-up | 4.96 | 85.1 | pipelines/compile_ms 2609, pipelines/front_ms 2534, main/busy/ms 110, stage/main/update 91.33, stream/drive/ms 84.71, pipelines/driver_ms 74.92, render/thread/ms 53.48, render/busy/ms 52.16, pipelines/wait_ms 26.81, stage/render/prepare_assets 26.52, stage/render/render 10.57, stage/main/post_update 9.21 |
| warm-up | 5.00 | 32.6 | main/busy/ms 42.00, render/thread/ms 33.57, render/busy/ms 31.25, stage/main/update 22.57, encode//sum 20.78, stage/render/render 18.45, render/graph/ms 15.53, stage/render/graph_submit 12.08, stage/main/post_update 10.96, encode/submit 7.91, stage/render/prepare_assets 7.20, encode/finish_wall 4.26 |
| warm-up | 5.04 | 45.4 | pipelines/compile_ms 546, pipelines/front_ms 528, main/busy/ms 59.19, render/thread/ms 53.22, render/busy/ms 50.91, stage/main/update 41.93, encode//sum 20.99, stage/render/prepare_assets 20.40, stage/render/render 19.56, pipelines/driver_ms 17.92, render/graph/ms 16.75, stage/render/graph_submit 13.35 |
| warm-up | 5.08 | 37.4 | pipelines/compile_ms 744, pipelines/front_ms 720, pipelines/wait_ms 82.53, render/thread/ms 54.45, render/busy/ms 52.94, main/busy/ms 43.02, stage/render/prepare_assets 30.08, stage/main/update 29.59, pipelines/driver_ms 23.60, stream/drive/ms 10.56, stage/render/render 9.45, stage/main/post_update 7.89 |
| warm-up | 5.13 | 27.3 | pipelines/compile_ms 128, pipelines/front_ms 120, render/thread/ms 38.76, render/busy/ms 37.18, main/busy/ms 34.74, stage/main/update 21.27, stage/render/render 13.02, render/graph/ms 11.06, stage/render/graph_submit 8.58, stage/main/post_update 8.20, encode//sum 7.92, pipelines/driver_ms 7.34 |
| warm-up | 5.18 | 34.4 | pipelines/compile_ms 279, pipelines/front_ms 268, main/busy/ms 44.05, render/thread/ms 42.23, render/busy/ms 40.53, stage/main/update 23.04, stage/render/render 20.17, render/graph/ms 17.74, stream/drive/ms 16.55, stage/render/graph_submit 14.32, stage/main/post_update 11.37, pipelines/driver_ms 10.72 |
| warm-up | 5.25 | 42.9 | pipelines/compile_ms 250, pipelines/front_ms 239, render/thread/ms 50.28, render/busy/ms 49.01, main/busy/ms 47.25, stage/main/update 22.00, stage/render/prepare_assets 20.70, stage/main/post_update 19.92, stage/render/render 13.33, render/graph/ms 11.49, pipelines/driver_ms 10.47, encode//sum 7.56 |
| warm-up | 5.29 | 41.0 | pipelines/compile_ms 981, pipelines/front_ms 945, render/thread/ms 65.78, render/busy/ms 63.02, main/busy/ms 55.85, stage/render/prepare_assets 37.96, pipelines/driver_ms 36.02, stage/main/update 25.46, stage/main/post_update 24.32, stage/render/render 14.65, render/graph/ms 11.23, encode//sum 7.30 |
| warm-up | 5.34 | 52.1 | pipelines/compile_ms 1013, pipelines/front_ms 978, render/thread/ms 78.85, render/busy/ms 76.02, main/busy/ms 64.68, stage/render/prepare_assets 34.90, pipelines/driver_ms 34.48, stage/main/update 23.88, stage/main/post_update 21.90, stage/render/prepare_resources 14.56, stage/main/pre_update 13.92, stage/render/render 11.93 |
| warm-up | 5.38 | 36.6 | pipelines/compile_ms 895, pipelines/front_ms 868, render/thread/ms 70.39, render/busy/ms 68.82, main/busy/ms 66.21, stage/render/prepare_assets 28.37, pipelines/driver_ms 26.75, stage/main/update 23.25, stage/main/post_update 19.94, stage/render/prepare_resources 15.64, stage/main/pre_update 13.49, stage/render/render 10.72 |
| warm-up | 5.41 | 30.8 | pipelines/compile_ms 700, pipelines/front_ms 678, render/thread/ms 52.30, render/busy/ms 50.72, main/busy/ms 44.28, pipelines/wait_ms 25.20, stage/render/prepare_assets 23.56, pipelines/driver_ms 22.30, stage/main/update 20.70, stage/render/render 12.85, stage/main/post_update 12.74, render/graph/ms 10.79 |
| warm-up | 5.55 | 28.5 | pipelines/compile_ms 605, pipelines/front_ms 593, render/thread/ms 39.56, render/busy/ms 38.29, main/busy/ms 26.40, pipelines/wait_ms 24.03, stage/render/prepare_assets 20.78, pipelines/driver_ms 12.45, stage/main/update 11.01, stage/main/post_update 10.32, stage/render/render 9.80, render/graph/ms 8.11 |
| warm-up | 5.62 | 35.5 | pipelines/compile_ms 208, pipelines/front_ms 204, render/thread/ms 53.71, render/busy/ms 52.32, main/busy/ms 34.22, stage/render/prepare_assets 26.30, stage/main/update 17.82, stage/render/render 16.01, render/graph/ms 13.91, stage/render/graph_submit 11.29, stage/main/post_update 10.42, encode//sum 8.69 |
| warm-up | 5.66 | 33.4 | pipelines/compile_ms 117, pipelines/front_ms 111, render/thread/ms 60.80, render/busy/ms 59.43, stage/render/prepare_assets 32.36, main/busy/ms 28.21, stage/render/render 14.31, stage/main/update 13.86, render/graph/ms 12.50, stage/render/graph_submit 9.75, stage/main/post_update 9.06, encode//sum 6.39 |
| warm-up | 5.72 | 44.8 | pipelines/compile_ms 165, pipelines/front_ms 158, render/thread/ms 51.40, render/busy/ms 49.96, main/busy/ms 35.16, stage/render/prepare_assets 23.27, stage/main/update 14.31, stage/render/render 13.95, encode//sum 12.97, stage/main/post_update 12.90, render/graph/ms 12.04, stage/render/graph_submit 9.01 |
| warm-up | 5.75 | 33.7 | pipelines/compile_ms 165, pipelines/front_ms 158, render/thread/ms 62.45, render/busy/ms 61.26, stage/render/prepare_assets 36.62, main/busy/ms 36.06, stage/main/update 15.89, stage/main/post_update 11.95, stage/render/render 10.99, render/graph/ms 9.38, encode//sum 8.07, pipelines/driver_ms 6.82 |
| warm-up | 5.78 | 31.7 | render/thread/ms 52.97, render/busy/ms 51.89, main/busy/ms 33.43, stage/render/prepare_assets 31.43, stage/main/update 14.45, stage/main/post_update 11.84, stage/render/render 9.48, render/graph/ms 7.97, encode//sum 7.24, stage/render/graph_submit 5.25, stream/drive/ms 4.57, encode/submit 3.18 |
| warm-up | 5.83 | 27.2 | pipelines/compile_ms 901, pipelines/front_ms 885, render/thread/ms 43.14, render/busy/ms 41.96, main/busy/ms 26.15, stage/render/prepare_assets 25.37, pipelines/driver_ms 16.23, stage/main/update 11.50, stage/render/render 9.85, stage/main/post_update 9.28, render/graph/ms 8.22, stage/render/graph_submit 6.10 |
| warm-up | 5.87 | 32.5 | pipelines/compile_ms 139, pipelines/front_ms 131, render/thread/ms 52.09, render/busy/ms 50.73, main/busy/ms 46.63, stage/main/update 31.42, stage/render/prepare_assets 19.07, stage/render/render 16.83, render/graph/ms 14.82, stage/render/graph_submit 11.25, stage/main/post_update 9.72, encode//sum 8.54 |
| warm-up | 5.91 | 43.1 | pipelines/compile_ms 139, pipelines/front_ms 131, render/thread/ms 57.66, render/busy/ms 56.27, main/busy/ms 51.23, stage/main/update 31.14, stage/render/prepare_assets 21.38, stage/render/render 16.57, render/graph/ms 14.38, stage/main/post_update 12.51, encode//sum 10.68, stage/render/graph_submit 10.09 |
| warm-up | 5.97 | 35.6 | main/busy/ms 46.41, render/thread/ms 44.35, render/busy/ms 42.35, stage/main/update 26.26, stage/render/render 19.02, stream/drive/ms 17.01, render/graph/ms 16.39, stage/main/post_update 13.41, stage/render/prepare_assets 13.34, stage/render/graph_submit 12.84, encode//sum 10.65, encode/submit 7.30 |
| warm-up | 6.01 | 42.6 | main/busy/ms 53.71, render/thread/ms 51.28, render/busy/ms 49.54, stage/main/update 33.13, stage/render/prepare_assets 21.05, stream/drive/ms 17.49, stage/render/render 17.16, render/graph/ms 14.92, stage/main/post_update 13.49, pipelines/compile_ms 13.31, pipelines/front_ms 12.12, stage/render/graph_submit 11.51 |
| warm-up | 6.04 | 26.9 | pipelines/compile_ms 1003, pipelines/front_ms 974, render/thread/ms 47.26, render/busy/ms 45.58, main/busy/ms 45.34, pipelines/driver_ms 28.77, stage/main/update 23.82, stage/render/prepare_assets 20.24, stage/render/render 13.95, stage/main/post_update 13.73, render/graph/ms 11.86, encode//sum 8.90 |
| warm-up | 6.18 | 26.0 | pipelines/compile_ms 61.05, pipelines/front_ms 58.27, main/busy/ms 38.71, render/thread/ms 34.52, render/busy/ms 32.64, stage/main/post_update 17.62, stage/render/render 16.61, stage/main/update 14.48, render/graph/ms 14.29, stage/render/graph_submit 9.07, stage/render/prepare_assets 7.29, encode//sum 6.89 |
| warm-up | 6.21 | 28.1 | pipelines/compile_ms 111, pipelines/front_ms 107, main/busy/ms 57.64, render/thread/ms 46.17, render/busy/ms 44.22, stage/main/update 32.52, stream/drive/ms 21.96, stage/render/render 17.57, stage/main/post_update 16.79, render/graph/ms 15.20, stage/render/graph_submit 9.49, encode//sum 8.89 |
| warm-up | 6.24 | 35.3 | pipelines/compile_ms 760, pipelines/front_ms 727, main/busy/ms 53.88, render/thread/ms 43.30, render/busy/ms 41.54, stage/main/update 33.14, pipelines/driver_ms 32.67, stream/drive/ms 22.34, stage/main/post_update 14.47, stage/render/render 13.95, render/graph/ms 11.78, stage/render/prepare_assets 11.19 |
| warm-up | 6.33 | 35.6 | pipelines/compile_ms 190, pipelines/front_ms 184, main/busy/ms 40.22, render/thread/ms 31.66, render/busy/ms 30.26, stage/main/update 19.44, stage/main/post_update 13.98, stage/render/render 13.15, render/graph/ms 11.37, stage/render/prepare_assets 10.52, encode//sum 8.13, stage/render/graph_submit 8.00 |
| warm-up | 6.53 | 27.1 | main/busy/ms 35.05, render/thread/ms 30.91, render/busy/ms 29.26, stage/main/update 16.67, stage/render/render 15.62, stage/main/post_update 14.18, render/graph/ms 13.35, encode//sum 10.19, stage/render/graph_submit 9.41, stage/render/prepare_assets 6.67, encode/submit 6.08, stage/render/camera_driver 3.74 |
| warm-up | 6.56 | 29.2 | main/busy/ms 44.26, render/thread/ms 41.44, render/busy/ms 39.86, stage/main/update 20.07, stage/render/render 17.03, stage/main/post_update 16.90, render/graph/ms 14.54, stage/render/prepare_assets 13.56, encode//sum 10.43, stage/render/graph_submit 10.07, stream/drive/ms 9.74, encode/finish_wall 5.31 |
| warm-up | 6.60 | 41.1 | render/thread/ms 49.01, render/busy/ms 47.89, main/busy/ms 43.61, stage/render/prepare_assets 28.72, stage/main/update 20.68, stage/main/post_update 15.40, stage/render/render 12.69, render/graph/ms 10.86, stream/drive/ms 10.47, encode//sum 9.32, stage/render/graph_submit 7.75, encode/finish_wall 4.47 |
| warm-up | 6.94 | 31.8 | pipelines/compile_ms 138, pipelines/front_ms 135, render/thread/ms 36.30, render/busy/ms 34.95, main/busy/ms 14.27, stage/render/render 9.79, stage/render/specialize 9.17, render/graph/ms 8.45, stage/main/post_update 7.68, encode//sum 7.11, stage/render/graph_submit 6.35, stage/render/queue 5.95 |
| warm-up | 8.14 | 29.2 | render/thread/ms 35.49, render/busy/ms 33.12, main/busy/ms 14.90, stage/render/render 9.42, stage/render/specialize 8.66, stage/main/post_update 8.37, render/graph/ms 8.27, stage/render/graph_submit 6.41, encode//sum 5.75, stage/render/queue 5.73, stage/render/prepare_meshes 4.05, encode/submit 3.82 |
| measure | 0.82 | 27.2 | render/thread/ms 34.45, render/busy/ms 31.68, main/busy/ms 13.31, stage/render/render 10.78, render/graph/ms 9.40, encode//sum 8.67, stage/render/specialize 8.18, stage/render/graph_submit 7.61, stage/main/post_update 7.13, stage/render/queue 6.09, encode/submit 4.57, stage/main/update 3.75 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora@2560x1440_arm=daygate-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 13%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (280 pmon samples)
- focus: clean
- nvidia-smi over the window (58 samples): GPU load **98%** (min 86%) · 2625 MHz (min 2625) · memory 8101 MHz · 97.0 W · 75 °C (max 79) · VRAM 6431 MiB · throttled 100% of samples

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
- `shadow.lamp.day_gate=on (command line: --set)`
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
