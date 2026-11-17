# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.face_cull=on (command line: --set); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6191 frames over 65.6 s · **median 10.15 ms (98 fps)** · mean 10.60 · p95 13.84 · p99 15.57 (1% low 64 fps) · worst 30.87 ms · 84 spikes over 1.5× median · 27.4 stutters/min (work 21.0/min)
- warm-up: 323 frames over 9.1 s, worst 2494.6 ms
- hitches (> 25.4 ms): **5** measured, 44 in warm-up
- timed GPU passes: 6.51 ms of a 10.60 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.15 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 30.87 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.980 | 2.029 | 2.062 | 0.98 | 0.173 |
| main_opaque_pass_3d | 1.255 | 1.286 | 1.635 | 0.98 | 0.064 |
| early prepass | 0.472 | 0.483 | 0.697 | 0.98 | 0.085 |
| bloom | 0.308 | 0.316 | 0.322 | 0.98 | 0.061 |
| atmosphere_luts | 0.302 | 0.310 | 0.372 | 0.98 | 0.003 |
| water_velocity | 0.257 | 0.263 | 0.350 | 0.98 | 0.002 |
| auxiliary_views | 0.202 | 0.207 | 1.029 | 0.98 | 0.190 |
| water_sspr | 0.196 | 0.201 | 0.203 | 0.98 | 0.002 |
| ssao | 0.184 | 0.188 | 0.193 | 0.98 | 0.007 |
| upscaling | 0.126 | 0.130 | 0.137 | 0.98 | 0.004 |
| water_fft | 0.126 | 0.129 | 0.137 | 0.98 | 0.003 |
| atmos_froxel_inject | 0.124 | 0.127 | 0.128 | 0.98 | 0.001 |
| uniform_allocation | 0.121 | 0.124 | 0.152 | 0.98 | 0.025 |
| clouds_composite | 0.102 | 0.105 | 0.108 | 0.98 | 0.002 |
| bin_unpacking | 0.095 | 0.097 | 0.118 | 0.98 | 0.018 |
| tonemapping | 0.085 | 0.087 | 0.091 | 0.98 | 0.003 |
| main_transmissive_pass_3d | 0.077 | 0.079 | 0.158 | 0.98 | 0.008 |
| clustering | 0.062 | 0.064 | 0.067 | 0.98 | 0.019 |
| clouds_trace | 0.054 | 0.056 | 0.073 | 0.98 | 0.001 |
| render_sky | 0.050 | 0.051 | 0.052 | 0.98 | 0.002 |
| clouds_resolve | 0.048 | 0.049 | 0.050 | 0.98 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.412 | 0.416 | 0.11 | 0.001 |
| surface_trail | 0.038 | 0.056 | 0.057 | 0.67 | 0.002 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.032 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.027 | 0.028 | 0.036 | 0.98 | 0.005 |
| water_snapshot | 0.024 | 0.025 | 0.028 | 0.98 | 0.002 |
| lightprobe_radiance_map | 0.024 | 0.218 | 0.228 | 0.11 | 0.005 |
| water_mips | 0.022 | 0.022 | 0.023 | 0.98 | 0.002 |
| night_sky | 0.016 | 0.016 | 0.037 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.023 | 0.98 | 0.123 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.98 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.009 | 0.010 | 0.014 | 0.98 | 0.004 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.11 | 0.003 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.180 | 12.449 | 22.01 | 56831.5 |
| main/busy/ms | 8.934 | 10.870 | 28.71 | 55311.8 |
| render/busy/ms | 8.664 | 11.945 | 21.50 | 53635.8 |
| stage/main/post_update | 5.569 | 7.157 | 10.81 | 34475.7 |
| stage/render/render | 5.508 | 8.123 | 13.55 | 34102.7 |
| render/graph/ms | 4.811 | 7.364 | 12.72 | 29784.8 |
| encode//sum | 4.193 | 6.507 | 15.47 | 25961.7 |
| stage/render/graph_submit | 3.596 | 5.446 | 8.64 | 22263.6 |
| encode/submit | 2.310 | 3.683 | 5.99 | 14301.0 |
| stage/main/update | 2.124 | 2.573 | 22.19 | 13148.0 |
| encode/finish_wall | 1.327 | 1.857 | 4.24 | 8213.4 |
| stage/render/camera_driver | 1.162 | 1.905 | 4.02 | 7196.7 |
| encode/pass:shadow_point_light/sum | 1.042 | 4.797 | 14.31 | 6453.5 |
| encode/pass:shadow_directional_light/sum | 0.758 | 1.220 | 1.97 | 4695.5 |
| encode/pass:main_opaque_pass_3d/sum | 0.741 | 1.057 | 2.08 | 4589.3 |
| encode/pass:early prepass/sum | 0.729 | 1.044 | 1.83 | 4510.7 |
| stage/render/prepare_write_phase_buffers | 0.719 | 1.051 | 2.14 | 4452.7 |
| bigspace/total/ms | 0.569 | 1.371 | 2.35 | 3521.7 |
| stage/render/prepare_assets | 0.568 | 0.716 | 8.62 | 3519.3 |
| stage/render/prepare_resources | 0.449 | 0.528 | 0.94 | 2779.9 |
| render/present/ms | 0.423 | 0.530 | 1.92 | 2618.8 |
| stage/render/prepare_meshes | 0.407 | 0.514 | 2.11 | 2521.3 |
| stage/main/first | 0.395 | 0.515 | 5.57 | 2443.1 |
| stage/main/pre_update | 0.352 | 0.501 | 1.76 | 2177.6 |
| terrain/update/ms | 0.315 | 0.412 | 2.50 | 1952.6 |
| terrain/select/ms | 0.281 | 0.368 | 1.00 | 1738.1 |
| controller/feed/ms | 0.266 | 0.331 | 0.85 | 1646.4 |
| stage/render/prepare_bind_groups | 0.253 | 0.422 | 2.23 | 1564.5 |
| stage/render/specialize | 0.252 | 0.384 | 3.29 | 1559.2 |
| stream/drive/ms | 0.246 | 0.332 | 19.99 | 1525.9 |
| encode/pass:main_transparent_pass_3d/sum | 0.241 | 0.356 | 1.00 | 1492.8 |
| stage/render/queue | 0.232 | 0.360 | 2.85 | 1434.6 |
| bigspace/low_precision/ms | 0.230 | 0.750 | 1.79 | 1421.1 |
| stage/main/fixed_main_loop | 0.207 | 0.356 | 1.03 | 1282.9 |
| far/select/ms | 0.200 | 0.265 | 0.61 | 1237.0 |
| stream/tick/ms | 0.191 | 0.261 | 0.66 | 1181.0 |
| stage/render/create_views | 0.190 | 0.245 | 0.98 | 1173.4 |
| stage/render/prepare_batch_phases | 0.184 | 0.263 | 0.63 | 1138.7 |
| bigspace/high_precision/ms | 0.172 | 0.754 | 1.63 | 1064.4 |
| stage/render/extract_commands | 0.170 | 0.197 | 0.48 | 1050.9 |
| anim/evaluate/ms | 0.170 | 0.315 | 1.51 | 1050.8 |
| stage/render/prepare_flush | 0.163 | 0.340 | 1.05 | 1008.3 |
| anim/evaluate_cpu/ms | 0.151 | 0.190 | 0.78 | 937.9 |
| stage/render/prepare_views | 0.136 | 0.170 | 0.46 | 843.2 |
| controller/move/ms | 0.131 | 0.257 | 0.78 | 812.7 |
| stage/main/last | 0.112 | 0.152 | 1.82 | 695.6 |
| encode/pass:main_transmissive_pass_3d/sum | 0.101 | 0.169 | 0.61 | 626.8 |
| render/acquire/ms | 0.093 | 0.120 | 0.39 | 576.9 |
| stage/render/prepare_collect_phase_buffers | 0.065 | 0.136 | 0.58 | 403.2 |
| physics/step/ms | 0.044 | 0.082 | 0.59 | 271.1 |
| stage/render/phase_sort | 0.034 | 0.057 | 0.53 | 207.7 |
| stage/render/cleanup | 0.033 | 0.106 | 0.79 | 206.1 |
| physics/insert/ms | 0.031 | 0.345 | 0.95 | 194.9 |
| encode/resolve_diagnostics/sum | 0.030 | 0.040 | 0.17 | 184.6 |
| pipelines/compile_ms | 0.016 | 0.000 | 87.89 | 98.7 |
| weather/map/ms | 0.016 | 0.021 | 0.27 | 98.0 |
| pipelines/front_ms | 0.015 | 0.000 | 85.15 | 95.8 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.021 | 0.54 | 89.6 |
| anim/apply/ms | 0.011 | 0.020 | 0.46 | 69.9 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.22 | 66.6 |
| controller/avoid/ms | 0.010 | 0.014 | 0.21 | 64.6 |
| weather/eval/ms | 0.009 | 0.012 | 0.13 | 58.3 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 43.4 |
| script/input/ms | 0.005 | 0.006 | 0.16 | 32.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.06 | 30.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.32 | 28.9 |
| script/ui/ms | 0.004 | 0.006 | 0.21 | 26.7 |
| controller/write/ms | 0.003 | 0.004 | 0.11 | 21.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.68 | 14.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 6.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.22 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.19 | 3.4 |
| pipelines/driver_ms | 0.000 | 0.000 | 2.75 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.1 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.02 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1277 | 2262 | 652 |
| crowd/c0/meshes/count | count | 353 | 646 | 162 |
| crowd/c1/count | count | 147 | 157 | 147 |
| crowd/c1/joints/count | count | 5539 | 5999 | 5544 |
| crowd/c1/meshes/count | count | 1177 | 1297 | 1179 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 164 | 216 | 205 |
| crowd/joints_extracted/count | count | 3685 | 7659 | 5763 |
| crowd/joints_uploaded/count | count | 3643 | 6220 | 5763 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.4 k | 59.4 k |
| encode//max | ms | 0.41 | 1.25 | 0.43 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.54 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.22 | 0.01 |
| encode/finish_critical | ms | 0.93 | 2.08 | 0.98 |
| encode/pass:early prepass/max | ms | 0.76 | 1.83 | 0.63 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.76 | 2.08 | 0.98 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.61 | 0.07 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.23 | 1.00 | 0.25 |
| encode/pass:shadow_directional_light/max | ms | 0.76 | 1.97 | 0.66 |
| encode/pass:shadow_point_light/max | ms | 0.78 | 1.54 | 0.78 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.17 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 8.97 | 28.71 | 7.73 |
| gpu/timed/ms | ms | 7.38 | 10.78 | 7.73 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 291.8 k | 291.8 k | 291.8 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 21 |
| phases/opaque3d/bins | count | 1286 | 1398 | 1357 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3362 | 5099 | 3193 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1286 | 1398 | 1357 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3362 | 5099 | 3193 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 17 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 13 | 178 | 4 |
| phases/shadow/bins | count | 3682 | 4414 | 1481 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6084 | 7344 | 3033 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 198 | 226 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 328 | 329 | 329 |
| pipelines/waiting | count | 0 | 1 | 0 |
| precache/draw_waiting | count | 0 | 1 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 513 | 9 |
| preprocess/bind_groups_created | count | 11 | 513 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 280 | 124 |
| process/cpu_usage | % | 26.09 | 41.83 | 23.08 |
| process/mem_usage | GiB | 5.41 | 5.58 | 5.58 |
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
| skin/blocks_changed/count | count | 13 | 86 | 13 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 164 | 216 | 205 |
| skin/joint_bytes_written/bytes | bytes | 282.1 k | 811.8 k | 444.7 k |
| skin/joints_written/count | count | 4408 | 12.7 k | 6948 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4918 | 5009 | 5009 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.73 | 36.87 | 29.25 |
| system/mem_usage | % | 54.32 | 54.91 | 54.86 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3016 | 3016 |
| textures/images | count | 2682 | 2740 | 2740 |
| views/all/batch_sets | count | 63 | 227 | 56 |
| views/all/bins | count | 6240 | 7211 | 4989 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 20.2 k | 10.7 k |
| views/all/pipelines | count | 243 | 273 | 67 |
| views/all/pipelines_distinct | count | 70 | 75 | 63 |
| views/all/work_items | count | 10.5 k | 17.5 k | 8152 |
| views/cascade/batch_sets | count | 8 | 27 | 4 |
| views/cascade/bins | count | 1440 | 1521 | 1481 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 3397 | 4446 | 3033 |
| views/cascade/pipelines | count | 22 | 27 | 19 |
| views/cascade/work_items | count | 321 | 4443 | 222 |
| views/lamp_face/batch_sets | count | 0 | 171 | 0 |
| views/lamp_face/bins | count | 2469 | 2964 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/culled | count | 23 | 48 | 0 |
| views/lamp_face/drawn | count | 0 | 44 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2747 | 3288 | 0 |
| views/lamp_face/pipelines | count | 175 | 199 | 0 |
| views/lamp_face/revealed | count | 0 | 4 | 0 |
| views/lamp_face/work_items | count | 0 | 2682 | 0 |
| views/main/batch_sets | count | 50 | 53 | 52 |
| views/main/bins | count | 2880 | 3572 | 3508 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8324 | 13.0 k | 7622 |
| views/main/pipelines | count | 47 | 49 | 48 |
| views/main/work_items | count | 8494 | 13.2 k | 7930 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.9 k | 30.8 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 45.55 | 251 | 282.0 k |
| anim/events/count | count | 0.10 | 3 | 590 |
| assets/Image/added | count | 0.10 | 34 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.66 | 152 | 4061 |
| assets/Mesh/removed | count | 0.78 | 1242 | 4842 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 457 |
| assets/StandardMaterial/modified | count | 3.50 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.76 | 1305 | 4675 |
| encode//count | count | 35.44 | 203 | 219.4 k |
| encode/bind_walk/init_textures | count | 13.5 k | 104.1 k | 83.80 M |
| encode/bind_walk/init_textures_skipped | count | 51.4 k | 113.1 k | 318.09 M |
| encode/bind_walk/inits | count | 458 | 1051 | 2.84 M |
| encode/bind_walk/inits_skipped | count | 98.14 | 140 | 607.6 k |
| encode/bind_walk/merge_textures | count | 13.5 k | 104.0 k | 83.41 M |
| encode/bind_walk/merge_textures_skipped | count | 5383 | 6012 | 33.33 M |
| encode/bind_walk/merges | count | 101 | 316 | 624.0 k |
| encode/bind_walk/merges_skipped | count | 79.52 | 123 | 492.3 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6191 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6191 |
| encode/pass:early prepass/count | count | 1 | 1 | 6191 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6191 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6191 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6191 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6191 |
| encode/pass:shadow_point_light/count | count | 2.13 | 44 | 13.2 k |
| encode/queue/command_buffers | count | 47.58 | 257 | 294.6 k |
| encode/queue/submits | count | 2.02 | 4 | 12.5 k |
| encode/queue/write_buffer/bytes | bytes | 1.81 M | 2.67 M | 11.19 G |
| encode/queue/write_buffer/calls | count | 255 | 767 | 1.58 M |
| encode/queue/write_buffer_with/bytes | bytes | 165.8 k | 3.52 M | 1.03 G |
| encode/queue/write_buffer_with/calls | count | 10.95 | 465 | 67.8 k |
| encode/queue/write_texture/bytes | bytes | 10.1 k | 8.59 M | 62.48 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 562 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6191 |
| physics/inserts | count | 0.36 | 111 | 2256 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 145 | 2335 |
| physics/steps | count | 0.64 | 2 | 3938 |
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
| stream/spawn/items | count | 0.06 | 17 | 378 |
| stream/upload/bytes | bytes | 5468 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1199 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.4 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2494.6 | main/busy/ms 2463, stage/main/post_update 2423, stage/main/update 31.05, stage/main/pre_update 5.01, stage/main/last 1.19, surface/advance_cpu_ms 0.74, stage/main/first 0.54, stage/main/fixed_main_loop 0.34, surface/cell_upload_ms 0.31, stream/drive/ms 0.30, bigspace/total/ms 0.09, stream/tick/ms 0.08 |
| warm-up | 3.24 | 745.0 | main/busy/ms 2606, stage/main/post_update 2449, render/thread/ms 711, render/busy/ms 540, stage/render/prepare_assets 490, stage/render/prepare_views 165, render/acquire/ms 165, stage/main/update 143, stage/render/create_views 28.22, stage/render/render 16.65, render/graph/ms 9.50, stage/render/camera_driver 7.43 |
| warm-up | 3.43 | 191.5 | render/thread/ms 885, render/busy/ms 605, stage/render/prepare_assets 541, main/busy/ms 172, stage/render/prepare_views 165, render/acquire/ms 165, pipelines/compile_ms 139, stage/render/render 131, stage/main/update 118, render/present/ms 115, pipelines/driver_ms 100, stage/main/post_update 39.44 |
| warm-up | 3.70 | 261.0 | pipelines/wait_ms 3699, pipelines/compile_ms 2748, pipelines/front_ms 2232, pipelines/driver_ms 515, encode//sum 368, main/busy/ms 150, render/thread/ms 139, render/busy/ms 137, stage/render/prepare_assets 69.96, stage/main/update 68.32, stage/render/render 59.98, render/graph/ms 56.80 |
| warm-up | 3.75 | 43.1 | pipelines/wait_ms 3782, pipelines/compile_ms 2936, pipelines/front_ms 2547, pipelines/driver_ms 389, encode//sum 379, render/thread/ms 169, render/busy/ms 166, stage/render/prepare_assets 87.72, stage/render/render 73.24, render/graph/ms 70.75, stage/render/graph_submit 68.20, encode/finish_wall 53.16 |
| warm-up | 3.93 | 82.5 | pipelines/compile_ms 1272, pipelines/front_ms 1234, main/busy/ms 60.25, render/thread/ms 60.20, pipelines/wait_ms 58.56, render/busy/ms 57.91, stage/main/update 46.62, stream/drive/ms 41.71, pipelines/driver_ms 38.01, stage/render/prepare_assets 37.77, stage/render/render 15.05, render/graph/ms 12.49 |
| warm-up | 3.99 | 44.3 | pipelines/compile_ms 545, pipelines/front_ms 522, render/thread/ms 49.47, render/busy/ms 47.14, main/busy/ms 43.28, stage/main/update 27.55, pipelines/wait_ms 25.71, pipelines/driver_ms 23.19, stage/render/render 21.22, stream/drive/ms 20.30, stage/render/prepare_assets 20.22, pipelines/lock_ms 19.25 |
| warm-up | 4.04 | 44.7 | pipelines/compile_ms 329, pipelines/front_ms 315, render/thread/ms 65.12, render/busy/ms 62.54, main/busy/ms 55.55, stage/render/prepare_assets 38.03, stage/main/update 34.98, stream/drive/ms 27.53, pipelines/wait_ms 22.37, stage/render/render 17.89, pipelines/lock_ms 16.20, render/graph/ms 14.71 |
| warm-up | 4.13 | 85.6 | pipelines/compile_ms 309, pipelines/front_ms 298, render/thread/ms 87.08, render/busy/ms 85.14, main/busy/ms 70.21, stage/render/prepare_assets 59.13, stage/main/update 43.26, stream/drive/ms 27.49, stage/main/post_update 16.68, stage/render/render 16.65, render/graph/ms 14.26, stage/render/graph_submit 12.00 |
| warm-up | 4.19 | 60.7 | pipelines/compile_ms 1335, pipelines/front_ms 1301, render/thread/ms 89.29, render/busy/ms 87.82, stage/render/prepare_assets 69.18, main/busy/ms 47.17, pipelines/driver_ms 34.34, stage/main/update 21.64, stage/main/post_update 16.20, pipelines/wait_ms 14.32, pipelines/lock_ms 13.51, stage/render/render 9.96 |
| warm-up | 4.22 | 36.4 | pipelines/compile_ms 1782, pipelines/front_ms 1734, render/thread/ms 70.19, render/busy/ms 68.18, main/busy/ms 65.22, pipelines/driver_ms 48.22, stage/main/update 41.39, stage/render/prepare_assets 33.67, pipelines/wait_ms 17.11, stage/render/render 16.73, pipelines/lock_ms 16.06, render/graph/ms 13.40 |
| warm-up | 4.31 | 84.1 | pipelines/compile_ms 828, pipelines/front_ms 800, render/thread/ms 82.75, render/busy/ms 80.59, main/busy/ms 80.10, stage/main/update 55.01, pipelines/wait_ms 36.62, stage/render/prepare_assets 35.41, pipelines/driver_ms 27.81, stage/render/render 19.26, render/graph/ms 15.68, stage/main/post_update 11.16 |
| warm-up | 4.42 | 112.9 | pipelines/compile_ms 1900, pipelines/front_ms 1849, pipelines/wait_ms 480, render/thread/ms 151, render/busy/ms 149, main/busy/ms 63.96, pipelines/driver_ms 51.47, stage/render/prepare_assets 45.65, encode//sum 34.41, stage/main/update 33.40, stage/render/prepare_resources 29.64, stage/render/render 24.62 |
| warm-up | 4.45 | 35.3 | pipelines/compile_ms 2651, pipelines/front_ms 2580, pipelines/wait_ms 821, render/thread/ms 135, render/busy/ms 131, pipelines/driver_ms 70.46, main/busy/ms 58.50, encode//sum 54.49, stage/render/render 34.11, render/graph/ms 30.41, stage/render/prepare_resources 29.25, stage/main/post_update 20.50 |
| warm-up | 4.52 | 63.6 | pipelines/compile_ms 1392, pipelines/front_ms 1351, pipelines/wait_ms 750, render/thread/ms 89.00, render/busy/ms 85.79, main/busy/ms 57.06, encode//sum 47.83, pipelines/driver_ms 41.19, stage/render/render 36.95, render/graph/ms 33.27, stage/main/post_update 28.47, stage/render/graph_submit 24.93 |
| warm-up | 4.59 | 73.9 | pipelines/compile_ms 1355, pipelines/front_ms 1309, pipelines/wait_ms 969, render/thread/ms 126, render/busy/ms 124, main/busy/ms 83.48, stage/main/post_update 63.15, encode//sum 49.46, stage/render/render 46.92, pipelines/driver_ms 45.52, render/graph/ms 40.07, bigspace/total/ms 27.90 |
| warm-up | 4.62 | 32.5 | pipelines/compile_ms 1312, pipelines/front_ms 1271, pipelines/wait_ms 689, main/busy/ms 150, render/thread/ms 98.07, render/busy/ms 95.89, stage/main/update 91.62, stream/drive/ms 53.42, stage/main/post_update 46.42, pipelines/driver_ms 40.82, stage/render/render 40.54, render/graph/ms 33.54 |
| warm-up | 4.77 | 144.8 | pipelines/compile_ms 2199, pipelines/front_ms 2143, pipelines/wait_ms 781, main/busy/ms 116, stage/main/update 95.62, render/thread/ms 86.08, render/busy/ms 84.13, pipelines/driver_ms 56.14, stream/drive/ms 54.81, stage/render/prepare_assets 41.13, stage/render/render 22.47, render/graph/ms 20.01 |
| warm-up | 4.80 | 31.2 | pipelines/compile_ms 2193, pipelines/front_ms 2135, pipelines/wait_ms 686, render/thread/ms 85.24, main/busy/ms 84.97, render/busy/ms 83.66, stage/main/update 63.20, pipelines/driver_ms 57.30, stream/drive/ms 51.14, stage/render/prepare_assets 40.50, stage/render/render 18.23, render/graph/ms 15.87 |
| warm-up | 4.90 | 102.7 | pipelines/compile_ms 698, pipelines/front_ms 675, main/busy/ms 87.02, render/thread/ms 68.24, render/busy/ms 66.84, stage/main/update 63.24, stream/drive/ms 50.47, stage/render/prepare_assets 37.36, pipelines/driver_ms 23.42, stage/render/render 15.31, stage/main/post_update 14.60, render/graph/ms 13.08 |
| warm-up | 4.94 | 33.9 | pipelines/compile_ms 444, pipelines/front_ms 426, main/busy/ms 90.89, render/thread/ms 70.60, render/busy/ms 68.81, stage/main/update 63.11, stream/drive/ms 46.20, stage/render/prepare_assets 41.46, stage/main/post_update 20.04, stage/render/render 18.63, pipelines/driver_ms 17.86, render/graph/ms 16.17 |
| warm-up | 5.03 | 94.2 | pipelines/compile_ms 1129, pipelines/front_ms 1098, main/busy/ms 91.33, stage/main/update 69.35, render/thread/ms 68.19, render/busy/ms 66.41, stream/drive/ms 45.63, stage/render/prepare_assets 37.47, pipelines/driver_ms 31.54, stage/render/render 18.81, render/graph/ms 16.40, stage/main/post_update 15.39 |
| warm-up | 5.06 | 29.5 | pipelines/compile_ms 1145, pipelines/front_ms 1113, render/thread/ms 62.38, render/busy/ms 60.97, main/busy/ms 55.83, stage/main/update 41.62, stage/render/prepare_assets 35.70, pipelines/driver_ms 31.57, stream/drive/ms 24.73, stage/render/render 13.46, render/graph/ms 11.52, encode//sum 10.22 |
| warm-up | 5.17 | 35.9 | pipelines/compile_ms 101, pipelines/front_ms 96.84, render/thread/ms 48.46, render/busy/ms 46.94, main/busy/ms 36.61, stage/main/post_update 18.43, stage/render/render 15.97, render/graph/ms 13.95, stage/render/prepare_assets 12.86, stage/main/update 12.65, stage/render/graph_submit 9.76, encode//sum 8.97 |
| warm-up | 5.21 | 44.5 | render/thread/ms 71.70, render/busy/ms 69.40, main/busy/ms 58.24, stage/main/update 32.95, stage/render/prepare_resources 20.56, stage/main/post_update 19.16, stage/render/render 15.90, render/graph/ms 13.88, stage/render/prepare_assets 13.14, stage/render/graph_submit 9.72, encode//sum 8.98, encode/submit 7.46 |
| warm-up | 5.25 | 43.8 | pipelines/compile_ms 312, pipelines/front_ms 304, render/thread/ms 73.31, render/busy/ms 71.01, main/busy/ms 54.45, stage/main/update 34.91, stage/render/prepare_assets 21.26, stage/render/prepare_resources 16.11, stage/render/render 15.61, render/graph/ms 13.71, stage/main/post_update 13.21, stage/render/graph_submit 9.46 |
| warm-up | 5.29 | 32.7 | pipelines/compile_ms 725, pipelines/front_ms 702, render/thread/ms 59.58, render/busy/ms 58.16, main/busy/ms 38.02, stage/render/prepare_assets 27.05, pipelines/driver_ms 23.21, stage/main/post_update 17.88, stage/render/render 14.47, stage/main/update 14.26, render/graph/ms 12.42, encode//sum 10.03 |
| warm-up | 5.32 | 31.1 | pipelines/compile_ms 1089, pipelines/front_ms 1059, render/thread/ms 57.95, render/busy/ms 56.41, main/busy/ms 38.93, pipelines/driver_ms 30.29, pipelines/wait_ms 21.41, stage/main/post_update 19.93, stage/render/render 16.25, render/graph/ms 14.06, stage/main/update 13.62, stage/render/prepare_write_phase_buffers 11.33 |
| warm-up | 5.51 | 27.0 | pipelines/compile_ms 169, pipelines/front_ms 164, render/thread/ms 42.35, render/busy/ms 41.15, main/busy/ms 30.10, stage/main/post_update 15.12, stage/render/prepare_assets 15.01, stage/render/render 14.54, render/graph/ms 12.49, encode//sum 11.95, stage/main/update 10.24, stage/render/graph_submit 7.48 |
| warm-up | 5.54 | 32.4 | render/thread/ms 53.73, render/busy/ms 52.34, main/busy/ms 38.04, stage/render/prepare_assets 22.31, stage/main/update 20.02, stage/render/render 14.88, stage/main/post_update 12.72, render/graph/ms 12.64, stream/drive/ms 9.93, encode//sum 9.90, stage/render/graph_submit 7.29, stage/render/camera_driver 5.13 |
| warm-up | 5.59 | 41.8 | render/thread/ms 60.59, render/busy/ms 59.25, main/busy/ms 34.45, stage/render/prepare_assets 28.83, stage/render/render 18.95, stage/main/update 18.15, render/graph/ms 16.95, pipelines/compile_ms 14.99, pipelines/front_ms 14.25, stage/render/graph_submit 11.70, stage/main/post_update 10.85, encode//sum 9.99 |
| warm-up | 5.62 | 32.0 | pipelines/compile_ms 312, pipelines/front_ms 293, render/thread/ms 58.54, render/busy/ms 57.23, stage/render/prepare_assets 31.80, main/busy/ms 25.07, pipelines/driver_ms 19.07, stage/render/render 16.54, render/graph/ms 14.52, stage/main/post_update 10.94, stage/render/graph_submit 10.55, encode//sum 9.16 |
| warm-up | 5.67 | 31.8 | pipelines/compile_ms 50.57, render/thread/ms 49.07, render/busy/ms 47.85, pipelines/front_ms 46.76, stage/render/prepare_assets 29.77, main/busy/ms 23.84, stage/render/render 11.89, stage/main/post_update 10.95, encode//sum 10.33, render/graph/ms 9.91, stage/main/update 8.07, stage/render/graph_submit 6.35 |
| warm-up | 5.70 | 28.4 | pipelines/compile_ms 55.87, render/thread/ms 53.74, pipelines/front_ms 52.85, render/busy/ms 52.55, stage/render/prepare_assets 32.44, main/busy/ms 25.94, stage/render/render 13.72, stage/main/post_update 12.37, render/graph/ms 11.89, encode//sum 10.51, stage/main/update 8.50, stage/render/graph_submit 7.77 |
| warm-up | 5.75 | 27.9 | pipelines/compile_ms 671, pipelines/front_ms 654, render/thread/ms 47.98, render/busy/ms 46.66, stage/render/prepare_assets 27.08, main/busy/ms 22.82, pipelines/driver_ms 16.96, stage/render/render 13.40, render/graph/ms 11.53, stage/main/post_update 9.96, encode//sum 9.86, stage/main/update 8.45 |
| warm-up | 6.12 | 25.5 | main/busy/ms 29.34, render/thread/ms 22.15, render/busy/ms 21.18, stage/main/update 18.87, stream/drive/ms 13.51, stage/render/render 10.16, render/graph/ms 8.66, stage/main/post_update 7.13, encode//sum 7.02, stage/render/graph_submit 5.61, encode/pass:shadow_point_light/sum 4.87, stage/render/prepare_assets 4.38 |
| warm-up | 6.26 | 31.1 | render/thread/ms 39.47, render/busy/ms 38.45, main/busy/ms 24.69, stage/render/prepare_assets 17.97, stage/render/render 10.55, stage/main/update 9.85, render/graph/ms 9.11, stage/main/post_update 8.98, encode//sum 8.70, stage/render/graph_submit 6.41, encode/pass:shadow_point_light/sum 5.58, encode/submit 4.17 |
| warm-up | 6.48 | 28.4 | pipelines/compile_ms 167, pipelines/front_ms 161, render/thread/ms 43.34, main/busy/ms 42.92, render/busy/ms 40.79, stage/render/render 20.37, render/graph/ms 17.21, stage/main/update 17.07, stage/main/post_update 16.33, stage/render/graph_submit 12.07, encode//sum 11.63, encode/submit 8.58 |
| warm-up | 6.51 | 28.4 | render/thread/ms 45.55, render/busy/ms 43.03, main/busy/ms 42.33, pipelines/compile_ms 23.26, pipelines/front_ms 21.66, stage/main/update 18.43, stage/render/render 16.41, stage/main/post_update 14.41, render/graph/ms 13.45, stage/render/prepare_assets 12.95, encode//sum 10.29, stage/render/graph_submit 9.19 |
| warm-up | 6.58 | 31.0 | main/busy/ms 40.91, render/thread/ms 34.51, render/busy/ms 32.88, stage/main/update 23.17, stage/render/render 17.93, render/graph/ms 15.34, stream/drive/ms 13.93, stage/main/post_update 13.47, encode//sum 12.02, stage/render/graph_submit 10.97, encode/submit 7.65, encode/pass:shadow_point_light/sum 7.16 |
| warm-up | 6.64 | 33.0 | main/busy/ms 38.28, render/thread/ms 37.50, render/busy/ms 36.29, stage/main/update 20.26, stage/render/render 14.69, render/graph/ms 12.97, stage/main/post_update 11.94, stage/render/prepare_assets 11.31, encode//sum 10.19, stage/render/graph_submit 9.32, stream/drive/ms 7.23, encode/pass:shadow_point_light/sum 6.79 |
| warm-up | 6.66 | 26.8 | pipelines/compile_ms 150, pipelines/front_ms 146, render/thread/ms 38.95, render/busy/ms 37.88, main/busy/ms 32.88, stage/render/prepare_assets 15.87, stage/main/update 15.53, stage/render/render 11.22, stage/main/post_update 10.71, render/graph/ms 9.73, encode//sum 8.01, stage/render/graph_submit 6.79 |
| warm-up | 6.82 | 26.3 | render/thread/ms 44.18, render/busy/ms 43.14, main/busy/ms 31.27, stage/render/prepare_assets 26.23, stage/main/update 14.12, stage/main/post_update 12.23, stage/render/render 9.64, render/graph/ms 8.16, encode//sum 6.95, stage/render/graph_submit 5.65, stream/drive/ms 4.79, encode/submit 3.32 |
| warm-up | 7.00 | 37.2 | main/busy/ms 41.58, render/thread/ms 27.66, render/busy/ms 26.64, stage/main/update 23.09, stream/drive/ms 13.03, stage/render/prepare_assets 11.72, stage/main/post_update 11.50, stage/render/render 9.10, render/graph/ms 7.63, encode//sum 6.52, stage/render/graph_submit 5.44, encode/submit 3.17 |
| measure | 0.75 | 26.2 | main/busy/ms 32.53, render/thread/ms 25.15, render/busy/ms 24.01, stage/main/update 17.10, stream/drive/ms 12.64, stage/main/post_update 12.21, stage/render/render 11.66, render/graph/ms 10.14, encode//sum 9.54, stage/render/graph_submit 7.44, stage/render/prepare_assets 5.77, encode/submit 4.28 |
| measure | 4.88 | 26.7 | main/busy/ms 32.86, render/thread/ms 23.40, render/busy/ms 22.36, stage/main/update 15.82, stage/main/post_update 14.22, stage/render/render 11.91, stream/drive/ms 11.30, render/graph/ms 10.22, encode//sum 9.33, stage/render/graph_submit 6.62, encode/pass:shadow_point_light/sum 6.01, stage/render/prepare_assets 4.13 |
| measure | 18.16 | 30.9 | main/busy/ms 38.39, stage/main/update 24.45, stream/drive/ms 20.25, render/thread/ms 17.65, render/busy/ms 16.60, stage/main/post_update 11.34, stage/render/render 9.80, render/graph/ms 8.35, encode//sum 6.96, stage/render/graph_submit 6.25, encode/submit 4.20, encode/finish_wall 2.07 |
| measure | 23.06 | 26.3 | render/thread/ms 36.07, render/busy/ms 34.93, main/busy/ms 29.33, encode//sum 16.85, stage/render/render 14.40, stage/main/post_update 13.49, render/graph/ms 12.74, stage/main/update 11.96, stage/render/graph_submit 8.94, stage/render/prepare_assets 8.13, encode/pass:shadow_point_light/sum 7.93, encode/submit 4.85 |
| measure | 59.01 | 26.8 | render/thread/ms 34.62, main/busy/ms 33.43, render/busy/ms 32.76, stage/render/render 20.20, stage/main/post_update 20.13, render/graph/ms 17.73, encode//sum 14.78, stage/render/graph_submit 13.21, stage/main/update 8.48, encode/submit 8.44, encode/finish_wall 4.86, stage/render/prepare_assets 4.64 |

## Run

- `target/profiling/perftests.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --set shadow.lamp.face_cull=on --resolution 2560x1440 --bench-out target/perf\pt_night\balmora_walk_night@2560x1440_arm=facecull-4 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 15%
- during the run: other processes 6% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **74%** (min 44%) · 2625 MHz (min 2625) · memory 8101 MHz · 77.4 W · 76 °C (max 82) · VRAM 6754 MiB · throttled 100% of samples

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
- `shadow.lamp.face_cull=on (command line: --set)`
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
