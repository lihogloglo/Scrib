# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.atlas.enabled=off (command line: --set); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6095 frames over 65.6 s · **median 10.60 ms (94 fps)** · mean 10.77 · p95 13.00 · p99 15.26 (1% low 66 fps) · worst 33.48 ms · 44 spikes over 1.5× median · 34.7 stutters/min (work 32.0/min)
- warm-up: 353 frames over 9.7 s, worst 2416.9 ms
- hitches (> 26.5 ms): **3** measured, 50 in warm-up
- timed GPU passes: 6.98 ms of a 10.77 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.60 ms, limit 11 ms
- **FAIL** frame_ms: 38 stutters (> 1.5x moving median of 60) in 1.09 min = 34.73/min, limit 1/min
- **FAIL** worst frame 33.48 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.973 | 2.039 | 2.070 | 0.97 | 0.179 |
| main_opaque_pass_3d | 1.608 | 1.661 | 2.367 | 0.97 | 0.066 |
| early prepass | 0.496 | 0.512 | 0.735 | 0.97 | 0.092 |
| bloom | 0.308 | 0.318 | 0.324 | 0.97 | 0.067 |
| atmosphere_luts | 0.303 | 0.313 | 0.377 | 0.97 | 0.003 |
| auxiliary_views | 0.232 | 0.240 | 0.850 | 0.97 | 0.215 |
| water_velocity | 0.219 | 0.226 | 0.247 | 0.97 | 0.002 |
| water_sspr | 0.196 | 0.202 | 0.203 | 0.97 | 0.002 |
| ssao | 0.182 | 0.188 | 0.193 | 0.97 | 0.008 |
| upscaling | 0.126 | 0.130 | 0.136 | 0.97 | 0.004 |
| atmos_froxel_inject | 0.124 | 0.128 | 0.130 | 0.97 | 0.001 |
| water_fft | 0.121 | 0.125 | 0.128 | 0.97 | 0.003 |
| uniform_allocation | 0.112 | 0.116 | 0.140 | 0.97 | 0.022 |
| clouds_composite | 0.101 | 0.105 | 0.109 | 0.97 | 0.002 |
| bin_unpacking | 0.089 | 0.092 | 0.112 | 0.97 | 0.017 |
| gpu_instance_materialization | 0.087 | 0.090 | 0.095 | 0.97 | 0.004 |
| tonemapping | 0.085 | 0.087 | 0.092 | 0.97 | 0.004 |
| main_transmissive_pass_3d | 0.069 | 0.071 | 0.135 | 0.97 | 0.009 |
| clustering | 0.062 | 0.064 | 0.067 | 0.97 | 0.020 |
| clouds_trace | 0.054 | 0.056 | 0.079 | 0.97 | 0.001 |
| early_mesh_preprocessing | 0.053 | 0.055 | 0.071 | 0.97 | 0.004 |
| render_sky | 0.051 | 0.053 | 0.054 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.043 | 0.416 | 0.417 | 0.10 | 0.001 |
| surface_trail | 0.038 | 0.057 | 0.057 | 0.68 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.028 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.023 | 0.220 | 0.231 | 0.10 | 0.004 |
| water_mips | 0.021 | 0.022 | 0.023 | 0.97 | 0.002 |
| range_unpacking | 0.021 | 0.021 | 0.026 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.027 | 0.97 | 0.005 |
| night_sky | 0.017 | 0.017 | 0.044 | 0.97 | 0.002 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.030 | 0.97 | 0.199 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.97 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.526 | 11.712 | 24.01 | 58062.6 |
| render/busy/ms | 9.033 | 11.214 | 23.57 | 55055.5 |
| main/busy/ms | 8.746 | 10.283 | 25.92 | 53308.8 |
| stage/render/render | 5.708 | 7.319 | 14.26 | 34790.3 |
| stage/main/post_update | 5.087 | 6.162 | 8.87 | 31006.6 |
| render/graph/ms | 5.034 | 6.599 | 13.47 | 30680.2 |
| encode//sum | 4.341 | 5.951 | 18.83 | 26456.8 |
| stage/render/graph_submit | 3.608 | 4.762 | 11.79 | 21990.6 |
| encode/submit | 2.321 | 3.236 | 10.08 | 14145.7 |
| stage/main/update | 1.873 | 2.369 | 20.32 | 11414.4 |
| stage/render/camera_driver | 1.362 | 1.886 | 3.60 | 8298.6 |
| encode/finish_wall | 1.334 | 1.765 | 3.30 | 8128.2 |
| encode/pass:main_opaque_pass_3d/sum | 0.810 | 1.163 | 1.86 | 4938.2 |
| encode/pass:shadow_directional_light/sum | 0.807 | 1.282 | 2.21 | 4918.8 |
| encode/pass:early prepass/sum | 0.800 | 1.143 | 2.03 | 4878.7 |
| stage/render/prepare_write_phase_buffers | 0.691 | 1.015 | 2.07 | 4210.5 |
| stage/main/fixed_main_loop | 0.547 | 0.948 | 1.49 | 3334.6 |
| stage/render/prepare_assets | 0.534 | 0.706 | 9.89 | 3252.4 |
| bigspace/total/ms | 0.532 | 1.316 | 2.77 | 3243.3 |
| encode/pass:shadow_point_light/sum | 0.496 | 2.630 | 4.99 | 3021.3 |
| stage/render/prepare_resources | 0.458 | 0.551 | 1.26 | 2790.7 |
| stage/render/prepare_meshes | 0.438 | 0.555 | 3.90 | 2668.6 |
| stage/main/first | 0.407 | 0.526 | 6.83 | 2481.5 |
| render/present/ms | 0.399 | 0.495 | 2.16 | 2434.7 |
| stage/main/pre_update | 0.394 | 0.573 | 3.29 | 2399.4 |
| controller/feed/ms | 0.320 | 0.414 | 1.00 | 1952.1 |
| stage/render/prepare_bind_groups | 0.311 | 0.510 | 2.00 | 1893.5 |
| stage/render/prepare_flush | 0.296 | 0.507 | 1.21 | 1803.4 |
| encode/pass:main_transparent_pass_3d/sum | 0.294 | 0.490 | 1.21 | 1791.2 |
| stage/render/prepare_batch_phases | 0.283 | 0.457 | 0.97 | 1727.7 |
| stage/main/last | 0.253 | 0.305 | 0.87 | 1543.1 |
| bigspace/low_precision/ms | 0.228 | 0.731 | 1.93 | 1387.5 |
| stream/drive/ms | 0.208 | 0.289 | 10.16 | 1266.2 |
| stage/render/create_views | 0.190 | 0.239 | 7.91 | 1155.4 |
| stage/render/extract_commands | 0.186 | 0.225 | 0.43 | 1131.5 |
| stage/render/specialize | 0.183 | 0.273 | 2.12 | 1113.1 |
| anim/evaluate/ms | 0.161 | 0.274 | 1.58 | 979.9 |
| anim/evaluate_cpu/ms | 0.154 | 0.193 | 0.94 | 939.2 |
| bigspace/high_precision/ms | 0.154 | 0.661 | 2.32 | 938.1 |
| stream/tick/ms | 0.152 | 0.214 | 0.69 | 927.8 |
| stage/render/prepare_views | 0.136 | 0.180 | 0.42 | 827.5 |
| stage/render/queue | 0.113 | 0.190 | 2.47 | 686.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.101 | 0.163 | 0.81 | 613.4 |
| stage/render/phase_sort | 0.094 | 0.138 | 0.37 | 574.8 |
| render/acquire/ms | 0.094 | 0.126 | 0.34 | 572.3 |
| controller/write/ms | 0.076 | 0.103 | 0.48 | 463.4 |
| controller/move/ms | 0.073 | 0.195 | 0.59 | 444.9 |
| terrain/update/ms | 0.063 | 0.305 | 1.86 | 387.0 |
| stage/render/prepare_collect_phase_buffers | 0.051 | 0.079 | 0.28 | 309.5 |
| physics/step/ms | 0.043 | 0.086 | 0.64 | 262.5 |
| encode/resolve_diagnostics/sum | 0.032 | 0.044 | 0.62 | 193.3 |
| physics/insert/ms | 0.027 | 0.335 | 0.90 | 166.2 |
| pipelines/compile_ms | 0.027 | 0.000 | 80.59 | 164.0 |
| pipelines/front_ms | 0.026 | 0.000 | 77.89 | 158.5 |
| terrain/select/ms | 0.025 | 0.268 | 0.72 | 151.8 |
| stage/render/cleanup | 0.023 | 0.033 | 0.88 | 142.2 |
| weather/map/ms | 0.018 | 0.024 | 0.10 | 108.6 |
| far/select/ms | 0.015 | 0.166 | 0.61 | 94.0 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.46 | 82.7 |
| anim/apply/ms | 0.012 | 0.021 | 0.15 | 73.8 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.28 | 65.9 |
| weather/eval/ms | 0.011 | 0.014 | 0.18 | 65.3 |
| script/update/ms | 0.008 | 0.012 | 0.10 | 50.9 |
| controller/avoid/ms | 0.007 | 0.013 | 0.14 | 40.1 |
| stream/despawn/ms | 0.006 | 0.000 | 0.89 | 35.7 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 30.5 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 22.4 |
| script/ui/ms | 0.004 | 0.005 | 0.33 | 21.7 |
| stream/upload/ms | 0.003 | 0.000 | 1.09 | 19.7 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 6.6 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.70 | 5.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.18 | 5.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 34 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 42 | 12 |
| crowd/c0/joints/count | count | 1301 | 2218 | 538 |
| crowd/c0/meshes/count | count | 353 | 633 | 129 |
| crowd/c1/count | count | 145 | 154 | 149 |
| crowd/c1/joints/count | count | 5503 | 5894 | 5658 |
| crowd/c1/meshes/count | count | 1168 | 1254 | 1212 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 216 | 204 |
| crowd/joints_extracted/count | count | 697 | 7035 | 582 |
| crowd/joints_uploaded/count | count | 3597 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.47 | 1.70 | 0.61 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.46 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.01 |
| encode/finish_critical | ms | 0.97 | 2.21 | 1.78 |
| encode/pass:early prepass/max | ms | 0.79 | 2.03 | 1.01 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 1.86 | 0.93 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.81 | 0.63 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.27 | 1.21 | 0.36 |
| encode/pass:shadow_directional_light/max | ms | 0.78 | 2.21 | 1.78 |
| encode/pass:shadow_point_light/max | ms | 0.13 | 1.87 | 0.56 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.62 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.07 | 25.92 | 9.98 |
| gpu/timed/ms | ms | 7.91 | 9.30 | 8.50 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 154 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 317 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 766 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2479 | 3552 | 2457 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 766 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2479 | 3552 | 2457 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 25 | 147 | 27 |
| phases/shadow/bins | count | 1189 | 1605 | 658 |
| phases/shadow/instance_batches | count | 4 | 8 | 8 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| phases/shadow/items | count | 2854 | 3549 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 159 | 209 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2678 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 184 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 334 | 335 | 335 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 17 | 443 | 43 |
| preprocess/bind_groups_created | count | 12 | 437 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 6 |
| preprocess/bind_groups_reused | count | 129 | 248 | 123 |
| process/cpu_usage | % | 26.44 | 37.98 | 24.04 |
| process/mem_usage | GiB | 5.51 | 5.63 | 5.63 |
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
| skin/blocks_changed/count | count | 14 | 121 | 13 |
| skin/blocks_skipped/count | count | 145 | 205 | 191 |
| skin/blocks_walked/count | count | 14 | 26 | 13 |
| skin/joint_bytes_written/bytes | bytes | 49.2 k | 811.6 k | 38.7 k |
| skin/joints_written/count | count | 768 | 12.7 k | 604 |
| skin/upload_writes/count | count | 7 | 21 | 10 |
| started_load_count |  | 6439 | 6564 | 6564 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.88 | 59.48 | 43.10 |
| system/mem_usage | % | 55.02 | 56.67 | 56.47 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 81 | 206 | 85 |
| views/all/bins | count | 3120 | 3757 | 2948 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 16 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 27.1 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 196.6 k |
| views/all/items | count | 8462 | 10.6 k | 7958 |
| views/all/pipelines | count | 205 | 256 | 66 |
| views/all/pipelines_distinct | count | 69 | 72 | 61 |
| views/all/work_items | count | 154.1 k | 206.9 k | 205.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 27 |
| views/cascade/bins | count | 719 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 8 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 13.5 k |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 98.3 k |
| views/cascade/items | count | 2112 | 2445 | 2014 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 100.3 k |
| views/lamp_face/batch_sets | count | 0 | 128 | 0 |
| views/lamp_face/bins | count | 485 | 811 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 24 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 719 | 1138 | 0 |
| views/lamp_face/pipelines | count | 133 | 181 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 666 | 0 |
| views/main/batch_sets | count | 57 | 61 | 58 |
| views/main/bins | count | 2010 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5910 | 7524 | 5944 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 104.6 k | 106.0 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 21.3 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 46.71 | 227 | 284.7 k |
| anim/events/count | count | 0.09 | 3 | 527 |
| assets/Image/added | count | 0.10 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.47 | 295 | 2874 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.23 | 10 | 7470 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.24 | 1154 | 1490 |
| encode//count | count | 37.42 | 204 | 228.1 k |
| encode/bind_walk/init_textures | count | 10.0 k | 26.5 k | 61.13 M |
| encode/bind_walk/init_textures_skipped | count | 53.0 k | 75.7 k | 323.32 M |
| encode/bind_walk/inits | count | 562 | 978 | 3.43 M |
| encode/bind_walk/inits_skipped | count | 135 | 235 | 825.3 k |
| encode/bind_walk/merge_textures | count | 9966 | 26.4 k | 60.74 M |
| encode/bind_walk/merge_textures_skipped | count | 11.7 k | 26.6 k | 71.47 M |
| encode/bind_walk/merges | count | 104 | 269 | 632.8 k |
| encode/bind_walk/merges_skipped | count | 126 | 231 | 767.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6095 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6095 |
| encode/pass:early prepass/count | count | 1 | 1 | 6095 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6095 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6095 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6095 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6095 |
| encode/pass:shadow_point_light/count | count | 2.37 | 44 | 14.4 k |
| encode/queue/command_buffers | count | 49.79 | 258 | 303.5 k |
| encode/queue/submits | count | 2.01 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.27 M | 14.21 G |
| encode/queue/write_buffer/calls | count | 274 | 675 | 1.67 M |
| encode/queue/write_buffer_with/bytes | bytes | 161.3 k | 1.28 M | 983.26 M |
| encode/queue/write_buffer_with/calls | count | 10.40 | 894 | 63.4 k |
| encode/queue/write_texture/bytes | bytes | 2089 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6095 |
| physics/inserts | count | 0.37 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 175 | 2335 |
| physics/steps | count | 0.65 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 1 | 3 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 27 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5554 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1218 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.5 ms; slowest 53, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.42 | 2416.9 | main/busy/ms 2385, stage/main/post_update 2342, stage/main/update 31.99, stage/main/pre_update 6.99, stage/main/last 1.03, surface/advance_cpu_ms 0.82, stream/drive/ms 0.64, stage/main/first 0.58, stage/main/fixed_main_loop 0.37, surface/cell_upload_ms 0.32, stream/tick/ms 0.12, bigspace/total/ms 0.07 |
| warm-up | 3.17 | 748.8 | main/busy/ms 2523, stage/main/post_update 2366, render/thread/ms 709, render/busy/ms 571, stage/render/prepare_assets 525, stage/main/update 141, stage/render/prepare_views 136, render/acquire/ms 136, stage/render/create_views 26.63, stage/render/render 11.48, stage/main/pre_update 8.58, render/graph/ms 7.60 |
| warm-up | 3.22 | 58.6 | render/thread/ms 750, render/busy/ms 610, stage/render/prepare_assets 551, main/busy/ms 160, stage/render/prepare_views 137, render/acquire/ms 136, stage/main/update 114, pipelines/compile_ms 64.42, pipelines/front_ms 59.67, stage/main/post_update 35.40, stage/render/create_views 27.07, terrain/update/ms 18.20 |
| warm-up | 3.26 | 37.1 | pipelines/wait_ms 249, pipelines/compile_ms 172, pipelines/front_ms 158, render/thread/ms 77.01, render/busy/ms 67.30, main/busy/ms 57.20, stage/main/post_update 38.85, stage/render/render 36.01, stage/render/prepare_assets 26.90, render/graph/ms 25.96, terrain/update/ms 25.02, pipelines/driver_ms 13.70 |
| warm-up | 3.37 | 110.4 | pipelines/wait_ms 2608, pipelines/compile_ms 962, pipelines/front_ms 904, render/thread/ms 144, render/busy/ms 135, main/busy/ms 79.86, pipelines/driver_ms 58.78, stage/render/render 42.64, stage/main/post_update 37.70, stage/render/prepare_resources 35.23, render/graph/ms 33.03, stage/main/update 31.15 |
| warm-up | 3.43 | 53.7 | pipelines/wait_ms 5256, pipelines/compile_ms 1430, pipelines/front_ms 1350, render/thread/ms 160, render/busy/ms 158, pipelines/driver_ms 79.67, main/busy/ms 66.14, stage/render/prepare_resources 53.58, stage/main/update 32.59, stage/render/queue 24.23, stage/render/prepare_assets 23.44, stage/main/post_update 21.58 |
| warm-up | 3.48 | 57.9 | pipelines/wait_ms 3183, pipelines/compile_ms 1122, pipelines/front_ms 1066, render/thread/ms 108, render/busy/ms 106, main/busy/ms 96.54, stage/main/update 57.29, pipelines/driver_ms 55.38, stage/render/prepare_resources 33.38, stage/render/prepare_assets 30.92, stage/render/render 26.14, render/graph/ms 23.33 |
| warm-up | 3.56 | 78.7 | pipelines/compile_ms 1688, pipelines/front_ms 1600, pipelines/wait_ms 354, render/thread/ms 112, render/busy/ms 109, main/busy/ms 99.39, pipelines/driver_ms 88.52, stage/main/update 59.66, stage/render/prepare_assets 57.29, stage/render/render 27.58, render/graph/ms 24.86, stage/main/post_update 19.70 |
| warm-up | 3.62 | 35.3 | pipelines/compile_ms 378, pipelines/front_ms 363, pipelines/wait_ms 195, render/thread/ms 51.26, render/busy/ms 49.56, main/busy/ms 32.04, stage/render/prepare_assets 30.08, pipelines/driver_ms 15.08, stage/main/post_update 14.28, stage/render/render 11.73, stage/main/update 9.58, terrain/update/ms 9.55 |
| warm-up | 3.71 | 69.8 | pipelines/compile_ms 850, pipelines/front_ms 816, render/thread/ms 90.38, render/busy/ms 88.09, main/busy/ms 77.17, pipelines/wait_ms 63.92, stage/main/update 58.18, stage/render/prepare_flush 50.23, pipelines/driver_ms 34.41, encode//sum 27.25, stage/render/render 18.06, render/graph/ms 15.23 |
| warm-up | 3.77 | 59.3 | pipelines/compile_ms 1579, pipelines/front_ms 1511, render/thread/ms 105, render/busy/ms 102, main/busy/ms 80.40, pipelines/wait_ms 68.68, pipelines/driver_ms 67.89, stage/main/update 60.84, stage/render/prepare_flush 50.28, stage/render/prepare_assets 27.64, stage/render/render 17.67, render/graph/ms 14.76 |
| warm-up | 3.82 | 47.3 | pipelines/compile_ms 1441, pipelines/front_ms 1372, render/thread/ms 74.69, render/busy/ms 70.68, pipelines/driver_ms 69.30, main/busy/ms 37.27, stage/render/prepare_assets 36.96, stage/render/render 26.78, render/graph/ms 22.42, pipelines/wait_ms 17.14, stage/main/update 17.01, stage/main/post_update 10.71 |
| warm-up | 3.86 | 39.4 | pipelines/compile_ms 1042, pipelines/front_ms 996, render/thread/ms 69.75, render/busy/ms 65.81, pipelines/driver_ms 45.93, pipelines/wait_ms 44.45, pipelines/lock_ms 33.86, stage/render/prepare_assets 33.03, main/busy/ms 30.12, stage/render/render 25.42, render/graph/ms 21.20, stage/main/update 14.40 |
| warm-up | 3.90 | 41.1 | pipelines/compile_ms 635, pipelines/front_ms 611, render/thread/ms 56.84, render/busy/ms 55.28, stage/render/prepare_assets 35.87, pipelines/wait_ms 34.74, pipelines/lock_ms 33.45, pipelines/driver_ms 24.12, main/busy/ms 18.54, stage/render/render 12.62, render/graph/ms 10.56, stage/main/update 8.37 |
| warm-up | 4.00 | 73.2 | pipelines/compile_ms 454, pipelines/front_ms 440, render/thread/ms 68.40, render/busy/ms 64.88, main/busy/ms 55.04, stage/main/update 29.43, stream/drive/ms 19.63, stage/render/prepare_flush 19.47, stage/main/post_update 17.82, stage/render/render 15.37, pipelines/driver_ms 14.41, render/graph/ms 11.67 |
| warm-up | 4.06 | 59.2 | pipelines/compile_ms 1102, pipelines/front_ms 1064, render/thread/ms 90.63, render/busy/ms 87.87, main/busy/ms 62.39, pipelines/driver_ms 37.98, stage/render/prepare_assets 25.69, stage/main/update 23.54, stage/main/post_update 20.59, stage/render/prepare_flush 19.32, stage/render/render 16.34, render/graph/ms 13.32 |
| warm-up | 4.18 | 120.2 | pipelines/compile_ms 1671, pipelines/front_ms 1619, render/thread/ms 116, render/busy/ms 115, stage/render/prepare_assets 88.53, main/busy/ms 75.91, pipelines/driver_ms 52.24, stage/main/update 46.41, stage/render/render 13.89, stage/main/post_update 12.02, render/graph/ms 11.71, pipelines/wait_ms 9.60 |
| warm-up | 4.25 | 66.6 | pipelines/compile_ms 1748, pipelines/front_ms 1701, main/busy/ms 163, stage/main/update 141, render/thread/ms 131, render/busy/ms 129, stage/render/prepare_assets 96.61, pipelines/driver_ms 47.14, stage/render/render 25.92, render/graph/ms 23.64, stage/render/graph_submit 21.14, pipelines/wait_ms 12.53 |
| warm-up | 4.38 | 131.0 | pipelines/compile_ms 1302, pipelines/front_ms 1262, main/busy/ms 141, stage/main/update 119, render/thread/ms 90.81, render/busy/ms 88.78, stage/render/prepare_assets 48.45, pipelines/driver_ms 40.20, stage/render/render 29.49, render/graph/ms 27.09, stage/render/graph_submit 24.63, stage/main/post_update 9.27 |
| warm-up | 4.48 | 99.5 | pipelines/compile_ms 1900, pipelines/front_ms 1834, render/thread/ms 135, render/busy/ms 133, main/busy/ms 111, pipelines/wait_ms 83.47, pipelines/driver_ms 65.44, stage/render/render 47.06, stage/main/update 46.25, render/graph/ms 44.44, stage/main/post_update 36.33, stage/render/prepare_assets 31.02 |
| warm-up | 4.52 | 44.2 | pipelines/compile_ms 1354, pipelines/front_ms 1309, render/thread/ms 129, render/busy/ms 127, main/busy/ms 116, pipelines/wait_ms 82.93, stage/main/update 51.45, stage/render/render 49.15, render/graph/ms 46.38, pipelines/driver_ms 45.18, stage/main/post_update 36.07, stage/render/graph_submit 31.90 |
| warm-up | 4.62 | 95.2 | pipelines/compile_ms 1963, pipelines/front_ms 1908, pipelines/wait_ms 347, render/thread/ms 113, render/busy/ms 111, main/busy/ms 84.96, pipelines/driver_ms 55.08, stage/main/post_update 42.13, stage/render/prepare_assets 27.63, encode//sum 24.49, stage/render/render 21.91, stage/render/prepare_write_phase_buffers 21.49 |
| warm-up | 4.67 | 53.8 | pipelines/compile_ms 2242, pipelines/front_ms 2180, pipelines/wait_ms 351, render/thread/ms 132, render/busy/ms 130, encode//sum 98.89, main/busy/ms 86.00, pipelines/driver_ms 62.69, stage/render/render 49.83, stage/main/post_update 46.76, render/graph/ms 46.45, stage/render/graph_submit 32.10 |
| warm-up | 4.74 | 73.6 | pipelines/compile_ms 863, pipelines/front_ms 831, render/thread/ms 122, render/busy/ms 119, main/busy/ms 98.92, encode//sum 93.51, stage/main/update 67.70, stage/render/render 67.62, render/graph/ms 63.97, stage/render/graph_submit 40.50, pipelines/driver_ms 31.28, encode/finish_wall 26.59 |
| warm-up | 4.86 | 111.9 | pipelines/compile_ms 1856, pipelines/front_ms 1799, render/thread/ms 144, render/busy/ms 142, pipelines/wait_ms 123, main/busy/ms 106, stage/main/update 77.05, pipelines/driver_ms 56.82, stage/render/prepare_assets 46.09, stage/render/render 35.89, stage/render/prepare_resources 34.80, render/graph/ms 33.23 |
| warm-up | 4.89 | 38.8 | pipelines/compile_ms 1514, pipelines/front_ms 1472, pipelines/wait_ms 124, main/busy/ms 115, render/thread/ms 107, render/busy/ms 105, stage/main/update 89.51, stream/drive/ms 53.67, stage/render/prepare_assets 47.08, pipelines/driver_ms 41.53, stage/render/prepare_resources 25.43, stage/render/render 14.60 |
| warm-up | 4.99 | 99.6 | pipelines/compile_ms 319, pipelines/front_ms 306, main/busy/ms 113, stage/main/update 86.29, render/thread/ms 62.91, stream/drive/ms 61.96, render/busy/ms 61.05, stage/render/prepare_assets 31.56, stage/main/post_update 16.49, stage/render/render 13.82, pipelines/driver_ms 12.39, render/graph/ms 11.22 |
| warm-up | 5.03 | 35.2 | pipelines/compile_ms 274, pipelines/front_ms 261, render/thread/ms 60.88, render/busy/ms 59.47, main/busy/ms 56.37, stage/main/update 33.96, stage/render/prepare_assets 22.32, stage/main/post_update 14.02, stage/render/render 13.48, pipelines/driver_ms 12.36, stream/drive/ms 12.23, render/graph/ms 11.49 |
| warm-up | 5.07 | 36.0 | pipelines/compile_ms 1128, pipelines/front_ms 1098, render/thread/ms 59.52, render/busy/ms 57.91, main/busy/ms 44.27, pipelines/driver_ms 30.24, stage/main/update 26.89, stage/render/prepare_assets 17.69, stage/render/render 15.53, render/graph/ms 13.36, encode//sum 9.79, stage/render/graph_submit 8.91 |
| warm-up | 5.29 | 31.4 | pipelines/compile_ms 641, pipelines/front_ms 620, main/busy/ms 56.04, render/thread/ms 53.22, render/busy/ms 51.59, stage/main/post_update 24.07, stage/main/update 22.83, pipelines/driver_ms 20.97, stage/render/render 20.13, render/graph/ms 17.87, stage/render/graph_submit 12.46, encode//sum 10.20 |
| warm-up | 5.32 | 30.8 | pipelines/compile_ms 1401, pipelines/front_ms 1359, render/thread/ms 54.93, main/busy/ms 54.41, render/busy/ms 53.21, pipelines/driver_ms 42.53, stage/main/update 22.65, stage/main/post_update 20.25, stage/render/prepare_write_phase_buffers 16.60, stage/render/render 16.58, render/graph/ms 14.41, encode//sum 10.23 |
| warm-up | 5.39 | 27.9 | pipelines/compile_ms 73.54, pipelines/front_ms 68.44, render/thread/ms 41.58, render/busy/ms 40.05, main/busy/ms 38.13, stage/main/update 21.72, stage/render/render 15.73, render/graph/ms 13.54, stream/drive/ms 13.49, stage/main/post_update 10.17, encode//sum 9.88, stage/render/graph_submit 8.59 |
| warm-up | 5.66 | 33.7 | pipelines/compile_ms 607, pipelines/front_ms 587, main/busy/ms 42.02, render/thread/ms 33.35, render/busy/ms 31.69, stage/main/update 26.45, pipelines/driver_ms 19.89, stream/drive/ms 16.16, stage/render/render 13.46, render/graph/ms 11.26, stage/main/post_update 10.25, encode//sum 9.92 |
| warm-up | 5.79 | 26.7 | pipelines/compile_ms 346, pipelines/front_ms 338, render/thread/ms 35.70, render/busy/ms 34.54, main/busy/ms 23.49, stage/render/prepare_assets 14.65, stage/render/render 11.99, stage/main/post_update 10.42, render/graph/ms 10.17, encode//sum 9.78, stage/main/update 8.54, pipelines/driver_ms 7.97 |
| warm-up | 6.03 | 28.5 | render/thread/ms 41.42, render/busy/ms 39.46, main/busy/ms 34.81, stage/render/render 21.07, render/graph/ms 18.63, stage/main/update 16.35, stage/main/post_update 14.12, encode//sum 13.41, stage/render/graph_submit 13.05, encode/submit 9.50, stage/render/prepare_assets 7.82, stage/render/camera_driver 5.37 |
| warm-up | 6.09 | 33.2 | render/thread/ms 41.87, render/busy/ms 40.49, main/busy/ms 26.01, pipelines/compile_ms 25.03, pipelines/front_ms 23.33, stage/render/prepare_assets 21.20, stage/render/render 11.33, encode//sum 10.96, stage/main/update 9.78, render/graph/ms 9.40, stage/main/post_update 8.87, stage/render/graph_submit 6.34 |
| warm-up | 6.19 | 32.6 | render/thread/ms 51.95, render/busy/ms 50.54, main/busy/ms 49.03, stage/main/update 31.39, stage/render/render 16.03, render/graph/ms 13.79, stage/main/post_update 11.12, encode//sum 10.56, stage/render/prepare_assets 9.86, stage/render/graph_submit 8.81, stream/drive/ms 6.65, encode/submit 6.02 |
| warm-up | 6.22 | 30.2 | render/thread/ms 53.23, render/busy/ms 51.88, main/busy/ms 51.45, stage/main/update 33.85, stage/render/prepare_assets 14.00, stage/render/render 13.96, render/graph/ms 11.97, stage/main/post_update 11.06, encode//sum 9.77, stage/render/graph_submit 7.82, stage/render/prepare_meshes 6.57, stream/drive/ms 5.87 |
| warm-up | 6.28 | 26.7 | pipelines/compile_ms 158, pipelines/front_ms 154, render/thread/ms 41.22, render/busy/ms 39.83, main/busy/ms 32.48, stage/main/update 15.28, stage/render/render 14.74, render/graph/ms 12.55, encode//sum 12.30, stage/main/post_update 12.06, stage/render/prepare_assets 8.98, stage/render/graph_submit 8.17 |
| warm-up | 6.33 | 31.5 | main/busy/ms 44.31, render/thread/ms 42.47, render/busy/ms 40.77, stage/main/update 23.20, stage/render/render 18.90, render/graph/ms 16.45, stage/main/post_update 15.05, encode//sum 12.47, stage/render/graph_submit 11.12, encode/submit 6.98, stage/render/prepare_assets 5.36, stage/render/camera_driver 5.14 |
| warm-up | 6.37 | 31.7 | render/thread/ms 50.84, render/busy/ms 49.27, main/busy/ms 44.86, stage/main/update 25.73, stage/render/render 15.93, stage/render/prepare_assets 13.96, render/graph/ms 13.65, stage/main/post_update 13.46, encode//sum 11.22, stage/render/graph_submit 8.94, stage/render/prepare_meshes 5.93, encode/submit 5.30 |
| warm-up | 6.40 | 36.0 | render/thread/ms 49.29, render/busy/ms 48.19, main/busy/ms 39.64, stage/render/prepare_assets 23.31, stage/main/update 21.48, stage/main/post_update 9.84, stage/render/render 9.80, encode//sum 8.81, render/graph/ms 8.18, stage/render/prepare_meshes 5.49, stage/render/graph_submit 5.18, stream/drive/ms 3.93 |
| warm-up | 6.43 | 30.7 | render/thread/ms 54.11, render/busy/ms 52.31, main/busy/ms 39.26, stage/render/render 19.25, stage/main/update 17.53, stage/render/prepare_assets 16.93, render/graph/ms 16.71, stage/main/post_update 12.46, stage/render/graph_submit 12.27, encode//sum 11.87, encode/submit 9.11, stage/render/prepare_meshes 4.34 |
| warm-up | 6.49 | 40.2 | pipelines/compile_ms 166, pipelines/front_ms 162, render/thread/ms 40.58, render/busy/ms 39.30, main/busy/ms 36.17, stage/main/update 20.17, stage/render/prepare_assets 16.11, stage/render/render 12.06, stage/main/post_update 10.36, render/graph/ms 10.02, encode//sum 9.92, stage/render/graph_submit 6.21 |
| warm-up | 6.63 | 34.5 | main/busy/ms 57.15, stage/main/update 41.51, render/thread/ms 35.60, render/busy/ms 34, stream/drive/ms 17.27, stage/render/render 13.76, encode//sum 11.75, render/graph/ms 11.33, stage/main/post_update 10.94, stage/render/graph_submit 7.48, stage/render/prepare_assets 6.68, stage/render/prepare_meshes 5.74 |
| warm-up | 6.67 | 46.1 | main/busy/ms 49.27, render/thread/ms 46.63, render/busy/ms 45.12, stage/main/update 31.24, stage/render/prepare_assets 18.08, stage/render/render 13.43, encode//sum 11.30, render/graph/ms 11.16, stage/main/post_update 10.28, stage/render/graph_submit 7.26, stream/drive/ms 6.81, stage/render/prepare_meshes 5.42 |
| warm-up | 6.71 | 31.5 | pipelines/compile_ms 144, pipelines/front_ms 140, render/thread/ms 47.53, render/busy/ms 46.21, main/busy/ms 31.29, stage/render/prepare_assets 22.34, stage/main/update 13.90, stage/render/render 12.08, render/graph/ms 10.18, encode//sum 10.00, stage/main/post_update 8.38, stage/render/graph_submit 6.83 |
| warm-up | 6.79 | 27.9 | render/thread/ms 39.30, render/busy/ms 37.69, main/busy/ms 37.32, stage/main/update 21.36, stage/render/render 14.84, render/graph/ms 12.87, encode//sum 11.61, stage/main/post_update 10.91, stage/render/graph_submit 8.23, stage/render/prepare_assets 8.03, stage/render/prepare_meshes 6.11, stream/drive/ms 5.48 |
| warm-up | 6.92 | 29.0 | render/thread/ms 36.33, main/busy/ms 35.45, render/busy/ms 34.88, stage/main/update 17.79, stage/render/render 14.78, render/graph/ms 12.81, encode//sum 12.08, stage/main/post_update 11.39, stage/render/graph_submit 8.49, stage/render/prepare_meshes 5.67, encode/submit 5.03, stream/drive/ms 4.85 |
| warm-up | 7.07 | 64.2 | pipelines/compile_ms 189, pipelines/front_ms 146, render/thread/ms 78.24, stage/render/render 68.02, render/present/ms 52.13, pipelines/driver_ms 43.50, render/busy/ms 25.87, main/busy/ms 25.18, render/graph/ms 14.98, encode//sum 10.87, stage/render/graph_submit 10.35, stage/main/post_update 10.22 |
| measure | 18.39 | 28.1 | main/busy/ms 35.17, stage/main/update 22.17, render/thread/ms 19.23, render/busy/ms 18.26, stage/render/render 8.99, stage/main/post_update 8.94, render/graph/ms 7.67, encode//sum 5.94, stage/render/graph_submit 5.50, stage/render/prepare_assets 3.95, encode/submit 3.59, stage/render/camera_driver 2.06 |
| measure | 59.18 | 33.5 | render/thread/ms 37.45, render/busy/ms 35.75, main/busy/ms 23.29, stage/render/render 15.84, render/graph/ms 13.99, stage/main/post_update 12.34, stage/render/prepare_assets 11.52, encode//sum 11.51, stage/render/graph_submit 9.71, encode/submit 6.12, stage/main/update 4.73, stage/render/camera_driver 4.06 |
| measure | 59.21 | 28.4 | render/thread/ms 36.08, render/busy/ms 34.97, stage/render/prepare_assets 18.90, main/busy/ms 17.94, stage/render/render 10.41, encode//sum 9.17, stage/main/post_update 9.08, render/graph/ms 9.02, stage/render/graph_submit 6.44, encode/submit 3.93, stage/main/update 3.26, stage/main/fixed_main_loop 2.62 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.atlas.enabled=off --resolution 2560x1440 --bench-out target/perf\shadow_review\balmora_walk_night@2560x1440_arm=stock-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (255 samples): GPU load **79%** (min 59%) · 2619 MHz (min 2610) · memory 8101 MHz · 80.1 W · 78 °C (max 84) · VRAM 5719 MiB · throttled 100% of samples

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
- `sf.graphics_preset=high (preset: level requested)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=high (preset: level requested)`
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
- `shadow.lamp.atlas.enabled=off (command line: --set)`
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
