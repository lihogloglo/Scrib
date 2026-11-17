# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: precombine · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5501 frames over 65.7 s · **median 11.67 ms (86 fps)** · mean 11.93 · p95 14.94 · p99 19.39 (1% low 52 fps) · worst 65.20 ms · 77 spikes over 1.5× median · 58.5 stutters/min (work 46.6/min)
- warm-up: 322 frames over 9.2 s, worst 2607.1 ms
- hitches (> 29.2 ms): **29** measured, 35 in warm-up
- timed GPU passes: 6.54 ms of a 11.93 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.67 ms, limit 11 ms
- **FAIL** frame_ms: 64 stutters (> 1.5x moving median of 60) in 1.09 min = 58.49/min, limit 1/min
- **FAIL** worst frame 65.20 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.872 | 2.036 | 2.067 | 0.92 | 0.176 |
| main_opaque_pass_3d | 1.467 | 1.595 | 2.019 | 0.92 | 0.068 |
| early prepass | 0.417 | 0.454 | 0.677 | 0.92 | 0.087 |
| bloom | 0.289 | 0.314 | 0.319 | 0.92 | 0.062 |
| atmosphere_luts | 0.281 | 0.305 | 0.366 | 0.92 | 0.003 |
| water_velocity | 0.257 | 0.279 | 0.347 | 0.92 | 0.002 |
| water_sspr | 0.186 | 0.202 | 0.203 | 0.92 | 0.002 |
| ssao | 0.174 | 0.190 | 0.194 | 0.92 | 0.007 |
| atmos_froxel_inject | 0.134 | 0.146 | 0.148 | 0.92 | 0.001 |
| clouds_trace | 0.129 | 0.141 | 0.232 | 0.92 | 0.001 |
| auxiliary_views | 0.126 | 0.137 | 0.481 | 0.92 | 0.147 |
| water_fft | 0.122 | 0.133 | 0.140 | 0.92 | 0.003 |
| uniform_allocation | 0.119 | 0.130 | 0.181 | 0.92 | 0.026 |
| upscaling | 0.107 | 0.117 | 0.124 | 0.92 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.109 | 0.92 | 0.002 |
| bin_unpacking | 0.097 | 0.105 | 0.144 | 0.92 | 0.019 |
| tonemapping | 0.078 | 0.085 | 0.088 | 0.92 | 0.003 |
| main_transmissive_pass_3d | 0.078 | 0.085 | 0.178 | 0.92 | 0.008 |
| clustering | 0.059 | 0.064 | 0.068 | 0.92 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.92 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.92 | 0.001 |
| surface_exposure | 0.040 | 0.043 | 0.044 | 0.92 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.69 | 0.002 |
| early_mesh_preprocessing | 0.035 | 0.039 | 0.062 | 0.92 | 0.005 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.81 | 0.002 |
| lightprobe_irradiance_map | 0.030 | 0.411 | 0.415 | 0.07 | 0.001 |
| atmos_froxel_integrate | 0.028 | 0.031 | 0.033 | 0.92 | 0.001 |
| clouds_shadow | 0.028 | 0.030 | 0.033 | 0.92 | 0.003 |
| water_snapshot | 0.026 | 0.029 | 0.033 | 0.92 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.024 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.016 | 0.218 | 0.228 | 0.07 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.92 | 0.001 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.025 | 0.92 | 0.121 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.92 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.014 | 0.015 | 0.07 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.07 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.07 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| main/busy/ms | 10.361 | 12.623 | 32.60 | 56995.3 |
| render/thread/ms | 9.664 | 11.948 | 53.77 | 53161.5 |
| render/busy/ms | 9.108 | 11.430 | 53.20 | 50104.0 |
| stage/main/post_update | 6.867 | 8.784 | 13.72 | 37777.7 |
| stage/render/render | 5.534 | 7.014 | 15.45 | 30442.4 |
| render/graph/ms | 4.796 | 6.284 | 14.32 | 26381.6 |
| stage/render/graph_submit | 3.617 | 4.828 | 9.98 | 19895.5 |
| encode//sum | 3.553 | 4.576 | 23.33 | 19546.7 |
| encode/submit | 2.316 | 3.262 | 6.93 | 12739.8 |
| stage/main/update | 2.196 | 2.672 | 23.57 | 12077.9 |
| encode/finish_wall | 1.356 | 2.049 | 4.73 | 7459.8 |
| stage/render/camera_driver | 1.121 | 1.530 | 4.27 | 6169.2 |
| encode/pass:shadow_directional_light/sum | 0.875 | 1.593 | 3.66 | 4813.5 |
| encode/pass:shadow_point_light/sum | 0.873 | 3.224 | 16.63 | 4801.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.792 | 1.089 | 2.04 | 4359.0 |
| encode/pass:early prepass/sum | 0.764 | 1.083 | 1.95 | 4204.9 |
| stage/render/prepare_write_phase_buffers | 0.757 | 1.128 | 2.33 | 4165.6 |
| bigspace/total/ms | 0.593 | 1.362 | 2.43 | 3264.6 |
| stage/render/prepare_assets | 0.574 | 0.740 | 13.33 | 3157.1 |
| stage/render/prepare_meshes | 0.527 | 0.647 | 8.02 | 2900.7 |
| render/present/ms | 0.459 | 0.679 | 2.27 | 2526.5 |
| stage/render/prepare_resources | 0.448 | 0.551 | 1.02 | 2465.0 |
| stage/main/first | 0.400 | 0.528 | 4.97 | 2199.3 |
| stage/render/specialize | 0.381 | 0.432 | 19.89 | 2095.5 |
| stage/main/pre_update | 0.361 | 0.543 | 2.06 | 1988.0 |
| stage/render/queue | 0.348 | 0.488 | 14.43 | 1912.5 |
| terrain/update/ms | 0.302 | 0.390 | 2.08 | 1663.3 |
| stage/render/prepare_bind_groups | 0.282 | 0.503 | 2.49 | 1553.6 |
| controller/feed/ms | 0.270 | 0.341 | 0.84 | 1486.5 |
| terrain/select/ms | 0.268 | 0.348 | 0.92 | 1476.5 |
| stream/drive/ms | 0.253 | 0.348 | 20.89 | 1394.3 |
| stage/main/fixed_main_loop | 0.235 | 0.377 | 1.49 | 1292.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.230 | 0.337 | 0.95 | 1262.5 |
| bigspace/low_precision/ms | 0.229 | 0.788 | 1.72 | 1259.7 |
| stage/render/create_views | 0.201 | 0.261 | 1.05 | 1103.9 |
| far/select/ms | 0.200 | 0.273 | 0.81 | 1100.6 |
| stream/tick/ms | 0.191 | 0.260 | 0.73 | 1053.3 |
| stage/render/prepare_batch_phases | 0.190 | 0.273 | 0.70 | 1045.4 |
| stage/render/extract_commands | 0.174 | 0.203 | 0.66 | 956.2 |
| stage/render/prepare_flush | 0.172 | 0.362 | 2.26 | 945.5 |
| anim/evaluate/ms | 0.164 | 0.278 | 1.47 | 904.0 |
| bigspace/high_precision/ms | 0.163 | 0.690 | 1.86 | 897.2 |
| anim/evaluate_cpu/ms | 0.154 | 0.196 | 0.88 | 846.6 |
| stage/render/prepare_views | 0.141 | 0.174 | 0.81 | 773.8 |
| controller/move/ms | 0.134 | 0.269 | 0.68 | 737.0 |
| stage/main/last | 0.124 | 0.167 | 1.16 | 679.5 |
| render/acquire/ms | 0.097 | 0.123 | 0.53 | 531.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.093 | 0.145 | 0.66 | 512.3 |
| stage/render/cleanup | 0.062 | 0.154 | 1.03 | 342.4 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.126 | 0.58 | 321.9 |
| pipelines/compile_ms | 0.055 | 0.000 | 128.32 | 299.8 |
| pipelines/front_ms | 0.052 | 0.000 | 124.00 | 287.5 |
| physics/step/ms | 0.052 | 0.089 | 0.44 | 284.6 |
| physics/insert/ms | 0.036 | 0.388 | 1.00 | 200.6 |
| encode/resolve_diagnostics/sum | 0.031 | 0.042 | 0.59 | 172.5 |
| stage/render/phase_sort | 0.028 | 0.047 | 0.52 | 152.1 |
| weather/map/ms | 0.025 | 0.033 | 0.22 | 138.6 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.47 | 80.8 |
| anim/apply/ms | 0.012 | 0.020 | 0.60 | 64.6 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.54 | 63.3 |
| controller/avoid/ms | 0.011 | 0.014 | 0.10 | 58.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.25 | 57.0 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 39.2 |
| stream/despawn/ms | 0.006 | 0.000 | 1.09 | 31.6 |
| script/input/ms | 0.005 | 0.007 | 0.28 | 30.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 28.3 |
| script/ui/ms | 0.005 | 0.006 | 0.13 | 25.1 |
| controller/write/ms | 0.003 | 0.004 | 0.23 | 18.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.81 | 14.1 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.32 | 12.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.6 |
| stage/render/prepare | 0.001 | 0.003 | 0.13 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.18 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1272 | 2262 | 583 |
| crowd/c0/meshes/count | count | 349 | 646 | 162 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5613 |
| crowd/c1/meshes/count | count | 1171 | 1295 | 1179 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4619 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4601 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 83.3 k | 91.8 k | 84.3 k |
| encode//max | ms | 0.39 | 1.28 | 0.35 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.47 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.54 | 0.01 |
| encode/finish_critical | ms | 0.97 | 3.66 | 1.07 |
| encode/pass:early prepass/max | ms | 0.78 | 1.95 | 1.07 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 2.04 | 0.68 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.66 | 0.08 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.95 | 0.17 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 3.66 | 0.99 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.57 | 0.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.59 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 10.58 | 53.20 | 8.81 |
| gpu/timed/ms | ms | 7.86 | 11.09 | 8.20 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 11.6 k | 12.8 k | 11.5 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1368 | 1535 | 1424 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 5274 | 7406 | 5167 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1368 | 1535 | 1424 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 5274 | 7406 | 5167 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 189 | 4 |
| phases/shadow/bins | count | 4382 | 8062 | 2594 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 16.8 k | 44.7 k | 12.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 195 | 251 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 181 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 539 | 6 |
| preprocess/bind_groups_created | count | 13 | 539 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 231 | 123 |
| process/cpu_usage | % | 26.92 | 44.23 | 22.44 |
| process/mem_usage | GiB | 5.50 | 5.68 | 5.68 |
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
| skin/blocks_changed/count | count | 14 | 121 | 13 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 365.1 k | 808.4 k | 489.5 k |
| skin/joints_written/count | count | 5704 | 12.6 k | 7648 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 5218 | 5306 | 5306 |
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
| stream/latency/s | s | 0.03 | 0.12 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 742 | 851 | 683 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.88 | 46.38 | 31.48 |
| system/mem_usage | % | 67.04 | 67.55 | 67.48 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 68 | 238 | 54 |
| views/all/bins | count | 7172 | 10.9 k | 6236 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 28.9 k | 59.2 k | 23.6 k |
| views/all/pipelines | count | 241 | 300 | 74 |
| views/all/pipelines_distinct | count | 70 | 75 | 61 |
| views/all/work_items | count | 14.8 k | 57.1 k | 12.1 k |
| views/cascade/batch_sets | count | 9 | 49 | 4 |
| views/cascade/bins | count | 2946 | 5977 | 2594 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 15.0 k | 42.0 k | 12.0 k |
| views/cascade/pipelines | count | 36 | 53 | 28 |
| views/cascade/work_items | count | 691 | 42.0 k | 182 |
| views/lamp_face/batch_sets | count | 0 | 182 | 0 |
| views/lamp_face/bins | count | 1398 | 2169 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1817 | 2845 | 0 |
| views/lamp_face/pipelines | count | 160 | 202 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2125 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 3154 | 3712 | 3642 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 12.0 k | 17.5 k | 11.6 k |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 12.2 k | 17.7 k | 11.9 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 55.0 k | 61.4 k | 55.9 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 65.27 | 299 | 359.1 k |
| anim/events/count | count | 0.10 | 3 | 538 |
| assets/Image/added | count | 0.11 | 38 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.51 | 189 | 2822 |
| assets/Mesh/removed | count | 0.86 | 1160 | 4740 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.92 | 11 | 21.5 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.70 | 1299 | 9344 |
| encode//count | count | 33.46 | 220 | 184.1 k |
| encode/bind_walk/init_textures | count | 12.5 k | 114.7 k | 68.94 M |
| encode/bind_walk/init_textures_skipped | count | 69.2 k | 159.6 k | 380.84 M |
| encode/bind_walk/inits | count | 461 | 1097 | 2.53 M |
| encode/bind_walk/inits_skipped | count | 112 | 166 | 613.4 k |
| encode/bind_walk/merge_textures | count | 12.5 k | 114.6 k | 68.54 M |
| encode/bind_walk/merge_textures_skipped | count | 5377 | 6012 | 29.58 M |
| encode/bind_walk/merges | count | 98.51 | 328 | 541.9 k |
| encode/bind_walk/merges_skipped | count | 85.72 | 129 | 471.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5501 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5501 |
| encode/pass:early prepass/count | count | 1 | 1 | 5501 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5501 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5501 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5501 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5501 |
| encode/pass:shadow_point_light/count | count | 1.37 | 48 | 7520 |
| encode/queue/command_buffers | count | 44.84 | 278 | 246.7 k |
| encode/queue/submits | count | 2.01 | 4 | 11.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.95 M | 6.63 M | 10.72 G |
| encode/queue/write_buffer/calls | count | 260 | 805 | 1.43 M |
| encode/queue/write_buffer_with/bytes | bytes | 172.5 k | 1.30 M | 948.78 M |
| encode/queue/write_buffer_with/calls | count | 10.52 | 576 | 57.9 k |
| encode/queue/write_texture/bytes | bytes | 12.9 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5501 |
| physics/inserts | count | 0.42 | 87 | 2322 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 129 | 2335 |
| physics/steps | count | 0.72 | 3 | 3936 |
| physics/steps_dropped | count | 0.00 | 1 | 3 |
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
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 16 | 351 |
| stream/upload/bytes | bytes | 6154 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.01 | 1 | 28 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1349 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.2 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.61 | 2607.1 | main/busy/ms 2575, stage/main/post_update 2465, stage/main/update 102, surface/advance_cpu_ms 72.87, stage/main/pre_update 4.06, stage/main/last 1.10, surface/cell_upload_ms 0.79, stream/drive/ms 0.55, stage/main/first 0.45, stage/main/fixed_main_loop 0.40, weather/map/ms 0.18, stream/tick/ms 0.12 |
| warm-up | 3.45 | 845.5 | main/busy/ms 2753, stage/main/post_update 2490, render/thread/ms 803, render/busy/ms 664, stage/render/prepare_assets 615, stage/main/update 248, stage/render/prepare_views 135, render/acquire/ms 135, surface/advance_cpu_ms 72.87, stage/render/create_views 28.53, stage/render/render 12.53, render/graph/ms 7.93 |
| warm-up | 3.57 | 119.0 | render/thread/ms 903, render/busy/ms 708, stage/render/prepare_assets 644, main/busy/ms 202, stage/main/update 151, stage/render/prepare_views 136, render/acquire/ms 136, pipelines/compile_ms 127, pipelines/front_ms 77.21, stage/render/render 74.69, render/present/ms 59.89, pipelines/driver_ms 50.24 |
| warm-up | 3.61 | 41.7 | pipelines/compile_ms 477, pipelines/front_ms 392, pipelines/wait_ms 197, render/thread/ms 140, stage/render/render 95.40, pipelines/driver_ms 84.97, render/busy/ms 81.84, main/busy/ms 68.89, render/present/ms 57.74, stage/main/post_update 47.15, render/graph/ms 36.55, pipelines/lock_ms 29.94 |
| warm-up | 3.67 | 60.5 | pipelines/wait_ms 1140, pipelines/compile_ms 676, pipelines/front_ms 627, main/busy/ms 95.16, render/thread/ms 93.09, render/busy/ms 89.94, stage/main/post_update 62.34, pipelines/driver_ms 49.23, stage/render/render 37.28, render/graph/ms 34.17, stage/render/prepare_flush 29.51, pipelines/lock_ms 29.20 |
| warm-up | 3.76 | 85.4 | pipelines/wait_ms 4813, pipelines/compile_ms 1179, pipelines/front_ms 1118, main/busy/ms 166, render/thread/ms 129, render/busy/ms 126, stage/main/update 86.59, pipelines/driver_ms 61.70, pipelines/lock_ms 46.73, stage/main/post_update 42.76, stage/render/prepare_flush 40.21, stage/render/prepare_resources 25.71 |
| warm-up | 3.87 | 109.9 | pipelines/wait_ms 4225, pipelines/compile_ms 1884, pipelines/front_ms 1782, render/thread/ms 142, render/busy/ms 140, main/busy/ms 137, pipelines/driver_ms 101, stage/main/update 89.90, stage/render/prepare_assets 64.57, pipelines/lock_ms 48.24, stage/render/prepare_resources 24.47, stage/main/post_update 21.71 |
| warm-up | 3.91 | 42.4 | pipelines/compile_ms 1629, pipelines/front_ms 1547, pipelines/wait_ms 418, render/thread/ms 106, render/busy/ms 104, encode//sum 94.51, pipelines/driver_ms 81.95, stage/render/prepare_assets 65.33, main/busy/ms 59.78, stage/main/post_update 36.55, stage/render/render 30.79, render/graph/ms 28.11 |
| warm-up | 3.96 | 43.9 | pipelines/compile_ms 1165, pipelines/front_ms 1100, pipelines/wait_ms 224, encode//sum 93.82, render/thread/ms 82.66, render/busy/ms 80.23, pipelines/driver_ms 64.94, main/busy/ms 48.48, stage/render/prepare_assets 45.28, stage/main/post_update 33.72, stage/render/render 27.05, render/graph/ms 24.30 |
| warm-up | 4.10 | 31.1 | pipelines/compile_ms 1236, pipelines/front_ms 1187, pipelines/driver_ms 49.74, main/busy/ms 40.65, stage/main/update 29.38, stream/drive/ms 25.18, render/thread/ms 22.74, render/busy/ms 21.01, stage/render/render 10.57, render/graph/ms 8.59, stage/main/post_update 7.51, stage/render/graph_submit 6.28 |
| warm-up | 4.15 | 43.0 | pipelines/compile_ms 297, pipelines/front_ms 289, main/busy/ms 42.41, render/thread/ms 32.86, render/busy/ms 31.63, stage/main/update 18.60, stage/main/post_update 12.34, stage/render/prepare_assets 11.06, stage/render/render 9.77, render/graph/ms 8.05, pipelines/driver_ms 7.98, pipelines/wait_ms 7.03 |
| warm-up | 4.35 | 53.0 | pipelines/compile_ms 1183, pipelines/front_ms 1141, render/thread/ms 65.12, render/busy/ms 61.72, pipelines/driver_ms 41.53, main/busy/ms 33.94, stage/render/render 32.77, render/graph/ms 29.12, pipelines/wait_ms 17.68, stage/main/update 17.63, stage/render/graph_submit 11.65, stage/render/prepare_assets 10.41 |
| warm-up | 4.39 | 31.3 | pipelines/compile_ms 1461, pipelines/front_ms 1410, render/thread/ms 74.29, render/busy/ms 70.99, pipelines/wait_ms 59.97, pipelines/driver_ms 50.60, main/busy/ms 48.88, stage/render/render 33.97, stage/main/update 33.82, render/graph/ms 30.26, pipelines/lock_ms 28.27, stage/render/graph_submit 11.64 |
| warm-up | 4.45 | 64.9 | pipelines/compile_ms 534, pipelines/front_ms 516, render/thread/ms 75.28, render/busy/ms 73.35, main/busy/ms 46.89, pipelines/wait_ms 43.91, stage/main/update 33.24, stage/render/render 29.35, render/graph/ms 26.89, pipelines/lock_ms 25.46, stage/render/prepare_assets 18.89, pipelines/driver_ms 18.04 |
| warm-up | 4.51 | 60.0 | pipelines/compile_ms 170, pipelines/front_ms 163, render/thread/ms 104, render/busy/ms 101, pipelines/wait_ms 87.04, encode//sum 53.93, stage/render/render 40.70, main/busy/ms 40.54, render/graph/ms 37.98, stage/render/prepare_assets 20.27, stage/render/camera_driver 19.99, stage/render/graph_submit 17.79 |
| warm-up | 4.57 | 55.6 | pipelines/compile_ms 1590, pipelines/front_ms 1535, pipelines/wait_ms 197, render/thread/ms 102, render/busy/ms 99.74, main/busy/ms 69.68, pipelines/driver_ms 55.42, encode//sum 52.51, stage/main/post_update 36.75, stage/render/render 31.42, render/graph/ms 28.99, stage/render/prepare_assets 22.56 |
| warm-up | 4.60 | 29.7 | pipelines/compile_ms 1703, pipelines/front_ms 1645, pipelines/wait_ms 120, render/thread/ms 70.05, render/busy/ms 68.63, pipelines/driver_ms 57.89, main/busy/ms 55.69, stage/main/post_update 33.71, stage/render/prepare_assets 28.69, stage/render/render 20.17, render/graph/ms 17.82, stage/main/update 14.73 |
| warm-up | 4.77 | 30.9 | pipelines/compile_ms 820, pipelines/front_ms 799, pipelines/wait_ms 77.90, render/thread/ms 39.63, render/busy/ms 38.20, main/busy/ms 36.58, stage/render/render 29.99, render/graph/ms 28.11, stage/main/post_update 27.96, pipelines/driver_ms 21.07, stage/render/graph_submit 20.76, stage/render/camera_driver 7.23 |
| warm-up | 4.95 | 60.8 | pipelines/compile_ms 409, pipelines/front_ms 396, render/thread/ms 82.39, render/busy/ms 81.01, main/busy/ms 35.01, stage/render/prepare_resources 20.72, stage/main/post_update 14.10, pipelines/driver_ms 13.73, stage/render/prepare_assets 13.23, stage/render/render 12.04, stage/main/update 10.98, render/graph/ms 10.07 |
| warm-up | 5.05 | 82.4 | pipelines/compile_ms 2285, pipelines/front_ms 2223, main/busy/ms 65.83, pipelines/driver_ms 61.93, stage/main/update 48.48, render/thread/ms 41.97, render/busy/ms 40.43, stage/render/prepare_assets 20.83, stage/render/render 11.21, stream/drive/ms 10.72, stage/main/post_update 9.49, render/graph/ms 9.09 |
| warm-up | 5.08 | 30.3 | pipelines/compile_ms 1874, pipelines/front_ms 1825, render/thread/ms 57.36, render/busy/ms 55.95, pipelines/driver_ms 49.14, main/busy/ms 24.27, stage/render/prepare_assets 20.74, stage/render/render 11.95, encode//sum 10.78, stage/main/post_update 10.34, stage/render/specialize 10.29, render/graph/ms 10.23 |
| warm-up | 5.29 | 32.3 | pipelines/compile_ms 399, pipelines/front_ms 386, render/thread/ms 43.77, render/busy/ms 42.06, main/busy/ms 35.73, stage/render/render 18.78, stage/main/post_update 17.17, render/graph/ms 16.41, pipelines/driver_ms 13.37, stage/main/update 11.39, encode//sum 10.55, stage/render/graph_submit 10.52 |
| warm-up | 5.32 | 37.8 | pipelines/compile_ms 840, pipelines/front_ms 811, render/thread/ms 60.31, render/busy/ms 58.26, main/busy/ms 42.58, pipelines/driver_ms 28.54, stage/render/render 20.61, stage/main/post_update 19.52, render/graph/ms 17.48, stage/render/prepare_assets 16.86, stage/main/update 13.15, encode//sum 11.38 |
| warm-up | 5.37 | 49.1 | pipelines/compile_ms 655, pipelines/front_ms 635, render/thread/ms 60.22, render/busy/ms 58.58, stage/render/prepare_assets 32.00, main/busy/ms 31.13, pipelines/driver_ms 19.75, stage/main/post_update 13.02, stage/render/render 12.45, render/graph/ms 9.94, stage/main/update 9.40, encode//sum 8.49 |
| warm-up | 5.46 | 48.7 | pipelines/compile_ms 449, pipelines/front_ms 437, render/thread/ms 63.39, render/busy/ms 61.75, main/busy/ms 42.67, stage/main/post_update 21.35, stage/main/update 15.53, stage/render/render 14.70, render/graph/ms 12.42, pipelines/driver_ms 12.11, stage/render/prepare_assets 12.02, stage/render/prepare_bind_groups 10.16 |
| warm-up | 5.54 | 46.7 | pipelines/compile_ms 255, pipelines/front_ms 246, render/thread/ms 50.40, render/busy/ms 49.08, stage/render/prepare_assets 27.57, main/busy/ms 24.97, stage/render/render 11.97, stage/main/post_update 11.19, render/graph/ms 10.09, pipelines/driver_ms 8.92, stage/main/update 8.28, encode//sum 7.88 |
| warm-up | 5.73 | 37.0 | render/thread/ms 40.72, main/busy/ms 40.19, render/busy/ms 39.29, pipelines/compile_ms 31.68, pipelines/front_ms 29.82, stage/main/post_update 17.44, stage/main/update 16.29, stage/render/render 15.04, render/graph/ms 12.73, encode//sum 10.52, stage/render/prepare_assets 9.71, stage/render/graph_submit 8.67 |
| warm-up | 5.82 | 35.8 | pipelines/compile_ms 752, pipelines/front_ms 731, main/busy/ms 48.34, stage/main/update 34.81, stream/drive/ms 30.13, render/thread/ms 24.76, render/busy/ms 23.32, pipelines/driver_ms 20.93, stage/render/render 10.08, stage/main/post_update 8.94, encode//sum 8.11, render/graph/ms 8.01 |
| warm-up | 5.89 | 39.6 | pipelines/compile_ms 183, pipelines/front_ms 179, main/busy/ms 43.93, stage/main/update 33.88, stream/drive/ms 29.73, render/thread/ms 23.32, render/busy/ms 21.96, stage/render/render 9.76, render/graph/ms 7.88, encode//sum 7.75, stage/main/post_update 7.25, stage/render/prepare_assets 6.63 |
| warm-up | 5.96 | 37.8 | pipelines/compile_ms 289, pipelines/front_ms 280, main/busy/ms 43.54, stage/main/update 31.46, render/thread/ms 29.97, render/busy/ms 28.49, stream/drive/ms 26.79, stage/render/render 12.19, stage/render/prepare_assets 10.88, render/graph/ms 10.09, stage/main/post_update 8.59, encode//sum 8.51 |
| warm-up | 6.01 | 35.0 | main/busy/ms 38.29, render/thread/ms 28.19, stage/main/update 27.72, render/busy/ms 27.04, stream/drive/ms 18.62, stage/render/render 10.70, render/graph/ms 9.08, encode//sum 7.24, stage/main/post_update 7.12, stage/render/graph_submit 6.87, stage/render/prepare_assets 6.84, encode/submit 4.93 |
| warm-up | 6.06 | 33.1 | pipelines/compile_ms 45.55, pipelines/front_ms 42.14, main/busy/ms 41.00, stage/main/update 29.57, render/thread/ms 24.19, render/busy/ms 22.87, stream/drive/ms 19.43, stage/render/render 11.33, render/graph/ms 9.16, stage/main/post_update 7.38, encode//sum 7.35, stage/render/graph_submit 6.43 |
| warm-up | 6.25 | 32.7 | pipelines/compile_ms 481, pipelines/front_ms 462, render/thread/ms 43.23, main/busy/ms 41.94, render/busy/ms 41.57, stage/main/post_update 20.26, pipelines/driver_ms 18.79, stage/main/update 16.52, stage/render/render 12.69, render/graph/ms 10.59, stage/render/prepare_assets 8.87, stage/render/graph_submit 7.34 |
| warm-up | 6.39 | 31.1 | render/thread/ms 43.18, render/busy/ms 41.87, main/busy/ms 25.22, stage/render/render 13.52, stage/render/prepare_resources 13.08, render/graph/ms 11.60, stage/main/post_update 10.45, stage/main/update 10.23, stage/render/graph_submit 8.59, encode//sum 7.74, encode/submit 6.20, stage/render/prepare_assets 5.29 |
| warm-up | 8.20 | 42.1 | render/thread/ms 48.87, render/busy/ms 47.82, main/busy/ms 23.99, stage/render/specialize 15.03, stage/main/post_update 11.87, stage/render/render 10.67, stage/render/queue 9.94, render/graph/ms 9.22, stage/main/update 8.03, encode//sum 7.24, stage/render/graph_submit 7.18, stage/render/prepare_meshes 5.64 |
| measure | 0.47 | 44.4 | render/thread/ms 48.26, render/busy/ms 47.03, main/busy/ms 19.11, stage/render/specialize 15.61, stage/main/post_update 11.76, stage/render/queue 11.21, stage/render/render 10.37, render/graph/ms 8.83, encode//sum 7.55, stage/render/graph_submit 6.32, stage/main/update 5.01, stage/render/prepare_meshes 4.60 |
| measure | 1.84 | 45.6 | render/thread/ms 48.83, render/busy/ms 47.67, main/busy/ms 19.65, stage/render/specialize 16.46, stage/main/post_update 12.23, stage/render/queue 11.08, stage/render/render 9.95, render/graph/ms 8.54, encode//sum 7.48, stage/render/graph_submit 6.69, stage/main/update 4.93, stage/render/prepare_meshes 4.86 |
| measure | 3.30 | 47.8 | render/thread/ms 56.78, render/busy/ms 55.69, main/busy/ms 22.52, stage/render/specialize 16.76, stage/main/post_update 14.15, stage/render/render 13.16, stage/render/queue 12.19, render/graph/ms 11.56, encode//sum 9.13, stage/render/graph_submit 8.50, stage/render/prepare_meshes 5.64, encode/submit 5.35 |
| measure | 4.75 | 44.8 | render/thread/ms 51.97, render/busy/ms 50.81, main/busy/ms 20.78, stage/render/specialize 16.77, stage/main/post_update 14.14, stage/render/render 12.19, stage/render/queue 10.68, render/graph/ms 10.64, stage/render/graph_submit 7.77, encode//sum 7.77, stage/render/prepare_meshes 5.67, encode/submit 4.93 |
| measure | 6.21 | 49.6 | render/thread/ms 53.82, render/busy/ms 52.65, main/busy/ms 21.32, stage/render/specialize 17.48, stage/main/post_update 13.56, stage/render/queue 11.56, stage/render/render 11.53, render/graph/ms 9.90, encode//sum 8.30, stage/render/graph_submit 7.31, stage/render/prepare_meshes 6.11, stage/main/update 4.90 |
| measure | 7.68 | 47.7 | render/thread/ms 52.21, render/busy/ms 51.28, main/busy/ms 23.57, stage/render/specialize 16.81, stage/main/post_update 15.62, stage/render/render 11.26, stage/render/queue 11.04, render/graph/ms 9.79, encode//sum 7.58, stage/render/graph_submit 7.44, stage/render/prepare_meshes 6.85, stage/main/update 4.42 |
| measure | 9.18 | 48.0 | render/thread/ms 54.41, render/busy/ms 53.32, main/busy/ms 22.35, stage/render/specialize 17.17, stage/main/post_update 14.83, stage/render/queue 12.44, stage/render/render 11.72, render/graph/ms 10.17, stage/render/graph_submit 7.80, encode//sum 7.33, stage/render/prepare_meshes 6.05, encode/submit 4.85 |
| measure | 10.74 | 47.8 | render/thread/ms 54.75, render/busy/ms 53.70, main/busy/ms 24.31, stage/render/specialize 16.96, stage/main/post_update 16.38, stage/render/render 12.42, stage/render/queue 11.74, render/graph/ms 10.89, encode//sum 8.98, stage/render/graph_submit 8.23, stage/render/prepare_meshes 6.63, encode/submit 5.22 |
| measure | 15.75 | 51.6 | render/thread/ms 57.78, render/busy/ms 56.43, main/busy/ms 23.21, stage/render/specialize 18.34, stage/main/post_update 15.82, stage/render/render 12.90, stage/render/queue 12.86, render/graph/ms 11.11, stage/render/graph_submit 8.67, encode//sum 7.97, stage/render/prepare_meshes 6.94, encode/submit 4.72 |
| measure | 17.29 | 46.4 | render/thread/ms 54.07, render/busy/ms 53.03, main/busy/ms 21.76, stage/render/specialize 17.13, stage/main/post_update 14.65, stage/render/queue 12.03, stage/render/render 11.43, render/graph/ms 9.87, encode//sum 7.94, stage/render/graph_submit 7.53, stage/render/prepare_meshes 6.20, encode/submit 4.67 |
| measure | 18.03 | 48.8 | render/thread/ms 53.54, render/busy/ms 52.49, main/busy/ms 21.79, stage/render/specialize 17.42, stage/main/post_update 14.24, stage/render/queue 11.58, stage/render/render 11.40, render/graph/ms 9.97, encode//sum 7.64, stage/render/graph_submit 7.56, stage/render/prepare_meshes 6.24, stage/main/update 4.66 |
| measure | 18.16 | 35.5 | main/busy/ms 43.78, stage/main/update 25.46, render/thread/ms 21.83, stream/drive/ms 21.15, render/busy/ms 20.82, stage/main/post_update 15.12, stage/render/render 11.80, render/graph/ms 10.22, stage/render/graph_submit 7.77, encode//sum 7.30, encode/submit 4.45, encode/finish_wall 3.35 |
| measure | 19.63 | 49.0 | render/thread/ms 53.02, render/busy/ms 51.95, main/busy/ms 22.00, stage/render/specialize 16.98, stage/main/post_update 14.34, stage/render/queue 11.94, stage/render/render 10.20, render/graph/ms 8.76, stage/render/prepare_meshes 6.99, stage/render/graph_submit 6.76, encode//sum 6.54, stage/main/update 4.57 |
| measure | 21.18 | 49.5 | render/thread/ms 51.98, render/busy/ms 51.00, main/busy/ms 22.41, stage/render/specialize 17.88, stage/main/post_update 15.11, stage/render/queue 11.20, stage/render/render 10.10, render/graph/ms 8.67, encode//sum 7.26, stage/render/graph_submit 6.58, stage/render/prepare_meshes 6.35, stage/main/update 4.33 |
| measure | 23.05 | 56.6 | render/thread/ms 58.85, render/busy/ms 57.71, main/busy/ms 29.51, stage/render/specialize 19.79, stage/main/post_update 16.40, stage/render/queue 13.30, stage/render/render 10.84, render/graph/ms 9.25, stage/render/graph_submit 7.23, stage/render/prepare_meshes 6.92, stage/main/update 6.83, encode//sum 6.44 |
| measure | 24.71 | 50.6 | render/thread/ms 55.87, render/busy/ms 54.71, main/busy/ms 24.49, stage/render/specialize 17.51, stage/main/post_update 15.72, stage/render/render 12.69, stage/render/queue 11.66, render/graph/ms 11.18, stage/render/graph_submit 8.39, encode//sum 7.75, stage/render/prepare_meshes 6.87, stage/main/update 6.22 |
| measure | 27.31 | 55.9 | render/thread/ms 62.73, render/busy/ms 61.70, main/busy/ms 29.97, stage/render/specialize 19.17, stage/main/post_update 17.28, stage/render/render 15.33, stage/render/queue 14.04, render/graph/ms 13.80, encode//sum 11.80, stage/render/graph_submit 10.98, encode/pass:shadow_point_light/sum 9.16, stage/render/prepare_meshes 7.13 |
| measure | 28.94 | 50.7 | render/thread/ms 55.37, render/busy/ms 54.11, main/busy/ms 23.70, stage/render/specialize 17.03, stage/main/post_update 16.41, stage/render/queue 12.59, stage/render/render 11.54, render/graph/ms 9.98, stage/render/graph_submit 7.78, stage/render/prepare_meshes 6.85, encode//sum 6.54, stage/main/update 4.71 |
| measure | 30.60 | 51.4 | render/thread/ms 56.88, render/busy/ms 55.79, main/busy/ms 26.18, stage/main/post_update 18.19, stage/render/specialize 17.48, stage/render/render 13.19, stage/render/queue 11.90, render/graph/ms 11.71, encode//sum 8.95, stage/render/graph_submit 8.76, encode/pass:shadow_point_light/sum 8.20, stage/render/prepare_meshes 7.03 |
| measure | 32.30 | 50.8 | render/thread/ms 56.80, render/busy/ms 55.73, main/busy/ms 24.61, stage/render/specialize 18.46, stage/main/post_update 16.29, stage/render/queue 12.28, stage/render/render 11.46, render/graph/ms 9.97, encode//sum 8.61, stage/render/graph_submit 7.73, stage/render/prepare_meshes 7.58, encode/submit 4.96 |
| measure | 35.37 | 57.9 | render/thread/ms 63.12, render/busy/ms 61.81, main/busy/ms 25.01, stage/render/specialize 20.24, stage/main/post_update 17.00, stage/render/queue 14.78, stage/render/render 12.71, render/graph/ms 11.09, stage/render/graph_submit 8.48, encode//sum 8.16, stage/render/prepare_meshes 7.78, encode/submit 5.00 |
| measure | 37.03 | 51.3 | render/thread/ms 55.70, render/busy/ms 54.58, main/busy/ms 23.46, stage/render/specialize 17.53, stage/main/post_update 16.81, stage/render/queue 12.22, stage/render/render 11.82, render/graph/ms 10.34, stage/render/graph_submit 7.60, encode//sum 7.26, stage/render/prepare_meshes 7.15, encode/pass:shadow_point_light/sum 6.73 |
| measure | 37.51 | 50.9 | render/thread/ms 57.80, render/busy/ms 56.84, main/busy/ms 25.74, stage/main/post_update 17.65, stage/render/specialize 17.44, stage/render/render 13.26, render/graph/ms 11.89, stage/render/queue 11.33, encode//sum 10.00, stage/render/graph_submit 9.33, stage/render/prepare_meshes 8.64, encode/submit 5.67 |
| measure | 40.83 | 53.7 | render/thread/ms 55.72, render/busy/ms 54.66, main/busy/ms 25.31, stage/render/specialize 18.26, stage/main/post_update 15.88, stage/render/queue 12.14, stage/render/render 11.00, render/graph/ms 9.56, stage/render/graph_submit 7.53, encode//sum 7.34, stage/render/prepare_meshes 5.94, stage/main/update 5.54 |
| measure | 43.20 | 44.8 | render/thread/ms 52.74, render/busy/ms 51.34, main/busy/ms 22.42, stage/render/specialize 17.91, stage/main/post_update 14.24, stage/render/render 12.01, stage/render/queue 10.95, render/graph/ms 10.16, stage/render/graph_submit 8.41, encode//sum 7.02, stage/render/prepare_meshes 5.23, encode/submit 4.70 |
| measure | 45.66 | 50.8 | render/thread/ms 52.79, render/busy/ms 51.79, main/busy/ms 21.51, stage/render/specialize 17.45, stage/main/post_update 13.44, stage/render/queue 12.39, stage/render/render 10.33, render/graph/ms 9.06, stage/render/graph_submit 7.13, encode//sum 6.63, stage/render/prepare_meshes 5.71, stage/main/update 4.91 |
| measure | 56.45 | 41.1 | render/thread/ms 48.76, render/busy/ms 47.70, main/busy/ms 17.20, stage/render/specialize 15.87, stage/main/post_update 10.74, stage/render/render 10.57, stage/render/queue 10.50, render/graph/ms 9.28, stage/render/graph_submit 7.27, encode//sum 6.89, stage/render/prepare_meshes 5.12, stage/main/update 4.06 |
| measure | 59.05 | 65.2 | render/thread/ms 73.08, render/busy/ms 71.75, main/busy/ms 31.23, stage/main/post_update 18.32, stage/render/prepare_assets 16.86, stage/render/specialize 16.75, stage/render/render 15.25, render/graph/ms 13.60, encode//sum 11.77, stage/render/queue 11.31, stage/render/graph_submit 10.63, stage/main/update 7.56 |
| measure | 62.60 | 48.3 | render/thread/ms 51.56, render/busy/ms 50.52, main/busy/ms 22.27, stage/render/specialize 17.01, stage/main/post_update 12.68, stage/render/render 11.11, stage/render/queue 10.98, render/graph/ms 9.75, stage/render/graph_submit 7.64, encode//sum 7.27, stage/main/update 6.76, stage/render/prepare_meshes 6.31 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=noprecomb-2 --bench-baseline none --off precombine
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 6%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **71%** (min 28%) · 2620 MHz (min 2610) · memory 8101 MHz · 75.3 W · 77 °C (max 82) · VRAM 6761 MiB · throttled 100% of samples

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
