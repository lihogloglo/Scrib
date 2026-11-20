# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=high (preset: level requested); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5996 frames over 65.6 s · **median 10.72 ms (93 fps)** · mean 10.95 · p95 13.25 · p99 15.79 (1% low 63 fps) · worst 41.89 ms · 51 spikes over 1.5× median · 43.0 stutters/min (work 34.7/min)
- warm-up: 355 frames over 9.5 s, worst 2566.7 ms
- hitches (> 26.8 ms): **5** measured, 47 in warm-up
- timed GPU passes: 6.92 ms of a 10.95 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.72 ms, limit 11 ms
- **FAIL** frame_ms: 47 stutters (> 1.5x moving median of 60) in 1.09 min = 42.96/min, limit 1/min
- **FAIL** worst frame 41.89 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.070 | 2.161 | 2.436 | 0.96 | 0.177 |
| main_opaque_pass_3d | 1.296 | 1.352 | 1.857 | 0.96 | 0.067 |
| early prepass | 0.508 | 0.530 | 0.751 | 0.96 | 0.091 |
| bloom | 0.327 | 0.341 | 0.665 | 0.96 | 0.066 |
| atmosphere_luts | 0.313 | 0.326 | 0.463 | 0.96 | 0.003 |
| auxiliary_views | 0.256 | 0.268 | 0.984 | 0.96 | 0.227 |
| water_velocity | 0.228 | 0.238 | 0.248 | 0.96 | 0.002 |
| water_sspr | 0.213 | 0.222 | 0.215 | 0.96 | 0.002 |
| ssao | 0.191 | 0.200 | 0.205 | 0.96 | 0.008 |
| atmos_froxel_inject | 0.132 | 0.138 | 0.131 | 0.96 | 0.001 |
| upscaling | 0.130 | 0.135 | 0.164 | 0.96 | 0.004 |
| water_fft | 0.124 | 0.130 | 0.127 | 0.96 | 0.003 |
| uniform_allocation | 0.115 | 0.120 | 0.142 | 0.96 | 0.023 |
| clouds_composite | 0.104 | 0.108 | 0.109 | 0.96 | 0.002 |
| gpu_instance_materialization | 0.092 | 0.096 | 0.100 | 0.96 | 0.004 |
| tonemapping | 0.091 | 0.095 | 0.104 | 0.96 | 0.004 |
| bin_unpacking | 0.090 | 0.094 | 0.114 | 0.96 | 0.017 |
| main_transmissive_pass_3d | 0.074 | 0.077 | 0.154 | 0.96 | 0.009 |
| clustering | 0.064 | 0.067 | 0.068 | 0.96 | 0.020 |
| early_mesh_preprocessing | 0.054 | 0.056 | 0.071 | 0.96 | 0.004 |
| clouds_trace | 0.054 | 0.056 | 0.078 | 0.96 | 0.001 |
| render_sky | 0.053 | 0.055 | 0.054 | 0.96 | 0.002 |
| clouds_resolve | 0.048 | 0.050 | 0.051 | 0.96 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.473 | 0.765 | 0.10 | 0.001 |
| surface_trail | 0.041 | 0.060 | 0.057 | 0.68 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.032 | 0.032 | 0.96 | 0.001 |
| water_snapshot | 0.025 | 0.026 | 0.034 | 0.96 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.96 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.230 | 0.10 | 0.004 |
| range_unpacking | 0.021 | 0.022 | 0.028 | 0.96 | 0.003 |
| per_view_shadow_pass | 0.019 | 0.020 | 0.027 | 0.96 | 0.005 |
| main_transparent_pass_3d | 0.016 | 0.017 | 0.031 | 0.96 | 0.205 |
| night_sky | 0.016 | 0.017 | 0.042 | 0.96 | 0.002 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.96 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.96 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.96 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.96 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.725 | 11.930 | 40.81 | 58313.0 |
| render/busy/ms | 9.205 | 11.439 | 40.32 | 55191.6 |
| main/busy/ms | 8.929 | 10.820 | 31.25 | 53539.4 |
| encode//sum | 6.770 | 11.242 | 56.24 | 40594.2 |
| stage/render/render | 5.960 | 7.650 | 37.22 | 35734.2 |
| stage/main/post_update | 5.294 | 6.710 | 10.79 | 31743.1 |
| render/graph/ms | 5.255 | 6.925 | 36.55 | 31507.0 |
| stage/render/graph_submit | 3.824 | 5.135 | 34.94 | 22927.2 |
| encode/submit | 2.374 | 3.260 | 33.58 | 14232.5 |
| stage/main/update | 1.869 | 2.398 | 24.24 | 11207.6 |
| encode/finish_wall | 1.511 | 2.117 | 13.59 | 9061.1 |
| stage/render/camera_driver | 1.367 | 1.868 | 4.35 | 8197.7 |
| encode/pass:shadow_directional_light/sum | 0.813 | 1.305 | 2.32 | 4873.3 |
| encode/pass:main_opaque_pass_3d/sum | 0.782 | 1.110 | 1.56 | 4687.0 |
| encode/pass:early prepass/sum | 0.765 | 1.087 | 13.09 | 4586.0 |
| stage/render/prepare_write_phase_buffers | 0.660 | 0.981 | 5.27 | 3955.6 |
| bigspace/total/ms | 0.577 | 1.580 | 4.81 | 3461.2 |
| stage/main/fixed_main_loop | 0.561 | 0.992 | 2.30 | 3362.2 |
| stage/render/prepare_assets | 0.537 | 0.679 | 9.98 | 3219.0 |
| stage/render/prepare_resources | 0.451 | 0.556 | 1.14 | 2703.5 |
| stage/render/prepare_meshes | 0.435 | 0.566 | 3.75 | 2606.9 |
| render/present/ms | 0.431 | 0.554 | 19.12 | 2584.8 |
| stage/main/first | 0.398 | 0.518 | 6.84 | 2388.6 |
| stage/main/pre_update | 0.377 | 0.521 | 2.37 | 2258.9 |
| encode/pass:shadow_point_light/sum | 0.369 | 1.774 | 4.48 | 2215.0 |
| controller/feed/ms | 0.330 | 0.432 | 0.93 | 1980.7 |
| stage/render/prepare_bind_groups | 0.313 | 0.514 | 2.45 | 1877.8 |
| encode/pass:main_transparent_pass_3d/sum | 0.312 | 0.488 | 0.98 | 1870.2 |
| stage/render/prepare_flush | 0.290 | 0.495 | 4.91 | 1738.6 |
| stage/render/prepare_batch_phases | 0.282 | 0.431 | 1.09 | 1689.8 |
| bigspace/low_precision/ms | 0.253 | 1.004 | 2.56 | 1517.7 |
| stage/main/last | 0.250 | 0.299 | 0.83 | 1497.0 |
| stream/drive/ms | 0.207 | 0.296 | 11.41 | 1242.3 |
| stage/render/create_views | 0.184 | 0.237 | 0.87 | 1106.0 |
| stage/render/specialize | 0.184 | 0.272 | 2.50 | 1103.8 |
| stage/render/extract_commands | 0.184 | 0.220 | 0.56 | 1102.2 |
| bigspace/high_precision/ms | 0.175 | 1.004 | 3.30 | 1046.9 |
| anim/evaluate/ms | 0.161 | 0.284 | 2.24 | 964.4 |
| anim/evaluate_cpu/ms | 0.153 | 0.196 | 0.58 | 918.5 |
| stream/tick/ms | 0.152 | 0.223 | 0.53 | 909.9 |
| stage/render/prepare_views | 0.132 | 0.176 | 0.47 | 794.0 |
| encode/pass:main_transmissive_pass_3d/sum | 0.125 | 0.228 | 0.77 | 748.5 |
| stage/render/queue | 0.112 | 0.186 | 2.32 | 669.2 |
| stage/render/phase_sort | 0.097 | 0.137 | 0.57 | 579.1 |
| render/acquire/ms | 0.089 | 0.121 | 0.39 | 536.6 |
| controller/move/ms | 0.087 | 0.286 | 0.54 | 524.3 |
| controller/write/ms | 0.074 | 0.100 | 0.43 | 441.7 |
| terrain/update/ms | 0.064 | 0.305 | 2.33 | 386.0 |
| stage/render/prepare_collect_phase_buffers | 0.050 | 0.076 | 0.35 | 297.2 |
| physics/step/ms | 0.048 | 0.115 | 0.54 | 286.4 |
| physics/insert/ms | 0.033 | 0.356 | 1.25 | 196.5 |
| pipelines/compile_ms | 0.030 | 0.000 | 84.17 | 178.0 |
| encode/resolve_diagnostics/sum | 0.029 | 0.040 | 0.35 | 172.9 |
| pipelines/front_ms | 0.029 | 0.000 | 81.23 | 171.9 |
| terrain/select/ms | 0.026 | 0.268 | 0.72 | 157.8 |
| stage/render/cleanup | 0.024 | 0.035 | 2.55 | 142.1 |
| weather/map/ms | 0.017 | 0.023 | 0.25 | 102.0 |
| far/select/ms | 0.016 | 0.168 | 0.92 | 94.9 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.022 | 0.22 | 78.1 |
| anim/apply/ms | 0.011 | 0.021 | 0.20 | 67.6 |
| weather/eval/ms | 0.011 | 0.014 | 0.25 | 64.6 |
| encode/camera_driver_span_end/sum | 0.010 | 0.020 | 0.29 | 61.7 |
| script/update/ms | 0.008 | 0.012 | 0.20 | 49.1 |
| controller/avoid/ms | 0.006 | 0.012 | 0.15 | 38.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.05 | 36.6 |
| script/input/ms | 0.005 | 0.006 | 0.17 | 29.4 |
| stream/spawn/ms | 0.004 | 0.000 | 1.13 | 22.4 |
| stream/upload/ms | 0.003 | 0.000 | 1.16 | 20.1 |
| script/ui/ms | 0.003 | 0.005 | 0.11 | 20.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.73 | 13.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 6.3 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.95 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.7 |
| stage/render/prepare | 0.001 | 0.003 | 0.08 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.4 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.01 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 18 | 34 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
| crowd/c0/count | count | 26 | 43 | 13 |
| crowd/c0/joints/count | count | 1285 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 147 | 156 | 147 |
| crowd/c1/joints/count | count | 5556 | 5942 | 5581 |
| crowd/c1/meshes/count | count | 1187 | 1272 | 1171 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 161 | 216 | 204 |
| crowd/joints_extracted/count | count | 654 | 7070 | 308 |
| crowd/joints_uploaded/count | count | 3617 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.2 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.57 | 13.48 | 0.63 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.22 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.29 | 0.01 |
| encode/finish_critical | ms | 0.98 | 13.48 | 0.92 |
| encode/pass:early prepass/max | ms | 0.77 | 13.09 | 0.58 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.81 | 1.56 | 0.92 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.11 | 0.77 | 0.14 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.30 | 0.98 | 0.36 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.32 | 0.54 |
| encode/pass:shadow_point_light/max | ms | 0.15 | 1.48 | 0.54 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.35 | 0.02 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 9.24 | 40.32 | 8.00 |
| gpu/timed/ms | ms | 7.92 | 9.69 | 8.00 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2597 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.4 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 157 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 338 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 157 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 338 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 972 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2474 | 3553 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 972 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2474 | 3553 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 26 | 139 | 8 |
| phases/shadow/bins | count | 1254 | 1700 | 658 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6580 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2897 | 3556 | 2014 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 154 | 197 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 335 | 336 | 336 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 17 | 435 | 7 |
| preprocess/bind_groups_created | count | 12 | 430 | 2 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 130 | 216 | 127 |
| process/cpu_usage | % | 25.75 | 39.01 | 32.69 |
| process/mem_usage | GiB | 5.39 | 5.57 | 5.56 |
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
| skin/blocks_changed/count | count | 14 | 127 | 8 |
| skin/blocks_skipped/count | count | 147 | 205 | 196 |
| skin/blocks_walked/count | count | 14 | 27 | 8 |
| skin/joint_bytes_written/bytes | bytes | 46.3 k | 811.3 k | 24.8 k |
| skin/joints_written/count | count | 724 | 12.7 k | 388 |
| skin/upload_writes/count | count | 7 | 26 | 8 |
| started_load_count |  | 6440 | 6564 | 6564 |
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
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.17 | 44.84 | 32.73 |
| system/mem_usage | % | 56.13 | 56.70 | 56.60 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2666 | 2724 | 2724 |
| views/all/batch_sets | count | 82 | 198 | 66 |
| views/all/bins | count | 3141 | 3894 | 2948 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 8444 | 10.8 k | 7956 |
| views/all/pipelines | count | 203 | 244 | 66 |
| views/all/pipelines_distinct | count | 70 | 73 | 61 |
| views/all/work_items | count | 154.0 k | 206.9 k | 154.0 k |
| views/cascade/batch_sets | count | 11 | 33 | 8 |
| views/cascade/bins | count | 710 | 860 | 658 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6580 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2114 | 2427 | 2014 |
| views/cascade/pipelines | count | 24 | 27 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 129 | 0 |
| views/lamp_face/bins | count | 552 | 879 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 22 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 794 | 1197 | 0 |
| views/lamp_face/pipelines | count | 130 | 171 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 670 | 0 |
| views/main/batch_sets | count | 58 | 61 | 58 |
| views/main/bins | count | 2016 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5908 | 7526 | 5942 |
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
| world/meshes | count | 21.1 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.04 | 205 | 282.0 k |
| anim/events/count | count | 0.08 | 3 | 493 |
| assets/Image/added | count | 0.10 | 34 | 586 |
| assets/Image/modified | count | 0.00 | 3 | 12 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.48 | 240 | 2874 |
| assets/Mesh/removed | count | 0.81 | 1212 | 4860 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.25 | 10 | 7481 |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 8 |
| ecs/entities_delta | count | 0.25 | 1154 | 1490 |
| encode//count | count | 38.02 | 204 | 228.0 k |
| encode/bind_walk/init_textures | count | 9361 | 23.9 k | 56.13 M |
| encode/bind_walk/init_textures_skipped | count | 52.2 k | 70.4 k | 313.06 M |
| encode/bind_walk/inits | count | 569 | 966 | 3.41 M |
| encode/bind_walk/inits_skipped | count | 138 | 231 | 828.6 k |
| encode/bind_walk/merge_textures | count | 9298 | 23.8 k | 55.75 M |
| encode/bind_walk/merge_textures_skipped | count | 9104 | 19.1 k | 54.59 M |
| encode/bind_walk/merges | count | 104 | 273 | 625.5 k |
| encode/bind_walk/merges_skipped | count | 127 | 224 | 760.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5996 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5996 |
| encode/pass:early prepass/count | count | 1 | 1 | 5996 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5996 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5996 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5996 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5996 |
| encode/pass:shadow_point_light/count | count | 2.51 | 44 | 15.1 k |
| encode/queue/command_buffers | count | 50.55 | 258 | 303.1 k |
| encode/queue/submits | count | 2.01 | 4 | 12.1 k |
| encode/queue/write_buffer/bytes | bytes | 2.33 M | 3.33 M | 13.97 G |
| encode/queue/write_buffer/calls | count | 274 | 665 | 1.64 M |
| encode/queue/write_buffer_with/bytes | bytes | 160.8 k | 1.28 M | 964.12 M |
| encode/queue/write_buffer_with/calls | count | 10.42 | 729 | 62.5 k |
| encode/queue/write_texture/bytes | bytes | 2299 | 1.05 M | 13.78 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 558 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5996 |
| physics/inserts | count | 0.37 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 175 | 2335 |
| physics/steps | count | 0.66 | 3 | 3939 |
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
| stream/deactivate/items | count | 0.04 | 35 | 223 |
| stream/despawn/items | count | 0.04 | 5 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 5646 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1238 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.8 ms; slowest 52, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2566.7 | main/busy/ms 2534, stage/main/post_update 2490, stage/main/update 34.00, stage/main/pre_update 6.38, surface/advance_cpu_ms 1.18, stage/main/last 1.15, stream/drive/ms 0.49, stage/main/first 0.48, stage/main/fixed_main_loop 0.44, surface/cell_upload_ms 0.30, stream/tick/ms 0.11, bigspace/total/ms 0.08 |
| warm-up | 3.37 | 799.5 | main/busy/ms 2690, stage/main/post_update 2522, render/thread/ms 762, render/busy/ms 593, stage/render/prepare_assets 545, stage/render/prepare_views 167, render/acquire/ms 167, stage/main/update 150, stage/render/create_views 29.03, stage/render/render 10.28, stage/main/pre_update 8.13, render/graph/ms 7.04 |
| warm-up | 3.43 | 63.8 | render/thread/ms 809, render/busy/ms 638, stage/render/prepare_assets 577, main/busy/ms 181, stage/render/prepare_views 168, render/acquire/ms 168, stage/main/update 123, pipelines/compile_ms 60.44, pipelines/front_ms 56.14, stage/main/post_update 44.10, stage/render/create_views 29.44, terrain/update/ms 26.31 |
| warm-up | 3.47 | 39.2 | pipelines/wait_ms 238, pipelines/compile_ms 177, pipelines/front_ms 159, render/thread/ms 85.10, render/busy/ms 78.72, main/busy/ms 64.90, stage/main/post_update 44.58, stage/render/render 37.53, stage/render/prepare_assets 32.74, terrain/update/ms 32.50, render/graph/ms 30.71, stage/render/camera_driver 20.15 |
| warm-up | 3.55 | 85.7 | pipelines/wait_ms 1650, pipelines/compile_ms 816, pipelines/front_ms 727, main/busy/ms 106, pipelines/driver_ms 89.27, stage/main/post_update 83.96, render/thread/ms 79.51, render/busy/ms 73.72, stage/render/render 36.66, stage/render/prepare_assets 31.43, render/graph/ms 30.36, bigspace/total/ms 24.64 |
| warm-up | 3.64 | 84.5 | pipelines/wait_ms 6066, pipelines/compile_ms 1621, pipelines/front_ms 1496, main/busy/ms 136, pipelines/driver_ms 124, render/thread/ms 124, render/busy/ms 122, stage/main/post_update 94.06, stage/render/prepare_assets 42.57, stage/render/prepare_resources 33.37, encode//sum 31.52, bigspace/total/ms 24.57 |
| warm-up | 3.67 | 29.7 | pipelines/wait_ms 5013, pipelines/compile_ms 985, pipelines/front_ms 930, main/busy/ms 147, render/thread/ms 110, render/busy/ms 109, stage/main/update 68.94, pipelines/driver_ms 54.67, stage/main/post_update 52.35, stage/render/prepare_resources 33.98, encode//sum 32.47, stage/render/prepare_flush 28.28 |
| warm-up | 3.79 | 118.8 | pipelines/compile_ms 1393, pipelines/front_ms 1333, pipelines/wait_ms 578, main/busy/ms 108, render/thread/ms 93.34, render/busy/ms 91.77, stage/main/update 67.08, stage/render/prepare_assets 65.63, pipelines/driver_ms 59.95, stage/main/post_update 22.97, stage/render/render 13.29, stage/main/pre_update 12.91 |
| warm-up | 3.84 | 33.2 | pipelines/compile_ms 1073, pipelines/front_ms 1025, pipelines/wait_ms 165, main/busy/ms 57.74, pipelines/driver_ms 47.49, render/thread/ms 43.26, render/busy/ms 41.01, stage/main/post_update 28.28, stage/render/prepare_assets 18.39, stage/render/render 15.12, render/graph/ms 12.33, stage/main/pre_update 12.32 |
| warm-up | 3.87 | 33.6 | pipelines/compile_ms 836, pipelines/front_ms 796, pipelines/wait_ms 122, render/thread/ms 48.84, main/busy/ms 47.47, render/busy/ms 46.70, pipelines/driver_ms 40.10, stage/main/post_update 23.32, stage/render/render 15.18, stage/render/prepare_assets 12.47, render/graph/ms 11.99, stage/main/pre_update 11.94 |
| warm-up | 3.93 | 52.0 | pipelines/compile_ms 438, pipelines/front_ms 424, render/thread/ms 75.23, render/busy/ms 72.06, main/busy/ms 69.61, stage/main/update 52.36, stage/render/render 50.18, render/graph/ms 46.17, stage/render/camera_driver 35.61, encode//sum 26.85, pipelines/driver_ms 14.05, stage/main/post_update 11.72 |
| warm-up | 4.02 | 90.5 | pipelines/compile_ms 1395, pipelines/front_ms 1325, render/thread/ms 121, pipelines/wait_ms 119, render/busy/ms 118, main/busy/ms 78.41, pipelines/driver_ms 70.73, stage/main/update 63.02, stage/render/prepare_assets 61.26, stage/render/render 52.10, render/graph/ms 48.72, stage/render/camera_driver 35.49 |
| warm-up | 4.05 | 29.0 | pipelines/compile_ms 1711, pipelines/front_ms 1627, pipelines/wait_ms 135, render/thread/ms 87.42, render/busy/ms 86.06, pipelines/driver_ms 84.30, stage/render/prepare_assets 69.68, main/busy/ms 31.83, stage/main/update 19.64, stage/render/render 11.36, render/graph/ms 9.70, stage/render/graph_submit 7.22 |
| warm-up | 4.10 | 48.8 | pipelines/compile_ms 1237, pipelines/front_ms 1197, render/thread/ms 46.07, render/busy/ms 44.87, pipelines/driver_ms 40.07, stage/render/prepare_assets 31.88, pipelines/wait_ms 22.71, main/busy/ms 18.38, stage/main/update 8.90, stage/render/render 8.79, render/graph/ms 7.24, encode//sum 5.15 |
| warm-up | 4.16 | 34.7 | pipelines/compile_ms 109, pipelines/front_ms 98.97, render/thread/ms 47.50, render/busy/ms 45.67, main/busy/ms 28.48, stage/render/render 21.94, render/graph/ms 19.84, encode//sum 15.52, stage/main/update 15.36, stage/render/graph_submit 12.90, pipelines/driver_ms 10.34, stage/main/post_update 9.60 |
| warm-up | 4.18 | 27.6 | render/thread/ms 56.13, render/busy/ms 54.36, pipelines/compile_ms 44.46, pipelines/front_ms 40.70, main/busy/ms 40.25, stage/render/render 20.66, render/graph/ms 18.53, stage/main/post_update 17.41, stage/main/update 15.50, stage/render/graph_submit 12.34, stage/render/prepare_resources 8.44, encode//sum 8.32 |
| warm-up | 4.25 | 61.7 | pipelines/compile_ms 437, pipelines/front_ms 421, render/thread/ms 68.31, render/busy/ms 66.08, main/busy/ms 50.07, stage/render/prepare_assets 34.16, stage/main/update 23.21, stage/render/render 20.28, render/graph/ms 17.65, encode//sum 17.02, pipelines/driver_ms 16.73, stage/main/post_update 15.72 |
| warm-up | 4.30 | 57.6 | pipelines/compile_ms 1764, pipelines/front_ms 1712, main/busy/ms 134, stage/main/update 116, render/thread/ms 78.38, render/busy/ms 76.39, stage/render/prepare_assets 55.67, pipelines/driver_ms 51.91, encode//sum 15.52, stage/render/render 14.99, render/graph/ms 12.70, stage/main/post_update 7.90 |
| warm-up | 4.48 | 177.6 | pipelines/compile_ms 2576, pipelines/front_ms 2475, main/busy/ms 149, stage/main/update 135, render/thread/ms 113, render/busy/ms 106, pipelines/driver_ms 100, stage/render/prepare_assets 60.57, stage/render/render 24.03, stage/render/prepare_bind_groups 21.10, render/graph/ms 17.44, pipelines/wait_ms 8.99 |
| warm-up | 4.54 | 58.8 | pipelines/compile_ms 1326, pipelines/front_ms 1258, render/thread/ms 122, render/busy/ms 115, pipelines/driver_ms 67.50, main/busy/ms 60.03, stage/render/prepare_assets 59.00, stage/main/update 43.61, pipelines/wait_ms 30.52, stage/render/render 26.96, stage/render/prepare_bind_groups 22.69, render/graph/ms 20.04 |
| warm-up | 4.63 | 92.9 | render/thread/ms 126, render/busy/ms 125, pipelines/compile_ms 122, pipelines/front_ms 113, main/busy/ms 78.06, stage/main/post_update 41.98, pipelines/wait_ms 37.54, stage/render/prepare_assets 31.40, stage/render/prepare_flush 31.10, stage/main/update 25.97, stage/render/prepare_bind_groups 18.37, stage/render/render 17.62 |
| warm-up | 4.69 | 61.9 | pipelines/compile_ms 1927, pipelines/front_ms 1866, render/thread/ms 138, render/busy/ms 137, pipelines/wait_ms 123, main/busy/ms 86.28, pipelines/driver_ms 60.30, stage/main/post_update 40.64, stage/render/prepare_flush 31.48, stage/render/prepare_write_phase_buffers 27.86, encode//sum 23.31, stage/render/render 22.70 |
| warm-up | 4.80 | 106.0 | pipelines/compile_ms 3204, pipelines/front_ms 3104, pipelines/wait_ms 1452, render/thread/ms 150, render/busy/ms 149, pipelines/driver_ms 99.51, main/busy/ms 78.03, stage/render/prepare_resources 46.19, encode//sum 40.05, stage/render/prepare_write_phase_buffers 28.96, stage/render/prepare_assets 28.41, stage/render/render 27.43 |
| warm-up | 4.90 | 97.9 | pipelines/compile_ms 2070, pipelines/front_ms 1996, pipelines/wait_ms 1527, render/thread/ms 189, render/busy/ms 187, main/busy/ms 129, stage/main/update 76.09, pipelines/driver_ms 74.08, stage/render/prepare_resources 60.41, encode//sum 49.05, stage/render/render 42.41, stage/render/prepare_assets 39.41 |
| warm-up | 5.02 | 117.6 | pipelines/compile_ms 2626, pipelines/front_ms 2558, pipelines/wait_ms 801, render/thread/ms 169, render/busy/ms 167, main/busy/ms 101, stage/render/prepare_assets 87.75, pipelines/driver_ms 67.65, stage/main/update 65.74, stage/render/render 32.94, render/graph/ms 30.05, encode//sum 25.49 |
| warm-up | 5.06 | 41.4 | pipelines/compile_ms 2114, pipelines/front_ms 2064, pipelines/wait_ms 619, render/thread/ms 108, render/busy/ms 106, stage/render/prepare_assets 85.54, main/busy/ms 55.47, pipelines/driver_ms 49.86, stage/main/update 19.61, stage/main/post_update 14.00, stage/render/render 12.81, stage/main/pre_update 12.00 |
| warm-up | 5.11 | 54.1 | pipelines/compile_ms 344, pipelines/front_ms 328, main/busy/ms 79.24, render/thread/ms 79.00, render/busy/ms 76.96, stage/main/update 35.77, stage/render/render 25.06, stage/render/prepare_assets 23.49, render/graph/ms 22.43, stage/main/post_update 22.00, stream/drive/ms 15.94, pipelines/driver_ms 15.60 |
| warm-up | 5.14 | 30.7 | pipelines/compile_ms 145, pipelines/front_ms 138, main/busy/ms 114, stage/main/update 86.45, render/thread/ms 73.61, render/busy/ms 71.80, stream/drive/ms 47.73, stage/render/render 26.52, render/graph/ms 24.11, stage/main/post_update 18.49, encode//sum 14.40, stage/render/graph_submit 12.89 |
| warm-up | 5.24 | 101.4 | pipelines/compile_ms 687, pipelines/front_ms 669, main/busy/ms 114, stage/main/update 97.14, render/thread/ms 69.44, render/busy/ms 67.80, stream/drive/ms 60.93, stage/render/prepare_assets 27.25, stage/render/render 22.70, render/graph/ms 20.33, pipelines/driver_ms 17.34, encode//sum 12.56 |
| warm-up | 5.28 | 35.0 | pipelines/compile_ms 1729, pipelines/front_ms 1687, render/thread/ms 71.15, render/busy/ms 69.72, main/busy/ms 67.68, stage/main/update 49.27, pipelines/driver_ms 41.94, stage/render/prepare_assets 31.46, stream/drive/ms 29.87, stage/render/render 19.83, render/graph/ms 17.73, encode//sum 10.43 |
| warm-up | 5.31 | 28.0 | pipelines/compile_ms 1514, pipelines/front_ms 1479, render/thread/ms 50.23, render/busy/ms 48.83, main/busy/ms 46.34, pipelines/driver_ms 35.35, stage/main/update 19.05, stage/main/post_update 18.16, stage/render/prepare_assets 15.79, stage/render/render 14.92, render/graph/ms 12.94, encode//sum 9.88 |
| warm-up | 5.34 | 30.6 | pipelines/compile_ms 900, pipelines/front_ms 878, render/thread/ms 51.07, render/busy/ms 49.66, main/busy/ms 45.73, pipelines/driver_ms 22.78, stage/main/post_update 19.42, stage/main/update 18.52, stage/render/render 16.55, render/graph/ms 14.63, encode//sum 11.26, stage/render/graph_submit 9.95 |
| warm-up | 5.38 | 38.3 | pipelines/compile_ms 482, pipelines/front_ms 467, render/thread/ms 64.20, render/busy/ms 62.70, main/busy/ms 47.12, stage/main/update 20.79, stage/main/post_update 19.39, stage/render/render 16.50, pipelines/driver_ms 14.52, render/graph/ms 14.36, stage/render/prepare_write_phase_buffers 14.11, encode//sum 12.06 |
| warm-up | 5.43 | 30.2 | pipelines/compile_ms 224, pipelines/front_ms 215, render/thread/ms 40.78, render/busy/ms 39.65, main/busy/ms 35.32, stage/main/update 14.60, stage/render/prepare_assets 14.01, stage/render/render 13.75, stage/main/post_update 12.22, render/graph/ms 12.06, encode//sum 10.72, pipelines/driver_ms 9.35 |
| warm-up | 5.60 | 29.2 | pipelines/compile_ms 191, pipelines/front_ms 185, main/busy/ms 37.76, render/thread/ms 35.83, render/busy/ms 34.50, stage/main/update 20.71, stage/render/render 14.92, stream/drive/ms 13.30, render/graph/ms 12.88, encode//sum 11.69, stage/main/post_update 9.37, stage/render/prepare_assets 8.63 |
| warm-up | 5.65 | 30.3 | render/thread/ms 44.09, render/busy/ms 42.69, main/busy/ms 32.90, stage/render/render 19.41, stage/main/update 17.59, render/graph/ms 17.29, stage/render/graph_submit 12.13, stage/render/prepare_assets 11.68, encode//sum 10.70, stage/main/post_update 8.95, encode/submit 6.42, encode/finish_wall 5.75 |
| warm-up | 5.87 | 31.4 | pipelines/compile_ms 749, pipelines/front_ms 730, main/busy/ms 42.81, render/thread/ms 32.67, render/busy/ms 31.31, stage/main/update 25.35, pipelines/driver_ms 19.16, stream/drive/ms 17.62, stage/render/render 15.53, render/graph/ms 13.43, stage/main/post_update 12.07, encode//sum 10.69 |
| warm-up | 6.01 | 27.8 | pipelines/compile_ms 192, pipelines/front_ms 188, render/thread/ms 37.66, render/busy/ms 36.53, main/busy/ms 25.54, stage/render/render 14.85, render/graph/ms 12.91, stage/render/prepare_assets 11.27, stage/main/post_update 11.08, encode//sum 11.04, stage/main/update 9.27, stage/render/graph_submit 8.96 |
| warm-up | 6.17 | 27.1 | pipelines/compile_ms 164, pipelines/front_ms 159, main/busy/ms 31.55, render/thread/ms 26.11, render/busy/ms 25.16, stage/main/update 21.37, stream/drive/ms 13.57, stage/render/render 10.25, encode//sum 9.20, render/graph/ms 8.73, stage/main/post_update 6.82, stage/render/prepare_assets 6.68 |
| warm-up | 6.29 | 29.8 | render/thread/ms 42.20, render/busy/ms 40.72, main/busy/ms 30.39, stage/render/render 17.69, render/graph/ms 15.56, stage/main/update 13.38, stage/main/post_update 12.71, encode//sum 11.75, stage/render/graph_submit 10.94, stage/render/prepare_assets 9.65, encode/submit 8.03, stage/render/camera_driver 4.47 |
| warm-up | 6.34 | 29.4 | render/thread/ms 35.66, render/busy/ms 34.77, main/busy/ms 24.78, stage/render/prepare_assets 16.76, stage/main/update 11.24, stage/render/render 9.02, encode//sum 8.40, stage/main/post_update 8.14, render/graph/ms 7.60, stage/render/graph_submit 4.49, stream/drive/ms 3.13, stage/render/camera_driver 2.94 |
| warm-up | 6.55 | 29.1 | render/thread/ms 44.44, render/busy/ms 42.86, main/busy/ms 41.19, stage/main/update 20.97, stage/render/render 18.92, render/graph/ms 16.81, stage/main/post_update 14.03, stage/render/graph_submit 12.10, encode//sum 11.47, stage/render/prepare_assets 9.50, pipelines/compile_ms 8.82, encode/submit 8.56 |
| warm-up | 6.61 | 33.8 | render/thread/ms 46.60, render/busy/ms 45.60, main/busy/ms 39.28, stage/main/update 20.22, stage/render/prepare_assets 17.82, stage/main/post_update 11.61, stage/render/render 10.67, encode//sum 10.06, render/graph/ms 9.17, stage/render/prepare_meshes 5.85, stage/render/graph_submit 5.16, stage/render/camera_driver 3.83 |
| warm-up | 6.65 | 36.8 | render/thread/ms 59.26, render/busy/ms 57.70, main/busy/ms 45.62, stage/main/update 22.84, stage/render/prepare_assets 21.02, stage/render/render 15.97, stage/main/post_update 14.87, render/graph/ms 13.15, encode//sum 11.03, stage/render/graph_submit 7.83, stage/render/prepare_meshes 7.79, stage/render/camera_driver 5.05 |
| warm-up | 6.70 | 30.5 | pipelines/compile_ms 158, pipelines/front_ms 155, render/thread/ms 38.99, render/busy/ms 38.03, main/busy/ms 33.33, stage/main/update 16.52, stage/render/prepare_assets 14.77, stage/main/post_update 10.75, stage/render/render 10.21, encode//sum 9.27, render/graph/ms 8.65, stage/render/graph_submit 5.28 |
| warm-up | 6.86 | 51.7 | main/busy/ms 59.85, render/thread/ms 43.90, render/busy/ms 42.45, stage/main/update 39.95, stage/render/render 16.04, stage/main/post_update 15.21, stream/drive/ms 14.25, render/graph/ms 13.74, stage/render/prepare_assets 12.09, encode//sum 11.37, stage/render/graph_submit 9.59, encode/submit 6.22 |
| warm-up | 6.89 | 30.2 | pipelines/compile_ms 146, pipelines/front_ms 143, render/thread/ms 49.30, render/busy/ms 47.93, main/busy/ms 45.59, stage/main/update 25.90, stage/render/prepare_assets 18.12, stage/render/render 15.03, stage/main/post_update 13.60, render/graph/ms 12.77, encode//sum 11.84, stage/render/graph_submit 8.40 |
| measure | 3.11 | 41.9 | render/thread/ms 52.06, render/busy/ms 51.08, stage/render/render 44.82, render/graph/ms 43.45, stage/render/graph_submit 39.92, encode/submit 36.45, main/busy/ms 17.37, encode//sum 10.50, stage/main/post_update 10.21, stage/main/update 3.59, encode/finish_wall 3.50, stage/render/camera_driver 3.40 |
| measure | 18.38 | 34.7 | main/busy/ms 41.49, stage/main/update 26.25, render/thread/ms 23.78, render/busy/ms 22.80, stage/render/render 12.86, render/graph/ms 11.44, stage/main/post_update 11.22, encode//sum 10.87, stage/render/graph_submit 8.13, encode/submit 5.46, stage/render/prepare_assets 3.93, stage/render/camera_driver 3.17 |
| measure | 32.65 | 27.5 | render/thread/ms 37.21, stage/render/render 29.02, main/busy/ms 22.91, render/busy/ms 22.15, encode//sum 16.61, render/present/ms 14.84, stage/main/post_update 13.64, render/graph/ms 13.54, stage/render/graph_submit 10.18, encode/submit 6.41, stage/main/update 5.02, encode/finish_wall 3.79 |
| measure | 47.63 | 30.4 | render/thread/ms 39.42, stage/render/render 31.13, render/present/ms 19.71, main/busy/ms 19.60, render/busy/ms 19.53, encode//sum 13.06, render/graph/ms 10.87, stage/main/post_update 10.44, stage/render/graph_submit 7.92, encode/submit 4.96, stage/main/update 4.43, encode/finish_wall 3.20 |
| measure | 59.09 | 28.2 | render/thread/ms 37.50, render/busy/ms 36.53, main/busy/ms 27.60, stage/render/prepare_assets 15.98, encode//sum 14.57, stage/main/post_update 13.96, stage/render/render 11.29, render/graph/ms 9.74, stage/render/graph_submit 6.19, stage/main/update 5.11, stage/main/fixed_main_loop 4.50, stage/render/camera_driver 3.20 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\bench-shadow-a3.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\shadow_a3_atlas\balmora_walk_night@2560x1440_arm=base-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 18%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1512 pmon samples)
- focus: clean
- nvidia-smi over the window (254 samples): GPU load **80%** (min 55%) · 2625 MHz (min 2625) · memory 8101 MHz · 77.3 W · 74 °C (max 79) · VRAM 6520 MiB · throttled 100% of samples

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
