# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.day_gate=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6023 frames over 65.6 s · **median 10.60 ms (94 fps)** · mean 10.90 · p95 13.60 · p99 17.26 (1% low 58 fps) · worst 46.45 ms · 81 spikes over 1.5× median · 64.9 stutters/min (work 53.0/min)
- warm-up: 322 frames over 9.1 s, worst 2560.1 ms
- hitches (> 26.5 ms): **32** measured, 42 in warm-up
- timed GPU passes: 6.62 ms of a 10.90 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.60 ms, limit 11 ms
- **FAIL** frame_ms: 71 stutters (> 1.5x moving median of 60) in 1.09 min = 64.89/min, limit 1/min
- **FAIL** worst frame 46.45 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.860 | 2.034 | 2.066 | 0.91 | 0.174 |
| main_opaque_pass_3d | 1.529 | 1.672 | 2.054 | 0.91 | 0.067 |
| early prepass | 0.448 | 0.490 | 0.716 | 0.91 | 0.086 |
| bloom | 0.288 | 0.315 | 0.321 | 0.91 | 0.061 |
| atmosphere_luts | 0.277 | 0.303 | 0.365 | 0.91 | 0.003 |
| water_velocity | 0.255 | 0.279 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.184 | 0.202 | 0.203 | 0.91 | 0.002 |
| ssao | 0.172 | 0.188 | 0.191 | 0.91 | 0.007 |
| auxiliary_views | 0.146 | 0.160 | 0.602 | 0.91 | 0.160 |
| atmos_froxel_inject | 0.133 | 0.145 | 0.147 | 0.91 | 0.001 |
| clouds_trace | 0.128 | 0.140 | 0.231 | 0.91 | 0.001 |
| water_fft | 0.120 | 0.131 | 0.138 | 0.91 | 0.003 |
| uniform_allocation | 0.119 | 0.130 | 0.181 | 0.91 | 0.026 |
| upscaling | 0.109 | 0.119 | 0.126 | 0.91 | 0.004 |
| clouds_composite | 0.097 | 0.106 | 0.108 | 0.91 | 0.002 |
| bin_unpacking | 0.096 | 0.105 | 0.141 | 0.91 | 0.019 |
| tonemapping | 0.077 | 0.085 | 0.088 | 0.91 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.085 | 0.178 | 0.91 | 0.008 |
| clustering | 0.058 | 0.064 | 0.067 | 0.91 | 0.018 |
| render_sky | 0.047 | 0.052 | 0.052 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.049 | 0.049 | 0.91 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.91 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.65 | 0.002 |
| lightprobe_irradiance_map | 0.036 | 0.411 | 0.415 | 0.09 | 0.001 |
| surface_exposure_far | 0.035 | 0.042 | 0.045 | 0.82 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.046 | 0.91 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.029 | 0.032 | 0.91 | 0.001 |
| water_snapshot | 0.023 | 0.025 | 0.028 | 0.91 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.019 | 0.219 | 0.229 | 0.09 | 0.005 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.026 | 0.91 | 0.122 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.407 | 11.845 | 40.16 | 56658.5 |
| main/busy/ms | 8.992 | 10.739 | 28.89 | 54161.6 |
| render/busy/ms | 8.863 | 11.320 | 39.59 | 53382.3 |
| stage/main/post_update | 5.584 | 6.998 | 11.30 | 33630.9 |
| stage/render/render | 5.479 | 7.127 | 15.21 | 32997.2 |
| render/graph/ms | 4.743 | 6.351 | 14.19 | 28565.1 |
| encode//sum | 3.743 | 4.993 | 23.84 | 22542.5 |
| stage/render/graph_submit | 3.552 | 4.804 | 10.55 | 21392.5 |
| encode/submit | 2.272 | 3.241 | 7.02 | 13681.3 |
| stage/main/update | 2.154 | 2.560 | 22.53 | 12975.9 |
| encode/finish_wall | 1.350 | 1.953 | 4.80 | 8133.3 |
| stage/render/camera_driver | 1.138 | 1.596 | 4.38 | 6854.7 |
| encode/pass:shadow_point_light/sum | 0.938 | 3.523 | 16.51 | 5647.3 |
| encode/pass:shadow_directional_light/sum | 0.854 | 1.536 | 2.52 | 5142.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.772 | 1.087 | 1.63 | 4646.9 |
| stage/render/prepare_write_phase_buffers | 0.770 | 1.099 | 2.91 | 4635.4 |
| encode/pass:early prepass/sum | 0.748 | 1.065 | 3.69 | 4504.4 |
| stage/render/prepare_assets | 0.569 | 0.726 | 8.98 | 3426.9 |
| bigspace/total/ms | 0.540 | 1.327 | 2.33 | 3250.4 |
| render/present/ms | 0.451 | 1.156 | 2.54 | 2716.8 |
| stage/render/prepare_resources | 0.448 | 0.531 | 0.98 | 2698.8 |
| stage/render/prepare_meshes | 0.446 | 0.573 | 7.59 | 2687.5 |
| stage/main/first | 0.395 | 0.517 | 5.09 | 2378.6 |
| stage/main/pre_update | 0.356 | 0.513 | 1.45 | 2141.3 |
| stage/render/specialize | 0.331 | 0.428 | 14.57 | 1993.9 |
| terrain/update/ms | 0.308 | 0.395 | 1.97 | 1853.8 |
| stage/render/queue | 0.299 | 0.433 | 7.94 | 1800.4 |
| stage/render/prepare_bind_groups | 0.285 | 0.513 | 2.37 | 1718.9 |
| terrain/select/ms | 0.274 | 0.353 | 1.01 | 1650.5 |
| controller/feed/ms | 0.267 | 0.335 | 0.84 | 1607.2 |
| stream/drive/ms | 0.251 | 0.335 | 20.52 | 1513.7 |
| encode/pass:main_transparent_pass_3d/sum | 0.234 | 0.347 | 0.84 | 1410.6 |
| bigspace/low_precision/ms | 0.226 | 0.730 | 1.80 | 1358.2 |
| stage/main/fixed_main_loop | 0.214 | 0.369 | 1.34 | 1290.2 |
| far/select/ms | 0.198 | 0.271 | 0.70 | 1193.2 |
| stage/render/create_views | 0.195 | 0.251 | 1.00 | 1171.9 |
| stream/tick/ms | 0.194 | 0.265 | 0.91 | 1167.7 |
| stage/render/prepare_batch_phases | 0.189 | 0.271 | 0.58 | 1138.8 |
| pipelines/compile_ms | 0.170 | 0.000 | 140.33 | 1025.3 |
| stage/render/extract_commands | 0.170 | 0.199 | 0.43 | 1024.8 |
| pipelines/front_ms | 0.165 | 0.000 | 137.02 | 995.0 |
| stage/render/prepare_flush | 0.162 | 0.307 | 0.74 | 974.1 |
| anim/evaluate/ms | 0.161 | 0.256 | 2.41 | 969.4 |
| anim/evaluate_cpu/ms | 0.154 | 0.193 | 0.67 | 930.5 |
| bigspace/high_precision/ms | 0.153 | 0.644 | 1.69 | 921.2 |
| controller/move/ms | 0.144 | 0.325 | 0.73 | 868.6 |
| stage/render/prepare_views | 0.137 | 0.169 | 0.55 | 824.2 |
| stage/main/last | 0.115 | 0.160 | 1.63 | 692.9 |
| encode/pass:main_transmissive_pass_3d/sum | 0.097 | 0.153 | 0.66 | 583.4 |
| render/acquire/ms | 0.093 | 0.118 | 0.36 | 559.4 |
| stage/render/prepare_collect_phase_buffers | 0.060 | 0.129 | 0.66 | 362.9 |
| physics/step/ms | 0.046 | 0.085 | 0.59 | 276.4 |
| stage/render/cleanup | 0.033 | 0.106 | 0.73 | 199.2 |
| physics/insert/ms | 0.031 | 0.349 | 0.71 | 189.0 |
| encode/resolve_diagnostics/sum | 0.031 | 0.040 | 0.53 | 183.7 |
| stage/render/phase_sort | 0.028 | 0.049 | 0.51 | 171.0 |
| weather/map/ms | 0.024 | 0.032 | 0.21 | 147.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.62 | 90.8 |
| anim/apply/ms | 0.012 | 0.021 | 0.35 | 71.4 |
| encode/camera_driver_span_end/sum | 0.012 | 0.019 | 0.52 | 69.4 |
| controller/avoid/ms | 0.011 | 0.014 | 0.04 | 64.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.24 | 61.4 |
| script/update/ms | 0.007 | 0.009 | 0.22 | 43.5 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 31.4 |
| stream/despawn/ms | 0.005 | 0.000 | 1.16 | 30.6 |
| pipelines/driver_ms | 0.005 | 0.000 | 4.38 | 30.3 |
| stream/spawn/ms | 0.005 | 0.000 | 1.14 | 28.8 |
| script/ui/ms | 0.004 | 0.005 | 0.14 | 24.3 |
| controller/write/ms | 0.004 | 0.004 | 0.19 | 21.4 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.58 | 14.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.20 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 30 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1277 | 2218 | 641 |
| crowd/c0/meshes/count | count | 353 | 633 | 174 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5555 |
| crowd/c1/meshes/count | count | 1167 | 1254 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4617 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4595 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.40 | 4.06 | 0.41 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.62 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.52 | 0.01 |
| encode/finish_critical | ms | 0.96 | 4.06 | 1.38 |
| encode/pass:early prepass/max | ms | 0.77 | 3.69 | 0.88 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.63 | 0.78 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.66 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.84 | 0.28 |
| encode/pass:shadow_directional_light/max | ms | 0.80 | 2.52 | 1.38 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.39 | 0.86 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.02 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.21 | 39.59 | 9.42 |
| gpu/timed/ms | ms | 7.97 | 8.98 | 8.14 |
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
| phases/alpha_mask3d/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 809 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3399 | 5110 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1296 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3399 | 5110 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 202 | 30 |
| phases/shadow/bins | count | 5033 | 8908 | 2431 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.3 k | 6386 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 257 | 30 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 186 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 13 | 561 | 63 |
| preprocess/bind_groups_created | count | 13 | 561 | 63 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 130 | 245 | 120 |
| process/cpu_usage | % | 26.44 | 43.48 | 22.60 |
| process/mem_usage | GiB | 5.51 | 5.67 | 5.67 |
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
| skin/blocks_changed/count | count | 14 | 127 | 12 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 366.6 k | 810.8 k | 490.0 k |
| skin/joints_written/count | count | 5728 | 12.7 k | 7656 |
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
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.82 | 41.28 | 28.25 |
| system/mem_usage | % | 55.31 | 55.90 | 55.86 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 69 | 249 | 80 |
| views/all/bins | count | 7653 | 11.8 k | 5929 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.5 k | 38.2 k | 14.0 k |
| views/all/pipelines | count | 251 | 304 | 76 |
| views/all/pipelines_distinct | count | 69 | 73 | 61 |
| views/all/work_items | count | 10.7 k | 34.9 k | 14.3 k |
| views/cascade/batch_sets | count | 9 | 48 | 30 |
| views/cascade/bins | count | 2806 | 6171 | 2431 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8908 | 23.5 k | 6386 |
| views/cascade/pipelines | count | 36 | 54 | 30 |
| views/cascade/work_items | count | 412 | 22.8 k | 6386 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2159 | 2766 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2418 | 3060 | 0 |
| views/lamp_face/pipelines | count | 168 | 209 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2932 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2888 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8410 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8574 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 56.17 | 241 | 338.3 k |
| anim/events/count | count | 0.09 | 3 | 523 |
| assets/Image/added | count | 0.10 | 33 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 152 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.59 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.78 | 1305 | 4675 |
| encode//count | count | 33.95 | 220 | 204.5 k |
| encode/bind_walk/init_textures | count | 12.9 k | 114.7 k | 77.55 M |
| encode/bind_walk/init_textures_skipped | count | 70.0 k | 167.8 k | 421.78 M |
| encode/bind_walk/inits | count | 461 | 1127 | 2.77 M |
| encode/bind_walk/inits_skipped | count | 111 | 281 | 669.1 k |
| encode/bind_walk/merge_textures | count | 12.8 k | 114.6 k | 77.11 M |
| encode/bind_walk/merge_textures_skipped | count | 5368 | 10.2 k | 32.33 M |
| encode/bind_walk/merges | count | 99.40 | 336 | 598.7 k |
| encode/bind_walk/merges_skipped | count | 85.05 | 213 | 512.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 2 | 6023 |
| encode/camera_driver_span_end/count | count | 1 | 2 | 6023 |
| encode/pass:early prepass/count | count | 1 | 2 | 6023 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 2 | 6023 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 2 | 6023 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 2 | 6023 |
| encode/pass:shadow_directional_light/count | count | 1 | 2 | 6023 |
| encode/pass:shadow_point_light/count | count | 1.50 | 48 | 9035 |
| encode/queue/command_buffers | count | 45.47 | 278 | 273.9 k |
| encode/queue/submits | count | 2.02 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.88 M | 11.34 G |
| encode/queue/write_buffer/calls | count | 260 | 837 | 1.57 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.7 k | 3.52 M | 1.07 G |
| encode/queue/write_buffer_with/calls | count | 11.01 | 465 | 66.3 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 70.87 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 2 | 6023 |
| physics/inserts | count | 0.37 | 110 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.65 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 8 |
| pipelines/created | count | 0.00 | 2 | 12 |
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
| stream/upload/bytes | bytes | 5621 | 2.83 M | 33.85 M |
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

