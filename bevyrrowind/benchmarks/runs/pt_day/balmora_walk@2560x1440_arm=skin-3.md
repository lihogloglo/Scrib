# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.skin.skip_unchanged=on (command line: --set); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6061 frames over 65.7 s · **median 10.52 ms (95 fps)** · mean 10.83 · p95 13.33 · p99 16.42 (1% low 61 fps) · worst 42.72 ms · 78 spikes over 1.5× median · 59.4 stutters/min (work 49.3/min)
- warm-up: 323 frames over 9.3 s, worst 2569.4 ms
- hitches (> 26.3 ms): **34** measured, 45 in warm-up
- timed GPU passes: 6.59 ms of a 10.83 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.52 ms, limit 11 ms
- **FAIL** frame_ms: 65 stutters (> 1.5x moving median of 60) in 1.09 min = 59.40/min, limit 1/min
- **FAIL** worst frame 42.72 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.876 | 2.034 | 2.065 | 0.92 | 0.174 |
| main_opaque_pass_3d | 1.506 | 1.633 | 2.050 | 0.92 | 0.066 |
| early prepass | 0.459 | 0.498 | 0.734 | 0.92 | 0.089 |
| bloom | 0.290 | 0.314 | 0.321 | 0.92 | 0.062 |
| atmosphere_luts | 0.279 | 0.303 | 0.365 | 0.92 | 0.003 |
| water_velocity | 0.258 | 0.279 | 0.347 | 0.92 | 0.002 |
| water_sspr | 0.186 | 0.202 | 0.203 | 0.92 | 0.002 |
| ssao | 0.173 | 0.188 | 0.191 | 0.92 | 0.007 |
| atmos_froxel_inject | 0.134 | 0.146 | 0.147 | 0.92 | 0.001 |
| clouds_trace | 0.129 | 0.140 | 0.231 | 0.92 | 0.001 |
| uniform_allocation | 0.120 | 0.130 | 0.181 | 0.92 | 0.025 |
| water_fft | 0.119 | 0.130 | 0.136 | 0.92 | 0.003 |
| upscaling | 0.110 | 0.119 | 0.126 | 0.92 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.108 | 0.92 | 0.002 |
| bin_unpacking | 0.097 | 0.105 | 0.143 | 0.92 | 0.019 |
| auxiliary_views | 0.092 | 0.100 | 0.464 | 0.92 | 0.103 |
| main_transmissive_pass_3d | 0.079 | 0.086 | 0.179 | 0.92 | 0.009 |
| tonemapping | 0.078 | 0.085 | 0.088 | 0.92 | 0.004 |
| clustering | 0.059 | 0.064 | 0.067 | 0.92 | 0.019 |
| render_sky | 0.047 | 0.052 | 0.052 | 0.92 | 0.002 |
| clouds_resolve | 0.044 | 0.048 | 0.049 | 0.92 | 0.001 |
| surface_exposure | 0.041 | 0.044 | 0.045 | 0.92 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.65 | 0.002 |
| lightprobe_irradiance_map | 0.036 | 0.412 | 0.416 | 0.09 | 0.001 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.92 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.92 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.031 | 0.92 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.92 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.92 | 0.002 |
| lightprobe_radiance_map | 0.019 | 0.219 | 0.230 | 0.09 | 0.004 |
| night_sky | 0.013 | 0.014 | 0.033 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.92 | 0.124 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.92 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.92 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.92 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.313 | 11.409 | 37.37 | 56445.3 |
| main/busy/ms | 9.021 | 10.730 | 25.07 | 54674.4 |
| render/busy/ms | 8.754 | 10.912 | 36.79 | 53055.9 |
| stage/main/post_update | 5.623 | 7.009 | 9.19 | 34078.6 |
| stage/render/render | 5.412 | 6.782 | 18.62 | 32799.6 |
| render/graph/ms | 4.663 | 6.058 | 17.53 | 28264.3 |
| encode//sum | 4.065 | 5.333 | 48.85 | 24636.4 |
| stage/render/graph_submit | 3.512 | 4.620 | 13.75 | 21286.5 |
| encode/submit | 2.217 | 3.115 | 7.58 | 13440.1 |
| stage/main/update | 2.132 | 2.548 | 18.58 | 12924.8 |
| encode/finish_wall | 1.372 | 1.991 | 7.37 | 8318.6 |
| stage/render/camera_driver | 1.099 | 1.511 | 3.98 | 6658.6 |
| encode/pass:shadow_directional_light/sum | 0.871 | 1.555 | 5.64 | 5279.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.789 | 1.102 | 1.71 | 4783.4 |
| encode/pass:early prepass/sum | 0.765 | 1.077 | 1.69 | 4638.1 |
| stage/render/prepare_write_phase_buffers | 0.754 | 1.062 | 2.71 | 4568.0 |
| encode/pass:shadow_point_light/sum | 0.601 | 2.796 | 16.99 | 3644.6 |
| stage/render/prepare_assets | 0.566 | 0.728 | 13.05 | 3432.5 |
| bigspace/total/ms | 0.545 | 1.318 | 2.53 | 3302.6 |
| render/present/ms | 0.464 | 0.960 | 2.44 | 2814.8 |
| stage/render/prepare_resources | 0.447 | 0.537 | 1.14 | 2706.8 |
| stage/render/prepare_meshes | 0.443 | 0.565 | 6.45 | 2687.8 |
| stage/main/first | 0.398 | 0.517 | 5.42 | 2412.9 |
| stage/main/pre_update | 0.362 | 0.525 | 2.92 | 2192.5 |
| stage/render/specialize | 0.323 | 0.412 | 12.11 | 1955.5 |
| terrain/update/ms | 0.314 | 0.409 | 1.94 | 1901.0 |
| stage/render/queue | 0.292 | 0.423 | 9.58 | 1766.9 |
| stage/render/prepare_bind_groups | 0.282 | 0.502 | 2.59 | 1708.9 |
| terrain/select/ms | 0.280 | 0.368 | 0.93 | 1696.6 |
| controller/feed/ms | 0.261 | 0.328 | 0.74 | 1584.2 |
| stream/drive/ms | 0.249 | 0.330 | 16.78 | 1509.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.241 | 0.358 | 1.85 | 1462.7 |
| bigspace/low_precision/ms | 0.222 | 0.716 | 1.64 | 1344.6 |
| stage/main/fixed_main_loop | 0.215 | 0.368 | 1.40 | 1305.0 |
| stage/render/create_views | 0.197 | 0.253 | 0.89 | 1196.1 |
| far/select/ms | 0.195 | 0.259 | 0.69 | 1181.5 |
| stream/tick/ms | 0.193 | 0.257 | 0.68 | 1171.8 |
| stage/render/prepare_batch_phases | 0.187 | 0.266 | 0.80 | 1135.7 |
| stage/render/extract_commands | 0.171 | 0.200 | 0.67 | 1033.8 |
| stage/render/prepare_flush | 0.164 | 0.322 | 0.86 | 991.8 |
| anim/evaluate/ms | 0.162 | 0.281 | 1.51 | 981.6 |
| bigspace/high_precision/ms | 0.160 | 0.693 | 1.78 | 970.2 |
| anim/evaluate_cpu/ms | 0.152 | 0.194 | 0.71 | 921.0 |
| controller/move/ms | 0.140 | 0.302 | 0.54 | 850.0 |
| stage/render/prepare_views | 0.138 | 0.172 | 0.43 | 835.4 |
| stage/main/last | 0.115 | 0.154 | 2.81 | 694.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.104 | 0.173 | 0.77 | 627.6 |
| render/acquire/ms | 0.095 | 0.121 | 0.38 | 574.6 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.134 | 0.54 | 387.5 |
| pipelines/compile_ms | 0.049 | 0.000 | 123.04 | 297.2 |
| pipelines/front_ms | 0.047 | 0.000 | 118.74 | 284.5 |
| physics/step/ms | 0.046 | 0.086 | 0.61 | 281.0 |
| stage/render/cleanup | 0.034 | 0.106 | 0.93 | 203.4 |
| stage/render/phase_sort | 0.033 | 0.056 | 0.53 | 199.1 |
| physics/insert/ms | 0.031 | 0.352 | 0.79 | 186.5 |
| encode/resolve_diagnostics/sum | 0.031 | 0.040 | 0.54 | 186.1 |
| weather/map/ms | 0.024 | 0.033 | 0.21 | 148.2 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.53 | 90.7 |
| anim/apply/ms | 0.011 | 0.020 | 0.23 | 69.6 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.42 | 68.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.04 | 61.9 |
| controller/avoid/ms | 0.010 | 0.013 | 0.19 | 61.7 |
| script/update/ms | 0.007 | 0.009 | 0.23 | 42.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 31.9 |
| stream/despawn/ms | 0.005 | 0.000 | 0.99 | 30.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 28.8 |
| script/ui/ms | 0.004 | 0.006 | 0.22 | 26.6 |
| controller/write/ms | 0.003 | 0.004 | 0.25 | 21.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.68 | 14.0 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.31 | 12.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.21 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 4.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.39 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.10 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 16 |
| controller/movers/count | count | 20 | 22 | 16 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 593 |
| crowd/c0/meshes/count | count | 355 | 646 | 156 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5603 |
| crowd/c1/meshes/count | count | 1181 | 1254 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 621 | 6883 | 382 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.42 | 2.53 | 0.39 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.53 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.42 | 0.01 |
| encode/finish_critical | ms | 0.98 | 5.64 | 0.85 |
| encode/pass:early prepass/max | ms | 0.78 | 1.69 | 0.85 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.71 | 0.72 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.77 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.85 | 0.45 |
| encode/pass:shadow_directional_light/max | ms | 0.82 | 5.64 | 0.84 |
| encode/pass:shadow_point_light/max | ms | 0.78 | 2.15 | 0.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.54 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.20 | 36.79 | 11.27 |
| gpu/timed/ms | ms | 7.84 | 10.99 | 8.24 |
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
| phases/alpha_mask3d/items | count | 820 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 820 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3430 | 5117 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3430 | 5117 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 12 | 232 | 4 |
| phases/shadow/bins | count | 5035 | 8760 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.3 k | 6382 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 254 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2345 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 185 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 623 | 26 |
| preprocess/bind_groups_created | count | 13 | 623 | 26 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 128 | 223 | 103 |
| process/cpu_usage | % | 26.92 | 38.94 | 25.48 |
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
| skin/blocks_changed/count | count | 13 | 127 | 8 |
| skin/blocks_skipped/count | count | 186 | 237 | 230 |
| skin/blocks_walked/count | count | 13 | 19 | 8 |
| skin/joint_bytes_written/bytes | bytes | 42.0 k | 812.0 k | 25.1 k |
| skin/joints_written/count | count | 656 | 12.7 k | 392 |
| skin/upload_writes/count | count | 7 | 29 | 6 |
| started_load_count |  | 4916 | 5009 | 5009 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.27 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.73 | 41.34 | 30.01 |
| system/mem_usage | % | 54.31 | 54.83 | 54.75 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 61 | 279 | 54 |
| views/all/bins | count | 7599 | 11.6 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.2 k | 14.0 k |
| views/all/pipelines | count | 249 | 301 | 75 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.7 k | 35.6 k | 8119 |
| views/cascade/batch_sets | count | 9 | 51 | 4 |
| views/cascade/bins | count | 2797 | 6187 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8918 | 23.5 k | 6382 |
| views/cascade/pipelines | count | 36 | 54 | 29 |
| views/cascade/work_items | count | 409 | 23.5 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 195 | 0 |
| views/lamp_face/bins | count | 2198 | 2778 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2431 | 3075 | 0 |
| views/lamp_face/pipelines | count | 168 | 210 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2989 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2860 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8500 | 13.0 k | 7610 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8660 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 57.15 | 262 | 346.4 k |
| anim/events/count | count | 0.09 | 2 | 539 |
| assets/Image/added | count | 0.10 | 38 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.57 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 52 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1296 | 4675 |
| encode//count | count | 31.89 | 222 | 193.3 k |
| encode/bind_walk/init_textures | count | 11.6 k | 119.2 k | 70.17 M |
| encode/bind_walk/init_textures_skipped | count | 63.6 k | 146.3 k | 385.64 M |
| encode/bind_walk/inits | count | 449 | 1227 | 2.72 M |
| encode/bind_walk/inits_skipped | count | 108 | 160 | 651.6 k |
| encode/bind_walk/merge_textures | count | 11.5 k | 119.1 k | 69.73 M |
| encode/bind_walk/merge_textures_skipped | count | 5364 | 6012 | 32.51 M |
| encode/bind_walk/merges | count | 96.03 | 356 | 582.0 k |
| encode/bind_walk/merges_skipped | count | 84.05 | 132 | 509.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6061 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6061 |
| encode/pass:early prepass/count | count | 1 | 1 | 6061 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6061 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6061 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6061 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6061 |
| encode/pass:shadow_point_light/count | count | 0.99 | 48 | 5978 |
| encode/queue/command_buffers | count | 42.89 | 280 | 260.0 k |
| encode/queue/submits | count | 2.02 | 4 | 12.2 k |
| encode/queue/write_buffer/bytes | bytes | 1.54 M | 4.43 M | 9.31 G |
| encode/queue/write_buffer/calls | count | 257 | 933 | 1.55 M |
| encode/queue/write_buffer_with/bytes | bytes | 179.2 k | 3.52 M | 1.09 G |
| encode/queue/write_buffer_with/calls | count | 11.00 | 465 | 66.6 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 71.46 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6061 |
| physics/inserts | count | 0.37 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.65 | 3 | 3939 |
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
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5585 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1225 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.3 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2569.4 | main/busy/ms 2534, stage/main/post_update 2427, stage/main/update 98.70, surface/advance_cpu_ms 69.85, stage/main/pre_update 5.23, stage/main/last 1.06, surface/cell_upload_ms 0.85, stage/main/first 0.64, stream/drive/ms 0.40, stage/main/fixed_main_loop 0.34, weather/map/ms 0.12, stream/tick/ms 0.09 |
| warm-up | 3.45 | 881.7 | main/busy/ms 2731, stage/main/post_update 2454, render/thread/ms 848, render/busy/ms 657, stage/render/prepare_assets 606, stage/main/update 263, stage/render/prepare_views 171, render/acquire/ms 170, surface/advance_cpu_ms 69.85, stage/render/render 31.74, stage/render/create_views 28.77, render/present/ms 21.17 |
| warm-up | 3.60 | 144.5 | render/thread/ms 975, render/busy/ms 718, stage/render/prepare_assets 652, main/busy/ms 226, stage/main/update 172, stage/render/prepare_views 171, render/acquire/ms 171, stage/render/render 103, render/present/ms 86.33, pipelines/compile_ms 73.73, pipelines/driver_ms 56.86, stage/main/post_update 40.37 |
| warm-up | 3.65 | 53.5 | pipelines/compile_ms 508, pipelines/wait_ms 356, pipelines/front_ms 340, render/thread/ms 175, pipelines/driver_ms 168, stage/render/render 113, render/busy/ms 94.71, main/busy/ms 82.16, render/present/ms 79.94, pipelines/lock_ms 61.62, stage/main/post_update 58.65, stage/render/prepare_assets 47.10 |
| warm-up | 3.78 | 128.3 | pipelines/wait_ms 4399, pipelines/compile_ms 1579, pipelines/front_ms 1416, render/thread/ms 173, pipelines/driver_ms 163, render/busy/ms 157, main/busy/ms 92.99, pipelines/lock_ms 70.81, stage/render/render 68.33, stage/main/post_update 64.35, render/graph/ms 51.87, stage/render/prepare_assets 46.26 |
| warm-up | 3.99 | 181.1 | pipelines/compile_ms 1885, pipelines/front_ms 1620, pipelines/wait_ms 499, pipelines/driver_ms 264, render/thread/ms 152, render/busy/ms 150, main/busy/ms 88.61, stage/render/prepare_assets 80.66, stage/main/update 61.56, stage/render/render 58.80, render/graph/ms 56.34, stage/render/graph_submit 53.76 |
| warm-up | 4.04 | 35.5 | pipelines/compile_ms 1425, pipelines/front_ms 1305, pipelines/wait_ms 214, pipelines/driver_ms 120, render/thread/ms 44.83, render/busy/ms 41.41, main/busy/ms 41.20, stage/render/render 29.61, render/graph/ms 26.40, stage/main/post_update 25.42, stage/render/graph_submit 23.26, encode//sum 19.84 |
| warm-up | 4.08 | 31.6 | pipelines/compile_ms 63.40, pipelines/front_ms 60.71, render/thread/ms 36.69, render/busy/ms 35.24, main/busy/ms 23.75, stage/render/prepare_assets 18.09, stage/main/post_update 14.67, stage/render/render 10.06, terrain/update/ms 8.65, encode//sum 8.29, render/graph/ms 8.11, stage/render/graph_submit 6.01 |
| warm-up | 4.18 | 88.4 | pipelines/compile_ms 1118, pipelines/front_ms 1069, main/busy/ms 55.16, render/thread/ms 54.13, render/busy/ms 51.13, pipelines/driver_ms 48.22, stage/main/update 44.25, stream/drive/ms 39.74, stage/render/prepare_assets 35.14, stage/render/render 12.36, render/graph/ms 8.25, encode//sum 6.76 |
| warm-up | 4.24 | 37.8 | pipelines/compile_ms 444, pipelines/front_ms 430, render/thread/ms 44.88, render/busy/ms 41.58, main/busy/ms 38.63, stage/render/prepare_assets 19.28, stage/main/update 18.97, stage/render/render 18.37, pipelines/driver_ms 14.09, render/graph/ms 11.91, stage/main/post_update 11.34, stream/drive/ms 10.21 |
| warm-up | 4.28 | 33.7 | pipelines/compile_ms 443, pipelines/front_ms 423, render/thread/ms 53.99, render/busy/ms 52.54, main/busy/ms 52.42, stage/render/prepare_assets 38.55, stage/main/update 37.20, stream/drive/ms 28.39, pipelines/driver_ms 19.40, stage/render/render 9.11, stage/main/post_update 7.35, render/graph/ms 7.33 |
| warm-up | 4.33 | 55.7 | pipelines/compile_ms 320, pipelines/front_ms 301, render/thread/ms 59.46, render/busy/ms 57.86, main/busy/ms 52.17, stage/render/prepare_assets 41.00, stage/main/update 38.79, stream/drive/ms 29.79, pipelines/driver_ms 18.39, stage/render/render 10.37, render/graph/ms 8.19, stage/main/post_update 7.39 |
| warm-up | 4.38 | 52.2 | pipelines/compile_ms 733, pipelines/front_ms 710, render/thread/ms 70.07, render/busy/ms 68.26, main/busy/ms 35.23, stage/render/prepare_assets 31.14, stage/render/render 25.27, render/graph/ms 22.85, pipelines/driver_ms 22.57, encode//sum 21.10, stage/render/graph_submit 18.98, encode/finish_wall 15.32 |
| warm-up | 4.45 | 70.0 | pipelines/compile_ms 1283, pipelines/front_ms 1248, render/thread/ms 83.66, render/busy/ms 81.82, main/busy/ms 70.92, stage/main/update 42.33, stage/render/prepare_assets 40.75, pipelines/driver_ms 35.20, stage/render/render 27.25, render/graph/ms 24.85, encode//sum 21.10, stage/render/graph_submit 20.94 |
| warm-up | 4.57 | 111.1 | pipelines/compile_ms 1576, pipelines/front_ms 1530, render/thread/ms 133, render/busy/ms 131, main/busy/ms 101, stage/render/prepare_assets 93.11, stage/main/update 51.67, pipelines/driver_ms 45.73, stage/main/post_update 39.24, pipelines/wait_ms 28.97, stage/render/render 23.34, render/graph/ms 20.59 |
| warm-up | 4.61 | 41.9 | pipelines/compile_ms 1903, pipelines/front_ms 1840, pipelines/wait_ms 141, render/thread/ms 120, render/busy/ms 118, main/busy/ms 81.27, stage/render/prepare_assets 73.54, pipelines/driver_ms 63.15, stage/main/update 36.74, stage/main/post_update 33.65, stage/render/render 22.28, render/graph/ms 19.72 |
| warm-up | 4.70 | 93.0 | pipelines/compile_ms 2709, pipelines/front_ms 2625, pipelines/wait_ms 777, render/thread/ms 116, render/busy/ms 114, pipelines/driver_ms 84.38, main/busy/ms 43.54, stage/render/prepare_assets 37.69, stage/main/update 26.31, stage/render/prepare_resources 25.13, encode//sum 20.44, stage/render/render 17.22 |
| warm-up | 4.79 | 86.0 | pipelines/compile_ms 2306, pipelines/front_ms 2234, pipelines/wait_ms 1102, render/thread/ms 166, render/busy/ms 164, main/busy/ms 97.81, pipelines/driver_ms 72.06, encode//sum 67.85, stage/main/post_update 65.99, stage/render/prepare_resources 47.36, stage/render/render 46.86, render/graph/ms 44.12 |
| warm-up | 4.88 | 92.0 | pipelines/compile_ms 1758, pipelines/front_ms 1699, pipelines/wait_ms 1468, render/thread/ms 152, render/busy/ms 150, main/busy/ms 114, stage/main/post_update 65.51, encode//sum 61.23, pipelines/driver_ms 58.74, stage/render/prepare_assets 53.68, stage/render/render 47.14, render/graph/ms 44.62 |
| warm-up | 4.93 | 49.1 | pipelines/compile_ms 2084, pipelines/front_ms 2023, pipelines/wait_ms 1412, render/thread/ms 101, render/busy/ms 99.17, main/busy/ms 97.74, stage/render/prepare_assets 67.44, stage/main/update 66.01, pipelines/driver_ms 61.29, stream/drive/ms 53.08, stage/render/render 16.30, render/graph/ms 14.01 |
| warm-up | 5.00 | 71.9 | pipelines/compile_ms 1757, pipelines/front_ms 1713, pipelines/wait_ms 805, main/busy/ms 106, stage/main/update 79.85, render/thread/ms 64.06, render/busy/ms 61.84, stream/drive/ms 54.73, pipelines/driver_ms 44.16, stage/render/prepare_assets 23.52, stage/render/render 18.02, render/graph/ms 14.59 |
| warm-up | 5.08 | 79.4 | pipelines/compile_ms 1919, pipelines/front_ms 1869, pipelines/wait_ms 961, main/busy/ms 123, stage/main/update 96.81, render/thread/ms 88.10, render/busy/ms 84.59, pipelines/driver_ms 49.72, stream/drive/ms 48.73, stage/render/render 29.91, stage/render/prepare_assets 25.54, render/graph/ms 24.91 |
| warm-up | 5.16 | 80.5 | pipelines/compile_ms 1095, pipelines/front_ms 1062, pipelines/wait_ms 539, main/busy/ms 109, render/thread/ms 93.94, render/busy/ms 91.13, stage/main/update 86.38, stream/drive/ms 47.45, stage/render/prepare_assets 44.82, pipelines/driver_ms 32.49, stage/render/render 23.27, render/graph/ms 19.43 |
| warm-up | 5.20 | 40.6 | pipelines/compile_ms 594, pipelines/front_ms 572, main/busy/ms 83.80, render/thread/ms 70.66, render/busy/ms 69.07, stage/main/update 65.28, stream/drive/ms 44.84, stage/render/prepare_assets 34.81, pipelines/driver_ms 21.58, stage/render/render 15.21, render/graph/ms 12.81, stage/main/post_update 10.84 |
| warm-up | 5.26 | 65.2 | pipelines/compile_ms 1103, pipelines/front_ms 1068, main/busy/ms 76.84, stage/main/update 61.94, render/thread/ms 57.95, render/busy/ms 56.37, stream/drive/ms 45.52, pipelines/driver_ms 35.05, stage/render/prepare_assets 23.84, stage/render/render 14.49, render/graph/ms 12.03, encode//sum 9.06 |
| warm-up | 5.33 | 47.1 | pipelines/compile_ms 170, pipelines/front_ms 163, main/busy/ms 57.11, stage/main/update 41.14, stream/drive/ms 34.86, render/thread/ms 33.98, render/busy/ms 32.68, stage/render/render 12.23, render/graph/ms 10.36, stage/main/post_update 9.80, stage/render/prepare_assets 9.00, encode//sum 8.95 |
| warm-up | 5.37 | 31.7 | pipelines/compile_ms 370, pipelines/front_ms 357, render/thread/ms 45.70, render/busy/ms 43.86, main/busy/ms 35.66, stage/main/post_update 17.34, pipelines/driver_ms 13.15, stage/main/update 12.55, stage/render/render 11.17, render/graph/ms 9.32, stage/render/prepare_assets 8.59, encode//sum 7.92 |
| warm-up | 5.51 | 53.1 | pipelines/compile_ms 1070, pipelines/front_ms 1041, render/thread/ms 47.54, render/busy/ms 45.56, main/busy/ms 37.09, pipelines/driver_ms 28.67, stage/render/prepare_assets 21.41, stage/main/update 19.59, stage/render/render 14.78, render/graph/ms 12.22, stage/main/post_update 11.58, stage/render/graph_submit 9.80 |
| warm-up | 5.55 | 38.8 | pipelines/compile_ms 813, pipelines/front_ms 787, render/thread/ms 64.46, render/busy/ms 62.80, main/busy/ms 38.51, stage/render/prepare_assets 35.87, pipelines/driver_ms 25.50, stage/main/update 21.98, stream/drive/ms 14.74, stage/render/render 14.32, render/graph/ms 12.04, stage/main/post_update 11.14 |
| warm-up | 5.95 | 30.0 | pipelines/compile_ms 92.72, pipelines/front_ms 88.58, main/busy/ms 36.56, render/thread/ms 25.61, render/busy/ms 24.28, stage/main/update 20.85, stream/drive/ms 14.13, stage/main/post_update 11.25, stage/render/render 10.95, render/graph/ms 8.98, encode//sum 8.09, stage/render/graph_submit 6.37 |
| warm-up | 6.34 | 31.0 | render/thread/ms 33.45, render/busy/ms 32.56, main/busy/ms 26.43, stage/render/prepare_assets 15.27, stage/main/update 11.60, stage/main/post_update 10.05, stage/render/render 8.21, render/graph/ms 6.93, encode//sum 5.89, stage/render/graph_submit 4.88, stream/drive/ms 3.50, encode/submit 3.10 |
| warm-up | 6.56 | 29.8 | render/thread/ms 45.35, render/busy/ms 44.08, main/busy/ms 39.10, stage/main/update 16.83, stage/main/post_update 15.41, stage/render/render 14.15, render/graph/ms 12.26, stage/render/prepare_assets 9.21, encode//sum 8.44, stage/render/graph_submit 7.94, stream/drive/ms 5.49, encode/submit 4.26 |
| warm-up | 7.10 | 34.4 | main/busy/ms 42.33, render/thread/ms 28.49, render/busy/ms 27.40, stage/main/update 24.61, stream/drive/ms 12.80, stage/render/prepare_assets 11.82, stage/main/post_update 10.78, stage/render/render 9.22, render/graph/ms 7.70, encode//sum 6.53, stage/render/graph_submit 5.49, encode/submit 3.34 |
| warm-up | 7.18 | 31.4 | pipelines/compile_ms 142, pipelines/front_ms 139, render/thread/ms 41.49, render/busy/ms 40.48, main/busy/ms 19.96, stage/main/post_update 11.24, stage/render/render 11.05, stage/render/specialize 10.51, render/graph/ms 9.63, encode//sum 8.42, stage/render/graph_submit 7.52, stage/render/queue 6.54 |
| warm-up | 8.44 | 30.5 | render/thread/ms 35.21, render/busy/ms 34.17, main/busy/ms 17.93, stage/main/post_update 10.27, stage/render/render 10.21, stage/render/specialize 9.56, render/graph/ms 8.75, encode//sum 7.14, stage/render/graph_submit 6.72, stage/render/queue 5.72, stage/main/update 4.99, encode/submit 3.71 |
| measure | 0.42 | 32.7 | render/thread/ms 36.92, render/busy/ms 35.76, main/busy/ms 16.54, stage/render/render 12.05, render/graph/ms 10.44, stage/main/post_update 10.16, stage/render/specialize 9.05, stage/render/graph_submit 8.42, encode//sum 7.05, stage/render/queue 6.52, encode/submit 5.34, stage/main/update 3.79 |
| measure | 1.71 | 32.4 | render/thread/ms 40.15, render/busy/ms 39.09, main/busy/ms 18.70, stage/render/render 13.94, render/graph/ms 12.43, stage/main/post_update 11.69, encode//sum 11.27, stage/render/graph_submit 9.54, stage/render/specialize 9.12, encode/pass:shadow_point_light/sum 7.02, stage/render/queue 6.06, encode/submit 6.05 |
| measure | 3.12 | 30.5 | render/thread/ms 38.57, render/busy/ms 37.50, main/busy/ms 20.65, stage/main/post_update 13.18, stage/render/render 12.22, render/graph/ms 10.75, stage/render/specialize 9.86, encode//sum 9.68, stage/render/graph_submit 8.19, stage/render/queue 6.10, encode/pass:shadow_point_light/sum 5.13, encode/submit 4.84 |
| measure | 4.57 | 29.7 | render/thread/ms 34.14, render/busy/ms 33.17, main/busy/ms 18.72, stage/main/post_update 11.89, stage/render/render 9.91, stage/render/specialize 9.51, render/graph/ms 8.45, encode//sum 7.11, stage/render/graph_submit 6.49, stage/render/queue 6.02, stage/main/update 4.02, encode/submit 3.55 |
| measure | 5.92 | 31.3 | render/thread/ms 36.48, render/busy/ms 35.33, main/busy/ms 22.12, stage/main/post_update 13.44, stage/render/render 10.93, stage/render/specialize 10.07, render/graph/ms 9.30, encode//sum 7.76, stage/render/graph_submit 7.02, stage/render/queue 6.40, encode/submit 4.07, stage/main/update 3.73 |
| measure | 7.25 | 29.2 | render/thread/ms 38.07, render/busy/ms 36.96, main/busy/ms 18.40, stage/render/render 12.62, stage/main/post_update 11.39, render/graph/ms 11.11, stage/render/specialize 9.47, encode//sum 9.23, stage/render/graph_submit 8.33, stage/render/queue 6.16, encode/submit 5.35, encode/pass:shadow_point_light/sum 4.91 |
| measure | 8.59 | 30.6 | render/thread/ms 38.36, render/busy/ms 37.37, main/busy/ms 19.44, stage/main/post_update 12.18, stage/render/render 10.65, stage/render/specialize 10.27, encode//sum 9.38, render/graph/ms 9.06, stage/render/queue 6.70, stage/render/graph_submit 6.50, stage/main/update 4.15, encode/pass:shadow_point_light/sum 3.59 |
| measure | 9.93 | 29.7 | render/thread/ms 35.14, render/busy/ms 34.22, main/busy/ms 19.16, stage/main/post_update 10.90, stage/render/render 10.75, stage/render/specialize 9.48, render/graph/ms 9.28, encode//sum 8.94, stage/render/graph_submit 7.02, stage/render/queue 6.23, stage/main/update 4.81, encode/submit 3.89 |
| measure | 11.22 | 29.8 | render/thread/ms 37.29, render/busy/ms 36.14, main/busy/ms 18.75, stage/main/post_update 11.73, stage/render/render 11.39, stage/render/specialize 9.88, render/graph/ms 9.76, encode//sum 9.35, stage/render/graph_submit 7.25, stage/render/queue 6.44, stage/main/update 4.52, encode/submit 4.25 |
| measure | 15.65 | 32.8 | render/thread/ms 40.37, render/busy/ms 39.25, main/busy/ms 17.94, stage/main/post_update 11.11, stage/render/render 10.82, stage/render/specialize 10.13, stage/render/queue 9.85, render/graph/ms 9.38, encode//sum 7.50, stage/render/graph_submit 7.39, stage/main/update 4.18, encode/submit 4.03 |
| measure | 18.01 | 30.3 | render/thread/ms 37.52, render/busy/ms 36.45, main/busy/ms 18.26, stage/main/post_update 11.55, stage/render/render 11.53, render/graph/ms 10.00, stage/render/specialize 9.65, encode//sum 9.14, stage/render/graph_submit 7.77, stage/render/queue 6.70, encode/submit 4.08, encode/finish_wall 3.75 |
| measure | 19.37 | 30.6 | render/thread/ms 35.92, render/busy/ms 34.98, main/busy/ms 18.91, stage/main/post_update 11.67, stage/render/render 10.27, stage/render/specialize 9.96, render/graph/ms 8.92, encode//sum 8.03, stage/render/graph_submit 6.77, stage/render/queue 6.73, stage/main/update 4.30, encode/submit 3.62 |
| measure | 20.72 | 30.3 | render/thread/ms 36.68, render/busy/ms 35.60, main/busy/ms 20.33, stage/main/post_update 12.26, stage/render/render 10.71, stage/render/specialize 10.41, render/graph/ms 9.26, encode//sum 8.31, stage/render/graph_submit 7.32, stage/render/queue 6.65, stage/main/update 5.03, encode/submit 3.88 |
| measure | 23.02 | 36.7 | render/thread/ms 41.71, render/busy/ms 40.60, main/busy/ms 25.30, stage/main/post_update 12.58, stage/render/specialize 12.51, stage/render/render 11.69, render/graph/ms 10.17, encode//sum 9.67, stage/render/queue 7.90, stage/render/graph_submit 7.73, stage/main/update 7.20, encode/submit 4.56 |
| measure | 24.50 | 30.6 | render/thread/ms 36.35, render/busy/ms 35.22, main/busy/ms 19.83, stage/main/post_update 11.90, stage/render/render 10.76, stage/render/specialize 9.49, render/graph/ms 9.11, encode//sum 7.32, stage/render/queue 7.26, stage/render/graph_submit 7.04, stage/main/update 5.17, encode/submit 4.11 |
| measure | 27.28 | 37.6 | render/thread/ms 44.85, render/busy/ms 43.78, main/busy/ms 25.47, stage/render/render 15.08, render/graph/ms 13.63, stage/main/post_update 12.81, encode//sum 12.57, stage/render/specialize 11.93, stage/render/graph_submit 10.60, encode/pass:shadow_point_light/sum 8.67, stage/render/queue 7.96, encode/submit 6.94 |
| measure | 28.72 | 30.3 | render/thread/ms 41.97, render/busy/ms 40.90, main/busy/ms 20.46, stage/render/render 14.80, encode//sum 13.94, stage/main/post_update 13.43, render/graph/ms 13.30, stage/render/specialize 9.96, stage/render/graph_submit 9.71, stage/render/queue 6.78, encode/pass:shadow_point_light/sum 6.43, encode/submit 5.66 |
| measure | 30.48 | 32.8 | render/thread/ms 40.66, render/busy/ms 39.51, main/busy/ms 20.44, stage/main/post_update 12.48, stage/render/render 11.24, stage/render/specialize 11.02, render/graph/ms 9.63, encode//sum 9.20, stage/render/graph_submit 7.69, stage/render/queue 7.65, stage/main/update 5.18, encode/submit 4.12 |
| measure | 31.94 | 29.9 | render/thread/ms 39.71, render/busy/ms 38.57, main/busy/ms 20.37, stage/main/post_update 13.29, stage/render/render 12.96, render/graph/ms 11.22, encode//sum 10.37, stage/render/specialize 9.99, stage/render/graph_submit 8.75, stage/render/queue 6.85, encode/submit 5.28, stage/main/update 4.18 |
| measure | 35.36 | 34.8 | render/thread/ms 40.03, render/busy/ms 38.98, main/busy/ms 19.86, stage/main/post_update 12.22, stage/render/render 11.22, stage/render/specialize 10.50, render/graph/ms 9.76, encode//sum 9.21, stage/render/graph_submit 7.65, stage/render/queue 7.43, stage/main/update 4.37, encode/submit 3.95 |
| measure | 36.70 | 31.7 | render/thread/ms 37.23, render/busy/ms 36.12, main/busy/ms 21.33, stage/main/post_update 12.10, stage/render/render 10.91, stage/render/specialize 10.13, render/graph/ms 9.36, encode//sum 8.04, stage/render/graph_submit 7.20, stage/render/queue 6.97, stage/main/update 4.17, encode/submit 3.70 |
| measure | 37.48 | 31.8 | render/thread/ms 37.53, render/busy/ms 36.31, main/busy/ms 18.60, stage/main/post_update 11.80, stage/render/render 10.71, stage/render/specialize 10.27, render/graph/ms 9.24, encode//sum 8.24, stage/render/graph_submit 7.12, stage/render/queue 6.80, stage/main/update 4.15, encode/submit 3.92 |
| measure | 40.80 | 31.3 | render/thread/ms 38.59, render/busy/ms 37.50, main/busy/ms 17.67, stage/render/render 11.12, stage/render/specialize 10.13, stage/main/post_update 9.95, render/graph/ms 9.70, encode//sum 8.12, stage/render/graph_submit 7.79, stage/render/queue 6.30, stage/main/update 4.36, encode/submit 4.17 |
| measure | 43.22 | 32.4 | render/thread/ms 38.63, render/busy/ms 37.55, main/busy/ms 16.89, stage/render/render 11.17, stage/render/specialize 10.58, render/graph/ms 9.85, stage/main/post_update 9.76, encode//sum 8.94, stage/render/graph_submit 7.99, stage/render/queue 6.83, stage/main/update 4.23, encode/submit 4.18 |
| measure | 45.67 | 32.1 | render/thread/ms 39.49, render/busy/ms 38.41, main/busy/ms 18.65, stage/render/render 11.65, stage/main/post_update 11.03, stage/render/specialize 10.45, render/graph/ms 10.17, encode//sum 8.34, stage/render/graph_submit 8.24, stage/render/queue 6.20, encode/submit 5.08, stage/main/update 4.55 |
| measure | 56.43 | 29.9 | render/thread/ms 36.48, render/busy/ms 35.48, main/busy/ms 15.73, stage/render/render 10.75, render/graph/ms 9.47, stage/render/specialize 9.13, stage/main/post_update 9.01, encode//sum 8.23, stage/render/graph_submit 7.42, stage/render/queue 6.59, encode/submit 4.29, stage/main/update 3.86 |
| measure | 58.97 | 42.7 | render/thread/ms 57.59, render/busy/ms 55.97, main/busy/ms 30.23, stage/render/render 19.78, render/graph/ms 17.67, stage/main/post_update 16.39, stage/render/graph_submit 14.55, encode//sum 11.37, stage/render/specialize 10.66, stage/main/update 8.54, encode/finish_wall 8.33, encode/pass:shadow_directional_light/sum 7.31 |
| measure | 59.01 | 36.2 | render/thread/ms 59.66, render/busy/ms 58.50, main/busy/ms 25.21, stage/render/prepare_assets 18.28, stage/main/post_update 12.26, stage/render/specialize 10.51, stage/render/render 10.34, encode//sum 8.98, render/graph/ms 8.98, stage/render/prepare_meshes 7.89, stage/render/graph_submit 6.98, stage/main/update 6.66 |
| measure | 62.58 | 32.8 | render/thread/ms 40.10, render/busy/ms 39.10, main/busy/ms 19.69, stage/render/render 11.21, stage/render/specialize 10.82, stage/main/post_update 10.45, render/graph/ms 9.86, encode//sum 9.01, stage/render/graph_submit 7.56, stage/render/queue 6.98, stage/main/update 6.23, stage/render/prepare_meshes 4.21 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set render.skin.skip_unchanged=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=skin-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 13%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **78%** (min 37%) · 2624 MHz (min 2610) · memory 8101 MHz · 80.9 W · 76 °C (max 82) · VRAM 7064 MiB · throttled 100% of samples

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
