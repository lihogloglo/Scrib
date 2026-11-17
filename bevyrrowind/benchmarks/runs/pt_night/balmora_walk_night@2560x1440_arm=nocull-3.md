# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 5930 frames over 65.6 s · **median 10.57 ms (95 fps)** · mean 11.07 · p95 15.26 · p99 17.36 (1% low 58 fps) · worst 28.93 ms · 220 spikes over 1.5× median · 25.6 stutters/min (work 28.3/min)
- warm-up: 323 frames over 9.6 s, worst 2496.7 ms
- hitches (> 26.4 ms): **3** measured, 58 in warm-up
- timed GPU passes: 6.59 ms of a 11.07 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.57 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- pass worst frame 28.93 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.976 | 2.030 | 2.063 | 0.97 | 0.174 |
| main_opaque_pass_3d | 1.270 | 1.305 | 1.649 | 0.97 | 0.063 |
| early prepass | 0.473 | 0.486 | 0.701 | 0.97 | 0.087 |
| bloom | 0.308 | 0.316 | 0.322 | 0.97 | 0.064 |
| atmosphere_luts | 0.301 | 0.309 | 0.372 | 0.97 | 0.003 |
| auxiliary_views | 0.255 | 0.262 | 1.118 | 0.97 | 0.248 |
| water_velocity | 0.254 | 0.261 | 0.349 | 0.97 | 0.002 |
| water_sspr | 0.196 | 0.202 | 0.203 | 0.97 | 0.002 |
| ssao | 0.183 | 0.188 | 0.193 | 0.97 | 0.007 |
| uniform_allocation | 0.143 | 0.147 | 0.177 | 0.97 | 0.025 |
| water_fft | 0.125 | 0.129 | 0.137 | 0.97 | 0.003 |
| upscaling | 0.125 | 0.128 | 0.136 | 0.97 | 0.004 |
| atmos_froxel_inject | 0.124 | 0.127 | 0.128 | 0.97 | 0.001 |
| clouds_composite | 0.102 | 0.104 | 0.108 | 0.97 | 0.002 |
| bin_unpacking | 0.100 | 0.103 | 0.125 | 0.97 | 0.017 |
| tonemapping | 0.085 | 0.087 | 0.092 | 0.97 | 0.003 |
| main_transmissive_pass_3d | 0.076 | 0.078 | 0.154 | 0.97 | 0.009 |
| clustering | 0.062 | 0.063 | 0.066 | 0.97 | 0.019 |
| clouds_trace | 0.054 | 0.056 | 0.072 | 0.97 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.97 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.97 | 0.001 |
| lightprobe_irradiance_map | 0.041 | 0.411 | 0.416 | 0.10 | 0.001 |
| surface_trail | 0.039 | 0.056 | 0.057 | 0.70 | 0.002 |
| early_mesh_preprocessing | 0.035 | 0.036 | 0.045 | 0.97 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.97 | 0.001 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.97 | 0.002 |
| lightprobe_radiance_map | 0.022 | 0.219 | 0.228 | 0.10 | 0.004 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.97 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.033 | 0.97 | 0.001 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.028 | 0.97 | 0.175 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.97 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.97 | 0.005 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.97 | 0.003 |
| lightprobe_copy | 0.001 | 0.012 | 0.013 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.012 | 0.013 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.665 | 13.598 | 25.72 | 57315.3 |
| render/busy/ms | 9.143 | 13.035 | 25.02 | 54219.3 |
| main/busy/ms | 8.723 | 10.823 | 28.48 | 51725.5 |
| stage/render/render | 5.716 | 8.726 | 16.68 | 33897.5 |
| stage/main/post_update | 5.305 | 7.016 | 9.68 | 31459.2 |
| render/graph/ms | 4.981 | 7.924 | 15.60 | 29535.1 |
| encode//sum | 4.402 | 7.814 | 31.33 | 26103.3 |
| stage/render/graph_submit | 3.635 | 5.743 | 11.10 | 21557.6 |
| encode/submit | 2.331 | 3.776 | 6.76 | 13822.6 |
| stage/main/update | 2.160 | 2.613 | 22.12 | 12810.3 |
| encode/finish_wall | 1.349 | 2.004 | 5.22 | 7998.7 |
| stage/render/camera_driver | 1.291 | 2.194 | 4.48 | 7656.9 |
| encode/pass:shadow_point_light/sum | 1.279 | 5.085 | 15.80 | 7583.7 |
| encode/pass:shadow_directional_light/sum | 0.737 | 1.224 | 2.08 | 4372.9 |
| stage/render/prepare_write_phase_buffers | 0.737 | 1.142 | 2.32 | 4367.8 |
| encode/pass:main_opaque_pass_3d/sum | 0.712 | 1.040 | 1.63 | 4221.2 |
| encode/pass:early prepass/sum | 0.707 | 1.040 | 1.98 | 4190.7 |
| stage/render/prepare_assets | 0.575 | 0.733 | 9.51 | 3408.4 |
| bigspace/total/ms | 0.513 | 1.214 | 2.65 | 3040.9 |
| stage/render/prepare_meshes | 0.480 | 0.657 | 2.22 | 2848.1 |
| stage/render/prepare_resources | 0.447 | 0.539 | 1.13 | 2653.4 |
| render/present/ms | 0.427 | 0.518 | 2.72 | 2530.6 |
| stage/main/first | 0.393 | 0.518 | 5.87 | 2331.9 |
| stage/main/pre_update | 0.352 | 0.502 | 1.38 | 2086.0 |
| terrain/update/ms | 0.312 | 0.398 | 2.17 | 1851.0 |
| encode/pass:main_transparent_pass_3d/sum | 0.297 | 0.438 | 0.93 | 1761.3 |
| stage/render/specialize | 0.290 | 0.443 | 4.13 | 1718.4 |
| terrain/select/ms | 0.278 | 0.357 | 0.97 | 1651.3 |
| stage/render/queue | 0.267 | 0.422 | 2.87 | 1586.2 |
| stage/render/prepare_batch_phases | 0.267 | 0.378 | 0.71 | 1583.3 |
| stage/render/prepare_bind_groups | 0.265 | 0.470 | 2.51 | 1573.4 |
| controller/feed/ms | 0.262 | 0.331 | 0.69 | 1553.7 |
| stream/drive/ms | 0.242 | 0.316 | 19.98 | 1432.7 |
| stage/main/fixed_main_loop | 0.216 | 0.362 | 1.08 | 1283.0 |
| bigspace/low_precision/ms | 0.208 | 0.573 | 2.26 | 1236.1 |
| far/select/ms | 0.197 | 0.262 | 0.76 | 1170.4 |
| stage/render/create_views | 0.193 | 0.248 | 0.95 | 1145.0 |
| stream/tick/ms | 0.186 | 0.244 | 0.58 | 1102.5 |
| stage/render/prepare_flush | 0.172 | 0.341 | 0.81 | 1022.3 |
| stage/render/extract_commands | 0.168 | 0.196 | 0.50 | 994.7 |
| anim/evaluate/ms | 0.160 | 0.277 | 2.31 | 948.9 |
| anim/evaluate_cpu/ms | 0.153 | 0.189 | 0.76 | 909.9 |
| controller/move/ms | 0.141 | 0.291 | 0.64 | 838.1 |
| bigspace/high_precision/ms | 0.141 | 0.563 | 1.95 | 837.1 |
| stage/render/prepare_views | 0.138 | 0.172 | 0.53 | 818.1 |
| stage/main/last | 0.118 | 0.163 | 1.70 | 700.7 |
| encode/pass:main_transmissive_pass_3d/sum | 0.100 | 0.163 | 0.69 | 592.5 |
| render/acquire/ms | 0.095 | 0.121 | 0.35 | 565.4 |
| stage/render/cleanup | 0.090 | 0.220 | 0.61 | 534.9 |
| stage/render/phase_sort | 0.067 | 0.095 | 0.45 | 399.4 |
| stage/render/prepare_collect_phase_buffers | 0.061 | 0.133 | 0.48 | 361.4 |
| physics/step/ms | 0.047 | 0.086 | 0.55 | 281.5 |
| physics/insert/ms | 0.035 | 0.373 | 0.95 | 208.1 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.49 | 178.5 |
| weather/map/ms | 0.016 | 0.020 | 0.05 | 94.6 |
| pipelines/compile_ms | 0.016 | 0.000 | 81.81 | 93.3 |
| pipelines/front_ms | 0.015 | 0.000 | 79.06 | 90.3 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.022 | 0.11 | 85.1 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.31 | 63.7 |
| controller/avoid/ms | 0.010 | 0.014 | 0.16 | 61.9 |
| anim/apply/ms | 0.010 | 0.018 | 0.17 | 61.3 |
| weather/eval/ms | 0.010 | 0.013 | 0.24 | 57.0 |
| script/update/ms | 0.007 | 0.009 | 0.33 | 41.1 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 30.4 |
| stream/despawn/ms | 0.005 | 0.000 | 0.96 | 29.2 |
| script/ui/ms | 0.005 | 0.006 | 0.30 | 28.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.12 | 28.2 |
| controller/write/ms | 0.004 | 0.004 | 0.03 | 20.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.60 | 13.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.14 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 4.8 |
| stage/render/prepare | 0.001 | 0.003 | 0.24 | 3.2 |
| pipelines/driver_ms | 0.001 | 0.000 | 2.75 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.3 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1258 | 2262 | 658 |
| crowd/c0/meshes/count | count | 343 | 646 | 188 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5894 | 5538 |
| crowd/c1/meshes/count | count | 1154 | 1254 | 1153 |
| crowd/c2/count | count | 398 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 163 | 215 | 204 |
| crowd/joints_extracted/count | count | 3671 | 7862 | 5731 |
| crowd/joints_uploaded/count | count | 3634 | 6186 | 5731 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.42 | 1.78 | 0.44 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.11 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.31 | 0.02 |
| encode/finish_critical | ms | 0.92 | 2.08 | 1.03 |
| encode/pass:early prepass/max | ms | 0.73 | 1.98 | 0.76 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.73 | 1.63 | 1.03 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.69 | 0.17 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.28 | 0.93 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.74 | 2.08 | 0.61 |
| encode/pass:shadow_point_light/max | ms | 0.77 | 1.38 | 0.92 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.49 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.97 | 28.48 | 7.75 |
| gpu/timed/ms | ms | 7.48 | 10.78 | 7.75 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 253 | 478 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3947 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 253 | 478 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3947 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 4119 | 4335 | 4335 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 4119 | 4335 | 4335 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 20 | 215 | 5 |
| phases/shadow/bins | count | 10.4 k | 11.7 k | 7980 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 27.2 k | 29.2 k | 25.7 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 196 | 236 | 20 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 194 | 476 | 19 |
| physics/pending | count | 0 | 181 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 16 | 596 | 6 |
| preprocess/bind_groups_created | count | 16 | 596 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 135 | 308 | 129 |
| process/cpu_usage | % | 25.11 | 37.50 | 21.63 |
| process/mem_usage | GiB | 5.49 | 5.66 | 5.65 |
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
| skin/blocks_changed/count | count | 13 | 92 | 11 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 163 | 215 | 204 |
| skin/joint_bytes_written/bytes | bytes | 280.3 k | 810.6 k | 441.9 k |
| skin/joints_written/count | count | 4380 | 12.7 k | 6904 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4920 | 5011 | 5011 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 432 | 528 | 396 |
| statics/materials_bound | count | 432 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.41 | 45.20 | 29.80 |
| system/mem_usage | % | 54.56 | 55.01 | 54.96 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 71 | 266 | 57 |
| views/all/bins | count | 19.0 k | 20.4 k | 17.6 k |
| views/all/count | count | 45 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 55.0 k | 57.6 k | 55.2 k |
| views/all/pipelines | count | 245 | 284 | 68 |
| views/all/pipelines_distinct | count | 71 | 76 | 63 |
| views/all/work_items | count | 41.5 k | 56.1 k | 42.2 k |
| views/cascade/batch_sets | count | 9 | 28 | 5 |
| views/cascade/bins | count | 7670 | 7981 | 7980 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 24.5 k | 25.7 k | 25.7 k |
| views/cascade/pipelines | count | 24 | 28 | 20 |
| views/cascade/work_items | count | 11.7 k | 25.7 k | 11.9 k |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 3454 | 4400 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 3660 | 4716 | 0 |
| views/lamp_face/pipelines | count | 171 | 208 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 4043 | 0 |
| views/main/batch_sets | count | 51 | 55 | 52 |
| views/main/bins | count | 8728 | 9612 | 9612 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.1 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 48 | 51 | 48 |
| views/main/work_items | count | 28.6 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 45.42 | 216 | 269.3 k |
| anim/events/count | count | 0.09 | 2 | 528 |
| assets/Image/added | count | 0.10 | 40 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.68 | 152 | 4061 |
| assets/Mesh/removed | count | 0.82 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.65 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.79 | 1296 | 4676 |
| encode//count | count | 37.60 | 219 | 223.0 k |
| encode/bind_walk/init_textures | count | 14.7 k | 115.1 k | 87.41 M |
| encode/bind_walk/init_textures_skipped | count | 56.5 k | 125.7 k | 334.82 M |
| encode/bind_walk/inits | count | 531 | 1249 | 3.15 M |
| encode/bind_walk/inits_skipped | count | 126 | 202 | 749.4 k |
| encode/bind_walk/merge_textures | count | 14.7 k | 115.1 k | 87.04 M |
| encode/bind_walk/merge_textures_skipped | count | 5533 | 6026 | 32.81 M |
| encode/bind_walk/merges | count | 107 | 352 | 634.5 k |
| encode/bind_walk/merges_skipped | count | 106 | 150 | 627.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5930 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5930 |
| encode/pass:early prepass/count | count | 1 | 1 | 5930 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 5930 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 5930 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 5930 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 5930 |
| encode/pass:shadow_point_light/count | count | 2.66 | 48 | 15.8 k |
| encode/queue/command_buffers | count | 50.27 | 277 | 298.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.0 k |
| encode/queue/write_buffer/bytes | bytes | 2.25 M | 4.50 M | 13.34 G |
| encode/queue/write_buffer/calls | count | 272 | 889 | 1.61 M |
| encode/queue/write_buffer_with/bytes | bytes | 170.7 k | 3.53 M | 1.01 G |
| encode/queue/write_buffer_with/calls | count | 11.04 | 465 | 65.5 k |
| encode/queue/write_texture/bytes | bytes | 10.6 k | 8.39 M | 63.07 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 563 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5930 |
| physics/inserts | count | 0.38 | 105 | 2261 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.39 | 145 | 2335 |
| physics/steps | count | 0.66 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 2 |
| precache/hit | count | 0.00 | 1 | 3 |
| precache/missed | count | 0.00 | 1 | 2 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 19 | 378 |
| stream/upload/bytes | bytes | 5709 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1252 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.4 ms; slowest 61, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.50 | 2496.7 | main/busy/ms 2466, stage/main/post_update 2424, stage/main/update 32.60, stage/main/pre_update 5.27, stage/main/last 1.31, surface/advance_cpu_ms 0.74, stream/drive/ms 0.61, stage/main/first 0.57, stage/main/fixed_main_loop 0.35, surface/cell_upload_ms 0.32, stream/tick/ms 0.16, bigspace/total/ms 0.08 |
| warm-up | 3.34 | 838.8 | main/busy/ms 2611, stage/main/post_update 2451, render/thread/ms 804, render/busy/ms 632, stage/render/prepare_assets 580, stage/render/prepare_views 168, render/acquire/ms 168, stage/main/update 145, stage/render/create_views 31.02, stage/render/render 12.99, render/graph/ms 8.25, stage/main/pre_update 7.33 |
| warm-up | 3.44 | 108.2 | render/thread/ms 895, render/busy/ms 673, stage/render/prepare_assets 607, stage/render/prepare_views 169, render/acquire/ms 168, main/busy/ms 165, stage/main/update 117, pipelines/compile_ms 102, stage/render/render 69.08, pipelines/front_ms 63.69, render/present/ms 54.22, pipelines/driver_ms 38.31 |
| warm-up | 3.49 | 45.0 | pipelines/compile_ms 466, pipelines/front_ms 388, pipelines/wait_ms 169, render/thread/ms 135, stage/render/render 91.63, render/busy/ms 78.55, pipelines/driver_ms 77.63, main/busy/ms 63.93, render/present/ms 55.61, stage/main/post_update 48.04, render/graph/ms 34.82, pipelines/lock_ms 31.02 |
| warm-up | 3.57 | 80.4 | pipelines/wait_ms 2009, pipelines/compile_ms 753, pipelines/front_ms 670, render/thread/ms 120, main/busy/ms 118, render/busy/ms 114, stage/main/post_update 108, pipelines/driver_ms 82.87, stage/render/render 45.70, render/graph/ms 39.02, stage/render/prepare_bind_groups 32.80, pipelines/lock_ms 30.35 |
| warm-up | 3.66 | 89.1 | pipelines/wait_ms 5958, pipelines/compile_ms 1577, pipelines/front_ms 1446, main/busy/ms 165, render/thread/ms 160, render/busy/ms 157, pipelines/driver_ms 131, stage/main/post_update 81.25, stage/main/update 65.39, encode//sum 62.08, stage/render/prepare_bind_groups 32.72, stage/render/prepare_assets 28.75 |
| warm-up | 3.78 | 117.1 | pipelines/wait_ms 4556, pipelines/compile_ms 2723, pipelines/front_ms 2381, pipelines/driver_ms 342, render/thread/ms 188, render/busy/ms 185, encode//sum 142, main/busy/ms 106, stage/main/update 69.58, stage/render/render 65.86, render/graph/ms 63.11, stage/render/graph_submit 59.62 |
| warm-up | 3.82 | 40.1 | pipelines/compile_ms 2308, pipelines/front_ms 1976, pipelines/wait_ms 522, pipelines/driver_ms 333, render/thread/ms 142, render/busy/ms 141, encode//sum 95.86, stage/render/prepare_assets 69.73, stage/render/render 65.16, render/graph/ms 62.92, stage/render/graph_submit 60.87, encode/finish_wall 47.32 |
| warm-up | 3.89 | 29.7 | pipelines/compile_ms 80.99, pipelines/front_ms 75.57, render/thread/ms 42.13, render/busy/ms 40.63, main/busy/ms 27.79, stage/render/prepare_assets 18.95, stage/main/post_update 15.18, stage/render/render 11.81, render/graph/ms 9.07, terrain/update/ms 8.37, stage/main/update 7.34, encode//sum 7.27 |
| warm-up | 3.98 | 80.6 | pipelines/compile_ms 1080, pipelines/front_ms 1044, main/busy/ms 54.86, render/thread/ms 48.57, render/busy/ms 46.81, stage/main/update 42.43, stream/drive/ms 38.04, pipelines/driver_ms 36.01, stage/render/prepare_assets 29.75, encode//sum 13.58, stage/render/render 12.10, render/graph/ms 10.19 |
| warm-up | 4.13 | 79.8 | pipelines/compile_ms 214, pipelines/front_ms 208, render/thread/ms 57.29, render/busy/ms 56.03, main/busy/ms 48.66, stage/render/prepare_assets 38.70, stage/main/update 35.31, stream/drive/ms 31.05, stage/render/render 11.30, render/graph/ms 9.66, stage/render/graph_submit 7.04, stage/main/post_update 6.94 |
| warm-up | 4.23 | 81.2 | pipelines/compile_ms 819, pipelines/front_ms 797, render/thread/ms 67.87, render/busy/ms 66.01, main/busy/ms 63.91, stage/render/prepare_assets 44.64, stage/main/update 42.56, pipelines/driver_ms 22.63, stage/main/post_update 13.45, stage/render/render 13.21, render/graph/ms 10.76, stage/render/graph_submit 7.52 |
| warm-up | 4.34 | 104.0 | pipelines/compile_ms 1432, pipelines/front_ms 1374, render/thread/ms 132, render/busy/ms 129, stage/render/prepare_assets 81.97, main/busy/ms 67.04, pipelines/driver_ms 58.31, stage/main/update 42.77, stage/render/render 40.54, render/graph/ms 37.21, pipelines/wait_ms 23.41, stage/render/graph_submit 18.00 |
| warm-up | 4.43 | 96.2 | pipelines/compile_ms 1338, pipelines/front_ms 1277, pipelines/wait_ms 273, render/thread/ms 150, render/busy/ms 147, pipelines/driver_ms 60.82, stage/render/prepare_assets 48.27, stage/render/render 41.41, render/graph/ms 38.00, main/busy/ms 32.68, stage/render/graph_submit 18.27, encode/submit 16.37 |
| warm-up | 4.55 | 119.3 | pipelines/compile_ms 2664, pipelines/front_ms 2581, pipelines/wait_ms 1063, render/thread/ms 174, render/busy/ms 172, pipelines/driver_ms 82.72, encode//sum 69.85, main/busy/ms 55.04, stage/render/prepare_assets 50.87, stage/main/post_update 30.78, stage/render/render 29.94, render/graph/ms 27.27 |
| warm-up | 4.64 | 90.1 | pipelines/compile_ms 2848, pipelines/front_ms 2760, pipelines/wait_ms 1244, render/thread/ms 194, render/busy/ms 192, encode//sum 113, main/busy/ms 106, pipelines/driver_ms 87.98, stage/render/prepare_assets 76.97, stage/main/post_update 72.82, stage/render/render 41.59, render/graph/ms 38.63 |
| warm-up | 4.72 | 74.2 | pipelines/compile_ms 1590, pipelines/front_ms 1535, pipelines/wait_ms 809, render/thread/ms 139, render/busy/ms 137, main/busy/ms 95.51, stage/render/prepare_assets 64.55, encode//sum 57.15, stage/main/post_update 56.02, pipelines/driver_ms 54.68, stage/render/render 28.34, stage/main/update 26.54 |
| warm-up | 4.78 | 62.1 | pipelines/compile_ms 1771, pipelines/front_ms 1721, pipelines/wait_ms 547, main/busy/ms 106, render/thread/ms 95.09, render/busy/ms 93.53, stage/main/update 67.16, stream/drive/ms 52.85, pipelines/driver_ms 49.71, stage/render/prepare_assets 48.58, stage/main/post_update 23.49, stage/render/render 22.67 |
| warm-up | 4.85 | 72.5 | pipelines/compile_ms 1949, pipelines/front_ms 1894, pipelines/wait_ms 257, main/busy/ms 99.49, render/thread/ms 79.57, render/busy/ms 77.66, stage/main/update 64.01, pipelines/driver_ms 55.38, stream/drive/ms 52.28, stage/render/prepare_assets 23.86, stage/main/post_update 23.62, stage/render/render 18.61 |
| warm-up | 4.89 | 35.8 | pipelines/compile_ms 917, pipelines/front_ms 884, main/busy/ms 119, pipelines/wait_ms 88.83, stage/main/update 87.21, render/thread/ms 67.36, render/busy/ms 65.59, stream/drive/ms 48.64, pipelines/driver_ms 32.71, stage/render/prepare_assets 20.05, stage/main/post_update 15.49, stage/render/prepare_resources 13.21 |
| warm-up | 5.03 | 148.2 | pipelines/compile_ms 2329, pipelines/front_ms 2268, main/busy/ms 124, render/thread/ms 93.11, render/busy/ms 91.64, stage/main/update 90.74, pipelines/driver_ms 60.43, stream/drive/ms 48.90, stage/render/prepare_assets 46.00, stage/render/render 24.13, render/graph/ms 21.95, pipelines/wait_ms 18.81 |
| warm-up | 5.08 | 46.5 | pipelines/compile_ms 2547, pipelines/front_ms 2480, render/thread/ms 103, render/busy/ms 101, main/busy/ms 92.29, pipelines/driver_ms 67.16, stage/main/update 65.61, stream/drive/ms 49.11, stage/render/prepare_assets 41.09, stage/render/render 36.26, render/graph/ms 33.90, pipelines/wait_ms 20.68 |
| warm-up | 5.15 | 65.1 | pipelines/compile_ms 566, pipelines/front_ms 545, main/busy/ms 88.23, render/thread/ms 74.22, render/busy/ms 72.53, stage/main/update 70.09, stream/drive/ms 49.44, stage/render/render 26.26, render/graph/ms 23.94, stage/render/prepare_assets 22.50, pipelines/driver_ms 20.64, stage/render/graph_submit 15.03 |
| warm-up | 5.19 | 43.6 | pipelines/compile_ms 646, pipelines/front_ms 623, main/busy/ms 70.14, render/thread/ms 67.05, render/busy/ms 64.66, stage/main/update 53.32, stream/drive/ms 35.83, stage/render/prepare_assets 25.00, pipelines/driver_ms 23.19, stage/render/render 16.21, render/graph/ms 14.08, encode//sum 10.10 |
| warm-up | 5.23 | 42.5 | pipelines/compile_ms 556, pipelines/front_ms 536, main/busy/ms 62.61, render/thread/ms 60.36, render/busy/ms 57.94, stage/main/update 45.96, stream/drive/ms 35.43, pipelines/driver_ms 20.32, stage/render/prepare_assets 16.21, stage/render/render 16.10, render/graph/ms 14.02, stage/render/graph_submit 10.17 |
| warm-up | 5.27 | 34.8 | pipelines/compile_ms 574, pipelines/front_ms 555, render/thread/ms 55.61, render/busy/ms 54.07, main/busy/ms 37.13, pipelines/driver_ms 18.42, stage/main/update 17.32, stage/render/prepare_assets 15.62, stage/render/render 13.98, stage/main/post_update 13.07, render/graph/ms 11.92, stage/render/specialize 9.82 |
| warm-up | 5.29 | 28.3 | pipelines/compile_ms 373, pipelines/front_ms 363, render/thread/ms 54.16, render/busy/ms 52.59, main/busy/ms 34.72, stage/render/prepare_assets 15.74, stage/render/render 14.85, stage/main/post_update 14.59, stage/main/update 14.08, render/graph/ms 12.60, pipelines/driver_ms 10.13, encode//sum 10.08 |
| warm-up | 5.33 | 33.5 | render/thread/ms 54.20, render/busy/ms 52.50, main/busy/ms 39.07, stage/main/update 20.21, stage/render/render 16.74, render/graph/ms 14.65, stage/main/post_update 13.20, stage/render/prepare_assets 11.02, stage/render/graph_submit 10.58, encode//sum 10.20, encode/submit 8.29, stage/render/prepare_write_phase_buffers 7.77 |
| warm-up | 5.38 | 48.3 | pipelines/compile_ms 963, pipelines/front_ms 938, render/thread/ms 72.14, render/busy/ms 70.40, main/busy/ms 38.59, stage/render/render 30.22, render/graph/ms 27.98, pipelines/driver_ms 24.88, stage/main/update 20.63, stage/render/prepare_assets 14.94, stage/render/camera_driver 14.10, stage/render/graph_submit 13.68 |
| warm-up | 5.45 | 50.0 | pipelines/compile_ms 991, pipelines/front_ms 958, render/thread/ms 60.19, render/busy/ms 58.77, main/busy/ms 42.60, pipelines/driver_ms 32.81, stage/render/prepare_assets 23.16, stage/main/update 21.33, stage/render/render 18.21, render/graph/ms 16.13, stage/main/post_update 13.41, stream/drive/ms 11.70 |
| warm-up | 5.51 | 31.6 | pipelines/compile_ms 207, pipelines/front_ms 202, render/thread/ms 49.56, render/busy/ms 47.45, main/busy/ms 34.34, stage/render/render 21.81, render/graph/ms 19.08, stage/main/update 15.11, stage/render/graph_submit 13.69, stage/main/post_update 13.37, encode//sum 11.00, encode/submit 10.67 |
| warm-up | 5.53 | 26.5 | pipelines/compile_ms 211, pipelines/front_ms 206, render/thread/ms 52.69, render/busy/ms 50.72, main/busy/ms 35.48, stage/render/render 19.66, render/graph/ms 16.89, stage/main/update 15.67, stage/render/prepare_assets 14.27, stage/main/post_update 13.57, stage/render/graph_submit 11.62, encode//sum 10.87 |
| warm-up | 5.57 | 39.4 | pipelines/compile_ms 866, pipelines/front_ms 848, render/thread/ms 52.70, render/busy/ms 51.25, main/busy/ms 34.38, stage/render/prepare_assets 24.48, pipelines/driver_ms 18.02, stage/main/update 16.23, stage/render/render 13.20, stage/main/post_update 12.38, render/graph/ms 10.86, encode//sum 10.16 |
| warm-up | 5.61 | 33.2 | pipelines/compile_ms 900, pipelines/front_ms 880, render/thread/ms 59.66, render/busy/ms 58.15, stage/render/prepare_assets 30.29, main/busy/ms 28.55, pipelines/driver_ms 19.74, stage/render/render 15.90, render/graph/ms 13.55, stage/main/update 12.70, encode//sum 11.84, stage/main/post_update 10.36 |
| warm-up | 5.64 | 30.3 | pipelines/compile_ms 140, pipelines/front_ms 133, render/thread/ms 58.99, render/busy/ms 57.38, main/busy/ms 31.36, stage/render/prepare_assets 21.88, stage/render/render 17.50, render/graph/ms 15.34, stage/main/update 12.70, stage/main/post_update 12.52, encode//sum 11.49, stage/render/graph_submit 11.00 |
| warm-up | 5.68 | 48.9 | pipelines/compile_ms 88.55, pipelines/front_ms 84.41, render/thread/ms 72.26, render/busy/ms 70.67, main/busy/ms 46.19, stage/render/prepare_assets 38.07, stage/main/update 24.30, stage/main/post_update 15.60, stage/render/render 15.22, stream/drive/ms 14.09, render/graph/ms 13.06, encode//sum 10.94 |
| warm-up | 5.72 | 39.1 | render/thread/ms 77.91, render/busy/ms 76.34, main/busy/ms 51.38, stage/render/prepare_assets 46.64, stage/main/update 29.15, stage/main/post_update 15.92, stage/render/render 13.94, stream/drive/ms 13.48, encode//sum 11.79, render/graph/ms 11.71, stage/render/graph_submit 7.37, encode/pass:shadow_point_light/sum 4.96 |
| warm-up | 5.78 | 52.1 | pipelines/compile_ms 209, pipelines/front_ms 204, render/thread/ms 79.13, render/busy/ms 77.47, main/busy/ms 43.08, stage/render/prepare_assets 38.25, stage/main/update 22.14, stage/render/render 14.90, stage/main/post_update 14.47, render/graph/ms 12.45, encode//sum 12.08, stage/render/graph_submit 7.82 |
| warm-up | 5.81 | 34.7 | pipelines/compile_ms 1196, pipelines/front_ms 1161, render/thread/ms 75.92, render/busy/ms 74.22, stage/render/prepare_assets 40.04, main/busy/ms 38.47, pipelines/driver_ms 35.24, stage/render/render 15.68, stage/main/post_update 14.92, render/graph/ms 12.95, stage/main/update 12.67, encode//sum 10.65 |
| warm-up | 5.84 | 34.1 | pipelines/compile_ms 1828, pipelines/front_ms 1769, pipelines/driver_ms 58.70, render/thread/ms 58.37, render/busy/ms 56.75, main/busy/ms 34.45, stage/render/prepare_assets 30.37, stage/render/render 16.11, stage/main/post_update 14.34, render/graph/ms 13.61, stage/main/update 9.35, stage/render/graph_submit 9.20 |
| warm-up | 5.87 | 29.4 | pipelines/compile_ms 1073, pipelines/front_ms 1039, render/thread/ms 54.95, render/busy/ms 53.55, pipelines/driver_ms 33.99, stage/render/prepare_assets 31.28, main/busy/ms 25.02, stage/render/render 14.71, render/graph/ms 12.49, stage/main/post_update 10.41, encode//sum 9.08, stage/main/update 8.77 |
| warm-up | 5.91 | 32.7 | pipelines/compile_ms 261, pipelines/front_ms 255, render/thread/ms 56.73, render/busy/ms 55.51, stage/render/prepare_assets 36.54, main/busy/ms 24.91, stage/render/render 12.83, render/graph/ms 10.78, encode//sum 10.35, stage/main/update 10.13, stage/main/post_update 9.31, stage/render/graph_submit 6.75 |
| warm-up | 5.98 | 26.9 | pipelines/compile_ms 203, pipelines/front_ms 197, render/thread/ms 43.35, render/busy/ms 41.90, main/busy/ms 29.42, stage/render/render 19.80, render/graph/ms 17.74, stage/main/post_update 12.44, stage/main/update 12.37, stage/render/graph_submit 10.55, encode//sum 10.45, stage/render/prepare_assets 7.95 |
| warm-up | 6.08 | 30.9 | pipelines/compile_ms 620, pipelines/front_ms 606, main/busy/ms 37.72, render/thread/ms 31.11, render/busy/ms 29.65, stage/main/update 22.95, stream/drive/ms 17.02, pipelines/driver_ms 13.94, stage/render/render 13.47, render/graph/ms 11.39, encode//sum 11.06, stage/main/post_update 9.36 |
| warm-up | 6.35 | 27.0 | main/busy/ms 33.54, render/thread/ms 25.09, render/busy/ms 24.02, stage/main/update 21.12, pipelines/compile_ms 20.69, pipelines/front_ms 19.76, stream/drive/ms 13.56, stage/render/render 11.66, render/graph/ms 9.84, encode//sum 9.28, stage/main/post_update 8.99, stage/render/graph_submit 6.29 |
| warm-up | 6.43 | 32.2 | render/thread/ms 46.59, render/busy/ms 45.24, main/busy/ms 27.65, stage/render/render 15.52, render/graph/ms 13.83, stage/main/post_update 11.89, stage/main/update 11.70, encode//sum 11.54, stage/render/graph_submit 9.88, stage/render/specialize 7.73, encode/pass:shadow_point_light/sum 6.91, encode/submit 6.85 |
| warm-up | 6.47 | 31.2 | render/thread/ms 34.42, render/busy/ms 33.46, main/busy/ms 23.05, stage/render/prepare_assets 13.14, stage/render/render 10.18, stage/main/post_update 9.58, render/graph/ms 8.77, stage/main/update 8.45, encode//sum 8.06, stage/render/graph_submit 6.02, encode/pass:shadow_point_light/sum 4.53, encode/submit 3.89 |
| warm-up | 6.70 | 28.4 | render/thread/ms 45.04, render/busy/ms 43.83, main/busy/ms 39.34, stage/main/update 19.69, stage/main/post_update 14.64, stage/render/render 12.89, stage/render/specialize 11.80, render/graph/ms 11.32, stage/render/graph_submit 8.24, encode//sum 7.95, stream/drive/ms 5.81, encode/submit 5.77 |
| warm-up | 6.73 | 30.7 | render/thread/ms 51.73, render/busy/ms 50.46, main/busy/ms 43.34, stage/main/update 18.31, stage/main/post_update 17.72, stage/render/render 13.81, render/graph/ms 12.09, stage/render/specialize 9.86, stage/render/prepare_assets 8.36, encode//sum 8.16, stage/render/graph_submit 7.54, stream/drive/ms 6.12 |
| warm-up | 6.75 | 27.8 | render/thread/ms 48.31, render/busy/ms 47.15, main/busy/ms 40.02, stage/main/update 17.27, stage/main/post_update 15.94, stage/render/prepare_assets 14.81, stage/render/render 12.34, render/graph/ms 10.53, encode//sum 8.49, stage/render/graph_submit 6.04, encode/pass:shadow_point_light/sum 5.29, stage/render/specialize 5.02 |
| warm-up | 6.78 | 27.0 | render/thread/ms 41.83, render/busy/ms 40.74, main/busy/ms 32.18, stage/render/prepare_assets 17.64, stage/main/update 14.67, stage/main/post_update 11.33, stage/render/render 11.31, render/graph/ms 9.79, encode//sum 9.30, stage/render/graph_submit 6.75, encode/pass:shadow_point_light/sum 5.10, stream/drive/ms 4.52 |
| warm-up | 6.84 | 30.9 | main/busy/ms 44.67, render/thread/ms 44.54, render/busy/ms 42.80, stage/main/update 25.17, stage/render/render 18.06, render/graph/ms 15.59, stream/drive/ms 14.22, stage/main/post_update 13.80, stage/render/graph_submit 11.86, encode//sum 11.61, encode/submit 8.95, encode/pass:shadow_point_light/sum 7.52 |
| warm-up | 6.87 | 34.2 | render/thread/ms 48.02, render/busy/ms 46.70, main/busy/ms 41.22, stage/main/update 21.84, stage/render/prepare_assets 14.78, stage/main/post_update 13.48, stage/render/render 10.86, encode//sum 9.55, render/graph/ms 8.95, encode/pass:shadow_point_light/sum 5.86, stage/render/graph_submit 5.28, stage/render/specialize 5.26 |
| warm-up | 7.07 | 29.0 | render/thread/ms 49.43, render/busy/ms 48.31, stage/render/prepare_assets 29.51, main/busy/ms 24.30, stage/main/post_update 11.50, stage/render/render 11.32, encode//sum 10.00, render/graph/ms 9.52, stage/main/update 8.78, stage/render/graph_submit 6.47, encode/pass:shadow_point_light/sum 5.31, encode/submit 4.04 |
| warm-up | 7.15 | 27.8 | render/thread/ms 47.04, render/busy/ms 46.04, main/busy/ms 26.16, stage/render/prepare_assets 20.01, stage/main/update 11.79, stage/render/render 10.25, stage/main/post_update 10.10, encode//sum 9.98, render/graph/ms 8.67, stage/render/graph_submit 5.68, encode/pass:shadow_point_light/sum 5.67, stage/render/specialize 4.23 |
| warm-up | 7.21 | 28.4 | render/thread/ms 38.13, render/busy/ms 36.98, main/busy/ms 31.93, stage/main/update 17.25, stage/render/render 13.69, encode//sum 12.05, render/graph/ms 12.04, stage/main/post_update 9.41, stage/render/graph_submit 9.01, stage/render/prepare_assets 8.80, stream/drive/ms 7.06, encode/pass:shadow_point_light/sum 5.79 |
| warm-up | 7.25 | 26.5 | render/thread/ms 37.76, render/busy/ms 36.57, main/busy/ms 26.79, stage/render/render 23.39, render/graph/ms 21.22, stage/render/graph_submit 16.31, stage/main/post_update 15.61, encode//sum 15.04, encode/submit 12.23, stage/main/update 7.97, encode/pass:shadow_point_light/sum 6.26, stage/render/camera_driver 4.75 |
| warm-up | 7.31 | 36.9 | main/busy/ms 40.97, render/thread/ms 33.38, render/busy/ms 32.16, stage/main/update 22.31, stream/drive/ms 12.78, stage/render/prepare_assets 12.22, stage/render/render 11.61, stage/main/post_update 11.11, render/graph/ms 9.85, encode//sum 8.78, stage/render/graph_submit 6.72, encode/pass:shadow_point_light/sum 5.06 |
| measure | 2.71 | 27.0 | render/thread/ms 36.28, render/busy/ms 35.12, encode//sum 26.14, stage/render/render 23.01, main/busy/ms 21.29, render/graph/ms 21.28, encode/pass:shadow_point_light/sum 18.67, stage/render/graph_submit 15.16, stage/main/post_update 13.56, encode/submit 9.61, stage/render/camera_driver 5.97, encode/finish_wall 5.56 |
| measure | 18.19 | 28.9 | main/busy/ms 38.71, stage/main/update 24.46, render/thread/ms 24.16, render/busy/ms 23.11, stream/drive/ms 20.21, stage/render/render 14.48, render/graph/ms 13.00, stage/main/post_update 11.47, encode//sum 11.15, stage/render/graph_submit 9.37, encode/pass:shadow_point_light/sum 7.76, encode/submit 6.20 |
| measure | 59.04 | 27.7 | render/thread/ms 39.88, render/busy/ms 38.61, main/busy/ms 24.68, stage/render/prepare_assets 14.23, stage/main/post_update 12.59, stage/render/render 11.93, render/graph/ms 10.26, encode//sum 8.76, stage/main/update 7.20, stage/render/graph_submit 7.07, encode/submit 4.22, stage/render/camera_driver 3.01 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=nocull-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (253 samples): GPU load **72%** (min 43%) · 2625 MHz (min 2625) · memory 8101 MHz · 74.9 W · 75 °C (max 81) · VRAM 7016 MiB · throttled 100% of samples

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