## Hitches (> 26.5 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2560.1 | main/busy/ms 2530, stage/main/post_update 2426, stage/main/update 95.42, surface/advance_cpu_ms 66.10, stage/main/pre_update 4.54, stage/main/last 1.17, surface/cell_upload_ms 0.79, stage/main/first 0.44, stage/main/fixed_main_loop 0.41, stream/drive/ms 0.40, weather/map/ms 0.12, stream/tick/ms 0.09 |
| warm-up | 3.32 | 762.6 | main/busy/ms 2666, stage/main/post_update 2449, render/thread/ms 729, render/busy/ms 557, stage/render/prepare_assets 506, stage/main/update 202, stage/render/prepare_views 166, render/acquire/ms 166, surface/advance_cpu_ms 66.10, stage/render/create_views 28.89, stage/render/render 15.19, render/graph/ms 8.66 |
| warm-up | 3.50 | 179.6 | render/thread/ms 889, render/busy/ms 623, stage/render/prepare_assets 557, pipelines/compile_ms 168, main/busy/ms 167, stage/render/prepare_views 167, render/acquire/ms 166, stage/main/update 115, stage/render/render 115, render/present/ms 100, pipelines/driver_ms 87.99, pipelines/front_ms 80.06 |
| warm-up | 3.73 | 214.1 | pipelines/wait_ms 4110, pipelines/compile_ms 2587, pipelines/front_ms 2328, pipelines/driver_ms 259, main/busy/ms 140, render/thread/ms 124, render/busy/ms 121, stage/render/prepare_assets 96.26, stage/main/update 77.17, stage/main/post_update 50.61, bigspace/total/ms 28.10, bigspace/high_precision/ms 27.95 |
| warm-up | 3.81 | 47.0 | pipelines/compile_ms 640, pipelines/front_ms 602, render/thread/ms 70.61, render/busy/ms 68.85, stage/render/prepare_assets 51.14, pipelines/driver_ms 37.86, main/busy/ms 35.74, pipelines/wait_ms 28.37, stage/main/post_update 15.68, terrain/update/ms 14.03, stage/main/update 12.81, stage/render/render 9.52 |
| warm-up | 3.86 | 29.4 | pipelines/compile_ms 677, pipelines/front_ms 640, pipelines/wait_ms 218, render/thread/ms 48.63, render/busy/ms 46.41, main/busy/ms 41.54, pipelines/driver_ms 37.37, stage/main/post_update 27.40, encode//sum 14.91, stage/render/render 14.50, render/graph/ms 12.17, stage/render/prepare_resources 11.16 |
| warm-up | 3.89 | 31.6 | pipelines/compile_ms 631, pipelines/front_ms 599, pipelines/wait_ms 172, main/busy/ms 76.84, render/thread/ms 58.54, render/busy/ms 55.86, stage/main/update 46.39, stream/drive/ms 42.20, pipelines/driver_ms 31.97, stage/main/post_update 26.14, stage/render/prepare_resources 21.51, stage/render/render 13.55 |
| warm-up | 3.97 | 77.6 | pipelines/compile_ms 1222, pipelines/front_ms 1165, render/thread/ms 69.45, main/busy/ms 68.37, render/busy/ms 66.86, pipelines/driver_ms 56.48, stage/main/update 47.65, stream/drive/ms 42.92, stage/render/prepare_assets 34.54, stage/main/post_update 15.53, stage/render/render 13.39, stage/render/prepare_resources 13.12 |
| warm-up | 4.03 | 52.8 | pipelines/compile_ms 465, pipelines/front_ms 433, render/thread/ms 44.31, render/busy/ms 42.71, main/busy/ms 36.30, pipelines/driver_ms 32.55, stage/render/prepare_assets 26.18, stage/main/update 17.56, stage/main/post_update 12.53, stream/drive/ms 12.25, stage/render/render 11.49, render/graph/ms 9.50 |
| warm-up | 4.07 | 38.2 | pipelines/compile_ms 707, pipelines/front_ms 667, render/thread/ms 62.36, main/busy/ms 61.35, render/busy/ms 52.82, stage/main/update 40.46, pipelines/driver_ms 39.34, stream/drive/ms 34.97, stage/render/prepare_assets 32.95, stage/render/render 20.92, stage/main/post_update 14.98, render/graph/ms 11.06 |
| warm-up | 4.15 | 76.0 | pipelines/compile_ms 569, pipelines/front_ms 547, render/thread/ms 80.94, render/busy/ms 71.32, main/busy/ms 66.99, stage/main/update 42.50, stage/render/prepare_assets 37.00, stream/drive/ms 36.38, stage/render/render 28.97, pipelines/driver_ms 22.10, render/graph/ms 18.92, stage/main/post_update 13.97 |
| warm-up | 4.22 | 68.2 | pipelines/compile_ms 579, pipelines/front_ms 553, render/thread/ms 103, render/busy/ms 101, main/busy/ms 72.19, stage/render/prepare_assets 65.78, stage/main/update 40.15, pipelines/driver_ms 26.20, stage/render/render 22.17, render/graph/ms 19.50, stage/main/post_update 18.70, pipelines/wait_ms 16.25 |
| warm-up | 4.34 | 127.6 | pipelines/compile_ms 1977, pipelines/front_ms 1924, render/thread/ms 137, render/busy/ms 134, stage/render/prepare_assets 80.53, main/busy/ms 71.90, pipelines/driver_ms 53.48, stage/main/update 42.52, stage/render/prepare_write_phase_buffers 24.74, pipelines/wait_ms 21.76, pipelines/lock_ms 20.66, stage/render/render 18.94 |
| warm-up | 4.42 | 72.7 | pipelines/compile_ms 1988, pipelines/front_ms 1925, render/thread/ms 146, render/busy/ms 143, pipelines/wait_ms 124, stage/render/prepare_assets 66.11, pipelines/driver_ms 62.72, main/busy/ms 51.00, stage/render/render 32.25, pipelines/lock_ms 31.97, render/graph/ms 29.09, stage/render/prepare_write_phase_buffers 26.47 |
| warm-up | 4.47 | 54.8 | pipelines/compile_ms 309, pipelines/front_ms 286, pipelines/wait_ms 107, render/thread/ms 103, render/busy/ms 101, main/busy/ms 52.81, encode//sum 38.89, stage/render/render 36.73, render/graph/ms 34.03, stage/render/prepare_assets 32.00, stage/render/graph_submit 25.59, pipelines/driver_ms 22.36 |
| warm-up | 4.56 | 86.1 | pipelines/compile_ms 862, pipelines/front_ms 838, pipelines/wait_ms 306, render/thread/ms 123, render/busy/ms 121, encode//sum 51.77, stage/render/render 47.34, render/graph/ms 44.81, main/busy/ms 41.89, stage/render/camera_driver 25.91, stage/render/prepare_flush 24.23, pipelines/driver_ms 23.78 |
| warm-up | 4.66 | 104.3 | pipelines/compile_ms 3298, pipelines/front_ms 3147, pipelines/wait_ms 1266, render/thread/ms 181, render/busy/ms 178, pipelines/driver_ms 151, main/busy/ms 88.22, stage/render/render 53.11, encode//sum 52.66, render/graph/ms 50.39, stage/main/post_update 40.09, stage/render/prepare_resources 32.18 |
| warm-up | 4.70 | 42.9 | pipelines/compile_ms 2491, pipelines/front_ms 2362, pipelines/wait_ms 960, render/thread/ms 131, pipelines/driver_ms 130, render/busy/ms 129, main/busy/ms 83.54, stage/main/post_update 42.23, stage/render/prepare_resources 31.84, encode//sum 30.91, stage/main/update 28.86, stage/render/render 25.66 |
| warm-up | 4.76 | 52.0 | pipelines/compile_ms 248, pipelines/front_ms 241, main/busy/ms 73.36, render/thread/ms 61.57, render/busy/ms 60.25, stage/main/update 54.34, stream/drive/ms 46.59, stage/render/prepare_assets 40.13, pipelines/wait_ms 18.55, stage/main/post_update 10.23, stage/render/render 9.47, render/graph/ms 7.54 |
| warm-up | 4.81 | 50.9 | pipelines/compile_ms 1604, pipelines/front_ms 1568, pipelines/wait_ms 756, main/busy/ms 109, stage/main/update 88.92, render/thread/ms 52.38, render/busy/ms 50.54, stream/drive/ms 47.25, pipelines/driver_ms 35.73, stage/render/prepare_assets 27.12, stage/render/render 13.64, render/graph/ms 10.96 |
| warm-up | 4.92 | 112.4 | pipelines/compile_ms 1610, pipelines/front_ms 1576, pipelines/wait_ms 747, main/busy/ms 145, stage/main/update 119, render/thread/ms 82.25, render/busy/ms 73.07, stream/drive/ms 53.84, pipelines/driver_ms 34.34, stage/render/prepare_assets 31.21, stage/render/render 19.47, render/graph/ms 15.77 |
| warm-up | 5.02 | 95.7 | pipelines/compile_ms 1678, pipelines/front_ms 1628, main/busy/ms 119, render/thread/ms 108, render/busy/ms 99.19, stage/main/update 94.60, pipelines/wait_ms 61.56, stream/drive/ms 53.87, pipelines/driver_ms 50.03, stage/render/prepare_assets 44.33, pipelines/lock_ms 19.26, stage/render/render 18.38 |
| warm-up | 5.05 | 39.3 | pipelines/compile_ms 2503, pipelines/front_ms 2432, main/busy/ms 88.83, pipelines/wait_ms 84.67, render/thread/ms 75.29, render/busy/ms 71.27, pipelines/driver_ms 71.08, stage/main/update 69.60, stream/drive/ms 47.83, pipelines/lock_ms 41.35, stage/render/prepare_assets 27.06, stage/render/render 19.96 |
| warm-up | 5.12 | 66.1 | pipelines/compile_ms 1487, pipelines/front_ms 1439, main/busy/ms 79.62, stage/main/update 63.20, render/thread/ms 52.57, render/busy/ms 48.73, pipelines/driver_ms 47.56, stream/drive/ms 47.05, pipelines/wait_ms 34.89, pipelines/lock_ms 33.13, stage/render/prepare_assets 21.04, stage/render/render 16.48 |
| warm-up | 5.18 | 40.6 | pipelines/compile_ms 307, pipelines/front_ms 297, main/busy/ms 48.71, stage/main/update 36.05, render/thread/ms 31.80, stream/drive/ms 31.23, render/busy/ms 30.20, stage/render/render 11.20, stage/render/prepare_assets 10.13, pipelines/driver_ms 9.61, render/graph/ms 9.04, encode//sum 8.91 |
| warm-up | 5.28 | 29.3 | render/thread/ms 43.74, render/busy/ms 42.17, main/busy/ms 39.24, stage/main/post_update 22.33, pipelines/compile_ms 20.20, pipelines/front_ms 19.71, stage/render/render 16.09, render/graph/ms 13.87, stage/main/update 10.87, stage/render/graph_submit 10.32, encode//sum 8.50, stage/render/prepare_assets 7.72 |
| warm-up | 5.31 | 38.3 | pipelines/compile_ms 178, pipelines/front_ms 171, render/thread/ms 61.39, render/busy/ms 59.35, main/busy/ms 49.59, stage/main/post_update 24.12, stage/render/render 19.62, render/graph/ms 16.78, stage/render/prepare_write_phase_buffers 16.38, stage/main/update 14.51, stage/render/graph_submit 12.34, encode//sum 8.63 |
| warm-up | 5.36 | 43.6 | pipelines/compile_ms 298, pipelines/front_ms 286, render/thread/ms 69.96, render/busy/ms 68.27, main/busy/ms 40.90, stage/render/prepare_assets 27.25, stage/main/update 17.15, stage/render/prepare_write_phase_buffers 14.28, stage/render/render 14.22, stage/main/post_update 12.89, pipelines/driver_ms 12.09, render/graph/ms 11.91 |
| warm-up | 5.44 | 76.5 | pipelines/compile_ms 989, pipelines/front_ms 955, render/thread/ms 104, render/busy/ms 102, stage/render/prepare_assets 69.23, main/busy/ms 66.42, pipelines/driver_ms 34.72, stage/main/update 31.90, pipelines/wait_ms 28.39, stage/main/fixed_main_loop 16.09, stream/drive/ms 14.01, stage/main/post_update 13.74 |
| warm-up | 5.47 | 37.9 | pipelines/compile_ms 1743, pipelines/front_ms 1691, render/thread/ms 99.73, render/busy/ms 98.41, stage/render/prepare_assets 65.95, main/busy/ms 63.26, pipelines/driver_ms 51.72, pipelines/wait_ms 29.43, stage/main/update 28.39, stage/main/fixed_main_loop 16.40, stage/main/post_update 13.65, stream/drive/ms 13.35 |
| warm-up | 6.17 | 31.3 | pipelines/compile_ms 149, pipelines/front_ms 146, render/thread/ms 43.03, render/busy/ms 41.96, main/busy/ms 31.14, stage/render/prepare_assets 17.38, stage/main/update 13.05, stage/main/post_update 10.81, stage/render/render 8.68, render/graph/ms 7.26, encode//sum 7.01, stage/render/specialize 5.84 |
| warm-up | 6.37 | 29.2 | pipelines/compile_ms 148, pipelines/front_ms 143, main/busy/ms 42.81, render/thread/ms 35.63, render/busy/ms 34.18, stage/main/update 21.10, stage/main/post_update 15.06, stage/render/render 13.97, render/graph/ms 11.37, encode//sum 9.46, stage/render/graph_submit 7.79, stream/drive/ms 6.08 |
| warm-up | 6.40 | 33.3 | pipelines/compile_ms 89.84, pipelines/front_ms 85.71, render/thread/ms 41.90, main/busy/ms 41.04, render/busy/ms 40.43, stage/main/update 18.44, stage/main/post_update 14.63, stage/render/prepare_assets 13.19, stage/render/render 13.15, render/graph/ms 10.51, encode//sum 10.00, stage/render/graph_submit 7.51 |
| warm-up | 6.43 | 30.3 | pipelines/compile_ms 79.91, pipelines/front_ms 76.47, render/thread/ms 42.71, render/busy/ms 41.63, main/busy/ms 34.50, stage/render/prepare_assets 21.23, stage/main/update 15.33, stage/main/post_update 11.97, stage/render/render 9.05, encode//sum 7.77, render/graph/ms 7.37, stage/render/graph_submit 5.37 |
| warm-up | 6.51 | 30.4 | render/thread/ms 38.31, render/busy/ms 37.27, main/busy/ms 33.45, stage/render/prepare_assets 17.87, stage/main/update 14.76, stage/main/post_update 13.79, stage/render/render 8.04, render/graph/ms 6.57, encode//sum 6.27, stage/render/graph_submit 4.44, stream/drive/ms 3.56, bigspace/total/ms 2.35 |
| warm-up | 6.68 | 50.5 | render/thread/ms 55.96, render/busy/ms 54.45, main/busy/ms 45.19, stage/render/prepare_assets 32.93, stage/main/update 28.66, stream/drive/ms 19.08, stage/main/post_update 11.38, stage/render/render 11.28, render/graph/ms 9.17, encode//sum 8.73, stage/render/graph_submit 6.63, encode/submit 4.30 |
| warm-up | 7.04 | 32.8 | render/thread/ms 37.61, render/busy/ms 36.51, main/busy/ms 17.88, stage/render/render 10.92, stage/main/post_update 10.09, stage/render/specialize 9.86, render/graph/ms 9.32, encode//sum 8.11, stage/render/graph_submit 7.31, stage/render/queue 6.61, stage/main/update 5.01, encode/submit 4.16 |
| warm-up | 8.30 | 32.9 | render/thread/ms 37.38, render/busy/ms 36.37, main/busy/ms 18.92, stage/main/post_update 11.05, stage/render/render 10.82, stage/render/specialize 9.78, render/graph/ms 9.40, encode//sum 7.46, stage/render/graph_submit 7.32, stage/render/queue 6.64, stage/main/update 5.25, encode/submit 3.77 |
| measure | 0.45 | 42.9 | render/thread/ms 48.87, render/busy/ms 47.81, main/busy/ms 17.88, stage/render/render 15.91, stage/render/specialize 14.86, render/graph/ms 14.48, stage/render/graph_submit 11.93, stage/main/post_update 10.82, encode/submit 9.25, encode//sum 8.68, stage/render/queue 6.83, stage/main/update 4.64 |
| measure | 1.77 | 31.6 | render/thread/ms 37.89, render/busy/ms 36.83, main/busy/ms 17.40, stage/render/render 12.05, stage/main/post_update 11.12, render/graph/ms 10.52, stage/render/specialize 9.04, encode//sum 8.96, stage/render/graph_submit 7.89, stage/render/queue 6.34, encode/submit 4.87, stage/render/prepare_meshes 4.44 |
| measure | 3.24 | 31.3 | render/thread/ms 42.54, render/busy/ms 41.52, main/busy/ms 20.09, stage/render/render 16.11, render/graph/ms 14.55, stage/main/post_update 13.21, stage/render/graph_submit 10.45, encode//sum 10.15, stage/render/specialize 9.51, encode/pass:shadow_point_light/sum 7.41, encode/submit 6.40, stage/render/queue 6.36 |
| measure | 4.67 | 30.4 | render/thread/ms 33.71, render/busy/ms 32.75, main/busy/ms 19.50, stage/main/post_update 12.79, stage/render/render 9.84, stage/render/specialize 9.16, render/graph/ms 8.39, encode//sum 7.12, stage/render/graph_submit 6.41, stage/render/queue 5.85, stage/main/update 4.07, encode/submit 3.40 |
| measure | 6.03 | 29.7 | render/thread/ms 36.36, render/busy/ms 35.24, main/busy/ms 19.42, stage/main/post_update 13.22, stage/render/render 11.44, render/graph/ms 9.89, stage/render/specialize 9.39, encode//sum 8.17, stage/render/graph_submit 7.52, stage/render/queue 6.64, encode/submit 4.65, stage/main/update 3.69 |
| measure | 8.65 | 30.4 | render/thread/ms 36.73, render/busy/ms 35.76, main/busy/ms 18.86, stage/main/post_update 11.91, stage/render/render 11.43, render/graph/ms 9.94, stage/render/specialize 9.75, encode//sum 8.99, stage/render/graph_submit 7.30, stage/render/queue 6.69, encode/submit 4.42, stage/main/update 4.27 |
| measure | 10.00 | 30.2 | render/thread/ms 35.97, render/busy/ms 34.96, main/busy/ms 19.31, stage/main/post_update 12.01, stage/render/render 11.39, render/graph/ms 9.94, stage/render/specialize 9.54, encode//sum 8.76, stage/render/graph_submit 7.46, stage/render/queue 6.26, stage/main/update 4.55, encode/submit 4.35 |
| measure | 11.29 | 30.2 | render/thread/ms 36.83, render/busy/ms 35.75, main/busy/ms 17.97, stage/main/post_update 11.37, stage/render/render 11.31, render/graph/ms 9.84, stage/render/specialize 9.47, encode//sum 8.95, stage/render/graph_submit 7.40, stage/render/queue 6.72, encode/submit 4.36, stage/main/update 3.90 |
| measure | 15.72 | 29.4 | render/thread/ms 38.69, render/busy/ms 37.68, main/busy/ms 18.72, stage/render/render 11.87, stage/main/post_update 11.27, render/graph/ms 10.38, stage/render/specialize 10.13, stage/render/graph_submit 8.09, encode//sum 7.69, stage/render/queue 7.37, encode/submit 4.52, stage/main/update 4.30 |
| measure | 18.00 | 30.1 | render/thread/ms 36.00, render/busy/ms 34.76, main/busy/ms 18.78, stage/main/post_update 11.67, stage/render/specialize 9.96, stage/render/render 9.51, render/graph/ms 7.98, stage/render/queue 7.00, encode//sum 6.93, stage/render/graph_submit 6.00, stage/main/update 4.30, encode/submit 3.85 |
| measure | 18.16 | 31.8 | main/busy/ms 38.50, stage/main/update 24.51, stream/drive/ms 20.76, render/thread/ms 18.18, render/busy/ms 17.08, stage/main/post_update 11.27, stage/render/render 10.14, render/graph/ms 8.56, encode//sum 7.53, stage/render/graph_submit 6.42, encode/submit 4.09, encode/finish_wall 2.38 |
| measure | 20.74 | 33.1 | render/thread/ms 35.64, render/busy/ms 34.50, main/busy/ms 19.43, stage/main/post_update 12.42, stage/render/specialize 10.15, stage/render/render 9.76, render/graph/ms 8.18, encode//sum 7.83, stage/render/queue 6.80, stage/render/graph_submit 6.04, stage/main/update 4.39, encode/submit 3.63 |
| measure | 23.03 | 33.3 | render/thread/ms 40.70, render/busy/ms 39.58, main/busy/ms 23.96, stage/main/post_update 12.32, stage/render/render 12.12, stage/render/specialize 11.14, render/graph/ms 10.65, encode//sum 8.68, stage/render/graph_submit 7.80, stage/render/queue 7.66, stage/main/update 6.17, encode/submit 4.71 |
| measure | 24.57 | 30.8 | render/thread/ms 37.66, render/busy/ms 36.65, main/busy/ms 20.76, stage/main/post_update 13.06, stage/render/render 11.20, stage/render/specialize 10.31, render/graph/ms 9.77, encode//sum 8.10, stage/render/graph_submit 7.33, stage/render/queue 6.53, encode/pass:shadow_point_light/sum 5.14, stage/main/update 5.09 |
| measure | 27.29 | 34.7 | render/thread/ms 47.36, render/busy/ms 46.33, main/busy/ms 25.76, stage/render/render 16.42, render/graph/ms 14.92, stage/main/post_update 13.00, encode//sum 12.74, stage/render/graph_submit 11.82, stage/render/specialize 11.35, encode/pass:shadow_point_light/sum 10.22, stage/render/queue 8.29, encode/submit 7.80 |
| measure | 28.87 | 31.3 | render/thread/ms 39.74, render/busy/ms 38.71, main/busy/ms 19.27, stage/render/render 12.53, stage/main/post_update 12.01, render/graph/ms 11.14, stage/render/specialize 9.98, stage/render/graph_submit 8.53, encode//sum 8.27, stage/render/queue 6.83, encode/pass:shadow_point_light/sum 5.78, encode/submit 5.10 |
| measure | 30.48 | 31.9 | render/thread/ms 39.40, render/busy/ms 38.33, main/busy/ms 21.48, stage/main/post_update 11.87, stage/render/render 11.39, stage/render/specialize 10.55, render/graph/ms 9.83, stage/render/graph_submit 7.65, stage/render/queue 7.39, encode//sum 7.28, stage/main/update 6.00, encode/submit 4.63 |
| measure | 31.96 | 30.1 | render/thread/ms 38.37, render/busy/ms 37.35, main/busy/ms 18.85, stage/main/post_update 11.91, stage/render/render 11.61, stage/render/specialize 10.12, render/graph/ms 10.03, encode//sum 7.71, stage/render/graph_submit 7.59, stage/render/queue 7.40, encode/submit 4.89, stage/main/update 4.38 |
| measure | 35.35 | 31.4 | render/thread/ms 40.44, render/busy/ms 39.45, main/busy/ms 19.48, stage/main/post_update 12.37, stage/render/render 12.19, render/graph/ms 10.78, stage/render/specialize 10.69, stage/render/graph_submit 8.46, stage/render/queue 7.89, encode//sum 7.56, encode/submit 5.21, stage/main/update 4.24 |
| measure | 40.80 | 35.6 | render/thread/ms 40.28, render/busy/ms 39.14, main/busy/ms 19.68, stage/main/post_update 11.47, stage/render/render 10.96, stage/render/specialize 10.51, render/graph/ms 9.43, stage/render/graph_submit 7.45, encode//sum 6.70, stage/render/queue 6.66, stage/main/update 4.67, encode/submit 3.99 |
| measure | 43.22 | 34.1 | render/thread/ms 37.71, render/busy/ms 36.74, main/busy/ms 16.83, stage/render/render 10.89, stage/render/specialize 10.28, render/graph/ms 9.58, stage/main/post_update 9.32, encode//sum 8.05, stage/render/graph_submit 7.78, stage/render/queue 6.84, stage/main/update 4.39, encode/submit 3.94 |
| measure | 45.66 | 32.8 | render/thread/ms 37.44, render/busy/ms 36.41, main/busy/ms 17.51, stage/render/specialize 10.20, stage/render/render 10.06, stage/main/post_update 9.90, render/graph/ms 8.69, encode//sum 7.09, stage/render/graph_submit 6.75, stage/render/queue 6.71, stage/main/update 4.77, stage/render/prepare_meshes 4.28 |
| measure | 56.44 | 29.2 | render/thread/ms 36.52, render/busy/ms 34.40, main/busy/ms 15.27, stage/render/specialize 9.99, stage/render/render 9.91, stage/main/post_update 8.52, render/graph/ms 7.49, stage/render/queue 6.61, encode//sum 6.51, stage/render/graph_submit 5.43, stage/render/prepare_meshes 4.36, stage/main/update 4.26 |
| measure | 59.04 | 46.5 | render/thread/ms 55.39, render/busy/ms 53.77, main/busy/ms 32.97, stage/main/post_update 19.28, stage/render/render 16.76, render/graph/ms 14.63, stage/render/specialize 11.77, stage/render/graph_submit 10.88, encode//sum 9.91, stage/main/update 8.91, stage/render/prepare_meshes 8.02, stage/render/queue 6.97 |
| measure | 59.07 | 30.5 | render/thread/ms 58.66, render/busy/ms 57.57, main/busy/ms 26.58, stage/render/prepare_assets 13.45, stage/main/post_update 13.21, stage/render/specialize 11.83, stage/render/render 10.94, render/graph/ms 9.57, stage/render/prepare_meshes 8.74, encode//sum 7.83, stage/render/graph_submit 7.56, stage/main/update 7.00 |
| measure | 62.42 | 31.6 | render/thread/ms 38.45, render/busy/ms 37.46, main/busy/ms 19.46, stage/render/render 10.81, stage/render/specialize 10.75, stage/main/post_update 10.53, render/graph/ms 9.44, encode//sum 7.23, stage/render/graph_submit 7.21, stage/render/queue 6.45, stage/main/update 6.27, stage/render/prepare_meshes 4.29 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.day_gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=daygate-5 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **78%** (min 49%) · 2625 MHz (min 2625) · memory 8101 MHz · 79.9 W · 76 °C (max 81) · VRAM 7062 MiB · throttled 100% of samples

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
