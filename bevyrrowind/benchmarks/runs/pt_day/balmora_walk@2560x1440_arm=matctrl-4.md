# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); mw.matctrl.gate=on (command line: --set); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6133 frames over 65.6 s · **median 10.54 ms (95 fps)** · mean 10.70 · p95 13.33 · p99 16.41 (1% low 61 fps) · worst 45.45 ms · 73 spikes over 1.5× median · 57.6 stutters/min (work 48.4/min)
- warm-up: 324 frames over 9.1 s, worst 2603.2 ms
- hitches (> 26.4 ms): **33** measured, 47 in warm-up
- timed GPU passes: 6.31 ms of a 10.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.54 ms, limit 11 ms
- **FAIL** frame_ms: 63 stutters (> 1.5x moving median of 60) in 1.09 min = 57.58/min, limit 1/min
- **FAIL** worst frame 45.45 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.782 | 2.033 | 2.064 | 0.88 | 0.171 |
| main_opaque_pass_3d | 1.424 | 1.624 | 2.046 | 0.88 | 0.058 |
| early prepass | 0.429 | 0.490 | 0.715 | 0.88 | 0.084 |
| bloom | 0.275 | 0.314 | 0.319 | 0.88 | 0.062 |
| atmosphere_luts | 0.266 | 0.304 | 0.366 | 0.88 | 0.003 |
| water_velocity | 0.246 | 0.280 | 0.347 | 0.88 | 0.002 |
| water_sspr | 0.177 | 0.201 | 0.203 | 0.88 | 0.002 |
| ssao | 0.165 | 0.188 | 0.191 | 0.88 | 0.006 |
| auxiliary_views | 0.165 | 0.188 | 0.608 | 0.88 | 0.175 |
| atmos_froxel_inject | 0.127 | 0.145 | 0.147 | 0.88 | 0.001 |
| clouds_trace | 0.123 | 0.140 | 0.232 | 0.88 | 0.001 |
| water_fft | 0.115 | 0.131 | 0.138 | 0.88 | 0.002 |
| uniform_allocation | 0.113 | 0.129 | 0.181 | 0.88 | 0.023 |
| upscaling | 0.104 | 0.119 | 0.126 | 0.88 | 0.004 |
| clouds_composite | 0.093 | 0.106 | 0.108 | 0.88 | 0.002 |
| bin_unpacking | 0.090 | 0.103 | 0.140 | 0.88 | 0.017 |
| tonemapping | 0.074 | 0.085 | 0.088 | 0.88 | 0.003 |
| main_transmissive_pass_3d | 0.074 | 0.085 | 0.179 | 0.88 | 0.008 |
| clustering | 0.056 | 0.064 | 0.067 | 0.88 | 0.018 |
| render_sky | 0.045 | 0.052 | 0.052 | 0.88 | 0.002 |
| clouds_resolve | 0.043 | 0.048 | 0.049 | 0.88 | 0.001 |
| surface_exposure | 0.039 | 0.044 | 0.045 | 0.88 | 0.001 |
| surface_trail | 0.034 | 0.056 | 0.057 | 0.61 | 0.002 |
| surface_exposure_far | 0.033 | 0.043 | 0.045 | 0.78 | 0.002 |
| clouds_shadow | 0.027 | 0.031 | 0.034 | 0.88 | 0.003 |
| early_mesh_preprocessing | 0.026 | 0.030 | 0.046 | 0.88 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.030 | 0.032 | 0.88 | 0.001 |
| lightprobe_irradiance_map | 0.025 | 0.410 | 0.415 | 0.06 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.88 | 0.002 |
| water_mips | 0.019 | 0.022 | 0.024 | 0.88 | 0.002 |
| lightprobe_radiance_map | 0.013 | 0.219 | 0.231 | 0.06 | 0.004 |
| night_sky | 0.012 | 0.014 | 0.034 | 0.88 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.88 | 0.124 |
| clouds_fog_light | 0.010 | 0.012 | 0.012 | 0.88 | 0.002 |
| per_view_shadow_pass | 0.009 | 0.011 | 0.022 | 0.88 | 0.004 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.88 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.87 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.06 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.06 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.06 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.093 | 11.203 | 39.42 | 55770.1 |
| main/busy/ms | 8.940 | 10.594 | 28.44 | 54828.3 |
| render/busy/ms | 8.528 | 10.697 | 38.75 | 52299.3 |
| stage/main/post_update | 5.569 | 6.856 | 10.34 | 34152.8 |
| stage/render/render | 5.450 | 7.013 | 16.35 | 33423.7 |
| render/graph/ms | 4.694 | 6.265 | 15.19 | 28785.5 |
| encode//sum | 4.083 | 5.390 | 30.78 | 25042.6 |
| stage/render/graph_submit | 3.517 | 4.792 | 11.09 | 21572.4 |
| encode/submit | 2.275 | 3.292 | 7.62 | 13949.5 |
| stage/main/update | 2.127 | 2.535 | 21.79 | 13044.1 |
| encode/finish_wall | 1.334 | 1.947 | 5.27 | 8178.7 |
| stage/render/camera_driver | 1.124 | 1.550 | 4.03 | 6892.7 |
| encode/pass:shadow_point_light/sum | 1.000 | 3.198 | 15.40 | 6132.8 |
| encode/pass:shadow_directional_light/sum | 0.811 | 1.522 | 4.11 | 4976.4 |
| stage/render/prepare_write_phase_buffers | 0.754 | 1.081 | 2.63 | 4625.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.721 | 1.041 | 2.06 | 4419.5 |
| encode/pass:early prepass/sum | 0.693 | 1.036 | 2.08 | 4251.1 |
| bigspace/total/ms | 0.578 | 1.377 | 2.44 | 3545.1 |
| stage/render/prepare_assets | 0.513 | 0.664 | 11.98 | 3145.1 |
| render/present/ms | 0.473 | 1.219 | 3.18 | 2901.9 |
| stage/render/prepare_resources | 0.451 | 0.540 | 1.33 | 2766.0 |
| stage/render/prepare_meshes | 0.412 | 0.511 | 6.93 | 2526.9 |
| stage/main/first | 0.394 | 0.516 | 6.26 | 2418.3 |
| stage/main/pre_update | 0.354 | 0.540 | 1.98 | 2169.8 |
| terrain/update/ms | 0.309 | 0.403 | 1.91 | 1897.3 |
| terrain/select/ms | 0.277 | 0.362 | 0.99 | 1696.6 |
| stage/render/prepare_bind_groups | 0.271 | 0.477 | 2.32 | 1664.5 |
| controller/feed/ms | 0.261 | 0.328 | 0.82 | 1599.5 |
| stream/drive/ms | 0.257 | 0.337 | 19.99 | 1577.8 |
| stage/render/specialize | 0.248 | 0.322 | 11.96 | 1523.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.236 | 0.353 | 0.95 | 1450.2 |
| bigspace/low_precision/ms | 0.231 | 0.936 | 1.82 | 1416.6 |
| stage/render/queue | 0.215 | 0.297 | 7.27 | 1320.1 |
| stage/main/fixed_main_loop | 0.207 | 0.360 | 1.10 | 1271.0 |
| stream/tick/ms | 0.199 | 0.270 | 0.71 | 1221.6 |
| far/select/ms | 0.197 | 0.264 | 0.65 | 1205.8 |
| stage/render/create_views | 0.194 | 0.248 | 0.93 | 1187.1 |
| stage/render/prepare_batch_phases | 0.186 | 0.262 | 0.44 | 1141.1 |
| bigspace/high_precision/ms | 0.181 | 0.924 | 1.75 | 1109.1 |
| anim/evaluate/ms | 0.173 | 0.367 | 2.30 | 1062.7 |
| stage/render/extract_commands | 0.167 | 0.196 | 0.50 | 1023.9 |
| stage/render/prepare_flush | 0.163 | 0.325 | 1.48 | 998.0 |
| anim/evaluate_cpu/ms | 0.152 | 0.196 | 0.71 | 933.3 |
| stage/render/prepare_views | 0.136 | 0.172 | 0.42 | 835.3 |
| controller/move/ms | 0.132 | 0.273 | 0.45 | 807.2 |
| stage/main/last | 0.116 | 0.164 | 2.62 | 714.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.097 | 0.163 | 0.67 | 595.4 |
| render/acquire/ms | 0.093 | 0.121 | 0.37 | 568.9 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.124 | 0.45 | 362.5 |
| pipelines/compile_ms | 0.047 | 0.000 | 121.77 | 290.5 |
| pipelines/front_ms | 0.045 | 0.000 | 117.88 | 278.9 |
| physics/step/ms | 0.044 | 0.082 | 0.60 | 271.9 |
| stage/render/phase_sort | 0.033 | 0.059 | 0.27 | 202.5 |
| stage/render/cleanup | 0.033 | 0.099 | 0.83 | 200.0 |
| encode/resolve_diagnostics/sum | 0.030 | 0.039 | 0.52 | 182.7 |
| physics/insert/ms | 0.030 | 0.345 | 0.83 | 181.6 |
| weather/map/ms | 0.025 | 0.032 | 0.23 | 151.6 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.11 | 86.0 |
| anim/apply/ms | 0.012 | 0.023 | 0.47 | 73.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.28 | 64.8 |
| controller/avoid/ms | 0.010 | 0.013 | 0.24 | 62.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.26 | 62.1 |
| script/update/ms | 0.007 | 0.010 | 0.22 | 44.0 |
| stream/despawn/ms | 0.006 | 0.000 | 1.14 | 36.3 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 31.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 31.0 |
| script/ui/ms | 0.005 | 0.006 | 0.17 | 28.8 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 20.7 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 13.4 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.91 | 11.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.10 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 42 | 12 |
| crowd/c0/joints/count | count | 1301 | 2204 | 563 |
| crowd/c0/meshes/count | count | 353 | 634 | 144 |
| crowd/c1/count | count | 145 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5982 | 5633 |
| crowd/c1/meshes/count | count | 1183 | 1284 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4596 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 1.98 | 0.31 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.11 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.01 |
| encode/finish_critical | ms | 0.94 | 4.11 | 0.91 |
| encode/pass:early prepass/max | ms | 0.71 | 2.08 | 0.71 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 2.06 | 0.44 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.67 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.95 | 0.24 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 4.11 | 0.91 |
| encode/pass:shadow_point_light/max | ms | 0.77 | 1.32 | 0.68 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.52 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.18 | 38.75 | 8.31 |
| gpu/timed/ms | ms | 7.95 | 9.34 | 8.31 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 811 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 811 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1289 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3402 | 5140 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/bins | count | 1289 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3402 | 5140 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 203 | 4 |
| phases/shadow/bins | count | 5030 | 9032 | 2413 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.5 k | 6364 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 251 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 563 | 9 |
| preprocess/bind_groups_created | count | 13 | 563 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 219 | 120 |
| process/cpu_usage | % | 27.40 | 41.15 | 17.79 |
| process/mem_usage | GiB | 5.53 | 5.70 | 5.70 |
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
| skin/blocks_changed/count | count | 13 | 127 | 6 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 368.1 k | 809.5 k | 491.5 k |
| skin/joints_written/count | count | 5752 | 12.6 k | 7680 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5011 | 5011 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.80 | 46.67 | 29.40 |
| system/mem_usage | % | 54.99 | 55.55 | 55.35 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 73 | 250 | 54 |
| views/all/bins | count | 7604 | 11.9 k | 5911 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.4 k | 14.0 k |
| views/all/pipelines | count | 249 | 298 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.7 k | 35.8 k | 8127 |
| views/cascade/batch_sets | count | 9 | 51 | 4 |
| views/cascade/bins | count | 2799 | 6174 | 2413 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8902 | 23.5 k | 6364 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 418 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 194 | 0 |
| views/lamp_face/bins | count | 2156 | 2896 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2412 | 3189 | 0 |
| views/lamp_face/pipelines | count | 168 | 203 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2912 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2870 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8420 | 13.1 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8580 | 13.3 k | 7926 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 62.23 | 298 | 381.7 k |
| anim/events/count | count | 0.09 | 3 | 545 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.79 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.23 | 11 | 7528 |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.76 | 1303 | 4675 |
| encode//count | count | 34.80 | 219 | 213.4 k |
| encode/bind_walk/init_textures | count | 13.3 k | 114.7 k | 81.65 M |
| encode/bind_walk/init_textures_skipped | count | 62.8 k | 143.8 k | 384.97 M |
| encode/bind_walk/inits | count | 464 | 1128 | 2.85 M |
| encode/bind_walk/inits_skipped | count | 105 | 159 | 644.9 k |
| encode/bind_walk/merge_textures | count | 13.2 k | 114.6 k | 81.21 M |
| encode/bind_walk/merge_textures_skipped | count | 5369 | 6012 | 32.93 M |
| encode/bind_walk/merges | count | 101 | 336 | 617.2 k |
| encode/bind_walk/merges_skipped | count | 81.93 | 130 | 502.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6133 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6133 |
| encode/pass:early prepass/count | count | 1 | 1 | 6133 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6133 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6133 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6133 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6133 |
| encode/pass:shadow_point_light/count | count | 1.71 | 48 | 10.5 k |
| encode/queue/command_buffers | count | 46.53 | 277 | 285.3 k |
| encode/queue/submits | count | 2.02 | 4 | 12.4 k |
| encode/queue/write_buffer/bytes | bytes | 1.87 M | 4.88 M | 11.48 G |
| encode/queue/write_buffer/calls | count | 264 | 840 | 1.62 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.8 k | 3.53 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 10.97 | 465 | 67.3 k |
| encode/queue/write_texture/bytes | bytes | 11.6 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6133 |
| physics/inserts | count | 0.37 | 109 | 2258 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5520 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1210 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2603.2 | main/busy/ms 2573, stage/main/post_update 2465, stage/main/update 97.79, surface/advance_cpu_ms 67.72, stage/main/pre_update 5.55, stage/main/last 1.37, surface/cell_upload_ms 0.76, stream/drive/ms 0.73, stage/main/first 0.71, stage/main/fixed_main_loop 0.54, stream/tick/ms 0.23, weather/map/ms 0.22 |
| warm-up | 3.35 | 743.4 | main/busy/ms 2747, stage/main/post_update 2495, render/thread/ms 702, render/busy/ms 584, stage/render/prepare_assets 554, stage/main/update 235, stage/render/prepare_views 114, render/acquire/ms 114, surface/advance_cpu_ms 67.72, stage/render/render 13.26, render/graph/ms 8.60, stage/render/create_views 8.27 |
| warm-up | 3.46 | 111.3 | render/thread/ms 812, render/busy/ms 640, stage/render/prepare_assets 581, pipelines/compile_ms 312, pipelines/front_ms 244, main/busy/ms 197, stage/main/update 142, stage/render/prepare_views 117, render/acquire/ms 116, stage/render/render 80.85, pipelines/driver_ms 68.30, render/present/ms 55.98 |
| warm-up | 3.67 | 189.7 | pipelines/wait_ms 4348, pipelines/compile_ms 1743, pipelines/front_ms 1605, main/busy/ms 169, pipelines/driver_ms 138, render/thread/ms 87.75, render/busy/ms 85.99, stage/main/update 71.31, stage/main/post_update 70.28, stage/render/prepare_assets 28.62, stage/render/prepare_resources 21.03, stage/render/render 20.28 |
| warm-up | 3.74 | 77.1 | pipelines/wait_ms 4491, pipelines/compile_ms 2448, pipelines/front_ms 2280, pipelines/driver_ms 168, render/thread/ms 158, render/busy/ms 155, main/busy/ms 114, stage/render/prepare_assets 88.62, stage/main/update 69.52, stage/main/post_update 34.02, stage/render/render 25.20, render/graph/ms 22.29 |
| warm-up | 3.80 | 30.9 | pipelines/compile_ms 453, pipelines/front_ms 423, render/thread/ms 50.26, render/busy/ms 48.69, main/busy/ms 40.84, stage/render/prepare_assets 32.59, pipelines/driver_ms 29.88, stage/main/post_update 22.46, pipelines/wait_ms 16.58, terrain/update/ms 10.21, stage/main/update 9.89, stage/render/render 8.75 |
| warm-up | 3.83 | 29.8 | pipelines/compile_ms 986, pipelines/front_ms 933, pipelines/wait_ms 152, render/thread/ms 54.83, render/busy/ms 52.95, pipelines/driver_ms 52.88, main/busy/ms 40.79, stage/main/post_update 27.78, stage/render/prepare_assets 19.85, encode//sum 17.94, stage/render/render 13.55, render/graph/ms 11.28 |
| warm-up | 3.87 | 32.0 | pipelines/compile_ms 477, pipelines/front_ms 461, render/thread/ms 43.67, render/busy/ms 41.67, main/busy/ms 33.75, stage/main/post_update 17.54, stage/render/prepare_assets 17.53, pipelines/driver_ms 15.63, stage/render/render 12.36, stage/main/update 11.49, encode//sum 10.16, render/graph/ms 9.84 |
| warm-up | 3.98 | 86.8 | pipelines/compile_ms 1189, pipelines/front_ms 1143, render/thread/ms 59.51, main/busy/ms 57.95, render/busy/ms 53.72, pipelines/driver_ms 45.91, stage/main/update 44.06, stream/drive/ms 38.67, stage/render/prepare_assets 35.68, stage/render/render 16.98, pipelines/wait_ms 13.34, render/graph/ms 10.87 |
| warm-up | 4.01 | 28.8 | pipelines/compile_ms 1432, pipelines/front_ms 1376, render/thread/ms 70.50, render/busy/ms 68.52, pipelines/driver_ms 56.25, main/busy/ms 52.15, stage/main/update 39.86, stage/render/prepare_assets 38.15, stream/drive/ms 34.62, stage/render/render 24.70, render/graph/ms 22.43, stage/render/graph_submit 17.25 |
| warm-up | 4.06 | 54.8 | pipelines/compile_ms 539, pipelines/front_ms 516, render/thread/ms 59.60, render/busy/ms 57.18, main/busy/ms 56.97, stage/main/update 40.16, stream/drive/ms 35.30, stage/render/render 26.02, stage/render/prepare_assets 24.20, pipelines/driver_ms 23.24, render/graph/ms 23.22, stage/render/graph_submit 17.58 |
| warm-up | 4.11 | 47.5 | pipelines/compile_ms 541, pipelines/front_ms 517, render/thread/ms 72.69, render/busy/ms 70.51, stage/render/prepare_assets 50.55, main/busy/ms 43.01, pipelines/driver_ms 23.47, stage/main/update 20.30, stage/main/post_update 14.44, stream/drive/ms 13.43, stage/render/render 12.74, render/graph/ms 10.08 |
| warm-up | 4.17 | 63.2 | pipelines/compile_ms 331, pipelines/front_ms 316, render/thread/ms 75.85, render/busy/ms 74.16, stage/render/prepare_assets 56.57, main/busy/ms 43.68, stage/main/update 20.78, pipelines/driver_ms 14.87, stage/main/post_update 13.61, stream/drive/ms 13.49, stage/render/render 10.29, render/graph/ms 8.17 |
| warm-up | 4.24 | 68.5 | pipelines/compile_ms 760, pipelines/front_ms 731, render/thread/ms 79.65, render/busy/ms 78.16, stage/render/prepare_assets 59.17, main/busy/ms 36.73, pipelines/driver_ms 28.42, stage/main/update 16.93, stage/render/render 10.55, stage/main/post_update 9.19, render/graph/ms 8.36, stage/render/graph_submit 5.71 |
| warm-up | 4.29 | 51.6 | pipelines/compile_ms 1916, pipelines/front_ms 1851, render/thread/ms 73.28, render/busy/ms 71.78, pipelines/driver_ms 65.07, main/busy/ms 57.13, stage/render/prepare_assets 53.89, stage/main/update 39.47, stage/render/render 9.87, stage/main/post_update 8.55, render/graph/ms 7.66, pipelines/wait_ms 6.91 |
| warm-up | 4.37 | 73.3 | pipelines/compile_ms 1362, pipelines/front_ms 1315, render/thread/ms 90.25, render/busy/ms 84.23, main/busy/ms 56.27, pipelines/driver_ms 46.46, stage/main/update 38.59, stage/render/render 38.53, stage/render/prepare_assets 37.47, render/graph/ms 32.06, pipelines/wait_ms 29.47, stage/render/camera_driver 23.50 |
| warm-up | 4.40 | 38.4 | pipelines/compile_ms 199, pipelines/front_ms 191, pipelines/wait_ms 120, render/thread/ms 92.07, render/busy/ms 85.97, main/busy/ms 45.68, stage/render/render 41.23, render/graph/ms 34.65, stage/render/prepare_assets 26.30, stage/render/camera_driver 24.72, stage/main/update 19.57, stage/main/post_update 19.11 |
| warm-up | 4.47 | 70.7 | pipelines/compile_ms 523, pipelines/front_ms 507, pipelines/wait_ms 225, render/thread/ms 90.63, render/busy/ms 89.02, main/busy/ms 43.19, stage/render/prepare_assets 34.30, stage/main/post_update 22.95, encode//sum 20.73, stage/render/render 17.38, pipelines/driver_ms 16.60, render/graph/ms 15.15 |
| warm-up | 4.55 | 78.0 | pipelines/compile_ms 2114, pipelines/front_ms 2048, pipelines/wait_ms 714, render/thread/ms 132, render/busy/ms 130, pipelines/driver_ms 65.98, main/busy/ms 59.06, stage/render/prepare_assets 34.86, encode//sum 29.47, stage/render/prepare_write_phase_buffers 24.64, stage/render/render 22.01, render/graph/ms 19.49 |
| warm-up | 4.64 | 85.9 | pipelines/compile_ms 2662, pipelines/front_ms 2518, pipelines/wait_ms 1388, encode//sum 274, render/thread/ms 153, render/busy/ms 150, main/busy/ms 146, pipelines/driver_ms 144, stage/main/update 103, stage/render/render 68.60, render/graph/ms 65.48, stage/render/graph_submit 53.31 |
| warm-up | 4.75 | 115.5 | pipelines/compile_ms 2490, pipelines/front_ms 2350, pipelines/wait_ms 1323, encode//sum 260, render/thread/ms 146, render/busy/ms 144, pipelines/driver_ms 140, main/busy/ms 127, stage/main/update 105, stage/render/render 65.12, render/graph/ms 62.21, stream/drive/ms 53.03 |
| warm-up | 4.80 | 41.5 | pipelines/compile_ms 2234, pipelines/front_ms 2168, pipelines/wait_ms 881, render/thread/ms 103, render/busy/ms 102, main/busy/ms 94.79, stage/main/update 71.96, pipelines/driver_ms 66.21, stream/drive/ms 60.53, stage/render/prepare_assets 49.11, stage/render/render 19.96, render/graph/ms 17.66 |
| warm-up | 4.91 | 113.3 | pipelines/compile_ms 1053, pipelines/front_ms 1019, pipelines/wait_ms 363, main/busy/ms 98.94, render/thread/ms 85.78, render/busy/ms 84.36, stage/main/update 75.76, stream/drive/ms 59.88, stage/render/prepare_assets 45.56, pipelines/driver_ms 34.55, stage/render/render 17.99, render/graph/ms 15.73 |
| warm-up | 4.95 | 43.5 | pipelines/compile_ms 701, pipelines/front_ms 674, main/busy/ms 91.12, render/thread/ms 85.40, render/busy/ms 83.81, stage/main/update 65.71, stage/render/prepare_assets 47.14, stream/drive/ms 44.69, pipelines/driver_ms 26.57, stage/main/post_update 16.53, stage/render/render 13.91, stage/render/prepare_write_phase_buffers 12.72 |
| warm-up | 5.02 | 67.6 | pipelines/compile_ms 2026, pipelines/front_ms 1988, main/busy/ms 82.56, stage/main/update 63.13, render/thread/ms 62.00, render/busy/ms 60.38, stream/drive/ms 44.38, pipelines/driver_ms 38.10, stage/render/prepare_assets 26.00, stage/render/render 13.64, stage/render/prepare_write_phase_buffers 12.38, stage/main/post_update 11.87 |
| warm-up | 5.09 | 43.9 | pipelines/compile_ms 90.76, pipelines/front_ms 85.72, main/busy/ms 55.92, stage/main/update 39.97, render/thread/ms 38.78, render/busy/ms 37.24, stream/drive/ms 28.67, stage/render/render 12.34, render/graph/ms 10.34, stage/render/prepare_assets 10.29, stage/main/post_update 9.68, encode//sum 9.13 |
| warm-up | 5.17 | 30.7 | pipelines/compile_ms 131, pipelines/front_ms 122, render/thread/ms 41.16, render/busy/ms 39.88, main/busy/ms 32.14, stage/main/post_update 14.48, stage/render/prepare_assets 13.63, stage/main/update 12.27, stage/render/render 11.49, render/graph/ms 9.55, pipelines/driver_ms 8.96, encode//sum 8.46 |
| warm-up | 5.22 | 31.5 | pipelines/compile_ms 1178, pipelines/front_ms 1136, render/thread/ms 47.81, render/busy/ms 46.13, pipelines/driver_ms 42.06, main/busy/ms 36.98, stage/main/post_update 15.32, stage/main/update 15.19, stage/render/render 14.59, stage/render/prepare_assets 12.29, render/graph/ms 11.87, encode//sum 9.62 |
| warm-up | 5.26 | 33.9 | pipelines/compile_ms 1312, pipelines/front_ms 1266, render/thread/ms 53.75, render/busy/ms 52.37, pipelines/driver_ms 45.75, main/busy/ms 34.47, stage/render/prepare_assets 25.46, stage/main/update 16.31, stage/render/render 13.33, stage/main/post_update 12.29, render/graph/ms 10.97, encode//sum 7.87 |
| warm-up | 5.29 | 38.3 | pipelines/compile_ms 220, pipelines/front_ms 210, render/thread/ms 58.40, render/busy/ms 56.79, main/busy/ms 56.52, stage/render/prepare_assets 30.71, stage/main/update 20.56, stage/main/post_update 16.90, stage/render/render 13.72, stage/main/first 13.31, render/graph/ms 11.72, stream/drive/ms 10.99 |
| warm-up | 5.33 | 35.4 | pipelines/compile_ms 110, pipelines/front_ms 104, main/busy/ms 61.84, render/thread/ms 53.78, render/busy/ms 52.31, stage/main/post_update 24.57, stage/render/prepare_assets 18.93, stage/main/update 18.38, stage/main/first 13.34, stage/render/render 11.54, stream/drive/ms 11.11, render/graph/ms 9.30 |
| warm-up | 5.39 | 63.2 | pipelines/compile_ms 399, pipelines/front_ms 387, render/thread/ms 83.07, render/busy/ms 78.58, main/busy/ms 35.15, stage/render/prepare_assets 23.61, stage/render/prepare_resources 23.12, stage/main/post_update 17.82, stage/main/update 12.08, pipelines/driver_ms 11.87, stage/render/render 10.72, render/graph/ms 8.68 |
| warm-up | 5.47 | 50.3 | pipelines/compile_ms 1879, pipelines/front_ms 1838, pipelines/wait_ms 107, render/thread/ms 64.74, render/busy/ms 63.24, main/busy/ms 42.97, pipelines/driver_ms 40.76, stage/render/prepare_assets 28.37, stage/main/post_update 18.54, stage/main/update 16.54, stage/render/prepare_resources 12.64, stage/render/render 11.43 |
| warm-up | 5.57 | 36.6 | pipelines/compile_ms 358, pipelines/front_ms 351, pipelines/wait_ms 121, render/thread/ms 45.91, render/busy/ms 44.48, main/busy/ms 36.67, stage/render/prepare_assets 19.87, stage/main/update 19.16, stage/render/render 14.02, stage/main/post_update 12.24, render/graph/ms 11.86, encode//sum 9.23 |
| warm-up | 5.62 | 29.3 | render/thread/ms 47.40, render/busy/ms 46.11, pipelines/compile_ms 40.16, pipelines/front_ms 37.37, stage/render/prepare_assets 26.93, main/busy/ms 25.33, stage/main/post_update 12.97, stage/render/render 12.31, render/graph/ms 10.32, stage/main/update 7.57, encode//sum 6.66, stage/render/graph_submit 6.07 |
| warm-up | 5.68 | 33.6 | pipelines/compile_ms 331, pipelines/front_ms 311, render/thread/ms 47.82, render/busy/ms 46.00, main/busy/ms 25.70, stage/render/prepare_assets 21.98, pipelines/driver_ms 20.30, stage/render/render 16.30, render/graph/ms 14.15, stage/main/post_update 11.60, stage/render/graph_submit 11.16, stage/main/update 9.03 |
| warm-up | 6.18 | 30.2 | render/thread/ms 36.30, render/busy/ms 35.24, main/busy/ms 26.47, stage/render/prepare_assets 17.39, stage/main/update 10.70, stage/main/post_update 9.94, stage/render/render 8.46, render/graph/ms 6.96, encode//sum 6.65, stage/render/graph_submit 4.62, stream/drive/ms 3.66, encode/submit 2.87 |
| warm-up | 6.37 | 30.7 | main/busy/ms 43.32, render/thread/ms 40.20, render/busy/ms 39.01, stage/main/update 21.62, stage/main/post_update 16.85, encode//sum 16.01, stage/render/render 14.30, render/graph/ms 12.47, stage/render/graph_submit 9.76, stage/render/prepare_assets 6.35, encode/finish_wall 6.04, stream/drive/ms 5.88 |
| warm-up | 6.40 | 32.2 | pipelines/compile_ms 141, pipelines/front_ms 135, render/thread/ms 50.79, render/busy/ms 49.56, main/busy/ms 45.41, stage/main/post_update 20.42, stage/main/update 17.87, encode//sum 17.10, stage/render/render 14.51, stage/render/prepare_assets 14.19, render/graph/ms 12.59, stage/render/graph_submit 9.38 |
| warm-up | 6.52 | 32.6 | main/busy/ms 36.91, render/thread/ms 36.06, render/busy/ms 34.75, stage/main/update 17.74, stage/render/prepare_assets 16.15, stage/main/post_update 11.93, stage/render/render 10.14, encode//sum 9.23, render/graph/ms 8.21, stage/render/graph_submit 5.17, stream/drive/ms 4.22, stage/render/camera_driver 2.91 |
| warm-up | 6.71 | 44.8 | render/thread/ms 49.13, render/busy/ms 48.13, main/busy/ms 38.67, stage/render/prepare_assets 30.98, stage/main/update 25.49, stream/drive/ms 19.38, stage/render/render 9.68, stage/main/post_update 9.59, render/graph/ms 8.18, encode//sum 7.45, stage/render/graph_submit 5.46, encode/submit 3.56 |
| warm-up | 6.93 | 35.9 | main/busy/ms 42.63, render/thread/ms 29.11, render/busy/ms 28.18, stage/main/update 24.76, stream/drive/ms 13.99, stage/render/prepare_assets 12.85, stage/main/post_update 10.68, stage/render/render 9.53, render/graph/ms 8.16, encode//sum 7.00, stage/render/graph_submit 5.96, encode/submit 3.17 |
| warm-up | 7.01 | 32.4 | render/thread/ms 42.67, render/busy/ms 41.62, pipelines/compile_ms 20.69, pipelines/front_ms 19.53, main/busy/ms 19.05, stage/render/render 11.49, stage/render/specialize 10.12, stage/main/post_update 10.10, render/graph/ms 9.97, stage/render/graph_submit 7.71, encode//sum 7.06, stage/render/queue 6.95 |
| warm-up | 8.28 | 31.8 | render/thread/ms 34.32, render/busy/ms 33.29, main/busy/ms 19.40, stage/main/post_update 11.66, stage/render/render 9.74, stage/render/specialize 9.57, render/graph/ms 8.30, encode//sum 7.39, stage/render/graph_submit 6.25, stage/render/queue 5.61, stage/main/update 5.24, encode/submit 3.45 |
| measure | 0.40 | 42.9 | render/thread/ms 46.31, render/busy/ms 45.14, encode//sum 18.63, main/busy/ms 17.11, stage/render/render 16.30, render/graph/ms 14.54, encode/pass:shadow_point_light/sum 10.88, stage/main/post_update 10.70, stage/render/graph_submit 10.29, stage/render/specialize 9.95, stage/render/queue 7.13, encode/submit 5.51 |
| measure | 1.69 | 33.2 | render/thread/ms 36.21, render/busy/ms 35.16, main/busy/ms 17.27, stage/main/post_update 10.91, stage/render/render 10.76, render/graph/ms 9.18, stage/render/specialize 8.98, encode//sum 8.32, stage/render/graph_submit 6.57, stage/render/queue 5.97, stage/render/prepare_meshes 4.61, stage/main/update 4.08 |
| measure | 4.34 | 30.2 | render/thread/ms 34.29, render/busy/ms 33.32, main/busy/ms 18.49, stage/main/post_update 12.12, stage/render/render 10.29, stage/render/specialize 9.34, render/graph/ms 8.89, stage/render/graph_submit 6.94, encode//sum 6.75, stage/render/queue 6.02, stage/main/update 4.01, encode/submit 3.75 |
| measure | 5.67 | 30.5 | render/thread/ms 35.22, render/busy/ms 34.00, main/busy/ms 18.65, stage/main/post_update 12.32, stage/render/render 10.92, render/graph/ms 9.30, stage/render/specialize 9.18, stage/render/graph_submit 7.14, encode//sum 7.13, stage/render/queue 6.59, encode/submit 4.27, stage/main/update 3.98 |
| measure | 6.95 | 29.2 | render/thread/ms 34.84, render/busy/ms 33.76, main/busy/ms 18.78, stage/main/post_update 11.81, stage/render/render 10.94, stage/render/specialize 9.37, render/graph/ms 9.35, encode//sum 8.22, stage/render/graph_submit 7.06, stage/render/queue 6.10, stage/main/update 4.32, encode/submit 4.21 |
| measure | 15.73 | 30.0 | render/thread/ms 39.24, render/busy/ms 37.01, main/busy/ms 17.43, stage/render/render 13.08, stage/main/post_update 10.54, render/graph/ms 10.38, stage/render/specialize 9.89, stage/render/graph_submit 7.81, encode//sum 7.39, stage/render/queue 6.96, encode/submit 6.38, stage/main/update 4.21 |
| measure | 18.17 | 32.1 | main/busy/ms 38.53, stage/main/update 23.74, render/thread/ms 20.56, stream/drive/ms 20.23, render/busy/ms 19.58, stage/main/post_update 12.01, stage/render/render 11.95, render/graph/ms 10.50, encode//sum 8.72, stage/render/graph_submit 7.65, encode/submit 4.62, encode/pass:shadow_point_light/sum 3.92 |
| measure | 23.03 | 34.1 | render/thread/ms 42.72, render/busy/ms 41.78, main/busy/ms 24.50, stage/render/render 13.32, stage/main/post_update 12.77, stage/render/specialize 12.17, render/graph/ms 11.96, encode//sum 10.79, stage/render/graph_submit 8.89, stage/render/queue 7.15, stage/main/update 6.29, encode/pass:shadow_point_light/sum 6.07 |
| measure | 24.52 | 29.8 | render/thread/ms 36.50, render/busy/ms 35.44, main/busy/ms 19.56, stage/main/post_update 11.87, stage/render/render 11.39, render/graph/ms 9.97, stage/render/specialize 9.59, encode//sum 9.38, stage/render/graph_submit 7.34, stage/render/queue 6.89, stage/main/update 4.97, encode/submit 4.59 |
| measure | 27.28 | 35.1 | render/thread/ms 47.43, render/busy/ms 46.15, main/busy/ms 27.43, encode//sum 19.73, stage/render/render 17.57, render/graph/ms 15.82, stage/main/post_update 13.15, stage/render/graph_submit 12.24, stage/render/specialize 11.20, encode/pass:shadow_point_light/sum 9.24, encode/submit 7.82, stage/render/queue 7.42 |
| measure | 30.47 | 31.5 | render/thread/ms 37.10, render/busy/ms 36.02, main/busy/ms 20.58, stage/main/post_update 11.69, stage/render/specialize 10.46, stage/render/render 10.40, render/graph/ms 8.91, encode//sum 8.29, stage/render/graph_submit 6.98, stage/render/queue 6.79, stage/main/update 5.37, encode/submit 4.08 |
| measure | 35.33 | 34.0 | render/thread/ms 39.24, render/busy/ms 38.30, main/busy/ms 19.85, stage/main/post_update 12.41, stage/render/render 12.15, stage/render/specialize 11.03, render/graph/ms 10.69, encode//sum 9.87, stage/render/graph_submit 8.37, stage/render/queue 6.38, encode/submit 4.69, stage/main/update 4.29 |
| measure | 36.65 | 31.9 | render/thread/ms 36.29, render/busy/ms 35.25, main/busy/ms 19.29, stage/main/post_update 11.66, stage/render/render 10.33, stage/render/specialize 9.70, render/graph/ms 8.76, encode//sum 7.48, stage/render/queue 7.29, stage/render/graph_submit 6.69, stage/main/update 4.69, encode/submit 3.61 |
| measure | 37.47 | 30.9 | render/thread/ms 34.14, render/busy/ms 33.10, main/busy/ms 18.06, stage/main/post_update 11.52, stage/render/specialize 9.75, stage/render/render 9.53, render/graph/ms 8.02, encode//sum 6.98, stage/render/queue 6.45, stage/render/graph_submit 6.10, stage/main/update 3.89, encode/submit 3.44 |
| measure | 40.79 | 34.5 | render/thread/ms 38.59, render/busy/ms 37.46, main/busy/ms 18.55, stage/main/post_update 10.76, stage/render/render 10.39, stage/render/specialize 10.21, render/graph/ms 8.92, encode//sum 7.54, stage/render/graph_submit 7.13, stage/render/queue 6.63, stage/main/update 4.30, encode/submit 3.76 |
| measure | 43.20 | 30.5 | render/thread/ms 35.96, render/busy/ms 34.91, main/busy/ms 16.89, stage/main/post_update 9.98, stage/render/specialize 9.98, stage/render/render 9.43, encode//sum 8.22, render/graph/ms 8.04, stage/render/queue 6.65, stage/render/graph_submit 6.34, stage/main/update 3.99, stage/render/prepare_meshes 3.51 |
| measure | 45.65 | 32.2 | render/thread/ms 38.76, render/busy/ms 37.84, main/busy/ms 17.33, stage/render/render 10.34, stage/render/specialize 9.96, stage/main/post_update 9.79, render/graph/ms 9.10, encode//sum 7.97, stage/render/graph_submit 7.19, stage/render/queue 6.93, stage/render/prepare_meshes 4.66, stage/main/update 4.52 |
| measure | 59.04 | 45.4 | render/thread/ms 55.50, render/busy/ms 53.91, main/busy/ms 31.30, stage/render/render 17.88, stage/main/post_update 17.60, render/graph/ms 15.81, stage/render/graph_submit 12.62, stage/render/specialize 11.32, encode//sum 10.80, stage/main/update 9.18, stage/render/prepare_meshes 7.27, stage/render/queue 6.71 |
| measure | 59.08 | 34.5 | render/thread/ms 58.99, render/busy/ms 58.00, main/busy/ms 25.15, stage/render/prepare_assets 16.48, stage/main/post_update 12.23, stage/render/specialize 11.40, stage/render/render 9.92, encode//sum 9.11, render/graph/ms 8.65, stage/render/prepare_meshes 8.22, stage/main/update 7.52, stage/render/queue 6.82 |
| measure | 62.57 | 30.5 | render/thread/ms 38.89, render/busy/ms 37.71, main/busy/ms 18.58, stage/render/render 11.79, stage/render/specialize 10.29, render/graph/ms 10.25, stage/main/post_update 9.95, encode//sum 8.26, stage/render/graph_submit 8.16, stage/render/queue 6.51, stage/main/update 5.64, encode/submit 4.47 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set mw.matctrl.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=matctrl-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **79%** (min 58%) · 2624 MHz (min 2610) · memory 8101 MHz · 81.1 W · 77 °C (max 83) · VRAM 6808 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `mw.matctrl.gate=on (command line: --set)`
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
