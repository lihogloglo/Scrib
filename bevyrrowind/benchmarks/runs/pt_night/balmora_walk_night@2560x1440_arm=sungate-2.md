# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.gate=on (command line: --set); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5881 frames over 65.6 s · **median 10.74 ms (93 fps)** · mean 11.16 · p95 14.78 · p99 17.51 (1% low 57 fps) · worst 33.58 ms · 110 spikes over 1.5× median · 23.8 stutters/min (work 21.0/min)
- warm-up: 323 frames over 9.4 s, worst 2408.5 ms
- hitches (> 26.9 ms): **4** measured, 50 in warm-up
- timed GPU passes: 6.70 ms of a 11.16 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.74 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.77/min, limit 1/min
- **FAIL** worst frame 33.58 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.000 | 2.029 | 2.062 | 0.99 | 0.173 |
| main_opaque_pass_3d | 1.294 | 1.312 | 1.647 | 0.99 | 0.069 |
| early prepass | 0.481 | 0.488 | 0.700 | 0.99 | 0.087 |
| bloom | 0.311 | 0.316 | 0.322 | 0.99 | 0.062 |
| atmosphere_luts | 0.305 | 0.309 | 0.372 | 0.99 | 0.003 |
| auxiliary_views | 0.293 | 0.298 | 1.091 | 0.99 | 0.282 |
| water_velocity | 0.258 | 0.262 | 0.349 | 0.99 | 0.002 |
| water_sspr | 0.199 | 0.202 | 0.203 | 0.99 | 0.002 |
| ssao | 0.186 | 0.188 | 0.193 | 0.99 | 0.007 |
| upscaling | 0.128 | 0.130 | 0.138 | 0.99 | 0.004 |
| water_fft | 0.127 | 0.129 | 0.137 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.126 | 0.127 | 0.128 | 0.99 | 0.001 |
| uniform_allocation | 0.123 | 0.125 | 0.152 | 0.99 | 0.026 |
| clouds_composite | 0.103 | 0.105 | 0.108 | 0.99 | 0.002 |
| bin_unpacking | 0.093 | 0.095 | 0.116 | 0.99 | 0.019 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.078 | 0.155 | 0.99 | 0.009 |
| clustering | 0.063 | 0.064 | 0.067 | 0.99 | 0.019 |
| clouds_trace | 0.055 | 0.056 | 0.074 | 0.99 | 0.001 |
| render_sky | 0.051 | 0.051 | 0.052 | 0.99 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.050 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.047 | 0.413 | 0.416 | 0.11 | 0.001 |
| surface_trail | 0.040 | 0.056 | 0.057 | 0.71 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.025 | 0.219 | 0.229 | 0.11 | 0.004 |
| water_snapshot | 0.025 | 0.025 | 0.028 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.99 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.038 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.025 | 0.99 | 0.126 |
| clouds_fog_light | 0.012 | 0.012 | 0.013 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.99 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.787 | 13.333 | 31.94 | 57557.7 |
| render/busy/ms | 9.272 | 12.785 | 31.39 | 54529.9 |
| main/busy/ms | 9.203 | 11.224 | 28.77 | 54122.8 |
| stage/render/render | 5.930 | 8.627 | 25.62 | 34876.0 |
| stage/main/post_update | 5.741 | 7.434 | 24.60 | 33764.1 |
| render/graph/ms | 5.227 | 7.824 | 24.78 | 30741.5 |
| encode//sum | 4.585 | 7.280 | 39.37 | 26962.5 |
| stage/render/graph_submit | 3.901 | 5.736 | 22.60 | 22943.3 |
| encode/submit | 2.518 | 3.857 | 6.17 | 14806.4 |
| stage/main/update | 2.188 | 2.683 | 22.13 | 12869.4 |
| encode/pass:shadow_point_light/sum | 1.549 | 5.098 | 16.57 | 9108.7 |
| encode/finish_wall | 1.421 | 1.992 | 18.14 | 8357.2 |
| stage/render/camera_driver | 1.272 | 2.066 | 4.19 | 7480.3 |
| stage/render/prepare_write_phase_buffers | 0.788 | 1.128 | 3.07 | 4634.7 |
| encode/pass:shadow_directional_light/sum | 0.766 | 1.272 | 2.18 | 4506.6 |
| encode/pass:main_opaque_pass_3d/sum | 0.735 | 1.079 | 1.95 | 4319.9 |
| encode/pass:early prepass/sum | 0.726 | 1.081 | 16.38 | 4269.6 |
| stage/render/prepare_assets | 0.597 | 0.762 | 9.43 | 3510.8 |
| bigspace/total/ms | 0.561 | 1.391 | 2.29 | 3297.3 |
| stage/render/prepare_resources | 0.461 | 0.555 | 1.64 | 2711.0 |
| stage/render/prepare_meshes | 0.429 | 0.547 | 2.31 | 2521.1 |
| render/present/ms | 0.422 | 0.521 | 2.65 | 2484.0 |
| stage/main/first | 0.397 | 0.522 | 5.75 | 2334.6 |
| stage/main/pre_update | 0.360 | 0.507 | 1.20 | 2119.6 |
| terrain/update/ms | 0.318 | 0.417 | 1.96 | 1869.8 |
| terrain/select/ms | 0.284 | 0.374 | 1.05 | 1670.5 |
| stage/render/prepare_bind_groups | 0.268 | 0.482 | 2.28 | 1574.2 |
| controller/feed/ms | 0.266 | 0.335 | 0.86 | 1565.9 |
| stage/render/specialize | 0.264 | 0.397 | 2.42 | 1552.0 |
| stream/drive/ms | 0.252 | 0.338 | 20.03 | 1484.2 |
| encode/pass:main_transparent_pass_3d/sum | 0.249 | 0.367 | 1.04 | 1462.1 |
| stage/render/queue | 0.246 | 0.393 | 3.02 | 1449.0 |
| bigspace/low_precision/ms | 0.232 | 0.726 | 1.69 | 1366.1 |
| stage/main/fixed_main_loop | 0.224 | 0.376 | 1.16 | 1315.2 |
| far/select/ms | 0.202 | 0.276 | 0.91 | 1190.2 |
| stage/render/create_views | 0.195 | 0.254 | 0.89 | 1147.5 |
| stream/tick/ms | 0.194 | 0.265 | 0.57 | 1142.0 |
| stage/render/prepare_batch_phases | 0.190 | 0.271 | 0.80 | 1114.8 |
| stage/render/extract_commands | 0.175 | 0.205 | 0.57 | 1026.4 |
| stage/render/prepare_flush | 0.170 | 0.327 | 1.64 | 999.0 |
| anim/evaluate/ms | 0.169 | 0.267 | 1.61 | 991.2 |
| anim/evaluate_cpu/ms | 0.161 | 0.209 | 0.76 | 947.2 |
| bigspace/high_precision/ms | 0.161 | 0.700 | 1.76 | 947.2 |
| controller/move/ms | 0.144 | 0.294 | 0.58 | 844.9 |
| stage/render/prepare_views | 0.136 | 0.170 | 0.45 | 800.4 |
| stage/main/last | 0.114 | 0.155 | 1.54 | 671.2 |
| encode/pass:main_transmissive_pass_3d/sum | 0.106 | 0.174 | 0.71 | 620.6 |
| render/acquire/ms | 0.092 | 0.119 | 0.40 | 543.9 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.128 | 0.51 | 349.5 |
| physics/step/ms | 0.049 | 0.087 | 0.76 | 287.5 |
| physics/insert/ms | 0.036 | 0.387 | 1.31 | 210.1 |
| stage/render/cleanup | 0.035 | 0.114 | 1.08 | 204.8 |
| stage/render/phase_sort | 0.033 | 0.056 | 0.23 | 192.1 |
| encode/resolve_diagnostics/sum | 0.030 | 0.041 | 0.55 | 179.2 |
| pipelines/compile_ms | 0.016 | 0.000 | 85.38 | 96.4 |
| pipelines/front_ms | 0.016 | 0.000 | 82.38 | 93.2 |
| weather/map/ms | 0.016 | 0.020 | 0.17 | 92.8 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.46 | 85.4 |
| anim/apply/ms | 0.012 | 0.024 | 0.31 | 71.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.44 | 64.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.35 | 63.2 |
| weather/eval/ms | 0.010 | 0.013 | 0.11 | 57.9 |
| script/update/ms | 0.007 | 0.009 | 0.19 | 44.1 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 31.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.10 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.16 | 29.1 |
| script/ui/ms | 0.004 | 0.006 | 0.47 | 25.5 |
| controller/write/ms | 0.003 | 0.004 | 0.20 | 19.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.62 | 14.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.54 | 5.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 5.1 |
| pipelines/driver_ms | 0.001 | 0.000 | 3.00 | 3.2 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| crowd/c0/count | count | 27 | 43 | 12 |
| crowd/c0/joints/count | count | 1291 | 2262 | 537 |
| crowd/c0/meshes/count | count | 353 | 646 | 146 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5573 | 5995 | 5659 |
| crowd/c1/meshes/count | count | 1180 | 1297 | 1195 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 204 |
| crowd/joints_extracted/count | count | 3700 | 7038 | 5731 |
| crowd/joints_uploaded/count | count | 3642 | 6220 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.42 | 16.75 | 0.73 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.46 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.44 | 0.01 |
| encode/finish_critical | ms | 0.96 | 16.75 | 1.50 |
| encode/pass:early prepass/max | ms | 0.76 | 16.38 | 1.07 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.75 | 1.95 | 0.84 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.10 | 0.71 | 0.28 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.24 | 1.04 | 0.30 |
| encode/pass:shadow_directional_light/max | ms | 0.77 | 2.18 | 1.50 |
| encode/pass:shadow_point_light/max | ms | 0.82 | 1.53 | 0.94 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.55 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.41 | 31.39 | 8.96 |
| gpu/timed/ms | ms | 7.47 | 10.85 | 7.67 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 791 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 791 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1298 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3341 | 5170 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1298 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3341 | 5170 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 22 | 191 | 19 |
| phases/shadow/bins | count | 3661 | 4483 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5898 | 7390 | 3025 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 197 | 228 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 194 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 323 | 324 | 324 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 12 | 543 | 37 |
| preprocess/bind_groups_created | count | 12 | 543 | 37 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 133 | 298 | 127 |
| process/cpu_usage | % | 26.21 | 51.58 | 25.24 |
| process/mem_usage | GiB | 5.41 | 5.57 | 5.57 |
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
| skin/blocks_changed/count | count | 13 | 82 | 10 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 216 | 204 |
| skin/joint_bytes_written/bytes | bytes | 280.3 k | 808.1 k | 445.4 k |
| skin/joints_written/count | count | 4380 | 12.6 k | 6960 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4920 | 5010 | 5010 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 432 | 528 | 396 |
| statics/materials_bound | count | 432 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.10 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.35 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.87 | 49.81 | 29.49 |
| system/mem_usage | % | 53.83 | 54.69 | 54.66 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 71 | 240 | 71 |
| views/all/bins | count | 6189 | 7474 | 4989 |
| views/all/count | count | 45 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.4 k | 20.5 k | 10.6 k |
| views/all/pipelines | count | 244 | 277 | 67 |
| views/all/pipelines_distinct | count | 71 | 75 | 63 |
| views/all/work_items | count | 10.4 k | 18.6 k | 10.9 k |
| views/cascade/batch_sets | count | 9 | 27 | 19 |
| views/cascade/bins | count | 1451 | 1560 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3388 | 4437 | 3025 |
| views/cascade/pipelines | count | 23 | 27 | 19 |
| views/cascade/work_items | count | 325 | 4434 | 3025 |
| views/lamp_face/batch_sets | count | 0 | 183 | 0 |
| views/lamp_face/bins | count | 2261 | 3039 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2472 | 3352 | 0 |
| views/lamp_face/pipelines | count | 174 | 202 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2836 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2896 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8270 | 13.1 k | 7616 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8432 | 13.3 k | 7924 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 63.76 | 367 | 374.9 k |
| anim/events/count | count | 0.09 | 3 | 529 |
| assets/Image/added | count | 0.10 | 32 | 592 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.69 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.68 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1378 | 4675 |
| encode//count | count | 39.20 | 219 | 230.5 k |
| encode/bind_walk/init_textures | count | 15.7 k | 112.9 k | 92.38 M |
| encode/bind_walk/init_textures_skipped | count | 51.0 k | 118.3 k | 299.72 M |
| encode/bind_walk/inits | count | 476 | 1102 | 2.80 M |
| encode/bind_walk/inits_skipped | count | 97.17 | 149 | 571.4 k |
| encode/bind_walk/merge_textures | count | 15.6 k | 112.9 k | 92.01 M |
| encode/bind_walk/merge_textures_skipped | count | 5396 | 6012 | 31.73 M |
| encode/bind_walk/merges | count | 107 | 335 | 627.8 k |
| encode/bind_walk/merges_skipped | count | 78.71 | 121 | 462.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5881 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5881 |
| encode/pass:early prepass/count | count | 1 | 1 | 5881 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5881 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5881 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5881 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5881 |
| encode/pass:shadow_point_light/count | count | 3.06 | 48 | 18.0 k |
| encode/queue/command_buffers | count | 52.28 | 277 | 307.4 k |
| encode/queue/submits | count | 2.02 | 4 | 11.9 k |
| encode/queue/write_buffer/bytes | bytes | 1.82 M | 2.71 M | 10.71 G |
| encode/queue/write_buffer/calls | count | 270 | 811 | 1.59 M |
| encode/queue/write_buffer_with/bytes | bytes | 169.9 k | 3.53 M | 999.12 M |
| encode/queue/write_buffer_with/calls | count | 11.06 | 465 | 65.0 k |
| encode/queue/write_texture/bytes | bytes | 10.9 k | 8.39 M | 64.12 M |
| encode/queue/write_texture/calls | count | 0.10 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5881 |
| physics/inserts | count | 0.39 | 109 | 2269 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5756 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1262 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.9 ms; slowest 54, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.41 | 2408.5 | main/busy/ms 2378, stage/main/post_update 2339, stage/main/update 31.17, stage/main/pre_update 4.58, stage/main/last 1.05, surface/advance_cpu_ms 0.73, stage/main/first 0.49, stage/main/fixed_main_loop 0.41, stream/drive/ms 0.39, surface/cell_upload_ms 0.28, bigspace/total/ms 0.08, stream/tick/ms 0.07 |
| warm-up | 3.24 | 832.6 | main/busy/ms 2549, stage/main/post_update 2367, render/thread/ms 798, render/busy/ms 625, stage/render/prepare_assets 574, stage/render/prepare_views 168, render/acquire/ms 168, stage/main/update 168, stage/render/create_views 28.02, stage/render/render 13.98, render/graph/ms 7.95, stage/render/prepare_resources 7.48 |
| warm-up | 3.37 | 133.0 | render/thread/ms 913, render/busy/ms 680, stage/render/prepare_assets 617, main/busy/ms 196, stage/render/prepare_views 169, render/acquire/ms 168, stage/main/update 143, pipelines/compile_ms 87.99, stage/render/render 78.19, render/present/ms 65.13, pipelines/driver_ms 52.44, stage/main/post_update 39.62 |
| warm-up | 3.41 | 32.2 | pipelines/compile_ms 418, pipelines/front_ms 290, render/thread/ms 145, pipelines/driver_ms 128, main/busy/ms 93.16, stage/render/render 86.96, render/busy/ms 81.71, pipelines/wait_ms 67.07, render/present/ms 61.90, stage/render/prepare_assets 44.60, stage/main/post_update 38.09, terrain/update/ms 28.96 |
| warm-up | 3.53 | 123.0 | pipelines/wait_ms 4381, pipelines/compile_ms 1271, pipelines/front_ms 1133, pipelines/driver_ms 138, render/thread/ms 105, render/busy/ms 100, main/busy/ms 86.41, stage/main/post_update 39.07, stage/render/render 32.67, render/graph/ms 29.22, stage/render/graph_submit 26.24, stage/render/prepare_assets 23.71 |
| warm-up | 3.59 | 64.1 | pipelines/wait_ms 4671, pipelines/compile_ms 2119, pipelines/front_ms 1996, pipelines/driver_ms 123, render/thread/ms 120, render/busy/ms 117, main/busy/ms 112, stage/main/update 80.17, stage/render/prepare_assets 30.83, stage/main/post_update 21.77, stage/render/prepare_flush 19.78, stage/render/prepare_resources 18.57 |
| warm-up | 3.72 | 126.7 | pipelines/compile_ms 2722, pipelines/front_ms 2605, pipelines/wait_ms 715, render/thread/ms 121, render/busy/ms 119, pipelines/driver_ms 117, main/busy/ms 110, stage/main/update 82.37, stage/render/prepare_assets 73.67, stage/main/post_update 18.37, terrain/update/ms 14.66, stage/render/prepare_resources 14.54 |
| warm-up | 3.78 | 57.3 | pipelines/compile_ms 2202, pipelines/front_ms 1932, pipelines/wait_ms 361, pipelines/driver_ms 270, render/thread/ms 132, render/busy/ms 130, stage/render/prepare_assets 111, main/busy/ms 31.57, encode//sum 17.04, stage/main/post_update 16.82, stage/render/render 13.68, render/graph/ms 11.70 |
| warm-up | 3.81 | 34.0 | pipelines/compile_ms 791, pipelines/front_ms 548, pipelines/driver_ms 243, render/thread/ms 88.95, render/busy/ms 83.46, stage/render/prepare_assets 46.80, main/busy/ms 31.41, stage/render/render 30.84, encode//sum 29.15, render/graph/ms 28.22, stage/render/graph_submit 24.17, stage/main/post_update 18.60 |
| warm-up | 3.86 | 34.6 | pipelines/compile_ms 62.40, pipelines/front_ms 57.28, render/thread/ms 38.44, render/busy/ms 37.00, main/busy/ms 21.42, stage/render/prepare_assets 21.28, stage/main/post_update 13.40, stage/render/render 9.90, terrain/update/ms 8.42, render/graph/ms 7.66, encode//sum 7.53, stage/render/graph_submit 5.55 |
| warm-up | 3.95 | 72.5 | pipelines/compile_ms 850, pipelines/front_ms 817, render/thread/ms 55.61, main/busy/ms 54.96, render/busy/ms 54.09, stage/main/update 41.59, stage/render/prepare_assets 39.05, stream/drive/ms 37.43, pipelines/driver_ms 33.62, stage/render/render 10.77, render/graph/ms 8.99, stage/main/post_update 6.44 |
| warm-up | 4.01 | 33.7 | pipelines/compile_ms 296, pipelines/front_ms 286, render/thread/ms 48.15, render/busy/ms 46.86, main/busy/ms 39.06, stage/main/update 26.30, stage/render/render 22.42, stream/drive/ms 21.05, render/graph/ms 20.87, stage/render/prepare_assets 19.41, stage/render/camera_driver 13.88, pipelines/driver_ms 9.56 |
| warm-up | 4.10 | 66.9 | pipelines/compile_ms 282, pipelines/front_ms 271, render/thread/ms 57.37, render/busy/ms 55.25, main/busy/ms 47.71, stage/render/prepare_assets 38.02, stage/main/update 29.76, stream/drive/ms 24.90, stage/render/render 11.74, pipelines/driver_ms 11.29, stage/main/post_update 10.06, render/graph/ms 9.00 |
| warm-up | 4.16 | 61.8 | pipelines/compile_ms 932, pipelines/front_ms 905, render/thread/ms 94.48, render/busy/ms 92.62, stage/render/prepare_assets 52.12, main/busy/ms 30.47, pipelines/driver_ms 26.64, stage/render/prepare_resources 15.08, stage/render/render 14.36, stage/main/update 12.72, render/graph/ms 11.69, stage/main/post_update 8.98 |
| warm-up | 4.23 | 74.4 | pipelines/compile_ms 1499, pipelines/front_ms 1458, render/thread/ms 97.72, render/busy/ms 96.31, stage/render/prepare_assets 60.08, main/busy/ms 48.33, pipelines/driver_ms 40.94, stage/main/post_update 20.94, stage/main/update 19.65, stage/render/prepare_resources 14.84, stage/render/render 11.21, render/graph/ms 9.07 |
| warm-up | 4.32 | 84.3 | pipelines/compile_ms 1705, pipelines/front_ms 1638, render/thread/ms 123, render/busy/ms 121, main/busy/ms 96.20, stage/render/prepare_assets 83.95, pipelines/driver_ms 66.57, stage/main/update 46.50, stage/main/post_update 23.79, stage/main/pre_update 20.70, stage/render/prepare_resources 17.55, stage/render/render 9.56 |
| warm-up | 4.41 | 95.3 | pipelines/compile_ms 1885, pipelines/front_ms 1802, pipelines/wait_ms 364, render/thread/ms 145, render/busy/ms 144, main/busy/ms 92.59, stage/render/prepare_assets 89.11, pipelines/driver_ms 83.33, stage/main/update 43.61, stage/main/post_update 22.03, stage/render/prepare_resources 21.46, stage/main/pre_update 21.00 |
| warm-up | 4.51 | 97.6 | pipelines/compile_ms 2614, pipelines/front_ms 2526, pipelines/wait_ms 692, render/thread/ms 160, render/busy/ms 158, pipelines/driver_ms 88.52, stage/render/prepare_assets 81.77, main/busy/ms 56.31, stage/render/prepare_resources 22.58, stage/main/post_update 20.27, stage/main/update 18.02, stage/render/render 15.67 |
| warm-up | 4.57 | 58.4 | pipelines/compile_ms 2490, pipelines/front_ms 2417, pipelines/wait_ms 634, render/thread/ms 148, render/busy/ms 146, pipelines/driver_ms 73.36, main/busy/ms 57.22, encode//sum 55.67, stage/render/prepare_assets 43.87, stage/render/render 24.12, stage/main/post_update 22.76, render/graph/ms 21.75 |
| warm-up | 4.63 | 64.8 | pipelines/compile_ms 1835, pipelines/front_ms 1781, pipelines/wait_ms 866, render/thread/ms 106, render/busy/ms 104, main/busy/ms 59.23, encode//sum 58.03, pipelines/driver_ms 53.51, stage/render/render 33.25, render/graph/ms 30.99, stage/main/post_update 25.56, stage/render/graph_submit 22.31 |
| warm-up | 4.69 | 60.7 | pipelines/compile_ms 2088, pipelines/front_ms 2023, pipelines/wait_ms 1778, render/thread/ms 108, render/busy/ms 106, main/busy/ms 104, stage/main/update 67.63, pipelines/driver_ms 64.74, stream/drive/ms 55.92, encode//sum 54.31, stage/render/render 38.20, render/graph/ms 35.77 |
| warm-up | 4.76 | 67.7 | pipelines/compile_ms 1562, pipelines/front_ms 1509, pipelines/wait_ms 1405, main/busy/ms 103, render/thread/ms 100, render/busy/ms 98.19, stage/main/update 73.60, stream/drive/ms 56.58, encode//sum 54.62, pipelines/driver_ms 52.54, stage/render/render 34.48, render/graph/ms 31.03 |
| warm-up | 4.82 | 56.4 | pipelines/compile_ms 1132, pipelines/front_ms 1093, pipelines/wait_ms 285, main/busy/ms 122, stage/main/update 98.76, render/thread/ms 88.64, render/busy/ms 86.03, stream/drive/ms 55.56, pipelines/driver_ms 38.89, stage/render/render 26.14, render/graph/ms 22.47, stage/render/prepare_assets 22.26 |
| warm-up | 4.99 | 167.4 | pipelines/compile_ms 1977, pipelines/front_ms 1931, pipelines/wait_ms 444, render/thread/ms 129, render/busy/ms 127, main/busy/ms 119, stage/main/update 91.29, stage/render/prepare_assets 62.00, stream/drive/ms 54.57, pipelines/driver_ms 46.40, stage/render/prepare_resources 22.85, stage/render/render 21.83 |
| warm-up | 5.03 | 46.6 | pipelines/compile_ms 1588, pipelines/front_ms 1549, pipelines/wait_ms 351, render/thread/ms 127, render/busy/ms 124, main/busy/ms 84.30, stage/main/update 56.16, stage/render/prepare_assets 52.04, pipelines/driver_ms 39.80, stream/drive/ms 39.26, stage/render/render 39.13, render/graph/ms 35.18 |
| warm-up | 5.11 | 73.8 | pipelines/compile_ms 129, pipelines/front_ms 119, main/busy/ms 87.98, render/thread/ms 79.83, render/busy/ms 76.57, stage/main/update 67.92, stream/drive/ms 39.43, stage/render/render 36.80, render/graph/ms 32.70, stage/render/prepare_assets 27.92, stage/render/graph_submit 26.69, encode/submit 21.50 |
| warm-up | 5.14 | 36.4 | main/busy/ms 73.08, render/thread/ms 65.24, render/busy/ms 63.82, stage/main/update 55.77, stream/drive/ms 35.50, stage/render/prepare_assets 30.50, stage/render/render 16.34, render/graph/ms 14.29, stage/render/graph_submit 10.34, encode//sum 10.17, stage/main/post_update 10.01, encode/submit 7.26 |
| warm-up | 5.18 | 35.6 | pipelines/compile_ms 1079, pipelines/front_ms 1051, main/busy/ms 57.29, render/thread/ms 44.88, render/busy/ms 43.16, stage/main/update 41.85, stream/drive/ms 34.98, pipelines/driver_ms 28.04, stage/render/render 17.54, render/graph/ms 15.39, stage/render/graph_submit 11.19, stage/render/prepare_assets 10.88 |
| warm-up | 5.25 | 32.2 | pipelines/compile_ms 138, pipelines/front_ms 132, render/thread/ms 42.87, render/busy/ms 41.61, main/busy/ms 30.27, stage/render/prepare_assets 18.25, stage/main/update 14.36, stage/render/render 12.18, render/graph/ms 10.24, encode//sum 10.07, stage/main/post_update 9.83, stage/render/graph_submit 6.55 |
| warm-up | 5.28 | 28.9 | pipelines/compile_ms 211, pipelines/front_ms 202, render/thread/ms 52.13, render/busy/ms 50.80, main/busy/ms 31.66, stage/render/prepare_assets 20.33, stage/render/render 15.76, stage/main/update 14.17, render/graph/ms 13.67, stage/main/post_update 11.93, encode//sum 10.50, stage/render/graph_submit 9.57 |
| warm-up | 5.35 | 42.6 | pipelines/compile_ms 311, pipelines/front_ms 300, render/thread/ms 54.76, render/busy/ms 53.37, main/busy/ms 34.54, stage/render/prepare_assets 25.94, stage/render/render 15.48, stage/main/update 14.54, render/graph/ms 13.55, stage/main/post_update 11.97, pipelines/driver_ms 10.62, encode//sum 9.29 |
| warm-up | 5.40 | 27.2 | pipelines/compile_ms 703, pipelines/front_ms 679, render/thread/ms 44.17, render/busy/ms 42.37, main/busy/ms 37.64, pipelines/driver_ms 23.86, stage/render/render 19.93, render/graph/ms 17.50, stage/main/post_update 16.61, stage/main/update 15.59, stage/render/graph_submit 12.23, encode//sum 11.59 |
| warm-up | 5.46 | 33.5 | pipelines/compile_ms 192, pipelines/front_ms 187, render/thread/ms 46.61, render/busy/ms 45.19, main/busy/ms 33.66, stage/render/prepare_assets 22.11, stage/main/update 15.10, stage/render/render 12.99, stage/main/post_update 12.96, encode//sum 11.15, render/graph/ms 10.82, stream/drive/ms 6.99 |
| warm-up | 5.48 | 28.7 | pipelines/compile_ms 191, pipelines/front_ms 185, render/thread/ms 52.19, render/busy/ms 50.91, main/busy/ms 29.84, stage/render/prepare_assets 29.27, stage/main/update 13.07, stage/render/render 12.21, stage/main/post_update 11.22, encode//sum 10.33, render/graph/ms 10.26, stage/render/graph_submit 6.99 |
| warm-up | 5.54 | 32.0 | pipelines/compile_ms 177, pipelines/front_ms 174, render/thread/ms 45.58, render/busy/ms 44.11, main/busy/ms 38.70, stage/main/update 19.24, stage/render/prepare_assets 16.68, stage/render/render 15.06, stage/main/post_update 13.56, render/graph/ms 13.12, stream/drive/ms 11.52, encode//sum 10.97 |
| warm-up | 5.57 | 32.6 | pipelines/compile_ms 535, pipelines/front_ms 521, render/thread/ms 57.65, render/busy/ms 56.14, main/busy/ms 44.22, stage/render/prepare_assets 24.11, stage/main/update 23.55, stage/render/render 15.46, stage/main/post_update 14.79, pipelines/driver_ms 13.95, render/graph/ms 13.09, encode//sum 11.81 |
| warm-up | 5.60 | 30.0 | pipelines/compile_ms 709, pipelines/front_ms 688, render/thread/ms 57.86, render/busy/ms 56.07, main/busy/ms 34.98, stage/render/prepare_assets 24.40, pipelines/driver_ms 21.96, stage/render/render 16.60, stage/main/update 14.52, stage/main/post_update 14.35, render/graph/ms 13.78, encode//sum 10.13 |
| warm-up | 5.64 | 38.0 | pipelines/compile_ms 442, pipelines/front_ms 426, render/thread/ms 57.09, render/busy/ms 55.38, main/busy/ms 34.11, stage/render/prepare_assets 31.25, pipelines/driver_ms 16.59, stage/main/update 16.16, stage/render/render 14.30, render/graph/ms 11.90, stage/main/post_update 11.30, encode//sum 9.06 |
| warm-up | 5.67 | 36.8 | pipelines/compile_ms 415, pipelines/front_ms 393, render/thread/ms 62.34, render/busy/ms 60.99, main/busy/ms 43.09, stage/render/prepare_assets 39.47, pipelines/driver_ms 22.09, stage/main/post_update 21.18, stage/main/update 15.64, stage/render/render 12.51, render/graph/ms 10.60, encode//sum 9.62 |
| warm-up | 5.93 | 32.5 | main/busy/ms 40.25, render/thread/ms 28.75, render/busy/ms 27.50, stage/main/update 22.40, stream/drive/ms 17.49, stage/render/render 14.24, render/graph/ms 12.60, stage/main/post_update 12.16, encode//sum 10.24, stage/render/graph_submit 8.90, stage/render/prepare_assets 6.48, encode/submit 6.28 |
| warm-up | 6.16 | 27.3 | pipelines/compile_ms 146, pipelines/front_ms 143, main/busy/ms 30.20, render/thread/ms 21.91, render/busy/ms 20.83, stage/main/update 19.06, stream/drive/ms 12.95, stage/render/render 10.18, render/graph/ms 8.64, encode//sum 8.60, stage/main/post_update 7.28, stage/render/graph_submit 5.83 |
| warm-up | 6.29 | 27.4 | render/thread/ms 41.32, render/busy/ms 40.29, main/busy/ms 28.76, stage/render/prepare_assets 16.74, stage/main/post_update 10.84, stage/main/update 10.65, stage/render/render 10.39, encode//sum 8.97, render/graph/ms 8.92, stage/render/graph_submit 6.15, encode/pass:shadow_point_light/sum 5.62, stream/drive/ms 4.18 |
| warm-up | 6.49 | 27.7 | main/busy/ms 39.26, render/thread/ms 38.91, render/busy/ms 37.79, stage/main/update 19.00, stage/main/post_update 13.55, stage/render/render 12.56, render/graph/ms 10.64, encode//sum 9.79, stage/render/prepare_assets 7.31, stage/render/graph_submit 7.08, encode/pass:shadow_point_light/sum 5.80, stream/drive/ms 5.16 |
| warm-up | 6.52 | 31.0 | render/thread/ms 48.98, render/busy/ms 47.71, main/busy/ms 47.09, stage/main/update 20.61, stage/main/post_update 18.75, stage/render/render 18.04, render/graph/ms 16.27, encode//sum 13.94, stage/render/prepare_assets 11.64, stage/render/graph_submit 11.55, encode/submit 8.09, encode/pass:shadow_point_light/sum 6.89 |
| warm-up | 6.63 | 42.0 | main/busy/ms 48.70, render/thread/ms 39.05, render/busy/ms 37.70, stage/main/update 26.93, stream/drive/ms 15.80, stage/render/render 15.80, stage/main/post_update 15.70, render/graph/ms 13.57, encode//sum 10.70, stage/render/graph_submit 9.01, stage/render/prepare_assets 8.20, pipelines/compile_ms 8.00 |
| warm-up | 6.66 | 29.2 | render/thread/ms 45.73, render/busy/ms 44.53, main/busy/ms 40.54, stage/main/update 18.78, stage/render/render 15.32, stage/main/post_update 15.11, render/graph/ms 13.39, stage/render/prepare_assets 13.04, encode//sum 10.37, stage/render/graph_submit 8.05, encode/pass:shadow_point_light/sum 6.05, stage/render/camera_driver 5.20 |
| warm-up | 6.69 | 27.3 | render/thread/ms 43.63, render/busy/ms 42.61, main/busy/ms 37.46, stage/main/update 20.44, stage/render/prepare_assets 17.94, stage/render/render 12.81, stage/main/post_update 11.71, render/graph/ms 11.18, encode//sum 9.91, stage/render/graph_submit 7.25, encode/pass:shadow_point_light/sum 5.95, encode/submit 4.46 |
| warm-up | 6.79 | 27.9 | pipelines/compile_ms 188, pipelines/front_ms 184, render/thread/ms 39.58, render/busy/ms 38.32, main/busy/ms 23.11, stage/render/prepare_assets 16.12, stage/render/render 12.73, render/graph/ms 10.97, stage/main/post_update 10.69, encode//sum 9.91, stage/main/update 9.23, stage/render/graph_submit 7.53 |
| warm-up | 6.84 | 28.6 | render/thread/ms 40.05, render/busy/ms 38.88, main/busy/ms 27.70, stage/render/prepare_assets 16.54, stage/main/update 13.28, stage/render/render 13.20, render/graph/ms 11.43, encode//sum 10.80, stage/main/post_update 10.34, stage/render/graph_submit 8.04, stream/drive/ms 6.83, encode/pass:shadow_point_light/sum 6.45 |
| warm-up | 7.07 | 34.8 | main/busy/ms 43.54, render/thread/ms 32.43, render/busy/ms 31.35, stage/main/update 24.78, stream/drive/ms 15.84, stage/render/prepare_assets 13.56, stage/main/post_update 11.80, stage/render/render 11.39, render/graph/ms 9.88, encode//sum 9.21, stage/render/graph_submit 6.69, encode/pass:shadow_point_light/sum 5.68 |
| measure | 0.78 | 29.3 | main/busy/ms 35.88, render/thread/ms 29.25, render/busy/ms 28.17, stage/main/update 18.03, stage/render/render 15.31, stage/main/post_update 14.24, render/graph/ms 13.82, encode//sum 13.74, stream/drive/ms 13.13, stage/render/graph_submit 9.67, encode/pass:shadow_point_light/sum 7.90, stage/render/prepare_assets 6.03 |
| measure | 3.27 | 33.6 | render/thread/ms 45.29, encode//sum 44.55, render/busy/ms 44.31, main/busy/ms 40.89, stage/render/render 33.14, stage/main/post_update 32.65, render/graph/ms 31.62, stage/render/graph_submit 27.30, encode/finish_wall 19.73, encode/pass:early prepass/sum 16.80, encode/pass:shadow_point_light/sum 8.79, encode/submit 7.61 |
| measure | 18.19 | 30.6 | main/busy/ms 39.13, render/thread/ms 24.48, stage/main/update 24.25, render/busy/ms 23.40, stream/drive/ms 20.26, stage/render/render 14.92, render/graph/ms 13.40, encode//sum 12.30, stage/main/post_update 12.22, stage/render/graph_submit 9.79, encode/pass:shadow_point_light/sum 8.50, encode/submit 6.56 |
| measure | 59.05 | 26.9 | render/thread/ms 36.45, render/busy/ms 34.93, main/busy/ms 26.19, stage/main/post_update 14.96, stage/render/render 14.19, render/graph/ms 12.19, stage/render/prepare_assets 12.07, encode//sum 11.07, stage/render/graph_submit 9.00, stage/main/update 6.25, encode/submit 5.79, encode/finish_wall 3.33 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.sun.gate=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=sungate-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1280 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **71%** (min 53%) · 2625 MHz (min 2625) · memory 8101 MHz · 75.0 W · 76 °C (max 82) · VRAM 6754 MiB · throttled 100% of samples

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
- `shadow.sun.gate=on (command line: --set)`
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
