# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6058 frames over 65.6 s · **median 10.65 ms (94 fps)** · mean 10.84 · p95 13.41 · p99 16.73 (1% low 60 fps) · worst 32.21 ms · 69 spikes over 1.5× median · 58.5 stutters/min (work 53.9/min)
- warm-up: 352 frames over 9.5 s, worst 2532.7 ms
- hitches (> 26.6 ms): **4** measured, 39 in warm-up
- timed GPU passes: 6.61 ms of a 10.84 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.65 ms, limit 11 ms
- **FAIL** frame_ms: 64 stutters (> 1.5x moving median of 60) in 1.09 min = 58.49/min, limit 1/min
- pass worst frame 32.21 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.833 | 2.035 | 2.066 | 0.90 | 0.172 |
| main_opaque_pass_3d | 1.431 | 1.589 | 1.952 | 0.90 | 0.073 |
| early prepass | 0.469 | 0.521 | 0.745 | 0.90 | 0.089 |
| bloom | 0.283 | 0.315 | 0.319 | 0.90 | 0.062 |
| atmosphere_luts | 0.275 | 0.305 | 0.365 | 0.90 | 0.003 |
| water_velocity | 0.252 | 0.280 | 0.347 | 0.90 | 0.002 |
| water_sspr | 0.182 | 0.202 | 0.203 | 0.90 | 0.002 |
| ssao | 0.169 | 0.188 | 0.193 | 0.90 | 0.007 |
| auxiliary_views | 0.168 | 0.186 | 0.618 | 0.90 | 0.186 |
| atmos_froxel_inject | 0.131 | 0.146 | 0.147 | 0.90 | 0.001 |
| clouds_trace | 0.127 | 0.141 | 0.233 | 0.90 | 0.001 |
| water_fft | 0.118 | 0.131 | 0.138 | 0.90 | 0.003 |
| uniform_allocation | 0.110 | 0.122 | 0.172 | 0.90 | 0.024 |
| upscaling | 0.105 | 0.117 | 0.125 | 0.90 | 0.004 |
| clouds_composite | 0.095 | 0.106 | 0.109 | 0.90 | 0.002 |
| bin_unpacking | 0.087 | 0.097 | 0.136 | 0.90 | 0.019 |
| gpu_instance_materialization | 0.082 | 0.091 | 0.096 | 0.90 | 0.004 |
| tonemapping | 0.077 | 0.085 | 0.088 | 0.90 | 0.003 |
| main_transmissive_pass_3d | 0.076 | 0.085 | 0.179 | 0.90 | 0.008 |
| clustering | 0.058 | 0.064 | 0.067 | 0.90 | 0.018 |
| early_mesh_preprocessing | 0.050 | 0.056 | 0.089 | 0.90 | 0.004 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.90 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.038 | 0.414 | 0.417 | 0.09 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.64 | 0.002 |
| surface_exposure | 0.036 | 0.039 | 0.040 | 0.90 | 0.001 |
| surface_exposure_far | 0.032 | 0.040 | 0.041 | 0.81 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.90 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.90 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.90 | 0.002 |
| range_unpacking | 0.021 | 0.023 | 0.032 | 0.90 | 0.003 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.90 | 0.002 |
| lightprobe_radiance_map | 0.020 | 0.219 | 0.230 | 0.09 | 0.004 |
| per_view_shadow_pass | 0.016 | 0.018 | 0.037 | 0.90 | 0.006 |
| night_sky | 0.012 | 0.014 | 0.033 | 0.90 | 0.002 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.90 | 0.002 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.90 | 0.003 |
| main_transparent_pass_3d | 0.008 | 0.009 | 0.020 | 0.90 | 0.085 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.595 | 12.104 | 30.01 | 58124.3 |
| render/busy/ms | 9.042 | 11.591 | 29.54 | 54774.4 |
| main/busy/ms | 8.778 | 10.469 | 29.12 | 53176.6 |
| stage/render/render | 5.657 | 7.326 | 18.39 | 34272.9 |
| stage/main/post_update | 5.211 | 6.556 | 10.38 | 31567.5 |
| render/graph/ms | 4.926 | 6.591 | 17.60 | 29840.8 |
| encode//sum | 4.455 | 6.405 | 26.17 | 26987.3 |
| stage/render/graph_submit | 3.693 | 5.028 | 16.35 | 22375.1 |
| encode/submit | 2.364 | 3.395 | 15.29 | 14319.5 |
| stage/main/update | 2.170 | 2.630 | 22.66 | 13142.8 |
| encode/finish_wall | 1.417 | 2.058 | 4.92 | 8581.2 |
| stage/render/camera_driver | 1.171 | 1.628 | 4.12 | 7091.7 |
| encode/pass:shadow_point_light/sum | 1.015 | 3.466 | 16.35 | 6148.0 |
| encode/pass:shadow_directional_light/sum | 0.865 | 1.605 | 2.49 | 5238.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.762 | 1.103 | 1.82 | 4615.6 |
| encode/pass:early prepass/sum | 0.739 | 1.081 | 1.82 | 4476.7 |
| stage/render/prepare_write_phase_buffers | 0.684 | 1.024 | 1.84 | 4142.8 |
| stage/render/prepare_assets | 0.555 | 0.708 | 11.86 | 3361.0 |
| bigspace/total/ms | 0.523 | 1.341 | 2.39 | 3171.2 |
| render/present/ms | 0.459 | 1.222 | 3.42 | 2778.7 |
| stage/render/prepare_resources | 0.451 | 0.541 | 1.05 | 2730.8 |
| stage/render/prepare_meshes | 0.436 | 0.544 | 4.86 | 2638.4 |
| stage/main/first | 0.398 | 0.525 | 6.03 | 2411.1 |
| stage/main/pre_update | 0.356 | 0.506 | 2.20 | 2158.6 |
| stage/render/prepare_bind_groups | 0.337 | 0.581 | 2.11 | 2042.5 |
| terrain/update/ms | 0.314 | 0.411 | 2.40 | 1902.7 |
| stage/render/prepare_flush | 0.290 | 0.503 | 2.88 | 1759.1 |
| controller/feed/ms | 0.282 | 0.364 | 0.70 | 1707.8 |
| terrain/select/ms | 0.280 | 0.369 | 1.04 | 1697.3 |
| stage/render/specialize | 0.271 | 0.377 | 5.64 | 1639.4 |
| stream/drive/ms | 0.252 | 0.335 | 10.95 | 1528.7 |
| stage/main/last | 0.250 | 0.295 | 1.56 | 1516.4 |
| stage/render/prepare_batch_phases | 0.230 | 0.343 | 0.59 | 1391.9 |
| bigspace/low_precision/ms | 0.221 | 0.789 | 2.10 | 1338.1 |
| stage/main/fixed_main_loop | 0.214 | 0.368 | 1.07 | 1297.1 |
| encode/pass:main_transparent_pass_3d/sum | 0.205 | 0.310 | 0.89 | 1244.3 |
| far/select/ms | 0.200 | 0.269 | 0.55 | 1213.5 |
| stream/tick/ms | 0.198 | 0.267 | 0.75 | 1201.6 |
| stage/render/queue | 0.195 | 0.308 | 3.87 | 1181.8 |
| stage/render/create_views | 0.187 | 0.243 | 9.01 | 1135.5 |
| stage/render/extract_commands | 0.183 | 0.218 | 0.61 | 1107.8 |
| anim/evaluate/ms | 0.158 | 0.260 | 1.86 | 959.3 |
| bigspace/high_precision/ms | 0.157 | 0.722 | 1.93 | 953.8 |
| anim/evaluate_cpu/ms | 0.153 | 0.194 | 0.73 | 929.3 |
| stage/render/prepare_views | 0.140 | 0.175 | 0.38 | 847.5 |
| controller/move/ms | 0.122 | 0.241 | 0.53 | 738.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.104 | 0.174 | 0.77 | 632.3 |
| render/acquire/ms | 0.094 | 0.123 | 0.33 | 571.2 |
| stage/render/phase_sort | 0.078 | 0.107 | 0.32 | 474.8 |
| pipelines/compile_ms | 0.063 | 0.000 | 123.10 | 378.6 |
| pipelines/front_ms | 0.060 | 0.000 | 119.34 | 364.2 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.073 | 0.50 | 286.2 |
| physics/step/ms | 0.047 | 0.087 | 0.61 | 284.0 |
| physics/insert/ms | 0.033 | 0.361 | 0.96 | 201.2 |
| encode/resolve_diagnostics/sum | 0.031 | 0.041 | 0.63 | 187.0 |
| weather/map/ms | 0.025 | 0.033 | 0.29 | 151.5 |
| stage/render/cleanup | 0.024 | 0.034 | 0.90 | 145.5 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.24 | 79.8 |
| anim/apply/ms | 0.012 | 0.021 | 0.09 | 71.1 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.36 | 62.8 |
| controller/avoid/ms | 0.010 | 0.014 | 0.22 | 62.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 61.9 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 44.0 |
| script/input/ms | 0.005 | 0.007 | 0.29 | 33.0 |
| stream/despawn/ms | 0.005 | 0.000 | 1.05 | 31.9 |
| script/ui/ms | 0.005 | 0.006 | 0.10 | 27.5 |
| stream/upload/ms | 0.004 | 0.000 | 1.19 | 22.0 |
| stream/spawn/ms | 0.003 | 0.000 | 1.12 | 20.6 |
| controller/write/ms | 0.003 | 0.004 | 0.18 | 20.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 15.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.17 | 14.5 |
| weather/fx/ms | 0.001 | 0.001 | 0.19 | 5.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.20 | 4.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1277 | 2262 | 593 |
| crowd/c0/meshes/count | count | 352 | 646 | 156 |
| crowd/c1/count | count | 145 | 154 | 148 |
| crowd/c1/joints/count | count | 5531 | 5872 | 5603 |
| crowd/c1/meshes/count | count | 1163 | 1230 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 237 |
| crowd/joints_extracted/count | count | 4599 | 8683 | 6494 |
| crowd/joints_uploaded/count | count | 4584 | 6835 | 6494 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 48.5 k | 56.8 k | 47.0 k |
| encode//max | ms | 0.43 | 1.39 | 0.47 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.24 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.36 | 0.01 |
| encode/finish_critical | ms | 0.97 | 2.49 | 0.85 |
| encode/pass:early prepass/max | ms | 0.76 | 1.82 | 0.85 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.82 | 0.65 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.77 | 0.10 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.19 | 0.89 | 0.19 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.49 | 0.85 |
| encode/pass:shadow_point_light/max | ms | 0.78 | 1.59 | 1.02 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.63 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.15 | 29.54 | 8.27 |
| gpu/timed/ms | ms | 8.03 | 9.93 | 8.17 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2599 | 2656 | 2656 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 299.7 k | 299.7 k | 299.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.9 k | 13.1 k | 11.8 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 338 | 588 | 516 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 338 | 588 | 516 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d/bins | count | 764 | 971 | 761 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2516 | 3561 | 2454 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 18 |
| phases/opaque3d_prepass/bins | count | 764 | 971 | 761 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2516 | 3561 | 2454 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 155 | 8 |
| phases/shadow/bins | count | 1938 | 3216 | 1097 |
| phases/shadow/instance_batches | count | 4 | 12 | 4 |
| phases/shadow/instance_draws | count | 6580 | 20.3 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| phases/shadow/items | count | 5974 | 12.6 k | 4391 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 167 | 230 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 322 | 323 | 323 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 18 | 468 | 11 |
| preprocess/bind_groups_created | count | 13 | 463 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 7 | 5 |
| preprocess/bind_groups_reused | count | 127 | 221 | 119 |
| process/cpu_usage | % | 25.96 | 40.38 | 25 |
| process/mem_usage | GiB | 5.52 | 5.69 | 5.69 |
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
| skin/blocks_changed/count | count | 13 | 98 | 7 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 237 |
| skin/joint_bytes_written/bytes | bytes | 370.7 k | 809.0 k | 491.5 k |
| skin/joints_written/count | count | 5792 | 12.6 k | 7680 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 6450 | 6574 | 6574 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.17 | 49.30 | 30.28 |
| system/mem_usage | % | 62.20 | 62.89 | 62.85 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2895 | 3010 | 3010 |
| textures/images | count | 2666 | 2726 | 2726 |
| views/all/batch_sets | count | 81 | 212 | 64 |
| views/all/bins | count | 3878 | 5212 | 3379 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 12 | 20 | 12 |
| views/all/instance_draws | count | 19.7 k | 33.8 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 245.8 k | 147.5 k |
| views/all/items | count | 11.7 k | 19.1 k | 10.3 k |
| views/all/pipelines | count | 214 | 277 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 59 |
| views/all/work_items | count | 154.0 k | 263.8 k | 153.8 k |
| views/cascade/batch_sets | count | 12 | 59 | 8 |
| views/cascade/bins | count | 1518 | 2524 | 1097 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 4 | 12 | 4 |
| views/cascade/instance_draws | count | 6580 | 20.3 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 147.5 k | 49.2 k |
| views/cascade/items | count | 5227 | 11.5 k | 4391 |
| views/cascade/pipelines | count | 38 | 55 | 31 |
| views/cascade/work_items | count | 49.4 k | 158.5 k | 49.3 k |
| views/lamp_face/batch_sets | count | 0 | 144 | 0 |
| views/lamp_face/bins | count | 413 | 765 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 669 | 1112 | 0 |
| views/lamp_face/pipelines | count | 130 | 177 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 722 | 0 |
| views/main/batch_sets | count | 57 | 61 | 56 |
| views/main/bins | count | 2022 | 2318 | 2282 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5952 | 7542 | 5940 |
| views/main/pipelines | count | 46 | 49 | 44 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.5 k | 26.4 k | 18.5 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 58.09 | 241 | 351.9 k |
| anim/events/count | count | 0.08 | 3 | 502 |
| assets/Image/added | count | 0.10 | 42 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.47 | 232 | 2874 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 471 |
| assets/StandardMaterial/modified | count | 3.58 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.25 | 1245 | 1490 |
| encode//count | count | 35.91 | 221 | 217.6 k |
| encode/bind_walk/init_textures | count | 13.3 k | 113.9 k | 80.67 M |
| encode/bind_walk/init_textures_skipped | count | 69.3 k | 161.1 k | 419.85 M |
| encode/bind_walk/inits | count | 458 | 981 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 95.83 | 162 | 580.5 k |
| encode/bind_walk/merge_textures | count | 13.2 k | 113.8 k | 80.23 M |
| encode/bind_walk/merge_textures_skipped | count | 5240 | 5983 | 31.75 M |
| encode/bind_walk/merges | count | 98.45 | 327 | 596.4 k |
| encode/bind_walk/merges_skipped | count | 69.75 | 131 | 422.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6058 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6058 |
| encode/pass:early prepass/count | count | 1 | 1 | 6058 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6058 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6058 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6058 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6058 |
| encode/pass:shadow_point_light/count | count | 1.74 | 48 | 10.5 k |
| encode/queue/command_buffers | count | 47.66 | 279 | 288.7 k |
| encode/queue/submits | count | 2.01 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.71 M | 5.28 M | 16.43 G |
| encode/queue/write_buffer/calls | count | 272 | 705 | 1.65 M |
| encode/queue/write_buffer_with/bytes | bytes | 172.2 k | 1.30 M | 1.04 G |
| encode/queue/write_buffer_with/calls | count | 10.41 | 705 | 63.1 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6058 |
| physics/inserts | count | 0.37 | 109 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.65 | 2 | 3939 |
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
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5588 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 30 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1225 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 43, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2532.7 | main/busy/ms 2503, stage/main/post_update 2398, stage/main/update 95.82, surface/advance_cpu_ms 66.38, stage/main/pre_update 5.58, stage/main/last 1.01, surface/cell_upload_ms 0.78, stream/drive/ms 0.60, stage/main/first 0.59, stage/main/fixed_main_loop 0.36, stream/tick/ms 0.15, weather/map/ms 0.12 |
| warm-up | 3.29 | 756.4 | main/busy/ms 2650, stage/main/post_update 2424, render/thread/ms 723, render/busy/ms 557, stage/render/prepare_assets 501, stage/main/update 210, stage/render/prepare_views 160, render/acquire/ms 160, surface/advance_cpu_ms 66.38, stage/render/create_views 28.69, stage/render/render 20.88, render/graph/ms 14.36 |
| warm-up | 3.47 | 178.5 | render/thread/ms 883, render/busy/ms 625, stage/render/prepare_assets 554, main/busy/ms 176, stage/render/prepare_views 161, render/acquire/ms 160, stage/main/update 121, stage/render/render 119, pipelines/compile_ms 98.53, render/present/ms 97.64, pipelines/driver_ms 79.42, stage/main/post_update 39.45 |
| warm-up | 3.51 | 41.8 | pipelines/compile_ms 428, pipelines/front_ms 214, pipelines/driver_ms 214, render/thread/ms 177, main/busy/ms 136, stage/render/render 107, render/present/ms 95.58, render/busy/ms 81.00, stage/main/update 70.83, stage/render/prepare_assets 53.30, stage/main/post_update 51.00, terrain/update/ms 26.44 |
| warm-up | 3.69 | 182.0 | pipelines/wait_ms 3736, pipelines/compile_ms 2440, pipelines/front_ms 2186, pipelines/driver_ms 254, main/busy/ms 140, render/thread/ms 117, render/busy/ms 112, stage/render/prepare_assets 91.10, stage/main/update 69.90, stage/main/post_update 52.20, stage/render/cleanup 23.77, terrain/update/ms 14.76 |
| warm-up | 3.80 | 104.2 | pipelines/wait_ms 3866, pipelines/compile_ms 2851, pipelines/front_ms 2512, pipelines/driver_ms 340, render/thread/ms 201, render/busy/ms 199, stage/render/prepare_assets 164, main/busy/ms 51.70, stage/render/render 28.18, render/graph/ms 25.49, stage/main/post_update 23.68, stage/render/graph_submit 22.92 |
| warm-up | 4.01 | 132.7 | pipelines/compile_ms 1067, pipelines/front_ms 1011, render/thread/ms 90.98, main/busy/ms 90.90, render/busy/ms 88.71, stage/main/update 63.95, pipelines/driver_ms 55.20, stage/render/prepare_assets 54.91, stage/render/render 26.40, render/graph/ms 23.46, stage/render/graph_submit 20.63, stage/main/post_update 20.13 |
| warm-up | 4.04 | 32.4 | pipelines/compile_ms 1422, pipelines/front_ms 1357, render/thread/ms 88.87, render/busy/ms 86.80, pipelines/driver_ms 64.68, stage/render/prepare_assets 60.14, main/busy/ms 32.10, stage/render/render 21.19, render/graph/ms 18.97, stage/render/graph_submit 16.64, pipelines/wait_ms 13.51, pipelines/lock_ms 12.70 |
| warm-up | 4.09 | 49.7 | pipelines/compile_ms 580, pipelines/front_ms 561, render/thread/ms 56.09, render/busy/ms 53.27, stage/render/prepare_assets 35.80, main/busy/ms 30.55, pipelines/driver_ms 19.22, stage/render/render 12.14, pipelines/wait_ms 11.45, stage/main/post_update 10.96, pipelines/lock_ms 10.96, stage/main/update 9.76 |
| warm-up | 4.24 | 98.5 | pipelines/compile_ms 878, pipelines/front_ms 849, render/thread/ms 80.39, render/busy/ms 78.46, main/busy/ms 60.83, pipelines/driver_ms 28.87, stage/main/update 28.40, stage/render/prepare_assets 27.94, stage/main/post_update 27.64, stream/drive/ms 19.96, stage/render/render 16.61, render/graph/ms 14.05 |
| warm-up | 4.28 | 44.9 | pipelines/compile_ms 2051, pipelines/front_ms 1985, render/thread/ms 103, render/busy/ms 101, main/busy/ms 70.33, pipelines/driver_ms 65.97, stage/main/update 44.05, stage/render/prepare_assets 33.78, stage/render/render 25.52, render/graph/ms 21.81, stage/render/graph_submit 17.22, stage/main/post_update 16.19 |
| warm-up | 4.45 | 169.8 | pipelines/compile_ms 1468, pipelines/front_ms 1419, main/busy/ms 182, stage/main/update 158, render/thread/ms 148, render/busy/ms 145, stage/render/prepare_assets 62.18, pipelines/driver_ms 48.93, stage/render/prepare_resources 32.87, stage/render/render 26.31, render/graph/ms 22.48, stage/render/graph_submit 17.58 |
| warm-up | 4.55 | 100.2 | pipelines/compile_ms 1314, pipelines/front_ms 1264, render/thread/ms 154, render/busy/ms 152, main/busy/ms 152, stage/main/update 131, stage/render/prepare_assets 84.84, pipelines/driver_ms 49.79, stage/render/prepare_resources 31.90, stage/render/render 17.65, render/graph/ms 15.04, stage/render/graph_submit 11.67 |
| warm-up | 4.60 | 46.1 | pipelines/compile_ms 1079, pipelines/front_ms 1038, render/thread/ms 85.20, render/busy/ms 83.66, main/busy/ms 50.95, pipelines/driver_ms 40.34, stage/render/prepare_assets 39.78, stage/main/update 26.14, stage/render/render 25.65, render/graph/ms 23.68, stage/render/graph_submit 20.98, encode/submit 15.04 |
| warm-up | 4.68 | 85.8 | pipelines/compile_ms 504, pipelines/front_ms 490, render/thread/ms 116, render/busy/ms 114, main/busy/ms 68.75, stage/render/prepare_assets 52.61, stage/main/update 45.76, stage/render/render 24.76, render/graph/ms 22.78, stage/render/graph_submit 20.03, stage/render/prepare_resources 17.14, encode/submit 14.48 |
| warm-up | 4.79 | 109.3 | pipelines/compile_ms 2630, pipelines/front_ms 2561, pipelines/wait_ms 548, render/thread/ms 156, render/busy/ms 155, stage/render/prepare_assets 82.64, pipelines/driver_ms 69.77, main/busy/ms 67.28, stage/main/update 40.89, stage/render/prepare_write_phase_buffers 22.49, stage/render/render 14.82, stage/main/fixed_main_loop 13.91 |
| warm-up | 4.87 | 75.0 | pipelines/compile_ms 2755, pipelines/front_ms 2682, pipelines/wait_ms 663, render/thread/ms 133, render/busy/ms 131, pipelines/driver_ms 73.10, main/busy/ms 52.14, stage/render/prepare_assets 47.71, stage/render/prepare_write_phase_buffers 23.04, stage/render/render 22.88, render/graph/ms 20.42, stage/main/update 19.91 |
| warm-up | 4.95 | 80.5 | pipelines/compile_ms 1476, pipelines/front_ms 1429, pipelines/wait_ms 323, render/thread/ms 130, render/busy/ms 128, main/busy/ms 93.04, encode//sum 58.51, stage/main/update 55.13, pipelines/driver_ms 47.02, stage/render/render 35.47, render/graph/ms 32.96, stage/render/prepare_resources 28.75 |
| warm-up | 5.04 | 92.9 | pipelines/compile_ms 2188, pipelines/front_ms 2119, pipelines/wait_ms 491, render/thread/ms 152, render/busy/ms 150, main/busy/ms 101, pipelines/driver_ms 69.38, stage/main/update 61.82, encode//sum 56.76, stage/render/prepare_assets 47.31, stage/render/render 36.76, render/graph/ms 34.13 |
| warm-up | 5.09 | 46.2 | pipelines/compile_ms 1806, pipelines/front_ms 1752, pipelines/wait_ms 284, render/thread/ms 113, render/busy/ms 111, main/busy/ms 61.85, pipelines/driver_ms 54.08, stage/render/prepare_assets 52.10, stage/main/update 31.85, stage/render/render 29.72, render/graph/ms 27.08, stage/render/graph_submit 20.00 |
| warm-up | 5.14 | 49.4 | pipelines/compile_ms 603, pipelines/front_ms 583, main/busy/ms 107, stage/main/update 81.07, render/thread/ms 72.35, render/busy/ms 70.68, stream/drive/ms 60.56, stage/render/prepare_assets 25.82, pipelines/driver_ms 20.68, stage/render/render 19.72, render/graph/ms 17.45, stage/render/graph_submit 13.01 |
| warm-up | 5.22 | 77.8 | pipelines/compile_ms 491, pipelines/front_ms 476, main/busy/ms 94.05, stage/main/update 75.65, render/thread/ms 60.79, render/busy/ms 59.44, stream/drive/ms 50.30, stage/render/prepare_assets 30.24, pipelines/driver_ms 14.98, stage/render/render 13.03, render/graph/ms 10.82, encode//sum 9.64 |
| warm-up | 5.31 | 47.8 | pipelines/compile_ms 650, pipelines/front_ms 628, main/busy/ms 48.42, render/thread/ms 47.61, render/busy/ms 45.72, stage/main/update 31.42, pipelines/driver_ms 21.94, stage/render/render 17.37, render/graph/ms 14.97, stage/render/prepare_assets 14.63, stage/render/graph_submit 11.59, stage/main/post_update 11.08 |
| warm-up | 5.35 | 40.3 | pipelines/compile_ms 439, pipelines/front_ms 425, render/thread/ms 65.31, render/busy/ms 63.89, main/busy/ms 51.42, stage/main/update 32.26, stage/render/prepare_assets 21.10, stage/render/render 13.58, stage/main/post_update 13.47, pipelines/driver_ms 13.22, render/graph/ms 11.65, stage/render/prepare_meshes 11.50 |
| warm-up | 5.40 | 50.2 | pipelines/compile_ms 136, pipelines/front_ms 131, render/thread/ms 77.88, render/busy/ms 76.46, main/busy/ms 58.66, stage/main/update 31.36, stage/render/prepare_assets 19.32, stage/render/render 14.42, stage/render/prepare_meshes 13.03, stage/main/post_update 12.95, stage/render/prepare_resources 12.45, render/graph/ms 12.23 |
| warm-up | 5.43 | 27.5 | pipelines/compile_ms 346, pipelines/front_ms 335, render/thread/ms 64.73, render/busy/ms 63.39, main/busy/ms 40.95, stage/render/prepare_assets 17.21, stage/render/render 15.15, stage/main/update 13.51, stage/main/post_update 13.11, render/graph/ms 12.98, stage/render/prepare_resources 12.91, pipelines/driver_ms 11.33 |
| warm-up | 5.48 | 29.2 | pipelines/compile_ms 653, pipelines/front_ms 633, main/busy/ms 58.73, render/thread/ms 48.10, render/busy/ms 45.87, stage/main/post_update 27.64, stage/main/update 25.91, pipelines/driver_ms 19.63, stage/render/render 18.41, render/graph/ms 15.86, stage/render/prepare_resources 15.30, stage/render/graph_submit 12.54 |
| warm-up | 5.51 | 27.2 | pipelines/compile_ms 870, pipelines/front_ms 846, main/busy/ms 53.03, render/thread/ms 43.57, render/busy/ms 42.22, pipelines/wait_ms 30.45, stage/main/update 25.67, pipelines/driver_ms 24.12, stage/main/post_update 20.09, stage/render/prepare_resources 15.52, stage/render/render 12.06, render/graph/ms 10.14 |
| warm-up | 5.63 | 34.7 | main/busy/ms 46.87, render/thread/ms 40.54, render/busy/ms 39.07, stage/main/update 25.80, stage/render/render 14.87, stage/main/post_update 13.95, stream/drive/ms 13.37, render/graph/ms 12.37, encode//sum 9.15, stage/render/graph_submit 8.02, stage/render/prepare_assets 6.47, encode/submit 5.77 |
| warm-up | 5.66 | 26.6 | pipelines/compile_ms 990, pipelines/front_ms 968, render/thread/ms 44.03, render/busy/ms 42.70, main/busy/ms 33.59, pipelines/driver_ms 21.50, stage/main/update 14.80, stage/render/prepare_assets 14.09, stage/main/post_update 12.37, pipelines/wait_ms 11.38, stage/render/render 11.27, render/graph/ms 9.17 |
| warm-up | 6.27 | 30.9 | render/thread/ms 46.40, render/busy/ms 44.72, main/busy/ms 36.39, stage/render/render 18.17, stage/main/update 16.60, render/graph/ms 15.72, stage/main/post_update 14.32, encode//sum 13.80, stage/render/prepare_assets 11.95, stage/render/graph_submit 11.12, encode/submit 7.27, stage/render/prepare_bind_groups 4.45 |
| warm-up | 6.30 | 26.9 | render/thread/ms 46.08, render/busy/ms 44.66, main/busy/ms 32.29, stage/render/prepare_assets 18.49, stage/main/update 14.60, stage/render/render 13.02, encode//sum 12.60, stage/main/post_update 12.07, render/graph/ms 10.77, stage/render/graph_submit 7.03, stream/drive/ms 4.39, stage/render/prepare_bind_groups 3.82 |
| warm-up | 6.44 | 29.7 | main/busy/ms 41.67, render/thread/ms 38.47, render/busy/ms 37.09, stage/main/update 28.38, stage/render/prepare_assets 12.19, stage/render/render 11.91, render/graph/ms 9.96, stage/main/post_update 8.93, encode//sum 8.10, stage/render/graph_submit 6.99, stream/drive/ms 6.68, encode/submit 4.02 |
| warm-up | 6.56 | 33.8 | pipelines/compile_ms 168, pipelines/front_ms 162, main/busy/ms 45.68, render/thread/ms 43.73, render/busy/ms 42.50, stage/main/update 24.14, stage/main/post_update 16.42, stage/render/render 15.57, encode//sum 14.26, render/graph/ms 13.57, stage/render/prepare_assets 8.73, stage/render/graph_submit 8.62 |
| warm-up | 6.61 | 33.8 | render/thread/ms 48.42, render/busy/ms 47.22, main/busy/ms 40.45, stage/main/update 20.52, pipelines/compile_ms 19.50, pipelines/front_ms 18.96, stage/render/prepare_assets 14.39, stage/render/render 11.98, stage/main/post_update 11.94, render/graph/ms 10.28, encode//sum 9.37, stage/render/graph_submit 6.88 |
| warm-up | 6.66 | 41.5 | render/thread/ms 64.14, render/busy/ms 62.60, main/busy/ms 53.55, stage/main/update 27.17, stage/render/prepare_assets 19.87, stage/main/post_update 18.00, stage/render/render 16.95, render/graph/ms 14.52, pipelines/compile_ms 11.72, pipelines/front_ms 11.47, encode//sum 10.95, stage/render/graph_submit 9.82 |
| warm-up | 6.71 | 30.4 | pipelines/compile_ms 84.38, pipelines/front_ms 78.91, render/thread/ms 37.57, render/busy/ms 36.51, main/busy/ms 30.41, stage/render/prepare_assets 16.48, stage/main/update 16.46, stage/main/post_update 9.57, stage/render/render 9.26, encode//sum 8.09, render/graph/ms 7.73, stream/drive/ms 6.48 |
| warm-up | 6.85 | 45.9 | main/busy/ms 55.40, render/thread/ms 39.46, stage/main/update 39.10, render/busy/ms 38.00, stream/drive/ms 14.40, stage/render/render 14.12, render/graph/ms 11.93, stage/main/post_update 11.85, stage/render/prepare_assets 10.86, encode//sum 9.33, stage/render/graph_submit 8.80, encode/submit 5.87 |
| warm-up | 6.88 | 31.8 | render/thread/ms 46.14, render/busy/ms 44.97, main/busy/ms 44.07, stage/main/update 25.75, stage/render/prepare_assets 19.35, stage/main/post_update 12.75, stage/render/render 12.60, render/graph/ms 10.88, encode//sum 8.98, stage/render/graph_submit 7.31, stream/drive/ms 6.67, stage/render/prepare_meshes 5.60 |
| measure | 8.13 | 31.1 | render/thread/ms 38.64, render/busy/ms 37.58, encode//sum 22.52, stage/render/render 20.08, render/graph/ms 18.51, encode/pass:shadow_point_light/sum 17.81, main/busy/ms 17.68, stage/render/graph_submit 13.15, stage/main/post_update 10.71, stage/render/create_views 9.18, encode/submit 8.21, stage/render/camera_driver 5.22 |
| measure | 18.13 | 32.2 | main/busy/ms 38.64, stage/main/update 24.63, render/thread/ms 22.27, render/busy/ms 21.14, stage/render/render 11.52, stage/main/post_update 10.82, render/graph/ms 9.87, encode//sum 8.49, stage/render/graph_submit 7.75, encode/submit 4.62, stage/render/prepare_assets 3.74, encode/finish_wall 3.20 |
| measure | 23.06 | 27.5 | render/thread/ms 37.87, render/busy/ms 36.75, main/busy/ms 28.91, stage/render/render 15.09, encode//sum 14.85, stage/main/post_update 13.56, render/graph/ms 13.30, stage/main/update 11.12, stage/render/prepare_assets 9.17, stage/render/graph_submit 8.99, encode/pass:shadow_point_light/sum 6.81, encode/submit 5.26 |
| measure | 59.01 | 30.7 | render/thread/ms 37.08, render/busy/ms 36.12, main/busy/ms 23.25, stage/render/prepare_assets 18.08, stage/main/post_update 10.88, stage/render/render 10.24, encode//sum 9.78, render/graph/ms 9.10, stage/render/graph_submit 6.65, stage/main/update 6.38, encode/submit 3.85, encode/finish_wall 2.85 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 11%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **80%** (min 55%) · 2625 MHz (min 2625) · memory 8101 MHz · 81.5 W · 78 °C (max 84) · VRAM 7017 MiB · throttled 100% of samples

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
