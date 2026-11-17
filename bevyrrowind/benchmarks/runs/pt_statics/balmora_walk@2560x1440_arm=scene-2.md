# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6157 frames over 65.6 s · **median 10.51 ms (95 fps)** · mean 10.66 · p95 13.17 · p99 16.25 (1% low 62 fps) · worst 62.75 ms · 68 spikes over 1.5× median · 57.6 stutters/min (work 53.9/min)
- warm-up: 353 frames over 9.4 s, worst 2576.1 ms
- hitches (> 26.3 ms): **6** measured, 43 in warm-up
- timed GPU passes: 6.45 ms of a 10.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.51 ms, limit 11 ms
- **FAIL** frame_ms: 63 stutters (> 1.5x moving median of 60) in 1.09 min = 57.58/min, limit 1/min
- **FAIL** worst frame 62.75 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.792 | 2.036 | 2.071 | 0.88 | 0.171 |
| main_opaque_pass_3d | 1.398 | 1.589 | 1.957 | 0.88 | 0.070 |
| early prepass | 0.459 | 0.521 | 0.744 | 0.88 | 0.090 |
| bloom | 0.278 | 0.315 | 0.321 | 0.88 | 0.062 |
| atmosphere_luts | 0.267 | 0.303 | 0.367 | 0.88 | 0.003 |
| water_velocity | 0.247 | 0.280 | 0.347 | 0.88 | 0.001 |
| water_sspr | 0.178 | 0.202 | 0.203 | 0.88 | 0.002 |
| ssao | 0.166 | 0.188 | 0.193 | 0.88 | 0.007 |
| auxiliary_views | 0.145 | 0.165 | 0.587 | 0.88 | 0.164 |
| atmos_froxel_inject | 0.129 | 0.146 | 0.148 | 0.88 | 0.001 |
| clouds_trace | 0.124 | 0.141 | 0.232 | 0.88 | 0.001 |
| water_fft | 0.116 | 0.132 | 0.139 | 0.88 | 0.003 |
| uniform_allocation | 0.107 | 0.121 | 0.170 | 0.88 | 0.023 |
| upscaling | 0.104 | 0.118 | 0.126 | 0.88 | 0.004 |
| clouds_composite | 0.093 | 0.106 | 0.109 | 0.88 | 0.002 |
| bin_unpacking | 0.085 | 0.097 | 0.134 | 0.88 | 0.018 |
| gpu_instance_materialization | 0.083 | 0.094 | 0.099 | 0.88 | 0.004 |
| main_transmissive_pass_3d | 0.075 | 0.085 | 0.178 | 0.88 | 0.009 |
| tonemapping | 0.075 | 0.085 | 0.088 | 0.88 | 0.003 |
| clustering | 0.056 | 0.064 | 0.067 | 0.88 | 0.018 |
| early_mesh_preprocessing | 0.050 | 0.057 | 0.091 | 0.88 | 0.004 |
| render_sky | 0.045 | 0.052 | 0.053 | 0.88 | 0.002 |
| clouds_resolve | 0.043 | 0.049 | 0.049 | 0.88 | 0.001 |
| surface_exposure | 0.035 | 0.040 | 0.040 | 0.88 | 0.001 |
| surface_trail | 0.034 | 0.056 | 0.057 | 0.61 | 0.002 |
| lightprobe_irradiance_map | 0.034 | 0.413 | 0.417 | 0.08 | 0.001 |
| surface_exposure_far | 0.032 | 0.040 | 0.042 | 0.79 | 0.002 |
| clouds_shadow | 0.027 | 0.031 | 0.034 | 0.88 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.88 | 0.001 |
| water_snapshot | 0.021 | 0.024 | 0.027 | 0.88 | 0.002 |
| range_unpacking | 0.021 | 0.024 | 0.033 | 0.88 | 0.003 |
| water_mips | 0.019 | 0.022 | 0.023 | 0.88 | 0.002 |
| lightprobe_radiance_map | 0.018 | 0.221 | 0.233 | 0.08 | 0.004 |
| per_view_shadow_pass | 0.016 | 0.018 | 0.037 | 0.88 | 0.006 |
| night_sky | 0.012 | 0.014 | 0.034 | 0.88 | 0.002 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.88 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.88 | 0.003 |
| main_transparent_pass_3d | 0.008 | 0.009 | 0.022 | 0.88 | 0.085 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.88 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.08 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.08 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.08 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.349 | 11.667 | 61.79 | 57561.8 |
| render/busy/ms | 8.795 | 11.162 | 61.31 | 54153.7 |
| main/busy/ms | 8.733 | 10.406 | 62.24 | 53770.3 |
| stage/render/render | 5.492 | 7.024 | 58.36 | 33817.1 |
| stage/main/post_update | 5.142 | 6.462 | 58.18 | 31658.0 |
| render/graph/ms | 4.761 | 6.296 | 57.74 | 29312.4 |
| encode//sum | 4.086 | 5.501 | 57.22 | 25158.3 |
| stage/render/graph_submit | 3.565 | 4.834 | 56.64 | 21947.2 |
| encode/submit | 2.267 | 3.318 | 5.80 | 13959.3 |
| stage/main/update | 2.192 | 2.625 | 24.79 | 13496.1 |
| encode/finish_wall | 1.387 | 1.974 | 53.86 | 8536.9 |
| stage/render/camera_driver | 1.136 | 1.536 | 4.14 | 6993.9 |
| encode/pass:shadow_point_light/sum | 0.922 | 3.348 | 16.70 | 5677.9 |
| encode/pass:shadow_directional_light/sum | 0.864 | 1.576 | 2.76 | 5317.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.783 | 1.104 | 2.56 | 4822.2 |
| encode/pass:early prepass/sum | 0.764 | 1.094 | 53.27 | 4705.4 |
| stage/render/prepare_write_phase_buffers | 0.666 | 0.982 | 1.72 | 4098.1 |
| bigspace/total/ms | 0.543 | 1.333 | 2.33 | 3341.3 |
| stage/render/prepare_assets | 0.537 | 0.692 | 10.96 | 3303.6 |
| render/present/ms | 0.458 | 1.228 | 5.12 | 2822.9 |
| stage/render/prepare_resources | 0.440 | 0.521 | 0.88 | 2706.2 |
| stage/render/prepare_meshes | 0.436 | 0.552 | 4.42 | 2685.6 |
| stage/main/first | 0.399 | 0.522 | 5.77 | 2454.1 |
| stage/main/pre_update | 0.355 | 0.504 | 2.00 | 2186.7 |
| stage/render/prepare_bind_groups | 0.328 | 0.556 | 2.09 | 2018.8 |
| terrain/update/ms | 0.309 | 0.400 | 2.57 | 1901.3 |
| stage/render/prepare_flush | 0.291 | 0.513 | 1.40 | 1789.9 |
| terrain/select/ms | 0.275 | 0.359 | 0.92 | 1692.2 |
| stage/render/specialize | 0.265 | 0.371 | 6.73 | 1629.9 |
| controller/feed/ms | 0.261 | 0.333 | 0.75 | 1609.0 |
| stage/main/last | 0.252 | 0.300 | 0.86 | 1551.2 |
| stream/drive/ms | 0.251 | 0.326 | 10.83 | 1544.1 |
| bigspace/low_precision/ms | 0.234 | 0.834 | 2.07 | 1440.2 |
| stage/render/prepare_batch_phases | 0.227 | 0.338 | 0.59 | 1397.0 |
| stage/main/fixed_main_loop | 0.212 | 0.369 | 1.17 | 1304.2 |
| far/select/ms | 0.198 | 0.266 | 0.56 | 1220.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.198 | 0.303 | 0.99 | 1217.1 |
| stream/tick/ms | 0.197 | 0.265 | 0.56 | 1209.9 |
| stage/render/queue | 0.189 | 0.307 | 4.30 | 1164.8 |
| stage/render/create_views | 0.185 | 0.238 | 7.45 | 1137.5 |
| stage/render/extract_commands | 0.177 | 0.211 | 0.42 | 1090.5 |
| anim/evaluate/ms | 0.172 | 0.272 | 53.43 | 1060.9 |
| bigspace/high_precision/ms | 0.165 | 0.743 | 1.75 | 1014.7 |
| anim/evaluate_cpu/ms | 0.156 | 0.196 | 0.78 | 960.5 |
| stage/render/prepare_views | 0.142 | 0.177 | 0.41 | 871.7 |
| controller/move/ms | 0.123 | 0.236 | 0.49 | 760.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.098 | 0.162 | 0.76 | 602.6 |
| render/acquire/ms | 0.095 | 0.123 | 0.32 | 585.1 |
| stage/render/phase_sort | 0.078 | 0.107 | 0.31 | 479.8 |
| pipelines/compile_ms | 0.060 | 0.000 | 127.52 | 371.0 |
| pipelines/front_ms | 0.058 | 0.000 | 123.35 | 356.3 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.074 | 0.29 | 285.4 |
| physics/step/ms | 0.046 | 0.086 | 0.63 | 283.3 |
| physics/insert/ms | 0.032 | 0.356 | 0.81 | 199.6 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.62 | 187.0 |
| weather/map/ms | 0.025 | 0.033 | 0.21 | 153.7 |
| stage/render/cleanup | 0.024 | 0.035 | 0.74 | 145.6 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.48 | 79.8 |
| anim/apply/ms | 0.012 | 0.022 | 0.23 | 73.3 |
| controller/avoid/ms | 0.011 | 0.014 | 0.19 | 65.3 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.49 | 63.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.14 | 62.9 |
| script/update/ms | 0.007 | 0.009 | 0.05 | 43.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.20 | 32.6 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 31.7 |
| script/ui/ms | 0.004 | 0.006 | 0.23 | 26.8 |
| controller/write/ms | 0.004 | 0.005 | 0.02 | 21.7 |
| stream/upload/ms | 0.004 | 0.000 | 1.22 | 21.6 |
| stream/spawn/ms | 0.003 | 0.000 | 1.15 | 19.9 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.16 | 14.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.11 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1283 | 2262 | 661 |
| crowd/c0/meshes/count | count | 353 | 646 | 173 |
| crowd/c1/count | count | 145 | 157 | 147 |
| crowd/c1/joints/count | count | 5510 | 6003 | 5535 |
| crowd/c1/meshes/count | count | 1164 | 1299 | 1168 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4605 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4585 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.7 k | 56.8 k | 47.0 k |
| encode//max | ms | 0.41 | 53.81 | 0.53 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.48 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.49 | 0.01 |
| encode/finish_critical | ms | 0.98 | 53.81 | 1.05 |
| encode/pass:early prepass/max | ms | 0.77 | 53.27 | 1.05 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 2.56 | 0.69 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.76 | 0.15 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.19 | 0.99 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.76 | 0.96 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.65 | 0.83 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.62 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.02 | 62.24 | 8.29 |
| gpu/timed/ms | ms | 8.05 | 9.13 | 8.29 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 299.7 k | 299.7 k | 299.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.9 k | 13.1 k | 11.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 158 | 387 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 337 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 158 | 387 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 337 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d/bins | count | 761 | 971 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2523 | 3555 | 2454 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 761 | 971 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2523 | 3555 | 2454 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 155 | 8 |
| phases/shadow/bins | count | 1945 | 3148 | 1081 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 6580 | 20.3 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 5913 | 12.5 k | 4381 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 164 | 222 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 195 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 327 | 328 | 328 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 17 | 468 | 11 |
| preprocess/bind_groups_created | count | 12 | 463 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 125 | 218 | 119 |
| process/cpu_usage | % | 25.59 | 36.36 | 21.23 |
| process/mem_usage | GiB | 5.41 | 5.58 | 5.58 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2976 | 3166 | 3166 |
| scene/batch/masked/live | count | 2976 | 3166 | 3166 |
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
| scene/batch/opaque/high_water | count | 18.8 k | 19.9 k | 19.9 k |
| scene/batch/opaque/live | count | 18.8 k | 19.9 k | 19.9 k |
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
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1591 | 1621 | 1621 |
| scene/chains/live | count | 1591 | 1621 | 1621 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_bump | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/scene_live | count | 21.7 k | 23.0 k | 23.0 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 129 | 9 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 367.9 k | 807.7 k | 489.6 k |
| skin/joints_written/count | count | 5748 | 12.6 k | 7650 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 6451 | 6575 | 6575 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.10 | 40.44 | 32.97 |
| system/mem_usage | % | 66.99 | 67.58 | 67.56 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2895 | 3010 | 3010 |
| textures/images | count | 2668 | 2726 | 2726 |
| views/all/batch_sets | count | 78 | 212 | 64 |
| views/all/bins | count | 3896 | 5207 | 3363 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 19.7 k | 33.8 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 11.8 k | 19.3 k | 10.3 k |
| views/all/pipelines | count | 210 | 268 | 75 |
| views/all/pipelines_distinct | count | 70 | 74 | 59 |
| views/all/work_items | count | 154.0 k | 263.7 k | 153.8 k |
| views/cascade/batch_sets | count | 13 | 60 | 8 |
| views/cascade/bins | count | 1521 | 2494 | 1081 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 6580 | 20.3 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 5229 | 11.5 k | 4381 |
| views/cascade/pipelines | count | 38 | 55 | 31 |
| views/cascade/work_items | count | 49.4 k | 158.4 k | 49.3 k |
| views/lamp_face/batch_sets | count | 0 | 145 | 0 |
| views/lamp_face/bins | count | 401 | 751 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 629 | 1064 | 0 |
| views/lamp_face/pipelines | count | 126 | 174 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 715 | 0 |
| views/main/batch_sets | count | 57 | 61 | 56 |
| views/main/bins | count | 2042 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5952 | 7532 | 5940 |
| views/main/pipelines | count | 46 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 105.9 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.6 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 61.43 | 272 | 378.2 k |
| anim/events/count | count | 0.09 | 3 | 580 |
| assets/Image/added | count | 0.10 | 39 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.47 | 256 | 2874 |
| assets/Mesh/removed | count | 0.79 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.52 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 9 |
| ecs/entities_delta | count | 0.24 | 1299 | 1489 |
| encode//count | count | 34.90 | 220 | 214.9 k |
| encode/bind_walk/init_textures | count | 12.8 k | 113.9 k | 78.52 M |
| encode/bind_walk/init_textures_skipped | count | 76.6 k | 163.7 k | 471.45 M |
| encode/bind_walk/inits | count | 454 | 981 | 2.80 M |
| encode/bind_walk/inits_skipped | count | 102 | 165 | 625.8 k |
| encode/bind_walk/merge_textures | count | 12.7 k | 113.8 k | 78.07 M |
| encode/bind_walk/merge_textures_skipped | count | 5237 | 5983 | 32.24 M |
| encode/bind_walk/merges | count | 96.92 | 328 | 596.7 k |
| encode/bind_walk/merges_skipped | count | 72.63 | 134 | 447.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6157 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6157 |
| encode/pass:early prepass/count | count | 1 | 1 | 6157 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6157 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6157 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6157 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6157 |
| encode/pass:shadow_point_light/count | count | 1.49 | 48 | 9177 |
| encode/queue/command_buffers | count | 46.41 | 278 | 285.7 k |
| encode/queue/submits | count | 2.01 | 4 | 12.4 k |
| encode/queue/write_buffer/bytes | bytes | 2.71 M | 5.16 M | 16.67 G |
| encode/queue/write_buffer/calls | count | 270 | 704 | 1.66 M |
| encode/queue/write_buffer_with/bytes | bytes | 172.2 k | 1.30 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 10.39 | 777 | 63.9 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.44 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6157 |
| physics/inserts | count | 0.36 | 104 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 161 | 2335 |
| physics/steps | count | 0.64 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 6 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 6 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5498 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 30 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1205 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.3 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2576.1 | main/busy/ms 2545, stage/main/post_update 2439, stage/main/update 96.67, surface/advance_cpu_ms 68.02, stage/main/pre_update 5.38, stage/main/last 1.19, surface/cell_upload_ms 0.75, stage/main/first 0.60, stream/drive/ms 0.55, stage/main/fixed_main_loop 0.39, weather/map/ms 0.13, stream/tick/ms 0.12 |
| warm-up | 3.42 | 844.2 | main/busy/ms 2731, stage/main/post_update 2468, render/thread/ms 811, render/busy/ms 632, stage/render/prepare_assets 581, stage/main/update 248, stage/render/prepare_views 174, render/acquire/ms 174, surface/advance_cpu_ms 68.02, stage/render/create_views 29.18, stage/render/render 16.36, render/graph/ms 9.15 |
| warm-up | 3.53 | 112.2 | render/thread/ms 906, render/busy/ms 674, stage/render/prepare_assets 608, main/busy/ms 207, stage/render/prepare_views 175, render/acquire/ms 174, stage/main/update 156, pipelines/compile_ms 106, stage/render/render 74.51, pipelines/front_ms 62.04, render/present/ms 58.11, pipelines/driver_ms 43.58 |
| warm-up | 3.58 | 45.9 | pipelines/compile_ms 428, pipelines/front_ms 320, render/thread/ms 139, pipelines/driver_ms 107, stage/render/render 95.36, render/busy/ms 85.21, main/busy/ms 63.72, pipelines/wait_ms 60.55, render/present/ms 52.98, stage/main/post_update 47.03, render/graph/ms 41.19, stage/render/prepare_assets 28.27 |
| warm-up | 3.72 | 138.1 | pipelines/wait_ms 4448, pipelines/compile_ms 1807, pipelines/front_ms 1587, pipelines/driver_ms 221, render/thread/ms 181, render/busy/ms 179, main/busy/ms 99.71, stage/render/prepare_resources 77.76, stage/main/post_update 69.37, stage/render/render 60.57, render/graph/ms 57.60, stage/render/camera_driver 25.73 |
| warm-up | 3.75 | 31.1 | pipelines/wait_ms 4568, pipelines/compile_ms 1922, pipelines/front_ms 1742, pipelines/driver_ms 180, render/thread/ms 165, render/busy/ms 162, main/busy/ms 127, stage/render/prepare_resources 77.40, stage/main/update 56.41, stage/main/post_update 45.17, stage/render/render 31.39, stage/render/prepare_assets 31.16 |
| warm-up | 3.86 | 113.8 | pipelines/compile_ms 1073, pipelines/front_ms 1029, pipelines/wait_ms 256, render/thread/ms 97.97, render/busy/ms 94.93, main/busy/ms 84.29, stage/render/prepare_assets 68.21, stage/main/update 59.17, pipelines/driver_ms 43.58, stage/render/render 20.54, stage/main/post_update 18.56, render/graph/ms 17.41 |
| warm-up | 3.92 | 59.8 | pipelines/compile_ms 1278, pipelines/front_ms 1171, pipelines/wait_ms 157, render/thread/ms 127, render/busy/ms 125, pipelines/driver_ms 107, stage/render/prepare_assets 89.59, main/busy/ms 29.68, stage/render/render 28.62, render/graph/ms 25.95, encode//sum 17.93, stage/render/camera_driver 17.44 |
| warm-up | 3.95 | 32.7 | pipelines/compile_ms 1201, pipelines/front_ms 1081, pipelines/driver_ms 120, render/thread/ms 86.51, pipelines/wait_ms 86.21, render/busy/ms 84.52, stage/render/prepare_assets 38.21, main/busy/ms 34.14, stage/render/render 31.46, render/graph/ms 29.10, stage/render/graph_submit 18.58, encode//sum 18.53 |
| warm-up | 4.02 | 49.3 | pipelines/compile_ms 400, pipelines/front_ms 373, pipelines/wait_ms 110, main/busy/ms 67.29, render/thread/ms 52.14, render/busy/ms 49.94, stage/main/update 40.57, pipelines/driver_ms 26.67, stage/main/post_update 21.42, stage/render/render 17.24, render/graph/ms 14.30, stage/render/graph_submit 11.41 |
| warm-up | 4.10 | 84.4 | pipelines/compile_ms 985, pipelines/front_ms 933, pipelines/wait_ms 111, render/thread/ms 99.01, render/busy/ms 96.01, main/busy/ms 75.56, stage/main/update 58.58, pipelines/driver_ms 51.93, stage/render/prepare_assets 48.44, stage/render/render 24.15, render/graph/ms 20.40, stage/render/graph_submit 14.49 |
| warm-up | 4.14 | 42.7 | pipelines/compile_ms 1472, pipelines/front_ms 1415, render/thread/ms 81.93, render/busy/ms 79.77, pipelines/driver_ms 56.59, stage/render/prepare_assets 56.48, main/busy/ms 37.97, stage/main/update 26.27, pipelines/wait_ms 26.23, stage/render/render 16.70, render/graph/ms 13.87, stage/render/graph_submit 8.68 |
| warm-up | 4.19 | 44.2 | pipelines/compile_ms 1112, pipelines/front_ms 1075, render/thread/ms 51.60, render/busy/ms 50.07, pipelines/driver_ms 37.31, stage/render/prepare_assets 31.60, pipelines/wait_ms 25.01, main/busy/ms 20.71, stage/render/render 13.67, render/graph/ms 11.82, stage/main/post_update 9.36, stage/render/graph_submit 9.31 |
| warm-up | 4.26 | 59.4 | pipelines/compile_ms 357, pipelines/front_ms 328, render/thread/ms 62.03, stage/render/prepare_views 31.29, render/acquire/ms 31.15, render/busy/ms 28.75, pipelines/driver_ms 28.66, main/busy/ms 28.09, stage/render/render 20.80, render/graph/ms 18.23, encode//sum 16.44, stage/render/graph_submit 15.81 |
| warm-up | 4.33 | 47.2 | pipelines/compile_ms 359, pipelines/front_ms 346, main/busy/ms 50.37, render/thread/ms 46.90, render/busy/ms 44.99, stage/main/update 31.70, stream/drive/ms 21.66, stage/render/render 14.70, stage/main/post_update 13.48, pipelines/driver_ms 12.92, stage/render/prepare_assets 12.57, render/graph/ms 12.19 |
| warm-up | 4.39 | 57.0 | pipelines/compile_ms 786, pipelines/front_ms 755, render/thread/ms 72.39, render/busy/ms 69.88, main/busy/ms 53.15, stage/render/prepare_assets 33.49, pipelines/driver_ms 31.79, stage/main/update 26.68, stage/render/render 17.59, stage/main/post_update 17.51, render/graph/ms 14.40, stage/render/graph_submit 10.17 |
| warm-up | 4.46 | 69.9 | pipelines/compile_ms 1154, pipelines/front_ms 1110, main/busy/ms 167, stage/main/update 146, render/thread/ms 87.57, render/busy/ms 85.46, stage/render/prepare_assets 57.16, pipelines/driver_ms 43.20, stage/render/render 17.10, render/graph/ms 14.43, stage/main/post_update 10.95, pipelines/wait_ms 10.83 |
| warm-up | 4.65 | 195.5 | pipelines/compile_ms 2352, pipelines/front_ms 2226, main/busy/ms 153, stage/main/update 137, pipelines/driver_ms 126, render/thread/ms 123, render/busy/ms 122, stage/render/prepare_assets 103, pipelines/wait_ms 14.62, stage/render/render 12.08, pipelines/lock_ms 11.34, render/graph/ms 10.22 |
| warm-up | 4.69 | 38.9 | pipelines/compile_ms 1883, pipelines/front_ms 1765, pipelines/driver_ms 118, render/thread/ms 115, render/busy/ms 113, stage/render/prepare_assets 76.18, main/busy/ms 47.61, stage/render/render 28.88, stage/main/update 27.49, render/graph/ms 24.12, stage/render/graph_submit 20.64, encode/submit 15.22 |
| warm-up | 4.77 | 81.7 | render/thread/ms 110, render/busy/ms 108, pipelines/compile_ms 78.15, pipelines/front_ms 67.94, main/busy/ms 65.64, stage/render/render 33.64, stage/render/prepare_assets 32.65, render/graph/ms 28.59, stage/main/update 27.90, stage/main/post_update 26.56, stage/render/graph_submit 23.93, stage/render/prepare_write_phase_buffers 20.66 |
| warm-up | 4.88 | 103.4 | pipelines/compile_ms 2408, pipelines/front_ms 2338, pipelines/wait_ms 377, render/thread/ms 166, render/busy/ms 146, main/busy/ms 125, stage/main/post_update 98.66, pipelines/driver_ms 69.60, stage/render/prepare_assets 48.93, stage/render/prepare_write_phase_buffers 27.33, stage/main/update 19.91, stage/render/prepare_views 18.44 |
| warm-up | 4.94 | 61.5 | pipelines/compile_ms 2479, pipelines/front_ms 2407, pipelines/wait_ms 491, render/thread/ms 139, render/busy/ms 120, main/busy/ms 110, stage/main/post_update 82.77, pipelines/driver_ms 72.24, stage/render/prepare_assets 45.43, stage/render/render 21.71, render/graph/ms 19.27, stage/render/prepare_views 18.35 |
| warm-up | 5.02 | 83.6 | pipelines/compile_ms 536, pipelines/front_ms 519, pipelines/wait_ms 128, render/thread/ms 106, render/busy/ms 104, main/busy/ms 68.99, stage/render/prepare_assets 46.47, stage/main/update 46.14, stage/render/render 26.13, render/graph/ms 23.24, encode//sum 22.70, pipelines/driver_ms 16.55 |
| warm-up | 5.11 | 91.4 | pipelines/compile_ms 2018, pipelines/front_ms 1948, pipelines/wait_ms 564, render/thread/ms 132, render/busy/ms 129, main/busy/ms 86.36, pipelines/driver_ms 70.42, stage/render/prepare_assets 64.06, stage/main/update 54.31, encode//sum 32.47, stage/render/render 29.67, render/graph/ms 26.16 |
| warm-up | 5.16 | 48.9 | pipelines/compile_ms 2075, pipelines/front_ms 2006, pipelines/wait_ms 553, render/thread/ms 111, render/busy/ms 109, main/busy/ms 73.90, pipelines/driver_ms 69.35, stage/render/prepare_assets 48.94, stage/main/update 33.77, stage/render/render 27.65, render/graph/ms 24.58, encode//sum 22.92 |
| warm-up | 5.23 | 69.6 | pipelines/compile_ms 1042, pipelines/front_ms 1014, main/busy/ms 106, render/thread/ms 84.28, render/busy/ms 82.53, stage/main/update 74.32, stream/drive/ms 53.12, stage/render/prepare_assets 34.25, pipelines/driver_ms 27.70, pipelines/wait_ms 19.72, stage/main/post_update 17.84, stage/render/render 17.83 |
| warm-up | 5.29 | 56.2 | pipelines/compile_ms 731, pipelines/front_ms 709, main/busy/ms 99.81, stage/main/update 77.14, render/thread/ms 72.95, render/busy/ms 71.36, stream/drive/ms 61.96, stage/render/prepare_assets 31.99, pipelines/driver_ms 22.12, stage/render/render 20.41, render/graph/ms 18.03, pipelines/wait_ms 17.56 |
| warm-up | 5.32 | 35.6 | pipelines/compile_ms 271, pipelines/front_ms 261, render/thread/ms 53.90, main/busy/ms 52.51, render/busy/ms 52.46, stage/main/update 34.53, stage/render/render 17.64, stage/render/prepare_assets 16.82, render/graph/ms 15.38, stream/drive/ms 13.76, stage/render/graph_submit 12.14, stage/main/post_update 11.03 |
| warm-up | 5.36 | 33.5 | pipelines/compile_ms 662, pipelines/front_ms 638, render/thread/ms 53.91, render/busy/ms 52.39, main/busy/ms 45.61, stage/main/update 27.75, pipelines/driver_ms 23.42, stage/render/prepare_assets 17.55, stage/render/render 13.48, render/graph/ms 11.35, stage/main/post_update 10.69, encode//sum 8.34 |
| warm-up | 5.41 | 34.8 | pipelines/compile_ms 207, pipelines/front_ms 202, render/thread/ms 53.07, render/busy/ms 51.60, main/busy/ms 44.12, stage/main/update 25.58, stage/render/render 16.24, render/graph/ms 14.14, stage/main/post_update 12.53, stage/render/graph_submit 10.71, stage/render/prepare_write_phase_buffers 10.19, encode//sum 9.65 |
| warm-up | 5.45 | 35.5 | pipelines/compile_ms 195, pipelines/front_ms 192, render/thread/ms 58.19, render/busy/ms 56.84, main/busy/ms 44.94, stage/main/update 28.50, stage/render/prepare_assets 20.66, stage/render/render 14.25, render/graph/ms 12.19, stage/main/post_update 10.43, stage/render/prepare_write_phase_buffers 9.51, stage/render/graph_submit 8.75 |
| warm-up | 5.63 | 36.8 | pipelines/compile_ms 202, pipelines/front_ms 195, main/busy/ms 41.41, render/thread/ms 33.34, render/busy/ms 31.71, stage/main/update 22.43, stage/main/post_update 14.08, stream/drive/ms 11.83, stage/render/render 9.91, encode//sum 8.77, render/graph/ms 7.68, pipelines/driver_ms 6.90 |
| warm-up | 5.66 | 27.0 | pipelines/compile_ms 415, pipelines/front_ms 402, render/thread/ms 46.01, render/busy/ms 44.76, main/busy/ms 31.57, stage/render/prepare_assets 15.28, stage/main/update 13.49, pipelines/driver_ms 13.24, stage/main/post_update 12.91, stage/render/render 10.37, encode//sum 9.07, render/graph/ms 8.44 |
| warm-up | 5.86 | 34.7 | pipelines/compile_ms 179, pipelines/front_ms 174, main/busy/ms 40.38, render/thread/ms 31.00, render/busy/ms 29.59, stage/main/update 23.53, stream/drive/ms 17.37, stage/main/post_update 12.57, stage/render/render 9.63, encode//sum 7.84, render/graph/ms 7.82, stage/render/prepare_assets 6.87 |
| warm-up | 6.22 | 29.6 | pipelines/compile_ms 134, pipelines/front_ms 127, render/thread/ms 42.83, render/busy/ms 41.73, main/busy/ms 33.02, stage/render/prepare_assets 19.67, stage/main/update 15.37, stage/main/post_update 11.65, stage/render/render 10.67, encode//sum 8.99, render/graph/ms 8.87, pipelines/driver_ms 7.09 |
| warm-up | 6.33 | 26.9 | render/thread/ms 31.82, render/busy/ms 30.67, main/busy/ms 28.74, stage/main/update 18.16, stage/render/render 10.01, stage/render/prepare_assets 9.41, render/graph/ms 8.30, stage/main/post_update 7.15, encode//sum 7.11, stage/render/graph_submit 6.08, stream/drive/ms 3.74, encode/submit 3.65 |
| warm-up | 6.44 | 31.6 | pipelines/compile_ms 136, pipelines/front_ms 132, render/thread/ms 40.67, render/busy/ms 39.54, main/busy/ms 36.80, stage/main/update 18.65, stage/main/post_update 13.51, stage/render/render 12.75, encode//sum 12.50, render/graph/ms 10.96, stage/render/prepare_assets 8.34, stage/render/graph_submit 7.19 |
| warm-up | 6.50 | 31.8 | render/thread/ms 42.46, render/busy/ms 41.08, main/busy/ms 34.99, stage/main/update 16.29, stage/render/prepare_assets 14.11, stage/main/post_update 12.02, stage/render/render 9.83, render/graph/ms 8.20, encode//sum 6.96, stage/render/prepare_meshes 5.67, stage/render/graph_submit 5.65, stream/drive/ms 3.77 |
| warm-up | 6.53 | 37.9 | render/thread/ms 59.91, render/busy/ms 58.29, main/busy/ms 43.76, stage/main/update 19.88, stage/render/render 18.70, stage/render/prepare_assets 18.02, stage/main/post_update 16.75, render/graph/ms 16.37, encode//sum 9.42, stage/render/graph_submit 8.10, stage/render/camera_driver 8.05, stage/render/prepare_meshes 6.76 |
| warm-up | 6.58 | 29.8 | pipelines/compile_ms 225, pipelines/front_ms 217, render/thread/ms 36.25, render/busy/ms 35.30, main/busy/ms 27.39, stage/render/prepare_assets 15.59, stage/main/update 14.38, stage/main/post_update 9.24, pipelines/driver_ms 8.51, stage/render/render 7.99, render/graph/ms 6.68, encode//sum 6.41 |
| warm-up | 6.72 | 45.7 | main/busy/ms 54.42, stage/main/update 37.51, render/thread/ms 35.62, render/busy/ms 34.09, stream/drive/ms 14.18, stage/render/render 12.70, stage/main/post_update 12.32, render/graph/ms 10.44, stage/render/prepare_assets 9.73, encode//sum 9.17, stage/render/graph_submit 7.14, stage/render/prepare_meshes 5.11 |
| warm-up | 6.75 | 30.6 | pipelines/compile_ms 134, pipelines/front_ms 130, render/thread/ms 43.39, render/busy/ms 41.87, main/busy/ms 41.50, stage/main/update 24.03, stage/render/prepare_assets 18.14, stage/main/post_update 11.72, stage/render/render 11.54, render/graph/ms 9.28, encode//sum 8.17, stream/drive/ms 6.44 |
| warm-up | 6.81 | 26.9 | main/busy/ms 36.25, render/thread/ms 28.26, render/busy/ms 27.27, stage/main/update 18.49, stage/main/post_update 11.85, stage/render/render 10.87, render/graph/ms 9.10, encode//sum 6.82, stage/render/graph_submit 6.78, stage/render/prepare_assets 4.93, stream/drive/ms 4.92, encode/submit 4.58 |
| measure | 0.92 | 62.8 | render/thread/ms 71.67, render/busy/ms 70.75, main/busy/ms 70.62, stage/render/render 64.16, render/graph/ms 62.82, stage/main/post_update 62.82, encode//sum 61.74, stage/render/graph_submit 60.70, encode/finish_wall 55.73, encode/pass:early prepass/sum 54.32, anim/evaluate/ms 53.54, encode/submit 4.99 |
| measure | 8.13 | 31.2 | render/thread/ms 37.47, render/busy/ms 36.28, encode//sum 27.27, stage/render/render 20.05, render/graph/ms 18.29, encode/pass:shadow_point_light/sum 17.26, main/busy/ms 16.23, stage/render/graph_submit 12.93, stage/main/post_update 9.80, stage/render/create_views 7.61, encode/submit 7.42, encode/finish_wall 5.52 |
| measure | 18.13 | 34.5 | main/busy/ms 40.34, stage/main/update 26.86, render/thread/ms 22.71, render/busy/ms 21.69, stage/render/render 11.36, stage/main/post_update 10.15, render/graph/ms 9.81, encode//sum 7.68, stage/render/graph_submit 7.30, encode/submit 4.46, stage/render/prepare_assets 4.30, encode/finish_wall 2.91 |
| measure | 22.98 | 26.3 | render/thread/ms 29.73, render/busy/ms 28.83, main/busy/ms 23.70, stage/main/post_update 11.47, stage/render/render 10.04, render/graph/ms 8.74, encode//sum 8.63, stage/render/specialize 7.00, stage/main/update 6.75, stage/render/graph_submit 6.72, stage/render/queue 4.50, encode/submit 3.85 |
| measure | 40.81 | 26.9 | render/thread/ms 31.24, render/busy/ms 30.26, main/busy/ms 17.60, stage/render/render 11.57, render/graph/ms 10.19, stage/main/post_update 9.29, encode//sum 8.48, stage/render/graph_submit 8.10, stage/main/update 5.05, stage/render/prepare_meshes 4.37, stage/render/specialize 4.19, encode/submit 4.16 |
| measure | 59.02 | 30.7 | render/thread/ms 37.08, render/busy/ms 36.06, main/busy/ms 25.10, stage/render/prepare_assets 16.55, stage/main/post_update 13.56, stage/render/render 10.73, render/graph/ms 9.51, encode//sum 8.02, stage/render/graph_submit 7.18, stage/main/update 6.22, encode/submit 4.11, encode/finish_wall 3.11 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **80%** (min 58%) · 2619 MHz (min 2610) · memory 8101 MHz · 81.9 W · 78 °C (max 83) · VRAM 7009 MiB · throttled 100% of samples

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
