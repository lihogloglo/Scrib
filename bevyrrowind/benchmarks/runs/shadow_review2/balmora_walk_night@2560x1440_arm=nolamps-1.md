# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=high (preset: level requested); sg.shadows=custom (preset: derived from its members); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=0 (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 7085 frames over 65.6 s · **median 9.14 ms (109 fps)** · mean 9.27 · p95 11.05 · p99 12.27 (1% low 81 fps) · worst 33.43 ms · 34 spikes over 1.5× median · 32.9 stutters/min (work 25.6/min)
- warm-up: 352 frames over 8.7 s, worst 2349.9 ms
- hitches (> 25.0 ms): **2** measured, 42 in warm-up
- timed GPU passes: 6.02 ms of a 9.27 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.14 ms, limit 11 ms
- **FAIL** frame_ms: 36 stutters (> 1.5x moving median of 60) in 1.09 min = 32.90/min, limit 1/min
- **FAIL** worst frame 33.43 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.800 | 2.033 | 2.064 | 0.89 | 0.173 |
| main_opaque_pass_3d | 1.361 | 1.537 | 2.380 | 0.89 | 0.062 |
| early prepass | 0.456 | 0.515 | 0.753 | 0.89 | 0.091 |
| bloom | 0.281 | 0.317 | 0.324 | 0.89 | 0.062 |
| atmosphere_luts | 0.277 | 0.313 | 0.378 | 0.89 | 0.003 |
| water_velocity | 0.199 | 0.225 | 0.247 | 0.89 | 0.002 |
| water_sspr | 0.179 | 0.202 | 0.204 | 0.89 | 0.002 |
| ssao | 0.167 | 0.189 | 0.191 | 0.89 | 0.008 |
| upscaling | 0.114 | 0.128 | 0.135 | 0.89 | 0.004 |
| atmos_froxel_inject | 0.113 | 0.128 | 0.130 | 0.89 | 0.001 |
| water_fft | 0.109 | 0.123 | 0.126 | 0.89 | 0.003 |
| uniform_allocation | 0.101 | 0.114 | 0.139 | 0.89 | 0.019 |
| clouds_composite | 0.093 | 0.105 | 0.109 | 0.89 | 0.002 |
| gpu_instance_materialization | 0.080 | 0.091 | 0.095 | 0.89 | 0.004 |
| bin_unpacking | 0.080 | 0.090 | 0.110 | 0.89 | 0.015 |
| tonemapping | 0.077 | 0.087 | 0.092 | 0.89 | 0.004 |
| main_transmissive_pass_3d | 0.064 | 0.072 | 0.133 | 0.89 | 0.008 |
| clustering | 0.057 | 0.064 | 0.068 | 0.89 | 0.021 |
| clouds_trace | 0.049 | 0.055 | 0.078 | 0.89 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.054 | 0.070 | 0.89 | 0.003 |
| render_sky | 0.047 | 0.053 | 0.054 | 0.89 | 0.002 |
| clouds_resolve | 0.044 | 0.050 | 0.051 | 0.89 | 0.001 |
| surface_trail | 0.031 | 0.056 | 0.057 | 0.54 | 0.002 |
| atmos_froxel_integrate | 0.026 | 0.029 | 0.031 | 0.89 | 0.001 |
| lightprobe_irradiance_map | 0.025 | 0.412 | 0.417 | 0.06 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.027 | 0.89 | 0.002 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.89 | 0.002 |
| range_unpacking | 0.019 | 0.021 | 0.026 | 0.89 | 0.003 |
| night_sky | 0.015 | 0.017 | 0.044 | 0.89 | 0.002 |
| per_view_shadow_pass | 0.014 | 0.016 | 0.026 | 0.89 | 0.005 |
| lightprobe_radiance_map | 0.014 | 0.221 | 0.231 | 0.06 | 0.004 |
| main_transparent_pass_3d | 0.013 | 0.015 | 0.030 | 0.89 | 0.185 |
| atmosphere_environment | 0.009 | 0.011 | 0.011 | 0.89 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.89 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.88 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.89 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.89 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.014 | 0.06 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.06 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.06 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.051 | 9.602 | 19.70 | 57041.0 |
| main/busy/ms | 7.650 | 8.787 | 30.24 | 54202.2 |
| render/busy/ms | 7.512 | 8.936 | 19.25 | 53222.8 |
| stage/render/render | 4.689 | 5.792 | 11.19 | 33218.1 |
| stage/main/post_update | 4.264 | 5.027 | 10.20 | 30208.6 |
| render/graph/ms | 4.048 | 4.976 | 9.60 | 28681.5 |
| encode//sum | 3.076 | 3.909 | 7.82 | 21791.3 |
| stage/render/graph_submit | 2.907 | 3.739 | 7.06 | 20594.3 |
| encode/submit | 1.835 | 2.689 | 4.54 | 13001.8 |
| stage/main/update | 1.776 | 2.214 | 24.62 | 12580.9 |
| encode/finish_wall | 1.155 | 1.512 | 2.65 | 8181.3 |
| stage/render/camera_driver | 1.082 | 1.341 | 2.85 | 7665.0 |
| encode/pass:early prepass/sum | 0.808 | 1.052 | 2.26 | 5723.2 |
| encode/pass:shadow_directional_light/sum | 0.803 | 1.206 | 2.08 | 5687.9 |
| encode/pass:main_opaque_pass_3d/sum | 0.795 | 1.053 | 1.77 | 5634.7 |
| bigspace/total/ms | 0.569 | 1.294 | 2.86 | 4031.9 |
| stage/render/prepare_assets | 0.512 | 0.650 | 12.34 | 3626.7 |
| stage/render/prepare_write_phase_buffers | 0.506 | 0.700 | 1.45 | 3581.5 |
| stage/main/fixed_main_loop | 0.497 | 1.009 | 3.19 | 3518.4 |
| render/present/ms | 0.426 | 1.143 | 1.90 | 3016.9 |
| stage/render/prepare_meshes | 0.419 | 0.533 | 3.74 | 2967.3 |
| stage/render/prepare_resources | 0.394 | 0.468 | 0.84 | 2793.0 |
| stage/main/pre_update | 0.379 | 0.501 | 2.17 | 2682.1 |
| controller/feed/ms | 0.320 | 0.405 | 0.77 | 2265.6 |
| stage/main/first | 0.313 | 0.389 | 6.64 | 2220.8 |
| stage/render/prepare_flush | 0.281 | 0.493 | 0.94 | 1993.5 |
| stage/render/prepare_bind_groups | 0.267 | 0.399 | 1.32 | 1893.6 |
| encode/pass:main_transparent_pass_3d/sum | 0.258 | 0.429 | 1.04 | 1831.4 |
| stage/render/prepare_batch_phases | 0.258 | 0.407 | 0.68 | 1829.2 |
| stage/main/last | 0.248 | 0.292 | 0.79 | 1754.9 |
| bigspace/low_precision/ms | 0.233 | 0.673 | 1.69 | 1652.9 |
| stream/drive/ms | 0.196 | 0.267 | 10.54 | 1387.2 |
| anim/evaluate/ms | 0.187 | 0.529 | 1.56 | 1327.9 |
| bigspace/high_precision/ms | 0.179 | 0.792 | 1.98 | 1264.8 |
| stage/render/extract_commands | 0.178 | 0.211 | 0.59 | 1263.8 |
| stage/render/create_views | 0.164 | 0.205 | 1.32 | 1162.6 |
| stage/render/prepare_views | 0.153 | 0.216 | 2.33 | 1086.5 |
| anim/evaluate_cpu/ms | 0.152 | 0.193 | 0.73 | 1079.1 |
| stream/tick/ms | 0.146 | 0.207 | 0.63 | 1031.4 |
| stage/render/specialize | 0.121 | 0.225 | 1.77 | 854.9 |
| render/acquire/ms | 0.113 | 0.152 | 2.29 | 801.3 |
| encode/pass:main_transmissive_pass_3d/sum | 0.086 | 0.135 | 0.67 | 607.7 |
| stage/render/queue | 0.083 | 0.150 | 1.57 | 585.0 |
| stage/render/phase_sort | 0.076 | 0.107 | 0.36 | 540.2 |
| controller/write/ms | 0.075 | 0.105 | 0.53 | 534.1 |
| controller/move/ms | 0.072 | 0.222 | 0.53 | 510.7 |
| terrain/update/ms | 0.058 | 0.288 | 1.56 | 408.2 |
| physics/step/ms | 0.049 | 0.149 | 0.69 | 345.4 |
| stage/render/prepare_collect_phase_buffers | 0.045 | 0.071 | 0.31 | 317.9 |
| encode/resolve_diagnostics/sum | 0.030 | 0.042 | 0.44 | 212.0 |
| stage/render/cleanup | 0.024 | 0.039 | 0.37 | 169.1 |
| physics/insert/ms | 0.022 | 0.230 | 0.84 | 156.0 |
| pipelines/compile_ms | 0.022 | 0.000 | 77.45 | 153.0 |
| pipelines/front_ms | 0.021 | 0.000 | 75.01 | 147.8 |
| terrain/select/ms | 0.020 | 0.253 | 0.72 | 144.1 |
| weather/map/ms | 0.018 | 0.025 | 0.22 | 124.4 |
| far/select/ms | 0.014 | 0.159 | 0.78 | 98.2 |
| encode/camera_driver_span_begin/sum | 0.013 | 0.021 | 0.48 | 91.1 |
| anim/apply/ms | 0.013 | 0.022 | 0.34 | 88.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.28 | 75.7 |
| encode/camera_driver_span_end/sum | 0.010 | 0.020 | 0.26 | 72.2 |
| script/update/ms | 0.008 | 0.012 | 0.20 | 56.0 |
| controller/avoid/ms | 0.006 | 0.012 | 0.04 | 40.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.21 | 35.5 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 34.9 |
| script/ui/ms | 0.003 | 0.005 | 0.26 | 23.7 |
| stream/spawn/ms | 0.003 | 0.000 | 1.12 | 21.6 |
| stream/upload/ms | 0.003 | 0.000 | 0.97 | 18.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.05 | 7.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 6.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.4 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.74 | 5.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.09 | 3.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 559 | 579 | 578 |
| anim/characters/count | count | 561 | 579 | 578 |
| anim/evaluated/count | count | 19 | 34 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 43 | 13 |
| crowd/c0/joints/count | count | 1306 | 2262 | 583 |
| crowd/c0/meshes/count | count | 355 | 646 | 162 |
| crowd/c1/count | count | 145 | 154 | 148 |
| crowd/c1/joints/count | count | 5475 | 5882 | 5613 |
| crowd/c1/meshes/count | count | 1166 | 1254 | 1179 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6362 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1814 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 159 | 216 | 205 |
| crowd/joints_extracted/count | count | 701 | 5583 | 557 |
| crowd/joints_uploaded/count | count | 3573 | 6220 | 5752 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 49.4 k | 57.5 k | 47.7 k |
| encode//max | ms | 0.38 | 1.60 | 0.50 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.48 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.26 | 0.01 |
| encode/finish_critical | ms | 0.92 | 2.26 | 0.91 |
| encode/pass:early prepass/max | ms | 0.81 | 2.26 | 0.87 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.80 | 1.77 | 0.90 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.08 | 0.67 | 0.06 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 1.04 | 0.47 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.08 | 0.91 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.44 | 0.03 |
| far/entities | count | 3224 | 3224 | 3224 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 60.4 k | 60.4 k | 60.4 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 204.15 M | 204.15 M | 204.15 M |
| far/triangles | count | 5.05 M | 5.05 M | 5.05 M |
| frame/work/ms | ms | 8.02 | 30.24 | 8.54 |
| gpu/timed/ms | ms | 7.27 | 9.28 | 8.54 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2595 | 2654 | 2654 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 302.0 k | 302.0 k | 302.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.5 k | 13.7 k | 12.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 332.38 M | 332.38 M | 332.38 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 146 | 387 | 379 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 314 | 587 | 515 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 146 | 387 | 379 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 663 | 713 | 713 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 314 | 587 | 515 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 23 | 23 |
| phases/opaque3d/bins | count | 765 | 973 | 766 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2516 | 3527 | 2456 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 765 | 973 | 766 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 6055 | 6055 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2516 | 3527 | 2456 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 34 | 8 |
| phases/shadow/bins | count | 727 | 842 | 671 |
| phases/shadow/instance_batches | count | 4 | 8 | 4 |
| phases/shadow/instance_draws | count | 6557 | 13.5 k | 6768 |
| phases/shadow/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| phases/shadow/items | count | 2141 | 2436 | 2008 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 24 | 28 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 1 | 1 |
| physics/bodies | count | 2725 | 3182 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 332 | 333 | 333 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 172 | 11 |
| preprocess/bind_groups_created | count | 8 | 166 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 3 | 0 |
| preprocess/bind_groups_ranges | count | 5 | 6 | 5 |
| preprocess/bind_groups_reused | count | 125 | 143 | 123 |
| process/cpu_usage | % | 26.33 | 40.38 | 28.57 |
| process/mem_usage | GiB | 4.95 | 5.28 | 5.28 |
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
| shadow_atlas/alloc_failures | count | 0 | 0 | 0 |
| shadow_atlas/captures | count | 0 | 0 | 0 |
| shadow_atlas/empty_faces | count | 0 | 0 | 0 |
| shadow_atlas/lamps | count | 0 | 0 | 0 |
| shadow_atlas/live_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/live_tiles | count | 0 | 0 | 0 |
| shadow_atlas/mover_draws | count | 0 | 0 | 0 |
| shadow_atlas/movers | count | 0 | 0 | 0 |
| shadow_atlas/moving_lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/blits | count | 0 | 0 | 0 |
| shadow_atlas/render/clears | count | 0 | 0 | 0 |
| shadow_atlas/render/lamps | count | 0 | 0 | 0 |
| shadow_atlas/render/live_direct | count | 0 | 0 | 0 |
| shadow_atlas/render/live_indirect | count | 0 | 0 | 0 |
| shadow_atlas/render/static_draws | count | 0 | 0 | 0 |
| shadow_atlas/render/views | count | 0 | 0 | 0 |
| shadow_atlas/resizes | count | 0 | 0 | 0 |
| shadow_atlas/static_occ_pct | % | 0 | 0 | 0 |
| shadow_atlas/static_tiles | count | 0 | 0 | 0 |
| shadow_atlas/views | count | 0 | 0 | 0 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| skin/blocks_changed/count | count | 14 | 96 | 11 |
| skin/blocks_skipped/count | count | 144 | 204 | 194 |
| skin/blocks_walked/count | count | 14 | 28 | 11 |
| skin/joint_bytes_written/bytes | bytes | 49.4 k | 807.6 k | 41.2 k |
| skin/joints_written/count | count | 772 | 12.6 k | 644 |
| skin/upload_writes/count | count | 7 | 22 | 10 |
| started_load_count |  | 6440 | 6564 | 6564 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 237 | 333 | 180 |
| statics/materials_bound | count | 237 | 333 | 180 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 4 | 0 |
| stream/latency/s | s | 0.03 | 0.06 | 0.05 |
| stream/model_queue | count | 0 | 46 | 0 |
| stream/models | count | 286 | 380 | 202 |
| stream/oldest_request/s | s | 0 | 0.23 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.29 | 56.46 | 34.70 |
| system/mem_usage | % | 46.61 | 47.10 | 46.96 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2885 | 3001 | 3001 |
| textures/images | count | 2664 | 2724 | 2724 |
| views/all/batch_sets | count | 69 | 93 | 66 |
| views/all/bins | count | 2755 | 3051 | 2961 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 12 | 16 | 12 |
| views/all/instance_draws | count | 19.7 k | 27.1 k | 20.3 k |
| views/all/instance_slots | count | 147.5 k | 196.6 k | 147.5 k |
| views/all/items | count | 7975 | 9687 | 7950 |
| views/all/pipelines | count | 70 | 75 | 66 |
| views/all/pipelines_distinct | count | 61 | 64 | 61 |
| views/all/work_items | count | 154.0 k | 206.5 k | 154.0 k |
| views/cascade/batch_sets | count | 13 | 34 | 8 |
| views/cascade/bins | count | 727 | 842 | 671 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 4 | 8 | 4 |
| views/cascade/instance_draws | count | 6557 | 13.5 k | 6768 |
| views/cascade/instance_slots | count | 49.2 k | 98.3 k | 49.2 k |
| views/cascade/items | count | 2141 | 2436 | 2008 |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 49.4 k | 100.7 k | 49.2 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 57 | 61 | 58 |
| views/main/bins | count | 2020 | 2326 | 2290 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.8 k | 13.5 k | 13.5 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5912 | 7492 | 5942 |
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
| world/meshes | count | 21.8 k | 27.0 k | 19.1 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 55.95 | 260 | 396.4 k |
| anim/events/count | count | 0.08 | 3 | 598 |
| assets/Image/added | count | 0.08 | 40 | 586 |
| assets/Image/modified | count | 0.00 | 9 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.41 | 266 | 2874 |
| assets/Mesh/removed | count | 0.69 | 1226 | 4860 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 464 |
| assets/StandardMaterial/modified | count | 1.06 | 10 | 7477 |
| assets/StandardMaterial/removed | count | 0.06 | 32 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0.00 | 1 | 9 |
| ecs/entities_delta | count | 0.21 | 1260 | 1490 |
| encode//count | count | 27.87 | 30 | 197.5 k |
| encode/bind_walk/init_textures | count | 8468 | 11.2 k | 60.00 M |
| encode/bind_walk/init_textures_skipped | count | 52.0 k | 68.1 k | 368.74 M |
| encode/bind_walk/inits | count | 521 | 782 | 3.69 M |
| encode/bind_walk/inits_skipped | count | 132 | 234 | 936.7 k |
| encode/bind_walk/merge_textures | count | 8405 | 11.0 k | 59.55 M |
| encode/bind_walk/merge_textures_skipped | count | 9912 | 17.2 k | 70.22 M |
| encode/bind_walk/merges | count | 91.15 | 103 | 645.8 k |
| encode/bind_walk/merges_skipped | count | 119 | 226 | 845.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 7085 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 7085 |
| encode/pass:early prepass/count | count | 1 | 1 | 7085 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 7085 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 7085 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 7085 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 7085 |
| encode/queue/command_buffers | count | 37.88 | 42 | 268.4 k |
| encode/queue/submits | count | 2.01 | 4 | 14.2 k |
| encode/queue/write_buffer/bytes | bytes | 2.30 M | 3.43 M | 16.33 G |
| encode/queue/write_buffer/calls | count | 241 | 301 | 1.71 M |
| encode/queue/write_buffer_with/bytes | bytes | 19.0 k | 1.14 M | 134.89 M |
| encode/queue/write_buffer_with/calls | count | 10.20 | 807 | 72.3 k |
| encode/queue/write_texture/bytes | bytes | 1862 | 1.05 M | 13.19 M |
| encode/queue/write_texture/calls | count | 0.08 | 42 | 557 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 7085 |
| physics/inserts | count | 0.32 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.33 | 175 | 2335 |
| physics/steps | count | 0.56 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 1 | 1 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 30 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.05 | 27 | 351 |
| stream/upload/bytes | bytes | 4778 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1048 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 44, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.35 | 2349.9 | main/busy/ms 2319, stage/main/post_update 2273, stage/main/update 34.95, stage/main/pre_update 7.43, stage/main/last 1.09, surface/advance_cpu_ms 0.80, stream/drive/ms 0.52, stage/main/first 0.46, stage/main/fixed_main_loop 0.39, surface/cell_upload_ms 0.33, stream/tick/ms 0.14, bigspace/total/ms 0.11 |
| warm-up | 3.10 | 751.2 | main/busy/ms 2447, stage/main/post_update 2295, render/thread/ms 716, render/busy/ms 566, stage/render/prepare_assets 504, stage/render/prepare_views 146, render/acquire/ms 145, stage/main/update 135, stage/render/create_views 29.80, stage/render/render 24.64, render/graph/ms 19.38, stage/render/camera_driver 17.43 |
| warm-up | 3.24 | 139.8 | render/thread/ms 839, render/busy/ms 626, stage/render/prepare_assets 550, main/busy/ms 158, stage/render/prepare_views 146, render/acquire/ms 146, stage/main/update 109, stage/render/render 92.15, pipelines/compile_ms 68.94, render/present/ms 67.42, pipelines/driver_ms 48.65, stage/main/post_update 35.47 |
| warm-up | 3.43 | 173.9 | pipelines/wait_ms 4154, pipelines/compile_ms 1926, pipelines/front_ms 1749, pipelines/driver_ms 177, main/busy/ms 169, stage/main/update 105, render/thread/ms 98.00, render/busy/ms 84.28, stage/main/post_update 50.36, encode//sum 40.13, stage/render/prepare_assets 25.35, stage/render/render 22.84 |
| warm-up | 3.51 | 81.4 | pipelines/wait_ms 4194, pipelines/compile_ms 2228, pipelines/front_ms 2081, render/thread/ms 161, render/busy/ms 149, pipelines/driver_ms 147, main/busy/ms 113, stage/render/prepare_assets 88.37, stage/main/update 73.02, encode//sum 42.88, stage/main/post_update 24.33, stage/render/render 23.53 |
| warm-up | 3.56 | 31.0 | pipelines/compile_ms 493, pipelines/front_ms 467, pipelines/wait_ms 102, render/thread/ms 49.00, render/busy/ms 47.48, main/busy/ms 43.00, pipelines/driver_ms 25.65, stage/render/prepare_assets 23.37, stage/main/post_update 21.34, stage/main/update 10.71, terrain/update/ms 10.17, stage/render/render 9.07 |
| warm-up | 3.61 | 43.4 | pipelines/compile_ms 373, pipelines/front_ms 358, pipelines/wait_ms 259, render/thread/ms 71.57, render/busy/ms 69.98, main/busy/ms 46.90, encode//sum 22.21, stage/main/post_update 18.90, stage/render/render 17.45, stage/render/prepare_assets 16.06, render/graph/ms 15.45, pipelines/driver_ms 14.91 |
| warm-up | 3.65 | 39.9 | pipelines/compile_ms 1069, pipelines/front_ms 1018, pipelines/wait_ms 230, main/busy/ms 95.32, render/thread/ms 81.03, render/busy/ms 79.00, stage/main/update 59.75, pipelines/driver_ms 51.13, encode//sum 28.90, stage/render/render 22.09, render/graph/ms 19.52, stage/main/post_update 16.78 |
| warm-up | 3.75 | 107.8 | pipelines/compile_ms 2548, pipelines/front_ms 2435, pipelines/wait_ms 187, pipelines/driver_ms 114, main/busy/ms 88.03, render/thread/ms 85.64, render/busy/ms 83.81, stage/main/update 59.28, stage/render/prepare_assets 45.09, stage/main/post_update 17.16, stage/render/render 13.65, encode//sum 13.27 |
| warm-up | 3.84 | 88.0 | pipelines/compile_ms 2273, pipelines/front_ms 2188, pipelines/wait_ms 127, render/thread/ms 113, render/busy/ms 111, stage/render/prepare_assets 95.07, pipelines/driver_ms 84.99, main/busy/ms 27.87, stage/render/render 11.74, stage/main/update 10.31, render/graph/ms 9.99, stage/main/post_update 9.91 |
| warm-up | 3.93 | 40.7 | pipelines/compile_ms 550, pipelines/front_ms 527, render/thread/ms 45.31, render/busy/ms 43.62, main/busy/ms 39.31, pipelines/driver_ms 22.94, stage/render/render 20.73, stage/main/update 20.48, render/graph/ms 18.58, stage/render/prepare_assets 14.92, stage/render/graph_submit 13.78, encode//sum 12.83 |
| warm-up | 3.98 | 44.4 | pipelines/compile_ms 491, pipelines/front_ms 472, render/thread/ms 54.25, render/busy/ms 52.73, main/busy/ms 31.30, stage/render/prepare_assets 28.55, pipelines/driver_ms 19.19, stage/render/render 14.67, stage/main/update 14.61, render/graph/ms 12.43, stage/main/post_update 8.21, stage/render/graph_submit 8.15 |
| warm-up | 4.05 | 71.3 | pipelines/compile_ms 1221, pipelines/front_ms 1189, render/thread/ms 64.65, render/busy/ms 63.15, stage/render/prepare_assets 42.87, pipelines/driver_ms 32.08, main/busy/ms 31.61, stage/render/render 12.18, stage/main/update 11.52, render/graph/ms 9.72, stage/main/post_update 9.18, encode//sum 7.86 |
| warm-up | 4.11 | 63.5 | pipelines/compile_ms 1416, pipelines/front_ms 1379, render/thread/ms 83.95, render/busy/ms 82.34, stage/render/prepare_assets 54.13, pipelines/driver_ms 37.28, main/busy/ms 36.26, stage/render/render 21.67, render/graph/ms 19.33, stage/main/update 16.94, stage/render/graph_submit 16.44, stage/main/post_update 9.36 |
| warm-up | 4.15 | 43.2 | pipelines/compile_ms 341, pipelines/front_ms 330, main/busy/ms 165, stage/main/update 145, render/thread/ms 82.48, render/busy/ms 76.63, pipelines/wait_ms 53.46, stage/render/prepare_assets 35.39, stage/render/render 27.00, render/graph/ms 20.89, stage/render/graph_submit 18.28, pipelines/driver_ms 10.64 |
| warm-up | 4.36 | 206.2 | pipelines/compile_ms 2102, pipelines/front_ms 2051, main/busy/ms 157, stage/main/update 140, render/thread/ms 113, render/busy/ms 107, pipelines/wait_ms 65.57, pipelines/driver_ms 51.59, stage/render/prepare_assets 38.38, stage/render/prepare_meshes 33.20, stage/render/render 22.20, render/graph/ms 16.01 |
| warm-up | 4.39 | 30.0 | pipelines/compile_ms 2083, pipelines/front_ms 2032, render/thread/ms 102, render/busy/ms 101, main/busy/ms 69.35, pipelines/wait_ms 58.06, pipelines/driver_ms 51.80, stage/render/prepare_assets 33.36, stage/render/prepare_meshes 31.75, stage/render/render 25.54, stage/main/post_update 22.38, render/graph/ms 22.34 |
| warm-up | 4.45 | 61.8 | pipelines/compile_ms 196, pipelines/front_ms 188, pipelines/wait_ms 71.89, main/busy/ms 69.46, render/thread/ms 42.32, render/busy/ms 40.84, stage/main/post_update 22.31, stage/main/update 21.19, stage/render/render 19.24, render/graph/ms 16.28, stage/render/graph_submit 13.56, anim/evaluate/ms 12.89 |
| warm-up | 4.52 | 73.1 | pipelines/compile_ms 235, pipelines/front_ms 227, render/thread/ms 82.75, render/busy/ms 81.59, main/busy/ms 73.69, stage/main/update 30.49, stage/main/post_update 28.15, pipelines/wait_ms 27.38, stage/render/prepare_write_phase_buffers 26.34, stage/render/prepare_resources 22.66, stream/drive/ms 19.18, stage/render/prepare_assets 16.11 |
| warm-up | 4.57 | 48.4 | pipelines/compile_ms 1860, pipelines/front_ms 1802, render/thread/ms 108, render/busy/ms 106, main/busy/ms 89.13, pipelines/driver_ms 57.51, stage/main/post_update 35.61, stage/main/update 34.42, pipelines/wait_ms 34.30, stage/render/prepare_write_phase_buffers 27.45, stage/render/prepare_assets 27.23, stage/render/prepare_resources 23.88 |
| warm-up | 4.61 | 37.4 | pipelines/compile_ms 2221, pipelines/front_ms 2135, pipelines/driver_ms 85.29, render/thread/ms 70.80, render/busy/ms 68.78, main/busy/ms 45.49, pipelines/wait_ms 35.47, stage/render/render 28.83, render/graph/ms 26.29, stage/render/prepare_assets 26.09, stage/main/update 16.17, stage/main/post_update 15.28 |
| warm-up | 4.67 | 61.5 | pipelines/compile_ms 781, pipelines/front_ms 741, render/thread/ms 75.36, render/busy/ms 71.17, main/busy/ms 67.91, stage/main/update 49.04, pipelines/driver_ms 40.29, stage/render/prepare_assets 28.35, stage/render/render 24.64, render/graph/ms 20.07, stage/render/camera_driver 11.71, stage/main/post_update 8.90 |
| warm-up | 4.74 | 71.9 | pipelines/compile_ms 503, pipelines/front_ms 487, render/thread/ms 91.56, render/busy/ms 87.56, main/busy/ms 74.57, pipelines/wait_ms 69.03, stage/main/update 54.12, stage/render/prepare_assets 49.45, pipelines/driver_ms 15.64, stage/render/render 14.71, stage/main/post_update 12.74, render/graph/ms 10.21 |
| warm-up | 4.77 | 29.9 | pipelines/compile_ms 470, pipelines/front_ms 455, main/busy/ms 107, pipelines/wait_ms 79.17, stage/main/update 77.20, render/thread/ms 74.27, render/busy/ms 72.74, stream/drive/ms 53.46, stage/render/prepare_assets 35.72, stage/render/render 15.19, stage/main/post_update 14.93, pipelines/driver_ms 14.83 |
| warm-up | 4.86 | 87.6 | pipelines/compile_ms 2125, pipelines/front_ms 2072, main/busy/ms 102, stage/main/update 77.00, pipelines/driver_ms 52.60, stream/drive/ms 52.24, render/thread/ms 49.84, render/busy/ms 48.34, pipelines/wait_ms 25.72, stage/render/prepare_assets 16.63, stage/render/render 14.91, render/graph/ms 13.06 |
| warm-up | 4.93 | 49.7 | pipelines/compile_ms 403, pipelines/front_ms 387, main/busy/ms 49.99, render/thread/ms 34.38, stage/main/update 33.51, render/busy/ms 32.26, pipelines/driver_ms 15.77, stage/render/prepare_assets 13.70, stream/drive/ms 12.27, stage/main/post_update 9.75, stage/render/render 9.07, encode//sum 6.65 |
| warm-up | 4.96 | 29.4 | pipelines/compile_ms 414, pipelines/front_ms 398, render/thread/ms 48.60, render/busy/ms 46.57, main/busy/ms 43.66, stage/main/update 25.58, pipelines/driver_ms 15.68, stage/render/prepare_assets 15.65, stage/render/render 12.63, stage/main/post_update 11.13, render/graph/ms 10.89, encode//sum 8.11 |
| warm-up | 4.98 | 29.4 | render/thread/ms 49.80, render/busy/ms 47.55, main/busy/ms 43.74, pipelines/compile_ms 41.76, pipelines/front_ms 40.04, stage/main/update 25.38, stage/render/render 13.65, stage/render/prepare_assets 12.39, render/graph/ms 11.73, stage/main/post_update 10.86, encode//sum 8.78, stage/render/graph_submit 8.17 |
| warm-up | 5.16 | 37.2 | pipelines/compile_ms 379, pipelines/front_ms 369, main/busy/ms 40.48, render/thread/ms 28.78, render/busy/ms 27.34, stage/main/update 19.86, stage/main/post_update 14.04, stream/drive/ms 12.46, stage/render/render 11.05, pipelines/driver_ms 9.84, render/graph/ms 9.25, stage/render/prepare_assets 7.78 |
| warm-up | 5.21 | 26.6 | render/thread/ms 38.90, render/busy/ms 37.35, main/busy/ms 32.90, stage/main/update 15.02, stage/render/render 12.64, stage/render/prepare_assets 10.76, stage/main/post_update 10.74, render/graph/ms 10.68, encode//sum 8.60, stage/render/graph_submit 7.63, stage/render/prepare_flush 5.10, encode/submit 5.06 |
| warm-up | 5.42 | 29.7 | pipelines/compile_ms 110, pipelines/front_ms 105, main/busy/ms 40.04, render/thread/ms 29.26, render/busy/ms 27.04, stage/main/update 25.80, stream/drive/ms 18.56, stage/render/render 13.82, render/graph/ms 11.18, stage/main/post_update 8.28, stage/render/graph_submit 8.28, encode//sum 6.63 |
| warm-up | 5.63 | 26.5 | main/busy/ms 31.99, stage/main/update 20.66, render/thread/ms 19.47, render/busy/ms 18.61, stream/drive/ms 13.18, stage/render/render 8.72, render/graph/ms 7.36, stage/main/post_update 6.62, encode//sum 6.15, stage/render/prepare_assets 4.47, stage/render/graph_submit 4.43, stage/render/camera_driver 2.75 |
| warm-up | 5.74 | 27.3 | render/thread/ms 37.62, render/busy/ms 36.02, main/busy/ms 25.07, stage/render/prepare_assets 15.07, stage/render/render 12.27, stage/main/update 11.89, render/graph/ms 10.15, stage/main/post_update 9.64, encode//sum 8.36, stage/render/graph_submit 7.00, encode/submit 4.30, stage/render/camera_driver 2.93 |
| warm-up | 5.79 | 28.4 | pipelines/compile_ms 133, pipelines/front_ms 130, render/thread/ms 30.53, render/busy/ms 29.60, main/busy/ms 21.92, stage/render/prepare_assets 14.56, stage/main/update 9.94, stage/render/render 6.83, stage/main/post_update 6.81, encode//sum 5.65, render/graph/ms 5.53, pipelines/driver_ms 3.46 |
| warm-up | 5.88 | 28.9 | render/thread/ms 34.87, render/busy/ms 33.55, main/busy/ms 31.15, stage/main/update 18.79, stage/render/prepare_assets 10.12, stage/render/render 9.41, render/graph/ms 7.86, stage/main/post_update 6.74, encode//sum 6.17, stage/render/graph_submit 5.56, stage/render/prepare_meshes 4.38, encode/submit 3.34 |
| warm-up | 6.00 | 26.0 | render/thread/ms 39.98, render/busy/ms 38.56, main/busy/ms 37.66, stage/main/update 19.89, stage/render/render 14.32, render/graph/ms 12.45, stage/main/post_update 11.05, stage/render/prepare_assets 9.28, encode//sum 9.11, stage/render/graph_submit 8.99, encode/submit 5.97, stage/render/prepare_meshes 4.13 |
| warm-up | 6.03 | 31.7 | render/thread/ms 43.51, render/busy/ms 42.40, main/busy/ms 39.30, stage/main/update 20.25, stage/render/prepare_assets 16.82, stage/main/post_update 10.64, stage/render/render 8.90, render/graph/ms 7.49, encode//sum 7.31, stage/render/prepare_meshes 5.65, stage/render/graph_submit 4.85, stream/drive/ms 4.45 |
| warm-up | 6.07 | 31.9 | render/thread/ms 53.06, render/busy/ms 51.28, main/busy/ms 44.64, stage/main/update 21.57, stage/render/prepare_assets 17.21, stage/render/render 13.51, stage/main/post_update 13.46, render/graph/ms 11.20, encode//sum 8.71, stage/render/graph_submit 7.10, stage/render/prepare_meshes 6.54, encode/submit 4.10 |
| warm-up | 6.12 | 30.9 | pipelines/compile_ms 148, pipelines/front_ms 144, render/thread/ms 37.57, render/busy/ms 36.65, main/busy/ms 32.52, stage/main/update 16.68, stage/render/prepare_assets 16.45, stage/main/post_update 9.65, stage/render/render 7.57, encode//sum 6.45, render/graph/ms 6.32, stage/render/graph_submit 3.97 |
| warm-up | 6.25 | 35.0 | main/busy/ms 45.25, render/thread/ms 30.73, stage/main/update 30.48, render/busy/ms 29.30, stream/drive/ms 13.15, stage/render/render 11.49, render/graph/ms 9.50, stage/main/post_update 9.24, encode//sum 8.58, stage/render/prepare_assets 7.21, stage/render/graph_submit 6.47, stage/render/prepare_meshes 4.46 |
| warm-up | 6.28 | 26.0 | main/busy/ms 45.81, render/thread/ms 39.20, render/busy/ms 37.61, stage/main/update 28.24, stage/render/render 13.62, stage/render/prepare_assets 11.81, render/graph/ms 11.20, stage/main/post_update 10.06, encode//sum 8.98, stage/render/graph_submit 7.54, stream/drive/ms 5.78, stage/render/prepare_meshes 5.68 |
| warm-up | 6.31 | 31.5 | pipelines/compile_ms 140, pipelines/front_ms 136, main/busy/ms 44.25, render/thread/ms 39.34, render/busy/ms 38.09, stage/main/update 25.09, stage/render/prepare_assets 14.56, stage/render/render 11.34, stage/main/post_update 11.03, render/graph/ms 9.34, encode//sum 7.24, stream/drive/ms 6.39 |
| measure | 18.25 | 33.4 | main/busy/ms 37.64, stage/main/update 26.53, render/thread/ms 18.62, render/busy/ms 17.65, stage/render/render 8.85, stage/main/post_update 7.64, render/graph/ms 7.58, encode//sum 5.61, stage/render/graph_submit 5.40, stage/render/prepare_assets 3.69, encode/submit 3.48, stage/render/camera_driver 2.05 |
| measure | 59.10 | 31.0 | render/thread/ms 36.63, render/busy/ms 35.65, main/busy/ms 26.41, stage/render/prepare_assets 17.94, stage/main/post_update 12.62, stage/render/render 8.29, encode//sum 7.01, render/graph/ms 6.98, stage/main/update 5.32, stage/main/fixed_main_loop 4.56, stage/render/graph_submit 4.42, encode/submit 2.59 |

## Run

- `../target/profiling/bench-shadow-review.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.count=0 --resolution 2560x1440 --bench-out target/perf\shadow_review2\balmora_walk_night@2560x1440_arm=nolamps-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 4% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1216 pmon samples)
- focus: clean
- nvidia-smi over the window (256 samples): GPU load **87%** (min 58%) · 2619 MHz (min 2610) · memory 8101 MHz · 88.2 W · 81 °C (max 86) · VRAM 5557 MiB · throttled 100% of samples

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
- `sg.post=high (preset: level requested)`
- `sg.shadows=custom (preset: derived from its members)`
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
- `shadow.lamp.count=0 (command line: --set)`
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
