# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6026 frames over 65.6 s · **median 10.66 ms (94 fps)** · mean 10.89 · p95 13.45 · p99 16.87 (1% low 59 fps) · worst 43.97 ms · 68 spikes over 1.5× median · 60.3 stutters/min (work 48.4/min)
- warm-up: 323 frames over 9.4 s, worst 2600.1 ms
- hitches (> 26.7 ms): **33** measured, 43 in warm-up
- timed GPU passes: 6.47 ms of a 10.89 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.66 ms, limit 11 ms
- **FAIL** frame_ms: 66 stutters (> 1.5x moving median of 60) in 1.09 min = 60.33/min, limit 1/min
- **FAIL** worst frame 43.97 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.836 | 2.034 | 2.068 | 0.90 | 0.172 |
| main_opaque_pass_3d | 1.501 | 1.664 | 2.056 | 0.90 | 0.072 |
| early prepass | 0.440 | 0.487 | 0.714 | 0.90 | 0.091 |
| bloom | 0.284 | 0.314 | 0.321 | 0.90 | 0.062 |
| atmosphere_luts | 0.274 | 0.304 | 0.366 | 0.90 | 0.004 |
| water_velocity | 0.253 | 0.280 | 0.347 | 0.90 | 0.002 |
| water_sspr | 0.182 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.170 | 0.188 | 0.193 | 0.90 | 0.007 |
| atmos_froxel_inject | 0.131 | 0.145 | 0.147 | 0.90 | 0.001 |
| clouds_trace | 0.127 | 0.140 | 0.232 | 0.90 | 0.001 |
| water_fft | 0.118 | 0.130 | 0.137 | 0.90 | 0.003 |
| uniform_allocation | 0.117 | 0.130 | 0.184 | 0.90 | 0.026 |
| upscaling | 0.105 | 0.117 | 0.124 | 0.90 | 0.004 |
| auxiliary_views | 0.102 | 0.113 | 0.427 | 0.90 | 0.118 |
| clouds_composite | 0.095 | 0.106 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.095 | 0.105 | 0.143 | 0.90 | 0.019 |
| tonemapping | 0.077 | 0.085 | 0.089 | 0.90 | 0.004 |
| main_transmissive_pass_3d | 0.076 | 0.085 | 0.177 | 0.90 | 0.008 |
| clustering | 0.058 | 0.064 | 0.067 | 0.90 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.90 | 0.002 |
| clouds_resolve | 0.044 | 0.049 | 0.049 | 0.90 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.90 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.64 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.80 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.033 | 0.90 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.90 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.027 | 0.411 | 0.416 | 0.06 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.90 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.90 | 0.002 |
| lightprobe_radiance_map | 0.014 | 0.219 | 0.229 | 0.06 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.90 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.90 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.90 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.90 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.06 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.06 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.06 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.366 | 11.462 | 38.13 | 56438.1 |
| main/busy/ms | 9.041 | 10.668 | 29.65 | 54482.3 |
| render/busy/ms | 8.815 | 10.962 | 37.57 | 53118.2 |
| stage/main/post_update | 5.599 | 6.899 | 10.88 | 33737.2 |
| stage/render/render | 5.418 | 6.788 | 14.36 | 32649.3 |
| render/graph/ms | 4.676 | 6.034 | 13.45 | 28178.6 |
| encode//sum | 3.804 | 4.886 | 19.95 | 22921.2 |
| stage/render/graph_submit | 3.514 | 4.631 | 9.49 | 21172.8 |
| encode/submit | 2.229 | 3.150 | 6.69 | 13431.4 |
| stage/main/update | 2.169 | 2.587 | 23.29 | 13068.7 |
| encode/finish_wall | 1.358 | 1.983 | 4.18 | 8185.5 |
| stage/render/camera_driver | 1.110 | 1.504 | 3.94 | 6688.6 |
| encode/pass:shadow_directional_light/sum | 0.879 | 1.568 | 2.93 | 5298.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.812 | 1.117 | 1.91 | 4890.7 |
| encode/pass:early prepass/sum | 0.776 | 1.092 | 1.61 | 4677.5 |
| stage/render/prepare_write_phase_buffers | 0.770 | 1.122 | 2.49 | 4640.1 |
| encode/pass:shadow_point_light/sum | 0.695 | 2.938 | 17.05 | 4187.3 |
| stage/render/prepare_assets | 0.576 | 0.757 | 11.05 | 3473.5 |
| bigspace/total/ms | 0.546 | 1.324 | 2.32 | 3289.4 |
| stage/render/prepare_resources | 0.459 | 0.549 | 3.16 | 2767.9 |
| render/present/ms | 0.456 | 1.183 | 2.62 | 2750.5 |
| stage/render/prepare_meshes | 0.443 | 0.548 | 7.78 | 2670.9 |
| stage/main/first | 0.404 | 0.532 | 5.40 | 2432.9 |
| stage/main/pre_update | 0.363 | 0.527 | 2.66 | 2186.5 |
| stage/render/specialize | 0.323 | 0.418 | 11.47 | 1945.8 |
| terrain/update/ms | 0.309 | 0.400 | 2.27 | 1864.5 |
| stage/render/queue | 0.296 | 0.430 | 7.83 | 1784.4 |
| stage/render/prepare_bind_groups | 0.284 | 0.499 | 2.58 | 1711.5 |
| terrain/select/ms | 0.275 | 0.357 | 1.05 | 1657.3 |
| controller/feed/ms | 0.269 | 0.343 | 0.95 | 1619.9 |
| stream/drive/ms | 0.248 | 0.334 | 21.21 | 1493.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.241 | 0.355 | 1.03 | 1452.4 |
| bigspace/low_precision/ms | 0.227 | 0.765 | 1.87 | 1370.8 |
| stage/main/fixed_main_loop | 0.216 | 0.369 | 1.41 | 1303.3 |
| far/select/ms | 0.200 | 0.271 | 0.55 | 1207.7 |
| stage/render/create_views | 0.200 | 0.257 | 0.91 | 1202.4 |
| stage/render/prepare_batch_phases | 0.191 | 0.279 | 0.52 | 1151.5 |
| stream/tick/ms | 0.190 | 0.265 | 0.48 | 1146.2 |
| stage/render/extract_commands | 0.172 | 0.200 | 0.44 | 1035.1 |
| anim/evaluate/ms | 0.165 | 0.304 | 1.76 | 993.6 |
| stage/render/prepare_flush | 0.163 | 0.309 | 0.78 | 982.6 |
| bigspace/high_precision/ms | 0.154 | 0.655 | 1.74 | 927.2 |
| anim/evaluate_cpu/ms | 0.152 | 0.189 | 0.69 | 915.4 |
| controller/move/ms | 0.146 | 0.301 | 0.65 | 878.0 |
| stage/render/prepare_views | 0.138 | 0.172 | 0.61 | 831.9 |
| stage/main/last | 0.114 | 0.158 | 1.71 | 688.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.161 | 0.74 | 603.6 |
| render/acquire/ms | 0.094 | 0.120 | 0.54 | 569.4 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.130 | 0.50 | 373.2 |
| pipelines/compile_ms | 0.050 | 0.000 | 127.67 | 301.2 |
| pipelines/front_ms | 0.048 | 0.000 | 123.62 | 288.8 |
| physics/step/ms | 0.046 | 0.084 | 0.59 | 277.2 |
| physics/insert/ms | 0.033 | 0.369 | 0.85 | 197.6 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.61 | 186.7 |
| stage/render/phase_sort | 0.030 | 0.052 | 0.72 | 182.1 |
| stage/render/cleanup | 0.030 | 0.081 | 0.79 | 179.9 |
| weather/map/ms | 0.025 | 0.034 | 0.24 | 153.5 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.57 | 91.6 |
| anim/apply/ms | 0.012 | 0.021 | 0.33 | 71.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.49 | 68.5 |
| controller/avoid/ms | 0.011 | 0.014 | 0.05 | 64.4 |
| weather/eval/ms | 0.011 | 0.014 | 0.21 | 63.7 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 43.5 |
| script/input/ms | 0.005 | 0.007 | 0.18 | 32.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.18 | 30.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.14 | 29.2 |
| script/ui/ms | 0.004 | 0.006 | 0.17 | 26.1 |
| controller/write/ms | 0.003 | 0.004 | 0.10 | 20.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 14.5 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.06 | 12.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.06 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.12 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1304 | 2262 | 603 |
| crowd/c0/meshes/count | count | 355 | 646 | 161 |
| crowd/c1/count | count | 145 | 157 | 148 |
| crowd/c1/joints/count | count | 5531 | 6050 | 5593 |
| crowd/c1/meshes/count | count | 1166 | 1324 | 1180 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4612 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4589 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 1.59 | 0.33 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.57 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.49 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.93 | 0.84 |
| encode/pass:early prepass/max | ms | 0.79 | 1.61 | 0.69 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.83 | 1.91 | 0.60 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.74 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.03 | 0.32 |
| encode/pass:shadow_directional_light/max | ms | 0.83 | 2.93 | 0.84 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.49 | 0.92 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.61 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.32 | 37.57 | 8.34 |
| gpu/timed/ms | ms | 7.91 | 9.00 | 8.34 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 814 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1290 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3413 | 5110 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1290 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3413 | 5110 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 15 | 228 | 4 |
| phases/shadow/bins | count | 4973 | 8903 | 2377 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.2 k | 6324 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 254 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 185 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 619 | 4 |
| preprocess/bind_groups_created | count | 13 | 619 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 207 | 125 |
| process/cpu_usage | % | 26.13 | 40 | 28.37 |
| process/mem_usage | GiB | 5.49 | 5.66 | 5.66 |
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
| skin/blocks_changed/count | count | 13 | 122 | 7 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 368.9 k | 809.7 k | 485.6 k |
| skin/joints_written/count | count | 5764 | 12.7 k | 7588 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4916 | 5009 | 5009 |
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
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.85 | 41.89 | 39.87 |
| system/mem_usage | % | 54.44 | 55.07 | 55.03 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 65 | 277 | 54 |
| views/all/bins | count | 7625 | 11.8 k | 5875 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.2 k | 13.9 k |
| views/all/pipelines | count | 248 | 301 | 74 |
| views/all/pipelines_distinct | count | 70 | 75 | 61 |
| views/all/work_items | count | 10.7 k | 35.8 k | 8123 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2785 | 6183 | 2377 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8883 | 23.5 k | 6324 |
| views/cascade/pipelines | count | 36 | 53 | 28 |
| views/cascade/work_items | count | 411 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2136 | 2866 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2395 | 3160 | 0 |
| views/lamp_face/pipelines | count | 165 | 216 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2888 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2886 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8438 | 13.0 k | 7614 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8600 | 13.2 k | 7922 |
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
| anim/bones_written/count | count | 56.92 | 231 | 343.0 k |
| anim/events/count | count | 0.09 | 2 | 540 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1242 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.59 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1296 | 4675 |
| encode//count | count | 32.19 | 220 | 194.0 k |
| encode/bind_walk/init_textures | count | 11.8 k | 119.2 k | 71.02 M |
| encode/bind_walk/init_textures_skipped | count | 67.4 k | 146.5 k | 406.29 M |
| encode/bind_walk/inits | count | 454 | 1208 | 2.73 M |
| encode/bind_walk/inits_skipped | count | 111 | 166 | 666.9 k |
| encode/bind_walk/merge_textures | count | 11.7 k | 119.1 k | 70.59 M |
| encode/bind_walk/merge_textures_skipped | count | 5365 | 6012 | 32.33 M |
| encode/bind_walk/merges | count | 96.55 | 353 | 581.8 k |
| encode/bind_walk/merges_skipped | count | 85.66 | 131 | 516.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6026 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6026 |
| encode/pass:early prepass/count | count | 1 | 1 | 6026 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6026 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6026 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6026 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6026 |
| encode/pass:shadow_point_light/count | count | 1.06 | 48 | 6383 |
| encode/queue/command_buffers | count | 43.27 | 278 | 260.7 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.31 G |
| encode/queue/write_buffer/calls | count | 255 | 924 | 1.54 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.1 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 11.01 | 465 | 66.3 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6026 |
| physics/inserts | count | 0.37 | 108 | 2246 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.65 | 3 | 3938 |
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
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 22 | 378 |
| stream/upload/bytes | bytes | 5618 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1232 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.7 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2600.1 | main/busy/ms 2566, stage/main/post_update 2463, stage/main/update 95.14, surface/advance_cpu_ms 65.96, stage/main/pre_update 4.37, stage/main/last 1.04, surface/cell_upload_ms 0.74, stage/main/first 0.59, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.31, weather/map/ms 0.24, bigspace/total/ms 0.10 |
| warm-up | 3.37 | 769.9 | main/busy/ms 2714, stage/main/post_update 2487, render/thread/ms 734, render/busy/ms 565, stage/render/prepare_assets 515, stage/main/update 213, stage/render/prepare_views 163, render/acquire/ms 163, surface/advance_cpu_ms 65.96, stage/render/create_views 28.60, stage/render/render 15.19, render/graph/ms 8.31 |
| warm-up | 3.54 | 172.2 | render/thread/ms 889, render/busy/ms 633, stage/render/prepare_assets 567, main/busy/ms 178, stage/render/prepare_views 164, render/acquire/ms 163, stage/main/update 125, pipelines/compile_ms 115, stage/render/render 108, render/present/ms 92.14, pipelines/driver_ms 72.91, pipelines/front_ms 42.50 |
| warm-up | 3.70 | 134.7 | pipelines/wait_ms 2847, pipelines/compile_ms 1639, pipelines/front_ms 1427, pipelines/driver_ms 211, main/busy/ms 176, render/thread/ms 97.98, render/busy/ms 87.61, stage/main/update 60.05, stage/main/post_update 50.56, stage/render/prepare_assets 48.37, stage/render/prepare_resources 23.75, stage/main/fixed_main_loop 18.58 |
| warm-up | 3.86 | 157.3 | pipelines/wait_ms 5191, pipelines/compile_ms 2650, pipelines/front_ms 2472, render/thread/ms 192, render/busy/ms 189, pipelines/driver_ms 178, main/busy/ms 150, stage/render/prepare_assets 143, stage/main/update 68.98, stage/main/post_update 36.17, stage/render/prepare_resources 24.54, stage/main/fixed_main_loop 17.66 |
| warm-up | 3.95 | 91.2 | pipelines/wait_ms 2793, pipelines/compile_ms 2633, pipelines/front_ms 2346, encode//sum 476, pipelines/driver_ms 287, render/thread/ms 203, render/busy/ms 200, stage/render/prepare_assets 118, stage/render/render 73.56, render/graph/ms 69.88, main/busy/ms 59.48, stage/render/graph_submit 58.43 |
| warm-up | 4.00 | 53.5 | pipelines/compile_ms 2052, pipelines/front_ms 1796, pipelines/wait_ms 564, encode//sum 486, pipelines/driver_ms 255, render/thread/ms 141, render/busy/ms 114, stage/render/render 80.20, render/graph/ms 77.55, stage/render/graph_submit 71.93, encode/finish_wall 56.44, encode/pass:shadow_directional_light/sum 51.81 |
| warm-up | 4.05 | 35.8 | pipelines/compile_ms 938, pipelines/front_ms 891, render/thread/ms 49.14, pipelines/driver_ms 47.55, render/busy/ms 40.78, main/busy/ms 33.98, stage/render/prepare_assets 20.90, stage/main/post_update 20.29, stage/render/render 11.48, encode//sum 10.43, render/graph/ms 9.43, terrain/update/ms 9.14 |
| warm-up | 4.15 | 78.4 | pipelines/compile_ms 261, pipelines/front_ms 250, main/busy/ms 51.03, render/thread/ms 49.48, render/busy/ms 47.95, stage/main/update 38.35, stream/drive/ms 34.07, stage/render/prepare_assets 30.02, stage/render/render 12.55, pipelines/driver_ms 11.29, render/graph/ms 10.72, terrain/update/ms 8.64 |
| warm-up | 4.22 | 54.3 | pipelines/compile_ms 480, pipelines/front_ms 466, render/thread/ms 60.36, render/busy/ms 58.32, main/busy/ms 45.89, stage/render/prepare_assets 34.77, stage/render/render 16.70, stage/main/update 16.17, pipelines/driver_ms 14.53, render/graph/ms 14.22, stage/main/post_update 13.30, stage/render/graph_submit 11.52 |
| warm-up | 4.28 | 52.8 | pipelines/compile_ms 389, pipelines/front_ms 379, render/thread/ms 49.73, render/busy/ms 48.22, stage/render/prepare_assets 35.18, main/busy/ms 26.50, pipelines/driver_ms 10.42, stage/main/update 9.76, stage/render/render 8.97, stage/main/post_update 8.51, render/graph/ms 7.15, encode//sum 6.03 |
| warm-up | 4.32 | 41.0 | pipelines/compile_ms 458, pipelines/front_ms 444, render/thread/ms 72.60, render/busy/ms 70.33, main/busy/ms 62.08, stage/render/prepare_assets 41.31, stage/main/update 39.65, stage/render/render 14.30, pipelines/driver_ms 13.60, render/graph/ms 11.78, stage/main/post_update 10.79, stage/render/graph_submit 8.25 |
| warm-up | 4.43 | 106.2 | pipelines/compile_ms 1316, pipelines/front_ms 1278, render/thread/ms 98.21, render/busy/ms 95.97, pipelines/wait_ms 69.70, main/busy/ms 61.14, stage/render/prepare_assets 58.34, stage/main/update 39.88, pipelines/driver_ms 38.37, stage/render/render 18.20, render/graph/ms 15.46, stage/main/post_update 10.84 |
| warm-up | 4.51 | 79.6 | pipelines/compile_ms 1839, pipelines/front_ms 1783, pipelines/wait_ms 175, render/thread/ms 147, render/busy/ms 145, main/busy/ms 56.75, pipelines/driver_ms 55.91, stage/render/prepare_assets 55.19, stage/render/render 29.85, render/graph/ms 27.32, stage/main/update 26.42, encode//sum 24.41 |
| warm-up | 4.56 | 58.3 | pipelines/compile_ms 1533, pipelines/front_ms 1485, pipelines/wait_ms 250, render/thread/ms 120, render/busy/ms 118, main/busy/ms 75.71, pipelines/driver_ms 48.12, stage/main/post_update 38.81, stage/render/prepare_resources 33.24, stage/render/render 29.54, stage/main/update 28.66, encode//sum 28.42 |
| warm-up | 4.65 | 81.3 | pipelines/compile_ms 1856, pipelines/front_ms 1791, pipelines/wait_ms 225, encode//sum 173, render/thread/ms 116, render/busy/ms 114, main/busy/ms 66.94, pipelines/driver_ms 64.30, stage/render/render 41.76, render/graph/ms 39.54, stage/main/update 34.67, stage/render/graph_submit 31.46 |
| warm-up | 4.71 | 65.3 | pipelines/compile_ms 1719, pipelines/front_ms 1646, encode//sum 169, render/thread/ms 119, render/busy/ms 117, pipelines/wait_ms 112, main/busy/ms 79.80, pipelines/driver_ms 73.18, stage/render/render 43.68, stage/main/update 42.73, render/graph/ms 40.62, stage/render/prepare_assets 35.39 |
| warm-up | 4.89 | 152.6 | pipelines/compile_ms 2019, pipelines/front_ms 1966, pipelines/wait_ms 120, render/thread/ms 104, render/busy/ms 103, main/busy/ms 98.43, stage/main/update 79.59, stage/render/prepare_assets 54.39, pipelines/driver_ms 53.24, stage/render/render 41.68, stream/drive/ms 40.09, render/graph/ms 39.54 |
| warm-up | 4.94 | 51.8 | pipelines/compile_ms 1569, pipelines/front_ms 1523, render/thread/ms 133, render/busy/ms 130, main/busy/ms 69.18, pipelines/wait_ms 66.53, stage/render/render 57.52, render/graph/ms 53.98, stage/render/prepare_assets 48.48, stage/render/graph_submit 48.22, pipelines/driver_ms 45.93, stage/main/update 42.84 |
| warm-up | 5.00 | 57.8 | main/busy/ms 108, render/thread/ms 95.32, render/busy/ms 92.18, pipelines/compile_ms 84.06, pipelines/front_ms 75.15, stage/main/update 63.75, stream/drive/ms 49.20, stage/main/post_update 35.47, stage/render/render 30.41, render/graph/ms 26.35, pipelines/wait_ms 20.68, stage/render/graph_submit 19.66 |
| warm-up | 5.11 | 105.9 | pipelines/compile_ms 2316, pipelines/front_ms 2251, pipelines/wait_ms 259, main/busy/ms 149, stage/main/update 110, stream/drive/ms 98.82, render/thread/ms 93.75, render/busy/ms 92.00, pipelines/driver_ms 64.81, stage/render/prepare_assets 39.71, stage/main/post_update 33.41, stage/render/render 16.47 |
| warm-up | 5.20 | 91.4 | pipelines/compile_ms 2774, pipelines/front_ms 2696, pipelines/wait_ms 242, main/busy/ms 101, render/thread/ms 90.59, render/busy/ms 88.91, pipelines/driver_ms 78.55, stage/main/update 74.01, stream/drive/ms 59.51, stage/render/prepare_assets 57.80, stage/main/post_update 17.51, stage/render/render 13.15 |
| warm-up | 5.26 | 65.3 | pipelines/compile_ms 1036, pipelines/front_ms 954, render/thread/ms 102, render/busy/ms 101, pipelines/driver_ms 82.13, main/busy/ms 81.68, stage/main/update 51.63, pipelines/wait_ms 51.05, stage/render/render 47.54, render/graph/ms 45.53, encode//sum 44.01, stage/render/graph_submit 42.32 |
| warm-up | 5.31 | 46.1 | pipelines/compile_ms 1077, pipelines/front_ms 978, pipelines/driver_ms 99.59, render/thread/ms 93.73, render/busy/ms 91.83, main/busy/ms 70.93, stage/render/render 50.96, pipelines/wait_ms 49.92, stage/main/update 49.75, render/graph/ms 48.59, stage/render/graph_submit 43.94, encode//sum 43.37 |
| warm-up | 5.35 | 41.3 | pipelines/compile_ms 642, pipelines/front_ms 603, render/thread/ms 70.50, render/busy/ms 68.24, main/busy/ms 45.13, pipelines/driver_ms 38.72, stage/render/prepare_assets 27.01, stage/main/update 23.63, stage/render/render 17.09, render/graph/ms 14.60, stage/main/post_update 11.22, stage/render/graph_submit 10.05 |
| warm-up | 5.38 | 30.7 | pipelines/compile_ms 84.10, pipelines/front_ms 79.43, render/thread/ms 58.64, render/busy/ms 56.78, main/busy/ms 43.33, stage/main/update 19.90, stage/render/render 14.61, stage/render/prepare_assets 13.87, stage/main/post_update 12.97, render/graph/ms 12.58, stage/render/graph_submit 9.66, encode//sum 8.13 |
| warm-up | 5.43 | 44.3 | render/thread/ms 61.84, render/busy/ms 60.59, main/busy/ms 52.62, stage/main/update 24.52, stage/main/post_update 21.85, stage/render/prepare_assets 20.88, stage/render/render 11.16, bigspace/total/ms 10.03, render/graph/ms 9.37, encode//sum 8.72, stage/render/prepare_flush 8.54, stage/render/graph_submit 6.54 |
| warm-up | 5.48 | 54.4 | pipelines/compile_ms 261, pipelines/front_ms 254, render/thread/ms 86.84, render/busy/ms 85.62, main/busy/ms 65.25, stage/render/prepare_assets 38.30, stage/main/update 37.32, stage/main/post_update 22.19, stage/render/render 11.99, bigspace/total/ms 10.76, render/graph/ms 10.08, encode//sum 9.54 |
| warm-up | 5.52 | 44.7 | pipelines/compile_ms 2349, pipelines/front_ms 2283, render/thread/ms 81.72, render/busy/ms 80.36, pipelines/driver_ms 65.80, main/busy/ms 49.87, stage/render/prepare_assets 48.13, stage/main/update 32.04, stage/render/render 12.86, stage/main/post_update 11.82, render/graph/ms 10.80, encode//sum 8.98 |
| warm-up | 5.56 | 34.2 | pipelines/compile_ms 2393, pipelines/front_ms 2324, pipelines/driver_ms 69.65, render/thread/ms 62.63, render/busy/ms 61.10, main/busy/ms 50.64, stage/render/prepare_assets 30.86, stage/main/update 29.04, stage/render/render 16.43, stage/main/post_update 15.40, render/graph/ms 14.20, stream/drive/ms 12.68 |
| warm-up | 5.69 | 32.3 | pipelines/compile_ms 180, pipelines/front_ms 174, pipelines/wait_ms 98.29, render/thread/ms 45.73, render/busy/ms 43.71, main/busy/ms 31.94, stage/render/prepare_assets 17.78, stage/render/render 13.89, stage/main/update 13.68, stage/main/post_update 11.54, render/graph/ms 10.87, encode//sum 9.95 |
| warm-up | 5.75 | 37.4 | pipelines/compile_ms 925, pipelines/front_ms 899, render/thread/ms 52.33, render/busy/ms 50.99, main/busy/ms 32.86, pipelines/driver_ms 26.45, stage/render/prepare_assets 24.95, stage/main/update 17.66, stage/render/render 11.00, stream/drive/ms 10.88, stage/main/post_update 10.41, render/graph/ms 9.12 |
| warm-up | 5.82 | 48.3 | pipelines/compile_ms 433, pipelines/front_ms 411, render/thread/ms 52.66, render/busy/ms 51.04, main/busy/ms 44.51, stage/render/prepare_assets 27.14, pipelines/driver_ms 22.36, stage/main/update 20.74, stage/main/post_update 17.38, stage/render/render 17.35, render/graph/ms 15.13, stage/render/graph_submit 11.05 |
| warm-up | 6.39 | 30.1 | pipelines/compile_ms 144, pipelines/front_ms 141, render/thread/ms 38.15, render/busy/ms 37.10, main/busy/ms 26.92, stage/render/prepare_assets 14.90, stage/main/update 10.07, stage/main/post_update 9.46, stage/render/render 8.80, render/graph/ms 7.31, encode//sum 6.79, stage/render/graph_submit 5.07 |
| warm-up | 6.59 | 29.6 | pipelines/compile_ms 140, pipelines/front_ms 137, main/busy/ms 38.32, render/thread/ms 38.14, render/busy/ms 37.00, stage/main/update 22.44, stage/main/post_update 12.07, stage/render/render 10.82, stage/render/prepare_assets 10.09, render/graph/ms 9.08, encode//sum 9.07, stream/drive/ms 6.18 |
| warm-up | 6.87 | 39.3 | main/busy/ms 54.79, render/thread/ms 51.14, render/busy/ms 49.48, stage/main/update 32.36, pipelines/compile_ms 24.78, pipelines/front_ms 24.35, stage/render/prepare_assets 23.43, stage/main/post_update 15.29, stream/drive/ms 15.03, stage/render/render 15.00, render/graph/ms 12.24, encode//sum 9.93 |
| warm-up | 6.92 | 53.4 | render/thread/ms 69.74, render/busy/ms 68.36, main/busy/ms 52.74, stage/render/prepare_assets 44.96, stage/main/update 31.77, pipelines/compile_ms 24.78, pipelines/front_ms 24.35, stream/drive/ms 19.18, stage/render/render 13.89, stage/main/post_update 13.85, render/graph/ms 11.79, stage/render/graph_submit 8.74 |
| warm-up | 7.28 | 33.3 | render/thread/ms 38.93, render/busy/ms 37.77, main/busy/ms 18.43, stage/render/render 11.56, stage/main/post_update 10.17, render/graph/ms 9.93, stage/render/specialize 9.47, encode//sum 8.02, stage/render/graph_submit 7.49, stage/render/queue 7.48, stage/main/update 5.51, encode/submit 4.30 |
| warm-up | 8.58 | 31.4 | render/thread/ms 36.64, render/busy/ms 35.63, main/busy/ms 18.86, stage/render/render 10.62, stage/main/post_update 10.31, stage/render/specialize 9.36, render/graph/ms 9.19, encode//sum 7.46, stage/render/graph_submit 6.92, stage/render/queue 6.52, stage/main/update 6.15, encode/submit 3.89 |
| measure | 0.44 | 31.5 | render/thread/ms 39.47, render/busy/ms 38.49, main/busy/ms 18.05, stage/render/render 12.40, stage/main/post_update 11.21, render/graph/ms 11.03, stage/render/specialize 9.62, encode//sum 9.54, stage/render/graph_submit 8.28, stage/render/queue 7.01, encode/pass:shadow_point_light/sum 5.30, encode/submit 4.95 |
| measure | 1.78 | 31.7 | render/thread/ms 36.59, render/busy/ms 35.50, main/busy/ms 20.18, stage/main/post_update 11.31, stage/render/render 10.28, stage/render/specialize 9.27, render/graph/ms 8.81, encode//sum 7.55, stage/render/graph_submit 6.59, stage/render/queue 6.06, stage/main/update 4.99, stage/render/prepare_meshes 4.91 |
| measure | 4.52 | 29.9 | render/thread/ms 36.20, render/busy/ms 35.19, main/busy/ms 19.43, stage/main/post_update 13.17, stage/render/render 11.36, render/graph/ms 9.90, encode//sum 9.30, stage/render/specialize 9.20, stage/render/graph_submit 7.16, stage/render/queue 6.10, encode/submit 4.22, stage/main/update 3.82 |
| measure | 5.88 | 29.4 | render/thread/ms 36.51, render/busy/ms 35.42, main/busy/ms 18.72, stage/main/post_update 12.21, stage/render/render 10.74, encode//sum 10.03, stage/render/specialize 9.69, render/graph/ms 9.10, stage/render/graph_submit 7.07, stage/render/queue 6.42, encode/submit 3.73, stage/main/update 3.69 |
| measure | 7.17 | 29.4 | render/thread/ms 35.37, render/busy/ms 34.32, main/busy/ms 17.85, stage/main/post_update 11.52, stage/render/render 10.52, stage/render/specialize 9.28, render/graph/ms 8.99, encode//sum 7.39, stage/render/graph_submit 6.90, stage/render/queue 6.38, stage/main/update 3.94, encode/submit 3.70 |
| measure | 8.43 | 29.7 | render/thread/ms 36.24, render/busy/ms 35.27, main/busy/ms 19.38, stage/render/render 11.78, stage/main/post_update 10.50, render/graph/ms 10.27, stage/render/specialize 9.34, encode//sum 8.61, stage/render/graph_submit 7.79, stage/render/queue 6.61, encode/submit 4.77, encode/pass:shadow_point_light/sum 4.19 |
| measure | 9.77 | 32.0 | render/thread/ms 41.27, render/busy/ms 40.25, main/busy/ms 20.59, stage/render/render 12.04, stage/main/post_update 11.83, stage/render/specialize 10.85, render/graph/ms 10.54, encode//sum 9.21, stage/render/graph_submit 8.08, stage/render/queue 7.97, stage/main/update 5.72, encode/submit 4.92 |
| measure | 11.09 | 29.1 | render/thread/ms 36.67, render/busy/ms 35.63, main/busy/ms 17.58, stage/render/render 11.56, stage/main/post_update 10.93, render/graph/ms 9.99, stage/render/specialize 9.29, encode//sum 8.16, stage/render/graph_submit 7.69, stage/render/queue 6.30, encode/submit 4.58, stage/main/update 4.08 |
| measure | 15.72 | 31.6 | render/thread/ms 40.33, render/busy/ms 39.31, main/busy/ms 18.92, stage/render/render 12.18, stage/main/post_update 11.84, render/graph/ms 10.75, stage/render/specialize 10.43, encode//sum 8.54, stage/render/graph_submit 8.19, stage/render/queue 8.02, encode/submit 5.18, stage/main/update 4.24 |
| measure | 18.17 | 30.3 | main/busy/ms 39.40, stage/main/update 25.37, stream/drive/ms 21.44, render/thread/ms 21.19, render/busy/ms 20.21, stage/render/render 11.84, stage/main/post_update 11.56, render/graph/ms 10.21, encode//sum 9.94, stage/render/graph_submit 7.53, encode/submit 4.56, encode/pass:shadow_point_light/sum 3.66 |
| measure | 20.75 | 30.3 | render/thread/ms 36.19, render/busy/ms 35.18, main/busy/ms 18.10, stage/main/post_update 11.19, stage/render/render 10.56, stage/render/specialize 10.05, render/graph/ms 9.11, encode//sum 8.80, stage/render/queue 6.82, stage/render/graph_submit 6.76, encode/submit 4.26, stage/main/update 3.84 |
| measure | 23.01 | 39.2 | render/thread/ms 41.62, render/busy/ms 40.42, main/busy/ms 28.44, stage/main/post_update 11.67, stage/render/specialize 11.49, stage/render/render 10.77, render/graph/ms 9.16, encode//sum 8.42, stage/render/queue 8.16, stage/render/graph_submit 7.05, stage/main/update 6.91, stage/main/first 5.13 |
| measure | 24.46 | 30.8 | render/thread/ms 39.03, render/busy/ms 37.73, main/busy/ms 20.33, stage/render/render 12.12, stage/main/post_update 11.82, render/graph/ms 10.24, stage/render/specialize 10.20, encode//sum 9.05, stage/render/graph_submit 7.88, stage/render/queue 7.13, stage/main/update 5.96, encode/submit 4.73 |
| measure | 27.28 | 36.4 | render/thread/ms 44.31, render/busy/ms 43.25, main/busy/ms 27.17, stage/render/render 15.59, render/graph/ms 14.10, stage/main/post_update 13.58, stage/render/specialize 11.56, stage/render/graph_submit 10.78, encode//sum 10.30, encode/pass:shadow_point_light/sum 9.20, stage/render/queue 7.43, encode/submit 6.90 |
| measure | 30.48 | 34.4 | render/thread/ms 41.67, render/busy/ms 40.51, main/busy/ms 23.11, stage/main/post_update 13.59, stage/render/render 11.82, stage/render/specialize 11.49, render/graph/ms 10.14, encode//sum 8.25, stage/render/graph_submit 8.06, stage/render/queue 7.80, stage/main/update 6.70, encode/submit 4.32 |
| measure | 31.92 | 30.9 | render/thread/ms 42.29, render/busy/ms 41.20, main/busy/ms 21.51, stage/main/post_update 13.77, stage/render/render 13.36, render/graph/ms 11.75, stage/render/specialize 10.44, stage/render/graph_submit 9.12, encode//sum 9.11, stage/render/queue 6.94, encode/submit 5.13, stage/main/update 4.63 |
| measure | 35.35 | 34.7 | render/thread/ms 42.46, render/busy/ms 41.30, main/busy/ms 19.98, stage/render/render 13.14, stage/main/post_update 12.19, stage/render/specialize 11.71, render/graph/ms 11.51, stage/render/graph_submit 9.08, encode//sum 8.76, stage/render/queue 7.44, encode/submit 5.18, stage/main/update 4.72 |
| measure | 36.82 | 31.5 | render/thread/ms 40.15, render/busy/ms 39.14, main/busy/ms 19.96, stage/render/render 12.85, stage/main/post_update 12.47, render/graph/ms 11.43, stage/render/specialize 10.44, encode//sum 9.28, stage/render/graph_submit 8.61, stage/render/queue 6.90, encode/pass:shadow_point_light/sum 5.48, encode/submit 5.15 |
| measure | 40.79 | 32.2 | render/thread/ms 38.93, render/busy/ms 37.85, main/busy/ms 19.41, stage/main/post_update 11.13, stage/render/render 10.36, stage/render/specialize 10.01, render/graph/ms 8.86, stage/render/graph_submit 6.93, encode//sum 6.87, stage/render/queue 6.74, stage/main/update 5.14, encode/submit 4.48 |
| measure | 43.21 | 30.4 | render/thread/ms 37.31, render/busy/ms 36.21, main/busy/ms 16.44, stage/render/render 10.66, stage/render/specialize 10.04, stage/main/post_update 9.60, render/graph/ms 9.21, encode//sum 7.46, stage/render/graph_submit 7.33, stage/render/queue 6.86, stage/main/update 4.34, encode/submit 3.83 |
| measure | 45.67 | 34.7 | render/thread/ms 40.68, render/busy/ms 39.62, main/busy/ms 18.86, stage/render/render 11.18, stage/main/post_update 10.79, stage/render/specialize 10.35, render/graph/ms 9.80, stage/render/graph_submit 7.74, encode//sum 7.55, stage/render/queue 6.66, stage/main/update 5.13, stage/render/prepare_meshes 4.67 |
| measure | 56.44 | 29.4 | render/thread/ms 38.19, render/busy/ms 37.09, main/busy/ms 16.44, stage/render/render 11.53, stage/main/post_update 10.13, render/graph/ms 10.09, stage/render/specialize 9.96, stage/render/graph_submit 8.01, encode//sum 7.17, stage/render/queue 6.49, encode/submit 4.89, stage/main/update 4.03 |
| measure | 59.04 | 44.0 | render/thread/ms 55.86, render/busy/ms 54.37, main/busy/ms 31.72, stage/main/post_update 18.49, stage/render/render 17.22, render/graph/ms 15.07, encode//sum 12.54, stage/render/graph_submit 11.79, stage/render/specialize 11.08, stage/main/update 9.08, stage/render/prepare_meshes 8.24, encode/submit 7.24 |
| measure | 59.07 | 32.3 | render/thread/ms 58.24, render/busy/ms 57.12, main/busy/ms 25.84, stage/render/prepare_assets 15.29, stage/main/post_update 12.56, stage/render/specialize 11.07, stage/render/render 10.25, stage/render/prepare_meshes 8.87, render/graph/ms 8.87, encode//sum 7.63, stage/main/update 7.57, stage/render/graph_submit 6.73 |
| measure | 62.58 | 31.3 | render/thread/ms 38.68, render/busy/ms 36.65, main/busy/ms 19.30, stage/render/render 11.49, stage/render/specialize 10.47, stage/main/post_update 10.24, render/graph/ms 9.21, encode//sum 7.78, stage/render/graph_submit 7.05, stage/render/queue 6.68, stage/main/update 6.59, encode/submit 4.68 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=daygate-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 5%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 55%) · 2621 MHz (min 2610) · memory 8101 MHz · 80.7 W · 77 °C (max 83) · VRAM 6800 MiB · throttled 100% of samples

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
