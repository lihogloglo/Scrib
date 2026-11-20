# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6259 frames over 65.7 s · **median 10.22 ms (98 fps)** · mean 10.49 · p95 12.83 · p99 14.72 (1% low 68 fps) · worst 29.14 ms · 37 spikes over 1.5× median · 29.2 stutters/min (work 23.8/min)
- warm-up: 353 frames over 9.3 s, worst 2523.2 ms
- hitches (> 25.5 ms): **2** measured, 44 in warm-up
- timed GPU passes: 6.73 ms of a 10.49 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.22 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.25/min, limit 1/min
- pass worst frame 29.14 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.993 | 2.037 | 2.072 | 0.98 | 0.173 |
| main_opaque_pass_3d | 1.251 | 1.278 | 1.553 | 0.98 | 0.068 |
| early prepass | 0.505 | 0.516 | 0.731 | 0.98 | 0.091 |
| bloom | 0.311 | 0.317 | 0.324 | 0.98 | 0.063 |
| atmosphere_luts | 0.302 | 0.309 | 0.374 | 0.98 | 0.003 |
| auxiliary_views | 0.281 | 0.287 | 1.051 | 0.98 | 0.249 |
| water_velocity | 0.221 | 0.226 | 0.246 | 0.98 | 0.002 |
| water_sspr | 0.197 | 0.202 | 0.203 | 0.98 | 0.002 |
| ssao | 0.184 | 0.188 | 0.191 | 0.98 | 0.007 |
| upscaling | 0.126 | 0.129 | 0.142 | 0.98 | 0.004 |
| atmos_froxel_inject | 0.125 | 0.128 | 0.129 | 0.98 | 0.001 |
| water_fft | 0.122 | 0.124 | 0.128 | 0.98 | 0.003 |
| uniform_allocation | 0.115 | 0.118 | 0.142 | 0.98 | 0.022 |
| clouds_composite | 0.102 | 0.105 | 0.108 | 0.98 | 0.002 |
| gpu_instance_materialization | 0.093 | 0.095 | 0.099 | 0.98 | 0.004 |
| bin_unpacking | 0.092 | 0.093 | 0.115 | 0.98 | 0.017 |
| tonemapping | 0.086 | 0.088 | 0.092 | 0.98 | 0.004 |
| main_transmissive_pass_3d | 0.070 | 0.071 | 0.137 | 0.98 | 0.008 |
| clustering | 0.062 | 0.064 | 0.067 | 0.98 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.078 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.053 | 0.054 | 0.070 | 0.98 | 0.004 |
| render_sky | 0.051 | 0.053 | 0.053 | 0.98 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.413 | 0.416 | 0.11 | 0.001 |
| surface_trail | 0.038 | 0.057 | 0.057 | 0.68 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 0.98 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.98 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.219 | 0.229 | 0.11 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.98 | 0.002 |
| range_unpacking | 0.021 | 0.022 | 0.027 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.027 | 0.98 | 0.005 |
| night_sky | 0.016 | 0.017 | 0.040 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.031 | 0.98 | 0.193 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.003 | 0.004 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.157 | 11.442 | 22.27 | 57310.9 |
| render/busy/ms | 8.670 | 10.943 | 21.53 | 54263.4 |
| main/busy/ms | 8.443 | 9.949 | 27.02 | 52843.9 |
| stage/render/render | 5.613 | 7.439 | 16.01 | 35133.2 |
| render/graph/ms | 4.950 | 6.750 | 14.74 | 30982.3 |
| stage/main/post_update | 4.903 | 5.979 | 9.63 | 30685.7 |
| encode//sum | 4.146 | 5.934 | 26.75 | 25948.5 |
| stage/render/graph_submit | 3.550 | 4.781 | 8.85 | 22217.6 |
| encode/submit | 2.318 | 3.302 | 5.05 | 14508.0 |
| stage/main/update | 1.814 | 2.289 | 20.37 | 11355.6 |
| stage/render/camera_driver | 1.347 | 1.956 | 7.03 | 8428.9 |
| encode/finish_wall | 1.276 | 1.651 | 4.04 | 7987.1 |
| encode/pass:shadow_directional_light/sum | 0.794 | 1.223 | 2.75 | 4972.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.791 | 1.071 | 1.81 | 4948.4 |
| encode/pass:early prepass/sum | 0.779 | 1.052 | 3.50 | 4875.5 |
| stage/render/prepare_write_phase_buffers | 0.681 | 1.004 | 1.89 | 4263.4 |
| stage/main/fixed_main_loop | 0.528 | 0.923 | 2.16 | 3302.8 |
| stage/render/prepare_assets | 0.526 | 0.663 | 10.55 | 3292.3 |
| bigspace/total/ms | 0.525 | 1.268 | 2.16 | 3283.0 |
| encode/pass:shadow_point_light/sum | 0.449 | 2.300 | 5.03 | 2807.9 |
| stage/render/prepare_resources | 0.435 | 0.518 | 4.01 | 2725.2 |
| stage/render/prepare_meshes | 0.430 | 0.538 | 3.52 | 2694.1 |
| render/present/ms | 0.401 | 0.481 | 2.20 | 2511.1 |
| stage/main/first | 0.394 | 0.514 | 4.94 | 2463.5 |
| stage/main/pre_update | 0.377 | 0.529 | 3.34 | 2358.6 |
| controller/feed/ms | 0.316 | 0.399 | 1.13 | 1977.1 |
| stage/render/prepare_bind_groups | 0.304 | 0.496 | 1.93 | 1903.6 |
| stage/render/prepare_flush | 0.290 | 0.501 | 1.32 | 1815.6 |
| stage/render/prepare_batch_phases | 0.273 | 0.413 | 0.80 | 1707.3 |
| encode/pass:main_transparent_pass_3d/sum | 0.271 | 0.431 | 0.81 | 1698.6 |
| stage/main/last | 0.252 | 0.298 | 0.80 | 1574.4 |
| bigspace/low_precision/ms | 0.222 | 0.697 | 1.75 | 1391.7 |
| stream/drive/ms | 0.200 | 0.283 | 11.22 | 1253.2 |
| stage/render/create_views | 0.191 | 0.245 | 1.06 | 1197.8 |
| stage/render/extract_commands | 0.165 | 0.195 | 0.51 | 1033.1 |
| stage/render/specialize | 0.161 | 0.242 | 2.62 | 1008.3 |
| anim/evaluate/ms | 0.159 | 0.267 | 1.47 | 994.2 |
| stage/render/queue | 0.154 | 0.222 | 2.19 | 964.0 |
| anim/evaluate_cpu/ms | 0.152 | 0.192 | 0.48 | 953.1 |
| bigspace/high_precision/ms | 0.146 | 0.590 | 1.50 | 916.1 |
| pipelines/compile_ms | 0.146 | 0.000 | 135.66 | 915.4 |
| stream/tick/ms | 0.146 | 0.212 | 0.46 | 913.2 |
| pipelines/front_ms | 0.142 | 0.000 | 132.17 | 891.5 |
| stage/render/prepare_views | 0.130 | 0.166 | 0.37 | 812.8 |
| encode/pass:main_transmissive_pass_3d/sum | 0.093 | 0.149 | 0.42 | 579.6 |
| render/acquire/ms | 0.086 | 0.111 | 0.28 | 536.4 |
| controller/move/ms | 0.076 | 0.197 | 0.46 | 475.4 |
| controller/write/ms | 0.073 | 0.101 | 0.28 | 459.3 |
| terrain/update/ms | 0.060 | 0.291 | 1.59 | 372.7 |
| stage/render/prepare_collect_phase_buffers | 0.046 | 0.072 | 0.25 | 288.6 |
| physics/step/ms | 0.041 | 0.085 | 0.58 | 254.2 |
| stage/render/phase_sort | 0.039 | 0.065 | 1.07 | 247.2 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.58 | 188.5 |
| physics/insert/ms | 0.027 | 0.322 | 0.90 | 171.1 |
| stage/render/cleanup | 0.023 | 0.033 | 0.89 | 143.4 |
| terrain/select/ms | 0.023 | 0.257 | 0.80 | 143.1 |
| weather/map/ms | 0.017 | 0.024 | 0.18 | 108.7 |
| far/select/ms | 0.015 | 0.167 | 0.38 | 96.3 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.27 | 82.3 |
| anim/apply/ms | 0.012 | 0.022 | 0.19 | 75.0 |
| encode/camera_driver_span_end/sum | 0.010 | 0.019 | 0.17 | 65.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.07 | 64.8 |
| script/update/ms | 0.008 | 0.011 | 0.21 | 47.6 |
| controller/avoid/ms | 0.006 | 0.012 | 0.03 | 37.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.28 | 35.6 |
| script/input/ms | 0.005 | 0.006 | 0.14 | 30.4 |
| script/ui/ms | 0.004 | 0.006 | 0.08 | 24.7 |
| pipelines/driver_ms | 0.004 | 0.000 | 3.49 | 23.9 |
| stream/spawn/ms | 0.004 | 0.000 | 1.13 | 22.2 |
| stream/upload/ms | 0.003 | 0.000 | 1.19 | 20.3 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.64 | 13.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 6.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.16 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.5 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 34 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 146 | 154 | 148 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5613 |
| crowd/c1/meshes/count | count | 1167 | 1254 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 162 | 216 | 204 |
| crowd/joints_extracted/count | count | 689 | 5576 | 564 |
| crowd/joints_uploaded/count | count | 3632 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.2 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.41 | 3.78 | 0.51 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.27 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.17 | 0.02 |
| encode/finish_critical | ms | 0.94 | 3.78 | 0.99 |
| encode/pass:early prepass/max | ms | 0.78 | 3.50 | 0.80 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.81 | 0.99 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.42 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.26 | 0.81 | 0.35 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.75 | 0.69 |
| encode/pass:shadow_point_light/max | ms | 0.11 | 1.34 | 0.64 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.58 | 0.05 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.76 | 27.02 | 8.44 |
| gpu/timed/ms | ms | 7.54 | 9.15 | 7.62 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 158 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 347 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 158 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 347 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 763 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2475 | 3524 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 763 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2475 | 3524 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 27 | 142 | 8 |
| phases/shadow/bins | count | 1251 | 1761 | 658 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2884 | 3648 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 158 | 190 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3182 | 2347 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 182 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 333 | 334 | 334 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 1 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 17 | 442 | 11 |
| preprocess/bind_groups_created | count | 12 | 437 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 130 | 274 | 123 |
| process/cpu_usage | % | 25.96 | 41.82 | 31.25 |
| process/mem_usage | GiB | 5.39 | 5.56 | 5.56 |
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
| skin/blocks_changed/count | count | 14 | 95 | 11 |
| skin/blocks_skipped/count | count | 147 | 211 | 193 |
| skin/blocks_walked/count | count | 14 | 30 | 11 |
| skin/joint_bytes_written/bytes | bytes | 48.4 k | 808.6 k | 40.2 k |
| skin/joints_written/count | count | 756 | 12.6 k | 628 |
| skin/upload_writes/count | count | 8 | 22 | 9 |
| started_load_count |  | 6441 | 6564 | 6564 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 236 | 333 | 180 |
| statics/materials_bound | count | 236 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.07 | 0.06 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.25 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.98 | 43.32 | 35.05 |
| system/mem_usage | % | 56.41 | 57.12 | 57.10 |
| terrain/patches | count | 511 | 520 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 83 | 201 | 66 |
| views/all/bins | count | 3124 | 3808 | 2948 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 8459 | 11.0 k | 7956 |
| views/all/pipelines | count | 206 | 237 | 66 |
| views/all/pipelines_distinct | count | 70 | 73 | 61 |
| views/all/work_items | count | 154.1 k | 206.8 k | 154.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 8 |
| views/cascade/bins | count | 722 | 852 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2124 | 2428 | 2014 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 132 | 0 |
| views/lamp_face/bins | count | 553 | 922 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 23 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 794 | 1241 | 0 |
| views/lamp_face/pipelines | count | 133 | 162 | 0 |
| views/lamp_face/revealed | count | 0 | 5 | 0 |
| views/lamp_face/work_items | count | 0 | 710 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2022 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5898 | 7486 | 5942 |
| views/main/pipelines | count | 47 | 49 | 46 |
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
| world/meshes | count | 21.1 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 48.36 | 225 | 302.7 k |
| anim/events/count | count | 0.09 | 3 | 569 |
| assets/Image/added | count | 0.09 | 35 | 586 |
| assets/Image/modified | count | 0.00 | 1 | 9 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.46 | 260 | 2874 |
| assets/Mesh/removed | count | 0.78 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.20 | 10 | 7481 |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 7 |
| ecs/entities_delta | count | 0.24 | 1168 | 1489 |
| encode//count | count | 39.64 | 204 | 248.1 k |
| encode/bind_walk/init_textures | count | 9684 | 23.9 k | 60.61 M |
| encode/bind_walk/init_textures_skipped | count | 45.5 k | 62.6 k | 284.66 M |
| encode/bind_walk/inits | count | 576 | 974 | 3.60 M |
| encode/bind_walk/inits_skipped | count | 133 | 228 | 829.6 k |
| encode/bind_walk/merge_textures | count | 9621 | 23.8 k | 60.22 M |
| encode/bind_walk/merge_textures_skipped | count | 10.6 k | 19.9 k | 66.62 M |
| encode/bind_walk/merges | count | 107 | 276 | 668.0 k |
| encode/bind_walk/merges_skipped | count | 126 | 225 | 786.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6259 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6259 |
| encode/pass:early prepass/count | count | 1 | 1 | 6259 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6259 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6259 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6259 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6259 |
| encode/pass:shadow_point_light/count | count | 2.93 | 44 | 18.3 k |
| encode/queue/command_buffers | count | 52.58 | 258 | 329.1 k |
| encode/queue/submits | count | 2.01 | 4 | 12.6 k |
| encode/queue/write_buffer/bytes | bytes | 2.32 M | 3.56 M | 14.50 G |
| encode/queue/write_buffer/calls | count | 279 | 674 | 1.75 M |
| encode/queue/write_buffer_with/bytes | bytes | 160.7 k | 1.28 M | 1.01 G |
| encode/queue/write_buffer_with/calls | count | 10.36 | 789 | 64.9 k |
| encode/queue/write_texture/bytes | bytes | 2035 | 589.8 k | 12.74 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6259 |
| physics/inserts | count | 0.36 | 97 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.37 | 175 | 2335 |
| physics/steps | count | 0.63 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 11 |
| pipelines/created | count | 0.00 | 1 | 12 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 3 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 34 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5409 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1186 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.5 ms; slowest 46, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2523.2 | main/busy/ms 2492, stage/main/post_update 2450, stage/main/update 32.21, stage/main/pre_update 6.43, stage/main/last 1.39, surface/advance_cpu_ms 0.72, stage/main/fixed_main_loop 0.46, stage/main/first 0.44, surface/cell_upload_ms 0.31, stream/drive/ms 0.31, bigspace/total/ms 0.08, stream/tick/ms 0.07 |
| warm-up | 3.36 | 839.3 | main/busy/ms 2638, stage/main/post_update 2477, render/thread/ms 804, render/busy/ms 660, stage/render/prepare_assets 615, stage/main/update 145, stage/render/prepare_views 141, render/acquire/ms 141, stage/render/create_views 27.68, stage/render/render 11.77, stage/main/pre_update 7.92, render/graph/ms 7.44 |
| warm-up | 3.44 | 80.6 | render/thread/ms 869, render/busy/ms 723, stage/render/prepare_assets 665, main/busy/ms 178, stage/render/prepare_views 141, render/acquire/ms 141, stage/main/update 120, stage/main/post_update 45.05, pipelines/compile_ms 40.47, pipelines/front_ms 36.11, stage/render/create_views 28.00, terrain/update/ms 20.07 |
| warm-up | 3.48 | 41.6 | pipelines/compile_ms 227, pipelines/wait_ms 218, pipelines/front_ms 206, render/thread/ms 103, render/busy/ms 94.63, main/busy/ms 83.09, stage/main/post_update 60.94, stage/render/prepare_assets 50.41, stage/render/render 37.69, terrain/update/ms 30.30, render/graph/ms 29.08, pipelines/lock_ms 21.94 |
| warm-up | 3.63 | 144.1 | pipelines/wait_ms 4784, pipelines/compile_ms 1403, pipelines/front_ms 1273, render/thread/ms 160, render/busy/ms 152, pipelines/driver_ms 130, main/busy/ms 84.58, stage/main/post_update 66.43, stage/render/prepare_resources 60.91, stage/render/render 45.91, render/graph/ms 37.94, stage/render/prepare_assets 32.36 |
| warm-up | 3.68 | 53.1 | pipelines/wait_ms 4808, pipelines/compile_ms 1463, pipelines/front_ms 1338, render/thread/ms 173, render/busy/ms 171, pipelines/driver_ms 125, main/busy/ms 99.64, stage/render/prepare_resources 82.82, stage/main/update 58.08, stage/render/prepare_assets 45.42, stage/main/post_update 36.54, encode//sum 28.53 |
| warm-up | 3.78 | 97.0 | pipelines/compile_ms 1377, pipelines/front_ms 1305, pipelines/wait_ms 386, render/thread/ms 128, render/busy/ms 126, main/busy/ms 82.94, stage/render/prepare_assets 78.68, pipelines/driver_ms 72.78, stage/main/update 57.02, stage/render/prepare_resources 25.32, stage/main/post_update 20.01, stage/render/render 14.27 |
| warm-up | 3.83 | 28.6 | pipelines/compile_ms 770, pipelines/front_ms 738, pipelines/wait_ms 119, render/thread/ms 47.98, render/busy/ms 46.49, main/busy/ms 45.29, pipelines/driver_ms 31.69, stage/render/prepare_assets 26.63, stage/main/post_update 17.19, stage/main/update 12.82, stage/render/render 12.15, render/graph/ms 10.36 |
| warm-up | 3.87 | 32.9 | pipelines/compile_ms 1371, pipelines/front_ms 1325, render/thread/ms 55.62, render/busy/ms 53.90, pipelines/driver_ms 46.52, main/busy/ms 41.52, pipelines/wait_ms 25.09, encode//sum 19.28, stage/render/prepare_assets 18.90, stage/main/post_update 18.08, stage/render/prepare_resources 16.66, stage/main/update 14.72 |
| warm-up | 3.90 | 29.8 | pipelines/compile_ms 935, pipelines/front_ms 907, main/busy/ms 88.88, stage/main/update 59.80, render/thread/ms 59.22, render/busy/ms 51.49, pipelines/driver_ms 27.51, pipelines/wait_ms 24.50, stage/main/post_update 22.75, stage/render/render 21.96, encode//sum 19.96, stage/render/prepare_resources 16.63 |
| warm-up | 4.02 | 125.2 | pipelines/compile_ms 1272, pipelines/front_ms 1218, main/busy/ms 79.87, render/thread/ms 79.24, render/busy/ms 71.84, stage/main/update 59.14, pipelines/driver_ms 54.75, stage/render/prepare_assets 46.56, encode//sum 20.27, stage/render/render 18.80, pipelines/wait_ms 15.19, stage/main/post_update 13.46 |
| warm-up | 4.11 | 86.4 | pipelines/compile_ms 1925, pipelines/front_ms 1852, render/thread/ms 123, render/busy/ms 121, stage/render/prepare_assets 105, pipelines/driver_ms 72.65, main/busy/ms 21.53, encode//sum 18.99, pipelines/wait_ms 16.29, stage/render/render 12.20, render/graph/ms 10.53, stage/main/update 9.05 |
| warm-up | 4.18 | 40.1 | pipelines/compile_ms 222, pipelines/front_ms 212, main/busy/ms 37.71, render/thread/ms 32.80, render/busy/ms 31.16, stage/main/update 21.59, stage/main/post_update 12.22, stage/render/render 11.90, stream/drive/ms 11.32, render/graph/ms 9.99, pipelines/driver_ms 9.70, stage/render/prepare_assets 6.82 |
| warm-up | 4.22 | 35.0 | pipelines/compile_ms 391, pipelines/front_ms 375, render/thread/ms 55.77, render/busy/ms 53.94, main/busy/ms 35.69, stage/render/prepare_assets 20.63, stage/render/render 17.14, pipelines/driver_ms 15.85, render/graph/ms 14.52, stage/main/update 13.91, stage/main/post_update 13.88, stage/render/graph_submit 9.94 |
| warm-up | 4.28 | 66.3 | pipelines/compile_ms 228, pipelines/front_ms 219, main/busy/ms 108, stage/main/update 89.59, render/thread/ms 76.43, render/busy/ms 74.63, stage/render/prepare_assets 51.42, stage/render/render 15.94, render/graph/ms 13.19, stage/render/graph_submit 9.89, pipelines/driver_ms 9.17, encode//sum 8.38 |
| warm-up | 4.42 | 135.3 | pipelines/compile_ms 1733, pipelines/front_ms 1695, render/thread/ms 111, render/busy/ms 109, main/busy/ms 103, stage/main/update 89.33, stage/render/prepare_assets 69.22, pipelines/driver_ms 38.05, stage/render/prepare_flush 24.12, stage/render/render 11.98, render/graph/ms 9.72, stage/render/graph_submit 7.47 |
| warm-up | 4.45 | 31.0 | pipelines/compile_ms 1788, pipelines/front_ms 1745, render/thread/ms 92.65, render/busy/ms 90.86, pipelines/driver_ms 43.18, stage/render/prepare_assets 42.05, main/busy/ms 40.89, stage/render/prepare_flush 24.12, stage/main/update 16.91, pipelines/wait_ms 16.37, stage/render/render 15.13, stage/main/post_update 14.99 |
| warm-up | 4.53 | 80.1 | pipelines/compile_ms 205, pipelines/front_ms 192, main/busy/ms 109, render/thread/ms 103, render/busy/ms 98.87, stage/main/update 77.02, stage/render/render 38.72, render/graph/ms 33.67, pipelines/wait_ms 32.11, stage/render/prepare_write_phase_buffers 17.74, stage/main/post_update 17.02, stage/render/prepare_flush 16.67 |
| warm-up | 4.62 | 96.3 | pipelines/compile_ms 1499, pipelines/front_ms 1453, render/thread/ms 131, render/busy/ms 127, main/busy/ms 112, stage/main/update 71.83, pipelines/driver_ms 46.33, stage/render/render 38.39, stage/render/prepare_assets 35.90, render/graph/ms 32.98, stage/main/post_update 27.51, pipelines/wait_ms 25.35 |
| warm-up | 4.70 | 80.0 | pipelines/compile_ms 2455, pipelines/front_ms 2373, render/thread/ms 122, render/busy/ms 120, main/busy/ms 97.91, pipelines/driver_ms 82.44, pipelines/wait_ms 66.06, stage/main/update 54.06, stage/render/render 51.06, render/graph/ms 48.36, encode//sum 45.76, stage/render/prepare_assets 44.36 |
| warm-up | 4.81 | 106.8 | pipelines/compile_ms 1308, pipelines/front_ms 1262, render/thread/ms 148, render/busy/ms 146, main/busy/ms 127, stage/main/update 92.43, stage/render/render 65.36, pipelines/wait_ms 64.82, encode//sum 63.55, render/graph/ms 61.63, stage/render/prepare_assets 47.89, pipelines/driver_ms 46.30 |
| warm-up | 4.89 | 83.0 | pipelines/compile_ms 2220, pipelines/front_ms 2162, render/thread/ms 133, render/busy/ms 130, main/busy/ms 88.58, pipelines/wait_ms 70.12, stage/render/prepare_assets 60.30, pipelines/driver_ms 58.03, stage/main/update 57.69, encode//sum 43.50, stage/render/render 38.66, render/graph/ms 34.96 |
| warm-up | 4.95 | 57.0 | pipelines/compile_ms 1965, pipelines/front_ms 1913, render/thread/ms 93.19, render/busy/ms 91.44, pipelines/wait_ms 65.12, pipelines/driver_ms 52.30, main/busy/ms 48.80, stage/render/prepare_assets 47.49, encode//sum 31.45, stage/render/render 24.65, stage/main/update 24.48, render/graph/ms 22.26 |
| warm-up | 4.99 | 34.3 | pipelines/compile_ms 455, pipelines/front_ms 442, main/busy/ms 102, render/thread/ms 74.20, render/busy/ms 72.68, stage/main/update 67.12, stream/drive/ms 39.74, pipelines/wait_ms 26.93, stage/render/prepare_assets 26.77, stage/render/render 21.57, render/graph/ms 19.58, encode//sum 18.53 |
| warm-up | 5.07 | 87.4 | pipelines/compile_ms 1335, pipelines/front_ms 1295, main/busy/ms 118, stage/main/update 84.58, stream/drive/ms 59.67, render/thread/ms 58.54, render/busy/ms 57.08, pipelines/driver_ms 40.43, pipelines/wait_ms 27.33, stage/render/prepare_assets 19.26, stage/render/render 18.56, render/graph/ms 16.51 |
| warm-up | 5.12 | 47.2 | pipelines/compile_ms 925, pipelines/front_ms 895, main/busy/ms 73.44, render/thread/ms 62.43, render/busy/ms 61.01, stage/main/update 54.20, pipelines/driver_ms 29.89, stream/drive/ms 25.56, stage/render/render 24.95, render/graph/ms 22.63, stage/render/prepare_assets 15.95, stage/main/post_update 11.74 |
| warm-up | 5.16 | 39.8 | pipelines/compile_ms 848, pipelines/front_ms 821, render/thread/ms 62.39, render/busy/ms 61.05, main/busy/ms 55.94, stage/main/update 35.51, pipelines/driver_ms 27.46, pipelines/wait_ms 24.81, stage/render/render 23.90, render/graph/ms 21.75, stage/render/prepare_assets 16.39, stage/main/post_update 11.55 |
| warm-up | 5.21 | 25.7 | pipelines/compile_ms 219, pipelines/front_ms 212, render/thread/ms 44.93, render/busy/ms 43.30, main/busy/ms 37.20, stage/main/update 17.89, stage/render/render 16.36, render/graph/ms 14.59, stage/main/post_update 12.40, encode//sum 11.06, stage/render/graph_submit 9.67, encode/submit 6.84 |
| warm-up | 5.25 | 26.1 | pipelines/compile_ms 375, pipelines/front_ms 364, render/thread/ms 35.02, render/busy/ms 33.69, main/busy/ms 26.17, stage/render/render 14.33, render/graph/ms 12.31, stage/main/update 12.10, stage/render/prepare_assets 11.79, pipelines/driver_ms 11.04, encode//sum 10.20, stage/main/post_update 8.34 |
| warm-up | 5.34 | 45.5 | pipelines/compile_ms 600, pipelines/front_ms 582, render/thread/ms 57.82, render/busy/ms 56.14, main/busy/ms 43.01, stage/render/prepare_flush 25.89, stage/main/post_update 18.46, pipelines/driver_ms 17.77, stage/render/render 17.02, render/graph/ms 14.77, stage/main/update 14.25, stage/render/graph_submit 10.88 |
| warm-up | 5.42 | 50.8 | pipelines/compile_ms 176, pipelines/front_ms 171, render/thread/ms 67.18, render/busy/ms 66.02, main/busy/ms 55.19, stage/main/update 33.39, stream/drive/ms 23.84, stage/render/prepare_assets 19.07, stage/render/prepare_resources 17.01, stage/render/render 13.49, stage/main/post_update 12.78, render/graph/ms 11.65 |
| warm-up | 5.46 | 39.7 | pipelines/compile_ms 201, pipelines/front_ms 195, render/thread/ms 81.18, render/busy/ms 80.01, main/busy/ms 58.15, stage/main/update 33.53, stage/render/prepare_resources 26.23, stream/drive/ms 23.06, stage/render/prepare_assets 21.69, stage/render/render 15.05, stage/main/post_update 14.64, render/graph/ms 13.07 |
| warm-up | 5.49 | 30.6 | pipelines/compile_ms 1584, pipelines/front_ms 1541, render/thread/ms 60.24, render/busy/ms 58.73, pipelines/driver_ms 42.92, main/busy/ms 36.42, stage/render/prepare_resources 21.03, stage/render/render 18.52, render/graph/ms 16.30, stage/main/post_update 13.13, stage/main/update 12.77, stage/render/graph_submit 11.68 |
| warm-up | 5.95 | 27.6 | pipelines/compile_ms 150, pipelines/front_ms 147, main/busy/ms 31.44, pipelines/wait_ms 29.33, stage/main/update 21.77, render/thread/ms 21.72, render/busy/ms 20.76, stream/drive/ms 14.20, stage/render/render 8.89, encode//sum 8.41, render/graph/ms 7.61, stage/main/post_update 6.87 |
| warm-up | 6.07 | 30.4 | render/thread/ms 42.02, render/busy/ms 40.38, main/busy/ms 30.30, stage/render/render 17.53, render/graph/ms 15.03, stage/main/update 13.43, stage/main/post_update 13.12, stage/render/prepare_assets 12.79, encode//sum 12.60, stage/render/graph_submit 10.09, encode/submit 6.90, stage/render/camera_driver 4.73 |
| warm-up | 6.12 | 31.5 | render/thread/ms 33.73, render/busy/ms 32.74, main/busy/ms 23.98, stage/render/prepare_assets 16.06, stage/main/update 11.39, stage/render/render 9.04, encode//sum 8.11, render/graph/ms 7.63, stage/main/post_update 7.59, stage/render/graph_submit 4.79, stream/drive/ms 3.56, stage/render/camera_driver 2.70 |
| warm-up | 6.31 | 29.9 | render/thread/ms 44.08, render/busy/ms 42.57, main/busy/ms 42.35, stage/main/update 23.30, stage/render/render 20.02, render/graph/ms 17.59, stage/main/post_update 13.58, stage/render/graph_submit 12.80, encode//sum 12.40, stage/render/prepare_assets 9.50, encode/submit 8.75, stage/render/camera_driver 4.63 |
| warm-up | 6.34 | 27.3 | render/thread/ms 46.10, render/busy/ms 44.60, main/busy/ms 43.90, stage/main/update 23.89, stage/render/render 19.17, render/graph/ms 16.81, stage/main/post_update 13.82, stage/render/graph_submit 12.13, encode//sum 11.96, stage/render/prepare_assets 10.74, encode/submit 8.49, stage/render/prepare_meshes 4.80 |
| warm-up | 6.37 | 28.8 | render/thread/ms 42.71, render/busy/ms 41.77, main/busy/ms 35.05, stage/main/update 18.93, stage/render/prepare_assets 18.24, stage/main/post_update 9.53, stage/render/render 8.87, encode//sum 8.62, render/graph/ms 7.46, stage/render/prepare_meshes 6.39, stage/render/graph_submit 4.56, stream/drive/ms 4.02 |
| warm-up | 6.42 | 32.0 | pipelines/compile_ms 153, pipelines/front_ms 150, render/thread/ms 48.65, render/busy/ms 47.20, main/busy/ms 34.37, stage/render/render 18.35, render/graph/ms 16.13, stage/main/update 15.11, encode//sum 13.11, stage/main/post_update 12.78, stage/render/graph_submit 11.04, stage/render/prepare_assets 8.30 |
| warm-up | 6.48 | 34.0 | render/thread/ms 38.67, render/busy/ms 37.77, main/busy/ms 31.23, stage/render/prepare_assets 17.47, stage/main/update 16.83, stage/render/render 9.69, stage/main/post_update 8.74, render/graph/ms 8.31, encode//sum 8.29, stage/render/graph_submit 5.28, stream/drive/ms 4.42, stage/render/camera_driver 2.90 |
| warm-up | 6.59 | 33.7 | pipelines/compile_ms 145, pipelines/front_ms 141, main/busy/ms 38.60, render/thread/ms 30.04, render/busy/ms 28.95, stage/main/update 24.99, stream/drive/ms 13.54, stage/render/render 12.27, encode//sum 11.23, render/graph/ms 10.63, stage/main/post_update 10.27, stage/render/graph_submit 6.83 |
| warm-up | 6.66 | 47.2 | main/busy/ms 48.85, render/thread/ms 39.90, render/busy/ms 38.78, stage/main/update 32.58, stage/render/prepare_assets 14.72, stage/render/render 14.29, encode//sum 12.77, render/graph/ms 12.70, stage/main/post_update 9.72, stage/render/graph_submit 9.33, stream/drive/ms 8.07, encode/submit 6.33 |
| warm-up | 6.69 | 29.0 | render/thread/ms 42.39, render/busy/ms 41.38, main/busy/ms 31.16, stage/render/prepare_assets 22.61, stage/main/update 14.71, encode//sum 9.96, stage/render/render 9.04, stage/main/post_update 8.75, render/graph/ms 7.59, stage/render/graph_submit 4.85, stage/render/prepare_meshes 4.45, stream/drive/ms 4.25 |
| measure | 18.30 | 28.5 | main/busy/ms 36.86, stage/main/update 22.20, render/thread/ms 20.85, render/busy/ms 19.87, stage/render/render 11.06, render/graph/ms 9.76, stage/main/post_update 9.74, encode//sum 7.38, stage/render/graph_submit 7.02, encode/submit 4.89, stage/render/prepare_assets 3.84, stage/render/camera_driver 2.61 |
| measure | 59.18 | 29.1 | render/thread/ms 36.80, render/busy/ms 35.77, main/busy/ms 27.08, stage/render/prepare_assets 15.68, stage/main/post_update 13.77, stage/render/render 11.08, render/graph/ms 9.67, encode//sum 8.70, stage/render/graph_submit 6.54, stage/main/update 4.99, stage/main/fixed_main_loop 4.19, encode/submit 4.05 |

## Run

- `..\target\profiling\bench-shadow-a1.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_a1_batched\balmora_walk_night@2560x1440_arm=base-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1386 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **78%** (min 61%) · 2625 MHz (min 2625) · memory 8101 MHz · 77.6 W · 74 °C (max 79) · VRAM 6211 MiB · throttled 100% of samples

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
