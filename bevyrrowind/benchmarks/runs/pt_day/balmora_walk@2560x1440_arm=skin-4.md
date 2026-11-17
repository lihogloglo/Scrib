# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5910 frames over 65.6 s · **median 10.79 ms (93 fps)** · mean 11.11 · p95 13.76 · p99 17.56 (1% low 57 fps) · worst 47.80 ms · 91 spikes over 1.5× median · 68.5 stutters/min (work 52.1/min)
- warm-up: 324 frames over 9.1 s, worst 2564.7 ms
- hitches (> 27.0 ms): **32** measured, 49 in warm-up
- timed GPU passes: 6.65 ms of a 11.11 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.79 ms, limit 11 ms
- **FAIL** frame_ms: 75 stutters (> 1.5x moving median of 60) in 1.09 min = 68.55/min, limit 1/min
- **FAIL** worst frame 47.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.873 | 2.032 | 2.064 | 0.92 | 0.174 |
| main_opaque_pass_3d | 1.501 | 1.628 | 2.037 | 0.92 | 0.074 |
| early prepass | 0.458 | 0.497 | 0.734 | 0.92 | 0.087 |
| bloom | 0.290 | 0.314 | 0.321 | 0.92 | 0.063 |
| atmosphere_luts | 0.278 | 0.302 | 0.364 | 0.92 | 0.003 |
| water_velocity | 0.257 | 0.279 | 0.347 | 0.92 | 0.002 |
| water_sspr | 0.186 | 0.201 | 0.203 | 0.92 | 0.002 |
| ssao | 0.174 | 0.189 | 0.193 | 0.92 | 0.007 |
| auxiliary_views | 0.151 | 0.164 | 0.604 | 0.92 | 0.172 |
| atmos_froxel_inject | 0.135 | 0.146 | 0.150 | 0.92 | 0.001 |
| clouds_trace | 0.129 | 0.140 | 0.232 | 0.92 | 0.001 |
| water_fft | 0.121 | 0.131 | 0.138 | 0.92 | 0.003 |
| uniform_allocation | 0.120 | 0.130 | 0.181 | 0.92 | 0.027 |
| upscaling | 0.109 | 0.119 | 0.126 | 0.92 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.109 | 0.92 | 0.002 |
| bin_unpacking | 0.096 | 0.104 | 0.141 | 0.92 | 0.020 |
| main_transmissive_pass_3d | 0.079 | 0.086 | 0.179 | 0.92 | 0.009 |
| tonemapping | 0.078 | 0.085 | 0.088 | 0.92 | 0.003 |
| clustering | 0.059 | 0.064 | 0.067 | 0.92 | 0.018 |
| render_sky | 0.047 | 0.051 | 0.052 | 0.92 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.92 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.92 | 0.001 |
| lightprobe_irradiance_map | 0.039 | 0.412 | 0.416 | 0.10 | 0.001 |
| surface_trail | 0.037 | 0.056 | 0.057 | 0.65 | 0.002 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.92 | 0.003 |
| water_snapshot | 0.028 | 0.030 | 0.034 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.046 | 0.92 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.030 | 0.032 | 0.92 | 0.001 |
| lightprobe_radiance_map | 0.021 | 0.218 | 0.228 | 0.10 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.92 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.92 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.92 | 0.003 |
| lightprobe_copy | 0.001 | 0.014 | 0.015 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.639 | 12.098 | 42.02 | 56964.3 |
| main/busy/ms | 9.129 | 10.853 | 28.54 | 53949.7 |
| render/busy/ms | 9.102 | 11.559 | 41.38 | 53793.8 |
| stage/main/post_update | 5.677 | 7.072 | 10.07 | 33551.8 |
| stage/render/render | 5.575 | 7.216 | 14.93 | 32950.7 |
| render/graph/ms | 4.842 | 6.455 | 13.88 | 28615.3 |
| encode//sum | 3.904 | 5.151 | 20.34 | 23072.7 |
| stage/render/graph_submit | 3.618 | 4.843 | 9.74 | 21381.4 |
| encode/submit | 2.292 | 3.237 | 6.75 | 13543.2 |
| stage/main/update | 2.168 | 2.574 | 22.01 | 12813.0 |
| encode/finish_wall | 1.382 | 1.995 | 4.14 | 8170.2 |
| stage/render/camera_driver | 1.170 | 1.666 | 4.32 | 6913.7 |
| encode/pass:shadow_point_light/sum | 0.954 | 3.534 | 17.01 | 5637.7 |
| encode/pass:shadow_directional_light/sum | 0.865 | 1.554 | 2.57 | 5111.0 |
| stage/render/prepare_write_phase_buffers | 0.793 | 1.135 | 2.42 | 4684.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.782 | 1.101 | 2.08 | 4624.0 |
| encode/pass:early prepass/sum | 0.749 | 1.076 | 1.79 | 4429.3 |
| stage/render/prepare_assets | 0.583 | 0.751 | 9.90 | 3443.4 |
| bigspace/total/ms | 0.528 | 1.274 | 2.39 | 3118.4 |
| stage/render/prepare_meshes | 0.465 | 0.601 | 6.83 | 2745.3 |
| stage/render/prepare_resources | 0.451 | 0.547 | 0.92 | 2667.5 |
| render/present/ms | 0.443 | 0.653 | 2.76 | 2617.5 |
| stage/main/first | 0.400 | 0.521 | 4.85 | 2361.7 |
| stage/render/specialize | 0.367 | 0.451 | 16.72 | 2168.1 |
| stage/main/pre_update | 0.365 | 0.522 | 2.91 | 2158.7 |
| stage/render/queue | 0.311 | 0.452 | 7.75 | 1840.4 |
| terrain/update/ms | 0.309 | 0.401 | 2.20 | 1824.3 |
| stage/render/prepare_bind_groups | 0.295 | 0.529 | 2.74 | 1743.7 |
| terrain/select/ms | 0.274 | 0.357 | 0.98 | 1620.9 |
| controller/feed/ms | 0.271 | 0.338 | 0.74 | 1600.3 |
| stream/drive/ms | 0.251 | 0.333 | 19.98 | 1480.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.244 | 0.360 | 1.03 | 1442.5 |
| stage/main/fixed_main_loop | 0.224 | 0.384 | 1.09 | 1325.2 |
| bigspace/low_precision/ms | 0.223 | 0.696 | 2.16 | 1319.5 |
| far/select/ms | 0.201 | 0.267 | 0.61 | 1186.3 |
| stage/render/create_views | 0.199 | 0.258 | 1.00 | 1178.1 |
| stream/tick/ms | 0.193 | 0.260 | 0.51 | 1142.2 |
| stage/render/prepare_batch_phases | 0.190 | 0.273 | 0.64 | 1125.8 |
| stage/render/extract_commands | 0.179 | 0.209 | 0.45 | 1058.2 |
| stage/render/prepare_flush | 0.165 | 0.308 | 0.88 | 973.1 |
| anim/evaluate/ms | 0.160 | 0.253 | 1.55 | 948.5 |
| anim/evaluate_cpu/ms | 0.156 | 0.195 | 0.75 | 924.0 |
| bigspace/high_precision/ms | 0.143 | 0.567 | 1.86 | 845.0 |
| stage/render/prepare_views | 0.138 | 0.171 | 0.49 | 812.8 |
| controller/move/ms | 0.137 | 0.336 | 0.61 | 812.3 |
| stage/main/last | 0.115 | 0.162 | 1.60 | 680.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.103 | 0.161 | 0.95 | 608.7 |
| render/acquire/ms | 0.094 | 0.119 | 0.44 | 553.0 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.129 | 0.44 | 369.5 |
| pipelines/compile_ms | 0.051 | 0.000 | 130.54 | 299.1 |
| physics/step/ms | 0.049 | 0.090 | 0.56 | 289.1 |
| pipelines/front_ms | 0.049 | 0.000 | 126.56 | 287.4 |
| physics/insert/ms | 0.032 | 0.357 | 0.97 | 191.1 |
| stage/render/cleanup | 0.032 | 0.098 | 0.71 | 190.9 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.79 | 189.4 |
| stage/render/phase_sort | 0.027 | 0.046 | 0.49 | 161.4 |
| weather/map/ms | 0.025 | 0.033 | 0.19 | 145.8 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.023 | 0.61 | 92.3 |
| anim/apply/ms | 0.012 | 0.021 | 0.23 | 69.8 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.57 | 69.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.31 | 63.5 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 60.2 |
| script/update/ms | 0.007 | 0.009 | 0.08 | 44.1 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 32.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.04 | 30.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 27.9 |
| script/ui/ms | 0.004 | 0.005 | 0.06 | 24.3 |
| controller/write/ms | 0.003 | 0.004 | 0.05 | 19.5 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.68 | 13.9 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.04 | 11.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 4.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.15 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
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
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1272 | 2218 | 641 |
| crowd/c0/meshes/count | count | 353 | 633 | 174 |
| crowd/c1/count | count | 146 | 155 | 146 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5523 |
| crowd/c1/meshes/count | count | 1167 | 1254 | 1159 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 637 | 5636 | 582 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 2.09 | 0.37 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.61 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.57 | 0.01 |
| encode/finish_critical | ms | 0.98 | 2.57 | 0.89 |
| encode/pass:early prepass/max | ms | 0.77 | 1.79 | 0.88 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 2.08 | 0.89 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.95 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.03 | 0.21 |
| encode/pass:shadow_directional_light/max | ms | 0.81 | 2.57 | 0.70 |
| encode/pass:shadow_point_light/max | ms | 0.82 | 1.41 | 0.81 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.79 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.36 | 41.38 | 8.77 |
| gpu/timed/ms | ms | 7.93 | 11.37 | 8.42 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 811 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 811 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1296 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3402 | 5113 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1296 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3402 | 5113 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 202 | 4 |
| phases/shadow/bins | count | 5033 | 8908 | 2423 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.4 k | 6374 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 257 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 561 | 9 |
| preprocess/bind_groups_created | count | 13 | 561 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 245 | 120 |
| process/cpu_usage | % | 25.96 | 42.72 | 21.43 |
| process/mem_usage | GiB | 5.48 | 5.65 | 5.65 |
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
| skin/blocks_changed/count | count | 14 | 118 | 14 |
| skin/blocks_skipped/count | count | 186 | 236 | 224 |
| skin/blocks_walked/count | count | 14 | 18 | 14 |
| skin/joint_bytes_written/bytes | bytes | 43.5 k | 815.9 k | 38.7 k |
| skin/joints_written/count | count | 680 | 12.7 k | 604 |
| skin/upload_writes/count | count | 8 | 30 | 7 |
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
| system/cpu_usage | % | 31.47 | 45.62 | 29.74 |
| system/mem_usage | % | 54.87 | 55.44 | 55.39 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 69 | 249 | 54 |
| views/all/bins | count | 7652 | 11.8 k | 5921 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 250 | 304 | 76 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.8 k | 33.9 k | 8127 |
| views/cascade/batch_sets | count | 9 | 54 | 4 |
| views/cascade/bins | count | 2807 | 6181 | 2423 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8910 | 23.5 k | 6374 |
| views/cascade/pipelines | count | 36 | 54 | 30 |
| views/cascade/work_items | count | 412 | 22.9 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2159 | 2761 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2418 | 3058 | 0 |
| views/lamp_face/pipelines | count | 168 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2933 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2888 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8422 | 13.0 k | 7618 |
| views/main/pipelines | count | 45 | 49 | 46 |
| views/main/work_items | count | 8584 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 56.53 | 239 | 334.1 k |
| anim/events/count | count | 0.09 | 3 | 530 |
| assets/Image/added | count | 0.10 | 36 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 139 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.66 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1328 | 4675 |
| encode//count | count | 34.02 | 220 | 201.1 k |
| encode/bind_walk/init_textures | count | 12.9 k | 114.7 k | 76.30 M |
| encode/bind_walk/init_textures_skipped | count | 63.6 k | 149.2 k | 375.64 M |
| encode/bind_walk/inits | count | 461 | 1127 | 2.72 M |
| encode/bind_walk/inits_skipped | count | 107 | 166 | 633.1 k |
| encode/bind_walk/merge_textures | count | 12.8 k | 114.6 k | 75.87 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 31.71 M |
| encode/bind_walk/merges | count | 99.49 | 336 | 588.0 k |
| encode/bind_walk/merges_skipped | count | 83.71 | 132 | 494.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5910 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5910 |
| encode/pass:early prepass/count | count | 1 | 1 | 5910 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5910 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5910 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5910 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5910 |
| encode/pass:shadow_point_light/count | count | 1.52 | 48 | 8964 |
| encode/queue/command_buffers | count | 45.56 | 278 | 269.2 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 1.55 M | 4.44 M | 9.13 G |
| encode/queue/write_buffer/calls | count | 267 | 843 | 1.58 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.7 k | 3.52 M | 1.05 G |
| encode/queue/write_buffer_with/calls | count | 11.05 | 426 | 65.3 k |
| encode/queue/write_texture/bytes | bytes | 12.0 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5910 |
| physics/inserts | count | 0.38 | 90 | 2260 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 161 | 2335 |
| physics/steps | count | 0.67 | 3 | 3939 |
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
| stream/upload/bytes | bytes | 5728 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1256 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.0 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2564.7 | main/busy/ms 2534, stage/main/post_update 2430, stage/main/update 95.70, surface/advance_cpu_ms 66.22, stage/main/pre_update 4.96, stage/main/last 1.17, surface/cell_upload_ms 0.73, stream/drive/ms 0.60, stage/main/first 0.50, stage/main/fixed_main_loop 0.38, stream/tick/ms 0.15, weather/map/ms 0.14 |
| warm-up | 3.30 | 737.9 | main/busy/ms 2682, stage/main/post_update 2457, render/thread/ms 704, render/busy/ms 537, stage/render/prepare_assets 487, stage/main/update 210, stage/render/prepare_views 164, render/acquire/ms 163, surface/advance_cpu_ms 66.22, stage/render/create_views 28.22, stage/render/render 13.30, render/graph/ms 8.61 |
| warm-up | 3.42 | 113.1 | render/thread/ms 799, render/busy/ms 579, stage/render/prepare_assets 514, main/busy/ms 169, stage/render/prepare_views 164, render/acquire/ms 164, pipelines/compile_ms 127, stage/main/update 118, pipelines/front_ms 86.68, stage/render/render 72.18, render/present/ms 56.43, pipelines/driver_ms 39.97 |
| warm-up | 3.46 | 42.4 | pipelines/compile_ms 358, pipelines/front_ms 290, render/thread/ms 122, stage/render/render 79.48, pipelines/driver_ms 68.49, main/busy/ms 65.96, render/present/ms 62.90, render/busy/ms 58.53, stage/main/post_update 49.01, stage/render/prepare_assets 27.32, terrain/update/ms 27.15, pipelines/wait_ms 17.00 |
| warm-up | 3.53 | 69.2 | pipelines/wait_ms 1349, pipelines/compile_ms 713, pipelines/front_ms 649, main/busy/ms 90.42, render/thread/ms 89.14, render/busy/ms 77.58, stage/main/post_update 66.66, pipelines/driver_ms 64.57, stage/render/render 40.94, render/graph/ms 28.98, stage/render/prepare_assets 26.80, stage/render/graph_submit 26.22 |
| warm-up | 3.66 | 128.4 | pipelines/wait_ms 6184, pipelines/compile_ms 1928, pipelines/front_ms 1820, render/thread/ms 175, render/busy/ms 172, main/busy/ms 138, pipelines/driver_ms 108, stage/main/update 71.09, stage/render/prepare_resources 64.63, stage/main/post_update 63.23, stage/render/prepare_assets 44.08, stage/render/render 37.24 |
| warm-up | 3.71 | 53.9 | pipelines/wait_ms 4910, pipelines/compile_ms 1934, pipelines/front_ms 1842, render/thread/ms 164, render/busy/ms 161, main/busy/ms 118, pipelines/driver_ms 92.41, stage/main/update 66.38, stage/render/prepare_resources 63.04, stage/render/prepare_assets 59.08, stage/main/post_update 43.36, stage/render/render 22.46 |
| warm-up | 3.76 | 46.9 | pipelines/compile_ms 1078, pipelines/front_ms 1025, pipelines/wait_ms 129, render/thread/ms 94.93, render/busy/ms 92.74, stage/render/prepare_assets 73.02, pipelines/driver_ms 52.96, main/busy/ms 48.89, stage/main/post_update 20.28, stage/main/update 19.04, stage/render/render 14.02, terrain/update/ms 12.06 |
| warm-up | 3.94 | 84.4 | pipelines/compile_ms 975, pipelines/front_ms 935, main/busy/ms 54.59, render/thread/ms 49.40, render/busy/ms 47.69, stage/main/update 43.87, pipelines/driver_ms 40.04, stream/drive/ms 38.88, stage/render/prepare_assets 31.29, stage/render/render 12.05, render/graph/ms 9.92, stage/render/graph_submit 7.87 |
| warm-up | 4.09 | 86.9 | pipelines/compile_ms 331, pipelines/front_ms 316, render/thread/ms 59.93, render/busy/ms 58.22, main/busy/ms 53.76, stage/render/prepare_assets 42.03, stage/main/update 33.84, stream/drive/ms 28.64, pipelines/driver_ms 14.22, stage/main/post_update 12.18, stage/render/render 11.58, render/graph/ms 9.41 |
| warm-up | 4.14 | 46.4 | pipelines/compile_ms 304, pipelines/front_ms 290, render/thread/ms 76.99, render/busy/ms 75.33, stage/render/prepare_assets 46.74, main/busy/ms 37.04, stage/main/post_update 18.38, stage/render/render 14.76, pipelines/driver_ms 13.51, render/graph/ms 12.57, stage/main/update 10.52, stage/render/graph_submit 10.20 |
| warm-up | 4.19 | 58.3 | pipelines/compile_ms 1354, pipelines/front_ms 1308, render/thread/ms 76.46, main/busy/ms 74.12, render/busy/ms 73.99, stage/main/update 46.11, pipelines/driver_ms 45.47, stage/render/prepare_assets 42.56, stage/main/post_update 20.25, stage/render/render 15.87, render/graph/ms 13.11, stage/render/graph_submit 10.07 |
| warm-up | 4.27 | 79.7 | pipelines/compile_ms 1798, pipelines/front_ms 1739, render/thread/ms 90.24, render/busy/ms 88.07, main/busy/ms 68.12, stage/render/prepare_assets 63.69, pipelines/driver_ms 59.42, stage/main/update 48.98, stage/render/render 12.42, stage/main/post_update 12.20, render/graph/ms 9.99, pipelines/wait_ms 8.37 |
| warm-up | 4.35 | 73.2 | pipelines/compile_ms 1097, pipelines/front_ms 1060, render/thread/ms 114, render/busy/ms 113, pipelines/wait_ms 98.95, main/busy/ms 67.66, stage/render/prepare_assets 59.95, pipelines/driver_ms 36.92, stage/main/update 32.22, stage/main/post_update 27.60, stage/render/queue 17.80, encode//sum 15.52 |
| warm-up | 4.43 | 86.9 | pipelines/compile_ms 1028, pipelines/front_ms 992, pipelines/wait_ms 181, render/thread/ms 139, render/busy/ms 138, main/busy/ms 71.55, stage/render/prepare_assets 70.62, pipelines/driver_ms 36.75, stage/main/post_update 32.14, stage/main/update 29.78, encode//sum 20.19, stage/render/queue 18.58 |
| warm-up | 4.53 | 100.5 | pipelines/compile_ms 2373, pipelines/front_ms 2262, pipelines/wait_ms 669, render/thread/ms 170, render/busy/ms 168, pipelines/driver_ms 110, main/busy/ms 95.64, stage/main/post_update 63.92, encode//sum 56.20, stage/render/prepare_assets 48.90, stage/render/prepare_resources 38.16, stage/render/render 25.55 |
| warm-up | 4.60 | 62.7 | pipelines/compile_ms 2732, pipelines/front_ms 2619, pipelines/wait_ms 935, render/thread/ms 149, render/busy/ms 137, pipelines/driver_ms 113, main/busy/ms 105, stage/main/post_update 62.96, encode//sum 60.11, stage/render/prepare_resources 33.87, stage/render/render 31.07, render/graph/ms 28.40 |
| warm-up | 4.67 | 68.1 | pipelines/compile_ms 1468, pipelines/front_ms 1422, pipelines/wait_ms 613, main/busy/ms 103, render/thread/ms 87.69, render/busy/ms 75.82, stage/main/update 73.01, stream/drive/ms 59.33, pipelines/driver_ms 46.09, stage/render/prepare_assets 38.76, stage/render/render 19.51, render/graph/ms 16.86 |
| warm-up | 4.73 | 65.2 | pipelines/compile_ms 1870, pipelines/front_ms 1810, pipelines/wait_ms 829, main/busy/ms 107, stage/main/update 82.36, render/thread/ms 63.33, render/busy/ms 61.33, stream/drive/ms 60.54, pipelines/driver_ms 60.42, stage/render/prepare_assets 28.90, stage/render/render 15.83, stage/main/post_update 13.03 |
| warm-up | 4.79 | 64.0 | pipelines/compile_ms 1798, pipelines/front_ms 1747, pipelines/wait_ms 875, main/busy/ms 124, stage/main/update 96.23, render/thread/ms 68.24, render/busy/ms 64.19, pipelines/driver_ms 50.97, stream/drive/ms 44.28, stage/render/prepare_assets 22.39, stage/render/render 21.26, stage/main/post_update 16.79 |
| warm-up | 4.90 | 108.1 | pipelines/compile_ms 896, pipelines/front_ms 870, pipelines/wait_ms 308, main/busy/ms 112, stage/main/update 87.86, render/thread/ms 86.43, render/busy/ms 83.02, stream/drive/ms 43.12, stage/render/prepare_assets 42.13, pipelines/driver_ms 26.54, stage/render/render 21.26, render/graph/ms 17.34 |
| warm-up | 4.94 | 32.9 | pipelines/compile_ms 990, pipelines/front_ms 959, main/busy/ms 84.79, render/thread/ms 69.37, render/busy/ms 67.94, stage/main/update 63.50, stream/drive/ms 44.51, stage/render/prepare_assets 36.84, pipelines/driver_ms 30.92, stage/render/render 16.00, render/graph/ms 13.87, stage/main/post_update 11.88 |
| warm-up | 5.01 | 70.8 | pipelines/compile_ms 1520, pipelines/front_ms 1480, main/busy/ms 76.66, stage/main/update 61.08, render/thread/ms 48.20, render/busy/ms 46.92, stream/drive/ms 44.61, pipelines/driver_ms 40.08, stage/render/prepare_assets 24.55, stage/render/render 10.11, render/graph/ms 8.20, encode//sum 7.60 |
| warm-up | 5.06 | 32.2 | pipelines/compile_ms 204, pipelines/front_ms 197, main/busy/ms 49.37, stage/main/update 35.36, render/thread/ms 33.54, render/busy/ms 32.18, stream/drive/ms 29.54, stage/render/render 12.07, render/graph/ms 10.31, stage/main/post_update 8.31, encode//sum 8.19, stage/render/graph_submit 7.42 |
| warm-up | 5.18 | 44.0 | pipelines/compile_ms 126, pipelines/front_ms 123, render/thread/ms 64.77, render/busy/ms 63.27, main/busy/ms 43.50, stage/render/render 20.26, stage/main/update 19.94, render/graph/ms 18.18, stage/main/post_update 18.11, stage/render/graph_submit 14.64, stage/render/prepare_assets 12.24, encode/finish_wall 9.13 |
| warm-up | 5.21 | 32.6 | pipelines/compile_ms 125, pipelines/front_ms 118, render/thread/ms 68.69, render/busy/ms 65.61, main/busy/ms 45.98, stage/render/render 22.86, stage/main/update 21.66, render/graph/ms 19.23, stage/main/post_update 17.55, stage/render/graph_submit 15.28, stage/render/prepare_assets 14.19, encode/finish_wall 9.49 |
| warm-up | 5.26 | 50.8 | pipelines/compile_ms 318, pipelines/front_ms 304, render/thread/ms 65.35, render/busy/ms 62.34, main/busy/ms 40.06, stage/render/prepare_assets 36.28, stage/main/update 22.05, stage/render/render 15.62, pipelines/driver_ms 13.19, render/graph/ms 11.99, stage/main/post_update 10.63, stage/render/graph_submit 8.84 |
| warm-up | 5.30 | 34.4 | pipelines/compile_ms 848, pipelines/front_ms 823, render/thread/ms 68.94, render/busy/ms 67.47, main/busy/ms 45.12, stage/render/prepare_assets 33.20, pipelines/driver_ms 24.89, stage/main/update 21.18, stage/main/post_update 13.84, stage/render/render 13.56, render/graph/ms 11.57, stage/render/prepare_resources 11.10 |
| warm-up | 5.33 | 35.2 | pipelines/compile_ms 1463, pipelines/front_ms 1422, render/thread/ms 64.86, render/busy/ms 63.36, main/busy/ms 45.55, pipelines/driver_ms 40.85, stage/main/post_update 22.65, stage/render/prepare_write_phase_buffers 18.90, stage/main/update 13.46, stage/render/render 12.04, pipelines/wait_ms 11.46, stage/render/prepare_resources 11.22 |
| warm-up | 5.39 | 31.9 | pipelines/compile_ms 410, pipelines/front_ms 403, render/thread/ms 44.75, render/busy/ms 43.39, main/busy/ms 31.50, stage/render/prepare_assets 20.29, stage/main/update 13.00, stage/main/post_update 12.08, stage/render/render 11.09, render/graph/ms 9.10, encode//sum 7.62, pipelines/driver_ms 7.42 |
| warm-up | 6.13 | 32.0 | render/thread/ms 40.39, render/busy/ms 39.46, main/busy/ms 29.58, stage/render/prepare_assets 17.76, stage/main/update 12.46, stage/main/post_update 11.10, stage/render/render 8.94, encode//sum 7.61, render/graph/ms 7.36, stage/render/graph_submit 5.23, stream/drive/ms 4.25, stage/render/specialize 3.65 |
| warm-up | 6.48 | 33.3 | render/thread/ms 35.94, render/busy/ms 34.71, main/busy/ms 33.44, stage/main/update 16.55, stage/render/prepare_assets 14.35, stage/main/post_update 11.41, stage/render/render 10.64, encode//sum 9.54, render/graph/ms 8.65, stage/render/graph_submit 5.84, stream/drive/ms 4.66, encode/finish_wall 3.00 |
| warm-up | 6.66 | 39.8 | render/thread/ms 49.00, render/busy/ms 47.98, main/busy/ms 32.49, stage/render/prepare_assets 31.56, stage/main/update 19.34, stream/drive/ms 13.60, stage/main/post_update 9.76, stage/render/render 9.68, encode//sum 8.84, render/graph/ms 8.11, stage/render/graph_submit 6.02, encode/submit 3.27 |
| warm-up | 6.87 | 34.1 | main/busy/ms 41.54, render/thread/ms 24.11, render/busy/ms 23.07, stage/main/update 21.35, stream/drive/ms 14.11, stage/main/post_update 13.52, stream/spawn/ms 12.61, stage/render/render 10.43, render/graph/ms 8.88, encode//sum 7.50, stage/render/graph_submit 6.46, stage/render/prepare_assets 4.92 |
| warm-up | 6.96 | 41.1 | render/thread/ms 47.76, render/busy/ms 46.58, main/busy/ms 24.24, stage/render/specialize 13.67, stage/main/update 10.73, stage/render/render 10.71, stage/main/post_update 9.88, render/graph/ms 9.02, stage/render/queue 9.00, encode//sum 8.48, stage/render/graph_submit 6.56, stage/render/prepare_assets 5.31 |
| warm-up | 8.25 | 34.9 | render/thread/ms 42.36, render/busy/ms 40.85, main/busy/ms 17.72, stage/render/specialize 12.93, stage/render/render 12.70, render/graph/ms 10.83, stage/main/post_update 10.18, stage/render/graph_submit 8.31, encode//sum 8.06, stage/render/queue 6.77, stage/main/update 5.25, encode/submit 4.54 |
| measure | 0.43 | 33.5 | render/thread/ms 45.41, render/busy/ms 44.09, encode//sum 20.65, main/busy/ms 19.77, stage/render/render 15.74, render/graph/ms 14.00, stage/render/specialize 13.20, stage/main/post_update 13.13, stage/render/graph_submit 11.48, stage/render/queue 7.03, encode/submit 6.92, encode/finish_wall 4.59 |
| measure | 1.74 | 33.2 | render/thread/ms 40.87, render/busy/ms 39.81, main/busy/ms 18.81, stage/render/render 12.19, stage/render/specialize 12.03, stage/main/post_update 11.76, render/graph/ms 10.67, encode//sum 10.29, stage/render/graph_submit 8.11, stage/render/queue 5.78, encode/submit 4.87, stage/render/prepare_meshes 4.57 |
| measure | 3.19 | 34.3 | render/thread/ms 43.73, render/busy/ms 42.56, main/busy/ms 21.32, stage/render/render 14.21, stage/main/post_update 12.88, stage/render/specialize 12.82, render/graph/ms 12.59, encode//sum 10.21, stage/render/graph_submit 9.24, encode/pass:shadow_point_light/sum 6.85, stage/render/queue 6.45, encode/submit 6.14 |
| measure | 4.64 | 33.5 | render/thread/ms 39.42, render/busy/ms 38.46, main/busy/ms 19.11, stage/render/specialize 12.91, stage/main/post_update 12.33, stage/render/render 10.14, render/graph/ms 8.69, encode//sum 7.82, stage/render/queue 7.18, stage/render/graph_submit 6.60, stage/main/update 4.02, encode/submit 3.65 |
| measure | 5.99 | 33.7 | render/thread/ms 38.44, render/busy/ms 37.49, main/busy/ms 19.13, stage/render/specialize 12.46, stage/main/post_update 12.27, stage/render/render 10.44, render/graph/ms 8.88, encode//sum 7.07, stage/render/graph_submit 6.78, stage/render/queue 6.39, stage/main/update 4.27, encode/submit 3.82 |
| measure | 7.35 | 34.6 | render/thread/ms 38.98, render/busy/ms 37.84, main/busy/ms 18.63, stage/render/specialize 12.48, stage/main/post_update 11.96, stage/render/render 11.40, render/graph/ms 9.78, encode//sum 9.43, stage/render/graph_submit 7.37, stage/render/queue 6.31, encode/submit 4.41, stage/main/update 3.89 |
| measure | 8.69 | 33.3 | render/thread/ms 42.00, render/busy/ms 40.92, main/busy/ms 19.29, stage/render/render 13.15, stage/render/specialize 12.46, stage/main/post_update 12.21, render/graph/ms 11.51, encode//sum 9.47, stage/render/graph_submit 8.46, stage/render/queue 6.51, encode/submit 5.05, stage/main/update 4.32 |
| measure | 10.02 | 32.5 | render/thread/ms 39.51, render/busy/ms 38.51, main/busy/ms 18.42, stage/render/specialize 12.48, stage/main/post_update 11.27, stage/render/render 10.77, render/graph/ms 9.27, encode//sum 7.92, stage/render/graph_submit 7.04, stage/render/queue 6.40, stage/main/update 4.57, encode/submit 3.96 |
| measure | 11.34 | 33.5 | render/thread/ms 41.00, render/busy/ms 39.87, main/busy/ms 19.45, stage/render/specialize 12.61, stage/render/render 12.45, stage/main/post_update 11.78, render/graph/ms 10.85, encode//sum 9.32, stage/render/graph_submit 8.20, stage/render/queue 6.39, encode/submit 4.79, stage/main/update 4.22 |
| measure | 15.73 | 33.2 | render/thread/ms 41.86, render/busy/ms 40.85, main/busy/ms 18.66, stage/render/specialize 13.64, stage/main/post_update 11.68, stage/render/render 10.84, render/graph/ms 9.40, stage/render/queue 7.92, encode//sum 7.87, stage/render/graph_submit 7.35, stage/main/update 4.10, encode/submit 4.08 |
| measure | 18.01 | 32.5 | render/thread/ms 40.53, render/busy/ms 39.35, main/busy/ms 17.93, stage/render/specialize 13.37, stage/main/post_update 10.84, stage/render/render 10.55, render/graph/ms 8.97, encode//sum 7.37, stage/render/graph_submit 6.94, stage/render/queue 6.58, stage/main/update 4.06, encode/submit 3.92 |
| measure | 18.17 | 31.8 | main/busy/ms 38.33, stage/main/update 23.85, stream/drive/ms 20.23, render/thread/ms 18.34, render/busy/ms 17.36, stage/main/post_update 11.67, stage/render/render 10.28, render/graph/ms 8.75, encode//sum 7.95, stage/render/graph_submit 6.39, encode/submit 4.13, encode/finish_wall 2.35 |
| measure | 19.41 | 32.4 | render/thread/ms 39.27, render/busy/ms 38.22, main/busy/ms 17.67, stage/render/specialize 13.15, stage/main/post_update 10.87, stage/render/render 9.45, render/graph/ms 8.09, stage/render/queue 7.57, encode//sum 7.33, stage/render/graph_submit 6.20, stage/main/update 4.11, encode/submit 3.72 |
| measure | 20.80 | 32.0 | render/thread/ms 39.68, render/busy/ms 38.69, main/busy/ms 19.45, stage/render/specialize 13.16, stage/main/post_update 11.88, stage/render/render 10.56, render/graph/ms 9.21, encode//sum 7.69, stage/render/queue 7.16, stage/render/graph_submit 7.03, encode/submit 4.50, stage/main/update 4.15 |
| measure | 23.04 | 38.0 | render/thread/ms 49.62, render/busy/ms 48.54, main/busy/ms 25.07, stage/render/specialize 17.10, stage/render/render 13.73, stage/main/post_update 12.64, render/graph/ms 12.00, encode//sum 9.66, stage/render/graph_submit 9.25, stage/render/queue 7.43, stage/main/update 6.54, encode/pass:shadow_point_light/sum 5.73 |
| measure | 24.60 | 33.7 | render/thread/ms 44.57, render/busy/ms 43.51, main/busy/ms 19.24, stage/render/render 13.79, stage/render/specialize 13.33, render/graph/ms 12.20, stage/main/post_update 12.07, encode//sum 9.34, stage/render/graph_submit 8.87, stage/render/queue 7.45, encode/submit 4.91, encode/pass:shadow_point_light/sum 4.89 |
| measure | 27.30 | 41.4 | render/thread/ms 50.87, render/busy/ms 49.75, main/busy/ms 26.52, stage/render/render 16.89, render/graph/ms 15.18, stage/render/specialize 15.00, stage/main/post_update 13.13, encode//sum 12.72, stage/render/graph_submit 11.78, encode/pass:shadow_point_light/sum 11.11, stage/render/queue 7.87, encode/submit 7.61 |
| measure | 28.90 | 32.4 | render/thread/ms 40.99, render/busy/ms 39.88, main/busy/ms 19.51, stage/render/specialize 12.73, stage/main/post_update 12.36, stage/render/render 11.97, render/graph/ms 10.33, encode//sum 9.04, stage/render/graph_submit 7.70, stage/render/queue 6.37, encode/submit 5.01, encode/pass:shadow_point_light/sum 4.80 |
| measure | 30.50 | 41.3 | render/thread/ms 46.97, render/busy/ms 45.89, main/busy/ms 21.76, stage/render/specialize 14.34, stage/render/render 13.67, stage/main/post_update 13.19, render/graph/ms 12.13, encode//sum 9.48, stage/render/graph_submit 8.95, stage/render/queue 8.11, encode/pass:shadow_point_light/sum 6.28, stage/main/update 5.37 |
| measure | 32.01 | 33.4 | render/thread/ms 41.29, render/busy/ms 40.33, main/busy/ms 20.49, stage/render/specialize 13.09, stage/main/post_update 12.06, stage/render/render 11.83, render/graph/ms 10.37, encode//sum 8.02, stage/render/graph_submit 7.85, stage/render/queue 6.61, encode/submit 4.58, stage/main/update 4.49 |
| measure | 35.36 | 38.1 | render/thread/ms 43.77, render/busy/ms 42.69, main/busy/ms 20.21, stage/render/specialize 14.11, stage/main/post_update 12.42, stage/render/render 11.32, render/graph/ms 9.75, encode//sum 7.67, stage/render/graph_submit 7.49, stage/render/queue 6.87, stage/main/update 4.58, encode/submit 4.35 |
| measure | 36.76 | 33.6 | render/thread/ms 41.33, render/busy/ms 40.22, main/busy/ms 21.36, stage/render/specialize 13.68, stage/main/post_update 11.74, stage/render/render 11.50, render/graph/ms 9.86, encode//sum 8.17, stage/render/graph_submit 7.44, stage/render/queue 7.03, encode/submit 4.58, stage/main/update 4.16 |
| measure | 37.48 | 35.4 | render/thread/ms 39.91, render/busy/ms 38.85, main/busy/ms 18.00, stage/render/specialize 13.22, stage/main/post_update 10.88, stage/render/render 10.85, render/graph/ms 9.37, encode//sum 7.83, stage/render/queue 6.98, stage/render/graph_submit 6.89, stage/main/update 4.54, encode/submit 4.33 |
| measure | 40.79 | 34.7 | render/thread/ms 40.17, render/busy/ms 39.07, main/busy/ms 18.53, stage/render/specialize 13.01, stage/main/post_update 10.69, stage/render/render 9.77, render/graph/ms 8.19, encode//sum 6.72, stage/render/graph_submit 6.28, stage/render/queue 6.09, stage/main/update 4.42, encode/submit 3.93 |
| measure | 43.24 | 34.8 | render/thread/ms 41.82, render/busy/ms 40.88, main/busy/ms 17.45, stage/render/specialize 13.50, stage/render/render 10.94, stage/main/post_update 10.30, render/graph/ms 9.61, stage/render/graph_submit 7.70, encode//sum 7.54, stage/render/queue 6.72, stage/main/update 4.52, encode/submit 3.98 |
| measure | 45.70 | 36.9 | render/thread/ms 43.19, render/busy/ms 42.08, main/busy/ms 18.22, stage/render/specialize 13.77, stage/render/render 10.73, stage/main/post_update 10.47, render/graph/ms 9.36, stage/render/graph_submit 7.40, encode//sum 7.20, stage/render/queue 7.12, stage/main/update 4.75, stage/render/prepare_meshes 4.55 |
| measure | 59.04 | 47.8 | render/thread/ms 59.47, render/busy/ms 57.75, main/busy/ms 30.99, stage/render/render 18.26, stage/main/post_update 16.77, render/graph/ms 15.85, stage/render/specialize 14.23, stage/render/graph_submit 12.86, encode//sum 11.31, stage/main/update 9.51, encode/submit 8.78, stage/render/prepare_meshes 7.25 |
| measure | 62.57 | 33.7 | render/thread/ms 41.20, render/busy/ms 40.08, main/busy/ms 18.47, stage/render/specialize 13.81, stage/render/render 10.92, render/graph/ms 9.45, stage/main/post_update 9.32, stage/render/graph_submit 7.42, encode//sum 7.34, stage/render/queue 6.20, stage/main/update 5.76, encode/submit 4.10 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=skin-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 6%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **78%** (min 50%) · 2625 MHz (min 2625) · memory 8101 MHz · 79.4 W · 76 °C (max 81) · VRAM 6802 MiB · throttled 100% of samples

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
- `render.skin.skip_unchanged=on (command line: --set)`
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
