# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5978 frames over 65.6 s · **median 10.69 ms (94 fps)** · mean 10.98 · p95 13.63 · p99 17.03 (1% low 59 fps) · worst 48.78 ms · 72 spikes over 1.5× median · 59.4 stutters/min (work 50.3/min)
- warm-up: 323 frames over 9.4 s, worst 2645.5 ms
- hitches (> 26.7 ms): **33** measured, 47 in warm-up
- timed GPU passes: 6.69 ms of a 10.98 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.69 ms, limit 11 ms
- **FAIL** frame_ms: 65 stutters (> 1.5x moving median of 60) in 1.09 min = 59.41/min, limit 1/min
- **FAIL** worst frame 48.78 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.887 | 2.033 | 2.065 | 0.93 | 0.174 |
| main_opaque_pass_3d | 1.511 | 1.627 | 2.038 | 0.93 | 0.067 |
| early prepass | 0.456 | 0.491 | 0.714 | 0.93 | 0.087 |
| bloom | 0.292 | 0.314 | 0.319 | 0.93 | 0.061 |
| atmosphere_luts | 0.281 | 0.302 | 0.365 | 0.93 | 0.003 |
| water_velocity | 0.259 | 0.279 | 0.347 | 0.93 | 0.002 |
| water_sspr | 0.187 | 0.201 | 0.202 | 0.93 | 0.002 |
| ssao | 0.174 | 0.188 | 0.191 | 0.93 | 0.007 |
| auxiliary_views | 0.152 | 0.164 | 0.593 | 0.93 | 0.162 |
| atmos_froxel_inject | 0.135 | 0.145 | 0.147 | 0.93 | 0.001 |
| clouds_trace | 0.130 | 0.141 | 0.231 | 0.93 | 0.001 |
| uniform_allocation | 0.121 | 0.130 | 0.181 | 0.93 | 0.025 |
| water_fft | 0.121 | 0.130 | 0.137 | 0.93 | 0.003 |
| upscaling | 0.111 | 0.119 | 0.126 | 0.93 | 0.004 |
| clouds_composite | 0.098 | 0.106 | 0.108 | 0.93 | 0.002 |
| bin_unpacking | 0.097 | 0.104 | 0.141 | 0.93 | 0.018 |
| main_transmissive_pass_3d | 0.079 | 0.085 | 0.177 | 0.93 | 0.008 |
| tonemapping | 0.079 | 0.085 | 0.088 | 0.93 | 0.003 |
| clustering | 0.059 | 0.064 | 0.067 | 0.93 | 0.018 |
| render_sky | 0.048 | 0.052 | 0.052 | 0.93 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.93 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.93 | 0.001 |
| lightprobe_irradiance_map | 0.039 | 0.411 | 0.415 | 0.09 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.66 | 0.002 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.93 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.93 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.93 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.024 | 0.93 | 0.002 |
| lightprobe_radiance_map | 0.021 | 0.218 | 0.229 | 0.09 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.93 | 0.001 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.026 | 0.93 | 0.124 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.93 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.507 | 11.964 | 39.57 | 56833.2 |
| main/busy/ms | 9.063 | 10.821 | 34.20 | 54175.8 |
| render/busy/ms | 8.946 | 11.453 | 39.02 | 53477.9 |
| stage/main/post_update | 5.659 | 7.062 | 30.79 | 33827.6 |
| stage/render/render | 5.594 | 7.222 | 31.21 | 33442.5 |
| render/graph/ms | 4.840 | 6.475 | 30.57 | 28934.9 |
| encode//sum | 4.204 | 5.817 | 28.52 | 25131.8 |
| stage/render/graph_submit | 3.644 | 4.899 | 29.32 | 21786.1 |
| encode/submit | 2.329 | 3.289 | 6.92 | 13921.6 |
| stage/main/update | 2.141 | 2.564 | 22.02 | 12798.8 |
| encode/finish_wall | 1.399 | 2.063 | 25.69 | 8362.2 |
| stage/render/camera_driver | 1.143 | 1.606 | 4.05 | 6830.3 |
| encode/pass:shadow_point_light/sum | 0.947 | 3.374 | 16.88 | 5663.6 |
| encode/pass:shadow_directional_light/sum | 0.857 | 1.596 | 2.85 | 5125.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.760 | 1.087 | 1.81 | 4545.2 |
| stage/render/prepare_write_phase_buffers | 0.759 | 1.114 | 2.51 | 4539.4 |
| encode/pass:early prepass/sum | 0.738 | 1.064 | 24.77 | 4410.9 |
| stage/render/prepare_assets | 0.568 | 0.735 | 14.94 | 3397.6 |
| bigspace/total/ms | 0.550 | 1.351 | 2.32 | 3286.3 |
| render/present/ms | 0.468 | 1.212 | 3.22 | 2795.0 |
| stage/render/prepare_resources | 0.453 | 0.546 | 1.85 | 2706.5 |
| stage/render/prepare_meshes | 0.440 | 0.564 | 5.90 | 2629.2 |
| stage/main/first | 0.398 | 0.520 | 5.19 | 2378.1 |
| stage/main/pre_update | 0.357 | 0.518 | 2.89 | 2134.0 |
| stage/render/specialize | 0.325 | 0.419 | 14.61 | 1940.0 |
| terrain/update/ms | 0.315 | 0.414 | 2.22 | 1885.9 |
| stage/render/queue | 0.294 | 0.420 | 7.82 | 1758.8 |
| stage/render/prepare_bind_groups | 0.284 | 0.501 | 2.84 | 1696.7 |
| terrain/select/ms | 0.281 | 0.371 | 1.05 | 1677.6 |
| controller/feed/ms | 0.274 | 0.341 | 1.60 | 1637.9 |
| stream/drive/ms | 0.251 | 0.332 | 20.05 | 1501.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.241 | 0.356 | 1.16 | 1438.7 |
| bigspace/low_precision/ms | 0.226 | 0.779 | 1.80 | 1353.0 |
| stage/main/fixed_main_loop | 0.217 | 0.370 | 1.00 | 1294.6 |
| far/select/ms | 0.202 | 0.273 | 0.90 | 1205.2 |
| stage/render/create_views | 0.196 | 0.253 | 0.94 | 1173.1 |
| stream/tick/ms | 0.195 | 0.266 | 0.82 | 1162.9 |
| stage/render/prepare_batch_phases | 0.191 | 0.272 | 0.54 | 1144.0 |
| stage/render/extract_commands | 0.171 | 0.201 | 0.46 | 1024.9 |
| stage/render/prepare_flush | 0.162 | 0.294 | 0.88 | 966.3 |
| bigspace/high_precision/ms | 0.160 | 0.704 | 1.81 | 954.4 |
| anim/evaluate/ms | 0.157 | 0.264 | 1.36 | 940.8 |
| anim/evaluate_cpu/ms | 0.150 | 0.193 | 0.61 | 898.2 |
| controller/move/ms | 0.138 | 0.285 | 0.49 | 822.4 |
| stage/render/prepare_views | 0.137 | 0.170 | 0.50 | 818.3 |
| stage/main/last | 0.115 | 0.162 | 1.81 | 688.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.103 | 0.171 | 0.68 | 613.4 |
| render/acquire/ms | 0.094 | 0.119 | 0.37 | 560.3 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.131 | 0.47 | 367.4 |
| pipelines/compile_ms | 0.049 | 0.000 | 120.23 | 290.6 |
| pipelines/front_ms | 0.047 | 0.000 | 116.04 | 278.8 |
| physics/step/ms | 0.046 | 0.086 | 0.66 | 275.4 |
| physics/insert/ms | 0.033 | 0.366 | 1.03 | 199.3 |
| stage/render/cleanup | 0.032 | 0.098 | 0.89 | 192.8 |
| stage/render/phase_sort | 0.032 | 0.055 | 0.33 | 190.9 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.28 | 179.1 |
| weather/map/ms | 0.024 | 0.032 | 0.12 | 146.5 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.41 | 87.1 |
| anim/apply/ms | 0.012 | 0.023 | 0.51 | 71.7 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.49 | 67.4 |
| controller/avoid/ms | 0.011 | 0.014 | 0.26 | 62.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.05 | 59.6 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 42.3 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 31.5 |
| stream/despawn/ms | 0.005 | 0.000 | 1.07 | 30.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 28.9 |
| script/ui/ms | 0.005 | 0.006 | 0.48 | 27.0 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 21.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.59 | 13.9 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.19 | 11.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.14 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.07 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 15 |
| crowd/c0/joints/count | count | 1301 | 2262 | 707 |
| crowd/c0/meshes/count | count | 353 | 646 | 189 |
| crowd/c1/count | count | 145 | 155 | 146 |
| crowd/c1/joints/count | count | 5487 | 5881 | 5489 |
| crowd/c1/meshes/count | count | 1150 | 1241 | 1152 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4614 | 8611 | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.42 | 25.00 | 0.41 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.41 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.49 | 0.01 |
| encode/finish_critical | ms | 0.97 | 25.00 | 1.23 |
| encode/pass:early prepass/max | ms | 0.75 | 24.77 | 0.84 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.77 | 1.81 | 0.90 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.68 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.16 | 0.22 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.85 | 1.23 |
| encode/pass:shadow_point_light/max | ms | 0.80 | 1.54 | 0.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.28 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.29 | 39.02 | 8.67 |
| gpu/timed/ms | ms | 7.95 | 8.94 | 8.14 |
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
| phases/alpha_mask3d/items | count | 808 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 808 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1291 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3392 | 5144 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1291 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3392 | 5144 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 226 | 29 |
| phases/shadow/bins | count | 4997 | 8927 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.3 k | 26.5 k | 6386 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 201 | 259 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 222 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 14 | 611 | 58 |
| preprocess/bind_groups_created | count | 14 | 611 | 58 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 237 | 123 |
| process/cpu_usage | % | 26.97 | 43.96 | 19.81 |
| process/mem_usage | GiB | 5.51 | 5.70 | 5.68 |
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
| skin/blocks_changed/count | count | 13 | 90 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 369.2 k | 808.7 k | 493.6 k |
| skin/joints_written/count | count | 5768 | 12.6 k | 7712 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4917 | 5010 | 5010 |
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
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.00 | 38.98 | 31.21 |
| system/mem_usage | % | 54.80 | 55.48 | 55.38 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 68 | 273 | 79 |
| views/all/bins | count | 7596 | 11.8 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.5 k | 14.0 k |
| views/all/pipelines | count | 246 | 306 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 35.1 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 48 | 29 |
| views/cascade/bins | count | 2804 | 6190 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8901 | 23.5 k | 6386 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 420 | 22.8 k | 6386 |
| views/lamp_face/batch_sets | count | 0 | 189 | 0 |
| views/lamp_face/bins | count | 2136 | 2870 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2387 | 3218 | 0 |
| views/lamp_face/pipelines | count | 166 | 213 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2901 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2888 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8384 | 13.1 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8553 | 13.3 k | 7918 |
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
| anim/bones_written/count | count | 61.64 | 287 | 368.5 k |
| anim/events/count | count | 0.09 | 3 | 529 |
| assets/Image/added | count | 0.10 | 43 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.62 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1304 | 4675 |
| encode//count | count | 34.25 | 222 | 204.7 k |
| encode/bind_walk/init_textures | count | 13.0 k | 119.2 k | 77.88 M |
| encode/bind_walk/init_textures_skipped | count | 66.7 k | 159.6 k | 398.55 M |
| encode/bind_walk/inits | count | 462 | 1209 | 2.76 M |
| encode/bind_walk/inits_skipped | count | 108 | 170 | 646.6 k |
| encode/bind_walk/merge_textures | count | 13.0 k | 119.1 k | 77.45 M |
| encode/bind_walk/merge_textures_skipped | count | 5367 | 6012 | 32.09 M |
| encode/bind_walk/merges | count | 99.94 | 350 | 597.4 k |
| encode/bind_walk/merges_skipped | count | 83.49 | 133 | 499.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5978 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5978 |
| encode/pass:early prepass/count | count | 1 | 1 | 5978 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5978 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5978 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5978 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5978 |
| encode/pass:shadow_point_light/count | count | 1.57 | 48 | 9397 |
| encode/queue/command_buffers | count | 45.84 | 280 | 274.0 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.89 M | 4.88 M | 11.27 G |
| encode/queue/write_buffer/calls | count | 261 | 912 | 1.56 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.1 k | 3.52 M | 1.07 G |
| encode/queue/write_buffer_with/calls | count | 11.02 | 465 | 65.9 k |
| encode/queue/write_texture/bytes | bytes | 12.0 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5978 |
| physics/inserts | count | 0.38 | 92 | 2246 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3939 |
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
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5663 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1242 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.7 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.65 | 2645.5 | main/busy/ms 2614, stage/main/post_update 2508, stage/main/update 98.44, surface/advance_cpu_ms 68.44, stage/main/pre_update 4.37, stage/main/last 1.11, surface/cell_upload_ms 0.76, stage/main/first 0.63, stream/drive/ms 0.63, stage/main/fixed_main_loop 0.41, stream/tick/ms 0.14, weather/map/ms 0.13 |
| warm-up | 3.51 | 862.4 | main/busy/ms 2766, stage/main/post_update 2532, render/thread/ms 821, render/busy/ms 636, stage/render/prepare_assets 585, stage/main/update 219, stage/render/prepare_views 180, render/acquire/ms 180, surface/advance_cpu_ms 68.44, stage/render/create_views 29.66, stage/render/render 15.27, render/graph/ms 8.77 |
| warm-up | 3.65 | 146.8 | render/thread/ms 952, render/busy/ms 695, stage/render/prepare_assets 631, stage/render/prepare_views 181, render/acquire/ms 180, main/busy/ms 179, stage/main/update 127, pipelines/compile_ms 121, stage/render/render 91.68, render/present/ms 76.29, pipelines/driver_ms 61.76, pipelines/front_ms 59.47 |
| warm-up | 3.71 | 52.3 | pipelines/compile_ms 586, pipelines/front_ms 411, pipelines/wait_ms 400, render/thread/ms 181, pipelines/driver_ms 175, stage/render/render 121, render/busy/ms 109, render/present/ms 72.00, render/graph/ms 48.13, stage/render/prepare_assets 46.82, stage/render/graph_submit 45.76, main/busy/ms 43.70 |
| warm-up | 3.78 | 77.2 | pipelines/wait_ms 2715, pipelines/compile_ms 1318, pipelines/front_ms 1154, pipelines/driver_ms 164, main/busy/ms 129, render/thread/ms 126, render/busy/ms 120, stage/main/update 80.86, stage/render/render 52.14, render/graph/ms 49.33, stage/render/graph_submit 47.14, stage/main/post_update 38.50 |
| warm-up | 3.91 | 121.9 | pipelines/wait_ms 4390, pipelines/compile_ms 1698, pipelines/front_ms 1571, render/thread/ms 154, render/busy/ms 149, main/busy/ms 146, pipelines/driver_ms 127, stage/render/prepare_assets 88.84, stage/main/update 85.13, stage/main/post_update 50.66, stage/render/prepare_collect_phase_buffers 22.56, bigspace/total/ms 18.36 |
| warm-up | 3.96 | 32.7 | pipelines/compile_ms 1095, pipelines/front_ms 1046, pipelines/wait_ms 250, render/thread/ms 54.52, render/busy/ms 53.01, pipelines/driver_ms 48.51, main/busy/ms 39.17, stage/render/prepare_assets 34.12, stage/main/post_update 23.15, stage/main/update 11.98, terrain/update/ms 9.46, stage/render/render 9.31 |
| warm-up | 4.00 | 31.8 | pipelines/compile_ms 1319, pipelines/front_ms 1263, pipelines/wait_ms 104, render/thread/ms 61.52, render/busy/ms 59.84, pipelines/driver_ms 56.16, main/busy/ms 41.71, stage/main/post_update 30.34, stage/render/prepare_assets 20.87, encode//sum 20.09, stage/render/render 15.76, render/graph/ms 13.62 |
| warm-up | 4.04 | 33.1 | pipelines/compile_ms 706, pipelines/front_ms 685, pipelines/wait_ms 142, render/thread/ms 42.84, render/busy/ms 40.35, main/busy/ms 29.20, pipelines/driver_ms 21.12, stage/render/prepare_assets 21.10, stage/main/post_update 14.69, stage/render/render 12.94, stage/main/update 10.02, render/graph/ms 9.77 |
| warm-up | 4.15 | 97.6 | pipelines/compile_ms 1221, pipelines/front_ms 1177, main/busy/ms 65.16, render/thread/ms 52.16, stage/main/update 51.06, render/busy/ms 50.18, stream/drive/ms 44.92, pipelines/driver_ms 44.09, stage/render/prepare_assets 36.98, pipelines/wait_ms 22.01, stage/main/post_update 9.10, stage/render/render 8.19 |
| warm-up | 4.22 | 37.4 | pipelines/compile_ms 520, pipelines/front_ms 501, render/thread/ms 45.04, main/busy/ms 41.12, render/busy/ms 40.47, stage/main/update 26.60, stage/render/prepare_assets 20.46, pipelines/driver_ms 18.89, stage/render/render 17.16, render/graph/ms 12.34, stage/render/graph_submit 9.70, stream/drive/ms 8.78 |
| warm-up | 4.25 | 31.1 | pipelines/compile_ms 534, pipelines/front_ms 518, render/thread/ms 52.74, render/busy/ms 51.21, main/busy/ms 46.43, stage/render/prepare_assets 33.23, stage/main/update 30.05, stream/drive/ms 26.71, pipelines/driver_ms 16.18, stage/render/render 13.93, render/graph/ms 11.85, stage/main/post_update 11.68 |
| warm-up | 4.33 | 76.7 | pipelines/compile_ms 125, pipelines/front_ms 119, render/thread/ms 81.68, render/busy/ms 78.87, main/busy/ms 63.72, stage/render/prepare_assets 41.12, stage/main/update 32.52, stream/drive/ms 28.40, stage/render/render 26.53, render/graph/ms 23.12, stage/main/post_update 17.76, stage/render/graph_submit 10.53 |
| warm-up | 4.38 | 50.8 | pipelines/compile_ms 665, pipelines/front_ms 636, render/thread/ms 88.78, render/busy/ms 85.15, main/busy/ms 70.77, stage/main/update 40.29, stage/render/prepare_assets 40.10, stage/render/render 29.69, pipelines/driver_ms 29.48, render/graph/ms 25.48, stage/main/post_update 14.83, stage/render/camera_driver 10.93 |
| warm-up | 4.51 | 134.2 | pipelines/compile_ms 2170, pipelines/front_ms 2092, render/thread/ms 122, render/busy/ms 120, pipelines/driver_ms 78.62, main/busy/ms 62.90, stage/render/prepare_assets 53.31, stage/main/update 42.71, stage/render/prepare_bind_groups 40.22, pipelines/wait_ms 24.35, pipelines/lock_ms 22.48, stage/render/render 17.88 |
| warm-up | 4.58 | 73.5 | pipelines/compile_ms 1960, pipelines/front_ms 1874, render/thread/ms 152, render/busy/ms 150, pipelines/wait_ms 128, pipelines/driver_ms 85.74, main/busy/ms 68.69, stage/render/prepare_assets 47.52, stage/render/prepare_bind_groups 39.90, stage/render/render 27.37, stage/main/post_update 26.18, render/graph/ms 25.00 |
| warm-up | 4.66 | 80.1 | pipelines/compile_ms 439, pipelines/front_ms 404, pipelines/wait_ms 184, render/thread/ms 130, render/busy/ms 128, main/busy/ms 67.15, stage/render/prepare_assets 37.68, stage/render/render 36.69, pipelines/driver_ms 34.97, render/graph/ms 34.15, stage/render/graph_submit 27.22, stage/main/post_update 25.27 |
| warm-up | 4.75 | 88.2 | pipelines/compile_ms 2523, pipelines/front_ms 2441, pipelines/wait_ms 767, render/thread/ms 150, render/busy/ms 148, pipelines/driver_ms 81.41, main/busy/ms 80.92, stage/main/post_update 54.06, encode//sum 50.36, stage/render/prepare_assets 33.64, stage/render/prepare_batch_phases 33.57, stage/render/render 32.92 |
| warm-up | 4.80 | 46.6 | pipelines/compile_ms 2774, pipelines/front_ms 2659, pipelines/wait_ms 1136, render/thread/ms 130, render/busy/ms 127, pipelines/driver_ms 115, main/busy/ms 87.45, encode//sum 68.02, stage/main/post_update 52.95, stage/render/render 34.49, stage/render/prepare_batch_phases 33.51, render/graph/ms 31.65 |
| warm-up | 4.84 | 44.2 | pipelines/compile_ms 564, pipelines/front_ms 520, pipelines/wait_ms 503, render/thread/ms 77.25, render/busy/ms 75.14, pipelines/driver_ms 43.14, main/busy/ms 41.62, encode//sum 28.29, stage/main/update 25.24, stage/render/render 24.24, render/graph/ms 21.67, stage/render/graph_submit 16.18 |
| warm-up | 4.90 | 55.9 | pipelines/compile_ms 298, pipelines/front_ms 289, main/busy/ms 91.34, stage/main/update 70.90, render/thread/ms 70.65, render/busy/ms 68.94, pipelines/wait_ms 62.88, stream/drive/ms 58.61, stage/render/prepare_assets 19.87, stage/render/prepare_batch_phases 15.26, stage/render/render 14.20, render/graph/ms 11.95 |
| warm-up | 4.95 | 52.8 | pipelines/compile_ms 1252, pipelines/front_ms 1217, pipelines/wait_ms 487, main/busy/ms 111, stage/main/update 84.84, render/thread/ms 63.28, render/busy/ms 61.09, stream/drive/ms 58.23, pipelines/driver_ms 35.22, stage/render/render 18.73, render/graph/ms 15.41, stage/main/post_update 14.56 |
| warm-up | 5.04 | 84.8 | pipelines/compile_ms 2488, pipelines/front_ms 2421, pipelines/wait_ms 767, main/busy/ms 125, stage/main/update 102, render/thread/ms 84.45, render/busy/ms 80.16, pipelines/driver_ms 67.18, stream/drive/ms 46.86, stage/render/render 38.49, render/graph/ms 32.90, stage/render/graph_submit 22.41 |
| warm-up | 5.13 | 96.8 | pipelines/compile_ms 1484, pipelines/front_ms 1443, pipelines/wait_ms 281, main/busy/ms 103, render/thread/ms 100, render/busy/ms 96.59, stage/main/update 82.99, stream/drive/ms 47.81, stage/render/prepare_assets 42.02, pipelines/driver_ms 40.99, stage/render/render 32.50, render/graph/ms 28.03 |
| warm-up | 5.18 | 48.8 | pipelines/compile_ms 785, pipelines/front_ms 738, main/busy/ms 88.38, render/thread/ms 81.79, render/busy/ms 79.93, stage/main/update 61.55, pipelines/driver_ms 47.87, stream/drive/ms 47.26, stage/render/prepare_assets 47.13, stage/render/render 18.26, stage/main/post_update 15.83, render/graph/ms 15.75 |
| warm-up | 5.27 | 87.5 | pipelines/compile_ms 1615, pipelines/front_ms 1548, main/busy/ms 95.82, render/thread/ms 81.80, render/busy/ms 79.90, stage/main/update 68.74, pipelines/driver_ms 66.56, stream/drive/ms 46.54, stage/render/prepare_assets 40.38, stage/render/render 28.07, render/graph/ms 25.69, stage/render/graph_submit 22.39 |
| warm-up | 5.35 | 50.7 | pipelines/compile_ms 250, pipelines/front_ms 242, main/busy/ms 59.56, stage/main/update 46.03, render/thread/ms 40.88, render/busy/ms 39.59, stream/drive/ms 29.55, stage/render/prepare_assets 15.29, stage/render/render 9.40, pipelines/driver_ms 8.06, encode//sum 7.86, render/graph/ms 7.55 |
| warm-up | 5.38 | 31.5 | pipelines/compile_ms 591, pipelines/front_ms 574, render/thread/ms 48.60, render/busy/ms 47.16, main/busy/ms 32.56, pipelines/driver_ms 16.52, stage/main/update 16.20, stage/render/render 12.17, stage/render/prepare_assets 12.09, stage/main/post_update 10.22, render/graph/ms 10.11, encode//sum 8.56 |
| warm-up | 5.46 | 36.5 | pipelines/compile_ms 154, pipelines/front_ms 149, render/thread/ms 56.15, render/busy/ms 54.26, main/busy/ms 42.30, stage/main/post_update 22.29, stage/render/render 15.16, stage/main/update 14.44, stage/render/prepare_assets 13.45, render/graph/ms 12.75, stage/render/graph_submit 9.35, encode//sum 7.89 |
| warm-up | 5.49 | 34.3 | pipelines/compile_ms 320, pipelines/front_ms 308, render/thread/ms 63.63, render/busy/ms 61.85, main/busy/ms 50.59, stage/main/post_update 21.16, stage/render/prepare_assets 20.56, stage/main/update 20.02, stage/render/render 14.55, render/graph/ms 12.04, pipelines/driver_ms 11.93, encode//sum 9.11 |
| warm-up | 5.55 | 52.8 | pipelines/compile_ms 469, pipelines/front_ms 452, render/thread/ms 68.86, render/busy/ms 67.57, main/busy/ms 43.46, stage/render/prepare_assets 42.82, stage/main/update 24.55, pipelines/driver_ms 17.46, stage/render/render 13.04, render/graph/ms 10.94, stage/main/post_update 9.85, stage/render/graph_submit 7.94 |
| warm-up | 5.62 | 47.3 | pipelines/compile_ms 1279, pipelines/front_ms 1247, render/thread/ms 72.66, render/busy/ms 71.23, main/busy/ms 58.64, stage/main/post_update 41.63, pipelines/driver_ms 31.12, stage/render/prepare_resources 17.91, stage/render/prepare_batch_phases 14.95, stage/render/render 12.78, stage/main/update 11.27, render/graph/ms 10.90 |
| warm-up | 5.69 | 41.8 | pipelines/compile_ms 666, pipelines/front_ms 651, render/thread/ms 50.21, render/busy/ms 48.80, pipelines/wait_ms 32.34, main/busy/ms 30.71, stage/render/prepare_assets 25.64, pipelines/driver_ms 14.81, stage/main/update 13.48, stage/render/render 12.56, render/graph/ms 10.35, stage/main/post_update 10.12 |
| warm-up | 5.79 | 43.0 | pipelines/compile_ms 354, pipelines/front_ms 345, pipelines/wait_ms 59.81, main/busy/ms 42.20, render/thread/ms 39.41, render/busy/ms 38.22, stage/main/update 22.62, stage/render/prepare_assets 19.36, stage/main/post_update 13.82, stream/drive/ms 12.43, stage/render/render 10.11, pipelines/driver_ms 8.99 |
| warm-up | 5.85 | 33.6 | render/thread/ms 48.59, render/busy/ms 47.24, stage/render/prepare_assets 26.44, main/busy/ms 25.38, stage/main/post_update 11.67, stage/render/render 10.89, render/graph/ms 9.10, stage/main/update 8.32, encode//sum 7.68, stage/render/graph_submit 6.90, encode/submit 4.21, encode/finish_wall 2.69 |
| warm-up | 6.41 | 32.2 | pipelines/compile_ms 144, pipelines/front_ms 140, render/thread/ms 39.49, render/busy/ms 38.47, main/busy/ms 27.41, stage/render/prepare_assets 14.82, stage/main/update 10.71, stage/main/post_update 9.27, stage/render/render 8.00, render/graph/ms 6.55, encode//sum 6.45, stage/render/specialize 5.83 |
| warm-up | 6.74 | 37.9 | main/busy/ms 45.43, render/thread/ms 29.39, render/busy/ms 28.39, stage/main/update 25.61, stream/drive/ms 14.58, stage/main/post_update 14.26, stage/render/render 10.35, encode//sum 9.48, render/graph/ms 8.78, stage/render/prepare_assets 6.78, stage/render/graph_submit 5.91, encode/submit 3.24 |
| warm-up | 6.77 | 30.2 | pipelines/compile_ms 158, pipelines/front_ms 154, main/busy/ms 39.73, render/thread/ms 38.92, render/busy/ms 37.68, stage/main/update 18.64, stage/main/post_update 14.93, stage/render/prepare_assets 13.75, stage/render/render 11.61, encode//sum 11.04, render/graph/ms 9.68, stage/render/graph_submit 6.02 |
| warm-up | 6.97 | 33.7 | render/thread/ms 47.29, render/busy/ms 46.16, main/busy/ms 28.40, stage/render/prepare_assets 26.74, stage/main/update 13.65, stage/render/render 10.78, stage/main/post_update 10.40, render/graph/ms 9.19, encode//sum 7.93, stage/render/graph_submit 6.99, stream/drive/ms 5.71, encode/submit 4.62 |
| warm-up | 7.17 | 39.0 | main/busy/ms 42.63, render/thread/ms 28.05, render/busy/ms 27.05, stage/main/update 24.13, pipelines/compile_ms 20.59, pipelines/front_ms 20.31, stream/drive/ms 12.47, stage/render/prepare_assets 12.12, stage/main/post_update 11.46, stage/render/render 8.75, render/graph/ms 7.34, encode//sum 6.54 |
| warm-up | 7.24 | 31.4 | render/thread/ms 41.26, render/busy/ms 40.17, main/busy/ms 18.93, stage/render/render 12.20, render/graph/ms 10.66, stage/main/post_update 10.38, stage/render/specialize 9.83, stage/render/graph_submit 8.41, encode//sum 8.24, stage/render/queue 7.09, stage/main/update 5.39, encode/submit 4.27 |
| warm-up | 8.52 | 30.6 | render/thread/ms 37.21, render/busy/ms 36.09, main/busy/ms 18.05, stage/render/render 11.60, stage/main/post_update 10.89, render/graph/ms 10.00, stage/render/specialize 9.71, stage/render/graph_submit 7.92, encode//sum 6.57, stage/render/queue 6.44, stage/main/update 4.80, encode/submit 4.33 |
| measure | 1.72 | 31.0 | render/thread/ms 39.32, render/busy/ms 38.38, main/busy/ms 18.73, stage/render/render 13.73, render/graph/ms 12.30, stage/main/post_update 11.94, stage/render/graph_submit 9.39, encode//sum 9.08, stage/render/specialize 8.47, encode/pass:shadow_point_light/sum 6.00, encode/submit 5.92, stage/render/queue 5.53 |
| measure | 5.91 | 30.8 | render/thread/ms 36.29, render/busy/ms 35.28, main/busy/ms 20.17, stage/main/post_update 13.16, stage/render/render 10.28, stage/render/specialize 9.24, render/graph/ms 8.76, encode//sum 6.99, stage/render/queue 6.89, stage/render/graph_submit 6.55, stage/main/update 4.33, encode/submit 3.51 |
| measure | 7.23 | 30.4 | render/thread/ms 37.75, render/busy/ms 36.73, main/busy/ms 18.72, stage/render/render 12.63, stage/main/post_update 11.50, render/graph/ms 11.07, stage/render/specialize 9.25, encode//sum 9.01, stage/render/graph_submit 8.36, stage/render/queue 6.27, encode/submit 5.16, encode/pass:shadow_point_light/sum 4.70 |
| measure | 7.28 | 36.1 | render/thread/ms 45.46, render/busy/ms 44.56, main/busy/ms 44.11, stage/render/render 37.36, stage/main/post_update 36.96, render/graph/ms 36.10, stage/render/graph_submit 33.49, encode//sum 32.66, encode/finish_wall 26.84, encode/pass:early prepass/sum 25.23, encode/submit 6.68, encode/pass:shadow_point_light/sum 5.63 |
| measure | 15.74 | 30.2 | render/thread/ms 39.50, render/busy/ms 38.40, main/busy/ms 19.42, stage/render/render 12.90, stage/main/post_update 12.02, render/graph/ms 11.41, stage/render/specialize 10.34, encode//sum 9.01, stage/render/graph_submit 8.92, stage/render/queue 6.95, encode/submit 6.04, stage/main/update 4.58 |
| measure | 18.18 | 31.1 | main/busy/ms 38.88, stage/main/update 24.16, render/thread/ms 20.62, stream/drive/ms 20.29, render/busy/ms 19.66, stage/render/render 11.93, stage/main/post_update 11.78, render/graph/ms 10.53, encode//sum 9.25, stage/render/graph_submit 7.55, encode/pass:shadow_point_light/sum 5.84, encode/submit 4.94 |
| measure | 19.45 | 31.3 | render/thread/ms 43.36, render/busy/ms 42.27, main/busy/ms 19.59, stage/render/render 14.63, render/graph/ms 12.98, stage/main/post_update 12.57, encode//sum 12.26, stage/render/specialize 10.72, stage/render/graph_submit 9.96, stage/render/queue 7.60, encode/pass:shadow_point_light/sum 6.44, encode/submit 6.34 |
| measure | 23.02 | 35.9 | render/thread/ms 39.63, render/busy/ms 38.29, main/busy/ms 24.12, stage/main/post_update 12.34, stage/render/specialize 11.76, stage/render/render 11.63, render/graph/ms 9.89, encode//sum 9.02, stage/render/graph_submit 7.61, stage/render/queue 7.14, stage/main/update 6.50, encode/submit 4.45 |
| measure | 24.54 | 30.1 | render/thread/ms 37.12, render/busy/ms 36.02, main/busy/ms 19.36, stage/main/post_update 12.25, stage/render/render 11.35, encode//sum 10.11, render/graph/ms 9.87, stage/render/specialize 9.32, stage/render/graph_submit 7.40, stage/render/queue 6.94, encode/submit 4.83, stage/main/update 4.73 |
| measure | 27.29 | 35.9 | render/thread/ms 46.31, render/busy/ms 45.22, main/busy/ms 25.91, stage/render/render 16.12, encode//sum 15.32, render/graph/ms 14.44, stage/main/post_update 13.26, stage/render/specialize 11.87, stage/render/graph_submit 11.23, stage/render/queue 8.08, encode/submit 7.19, encode/pass:shadow_point_light/sum 6.86 |
| measure | 28.87 | 31.9 | render/thread/ms 42.38, render/busy/ms 41.29, main/busy/ms 20.92, stage/render/render 15.18, render/graph/ms 13.64, stage/main/post_update 13.47, encode//sum 13.43, stage/render/graph_submit 10.57, stage/render/specialize 9.38, encode/pass:shadow_point_light/sum 7.35, stage/render/queue 6.73, encode/submit 6.33 |
| measure | 30.47 | 31.3 | render/thread/ms 39.08, render/busy/ms 37.97, main/busy/ms 20.16, stage/main/post_update 12.13, stage/render/render 11.09, stage/render/specialize 10.38, render/graph/ms 9.54, stage/render/graph_submit 7.67, encode//sum 7.65, stage/render/queue 7.40, stage/main/update 5.32, encode/submit 4.20 |
| measure | 31.91 | 30.1 | render/thread/ms 38.39, render/busy/ms 37.36, main/busy/ms 20.88, stage/main/post_update 13.92, stage/render/render 11.53, stage/render/specialize 10.11, render/graph/ms 10.06, encode//sum 9.44, stage/render/graph_submit 7.79, stage/render/queue 6.66, encode/submit 4.10, stage/main/update 4.07 |
| measure | 35.35 | 31.7 | render/thread/ms 40.51, render/busy/ms 39.26, main/busy/ms 20.85, stage/main/post_update 12.65, stage/render/render 12.46, stage/render/specialize 11.02, render/graph/ms 10.82, encode//sum 9.55, stage/render/graph_submit 8.22, stage/render/queue 7.02, encode/submit 5.05, stage/main/update 4.88 |
| measure | 37.48 | 30.5 | render/thread/ms 38.68, render/busy/ms 37.71, main/busy/ms 19.39, stage/main/post_update 12.40, stage/render/render 12.20, render/graph/ms 10.79, encode//sum 10.70, stage/render/specialize 9.99, stage/render/graph_submit 8.25, stage/render/queue 6.55, encode/submit 4.67, stage/main/update 4.15 |
| measure | 40.81 | 32.7 | render/thread/ms 39.24, render/busy/ms 38.17, main/busy/ms 19.63, stage/main/post_update 11.59, stage/render/render 11.14, stage/render/specialize 10.15, render/graph/ms 9.63, encode//sum 8.78, stage/render/graph_submit 7.69, stage/render/queue 6.27, stage/main/update 4.87, encode/submit 4.44 |
| measure | 43.22 | 31.0 | render/thread/ms 37.27, render/busy/ms 36.26, main/busy/ms 17.30, stage/render/render 10.60, stage/render/specialize 10.29, stage/main/post_update 10.25, render/graph/ms 9.33, encode//sum 8.11, stage/render/graph_submit 7.61, stage/render/queue 6.86, stage/main/update 4.51, encode/submit 4.31 |
| measure | 45.69 | 33.7 | render/thread/ms 41.11, render/busy/ms 39.89, main/busy/ms 18.52, stage/render/specialize 10.93, stage/render/render 10.89, stage/main/post_update 10.51, render/graph/ms 9.40, encode//sum 9.02, stage/render/graph_submit 7.37, stage/render/queue 7.29, stage/main/update 5.16, stage/render/prepare_meshes 4.68 |
| measure | 56.43 | 30.9 | render/thread/ms 36.02, render/busy/ms 34.79, main/busy/ms 16.14, stage/render/render 9.84, stage/render/specialize 9.17, stage/main/post_update 9.01, encode//sum 8.44, render/graph/ms 8.38, stage/render/queue 6.91, stage/render/graph_submit 6.22, stage/main/update 4.04, stage/render/prepare_meshes 3.85 |
| measure | 59.05 | 48.8 | render/thread/ms 56.37, render/busy/ms 55.03, main/busy/ms 26.15, stage/render/render 16.16, stage/main/post_update 15.06, stage/render/specialize 14.83, render/graph/ms 14.42, encode//sum 13.64, stage/render/graph_submit 11.11, stage/main/update 7.00, stage/render/queue 6.58, stage/render/prepare_meshes 6.29 |
| measure | 59.09 | 46.8 | render/thread/ms 63.54, render/busy/ms 62.50, main/busy/ms 20.50, stage/render/prepare_assets 18.83, stage/render/specialize 14.94, stage/main/post_update 10.62, stage/render/render 9.73, render/graph/ms 8.51, encode//sum 8.30, stage/render/prepare_meshes 7.05, stage/render/queue 6.75, stage/render/graph_submit 6.53 |
| measure | 62.58 | 31.7 | render/thread/ms 38.69, render/busy/ms 37.74, main/busy/ms 20.78, stage/main/post_update 11.26, stage/render/render 11.02, stage/render/specialize 10.81, render/graph/ms 9.80, encode//sum 9.55, stage/render/graph_submit 7.50, stage/render/queue 6.49, stage/main/update 6.40, stage/render/prepare_meshes 4.19 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=base-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 10%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **77%** (min 54%) · 2625 MHz (min 2625) · memory 8101 MHz · 79.7 W · 77 °C (max 83) · VRAM 7062 MiB · throttled 100% of samples

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
