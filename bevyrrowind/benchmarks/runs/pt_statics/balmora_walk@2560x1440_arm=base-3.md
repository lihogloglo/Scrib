# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6048 frames over 65.6 s · **median 10.59 ms (94 fps)** · mean 10.85 · p95 13.40 · p99 16.92 (1% low 59 fps) · worst 51.72 ms · 77 spikes over 1.5× median · 58.5 stutters/min (work 55.8/min)
- warm-up: 322 frames over 9.2 s, worst 2592.1 ms
- hitches (> 26.5 ms): **35** measured, 45 in warm-up
- timed GPU passes: 6.56 ms of a 10.85 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.59 ms, limit 11 ms
- **FAIL** frame_ms: 64 stutters (> 1.5x moving median of 60) in 1.09 min = 58.50/min, limit 1/min
- **FAIL** worst frame 51.72 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.861 | 2.037 | 2.070 | 0.91 | 0.177 |
| main_opaque_pass_3d | 1.491 | 1.632 | 2.049 | 0.91 | 0.068 |
| early prepass | 0.448 | 0.491 | 0.718 | 0.91 | 0.088 |
| bloom | 0.287 | 0.314 | 0.321 | 0.91 | 0.063 |
| atmosphere_luts | 0.279 | 0.305 | 0.367 | 0.91 | 0.003 |
| water_velocity | 0.256 | 0.280 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.191 | 0.210 | 0.211 | 0.91 | 0.002 |
| ssao | 0.172 | 0.189 | 0.193 | 0.91 | 0.007 |
| atmos_froxel_inject | 0.133 | 0.145 | 0.147 | 0.91 | 0.001 |
| clouds_trace | 0.128 | 0.140 | 0.232 | 0.91 | 0.001 |
| water_fft | 0.120 | 0.131 | 0.138 | 0.91 | 0.003 |
| uniform_allocation | 0.119 | 0.130 | 0.181 | 0.91 | 0.025 |
| auxiliary_views | 0.112 | 0.122 | 0.487 | 0.91 | 0.124 |
| upscaling | 0.106 | 0.116 | 0.124 | 0.91 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.096 | 0.105 | 0.142 | 0.91 | 0.018 |
| tonemapping | 0.078 | 0.085 | 0.089 | 0.91 | 0.004 |
| main_transmissive_pass_3d | 0.077 | 0.085 | 0.177 | 0.91 | 0.009 |
| clustering | 0.058 | 0.064 | 0.067 | 0.91 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.91 | 0.001 |
| surface_exposure | 0.040 | 0.043 | 0.044 | 0.91 | 0.001 |
| lightprobe_irradiance_map | 0.036 | 0.412 | 0.416 | 0.09 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.64 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.82 | 0.002 |
| early_mesh_preprocessing | 0.028 | 0.031 | 0.047 | 0.91 | 0.005 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.91 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.91 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.91 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.019 | 0.219 | 0.230 | 0.09 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.91 | 0.001 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.91 | 0.124 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.352 | 11.494 | 49.95 | 56563.4 |
| main/busy/ms | 9.045 | 10.869 | 46.12 | 54704.3 |
| render/busy/ms | 8.790 | 10.973 | 49.26 | 53161.9 |
| stage/main/post_update | 5.653 | 7.066 | 42.59 | 34188.3 |
| stage/render/render | 5.467 | 6.894 | 46.39 | 33061.7 |
| render/graph/ms | 4.712 | 6.146 | 45.49 | 28495.6 |
| encode//sum | 3.933 | 5.029 | 255.87 | 23785.8 |
| stage/render/graph_submit | 3.545 | 4.692 | 43.89 | 21439.3 |
| encode/submit | 2.269 | 3.192 | 6.80 | 13724.6 |
| stage/main/update | 2.133 | 2.584 | 22.30 | 12903.0 |
| encode/finish_wall | 1.357 | 1.972 | 37.12 | 8209.6 |
| stage/render/camera_driver | 1.114 | 1.523 | 3.88 | 6735.8 |
| encode/pass:shadow_directional_light/sum | 0.859 | 1.549 | 2.52 | 5194.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.783 | 1.094 | 35.44 | 4735.8 |
| encode/pass:early prepass/sum | 0.753 | 1.085 | 1.97 | 4552.0 |
| stage/render/prepare_write_phase_buffers | 0.736 | 1.074 | 2.35 | 4454.1 |
| encode/pass:shadow_point_light/sum | 0.723 | 2.831 | 16.21 | 4373.5 |
| stage/render/prepare_assets | 0.574 | 0.744 | 9.95 | 3469.5 |
| bigspace/total/ms | 0.552 | 1.339 | 3.27 | 3339.4 |
| render/present/ms | 0.468 | 1.202 | 3.07 | 2829.8 |
| stage/render/prepare_resources | 0.452 | 0.553 | 1.39 | 2732.9 |
| stage/render/prepare_meshes | 0.444 | 0.571 | 7.16 | 2685.3 |
| stage/main/first | 0.397 | 0.520 | 5.88 | 2399.6 |
| stage/main/pre_update | 0.355 | 0.503 | 1.69 | 2148.4 |
| stage/render/specialize | 0.320 | 0.417 | 11.77 | 1934.8 |
| terrain/update/ms | 0.308 | 0.400 | 1.74 | 1860.2 |
| stage/render/queue | 0.288 | 0.420 | 7.66 | 1741.8 |
| pipelines/compile_ms | 0.286 | 0.000 | 162.65 | 1728.4 |
| stage/render/prepare_bind_groups | 0.280 | 0.501 | 2.56 | 1692.0 |
| pipelines/front_ms | 0.277 | 0.000 | 159.38 | 1677.8 |
| terrain/select/ms | 0.274 | 0.356 | 1.09 | 1655.4 |
| controller/feed/ms | 0.266 | 0.337 | 0.80 | 1611.1 |
| stream/drive/ms | 0.245 | 0.335 | 20.28 | 1482.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.242 | 0.356 | 1.07 | 1461.3 |
| bigspace/low_precision/ms | 0.224 | 0.743 | 2.07 | 1357.1 |
| stage/main/fixed_main_loop | 0.215 | 0.371 | 1.09 | 1298.5 |
| far/select/ms | 0.199 | 0.273 | 0.78 | 1204.0 |
| stage/render/create_views | 0.197 | 0.254 | 1.08 | 1191.3 |
| stage/render/prepare_batch_phases | 0.190 | 0.272 | 0.97 | 1147.3 |
| stream/tick/ms | 0.188 | 0.261 | 0.60 | 1138.7 |
| stage/render/extract_commands | 0.172 | 0.204 | 0.42 | 1041.9 |
| stage/render/prepare_flush | 0.167 | 0.324 | 1.11 | 1009.9 |
| anim/evaluate/ms | 0.165 | 0.289 | 1.77 | 1000.0 |
| bigspace/high_precision/ms | 0.162 | 0.705 | 1.84 | 982.1 |
| anim/evaluate_cpu/ms | 0.154 | 0.191 | 0.85 | 930.0 |
| stage/render/prepare_views | 0.138 | 0.174 | 0.45 | 834.7 |
| controller/move/ms | 0.135 | 0.289 | 0.55 | 816.4 |
| stage/main/last | 0.116 | 0.160 | 1.52 | 702.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.163 | 0.70 | 603.0 |
| render/acquire/ms | 0.095 | 0.122 | 0.38 | 571.7 |
| stage/render/prepare_collect_phase_buffers | 0.055 | 0.119 | 0.43 | 334.7 |
| physics/step/ms | 0.047 | 0.087 | 0.76 | 282.7 |
| encode/resolve_diagnostics/sum | 0.037 | 0.043 | 34.61 | 225.7 |
| stage/render/phase_sort | 0.032 | 0.057 | 0.33 | 196.1 |
| physics/insert/ms | 0.032 | 0.350 | 0.87 | 195.2 |
| stage/render/cleanup | 0.030 | 0.085 | 0.79 | 184.2 |
| weather/map/ms | 0.025 | 0.033 | 0.36 | 151.6 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.60 | 88.6 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.52 | 68.9 |
| anim/apply/ms | 0.011 | 0.019 | 0.39 | 67.9 |
| controller/avoid/ms | 0.011 | 0.014 | 0.06 | 64.5 |
| weather/eval/ms | 0.011 | 0.014 | 0.07 | 63.7 |
| pipelines/driver_ms | 0.008 | 0.000 | 4.60 | 50.7 |
| script/update/ms | 0.007 | 0.009 | 0.23 | 43.0 |
| script/input/ms | 0.005 | 0.007 | 0.18 | 32.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 30.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.15 | 29.7 |
| script/ui/ms | 0.005 | 0.006 | 0.04 | 28.3 |
| controller/write/ms | 0.004 | 0.005 | 0.36 | 21.2 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.61 | 14.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.37 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1249 | 2262 | 595 |
| crowd/c0/meshes/count | count | 349 | 646 | 158 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5542 | 5982 | 5601 |
| crowd/c1/meshes/count | count | 1168 | 1284 | 1183 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4618 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 34.71 | 0.36 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.60 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.52 | 0.01 |
| encode/finish_critical | ms | 0.97 | 35.44 | 1.18 |
| encode/pass:early prepass/max | ms | 0.77 | 1.97 | 0.67 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.79 | 35.44 | 1.18 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.70 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.07 | 0.28 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.52 | 1.03 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.75 | 1.03 |
| encode/resolve_diagnostics/max | ms | 0.03 | 34.61 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.23 | 49.26 | 8.78 |
| gpu/timed/ms | ms | 7.88 | 8.90 | 8.30 |
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
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1284 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3406 | 5131 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1284 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3406 | 5131 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 16 | 203 | 4 |
| phases/shadow/bins | count | 5030 | 8823 | 2413 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.3 k | 6366 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 252 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 12 | 559 | 6 |
| preprocess/bind_groups_created | count | 12 | 559 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 223 | 123 |
| process/cpu_usage | % | 26.44 | 37.98 | 20.67 |
| process/mem_usage | GiB | 5.49 | 5.67 | 5.67 |
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
| skin/blocks_changed/count | count | 13 | 128 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 201 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.9 k | 807.2 k | 491.5 k |
| skin/joints_written/count | count | 5780 | 12.6 k | 7680 |
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
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.43 | 63.47 | 29.72 |
| system/mem_usage | % | 62.73 | 65.06 | 63.09 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2900 | 3016 | 3016 |
| textures/images | count | 2680 | 2740 | 2740 |
| views/all/batch_sets | count | 64 | 250 | 54 |
| views/all/bins | count | 7592 | 11.7 k | 5911 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 249 | 299 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 35.8 k | 8121 |
| views/cascade/batch_sets | count | 9 | 51 | 4 |
| views/cascade/bins | count | 2796 | 6195 | 2413 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8904 | 23.5 k | 6366 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 407 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 2152 | 2792 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2412 | 3094 | 0 |
| views/lamp_face/pipelines | count | 168 | 205 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2949 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2870 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8428 | 13.0 k | 7612 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8588 | 13.2 k | 7920 |
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
| anim/bones_written/count | count | 49.81 | 204 | 301.2 k |
| anim/events/count | count | 0.08 | 3 | 513 |
| assets/Image/added | count | 0.10 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.58 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1245 | 4675 |
| encode//count | count | 32.54 | 220 | 196.8 k |
| encode/bind_walk/init_textures | count | 12.0 k | 114.7 k | 72.35 M |
| encode/bind_walk/init_textures_skipped | count | 63.5 k | 141.1 k | 383.97 M |
| encode/bind_walk/inits | count | 453 | 1131 | 2.74 M |
| encode/bind_walk/inits_skipped | count | 107 | 164 | 650.1 k |
| encode/bind_walk/merge_textures | count | 11.9 k | 114.6 k | 71.91 M |
| encode/bind_walk/merge_textures_skipped | count | 5369 | 6012 | 32.47 M |
| encode/bind_walk/merges | count | 97.15 | 338 | 587.6 k |
| encode/bind_walk/merges_skipped | count | 84.13 | 135 | 508.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6048 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6048 |
| encode/pass:early prepass/count | count | 1 | 1 | 6048 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6048 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6048 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6048 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6048 |
| encode/pass:shadow_point_light/count | count | 1.15 | 48 | 6937 |
| encode/queue/command_buffers | count | 43.70 | 278 | 264.3 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.36 G |
| encode/queue/write_buffer/calls | count | 253 | 841 | 1.53 M |
| encode/queue/write_buffer_with/bytes | bytes | 178.1 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 11.00 | 465 | 66.5 k |
| encode/queue/write_texture/bytes | bytes | 11.7 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6048 |
| physics/inserts | count | 0.37 | 110 | 2260 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 129 | 2335 |
| physics/steps | count | 0.65 | 3 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 21 |
| pipelines/created | count | 0.00 | 2 | 25 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5597 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1227 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.5 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2592.1 | main/busy/ms 2561, stage/main/post_update 2454, stage/main/update 97.49, surface/advance_cpu_ms 68.25, stage/main/pre_update 4.95, stage/main/last 1.08, surface/cell_upload_ms 0.77, stage/main/first 0.61, stage/main/fixed_main_loop 0.39, stream/drive/ms 0.34, weather/map/ms 0.12, stream/tick/ms 0.11 |
| warm-up | 3.33 | 739.9 | main/busy/ms 2707, stage/main/post_update 2478, render/thread/ms 708, render/busy/ms 544, stage/render/prepare_assets 495, stage/main/update 214, stage/render/prepare_views 159, render/acquire/ms 158, surface/advance_cpu_ms 68.25, stage/render/create_views 28.50, stage/render/render 16.35, render/graph/ms 9.07 |
| warm-up | 3.53 | 198.4 | render/thread/ms 890, render/busy/ms 611, stage/render/prepare_assets 546, main/busy/ms 176, stage/render/prepare_views 159, render/acquire/ms 159, pipelines/compile_ms 144, stage/render/render 136, stage/main/update 124, render/present/ms 120, pipelines/driver_ms 106, pipelines/front_ms 38.56 |
| warm-up | 3.73 | 171.8 | pipelines/wait_ms 3546, pipelines/compile_ms 2048, pipelines/front_ms 1779, pipelines/driver_ms 269, render/thread/ms 145, render/busy/ms 127, main/busy/ms 113, stage/render/prepare_assets 93.51, stage/main/update 55.63, stage/render/render 39.69, stage/main/post_update 32.49, render/graph/ms 21.65 |
| warm-up | 3.76 | 33.7 | pipelines/wait_ms 3729, pipelines/compile_ms 2285, pipelines/front_ms 2151, render/thread/ms 149, render/busy/ms 140, pipelines/driver_ms 134, stage/render/prepare_assets 115, main/busy/ms 77.62, stage/render/render 24.18, stage/main/post_update 24.04, stage/main/pre_update 23.91, stage/main/fixed_main_loop 18.43 |
| warm-up | 3.81 | 42.8 | pipelines/compile_ms 945, pipelines/front_ms 897, pipelines/wait_ms 234, render/thread/ms 62.53, render/busy/ms 60.73, main/busy/ms 51.41, pipelines/driver_ms 47.85, stage/render/prepare_assets 44.63, stage/main/pre_update 24.29, terrain/update/ms 14.64, stage/main/post_update 14.40, stage/render/render 8.95 |
| warm-up | 3.84 | 33.7 | pipelines/compile_ms 557, pipelines/front_ms 527, pipelines/wait_ms 107, render/thread/ms 63.66, render/busy/ms 61.78, main/busy/ms 40.57, pipelines/driver_ms 29.34, stage/render/prepare_assets 23.25, stage/render/prepare_resources 18.74, encode//sum 17.30, stage/main/update 16.71, stage/main/post_update 16.69 |
| warm-up | 3.91 | 43.1 | pipelines/compile_ms 873, pipelines/front_ms 829, pipelines/wait_ms 82.43, main/busy/ms 79.68, render/thread/ms 64.81, render/busy/ms 62.98, stage/main/update 53.93, stream/drive/ms 45.89, pipelines/driver_ms 43.57, stage/main/post_update 21.71, stage/render/prepare_resources 17.50, stage/render/render 16.23 |
| warm-up | 3.99 | 80.7 | pipelines/compile_ms 1822, pipelines/front_ms 1756, render/thread/ms 91.92, render/busy/ms 90.14, main/busy/ms 79.67, pipelines/driver_ms 66.35, stage/main/update 58.13, stream/drive/ms 47.47, stage/render/prepare_assets 46.07, pipelines/wait_ms 20.50, stage/render/prepare_resources 17.49, stage/main/post_update 17.27 |
| warm-up | 4.05 | 37.7 | pipelines/compile_ms 316, pipelines/front_ms 292, render/thread/ms 48.92, render/busy/ms 46.71, main/busy/ms 33.58, pipelines/driver_ms 24.03, stage/render/prepare_assets 23.30, stage/main/update 20.10, stage/render/render 17.03, render/graph/ms 14.57, stream/drive/ms 10.77, stage/render/graph_submit 10.76 |
| warm-up | 4.15 | 76.6 | pipelines/compile_ms 504, pipelines/front_ms 486, main/busy/ms 63.75, render/thread/ms 61.64, render/busy/ms 59.29, stage/main/update 39.35, stage/render/prepare_assets 35.56, stream/drive/ms 32.90, pipelines/driver_ms 18.38, stage/render/render 16.81, render/graph/ms 14.00, stage/main/post_update 12.96 |
| warm-up | 4.21 | 62.2 | pipelines/compile_ms 490, pipelines/front_ms 470, render/thread/ms 81.94, render/busy/ms 79.85, main/busy/ms 67.89, stage/render/prepare_assets 55.52, stage/main/update 41.13, pipelines/driver_ms 19.78, stage/render/render 16.08, stage/main/post_update 13.51, render/graph/ms 13.18, stage/render/graph_submit 9.35 |
| warm-up | 4.38 | 166.3 | pipelines/compile_ms 1534, pipelines/front_ms 1437, render/thread/ms 176, render/busy/ms 155, pipelines/driver_ms 96.40, main/busy/ms 65.84, stage/render/prepare_assets 64.28, pipelines/wait_ms 64.20, stage/render/render 60.90, stage/main/update 42.51, stage/render/prepare_bind_groups 40.46, render/graph/ms 39.42 |
| warm-up | 4.43 | 48.7 | pipelines/compile_ms 1456, pipelines/front_ms 1355, render/thread/ms 175, render/busy/ms 154, pipelines/driver_ms 101, pipelines/wait_ms 63.41, stage/render/render 60.51, stage/render/prepare_assets 52.18, main/busy/ms 48.91, stage/render/prepare_bind_groups 41.98, render/graph/ms 39.37, stage/main/update 25.37 |
| warm-up | 4.51 | 79.4 | pipelines/compile_ms 494, pipelines/front_ms 468, pipelines/wait_ms 207, render/thread/ms 105, render/busy/ms 103, main/busy/ms 63.56, stage/render/prepare_assets 32.41, stage/render/render 26.70, pipelines/driver_ms 25.88, render/graph/ms 23.95, stage/main/post_update 23.41, stage/main/update 23.19 |
| warm-up | 4.59 | 78.8 | pipelines/compile_ms 2323, pipelines/front_ms 2254, pipelines/wait_ms 700, render/thread/ms 139, render/busy/ms 137, pipelines/driver_ms 69.06, main/busy/ms 60.15, stage/render/prepare_assets 47.18, stage/main/post_update 31.77, stage/render/render 29.71, render/graph/ms 26.82, encode//sum 25.53 |
| warm-up | 4.63 | 46.3 | pipelines/compile_ms 2058, pipelines/front_ms 1998, pipelines/wait_ms 569, render/thread/ms 118, render/busy/ms 116, encode//sum 74.02, pipelines/driver_ms 59.74, main/busy/ms 44.84, stage/render/render 35.01, stage/render/prepare_assets 33.09, render/graph/ms 32.01, stage/main/post_update 22.45 |
| warm-up | 4.67 | 36.5 | pipelines/compile_ms 307, pipelines/front_ms 295, pipelines/wait_ms 75.13, render/thread/ms 70.69, render/busy/ms 68.64, encode//sum 64.32, main/busy/ms 38.51, stage/render/render 31.90, render/graph/ms 29.20, stage/render/graph_submit 21.47, stage/main/update 18.47, stage/render/prepare_assets 13.53 |
| warm-up | 4.71 | 36.8 | pipelines/compile_ms 539, pipelines/front_ms 522, main/busy/ms 90.11, stage/main/update 71.57, stream/drive/ms 58.57, render/thread/ms 54.42, render/busy/ms 52.90, stage/render/prepare_assets 23.93, pipelines/driver_ms 16.50, stage/render/render 15.75, render/graph/ms 13.63, stage/render/graph_submit 10.72 |
| warm-up | 4.77 | 66.0 | pipelines/compile_ms 1787, pipelines/front_ms 1742, pipelines/wait_ms 402, main/busy/ms 104, stage/main/update 80.89, stream/drive/ms 60.02, render/thread/ms 45.57, pipelines/driver_ms 45.10, render/busy/ms 43.76, stage/render/prepare_assets 17.01, stage/render/render 15.89, render/graph/ms 13.32 |
| warm-up | 4.86 | 85.7 | pipelines/compile_ms 1949, pipelines/front_ms 1895, pipelines/wait_ms 404, main/busy/ms 118, stage/main/update 93.74, render/thread/ms 66.70, render/busy/ms 63.65, pipelines/driver_ms 54.27, stream/drive/ms 52.80, stage/render/prepare_assets 26.73, stage/render/render 16.21, render/graph/ms 12.55 |
| warm-up | 5.02 | 161.5 | pipelines/compile_ms 1783, pipelines/front_ms 1728, render/thread/ms 140, render/busy/ms 137, main/busy/ms 120, stage/main/update 89.03, stage/render/prepare_assets 70.99, pipelines/driver_ms 54.50, stream/drive/ms 52.46, stage/render/prepare_resources 26.33, stage/main/post_update 16.80, stage/render/render 16.52 |
| warm-up | 5.07 | 52.6 | pipelines/compile_ms 1423, pipelines/front_ms 1380, render/thread/ms 134, render/busy/ms 133, main/busy/ms 102, stage/main/update 70.33, stage/render/prepare_assets 60.66, stream/drive/ms 48.51, pipelines/driver_ms 42.95, stage/render/render 28.98, render/graph/ms 26.82, stage/render/prepare_resources 26.79 |
| warm-up | 5.14 | 71.5 | pipelines/compile_ms 88.28, main/busy/ms 87.37, pipelines/front_ms 82.99, render/thread/ms 74.00, render/busy/ms 72.47, stage/main/update 67.00, stream/drive/ms 48.04, stage/render/prepare_assets 29.28, stage/render/render 26.69, render/graph/ms 24.58, stage/render/graph_submit 21.53, encode/submit 18.87 |
| warm-up | 5.17 | 30.7 | main/busy/ms 71.74, render/thread/ms 59.07, render/busy/ms 57.35, stage/main/update 54.45, stream/drive/ms 37.00, stage/render/prepare_assets 22.99, stage/render/render 16.20, render/graph/ms 13.83, stage/render/graph_submit 9.75, stage/main/post_update 9.49, encode//sum 8.90, encode/submit 6.88 |
| warm-up | 5.23 | 55.2 | pipelines/compile_ms 617, pipelines/front_ms 599, main/busy/ms 64.88, render/thread/ms 48.68, stage/main/update 47.49, render/busy/ms 47.04, stream/drive/ms 37.03, pipelines/driver_ms 18.42, stage/render/prepare_assets 15.81, stage/render/render 15.48, render/graph/ms 13.30, stage/main/post_update 10.05 |
| warm-up | 5.35 | 40.5 | pipelines/compile_ms 78.96, pipelines/front_ms 74.74, render/thread/ms 58.82, render/busy/ms 57.32, main/busy/ms 35.78, stage/render/prepare_assets 23.37, stage/main/post_update 16.08, stage/main/update 12.91, stage/render/render 12.32, render/graph/ms 10.11, encode//sum 9.02, stage/render/graph_submit 7.11 |
| warm-up | 5.42 | 38.5 | pipelines/compile_ms 162, pipelines/front_ms 155, render/thread/ms 55.03, render/busy/ms 52.48, main/busy/ms 43.27, stage/render/prepare_assets 24.87, stage/main/update 21.30, stage/main/post_update 16.05, stage/render/render 14.75, render/graph/ms 11.61, stage/render/graph_submit 8.38, encode//sum 8.18 |
| warm-up | 5.45 | 34.3 | pipelines/compile_ms 324, pipelines/front_ms 312, render/thread/ms 60.30, render/busy/ms 57.48, main/busy/ms 40.38, stage/render/prepare_assets 23.87, stage/main/update 17.28, stage/main/post_update 16.75, stage/render/render 15.67, pipelines/driver_ms 11.96, render/graph/ms 11.87, stage/render/graph_submit 8.01 |
| warm-up | 5.49 | 41.2 | pipelines/compile_ms 709, pipelines/front_ms 686, render/thread/ms 63.16, render/busy/ms 61.46, main/busy/ms 46.43, stage/render/prepare_assets 31.72, stage/main/post_update 25.07, pipelines/driver_ms 22.95, stage/main/update 14.12, stage/render/render 14.03, render/graph/ms 11.12, bigspace/total/ms 9.89 |
| warm-up | 5.52 | 31.4 | pipelines/compile_ms 811, pipelines/front_ms 790, render/thread/ms 55.63, render/busy/ms 54.24, main/busy/ms 52.64, stage/render/prepare_assets 30.57, stage/main/update 22.88, stage/main/post_update 22.12, pipelines/driver_ms 20.90, stream/drive/ms 14.61, stage/render/render 11.76, bigspace/total/ms 9.73 |
| warm-up | 5.57 | 45.2 | pipelines/compile_ms 418, pipelines/front_ms 409, render/thread/ms 61.73, render/busy/ms 60.23, main/busy/ms 55.98, stage/render/prepare_assets 34.54, stage/main/post_update 28.73, stage/main/update 20.16, stream/drive/ms 13.39, stage/render/render 11.96, pipelines/driver_ms 9.62, render/graph/ms 9.57 |
| warm-up | 5.64 | 51.1 | pipelines/compile_ms 218, pipelines/front_ms 213, render/thread/ms 58.64, render/busy/ms 56.66, main/busy/ms 41.62, stage/render/prepare_assets 29.20, stage/main/post_update 25.27, stage/render/render 15.23, render/graph/ms 12.18, stage/main/update 11.38, encode//sum 10.61, stage/render/graph_submit 8.63 |
| warm-up | 6.24 | 29.6 | render/thread/ms 40.85, render/busy/ms 39.86, main/busy/ms 28.25, stage/render/prepare_assets 19.01, stage/main/update 12.93, stage/main/post_update 9.16, stage/render/render 8.06, encode//sum 6.77, render/graph/ms 6.68, stream/drive/ms 4.75, stage/render/graph_submit 4.68, stage/render/specialize 3.79 |
| warm-up | 6.74 | 37.1 | pipelines/compile_ms 144, pipelines/front_ms 139, render/thread/ms 43.79, render/busy/ms 42.47, main/busy/ms 39.01, stage/main/update 24.34, stage/render/prepare_assets 22.79, stream/drive/ms 17.24, stage/render/render 11.53, stage/main/post_update 9.87, render/graph/ms 9.59, encode//sum 9.05 |
| warm-up | 7.09 | 31.5 | render/thread/ms 37.48, render/busy/ms 36.33, main/busy/ms 16.97, stage/render/render 10.86, stage/render/specialize 9.84, stage/main/post_update 9.53, render/graph/ms 9.25, stage/render/graph_submit 7.25, encode//sum 6.54, stage/render/queue 6.39, stage/main/update 5.11, encode/submit 4.09 |
| warm-up | 8.35 | 31.5 | render/thread/ms 36.63, render/busy/ms 35.44, main/busy/ms 18.98, stage/render/render 10.60, stage/main/post_update 10.31, stage/render/specialize 9.87, render/graph/ms 9.15, encode//sum 7.78, stage/render/graph_submit 6.98, stage/render/queue 6.35, stage/main/update 5.78, encode/submit 3.81 |
| measure | 0.44 | 31.5 | render/thread/ms 36.54, render/busy/ms 35.46, main/busy/ms 17.40, stage/render/render 10.97, stage/main/post_update 10.89, render/graph/ms 9.43, stage/render/specialize 9.34, encode//sum 8.36, stage/render/graph_submit 7.05, stage/render/queue 6.10, encode/submit 4.08, stage/main/update 3.93 |
| measure | 0.76 | 29.7 | main/busy/ms 35.53, render/thread/ms 29.20, render/busy/ms 27.90, stage/main/update 18.26, pipelines/compile_ms 15.44, pipelines/front_ms 14.88, stage/render/render 14.08, stage/main/post_update 14.06, stream/drive/ms 13.53, render/graph/ms 12.38, encode//sum 11.15, stage/render/graph_submit 9.44 |
| measure | 1.88 | 31.8 | render/thread/ms 39.63, render/busy/ms 38.64, main/busy/ms 18.59, stage/render/render 13.09, render/graph/ms 11.60, stage/main/post_update 10.99, encode//sum 9.34, stage/render/specialize 9.24, stage/render/graph_submit 8.71, stage/render/queue 6.18, encode/pass:shadow_point_light/sum 5.51, encode/submit 5.17 |
| measure | 2.78 | 51.7 | encode//sum 260, render/thread/ms 59.24, render/busy/ms 58.07, main/busy/ms 56.55, stage/render/render 51.15, stage/main/post_update 49.62, render/graph/ms 49.51, stage/render/graph_submit 46.41, encode/finish_wall 38.32, encode/pass:main_opaque_pass_3d/sum 35.88, encode/resolve_diagnostics/sum 34.64, encode/submit 8.12 |
| measure | 3.40 | 31.0 | render/thread/ms 37.46, render/busy/ms 36.32, main/busy/ms 20.01, stage/main/post_update 12.59, stage/render/render 12.28, render/graph/ms 10.71, stage/render/specialize 9.80, encode//sum 9.33, stage/render/graph_submit 8.19, stage/render/queue 5.83, encode/submit 4.94, stage/main/update 4.86 |
| measure | 4.92 | 36.0 | render/thread/ms 36.11, render/busy/ms 35.15, main/busy/ms 23.36, stage/main/post_update 13.45, stage/render/render 10.48, stage/render/specialize 10.14, render/graph/ms 9.15, encode//sum 7.80, stage/render/graph_submit 7.31, stage/render/queue 6.67, stage/main/update 5.58, encode/submit 4.00 |
| measure | 7.53 | 29.9 | render/thread/ms 34.22, render/busy/ms 33.20, main/busy/ms 18.59, stage/main/post_update 11.91, stage/render/render 10.25, stage/render/specialize 9.15, render/graph/ms 8.89, encode//sum 7.48, stage/render/graph_submit 6.65, stage/render/queue 6.18, stage/main/update 4.07, encode/submit 3.56 |
| measure | 15.72 | 31.8 | render/thread/ms 38.77, render/busy/ms 37.73, main/busy/ms 19.87, stage/main/post_update 12.10, stage/render/render 11.52, stage/render/specialize 10.47, render/graph/ms 10.07, stage/render/graph_submit 7.79, encode//sum 7.49, stage/render/queue 7.20, encode/submit 4.61, stage/main/update 4.16 |
| measure | 17.00 | 29.9 | render/thread/ms 35.39, render/busy/ms 34.36, main/busy/ms 17.93, stage/main/post_update 11.31, stage/render/render 10.42, stage/render/specialize 9.90, render/graph/ms 8.99, encode//sum 7.51, stage/render/graph_submit 7.08, stage/render/queue 6.63, stage/main/update 4.12, encode/submit 4.07 |
| measure | 18.01 | 30.6 | render/thread/ms 37.62, render/busy/ms 36.64, main/busy/ms 17.47, stage/main/post_update 10.97, stage/render/render 10.86, stage/render/specialize 10.28, render/graph/ms 9.39, stage/render/graph_submit 7.33, encode//sum 7.32, stage/render/queue 6.83, encode/submit 4.14, stage/main/update 3.84 |
| measure | 18.16 | 31.9 | main/busy/ms 38.43, stage/main/update 24.29, stream/drive/ms 20.51, render/thread/ms 18.69, render/busy/ms 17.70, stage/main/post_update 11.04, stage/render/render 10.38, render/graph/ms 8.97, encode//sum 7.38, stage/render/graph_submit 6.70, encode/submit 4.04, encode/finish_wall 2.72 |
| measure | 19.37 | 29.8 | render/thread/ms 35.85, render/busy/ms 34.74, main/busy/ms 17.85, stage/main/post_update 11.22, stage/render/render 10.20, stage/render/specialize 9.91, render/graph/ms 8.58, encode//sum 8.11, stage/render/graph_submit 6.55, stage/render/queue 6.42, stage/main/update 3.79, encode/submit 3.76 |
| measure | 20.72 | 31.2 | render/thread/ms 37.35, render/busy/ms 36.08, main/busy/ms 19.68, stage/main/post_update 12.27, stage/render/render 11.68, render/graph/ms 10.02, stage/render/specialize 9.76, encode//sum 8.18, stage/render/graph_submit 7.43, stage/render/queue 6.91, stage/main/update 4.51, encode/submit 4.34 |
| measure | 23.01 | 34.6 | render/thread/ms 40.90, render/busy/ms 39.77, main/busy/ms 22.74, stage/main/post_update 12.71, stage/render/specialize 11.74, stage/render/render 10.97, render/graph/ms 9.43, encode//sum 8.77, stage/render/queue 8.05, stage/render/graph_submit 7.27, stage/main/update 6.29, encode/submit 4.14 |
| measure | 24.53 | 29.8 | render/thread/ms 34.84, render/busy/ms 33.76, main/busy/ms 18.89, stage/main/post_update 11.79, stage/render/render 9.79, stage/render/specialize 9.78, render/graph/ms 8.29, encode//sum 7.60, stage/render/queue 6.67, stage/render/graph_submit 6.28, stage/main/update 4.31, encode/submit 3.69 |
| measure | 27.28 | 34.8 | render/thread/ms 46.11, render/busy/ms 44.92, main/busy/ms 26.93, stage/render/render 15.38, render/graph/ms 13.70, stage/main/post_update 13.12, stage/render/specialize 12.08, encode//sum 11.76, stage/render/graph_submit 10.78, stage/render/queue 7.81, encode/submit 6.73, stage/main/update 6.73 |
| measure | 30.45 | 32.5 | render/thread/ms 37.41, render/busy/ms 36.35, main/busy/ms 21.19, stage/main/post_update 12.58, stage/render/specialize 10.71, stage/render/render 10.24, render/graph/ms 8.80, stage/render/queue 7.25, encode//sum 7.21, stage/render/graph_submit 7.00, stage/main/update 5.51, encode/submit 3.77 |
| measure | 31.90 | 30.9 | render/thread/ms 37.96, render/busy/ms 36.82, main/busy/ms 20.99, stage/main/post_update 13.27, stage/render/render 11.37, stage/render/specialize 9.83, render/graph/ms 9.82, stage/render/graph_submit 7.81, encode//sum 7.79, stage/render/queue 7.22, stage/main/update 4.82, encode/submit 4.60 |
| measure | 35.35 | 32.3 | render/thread/ms 39.73, render/busy/ms 38.54, main/busy/ms 19.29, stage/render/render 11.99, stage/main/post_update 11.53, stage/render/specialize 10.46, render/graph/ms 10.45, encode//sum 8.20, stage/render/graph_submit 7.64, stage/render/queue 6.90, stage/main/update 4.68, encode/submit 4.16 |
| measure | 36.71 | 32.8 | render/thread/ms 39.76, render/busy/ms 38.62, main/busy/ms 19.36, stage/main/post_update 12.47, stage/render/render 12.09, render/graph/ms 10.58, stage/render/specialize 10.19, encode//sum 8.80, stage/render/graph_submit 8.24, stage/render/queue 7.41, encode/submit 4.71, stage/main/update 4.14 |
| measure | 37.48 | 30.3 | render/thread/ms 38.22, render/busy/ms 37.14, main/busy/ms 17.79, stage/render/render 12.06, stage/main/post_update 11.26, render/graph/ms 10.55, stage/render/specialize 9.92, encode//sum 8.66, stage/render/graph_submit 8.14, stage/render/queue 6.61, encode/submit 4.80, stage/main/update 4.05 |
| measure | 40.78 | 32.2 | render/thread/ms 38.92, render/busy/ms 37.86, main/busy/ms 18.35, stage/render/render 10.58, stage/main/post_update 10.34, stage/render/specialize 9.84, render/graph/ms 9.11, encode//sum 7.93, stage/render/graph_submit 7.08, stage/render/queue 6.46, stage/main/update 4.48, encode/submit 3.86 |
| measure | 43.23 | 31.3 | render/thread/ms 36.84, render/busy/ms 35.73, main/busy/ms 17.25, stage/render/render 10.28, stage/main/post_update 10.02, stage/render/specialize 9.78, render/graph/ms 8.83, encode//sum 7.08, stage/render/graph_submit 6.98, stage/render/queue 6.85, encode/submit 4.04, stage/main/update 3.97 |
| measure | 45.67 | 31.7 | render/thread/ms 40.62, render/busy/ms 39.58, main/busy/ms 17.31, stage/render/render 11.69, stage/render/specialize 10.62, render/graph/ms 10.34, stage/main/post_update 9.92, stage/render/graph_submit 8.31, encode//sum 7.58, stage/render/queue 6.45, encode/finish_wall 4.63, stage/render/prepare_meshes 4.40 |
| measure | 56.44 | 30.1 | render/thread/ms 35.95, render/busy/ms 34.69, main/busy/ms 15.46, stage/render/render 10.19, stage/render/specialize 9.26, stage/main/post_update 8.67, render/graph/ms 8.59, encode//sum 8.38, stage/render/queue 6.69, stage/render/graph_submit 6.46, stage/main/update 4.11, stage/render/prepare_meshes 3.90 |
| measure | 59.04 | 46.0 | render/thread/ms 51.46, render/busy/ms 49.81, main/busy/ms 31.52, stage/main/post_update 18.59, stage/render/render 14.58, render/graph/ms 12.52, encode//sum 11.44, stage/render/specialize 10.95, stage/render/graph_submit 9.27, stage/main/update 8.51, stage/render/prepare_meshes 7.66, stage/render/queue 6.26 |
| measure | 62.57 | 31.1 | render/thread/ms 39.01, render/busy/ms 38.01, main/busy/ms 18.56, stage/render/render 11.89, stage/render/specialize 11.12, render/graph/ms 10.63, stage/main/post_update 10.10, stage/render/graph_submit 8.57, encode//sum 8.39, stage/render/queue 6.27, stage/main/update 5.77, encode/submit 4.41 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 10%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **78%** (min 45%) · 2620 MHz (min 2610) · memory 8101 MHz · 81.0 W · 78 °C (max 84) · VRAM 7019 MiB · throttled 100% of samples

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
