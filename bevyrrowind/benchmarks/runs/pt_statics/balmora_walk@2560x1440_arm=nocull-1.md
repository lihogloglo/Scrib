# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5983 frames over 65.7 s · **median 10.65 ms (94 fps)** · mean 10.97 · p95 13.59 · p99 17.74 (1% low 56 fps) · worst 40.11 ms · 95 spikes over 1.5× median · 70.4 stutters/min (work 72.2/min)
- warm-up: 323 frames over 9.4 s, worst 2531.7 ms
- hitches (> 26.6 ms): **31** measured, 60 in warm-up
- timed GPU passes: 6.80 ms of a 10.97 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.65 ms, limit 11 ms
- **FAIL** frame_ms: 77 stutters (> 1.5x moving median of 60) in 1.09 min = 70.37/min, limit 1/min
- **FAIL** worst frame 40.11 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.905 | 2.037 | 2.072 | 0.94 | 0.176 |
| main_opaque_pass_3d | 1.534 | 1.640 | 2.063 | 0.94 | 0.062 |
| early prepass | 0.470 | 0.502 | 0.738 | 0.94 | 0.083 |
| bloom | 0.294 | 0.315 | 0.321 | 0.94 | 0.063 |
| atmosphere_luts | 0.284 | 0.303 | 0.365 | 0.94 | 0.003 |
| water_velocity | 0.260 | 0.278 | 0.347 | 0.94 | 0.002 |
| water_sspr | 0.189 | 0.202 | 0.203 | 0.94 | 0.002 |
| ssao | 0.176 | 0.188 | 0.193 | 0.94 | 0.006 |
| auxiliary_views | 0.144 | 0.154 | 0.531 | 0.94 | 0.153 |
| uniform_allocation | 0.141 | 0.151 | 0.208 | 0.94 | 0.025 |
| atmos_froxel_inject | 0.137 | 0.147 | 0.150 | 0.94 | 0.001 |
| clouds_trace | 0.130 | 0.139 | 0.231 | 0.94 | 0.001 |
| water_fft | 0.123 | 0.131 | 0.138 | 0.94 | 0.003 |
| upscaling | 0.110 | 0.118 | 0.126 | 0.94 | 0.004 |
| bin_unpacking | 0.103 | 0.110 | 0.148 | 0.94 | 0.017 |
| clouds_composite | 0.099 | 0.106 | 0.109 | 0.94 | 0.002 |
| main_transmissive_pass_3d | 0.082 | 0.087 | 0.180 | 0.94 | 0.008 |
| tonemapping | 0.079 | 0.084 | 0.088 | 0.94 | 0.003 |
| clustering | 0.060 | 0.064 | 0.067 | 0.94 | 0.018 |
| render_sky | 0.048 | 0.052 | 0.053 | 0.94 | 0.002 |
| clouds_resolve | 0.045 | 0.048 | 0.049 | 0.94 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.412 | 0.416 | 0.11 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.94 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.67 | 0.002 |
| early_mesh_preprocessing | 0.037 | 0.040 | 0.057 | 0.94 | 0.004 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.83 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.94 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.94 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.221 | 0.233 | 0.11 | 0.004 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.94 | 0.002 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.94 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.030 | 0.94 | 0.170 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.94 | 0.001 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.94 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.94 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.574 | 11.964 | 36.50 | 57280.2 |
| render/busy/ms | 9.039 | 11.451 | 35.89 | 54081.0 |
| main/busy/ms | 8.616 | 10.226 | 28.31 | 51551.8 |
| stage/render/render | 5.473 | 6.934 | 14.85 | 32745.8 |
| stage/main/post_update | 5.201 | 6.474 | 9.88 | 31117.6 |
| render/graph/ms | 4.715 | 6.137 | 13.78 | 28208.6 |
| encode//sum | 4.156 | 5.661 | 21.88 | 24864.5 |
| stage/render/graph_submit | 3.482 | 4.552 | 9.76 | 20832.5 |
| encode/submit | 2.201 | 3.024 | 7.22 | 13170.6 |
| stage/main/update | 2.148 | 2.543 | 22.43 | 12849.0 |
| encode/finish_wall | 1.348 | 1.913 | 4.26 | 8066.3 |
| stage/render/camera_driver | 1.179 | 1.647 | 3.95 | 7051.8 |
| encode/pass:shadow_point_light/sum | 0.881 | 3.078 | 16.11 | 5268.1 |
| encode/pass:shadow_directional_light/sum | 0.819 | 1.511 | 2.56 | 4898.9 |
| stage/render/prepare_write_phase_buffers | 0.792 | 1.177 | 2.27 | 4740.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.739 | 1.067 | 1.57 | 4420.0 |
| encode/pass:early prepass/sum | 0.713 | 1.053 | 1.56 | 4266.1 |
| stage/render/prepare_assets | 0.565 | 0.734 | 12.76 | 3383.3 |
| stage/render/prepare_meshes | 0.495 | 0.692 | 3.89 | 2964.0 |
| bigspace/total/ms | 0.492 | 1.152 | 2.26 | 2945.5 |
| stage/render/prepare_resources | 0.451 | 0.540 | 1.10 | 2695.5 |
| render/present/ms | 0.443 | 0.700 | 2.11 | 2650.6 |
| stage/main/first | 0.404 | 0.526 | 6.28 | 2414.6 |
| stage/main/pre_update | 0.353 | 0.504 | 1.63 | 2112.1 |
| stage/render/specialize | 0.352 | 0.454 | 14.40 | 2107.7 |
| stage/render/queue | 0.308 | 0.453 | 7.69 | 1840.4 |
| terrain/update/ms | 0.308 | 0.399 | 2.10 | 1840.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.295 | 0.435 | 1.01 | 1767.7 |
| stage/render/prepare_bind_groups | 0.285 | 0.524 | 2.60 | 1705.1 |
| terrain/select/ms | 0.274 | 0.356 | 0.87 | 1640.4 |
| stage/render/prepare_batch_phases | 0.260 | 0.367 | 0.61 | 1556.7 |
| controller/feed/ms | 0.260 | 0.325 | 1.07 | 1556.2 |
| stream/drive/ms | 0.242 | 0.316 | 20.44 | 1450.3 |
| stage/main/fixed_main_loop | 0.217 | 0.368 | 1.37 | 1297.6 |
| bigspace/low_precision/ms | 0.206 | 0.612 | 1.64 | 1232.0 |
| far/select/ms | 0.194 | 0.259 | 0.52 | 1162.3 |
| stage/render/create_views | 0.193 | 0.246 | 0.86 | 1152.4 |
| stream/tick/ms | 0.187 | 0.249 | 0.60 | 1119.0 |
| stage/render/extract_commands | 0.167 | 0.197 | 0.47 | 1001.7 |
| stage/render/prepare_flush | 0.163 | 0.305 | 1.14 | 974.1 |
| anim/evaluate/ms | 0.152 | 0.266 | 1.36 | 908.1 |
| anim/evaluate_cpu/ms | 0.148 | 0.181 | 0.79 | 883.8 |
| controller/move/ms | 0.143 | 0.341 | 0.53 | 857.3 |
| stage/render/prepare_views | 0.134 | 0.166 | 0.47 | 803.3 |
| bigspace/high_precision/ms | 0.129 | 0.370 | 1.61 | 769.9 |
| stage/main/last | 0.118 | 0.167 | 1.58 | 704.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.167 | 0.76 | 597.1 |
| render/acquire/ms | 0.092 | 0.116 | 0.30 | 548.6 |
| stage/render/cleanup | 0.078 | 0.200 | 0.66 | 467.3 |
| stage/render/phase_sort | 0.063 | 0.093 | 0.33 | 374.0 |
| stage/render/prepare_collect_phase_buffers | 0.062 | 0.131 | 0.51 | 369.6 |
| pipelines/compile_ms | 0.047 | 0.000 | 120.45 | 278.5 |
| physics/step/ms | 0.046 | 0.085 | 0.61 | 277.0 |
| pipelines/front_ms | 0.045 | 0.000 | 116.95 | 267.5 |
| physics/insert/ms | 0.033 | 0.360 | 0.88 | 196.4 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.60 | 181.7 |
| weather/map/ms | 0.025 | 0.032 | 0.24 | 148.8 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.46 | 85.0 |
| anim/apply/ms | 0.011 | 0.019 | 0.05 | 68.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.45 | 63.9 |
| controller/avoid/ms | 0.010 | 0.014 | 0.09 | 62.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.21 | 60.3 |
| script/update/ms | 0.007 | 0.008 | 0.19 | 40.6 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 31.7 |
| stream/despawn/ms | 0.005 | 0.000 | 1.12 | 29.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 28.9 |
| script/ui/ms | 0.005 | 0.006 | 0.04 | 27.8 |
| controller/write/ms | 0.004 | 0.005 | 0.21 | 21.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 14.4 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.56 | 11.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.31 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 593 |
| crowd/c0/meshes/count | count | 353 | 646 | 156 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5892 | 5571 |
| crowd/c1/meshes/count | count | 1179 | 1255 | 1177 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4613 | 10.1 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.43 | 1.53 | 0.93 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.46 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.45 | 0.01 |
| encode/finish_critical | ms | 0.95 | 2.56 | 0.93 |
| encode/pass:early prepass/max | ms | 0.72 | 1.56 | 0.80 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.57 | 0.71 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.76 | 0.11 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 1.01 | 0.31 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 2.56 | 0.91 |
| encode/pass:shadow_point_light/max | ms | 0.79 | 1.58 | 0.94 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.60 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.96 | 35.89 | 8.98 |
| gpu/timed/ms | ms | 7.98 | 11.49 | 8.25 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 249 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3902 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3902 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 4104 | 4330 | 4330 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 4104 | 4330 | 4330 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 204 | 5 |
| phases/shadow/bins | count | 14.4 k | 19.1 k | 12.1 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 40.4 k | 56.7 k | 39.8 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 250 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 19 | 569 | 6 |
| preprocess/bind_groups_created | count | 19 | 569 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 235 | 125 |
| process/cpu_usage | % | 25 | 37.98 | 24.04 |
| process/mem_usage | GiB | 5.51 | 5.68 | 5.67 |
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
| skin/blocks_changed/count | count | 14 | 121 | 15 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 367.5 k | 812.8 k | 485.1 k |
| skin/joints_written/count | count | 5742 | 12.7 k | 7580 |
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
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.17 | 49.83 | 29.64 |
| system/mem_usage | % | 67.30 | 67.86 | 67.82 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 72 | 253 | 55 |
| views/all/bins | count | 23.1 k | 27.7 k | 21.7 k |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 69.0 k | 85.3 k | 69.4 k |
| views/all/pipelines | count | 251 | 297 | 77 |
| views/all/pipelines_distinct | count | 70 | 76 | 61 |
| views/all/work_items | count | 41.4 k | 81.8 k | 42.2 k |
| views/cascade/batch_sets | count | 11 | 54 | 5 |
| views/cascade/bins | count | 12.0 k | 15.3 k | 12.1 k |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 39.0 k | 53.1 k | 39.8 k |
| views/cascade/pipelines | count | 38 | 55 | 31 |
| views/cascade/work_items | count | 11.7 k | 52.8 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 3129 | 4306 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3388 | 4592 | 0 |
| views/lamp_face/pipelines | count | 167 | 201 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 4138 | 0 |
| views/main/batch_sets | count | 51 | 55 | 50 |
| views/main/bins | count | 8672 | 9602 | 9602 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 47 | 51 | 46 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 56.08 | 214 | 335.5 k |
| anim/events/count | count | 0.09 | 3 | 547 |
| assets/Image/added | count | 0.10 | 37 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.62 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1298 | 4676 |
| encode//count | count | 33.88 | 220 | 202.7 k |
| encode/bind_walk/init_textures | count | 12.8 k | 114.7 k | 76.81 M |
| encode/bind_walk/init_textures_skipped | count | 69.1 k | 149.5 k | 413.62 M |
| encode/bind_walk/inits | count | 523 | 1250 | 3.13 M |
| encode/bind_walk/inits_skipped | count | 136 | 204 | 812.3 k |
| encode/bind_walk/merge_textures | count | 12.8 k | 114.6 k | 76.38 M |
| encode/bind_walk/merge_textures_skipped | count | 5501 | 6026 | 32.91 M |
| encode/bind_walk/merges | count | 102 | 339 | 608.5 k |
| encode/bind_walk/merges_skipped | count | 110 | 157 | 659.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5983 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5983 |
| encode/pass:early prepass/count | count | 1 | 1 | 5983 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5983 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5983 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5983 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5983 |
| encode/pass:shadow_point_light/count | count | 1.48 | 48 | 8856 |
| encode/queue/command_buffers | count | 45.38 | 278 | 271.5 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 5.97 M | 13.96 G |
| encode/queue/write_buffer/calls | count | 267 | 850 | 1.60 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.5 k | 3.52 M | 1.06 G |
| encode/queue/write_buffer_with/calls | count | 11.02 | 465 | 65.9 k |
| encode/queue/write_texture/bytes | bytes | 11.9 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5983 |
| physics/inserts | count | 0.37 | 103 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 25 | 378 |
| stream/upload/bytes | bytes | 5658 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1241 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.6 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2531.7 | main/busy/ms 2501, stage/main/post_update 2394, stage/main/update 97.93, surface/advance_cpu_ms 66.66, stage/main/pre_update 5.61, stage/main/last 1.18, surface/cell_upload_ms 0.77, stage/main/first 0.58, stream/drive/ms 0.45, stage/main/fixed_main_loop 0.35, weather/map/ms 0.16, bigspace/total/ms 0.08 |
| warm-up | 3.27 | 733.6 | main/busy/ms 2637, stage/main/post_update 2417, render/thread/ms 701, render/busy/ms 558, stage/render/prepare_assets 491, stage/main/update 206, stage/render/prepare_views 137, render/acquire/ms 137, surface/advance_cpu_ms 66.66, stage/render/create_views 44.48, stage/render/render 16.51, render/graph/ms 9.72 |
| warm-up | 3.44 | 175.2 | render/thread/ms 859, render/busy/ms 628, stage/render/prepare_assets 543, main/busy/ms 170, stage/render/prepare_views 138, render/acquire/ms 138, stage/main/update 117, stage/render/render 111, pipelines/compile_ms 95.43, render/present/ms 94.06, pipelines/driver_ms 74.36, stage/render/create_views 44.94 |
| warm-up | 3.62 | 166.9 | pipelines/wait_ms 4165, pipelines/compile_ms 1843, pipelines/front_ms 1637, pipelines/driver_ms 207, main/busy/ms 145, render/thread/ms 113, render/busy/ms 110, stage/main/post_update 71.29, stage/main/update 60.56, stage/render/prepare_resources 34.86, stage/render/prepare_assets 29.64, stage/render/prepare_bind_groups 24.24 |
| warm-up | 3.69 | 66.6 | pipelines/wait_ms 4311, pipelines/compile_ms 2159, pipelines/front_ms 2029, render/thread/ms 163, render/busy/ms 160, pipelines/driver_ms 130, main/busy/ms 121, stage/render/prepare_assets 82.13, stage/main/update 61.42, stage/main/post_update 46.82, stage/render/prepare_resources 33.59, stage/render/prepare_bind_groups 24.23 |
| warm-up | 3.74 | 46.9 | pipelines/compile_ms 1046, pipelines/front_ms 1003, pipelines/wait_ms 319, render/thread/ms 108, render/busy/ms 107, stage/render/prepare_assets 77.94, pipelines/driver_ms 43.38, main/busy/ms 37.42, stage/main/post_update 21.10, terrain/update/ms 15.25, stage/render/render 12.33, render/graph/ms 10.17 |
| warm-up | 3.77 | 34.0 | pipelines/compile_ms 713, pipelines/front_ms 681, pipelines/wait_ms 306, render/thread/ms 76.96, render/busy/ms 74.81, main/busy/ms 47.04, pipelines/driver_ms 31.93, stage/render/prepare_assets 29.69, encode//sum 23.32, stage/main/post_update 20.94, stage/render/render 15.47, render/graph/ms 13.08 |
| warm-up | 3.80 | 28.9 | pipelines/compile_ms 673, pipelines/front_ms 609, pipelines/wait_ms 162, pipelines/driver_ms 63.61, render/thread/ms 59.90, render/busy/ms 57.78, main/busy/ms 42.90, encode//sum 24.34, stage/main/post_update 18.78, stage/render/prepare_resources 17.97, stage/render/render 15.26, render/graph/ms 13.05 |
| warm-up | 3.97 | 106.7 | pipelines/compile_ms 1178, pipelines/front_ms 1124, render/thread/ms 77.99, render/busy/ms 75.81, main/busy/ms 64.44, pipelines/driver_ms 54.25, stage/render/prepare_assets 51.10, stage/main/update 49.44, stream/drive/ms 43.94, stage/render/render 14.87, render/graph/ms 12.25, stage/main/post_update 10.10 |
| warm-up | 4.12 | 80.5 | pipelines/compile_ms 249, pipelines/front_ms 241, render/thread/ms 61.12, render/busy/ms 58.97, main/busy/ms 52.48, stage/render/prepare_assets 39.46, stage/main/update 33.63, stream/drive/ms 27.33, stage/render/render 14.31, render/graph/ms 11.61, stage/main/post_update 11.34, stage/render/graph_submit 8.35 |
| warm-up | 4.24 | 87.9 | pipelines/compile_ms 1047, pipelines/front_ms 1015, render/thread/ms 75.59, render/busy/ms 74.05, main/busy/ms 62.16, stage/render/prepare_assets 58.32, stage/main/update 39.33, pipelines/driver_ms 31.86, stage/main/post_update 14.49, stage/render/render 9.27, render/graph/ms 7.13, encode//sum 5.64 |
| warm-up | 4.33 | 90.5 | pipelines/compile_ms 1566, pipelines/front_ms 1504, render/thread/ms 130, render/busy/ms 127, stage/render/prepare_assets 96.68, main/busy/ms 65.99, pipelines/driver_ms 62.67, stage/main/update 44.12, stage/render/render 22.44, render/graph/ms 19.89, stage/render/graph_submit 17.30, encode/submit 15.71 |
| warm-up | 4.41 | 80.0 | pipelines/compile_ms 801, pipelines/front_ms 759, pipelines/wait_ms 319, render/thread/ms 147, render/busy/ms 145, main/busy/ms 64.55, stage/render/prepare_assets 61.40, pipelines/driver_ms 41.96, stage/render/render 41.27, render/graph/ms 38.47, stage/render/graph_submit 34.56, stage/main/post_update 33.90 |
| warm-up | 4.53 | 121.5 | pipelines/compile_ms 2310, pipelines/front_ms 2238, pipelines/wait_ms 563, render/thread/ms 180, render/busy/ms 179, main/busy/ms 82.32, pipelines/driver_ms 71.58, stage/render/prepare_resources 54.91, stage/render/render 45.92, render/graph/ms 43.14, stage/main/post_update 34.47, encode//sum 34.01 |
| warm-up | 4.61 | 77.0 | pipelines/compile_ms 2982, pipelines/front_ms 2886, pipelines/wait_ms 493, render/thread/ms 179, render/busy/ms 177, pipelines/driver_ms 96.28, encode//sum 77.28, main/busy/ms 73.45, stage/render/render 55.81, render/graph/ms 53.01, stage/render/prepare_resources 43.02, stage/render/graph_submit 37.78 |
| warm-up | 4.70 | 87.5 | pipelines/compile_ms 1188, pipelines/front_ms 1148, pipelines/wait_ms 303, render/thread/ms 150, render/busy/ms 148, encode//sum 92.28, main/busy/ms 65.35, stage/render/render 51.67, render/graph/ms 49.17, pipelines/driver_ms 40.28, stage/render/graph_submit 39.62, encode/finish_wall 26.65 |
| warm-up | 4.74 | 41.3 | pipelines/compile_ms 1257, pipelines/front_ms 1221, pipelines/wait_ms 126, render/thread/ms 107, render/busy/ms 105, main/busy/ms 103, stage/main/update 77.95, stream/drive/ms 59.35, pipelines/driver_ms 36.72, encode//sum 29.88, stage/render/render 23.97, stage/render/prepare_assets 21.83 |
| warm-up | 4.82 | 82.5 | pipelines/compile_ms 2461, pipelines/front_ms 2391, pipelines/wait_ms 177, main/busy/ms 116, stage/main/update 91.50, pipelines/driver_ms 70.35, render/thread/ms 61.38, render/busy/ms 59.12, stream/drive/ms 58.95, stage/render/render 27.22, render/graph/ms 23.92, stage/render/prepare_assets 17.30 |
| warm-up | 4.91 | 89.6 | pipelines/compile_ms 1716, pipelines/front_ms 1669, main/busy/ms 150, pipelines/wait_ms 129, stage/main/update 118, render/thread/ms 92.17, render/busy/ms 89.66, stream/drive/ms 57.58, pipelines/driver_ms 47.16, stage/render/render 34.10, render/graph/ms 30.63, stage/render/prepare_assets 28.89 |
| warm-up | 5.03 | 122.8 | pipelines/compile_ms 1822, pipelines/front_ms 1768, main/busy/ms 131, render/thread/ms 110, render/busy/ms 108, stage/main/update 96.49, stream/drive/ms 58.71, stage/render/prepare_assets 55.57, pipelines/driver_ms 53.65, pipelines/wait_ms 39.25, pipelines/lock_ms 36.38, stage/main/post_update 25.58 |
| warm-up | 5.08 | 49.5 | pipelines/compile_ms 2352, pipelines/front_ms 2288, render/thread/ms 88.80, render/busy/ms 86.99, main/busy/ms 84.99, pipelines/driver_ms 63.55, stage/main/update 58.64, stage/render/prepare_assets 50.28, stream/drive/ms 43.37, pipelines/wait_ms 29.62, pipelines/lock_ms 28.54, stage/render/render 17.54 |
| warm-up | 5.17 | 84.0 | pipelines/compile_ms 1158, pipelines/front_ms 1121, main/busy/ms 93.01, render/thread/ms 83.56, render/busy/ms 81.49, stage/main/update 63.81, stream/drive/ms 43.20, pipelines/driver_ms 36.88, stage/render/prepare_assets 35.26, stage/render/render 28.79, render/graph/ms 26.28, stage/render/graph_submit 23.00 |
| warm-up | 5.21 | 40.0 | pipelines/compile_ms 350, pipelines/front_ms 332, main/busy/ms 81.59, render/thread/ms 74.90, render/busy/ms 72.82, stage/main/update 55.64, stream/drive/ms 33.40, stage/render/prepare_assets 27.34, stage/render/render 26.59, render/graph/ms 24.18, stage/render/graph_submit 21.06, pipelines/driver_ms 17.88 |
| warm-up | 5.27 | 58.8 | pipelines/compile_ms 457, pipelines/front_ms 445, main/busy/ms 70.72, render/thread/ms 58.56, render/busy/ms 56.80, stage/main/update 53.20, stream/drive/ms 32.86, stage/render/prepare_assets 22.58, stage/render/render 12.65, pipelines/driver_ms 12, render/graph/ms 10.41, stage/main/post_update 9.52 |
| warm-up | 5.30 | 34.0 | pipelines/compile_ms 1039, pipelines/front_ms 1014, render/thread/ms 58.53, render/busy/ms 56.74, main/busy/ms 43.16, pipelines/driver_ms 25.68, stage/main/update 20.79, stage/render/prepare_assets 16.39, stage/main/post_update 14.83, stage/render/render 13.86, render/graph/ms 11.47, stage/render/specialize 9.97 |
| warm-up | 5.33 | 31.7 | pipelines/compile_ms 583, pipelines/front_ms 569, render/thread/ms 57.05, render/busy/ms 55.22, main/busy/ms 37.17, stage/render/render 15.41, stage/main/update 15.05, stage/main/post_update 14.57, pipelines/driver_ms 13.68, render/graph/ms 12.84, stage/render/prepare_assets 11.15, stage/render/specialize 10.60 |
| warm-up | 5.37 | 38.6 | render/thread/ms 60.94, render/busy/ms 59.45, main/busy/ms 31.79, stage/render/prepare_assets 19.82, stage/main/update 16.02, stage/render/render 12.94, render/graph/ms 10.67, stage/render/specialize 9.93, stage/main/post_update 9.47, encode//sum 9.21, stage/render/graph_submit 7.71, stream/drive/ms 5.73 |
| warm-up | 5.41 | 43.1 | pipelines/compile_ms 322, pipelines/front_ms 309, render/thread/ms 72.43, render/busy/ms 70.91, main/busy/ms 45.43, stage/main/post_update 22.45, stage/render/prepare_assets 20.46, stage/render/prepare_write_phase_buffers 17.75, stage/main/update 17.04, pipelines/driver_ms 13.35, stage/render/render 12.95, render/graph/ms 10.82 |
| warm-up | 5.50 | 60.0 | pipelines/compile_ms 1764, pipelines/front_ms 1719, render/thread/ms 70.10, render/busy/ms 68.37, pipelines/driver_ms 45.23, main/busy/ms 41.61, stage/render/prepare_assets 35.73, stage/main/update 19.48, stage/main/post_update 13.83, stage/render/render 13.67, render/graph/ms 11.28, encode//sum 7.84 |
| warm-up | 5.54 | 34.3 | pipelines/compile_ms 1449, pipelines/front_ms 1415, render/thread/ms 73.55, render/busy/ms 72.01, main/busy/ms 48.64, stage/render/prepare_assets 41.98, pipelines/driver_ms 34.30, stage/main/update 28.06, stream/drive/ms 15.46, stage/render/render 13.83, stage/main/post_update 13.09, render/graph/ms 11.43 |
| warm-up | 5.57 | 36.1 | pipelines/compile_ms 110, pipelines/front_ms 104, render/thread/ms 57.28, render/busy/ms 55.77, main/busy/ms 46.01, stage/render/prepare_assets 25.31, stage/main/update 24.22, stream/drive/ms 15.07, stage/main/post_update 13.13, stage/render/render 12.85, render/graph/ms 10.47, encode//sum 8.96 |
| warm-up | 5.62 | 29.2 | pipelines/compile_ms 157, pipelines/front_ms 150, render/thread/ms 46.75, render/busy/ms 45.28, main/busy/ms 26.90, stage/render/prepare_assets 22.25, stage/main/post_update 12.44, stage/render/render 11.45, render/graph/ms 9.29, encode//sum 8.94, stage/main/update 8.79, pipelines/driver_ms 7.31 |
| warm-up | 5.67 | 28.9 | pipelines/compile_ms 285, pipelines/front_ms 274, render/thread/ms 45.53, render/busy/ms 43.99, main/busy/ms 31.29, stage/render/prepare_assets 20.16, stage/render/render 13.62, stage/main/post_update 13.24, stage/main/update 11.64, pipelines/driver_ms 11.57, render/graph/ms 11.42, encode//sum 8.34 |
| warm-up | 5.71 | 32.4 | pipelines/compile_ms 382, pipelines/front_ms 368, render/thread/ms 54.23, render/busy/ms 52.91, main/busy/ms 32.98, stage/render/prepare_assets 31.64, pipelines/driver_ms 14.61, stage/main/post_update 13.68, stage/render/render 10.61, stage/main/update 10.18, render/graph/ms 8.45, encode//sum 7.34 |
| warm-up | 5.74 | 36.6 | pipelines/compile_ms 720, pipelines/front_ms 701, render/thread/ms 63.10, render/busy/ms 61.71, main/busy/ms 41.58, stage/render/prepare_assets 35.12, pipelines/driver_ms 18.80, stage/main/update 18.73, stage/main/post_update 13.76, stage/render/render 13.72, render/graph/ms 11.42, encode//sum 9.24 |
| warm-up | 5.82 | 45.3 | pipelines/compile_ms 221, pipelines/front_ms 215, render/thread/ms 63.30, render/busy/ms 61.75, stage/render/prepare_assets 39.00, main/busy/ms 35.91, stage/main/post_update 15.69, stage/main/update 13.27, stage/render/render 11.99, render/graph/ms 9.77, encode//sum 8.74, stage/render/graph_submit 6.88 |
| warm-up | 5.86 | 29.8 | pipelines/compile_ms 421, pipelines/front_ms 410, render/thread/ms 39.67, render/busy/ms 38.10, main/busy/ms 31.39, stage/render/render 14.36, render/graph/ms 12.14, stage/main/post_update 12.04, stage/render/prepare_assets 11.52, stage/main/update 11.51, pipelines/driver_ms 11.26, stage/render/graph_submit 9.21 |
| warm-up | 6.17 | 29.5 | pipelines/compile_ms 97.76, pipelines/front_ms 92.31, render/thread/ms 47.32, render/busy/ms 45.99, main/busy/ms 34.06, stage/main/post_update 21.29, stage/render/render 14.49, stage/render/prepare_resources 12.94, render/graph/ms 12.66, stage/render/prepare_assets 12.57, stage/render/graph_submit 9.52, stage/main/update 8.25 |
| warm-up | 6.49 | 31.5 | pipelines/compile_ms 140, pipelines/front_ms 136, render/thread/ms 36.55, render/busy/ms 35.41, main/busy/ms 28.20, stage/render/prepare_assets 16.40, stage/main/update 13.43, stage/main/post_update 9.19, stage/render/render 7.90, encode//sum 6.62, render/graph/ms 6.45, stage/render/graph_submit 4.35 |
| warm-up | 6.73 | 32.7 | pipelines/compile_ms 87.72, pipelines/front_ms 80.41, render/thread/ms 53.30, render/busy/ms 51.88, main/busy/ms 43.43, stage/main/update 17.72, stage/main/post_update 17.54, stage/render/render 15.12, render/graph/ms 13.21, stage/render/specialize 10.21, stage/render/graph_submit 9.41, encode//sum 9.35 |
| warm-up | 6.76 | 30.6 | pipelines/compile_ms 255, pipelines/front_ms 241, render/thread/ms 52.23, render/busy/ms 51.01, main/busy/ms 40.81, stage/main/update 18.49, stage/main/post_update 14.84, pipelines/driver_ms 13.89, stage/render/render 13.82, stage/render/prepare_assets 13.13, render/graph/ms 12.03, encode//sum 9.34 |
| warm-up | 6.88 | 32.2 | render/thread/ms 46.44, render/busy/ms 45.35, main/busy/ms 35.48, stage/main/update 17.33, stage/render/prepare_assets 13.37, stage/main/post_update 12.36, stage/render/specialize 9.99, stage/render/render 7.37, encode//sum 6.55, render/graph/ms 5.81, stage/render/queue 5.74, stage/render/graph_submit 3.81 |
| warm-up | 7.01 | 36.4 | render/thread/ms 52.61, render/busy/ms 51.13, main/busy/ms 37.55, stage/render/prepare_assets 23.21, stage/main/update 21.20, stage/render/render 15.08, stream/drive/ms 14.21, render/graph/ms 12.89, stage/main/post_update 11.77, encode//sum 11.50, stage/render/graph_submit 9.90, encode/submit 5.41 |
| warm-up | 7.07 | 37.4 | render/thread/ms 57.27, render/busy/ms 56.05, main/busy/ms 33.06, stage/render/prepare_assets 22.19, stage/main/post_update 15.12, stage/main/update 12.19, stage/render/render 9.88, stage/render/specialize 8.54, render/graph/ms 8.12, encode//sum 6.92, stage/render/graph_submit 5.51, stream/drive/ms 5.19 |
| warm-up | 7.12 | 31.7 | pipelines/compile_ms 155, pipelines/front_ms 151, render/thread/ms 47.14, render/busy/ms 46.06, main/busy/ms 28.92, stage/render/prepare_assets 24.01, stage/main/update 11.61, stage/main/post_update 11.31, stage/render/render 9.94, render/graph/ms 8.25, encode//sum 7.99, stage/render/graph_submit 5.75 |
| warm-up | 7.43 | 29.2 | render/thread/ms 38.79, render/busy/ms 37.77, main/busy/ms 19.71, stage/render/render 11.64, render/graph/ms 10.14, stage/main/post_update 10.07, stage/render/specialize 9.97, stage/render/graph_submit 7.82, encode//sum 7.50, stage/main/update 6.45, stage/render/queue 6.26, encode/submit 4.55 |
| measure | 7.06 | 29.2 | render/thread/ms 36.72, render/busy/ms 35.55, main/busy/ms 18.67, stage/render/render 12.09, stage/main/post_update 11.27, render/graph/ms 10.37, stage/render/specialize 9.55, encode//sum 8.98, stage/render/graph_submit 7.80, stage/render/queue 6.57, stage/main/update 5.07, encode/submit 4.90 |
| measure | 9.75 | 29.0 | render/thread/ms 39.14, render/busy/ms 38.06, main/busy/ms 22.17, stage/main/post_update 13.54, stage/render/render 13.10, encode//sum 11.26, render/graph/ms 10.99, stage/render/specialize 9.69, stage/render/graph_submit 8.10, stage/render/queue 6.19, stage/main/update 5.26, encode/submit 4.92 |
| measure | 15.72 | 30.5 | render/thread/ms 36.79, render/busy/ms 35.77, main/busy/ms 19.31, stage/main/post_update 12.15, stage/render/render 10.87, stage/render/specialize 10.59, render/graph/ms 9.37, encode//sum 8.49, stage/render/graph_submit 7.14, stage/render/queue 6.90, stage/main/update 4.15, encode/submit 4.13 |
| measure | 18.18 | 29.9 | main/busy/ms 37.95, stage/main/update 24.61, render/thread/ms 20.86, stream/drive/ms 20.67, render/busy/ms 19.79, stage/render/render 11.79, stage/main/post_update 10.64, encode//sum 10.39, render/graph/ms 10.23, stage/render/graph_submit 7.41, encode/submit 4.79, encode/pass:shadow_point_light/sum 4.47 |
| measure | 19.43 | 28.8 | render/thread/ms 36.59, render/busy/ms 35.56, main/busy/ms 17.49, stage/render/render 11.51, encode//sum 10.19, stage/main/post_update 9.94, render/graph/ms 9.89, stage/render/specialize 9.52, stage/render/graph_submit 7.32, stage/render/queue 6.74, stage/main/update 4.85, encode/submit 4.24 |
| measure | 23.03 | 33.8 | render/thread/ms 42.85, render/busy/ms 41.81, main/busy/ms 26.67, stage/main/post_update 14.28, stage/render/render 12.99, stage/render/specialize 11.96, render/graph/ms 11.44, encode//sum 9.49, stage/render/graph_submit 8.66, stage/render/queue 7.97, stage/main/update 6.23, encode/submit 5.25 |
| measure | 27.29 | 34.2 | render/thread/ms 44.20, render/busy/ms 43.14, main/busy/ms 27.70, stage/render/render 15.15, stage/main/post_update 14.48, render/graph/ms 13.50, encode//sum 11.89, stage/render/specialize 11.33, stage/render/graph_submit 10.46, stage/render/queue 8.04, encode/pass:shadow_point_light/sum 7.67, encode/submit 6.88 |
| measure | 28.82 | 29.4 | render/thread/ms 40.11, render/busy/ms 39.04, main/busy/ms 18.86, stage/render/render 14.09, render/graph/ms 12.48, stage/main/post_update 11.73, encode//sum 11.05, stage/render/specialize 10.53, stage/render/graph_submit 9.35, stage/render/queue 6.95, encode/pass:shadow_point_light/sum 6.55, encode/submit 5.84 |
| measure | 30.48 | 36.0 | render/thread/ms 40.83, render/busy/ms 39.81, main/busy/ms 21.20, stage/main/post_update 12.56, stage/render/specialize 11.80, stage/render/render 11.24, render/graph/ms 9.67, encode//sum 9.05, stage/render/queue 7.65, stage/render/graph_submit 7.31, stage/main/update 5.66, encode/submit 4.26 |
| measure | 35.35 | 33.7 | render/thread/ms 41.49, render/busy/ms 40.36, main/busy/ms 20.97, stage/render/render 13.93, stage/main/post_update 13.50, render/graph/ms 12.21, stage/render/specialize 10.61, encode//sum 9.58, stage/render/graph_submit 9.41, stage/render/queue 7.08, encode/submit 5.31, stage/main/update 4.62 |
| measure | 36.78 | 29.2 | render/thread/ms 38.67, render/busy/ms 37.65, main/busy/ms 18.87, stage/render/render 13.01, stage/main/post_update 11.78, render/graph/ms 11.45, encode//sum 11.37, stage/render/specialize 9.67, stage/render/graph_submit 8.69, stage/render/queue 7.11, encode/submit 5.08, stage/main/update 4.44 |
| measure | 37.49 | 28.8 | render/thread/ms 38.75, render/busy/ms 37.65, main/busy/ms 19.21, stage/render/render 12.96, stage/main/post_update 12.01, encode//sum 11.77, render/graph/ms 11.43, stage/render/specialize 10.38, stage/render/graph_submit 8.70, stage/render/queue 6.94, encode/pass:shadow_point_light/sum 6.31, encode/submit 5.46 |
| measure | 40.81 | 34.9 | render/thread/ms 40.01, render/busy/ms 39.00, main/busy/ms 18.74, stage/render/render 11.50, stage/main/post_update 11.11, stage/render/specialize 10.68, encode//sum 10.11, render/graph/ms 9.94, stage/render/graph_submit 7.76, stage/render/queue 6.64, stage/main/update 4.02, encode/submit 3.92 |
| measure | 43.21 | 29.6 | render/thread/ms 36.12, render/busy/ms 35.07, main/busy/ms 16.28, stage/render/specialize 10.49, stage/render/render 10.37, stage/main/post_update 9.98, render/graph/ms 8.96, encode//sum 7.10, stage/render/graph_submit 7.00, stage/render/queue 6.73, encode/submit 3.86, stage/main/update 3.85 |
| measure | 45.67 | 34.3 | render/thread/ms 37.23, render/busy/ms 36.25, main/busy/ms 17.71, stage/render/specialize 10.97, stage/render/render 10.23, stage/main/post_update 10.22, encode//sum 9.18, render/graph/ms 9.00, stage/render/graph_submit 7.05, stage/render/queue 6.91, stage/main/update 4.43, encode/submit 3.72 |
| measure | 59.03 | 40.1 | render/thread/ms 54.66, render/busy/ms 53.04, main/busy/ms 30.55, stage/render/render 18.82, stage/main/post_update 17.00, render/graph/ms 16.73, stage/render/specialize 14.58, stage/render/graph_submit 13.43, encode//sum 13.07, encode/submit 9.10, stage/main/update 8.92, stage/render/queue 6.34 |
| measure | 59.06 | 38.3 | render/thread/ms 59.89, render/busy/ms 58.77, main/busy/ms 24.37, stage/render/prepare_assets 15.38, stage/render/specialize 15.10, stage/main/post_update 11.89, stage/render/render 10.43, encode//sum 9.46, render/graph/ms 9.09, stage/render/graph_submit 7.13, stage/main/update 6.97, stage/render/queue 6.61 |
| measure | 62.58 | 33.6 | render/thread/ms 39.44, render/busy/ms 38.36, main/busy/ms 18.11, stage/render/specialize 12.16, stage/render/render 10.80, render/graph/ms 9.32, stage/main/post_update 9.19, encode//sum 8.36, stage/render/graph_submit 7.26, stage/render/queue 6.89, stage/main/update 6.14, encode/submit 4.07 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_statics\balmora_walk@2560x1440_arm=nocull-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **78%** (min 45%) · 2622 MHz (min 2610) · memory 8101 MHz · 79.4 W · 76 °C (max 82) · VRAM 7014 MiB · throttled 100% of samples

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
