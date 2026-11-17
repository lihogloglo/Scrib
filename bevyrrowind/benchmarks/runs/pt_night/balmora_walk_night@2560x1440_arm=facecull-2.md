# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.face_cull=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6201 frames over 65.6 s · **median 10.23 ms (98 fps)** · mean 10.59 · p95 13.43 · p99 15.58 (1% low 64 fps) · worst 27.54 ms · 71 spikes over 1.5× median · 20.1 stutters/min (work 17.4/min)
- warm-up: 323 frames over 9.3 s, worst 2500.5 ms
- hitches (> 25.6 ms): **5** measured, 51 in warm-up
- timed GPU passes: 6.54 ms of a 10.59 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.23 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- pass worst frame 27.54 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.988 | 2.029 | 2.062 | 0.98 | 0.172 |
| main_opaque_pass_3d | 1.262 | 1.287 | 1.616 | 0.98 | 0.060 |
| early prepass | 0.475 | 0.485 | 0.695 | 0.98 | 0.086 |
| bloom | 0.309 | 0.316 | 0.323 | 0.98 | 0.062 |
| atmosphere_luts | 0.304 | 0.311 | 0.373 | 0.98 | 0.003 |
| water_velocity | 0.258 | 0.263 | 0.350 | 0.98 | 0.002 |
| auxiliary_views | 0.203 | 0.207 | 0.883 | 0.98 | 0.192 |
| water_sspr | 0.197 | 0.201 | 0.203 | 0.98 | 0.002 |
| ssao | 0.184 | 0.188 | 0.193 | 0.98 | 0.007 |
| upscaling | 0.127 | 0.130 | 0.137 | 0.98 | 0.004 |
| water_fft | 0.126 | 0.129 | 0.137 | 0.98 | 0.003 |
| atmos_froxel_inject | 0.125 | 0.127 | 0.128 | 0.98 | 0.002 |
| uniform_allocation | 0.122 | 0.124 | 0.152 | 0.98 | 0.025 |
| clouds_composite | 0.103 | 0.105 | 0.108 | 0.98 | 0.002 |
| bin_unpacking | 0.095 | 0.097 | 0.118 | 0.98 | 0.018 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.079 | 0.155 | 0.98 | 0.009 |
| clustering | 0.063 | 0.064 | 0.068 | 0.98 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.074 | 0.98 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.98 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.046 | 0.412 | 0.415 | 0.11 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.68 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.037 | 0.98 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.98 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.219 | 0.229 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| night_sky | 0.016 | 0.017 | 0.042 | 0.98 | 0.001 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.98 | 0.125 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.98 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.136 | 12.060 | 22.48 | 56650.7 |
| main/busy/ms | 8.946 | 10.833 | 25.03 | 55473.2 |
| render/busy/ms | 8.622 | 11.547 | 21.92 | 53465.2 |
| stage/main/post_update | 5.534 | 7.035 | 9.93 | 34316.6 |
| stage/render/render | 5.459 | 7.699 | 13.86 | 33851.0 |
| render/graph/ms | 4.763 | 6.967 | 12.95 | 29538.4 |
| encode//sum | 4.014 | 6.009 | 19.89 | 24892.6 |
| stage/render/graph_submit | 3.547 | 5.140 | 9.31 | 21993.6 |
| encode/submit | 2.301 | 3.509 | 5.92 | 14266.2 |
| stage/main/update | 2.166 | 2.629 | 18.78 | 13432.3 |
| encode/finish_wall | 1.283 | 1.769 | 3.68 | 7956.3 |
| stage/render/camera_driver | 1.164 | 1.812 | 3.87 | 7219.7 |
| encode/pass:shadow_point_light/sum | 1.054 | 4.321 | 15.45 | 6533.2 |
| encode/pass:shadow_directional_light/sum | 0.734 | 1.192 | 1.72 | 4550.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.719 | 1.029 | 1.61 | 4458.5 |
| stage/render/prepare_write_phase_buffers | 0.718 | 1.037 | 2.20 | 4452.7 |
| encode/pass:early prepass/sum | 0.706 | 1.022 | 1.53 | 4378.1 |
| bigspace/total/ms | 0.571 | 1.366 | 2.82 | 3541.6 |
| stage/render/prepare_assets | 0.570 | 0.723 | 9.88 | 3535.6 |
| stage/render/prepare_resources | 0.447 | 0.534 | 0.92 | 2770.0 |
| render/present/ms | 0.419 | 0.531 | 2.40 | 2600.4 |
| stage/render/prepare_meshes | 0.408 | 0.526 | 2.26 | 2528.1 |
| stage/main/first | 0.396 | 0.522 | 6.22 | 2457.8 |
| stage/main/pre_update | 0.352 | 0.501 | 1.45 | 2182.7 |
| terrain/update/ms | 0.313 | 0.404 | 2.14 | 1940.9 |
| terrain/select/ms | 0.280 | 0.365 | 0.95 | 1733.3 |
| controller/feed/ms | 0.254 | 0.324 | 0.89 | 1578.0 |
| stage/render/prepare_bind_groups | 0.249 | 0.412 | 2.22 | 1541.2 |
| stage/render/specialize | 0.248 | 0.379 | 2.37 | 1538.9 |
| stream/drive/ms | 0.246 | 0.331 | 16.82 | 1522.6 |
| stage/render/queue | 0.237 | 0.378 | 2.32 | 1470.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.236 | 0.348 | 0.93 | 1465.4 |
| bigspace/low_precision/ms | 0.232 | 0.754 | 1.60 | 1440.3 |
| stage/main/fixed_main_loop | 0.209 | 0.365 | 1.03 | 1294.6 |
| far/select/ms | 0.202 | 0.269 | 0.65 | 1252.8 |
| stage/render/create_views | 0.192 | 0.248 | 1.47 | 1192.8 |
| stream/tick/ms | 0.191 | 0.261 | 0.57 | 1182.5 |
| stage/render/prepare_batch_phases | 0.187 | 0.265 | 0.70 | 1161.3 |
| anim/evaluate/ms | 0.174 | 0.377 | 1.90 | 1079.4 |
| stage/render/extract_commands | 0.171 | 0.200 | 0.43 | 1061.9 |
| bigspace/high_precision/ms | 0.170 | 0.788 | 1.88 | 1056.8 |
| stage/render/prepare_flush | 0.169 | 0.355 | 0.97 | 1049.5 |
| anim/evaluate_cpu/ms | 0.154 | 0.191 | 0.91 | 954.8 |
| controller/move/ms | 0.148 | 0.246 | 0.59 | 915.6 |
| stage/render/prepare_views | 0.138 | 0.173 | 0.52 | 855.5 |
| stage/main/last | 0.112 | 0.151 | 1.76 | 697.2 |
| pipelines/compile_ms | 0.107 | 0.000 | 133.03 | 663.9 |
| pipelines/front_ms | 0.104 | 0.000 | 129.98 | 646.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.098 | 0.162 | 0.68 | 604.6 |
| render/acquire/ms | 0.094 | 0.121 | 0.33 | 585.2 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.127 | 0.47 | 364.0 |
| physics/step/ms | 0.046 | 0.086 | 0.58 | 282.4 |
| stage/render/phase_sort | 0.036 | 0.061 | 0.32 | 221.9 |
| stage/render/cleanup | 0.032 | 0.097 | 0.86 | 199.6 |
| physics/insert/ms | 0.032 | 0.351 | 1.03 | 199.2 |
| encode/resolve_diagnostics/sum | 0.029 | 0.040 | 0.50 | 182.6 |
| weather/map/ms | 0.016 | 0.021 | 0.22 | 102.1 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.19 | 87.7 |
| anim/apply/ms | 0.011 | 0.021 | 0.21 | 68.2 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.46 | 66.4 |
| controller/avoid/ms | 0.010 | 0.013 | 0.18 | 63.9 |
| weather/eval/ms | 0.010 | 0.013 | 0.13 | 60.0 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 42.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.15 | 31.3 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 31.1 |
| script/ui/ms | 0.005 | 0.006 | 0.26 | 28.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 28.3 |
| controller/write/ms | 0.003 | 0.004 | 0.21 | 21.6 |
| pipelines/driver_ms | 0.003 | 0.000 | 3.22 | 17.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.63 | 14.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.48 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1281 | 2262 | 594 |
| crowd/c0/meshes/count | count | 353 | 646 | 150 |
| crowd/c1/count | count | 146 | 157 | 147 |
| crowd/c1/joints/count | count | 5531 | 5999 | 5570 |
| crowd/c1/meshes/count | count | 1177 | 1297 | 1183 |
| crowd/c2/count | count | 397 | 418 | 418 |
| crowd/c2/joints/count | count | 6406 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1822 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 215 | 204 |
| crowd/joints_extracted/count | count | 3696 | 7455 | 5731 |
| crowd/joints_uploaded/count | count | 3649 | 6186 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.40 | 1.34 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.19 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.46 | 0.01 |
| encode/finish_critical | ms | 0.91 | 1.72 | 0.87 |
| encode/pass:early prepass/max | ms | 0.73 | 1.53 | 0.76 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.74 | 1.61 | 0.87 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.68 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 0.93 | 0.19 |
| encode/pass:shadow_directional_light/max | ms | 0.74 | 1.72 | 0.77 |
| encode/pass:shadow_point_light/max | ms | 0.75 | 1.63 | 0.81 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.50 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.04 | 25.03 | 7.96 |
| gpu/timed/ms | ms | 7.38 | 10.76 | 7.73 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1285 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3362 | 5102 | 3193 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1285 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3362 | 5102 | 3193 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 193 | 4 |
| phases/shadow/bins | count | 3766 | 4402 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6158 | 7378 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 198 | 227 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 11 | 544 | 9 |
| preprocess/bind_groups_created | count | 11 | 544 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 240 | 124 |
| process/cpu_usage | % | 27.18 | 40.38 | 22.60 |
| process/mem_usage | GiB | 5.39 | 5.55 | 5.55 |
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
| skin/blocks_changed/count | count | 13 | 86 | 9 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 215 | 204 |
| skin/joint_bytes_written/bytes | bytes | 277.5 k | 811.1 k | 446.0 k |
| skin/joints_written/count | count | 4336 | 12.7 k | 6968 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5009 | 5009 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.43 | 47.20 | 33.53 |
| system/mem_usage | % | 53.78 | 54.27 | 54.21 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 63 | 242 | 56 |
| views/all/bins | count | 6286 | 7268 | 4989 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 20.2 k | 10.6 k |
| views/all/pipelines | count | 245 | 274 | 67 |
| views/all/pipelines_distinct | count | 70 | 75 | 63 |
| views/all/work_items | count | 10.6 k | 17.6 k | 8152 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1443 | 1521 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3400 | 4450 | 3025 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 334 | 4450 | 222 |
| views/lamp_face/batch_sets | count | 0 | 170 | 0 |
| views/lamp_face/bins | count | 2509 | 2993 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 24 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2747 | 3259 | 0 |
| views/lamp_face/pipelines | count | 175 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 7 | 0 |
| views/lamp_face/work_items | count | 0 | 2594 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2880 | 3590 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8324 | 13.0 k | 7622 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8494 | 13.2 k | 7930 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 43.22 | 238 | 268.0 k |
| anim/events/count | count | 0.08 | 3 | 492 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.65 | 152 | 4061 |
| assets/Mesh/removed | count | 0.78 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.50 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.75 | 1272 | 4675 |
| encode//count | count | 35.53 | 203 | 220.3 k |
| encode/bind_walk/init_textures | count | 13.6 k | 106.3 k | 84.14 M |
| encode/bind_walk/init_textures_skipped | count | 59.3 k | 128.0 k | 367.63 M |
| encode/bind_walk/inits | count | 458 | 1093 | 2.84 M |
| encode/bind_walk/inits_skipped | count | 104 | 145 | 641.8 k |
| encode/bind_walk/merge_textures | count | 13.5 k | 106.2 k | 83.75 M |
| encode/bind_walk/merge_textures_skipped | count | 5385 | 6012 | 33.39 M |
| encode/bind_walk/merges | count | 101 | 326 | 626.0 k |
| encode/bind_walk/merges_skipped | count | 81.67 | 125 | 506.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6201 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6201 |
| encode/pass:early prepass/count | count | 1 | 1 | 6201 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6201 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6201 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6201 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6201 |
| encode/pass:shadow_point_light/count | count | 2.15 | 44 | 13.3 k |
| encode/queue/command_buffers | count | 47.69 | 257 | 295.7 k |
| encode/queue/submits | count | 2.02 | 4 | 12.5 k |
| encode/queue/write_buffer/bytes | bytes | 1.81 M | 2.74 M | 11.20 G |
| encode/queue/write_buffer/calls | count | 254 | 814 | 1.58 M |
| encode/queue/write_buffer_with/bytes | bytes | 167.2 k | 3.53 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 10.95 | 465 | 67.9 k |
| encode/queue/write_texture/bytes | bytes | 10.2 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6201 |
| physics/inserts | count | 0.36 | 111 | 2249 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 7 |
| pipelines/created | count | 0.00 | 1 | 8 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5459 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1197 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.6 ms; slowest 56, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2500.5 | main/busy/ms 2465, stage/main/post_update 2425, stage/main/update 31.05, stage/main/pre_update 4.80, stage/main/last 1.14, surface/advance_cpu_ms 0.73, stage/main/first 0.57, stream/drive/ms 0.47, stage/main/fixed_main_loop 0.33, surface/cell_upload_ms 0.29, bigspace/total/ms 0.24, bigspace/high_precision/ms 0.21 |
| warm-up | 3.25 | 749.5 | main/busy/ms 2602, stage/main/post_update 2450, render/thread/ms 717, render/busy/ms 541, stage/render/prepare_assets 492, stage/render/prepare_views 171, render/acquire/ms 171, stage/main/update 138, stage/render/create_views 28.27, stage/render/render 14.74, render/graph/ms 8.34, stage/main/pre_update 6.61 |
| warm-up | 3.43 | 179.1 | render/thread/ms 879, render/busy/ms 608, stage/render/prepare_assets 543, stage/render/prepare_views 171, render/acquire/ms 171, pipelines/compile_ms 169, main/busy/ms 166, stage/render/render 115, stage/main/update 114, render/present/ms 100, pipelines/front_ms 85.82, pipelines/driver_ms 83.06 |
| warm-up | 3.46 | 27.6 | pipelines/compile_ms 504, pipelines/front_ms 298, pipelines/driver_ms 205, render/thread/ms 178, stage/render/render 109, main/busy/ms 104, render/present/ms 96.29, render/busy/ms 80.95, stage/main/post_update 72.10, stage/render/prepare_assets 51.91, terrain/update/ms 26.56, stage/main/update 16.75 |
| warm-up | 3.59 | 129.0 | pipelines/wait_ms 3929, pipelines/compile_ms 1932, pipelines/front_ms 1697, pipelines/driver_ms 235, main/busy/ms 168, render/thread/ms 91.89, render/busy/ms 89.44, stage/main/update 88.64, stage/main/post_update 69.59, stage/render/prepare_assets 29.03, stage/render/prepare_resources 27.05, stage/render/render 21.12 |
| warm-up | 3.69 | 101.5 | pipelines/wait_ms 4190, pipelines/compile_ms 2314, pipelines/front_ms 2173, render/thread/ms 153, render/busy/ms 151, pipelines/driver_ms 142, main/busy/ms 115, stage/render/prepare_assets 93.14, stage/main/update 83.79, stage/render/prepare_resources 26.29, stage/main/post_update 23.30, stage/render/render 19.39 |
| warm-up | 3.73 | 46.4 | pipelines/compile_ms 829, pipelines/front_ms 794, pipelines/wait_ms 302, render/thread/ms 122, render/busy/ms 120, main/busy/ms 79.01, encode//sum 71.61, stage/render/prepare_assets 70.31, stage/main/post_update 56.45, stage/render/render 39.14, render/graph/ms 36.73, pipelines/driver_ms 35.44 |
| warm-up | 3.77 | 34.6 | pipelines/compile_ms 455, pipelines/front_ms 427, pipelines/wait_ms 82.27, encode//sum 72.11, main/busy/ms 71.23, render/thread/ms 66.26, render/busy/ms 64.31, stage/main/post_update 54.88, stage/render/render 37.36, render/graph/ms 35.34, stage/render/graph_submit 30.11, pipelines/driver_ms 28.44 |
| warm-up | 3.86 | 27.0 | pipelines/compile_ms 75.46, pipelines/front_ms 71.41, main/busy/ms 54.53, render/thread/ms 47.45, render/busy/ms 45.36, stage/main/update 40.86, stream/drive/ms 36.72, stage/render/prepare_assets 20.92, stage/render/render 13.66, encode//sum 12.73, terrain/update/ms 11.63, render/graph/ms 11.31 |
| warm-up | 3.91 | 58.7 | pipelines/compile_ms 1166, pipelines/front_ms 1117, render/thread/ms 60.56, render/busy/ms 58.60, main/busy/ms 55.76, pipelines/driver_ms 48.60, stage/main/update 42.57, stream/drive/ms 37.08, stage/render/prepare_assets 36.86, stage/render/render 11.94, render/graph/ms 9.77, stage/main/post_update 7.48 |
| warm-up | 3.95 | 30.9 | pipelines/compile_ms 1162, pipelines/front_ms 1110, render/thread/ms 64.14, render/busy/ms 60.71, main/busy/ms 59.32, pipelines/driver_ms 51.63, stage/main/update 45.02, stage/render/prepare_assets 38.45, stream/drive/ms 36.28, stage/render/render 18.86, render/graph/ms 15.00, stage/main/post_update 8.48 |
| warm-up | 4.01 | 62.1 | pipelines/compile_ms 412, pipelines/front_ms 396, render/thread/ms 67.30, render/busy/ms 63.59, main/busy/ms 59.90, stage/main/update 45.06, stage/render/prepare_assets 38.38, stream/drive/ms 37.19, stage/render/render 20.68, render/graph/ms 16.31, pipelines/driver_ms 16.12, pipelines/wait_ms 14.51 |
| warm-up | 4.04 | 31.3 | pipelines/compile_ms 595, pipelines/front_ms 575, render/thread/ms 66.97, render/busy/ms 64.49, main/busy/ms 41.72, stage/render/prepare_assets 35.43, stage/main/update 24.17, pipelines/driver_ms 19.80, stage/render/render 19.41, render/graph/ms 15.06, stream/drive/ms 14.60, pipelines/wait_ms 12.66 |
| warm-up | 4.12 | 77.9 | pipelines/compile_ms 445, pipelines/front_ms 426, render/thread/ms 78.12, render/busy/ms 75.83, stage/render/prepare_assets 47.68, main/busy/ms 44.14, stage/main/update 24.93, pipelines/driver_ms 18.99, stage/render/render 18.61, stream/drive/ms 15.22, render/graph/ms 14.68, stage/main/post_update 11.61 |
| warm-up | 4.17 | 48.9 | pipelines/compile_ms 1144, pipelines/front_ms 1106, render/thread/ms 79.74, render/busy/ms 78.30, stage/render/prepare_assets 62.05, pipelines/driver_ms 38.14, main/busy/ms 34.97, stage/main/update 17.56, pipelines/wait_ms 13.29, stage/main/post_update 11.01, stage/render/render 10.41, render/graph/ms 8.49 |
| warm-up | 4.21 | 48.5 | pipelines/compile_ms 1616, pipelines/front_ms 1568, render/thread/ms 59.85, render/busy/ms 58.36, pipelines/driver_ms 48.03, stage/render/prepare_assets 43.63, main/busy/ms 39.53, stage/main/update 24.24, pipelines/wait_ms 13.12, stage/render/render 8.91, stage/main/post_update 8.20, pipelines/lock_ms 7.19 |
| warm-up | 4.26 | 49.5 | pipelines/compile_ms 774, pipelines/front_ms 750, main/busy/ms 79.21, render/thread/ms 75.47, render/busy/ms 73.67, stage/main/update 52.59, stage/render/prepare_assets 26.27, pipelines/driver_ms 23.71, stage/render/render 21.92, render/graph/ms 19.81, stage/render/prepare_resources 15.97, stage/main/post_update 11.85 |
| warm-up | 4.31 | 47.4 | pipelines/compile_ms 134, pipelines/front_ms 128, main/busy/ms 79.18, render/thread/ms 70.53, render/busy/ms 68.87, stage/main/update 49.05, stage/render/prepare_assets 21.94, stage/render/render 21.42, render/graph/ms 19.36, stage/render/prepare_resources 16.04, pipelines/wait_ms 13.59, stage/main/post_update 13.32 |
| warm-up | 4.36 | 49.3 | pipelines/compile_ms 476, pipelines/front_ms 463, render/thread/ms 66.75, render/busy/ms 64.84, main/busy/ms 58.88, stage/main/update 27.12, stage/render/prepare_assets 23.85, stage/main/post_update 15.83, stage/render/prepare_resources 15.38, pipelines/driver_ms 13.53, pipelines/wait_ms 13.00, stage/render/render 9.86 |
| warm-up | 4.44 | 79.2 | pipelines/compile_ms 1991, pipelines/front_ms 1934, pipelines/wait_ms 128, render/thread/ms 110, render/busy/ms 108, main/busy/ms 88.31, pipelines/driver_ms 56.88, stage/main/post_update 48.16, stage/render/prepare_assets 36.28, stage/render/prepare_resources 34.28, stage/main/update 24.93, stage/render/render 13.11 |
| warm-up | 4.52 | 81.5 | pipelines/compile_ms 2713, pipelines/front_ms 2635, pipelines/wait_ms 420, render/thread/ms 145, render/busy/ms 143, main/busy/ms 97.07, pipelines/driver_ms 78.59, stage/main/post_update 68.15, stage/render/prepare_resources 44.48, stage/render/prepare_assets 42.54, bigspace/total/ms 31.54, stage/render/render 20.09 |
| warm-up | 4.59 | 69.8 | pipelines/compile_ms 1405, pipelines/front_ms 1358, pipelines/wait_ms 369, main/busy/ms 173, render/thread/ms 139, render/busy/ms 136, stage/main/update 118, encode//sum 73.59, stream/drive/ms 60.24, pipelines/driver_ms 47.78, stage/render/render 39.38, stage/main/post_update 36.49 |
| warm-up | 4.76 | 169.6 | pipelines/compile_ms 3306, pipelines/front_ms 3194, pipelines/wait_ms 718, main/busy/ms 149, render/thread/ms 141, render/busy/ms 138, stage/main/update 118, pipelines/driver_ms 111, encode//sum 69.96, stream/drive/ms 60.94, stage/render/prepare_assets 44.10, stage/render/render 42.97 |
| warm-up | 4.82 | 60.2 | pipelines/compile_ms 3594, pipelines/front_ms 3473, pipelines/wait_ms 754, render/thread/ms 134, render/busy/ms 132, pipelines/driver_ms 121, main/busy/ms 91.17, stage/main/update 65.44, encode//sum 56.03, stream/drive/ms 54.70, stage/render/render 47.16, stage/render/prepare_assets 47.04 |
| warm-up | 4.91 | 92.2 | pipelines/compile_ms 864, pipelines/front_ms 827, pipelines/wait_ms 124, main/busy/ms 106, render/thread/ms 103, render/busy/ms 101, stage/main/update 78.13, stream/drive/ms 54.65, encode//sum 51.27, stage/render/prepare_assets 42.99, pipelines/driver_ms 36.88, stage/render/render 36.20 |
| warm-up | 4.96 | 46.0 | pipelines/compile_ms 735, pipelines/front_ms 707, main/busy/ms 98.39, render/thread/ms 83.80, render/busy/ms 81.83, stage/main/update 72.84, stage/render/prepare_assets 54.90, stream/drive/ms 49.01, pipelines/driver_ms 27.81, stage/main/post_update 15.84, stage/render/render 12.99, pipelines/wait_ms 11.00 |
| warm-up | 5.03 | 72.0 | pipelines/compile_ms 1558, pipelines/front_ms 1510, main/busy/ms 83.10, render/thread/ms 72.67, render/busy/ms 70.55, stage/main/update 61.48, pipelines/driver_ms 48.74, stream/drive/ms 48.28, stage/render/prepare_assets 28.06, stage/render/render 24.12, render/graph/ms 21.26, stage/render/graph_submit 16.80 |
| warm-up | 5.06 | 33.2 | pipelines/compile_ms 1157, pipelines/front_ms 1122, main/busy/ms 74.48, render/thread/ms 64.67, render/busy/ms 62.96, stage/main/update 54.89, pipelines/driver_ms 34.94, stream/drive/ms 32.42, stage/render/render 23.85, render/graph/ms 21.59, stage/render/prepare_assets 20.62, stage/render/graph_submit 16.67 |
| warm-up | 5.12 | 57.4 | pipelines/compile_ms 532, pipelines/front_ms 516, main/busy/ms 64.41, render/thread/ms 49.59, stage/main/update 49.33, render/busy/ms 47.82, stream/drive/ms 31.99, stage/render/prepare_assets 21.74, pipelines/driver_ms 15.48, stage/render/render 14.22, render/graph/ms 11.66, encode//sum 11.36 |
| warm-up | 5.15 | 26.6 | pipelines/compile_ms 738, pipelines/front_ms 717, render/thread/ms 47.53, render/busy/ms 45.79, main/busy/ms 33.89, pipelines/driver_ms 20.81, stage/render/render 17.80, render/graph/ms 15.29, stage/main/update 14.22, stage/main/post_update 13.15, encode//sum 12.57, stage/render/prepare_assets 12.42 |
| warm-up | 5.18 | 29.0 | pipelines/compile_ms 232, pipelines/front_ms 226, render/thread/ms 50.25, render/busy/ms 48.53, main/busy/ms 38.73, stage/render/render 19.68, render/graph/ms 17.37, stage/main/update 16.93, stage/main/post_update 14.90, encode//sum 12.79, stage/render/graph_submit 12.14, encode/submit 8.42 |
| warm-up | 5.21 | 30.3 | pipelines/compile_ms 176, pipelines/front_ms 171, render/thread/ms 50.71, render/busy/ms 49.04, main/busy/ms 33.52, stage/render/render 17.02, stage/main/update 16.45, render/graph/ms 14.87, stage/render/prepare_assets 13.39, encode//sum 10.89, stage/main/post_update 10.84, stage/render/graph_submit 10.77 |
| warm-up | 5.24 | 36.7 | pipelines/compile_ms 176, pipelines/front_ms 171, render/thread/ms 59.41, render/busy/ms 57.74, main/busy/ms 39.86, stage/render/render 19.67, stage/main/post_update 17.80, render/graph/ms 17.54, stage/main/update 15.85, stage/render/prepare_assets 14.44, stage/render/graph_submit 11.92, encode//sum 11.03 |
| warm-up | 5.28 | 38.1 | pipelines/compile_ms 121, pipelines/front_ms 117, render/thread/ms 70.20, render/busy/ms 68.40, main/busy/ms 58.39, stage/main/update 26.13, stage/render/render 24.55, stage/main/post_update 20.26, render/graph/ms 18.81, stage/render/graph_submit 11.85, encode//sum 10.95, stage/render/prepare_assets 10.78 |
| warm-up | 5.34 | 61.3 | pipelines/compile_ms 918, pipelines/front_ms 894, render/thread/ms 79.50, render/busy/ms 77.92, main/busy/ms 49.31, stage/render/prepare_assets 34.98, stage/main/update 24.66, pipelines/driver_ms 23.98, stage/render/render 20.71, render/graph/ms 15.05, stage/main/post_update 13.05, stage/render/graph_submit 9.83 |
| warm-up | 5.38 | 33.3 | pipelines/compile_ms 1268, pipelines/front_ms 1229, render/thread/ms 74.99, render/busy/ms 73.44, main/busy/ms 43.01, pipelines/driver_ms 38.90, stage/render/prepare_assets 33.54, stage/main/update 26.15, stage/render/render 17.40, stream/drive/ms 15.55, render/graph/ms 15.24, stage/render/graph_submit 11.00 |
| warm-up | 5.41 | 31.5 | pipelines/compile_ms 1139, pipelines/front_ms 1105, render/thread/ms 57.45, render/busy/ms 55.92, main/busy/ms 41.45, pipelines/driver_ms 34.13, stage/main/update 24.79, stage/render/prepare_assets 18.23, stage/render/render 15.35, stream/drive/ms 14.42, render/graph/ms 13.27, encode//sum 10.82 |
| warm-up | 5.45 | 27.0 | pipelines/compile_ms 223, pipelines/front_ms 216, render/thread/ms 34.62, render/busy/ms 33.44, main/busy/ms 22.37, stage/render/prepare_assets 15.19, stage/render/render 12.25, stage/main/post_update 10.56, encode//sum 10.53, render/graph/ms 10.38, stage/main/update 7.35, pipelines/driver_ms 6.90 |
| warm-up | 5.52 | 27.3 | render/thread/ms 40.94, render/busy/ms 39.74, main/busy/ms 23.55, stage/render/prepare_assets 21.71, stage/render/render 11.56, encode//sum 11.17, stage/main/post_update 9.98, render/graph/ms 9.72, stage/main/update 8.90, stage/render/graph_submit 5.97, encode/pass:shadow_point_light/sum 4.07, stage/render/camera_driver 3.62 |
| warm-up | 5.61 | 30.0 | render/thread/ms 46.07, render/busy/ms 44.79, main/busy/ms 29.35, stage/render/prepare_assets 22.68, stage/render/render 15.98, render/graph/ms 14.20, stage/main/post_update 12.78, encode//sum 12.14, stage/main/update 11.58, stage/render/graph_submit 9.69, encode/submit 6.77, stream/drive/ms 5.97 |
| warm-up | 5.64 | 25.7 | render/thread/ms 44.72, render/busy/ms 43.44, stage/render/prepare_assets 26.14, main/busy/ms 23.65, stage/render/render 10.77, stage/main/update 10.49, render/graph/ms 8.98, encode//sum 8.53, stage/main/post_update 6.82, stage/render/graph_submit 6.21, stream/drive/ms 4.92, encode/submit 4.20 |
| warm-up | 5.78 | 27.4 | pipelines/compile_ms 247, pipelines/front_ms 231, main/busy/ms 30.89, render/thread/ms 26.45, render/busy/ms 25.24, stage/main/update 17.57, pipelines/driver_ms 15.53, stream/drive/ms 12.49, stage/render/render 12.45, encode//sum 10.89, render/graph/ms 10.55, stage/main/post_update 10.24 |
| warm-up | 6.03 | 26.9 | pipelines/compile_ms 137, pipelines/front_ms 134, main/busy/ms 30.22, render/thread/ms 22.43, render/busy/ms 21.45, stage/main/update 19.72, stream/drive/ms 12.83, stage/render/render 9.72, encode//sum 8.48, render/graph/ms 8.28, stage/main/post_update 7.26, stage/render/graph_submit 5.70 |
| warm-up | 6.10 | 28.8 | render/thread/ms 41.00, render/busy/ms 39.44, main/busy/ms 31.02, stage/render/render 16.50, stage/main/update 15.72, render/graph/ms 14.39, encode//sum 11.77, stage/main/post_update 11.40, stage/render/graph_submit 10.19, encode/submit 7.01, encode/pass:shadow_point_light/sum 7.00, stage/render/prepare_assets 6.12 |
| warm-up | 6.15 | 30.6 | render/thread/ms 37.50, render/busy/ms 36.41, main/busy/ms 23.99, stage/render/prepare_assets 13.18, stage/render/render 10.44, stage/main/update 9.59, render/graph/ms 8.97, encode//sum 8.65, stage/main/post_update 8.59, stage/render/graph_submit 6.03, encode/pass:shadow_point_light/sum 4.93, encode/submit 3.74 |
| warm-up | 6.39 | 31.3 | render/thread/ms 43.05, render/busy/ms 41.66, main/busy/ms 38.41, stage/render/render 18.64, stage/main/update 17.18, render/graph/ms 16.53, stage/main/post_update 15.56, encode//sum 13.70, stage/render/graph_submit 10.57, encode/pass:shadow_point_light/sum 9.37, stage/render/prepare_assets 7.52, encode/submit 6.94 |
| warm-up | 6.44 | 29.2 | pipelines/compile_ms 153, pipelines/front_ms 150, render/thread/ms 40.87, render/busy/ms 39.65, main/busy/ms 32.48, stage/render/prepare_assets 15.31, stage/main/update 14.97, stage/render/render 12.83, stage/main/post_update 11.75, render/graph/ms 11.25, encode//sum 8.34, stage/render/graph_submit 7.89 |
| warm-up | 6.52 | 32.6 | render/thread/ms 40.21, render/busy/ms 39.04, main/busy/ms 38.35, stage/main/update 21.81, stage/render/prepare_assets 16.28, stage/main/post_update 11.45, stage/render/render 11.20, encode//sum 9.50, render/graph/ms 9.28, encode/pass:shadow_point_light/sum 5.49, stage/render/graph_submit 5.28, stage/render/camera_driver 3.84 |
| warm-up | 6.68 | 42.8 | render/thread/ms 55.70, render/busy/ms 54.45, main/busy/ms 48.11, pipelines/compile_ms 31.92, stage/main/update 31.33, stage/render/prepare_assets 30.84, pipelines/front_ms 30.06, stream/drive/ms 21.96, stage/render/render 14.37, render/graph/ms 12.42, encode//sum 12.39, stage/main/post_update 12.33 |
| warm-up | 6.75 | 27.6 | render/thread/ms 46.14, render/busy/ms 45.09, main/busy/ms 31.61, stage/render/prepare_assets 25.83, stage/main/post_update 12.55, stage/main/update 11.34, stage/render/render 10.80, encode//sum 9.99, render/graph/ms 9.17, stage/render/graph_submit 6.01, encode/pass:shadow_point_light/sum 5.09, encode/submit 3.57 |
| warm-up | 6.96 | 28.1 | main/busy/ms 40.81, render/thread/ms 28.61, render/busy/ms 27.51, stage/main/update 24.46, stream/drive/ms 13.27, stage/render/render 12.72, stage/main/post_update 11.20, render/graph/ms 11.14, encode//sum 9.89, encode/pass:shadow_point_light/sum 7.67, stage/render/graph_submit 7.63, stage/render/prepare_assets 7.05 |
| measure | 0.77 | 27.1 | main/busy/ms 33.40, render/thread/ms 28.81, render/busy/ms 27.58, stage/main/update 17.41, stage/render/render 15.33, render/graph/ms 13.75, stage/main/post_update 12.83, stream/drive/ms 12.68, encode//sum 12.16, stage/render/graph_submit 9.54, encode/pass:shadow_point_light/sum 8.98, encode/submit 6.36 |
| measure | 18.17 | 27.5 | main/busy/ms 33.93, stage/main/update 21.20, render/thread/ms 17.67, stream/drive/ms 17.09, render/busy/ms 16.76, stage/main/post_update 10.29, stage/render/render 9.69, render/graph/ms 8.44, encode//sum 6.62, stage/render/graph_submit 6.20, encode/submit 4.14, stage/render/camera_driver 2.13 |
| measure | 23.05 | 27.1 | render/thread/ms 36.53, render/busy/ms 35.45, main/busy/ms 32.20, stage/main/post_update 15.69, encode//sum 14.73, stage/render/render 14.08, stage/main/update 12.45, render/graph/ms 12.44, encode/pass:shadow_point_light/sum 9.31, stage/render/prepare_assets 9.23, stage/render/graph_submit 8.42, encode/submit 4.61 |
| measure | 59.01 | 26.9 | render/thread/ms 33.83, render/busy/ms 32.41, main/busy/ms 31.87, stage/main/post_update 18.95, stage/render/render 16.85, render/graph/ms 14.86, encode//sum 11.97, stage/render/graph_submit 10.59, stage/main/update 8.17, encode/submit 7.00, stage/render/prepare_assets 5.70, stage/render/camera_driver 4.10 |
| measure | 59.03 | 27.2 | render/thread/ms 37.10, render/busy/ms 36.07, main/busy/ms 27.05, stage/render/prepare_assets 14.96, stage/main/post_update 14.66, stage/render/render 11.39, render/graph/ms 10.00, encode//sum 8.62, stage/main/update 6.81, stage/render/graph_submit 6.72, encode/submit 4.09, stage/render/camera_driver 3.12 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.face_cull=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=facecull-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 5%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **73%** (min 47%) · 2625 MHz (min 2625) · memory 8101 MHz · 77.6 W · 77 °C (max 82) · VRAM 7006 MiB · throttled 100% of samples

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
- `shadow.lamp.face_cull=on (command line: --set)`
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
